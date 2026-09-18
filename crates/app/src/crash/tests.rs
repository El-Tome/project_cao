//! What app · crash.rs is held to.

use super::*;

#[test]
fn the_journal_sits_beside_the_data_the_platform_gave() {
    let at = Locations {
        config: "/config".into(),
        data: "/data".into(),
        documents: None,
    };

    assert_eq!(crash_log_path(&at), PathBuf::from("/data/plantages.log"));
}

#[test]
fn a_crash_is_written_down_with_its_hour_and_its_stack() {
    // A run of the suite next to this one must not be writing the same
    // file: the two would read each other's entries.
    let folder = std::env::temp_dir().join(format!("cao-essai-plantage-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&folder);
    let path = folder.join("plantages.log");

    append_crash(&path, "premier essai").expect("the file is written");
    append_crash(&path, "second essai").expect("it is written again");

    let written = std::fs::read_to_string(&path).expect("the journal is there");
    assert!(written.contains("premier essai"));
    assert!(
        written.contains("second essai"),
        "the second is added to the first",
    );
    assert!(written.contains("append_crash"), "with the call stack");
    let _ = std::fs::remove_dir_all(&folder);
}
