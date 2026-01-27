use std::{
    collections::{HashSet, VecDeque},
    fs,
    path::PathBuf,
};

/// Does a BFS to discover novel paths i.e. not previously visited.
pub fn discover(
    roots: &[PathBuf],
    token: &str,
    max_depth: usize,
    max_results: usize,
) -> Vec<PathBuf> {
    let mut results = Vec::new();
    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();

    for root in roots {
        if visited.insert(root.clone()) {
            queue.push_back((root.clone(), 0));
        }
    }

    while let Some((dir, depth)) = queue.pop_front() {
        if depth > max_depth || results.len() >= max_results {
            continue;
        }

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

            if matches_token(name, token) {
                results.push(path.clone());

                if results.len() >= max_results {
                    break;
                }
            }

            queue.push_back((path, depth + 1));
        }
    }

    results
}

fn matches_token(name: &str, token: &str) -> bool {
    let name = name.to_lowercase();
    let token = token.to_lowercase();

    name == token || name.starts_with(&token) || name.contains(&token)
}
