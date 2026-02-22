use std::{
    io::{self, Write},
};

use crate::{discovery::DiscoveryCandidate, ranking::{ConfidenceRules, RankedCandidate}};

pub fn pick_directory(candidates: &[RankedCandidate]) -> Option<&DiscoveryCandidate> {
    if candidates.is_empty() {
        return None;
    }
  
    println!(
        "Select a directory (1-{}), or 0 to cancel:",
        candidates.len()
    );
    
    for (i, candidate) in candidates.iter().enumerate() {
        println!("{}) {}", i + 1, candidate.path.display());
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

/// Pick candidate based on the highest score and the distance between it and the second best being great enough.
pub fn confident_pick<'a>(
    candidates: &'a [RankedCandidate],
    rules: &ConfidenceRules,
) -> Option<&'a RankedCandidate> {
    if candidates.len() < 2 {
        return candidates.first();
    }
    
    let first = &candidates[0];
    let second = &candidates[1];

    if first.ml_score >= rules.min_score && first.ml_score - second.ml_score >= rules.min_gap {
        Some(&first)
    } else {
        None
    }
}
