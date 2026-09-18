//! What app · wording/dimension.rs is held to.

use cao_sketch::{ArcId, CircleId, PointId, SegmentId};

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
    label(&Catalogue::french(), target, value)
}

fn across(target: &DimensionTarget) -> String {
    spans(&Catalogue::french(), target)
}

#[test]
fn a_dimension_is_named_after_what_it_measures() {
    let measured = [
        (
            DimensionTarget::Angle {
                first: SegmentId(0),
                second: SegmentId(1),
            },
            "Angle 60°",
        ),
        (
            DimensionTarget::AxisAngle {
                segment: SEGMENT,
                axis: SketchAxis::U,
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
fn a_measured_value_is_cut_to_two_decimals_and_keeps_no_trailing_zero() {
    let length = DimensionTarget::Length(SEGMENT);

    assert_eq!(named(&length, 60.878_967), "Cote 60.88 mm");
    assert_eq!(named(&length, 60.0), "Cote 60 mm");
    assert_eq!(named(&length, 60.1), "Cote 60.1 mm");
    assert_eq!(named(&length, 0.001), "Cote 0 mm");
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
            DimensionTarget::Angle {
                first: SegmentId(0),
                second: SegmentId(1),
            },
            "traits 0 et 1",
        ),
        (
            DimensionTarget::AxisAngle {
                segment: SEGMENT,
                axis: SketchAxis::U,
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
