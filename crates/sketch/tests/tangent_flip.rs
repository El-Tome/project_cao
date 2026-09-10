//! A tangency's contact point is held on the infinite line its segment lies
//! on, square under the circle's centre — nothing in those two equations
//! stops that foot from sliding past either end of the segment once the
//! centre is dragged far enough. Past that point the tangency is no longer
//! physically real: the circle brushes empty space beyond the trait, not the
//! trait itself. A solve that reaches such a state used to be reported as
//! `Exact` all the same.

use cao_sketch::{Constraint, LengthOutcome, Sketch, WorkPlane};
use glam::DVec2;

#[test]
fn a_tangency_refuses_to_slide_its_contact_past_the_end_of_its_segment() {
    // A triangle carrying no dimension, and a circle nested in the corner at
    // `b`, tangent to both edges that meet there — the shape from the
    // original report.
    let mut sketch = Sketch::new(WorkPlane::XY);
    let a = sketch.add_point(DVec2::new(0.0, 0.0));
    let b = sketch.add_point(DVec2::new(200.0, 0.0));
    let c = sketch.add_point(DVec2::new(100.0, 150.0));
    let ab = sketch.add_segment(a, b);
    let bc = sketch.add_segment(b, c);
    let _ca = sketch.add_segment(c, a);
    let center = sketch.add_point(DVec2::new(140.0, 40.0));
    let circle = sketch.add_circle(center, 20.0);
    sketch.add_constraint(Constraint::Tangent {
        circle,
        segment: ab,
        at: None,
    });
    sketch.add_constraint(Constraint::Tangent {
        circle,
        segment: bc,
        at: None,
    });
    assert_eq!(sketch.resolve(1.0), LengthOutcome::Exact);

    let mut refused_once = false;
    // Walking `b` back towards `a` is what used to push the tangency's
    // contact with `ab` past `b` itself, the flip the report describes.
    for step in 0..30 {
        let target = DVec2::new(200.0 - step as f64 * 6.0, step as f64);
        let before = sketch.point(b);
        let outcome = sketch.settle_around(b, target, 1.0);

        let Some(Constraint::Tangent {
            at: Some(contact), ..
        }) = sketch
            .constraints()
            .iter()
            .find(|constraint| matches!(constraint, Constraint::Tangent { segment, .. } if *segment == ab))
            .copied()
        else {
            panic!("the tangency to ab has no contact point");
        };
        let span = sketch.point(b) - sketch.point(a);
        let t = (sketch.point(contact) - sketch.point(a)).dot(span) / span.length_squared();
        assert!(
            (-1e-6..=1.0 + 1e-6).contains(&t),
            "step {step}: the contact left segment ab at t = {t}"
        );

        if outcome == LengthOutcome::BestEffort {
            refused_once = true;
            assert_eq!(
                sketch.point(b),
                before,
                "step {step}: a refused drag should leave the point where it was"
            );
        }
    }

    assert!(
        refused_once,
        "dragging this far should have hit the tangency's limit at least once"
    );
}
