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
mod tests {
    use super::*;
    use crate::adapters::files::DiskFiles;
    use cao_part::PartDocument;

    fn at(text: &str) -> chrono::DateTime<Utc> {
        text.parse().expect("a date")
    }

    fn temp_dir(name: &str) -> std::path::PathBuf {
        let directory = std::env::temp_dir().join(format!("cao_autosave_{name}"));
        std::fs::remove_dir_all(&directory).ok();
        std::fs::create_dir_all(&directory).expect("temp dir");
        directory
    }

    #[test]
    fn a_part_is_written_once_the_hand_comes_off() {
        let directory = temp_dir("hand_off");
        let path = directory.join("piece.caopart");
        let document = PartDocument::new("Support", at("2026-01-02T09:00:00Z"));
        let mut autosave = Autosave::default();

        autosave.touched();
        assert!(
            autosave
                .write_if_due(&DiskFiles, &document, &path, false, &Catalogue::french())
                .is_none()
        );
        assert!(!path.exists(), "a drag does not write the file");

        assert!(
            autosave
                .write_if_due(&DiskFiles, &document, &path, true, &Catalogue::french())
                .is_none()
        );
        assert!(path.exists(), "letting go writes it");

        std::fs::remove_dir_all(&directory).ok();
    }

    #[test]
    fn a_part_on_the_way_out_is_written_and_owes_the_disk_nothing_after() {
        let directory = temp_dir("put_away");
        let path = directory.join("piece.caopart");
        let document = PartDocument::new("Support", at("2026-01-02T09:00:00Z"));
        let mut autosave = Autosave::default();

        autosave.touched();
        assert!(
            autosave
                .put_away_if_due(&DiskFiles, &document, &path, &Catalogue::french())
                .is_none()
        );
        assert!(path.exists(), "the way out writes it, hand down or not");

        std::fs::remove_file(&path).expect("removes");
        assert!(
            autosave
                .put_away_if_due(&DiskFiles, &document, &path, &Catalogue::french())
                .is_none()
        );
        assert!(!path.exists(), "nothing is owed, so nothing is written");

        std::fs::remove_dir_all(&directory).ok();
    }

    #[test]
    fn a_part_written_by_a_gesture_is_written_again_on_the_way_out_for_its_geometry() {
        let directory = temp_dir("gesture_then_out");
        let path = directory.join("piece.caopart");
        let document = PartDocument::new("Support", at("2026-01-02T09:00:00Z"));
        let mut autosave = Autosave::default();

        autosave.touched();
        autosave.write_if_due(&DiskFiles, &document, &path, true, &Catalogue::french());
        std::fs::remove_file(&path).expect("removes");

        assert!(
            autosave
                .put_away_if_due(&DiskFiles, &document, &path, &Catalogue::french())
                .is_none()
        );
        assert!(
            path.exists(),
            "a gesture writes the design alone, so the geometry is still owed \
             the moment the part is put away",
        );

        std::fs::remove_file(&path).expect("removes");
        autosave.put_away_if_due(&DiskFiles, &document, &path, &Catalogue::french());
        assert!(
            !path.exists(),
            "the geometry has just been written, and nothing has happened since",
        );

        std::fs::remove_dir_all(&directory).ok();
    }

    #[test]
    fn a_part_that_has_not_changed_is_not_written_again() {
        let directory = temp_dir("unchanged");
        let path = directory.join("piece.caopart");
        let document = PartDocument::new("Support", at("2026-01-02T09:00:00Z"));
        let mut autosave = Autosave::default();

        autosave.touched();
        autosave.write_if_due(&DiskFiles, &document, &path, true, &Catalogue::french());
        std::fs::remove_file(&path).expect("removes");

        assert!(
            autosave
                .write_if_due(&DiskFiles, &document, &path, true, &Catalogue::french())
                .is_none()
        );
        assert!(!path.exists(), "nothing changed, so nothing is written");

        std::fs::remove_dir_all(&directory).ok();
    }
}
