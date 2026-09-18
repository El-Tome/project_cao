use std::path::Path;

use cao_part::{Files, PartDocument};
use chrono::Utc;

use crate::lang::Catalogue;

/// Whether what is on screen still matches what is on disk.
///
/// Writing a part is linear in the length of its history — measured at 2.29 ms
/// for three thousand steps, against a 16.6 ms frame — so a change made while
/// the hand is still down marks the part, and the file is replaced once the
/// gesture ends. The geometry it has come to does not go in there: it is
/// written once, when the part is put away.
///
/// Which is why two things are owed and not one. A gesture clears what the
/// design owes and leaves the geometry owed, since it did not write any; only
/// a put-away clears that. Without the second, a part whose last gesture
/// settled before it was closed kept the archive that gesture wrote — the
/// design alone — and opened by replaying it ever after.
///
/// A part merely looked at owes neither, and is left exactly as it was. That
/// matters: writing it would move the hour it carries, and the files panel
/// sorts on that.
#[derive(Default)]
pub struct Autosave {
    pending: bool,
    geometry_owed: bool,
}

impl Autosave {
    pub fn touched(&mut self) {
        self.pending = true;
        self.geometry_owed = true;
    }

    /// Writes the part when it owes the disk something and nothing is being
    /// held, and answers with what went wrong when it could not.
    pub fn write_if_due(
        &mut self,
        files: &impl Files,
        document: &PartDocument,
        path: &Path,
        at_rest: bool,
        lang: &Catalogue,
    ) -> Option<String> {
        if !self.pending || !at_rest {
            return None;
        }
        self.written(document.save(files, path, Utc::now()), lang)
    }

    /// Writes the part on the way out of it, geometry and all, when it owes
    /// the disk anything — the design, the geometry, or both.
    pub fn put_away_if_due(
        &mut self,
        files: &impl Files,
        document: &PartDocument,
        path: &Path,
        lang: &Catalogue,
    ) -> Option<String> {
        if !self.pending && !self.geometry_owed {
            return None;
        }
        let outcome = document.put_away(files, path, Utc::now());
        if outcome.is_ok() {
            self.geometry_owed = false;
        }
        self.written(outcome, lang)
    }

    fn written(
        &mut self,
        outcome: Result<(), cao_part::PartFileError>,
        lang: &Catalogue,
    ) -> Option<String> {
        match outcome {
            Ok(()) => {
                self.pending = false;
                None
            }
            Err(error) => Some(crate::wording::part_file::say(lang, &error)),
        }
    }
}

#[cfg(test)]
mod tests;
