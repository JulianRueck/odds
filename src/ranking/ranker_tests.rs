use crate::persistence::{history::HistoryEntry, markov::Markov};

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
        chain: Markov::default(),
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
        chain: Markov::default(),
    };
    let high_visits = History {
        entries: vec![HistoryEntry {
            path: path.clone(),
            visits: 10,
            last_visited: now,
        }],
        chain: Markov::default(),
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
        chain: Markov::default(),
    };
    let old = History {
        entries: vec![HistoryEntry {
            path: path.clone(),
            visits: 1,
            last_visited: now - 86400,
        }],
        chain: Markov::default(),
    };

    assert!(
        calculate_frecency_score_at(&path, &recent, now)
            > calculate_frecency_score_at(&path, &old, now)
    );
}
