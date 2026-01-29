use std::{
    io::{self, Write},
};

use crate::discovery::DiscoveryCandidate;

pub fn pick_directory(candidates: &[DiscoveryCandidate]) -> Option<&DiscoveryCandidate> {
    if candidates.is_empty() {
        return None;
    }
    // TODO: Maybe add a guard e.g. iter().take(max)
    // Do I sync it with max_results?
    println!(
        "Select a directory (1-{}), or 0 to cancel:",
        candidates.len()
    );
    for (i, candidate) in candidates.iter().enumerate() {
        println!("{}) {} | score: {}", i + 1, candidate.path.display(), candidate.score);
    }

    print!("Enter number: ");
    io::stdout().flush().ok()?;

    let mut input = String::new();
    io::stdin().read_line(&mut input).ok()?;
    let choice: usize = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => return None,
    };

    if choice == 0 || choice > candidates.len() {
        return None;
    } else {
        Some(&candidates[choice - 1])
    }
}
