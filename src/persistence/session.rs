use serde::{Deserialize, Serialize};
use std::{
    path::{Path, PathBuf},
    time::{SystemTime, UNIX_EPOCH},
};

use crate::{paths, persistence::persistable::Persistable};

#[derive(Debug, Serialize, Deserialize)]
pub struct SessionEntry {
    pub path: PathBuf,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct SessionStack {
    max_size: usize,
    entries: Vec<SessionEntry>,
    saved_at: u64,
}

const SESSION_EXPIRY_SECS: u64 = 86400; // 1 day
const MAX_SIZE: usize = 10;

impl SessionStack {
    /// Push a directory onto the stack.
    pub fn push<P: AsRef<Path>>(&mut self, path: P) {
        let path = paths::normalize(path);

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

    /// Human-readable stack (for `cdd stack`).
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

    /// Load the existing session stack or create and return a new one if the old one is expired or it doesn't exist yet.
    pub fn load_or_new() -> Self {
        if let Ok(session) = Self::load() {
            // Expire sessions older than SESSION_EXPIRY_SECS; saturating_sub guards against clock skew.
            if time_now().saturating_sub(session.saved_at) < SESSION_EXPIRY_SECS {
                return session;
            }
        }

        let new_session = Self {
            max_size: MAX_SIZE,
            entries: Vec::new(),
            saved_at: time_now(),
        };
        // TODO: maybe handle potential errors
        let _ = Self::save(&new_session);

        new_session
    }
}

fn time_now() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}

impl Persistable for SessionStack {
    const FILE: &'static str = "session.json";
}
