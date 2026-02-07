use crate::{
    discovery::{DiscoveryCandidate, Matchkind},
    history::{History},
    session::SessionStack,
};

/// Determines how much to 'trust' certain signals.
/// Some of these values might seem duplicates (matchers.rs).
/// However, one should consider those signal strength.
/// Here signals are modulated.
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

pub fn rank_candidates(
    mut candidates: Vec<DiscoveryCandidate>,
    history: &History,
    session_stack: &SessionStack,
    weights: &MlWeights,
    max_results: usize,
) -> Vec<DiscoveryCandidate> {
    candidates.sort_by(|a, b| {
        let sa = score_candidate(a, history, session_stack, weights);
        let sb = score_candidate(b, history, session_stack, weights);

        // Sort by score descending and tie-break lexicographically.
        sb.total_cmp(&sa).then_with(|| a.path.cmp(&b.path))
    });

    candidates.truncate(max_results);

    candidates
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

    // History frequency.
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
