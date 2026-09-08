//! What a history entry says today, witnessed before the sentences move up
//! into `cao_app`. Nothing here is a specification: it is what the history
//! panel shows, so that the move can be read as a move.
//!
//! An integration test rather than a module inside `history.rs`: that file is
//! past the line budget, so the gate refuses it any growth at all — including a
//! witness that leaves again in the next commit.

use cao_part::history::{ExtrusionMode, Operation, PointRef, RevolutionAxis};
use cao_sketch::{
    CircleId, Constraint, DimensionTarget, Element, PointId, SegmentId, SketchAxis, WorkPlane,
};
use glam::DVec2;

fn point(id: usize) -> PointId {
    PointId(id)
}

fn segment(id: usize) -> SegmentId {
    SegmentId(id)
}

#[test]
fn a_drawing_step_is_named_after_what_was_drawn() {
    let steps = [
        (
            Operation::CreateSketch {
                plane: WorkPlane::XY,
            },
            "Esquisse — Plan XY",
        ),
        (
            Operation::AddPoint {
                sketch: 0,
                position: DVec2::ZERO,
            },
            "Point",
        ),
        (
            Operation::AddSegment {
                sketch: 0,
                start: PointRef::New(DVec2::ZERO),
                end: PointRef::New(DVec2::X),
            },
            "Trait",
        ),
        (
            Operation::AddRectangle {
                sketch: 0,
                corner: PointRef::New(DVec2::ZERO),
                opposite: PointRef::New(DVec2::X),
            },
            "Rectangle",
        ),
        (
            Operation::AddCircle {
                sketch: 0,
                center: PointRef::New(DVec2::ZERO),
                radius: 5.0,
                rim: Vec::new(),
            },
            "Cercle",
        ),
        (
            Operation::MovePoint {
                sketch: 0,
                point: point(1),
                position: DVec2::X,
            },
            "Déplacement",
        ),
        (
            Operation::MoveMany {
                sketch: 0,
                points: vec![point(1)],
                by: DVec2::X,
            },
            "Déplacement",
        ),
        (
            Operation::MoveDimension {
                sketch: 0,
                target: DimensionTarget::Length(segment(0)),
                offset: DVec2::X,
            },
            "Cote déplacée",
        ),
        (
            Operation::MergePoints {
                sketch: 0,
                kept: point(1),
                dropped: point(2),
            },
            "Sommets fusionnés",
        ),
        (
            Operation::Constrain {
                sketch: 0,
                constraint: Constraint::Parallel {
                    first: segment(0),
                    second: segment(1),
                },
            },
            "Parallèle",
        ),
    ];

    for (operation, reads) in steps {
        assert_eq!(operation.label(), reads, "{operation:?} reads {reads:?}");
    }
}

#[test]
fn matter_added_and_matter_taken_away_read_apart() {
    let extrusion = |mode| Operation::Extrude {
        sketch: 0,
        picks: vec![DVec2::ZERO],
        distance: 12.0,
        mode,
    };
    assert_eq!(extrusion(ExtrusionMode::Add).label(), "Extrusion 12 mm");
    assert_eq!(extrusion(ExtrusionMode::Cut).label(), "Enlèvement 12 mm");

    let revolution = |mode| Operation::Revolve {
        sketch: 0,
        picks: vec![DVec2::ZERO],
        axis: RevolutionAxis::Sketch(SketchAxis::U),
        angle: 90.0,
        mode,
    };
    assert_eq!(revolution(ExtrusionMode::Add).label(), "Révolution 90°");
    assert_eq!(
        revolution(ExtrusionMode::Cut).label(),
        "Révolution creusée 90°",
    );
}

#[test]
fn a_single_deletion_says_what_went_and_a_grouped_one_says_how_many() {
    let erased = |elements: Vec<Element>, dimensions: Vec<DimensionTarget>, constraints| {
        Operation::EraseMany {
            sketch: 0,
            elements,
            dimensions,
            constraints,
        }
    };
    let rule = Constraint::Parallel {
        first: segment(0),
        second: segment(1),
    };

    assert_eq!(
        erased(vec![Element::Point(point(1))], Vec::new(), Vec::new()).label(),
        "Point supprimé",
    );
    assert_eq!(
        erased(vec![Element::Segment(segment(0))], Vec::new(), Vec::new()).label(),
        "Trait supprimé",
    );
    assert_eq!(
        erased(vec![Element::Circle(CircleId(0))], Vec::new(), Vec::new()).label(),
        "Cercle supprimé",
    );
    assert_eq!(
        erased(
            Vec::new(),
            vec![DimensionTarget::Length(segment(0))],
            Vec::new(),
        )
        .label(),
        "Cote supprimée",
    );
    assert_eq!(
        erased(Vec::new(), Vec::new(), vec![rule]).label(),
        "Parallèle supprimée",
    );
    assert_eq!(
        erased(
            vec![Element::Point(point(1)), Element::Point(point(2))],
            vec![DimensionTarget::Length(segment(0))],
            vec![rule],
        )
        .label(),
        "4 éléments supprimés",
    );
}

