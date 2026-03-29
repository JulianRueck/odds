use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    path::PathBuf,
    time::{SystemTime, UNIX_EPOCH},
};

use crate::{paths, persistence::persistable::Persistable};

#[derive(Debug, Serialize, Deserialize)]
pub struct SessionEntry {
    pub path: PathBuf,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Session {
    max_size: usize,
    entries: Vec<SessionEntry>,
    saved_at: u64,
    chain: HashMap<String, HashMap<String, usize>>,
}

const SESSION_EXPIRY_SECS: u64 = 43200; // 12 hours
const MAX_SIZE: usize = 10;

impl Default for Session {
    fn default() -> Self {
        Self {
            max_size: MAX_SIZE,
            entries: Vec::new(),
            saved_at: time_now(),
            chain: HashMap::new(),
        }
    }
}

impl Session {
    /// Push a directory onto the session stack.
    pub fn push(&mut self, path: &PathBuf) {
        let path = paths::normalize(path);

        self.register_markov_chain(&path);

        // If already current do nothing
        if self.entries.first().map(|e| &e.path) == Some(&path) {
            return;
        }

        // Remove existing occurrences
        self.entries.retain(|e| e.path != path);

        // Insert at top
        self.entries.insert(0, SessionEntry { path });

        // Enforce max size
        if self.entries.len() > self.max_size {
            self.entries.truncate(self.max_size);
        }
    }

    /// Get current directory.
    pub fn current(&self) -> Option<&PathBuf> {
        self.entries.first().map(|e| &e.path)
    }

    /// List all directories (most recent first).
    pub fn list(&self) -> &[SessionEntry] {
        &self.entries
    }

    pub fn contains(&self, path: &PathBuf) -> bool {
        self.entries.iter().any(|e| e.path == *path)
    }

    /// Human-readable session (for `o session`).
    pub fn formatted(&self) -> Vec<String> {
        self.entries
            .iter()
            .enumerate()
            .map(|(i, e)| {
                if i == 0 {
                    format!("{} {} <-- current", i + 1, e.path.display())
                } else {
                    format!("{} {}", i + 1, e.path.display())
                }
            })
            .collect()
    }

    /// Load the existing session or create and return a new one if the old one is expired or it doesn't exist yet.
    pub fn load_or_new() -> Self {
        if let Ok(session) = Self::load() {
            // Expire sessions older than SESSION_EXPIRY_SECS; saturating_sub guards against clock skew.
            if time_now().saturating_sub(session.saved_at) < SESSION_EXPIRY_SECS {
                return session;
            }
        }

        let mut new_session = Self::default();

        if let Err(e) = new_session.save() {
            eprintln!("Error saving session: {e}");
        }

        new_session
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

    fn register_markov_chain(&mut self, path: &PathBuf) {
        if let Some(current_path) = self.current() {
            let to_str = path.to_str().expect("Invalid UTF-8 in path.");
            let from_str = current_path
                .to_str()
                .expect("Invalid UTF-8 in current path.");

            if to_str != from_str {
                let dest_map = self.chain.entry(from_str.to_string()).or_default();
                let count = dest_map.entry(to_str.to_string()).or_insert(0);

                *count += 1;
            }
        }
    }
}

fn time_now() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}

impl Persistable for Session {
    const FILE: &'static str = "session.json";

    fn before_save(&mut self) {
        self.saved_at = time_now();
    }
}
