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
use chrono::{DateTime, FixedOffset, TimeZone, Utc};

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

    /// The moment `at`, shaped by the `chrono` pattern `key` holds and read on
    /// the clock of `zone`: the language file says how a moment is written, and
    /// it is the machine, not the file, that says which hour it is.
    pub fn t_moment(&self, key: &str, at: DateTime<Utc>, zone: &impl TimeZone) -> String {
        let at = at.with_timezone(zone).fixed_offset();
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
fn shaped(pattern: &str, at: DateTime<FixedOffset>) -> Option<String> {
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
mod tests;
