use std::{fs, io};
use crate::paths;

pub trait Persistable {
    fn save(&self, file: &str) -> io::Result<()> {
        let path = paths::persistence_path(file);

        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }

        fs::write(path, serde_json::to_string_pretty(self)?)
    }
}
