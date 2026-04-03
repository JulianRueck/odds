use serde::{Deserialize, Serialize};
use std::{
    path::PathBuf,
    time::{SystemTime, UNIX_EPOCH},
};

use crate::{
    discovery::{DiscoveryCandidate, matcher::match_candidate},
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
        let token_l = token.to_lowercase();

        self.entries
            .iter()
            .filter_map(|entry| {
                let name_l = entry.path.file_name()?.to_str()?.to_lowercase();

                match_candidate(&entry.path, &name_l, &token_l)
            })
            .collect()
    }

    pub fn load_or_new() -> Self {
        if let Ok(history) = Self::load() {
            return history;
        }

        let mut new_history = Self::default();

        if let Err(e) = new_history.save() {
            eprintln!("Error saving history: {e}")
        }

        new_history
    }

    pub fn visit_count(&self, path: &PathBuf) -> u64 {
        self.entries
            .iter()
            .find(|e| e.path == *path)
            .map(|e| e.visits)
            .unwrap_or(0)
    }

    pub fn seconds_since_last_visit_at(&self, path: &PathBuf, now: u64) -> Option<u64> {
        self.entries
            .iter()
            .find(|e| e.path == *path)
            .map(|e| now.saturating_sub(e.last_visited))
    }

    pub fn seconds_since_last_visit(&self, path: &PathBuf) -> Option<u64> {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        self.seconds_since_last_visit_at(path, now)
    }
}

impl Persistable for History {
    const FILE: &'static str = "history.json";
}
