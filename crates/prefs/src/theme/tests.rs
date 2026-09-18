//! What prefs · theme.rs is held to.

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
