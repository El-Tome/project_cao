use glam::DVec2;

use super::*;
use crate::constraints::{Constraint, SketchAxis};
use crate::plane::WorkPlane;

const TOLERANCE: f64 = 1e-9;

/// A right-angled triangle standing entirely east of the V axis.
fn a_triangle_east_of_the_axis() -> (Sketch, Vec<Element>) {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let a = sketch.add_point(DVec2::new(2.0, 0.0));
    let b = sketch.add_point(DVec2::new(6.0, 0.0));
    let c = sketch.add_point(DVec2::new(2.0, 3.0));
    let held = vec![
        Element::Segment(sketch.add_segment(a, b)),
        Element::Segment(sketch.add_segment(b, c)),
        Element::Segment(sketch.add_segment(c, a)),
    ];
    (sketch, held)
}

fn places(sketch: &Sketch, made: &Duplicated) -> Vec<DVec2> {
    made.points.iter().map(|id| sketch.point(*id)).collect()
}

#[test]
fn a_selection_mirrored_across_an_axis_lands_the_other_side_of_it() {
    let (mut sketch, held) = a_triangle_east_of_the_axis();

    let made = sketch
        .mirror(&held, ChosenAxis::Sketch(SketchAxis::V))
        .expect("an axis to mirror across");

    let mut wanted = [
        DVec2::new(-2.0, 0.0),
        DVec2::new(-6.0, 0.0),
        DVec2::new(-2.0, 3.0),
    ];
    wanted.sort_by(|a, b| a.x.total_cmp(&b.x));
    let mut got = places(&sketch, &made);
    got.sort_by(|a, b| a.x.total_cmp(&b.x));
    assert_eq!(got.len(), 3);
    for (at, should) in got.iter().zip(wanted) {
        assert!(
            at.distance(should) <= TOLERANCE,
            "mirrored to {at:?}, wanted {should:?}"
        );
    }
}

#[test]
fn the_copy_keeps_the_shape_the_original_had() {
    let (mut sketch, held) = a_triangle_east_of_the_axis();

    let made = sketch
        .mirror(&held, ChosenAxis::Sketch(SketchAxis::V))
        .expect("an axis to mirror across");

    assert_eq!(made.segments.len(), 3, "three sides, as it had");
    let mut lengths: Vec<f64> = made
        .segments
        .iter()
        .map(|id| {
            let (from, to) = sketch.endpoints(*id);
            from.distance(to)
        })
        .collect();
    lengths.sort_by(f64::total_cmp);
    let mut wanted = [4.0, 3.0, 5.0];
    wanted.sort_by(f64::total_cmp);
    for (got, should) in lengths.iter().zip(wanted) {
        assert!(
            (got - should).abs() <= TOLERANCE,
            "side {got}, wanted {should}"
        );
    }
}

#[test]
fn the_original_is_left_exactly_as_it_was() {
    let (mut sketch, held) = a_triangle_east_of_the_axis();
    let before: Vec<DVec2> = sketch.live_points().map(|(_, at)| at).collect();

    sketch
        .mirror(&held, ChosenAxis::Sketch(SketchAxis::V))
        .expect("an axis to mirror across");

    let after: Vec<DVec2> = sketch
        .live_points()
        .map(|(_, at)| at)
        .take(before.len())
        .collect();
    assert_eq!(after, before, "a mirror copies, it does not move");
    assert_eq!(
        sketch.live_segments().count(),
        6,
        "three sides, and three more"
    );
}

#[test]
fn the_copy_comes_out_bare_of_the_rules_the_original_carried() {
    let (mut sketch, held) = a_triangle_east_of_the_axis();
    let [first, second] = [held[0], held[1]].map(|element| match element {
        Element::Segment(id) => id,
        _ => unreachable!(),
    });
    sketch.add_constraint(Constraint::Perpendicular { first, second });
    let rules = sketch.constraints().len();

    sketch
        .mirror(&held, ChosenAxis::Sketch(SketchAxis::V))
        .expect("an axis to mirror across");

    assert_eq!(
        sketch.constraints().len(),
        rules,
        "the copy is geometry and nothing else"
    );
}

#[test]
fn a_trait_of_the_drawing_serves_as_the_axis() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let low = sketch.add_point(DVec2::new(0.0, -5.0));
    let high = sketch.add_point(DVec2::new(0.0, 5.0));
    let axis = sketch.add_segment(low, high);
    let a = sketch.add_point(DVec2::new(3.0, 1.0));
    let b = sketch.add_point(DVec2::new(7.0, 1.0));
    let side = Element::Segment(sketch.add_segment(a, b));

    let made = sketch
        .mirror(&[side], ChosenAxis::Trait(axis))
        .expect("a trait to mirror across");

    let mut got = places(&sketch, &made);
    got.sort_by(|a, b| a.x.total_cmp(&b.x));
    assert!(
        got[0].distance(DVec2::new(-7.0, 1.0)) <= TOLERANCE,
        "got {got:?}"
    );
    assert!(
        got[1].distance(DVec2::new(-3.0, 1.0)) <= TOLERANCE,
        "got {got:?}"
    );
}

