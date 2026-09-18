use cao_part::{FileError, PartFileError};

use crate::lang::Catalogue;
use crate::wording::file;

/// The only place a part file failure is turned into a sentence.
///
/// `cao_part` names the case; this decides how it reads. The three cases it
/// carries from below — a file, a zip, a JSON document — say nothing a user
/// can act on, so what is said here is what went wrong with their part, not
/// what the library reported.
pub fn say(lang: &Catalogue, error: &PartFileError) -> String {
    match error {
        PartFileError::MissingEntry(entry) => {
            lang.t_with("part_file.missing_entry", &[("entry", entry)])
        }
        PartFileError::UnsupportedVersion(version) => lang.t_with(
            "part_file.unsupported_version",
            &[("version", &version.to_string())],
        ),
        PartFileError::File(fate) => match fate {
            FileError::Absent(path) => file::absent(lang, path),
            FileError::Refused(path) => file::refused(lang, path),
            FileError::Interrupted(path) => file::interrupted(lang, path),
        },
        PartFileError::Archive(_) => lang.t("part_file.archive_unreadable"),
        PartFileError::Json(_) => lang.t("part_file.json_unreadable"),
        PartFileError::BrokenDesign => lang.t("part_file.broken_design"),
        PartFileError::BlankName => lang.t("part_file.blank_name"),
        PartFileError::NameTaken(path) => lang.t_with(
            "part_file.name_taken",
            &[("name", &cao_part::library::name_of(path))],
        ),
    }
}

#[cfg(test)]
mod tests;
