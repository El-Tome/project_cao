use std::cell::RefCell;
use std::collections::HashMap;
use std::path::{Path, PathBuf};

use crate::ports::{FileError, Files};

/// A filesystem that never leaves the process, for tests that are about a part
/// rather than about a disk.
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
}
