//! What app · autosave.rs is held to.

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
