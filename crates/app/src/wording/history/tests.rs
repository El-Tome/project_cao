//! What app · wording/history/mod.rs is held to.
//!
//! Closes #519.
//! - the history says a length through the one formatter, to 4 decimals,
//!   trailing zeros trimmed, a place's coordinates included —
//!   `the_history_says_a_radius_to_a_tenth_of_a_micron`,
//!   `the_history_says_a_place_to_a_tenth_of_a_micron`; the pins that read
//!   « (20.0, 0.0) », « rayon 5.00 » and « 30.0° » changed on purpose —
//!   `a_side_moved_and_a_shape_turned_say_so_and_unfold_into_what_they_did`
//!
//! Closes #193.
//! - decided in review: a sketch off the origin held by no face reads « Plan
//!   décalé », and one anchored to a face « Face de la pièce » —
//!   `an_unanchored_sketch_off_the_origin_reads_as_an_offset_plane`,
//!   `a_sketch_anchored_to_a_face_reads_as_a_face_of_the_part`; the one laid on
//!   a cut is held where it is started, in
//!   `crates/app/src/screens/viewport/input/planes/tests.rs`
//! - a face sketch from a file older than its anchor is that same value, a
//!   plane off the origin and no face: it reads « Plan décalé » too, which is
//!   true of it, rather than « Face de la pièce », which the file no longer
//!   proves. Telling it from a sketch laid on a cut would take a mark in the
//!   format, set aside — `a_face_sketch_from_a_file_older_than_its_anchor_reads_as_an_offset_plane`

use cao_part::history::{PointRef, RevolutionAxis};
use cao_sketch::{Area, CircleId, CurveId, PointId, SegmentId, SketchAxis, WorkPlane};
use glam::DVec2;

use super::*;

fn said(operation: &Operation) -> String {
    label(
        &Catalogue::french(),
        &cao_part::Variables::default(),
        operation,
    )
}

/// A part of two sketches, numbered 1 and 2 by the part itself: what a line
/// unfolded names a sketch by.
fn a_part_of_two_sketches() -> cao_part::PartDocument {
    let mut document =
        cao_part::PartDocument::new("part", "2026-11-12T09:00:00Z".parse().expect("a date"));
    for _ in 0..2 {
        document.apply(Operation::CreateSketch {
            plane: WorkPlane::XY,
            on: None,
        });
    }
    document
}

/// What a line of the history of `a_part_of_two_sketches` reads once unfolded.
fn unfolded(operation: &Operation) -> String {
    detail_in(&Catalogue::french(), &a_part_of_two_sketches(), operation)
}

const SEGMENT: SegmentId = SegmentId(0);
const CIRCLE: CircleId = CircleId(3);
const AWAY: DVec2 = DVec2::new(3.0, 0.0);
const RULE: Constraint = Constraint::Parallel {
    first: SegmentId(0),
    second: SegmentId(1),
};

fn scrubbed(
    elements: Vec<Element>,
    dimensions: Vec<DimensionTarget>,
    constraints: Vec<Constraint>,
) -> Operation {
    Operation::EraseMany {
        sketch: 0,
        elements,
        dimensions,
        constraints,
    }
}

/// One area, named by the one trait bounding it. What it is named by does
/// not matter here: the wording only counts them.
fn one_area() -> Vec<Area> {
    vec![Area {
        bounds: vec![vec![CurveId::Segment(SegmentId(0))]],
        inside: DVec2::ZERO,
    }]
}

fn raised(mode: ExtrusionMode) -> Operation {
    Operation::Extrude {
        sketch: 0,
        areas: one_area(),
        distance: 12.0.into(),
        mode,
    }
}

fn swept(axis: RevolutionAxis, mode: ExtrusionMode) -> Operation {
    Operation::Revolve {
        sketch: 0,
        areas: one_area(),
        axis,
        angle: 90.0.into(),
        mode,
    }
}

#[test]
fn a_drawn_step_is_named_after_the_shape_it_left_behind() {
    let here = PointRef::New(DVec2::ZERO);
    let drawn = [
        Operation::AddPoint {
            sketch: 0,
            position: DVec2::ZERO,
            on: Vec::new(),
        },
        Operation::AddSegment {
            sketch: 0,
            start: here.clone(),
            end: PointRef::New(AWAY),
            construction: false,
        },
        Operation::AddRectangle {
            sketch: 0,
            corner: here.clone(),
            opposite: PointRef::New(AWAY),
            construction: false,
        },
        Operation::AddCircle {
            sketch: 0,
            center: here,
            radius: 5.0,
            rim: Vec::new(),
            construction: false,
        },
        Operation::MovePoint {
            sketch: 0,
            point: PointId(1),
            position: AWAY,
            merged_into: None,
            let_go: false,
            on: Vec::new(),
        },
        Operation::MovePoint {
            sketch: 0,
            point: PointId(1),
            position: AWAY,
            merged_into: Some(PointId(2)),
            let_go: false,
            on: Vec::new(),
        },
        Operation::MoveMany {
            sketch: 0,
            points: vec![PointId(1)],
            by: AWAY,
        },
        Operation::MoveDimension {
            sketch: 0,
            target: DimensionTarget::Length(SEGMENT),
            offset: AWAY,
        },
        Operation::MergePoints {
            sketch: 0,
            kept: PointId(1),
            dropped: PointId(2),
        },
    ];
    let names = [
        "Point",
        "Trait",
        "Rectangle",
        "Cercle",
        "Déplacement",
        "Sommets fusionnés",
        "Déplacement",
        "Cote déplacée",
        "Sommets fusionnés",
    ];

    for (operation, reads) in drawn.iter().zip(names) {
        assert_eq!(said(operation), reads, "{operation:?} reads {reads:?}");
    }
}

