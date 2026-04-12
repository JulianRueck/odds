use std::path::PathBuf;

use crate::ranking::RankedCandidate;

use super::picker;

pub fn do_jump(dir: &PathBuf) {
    println!("{}", dir.display());
}

pub fn pick_and_jump(candidates: &[RankedCandidate]) {
    // No need for picker if there's only one result.
    if candidates.len() == 1 {
        if let Some(candidate) = candidates.first() {
            do_jump(&candidate.path);
        };
    } else if let Some(picked) = picker::pick_directory(candidates) {
        do_jump(&picked.path);
    } else {
        eprintln!("No directory selected.");
    }
}

#[cfg(test)]
#[path = "navigator_tests.rs"]
mod navigator_tests;
