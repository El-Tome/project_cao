//! What prefs · theme.rs is held to.
//!
//! Closes #286.
//! - the alert colour is a colour of the theme, and a theme saved before it
//!   existed still reads —
//!   `a_theme_saved_before_the_alert_colour_existed_still_reads`
//! - it is edited in the settings like the others — no test: the row is one
//!   call in `settings/appearance.rs`, which draws and decides nothing

use super::*;

fn gradient() -> Background {
    Background::Linear {
        angle_degrees: 0.0,
        stops: vec![
            Stop::new(0.0, Rgba::opaque(0.0, 0.0, 0.0)),
            Stop::new(0.5, Rgba::opaque(1.0, 0.0, 0.0)),
            Stop::new(1.0, Rgba::opaque(1.0, 1.0, 1.0)),
        ],
    }
}

#[test]
fn a_gradient_reads_between_its_stops() {
    let background = gradient();
    assert_eq!(background.sample(0.0), Rgba::opaque(0.0, 0.0, 0.0));
    assert_eq!(background.sample(0.5), Rgba::opaque(1.0, 0.0, 0.0));
    assert_eq!(background.sample(1.0), Rgba::opaque(1.0, 1.0, 1.0));

    let quarter = background.sample(0.25);
    assert!((quarter.r - 0.5).abs() < 1e-5, "{quarter:?}");
    assert!(quarter.g.abs() < 1e-5);
}

/// Past either end, a gradient holds its last colour rather than fading to
/// nothing.
#[test]
fn a_gradient_holds_its_ends() {
    let background = gradient();
    assert_eq!(background.sample(-3.0), Rgba::opaque(0.0, 0.0, 0.0));
    assert_eq!(background.sample(9.0), Rgba::opaque(1.0, 1.0, 1.0));
}

#[test]
fn a_gradient_with_no_stops_still_gives_a_colour() {
    let empty = Background::Linear {
        angle_degrees: 0.0,
        stops: Vec::new(),
    };
    assert_eq!(empty.sample(0.5), Rgba::opaque(0.0, 0.0, 0.0));
}

#[test]
fn a_solid_background_is_the_same_everywhere() {
    let background = Background::Solid(Rgba::opaque(0.2, 0.3, 0.4));
    assert_eq!(background.sample(0.0), background.sample(1.0));
}

/// The alert colour landed after people already had themes on disk, so a file
/// that predates it has to read as a theme with the default.
#[test]
fn a_theme_saved_before_the_alert_colour_existed_still_reads() {
    let mut saved = serde_json::to_value(Theme::default()).expect("a theme writes out");
    saved
        .as_object_mut()
        .expect("a theme is an object")
        .remove("going")
        .expect("the alert colour is written out with the rest");

    let read: Theme = serde_json::from_value(saved).expect("a theme from before the colour");

    assert_eq!(read.going, Theme::default().going);
}
