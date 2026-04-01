use std::path::PathBuf;

use crate::discovery::DiscoveryCandidate;

use super::Matchkind;

/// Tries to match a candidate to the token. First through strong matching e.g.
/// - Exact
/// - Prefix
/// - Substring
///
/// When none of these match a potential fuzzy match is computated.
pub fn match_candidate(path: &PathBuf, name: &str, token: &str) -> Option<DiscoveryCandidate> {
    if name.is_empty() || token.is_empty() {
        return None;
    }

    // Phase 1: Strong matches.
    if let Some((match_kind, score)) = strong_match(name, token) {
        return Some(DiscoveryCandidate {
            path: path.clone(),
            match_kind,
            score,
        });
    }

    // Phase 2: Fuzzy fallback.
    fuzzy_match(name, token).map(|score| DiscoveryCandidate {
        path: path.clone(),
        match_kind: Matchkind::Fuzzy,
        score: score.min(45.0),
    })
}

// Matches using equality, prefix or substring. In that order.
fn strong_match(name: &str, token: &str) -> Option<(Matchkind, f32)> {
    if name == token {
        Some((Matchkind::Exact, 100.0))
    } else if name.starts_with(&token) {
        Some((Matchkind::Prefix, 70.0))
    } else if name.contains(&token) {
        Some((Matchkind::Substring, 50.0))
    } else {
        None
    }
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
