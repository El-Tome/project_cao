//! What app · wording/dimension.rs is held to.
//!
//! Closes #519.
//! - one formatter for every length: the history and the fields say the
//!   number the canvas says, to 4 decimals, trailing zeros trimmed —
//!   `a_measured_value_is_cut_to_a_tenth_of_a_micron_and_keeps_no_trailing_zero`,
//!   `the_canvas_and_the_history_say_a_length_the_same_way`
//! - angles to a fixed 4 decimals of a degree, trailing zeros trimmed —
//!   `an_angle_is_said_to_four_decimals_of_a_degree_and_keeps_no_trailing_zero`
//! - a non-zero value is never shown as zero, length or angle —
//!   `a_value_under_a_tenth_of_a_micron_is_never_said_to_be_nothing`,
//!   `an_angle_under_a_ten_thousandth_of_a_degree_is_never_said_to_be_nothing`
//! - `wording/` tests keep passing, the one pinning 0.001 to « Cote 0 mm »
//!   changed on purpose and renamed from `…_cut_to_two_decimals_…` —
//!   `a_measured_value_is_cut_to_a_tenth_of_a_micron_and_keeps_no_trailing_zero`

use cao_prefs::config::{LengthUnit, UnitDisplay};
use cao_sketch::{ArcId, AxisToward, CircleId, PointId, SegmentId, Toward};

use super::*;

const SEGMENT: SegmentId = SegmentId(0);
const CIRCLE: CircleId = CircleId(3);
const ARC: ArcId = ArcId(2);

fn along(axis: SketchAxis) -> DimensionTarget {
    DimensionTarget::Projected {
        from: PointId(1),
        to: PointId(2),
        axis,
    }
}

fn named(target: &DimensionTarget, value: f64) -> String {
    label(&Catalogue::french(), target, &short(value))
}

fn across(target: &DimensionTarget) -> String {
    spans(&Catalogue::french(), target)
}

#[test]
fn a_dimension_is_named_after_what_it_measures() {
    let measured = [
        (
            DimensionTarget::corner(SegmentId(0), SegmentId(1)),
            "Angle 60°",
        ),
        (
            DimensionTarget::AxisAngle {
                segment: SEGMENT,
                segment_toward: Toward::End,
                axis: SketchAxis::U,
                axis_toward: AxisToward::Positive,
            },
            "Angle 60° / axe horizontal",
        ),
        (along(SketchAxis::U), "Largeur 60 mm"),
        (along(SketchAxis::V), "Hauteur 60 mm"),
        (DimensionTarget::Radius(CIRCLE), "Rayon 60 mm"),
        (DimensionTarget::Diameter(CIRCLE), "Diamètre 60 mm"),
        (DimensionTarget::ArcRadius(ARC), "Rayon 60 mm"),
        (DimensionTarget::ArcSweep(ARC), "Angle 60°"),
        (DimensionTarget::Length(SEGMENT), "Cote 60 mm"),
        (
            DimensionTarget::Distance {
                from: PointId(1),
                to: PointId(2),
            },
            "Cote 60 mm",
        ),
        (
            DimensionTarget::PointToSegment {
                point: PointId(1),
                segment: SEGMENT,
            },
            "Cote 60 mm",
        ),
    ];

    for (target, reads) in measured {
        assert_eq!(named(&target, 60.0), reads, "{target:?} reads {reads:?}");
    }
}

#[test]
fn a_measured_value_is_cut_to_a_tenth_of_a_micron_and_keeps_no_trailing_zero() {
    let length = DimensionTarget::Length(SEGMENT);

    assert_eq!(named(&length, 37.461_829_3), "Cote 37.4618 mm");
    assert_eq!(named(&length, 60.878_967), "Cote 60.879 mm");
    assert_eq!(named(&length, 60.0), "Cote 60 mm");
    assert_eq!(named(&length, 60.1), "Cote 60.1 mm");
    assert_eq!(named(&length, 0.001), "Cote 0.001 mm");
}

