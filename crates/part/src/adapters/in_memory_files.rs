use std::cell::RefCell;
use std::collections::{BTreeSet, HashMap};
use std::path::{Path, PathBuf};

use crate::ports::{Entry, FileError, Files, Folders};

/// A filesystem that never leaves the process, for tests that are about a part
/// rather than about a disk.
///
/// Folders are the ancestors of whatever has been written, plus the ones
/// `create` was asked for: an empty folder has to be able to exist, or the
/// screen that makes one has nothing to show afterwards.
#[derive(Default)]
pub struct InMemoryFiles {
    files: RefCell<HashMap<PathBuf, Vec<u8>>>,
    folders: RefCell<BTreeSet<PathBuf>>,
}

impl Files for InMemoryFiles {
    fn read(&self, path: &Path) -> Result<Vec<u8>, FileError> {
        self.files
            .borrow()
            .get(path)
            .cloned()
            .ok_or_else(|| FileError::Absent(path.to_path_buf()))
    }

    fn write(&self, path: &Path, bytes: &[u8]) -> Result<(), FileError> {
        self.files
            .borrow_mut()
            .insert(path.to_path_buf(), bytes.to_vec());
        let mut folders = self.folders.borrow_mut();
        let mut parent = path.parent();
        while let Some(folder) = parent {
            folders.insert(folder.to_path_buf());
            parent = folder.parent();
        }
        Ok(())
    }

    fn exists(&self, path: &Path) -> bool {
        self.files.borrow().contains_key(path) || self.folders.borrow().contains(path)
    }
}

impl Folders for InMemoryFiles {
    fn entries(&self, folder: &Path) -> Result<Vec<Entry>, FileError> {
        let mut entries: BTreeSet<Entry> = BTreeSet::new();
        for path in self.files.borrow().keys() {
            if path.parent() == Some(folder) {
                entries.insert(Entry {
                    path: path.clone(),
                    folder: false,
                });
            }
        }
        for path in self.folders.borrow().iter() {
            if path.parent() == Some(folder) {
                entries.insert(Entry {
                    path: path.clone(),
                    folder: true,
                });
            }
        }
        Ok(entries.into_iter().collect())
    }

    fn create(&self, folder: &Path) -> Result<(), FileError> {
        let mut folders = self.folders.borrow_mut();
        let mut at = Some(folder);
        while let Some(path) = at {
            folders.insert(path.to_path_buf());
            at = path.parent();
        }
        Ok(())
    }

    fn rename(&self, from: &Path, to: &Path) -> Result<(), FileError> {
        if !self.exists(from) {
            return Err(FileError::Absent(from.to_path_buf()));
        }
        let moved: Vec<PathBuf> = self
            .files
            .borrow()
            .keys()
            .filter(|path| path.starts_with(from))
            .cloned()
            .collect();
        for path in moved {
            let bytes = self.files.borrow_mut().remove(&path).unwrap_or_default();
            self.files
                .borrow_mut()
                .insert(rebased(&path, from, to), bytes);
        }
        let folders: Vec<PathBuf> = self
            .folders
            .borrow()
            .iter()
            .filter(|path| path.starts_with(from))
            .cloned()
            .collect();
        for path in folders {
            self.folders.borrow_mut().remove(&path);
            self.folders.borrow_mut().insert(rebased(&path, from, to));
        }
        Ok(())
    }

    fn discard(&self, path: &Path) -> Result<(), FileError> {
        if !self.exists(path) {
            return Err(FileError::Absent(path.to_path_buf()));
        }
        self.files
            .borrow_mut()
            .retain(|at, _| !at.starts_with(path));
        self.folders.borrow_mut().retain(|at| !at.starts_with(path));
        Ok(())
    }
}

fn rebased(path: &Path, from: &Path, to: &Path) -> PathBuf {
    match path.strip_prefix(from) {
        Ok(rest) => to.join(rest),
        Err(_) => path.to_path_buf(),
    }
}

#[cfg(test)]
mod tests;
