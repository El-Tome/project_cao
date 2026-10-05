use glam::DVec2;

use crate::constraints::{Constraint, SketchAxis};
use crate::length::LengthOutcome;
use crate::plane::WorkPlane;
use crate::sketch::{SegmentId, Sketch};

fn drawn(corners: &[DVec2]) -> Sketch {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let points: Vec<_> = corners
        .iter()
        .map(|corner| sketch.add_point(*corner))
        .collect();
    for index in 0..points.len() {
        sketch.add_segment(points[index], points[(index + 1) % points.len()]);
    }
    sketch
}

fn ruled(mut sketch: Sketch, rules: &[Constraint]) -> Sketch {
    for rule in rules {
        assert_eq!(
            sketch.lay_rule(*rule, 1.0),
            LengthOutcome::Exact,
            "the solver counts {rule:?} as kept"
        );
    }
    sketch
}

fn off_the_axis(sketch: &Sketch, side: SegmentId) -> f64 {
    let (start, end) = sketch.endpoints(side);
    start.x.abs().max(end.x.abs())
}

fn off_parallel(sketch: &Sketch, first: SegmentId, second: SegmentId) -> f64 {
    let (a, b) = sketch.endpoints(first);
    let (c, d) = sketch.endpoints(second);
    (b - a).normalize().perp_dot(d - c).abs()
}

fn off_square(sketch: &Sketch, first: SegmentId, second: SegmentId) -> f64 {
    let (a, b) = sketch.endpoints(first);
    let (c, d) = sketch.endpoints(second);
    (b - a).normalize().dot(d - c).abs()
}

fn on_the_axis(segment: SegmentId) -> Constraint {
    Constraint::AxisCollinear {
        segment,
        axis: SketchAxis::V,
    }
}

const QUADRILATERAL: [DVec2; 4] = [
    DVec2::new(1.0, 2.0),
    DVec2::new(20.0, -1.0),
    DVec2::new(35.0, 30.0),
    DVec2::new(1.0, 35.0),
];

const TRIANGLE: [DVec2; 3] = [
    DVec2::new(1.0, 0.5),
    DVec2::new(20.0, -1.0),
    DVec2::new(1.5, 30.0),
];

#[test]
fn a_rule_the_solver_counts_as_kept_holds_within_the_drawing_s_resolution() {
    let parallel = ruled(
        drawn(&QUADRILATERAL),
        &[
            on_the_axis(SegmentId(3)),
            Constraint::Parallel {
                first: SegmentId(0),
                second: SegmentId(2),
            },
        ],
    );
    let square_triangle = ruled(
        drawn(&TRIANGLE),
        &[
            on_the_axis(SegmentId(2)),
            Constraint::Perpendicular {
                first: SegmentId(0),
                second: SegmentId(2),
            },
        ],
    );
    let square_quadrilateral = ruled(
        drawn(&QUADRILATERAL),
        &[
            on_the_axis(SegmentId(3)),
            Constraint::Perpendicular {
                first: SegmentId(0),
                second: SegmentId(3),
            },
        ],
    );

    for (name, sketch, side, off) in [
        (
            "the quadrilateral made parallel",
            &parallel,
            SegmentId(3),
            off_parallel(&parallel, SegmentId(0), SegmentId(2)),
        ),
        (
            "the triangle made square",
            &square_triangle,
            SegmentId(2),
            off_square(&square_triangle, SegmentId(0), SegmentId(2)),
        ),
        (
            "the quadrilateral made square",
            &square_quadrilateral,
            SegmentId(3),
            off_square(&square_quadrilateral, SegmentId(0), SegmentId(3)),
        ),
    ] {
        let resolution = sketch.resolution();
        let from_the_axis = off_the_axis(sketch, side);
        assert!(
            from_the_axis <= resolution && off <= resolution,
            "{name}: {from_the_axis:e} off the axis and {off:e} off its rule, \
             against a resolution of {resolution:e}"
        );
    }
}
