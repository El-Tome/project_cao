use std::fs;
use std::io::Write;
use std::path::Path;
use std::sync::atomic::{AtomicU64, Ordering};

use cao_part::{FileError, Files};

pub struct DiskFiles;

impl Files for DiskFiles {
    fn read(&self, path: &Path) -> Result<Vec<u8>, FileError> {
        fs::read(path).map_err(|error| reading(path, &error))
    }

    fn write(&self, path: &Path, bytes: &[u8]) -> Result<(), FileError> {
        if let Some(folder) = path.parent() {
            fs::create_dir_all(folder).map_err(|error| writing(path, &error))?;
        }
        replace_whole(path, |file| file.write_all(bytes)).map_err(|error| writing(path, &error))
    }

    fn exists(&self, path: &Path) -> bool {
        path.exists()
    }
}

fn reading(path: &Path, error: &std::io::Error) -> FileError {
    match error.kind() {
        std::io::ErrorKind::NotFound => FileError::Absent(path.to_path_buf()),
        std::io::ErrorKind::PermissionDenied => FileError::Refused(path.to_path_buf()),
        _ => FileError::Interrupted(path.to_path_buf()),
    }
}

fn writing(path: &Path, error: &std::io::Error) -> FileError {
    match error.kind() {
        std::io::ErrorKind::PermissionDenied => FileError::Refused(path.to_path_buf()),
        _ => FileError::Interrupted(path.to_path_buf()),
    }
}

/// Puts `write`'s bytes at `path`, or leaves whatever was there untouched.
///
/// The temporary goes in the destination folder because `rename` is only atomic
/// within one filesystem, and `sync_all` comes before it because some
/// filesystems otherwise reorder the two and leave the renamed file empty.
fn replace_whole(
    path: &Path,
    write: impl FnOnce(&mut fs::File) -> std::io::Result<()>,
) -> std::io::Result<()> {
    let directory = path.parent().unwrap_or_else(|| Path::new("."));
    let temporary = directory.join(beside(path));

    match fill(&temporary, write).and_then(|()| fs::rename(&temporary, path)) {
        Ok(()) => Ok(()),
        Err(error) => {
            fs::remove_file(&temporary).ok();
            Err(error)
        }
    }
}

fn fill(
    path: &Path,
    write: impl FnOnce(&mut fs::File) -> std::io::Result<()>,
) -> std::io::Result<()> {
    let mut file = fs::File::create(path)?;
    write(&mut file)?;
    file.sync_all()
}

fn beside(path: &Path) -> String {
    static NEXT: AtomicU64 = AtomicU64::new(0);
    format!(
        ".{}.{}.{}.tmp",
        path.file_name().unwrap_or_default().to_string_lossy(),
        std::process::id(),
        NEXT.fetch_add(1, Ordering::Relaxed),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    fn temp_dir() -> std::path::PathBuf {
        let directory = std::env::temp_dir().join(format!(
            "cao_files_{}_{}",
            std::process::id(),
            NEXT_DIRECTORY.fetch_add(1, Ordering::Relaxed),
        ));
        fs::create_dir_all(&directory).expect("temp dir");
        directory
    }

    static NEXT_DIRECTORY: AtomicU64 = AtomicU64::new(0);

    #[test]
    fn a_write_makes_the_folder_it_lands_in_and_reads_back_whole() {
        let root = temp_dir();
        let path = root.join("parts").join("drafts").join("piece.caopart");

        assert!(!DiskFiles.exists(&path));
        DiskFiles.write(&path, b"the whole thing").expect("writes");

        assert!(DiskFiles.exists(&path));
        assert_eq!(DiskFiles.read(&path).expect("reads"), b"the whole thing");

        fs::remove_dir_all(&root).ok();
    }

    #[test]
    fn reading_a_file_that_was_never_written_says_it_is_absent() {
        let root = temp_dir();

        let error = DiskFiles
            .read(&root.join("nothing.caopart"))
            .expect_err("nothing there");

        assert!(matches!(error, FileError::Absent(_)));

        fs::remove_dir_all(&root).ok();
    }

    #[test]
    fn a_write_that_stops_partway_leaves_the_previous_content_intact() {
        let directory = temp_dir();
        let path = directory.join("piece.caopart");
        DiskFiles.write(&path, b"the whole thing").expect("writes");

        let error = replace_whole(&path, |file| {
            file.write_all(b"half of it")?;
            Err(std::io::Error::other("the write stops here"))
        })
        .expect_err("the replacement must fail");

        assert_eq!(error.to_string(), "the write stops here");
        assert_eq!(fs::read(&path).expect("bytes"), b"the whole thing");
        assert_eq!(
            fs::read_dir(&directory).expect("directory").count(),
            1,
            "nothing is left beside it",
        );

        fs::remove_dir_all(&directory).ok();
    }
}
