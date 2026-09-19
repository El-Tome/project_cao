//! A point of a drawing dropped on a corner of the part.
//!
//! Closes #359.
//! - a point dropped on a corner is held there and follows that corner when
//!   the part changes — `a_point_on_a_corner_travels_with_it`
//! - it reads as fully determined — `a_point_on_a_corner_can_no_longer_move`
//! - a corner the part no longer has marks the drawing rather than catching
//!   the one next door — `a_corner_that_is_gone_leaves_the_point_where_it_was`
//! - a corner of the part is something a drawing can land on, and only the
//!   ones on its own plane are — `a_drawing_lands_on_the_corners_of_its_own_face`
//! - a hold the drawing's rules cannot honour gives, rather than the drawing
//!   coming apart — `a_hold_the_rules_cannot_honour_gives`, and one nothing
//!   contradicts stands — `a_hold_the_rules_can_honour_is_kept`
//! - a drawing its own rules already determine keeps its shape and every
//!   hold is let go — no test: the rule is the drawing's own, and
//!   `crates/sketch/src/sketch/anchored/tests.rs` puts it to a drawing
//!   directly, without a part's scale in the way

use cao_part::history::{ExtrusionMode, FaceAnchor, Operation, PointRef};
use cao_part::{History, PartState};
use cao_sketch::{Area, Constraint, PointId, SegmentId, SketchAxis, WorkPlane};
use glam::{DVec2, DVec3};

fn clicked(history: &History, sketch: usize, at: DVec2) -> Vec<Area> {
    PartState::rebuild(history).areas_at(sketch, &[at])
}

/// A block raised from a rectangle on XY, and a drawing started on its top.
fn a_block_with_a_drawing_on_top() -> History {
    let mut history = History::default();
    history.push(Operation::CreateSketch {
        plane: WorkPlane::XY,
        on: None,
    });
    history.push(Operation::AddRectangle {
        sketch: 0,
        corner: PointRef::New(DVec2::ZERO),
        opposite: PointRef::New(DVec2::new(40.0, 20.0)),
        construction: false,
    });
    history.push(Operation::Extrude {
        sketch: 0,
        areas: clicked(&history, 0, DVec2::new(20.0, 10.0)),
        distance: 10.0,
        mode: ExtrusionMode::Add,
    });
    history.push(Operation::CreateSketch {
        plane: WorkPlane {
            origin: DVec3::new(0.0, 0.0, 10.0),
            u: DVec3::X,
            v: DVec3::Y,
        },
        on: Some(FaceAnchor {
            face: 1,
            up: DVec3::Y,
        }),
    });
    history
}

/// The faces meeting at the corner of the block nearest this place, and where
/// it stands — what a click on it hands over.
fn corner_nearest(history: &History, at: DVec3) -> (Vec<usize>, DVec3) {
    let state = PartState::rebuild(history);
    let corner = state
        .body
        .corners()
        .into_iter()
        .min_by(|a, b| a.at.distance(at).total_cmp(&b.at.distance(at)))
        .expect("the part has corners");
    (corner.faces, corner.at)
}

/// Drags the far corner of the base rectangle, which is what widens the block.
fn widen(history: &mut History) {
    history.push(Operation::MovePoint {
        sketch: 0,
        point: PointId(2),
        position: DVec2::new(90.0, 20.0),
        merged_into: None,
    });
}

#[test]
fn a_point_on_a_corner_travels_with_it() {
    let mut history = a_block_with_a_drawing_on_top();
    let (faces, at) = corner_nearest(&history, DVec3::new(40.0, 20.0, 10.0));
    history.push(Operation::AddSegment {
        sketch: 1,
        start: PointRef::OnCorner {
            at: DVec2::new(at.x, at.y),
            faces,
        },
        end: PointRef::New(DVec2::new(5.0, 5.0)),
        construction: false,
    });
    let before = PartState::rebuild(&history).sketches[1].point(PointId(1));

    widen(&mut history);

    let after = PartState::rebuild(&history).sketches[1].point(PointId(1));
    assert!(
        (before - DVec2::new(40.0, 20.0)).length() < 1e-6,
        "the point was dropped on the far corner of the block: {before:?}",
    );
    assert!(
        (after - DVec2::new(90.0, 20.0)).length() < 1e-6,
        "the corner moved to x = 90 and the point went with it: {after:?}",
    );
}

