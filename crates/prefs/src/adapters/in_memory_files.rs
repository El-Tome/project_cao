use std::cell::RefCell;
use std::collections::HashMap;
use std::path::{Path, PathBuf};

use crate::ports::{FileError, Files};

/// A filesystem that never leaves the process, for tests that are about what
/// the installation remembers rather than about a disk.
#[derive(Default)]
pub struct InMemoryFiles(RefCell<HashMap<PathBuf, Vec<u8>>>);

impl Files for InMemoryFiles {
    fn read(&self, path: &Path) -> Result<Vec<u8>, FileError> {
        self.0
            .borrow()
            .get(path)
            .cloned()
            .ok_or_else(|| FileError::Absent(path.to_path_buf()))
    }

    fn write(&self, path: &Path, bytes: &[u8]) -> Result<(), FileError> {
        self.0
            .borrow_mut()
            .insert(path.to_path_buf(), bytes.to_vec());
        Ok(())
    }

    fn exists(&self, path: &Path) -> bool {
        self.0.borrow().contains_key(path)
    }
}

#[cfg(test)]
mod tests;
