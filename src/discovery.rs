use std::{
    collections::{HashSet, VecDeque},
    fs,
    path::PathBuf,
};

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

/// Does a BFS to discover novel paths i.e. not previously visited, scored by match kind and fuzzy.
pub fn discover(
    roots: &[PathBuf],
    token: &str,
    max_depth: usize,
    max_results: usize,
) -> Vec<DiscoveryCandidate> {
    let mut results = Vec::new();
    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();

    for root in roots {
        if visited.insert(root.clone()) {
            queue.push_back((root.clone(), 0));
        }
    }

    while let Some((dir, depth)) = queue.pop_front() {
        // TODO: Revisted this multiplication (fuzzy stuff)
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

            if let Some((match_kind, score)) = matches_token(name, token) {
                results.push(DiscoveryCandidate {
                    path: path.clone(),
                    match_kind,
                    score,
                });
            };

            queue.push_back((path, depth + 1));
        }
    }

    // Sort by score descending and tie-break lexicographically
    results.sort_by(|a, b| {
        b.score
            .total_cmp(&a.score)
            .then_with(|| a.path.cmp(&b.path))
    });

    results
}

fn matches_token(name: &str, token: &str) -> Option<(Matchkind, f32)> {
    let name_l = name.to_lowercase();
    let token_l = token.to_lowercase();

    if name_l == token_l {
        println!("exact");
        Some((Matchkind::Exact, 100.0))
    } else if name_l.starts_with(&token_l) {
        println!("prefix");
        Some((Matchkind::Prefix, 70.0))
    } else if name_l.contains(&token_l) {
        println!("substr");
        Some((Matchkind::Substring, 50.0))
    } else if let Some(score) = fuzzy_match(&name_l, &token_l) {
        println!("{} | {} | {}", name_l, token_l, score);
        Some((Matchkind::Fuzzy, score.min(50.0)))
    } else {
        None
    }
}

fn fuzzy_match(candidate: &str, token: &str) -> Option<f32> {
    let mut score = 0.0;
    let mut last_match = None;
    let mut chars = token.chars();
    let mut current = chars.next()?;

    for (i, c) in candidate.chars().enumerate() {
        if c == current {
            score += 10.0;

            if let Some(prev) = last_match {
                if i == prev + 1 {
                    score += 15.0;
                } else {
                    score -= (i - prev - 1) as f32;
                }
            } else {
                score -= i as f32 * 0.5;
            }

            last_match = Some(i);

            if let Some(next) = chars.next() {
                current = next;
            } else {
                // Ensure there's always some positive score using max().
                return Some(score.max(1.0));
            }
        }
    }

    None
}
