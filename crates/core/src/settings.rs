use std::fs;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

use crate::config::ViewportConfig;
use crate::shortcuts::Shortcuts;
use crate::storage::{StorageError, project_dirs};
use crate::theme::Theme;
use crate::toolbar::ToolbarLayout;

/// Bumped when the shape of a saved profile changes.
pub const SETTINGS_VERSION: u32 = 1;

/// Everything the user can set. One value, so a profile is one thing to save,
/// to reset, and to hand to somebody else.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Settings {
    pub viewport: ViewportConfig,
    pub theme: Theme,
    pub shortcuts: Shortcuts,
    pub toolbar: ToolbarLayout,
}

/// A named set of settings, as written to disk and as shared.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Profile {
    pub version: u32,
    pub name: String,
    pub settings: Settings,
}

impl Profile {
    pub fn new(name: impl Into<String>, settings: Settings) -> Self {
        Self {
            version: SETTINGS_VERSION,
            name: name.into(),
            settings,
        }
    }

    /// Reads a profile somebody else wrote out.
    ///
    /// Every field has a default, so a profile from a version that knew fewer
    /// settings still loads — the ones it never heard of simply keep their
    /// defaults. Only a different overall version is refused.
    pub fn import(path: &Path) -> Result<Self, StorageError> {
        let profile: Self = serde_json::from_str(&fs::read_to_string(path)?)?;
        if profile.version != SETTINGS_VERSION {
            return Err(StorageError::UnsupportedVersion(profile.version));
        }
        Ok(profile)
    }

    pub fn export(&self, path: &Path) -> Result<(), StorageError> {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::write(path, serde_json::to_string_pretty(self)?)?;
        Ok(())
    }
}

/// File extension for a shared profile.
pub const PROFILE_EXTENSION: &str = "caoprofile";

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

/// The profile that always exists. Deleting the last one would leave nothing to
/// fall back to, so this one is never removed.
pub const DEFAULT_PROFILE: &str = "Par défaut";

impl Profiles {
    fn state_path() -> Result<PathBuf, StorageError> {
        Ok(project_dirs()?.config_dir().join("settings.json"))
    }

    /// Reads the settings, falling back to the defaults rather than failing.
    ///
    /// A settings file that cannot be read must not stop the application from
    /// starting: the user would be left with no way in, and no way to fix it
    /// from inside.
    pub fn load() -> Self {
        Self::read().unwrap_or_default()
    }

    fn read() -> Result<Self, StorageError> {
        let path = Self::state_path()?;
        if !path.exists() {
            return Ok(Self::default());
        }
        let mut profiles: Self = serde_json::from_str(&fs::read_to_string(path)?)?;
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

    pub fn save(&self) -> Result<(), StorageError> {
        let path = Self::state_path()?;
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::write(path, serde_json::to_string_pretty(self)?)?;
        Ok(())
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
        self.profiles.push(Profile { name: name.clone(), ..profile });
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
    use super::*;
    use crate::theme::Rgba;

    fn changed() -> Settings {
        let mut settings = Settings::default();
        settings.theme.solid = Rgba::opaque(1.0, 0.0, 0.0);
        settings.viewport.cube_size = 42.0;
        settings
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
        profiles.add(Profile::new("Atelier", changed()));
        assert_eq!(profiles.active_name(), "Atelier");
        assert_eq!(profiles.active().viewport.cube_size, 42.0);

        assert!(profiles.switch_to(DEFAULT_PROFILE));
        assert_eq!(profiles.active().viewport.cube_size, 96.0);
    }

    /// Two profiles of the same name would be indistinguishable in the list.
    #[test]
    fn a_taken_name_is_made_free() {
        let mut profiles = Profiles::default();
        let name = profiles.add(Profile::new(DEFAULT_PROFILE, Settings::default()));
        assert_eq!(name, "Par défaut 2");
        assert_eq!(profiles.names().count(), 2);
    }

    #[test]
    fn resetting_only_touches_the_profile_in_use() {
        let mut profiles = Profiles::default();
        profiles.add(Profile::new("Atelier", changed()));
        profiles.reset_active();

        assert_eq!(profiles.active(), &Settings::default());
        assert!(profiles.switch_to("Atelier"));
    }

    #[test]
    fn the_default_profile_cannot_be_removed() {
        let mut profiles = Profiles::default();
        profiles.add(Profile::new("Atelier", changed()));

        assert!(!profiles.remove(DEFAULT_PROFILE));
        assert!(profiles.remove("Atelier"));
        assert_eq!(profiles.active_name(), DEFAULT_PROFILE);
        assert!(!profiles.remove(DEFAULT_PROFILE), "il en reste un seul");
    }

    #[test]
    fn a_profile_survives_being_written_and_read_back() {
        let directory = std::env::temp_dir().join(format!("cao_profile_{}", uuid::Uuid::new_v4()));
        let path = directory.join(format!("atelier.{PROFILE_EXTENSION}"));
        let profile = Profile::new("Atelier", changed());

        profile.export(&path).expect("écriture");
        let read = Profile::import(&path).expect("lecture");
        assert_eq!(read, profile);

        let _ = fs::remove_dir_all(&directory);
    }

    /// A profile written by a version that knew fewer settings still loads.
    #[test]
    fn a_profile_missing_fields_falls_back_to_the_defaults() {
        let directory = std::env::temp_dir().join(format!("cao_partial_{}", uuid::Uuid::new_v4()));
        fs::create_dir_all(&directory).expect("dossier");
        let path = directory.join("partial.caoprofile");
        fs::write(
            &path,
            format!(r#"{{"version":{SETTINGS_VERSION},"name":"Minimal","settings":{{}}}}"#),
        )
        .expect("écriture");

        let read = Profile::import(&path).expect("lecture");
        assert_eq!(read.settings, Settings::default());

        let _ = fs::remove_dir_all(&directory);
    }
}
