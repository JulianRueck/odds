use std::path::PathBuf;

use clap::Parser;

use odds::{
    args::{Cli, Commands},
    discovery::{self},
    navigation::{
        navigator,
        picker::{self, ConfidenceRules},
    },
    paths,
    persistence::{History, Session, markov::MARKOV_N, persistable::Persistable},
    ranking::ranker,
    seeder,
};

const MAX_RESULTS: usize = 9;
const MAX_DEPTH: usize = 5;

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Init { shell }) => {
            Cli::handle_init(shell);
        }

        Some(Commands::Seed) => {
            if let Err(e) = seeder::seed() {
                eprintln!("Error seeding odds: {e}");
            }
        }

        Some(Commands::Register { pwd }) => {
            handle_register(pwd);
        }
        // Main logic
        Some(Commands::Query { tokens }) => {
            odds(tokens);
        }

        Some(Commands::Reset) => {
            History::reset();
            Session::reset();
        }

        Some(Commands::Where { tokens }) => {
            handle_where(tokens);
        }

        None => {
            eprintln!("Usage: odds [COMMAND]");
        }
    }
}

fn handle_register(pwd: &String) {
    let path = PathBuf::from(pwd);
    let mut history = History::load_or_new();
    let mut session = Session::load_or_new();

    if let Some(from) = session.current() {
        let context: Vec<&PathBuf> = session
            .entries
            .iter()
            .skip(1)
            .take(MARKOV_N - 1)
            .map(|e| &e.path)
            .collect();

        history.chain.register(&context, from, &path);
    }

    history.record_visit(&path);
    session.push(&path);

    if let Err(e) = history.save() {
        eprintln!("Error saving history while registering regular cd: {e}")
    }

    if let Err(e) = session.save() {
        eprintln!("Error saving session while registering regular cd: {e}")
    }

    return;
}

fn odds(raw_tokens: &[String]) {
    let session = Session::load_or_new();
    let history = History::load_or_new();

    // No arguments jumps to ~ just like cd while still registering the jump.
    if raw_tokens.is_empty() {
        navigator::do_jump(&paths::home_dir());
        return;
    }

    // Mimicking 'cd -' behaviour whilst still registering the jump.
    if raw_tokens.len() == 1 && raw_tokens[0] == "-" {
        if let Some(previous) = session.previous().cloned() {
            let home = paths::home_dir();
            let display = previous
                .strip_prefix(&home)
                .map(|p| format!("~/{}", p.display()))
                .unwrap_or_else(|_| previous.display().to_string());
            eprintln!("{}", display);
            navigator::do_jump(&previous);
        }
        return;
    }

    let tokens: Vec<&str> = raw_tokens.iter().map(|t| t.as_str()).collect();

    // Jump immediately if it's an explicit path.
    if let Some(dir) = paths::detect_explicit_path(tokens[0]) {
        navigator::do_jump(&dir);
        return;
    }

    let history_candidates = history.history_candidates(&tokens);

    let ranked_candidates =
        ranker::rank_candidates(history_candidates, &history, &session, MAX_RESULTS);

    // If confident auto jump.
    if let Some(choice) = picker::confident_pick(&ranked_candidates, ConfidenceRules::default()) {
        navigator::do_jump(&choice.candidate.path);
        return;
    }

    // Bounded discovery.
    let discovery_candidates = discovery::discover(&tokens, MAX_DEPTH, MAX_RESULTS);

    let ranked_candidates =
        ranker::rank_candidates(discovery_candidates, &history, &session, MAX_RESULTS);

    // Picker.
    navigator::pick_and_jump(&ranked_candidates);
}

fn handle_where(tokens: &[String]) {
    let tokens: Vec<&str> = tokens.iter().map(|t| t.as_str()).collect();
    let history = History::load_or_new();
    let session = Session::load_or_new();

    let history_candidates = history.history_candidates(&tokens);
    let mut ranked = ranker::rank_candidates(history_candidates, &history, &session, MAX_RESULTS);

    if ranked.is_empty() {
        let discovery_candidates = discovery::discover(&tokens, MAX_DEPTH, MAX_RESULTS);
        ranked = ranker::rank_candidates(discovery_candidates, &history, &session, MAX_RESULTS);
    }

    if ranked.is_empty() {
        eprintln!("No candidates found for: {}", tokens.join(" "));
        return;
    }

    let would_jump = picker::confident_pick(&ranked, ConfidenceRules::default()).is_some();

    if would_jump {
        let top = &ranked[0];

        eprintln!("  Would jump to: {}", top.candidate.path.display());
        eprintln!("  Match score:   {:.1} / 10.0", top.candidate.score);
        eprintln!("  Ranked score:  {:.1}", top.ranked_score);
        eprintln!("  Visits:        {}", history.visit_count(&top.candidate.path));

        if let Some(secs) = history.seconds_since_last_visit(&top.candidate.path) {
            eprintln!("  Last visited:  {}", format_duration(secs));
        }

        let from_str = session.current().and_then(|p| p.to_str()).unwrap_or("");
        let to_str = top.candidate.path.to_str().unwrap_or("");
        let markov_prob = history.chain.calculate_probability_from(&[], from_str, to_str);
        if markov_prob > 0.0 {
            eprintln!("  Markov:        {:.0}% likely from current location", markov_prob * 100.0);
        }

        if ranked.len() > 1 {
            eprintln!("\n  Other candidates:");
            for (i, c) in ranked.iter().skip(1).enumerate() {
                eprintln!(
                    "  {}  {}  match: {:.1}  ranked: {:.1}",
                    i + 2,
                    c.candidate.path.display(),
                    c.candidate.score,
                    c.ranked_score,
                );
            }
        }
    } else {
        eprintln!("  No confident match found. Would show picker:\n");
        for (i, c) in ranked.iter().enumerate() {
            eprintln!(
                "  {}  {}  match: {:.1}  ranked: {:.1}",
                i + 1,
                c.candidate.path.display(),
                c.candidate.score,
                c.ranked_score,
            );
        }

        if let Some(top) = ranked.first() {
            if history.visit_count(&top.candidate.path) > 0 {
                eprintln!(
                    "\n  Tip: visit {} a few more times to build confidence for auto-jump.",
                    top.candidate.path.display()
                );
            }
        }
    }
}

fn format_duration(secs: u64) -> String {
    if secs < 60 {
        "just now".to_string()
    } else if secs < 3600 {
        format!("{} minutes ago", secs / 60)
    } else if secs < 86400 {
        format!("{} hours ago", secs / 3600)
    } else {
        format!("{} days ago", secs / 86400)
    }
}
