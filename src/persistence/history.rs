use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap, path::PathBuf, time::{SystemTime, UNIX_EPOCH}
};

use crate::{
    discovery::{DiscoveryCandidate, matcher::match_candidate_multi},
    persistence::persistable::Persistable,
};

#[derive(Debug, Serialize, Deserialize)]
pub struct HistoryEntry {
    pub path: PathBuf,
    pub visits: usize,
    pub last_visited: u64,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct History {
    pub entries: Vec<HistoryEntry>,
    pub chain: HashMap<String, HashMap<String, usize>>,
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
    pub fn history_candidates(&self, tokens: &[&str]) -> Vec<DiscoveryCandidate> {
        self.entries
            .iter()
            .filter_map(|entry| {
                match_candidate_multi(&entry.path, tokens)
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

    pub fn visit_count(&self, path: &PathBuf) -> usize {
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

    /// Calculate the probability of visiting the target from the source.
    pub fn calculate_probability_from(&self, to: &str, from: &str) -> f32 {
        if let Some(dest_map) = self.chain.get(from) {
            let count = *dest_map.get(to).unwrap_or(&0) as f32;
            let total: usize = dest_map.values().sum();

            if total > 0 {
                return count / total as f32;
            }
        }
        0.0
    }

    /// Register the amount of transitions from one path to another.
    pub fn register_markov_chain(&mut self, from: &PathBuf, to: &PathBuf) {
        if !from.is_absolute() || !to.is_absolute() {
            return;
        }
        
        let from_str = from.to_str().expect("Invalid UTF-8 in current path.");
        let to_str = to.to_str().expect("Invalid UTF-8 in path.");

        if from_str != to_str {
            let dest_map = self.chain.entry(from_str.to_string()).or_default();
            let count = dest_map.entry(to_str.to_string()).or_insert(0);

            *count += 1;
        }
    }

    pub fn transition_count(&self) -> usize {
        self.chain.values().map(|dest_map| dest_map.len()).sum()
    }
}

impl Persistable for History {
    const FILE: &'static str = "history.json";
}
