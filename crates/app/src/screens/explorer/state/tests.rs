use std::path::Path;

use cao_part::{Files, InMemoryFiles, PartDocument, PartFileError};

use super::*;

fn library_of(parts: &[&str]) -> InMemoryFiles {
    let files = InMemoryFiles::default();
    for path in parts {
        files.write(Path::new(path), b"PK").expect("writes");
    }
    files
}

fn panel_on(files: &InMemoryFiles) -> Explorer {
    let mut explorer = Explorer::at("/CAO".into());
    explorer.refresh(files);
    explorer
}

/// Types `name` into the field the panel has open, the way the view does:
/// the naming is taken out, written to, and handed back.
fn typing(explorer: &mut Explorer, name: &str) {
    let mut naming = explorer.naming().cloned().expect("a name is asked for");
    *naming.typed_mut() = name.into();
    explorer.set_naming(Some(naming));
}

fn written_part(files: &InMemoryFiles, inside: &str, name: &str) -> PathBuf {
    let (_, path) = PartDocument::create_in(
        files,
        Path::new(inside),
        name,
        "2026-01-02T09:00:00Z".parse().expect("a date"),
    )
    .expect("the part is written");
    path
}

#[test]
fn a_panel_just_opened_shows_the_folders_and_the_parts_of_the_library() {
    let files = library_of(&["/CAO/support.caopart", "/CAO/drafts/bride.caopart"]);

    let explorer = panel_on(&files);

    assert_eq!(explorer.library().parts[0].name, "support");
    assert_eq!(explorer.library().folders[0].name, "drafts");
}

#[test]
fn a_folder_is_open_to_begin_with_and_stays_shut_once_folded() {
    let files = library_of(&["/CAO/drafts/bride.caopart"]);
    let mut explorer = panel_on(&files);
    let drafts = Path::new("/CAO/drafts");

    assert!(explorer.unfolded(drafts), "the panel opens showing nothing");
    explorer.fold(drafts, false);

    assert!(!explorer.unfolded(drafts));
}

#[test]
fn a_new_folder_lands_inside_the_folder_selected() {
    let files = library_of(&["/CAO/drafts/bride.caopart"]);
    let mut explorer = panel_on(&files);
    explorer.select(Path::new("/CAO/drafts"));

    explorer.start_new_folder();
    typing(&mut explorer, "Kept");
    explorer.confirm_naming(&files, &files);

    assert_eq!(explorer.selected(), Some(Path::new("/CAO/drafts/Kept")));
    assert_eq!(explorer.library().folders[0].folders[0].name, "Kept");
}

#[test]
fn a_new_folder_lands_beside_the_part_selected_rather_than_inside_it() {
    let files = library_of(&["/CAO/drafts/bride.caopart"]);
    let mut explorer = panel_on(&files);
    explorer.select(Path::new("/CAO/drafts/bride.caopart"));

    explorer.start_new_folder();
    typing(&mut explorer, "Kept");
    explorer.confirm_naming(&files, &files);

    assert_eq!(explorer.selected(), Some(Path::new("/CAO/drafts/Kept")));
}

#[test]
fn a_new_folder_lands_at_the_root_when_nothing_is_selected() {
    let files = library_of(&["/CAO/support.caopart"]);
    let mut explorer = panel_on(&files);

    explorer.start_new_folder();
    typing(&mut explorer, "Brides");
    explorer.confirm_naming(&files, &files);

    assert_eq!(explorer.selected(), Some(Path::new("/CAO/Brides")));
}

#[test]
fn a_name_already_taken_keeps_the_field_open_rather_than_dropping_what_was_typed() {
    let files = library_of(&["/CAO/drafts/bride.caopart"]);
    let mut explorer = panel_on(&files);

    explorer.start_new_folder();
    typing(&mut explorer, "drafts");
    explorer.confirm_naming(&files, &files);

    assert!(matches!(
        explorer.trouble(),
        Some(PartFileError::NameTaken(_))
    ));
    assert!(
        explorer.naming().is_some(),
        "the user has to type the whole name again",
    );
}

#[test]
fn renaming_a_part_leaves_the_selection_on_it_under_its_new_name() {
    let files = InMemoryFiles::default();
    let path = written_part(&files, "/CAO", "Support");
    let mut explorer = panel_on(&files);
    explorer.select(&path);

    explorer.start_rename();
    typing(&mut explorer, "Bride");
    explorer.confirm_naming(&files, &files);

    assert_eq!(explorer.selected(), Some(Path::new("/CAO/Bride.caopart")));
    assert_eq!(explorer.library().parts[0].name, "Bride");
}

#[test]
fn a_rename_starts_from_the_name_the_thing_already_has() {
    let files = library_of(&["/CAO/support.caopart"]);
    let mut explorer = panel_on(&files);
    explorer.select(Path::new("/CAO/support.caopart"));

    explorer.start_rename();

    assert_eq!(
        explorer.naming(),
        Some(&Naming::Rename {
            path: "/CAO/support.caopart".into(),
            typed: "support".into(),
        }),
    );
}

#[test]
fn the_part_this_window_is_drawing_cannot_be_renamed_from_the_panel() {
    let files = library_of(&["/CAO/support.caopart"]);
    let mut explorer = panel_on(&files);
    explorer.drawing(Some(Path::new("/CAO/support.caopart")));
    explorer.select(Path::new("/CAO/support.caopart"));

    explorer.start_rename();

    assert!(
        explorer.naming().is_none(),
        "the part being written to behind the panel's back is renamed under it",
    );
    assert!(!explorer.may_tidy());
}

