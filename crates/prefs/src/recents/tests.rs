//! What prefs · recents.rs is held to.

use std::path::Path;

use super::*;
use crate::adapters::InMemoryFiles;

fn at_hour(text: &str) -> DateTime<Utc> {
    text.parse().expect("a date")
}

#[test]
fn a_recent_part_carries_the_hour_it_was_handed() {
    let opened = at_hour("2026-01-02T09:00:00Z");
    let mut recents = RecentList::default();

    recents.push("/parts/support.caopart", "Support", opened);

    assert_eq!(recents.entries()[0].opened_at, opened);
}

#[test]
fn the_part_just_opened_comes_first_and_only_once() {
    let mut recents = RecentList::default();
    recents.push(
        "/parts/support.caopart",
        "Support",
        at_hour("2026-01-02T09:00:00Z"),
    );
    recents.push(
        "/parts/bride.caopart",
        "Bride",
        at_hour("2026-01-02T09:01:00Z"),
    );
    recents.push(
        "/parts/support.caopart",
        "Support",
        at_hour("2026-01-02T09:02:00Z"),
    );

    let paths: Vec<&PathBuf> = recents.entries().iter().map(|e| &e.path).collect();
    assert_eq!(
        paths,
        [
            &PathBuf::from("/parts/support.caopart"),
            &PathBuf::from("/parts/bride.caopart"),
        ],
    );
}

#[test]
fn the_recent_parts_are_written_where_the_platform_says_and_read_back_from_there() {
    let at = Locations {
        config: "/config".into(),
        data: "/data".into(),
        documents: None,
    };
    let files = InMemoryFiles::default();
    let mut recents = RecentList::default();
    recents.push(
        "/parts/support.caopart",
        "Support",
        at_hour("2026-01-02T09:00:00Z"),
    );

    recents.save(&files, &at).expect("the list is written");

    assert!(
        files.exists(Path::new("/config/recents.json")),
        "the platform decides where, not the library",
    );
    assert_eq!(
        RecentList::load(&files, &at).expect("reads").entries()[0].name,
        "Support",
    );
}

#[test]
fn a_list_nobody_has_written_yet_opens_empty() {
    let at = Locations {
        config: "/config".into(),
        data: "/data".into(),
        documents: None,
    };

    let recents = RecentList::load(&InMemoryFiles::default(), &at).expect("nothing to read");

    assert!(recents.entries().is_empty());
}

#[test]
fn a_list_that_cannot_be_read_says_so_rather_than_opening_empty() {
    let at = Locations {
        config: "/config".into(),
        data: "/data".into(),
        documents: None,
    };
    let files = InMemoryFiles::default();
    files
        .write(Path::new("/config/recents.json"), b"{ half a file")
        .expect("writes");

    let read = RecentList::load(&files, &at);

    assert!(
        matches!(read, Err(StorageError::Json(_))),
        "a list that was written and then damaged is not the same thing as \
         one that was never written, and quietly forgetting the parts the \
         user opened is the wrong answer to it",
    );
}

#[test]
fn a_part_that_moved_away_stops_being_offered() {
    let files = InMemoryFiles::default();
    files
        .write(Path::new("/parts/support.caopart"), b"PK")
        .expect("writes");
    let mut recents = RecentList::default();
    recents.push(
        "/parts/bride.caopart",
        "Bride",
        at_hour("2026-01-02T09:00:00Z"),
    );
    recents.push(
        "/parts/support.caopart",
        "Support",
        at_hour("2026-01-02T09:01:00Z"),
    );

    recents.prune_missing(&files);

    let paths: Vec<&PathBuf> = recents.entries().iter().map(|e| &e.path).collect();
    assert_eq!(paths, [&PathBuf::from("/parts/support.caopart")]);
}

#[test]
fn the_oldest_go_when_the_list_reaches_what_the_start_menu_shows() {
    let pushed = MAX_RECENTS + 3;
    let mut recents = RecentList::default();
    for index in 0..pushed {
        recents.push(
            format!("/parts/piece-{index}.caopart"),
            "Piece",
            at_hour("2026-01-02T09:00:00Z"),
        );
    }

    assert_eq!(recents.entries().len(), MAX_RECENTS);
    assert_eq!(
        recents.entries()[0].path,
        PathBuf::from(format!("/parts/piece-{}.caopart", pushed - 1)),
    );
    assert_eq!(
        recents.entries()[MAX_RECENTS - 1].path,
        PathBuf::from(format!("/parts/piece-{}.caopart", pushed - MAX_RECENTS)),
        "the oldest go, not entries taken from the middle",
    );
}
