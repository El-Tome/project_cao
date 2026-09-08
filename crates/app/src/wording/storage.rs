use cao_prefs::{FileError, StorageError};

use crate::wording::file;

/// The only place a settings failure is turned into a sentence.
///
/// `cao_prefs` names the case; this decides how it reads. What it carries from
/// below — a file, a JSON document — says nothing a user can act on, so what
/// is said here is what went wrong with their settings, not what the library
/// reported.
pub fn say(error: &StorageError) -> String {
    match error {
        StorageError::NoProjectDirs => {
            "Ce poste n'offre aucun dossier où garder les réglages.".to_string()
        }
        StorageError::File(fate) => match fate {
            FileError::Absent(path) => file::absent(path),
            FileError::Refused(path) => file::refused(path),
            FileError::Interrupted(path) => file::interrupted(path),
        },
        StorageError::UnsupportedVersion(version) => format!(
            "Ce profil a été enregistré dans une autre version du logiciel (v{version}) \
             et ne peut pas être ouvert."
        ),
        StorageError::Json(_) => {
            "Ce fichier de réglages est abîmé : son contenu ne se relit pas.".to_string()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_platform_with_nowhere_to_keep_settings_says_so_rather_than_naming_a_directory_kind() {
        let said = say(&StorageError::NoProjectDirs);

        assert!(!said.is_empty(), "the reader is left with nothing to read");
        assert!(
            !said.contains("config"),
            "the reader is shown the developer's words: {said}",
        );
    }

    #[test]
    fn a_settings_file_that_could_not_be_read_names_the_path_and_says_what_stopped_it() {
        let absent = say(&StorageError::File(FileError::Absent(
            "/etc/cao.json".into(),
        )));
        let refused = say(&StorageError::File(FileError::Refused(
            "/etc/cao.json".into(),
        )));

        assert!(absent.contains("/etc/cao.json"), "{absent}");
        assert!(refused.contains("/etc/cao.json"), "{refused}");
        assert_ne!(
            absent, refused,
            "a missing file and a refused one read alike"
        );
    }

    #[test]
    fn a_profile_from_an_older_settings_version_says_which_version_wrote_it() {
        let said = say(&StorageError::UnsupportedVersion(2));

        assert!(
            said.contains('2'),
            "the reader is told which version wrote the profile: {said}",
        );
    }

    #[test]
    fn a_profile_that_is_not_readable_json_says_so_instead_of_showing_the_parse_error() {
        use cao_prefs::Files as _;

        let files = cao_prefs::InMemoryFiles::default();
        let path = std::path::Path::new("/config/atelier.caoprofile");
        files
            .write(path, b"not json at all")
            .expect("the file is written");

        let said = say(&cao_prefs::Profile::import(&files, path).expect_err("not json"));

        assert!(
            said.contains("réglages"),
            "the reader is told it is their settings that failed: {said}",
        );
        assert!(
            !said.contains("expected"),
            "the reader is shown what the parser said: {said}",
        );
    }
}
