use std::{path::{Path, PathBuf}};
use crate::{paths};

#[derive(Debug)]
pub struct SessionEntry {
    pub path: PathBuf,
}

#[derive(Debug)]
pub struct SessionStack {
    max_size: usize,
    entries: Vec<SessionEntry>
}

impl SessionStack {
    /// Create a new empty session stack.
    pub fn new(max_size: usize) -> Self {
        Self { 
            max_size, 
            entries:Vec::new() 
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
}
