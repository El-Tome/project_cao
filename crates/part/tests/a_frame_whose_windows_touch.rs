//! A frame whose windows touch each other, raised: one opening where they
//! touch, and the matter the frame measures.
//!
//! Closes #540.
//! - the three drawings, whole side, part of a side and one corner, are tests
//!   through `PartDocument`, each frame raising its measure times its depth,
//!   closed and uncrossed, with no face standing between the windows —
//!   `windows_sharing_a_whole_side_leave_the_frame_one_opening`,
//!   `windows_sharing_part_of_a_side_leave_the_frame_one_opening`,
//!   `windows_touching_at_one_corner_leave_the_frame_one_opening`; the same
//!   on a part already on the flats —
//!   `windows_touching_in_a_frame_raised_by_the_flats_leave_one_opening`
//! - the frame's matter equals its measure in each of them, and it is what
//!   the extrusion panel and the measure tool light —
//!   `the_matter_of_a_frame_whose_windows_touch_is_its_measure`
//! - window B drawn after the frame was raised leaves a part of 5000 mm³ —
//!   `a_window_drawn_beside_another_after_the_raise_leaves_the_frame_raised`
//! - the corridors `triangulate` lets through on purpose stay seams: a trait
//!   joining two windows, or a window to its frame, stands no wall along
//!   itself —
//!   `windows_a_trait_joins_leave_the_frame_two_openings_and_no_wall_along_the_trait`,
//!   `a_trait_from_the_frame_to_a_window_never_stands_a_wall_in_the_matter`
//! - the owner's notch and the controls still give their figures —
//!   `a_rectangle_in_a_notch_of_a_square_leaves_two_areas_and_raises_whole`,
//!   `windows_a_millimetre_apart_leave_the_frame_its_matter`,
//!   `windows_touching_cut_from_a_block_leave_it_its_matter`
//! - #503's ring, #529's hole touching its outline, #493 and #494 hold — no
//!   test: held by the tests the issue names, unchanged and still passing
//! - #500's campaign gives what it gave before — no test: run by hand on main
//!   and on this branch over the same 20000 seeds, the same 1732 drawings
//!   break the same rules

use cao_part::PartDocument;
use cao_part::history::{ExtrusionMode, Operation, PointRef};
use cao_sketch::{PointId, Region, WorkPlane};
use cao_solid::soundness;
use chrono::{DateTime, Utc};
use glam::{DVec2, DVec3};

const DEPTH: f64 = 10.0;

/// How near the arithmetic a raised volume or surface comes: what is left
/// between the two is rounding.
const EXACT: f64 = 1e-9;

fn at(text: &str) -> DateTime<Utc> {
    text.parse().expect("a date")
}

fn a_drawing() -> PartDocument {
    let mut document = PartDocument::new("Frame", at("2026-10-08T09:00:00Z"));
    document.apply(Operation::CreateSketch {
        plane: WorkPlane::XY,
        on: None,
    });
    document
}

/// The point of the drawing standing at `place`, as the rectangle tool snaps
/// a corner started on it.
fn point_at(document: &PartDocument, place: DVec2) -> PointId {
    let found = document.sketches()[0]
        .points()
        .iter()
        .position(|point| point.distance(place) < 1e-9)
        .unwrap_or_else(|| panic!("a point of the drawing at {place}"));
    PointId(found)
}

fn rectangle(document: &mut PartDocument, corner: PointRef, opposite: DVec2) {
    document.apply(Operation::AddRectangle {
        sketch: 0,
        corner,
        opposite: PointRef::New(opposite),
        construction: false,
    });
}

/// The frame (0,0)–(30,30) and window A (5,5)–(15,25) in it.
fn a_frame_and_one_window() -> PartDocument {
    let mut document = a_drawing();
    rectangle(
        &mut document,
        PointRef::New(DVec2::ZERO),
        DVec2::splat(30.0),
    );
    rectangle(
        &mut document,
        PointRef::New(DVec2::splat(5.0)),
        DVec2::new(15.0, 25.0),
    );
    document
}

/// Window B, from `corner` — a corner of window A, snapped — to `opposite`.
fn window_from_a_corner(document: &mut PartDocument, corner: DVec2, opposite: DVec2) {
    let snapped = point_at(document, corner);
    rectangle(document, PointRef::Existing(snapped), opposite);
}

