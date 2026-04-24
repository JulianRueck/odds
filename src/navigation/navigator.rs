use std::path::PathBuf;

use crate::ranking::RankedCandidate;

use super::picker;

pub fn do_jump(dir: &PathBuf) {
    println!("{}", dir.display());
}

pub fn pick_and_jump(candidates: &[RankedCandidate]) {
    if let Some(picked) = picker::pick_directory(candidates) {
        do_jump(&picked.path);
    } else {
        eprintln!("No directory selected.");
    }
}

#[cfg(test)]
#[path = "navigator_tests.rs"]
mod navigator_tests;
