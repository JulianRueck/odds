use std::{collections::HashMap, fs, path::PathBuf};

#[derive(Debug)]
pub struct FsCache {
    dirs: HashMap<PathBuf, Vec<PathBuf>>,
}

impl FsCache {
    pub fn new() -> Self {
        Self { dirs: HashMap::new() }
    }

    pub fn list_dirs(&mut self, dir: &PathBuf) -> Vec<PathBuf> {
        if let Some(chached) = self.dirs.get(dir) {
            return chached.clone();
        }

        let mut results = Vec::new();

        if let Ok(entries) = fs::read_dir(dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_dir() {
                    results.push(path);
                }
            }
        }

        results
    }
}
