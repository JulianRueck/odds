use serde::{Serialize, de::DeserializeOwned};
use std::{fs, io};

use crate::paths;

pub trait Persistable: Serialize + DeserializeOwned + Sized {
    const FILE: &'static str;

    fn save(&self) -> io::Result<()> {
        let path = paths::persistence_path(Self::FILE);

        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }

        fs::write(path, serde_json::to_string_pretty(self)?)
    }

    fn load() -> io::Result<Self> {
        let path = paths::persistence_path(Self::FILE);

        let data = fs::read_to_string(path)?;
        let implemenation: Self = serde_json::from_str(&data)?;

        Ok(implemenation)
    }
}
