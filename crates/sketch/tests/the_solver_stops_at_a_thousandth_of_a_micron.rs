//! What a value is typed as, the drawing lands — to under a thousandth of a
//! micron on a part up to a metre, whatever the part's size.
//!
//! Closes #517.
//! - the solver stops at a thousandth of a micron of the drawing's size, on a
//!   part of ten millimetres as on a metre —
//!   `a_typed_height_lands_where_it_was_typed_at_any_size`
//! - an edit finer than the old stop line is applied, and lands exact —
//!   `an_edit_of_a_third_of_a_micron_is_applied_and_lands_exact`
//! - a tangency's contact holds to the stop line —
//!   `a_tangent_circle_stands_on_its_line_to_the_stop_line`
//! - every tolerance coupled to the stop line moves with it — no test: held
//!   by the solver, drag and resize suites, which the gate runs
//! - the random-drawing campaign before and after, compared like for like —
//!   no test: the record in the pull request

use cao_sketch::{
    Constraint, DimensionTarget, LengthOutcome, PointId, SegmentId, Sketch, WorkPlane,
};
use glam::DVec2;

/// A thousandth of a micron on a part up to a metre, as a share of the
/// drawing's size, with room for the solver to stop just under it.
const STOP_LINE: f64 = 2e-9;

fn rectangle(width: f64, height: f64) -> (Sketch, [SegmentId; 4]) {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let corners = [
        DVec2::ZERO,
        DVec2::new(width, 0.0),
        DVec2::new(width, height),
        DVec2::new(0.0, height),
    ];
    let points = corners.map(|place| sketch.add_point(place));
    let sides: [SegmentId; 4] =
        std::array::from_fn(|rank| sketch.add_segment(points[rank], points[(rank + 1) % 4]));
    for rank in 0..3 {
        sketch.add_constraint(Constraint::Perpendicular {
            first: sides[rank],
            second: sides[rank + 1],
        });
    }
    (sketch, sides)
}

fn length(sketch: &Sketch, side: SegmentId) -> f64 {
    let drawn = sketch.segments()[side.0];
    sketch.point(drawn.start).distance(sketch.point(drawn.end))
}

#[test]
fn a_typed_height_lands_where_it_was_typed_at_any_size() {
    for size in [10.0, 100.0, 1000.0] {
        let (mut sketch, sides) = rectangle(size, size / 2.0);
        sketch.set_dimension(DimensionTarget::Length(sides[0]), size, false);
        sketch.set_dimension(DimensionTarget::Length(sides[1]), size / 2.0, false);
        sketch.set_dimension(
            DimensionTarget::AxisAngle {
                segment: sides[0],
                segment_toward: cao_sketch::Toward::End,
                axis: cao_sketch::SketchAxis::U,
                axis_toward: cao_sketch::AxisToward::Positive,
            },
            30.0,
            false,
        );
        assert_eq!(sketch.resolve(1.0), LengthOutcome::Exact);

        let target = DimensionTarget::Length(sides[1]);
        let asked = size / 2.0 * 1.01;
        sketch.set_dimension(target, asked, false);
        assert_eq!(
            sketch.land_value(target, 1.0),
            LengthOutcome::Exact,
            "a hundredth more of height, on {size}",
        );

        let diagonal = (size * size * 1.25).sqrt();
        let landed = length(&sketch, sides[1]);
        assert!(
            (landed - asked).abs() <= diagonal * STOP_LINE,
            "on {size}: asked {asked}, landed {landed}, off by {} of the size",
            (landed - asked).abs() / diagonal,
        );
    }
}

#[test]
fn an_edit_of_a_third_of_a_micron_is_applied_and_lands_exact() {
    let (mut sketch, sides) = rectangle(100.0, 50.0);
    sketch.set_dimension(DimensionTarget::Length(sides[0]), 100.0, false);
    sketch.set_dimension(DimensionTarget::Length(sides[1]), 50.0, false);
    assert_eq!(sketch.resolve(1.0), LengthOutcome::Exact);

    let target = DimensionTarget::Length(sides[1]);
    let asked = 50.0 + 3e-4;
    sketch.set_dimension(target, asked, false);
    assert_eq!(
        sketch.land_value(target, 1.0),
        LengthOutcome::Exact,
        "a third of a micron more of height",
    );

    let diagonal = (100.0f64 * 100.0 * 1.25).sqrt();
    let landed = length(&sketch, sides[1]);
    assert!(
        (landed - asked).abs() <= diagonal * STOP_LINE,
        "asked {asked}, landed {landed}, off by {} of the size",
        (landed - asked).abs() / diagonal,
    );
}

#[test]
fn a_tangent_circle_stands_on_its_line_to_the_stop_line() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let from = sketch.add_point(DVec2::ZERO);
    let to = sketch.add_point(DVec2::new(100.0, 0.0));
    let ground = sketch.add_segment(from, to);
    let centre = sketch.add_point(DVec2::new(50.0, 26.0));
    let circle = sketch.add_circle(centre, 25.0);
    sketch.add_constraint(Constraint::Tangent {
        circle,
        segment: ground,
        at: None,
        from: cao_sketch::LaidFrom::Nowhere,
    });
    assert_eq!(sketch.resolve(1.0), LengthOutcome::Exact);
    let target = DimensionTarget::Radius(circle);
    sketch.set_dimension(target, 26.0, false);
    assert_eq!(sketch.land_value(target, 1.0), LengthOutcome::Exact);

    let gap = touch_gap(&sketch, ground, centre, circle);
    let diagonal = sketch_diagonal(&sketch);
    assert!(
        gap <= diagonal * STOP_LINE,
        "the contact stands {gap} off its line, {} of the size",
        gap / diagonal,
    );
}

fn touch_gap(
    sketch: &Sketch,
    ground: SegmentId,
    centre: PointId,
    circle: cao_sketch::CircleId,
) -> f64 {
    let drawn = sketch.segments()[ground.0];
    let (a, b) = (sketch.point(drawn.start), sketch.point(drawn.end));
    let unit = (b - a).normalize();
    let aside = unit.perp_dot(sketch.point(centre) - a).abs();
    (aside - sketch.circles()[circle.0].radius).abs()
}

fn sketch_diagonal(sketch: &Sketch) -> f64 {
    let places: Vec<DVec2> = (0..sketch.points().len())
        .map(|index| sketch.point(PointId(index)))
        .collect();
    let low = places.iter().fold(DVec2::splat(f64::MAX), |a, b| a.min(*b));
    let high = places.iter().fold(DVec2::splat(f64::MIN), |a, b| a.max(*b));
    (high - low).length()
}
