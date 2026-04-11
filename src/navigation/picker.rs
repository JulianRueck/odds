use std::io::{self, Write};

use crate::ranking::RankedCandidate;

/// Denotes confidence in a candidate in order to effectuate auto jump functionality.
#[derive(Debug)]
pub struct ConfidenceRules {
    pub min_ranked_score: f32,
    pub min_score: f32,
}

impl Default for ConfidenceRules {
    fn default() -> Self {
        Self {
            min_ranked_score: 70.0,
            min_score: 35.0,
        }
    }
}

pub enum SelectionStrategy {
    Manual { choice: usize },
    Confident { rules: ConfidenceRules },
}

pub fn pick_directory(candidates: &[RankedCandidate]) -> Option<&RankedCandidate> {
    if candidates.is_empty() {
        return None;
    }

    eprintln!("Select a directory (1-{}):", candidates.len());

    for (i, candidate) in candidates.iter().enumerate() {
        eprintln!("{}) {}", i + 1, candidate.path.display());
    }

    eprint!("Enter number: ");
    io::stdout().flush().ok()?;

    let mut input = String::new();
    io::stdin().read_line(&mut input).ok()?;
    let choice: usize = input.trim().parse().ok()?;

    let index = select_index(candidates, SelectionStrategy::Manual { choice })?;

    candidates.get(index)
}

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
            let first = candidates.first()?;

            if first.score < rules.min_score {
                return None;
            }

            let second = match candidates.get(1) {
                Some(s) => s,
                None => return Some(0),
            };

            let first_valid = first.ranked_score >= rules.min_ranked_score && first.score >= second.score;

            let second_valid = second.score >= first.score
                && second.ranked_score >= rules.min_ranked_score
                && second.score > rules.min_score;

            if first_valid {
                Some(0)
            } else if second_valid {
                Some(1)
            } else {
                None
            }
        }
    }
}
