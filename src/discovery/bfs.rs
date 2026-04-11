use std::{
    collections::{HashSet, VecDeque},
    path::PathBuf,
};

use crate::discovery::matcher::{SUBSTRING_SCORE, match_candidate_multi};

use super::DiscoveryCandidate;
use super::cache;

/// Does a BFS to discover novel paths i.e. not previously visited.
pub fn bfs_discover(
    roots: &[PathBuf],
    tokens: &[&str],
    max_depth: usize,
    max_results: usize,
    cache: &mut cache::FsCache,
) -> Vec<DiscoveryCandidate> {
    let mut candidates = Vec::new();
    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();

    for root in roots {
        if visited.insert(root.clone()) {
            queue.push_back(root.clone());
        }
    }

     for depth in 1..=max_depth {
        let mut next_queue = VecDeque::new();

        while let Some(dir) = queue.pop_front() {
            for path in cache.list_dirs(&dir) {
                if !visited.insert(path.clone()) {
                    continue;
                }

                if let Some(candidate) = match_candidate_multi(&path, tokens) {
                    candidates.push(candidate);
                }

                if depth < max_depth {
                    next_queue.push_back(path);
                }
            }
        }

        queue = next_queue;

        // Prioritise strong matches i.e., exact, prefix and substring.
        let strong_match_count = candidates
            .iter()
            .filter(|c| c.score >= SUBSTRING_SCORE)
            .count();

        if strong_match_count >= max_results {
            break;
        }
    }

    // Sort by score descending.
    candidates.sort_by(|a, b| b.score.total_cmp(&a.score));
    candidates.truncate(max_results);

    candidates
}
