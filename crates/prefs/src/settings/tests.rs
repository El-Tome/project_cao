//! What prefs · settings.rs is held to.
//!
//! Closes #321.
//! - the panel of the copy tools comes back where it was last left, one
//!   position shared by every tool that uses it, kept across restarts —
//!   `the_copy_panel_s_place_survives_being_written_and_read_back`
//! - a settings file written before the panel existed still loads, with no
//!   place for it yet — `a_settings_file_written_before_the_panel_still_loads`

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

#[test]
fn the_copy_panel_s_place_survives_being_written_and_read_back() {
    let files = InMemoryFiles::default();
    let path = PathBuf::from(format!("/shared/panel.{PROFILE_EXTENSION}"));
    let settings = Settings {
        copy_panel_at: Some([320.0, 180.0]),
        ..Settings::default()
    };

    Profile::new("Panel", settings)
        .export(&files, &path)
        .expect("writes");

    let read = Profile::import(&files, &path).expect("reads");
    assert_eq!(read.settings.copy_panel_at, Some([320.0, 180.0]));
}

#[test]
fn a_settings_file_written_before_the_panel_still_loads() {
    let files = InMemoryFiles::default();
    let path = PathBuf::from(format!("/shared/before.{PROFILE_EXTENSION}"));
    files
        .write(
            &path,
            format!(
                r#"{{"version":{SETTINGS_VERSION},"name":"Before","settings":{{"language":"fr"}}}}"#
            )
            .as_bytes(),
        )
        .expect("writes");

    let read = Profile::import(&files, &path).expect("reads");

    assert_eq!(read.settings.copy_panel_at, None);
    assert_eq!(read.settings.language, "fr");
}
