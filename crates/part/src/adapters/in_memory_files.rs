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
mod tests {
    use std::path::Path;

    use super::*;

    #[test]
    fn a_file_reads_back_exactly_what_was_written() {
        let files = InMemoryFiles::default();
        files
            .write(Path::new("/parts/support.caopart"), b"PK bytes")
            .expect("writes");

        assert_eq!(
            files
                .read(Path::new("/parts/support.caopart"))
                .expect("reads"),
            b"PK bytes"
        );
        assert!(files.exists(Path::new("/parts/support.caopart")));
    }

    #[test]
    fn a_second_write_replaces_the_first_whole() {
        let files = InMemoryFiles::default();
        let path = Path::new("/parts/support.caopart");
        files.write(path, b"the first one").expect("writes");
        files.write(path, b"the second").expect("writes again");

        assert_eq!(files.read(path).expect("reads"), b"the second");
    }

    #[test]
    fn reading_what_was_never_written_says_so() {
        let files = InMemoryFiles::default();
        let error = files
            .read(Path::new("/parts/absent.caopart"))
            .expect_err("nothing there");

        assert!(matches!(error, FileError::Absent(_)));
        assert!(!files.exists(Path::new("/parts/absent.caopart")));
    }

    #[test]
    fn a_written_file_makes_the_folders_above_it_exist() {
        let files = InMemoryFiles::default();
        files
            .write(Path::new("/parts/drafts/support.caopart"), b"PK")
            .expect("writes");

        assert_eq!(
            files.entries(Path::new("/parts")).expect("reads"),
            [Entry {
                path: "/parts/drafts".into(),
                folder: true,
            }],
        );
    }

    #[test]
    fn a_folder_nobody_has_put_anything_in_still_shows_up() {
        let files = InMemoryFiles::default();
        files.create(Path::new("/parts/empty")).expect("creates");

        assert_eq!(
            files.entries(Path::new("/parts")).expect("reads"),
            [Entry {
                path: "/parts/empty".into(),
                folder: true,
            }],
        );
    }

    #[test]
    fn renaming_a_folder_takes_what_it_held_with_it() {
        let files = InMemoryFiles::default();
        files
            .write(Path::new("/parts/drafts/support.caopart"), b"PK")
            .expect("writes");

        files
            .rename(Path::new("/parts/drafts"), Path::new("/parts/kept"))
            .expect("renames");

        assert!(files.exists(Path::new("/parts/kept/support.caopart")));
        assert!(!files.exists(Path::new("/parts/drafts")));
    }

    #[test]
    fn discarding_a_folder_takes_what_it_held_with_it() {
        let files = InMemoryFiles::default();
        files
            .write(Path::new("/parts/drafts/support.caopart"), b"PK")
            .expect("writes");

        files.discard(Path::new("/parts/drafts")).expect("discards");

        assert!(!files.exists(Path::new("/parts/drafts/support.caopart")));
        assert!(
            files
                .entries(Path::new("/parts"))
                .expect("reads")
                .is_empty()
        );
    }

    #[test]
    fn discarding_what_is_not_there_says_so_rather_than_passing_quietly() {
        let files = InMemoryFiles::default();

        let error = files
            .discard(Path::new("/parts/absent.caopart"))
            .expect_err("nothing there");

        assert!(matches!(error, FileError::Absent(_)));
    }
}
