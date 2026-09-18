//! What app · wording/storage.rs is held to.

use std::collections::BTreeSet;

use super::*;

#[test]
fn a_platform_with_nowhere_to_keep_settings_says_so_rather_than_naming_a_directory_kind() {
    let said = nowhere_to_keep_settings(&Catalogue::french());

    assert!(!said.is_empty(), "the reader is left with nothing to read");
    assert!(
        !said.contains("config"),
        "the reader is shown the developer's words: {said}",
    );
}

#[test]
fn every_fate_the_disk_reports_names_the_path_and_reads_differently() {
    let fates = [
        FileError::Absent("/etc/cao.json".into()),
        FileError::Refused("/etc/cao.json".into()),
        FileError::Interrupted("/etc/cao.json".into()),
    ];

    let lang = Catalogue::french();
    let said: BTreeSet<String> = fates
        .into_iter()
        .map(|fate| say(&lang, &StorageError::File(fate)))
        .inspect(|said| assert!(said.contains("/etc/cao.json"), "{said}"))
        .collect();

    assert_eq!(
        said.len(),
        3,
        "two fates of the disk read the same: {said:?}"
    );
}

#[test]
fn a_profile_from_an_older_settings_version_says_which_version_wrote_it() {
    let said = say(&Catalogue::french(), &StorageError::UnsupportedVersion(2));

    assert!(
        said.contains("v2"),
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

    let said = say(
        &Catalogue::french(),
        &cao_prefs::Profile::import(&files, path).expect_err("not json"),
    );

    assert!(
        said.contains("réglages"),
        "the reader is told it is their settings that failed: {said}",
    );
    assert!(
        !said.contains("expected"),
        "the reader is shown what the parser said: {said}",
    );
}