/// Window B sharing A's whole right side.
fn a_whole_side() -> PartDocument {
    let mut document = a_frame_and_one_window();
    window_from_a_corner(&mut document, DVec2::new(15.0, 5.0), DVec2::new(25.0, 25.0));
    document
}

/// Window B lying along the middle of A's right side.
fn part_of_a_side() -> PartDocument {
    let mut document = a_frame_and_one_window();
    document.apply(Operation::AddRectangle {
        sketch: 0,
        corner: PointRef::New(DVec2::new(15.0, 10.0)),
        opposite: PointRef::New(DVec2::new(25.0, 20.0)),
        construction: false,
    });
    document
}

/// Window A (5,5)–(15,15) and window B (15,15)–(25,25), meeting at a corner.
fn one_corner() -> PartDocument {
    let mut document = a_drawing();
    rectangle(
        &mut document,
        PointRef::New(DVec2::ZERO),
        DVec2::splat(30.0),
    );
    rectangle(
        &mut document,
        PointRef::New(DVec2::splat(5.0)),
        DVec2::splat(15.0),
    );
    window_from_a_corner(&mut document, DVec2::splat(15.0), DVec2::splat(25.0));
    document
}

fn raise(document: &mut PartDocument, place: DVec2, mode: ExtrusionMode) {
    let areas = document.areas_at(0, &[place]);
    assert!(!areas.is_empty(), "an area at {place}");
    document.apply(Operation::Extrude {
        sketch: 0,
        areas,
        distance: DEPTH.into(),
        mode,
    });
}

/// The frame: the area the point (2, 2), inside its left band, falls in.
fn the_frame(document: &PartDocument) -> Region {
    document.sketches()[0]
        .regions()
        .into_iter()
        .filter(|region| region.contains(DVec2::splat(2.0)))
        .max_by_key(|region| region.depth)
        .expect("the frame is an area")
}

fn surface(triangles: &[[DVec3; 3]]) -> f64 {
    triangles
        .iter()
        .map(|[a, b, c]| (*b - *a).cross(*c - *a).length() / 2.0)
        .sum()
}

fn assert_near(made: f64, expected: f64, what: &str) {
    assert!(
        (made - expected).abs() <= EXACT * expected,
        "{what}: {made} where the arithmetic gives {expected}",
    );
}

/// The frame raised alone: its measure times the depth, closed, uncrossed,
/// and no more surface than its two caps and the walls round its outline and
/// round its openings, `opening` round in all — a wall between the windows
/// would add to it.
fn assert_raised_round_its_openings(mut document: PartDocument, measure: f64, opening: f64) {
    assert_near(the_frame(&document).area(), measure, "the frame's measure");
    raise(&mut document, DVec2::splat(2.0), ExtrusionMode::Add);

    let body = document.body();
    let triangles = body.triangles();
    soundness::closed(&triangles).expect("the frame is raised closed");
    soundness::uncrossed(&triangles).expect("the frame is raised uncrossed");
    assert_near(body.volume(), measure * DEPTH, "the frame raised");
    let caps = 2.0 * measure;
    let walls = (120.0 + opening) * DEPTH;
    assert_near(surface(&triangles), caps + walls, "the frame's surface");
}

#[test]
fn windows_sharing_a_whole_side_leave_the_frame_one_opening() {
    assert_raised_round_its_openings(a_whole_side(), 500.0, 80.0);
}

#[test]
fn windows_sharing_part_of_a_side_leave_the_frame_one_opening() {
    assert_raised_round_its_openings(part_of_a_side(), 600.0, 80.0);
}

#[test]
fn windows_touching_at_one_corner_leave_the_frame_one_opening() {
    assert_raised_round_its_openings(one_corner(), 700.0, 80.0);
}

