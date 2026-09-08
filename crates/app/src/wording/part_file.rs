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
            "Ce fichier de pièce est abîmé : son archive ne s'ouvre pas.".to_string()
        }
        PartFileError::Json(_) => {
            "Ce fichier de pièce est abîmé : son contenu ne se relit pas.".to_string()
        }
    }
}

#[cfg(test)]
mod tests {
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
            said.contains('1'),
            "the reader is told which version wrote the part: {said}",
        );
    }

    #[test]
    fn a_file_that_could_not_be_read_names_the_path_and_says_what_stopped_it() {
        let absent = say(&PartFileError::File(FileError::Absent(
            "/parts/a.caopart".into(),
        )));
        let refused = say(&PartFileError::File(FileError::Refused(
            "/parts/a.caopart".into(),
        )));

        assert!(absent.contains("/parts/a.caopart"), "{absent}");
        assert!(refused.contains("/parts/a.caopart"), "{refused}");
        assert_ne!(
            absent, refused,
            "a missing file and a refused one read alike"
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

        let said = say(&cao_part::PartDocument::load(&files, path).expect_err("not an archive"));

        assert!(
            said.contains("pièce"),
            "the reader is told it is the part file that failed: {said}",
        );
    }
}
