//! What a step of matter stands on when its area shares a stretch of border
//! with another curve of the drawing.
//!
//! A stretch two curves lie along is one border (#493), and both curves name
//! it: either one still standing there keeps the area its name. Erasing the
//! neighbour whose side runs along it, or the first of two copies of a trait,
//! leaves the shape as closed as it was, and the matter stands.
//!
//! Closes #493.
//! - an area keeps its name when the neighbour sharing part of a side is
//!   erased — `erasing_the_neighbour_along_part_of_a_side_keeps_the_matter`
//! - and when the side it shares lies wholly along the neighbour's —
//!   `erasing_the_neighbour_whose_side_holds_a_short_one_keeps_the_matter`
//! - an area closed by a trait drawn twice keeps its name when either copy is
//!   erased — `erasing_the_first_copy_of_a_trait_drawn_twice_keeps_the_matter`,
//!   `erasing_the_second_copy_of_a_trait_drawn_twice_keeps_the_matter`
//! - a disc whose circle was drawn twice keeps its name when the first is
//!   erased — `erasing_the_first_of_two_circles_drawn_on_each_other_keeps_the_matter`
//! - two shapes sharing a stretch, pulled apart, each keep their matter —
//!   `pulling_the_second_rectangle_off_the_first_keeps_its_matter`
//! - a side lying wholly along the neighbour's can go, the neighbour still
//!   closing the shape —
//!   `erasing_a_side_that_lies_wholly_along_the_neighbour_keeps_the_matter`
//! - a border really lost still loses the area —
//!   `erasing_a_side_of_its_own_still_leaves_no_matter`

use cao_part::history::{ExtrusionMode, Operation, PointRef};
use cao_part::{History, PartState};
use cao_sketch::{CircleId, Element, PointId, SegmentId, WorkPlane};
use glam::DVec2;

const DEPTH: f64 = 4.0;

/// A drawing holding two rectangles, each given by two opposite corners. The
/// first one's sides are traits 0 to 3, bottom, right, top, left; the second
/// one's are 4 to 7 in the same order.
fn two_rectangles(first: [DVec2; 2], second: [DVec2; 2]) -> History {
    let mut history = History::default();
    history.push(Operation::CreateSketch {
        plane: WorkPlane::XY,
        on: None,
    });
    for [corner, opposite] in [first, second] {
        history.push(Operation::AddRectangle {
            sketch: 0,
            corner: PointRef::New(corner),
            opposite: PointRef::New(opposite),
            construction: false,
        });
    }
    history
}

/// A square two across, its left side drawn twice between the same two
/// corners: trait 3 first, then trait 4.
fn a_square_closed_twice() -> History {
    let mut history = History::default();
    history.push(Operation::CreateSketch {
        plane: WorkPlane::XY,
        on: None,
    });
    let corners = [
        DVec2::ZERO,
        DVec2::new(2.0, 0.0),
        DVec2::new(2.0, 2.0),
        DVec2::new(0.0, 2.0),
    ];
    for step in 0..3 {
        history.push(Operation::AddSegment {
            sketch: 0,
            start: match step {
                0 => PointRef::New(corners[0]),
                _ => PointRef::Existing(PointId(step + 1)),
            },
            end: PointRef::New(corners[step + 1]),
            construction: false,
        });
    }
    for (start, end) in [(4, 1), (1, 4)] {
        history.push(Operation::AddSegment {
            sketch: 0,
            start: PointRef::Existing(PointId(start)),
            end: PointRef::Existing(PointId(end)),
            construction: false,
        });
    }
    history
}

fn raise(history: &mut History, place: DVec2) {
    history.push(Operation::Extrude {
        sketch: 0,
        areas: PartState::rebuild(history).areas_at(0, &[place]),
        distance: DEPTH.into(),
        mode: ExtrusionMode::Add,
    });
}

fn erase(history: &mut History, segments: impl IntoIterator<Item = usize>) {
    erase_elements(
        history,
        segments
            .into_iter()
            .map(|segment| Element::Segment(SegmentId(segment)))
            .collect(),
    );
}

fn erase_elements(history: &mut History, elements: Vec<Element>) {
    history.push(Operation::EraseMany {
        sketch: 0,
        elements,
        dimensions: Vec::new(),
        constraints: Vec::new(),
    });
}

fn raised(history: &History) -> f64 {
    PartState::rebuild(history).body.volume()
}

/// Raises the area under the place, checks it was raised as an area of that
/// size, then erases the traits and returns what is raised afterwards.
fn raised_once_erased(
    mut history: History,
    place: DVec2,
    area: f64,
    erased: impl IntoIterator<Item = usize>,
) -> f64 {
    raise(&mut history, place);
    let before = raised(&history);
    assert!(
        (before - area * DEPTH).abs() < 1e-6,
        "the area under {place} is raised on its own before anything is erased: {before}",
    );
    erase(&mut history, erased);
    raised(&history)
}