#[test]
fn the_part_this_window_is_drawing_cannot_be_thrown_away_from_the_panel() {
    let files = library_of(&["/CAO/support.caopart"]);
    let mut explorer = panel_on(&files);
    explorer.drawing(Some(Path::new("/CAO/support.caopart")));
    explorer.select(Path::new("/CAO/support.caopart"));

    explorer.ask_to_discard();

    assert!(explorer.confirming().is_none());
}

#[test]
fn the_part_this_window_is_drawing_is_not_opened_a_second_time() {
    let files = library_of(&["/CAO/support.caopart", "/CAO/bride.caopart"]);
    let mut explorer = panel_on(&files);
    explorer.drawing(Some(Path::new("/CAO/support.caopart")));

    assert_eq!(
        explorer.asked_to_open(Path::new("/CAO/support.caopart")),
        None,
        "a second window on the same file would give it two autosaves",
    );
    assert_eq!(
        explorer.asked_to_open(Path::new("/CAO/bride.caopart")),
        Some(PathBuf::from("/CAO/bride.caopart")),
    );
}

#[test]
fn the_folder_holding_the_part_this_window_is_drawing_cannot_be_thrown_away_either() {
    let files = library_of(&["/CAO/drafts/support.caopart"]);
    let mut explorer = panel_on(&files);
    explorer.drawing(Some(Path::new("/CAO/drafts/support.caopart")));
    explorer.select(Path::new("/CAO/drafts"));

    explorer.ask_to_discard();

    assert!(
        explorer.confirming().is_none(),
        "the next autosave would make the folder and that one part again, and \
         leave everything else it held in the bin",
    );
    assert!(!explorer.may_tidy());
}

#[test]
fn a_panel_is_showing_before_anybody_asks_for_it() {
    assert!(Explorer::at("/CAO".into()).is_open());
}

#[test]
fn opening_the_panel_reads_the_library_again() {
    let files = library_of(&["/CAO/support.caopart"]);
    let mut explorer = panel_on(&files);
    explorer.toggle();
    files
        .write(Path::new("/CAO/bride.caopart"), b"PK")
        .expect("another window writes a part");

    explorer.toggle();
    explorer.refresh_if_stale(&files);

    assert!(explorer.is_open());
    assert_eq!(explorer.library().parts.len(), 2);
}

fn part_with_a_picture(files: &InMemoryFiles, inside: &str, name: &str) -> PathBuf {
    let path = written_part(files, inside, name);
    let mut document = PartDocument::load(files, &path).expect("reads");
    document.set_picture(cao_part::Picture::new(2, 2, (0..16).collect()).expect("a picture"));
    document
        .save(
            files,
            &path,
            "2026-01-02T09:00:00Z".parse().expect("a date"),
        )
        .expect("the part is written");
    path
}

#[test]
fn only_the_parts_a_row_was_drawn_for_are_read_off_the_disk() {
    let files = InMemoryFiles::default();
    let shown = part_with_a_picture(&files, "/CAO", "Support");
    let unseen = part_with_a_picture(&files, "/CAO", "Bride");
    let mut explorer = panel_on(&files);

    explorer.wants_pictures_of(vec![shown.clone()]);
    explorer.read_pictures(&files);

    assert!(explorer.picture_of(&shown).is_some());
    assert!(
        explorer.picture_of(&unseen).is_none(),
        "a folder of a thousand parts would be a thousand reads and a thousand textures",
    );
}

#[test]
fn a_part_that_carries_no_picture_is_not_read_again_the_next_frame() {
    let files = InMemoryFiles::default();
    let bare = written_part(&files, "/CAO", "Support");
    let mut explorer = panel_on(&files);

    explorer.wants_pictures_of(vec![bare.clone()]);
    explorer.read_pictures(&files);
    files
        .write(Path::new("/CAO/Support.caopart"), b"not an archive at all")
        .expect("writes");
    explorer.wants_pictures_of(vec![bare.clone()]);
    explorer.read_pictures(&files);

    assert!(
        explorer.picture_of(&bare).is_none(),
        "having none is remembered, so the archive is opened once and not once a frame",
    );
}

#[test]
fn reading_the_library_again_forgets_the_pictures_it_had_read() {
    let files = InMemoryFiles::default();
    let part = part_with_a_picture(&files, "/CAO", "Support");
    let mut explorer = panel_on(&files);
    explorer.wants_pictures_of(vec![part.clone()]);
    explorer.read_pictures(&files);

    explorer.refresh(&files);

    assert!(
        explorer.picture_of(&part).is_none(),
        "a part renamed or drawn on since would keep showing what it used to look like",
    );
}

#[test]
fn nothing_is_thrown_away_before_the_question_is_answered() {
    let files = library_of(&["/CAO/support.caopart"]);
    let mut explorer = panel_on(&files);
    explorer.select(Path::new("/CAO/support.caopart"));

    explorer.ask_to_discard();

    assert_eq!(
        explorer.confirming(),
        Some(Path::new("/CAO/support.caopart"))
    );
    assert_eq!(explorer.library().parts.len(), 1, "it is already gone");
}

#[test]
fn a_part_thrown_away_leaves_the_library_and_the_selection() {
    let files = library_of(&["/CAO/support.caopart"]);
    let mut explorer = panel_on(&files);
    explorer.select(Path::new("/CAO/support.caopart"));

    explorer.ask_to_discard();
    explorer.discard(&files);

    assert!(explorer.library().parts.is_empty());
    assert_eq!(explorer.selected(), None);
}

#[test]
fn a_question_answered_no_leaves_the_part_where_it_is() {
    let files = library_of(&["/CAO/support.caopart"]);
    let mut explorer = panel_on(&files);
    explorer.select(Path::new("/CAO/support.caopart"));

    explorer.ask_to_discard();
    explorer.cancel_discard();
    explorer.discard(&files);

    assert_eq!(explorer.library().parts.len(), 1);
}
