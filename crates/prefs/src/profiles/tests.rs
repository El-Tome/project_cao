//! What prefs · profiles.rs is held to.

use std::path::Path;

use super::*;
use crate::adapters::InMemoryFiles;
use crate::theme::Rgba;
use crate::toolbar::ToolbarLayout;

/// Where the platform says things go. A test never asks the machine it runs
/// on, so nothing it writes can land in a developer's own configuration.
fn at() -> Locations {
    Locations {
        config: "/config".into(),
        data: "/data".into(),
        documents: None,
    }
}

fn changed() -> Settings {
    let mut settings = Settings::default();
    settings.theme.solid = Rgba::opaque(1.0, 0.0, 0.0);
    settings.viewport.cube_size = 42.0;
    settings
}

#[test]
fn the_profile_that_always_exists_is_named_by_a_key() {
    assert_eq!(DEFAULT_PROFILE, "default");
    assert!(
        DEFAULT_PROFILE.is_ascii(),
        "a key the interface says in its own words holds no accent",
    );
}

#[test]
fn the_default_profile_is_always_there() {
    let profiles = Profiles::default();
    assert_eq!(profiles.active_name(), DEFAULT_PROFILE);
    assert_eq!(profiles.names().count(), 1);
}

#[test]
fn a_new_profile_becomes_the_active_one() {
    let mut profiles = Profiles::default();
    profiles.add(Profile::new("Workshop", changed()));
    assert_eq!(profiles.active_name(), "Workshop");
    assert_eq!(profiles.active().viewport.cube_size, 42.0);

    assert!(profiles.switch_to(DEFAULT_PROFILE));
    assert_eq!(profiles.active().viewport.cube_size, 96.0);
}

/// Two profiles of the same name would be indistinguishable in the list.
#[test]
fn a_taken_name_is_made_free() {
    let mut profiles = Profiles::default();
    let name = profiles.add(Profile::new(DEFAULT_PROFILE, Settings::default()));
    assert_eq!(name, "default 2");
    assert_eq!(profiles.names().count(), 2);
}

#[test]
fn resetting_only_touches_the_profile_in_use() {
    let mut profiles = Profiles::default();
    profiles.add(Profile::new("Workshop", changed()));
    profiles.reset_active();

    assert_eq!(profiles.active(), &Settings::default());
    assert!(profiles.switch_to("Workshop"));
}

#[test]
fn the_default_profile_cannot_be_removed() {
    let mut profiles = Profiles::default();
    profiles.add(Profile::new("Workshop", changed()));

    assert!(!profiles.remove(DEFAULT_PROFILE));
    assert!(profiles.remove("Workshop"));
    assert_eq!(profiles.active_name(), DEFAULT_PROFILE);
    assert!(!profiles.remove(DEFAULT_PROFILE), "only one is left");
}

#[test]
fn the_profiles_are_written_where_the_platform_says_and_read_back_from_there() {
    let at = at();
    let files = InMemoryFiles::default();
    let mut profiles = Profiles::default();
    profiles.add(Profile::new("Workshop", changed()));

    profiles
        .save(&files, &at)
        .expect("the settings are written");

    assert!(
        files.exists(Path::new("/config/settings.json")),
        "the platform decides where, not the library",
    );
    assert_eq!(Profiles::load(&files, &at).active_name(), "Workshop");
}

#[test]
fn settings_nobody_has_written_yet_open_on_the_defaults() {
    assert_eq!(
        Profiles::load(&InMemoryFiles::default(), &at()),
        Profiles::default(),
    );
}

#[test]
fn settings_nobody_can_read_open_on_the_defaults_rather_than_barring_the_way_in() {
    let at = at();
    let files = InMemoryFiles::default();
    files
        .write(Path::new("/config/settings.json"), b"{ half a file")
        .expect("writes");

    assert_eq!(Profiles::load(&files, &at), Profiles::default());
}

#[test]
fn settings_written_by_another_version_open_on_the_defaults() {
    let at = at();
    let files = InMemoryFiles::default();
    let mut profiles = Profiles::default();
    profiles.add(Profile::new("Workshop", changed()));
    profiles.version = SETTINGS_VERSION - 1;
    profiles
        .save(&files, &at)
        .expect("the settings are written");

    assert_eq!(Profiles::load(&files, &at), Profiles::default());
}

#[test]
fn a_toolbar_arranged_before_a_tool_existed_hears_of_it_when_the_settings_open() {
    let at = at();
    let files = InMemoryFiles::default();
    let mut saved = ToolbarLayout::default();
    saved
        .remove(&[0, 0])
        .expect("a command of the standard toolbar");
    let mut profiles = Profiles::default();
    profiles.active_mut().toolbar = saved.clone();
    profiles
        .save(&files, &at)
        .expect("the settings are written");

    let mut expected = saved.clone();
    expected.adopt_new_commands(&ToolbarLayout::default());

    assert_ne!(expected, saved, "the saved toolbar is missing a command");
    assert_eq!(
        Profiles::load(&files, &at).active().toolbar,
        expected,
        "opening the settings is where a toolbar hears of the tools added since",
    );
}

#[test]
fn settings_holding_no_profile_at_all_open_on_the_defaults() {
    let at = at();
    let files = InMemoryFiles::default();
    files
        .write(
            Path::new("/config/settings.json"),
            format!(r#"{{"version":{SETTINGS_VERSION},"active":"default","profiles":[]}}"#)
                .as_bytes(),
        )
        .expect("writes");

    assert_eq!(Profiles::load(&files, &at), Profiles::default());
}
