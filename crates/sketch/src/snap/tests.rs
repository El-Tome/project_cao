//! What sketch · snap.rs is held to.

use super::*;
use crate::WorkPlane;

fn settings() -> SnapSettings {
    SnapSettings {
        point_reach: 1.0,
        curve_reach: 1.0,
        grid_step: Some(10.0),
        grid_reach: 2.0,
    }
}

fn with_two_crossing_traits() -> Sketch {
    let mut sketch = with_a_trait(DVec2::new(0.0, 0.0), DVec2::new(10.0, 0.0));
    let from = sketch.add_point(DVec2::new(4.0, -2.0));
    let to = sketch.add_point(DVec2::new(4.0, 6.0));
    sketch.add_segment(from, to);
    sketch
}

/// The two traits above cross here, and no point of the drawing stands on
/// it — which is what makes it a crossing rather than a point.
const WHERE_THEY_CROSS: DVec2 = DVec2::new(4.0, 0.0);

const TOLERANCE: f64 = 1e-9;

fn caught_the_crossing(at: DVec2, caught: Option<Snap>) {
    assert!(
        matches!(caught, Some(Snap::Crossing(_))),
        "the crossing at {WHERE_THEY_CROSS} was there to be caught, and the cursor met {caught:?}",
    );
    assert!(
        at.distance(WHERE_THEY_CROSS) < TOLERANCE,
        "and it is pulled onto {WHERE_THEY_CROSS}, not to {at}",
    );
}

#[test]
fn the_cursor_is_pulled_onto_the_place_two_traits_cross() {
    let sketch = with_two_crossing_traits();

    let (at, caught) = sketch.magnetise(DVec2::new(4.2, 0.15), &settings());

    caught_the_crossing(at, caught);
}

#[test]
fn a_crossing_holds_the_cursor_against_a_midpoint_that_is_nearer() {
    let sketch = with_two_crossing_traits();

    let (at, caught) = sketch.magnetise(DVec2::new(4.8, 0.1), &settings());

    caught_the_crossing(at, caught);
}

#[test]
fn a_crossing_the_drawing_already_has_a_point_on_is_that_point() {
    let mut sketch = with_two_crossing_traits();
    sketch.add_point(WHERE_THEY_CROSS);

    assert!(
        sketch.crossings().is_empty(),
        "the place is named, so there is nothing left to invent: {:?}",
        sketch.crossings(),
    );

    let (at, caught) = sketch.magnetise(DVec2::new(4.2, 0.15), &settings());

    assert_eq!(caught, Some(Snap::Point));
    assert!(at.distance(WHERE_THEY_CROSS) < TOLERANCE, "got {at}");
}

fn caught_on_curve(at: DVec2, caught: Option<Snap>, expected: DVec2) {
    assert!(
        matches!(caught, Some(Snap::OnCurve(_))),
        "the curve was there to be caught, and the cursor met {caught:?}",
    );
    assert!(
        at.distance(expected) < TOLERANCE,
        "and it is pulled onto {expected}, not to {at}",
    );
}

fn with_a_circle(at: DVec2, radius: f64) -> Sketch {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let centre = sketch.add_point(at);
    sketch.add_circle(centre, radius);
    sketch
}

#[test]
fn a_cursor_near_a_circle_is_pulled_onto_its_rim() {
    let sketch = with_a_circle(DVec2::new(30.0, 30.0), 9.0);

    let (at, caught) = sketch.magnetise(DVec2::new(30.0, 20.6), &settings());

    caught_on_curve(at, caught, DVec2::new(30.0, 21.0));
}

/// A trait along y = 20, and a circle whose nearest place is (30, 21): one
/// unit apart, so a cursor between them is nearer whichever it is put next
/// to.
fn with_a_trait_under_a_circle() -> Sketch {
    let mut sketch = with_a_trait(DVec2::new(0.0, 20.0), DVec2::new(40.0, 20.0));
    let centre = sketch.add_point(DVec2::new(30.0, 30.0));
    sketch.add_circle(centre, 9.0);
    sketch
}

#[test]
fn a_cursor_nearer_the_circle_than_the_trait_lands_on_the_circle() {
    let sketch = with_a_trait_under_a_circle();

    let (at, caught) = sketch.magnetise(DVec2::new(30.0, 20.6), &settings());

    caught_on_curve(at, caught, DVec2::new(30.0, 21.0));
}

#[test]
fn a_cursor_nearer_the_trait_than_the_circle_lands_on_the_trait() {
    let sketch = with_a_trait_under_a_circle();

    let (at, caught) = sketch.magnetise(DVec2::new(30.0, 20.2), &settings());

    caught_on_curve(at, caught, DVec2::new(30.0, 20.0));
}

fn with_a_quarter_arc() -> Sketch {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let east = sketch.add_point(DVec2::new(7.0, 0.0));
    let north = sketch.add_point(DVec2::new(0.0, 7.0));
    sketch.add_arc(Sketch::ORIGIN, east, north);
    sketch
}

#[test]
fn a_cursor_near_an_arc_is_pulled_onto_it() {
    let sketch = with_a_quarter_arc();
    let northeast = DVec2::new(1.0, 1.0).normalize();

    let (at, caught) = sketch.magnetise(northeast * 7.3, &settings());

    caught_on_curve(at, caught, northeast * 7.0);
}

