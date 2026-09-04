use std::path::PathBuf;

use directories::{ProjectDirs, UserDirs};

#[derive(Debug, thiserror::Error)]
pub enum StorageError {
    #[error("could not resolve a config/data directory on this platform")]
    NoProjectDirs,
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    Json(#[from] serde_json::Error),
}

pub fn project_dirs() -> Result<ProjectDirs, StorageError> {
    ProjectDirs::from("dev", "cao", "cao").ok_or(StorageError::NoProjectDirs)
}

/// Where new parts land by default: `<Documents>/CAO` if a documents folder
/// exists on this platform, otherwise the app's own data directory.
pub fn default_projects_dir() -> Result<PathBuf, StorageError> {
    if let Some(user_dirs) = UserDirs::new()
        && let Some(docs) = user_dirs.document_dir()
    {
        return Ok(docs.join("CAO"));
    }
    Ok(project_dirs()?.data_dir().join("projects"))
}
