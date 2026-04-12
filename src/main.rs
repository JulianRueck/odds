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
    persistence::{History, Session, persistable::Persistable},
    ranking::ranker,
    seeder,
};

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Init { shell }) => {
            Cli::handle_init(shell);
            return;
        }

        Some(Commands::Seed) => {
            if let Err(e) = seeder::seed() {
                eprintln!("Error seeding odds: {e}");
            }
            return;
        }

        Some(Commands::Register { pwd }) => {
            let path = PathBuf::from(pwd);
            let mut history = History::load_or_new();
            let mut session = Session::load_or_new();

            if let Some(current) = session.current() {
                history.register_markov_chain(current, &path);
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

        Some(Commands::Query { tokens }) => {  
            odds(tokens);
        }

        None => {
            eprintln!("Usage: odds [COMMAND]");
        }
    }
}

fn odds(raw_tokens: &[String]) {
    let session = Session::load_or_new();
    let history = History::load_or_new();

    const MAX_RESULTS: usize = 9;
    const MAX_DEPTH: usize = 5;

    // No arguments jumps to ~ just like cd while still registering the jump.
    if raw_tokens.is_empty() {
        navigator::do_jump(&paths::home_dir());
        return;
    }

    // Mimicking 'cd -' behaviour whilst still registering the jump.
    if raw_tokens.len() == 1 && raw_tokens[0] == "-" {
        if let Some(previous) = session.previous().cloned() {
            let home = paths::home_dir();
            let display = previous.strip_prefix(&home)
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

    let ranked_candidates = ranker::rank_candidates(history_candidates, &history, &session, MAX_RESULTS);

    // If confident auto jump.
    if let Some(choice) = picker::confident_pick(&ranked_candidates, ConfidenceRules::default()) {
        navigator::do_jump(&choice.candidate.path);
        return;
    }

    // Bounded discovery.
    let discovery_candidates = discovery::discover(&tokens, MAX_DEPTH, MAX_RESULTS);

    let ranked_candidates = ranker::rank_candidates(discovery_candidates, &history, &session, MAX_RESULTS);

    // Picker.
    navigator::pick_and_jump(&ranked_candidates);
}
