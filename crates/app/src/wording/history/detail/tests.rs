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
            bounds: vec![CurveId::Segment(SegmentId(0))],
            inside: DVec2::ZERO,
        }],
        axis,
        angle: 90.0,
        mode: ExtrusionMode::Add,
    }
}

fn unfolded(operation: &Operation) -> String {
    detail(&Catalogue::french(), operation)
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
            "Esquisse 2 · (1.2, -3.0)",
        ),
        (
            Operation::AddSegment {
                sketch: 0,
                start: PointRef::Existing(PointId(7)),
                end: PointRef::New(DVec2::new(2.0, 4.0)),
                construction: false,
            },
            "Esquisse 0 · de point 7 à (2.0, 4.0)",
        ),
        (
            Operation::AddSymmetricSegment {
                sketch: 0,
                middle: PointRef::Existing(PointId(3)),
                end: PointRef::New(DVec2::new(20.0, 0.0)),
                construction: false,
            },
            "Esquisse 0 · centré en point 3, jusqu'à (20.0, 0.0)",
        ),
        (
            Operation::AddCircle {
                sketch: 1,
                center: PointRef::New(DVec2::ZERO),
                radius: 5.0,
                rim: Vec::new(),
                construction: false,
            },
            "Esquisse 1 · rayon 5.00",
        ),
        (
            Operation::MoveMany {
                sketch: 0,
                points: vec![PointId(1), PointId(2)],
                by: AWAY,
            },
            "Esquisse 0 · 2 points de (3.0, 0.0)",
        ),
        (
            Operation::EraseMany {
                sketch: 0,
                elements: vec![Element::Point(PointId(1))],
                dimensions: Vec::new(),
                constraints: Vec::new(),
            },
            "Esquisse 0 · 1 tracé(s), 0 cote(s), 0 contrainte(s)",
        ),
        (
            swept(RevolutionAxis::Segment(SegmentId(4))),
            "Esquisse 0 · 1 aire(s) autour de trait 4",
        ),
        (
            swept(RevolutionAxis::Sketch(SketchAxis::V)),
            "Esquisse 0 · 1 aire(s) autour de axe vertical",
        ),
        (
            Operation::SetDimension {
                sketch: 0,
                target: DimensionTarget::Radius(CircleId(3)),
                value: 60.0,
                placement: None,
            },
            "Esquisse 0 · cercle 3",
        ),
    ];

    for (operation, reads) in steps {
        assert_eq!(unfolded(&operation), reads, "{operation:?} reads {reads:?}");
    }
}
