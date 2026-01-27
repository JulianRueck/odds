use cdd::{args, config, discovery, explain, history::{self, History}, model, paths, ranking, session, tui};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    // Exit if arguments are shit; TODO: Add some more check maybe
    if args.len() < 2 {
        eprintln!("Usage: cdd <name>");
        return;
    }

    let mut session = session::SessionStack::new(10);

    let mut history = History::load().unwrap_or_default();

    // Do a regular cd if it's an explicit path
    if let Some(dir) = paths::detect_explicit_path(&args[1]) {
        print!("{dir}");

        // Store in short term session stack
        session.push(&dir);

        // Store in history
        history.record_visit(&dir);
        history.save().ok();

        return;
    }

    // Model / history lookup
    // if confident -> cd
    // if ambigous -> TUI picker
    // if no candidates -> bounded discovery below

    // Bounded discovery
    // cd + learn

    let roots = paths::search_roots();
    let results = discovery::discover(&roots, &args[1], 5, 10);
    // TODO: pass results to TUI picker
}
