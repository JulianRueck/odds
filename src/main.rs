use cdd::{
    discovery::{self},
    history::History,
    navigator::Navigator,
    paths,
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

    let mut navigator = Navigator {
        session_stack: &mut session_stack,
        history: &mut history,
    };

    // Do a regular cd if it's an explicit path
    if let Some(dir) = paths::detect_explicit_path(&args[1]) {
        navigator.do_jump(&dir);

        return;
    }

    // Model / history lookup
    // if confident -> cd
    // if ambigous -> picker
    // if no candidates -> bounded discovery below

    // Bounded discovery
    let results = discovery::discover(&args[1], 5, 9);

    navigator.pick_and_jump(&results);
}
