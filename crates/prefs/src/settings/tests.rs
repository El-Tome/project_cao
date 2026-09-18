//! What prefs · settings.rs is held to.

use std::path::PathBuf;

use super::*;
use crate::adapters::InMemoryFiles;
use crate::theme::Rgba;

fn changed() -> Settings {
    let mut settings = Settings::default();
    settings.theme.solid = Rgba::opaque(1.0, 0.0, 0.0);
    settings.viewport.cube_size = 42.0;
    settings
}

#[test]
fn a_profile_survives_being_written_and_read_back() {
    let files = InMemoryFiles::default();
    let path = PathBuf::from(format!("/shared/workshop.{PROFILE_EXTENSION}"));
    let profile = Profile::new("Workshop", changed());

    profile.export(&files, &path).expect("writes");

    assert_eq!(Profile::import(&files, &path).expect("reads"), profile);
}

#[test]
fn a_profile_written_before_languages_existed_reads_in_french() {
    let files = InMemoryFiles::default();
    let path = PathBuf::from(format!("/shared/older.{PROFILE_EXTENSION}"));
    files
        .write(
            &path,
            format!(r#"{{"version":{SETTINGS_VERSION},"name":"Older","settings":{{}}}}"#)
                .as_bytes(),
        )
        .expect("writes");

    let read = Profile::import(&files, &path).expect("reads");

    assert_eq!(read.settings.language, DEFAULT_LANGUAGE);
}

#[test]
fn a_profile_missing_fields_falls_back_to_the_defaults() {
    let files = InMemoryFiles::default();
    let path = PathBuf::from(format!("/shared/minimal.{PROFILE_EXTENSION}"));
    files
        .write(
            &path,
            format!(r#"{{"version":{SETTINGS_VERSION},"name":"Minimal","settings":{{}}}}"#)
                .as_bytes(),
        )
        .expect("writes");

    let read = Profile::import(&files, &path).expect("reads");

    assert_eq!(read.settings, Settings::default());
}