#[test]
fn a_step_says_which_plane_which_rule_and_which_way_matter_went() {
    let turn = |mode| swept(RevolutionAxis::Sketch(SketchAxis::V), mode);

    assert_eq!(said(&raised(ExtrusionMode::Add)), "Extrusion 12 mm");
    assert_eq!(said(&raised(ExtrusionMode::Cut)), "Enlèvement 12 mm");
    assert_eq!(said(&turn(ExtrusionMode::Add)), "Révolution 90°");
    assert_eq!(said(&turn(ExtrusionMode::Cut)), "Révolution creusée 90°");
    let origin = Operation::CreateSketch {
        plane: WorkPlane::XY,
        on: None,
    };
    assert_eq!(said(&origin), "Esquisse — Plan XY");
    assert_eq!(
        said(&Operation::Constrain {
            sketch: 0,
            constraint: RULE,
        }),
        "Parallèle",
    );
}

#[test]
fn a_single_deletion_says_what_went_and_a_grouped_one_says_how_many() {
    let one = |element| scrubbed(vec![element], Vec::new(), Vec::new());
    let measure = DimensionTarget::Length(SEGMENT);

    assert_eq!(said(&one(Element::Point(PointId(1)))), "Point supprimé");
    assert_eq!(said(&one(Element::Segment(SEGMENT))), "Trait supprimé");
    assert_eq!(said(&one(Element::Circle(CIRCLE))), "Cercle supprimé");
    assert_eq!(
        said(&scrubbed(Vec::new(), vec![measure], Vec::new())),
        "Cote supprimée",
    );
    assert_eq!(
        said(&scrubbed(Vec::new(), Vec::new(), vec![RULE])),
        "Parallèle supprimée",
    );
    assert_eq!(
        said(&scrubbed(
            vec![Element::Point(PointId(1)), Element::Point(PointId(2))],
            vec![measure],
            vec![RULE],
        )),
        "4 éléments supprimés",
    );
}

#[test]
fn a_side_moved_and_a_shape_turned_say_so_and_unfold_into_what_they_did() {
    let moved = Operation::MoveSegment {
        sketch: 0,
        segment: SegmentId(2),
        by: DVec2::new(0.0, 15.0),
    };
    let turned = Operation::TurnShape {
        sketch: 0,
        points: vec![PointId(1), PointId(2), PointId(3), PointId(4)],
        about: DVec2::new(70.0, 52.5),
        angle: std::f64::consts::FRAC_PI_6,
    };

    assert_eq!(said(&moved), "Côté déplacé");
    assert_eq!(said(&turned), "Forme tournée");
    assert_eq!(unfolded(&moved), "Esquisse 1 · trait 2 de (0, 15)");
    assert_eq!(
        unfolded(&turned),
        "Esquisse 1 · 4 points tournés de 30° autour de (70, 52.5)"
    );
}

#[test]
fn a_rule_laid_in_place_of_a_dimension_is_named_after_the_rule() {
    let in_place = Operation::Gesture(vec![
        scrubbed(
            Vec::new(),
            vec![DimensionTarget::Length(SEGMENT)],
            Vec::new(),
        ),
        Operation::Constrain {
            sketch: 0,
            constraint: RULE,
        },
    ]);

    assert_eq!(said(&in_place), "Parallèle");
}

#[test]
fn the_history_says_a_radius_to_a_tenth_of_a_micron() {
    let circle = Operation::AddCircle {
        sketch: 1,
        center: PointRef::New(DVec2::ZERO),
        radius: 37.461_829_3,
        rim: Vec::new(),
        construction: false,
    };

    assert_eq!(unfolded(&circle), "Esquisse 2 · rayon 37.4618",);
}

#[test]
fn the_history_says_a_place_to_a_tenth_of_a_micron() {
    let point = Operation::AddPoint {
        sketch: 0,
        position: DVec2::new(12.5, 37.461_829_3),
        on: Vec::new(),
    };

    assert_eq!(unfolded(&point), "Esquisse 1 · (12.5, 37.4618)",);
}

fn on_the_top() -> WorkPlane {
    WorkPlane::from_normal(glam::DVec3::Z * 4.0, glam::DVec3::Z)
}

#[test]
fn an_unanchored_sketch_off_the_origin_reads_as_an_offset_plane() {
    let unanchored = Operation::CreateSketch {
        plane: on_the_top(),
        on: None,
    };

    assert_eq!(said(&unanchored), "Esquisse — Plan décalé");
}

#[test]
fn a_sketch_anchored_to_a_face_reads_as_a_face_of_the_part() {
    let anchored = Operation::CreateSketch {
        plane: on_the_top(),
        on: Some(cao_part::history::FaceAnchor {
            face: 0,
            up: glam::DVec3::Y,
        }),
    };

    assert_eq!(said(&anchored), "Esquisse — Face de la pièce");
}

#[test]
fn a_face_sketch_from_a_file_older_than_its_anchor_reads_as_an_offset_plane() {
    let mut written = serde_json::to_value(Operation::CreateSketch {
        plane: on_the_top(),
        on: None,
    })
    .expect("a sketch is written");
    let fields = written["CreateSketch"]
        .as_object_mut()
        .expect("a sketch is written as its fields");
    fields.retain(|name, _| name == "plane");
    let read: Operation =
        serde_json::from_value(written).expect("a sketch written before #363 is still read");

    assert_eq!(
        said(&read),
        "Esquisse — Plan décalé",
        "a sketch from before faces were recorded reads as a cut, which it may never have been",
    );
}
