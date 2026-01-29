use std::path::PathBuf;

use crate::discovery::bfs::bfs_discover;
use crate::paths;

mod bfs;
mod cache;
mod matcher;

#[derive(Debug)]
pub struct DiscoveryCandidate {
    pub path: PathBuf,
    pub match_kind: Matchkind,
    pub score: f32,
}

#[derive(Debug)]
pub enum Matchkind {
    Exact,
    Prefix,
    Substring,
    Fuzzy,
}

pub fn discover(token: &str, max_depth: usize, max_results: usize) -> Vec<DiscoveryCandidate> {
    // TODO: Init FsCache

    let roots = paths::search_roots();

    bfs_discover(&roots, token, max_depth, max_results)
}
