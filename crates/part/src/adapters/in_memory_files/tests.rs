//! What part · adapters/in_memory_files.rs is held to.

use std::path::Path;

use super::*;

#[test]
fn a_file_reads_back_exactly_what_was_written() {
    let files = InMemoryFiles::default();
    files
        .write(Path::new("/parts/support.caopart"), b"PK bytes")
        .expect("writes");

    assert_eq!(
        files
            .read(Path::new("/parts/support.caopart"))
            .expect("reads"),
        b"PK bytes"
    );
    assert!(files.exists(Path::new("/parts/support.caopart")));
}

#[test]
fn a_second_write_replaces_the_first_whole() {
    let files = InMemoryFiles::default();
    let path = Path::new("/parts/support.caopart");
    files.write(path, b"the first one").expect("writes");
    files.write(path, b"the second").expect("writes again");

    assert_eq!(files.read(path).expect("reads"), b"the second");
}

#[test]
fn reading_what_was_never_written_says_so() {
    let files = InMemoryFiles::default();
    let error = files
        .read(Path::new("/parts/absent.caopart"))
        .expect_err("nothing there");

    assert!(matches!(error, FileError::Absent(_)));
    assert!(!files.exists(Path::new("/parts/absent.caopart")));
}

#[test]
fn a_written_file_makes_the_folders_above_it_exist() {
    let files = InMemoryFiles::default();
    files
        .write(Path::new("/parts/drafts/support.caopart"), b"PK")
        .expect("writes");

    assert_eq!(
        files.entries(Path::new("/parts")).expect("reads"),
        [Entry {
            path: "/parts/drafts".into(),
            folder: true,
        }],
    );
}

#[test]
fn a_folder_nobody_has_put_anything_in_still_shows_up() {
    let files = InMemoryFiles::default();
    files.create(Path::new("/parts/empty")).expect("creates");

    assert_eq!(
        files.entries(Path::new("/parts")).expect("reads"),
        [Entry {
            path: "/parts/empty".into(),
            folder: true,
        }],
    );
}

#[test]
fn renaming_a_folder_takes_what_it_held_with_it() {
    let files = InMemoryFiles::default();
    files
        .write(Path::new("/parts/drafts/support.caopart"), b"PK")
        .expect("writes");

    files
        .rename(Path::new("/parts/drafts"), Path::new("/parts/kept"))
        .expect("renames");

    assert!(files.exists(Path::new("/parts/kept/support.caopart")));
    assert!(!files.exists(Path::new("/parts/drafts")));
}

#[test]
fn discarding_a_folder_takes_what_it_held_with_it() {
    let files = InMemoryFiles::default();
    files
        .write(Path::new("/parts/drafts/support.caopart"), b"PK")
        .expect("writes");

    files.discard(Path::new("/parts/drafts")).expect("discards");

    assert!(!files.exists(Path::new("/parts/drafts/support.caopart")));
    assert!(
        files
            .entries(Path::new("/parts"))
            .expect("reads")
            .is_empty()
    );
}

#[test]
fn discarding_what_is_not_there_says_so_rather_than_passing_quietly() {
    let files = InMemoryFiles::default();

    let error = files
        .discard(Path::new("/parts/absent.caopart"))
        .expect_err("nothing there");

    assert!(matches!(error, FileError::Absent(_)));
}
