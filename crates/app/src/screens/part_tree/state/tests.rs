//! What the tree of a part holds.
//!
//! Closes #345.
//! - an area one of whose bounding traits is erased raises nothing, and the
//!   tree says which step lost it —
//!   `a_step_whose_area_is_gone_says_so_rather_than_reading_as_one_raised_from_nothing`
//! - a step that still has every area it stands on is not marked —
//!   `a_step_whose_areas_are_all_there_is_not_marked`

use cao_part::history::{FaceAnchor, PointRef};
use cao_sketch::{DimensionTarget, Element, SegmentId, WorkPlane};
use chrono::{DateTime, Utc};
use glam::{DVec2, DVec3};

use super::*;

fn at(text: &str) -> DateTime<Utc> {
    text.parse().expect("a date")
}

fn french() -> Catalogue {
    Catalogue::french()
}

/// A rectangle on the XY plane, raised into a block, and a second sketch.
fn a_part() -> PartDocument {
    let mut document = PartDocument::new("Test", at("2026-01-02T09:00:00Z"));
    document.apply(Operation::CreateSketch {
        plane: WorkPlane::XY,
        on: None,
    });
    document.apply(Operation::AddRectangle {
        sketch: 0,
        corner: PointRef::New(DVec2::ZERO),
        opposite: PointRef::New(DVec2::new(10.0, 20.0)),
        construction: false,
    });
    document.apply(Operation::SetDimension {
        sketch: 0,
        target: DimensionTarget::Length(SegmentId(0)),
        value: 100.0,
        placement: None,
    });
    document.apply(Operation::Extrude {
        sketch: 0,
        areas: document.areas_at(0, &[DVec2::new(5.0, 10.0)]),
        distance: 4.0,
        mode: ExtrusionMode::Add,
    });
    document.apply(Operation::CreateSketch {
        plane: WorkPlane::XZ,
        on: None,
    });
    document
}

#[test]
fn the_tree_holds_the_bodies_and_the_sketches_and_not_the_run_of_steps() {
    let tree = PartTree::of(&a_part(), &french());

    assert_eq!(tree.bodies.len(), 1);
    assert_eq!(tree.sketches.len(), 2);
    assert_eq!(tree.bodies[0].name, "Extrusion 1");
    assert_eq!(tree.sketches[0].name, "Esquisse 1");
    assert_eq!(tree.sketches[1].name, "Esquisse 2");
}

#[test]
fn a_step_that_made_matter_reads_how_far_it_went_and_which_way() {
    let tree = PartTree::of(&a_part(), &french());

    assert_eq!(tree.bodies[0].reads, "4 mm · ajout de matière");
}

#[test]
fn a_step_whose_area_is_gone_says_so_rather_than_reading_as_one_raised_from_nothing() {
    let mut document = a_part();
    document.apply(Operation::EraseMany {
        sketch: 0,
        elements: vec![Element::Segment(SegmentId(0))],
        dimensions: Vec::new(),
        constraints: Vec::new(),
    });

    let tree = PartTree::of(&document, &french());

    assert!(tree.bodies[0].lost);
    assert!(
        tree.bodies[0].areas.is_empty(),
        "there is nothing left to point at",
    );
}

#[test]
fn a_step_whose_areas_are_all_there_is_not_marked() {
    let tree = PartTree::of(&a_part(), &french());

    assert!(!tree.bodies[0].lost);
}

#[test]
fn a_step_that_made_matter_names_the_areas_it_stands_on() {
    let tree = PartTree::of(&a_part(), &french());

    assert_eq!(
        tree.bodies[0].areas,
        [Row {
            name: "Aire 1".to_string(),
            points: Points::Area { sketch: 0, rank: 0 },
        }],
    );
}

#[test]
fn a_sketch_holds_what_is_drawn_on_it_sorted_by_kind() {
    let tree = PartTree::of(&a_part(), &french());
    let drawing = &tree.sketches[0];

    assert_eq!(drawing.areas.len(), 1, "the rectangle encloses one area");
    assert_eq!(drawing.strokes.len(), 4, "and stands on four traits");
    assert_eq!(drawing.dimensions.len(), 1);
    assert_eq!(drawing.strokes[0].name, "Trait 1");
    assert_eq!(drawing.dimensions[0].name, "Cote 1");
}

#[test]
fn an_area_named_under_a_body_is_the_one_its_sketch_names() {
    let tree = PartTree::of(&a_part(), &french());

    assert_eq!(tree.bodies[0].areas[0], tree.sketches[0].areas[0]);
}

#[test]
fn a_drag_leaves_no_line_of_its_own() {
    let mut document = a_part();
    let before = PartTree::of(&document, &french());

    document.apply(Operation::MovePoint {
        sketch: 0,
        point: cao_sketch::PointId(2),
        position: DVec2::new(11.0, 21.0),
        merged_into: None,
    });

    let after = PartTree::of(&document, &french());
    assert_eq!(
        after.sketches[0].strokes, before.sketches[0].strokes,
        "the tree shows what the part is made of, and a drag leaves no element \
         behind it",
    );
}

#[test]
fn a_part_with_nothing_drawn_on_it_holds_nothing() {
    let document = PartDocument::new("Test", at("2026-01-02T09:00:00Z"));

    assert!(PartTree::of(&document, &french()).is_empty());
}

#[test]
fn a_sketch_that_lost_its_face_is_marked_in_the_tree() {
    let mut document = PartDocument::new("Test", at("2026-01-02T09:00:00Z"));
    document.apply(Operation::CreateSketch {
        plane: WorkPlane::XY,
        on: Some(FaceAnchor {
            face: 404,
            up: DVec3::Y,
        }),
    });

    let tree = PartTree::of(&document, &french());

    assert!(
        tree.sketches[0].adrift,
        "a drawing whose face the part no longer has must say so",
    );
}
