use std::collections::HashMap;

use crate::persistence::history::HistoryEntry;

use super::*;

#[test]
fn decay_math_exact() {
    let path = PathBuf::from("/a/b");
    let now = 1_000_000u64;
    let history = History {
        entries: vec![HistoryEntry {
            path: path.clone(),
            visits: 1,
            last_visited: now - 3600, // exactly 1 hour ago
        }],
        chain: HashMap::new()
    };

    let score = calculate_frecency_score_at(&path, &history, now);
    let expected = 1.0 * (-LAMBDA * 3600.0_f32).exp() * FRECENCY_WEIGHT;

    assert!((score - expected) < 0.001);
}

#[test]
fn higher_visit_count_scores_higher() {
    let path = PathBuf::from("/a/b");
    let now = 1_000_000u64;

    let low_visits = History {
        entries: vec![HistoryEntry {
            path: path.clone(),
            visits: 1,
            last_visited: now,
        }],
        chain: HashMap::new(),
    };
    let high_visits = History {
        entries: vec![HistoryEntry {
            path: path.clone(),
            visits: 10,
            last_visited: now,
        }],
        chain: HashMap::new(),
    };

    assert!(
        calculate_frecency_score_at(&path, &high_visits, now)
            > calculate_frecency_score_at(&path, &low_visits, now)
    );
}

#[test]
fn older_visit_scores_lower() {
    let path = PathBuf::from("/a/b");
    let now = 1_000_000u64;

    let recent = History {
        entries: vec![HistoryEntry {
            path: path.clone(),
            visits: 1,
            last_visited: now - 60,
        }],
        chain: HashMap::new(),
    };
    let old = History {
        entries: vec![HistoryEntry {
            path: path.clone(),
            visits: 1,
            last_visited: now - 86400,
        }],
        chain: HashMap::new(),
    };

    assert!(
        calculate_frecency_score_at(&path, &recent, now)
            > calculate_frecency_score_at(&path, &old, now)
    );
}
