use cdd::{
    discovery::{self},
    navigator, paths,
    persistence::{History, SessionStack, persistable::Persistable},
    picker,
    ranking::{self, ConfidenceRules, MlWeights},
};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: cdd <name>");
        return;
    }

    // TODO: Config; SessionStack: max_size, discovery: max_depth, max_results
    // TODO: Make session expire
    let mut session_stack = SessionStack::load_or_new();
    let mut history = History::load().unwrap(); // TODO: Error handling 

    let token = &args[1];
    let max_results = 9;
    let max_depth = 5;

    // Do a regular cd if it's an explicit path
    if let Some(dir) = paths::detect_explicit_path(&args[1]) {
        navigator::do_jump(&dir, &mut history, &mut session_stack);

        return;
    }

    let history_candidates = history.history_candidates(token);

    let ranked_candidates = ranking::rank_candidates(
        history_candidates,
        &history,
        &session_stack,
        &MlWeights::default(),
        max_results,
    );

    // If confident auto jump
    if let Some(choice) = picker::confident_pick(&ranked_candidates, ConfidenceRules::default()) {
        navigator::do_jump(&choice.candidate.path, &mut history, &mut session_stack);

        return;
    }

    // Ambigious -> picker

    // Bounded discovery
    let discovery_candidates = discovery::discover(token, max_depth, max_results);

    let ranked_candidates = ranking::rank_candidates(
        discovery_candidates,
        &history,
        &session_stack,
        &MlWeights::default(),
        max_results,
    );

    // Picker
    navigator::pick_and_jump(&ranked_candidates, &mut history, &mut session_stack);
}
