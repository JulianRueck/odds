use std::path::PathBuf;

use crate::discovery::bfs::bfs_discover;
use crate::paths;

pub mod cache;

mod bfs;
mod matcher;

#[derive(Debug)]
pub struct DiscoveryCandidate {
    pub path: PathBuf,
    pub match_kind: Matchkind,
    pub score: f32,
}

impl Default for DiscoveryCandidate {
    fn default() -> Self {
        Self {
            path: PathBuf::new(),
            match_kind: Matchkind::Exact,
            score: 0.0,
        }
    }
}

#[derive(Debug)]
pub enum Matchkind {
    Exact,
    Prefix,
    Substring,
    Fuzzy,
}

pub fn discover(token: &str, max_depth: usize, max_results: usize) -> Vec<DiscoveryCandidate> {
    let mut cache: cache::FsCache = cache::FsCache::new();

    let roots = paths::search_roots();

    bfs_discover(&roots, token, max_depth, max_results, &mut cache)
}
