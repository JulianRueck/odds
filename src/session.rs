use serde::{Deserialize, Serialize};
use std::{
    fs,
    io,
    path::{Path, PathBuf},
    time::{SystemTime, UNIX_EPOCH},
};

use crate::paths;

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
const SESSION_FILE: &str = "session.json";
const SESSION_EXPIRY_SECS: u64 = 86400; // 1 day

impl SessionStack {
    /// Create a new empty session stack.
    pub fn new(max_size: usize) -> Self {
        Self {
            max_size,
            entries: Vec::new(),
            saved_at: time_now(),
        }
    }

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

    /// Load the session stack. TODO: Same as save
    pub fn load() -> io::Result<Self> {
        let path = paths::persistence_path(SESSION_FILE);

        let data = fs::read_to_string(path)?;
        let session: Self = serde_json::from_str(&data)?;

        // Expire sessions older than SESSION_EXPIRY_SECS; saturating_sub guards against clock skew.
        if time_now().saturating_sub(session.saved_at) > SESSION_EXPIRY_SECS {
            return Err(io::Error::new(io::ErrorKind::Other, "Session expired."));
        }

        Ok(session)
    }

    /// TODO: Create super persitence 'class' for this and history
    pub fn save(&self) -> io::Result<()> {
        let path = paths::persistence_path(SESSION_FILE);

        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }

        fs::write(path, serde_json::to_string_pretty(self)?)
    }
}

fn time_now() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}
