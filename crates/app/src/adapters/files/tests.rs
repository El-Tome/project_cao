//! What app · adapters/files.rs is held to.

use super::*;

fn temp_dir() -> std::path::PathBuf {
    let directory = std::env::temp_dir().join(format!(
        "cao_files_{}_{}",
        std::process::id(),
        NEXT_DIRECTORY.fetch_add(1, Ordering::Relaxed),
    ));
    fs::create_dir_all(&directory).expect("temp dir");
    directory
}

static NEXT_DIRECTORY: AtomicU64 = AtomicU64::new(0);

#[test]
fn a_write_makes_the_folder_it_lands_in_and_reads_back_whole() {
    let root = temp_dir();
    let path = root.join("parts").join("drafts").join("piece.caopart");

    assert!(!DiskFiles.exists(&path));
    DiskFiles.write(&path, b"the whole thing").expect("writes");

    assert!(DiskFiles.exists(&path));
    assert_eq!(DiskFiles.read(&path).expect("reads"), b"the whole thing");

    fs::remove_dir_all(&root).ok();
}

#[test]
fn reading_a_file_that_was_never_written_says_it_is_absent() {
    let root = temp_dir();

    let error = DiskFiles
        .read(&root.join("nothing.caopart"))
        .expect_err("nothing there");

    assert!(matches!(error, FileError::Absent(_)));

    fs::remove_dir_all(&root).ok();
}

#[test]
fn a_folder_tells_the_folders_it_holds_from_the_files() {
    let root = temp_dir();
    DiskFiles
        .write(&root.join("support.caopart"), b"PK")
        .expect("writes");
    DiskFiles.create(&root.join("drafts")).expect("creates");

    let mut entries = DiskFiles.entries(&root).expect("reads");
    entries.sort();

    assert_eq!(
        entries,
        [
            Entry {
                path: root.join("drafts"),
                folder: true,
            },
            Entry {
                path: root.join("support.caopart"),
                folder: false,
            },
        ],
    );

    fs::remove_dir_all(&root).ok();
}

#[test]
fn a_library_the_user_has_not_started_yet_reads_as_empty_rather_than_failing() {
    let root = temp_dir();

    let entries = DiskFiles
        .entries(&root.join("nothing here"))
        .expect("reads");

    assert!(entries.is_empty());

    fs::remove_dir_all(&root).ok();
}

#[test]
fn a_renamed_folder_takes_the_parts_it_held_with_it() {
    let root = temp_dir();
    DiskFiles
        .write(&root.join("drafts").join("support.caopart"), b"PK")
        .expect("writes");

    DiskFiles
        .rename(&root.join("drafts"), &root.join("kept"))
        .expect("renames");

    assert!(DiskFiles.exists(&root.join("kept").join("support.caopart")));
    assert!(!DiskFiles.exists(&root.join("drafts")));

    fs::remove_dir_all(&root).ok();
}

#[test]
fn a_write_that_stops_partway_leaves_the_previous_content_intact() {
    let directory = temp_dir();
    let path = directory.join("piece.caopart");
    DiskFiles.write(&path, b"the whole thing").expect("writes");

    let error = replace_whole(&path, |file| {
        file.write_all(b"half of it")?;
        Err(std::io::Error::other("the write stops here"))
    })
    .expect_err("the replacement must fail");

    assert_eq!(error.to_string(), "the write stops here");
    assert_eq!(fs::read(&path).expect("bytes"), b"the whole thing");
    assert_eq!(
        fs::read_dir(&directory).expect("directory").count(),
        1,
        "nothing is left beside it",
    );

    fs::remove_dir_all(&directory).ok();
}
