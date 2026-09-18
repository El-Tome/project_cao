//! What app · lang/mod.rs is held to.

use cao_prefs::InMemoryFiles;
use chrono::{FixedOffset, TimeZone};

use super::*;

fn a_moment() -> DateTime<Utc> {
    Utc.with_ymd_and_hms(2026, 9, 11, 14, 30, 0)
        .single()
        .expect("one moment answers")
}

#[test]
fn the_french_catalogue_says_what_a_key_stands_for() {
    assert_eq!(
        Catalogue::french().t("history.sketch"),
        "Esquisse — {plane}"
    );
}

#[test]
fn a_named_hole_is_filled_with_what_the_caller_hands_over() {
    assert_eq!(
        Catalogue::french().t_with("history.sketch", &[("plane", "Plan XY")]),
        "Esquisse — Plan XY",
    );
}

#[test]
fn a_moment_is_written_the_way_the_french_file_shapes_it() {
    assert_eq!(
        Catalogue::french().t_moment("start_menu.recent_date", a_moment(), &Utc),
        "11/09/2026 14:30",
    );
}

#[test]
fn a_moment_is_written_on_the_clock_of_whoever_reads_it() {
    let two_hours_east = FixedOffset::east_opt(2 * 3600).expect("a zone two hours east");

    assert_eq!(
        Catalogue::french().t_moment("start_menu.recent_date", a_moment(), &two_hours_east),
        "11/09/2026 16:30",
    );
}

#[test]
fn a_pattern_naming_no_part_of_a_date_still_leaves_a_date_standing() {
    let files = InMemoryFiles::default();
    files
        .write(
            Path::new("/config/lang/en.json"),
            br#"{"start_menu.recent_date": "hier"}"#,
        )
        .expect("writes");
    let catalogue = Catalogue::load(&files, Path::new("/config/lang"), "en");

    assert_eq!(
        catalogue.t_moment("start_menu.recent_date", a_moment(), &Utc),
        "2026-09-11 14:30",
    );
}

#[test]
fn a_key_no_entry_claims_leaves_a_date_standing_where_it_would_show_itself() {
    let never_written = ["start_menu", "recent_date", "unwritten"].join(".");

    assert_eq!(
        Catalogue::french().t_moment(&never_written, a_moment(), &Utc),
        "2026-09-11 14:30",
    );
}

#[test]
fn a_pattern_no_clock_can_read_leaves_a_date_standing() {
    let files = InMemoryFiles::default();
    files
        .write(
            Path::new("/config/lang/en.json"),
            br#"{"start_menu.recent_date": "%d/%"}"#,
        )
        .expect("writes");
    let catalogue = Catalogue::load(&files, Path::new("/config/lang"), "en");

    assert_eq!(
        catalogue.t_moment("start_menu.recent_date", a_moment(), &Utc),
        "2026-09-11 14:30",
    );
}

#[test]
fn a_key_the_active_language_never_translated_is_still_said_in_french() {
    let files = InMemoryFiles::default();
    files
        .write(
            Path::new("/config/lang/en.json"),
            br#"{"history.sketch": "Sketch on {plane}"}"#,
        )
        .expect("writes");

    let catalogue = Catalogue::load(&files, Path::new("/config/lang"), "en");

    assert_eq!(
        catalogue.t_with("history.sketch", &[("plane", "XY plane")]),
        "Sketch on XY plane",
    );
    assert_eq!(catalogue.t("history.point"), "Point");
}

#[test]
fn a_language_file_that_is_absent_or_broken_leaves_french_standing() {
    let files = InMemoryFiles::default();
    files
        .write(Path::new("/config/lang/de.json"), b"{ this is not json")
        .expect("writes");

    for language in ["de", "it"] {
        let catalogue = Catalogue::load(&files, Path::new("/config/lang"), language);

        assert_eq!(
            catalogue.t("history.point"),
            "Point",
            "{language} falls back"
        );
    }
}

#[test]
fn the_embedded_french_file_parses_and_is_not_empty() {
    assert!(!entries_of(FRENCH).is_empty());
}
