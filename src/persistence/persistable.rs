use anyhow::{Context, Ok};
use serde::{Serialize, de::DeserializeOwned};
use std::fs;

use crate::paths;

pub trait Persistable: Serialize + DeserializeOwned + Sized {
    const FILE: &'static str;

    fn before_save(&mut self) {}

    fn save(&mut self) -> anyhow::Result<()> {
        // A hook for session so it can update its timestamp
        self.before_save();

        let path = paths::persistence_path(Self::FILE);

        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)
                .with_context(|| format!("Failed to create directory structure: {:?}", parent))?;
        }

        let contents = serde_json::to_string_pretty(self)
            .with_context(|| "Failed to serialise data to JSON.")?;

        fs::write(&path, contents)
            .with_context(|| format!("Failed to write persistence file to {:?}", path))?;

        Ok(())
    }

    fn load() -> anyhow::Result<Self> {
        let path = paths::persistence_path(Self::FILE);

        let data = fs::read_to_string(&path)
            .with_context(|| format!("Failed to read file at {:?}", path))?;
        let result: Self = serde_json::from_str(&data)
            .with_context(|| "Failed to deserialise persistence JSON.")?;

        Ok(result)
    }
}
