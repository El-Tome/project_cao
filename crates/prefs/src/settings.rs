use std::path::Path;

use serde::{Deserialize, Serialize};

use crate::config::ViewportConfig;
use crate::ports::Files;
use crate::shortcuts::Shortcuts;
use crate::storage::StorageError;
use crate::theme::Theme;
use crate::toolbar::ToolbarLayout;

/// Bumped when the shape of a saved profile changes.
///
/// 2 renamed the profile that always exists from a French sentence to the key
/// `default`. A file written by 1 named it something this version no longer
/// recognises, so it is not read at all: settings go back to the defaults
/// rather than opening half understood, with a protected profile the code
/// would let the user delete.
pub const SETTINGS_VERSION: u32 = 2;

/// Everything the user can set. One value, so a profile is one thing to save,
/// to reset, and to hand to somebody else.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Settings {
    pub viewport: ViewportConfig,
    pub theme: Theme,
    pub shortcuts: Shortcuts,
    pub toolbar: ToolbarLayout,
    /// Which language file the interface reads. A code naming the file, never
    /// a sentence: what it stands for is decided in `cao_app`.
    pub language: String,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            viewport: ViewportConfig::default(),
            theme: Theme::default(),
            shortcuts: Shortcuts::default(),
            toolbar: ToolbarLayout::default(),
            language: DEFAULT_LANGUAGE.to_string(),
        }
    }
}

/// The language the interface falls back on, and the only one carrying a
/// complete set of entries today.
pub const DEFAULT_LANGUAGE: &str = "fr";

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
    pub fn import(files: &impl Files, path: &Path) -> Result<Self, StorageError> {
        let profile: Self = serde_json::from_slice(&files.read(path)?)?;
        if profile.version != SETTINGS_VERSION {
            return Err(StorageError::UnsupportedVersion(profile.version));
        }
        Ok(profile)
    }

    pub fn export(&self, files: &impl Files, path: &Path) -> Result<(), StorageError> {
        let json = serde_json::to_string_pretty(self)?;
        Ok(files.write(path, json.as_bytes())?)
    }
}

/// File extension for a shared profile.
pub const PROFILE_EXTENSION: &str = "caoprofile";

/// The profile that always exists. Deleting the last one would leave nothing to
/// fall back to, so this one is never removed.
///
/// A key rather than a name: it is written into `settings.json`, and it is what
/// `remove` compares against to refuse. What the user reads is decided in
/// `cao_app`, so translating it moves no identity.
pub const DEFAULT_PROFILE: &str = "default";

#[cfg(test)]
mod tests {
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
}
