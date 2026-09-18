//! What part · library/tidying.rs is held to.

use super::*;

use crate::adapters::InMemoryFiles;
use crate::library;
use chrono::{DateTime, Utc};

fn at(text: &str) -> DateTime<Utc> {
    text.parse().expect("a date")
}

fn library_with(name: &str) -> (InMemoryFiles, PathBuf) {
    let files = InMemoryFiles::default();
    let (_, path) =
        PartDocument::create_in(&files, Path::new("/CAO"), name, at("2026-01-02T09:00:00Z"))
            .expect("the part is written");
    (files, path)
}

#[test]
fn a_new_folder_shows_up_where_it_was_asked_for() {
    let files = InMemoryFiles::default();

    let made = create_folder(&files, Path::new("/CAO"), "Brides").expect("the folder is made");

    assert_eq!(made, PathBuf::from("/CAO/Brides"));
    assert_eq!(
        library::read(&files, Path::new("/CAO"))
            .expect("reads")
            .folders[0]
            .name,
        "Brides",
    );
}

#[test]
fn a_folder_asked_for_twice_is_refused_rather_than_merged_into_the_first() {
    let files = InMemoryFiles::default();
    create_folder(&files, Path::new("/CAO"), "Brides").expect("the folder is made");

    let again = create_folder(&files, Path::new("/CAO"), "Brides");

    assert!(matches!(again, Err(PartFileError::NameTaken(_))));
}

#[test]
fn a_folder_named_with_blanks_alone_is_refused() {
    let files = InMemoryFiles::default();

    let made = create_folder(&files, Path::new("/CAO"), "   ");

    assert!(matches!(made, Err(PartFileError::BlankName)));
}

#[test]
fn a_renamed_folder_keeps_the_parts_it_held() {
    let files = InMemoryFiles::default();
    PartDocument::create_in(
        &files,
        Path::new("/CAO/drafts"),
        "Support",
        at("2026-01-02T09:00:00Z"),
    )
    .expect("the part is written");

    let moved =
        rename_folder(&files, Path::new("/CAO/drafts"), "Kept").expect("the folder is renamed");

    assert_eq!(moved, PathBuf::from("/CAO/Kept"));
    let library = library::read(&files, Path::new("/CAO")).expect("reads");
    assert_eq!(library.folders[0].name, "Kept");
    assert_eq!(library.folders[0].parts[0].name, "Support");
}

#[test]
fn a_renamed_part_answers_to_the_new_name_inside_the_archive_too() {
    let (files, path) = library_with("Support");

    let moved = rename_part(&files, &files, &path, "Bride").expect("the part is renamed");

    assert_eq!(moved, PathBuf::from("/CAO/Bride.caopart"));
    assert_eq!(
        PartDocument::load(&files, &moved).expect("reads").name(),
        "Bride",
        "the panel would read one name and the title bar another",
    );
}

#[test]
fn a_renamed_part_leaves_nothing_behind_under_the_old_name() {
    let (files, path) = library_with("Support");

    rename_part(&files, &files, &path, "Bride").expect("the part is renamed");

    let library = library::read(&files, Path::new("/CAO")).expect("reads");
    let names: Vec<&str> = library
        .parts
        .iter()
        .map(|part| part.name.as_str())
        .collect();
    assert_eq!(names, ["Bride"]);
}

#[test]
fn renaming_a_part_is_not_working_on_it_and_leaves_its_hour_alone() {
    let (files, path) = library_with("Support");
    let before = PartDocument::load(&files, &path)
        .expect("reads")
        .metadata
        .modified_at;

    let moved = rename_part(&files, &files, &path, "Bride").expect("the part is renamed");

    assert_eq!(
        PartDocument::load(&files, &moved)
            .expect("reads")
            .metadata
            .modified_at,
        before,
    );
}

#[test]
fn a_part_renamed_onto_one_already_there_is_refused_rather_than_written_over() {
    let (files, path) = library_with("Support");
    PartDocument::create_in(
        &files,
        Path::new("/CAO"),
        "Bride",
        at("2026-01-02T09:00:00Z"),
    )
    .expect("the part is written");

    let moved = rename_part(&files, &files, &path, "Bride");

    assert!(matches!(moved, Err(PartFileError::NameTaken(_))));
    assert!(
        PartDocument::load(&files, &path).is_ok(),
        "the part that was to be renamed is gone",
    );
}

#[test]
fn a_part_renamed_to_the_name_it_already_has_is_left_where_it_is() {
    let (files, path) = library_with("Support");

    let moved = rename_part(&files, &files, &path, "Support").expect("nothing to do");

    assert_eq!(moved, path);
    assert!(PartDocument::load(&files, &path).is_ok());
}

#[test]
fn a_renamed_part_is_moved_rather_than_copied_and_thrown_away() {
    let (files, path) = library_with("Support");

    rename_part(&files, &files, &path, "Bride").expect("the part is renamed");

    assert!(
        !files.exists(Path::new("/CAO/Support.caopart")),
        "a copy of the part is left behind at every rename",
    );
}

#[test]
fn a_folder_whose_name_differs_only_in_case_is_refused() {
    let files = InMemoryFiles::default();
    create_folder(&files, Path::new("/CAO"), "Brides").expect("the folder is made");

    let again = create_folder(&files, Path::new("/CAO"), "brides");

    assert!(
        matches!(again, Err(PartFileError::NameTaken(_))),
        "a filesystem that ignores case would hand back the folder already there",
    );
}

#[test]
fn a_part_can_be_renamed_to_the_name_it_has_in_another_case() {
    let (files, path) = library_with("Support");

    let moved = rename_part(&files, &files, &path, "support").expect("the part is renamed");

    assert_eq!(moved, PathBuf::from("/CAO/support.caopart"));
    assert_eq!(
        PartDocument::load(&files, &moved).expect("reads").name(),
        "support",
    );
}

#[test]
fn a_renamed_part_keeps_its_picture() {
    let (files, path) = library_with("Support");
    let mut document = PartDocument::load(&files, &path).expect("reads");
    let picture = crate::Picture::new(2, 2, (0..16).collect()).expect("a picture");
    document.set_picture(picture.clone());
    document
        .save(&files, &path, at("2026-01-02T09:00:00Z"))
        .expect("the part is written");

    let moved = rename_part(&files, &files, &path, "Bride").expect("the part is renamed");

    assert_eq!(
        PartDocument::load(&files, &moved).expect("reads").picture(),
        Some(&picture),
        "a rename reads the part and writes it back, and would drop it",
    );
}

#[test]
fn a_part_thrown_away_is_gone_from_the_library() {
    let (files, path) = library_with("Support");

    discard(&files, &path).expect("the part is thrown away");

    assert!(
        library::read(&files, Path::new("/CAO"))
            .expect("reads")
            .parts
            .is_empty()
    );
}
