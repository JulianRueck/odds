use std::env;

mod utils;
mod session;

use crate::session::SessionStack;

fn main() {
    let args: Vec<String> = env::args().collect();

    // Exit if arguments are shit; TODO: Add some more check maybe
    if args.len() < 2 {
        eprintln!("Usage: cdd <name>");
        return;
    }

    let mut session = SessionStack::new(10);

    // Do a regular cd if it's an explicit path
    if let Some(dir) = utils::detect_explicit_path(&args[1]) {
        print!("{dir}");

        // Store in short term session stask
        session.push(dir);


        // TODO: add to history
        return;
    }

    
    println!("Hello, {}!", args[1]); // TODO: remove
}