#[test]
fn a_point_on_a_corner_can_no_longer_move() {
    let mut history = a_block_with_a_drawing_on_top();
    let (faces, at) = corner_nearest(&history, DVec3::new(40.0, 20.0, 10.0));
    history.push(Operation::AddSegment {
        sketch: 1,
        start: PointRef::OnCorner {
            at: DVec2::new(at.x, at.y),
            faces,
        },
        end: PointRef::New(DVec2::new(5.0, 5.0)),
        construction: false,
    });

    let state = PartState::rebuild(&history);
    let drawing = &state.sketches[1];

    assert!(drawing.is_anchored(PointId(1)));
    assert!(
        drawing.settled_points(1.0)[1],
        "the part holds it, so the drawing reads it the way it reads a point \
         nothing can move",
    );
    assert!(
        !drawing.settled_points(1.0)[2],
        "the other end is still loose, or this would prove nothing",
    );
}

#[test]
fn a_corner_that_is_gone_leaves_the_point_where_it_was() {
    let mut history = a_block_with_a_drawing_on_top();
    let stood_at = DVec2::new(40.0, 20.0);
    history.push(Operation::AddSegment {
        sketch: 1,
        start: PointRef::OnCorner {
            at: stood_at,
            // Faces the part has never had.
            faces: vec![90, 91, 92],
        },
        end: PointRef::New(DVec2::new(5.0, 5.0)),
        construction: false,
    });

    let state = PartState::rebuild(&history);

    assert!(
        (state.sketches[1].point(PointId(1)) - stood_at).length() < 1e-9,
        "it stays where it was drawn rather than jumping to a corner nobody \
         pointed at",
    );
    assert!(!state.sketches[1].is_anchored(PointId(1)));
    assert!(state.is_unanchored(1), "and the drawing says so");
}

#[test]
fn a_drawing_lands_on_the_corners_of_its_own_face() {
    let history = a_block_with_a_drawing_on_top();
    let state = PartState::rebuild(&history);

    let offered = state.corners_on(1);

    assert_eq!(
        offered.len(),
        4,
        "the four corners of the top, and not the four underneath: a point \
         landed on a corner standing somewhere else would be drawn against \
         its shadow — {offered:?}",
    );
    let mut places: Vec<(i64, i64)> = offered
        .iter()
        .map(|(at, _)| (at.x.round() as i64, at.y.round() as i64))
        .collect();
    places.sort_unstable();
    assert_eq!(places, [(0, 0), (0, 20), (40, 0), (40, 20)]);
    assert_eq!(
        state.corners_on(0).len(),
        4,
        "the first drawing is on XY: the four corners of the bottom sit on \
         it, and the four of the top do not",
    );
}

/// A drawing on the top face, with a trait laid across two corners of the
/// part that do not stand level with each other.
fn a_trait_across_two_corners() -> History {
    let mut history = a_block_with_a_drawing_on_top();
    let (near, at_near) = corner_nearest(&history, DVec3::new(0.0, 0.0, 10.0));
    let (far, at_far) = corner_nearest(&history, DVec3::new(40.0, 20.0, 10.0));
    history.push(Operation::AddSegment {
        sketch: 1,
        start: PointRef::OnCorner {
            at: DVec2::new(at_near.x, at_near.y),
            faces: near,
        },
        end: PointRef::OnCorner {
            at: DVec2::new(at_far.x, at_far.y),
            faces: far,
        },
        construction: false,
    });
    history
}

#[test]
fn a_hold_the_rules_cannot_honour_gives() {
    let mut history = a_trait_across_two_corners();
    // The two corners stand twenty apart across the face, and this asks the
    // trait between them to lie flat along the drawing's own axis. Held at
    // both ends, it cannot.
    history.push(Operation::Constrain {
        sketch: 1,
        constraint: Constraint::AxisCollinear {
            segment: SegmentId(0),
            axis: SketchAxis::U,
        },
    });

    let state = PartState::rebuild(&history);
    let drawing = &state.sketches[1];
    let (start, end) = drawing.endpoints(SegmentId(0));

    assert!(
        (start.y - end.y).abs() < 1e-3,
        "the rule is what the drawing keeps: {start:?} to {end:?}",
    );
    assert!(
        !drawing.is_anchored(PointId(1)) && !drawing.is_anchored(PointId(2)),
        "both holds gave, rather than the drawing being pulled apart between \
         them",
    );
    assert!(state.has_let_go(1), "and the drawing says it gave");
}

#[test]
fn a_hold_the_rules_can_honour_is_kept() {
    let state = PartState::rebuild(&a_trait_across_two_corners());

    assert!(
        state.sketches[1].is_anchored(PointId(1)),
        "nothing contradicts the holds, so they stand",
    );
    assert!(!state.has_let_go(1));
}
