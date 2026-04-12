use std::{
    path::PathBuf,
    time::{SystemTime, UNIX_EPOCH},
};

use crate::{
    discovery::DiscoveryCandidate, persistence::{History, Session}, ranking::RankedCandidate
};

pub const FRECENCY_WEIGHT: f32 = 10.0;
pub const MARKOV_WEIGHT: f32 = 15.0;
pub const SESSION_WEIGHT: f32 = 5.0;

pub const HALF_LIFE_DAYS: f32 = 3.0;
// λ = ln(2) / T1/2
pub const LAMBDA: f32 = 0.69314718 / (HALF_LIFE_DAYS * 24.0 * 60.0 * 60.0);

pub fn rank_candidates(
    candidates: Vec<DiscoveryCandidate>,
    history: &History,
    session: &Session,
    max_results: usize,
) -> Vec<RankedCandidate> {
    let current_path = session.current();

    let mut ranked: Vec<RankedCandidate> = candidates
        .into_iter()
        .map(|candidate| {
            let ranked_score = score_candidate(&candidate, history, session, current_path);
            RankedCandidate {
                candidate,
                ranked_score,
            }
        })
        .collect();

    // Sort by score descending.
    ranked.sort_by(|a, b| b.ranked_score.total_cmp(&a.ranked_score));
    ranked.truncate(max_results);

    ranked
}

fn score_candidate(
    candidate: &DiscoveryCandidate,
    history: &History,
    session: &Session,
    current_path: Option<&PathBuf>,
) -> f32 {
    let mut score: f32 = 0.0;

    // Exponential Decay (Frecency).
    score += calculate_frecency_score(&candidate.path, history);

    // Markov Chain boost.
    if let Some(from) = current_path {
        if candidate.path != *from {
            let to_str = candidate.path.to_str().expect("Invalid UTF-8 in path.");
            let from_str = from.to_str().expect("Invalid UTF-8 in current path.");

            let prob = history.calculate_probability_from(to_str, from_str);

            score += prob * MARKOV_WEIGHT;
        }
    }

    // Session boost.
    if session.contains(&candidate.path) {
        score += SESSION_WEIGHT;
    }

    // Mixing ranking score with match score and passing it into sigmoid; making the returned score always between 0 and 1.
    sigmoid(score * candidate.score)
}

// Having the ability to inject time makes the decay testable.
fn calculate_frecency_score_at(path: &PathBuf, history: &History, now: u64) -> f32 {
    // The logarithm flattens out potential dominance caused by a high visit count.
    let frequency = (history.visit_count(path) as f32 + 1.0).ln();

    if let Some(seconds_ago) = history.seconds_since_last_visit_at(path, now) {
        let decay_factor = (-LAMBDA * seconds_ago as f32).exp();
        return frequency * decay_factor * FRECENCY_WEIGHT;
    }

    0.0
}

fn calculate_frecency_score(path: &PathBuf, history: &History) -> f32 {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();
    calculate_frecency_score_at(path, history, now)
}

fn sigmoid(x: f32) -> f32 {
    1.0 / (1.0 + (-x / 100.0).exp())
}

#[cfg(test)]
#[path = "ranker_tests.rs"]
mod ranker_tests;
