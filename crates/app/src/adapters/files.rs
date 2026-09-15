use std::fs;
use std::io::Write;
use std::path::Path;
use std::sync::atomic::{AtomicU64, Ordering};

use cao_part::{Entry, FileError, Files, Folders};

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

impl Folders for DiskFiles {
    fn entries(&self, folder: &Path) -> Result<Vec<Entry>, FileError> {
        let listing = match fs::read_dir(folder) {
            Ok(listing) => listing,
            // A library the user has not started yet is not a failure to
            // report: the panel shows an empty root, and the first part made
            // brings the folder into being.
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(Vec::new()),
            Err(error) => return Err(reading(folder, &error)),
        };

        let mut entries = Vec::new();
        for entry in listing.flatten() {
            // `file_type` does not follow a link, which is what keeps a folder
            // pointing at one of its own parents from being walked for ever:
            // a link is neither a folder nor a part, and is left out.
            let Ok(kind) = entry.file_type() else {
                continue;
            };
            if kind.is_dir() || kind.is_file() {
                entries.push(Entry {
                    path: entry.path(),
                    folder: kind.is_dir(),
                });
            }
        }
        Ok(entries)
    }

    fn create(&self, folder: &Path) -> Result<(), FileError> {
        fs::create_dir_all(folder).map_err(|error| writing(folder, &error))
    }

    fn rename(&self, from: &Path, to: &Path) -> Result<(), FileError> {
        fs::rename(from, to).map_err(|error| writing(to, &error))
    }

    fn discard(&self, path: &Path) -> Result<(), FileError> {
        trash::delete(path).map_err(|_| FileError::Refused(path.to_path_buf()))
    }
}

/// The preferences ask for the same three things from their own port: the
/// dependency graph forbids `cao_prefs` from reaching into `cao_part`.
impl cao_prefs::Files for DiskFiles {
    fn read(&self, path: &Path) -> Result<Vec<u8>, cao_prefs::FileError> {
        Files::read(self, path).map_err(as_preference)
    }

    fn write(&self, path: &Path, bytes: &[u8]) -> Result<(), cao_prefs::FileError> {
        Files::write(self, path, bytes).map_err(as_preference)
    }

    fn exists(&self, path: &Path) -> bool {
        Files::exists(self, path)
    }
}

fn as_preference(error: FileError) -> cao_prefs::FileError {
    match error {
        FileError::Absent(path) => cao_prefs::FileError::Absent(path),
        FileError::Refused(path) => cao_prefs::FileError::Refused(path),
        FileError::Interrupted(path) => cao_prefs::FileError::Interrupted(path),
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
    fn a_folder_tells_the_folders_it_holds_from_the_files() {
        let root = temp_dir();
        DiskFiles
            .write(&root.join("support.caopart"), b"PK")
            .expect("writes");
        DiskFiles.create(&root.join("drafts")).expect("creates");

        let mut entries = DiskFiles.entries(&root).expect("reads");
        entries.sort();

        assert_eq!(
            entries,
            [
                Entry {
                    path: root.join("drafts"),
                    folder: true,
                },
                Entry {
                    path: root.join("support.caopart"),
                    folder: false,
                },
            ],
        );

        fs::remove_dir_all(&root).ok();
    }

    #[test]
    fn a_library_the_user_has_not_started_yet_reads_as_empty_rather_than_failing() {
        let root = temp_dir();

        let entries = DiskFiles
            .entries(&root.join("nothing here"))
            .expect("reads");

        assert!(entries.is_empty());

        fs::remove_dir_all(&root).ok();
    }

    #[test]
    fn a_renamed_folder_takes_the_parts_it_held_with_it() {
        let root = temp_dir();
        DiskFiles
            .write(&root.join("drafts").join("support.caopart"), b"PK")
            .expect("writes");

        DiskFiles
            .rename(&root.join("drafts"), &root.join("kept"))
            .expect("renames");

        assert!(DiskFiles.exists(&root.join("kept").join("support.caopart")));
        assert!(!DiskFiles.exists(&root.join("drafts")));

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