#[test]
fn the_trait_serving_as_the_axis_is_not_copied_onto_itself() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let low = sketch.add_point(DVec2::new(0.0, -5.0));
    let high = sketch.add_point(DVec2::new(0.0, 5.0));
    let axis = sketch.add_segment(low, high);
    let a = sketch.add_point(DVec2::new(3.0, 1.0));
    let b = sketch.add_point(DVec2::new(7.0, 1.0));
    let side = Element::Segment(sketch.add_segment(a, b));

    let made = sketch
        .mirror(&[Element::Segment(axis), side], ChosenAxis::Trait(axis))
        .expect("a trait to mirror across");

    assert_eq!(
        made.segments.len(),
        1,
        "the axis lies on itself, and a copy of it there is one nobody can tell from it"
    );
}

#[test]
fn a_mirrored_curve_still_turns_the_way_it_looks() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let centre = sketch.add_point(DVec2::new(5.0, 0.0));
    let east = sketch.add_point(DVec2::new(8.0, 0.0));
    let north = sketch.add_point(DVec2::new(5.0, 3.0));
    let arc = sketch.add_arc(centre, east, north);
    let sweep = sketch.arc_sweep(arc);

    let made = sketch
        .mirror(&[Element::Arc(arc)], ChosenAxis::Sketch(SketchAxis::V))
        .expect("an axis to mirror across");

    let copy = made.arcs[0];
    assert!(
        (sketch.arc_sweep(copy) - sweep).abs() <= TOLERANCE,
        "a reflection turns a curve over, so its ends swap or it sweeps the long way: \
         got {} against {sweep}",
        sketch.arc_sweep(copy)
    );
}

#[test]
fn a_circle_keeps_its_size_across_the_axis() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let centre = sketch.add_point(DVec2::new(4.0, 2.0));
    let round = sketch.add_circle(centre, 1.5);

    let made = sketch
        .mirror(&[Element::Circle(round)], ChosenAxis::Sketch(SketchAxis::V))
        .expect("an axis to mirror across");

    let copy = sketch.circles()[made.circles[0].0];
    assert!((copy.radius - 1.5).abs() <= TOLERANCE);
    assert!(sketch.point(copy.center).distance(DVec2::new(-4.0, 2.0)) <= TOLERANCE);
}

#[test]
fn a_trait_with_no_length_cannot_be_an_axis() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let only = sketch.add_point(DVec2::new(1.0, 1.0));
    let nowhere = sketch.add_segment(only, only);
    let a = sketch.add_point(DVec2::new(3.0, 1.0));
    let b = sketch.add_point(DVec2::new(7.0, 1.0));
    let side = Element::Segment(sketch.add_segment(a, b));

    assert_eq!(sketch.mirror(&[side], ChosenAxis::Trait(nowhere)), None);
}

#[test]
fn a_point_sitting_on_the_axis_is_not_copied_onto_itself() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let on_the_axis = sketch.add_point(DVec2::new(0.0, 2.0));
    let a = sketch.add_point(DVec2::new(3.0, 1.0));
    let b = sketch.add_point(DVec2::new(7.0, 1.0));
    let side = Element::Segment(sketch.add_segment(a, b));

    let made = sketch
        .mirror(
            &[Element::Point(on_the_axis), side],
            ChosenAxis::Sketch(SketchAxis::V),
        )
        .expect("an axis to mirror across");

    assert_eq!(
        made.points.len(),
        2,
        "the point lies on the axis, and a copy of it there is one nobody can tell from it"
    );
}

#[test]
fn a_circle_centred_on_the_axis_is_not_copied_onto_itself() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let centre = sketch.add_point(DVec2::new(0.0, 2.0));
    let round = sketch.add_circle(centre, 1.5);

    let made = sketch
        .mirror(&[Element::Circle(round)], ChosenAxis::Sketch(SketchAxis::V))
        .expect("an axis to mirror across");

    assert!(
        made.circles.is_empty(),
        "the circle covers itself across the axis, and a second one there is one nobody can see"
    );
    assert!(made.points.is_empty(), "nor is its centre laid twice");
}

#[test]
fn an_arc_symmetric_about_the_axis_is_not_copied_onto_itself() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let centre = sketch.add_point(DVec2::ZERO);
    let east = sketch.add_point(DVec2::new(3.0, 0.0));
    let west = sketch.add_point(DVec2::new(-3.0, 0.0));
    let arc = sketch.add_arc(centre, east, west);

    let made = sketch
        .mirror(&[Element::Arc(arc)], ChosenAxis::Sketch(SketchAxis::V))
        .expect("an axis to mirror across");

    assert!(
        made.arcs.is_empty(),
        "the curve reads the same either side of the axis, so its copy covers it"
    );
}
