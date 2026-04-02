use std::{ops::Deref, path::PathBuf};

use crate::{
    discovery::{DiscoveryCandidate, Matchkind},
    persistence::History,
    persistence::Session,
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
    pub frecency: f32,
    pub markov: f32,
    pub session: f32,
}

impl MlWeights {
    pub const EXACT: f32 = 100.0;
    pub const PREFIX: f32 = 70.0;
    pub const SUBSTRING: f32 = 50.0;
    pub const FUZZY: f32 = 20.0;
    pub const FRECENCY: f32 = 10.0;
    pub const MARKOV: f32 = 40.0;
    pub const SESSION: f32 = 12.0;

    pub const HALF_LIFE_DAYS: f32 = 3.0;
    /// ln(2) is approximately 0.69314718
    pub const LAMBDA: f32 = 0.69314718 / (Self::HALF_LIFE_DAYS * 24.0 * 60.0 * 60.0);
}

impl Default for MlWeights {
    fn default() -> Self {
        Self {
            exact: Self::EXACT,
            prefix: Self::PREFIX,
            substring: Self::SUBSTRING,
            fuzzy: Self::FUZZY,
            frecency: Self::FRECENCY,
            markov: Self::MARKOV,
            session: Self::SESSION,
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
    session: &Session,
    weights: &MlWeights,
    max_results: usize,
) -> Vec<RankedCandidate> {
    let current_path = session.current();

    let mut ranked: Vec<RankedCandidate> = candidates
        .into_iter()
        .map(|c| {
            let ml_score = score_candidate(&c, history, session, weights, current_path);
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
    session: &Session,
    weights: &MlWeights,
    current_path: Option<&PathBuf>,
) -> f32 {
    let mut score = match candidate.match_kind {
        Matchkind::Exact => weights.exact,
        Matchkind::Prefix => weights.prefix,
        Matchkind::Substring => weights.substring,
        Matchkind::Fuzzy => weights.fuzzy,
    };

    // Exponential Decay (Frecency).
    let frequency = history.visit_count(&candidate.path) as f32;

    if let Some(seconds_ago) = history.seconds_since_last_visit(&candidate.path) {
        // Frecency = Frequency * e^(-λ * t)
        let decay_factor = (-MlWeights::LAMBDA * seconds_ago as f32).exp();
        let frecency_score = frequency * decay_factor;

        score += frecency_score * weights.frecency;
    }

    // Markov Chain boost.
    if let Some(from) = current_path {
        if candidate.path != *from {
            let to_str = candidate.path.to_str().expect("Invalid UTF-8 in path.");
            let from_str = from.to_str().expect("Invalid UTF-8 in current path.");

            let prob = session.calculate_probability_from(to_str, from_str);

            score += prob * weights.markov;
        }
    }

    // Session boost.
    if session.contains(&candidate.path) {
        score += weights.session;
    }

    score
}