#[test]
fn erasing_the_neighbour_along_part_of_a_side_keeps_the_matter() {
    let history = two_rectangles(
        [DVec2::ZERO, DVec2::new(1.0, 2.0)],
        [DVec2::new(1.0, 1.0), DVec2::new(2.0, 3.0)],
    );
    let after = raised_once_erased(history, DVec2::new(1.5, 2.0), 2.0, 0..4);
    assert!(
        (after - 2.0 * DEPTH).abs() < 1e-6,
        "the second rectangle is as closed as it was, its own side now bounding \
         it where the first one's did: {after}",
    );
}

#[test]
fn erasing_the_neighbour_whose_side_holds_a_short_one_keeps_the_matter() {
    let history = two_rectangles(
        [DVec2::ZERO, DVec2::new(1.0, 3.0)],
        [DVec2::new(1.0, 1.0), DVec2::new(2.0, 2.0)],
    );
    let after = raised_once_erased(history, DVec2::new(1.5, 1.5), 1.0, 0..4);
    assert!(
        (after - DEPTH).abs() < 1e-6,
        "the short side lay wholly along the long one, and bounds the area \
         alone once the long one is gone: {after}",
    );
}

#[test]
fn erasing_the_first_copy_of_a_trait_drawn_twice_keeps_the_matter() {
    let after = raised_once_erased(a_square_closed_twice(), DVec2::ONE, 4.0, [3]);
    assert!(
        (after - 4.0 * DEPTH).abs() < 1e-6,
        "the second copy still closes the square: {after}",
    );
}

#[test]
fn erasing_the_second_copy_of_a_trait_drawn_twice_keeps_the_matter() {
    let after = raised_once_erased(a_square_closed_twice(), DVec2::ONE, 4.0, [4]);
    assert!(
        (after - 4.0 * DEPTH).abs() < 1e-6,
        "the first copy still closes the square: {after}",
    );
}

#[test]
fn erasing_the_first_of_two_circles_drawn_on_each_other_keeps_the_matter() {
    let mut history = History::default();
    history.push(Operation::CreateSketch {
        plane: WorkPlane::XY,
        on: None,
    });
    for _ in 0..2 {
        history.push(Operation::AddCircle {
            sketch: 0,
            center: PointRef::New(DVec2::new(8.0, 4.0)),
            radius: 2.0,
            rim: Vec::new(),
            construction: false,
        });
    }
    raise(&mut history, DVec2::new(8.0, 4.0));
    let before = raised(&history);
    let disc = std::f64::consts::PI * 4.0 * DEPTH;
    assert!(
        (before - disc).abs() / disc < 0.02,
        "the disc is raised once before anything is erased: {before}",
    );

    erase_elements(&mut history, vec![Element::Circle(CircleId(0))]);

    let after = raised(&history);
    assert!(
        (after - before).abs() < 1e-6,
        "the second circle still closes the disc: {before} before, {after} after",
    );
}

#[test]
fn pulling_the_second_rectangle_off_the_first_keeps_its_matter() {
    let mut history = two_rectangles(
        [DVec2::ZERO, DVec2::new(1.0, 2.0)],
        [DVec2::new(1.0, 1.0), DVec2::new(2.0, 3.0)],
    );
    raise(&mut history, DVec2::new(1.5, 2.0));
    assert!((raised(&history) - 2.0 * DEPTH).abs() < 1e-6);

    // The second rectangle's four corners, after the origin and the first's.
    history.push(Operation::MoveMany {
        sketch: 0,
        points: (5..9).map(PointId).collect(),
        by: DVec2::new(5.0, 0.0),
    });

    let after = raised(&history);
    assert!(
        (after - 2.0 * DEPTH).abs() < 1e-6,
        "the second rectangle no longer touches the first, and is bounded by \
         its own four sides: {after}",
    );
}

#[test]
fn erasing_a_side_that_lies_wholly_along_the_neighbour_keeps_the_matter() {
    let history = two_rectangles(
        [DVec2::ZERO, DVec2::new(1.0, 3.0)],
        [DVec2::new(1.0, 1.0), DVec2::new(2.0, 2.0)],
    );
    let after = raised_once_erased(history, DVec2::new(1.5, 1.5), 1.0, [7]);
    assert!(
        (after - DEPTH).abs() < 1e-6,
        "the first rectangle's side still closes the second one: {after}",
    );
}

#[test]
fn erasing_a_side_of_its_own_still_leaves_no_matter() {
    let history = two_rectangles(
        [DVec2::ZERO, DVec2::new(1.0, 2.0)],
        [DVec2::new(1.0, 1.0), DVec2::new(2.0, 3.0)],
    );
    let after = raised_once_erased(history, DVec2::new(1.5, 2.0), 2.0, [6]);
    assert!(
        after.abs() < 1e-6,
        "the second rectangle has lost its top: the step stands on nothing, \
         rather than on some other area — {after}",
    );
}
