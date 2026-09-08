use std::path::{Path, PathBuf};

/// The whole of what a part needs from a filesystem.
pub trait Files {
    fn read(&self, path: &Path) -> Result<Vec<u8>, FileError>;

    /// Replaces the file at `path` in one step, making its parent folder if
    /// needed. Either the previous content stands or the new one does, never
    /// half of either.
    fn write(&self, path: &Path, bytes: &[u8]) -> Result<(), FileError>;

    fn exists(&self, path: &Path) -> bool;
}

#[derive(Debug, thiserror::Error)]
pub enum FileError {
    #[error("nothing at {0}")]
    Absent(PathBuf),
    #[error("the system refused {0}")]
    Refused(PathBuf),
    #[error("the exchange with {0} stopped partway")]
    Interrupted(PathBuf),
}