#[test]
fn windows_touching_in_a_frame_raised_by_the_flats_leave_one_opening() {
    for (drawing, measure) in [
        (a_whole_side as fn() -> PartDocument, 500.0),
        (part_of_a_side, 600.0),
        (one_corner, 700.0),
    ] {
        let mut document = drawing();
        document.apply(Operation::AddEllipse {
            sketch: 0,
            center: PointRef::New(DVec2::new(60.0, 15.0)),
            first: [
                PointRef::New(DVec2::new(50.0, 15.0)),
                PointRef::New(DVec2::new(70.0, 15.0)),
            ],
            second: [
                PointRef::New(DVec2::new(60.0, 10.0)),
                PointRef::New(DVec2::new(60.0, 20.0)),
            ],
            construction: false,
            drawn: None,
        });
        raise(&mut document, DVec2::new(60.0, 15.0), ExtrusionMode::Add);
        let oval = document.body().volume();
        raise(&mut document, DVec2::splat(2.0), ExtrusionMode::Add);

        let triangles = document.body().triangles();
        soundness::closed(&triangles).expect("the part is raised closed");
        soundness::uncrossed(&triangles).expect("the part is raised uncrossed");
        let frame = document.body().volume() - oval;
        assert_near(frame, measure * DEPTH, "the frame's share, by the flats");
    }
}

fn covered(triangles: &[[DVec2; 3]], place: DVec2) -> bool {
    triangles.iter().any(|[a, b, c]| {
        let sides =
            [(*a, *b), (*b, *c), (*c, *a)].map(|(from, to)| (to - from).perp_dot(place - from));
        sides.iter().all(|side| *side >= 0.0) || sides.iter().all(|side| *side <= 0.0)
    })
}

#[test]
fn the_matter_of_a_frame_whose_windows_touch_is_its_measure() {
    for (drawing, windows) in [
        (
            a_whole_side as fn() -> PartDocument,
            [DVec2::new(10.0, 15.0), DVec2::new(20.0, 15.0)],
        ),
        (
            part_of_a_side,
            [DVec2::new(10.0, 15.0), DVec2::new(20.0, 15.0)],
        ),
        (one_corner, [DVec2::splat(10.0), DVec2::splat(20.0)]),
    ] {
        let frame = the_frame(&drawing());
        let lit = frame.face_triangles();
        let matter: f64 = lit
            .iter()
            .map(|[a, b, c]| (*b - *a).perp_dot(*c - *a).abs() / 2.0)
            .sum();
        assert_near(matter, frame.area(), "the frame's matter");
        for place in [
            DVec2::splat(2.0),
            DVec2::new(2.0, 15.0),
            DVec2::new(15.0, 2.0),
            DVec2::new(28.0, 28.0),
        ] {
            assert!(covered(&lit, place), "the frame is lit at {place}");
        }
        for window in windows {
            assert!(
                !covered(&lit, window),
                "the window at {window} is left dark"
            );
        }
    }
}

#[test]
fn a_window_drawn_beside_another_after_the_raise_leaves_the_frame_raised() {
    let mut document = a_frame_and_one_window();
    raise(&mut document, DVec2::splat(2.0), ExtrusionMode::Add);
    assert_near(document.body().volume(), 7000.0, "the frame raised with A");

    window_from_a_corner(&mut document, DVec2::new(15.0, 5.0), DVec2::new(25.0, 25.0));

    assert_near(
        document.body().volume(),
        5000.0,
        "the frame once B is drawn",
    );
}

#[test]
fn a_rectangle_in_a_notch_of_a_square_leaves_two_areas_and_raises_whole() {
    let mut document = a_drawing();
    rectangle(
        &mut document,
        PointRef::New(DVec2::ZERO),
        DVec2::splat(40.0),
    );
    rectangle(
        &mut document,
        PointRef::New(DVec2::new(0.0, 20.0)),
        DVec2::new(15.0, 30.0),
    );
    let mut measures: Vec<f64> = document.sketches()[0]
        .regions()
        .iter()
        .map(Region::area)
        .collect();
    measures.sort_by(f64::total_cmp);
    assert_eq!(measures.len(), 2, "two areas: {measures:?}");
    assert_near(measures[0], 150.0, "the notch");
    assert_near(measures[1], 1450.0, "the square around it");

    raise(&mut document, DVec2::splat(2.0), ExtrusionMode::Add);

    let triangles = document.body().triangles();
    soundness::closed(&triangles).expect("the square is raised closed");
    soundness::uncrossed(&triangles).expect("the square is raised uncrossed");
    assert_near(document.body().volume(), 14500.0, "the square raised");
}

