/// What can go wrong reading or writing a `.caopart`.
///
/// A part archive fails for reasons a configuration file never meets — a
/// missing zip entry, a schema version — and the reverse holds too. One error
/// for both made each side carry cases it can never produce.
#[derive(Debug, thiserror::Error)]
pub enum PartFileError {
    #[error(transparent)]
    File(#[from] crate::ports::FileError),
    #[error(transparent)]
    Json(#[from] serde_json::Error),
    #[error(transparent)]
    Archive(#[from] zip::result::ZipError),
    #[error("part file has no entry named {0}")]
    MissingEntry(String),
    #[error("part written by an unsupported schema version (v{0})")]
    UnsupportedVersion(u32),
}
