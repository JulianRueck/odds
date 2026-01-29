use std::{
    collections::{HashSet, VecDeque},
    fs,
    path::PathBuf,
};

use super::DiscoveryCandidate;
use super::Matchkind;
use super::matcher::{fuzzy_match, strong_match};

/// Does a BFS to discover novel paths i.e. not previously visited, scored by match kind and fuzzy.
pub fn bfs_discover(
    roots: &[PathBuf],
    token: &str,
    max_depth: usize,
    max_results: usize,
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

    let token_l = token.to_lowercase();

    while let Some((dir, depth)) = queue.pop_front() {
        if depth > max_depth {
            continue;
        }
        // TODO: Add caching; read_dir is expensive dangit
        let entries = match fs::read_dir(&dir) {
            Ok(e) => e,
            Err(_) => continue,
        };

        for entry in entries.flatten() {
            let path = entry.path();

            if !path.is_dir() {
                continue;
            }

            // Skip duplicates
            if !visited.insert(path.clone()) {
                continue;
            }

            // Extract directory name
            let Some(name) = path.file_name().and_then(|n| n.to_str()) else {
                continue;
            };

            let name_l = name.to_lowercase();

            // Phase 1: Strong matches
            if let Some((match_kind, score)) = strong_match(&name_l, &token_l) {
                strong_results.push(DiscoveryCandidate {
                    path: path.clone(),
                    match_kind,
                    score,
                });

            // Phase 2: Fuzzy fallback
            } else if strong_results.len() < max_results {
                if let Some(score) = fuzzy_match(&name_l, &token_l) {
                    fuzzy_results.push(DiscoveryCandidate {
                        path: path.clone(),
                        match_kind: Matchkind::Fuzzy,
                        score: score.min(45.0),
                    });
                }
            }

            queue.push_back((path, depth + 1));
        }
    }

    sort_candidates(&mut strong_results);

    if strong_results.len() >= max_results {
        strong_results.truncate(max_results);

        return strong_results;
    }

    sort_candidates(&mut fuzzy_results);

    strong_results.extend(fuzzy_results);
    strong_results.truncate(max_results);

    strong_results
}

/// Sort by score descending and tie-break lexicographically.
fn sort_candidates(candidates: &mut [DiscoveryCandidate]) {
    candidates.sort_by(|a, b| {
        b.score
            .total_cmp(&a.score)
            .then_with(|| a.path.cmp(&b.path))
    });
}
