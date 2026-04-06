use clap::Parser;

use odds::{
    args::{Cli, Commands},
    discovery::{self},
    navigation::{
        navigator,
        picker::{self, ConfidenceRules},
    },
    paths,
    persistence::{History, Session},
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

        Some(Commands::Query { tokens }) => {
            if !tokens.is_empty() {
                odds(tokens);
            }
        }

        None => {
            eprintln!("Usage: odds <COMMAND>");
        }
    }
}

fn odds(raw_tokens: &[String]) {
    let tokens: Vec<&str> = raw_tokens.iter().map(|t| t.as_str()).collect();

    let mut session = Session::load_or_new();
    let mut history = History::load_or_new();

    const MAX_RESULTS: usize = 9;
    const MAX_DEPTH: usize = 5;

    // Do a regular cd if it's an explicit path.
    if let Some(dir) = paths::detect_explicit_path(tokens[0]) {
        navigator::do_jump(&dir, &mut history, &mut session);
        return;
    }

    let history_candidates = history.history_candidates(&tokens);

    let ranked_candidates = ranker::rank_candidates(history_candidates, &history, &session, MAX_RESULTS);

    // If confident auto jump.
    if let Some(choice) = picker::confident_pick(&ranked_candidates, ConfidenceRules::default()) {
        navigator::do_jump(&choice.candidate.path, &mut history, &mut session);
        return;
    }

    // Bounded discovery.
    let discovery_candidates = discovery::discover(&tokens, MAX_DEPTH, MAX_RESULTS);

    let ranked_candidates = ranker::rank_candidates(discovery_candidates, &history, &session, MAX_RESULTS);

    // Picker.
    navigator::pick_and_jump(&ranked_candidates, &mut history, &mut session);
}
