//! What the measure tool says, held to what #176 asked it to say.
//!
//! Closes #176.
//! - a straight run says the distance and the reach along each axis, one to a
//!   side of the triangle — `a_run_says_one_number_for_each_side_of_its_triangle`
//! - a trait's length is said the same way, offsets included —
//!   `a_length_carries_its_two_offsets_as_a_distance_does`
//! - the two reaches carry no word, since the colour of the side they are
//!   written on says which axis they run along —
//!   `a_reach_is_a_bare_number_because_its_side_already_says_which_axis`
//! - a circle says its radius and its diameter together —
//!   `a_round_says_both_the_radius_and_the_diameter`
//! - an angle is said in degrees — `an_angle_is_said_in_degrees`
//! - nothing a measure says is rounded on the way to the screen —
//!   `a_measure_rounds_nothing_off_what_it_read`

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

fn millimetres() -> UnitDisplay {
    UnitDisplay::Fixed(cao_prefs::config::LengthUnit::Millimeter)
}

fn a_run(target: DimensionTarget, span: f64, offsets: DVec2) -> Said {
    says(
        &french(),
        target,
        Reading::Gap { span, offsets },
        millimetres(),
    )
}

#[test]
fn a_run_says_one_number_for_each_side_of_its_triangle() {
    let Said::Triangle { span, across, up } = a_run(distance(), 28.2, DVec2::new(2.32, 28.1))
    else {
        panic!("a straight run is shown as a triangle");
    };

    assert!(span.contains("28.2"), "the direct distance: {span:?}");
    assert!(across.contains("2.3"), "the reach across: {across:?}");
    assert!(up.contains("28.1"), "the reach up: {up:?}");
}

#[test]
fn a_reach_is_a_bare_number_because_its_side_already_says_which_axis() {
    let Said::Triangle { span, across, up } = a_run(distance(), 28.2, DVec2::new(2.32, 28.1))
    else {
        panic!("a straight run is shown as a triangle");
    };

    assert!(
        span.chars().any(char::is_alphabetic),
        "the hypotenuse says what it is, since nothing else does: {span:?}",
    );
    for reach in [&across, &up] {
        assert!(
            !reach.chars().any(|c| c.is_alphabetic() && c != 'm'),
            "a side drawn in its axis's colour needs no word on top of it, \
             and a label crowded onto a short side is the first thing to go \
             unreadable: {reach:?}",
        );
    }
}

#[test]
fn a_length_carries_its_two_offsets_as_a_distance_does() {
    let Said::Triangle { span, across, up } = a_run(
        DimensionTarget::Length(SegmentId(0)),
        5.0,
        DVec2::new(3.0, 4.0),
    ) else {
        panic!("a trait is shown as a triangle too");
    };

    assert!(
        across.contains('3') && up.contains('4'),
        "{across:?} {up:?}"
    );
    let Said::Triangle {
        span: as_a_distance,
        ..
    } = a_run(distance(), 5.0, DVec2::new(3.0, 4.0))
    else {
        panic!("a distance is shown as a triangle");
    };
    assert_ne!(
        span, as_a_distance,
        "a trait has a length; two loose points have a distance",
    );
}

#[test]
fn a_round_says_both_the_radius_and_the_diameter() {
    let Said::Beside(lines) = says(
        &french(),
        DimensionTarget::ArcRadius(cao_sketch::ArcId(0)),
        Reading::Round { radius: 12.5 },
        millimetres(),
    ) else {
        panic!("a circle is not a run, and is written beside itself");
    };

    assert_eq!(lines.len(), 2, "a radius and a diameter: {lines:?}");
    assert!(lines[0].contains("12.5"), "the radius: {lines:?}");
    assert!(
        lines[1].contains("25"),
        "the diameter is twice it, worked out here rather than asked for: {lines:?}",
    );
}

#[test]
fn an_angle_is_said_in_degrees() {
    let Said::Beside(lines) = says(
        &french(),
        DimensionTarget::Angle {
            first: SegmentId(0),
            second: SegmentId(1),
        },
        Reading::Opening { degrees: 37.25 },
        millimetres(),
    ) else {
        panic!("an angle is not a run either");
    };

    assert_eq!(lines.len(), 1, "an angle says one thing: {lines:?}");
    assert!(
        lines[0].contains("37.2") && lines[0].contains('°'),
        "the opening in degrees: {lines:?}",
    );
}

#[test]
fn a_measure_rounds_nothing_off_what_it_read() {
    // A run of forty and a hair. Rounded to the tenth a dimension shows, all
    // three of these read as round numbers and the drawing looks exact when it
    // is not — which is the one thing a measuring tool may not do.
    let Said::Triangle { span, across, up } = a_run(
        distance(),
        40.001_531_099_2,
        DVec2::new(40.000_02, 0.350_000_7),
    ) else {
        panic!("a straight run is shown as a triangle");
    };

    assert!(
        span.contains("40.0015310992"),
        "the run keeps every digit it has: {span:?}",
    );
    assert!(
        across.contains("40.00002") && up.contains("0.3500007"),
        "and so do both reaches: {across:?} {up:?}",
    );

    let Said::Beside(lines) = says(
        &french(),
        DimensionTarget::Angle {
            first: SegmentId(0),
            second: SegmentId(1),
        },
        Reading::Opening {
            degrees: 37.249_998_3,
        },
        millimetres(),
    ) else {
        panic!("an angle is written beside itself");
    };
    assert!(
        lines[0].contains("37.2499983"),
        "an angle is a reading too, and was the last thing still rounding \
         itself to a tenth of a degree: {lines:?}",
    );
}
