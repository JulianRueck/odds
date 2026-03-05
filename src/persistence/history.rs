use serde::{Deserialize, Serialize};
use std::{
    path::PathBuf,
    time::{SystemTime, UNIX_EPOCH},
};

use crate::{
    discovery::{DiscoveryCandidate, Matchkind},
    persistence::persistable::Persistable,
};

#[derive(Debug, Serialize, Deserialize)]
pub struct HistoryEntry {
    pub path: PathBuf,
    pub visits: u64,
    pub last_visited: u64,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct History {
    pub entries: Vec<HistoryEntry>,
}

impl History {
    /// Record a visit in memory.
    pub fn record_visit(&mut self, path: &PathBuf) {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        if let Some(entry) = self.entries.iter_mut().find(|e| e.path == *path) {
            entry.visits += 1;
            entry.last_visited = now;
        } else {
            self.entries.push(HistoryEntry {
                path: path.clone(),
                visits: 1,
                last_visited: now,
            });
        }
    }

    /// Collect all candidate entries from history.
    pub fn history_candidates(&self, token: &str) -> Vec<DiscoveryCandidate> {
        self.entries
            .iter()
            .filter_map(|entry| {
                entry
                    .path
                    .file_name()
                    .and_then(|name| name.to_str())
                    .filter(|&name| name == token)
                    .map(|_| DiscoveryCandidate {
                        // TODO: revisit this use of DiscoveryCandidate
                        path: entry.path.clone(),
                        match_kind: Matchkind::Exact,
                        score: 0.00,
                    })
            })
            .collect()
    }

    pub fn visit_count(&self, path: &PathBuf) -> u64 {
        self.entries
            .iter()
            .find(|e| e.path == *path)
            .map(|e| e.visits)
            .unwrap_or(0)
    }

    pub fn seconds_since_last_visit(&self, path: &PathBuf) -> Option<u64> {
        self.entries.iter().find(|e| e.path == *path).map(|e| {
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs()
                - e.last_visited
        })
    }
}

impl Persistable for History {
    const FILE: &'static str = "history.json";
}
