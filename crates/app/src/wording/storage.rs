use cao_prefs::{FileError, StorageError};

use crate::lang::Catalogue;
use crate::wording::file;

/// A platform that gave no directory to keep the settings in.
///
/// The case is named by the shell, which is where asking the platform now
/// happens; only the sentence is decided here.
pub fn nowhere_to_keep_settings(lang: &Catalogue) -> String {
    lang.t("storage.nowhere_to_keep")
}

/// The only place a settings failure is turned into a sentence.
///
/// `cao_prefs` names the case; this decides how it reads. What it carries from
/// below — a file, a JSON document — says nothing a user can act on, so what
/// is said here is what went wrong with their settings, not what the library
/// reported.
pub fn say(lang: &Catalogue, error: &StorageError) -> String {
    match error {
        StorageError::File(fate) => match fate {
            FileError::Absent(path) => file::absent(lang, path),
            FileError::Refused(path) => file::refused(lang, path),
            FileError::Interrupted(path) => file::interrupted(lang, path),
        },
        StorageError::UnsupportedVersion(version) => lang.t_with(
            "storage.unsupported_version",
            &[("version", &version.to_string())],
        ),
        StorageError::Json(_) => lang.t("storage.unreadable_json"),
    }
}

#[cfg(test)]
mod tests;
