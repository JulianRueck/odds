use std::{path::PathBuf};

use crate::ranking::RankedCandidate;

use super::picker;

pub fn do_jump(dir: &PathBuf) {
    println!("{}", dir.display());
}

pub fn pick_and_jump(candidates: &[RankedCandidate]) {
    // Ordering on match score instead of ranked score,
    // This makes more sense if there's not enough confidence in ranked score to auto jump.
    let mut picker_candidates = candidates.to_vec();
    picker_candidates.sort_by(|a, b| {
        b.candidate
            .score
            .total_cmp(&a.candidate.score)
            .then_with(|| b.ranked_score.total_cmp(&a.ranked_score))
    });

    if let Some(picked) = picker::pick_directory(&picker_candidates) {
        do_jump(&picked.path);
    } else {
        eprintln!("No directory selected.");
    }
}

#[cfg(test)]
#[path = "navigator_tests.rs"]
mod navigator_tests;
