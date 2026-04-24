use std::path::PathBuf;

use crate::discovery::bfs::bfs_discover;
use crate::discovery::cache::FsCache;
use crate::paths;

mod bfs;
mod cache;

pub mod matcher;

#[derive(Debug, Clone)]
pub struct DiscoveryCandidate {
    pub path: PathBuf,
    pub score: f32,
}

impl Default for DiscoveryCandidate {
    fn default() -> Self {
        Self {
            path: PathBuf::new(),
            score: 0.0,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Matchkind {
    Exact,
    Prefix,
    Substring,
    Fuzzy,
}

pub fn discover(tokens: &[&str], max_depth: usize, max_results: usize) -> Vec<DiscoveryCandidate> {
    let roots = paths::search_roots();
    let mut cache = FsCache::new();
    
    bfs_discover(&roots, tokens, max_depth, max_results, &mut cache)
}
