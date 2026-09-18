//! What part · library/mod.rs is held to.

use super::*;

use crate::adapters::InMemoryFiles;
use crate::ports::Files;

fn library_of(paths: &[&str]) -> InMemoryFiles {
    let files = InMemoryFiles::default();
    for path in paths {
        files.write(Path::new(path), b"PK").expect("writes");
    }
    files
}

#[test]
fn a_folder_hands_back_the_parts_it_holds_and_leaves_the_rest_alone() {
    let files = library_of(&[
        "/CAO/support.caopart",
        "/CAO/notes.txt",
        "/CAO/bride.caopart",
    ]);

    let library = read(&files, Path::new("/CAO")).expect("reads");

    let names: Vec<&str> = library
        .parts
        .iter()
        .map(|part| part.name.as_str())
        .collect();
    assert_eq!(names, ["bride", "support"]);
}

#[test]
fn a_part_is_offered_under_the_name_its_file_carries_without_the_extension() {
    let files = library_of(&["/CAO/support 2.caopart"]);

    let library = read(&files, Path::new("/CAO")).expect("reads");

    assert_eq!(library.parts[0].name, "support 2");
    assert_eq!(
        library.parts[0].path,
        PathBuf::from("/CAO/support 2.caopart")
    );
}

#[test]
fn a_subfolder_comes_with_everything_under_it() {
    let files = library_of(&["/CAO/drafts/first/support.caopart"]);

    let library = read(&files, Path::new("/CAO")).expect("reads");

    assert_eq!(library.folders[0].name, "drafts");
    assert_eq!(library.folders[0].folders[0].name, "first");
    assert_eq!(library.folders[0].folders[0].parts[0].name, "support");
}

#[test]
fn folders_and_parts_each_come_in_the_order_a_reader_expects() {
    let files = library_of(&[
        "/CAO/zinc/part.caopart",
        "/CAO/Alpha/part.caopart",
        "/CAO/beta.caopart",
        "/CAO/Alpha.caopart",
    ]);

    let library = read(&files, Path::new("/CAO")).expect("reads");

    let folders: Vec<&str> = library
        .folders
        .iter()
        .map(|folder| folder.name.as_str())
        .collect();
    let parts: Vec<&str> = library
        .parts
        .iter()
        .map(|part| part.name.as_str())
        .collect();
    assert_eq!(
        folders,
        ["Alpha", "zinc"],
        "a capital does not send a folder to the top"
    );
    assert_eq!(parts, ["Alpha", "beta"]);
}

#[test]
fn a_library_nobody_has_started_reads_as_an_empty_one() {
    let library = read(&InMemoryFiles::default(), Path::new("/CAO")).expect("reads");

    assert!(library.folders.is_empty());
    assert!(library.parts.is_empty());
    assert_eq!(library.name, "CAO");
}
