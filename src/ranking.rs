use std::ops::Deref;

use crate::{
    discovery::{DiscoveryCandidate, Matchkind},
    history::History,
    session::SessionStack,
};

/// Determines how much to 'trust' certain signals.
/// Some of these values might seem duplicates (matchers.rs).
/// However, one should consider those signal strength.
/// This modulates those signals.
#[derive(Debug)]
pub struct MlWeights {
    pub exact: f32,
    pub prefix: f32,
    pub substring: f32,
    pub fuzzy: f32,
    pub frequency: f32,
    pub recency: f32,
    pub session_stack: f32,
}

impl Default for MlWeights {
    fn default() -> Self {
        Self {
            exact: 100.0,
            prefix: 70.0,
            substring: 50.0,
            fuzzy: 20.0,
            frequency: 2.0,
            recency: 10.0,
            session_stack: 5.0,
        }
    }
}

/// Denotes confidence in a candidate in order effectuate auto jump functionality.
#[derive(Debug)]
pub struct ConfidenceRules {
    pub min_score: f32,
    pub min_gap: f32,
}

impl Default for ConfidenceRules {
    fn default() -> Self {
        Self {
            min_score: 120.0,
            min_gap: 20.0,
        }
    }
}

#[derive(Debug)]
pub struct RankedCandidate {
    pub candidate: DiscoveryCandidate,
    pub ml_score: f32,
}

impl Default for RankedCandidate {
    fn default() -> Self {
        Self {
            candidate: DiscoveryCandidate::default(),
            ml_score: 0.0,
        }
    }
}

impl Deref for RankedCandidate {
    type Target = DiscoveryCandidate;
    fn deref(&self) -> &Self::Target {
        &self.candidate
    }
}

pub fn rank_candidates(
    candidates: Vec<DiscoveryCandidate>,
    history: &History,
    session_stack: &SessionStack,
    weights: &MlWeights,
    max_results: usize,
) -> Vec<RankedCandidate> {
    let mut ranked: Vec<RankedCandidate> = candidates
        .into_iter()
        .map(|c| {
            let ml_score = score_candidate(&c, history, session_stack, weights);
            RankedCandidate {
                candidate: c,
                ml_score,
            }
        })
        .collect();

    // Sort by score descending and tie-break lexicographically.
    ranked.sort_by(|a, b| {
        b.ml_score
            .total_cmp(&a.ml_score)
            .then_with(|| a.candidate.path.cmp(&b.candidate.path))
    });

    ranked.truncate(max_results);

    ranked
}

fn score_candidate(
    candidate: &DiscoveryCandidate,
    history: &History,
    session_stack: &SessionStack,
    weights: &MlWeights,
) -> f32 {
    let mut score = match candidate.match_kind {
        Matchkind::Exact => weights.exact,
        Matchkind::Prefix => weights.prefix,
        Matchkind::Substring => weights.substring,
        Matchkind::Fuzzy => weights.fuzzy,
    };

    // Frequency.
    let frequency = history.visit_count(&candidate.path) as f32;
    score += frequency * weights.frequency;

    // Recency
    if let Some(seconds_ago) = history.seconds_since_last_visit(&candidate.path) {
        let recency_score = 1.0 / (1.0 + seconds_ago as f32);
        score += recency_score * weights.recency;
    }

    // Session boost
    if session_stack.contains(&candidate.path) {
        score += weights.session_stack;
    }

    score
}
