use std::{collections::HashMap, fs, path::PathBuf};

use crate::paths;

#[derive(Debug)]
pub struct FsCache {
    dirs: HashMap<PathBuf, Vec<PathBuf>>,
}

impl FsCache {
    pub fn new() -> Self {
        Self {
            dirs: HashMap::new(),
        }
    }

    /// Returns directories contained within specified directory.
    /// Either from cache if seen before,
    /// or from the file system if it's the first time.
    pub fn list_dirs(&mut self, dir: &PathBuf) -> Vec<PathBuf> {
        let normalized_dir = paths::normalize(dir);

        if let Some(cached) = self.dirs.get(&normalized_dir) {
            return cached.clone();
        }

        let mut results = Vec::new();

        if let Ok(entries) = fs::read_dir(&normalized_dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_dir() {
                    results.push(path);
                }
            }
        }

        self.dirs.insert(normalized_dir.clone(), results.clone());

        results
    }
}
