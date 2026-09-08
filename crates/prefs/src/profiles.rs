//! Every profile the installation keeps, and which one is in use.
//!
//! One profile is a `Settings` under a name; this is the set of them, read at
//! start-up and written back whenever one is changed.

use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::locations::Locations;
use crate::ports::Files;
use crate::settings::{DEFAULT_PROFILE, Profile, SETTINGS_VERSION, Settings};
use crate::storage::StorageError;

/// Every profile the user has, and which one is in use.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Profiles {
    pub version: u32,
    active: String,
    profiles: Vec<Profile>,
}

impl Default for Profiles {
    fn default() -> Self {
        Self {
            version: SETTINGS_VERSION,
            active: DEFAULT_PROFILE.to_string(),
            profiles: vec![Profile::new(DEFAULT_PROFILE, Settings::default())],
        }
    }
}

impl Profiles {
    fn state_path(at: &Locations) -> PathBuf {
        at.config.join("settings.json")
    }

    /// Reads the settings, falling back to the defaults rather than failing.
    ///
    /// A settings file that cannot be read must not stop the application from
    /// starting: the user would be left with no way in, and no way to fix it
    /// from inside.
    pub fn load(files: &impl Files, at: &Locations) -> Self {
        Self::read(files, at).unwrap_or_default()
    }

    fn read(files: &impl Files, at: &Locations) -> Result<Self, StorageError> {
        let path = Self::state_path(at);
        if !files.exists(&path) {
            return Ok(Self::default());
        }
        let mut profiles: Self = serde_json::from_slice(&files.read(&path)?)?;
        if profiles.version != SETTINGS_VERSION || profiles.profiles.is_empty() {
            return Ok(Self::default());
        }
        // A toolbar saved before a tool existed would never hear of it. What
        // the user arranged stays put; only the new buttons are added.
        let standard = crate::toolbar::ToolbarLayout::default();
        for profile in &mut profiles.profiles {
            profile.settings.toolbar.adopt_new_commands(&standard);
        }
        Ok(profiles)
    }

    pub fn save(&self, files: &impl Files, at: &Locations) -> Result<(), StorageError> {
        let json = serde_json::to_string_pretty(self)?;
        Ok(files.write(&Self::state_path(at), json.as_bytes())?)
    }

    pub fn names(&self) -> impl Iterator<Item = &str> {
        self.profiles.iter().map(|profile| profile.name.as_str())
    }

    pub fn active_name(&self) -> &str {
        &self.active
    }

    fn active_index(&self) -> usize {
        self.profiles
            .iter()
            .position(|profile| profile.name == self.active)
            .unwrap_or(0)
    }

    pub fn active(&self) -> &Settings {
        &self.profiles[self.active_index()].settings
    }

    pub fn active_mut(&mut self) -> &mut Settings {
        let index = self.active_index();
        &mut self.profiles[index].settings
    }

    pub fn switch_to(&mut self, name: &str) -> bool {
        if !self.profiles.iter().any(|profile| profile.name == name) {
            return false;
        }
        self.active = name.to_string();
        true
    }

    /// Adds a profile, taking a free name when the one asked for is taken.
    pub fn add(&mut self, profile: Profile) -> String {
        let name = self.free_name(&profile.name);
        self.profiles.push(Profile {
            name: name.clone(),
            ..profile
        });
        self.active = name.clone();
        name
    }

    pub fn duplicate_active(&mut self, name: &str) -> String {
        let settings = self.active().clone();
        self.add(Profile::new(name, settings))
    }

    fn free_name(&self, wanted: &str) -> String {
        let taken = |name: &str| self.profiles.iter().any(|profile| profile.name == name);
        if !taken(wanted) {
            return wanted.to_string();
        }
        (2..)
            .map(|suffix| format!("{wanted} {suffix}"))
            .find(|name| !taken(name))
            .unwrap_or_else(|| wanted.to_string())
    }

    /// Removes a profile. The default one stays, and so does the last one.
    pub fn remove(&mut self, name: &str) -> bool {
        if name == DEFAULT_PROFILE || self.profiles.len() <= 1 {
            return false;
        }
        let before = self.profiles.len();
        self.profiles.retain(|profile| profile.name != name);
        if self.profiles.len() == before {
            return false;
        }
        if self.active == name {
            self.active = self.profiles[0].name.clone();
        }
        true
    }

    /// Puts the settings of the profile in use back to the defaults, leaving
    /// the other profiles alone.
    pub fn reset_active(&mut self) {
        *self.active_mut() = Settings::default();
    }

    pub fn active_profile(&self) -> Profile {
        self.profiles[self.active_index()].clone()
    }
}

#[cfg(test)]
mod tests {
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
}
