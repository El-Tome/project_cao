use cao_part::{FileError, PartFileError};

use crate::wording::file;

/// The only place a part file failure is turned into a sentence.
///
/// `cao_part` names the case; this decides how it reads. The three cases it
/// carries from below — a file, a zip, a JSON document — say nothing a user
/// can act on, so what is said here is what went wrong with their part, not
/// what the library reported.
pub fn say(error: &PartFileError) -> String {
    match error {
        PartFileError::MissingEntry(entry) => {
            format!("Le fichier de pièce ne contient pas « {entry} ».")
        }
        PartFileError::UnsupportedVersion(version) => format!(
            "Cette pièce a été enregistrée dans une autre version du logiciel (v{version}) \
             et ne peut pas être ouverte."
        ),
        PartFileError::File(fate) => match fate {
            FileError::Absent(path) => file::absent(path),
            FileError::Refused(path) => file::refused(path),
            FileError::Interrupted(path) => file::interrupted(path),
        },
        PartFileError::Archive(_) => {
            "L'archive de cette pièce n'a pas pu être traitée.".to_string()
        }
        PartFileError::Json(_) => "Le contenu de cette pièce n'a pas pu être traité.".to_string(),
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use super::*;

    #[test]
    fn an_archive_missing_an_entry_says_which_one_is_missing() {
        let said = say(&PartFileError::MissingEntry("piece.json".into()));

        assert!(
            said.contains("piece.json"),
            "the reader is told which entry is missing: {said}",
        );
    }

    #[test]
    fn a_part_from_an_older_schema_says_which_version_wrote_it() {
        let said = say(&PartFileError::UnsupportedVersion(1));

        assert!(
            said.contains("v1"),
            "the reader is told which version wrote the part: {said}",
        );
    }

    #[test]
    fn every_fate_the_disk_reports_names_the_path_and_reads_differently() {
        let fates = [
            FileError::Absent("/parts/a.caopart".into()),
            FileError::Refused("/parts/a.caopart".into()),
            FileError::Interrupted("/parts/a.caopart".into()),
        ];

        let said: BTreeSet<String> = fates
            .into_iter()
            .map(|fate| say(&PartFileError::File(fate)))
            .inspect(|said| assert!(said.contains("/parts/a.caopart"), "{said}"))
            .collect();

        assert_eq!(
            said.len(),
            3,
            "two fates of the disk read the same: {said:?}"
        );
    }

    #[test]
    fn a_file_that_is_not_a_part_archive_says_so_instead_of_showing_the_zip_error() {
        use cao_part::Files as _;

        let files = cao_part::InMemoryFiles::default();
        let path = std::path::Path::new("/parts/a.caopart");
        files
            .write(path, b"PK\x03\x04 and nothing behind it")
            .expect("the file is written");

        let error = cao_part::PartDocument::load(&files, path).expect_err("not an archive");

        assert!(
            matches!(error, PartFileError::Archive(_)),
            "the guard in PartDocument::load changed and this no longer reaches the zip: {error:?}",
        );
        assert_ne!(
            say(&error),
            error.to_string(),
            "the reader is shown what the zip library said",
        );
        assert!(say(&error).contains("archive"), "{}", say(&error));
    }

    #[test]
    fn contents_that_are_not_readable_json_say_so_without_the_words_of_the_parser() {
        let broken = serde_json::from_str::<u32>("not json at all").expect_err("a parse error");

        let said = say(&PartFileError::Json(broken));

        assert!(
            said.contains("pièce"),
            "the reader is told it is the part that failed: {said}",
        );
        assert!(
            !said.contains("expected"),
            "the reader is shown what the parser said: {said}",
        );
    }
}
