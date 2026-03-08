use std::{
    collections::{HashSet, VecDeque},
    path::PathBuf,
};

use crate::discovery::matcher::match_candidate;

use super::DiscoveryCandidate;
use super::Matchkind;
use super::cache;

/// Does a BFS to discover novel paths i.e. not previously visited, scored by match kind and fuzzy.
pub fn bfs_discover(
    roots: &[PathBuf],
    token: &str,
    max_depth: usize,
    max_results: usize,
    cache: &mut cache::FsCache,
) -> Vec<DiscoveryCandidate> {
    let mut strong_results = Vec::new();
    let mut fuzzy_results = Vec::new();

    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();

    for root in roots {
        if visited.insert(root.clone()) {
            queue.push_back((root.clone(), 0));
        }
    }

    while let Some((dir, depth)) = queue.pop_front() {
        if depth > max_depth {
            continue;
        }
        for path in cache.list_dirs(&dir) {
            // Skip duplicates
            if !visited.insert(path.clone()) {
                continue;
            }

            // Extract directory name
            let Some(name) = path.file_name().and_then(|n| n.to_str()) else {
                continue;
            };

            let name_l = name.to_lowercase();
            let token_l = token.to_lowercase();

            if let Some(candidate) = match_candidate(&path, &name_l, &token_l) {
                // 1. If it's a Strong match (Exact, Prefix, Substring), always keep it.
                if candidate.match_kind != Matchkind::Fuzzy {
                    strong_results.push(candidate);
                // 2. If it's Fuzzy, only keep it if there's still room.
                } else if strong_results.len() < max_results {
                    fuzzy_results.push(candidate);
                }
            }

            queue.push_back((path, depth + 1));
        }
    }

    strong_results.extend(fuzzy_results);

    strong_results.truncate(max_results);

    strong_results
}
