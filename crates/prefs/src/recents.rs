use std::fs;
use std::io::Write;
use std::path::PathBuf;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::storage::{StorageError, project_dirs, replace_whole};

/// How many recent parts we remember and show in the start menu.
pub const MAX_RECENTS: usize = 10;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecentEntry {
    pub path: PathBuf,
    pub name: String,
    pub opened_at: DateTime<Utc>,
}

/// Most-recently-opened-first list of parts, persisted next to the app's
/// other config so it survives restarts. Independent of where the parts
/// themselves are stored.
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct RecentList {
    entries: Vec<RecentEntry>,
}

impl RecentList {
    fn state_path() -> Result<PathBuf, StorageError> {
        Ok(project_dirs()?.config_dir().join("recents.json"))
    }

    pub fn load() -> Result<Self, StorageError> {
        let path = Self::state_path()?;
        if !path.exists() {
            return Ok(Self::default());
        }
        let json = fs::read_to_string(path)?;
        Ok(serde_json::from_str(&json)?)
    }

    pub fn save(&self) -> Result<(), StorageError> {
        let json = serde_json::to_string_pretty(self)?;
        replace_whole(&Self::state_path()?, |file| file.write_all(json.as_bytes()))?;
        Ok(())
    }

    pub fn entries(&self) -> &[RecentEntry] {
        &self.entries
    }

    /// Records `path` as just opened, moving it to the front and dropping
    /// anything past `MAX_RECENTS`.
    pub fn push(&mut self, path: impl Into<PathBuf>, name: impl Into<String>) {
        let path = path.into();
        self.entries.retain(|e| e.path != path);
        self.entries.insert(
            0,
            RecentEntry {
                path,
                name: name.into(),
                opened_at: Utc::now(),
            },
        );
        self.entries.truncate(MAX_RECENTS);
    }

    /// Drops entries whose file no longer exists on disk.
    pub fn prune_missing(&mut self) {
        self.entries.retain(|e| e.path.exists());
    }
}
