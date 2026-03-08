use std::io::{self, Write};

use crate::ranking::{ConfidenceRules, RankedCandidate};

pub enum SelectionStrategy {
    Manual { choice: usize },
    Confident { rules: ConfidenceRules },
}

pub fn pick_directory(candidates: &[RankedCandidate]) -> Option<&RankedCandidate> {
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
    let choice: usize = input.trim().parse().ok()?;

    let index = select_index(candidates, SelectionStrategy::Manual { choice })?;

    candidates.get(index)
}

/// Pick candidate based on the highest score and the distance between it and the second best being great enough.
pub fn confident_pick(
    candidates: &[RankedCandidate],
    rules: ConfidenceRules,
) -> Option<&RankedCandidate> {
    let index = select_index(candidates, SelectionStrategy::Confident { rules: rules })?;

    candidates.get(index)
}

pub fn select_index(candidates: &[RankedCandidate], strategy: SelectionStrategy) -> Option<usize> {
    match strategy {
        SelectionStrategy::Manual { choice } => {
            if choice == 0 || choice > candidates.len() {
                None
            } else {
                Some(choice - 1)
            }
        }

        SelectionStrategy::Confident { rules } => {
            if candidates.len() <= 1 {
                return (!candidates.is_empty()).then_some(0);
            }

            let first = &candidates[0];
            let second = &candidates[1];

            if first.ml_score >= rules.min_score
                && first.ml_score - second.ml_score >= rules.min_gap
            {
                Some(0)
            } else {
                None
            }
        }
    }
}