#[test]
fn a_cursor_beyond_an_arc_is_not_pulled_onto_the_rest_of_its_circle() {
    let sketch = with_a_quarter_arc();
    // Away from both axes, which are magnets of their own.
    let beyond = DVec2::new(-1.0, -1.0).normalize() * 7.2;

    assert_eq!(
        sketch.magnetise(beyond, &settings()),
        (beyond, None),
        "the quarter turn stops due north, and what lies on past it is not drawn",
    );
}

#[test]
fn the_centre_of_a_circle_pulls_like_any_other_point() {
    let sketch = with_a_circle(DVec2::new(30.0, 30.0), 9.0);

    let (at, caught) = sketch.magnetise(DVec2::new(30.4, 30.0), &settings());

    assert_eq!(caught, Some(Snap::Point));
    assert_eq!(at, DVec2::new(30.0, 30.0));
}

fn with_a_trait(start: DVec2, end: DVec2) -> Sketch {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let from = sketch.add_point(start);
    let to = sketch.add_point(end);
    sketch.add_segment(from, to);
    sketch
}

#[test]
fn an_existing_point_wins_over_the_grid() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    sketch.add_point(DVec2::new(10.4, 0.0));

    let (at, caught) = sketch.magnetise(DVec2::new(10.0, 0.0), &settings());

    assert_eq!(caught, Some(Snap::Point));
    assert_eq!(
        at,
        DVec2::new(10.4, 0.0),
        "the grid line is right there, and the point still wins",
    );
}

#[test]
fn the_middle_of_a_trait_pulls_harder_than_its_body() {
    let sketch = with_a_trait(DVec2::new(0.0, 40.0), DVec2::new(40.0, 40.0));

    let (at, caught) = sketch.magnetise(DVec2::new(20.3, 40.2), &settings());

    assert_eq!(caught, Some(Snap::Midpoint(DVec2::new(20.0, 40.0))));
    assert_eq!(at, DVec2::new(20.0, 40.0));
}

#[test]
fn a_cursor_along_a_trait_but_away_from_its_middle_lands_on_the_body() {
    let sketch = with_a_trait(DVec2::new(0.0, 40.0), DVec2::new(40.0, 40.0));

    let (at, caught) = sketch.magnetise(DVec2::new(33.0, 40.2), &settings());

    assert_eq!(caught, Some(Snap::OnCurve(DVec2::new(33.0, 40.0))));
    assert_eq!(at, DVec2::new(33.0, 40.0));
}

#[test]
fn a_cursor_near_a_grid_line_is_pulled_onto_it_without_a_mark() {
    let sketch = Sketch::new(WorkPlane::XY);

    let (at, caught) = sketch.magnetise(DVec2::new(21.0, 39.0), &settings());

    assert_eq!(at, DVec2::new(20.0, 40.0));
    assert_eq!(caught, None, "the grid needs no mark: it is already drawn");
}

#[test]
fn a_cursor_too_far_from_the_grid_is_left_where_it_is() {
    let sketch = Sketch::new(WorkPlane::XY);
    let cursor = DVec2::new(25.0, 35.0);

    assert_eq!(sketch.magnetise(cursor, &settings()), (cursor, None));
}

#[test]
fn snapping_is_off_when_no_grid_step_is_given() {
    let sketch = Sketch::new(WorkPlane::XY);
    let cursor = DVec2::new(21.0, 39.0);
    let loose = SnapSettings {
        grid_step: None,
        ..settings()
    };

    assert_eq!(sketch.magnetise(cursor, &loose), (cursor, None));
}

#[test]
fn the_cursor_is_pulled_onto_an_axis_of_the_plane() {
    let sketch = Sketch::new(WorkPlane::XY);

    let (at, caught) = sketch.magnetise(DVec2::new(23.0, 0.3), &settings());

    assert!(
        matches!(caught, Some(Snap::OnCurve(_))),
        "the horizontal axis was there to be landed on, and the cursor met {caught:?}",
    );
    assert!(
        at.distance(DVec2::new(23.0, 0.0)) < TOLERANCE,
        "and it is pulled onto the axis, not to {at}",
    );
}

#[test]
fn the_cursor_is_pulled_onto_the_place_a_trait_crosses_an_axis() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let from = sketch.add_point(DVec2::new(20.0, -5.0));
    let to = sketch.add_point(DVec2::new(24.0, 5.0));
    sketch.add_segment(from, to);

    let (at, caught) = sketch.magnetise(DVec2::new(22.3, 0.2), &settings());

    assert!(
        matches!(caught, Some(Snap::Crossing(_))),
        "the trait crosses the axis at (22, 0), and the cursor met {caught:?}",
    );
    assert!(
        at.distance(DVec2::new(22.0, 0.0)) < TOLERANCE,
        "and it is pulled onto that crossing, not to {at}",
    );
}

#[test]
fn a_trait_lying_along_an_axis_crosses_it_nowhere() {
    let sketch = with_a_trait(DVec2::new(0.0, 0.0), DVec2::new(10.0, 0.0));

    let (at, caught) = sketch.magnetise(DVec2::new(7.0, 0.2), &settings());

    assert!(
        matches!(caught, Some(Snap::OnCurve(_))),
        "a trait lying on the axis has no crossing with it, and the cursor met {caught:?}",
    );
    assert!(at.distance(DVec2::new(7.0, 0.0)) < TOLERANCE, "at {at}");
}
