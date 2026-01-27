use serde::{Deserialize, Serialize};
use std::{
    fs, io,
    path::PathBuf,
    time::{SystemTime, UNIX_EPOCH},
};

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct HistoryEntry {
    pub path: PathBuf,
    pub visits: u64,
    pub last_visited: u64,
    pub total_time: u64, // Time spent in dir optional4now
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct History {
    pub entries: Vec<HistoryEntry>,
}

const HISTORY_PATH: &str = ".local/share/cdd/history.json";

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
                total_time: 0,
            });
        }
    }

    /// Load the history.
    pub fn load() -> io::Result<Self> {
        let path = history_file();

        if !path.exists() {
            return Ok(Self::default());
        }

        let data = fs::read_to_string(path)?;
        let history = serde_json::from_str(&data)?;
        Ok(history)
    }

    /// Persits state into history.
    pub fn save(&self) -> io::Result<()> {
        let path = history_file();

        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }

        let data = serde_json::to_string_pretty(self)?;
        fs::write(path, data)?;
        Ok(())
    }
}

fn history_file() -> PathBuf {
    let home = std::env::var("HOME").unwrap_or_else((|_| ".".into()));
    PathBuf::from(home).join(HISTORY_PATH)
}
