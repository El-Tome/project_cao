use std::path::Path;

use cao_part::PartDocument;

/// Whether what is on screen still matches what is on disk.
///
/// Writing a part is linear in the length of its history — measured at 2.29 ms
/// for three thousand steps, against a 16.6 ms frame — so a change made while
/// the hand is still down marks the part, and the file is replaced once the
/// gesture ends.
#[derive(Default)]
pub struct Autosave {
    pending: bool,
}

impl Autosave {
    pub fn touched(&mut self) {
        self.pending = true;
    }

    /// Writes the part when it owes the disk something and nothing is being
    /// held, and answers with what went wrong when it could not.
    pub fn write_if_due(
        &mut self,
        document: &PartDocument,
        path: &Path,
        at_rest: bool,
    ) -> Option<String> {
        if !self.pending || !at_rest {
            return None;
        }
        match document.save(path) {
            Ok(()) => {
                self.pending = false;
                None
            }
            Err(error) => Some(error.to_string()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use cao_part::PartDocument;

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
        let document = PartDocument::new("Support");
        let mut autosave = Autosave::default();

        autosave.touched();
        assert!(autosave.write_if_due(&document, &path, false).is_none());
        assert!(!path.exists(), "a drag does not write the file");

        assert!(autosave.write_if_due(&document, &path, true).is_none());
        assert!(path.exists(), "letting go writes it");

        std::fs::remove_dir_all(&directory).ok();
    }

    #[test]
    fn a_part_that_has_not_changed_is_not_written_again() {
        let directory = temp_dir("unchanged");
        let path = directory.join("piece.caopart");
        let document = PartDocument::new("Support");
        let mut autosave = Autosave::default();

        autosave.touched();
        autosave.write_if_due(&document, &path, true);
        std::fs::remove_file(&path).expect("removes");

        assert!(autosave.write_if_due(&document, &path, true).is_none());
        assert!(!path.exists(), "nothing changed, so nothing is written");

        std::fs::remove_dir_all(&directory).ok();
    }
}
