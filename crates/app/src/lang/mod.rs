//! What a key stands for, in the language the user reads.
//!
//! `wording/` decides which key a domain case deserves; this decides what that
//! key says. One JSON file per language, so a translation is data somebody can
//! contribute without knowing Rust.

//! What a key says, in the language the user reads.

use std::collections::BTreeMap;
use std::fmt::Write;
use std::path::Path;

use cao_prefs::Files;
use chrono::format::{Item, StrftimeItems};
use chrono::{DateTime, Utc};

/// The French entries, built into the binary rather than read from disk: it is
/// the fallback every other language leans on, so it has to exist before
/// anything has been loaded, and on a machine offering nowhere to load from.
const FRENCH: &str = include_str!("fr.json");

/// How a moment is written when the language file hands over no usable
/// pattern — an entry nobody wrote, since `t` answers with the key itself; a
/// word standing where a pattern belongs; a pattern no clock can read. All
/// three would otherwise show up where the reader expects a date.
const A_MOMENT_NOBODY_SHAPED: &str = "%Y-%m-%d %H:%M";

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

    /// The moment `at`, shaped by the `chrono` pattern `key` holds.
    pub fn t_moment(&self, key: &str, at: DateTime<Utc>) -> String {
        shaped(&self.t(key), at).unwrap_or_else(|| at.format(A_MOMENT_NOBODY_SHAPED).to_string())
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

/// Nothing, unless `pattern` both reads as `strftime` and names something a
/// clock or a calendar answers — text alone is a mistake, never a date.
fn shaped(pattern: &str, at: DateTime<Utc>) -> Option<String> {
    let items = StrftimeItems::new(pattern).parse().ok()?;
    if !items
        .iter()
        .any(|item| matches!(item, Item::Numeric(..) | Item::Fixed(_)))
    {
        return None;
    }

    let mut said = String::new();
    write!(said, "{}", at.format_with_items(items.iter())).ok()?;
    Some(said)
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
    use chrono::TimeZone;

    use super::*;

    fn a_moment() -> DateTime<Utc> {
        Utc.with_ymd_and_hms(2026, 9, 11, 14, 30, 0)
            .single()
            .expect("one moment answers")
    }

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
    fn a_moment_is_written_the_way_the_french_file_shapes_it() {
        assert_eq!(
            Catalogue::french().t_moment("start_menu.recent_date", a_moment()),
            "11/09/2026 14:30",
        );
    }

    #[test]
    fn a_pattern_naming_no_part_of_a_date_still_leaves_a_date_standing() {
        let files = InMemoryFiles::default();
        files
            .write(
                Path::new("/config/lang/en.json"),
                br#"{"start_menu.recent_date": "hier"}"#,
            )
            .expect("writes");
        let catalogue = Catalogue::load(&files, Path::new("/config/lang"), "en");

        assert_eq!(
            catalogue.t_moment("start_menu.recent_date", a_moment()),
            "2026-09-11 14:30",
        );
    }

    #[test]
    fn a_key_no_entry_claims_leaves_a_date_standing_where_it_would_show_itself() {
        let never_written = ["start_menu", "recent_date", "unwritten"].join(".");

        assert_eq!(
            Catalogue::french().t_moment(&never_written, a_moment()),
            "2026-09-11 14:30",
        );
    }

    #[test]
    fn a_pattern_no_clock_can_read_leaves_a_date_standing() {
        let files = InMemoryFiles::default();
        files
            .write(
                Path::new("/config/lang/en.json"),
                br#"{"start_menu.recent_date": "%d/%"}"#,
            )
            .expect("writes");
        let catalogue = Catalogue::load(&files, Path::new("/config/lang"), "en");

        assert_eq!(
            catalogue.t_moment("start_menu.recent_date", a_moment()),
            "2026-09-11 14:30",
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
