//! What prefs · window_place.rs is held to.
//!
//! Closes #780.
//! - Closed at a size and a position, not maximised, the application reopens
//!   at that size and that position —
//!   `a_window_closed_where_it_stood_reopens_there_at_that_size`, the place
//!   kept from one launch to the next by
//!   `a_place_written_on_close_reads_back_as_it_was`
//! - Closed maximised, it reopens maximised; un-maximised then, it returns to
//!   the size and position it had before being maximised —
//!   `a_window_closed_maximised_reopens_maximised_over_the_rect_it_had_before`,
//!   the flag kept by `a_place_written_on_close_reads_back_as_it_was`
//! - At the first launch, with nothing remembered, it opens maximised;
//!   un-maximised then, it takes a default size, centred —
//!   `nothing_remembered_opens_maximised_over_the_default_size_centred_on_the_main_screen`,
//!   `the_default_size_is_counted_in_points_on_a_screen_of_scale_two`,
//!   `a_main_screen_smaller_than_the_default_size_still_opens_maximised_and_centred`
//!   and `a_place_that_cannot_be_read_back_reads_as_nothing_remembered`
//! - A setting « Rouvrir à la dernière taille », on by default; off, the
//!   application opens maximised whatever was remembered — the off half here,
//!   `with_the_setting_off_a_remembered_place_opens_as_if_nothing_were_remembered`;
//!   on by default in `settings/tests.rs`, the box in
//!   `crates/app/tests/reopening_the_window.rs`
//! - A remembered position that fits no screen opens maximised on the main
//!   screen — `a_place_on_a_screen_since_unplugged_opens_maximised_on_the_main_screen`,
//!   against `a_place_on_the_second_screen_reopens_there_while_it_is_plugged_in`
//!
//! Positions and sizes are in desktop units, the ones the system lays its
//! screens out in — points on macOS, pixels elsewhere: the place is the
//! window's outer top-left corner and its inner size. The default size is
//! 1280 × 800 points, times the main screen's scale, which is how many desktop
//! units a point takes there: 1 on macOS, 2 on a Windows screen at 200 %. The
//! main screen is the first one reported, and a place fits a screen when its
//! centre lies on it.
//! With no screen reported at all, nothing is decided against the screens —
//! `no_screen_reported_at_all_still_gives_a_place_to_open_at`.
//!
//! What these cannot reach is the window itself. Un-maximising is the
//! system's: macOS gives a window back the frame it had when it was maximised,
//! so the shell sets the rect first and maximises after; and while the window
//! is maximised, the rect held for the next launch is still the one from
//! before. Both happen on the real window, which no test of this workspace
//! has, and are tried in the app.

use std::path::Path;

use super::*;
use crate::adapters::InMemoryFiles;
use crate::locations::Locations;
use crate::ports::Files;

const REOPENING_AT_THE_LAST_SIZE: bool = true;
const ALWAYS_MAXIMISED: bool = false;

const FULL_HD_AT_SCALE_ONE: Monitor = Monitor {
    x: 0,
    y: 0,
    width: 1920,
    height: 1080,
    scale: 1.0,
};

const RETINA_AT_SCALE_TWO: Monitor = Monitor {
    x: 0,
    y: 0,
    width: 2880,
    height: 1800,
    scale: 2.0,
};

const A_SECOND_SCREEN_TO_THE_LEFT: Monitor = Monitor {
    x: -2560,
    y: 0,
    width: 2560,
    height: 1440,
    scale: 1.0,
};

const THE_DEFAULT_CENTRED_ON_FULL_HD_AND_MAXIMISED: WindowPlace = WindowPlace {
    x: 320,
    y: 140,
    width: 1280,
    height: 800,
    maximised: true,
};

const LEFT_ON_THE_SECOND_SCREEN: WindowPlace = WindowPlace {
    x: -2000,
    y: 300,
    width: 1200,
    height: 800,
    maximised: false,
};

fn at() -> Locations {
    Locations {
        config: "/config".into(),
        data: "/data".into(),
        documents: None,
    }
}

#[test]
fn a_window_closed_where_it_stood_reopens_there_at_that_size() {
    let closed = WindowPlace {
        x: 200,
        y: 120,
        width: 1000,
        height: 700,
        maximised: false,
    };

    let opened = where_the_window_opens(
        Some(closed),
        REOPENING_AT_THE_LAST_SIZE,
        &[FULL_HD_AT_SCALE_ONE],
    );

    assert_eq!(opened, closed);
}

#[test]
fn a_window_closed_maximised_reopens_maximised_over_the_rect_it_had_before() {
    let closed = WindowPlace {
        x: 400,
        y: 200,
        width: 1600,
        height: 1000,
        maximised: true,
    };

    let opened = where_the_window_opens(
        Some(closed),
        REOPENING_AT_THE_LAST_SIZE,
        &[RETINA_AT_SCALE_TWO],
    );

    assert_eq!(
        opened, closed,
        "maximised again, over the rect to go back to once restored rather than the \
         screen's",
    );
}

