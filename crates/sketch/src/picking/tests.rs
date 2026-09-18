//! What sketch · picking.rs is held to.

use super::*;
use crate::element::kinds::{name_of, one_of_every_kind, somewhere_on};
use crate::plane::WorkPlane;

const TOLERANCE: f64 = 1e-9;

fn quarter() -> (Sketch, ArcId) {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let centre = sketch.add_point(DVec2::ZERO);
    let east = sketch.add_point(DVec2::new(10.0, 0.0));
    let north = sketch.add_point(DVec2::new(0.0, 10.0));
    let arc = sketch.add_arc(centre, east, north);
    (sketch, arc)
}

#[test]
fn a_click_on_the_part_of_the_circle_the_arc_does_not_draw_finds_nothing() {
    let (sketch, arc) = quarter();
    let corner = DVec2::splat(10.0 / 2.0_f64.sqrt());

    assert_eq!(sketch.nearest_arc(corner, 0.5), Some(arc));
    assert_eq!(sketch.nearest_arc(DVec2::new(-10.0, 0.0), 0.5), None);
    assert_eq!(sketch.nearest_arc(DVec2::new(0.0, -10.0), 0.5), None);
}

#[test]
fn past_its_end_an_arc_is_as_far_away_as_that_end_is() {
    let (sketch, arc) = quarter();
    let beyond = DVec2::new(13.0, -4.0);
    let distance = sketch.distance_to_arc(arc, beyond);

    let to_the_end = beyond.distance(DVec2::new(10.0, 0.0));
    assert!(
        (distance - to_the_end).abs() < TOLERANCE,
        "{distance} away, where its end is {to_the_end} away",
    );
}

#[test]
fn an_erased_arc_is_not_under_the_cursor_any_more() {
    let (mut sketch, arc) = quarter();
    sketch.erase(Element::Arc(arc));

    assert_eq!(sketch.nearest_arc(DVec2::new(10.0, 0.0), 0.5), None);
}

const METRICS: AnnotationMetrics = AnnotationMetrics {
    offset_pixels: 22.0,
    arrow_pixels: 8.0,
    arc_pixels: 34.0,
    pixel: 1.0,
    nudge: DVec2::ZERO,
};

#[test]
fn a_click_on_the_curve_of_an_arc_takes_hold_of_the_arc() {
    let (sketch, arc) = quarter();
    let on_the_curve = DVec2::splat(10.0 / 2.0_f64.sqrt());

    assert_eq!(
        sketch.pick(on_the_curve, 1.0, METRICS),
        Some(Selection::Element(Element::Arc(arc))),
    );
}

#[test]
fn a_point_wins_over_the_trait_it_sits_on() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let start = sketch.add_point(DVec2::new(10.0, 0.0));
    let end = sketch.add_point(DVec2::new(50.0, 0.0));
    sketch.add_segment(start, end);

    let picked = sketch.pick(DVec2::new(10.2, 0.0), 1.0, METRICS);

    assert_eq!(
        picked,
        Some(Selection::Element(Element::Point(start))),
        "the cursor is within reach of both, and the point is the harder aim"
    );
}
#[test]
fn a_selected_circle_carries_its_centre() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let center = sketch.add_point(DVec2::new(25.0, 25.0));
    let drawn = sketch.add_circle(center, 5.0);

    let carried = sketch.points_of(&[Selection::Element(Element::Circle(drawn))]);

    assert_eq!(
        carried,
        vec![center],
        "dragging a circle moves it whole, and its centre is what it is placed by"
    );
}

#[test]
fn a_trait_carries_both_its_ends_and_never_the_origin() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let far = sketch.add_point(DVec2::new(40.0, 0.0));
    let held = sketch.add_segment(Sketch::ORIGIN, far);

    let carried = sketch.points_of(&[
        Selection::Element(Element::Segment(held)),
        Selection::Element(Element::Point(far)),
    ]);

    assert_eq!(
        carried,
        vec![far],
        "the end anchored on the origin stays put, and the other is carried once"
    );
}

#[test]
fn an_annotation_is_easier_to_hit_than_the_trait_it_measures() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let start = sketch.add_point(DVec2::new(0.0, 0.0));
    let end = sketch.add_point(DVec2::new(40.0, 0.0));
    let measured = DimensionTarget::Length(sketch.add_segment(start, end));
    sketch.set_dimension(measured, 40.0, false);
    let written_at = sketch.place(measured, METRICS).unwrap().text_at;

    let just_off = written_at + DVec2::new(0.0, 1.2);
    assert_eq!(
        sketch.pick(just_off, 1.0, METRICS),
        Some(Selection::Dimension(measured)),
        "an annotation is read rather than aimed at, so it answers past the reach"
    );
    assert_eq!(
        sketch.pick(DVec2::new(20.0, 1.2), 1.0, METRICS),
        None,
        "the geometry itself answers only within the reach"
    );
}

#[test]
fn the_origin_is_never_picked() {
    let sketch = Sketch::new(WorkPlane::XY);

    assert_eq!(
        sketch.pick(DVec2::new(0.1, 0.0), 1.0, METRICS),
        None,
        "the origin is there to be measured from, not taken hold of"
    );
}

#[test]
fn a_click_on_the_drawing_finds_one_of_every_kind() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let drawn = one_of_every_kind(&mut sketch);

    for element in drawn {
        let at = somewhere_on(&sketch, element);
        let found = sketch.pick(at, 1.0, METRICS);

        assert_eq!(
            found,
            Some(Selection::Element(element)),
            "a click on a {} found {found:?}",
            name_of(&element),
        );
    }
}
