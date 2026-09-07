use std::path::PathBuf;

use directories::{ProjectDirs, UserDirs};

#[derive(Debug, thiserror::Error)]
pub enum StorageError {
    #[error("could not resolve a config/data directory on this platform")]
    NoProjectDirs,
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    Json(#[from] serde_json::Error),
    #[error(transparent)]
    Archive(#[from] zip::result::ZipError),
    #[error("le fichier de pièce ne contient pas « {0} »")]
    MissingEntry(String),
    #[error("pièce enregistrée dans une version antérieure (v{0}), non prise en charge")]
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
    #[test]
    fn a_crash_is_written_down_with_its_hour_and_its_stack() {
        let folder = std::env::temp_dir().join("cao-essai-plantage");
        let _ = std::fs::remove_dir_all(&folder);
        let path = folder.join("plantages.log");

        super::append_crash(&path, "premier essai").expect("le fichier s'écrit");
        super::append_crash(&path, "second essai").expect("il s'écrit encore");

        let written = std::fs::read_to_string(&path).expect("le journal existe");
        assert!(written.contains("premier essai"));
        assert!(written.contains("second essai"), "le second s'ajoute au premier");
        assert!(written.contains("append_crash"), "avec la pile d'appels");
        let _ = std::fs::remove_dir_all(&folder);
    }
}
