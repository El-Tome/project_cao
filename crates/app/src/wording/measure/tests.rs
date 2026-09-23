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
//! - a measure is held to the figures the reader asked for, lengths and angle
//!   alike — `a_measure_is_held_to_the_figures_the_reader_asked_for`
//! - and turning that setting down turns every one of them down —
//!   `fewer_figures_shortens_every_number_a_measure_says`

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

/// The figures a fresh installation shows, which is as many as the arithmetic
/// is trusted for.
const FIGURES: u32 = cao_prefs::config::MOST_FIGURES;

fn a_run(target: DimensionTarget, span: f64, offsets: DVec2) -> Said {
    says(
        &french(),
        target,
        Reading::Gap { span, offsets },
        millimetres(),
        FIGURES,
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
        FIGURES,
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
        FIGURES,
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
fn a_measure_is_held_to_the_figures_the_reader_asked_for() {
    // A run of forty and a hair. Rounded the way a dimension is, all three of
    // these read as round numbers and the drawing looks exact when it is not —
    // which is the one thing a measuring tool may not do.
    let Said::Triangle { span, across, up } = a_run(
        distance(),
        40.001_531_099_2,
        DVec2::new(40.000_02, 0.350_000_7),
    ) else {
        panic!("a straight run is shown as a triangle");
    };

    assert!(span.contains("40.0015"), "the run: {span:?}");
    assert!(up.contains("0.350001"), "and each reach: {up:?}");
    assert!(
        across.contains("40 "),
        "a reach round to six figures says so plainly, rather than 40.0000: {across:?}",
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
        FIGURES,
    ) else {
        panic!("an angle is written beside itself");
    };
    assert!(
        lines[0].contains("37.25"),
        "an angle is a reading too, and was the last thing still holding itself \
         to a tenth of a degree whatever was asked: {lines:?}",
    );
}

#[test]
fn fewer_figures_shortens_every_number_a_measure_says() {
    let read = |figures| {
        says(
            &french(),
            distance(),
            Reading::Gap {
                span: 1234.5678,
                offsets: DVec2::new(std::f64::consts::SQRT_2, 1234.0),
            },
            millimetres(),
            figures,
        )
    };

    let Said::Triangle { span, across, .. } = read(cao_prefs::config::FEWEST_FIGURES) else {
        panic!("a straight run is shown as a triangle");
    };

    assert!(
        span.contains("1230"),
        "three figures of a thousand is a round thousand, not 1235 — which is \
         four figures wearing a round face: {span:?}",
    );
    assert!(across.contains("1.41"), "and the reach with it: {across:?}");
    assert_ne!(
        read(cao_prefs::config::MOST_FIGURES),
        read(cao_prefs::config::FEWEST_FIGURES),
        "the setting is the reader's, and has to actually move something",
    );
}
