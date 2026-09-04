use std::fs;
use std::path::{Path, PathBuf};

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::storage::StorageError;

/// File extension used for a CAO part document.
pub const PART_EXTENSION: &str = "caopart";

/// The minimal on-disk representation of a part. Today it only carries
/// metadata; the sketch/extrude feature tree lands here once the sketch
/// mode exists.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PartDocument {
    pub id: Uuid,
    pub name: String,
    pub created_at: DateTime<Utc>,
    pub modified_at: DateTime<Utc>,
}

impl PartDocument {
    pub fn new(name: impl Into<String>) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            name: name.into(),
            created_at: now,
            modified_at: now,
        }
    }

    /// Creates a new part and writes it to `dir/<name>.caopart`.
    pub fn create_in(dir: &Path, name: impl Into<String>) -> Result<(Self, PathBuf), StorageError> {
        fs::create_dir_all(dir)?;
        let doc = Self::new(name);
        let path = dir.join(format!("{}.{PART_EXTENSION}", sanitize_filename(&doc.name)));
        doc.save(&path)?;
        Ok((doc, path))
    }

    pub fn save(&self, path: &Path) -> Result<(), StorageError> {
        let json = serde_json::to_string_pretty(self)?;
        fs::write(path, json)?;
        Ok(())
    }

    pub fn load(path: &Path) -> Result<Self, StorageError> {
        let json = fs::read_to_string(path)?;
        Ok(serde_json::from_str(&json)?)
    }
}

fn sanitize_filename(name: &str) -> String {
    let cleaned: String = name
        .chars()
        .map(|c| if c.is_alphanumeric() || c == '-' || c == '_' || c == ' ' { c } else { '_' })
        .collect();
    let trimmed = cleaned.trim();
    if trimmed.is_empty() {
        "Sans titre".to_string()
    } else {
        trimmed.to_string()
    }
}