#[test]
fn a_dimension_is_named_after_what_it_measures() {
    let dimension = |target| Operation::SetDimension {
        sketch: 0,
        target,
        value: 60.0,
        placement: None,
    };

    assert_eq!(
        dimension(DimensionTarget::Angle {
            first: segment(0),
            second: segment(1),
        })
        .label(),
        "Angle 60°",
    );
    assert_eq!(
        dimension(DimensionTarget::AxisAngle {
            segment: segment(0),
            axis: SketchAxis::U,
        })
        .label(),
        "Angle 60° / axe horizontal",
    );
    assert_eq!(
        dimension(DimensionTarget::Radius(CircleId(0))).label(),
        "Rayon 60 mm"
    );
    assert_eq!(
        dimension(DimensionTarget::Diameter(CircleId(0))).label(),
        "Diamètre 60 mm",
    );
    assert_eq!(
        dimension(DimensionTarget::Projected {
            from: point(1),
            to: point(2),
            axis: SketchAxis::U,
        })
        .label(),
        "Largeur 60 mm",
    );
    assert_eq!(
        dimension(DimensionTarget::Projected {
            from: point(1),
            to: point(2),
            axis: SketchAxis::V,
        })
        .label(),
        "Hauteur 60 mm",
    );
    assert_eq!(
        dimension(DimensionTarget::Length(segment(0))).label(),
        "Cote 60 mm"
    );
    assert_eq!(
        dimension(DimensionTarget::Distance {
            from: point(1),
            to: point(2),
        })
        .label(),
        "Cote 60 mm",
    );
    assert_eq!(
        dimension(DimensionTarget::PointToSegment {
            point: point(1),
            segment: segment(0),
        })
        .label(),
        "Cote 60 mm",
    );
}

/// A dimension taken from the drawing itself is a full float, and
/// "Cote 60.878967 mm" is unreadable.
#[test]
fn a_measured_value_is_cut_to_two_decimals_and_keeps_no_trailing_zero() {
    let dimension = |value| {
        Operation::SetDimension {
            sketch: 0,
            target: DimensionTarget::Length(segment(0)),
            value,
            placement: None,
        }
        .label()
    };

    assert_eq!(dimension(60.878_967), "Cote 60.88 mm");
    assert_eq!(dimension(60.0), "Cote 60 mm");
    assert_eq!(dimension(60.1), "Cote 60.1 mm");
    assert_eq!(dimension(60.10), "Cote 60.1 mm");
    assert_eq!(dimension(0.001), "Cote 0 mm");
}

#[test]
fn an_unfolded_step_says_which_sketch_it_belongs_to_and_what_it_touched() {
    assert_eq!(
        Operation::CreateSketch {
            plane: WorkPlane::XY,
        }
        .detail(),
        "Plan d'origine (0, 0, 1)",
    );
    assert_eq!(
        Operation::AddPoint {
            sketch: 2,
            position: DVec2::new(1.25, -3.0),
        }
        .detail(),
        "Esquisse 2 · (1.2, -3.0)",
    );
    assert_eq!(
        Operation::AddSegment {
            sketch: 0,
            start: PointRef::Existing(point(7)),
            end: PointRef::New(DVec2::new(2.0, 4.0)),
        }
        .detail(),
        "Esquisse 0 · point 7 → (2.0, 4.0)",
    );
    assert_eq!(
        Operation::AddCircle {
            sketch: 1,
            center: PointRef::New(DVec2::ZERO),
            radius: 5.0,
            rim: Vec::new(),
        }
        .detail(),
        "Esquisse 1 · rayon 5.00",
    );
    assert_eq!(
        Operation::MoveMany {
            sketch: 0,
            points: vec![point(1), point(2)],
            by: DVec2::new(3.0, 0.0),
        }
        .detail(),
        "Esquisse 0 · 2 points de (3.0, 0.0)",
    );
    assert_eq!(
        Operation::EraseMany {
            sketch: 0,
            elements: vec![Element::Point(point(1))],
            dimensions: Vec::new(),
            constraints: Vec::new(),
        }
        .detail(),
        "Esquisse 0 · 1 tracé(s), 0 cote(s), 0 contrainte(s)",
    );
    assert_eq!(
        Operation::Revolve {
            sketch: 0,
            picks: vec![DVec2::ZERO],
            axis: RevolutionAxis::Segment(segment(4)),
            angle: 90.0,
            mode: ExtrusionMode::Add,
        }
        .detail(),
        "Esquisse 0 · 1 aire(s) autour de trait 4",
    );
    assert_eq!(
        Operation::SetDimension {
            sketch: 0,
            target: DimensionTarget::Radius(CircleId(3)),
            value: 60.0,
            placement: None,
        }
        .detail(),
        "Esquisse 0 · cercle 3",
    );
}

#[test]
fn an_axis_of_revolution_is_a_sketch_axis_or_a_drawn_trait() {
    assert_eq!(
        RevolutionAxis::Sketch(SketchAxis::V).label(),
        "axe vertical",
    );
    assert_eq!(RevolutionAxis::Segment(segment(4)).label(), "trait 4");
}
