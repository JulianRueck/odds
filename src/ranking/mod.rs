use std::ops::Deref;

use crate::discovery::DiscoveryCandidate;

pub mod ranker;

#[derive(Debug)]
pub struct RankedCandidate {
    pub candidate: DiscoveryCandidate,
    pub ranked_score: f32,
}

impl Default for RankedCandidate {
    fn default() -> Self {
        Self {
            candidate: DiscoveryCandidate::default(),
            ranked_score: 0.0,
        }
    }
}

impl Deref for RankedCandidate {
    type Target = DiscoveryCandidate;
    fn deref(&self) -> &Self::Target {
        &self.candidate
    }
}
