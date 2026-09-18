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
        // A toolbar and a set of shortcuts saved before a tool existed would
        // never hear of it. What the user arranged stays put; only the new
        // buttons and the free chords are added.
        let standard = crate::toolbar::ToolbarLayout::default();
        let chords = crate::shortcuts::Shortcuts::default();
        for profile in &mut profiles.profiles {
            profile.settings.toolbar.adopt_new_commands(&standard);
            profile.settings.shortcuts.adopt_new_bindings(&chords);
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
mod tests;
