//! What prefs · adapters/in_memory_files.rs is held to.

use super::*;

#[test]
fn a_file_reads_back_exactly_what_was_written() {
    let files = InMemoryFiles::default();
    files
        .write(Path::new("/config/settings.json"), b"{}")
        .expect("writes");

    assert_eq!(
        files
            .read(Path::new("/config/settings.json"))
            .expect("reads"),
        b"{}"
    );
    assert!(files.exists(Path::new("/config/settings.json")));
}

#[test]
fn a_second_write_replaces_the_first_whole() {
    let files = InMemoryFiles::default();
    let path = Path::new("/config/settings.json");
    files.write(path, b"the first one").expect("writes");
    files.write(path, b"the second").expect("writes again");

    assert_eq!(files.read(path).expect("reads"), b"the second");
}

#[test]
fn reading_what_was_never_written_says_so() {
    let files = InMemoryFiles::default();
    let error = files
        .read(Path::new("/config/absent.json"))
        .expect_err("nothing there");

    assert!(matches!(error, FileError::Absent(_)));
    assert!(!files.exists(Path::new("/config/absent.json")));
}
