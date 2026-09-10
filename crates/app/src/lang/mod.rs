//! What a key stands for, in the language the user reads.
//!
//! `wording/` decides which key a domain case deserves; this decides what that
//! key says. One JSON file per language, so a translation is data somebody can
//! contribute without knowing Rust.

//! What a key says, in the language the user reads.

use std::collections::BTreeMap;
use std::path::Path;

use cao_prefs::Files;

/// The French entries, built into the binary rather than read from disk: it is
/// the fallback every other language leans on, so it has to exist before
/// anything has been loaded, and on a machine offering nowhere to load from.
const FRENCH: &str = include_str!("fr.json");

type Entries = BTreeMap<String, String>;

/// Every sentence the interface can say, in one language.
pub struct Catalogue {
    spoken: Entries,
}

impl Catalogue {
    pub fn french() -> Self {
        Self {
            spoken: entries_of(FRENCH),
        }
    }

    /// French, with `language` laid over it where it has something to say.
    ///
    /// A language file that is absent, unreadable or malformed leaves French
    /// standing rather than stopping the shell: the interface keeps speaking.
    pub fn load(files: &impl Files, at: &Path, language: &str) -> Self {
        let mut catalogue = Self::french();
        let Ok(bytes) = files.read(&at.join(format!("{language}.json"))) else {
            return catalogue;
        };
        let Ok(text) = String::from_utf8(bytes) else {
            return catalogue;
        };
        catalogue.spoken.extend(entries_of(&text));
        catalogue
    }

    /// What `key` says, or the key itself when nothing claims it — a missing
    /// entry is a developer's oversight, and showing the key names it.
    pub fn t(&self, key: &str) -> String {
        self.spoken
            .get(key)
            .cloned()
            .unwrap_or_else(|| key.to_string())
    }

    /// The same, with `{name}` holes filled in. A hole nobody names is left
    /// standing, so a translation that dropped one shows where it went.
    pub fn t_with(&self, key: &str, values: &[(&str, &str)]) -> String {
        let mut said = self.t(key);
        for (name, value) in values {
            said = said.replace(&format!("{{{name}}}"), value);
        }
        said
    }
}

/// A language file that will not parse leaves no entry behind. The embedded
/// French one is held to a test instead, since a broken build of it would
/// otherwise show up as an interface saying nothing but keys.
fn entries_of(json: &str) -> Entries {
    serde_json::from_str(json).unwrap_or_default()
}

#[cfg(test)]
mod tests {
    use cao_prefs::InMemoryFiles;

    use super::*;

    #[test]
    fn the_french_catalogue_says_what_a_key_stands_for() {
        assert_eq!(
            Catalogue::french().t("history.sketch"),
            "Esquisse — {plane}"
        );
    }

    #[test]
    fn a_named_hole_is_filled_with_what_the_caller_hands_over() {
        assert_eq!(
            Catalogue::french().t_with("history.sketch", &[("plane", "Plan XY")]),
            "Esquisse — Plan XY",
        );
    }

    #[test]
    fn a_key_the_active_language_never_translated_is_still_said_in_french() {
        let files = InMemoryFiles::default();
        files
            .write(
                Path::new("/config/lang/en.json"),
                br#"{"history.sketch": "Sketch on {plane}"}"#,
            )
            .expect("writes");

        let catalogue = Catalogue::load(&files, Path::new("/config/lang"), "en");

        assert_eq!(
            catalogue.t_with("history.sketch", &[("plane", "XY plane")]),
            "Sketch on XY plane",
        );
        assert_eq!(catalogue.t("history.point"), "Point");
    }

    #[test]
    fn a_language_file_that_is_absent_or_broken_leaves_french_standing() {
        let files = InMemoryFiles::default();
        files
            .write(Path::new("/config/lang/de.json"), b"{ this is not json")
            .expect("writes");

        for language in ["de", "it"] {
            let catalogue = Catalogue::load(&files, Path::new("/config/lang"), language);

            assert_eq!(
                catalogue.t("history.point"),
                "Point",
                "{language} falls back"
            );
        }
    }

    #[test]
    fn the_embedded_french_file_parses_and_is_not_empty() {
        assert!(!entries_of(FRENCH).is_empty());
    }
}
