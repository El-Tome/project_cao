//! What app · adapters/window_place.rs is held to.
//!
//! Closes #780.
//! - Closed at a size and a position, not maximised, the application reopens
//!   at that size and that position — on a Mac whose screens differ in scale
//!   too, the window read in the points macOS lays its screens out in,
//!   `a_window_on_a_points_desktop_is_read_in_points_whichever_screen_it_is_on`,
//!   and elsewhere in pixels, `a_window_on_a_pixels_desktop_is_read_as_reported`
//! - A remembered position that fits no screen opens maximised on the main
//!   screen — and one that fits a screen of another scale is not taken for
//!   lost: the screens lie in one space, side by side,
//!   `a_retina_screen_and_a_scale_one_screen_beside_it_lie_side_by_side_in_points`,
//!   `a_retina_screen_left_of_the_main_one_ends_where_the_main_one_begins`
//!   and `on_a_pixels_desktop_a_screen_stands_as_reported_with_its_own_scale`
//!
//! What the windowing library reports is in physical pixels, each value
//! counted at the scale of the screen or the window it was read from. On
//! macOS those do not make one space: a Retina main screen and a scale-1
//! screen beside it overlap in pixels, and only their points lie side by side.
//! Windows and X11 lay their screens out in pixels already.
//!
//! What these do not reach: reading the screens and the window through winit,
//! and setting the place back on the window — in points on macOS, in pixels
//! elsewhere, with nothing to convert on the way. That is the real window's,
//! which no test of this workspace has, and is tried in the app, on a second
//! screen of another scale if one is at hand.

use cao_prefs::Monitor;

use super::*;

const A_RETINA_MAIN_SCREEN: PhysicalRect = PhysicalRect {
    x: 0,
    y: 0,
    width: 2880,
    height: 1800,
    scale_factor: 2.0,
};

#[test]
fn a_retina_screen_and_a_scale_one_screen_beside_it_lie_side_by_side_in_points() {
    let beside_it = PhysicalRect {
        x: 1440,
        y: 0,
        width: 1920,
        height: 1080,
        scale_factor: 1.0,
    };

    let main = Desktop::Points.monitor(A_RETINA_MAIN_SCREEN);
    let external = Desktop::Points.monitor(beside_it);

    assert_eq!(
        main,
        Monitor {
            x: 0,
            y: 0,
            width: 1440,
            height: 900,
            scale: 1.0,
        },
    );
    assert_eq!(
        external,
        Monitor {
            x: 1440,
            y: 0,
            width: 1920,
            height: 1080,
            scale: 1.0,
        },
    );
    assert_eq!(
        main.x + main.width as i32,
        external.x,
        "the external screen starts where the main one ends, neither over it nor apart",
    );
}

#[test]
fn a_retina_screen_left_of_the_main_one_ends_where_the_main_one_begins() {
    let main_at_scale_one = PhysicalRect {
        x: 0,
        y: 0,
        width: 1920,
        height: 1080,
        scale_factor: 1.0,
    };
    let a_retina_on_its_left = PhysicalRect {
        x: -2880,
        y: 0,
        width: 2880,
        height: 1800,
        scale_factor: 2.0,
    };

    let main = Desktop::Points.monitor(main_at_scale_one);
    let left = Desktop::Points.monitor(a_retina_on_its_left);

    assert_eq!(
        left,
        Monitor {
            x: -1440,
            y: 0,
            width: 1440,
            height: 900,
            scale: 1.0,
        },
    );
    assert_eq!(
        left.x + left.width as i32,
        main.x,
        "the screen on the left ends where the main one begins",
    );
}

#[test]
fn on_a_pixels_desktop_a_screen_stands_as_reported_with_its_own_scale() {
    let beside_it = PhysicalRect {
        x: 2880,
        y: 0,
        width: 1920,
        height: 1080,
        scale_factor: 1.0,
    };

    assert_eq!(
        Desktop::Pixels.monitor(A_RETINA_MAIN_SCREEN),
        Monitor {
            x: 0,
            y: 0,
            width: 2880,
            height: 1800,
            scale: 2.0,
        },
        "a screen at 200 % counts two pixels to the point, which the default size is \
         multiplied by",
    );
    assert_eq!(
        Desktop::Pixels.monitor(beside_it),
        Monitor {
            x: 2880,
            y: 0,
            width: 1920,
            height: 1080,
            scale: 1.0,
        },
    );
}

#[test]
fn a_window_on_a_points_desktop_is_read_in_points_whichever_screen_it_is_on() {
    let on_the_retina = PhysicalRect {
        x: 400,
        y: 200,
        width: 2560,
        height: 1600,
        scale_factor: 2.0,
    };
    let on_the_screen_beside_it = PhysicalRect {
        x: 1600,
        y: 100,
        width: 1200,
        height: 800,
        scale_factor: 1.0,
    };

    assert_eq!(
        Desktop::Points.window(on_the_retina),
        DesktopRect {
            x: 200,
            y: 100,
            width: 1280,
            height: 800,
        },
    );
    assert_eq!(
        Desktop::Points.window(on_the_screen_beside_it),
        DesktopRect {
            x: 1600,
            y: 100,
            width: 1200,
            height: 800,
        },
        "a window on the scale-1 screen is counted at its own scale, not the main one's",
    );
}

#[test]
fn a_window_on_a_pixels_desktop_is_read_as_reported() {
    let at_200_percent = PhysicalRect {
        x: 400,
        y: 200,
        width: 2560,
        height: 1600,
        scale_factor: 2.0,
    };

    assert_eq!(
        Desktop::Pixels.window(at_200_percent),
        DesktopRect {
            x: 400,
            y: 200,
            width: 2560,
            height: 1600,
        },
    );
}
