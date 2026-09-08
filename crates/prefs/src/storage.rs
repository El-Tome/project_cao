use std::path::PathBuf;

use directories::{ProjectDirs, UserDirs};
use uuid::Uuid;

/// What can go wrong reading or writing what the installation remembers:
/// profiles, themes, shortcuts, the toolbar layout, the recent files.
#[derive(Debug, thiserror::Error)]
pub enum StorageError {
    #[error("could not resolve a config/data directory on this platform")]
    NoProjectDirs,
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    Json(#[from] serde_json::Error),
    #[error("profile written by an unsupported settings version (v{0})")]
    UnsupportedVersion(u32),
}

pub fn project_dirs() -> Result<ProjectDirs, StorageError> {
    ProjectDirs::from("dev", "cao", "cao").ok_or(StorageError::NoProjectDirs)
}

/// Where new parts land by default: `<Documents>/CAO` if a documents folder
/// exists on this platform, otherwise the app's own data directory.
pub fn default_projects_dir() -> Result<PathBuf, StorageError> {
    if let Some(user_dirs) = UserDirs::new()
        && let Some(docs) = user_dirs.document_dir()
    {
        return Ok(docs.join("CAO"));
    }
    Ok(project_dirs()?.data_dir().join("projects"))
}

/// Puts `write`'s bytes at `path`, making the folder if it is not there yet,
/// or leaves whatever was there untouched.
///
/// The temporary goes in the destination folder because `rename` is only atomic
/// within one filesystem, and `sync_all` comes before it because some
/// filesystems otherwise reorder the two and leave the renamed file empty.
pub(crate) fn replace_whole(
    path: &std::path::Path,
    write: impl FnOnce(&mut std::fs::File) -> std::io::Result<()>,
) -> std::io::Result<()> {
    if let Some(folder) = path.parent() {
        std::fs::create_dir_all(folder)?;
    }
    let temporary = path.with_extension(format!("{}.tmp", Uuid::new_v4()));

    match fill(&temporary, write).and_then(|()| std::fs::rename(&temporary, path)) {
        Ok(()) => Ok(()),
        Err(error) => {
            std::fs::remove_file(&temporary).ok();
            Err(error)
        }
    }
}

fn fill(
    path: &std::path::Path,
    write: impl FnOnce(&mut std::fs::File) -> std::io::Result<()>,
) -> std::io::Result<()> {
    let mut file = std::fs::File::create(path)?;
    write(&mut file)?;
    file.sync_all()
}

/// Where a crash is written down.
///
/// A graphical build on Windows has no console, so a panic leaves nothing
/// behind and "ça a planté" is all one has to go on. Writing it to a file
/// beside the parts turns that into something readable.
pub fn crash_log_path() -> Result<PathBuf, StorageError> {
    Ok(project_dirs()?.data_dir().join("plantages.log"))
}

/// Writes a panic down before the window disappears, then lets the usual
/// handler run so a terminal still shows it.
pub fn record_panics() {
    let previous = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |info| {
        if let Ok(path) = crash_log_path() {
            let _ = append_crash(&path, &info.to_string());
        }
        previous(info);
    }));
}

/// Adds one entry to the crash file, making the folder if it is not there yet.
fn append_crash(path: &std::path::Path, message: &str) -> std::io::Result<()> {
    use std::io::Write;
    if let Some(folder) = path.parent() {
        std::fs::create_dir_all(folder)?;
    }
    let mut file = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(path)?;
    writeln!(
        file,
        "--- {}\n{message}\n{}",
        chrono::Local::now().format("%Y-%m-%d %H:%M:%S"),
        std::backtrace::Backtrace::force_capture()
    )
}

#[cfg(test)]
mod tests {
    use std::io::Write;

    #[test]
    fn a_write_interrupted_partway_leaves_the_previous_file_intact() {
        let folder = std::env::temp_dir().join(format!("cao-test-write-{}", uuid::Uuid::new_v4()));
        let path = folder.join("settings.json");
        super::replace_whole(&path, |file| file.write_all(b"the whole thing")).expect("writes");

        let error = super::replace_whole(&path, |file| {
            file.write_all(b"half")?;
            Err(std::io::Error::other("the write stops here"))
        })
        .expect_err("the replacement must fail");

        let after = std::fs::read_to_string(&path).expect("the file is still there");
        let beside = std::fs::read_dir(&folder).expect("folder").count();

        assert_eq!(error.to_string(), "the write stops here");
        assert_eq!(after, "the whole thing");
        assert_eq!(beside, 1, "nothing is left beside it");

        let _ = std::fs::remove_dir_all(&folder);
    }

    #[test]
    fn a_crash_is_written_down_with_its_hour_and_its_stack() {
        let folder = std::env::temp_dir().join("cao-essai-plantage");
        let _ = std::fs::remove_dir_all(&folder);
        let path = folder.join("plantages.log");

        super::append_crash(&path, "premier essai").expect("le fichier s'écrit");
        super::append_crash(&path, "second essai").expect("il s'écrit encore");

        let written = std::fs::read_to_string(&path).expect("le journal existe");
        assert!(written.contains("premier essai"));
        assert!(
            written.contains("second essai"),
            "le second s'ajoute au premier"
        );
        assert!(written.contains("append_crash"), "avec la pile d'appels");
        let _ = std::fs::remove_dir_all(&folder);
    }
}
