use std::path::{Path, PathBuf};

use super::files::FileError;

/// What a folder holds, and the few ways its contents can be rearranged.
///
/// Apart from `Files`, which is about the bytes of one file: browsing a
/// library of parts asks nothing of a file's content, and a screen that only
/// lists what is there should not be handed the means to rewrite it.
pub trait Folders {
    /// What sits directly in `folder`, in no particular order. A folder
    /// nobody made yet is empty rather than an error: a library the user has
    /// not started is not a failure to report.
    fn entries(&self, folder: &Path) -> Result<Vec<Entry>, FileError>;

    fn create(&self, folder: &Path) -> Result<(), FileError>;

    fn rename(&self, from: &Path, to: &Path) -> Result<(), FileError>;

    /// Hands `path` to wherever the platform keeps what the user threw away,
    /// so that a part deleted by mistake can be fetched back.
    fn discard(&self, path: &Path) -> Result<(), FileError>;
}

/// One line of a folder's contents. Whether it is a folder is all the
/// browsing needs; a symbolic link is neither, and is left out.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Entry {
    pub path: PathBuf,
    pub folder: bool,
}
