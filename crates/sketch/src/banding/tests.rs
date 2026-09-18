//! What sketch · banding.rs is held to.

use super::*;
use crate::constraints::DimensionTarget;
use crate::element::kinds::{name_of, one_of_every_kind};
use crate::plane::WorkPlane;

const METRICS: AnnotationMetrics = AnnotationMetrics {
    offset_pixels: 22.0,
    arrow_pixels: 8.0,
    arc_pixels: 34.0,
    pixel: 1.0,
    nudge: DVec2::ZERO,
};

#[test]
fn a_trait_with_one_end_outside_the_band_is_not_taken() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let inside = sketch.add_point(DVec2::new(10.0, 10.0));
    let outside = sketch.add_point(DVec2::new(90.0, 10.0));
    let held = sketch.add_segment(inside, outside);

    let caught = sketch.inside_band(DVec2::new(0.0, 0.0), DVec2::new(50.0, 50.0), METRICS);

    assert!(
        caught.contains(&Selection::Element(Element::Point(inside))),
        "the end that is in the box is taken on its own account: {caught:?}"
    );
    assert!(
        !caught.contains(&Selection::Element(Element::Segment(held))),
        "half a trait cannot be deleted, so the box may not claim it: {caught:?}"
    );
}

#[test]
fn a_circle_is_taken_only_when_its_whole_rim_is_inside() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let center = sketch.add_point(DVec2::new(25.0, 25.0));
    let held = sketch.add_circle(center, 5.0);
    let spilling = sketch.add_point(DVec2::new(25.0, 45.0));
    let over_the_edge = sketch.add_circle(spilling, 20.0);

    let caught = sketch.inside_band(DVec2::new(0.0, 0.0), DVec2::new(50.0, 50.0), METRICS);

    assert!(
        caught.contains(&Selection::Element(Element::Circle(held))),
        "a circle whose rim is entirely in the box is taken: {caught:?}"
    );
    assert!(
        !caught.contains(&Selection::Element(Element::Circle(over_the_edge))),
        "a circle whose centre is in the box but whose rim is not stays out: {caught:?}"
    );
}

#[test]
fn an_arc_whose_whole_curve_is_inside_is_taken() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let centre = sketch.add_point(DVec2::new(25.0, 25.0));
    let east = sketch.add_point(DVec2::new(30.0, 25.0));
    let north = sketch.add_point(DVec2::new(25.0, 30.0));
    let held = sketch.add_arc(centre, east, north);

    let caught = sketch.inside_band(DVec2::new(0.0, 0.0), DVec2::new(50.0, 50.0), METRICS);

    assert!(
        caught.contains(&Selection::Element(Element::Arc(held))),
        "the box holds the curve whole, so it takes the arc and not its points alone: {caught:?}"
    );
}

#[test]
fn an_arc_bulging_out_of_the_box_stays_out_of_it_though_its_ends_are_in() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let reach = 30.0 / 2.0_f64.sqrt();
    let centre = sketch.add_point(DVec2::new(25.0, 25.0));
    let below = sketch.add_point(DVec2::new(25.0 + reach, 25.0 - reach));
    let above = sketch.add_point(DVec2::new(25.0 + reach, 25.0 + reach));
    let bulging = sketch.add_arc(centre, below, above);

    let caught = sketch.inside_band(DVec2::new(0.0, 0.0), DVec2::new(50.0, 50.0), METRICS);

    assert!(
        !caught.contains(&Selection::Element(Element::Arc(bulging))),
        "the curve swings east past the box between its two ends: {caught:?}"
    );
}

#[test]
fn the_origin_is_never_taken_by_a_band() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let drawn = sketch.add_point(DVec2::new(10.0, 10.0));

    let caught = sketch.inside_band(DVec2::new(-10.0, -10.0), DVec2::new(50.0, 50.0), METRICS);

    assert!(
        caught.contains(&Selection::Element(Element::Point(drawn))),
        "a point the user drew is taken: {caught:?}"
    );
    assert!(
        !caught.contains(&Selection::Element(Element::Point(Sketch::ORIGIN))),
        "the origin is measured from, never moved or deleted: {caught:?}"
    );
}

#[test]
fn an_annotation_is_taken_by_the_box_that_reaches_where_it_is_drawn() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let start = sketch.add_point(DVec2::new(10.0, 10.0));
    let end = sketch.add_point(DVec2::new(40.0, 10.0));
    let measured = DimensionTarget::Length(sketch.add_segment(start, end));
    let elsewhere = DimensionTarget::Distance {
        from: start,
        to: end,
    };
    sketch.set_dimension(measured, 30.0, false);
    sketch.set_dimension(elsewhere, 30.0, false);
    // Dragged well clear of the box, and recorded there.
    sketch.offset_dimension(elsewhere, DVec2::new(1000.0, 1000.0));
    let written_at = sketch.place(measured, METRICS).unwrap().text_at;

    let caught = sketch.inside_band(
        written_at - DVec2::splat(5.0),
        written_at + DVec2::splat(5.0),
        METRICS,
    );

    assert!(
        caught.contains(&Selection::Dimension(measured)),
        "the box reaches where that annotation is written: {caught:?}"
    );
    assert!(
        !caught.contains(&Selection::Dimension(elsewhere)),
        "an annotation drawn outside the box stays out of it: {caught:?}"
    );
}

#[test]
fn a_box_over_the_whole_drawing_catches_one_of_every_kind() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let drawn = one_of_every_kind(&mut sketch);

    let caught = sketch.inside_band(DVec2::ZERO, DVec2::splat(60.0), METRICS);

    for element in drawn {
        assert!(
            caught.contains(&Selection::Element(element)),
            "a {} was inside the box and the box did not take it: {caught:?}",
            name_of(&element),
        );
    }
}