fn number_in(said: &str) -> f64 {
    said.trim_end_matches('°')
        .parse()
        .unwrap_or_else(|_| panic!("{said:?} is not a number"))
}

#[test]
fn the_canvas_and_the_history_say_a_length_the_same_way() {
    let canvas = UnitDisplay::Fixed(LengthUnit::Millimeter);

    for length in [37.461_829_3, 1_234.567_89, 50.0003, 12.5, 60.0, 0.000_03] {
        assert_eq!(
            canvas.format(length),
            format!("{} mm", short(length)),
            "{length} mm reads one way on the canvas and another in the history",
        );
    }
}

#[test]
fn a_value_under_a_tenth_of_a_micron_is_never_said_to_be_nothing() {
    for value in [0.000_03_f64, -0.000_03, 1e-9] {
        let said = short(value);
        let number = number_in(&said);
        assert!(
            number != 0.0 && number.signum() == value.signum(),
            "{value} is not nothing, and reads {said:?}",
        );
    }
    assert_eq!(number_in(&short(0.0)), 0.0, "nothing still reads nothing");
}

#[test]
fn an_angle_is_said_to_four_decimals_of_a_degree_and_keeps_no_trailing_zero() {
    for (angle, reads) in [
        (12.5, "12.5°"),
        (60.0, "60°"),
        (0.01, "0.01°"),
        (33.123_456, "33.1235°"),
    ] {
        assert_eq!(degrees(angle), reads, "{angle}°");
    }
    assert_eq!(
        named(&DimensionTarget::corner(SegmentId(0), SegmentId(1)), 0.01),
        "Angle 0.01°",
        "the history says the angle the canvas says",
    );
}

#[test]
fn an_angle_under_a_ten_thousandth_of_a_degree_is_never_said_to_be_nothing() {
    for angle in [0.000_02_f64, -0.000_02, 1e-9] {
        let said = degrees(angle);
        let number = number_in(&said);
        assert!(
            number != 0.0 && number.signum() == angle.signum(),
            "{angle}° is not nothing, and reads {said:?}",
        );
    }
    assert_eq!(degrees(0.0), "0°", "nothing still reads nothing");
}

#[test]
fn a_dimension_says_what_it_was_taken_across() {
    let spanned = [
        (
            DimensionTarget::Distance {
                from: PointId(1),
                to: PointId(2),
            },
            "points 1 et 2",
        ),
        (DimensionTarget::Length(SEGMENT), "trait 0"),
        (
            DimensionTarget::corner(SegmentId(0), SegmentId(1)),
            "traits 0 et 1",
        ),
        (
            DimensionTarget::AxisAngle {
                segment: SEGMENT,
                segment_toward: Toward::End,
                axis: SketchAxis::U,
                axis_toward: AxisToward::Positive,
            },
            "trait 0 / axe horizontal",
        ),
        (
            DimensionTarget::PointToSegment {
                point: PointId(1),
                segment: SEGMENT,
            },
            "point 1 au trait 0",
        ),
        (along(SketchAxis::V), "points 1 et 2 sur l'axe vertical"),
        (DimensionTarget::Radius(CIRCLE), "cercle 3"),
        (DimensionTarget::Diameter(CIRCLE), "cercle 3"),
        (DimensionTarget::ArcRadius(ARC), "arc 2"),
        (DimensionTarget::ArcSweep(ARC), "arc 2"),
    ];

    for (target, reads) in spanned {
        assert_eq!(across(&target), reads, "{target:?} reads {reads:?}");
    }
}

#[test]
fn a_value_that_measures_nothing_new_says_so_in_the_language_file() {
    assert!(
        redundant_warning(&Catalogue::french()).starts_with("Cette cote n'apporte rien"),
        "the warning comes from the catalogue, not from a constant",
    );
}

#[test]
fn a_value_a_fixed_dimension_already_rules_out_says_so_in_the_language_file() {
    assert!(
        conflict_warning(&Catalogue::french()).starts_with("Cette valeur est impossible"),
        "the warning comes from the catalogue, not from a constant",
    );
}
