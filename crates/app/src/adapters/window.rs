use std::path::Path;
use std::process::Command;

/// Opens `part` in a window of its own, by starting the application again.
///
/// Tabs are what this is to become; a second process is what puts two parts
/// side by side today without the shell having to learn to hold more than one
/// at a time. What the two share is what the installation remembers — the
/// settings and the recent list — and the last one to write a change to those
/// is the one that stands.
pub fn open_another(part: &Path) -> bool {
    let Ok(program) = std::env::current_exe() else {
        return false;
    };
    Command::new(program).arg(part).spawn().is_ok()
}
