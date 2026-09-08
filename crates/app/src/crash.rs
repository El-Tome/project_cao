use std::path::{Path, PathBuf};

use cao_prefs::Locations;

/// Where a crash is written down.
///
/// A graphical build on Windows has no console, so a panic leaves nothing
/// behind and "ça a planté" is all one has to go on. Writing it to a file
/// beside the parts turns that into something readable.
fn crash_log_path(at: &Locations) -> PathBuf {
    at.data.join("plantages.log")
}

/// Writes a panic down before the window disappears, then lets the usual
/// handler run so a terminal still shows it.
pub fn record_panics(at: &Locations) {
    let path = crash_log_path(at);
    let previous = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |info| {
        let _ = append_crash(&path, &info.to_string());
        previous(info);
    }));
}

/// Adds one entry to the crash file, making the folder if it is not there yet.
fn append_crash(path: &Path, message: &str) -> std::io::Result<()> {
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
    use super::*;

    #[test]
    fn the_journal_sits_beside_the_data_the_platform_gave() {
        let at = Locations {
            config: "/config".into(),
            data: "/data".into(),
            documents: None,
        };

        assert_eq!(crash_log_path(&at), PathBuf::from("/data/plantages.log"));
    }

    #[test]
    fn a_crash_is_written_down_with_its_hour_and_its_stack() {
        // A run of the suite next to this one must not be writing the same
        // file: the two would read each other's entries.
        let folder =
            std::env::temp_dir().join(format!("cao-essai-plantage-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&folder);
        let path = folder.join("plantages.log");

        append_crash(&path, "premier essai").expect("the file is written");
        append_crash(&path, "second essai").expect("it is written again");

        let written = std::fs::read_to_string(&path).expect("the journal is there");
        assert!(written.contains("premier essai"));
        assert!(
            written.contains("second essai"),
            "the second is added to the first",
        );
        assert!(written.contains("append_crash"), "with the call stack");
        let _ = std::fs::remove_dir_all(&folder);
    }
}
