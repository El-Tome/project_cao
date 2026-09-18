use std::path::PathBuf;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::locations::Locations;
use crate::ports::Files;
use crate::storage::StorageError;

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
    fn state_path(at: &Locations) -> PathBuf {
        at.config.join("recents.json")
    }

    pub fn load(files: &impl Files, at: &Locations) -> Result<Self, StorageError> {
        let path = Self::state_path(at);
        if !files.exists(&path) {
            return Ok(Self::default());
        }
        Ok(serde_json::from_slice(&files.read(&path)?)?)
    }

    pub fn save(&self, files: &impl Files, at: &Locations) -> Result<(), StorageError> {
        let json = serde_json::to_string_pretty(self)?;
        Ok(files.write(&Self::state_path(at), json.as_bytes())?)
    }

    pub fn entries(&self) -> &[RecentEntry] {
        &self.entries
    }

    /// Records `path` as just opened, moving it to the front and dropping
    /// anything past `MAX_RECENTS`.
    pub fn push(&mut self, path: impl Into<PathBuf>, name: impl Into<String>, now: DateTime<Utc>) {
        let path = path.into();
        self.entries.retain(|e| e.path != path);
        self.entries.insert(
            0,
            RecentEntry {
                path,
                name: name.into(),
                opened_at: now,
            },
        );
        self.entries.truncate(MAX_RECENTS);
    }

    /// Drops entries whose file is no longer where it was opened from.
    pub fn prune_missing(&mut self, files: &impl Files) {
        self.entries.retain(|e| files.exists(&e.path));
    }
}

#[cfg(test)]
mod tests;
