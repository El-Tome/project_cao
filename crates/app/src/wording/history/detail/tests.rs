use cao_part::history::{ExtrusionMode, PointRef, RevolutionAxis};
use cao_sketch::{
    Area, CircleId, CurveId, DimensionTarget, Element, PointId, SegmentId, SketchAxis, WorkPlane,
};
use glam::DVec2;

use super::*;

const AWAY: DVec2 = DVec2::new(3.0, 0.0);

fn swept(axis: RevolutionAxis) -> Operation {
    Operation::Revolve {
        sketch: 0,
        areas: vec![Area {
            bounds: vec![vec![CurveId::Segment(SegmentId(0))]],
            inside: DVec2::ZERO,
        }],
        axis,
        angle: 90.0.into(),
        mode: ExtrusionMode::Add,
    }
}

/// A part of three sketches, numbered 1 to 3 by the part itself: what a line
/// unfolded names a sketch by.
fn a_part_of_three_sketches() -> PartDocument {
    let mut document = PartDocument::new("part", "2026-11-12T09:00:00Z".parse().expect("a date"));
    for _ in 0..3 {
        document.apply(Operation::CreateSketch {
            plane: WorkPlane::XY,
            on: None,
        });
    }
    document
}

fn unfolded(operation: &Operation) -> String {
    detail_in(&Catalogue::french(), &a_part_of_three_sketches(), operation)
}

#[test]
fn an_unfolded_step_says_which_sketch_it_belongs_to_and_what_it_touched() {
    let steps = [
        (
            Operation::CreateSketch {
                plane: WorkPlane::XY,
                on: None,
            },
            "Plan d'origine (0, 0, 1)",
        ),
        (
            Operation::AddPoint {
                sketch: 2,
                position: DVec2::new(1.25, -3.0),
                on: Vec::new(),
            },
            "Esquisse 3 · (1.25, -3)",
        ),
        (
            Operation::AddSegment {
                sketch: 0,
                start: PointRef::Existing(PointId(7)),
                end: PointRef::New(DVec2::new(2.0, 4.0)),
                construction: false,
            },
            "Esquisse 1 · de point 7 à (2, 4)",
        ),
        (
            Operation::AddSymmetricSegment {
                sketch: 0,
                middle: PointRef::Existing(PointId(3)),
                end: PointRef::New(DVec2::new(20.0, 0.0)),
                construction: false,
            },
            "Esquisse 1 · centré en point 3, jusqu'à (20, 0)",
        ),
        (
            Operation::AddCircle {
                sketch: 1,
                center: PointRef::New(DVec2::ZERO),
                radius: 5.0,
                rim: Vec::new(),
                construction: false,
            },
            "Esquisse 2 · rayon 5",
        ),
        (
            Operation::MoveMany {
                sketch: 0,
                points: vec![PointId(1), PointId(2)],
                by: AWAY,
            },
            "Esquisse 1 · 2 points de (3, 0)",
        ),
        (
            Operation::EraseMany {
                sketch: 0,
                elements: vec![Element::Point(PointId(1))],
                dimensions: Vec::new(),
                constraints: Vec::new(),
            },
            "Esquisse 1 · 1 tracé(s), 0 cote(s), 0 contrainte(s)",
        ),
        (
            swept(RevolutionAxis::Segment(SegmentId(4))),
            "Esquisse 1 · 1 aire(s) autour de trait 4",
        ),
        (
            swept(RevolutionAxis::Sketch(SketchAxis::V)),
            "Esquisse 1 · 1 aire(s) autour de axe vertical",
        ),
        (
            Operation::SetDimension {
                sketch: 0,
                target: DimensionTarget::Radius(CircleId(3)),
                value: 60.0.into(),
                placement: None,
            },
            "Esquisse 1 · cercle 3",
        ),
    ];

    for (operation, reads) in steps {
        assert_eq!(unfolded(&operation), reads, "{operation:?} reads {reads:?}");
    }
}
