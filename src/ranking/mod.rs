use std::ops::Deref;

use crate::discovery::DiscoveryCandidate;

pub mod ranker;

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
