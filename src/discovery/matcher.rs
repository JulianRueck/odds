use std::path::PathBuf;

use pathfinding::{matrix::Matrix, prelude::kuhn_munkres};

use crate::discovery::DiscoveryCandidate;

pub const EXACT_SCORE: f32 = 100.0;
pub const PREFIX_SCORE: f32 = 70.0;
pub const SUBSTRING_SCORE: f32 = 50.0;
pub const FUZZY_SCORE_CAP: f32 = 30.0;

/// Matches a path against multiple tokens using the Hungarian algorithm to find
/// the optimal assignment of tokens to path segments.
///
/// Each token is matched against every segment of the path, producing a score
/// matrix. The Hungarian algorithm then finds the assignment that maximises the
/// total score, ensuring each segment is claimed by at most one token.
///
/// The returned score is the average across all tokens, so partial matches
/// (where some tokens find no segment) are penalised proportionally rather
/// than rejected outright. A candidate is only rejected if no token matches
/// any segment at all.
///
/// # Example
///
/// Given the path `/home/user/projects/api` and tokens `["proj", "api"]`:
/// - `"proj"` matches `"projects"` as a prefix (70.0)
/// - `"api"` matches `"api"` as exact (100.0)
/// - average score: 85.0
pub fn match_candidate_multi(path: &PathBuf, tokens: &[&str]) -> Option<DiscoveryCandidate> {
    if tokens.is_empty() {
        return None;
    }

    let segments: Vec<String> = path
        .components()
        .filter_map(|c| c.as_os_str().to_str())
        .map(|s| s.to_lowercase())
        .collect();

    if segments.is_empty() {
        return None;
    }

    let tokens_l: Vec<String> = tokens.iter().map(|t| t.to_lowercase()).collect();

    let cols = segments.len();
    // Having more rows than columns would make Hungarian panic.
    // Average is still calculated by total length of tokens, making for a penalty.
    let rows = tokens_l.len().min(cols);

    let matrix = Matrix::from_fn(rows, cols, |(t, s)| {
        match_candidate(&segments[s], &tokens_l[t])
            .map(|score| (score * 100.0) as i64)
            .unwrap_or(0)
    });

    let (total, _) = kuhn_munkres(&matrix);

    let avg = total as f32 / (tokens_l.len() as f32 * 100.0);

    if avg == 0.0 {
        return None;
    }

    Some(DiscoveryCandidate {
        path: path.clone(),
        score: avg,
    })
}

fn match_candidate(name: &str, token: &str) -> Option<f32> {
    if name.is_empty() || token.is_empty() {
        return None;
    }

    if let Some(score) = strong_match(name, token) {
        return Some(score);
    }

    fuzzy_match(name, token).map(|score| score.min(FUZZY_SCORE_CAP))
}

fn strong_match(name: &str, token: &str) -> Option<f32> {
    if name == token {
        return Some(EXACT_SCORE);
    }

    let base_score = 
    if name.starts_with(token) {
        Some(PREFIX_SCORE)
    } else if name.contains(token) {
        Some(SUBSTRING_SCORE)
    } else {
        None
    };

    let ratio = token.len() as f32 / name.len() as f32;
    base_score
        .map(|score| score * ratio)
        .filter(|&score| score >= FUZZY_SCORE_CAP)
}

// Sequential character fuzzy match with position based scoring.
fn fuzzy_match(name: &str, token: &str) -> Option<f32> {
    let mut score = 0.0;
    let mut last_match = None;
    let mut chars = token.chars();
    let mut current = chars.next()?;

    for (i, c) in name.chars().enumerate() {
        if c == current {
            score += 10.0;

            if let Some(prev) = last_match {
                if i == prev + 1 {
                    score += 15.0;
                } else {
                    score -= (i - prev - 1) as f32;
                }
            } else {
                score -= i as f32 * 0.5;
            }

            last_match = Some(i);

            if let Some(next) = chars.next() {
                current = next;
            } else {
                // Ensure there's always some positive score using max().
                return Some(score.max(1.0));
            }
        }
    }

    None
}

#[cfg(test)]
#[path = "matcher_tests.rs"]
mod matcher_tests;