#[test]
fn nothing_remembered_opens_maximised_over_the_default_size_centred_on_the_main_screen() {
    let opened = where_the_window_opens(
        None,
        REOPENING_AT_THE_LAST_SIZE,
        &[FULL_HD_AT_SCALE_ONE, A_SECOND_SCREEN_TO_THE_LEFT],
    );

    assert_eq!(
        opened, THE_DEFAULT_CENTRED_ON_FULL_HD_AND_MAXIMISED,
        "1280 × 800 centred on the first screen reported, not on the second nor \
         across both",
    );
}

#[test]
fn the_default_size_is_counted_in_points_on_a_screen_of_scale_two() {
    let opened = where_the_window_opens(None, REOPENING_AT_THE_LAST_SIZE, &[RETINA_AT_SCALE_TWO]);

    assert_eq!(
        opened,
        WindowPlace {
            x: 160,
            y: 100,
            width: 2560,
            height: 1600,
            maximised: true,
        },
        "1280 × 800 points are 2560 × 1600 pixels at scale 2",
    );
}

#[test]
fn a_main_screen_smaller_than_the_default_size_still_opens_maximised_and_centred() {
    let small = Monitor {
        x: 0,
        y: 0,
        width: 1024,
        height: 768,
        scale: 1.0,
    };

    let opened = where_the_window_opens(None, REOPENING_AT_THE_LAST_SIZE, &[small]);

    assert!(opened.maximised, "{opened:?}");
    let centre = (
        opened.x + opened.width as i32 / 2,
        opened.y + opened.height as i32 / 2,
    );
    assert!(
        (centre.0 - 512).abs() <= 1 && (centre.1 - 384).abs() <= 1,
        "the rect to go back to is centred on the 1024 × 768 screen, whatever its size: \
         {opened:?} is centred on {centre:?}",
    );
}

#[test]
fn with_the_setting_off_a_remembered_place_opens_as_if_nothing_were_remembered() {
    let closed = WindowPlace {
        x: 200,
        y: 120,
        width: 1000,
        height: 700,
        maximised: false,
    };

    let opened = where_the_window_opens(Some(closed), ALWAYS_MAXIMISED, &[FULL_HD_AT_SCALE_ONE]);

    assert_eq!(opened, THE_DEFAULT_CENTRED_ON_FULL_HD_AND_MAXIMISED);
}

#[test]
fn a_place_on_the_second_screen_reopens_there_while_it_is_plugged_in() {
    let opened = where_the_window_opens(
        Some(LEFT_ON_THE_SECOND_SCREEN),
        REOPENING_AT_THE_LAST_SIZE,
        &[FULL_HD_AT_SCALE_ONE, A_SECOND_SCREEN_TO_THE_LEFT],
    );

    assert_eq!(opened, LEFT_ON_THE_SECOND_SCREEN);
}

#[test]
fn a_place_on_a_screen_since_unplugged_opens_maximised_on_the_main_screen() {
    let opened = where_the_window_opens(
        Some(LEFT_ON_THE_SECOND_SCREEN),
        REOPENING_AT_THE_LAST_SIZE,
        &[FULL_HD_AT_SCALE_ONE],
    );

    assert_eq!(
        opened, THE_DEFAULT_CENTRED_ON_FULL_HD_AND_MAXIMISED,
        "the screen the window was left on is gone: the remembered size is not moved \
         onto the main screen, the window opens there maximised",
    );
}

#[test]
fn no_screen_reported_at_all_still_gives_a_place_to_open_at() {
    let closed = WindowPlace {
        x: 200,
        y: 120,
        width: 1000,
        height: 700,
        maximised: false,
    };

    assert_eq!(
        where_the_window_opens(Some(closed), REOPENING_AT_THE_LAST_SIZE, &[]),
        closed,
        "with no screen to hold it against, the remembered place is kept and the system \
         places the window",
    );
    let first = where_the_window_opens(None, REOPENING_AT_THE_LAST_SIZE, &[]);
    assert_eq!(
        (first.width, first.height, first.maximised),
        (1280, 800, true),
        "with nothing remembered and no screen, the default size at scale 1, maximised",
    );
}

#[test]
fn a_place_written_on_close_reads_back_as_it_was() {
    let files = InMemoryFiles::default();
    let closed = WindowPlace {
        maximised: true,
        ..LEFT_ON_THE_SECOND_SCREEN
    };
    assert_eq!(
        WindowPlace::load(&files, &at()),
        None,
        "before anything is written, nothing is remembered",
    );

    closed.save(&files, &at()).expect("the place is written");

    assert!(
        files.exists(Path::new("/config/window.json")),
        "the place is kept beside the preferences, in a file of its own",
    );
    assert_eq!(WindowPlace::load(&files, &at()), Some(closed));
}

#[test]
fn a_place_that_cannot_be_read_back_reads_as_nothing_remembered() {
    let files = InMemoryFiles::default();
    LEFT_ON_THE_SECOND_SCREEN
        .save(&files, &at())
        .expect("the place is written");
    assert_eq!(
        WindowPlace::load(&files, &at()),
        Some(LEFT_ON_THE_SECOND_SCREEN)
    );

    files
        .write(Path::new("/config/window.json"), b"\xff{ half a window")
        .expect("writes");

    assert_eq!(
        WindowPlace::load(&files, &at()),
        None,
        "a damaged place costs one launch at the default size, which is less than \
         refusing to open over it",
    );
}
