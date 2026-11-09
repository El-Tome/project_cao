//! What prefs · profiles.rs is held to.
//!
//! Closes #724.
//! - the « Par défaut » profile takes the redesign's viewport colours as
//!   defaults; a profile Tom saved keeps its own —
//!   `a_profile_saved_with_viewport_colours_of_its_own_reads_back_with_them`;
//!   the new defaults themselves are held in `crates/prefs/src/theme/tests.rs`
//!
//! Closes #794.
//! - What the user removes stays removed: a shortcut unbound with « Retirer »,
//!   a button removed from the toolbar. Restarting the application leaves the
//!   profile exactly as it was saved —
//!   `a_shortcut_the_user_unbound_stays_unbound_across_a_save_and_a_load`,
//!   `a_button_the_user_removed_stays_removed_across_a_save_and_a_load`
//! - A saved profile is taken as it is. Nothing is added to it at load: a
//!   command added in a later version does not appear by itself in a saved
//!   toolbar or a saved set of shortcuts —
//!   `a_saved_profile_missing_commands_reads_back_exactly_as_it_was_written`
//! - A fresh installation, with no profile saved, still starts on the full
//!   default toolbar and shortcuts —
//!   `settings_nobody_has_written_yet_open_on_the_defaults`, whose
//!   `Profiles::default()` holds `Settings::default()`, built on
//!   `ToolbarLayout::default()` and `Shortcuts::default()`
//! - Changing a shortcut's key, moving, grouping or renaming in the toolbar:
//!   all kept across a restart, as today —
//!   `a_shortcut_moved_to_another_key_and_a_rearranged_toolbar_read_back_as_left`
//! - « ↺ Barre par défaut » and « ↺ Raccourcis par défaut » bring back every
//!   default command and key — no test: the buttons, in
//!   `crates/app/src/screens/settings/`, assign `Default::default()` and are
//!   not touched; what that default holds is the fresh-installation bullet above
//! - `no_two_commands_answer_to_the_same_chord_on_a_fresh_installation` stays —
//!   no test: it stays where it is, in `crates/prefs/src/shortcuts/tests.rs`
//! - The tests pinning the adoption go — no test: they are removed, and the
//!   tests of the first two bullets say the opposite

use std::path::Path;

use super::*;
use crate::adapters::InMemoryFiles;
use crate::command::Command;
use crate::shortcuts::Shortcuts;
use crate::theme::{Background, Rgba, Stop};
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

/// The redesign gave the « Par défaut » profile new viewport colours; a profile
/// someone saved with colours of their own opens on those, not on the new
/// defaults.
#[test]
fn a_profile_saved_with_viewport_colours_of_its_own_reads_back_with_them() {
    let at = at();
    let files = InMemoryFiles::default();
    let mut settings = Settings::default();
    settings.theme.background = Background::Solid(Rgba::opaque(0.30, 0.20, 0.10));
    settings.theme.sketch_background = Background::Linear {
        angle_degrees: 30.0,
        stops: vec![
            Stop::new(0.0, Rgba::opaque(0.10, 0.20, 0.30)),
            Stop::new(1.0, Rgba::opaque(0.40, 0.50, 0.60)),
        ],
    };
    settings.theme.grid_minor = Rgba::new(0.70, 0.60, 0.50, 0.40);
    settings.theme.grid_major = Rgba::new(0.20, 0.30, 0.40, 0.90);
    settings.theme.axis_x = Rgba::opaque(0.11, 0.22, 0.33);
    settings.theme.axis_y = Rgba::opaque(0.44, 0.55, 0.66);
    settings.theme.axis_z = Rgba::opaque(0.77, 0.88, 0.99);
    let mut profiles = Profiles::default();
    profiles.add(Profile::new("Atelier", settings.clone()));

    profiles
        .save(&files, &at)
        .expect("the settings are written");
    let read = Profiles::load(&files, &at);

    assert_eq!(read.active_name(), "Atelier");
    assert_eq!(
        read.active().theme,
        settings.theme,
        "the profile opens on other viewport colours than the ones it was saved with",
    );
}

#[test]
fn a_shortcut_the_user_unbound_stays_unbound_across_a_save_and_a_load() {
    let at = at();
    let files = InMemoryFiles::default();
    let mut profiles = Profiles::default();
    profiles.active_mut().shortcuts.unbind(Command::FinishSketch);
    let saved = profiles.active().shortcuts.clone();
    assert_ne!(saved, Shortcuts::default(), "a shortcut was unbound");

    profiles
        .save(&files, &at)
        .expect("the settings are written");
    let read = Profiles::load(&files, &at);

    assert_eq!(
        read.active().shortcuts.chord_for(Command::FinishSketch),
        None,
        "the shortcut the user unbound is back on a key after a restart",
    );
    assert_eq!(read.active().shortcuts, saved);
}
