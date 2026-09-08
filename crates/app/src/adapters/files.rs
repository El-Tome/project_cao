use std::fs;
use std::io::Write;
use std::path::Path;

use cao_part::{FileError, Files};

/// The filesystem itself, which is what the port exists to keep out of the
/// layers below.
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
    let stamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|since| since.as_nanos())
        .unwrap_or_default();
    format!(
        ".{}.{}.{stamp}.tmp",
        path.file_name().unwrap_or_default().to_string_lossy(),
        std::process::id(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    fn temp_dir() -> std::path::PathBuf {
        let directory = std::env::temp_dir().join(format!("cao_files{}", beside(Path::new(""))));
        fs::create_dir_all(&directory).expect("temp dir");
        directory
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