#[test]
fn windows_a_trait_joins_leave_the_frame_two_openings_and_no_wall_along_the_trait() {
    let mut document = a_drawing();
    rectangle(
        &mut document,
        PointRef::New(DVec2::ZERO),
        DVec2::splat(30.0),
    );
    rectangle(
        &mut document,
        PointRef::New(DVec2::splat(5.0)),
        DVec2::splat(10.0),
    );
    rectangle(
        &mut document,
        PointRef::New(DVec2::splat(20.0)),
        DVec2::splat(25.0),
    );
    let ends = [10.0, 20.0].map(|at| PointRef::Existing(point_at(&document, DVec2::splat(at))));
    document.apply(Operation::AddSegment {
        sketch: 0,
        start: ends[0].clone(),
        end: ends[1].clone(),
        construction: false,
    });

    assert_raised_round_its_openings(document, 850.0, 40.0);
}

#[test]
fn a_trait_from_the_frame_to_a_window_never_stands_a_wall_in_the_matter() {
    let mut document = a_drawing();
    rectangle(
        &mut document,
        PointRef::New(DVec2::ZERO),
        DVec2::splat(30.0),
    );
    rectangle(
        &mut document,
        PointRef::New(DVec2::splat(10.0)),
        DVec2::splat(20.0),
    );
    let ends = [0.0, 10.0].map(|at| PointRef::Existing(point_at(&document, DVec2::splat(at))));
    document.apply(Operation::AddSegment {
        sketch: 0,
        start: ends[0].clone(),
        end: ends[1].clone(),
        construction: false,
    });
    raise(&mut document, DVec2::new(2.0, 28.0), ExtrusionMode::Add);

    let triangles = document.body().triangles();
    soundness::closed(&triangles).expect("whatever is raised is closed");
    soundness::uncrossed(&triangles).expect("whatever is raised is uncrossed");
}

#[test]
fn windows_a_millimetre_apart_leave_the_frame_its_matter() {
    let mut document = a_drawing();
    rectangle(
        &mut document,
        PointRef::New(DVec2::ZERO),
        DVec2::splat(30.0),
    );
    rectangle(
        &mut document,
        PointRef::New(DVec2::splat(5.0)),
        DVec2::new(14.5, 25.0),
    );
    rectangle(
        &mut document,
        PointRef::New(DVec2::new(15.5, 5.0)),
        DVec2::new(25.0, 25.0),
    );
    raise(&mut document, DVec2::splat(2.0), ExtrusionMode::Add);

    assert_near(document.body().volume(), 5200.0, "the frame raised");
}

#[test]
fn windows_touching_cut_from_a_block_leave_it_its_matter() {
    let mut document = a_drawing();
    rectangle(
        &mut document,
        PointRef::New(DVec2::ZERO),
        DVec2::splat(30.0),
    );
    raise(&mut document, DVec2::splat(2.0), ExtrusionMode::Add);
    document.apply(Operation::CreateSketch {
        plane: WorkPlane::XY,
        on: None,
    });
    let window = |document: &mut PartDocument, corner: DVec2, opposite: DVec2| {
        document.apply(Operation::AddRectangle {
            sketch: 1,
            corner: PointRef::New(corner),
            opposite: PointRef::New(opposite),
            construction: false,
        });
    };
    window(&mut document, DVec2::splat(5.0), DVec2::new(15.0, 25.0));
    window(&mut document, DVec2::new(15.0, 5.0), DVec2::new(25.0, 25.0));
    let areas = document.areas_at(1, &[DVec2::new(10.0, 15.0), DVec2::new(20.0, 15.0)]);
    assert_eq!(areas.len(), 2, "both windows are clicked");
    document.apply(Operation::Extrude {
        sketch: 1,
        areas,
        distance: DEPTH.into(),
        mode: ExtrusionMode::Cut,
    });

    let triangles = document.body().triangles();
    soundness::closed(&triangles).expect("the block is cut closed");
    soundness::uncrossed(&triangles).expect("the block is cut uncrossed");
    assert_near(document.body().volume(), 5000.0, "the block cut");
}
