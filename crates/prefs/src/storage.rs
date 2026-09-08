use crate::ports::FileError;

/// What can go wrong reading or writing what the installation remembers:
/// profiles, themes, shortcuts, the toolbar layout, the recent files.
///
/// Finding out where the platform keeps those is not here: that is a question
/// asked once, by the shell, and answered by a `Locations`.
#[derive(Debug, thiserror::Error)]
pub enum StorageError {
    #[error(transparent)]
    File(#[from] FileError),
    #[error(transparent)]
    Json(#[from] serde_json::Error),
    #[error("profile written by an unsupported settings version (v{0})")]
    UnsupportedVersion(u32),
}
