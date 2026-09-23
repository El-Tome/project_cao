//! What the measure tool says, held to what #176 asked it to say.
//!
//! Closes #176.
//! - a distance is said first, then the reach along each axis —
//!   `a_distance_is_said_first_and_the_two_reaches_under_it`
//! - a trait's length is said the same way, offsets included —
//!   `a_length_carries_its_two_offsets_as_a_distance_does`
//! - a circle says its radius and its diameter together —
//!   `a_round_says_both_the_radius_and_the_diameter`
//! - an angle is said in degrees — `an_angle_is_said_in_degrees`

use super::*;

use cao_sketch::{PointId, SegmentId};
use glam::DVec2;

fn french() -> Catalogue {
    Catalogue::french()
}

fn distance() -> DimensionTarget {
    DimensionTarget::Distance {
        from: PointId(1),
        to: PointId(2),
    }
}

#[test]
fn a_distance_is_said_first_and_the_two_reaches_under_it() {
    let lines = lines(
        &french(),
        distance(),
        Reading::Gap {
            span: 48.09,
            offsets: DVec2::new(40.0, 26.7),
        },
        UnitDisplay::Fixed(cao_prefs::config::LengthUnit::Millimeter),
    );

    assert_eq!(lines.len(), 3, "a distance and its two reaches: {lines:?}");
    assert!(
        lines[0].contains("48"),
        "the direct distance comes first: {lines:?}",
    );
    assert!(
        lines[1].contains("40") && lines[2].contains("26"),
        "the reach along each axis follows it: {lines:?}",
    );
}

#[test]
fn a_length_carries_its_two_offsets_as_a_distance_does() {
    let lines = lines(
        &french(),
        DimensionTarget::Length(SegmentId(0)),
        Reading::Gap {
            span: 5.0,
            offsets: DVec2::new(3.0, 4.0),
        },
        UnitDisplay::Fixed(cao_prefs::config::LengthUnit::Millimeter),
    );

    assert_eq!(
        lines.len(),
        3,
        "a trait is a distance between its two ends, and says so: {lines:?}",
    );
    assert_ne!(
        lines[0],
        self::lines(
            &french(),
            distance(),
            Reading::Gap {
                span: 5.0,
                offsets: DVec2::new(3.0, 4.0),
            },
            UnitDisplay::Fixed(cao_prefs::config::LengthUnit::Millimeter),
        )[0],
        "a trait has a length; two loose points have a distance",
    );
}

#[test]
fn a_round_says_both_the_radius_and_the_diameter() {
    let lines = lines(
        &french(),
        DimensionTarget::ArcRadius(cao_sketch::ArcId(0)),
        Reading::Round { radius: 12.5 },
        UnitDisplay::Fixed(cao_prefs::config::LengthUnit::Millimeter),
    );

    assert_eq!(lines.len(), 2, "a radius and a diameter: {lines:?}");
    assert!(
        lines[0].contains("12.5"),
        "the radius as it was read: {lines:?}",
    );
    assert!(
        lines[1].contains("25"),
        "the diameter is twice it, worked out here rather than asked for: {lines:?}",
    );
}

#[test]
fn an_angle_is_said_in_degrees() {
    let lines = lines(
        &french(),
        DimensionTarget::Angle {
            first: SegmentId(0),
            second: SegmentId(1),
        },
        Reading::Opening { degrees: 37.25 },
        UnitDisplay::Fixed(cao_prefs::config::LengthUnit::Millimeter),
    );

    assert_eq!(lines.len(), 1, "an angle says one thing: {lines:?}");
    assert!(
        lines[0].contains("37.2") && lines[0].contains('°'),
        "the opening in degrees: {lines:?}",
    );
}
