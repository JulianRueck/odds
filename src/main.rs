use cdd::{
    discovery::{self},
    history::History,
    navigator,
    paths,
    ranking::{self, MlWeights},
    session::SessionStack,
};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    // Exit if arguments are shit; TODO: Add some more checks maybe
    if args.len() < 2 {
        eprintln!("Usage: cdd <name>");
        return;
    }

    // TODO: Config; SessionStack: max_size, discovery: max_depth, max_results

    let mut session_stack = SessionStack::new(10);
    let mut history = History::load().unwrap_or_default();

    let token = &args[1];
    let max_results = 9;
    let max_depth = 5;

    // Do a regular cd if it's an explicit path
    if let Some(dir) = paths::detect_explicit_path(&args[1]) {
        navigator::do_jump(&dir, &mut history, &mut session_stack);

        return;
    }

    // Model / history lookup
    // if confident -> cd
    // if ambigous -> picker
    // if no candidates -> bounded discovery below

    // Bounded discovery
    let discovery_candidates = discovery::discover(token, max_depth, max_results);

    // ML ranking
    let ranked_candidates = ranking::rank_candidates(
        discovery_candidates,
        &history,
        &session_stack,
        &MlWeights::default(),
        max_results,
    );

    navigator::pick_and_jump(&ranked_candidates, &mut history, &mut session_stack);
}
