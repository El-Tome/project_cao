//! A trait held against an ellipse.
//!
//! Closes #415.
//! - the tangency tool takes a trait and an ellipse, and the trait ends up
//!   touching it — `a_trait_told_to_brush_an_ellipse_ends_up_touching_it`,
//!   turning about its own end when that is the one way left —
//!   `a_trait_pinned_at_one_end_turns_until_it_brushes` — and the curve growing
//!   to the trait is as good an answer as the trait moving to the curve —
//!   `an_ellipse_free_to_grow_swells_until_it_meets_the_trait`
//! - the rule leaves a point where the two touch, as a tangency on a circle
//!   does, and takes it back when it goes —
//!   `the_tangency_leaves_a_point_where_the_two_touch`
//! - dragging the ellipse keeps them touching —
//!   `the_ellipse_dragged_keeps_the_trait_against_it`, and to the last decimal,
//!   not just to the eye, when nothing is left holding it but the rule itself —
//!   `a_tangency_whose_touch_was_rubbed_out_still_holds_the_trait_exactly`
//! - dragging the trait keeps them touching —
//!   `the_trait_dragged_keeps_brushing_the_ellipse`
//! - an arc of ellipse takes a tangency the same way —
//!   `an_arc_of_ellipse_is_brushed_where_it_is_drawn`
//! - the tool offers an ellipse to point at — no test: it is one line of the
//!   canvas's pick order, `nearest_rule_pick`, where the canvas has no net
//!   (see docs/code-map.md); what the rule means once pointed at is
//!   `a_trait_and_an_ellipse_make_a_tangency`

use cao_sketch::{Constraint, Element, Rule, RulePick, SegmentId, Sketch, WorkPlane, rule_intent};
use glam::DVec2;

/// An ellipse about the origin, sixty wide and forty high, and a trait lying
/// well clear of it below.
fn an_ellipse_and_a_trait() -> (Sketch, cao_sketch::EllipseId, cao_sketch::SegmentId) {
    an_ellipse_and_a_trait_of(30.0, 20.0)
}

/// The same, of whatever half-axes are asked for.
fn an_ellipse_and_a_trait_of(
    wide: f64,
    high: f64,
) -> (Sketch, cao_sketch::EllipseId, cao_sketch::SegmentId) {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let centre = sketch.add_point(DVec2::ZERO);
    let west = sketch.add_point(DVec2::new(-wide, 0.0));
    let east = sketch.add_point(DVec2::new(wide, 0.0));
    let south = sketch.add_point(DVec2::new(0.0, -high));
    let north = sketch.add_point(DVec2::new(0.0, high));
    let ellipse = sketch.add_ellipse(centre, [west, east], [south, north]);
    let from = sketch.add_point(DVec2::new(-wide - 10.0, -high - 15.0));
    let to = sketch.add_point(DVec2::new(wide + 10.0, -high - 15.0));
    let segment = sketch.add_segment(from, to);
    (sketch, ellipse, segment)
}

/// How far the trait stands from the curve, at its nearest.
fn apart(sketch: &Sketch, ellipse: cao_sketch::EllipseId, segment: cao_sketch::SegmentId) -> f64 {
    let (from, to) = sketch.endpoints(segment);
    let drawn = sketch.ellipse_draft(ellipse);
    (0..=200)
        .map(|step| {
            let along = from.lerp(to, step as f64 / 200.0);
            drawn.distance(along)
        })
        .fold(f64::INFINITY, f64::min)
}

/// How far the trait stands from the stretch of curve actually drawn, at its
/// nearest — which on an arc of ellipse is not the same question as `apart`.
fn along_the_stretch(sketch: &Sketch, ellipse: cao_sketch::EllipseId, segment: SegmentId) -> f64 {
    let (from, to) = sketch.endpoints(segment);
    let span = to - from;
    sketch
        .ellipse_polyline(ellipse)
        .iter()
        .map(|place| {
            let along = ((*place - from).dot(span) / span.length_squared()).clamp(0.0, 1.0);
            place.distance(from.lerp(to, along))
        })
        .fold(f64::INFINITY, f64::min)
}

/// The point the drawing's one ellipse tangency holds where the two touch.
fn the_touch_point(sketch: &Sketch) -> cao_sketch::PointId {
    sketch
        .constraints()
        .iter()
        .find_map(|rule| match rule {
            Constraint::EllipseTangent { at, .. } => *at,
            _ => None,
        })
        .expect("a tangency keeps a point where the two touch")
}

/// How far the tangency itself is from being kept: the gap between the line
/// the trait lies on and how far the ellipse reaches square to that line.
///
/// This is what the rule holds, and it is not `apart`: the trait is a piece of
/// its line, and a piece can slide well past the curve while the line it lies
/// on still brushes it.
fn off_the_curve(sketch: &Sketch, ellipse: cao_sketch::EllipseId, segment: SegmentId) -> f64 {
    let (from, to) = sketch.endpoints(segment);
    let drawn = sketch.ellipse_draft(ellipse);
    let across = (to - from).perp() / (to - from).length();
    let reach = across
        .dot(drawn.first)
        .hypot(across.dot(drawn.second_axis()));
    (across.dot(drawn.centre - from).abs() - reach).abs()
}

#[test]
fn a_trait_and_an_ellipse_make_a_tangency() {
    let (sketch, ellipse, segment) = an_ellipse_and_a_trait();
    let picks = [
        RulePick::Element(Element::Segment(segment)),
        RulePick::Element(Element::Ellipse(ellipse)),
    ];

    let meant = rule_intent(Rule::Tangent, &picks, &sketch);

    assert_eq!(
        meant,
        Some(cao_sketch::RuleIntent::Constrain(
            Constraint::EllipseTangent {
                ellipse,
                segment,
                at: None,
            }
        )),
    );
}

#[test]
fn a_trait_told_to_brush_an_ellipse_ends_up_touching_it() {
    let (mut sketch, ellipse, segment) = an_ellipse_and_a_trait();
    let before = apart(&sketch, ellipse, segment);
    assert!(before > 10.0, "it starts well clear: {before}");

    sketch.add_constraint(Constraint::EllipseTangent {
        ellipse,
        segment,
        at: None,
    });
    sketch.resolve(1.0);

    let after = apart(&sketch, ellipse, segment);
    assert!(
        after < 1e-3,
        "it brushes the curve, standing {after} off it"
    );
}

#[test]
fn the_tangency_leaves_a_point_where_the_two_touch() {
    let (mut sketch, ellipse, segment) = an_ellipse_and_a_trait();
    let before = sketch.live_points().count();

    sketch.add_constraint(Constraint::EllipseTangent {
        ellipse,
        segment,
        at: None,
    });
    sketch.resolve(1.0);

    assert_eq!(
        sketch.live_points().count(),
        before + 1,
        "one point more, as a tangency on a circle leaves one",
    );
    let touch = sketch.point(the_touch_point(&sketch));
    let drawn = sketch.ellipse_draft(ellipse);
    // Loose on purpose: what this holds is that the point lands on the touch
    // rather than anywhere else. How exactly the rule itself is kept is
    // measured by a_tangency_whose_touch_was_rubbed_out_still_holds_the_trait_exactly.
    assert!(drawn.distance(touch) < 1e-2, "it is on the curve: {touch}");
    let (from, to) = sketch.endpoints(segment);
    let across = (to - from).perp().normalize();
    assert!(
        across.dot(touch - from).abs() < 1e-2,
        "and on the trait: {touch}",
    );

    sketch.erase_constraint(Constraint::EllipseTangent {
        ellipse,
        segment,
        at: None,
    });

    assert_eq!(
        sketch.live_points().count(),
        before,
        "and it goes when the rule does, rather than being left in mid-air",
    );
}

#[test]
fn a_trait_pinned_at_one_end_turns_until_it_brushes() {
    let (mut sketch, ellipse, segment) = an_ellipse_and_a_trait();
    // Nothing may move but the trait's far end, so the only way to the curve
    // is to turn about the near one.
    sketch.add_constraint(Constraint::Fixed {
        element: Element::Ellipse(ellipse),
    });
    sketch.add_constraint(Constraint::Fixed {
        element: Element::Point(sketch.segments()[segment.0].start),
    });
    let held = sketch.point(sketch.segments()[segment.0].start);

    sketch.add_constraint(Constraint::EllipseTangent {
        ellipse,
        segment,
        at: None,
    });
    sketch.resolve(1.0);

    let after = apart(&sketch, ellipse, segment);
    assert!(
        after < 1e-3,
        "it brushes the curve, standing {after} off it"
    );
    let still = sketch.point(sketch.segments()[segment.0].start);
    assert!(still.distance(held) < 1e-9, "the near end stayed: {still}");
    let axes = sketch.ellipse_draft(ellipse);
    assert!(
        axes.first.distance(DVec2::new(30.0, 0.0)) < 1e-9,
        "and the ellipse never stirred: {}",
        axes.first,
    );
}

#[test]
fn an_ellipse_free_to_grow_swells_until_it_meets_the_trait() {
    let (mut sketch, ellipse, segment) = an_ellipse_and_a_trait();
    // Held by its centre, with the trait nailed down: the one way left to the
    // trait is to grow, as a circle meets a tangency by changing its radius.
    let line = sketch.segments()[segment.0];
    for point in [line.start, line.end, sketch.ellipses()[ellipse.0].center] {
        sketch.add_constraint(Constraint::Fixed {
            element: Element::Point(point),
        });
    }

    sketch.add_constraint(Constraint::EllipseTangent {
        ellipse,
        segment,
        at: None,
    });
    sketch.resolve(1.0);

    let drawn = sketch.ellipse_draft(ellipse);
    assert!(
        (drawn.second_axis().length() - 35.0).abs() < 1e-6,
        "it grew down to the trait: {}",
        drawn.second_axis(),
    );
    assert!(
        (drawn.first.length() - 30.0).abs() < 1e-6,
        "and no wider, the trait asking nothing of that axis: {}",
        drawn.first,
    );
    assert!(apart(&sketch, ellipse, segment) < 1e-6);
}

#[test]
fn the_ellipse_dragged_keeps_the_trait_against_it() {
    let (mut sketch, ellipse, segment) = an_ellipse_and_a_trait();
    sketch.add_constraint(Constraint::EllipseTangent {
        ellipse,
        segment,
        at: None,
    });
    sketch.resolve(1.0);
    let centre = sketch.ellipses()[ellipse.0].center;

    sketch.settle_around(centre, DVec2::new(15.0, 12.0), 1.0);

    let after = apart(&sketch, ellipse, segment);
    assert!(after < 1e-3, "it still brushes, standing {after} off it");
}

#[test]
fn a_tangency_whose_touch_was_rubbed_out_still_holds_the_trait_exactly() {
    // The touch point is a point of the drawing and can be deleted like any
    // other; the rule stays, with nothing left holding it but its own sum.
    // Long and thin is where how far the curve reaches swings fastest as the
    // trait turns, and the bound is far below anything a screen shows: what is
    // measured here is the sum, not the picture.
    for turn in 0..12 {
        let (mut sketch, ellipse, segment) = an_ellipse_and_a_trait_of(10.0, 60.0);
        sketch.add_constraint(Constraint::EllipseTangent {
            ellipse,
            segment,
            at: None,
        });
        sketch.resolve(1.0);
        let touch = the_touch_point(&sketch);
        sketch.erase(Element::Point(touch));
        let centre = sketch.ellipses()[ellipse.0].center;
        let angle = turn as f64 / 12.0 * std::f64::consts::TAU;

        sketch.settle_around(centre, DVec2::from_angle(angle) * 36.0, 1.0);

        let off = off_the_curve(&sketch, ellipse, segment);
        assert!(
            off < 1e-6,
            "dragged {turn} twelfths round, it sits {off} off"
        );
    }
}

#[test]
fn the_trait_dragged_keeps_brushing_the_ellipse() {
    let (mut sketch, ellipse, segment) = an_ellipse_and_a_trait();
    sketch.add_constraint(Constraint::EllipseTangent {
        ellipse,
        segment,
        at: None,
    });
    sketch.resolve(1.0);
    let end = sketch.segments()[segment.0].end;

    sketch.settle_around(end, DVec2::new(45.0, -10.0), 1.0);

    let after = apart(&sketch, ellipse, segment);
    assert!(after < 1e-3, "it still brushes, standing {after} off it");
}

#[test]
fn an_arc_of_ellipse_is_brushed_where_it_is_drawn() {
    let (mut sketch, ellipse, segment) = an_ellipse_and_a_trait();
    let (south, north) = {
        let points = sketch.ellipse_points(ellipse);
        (points[3], points[4])
    };
    // Cut down to the half the trait below can reach.
    sketch.trim_ellipse(ellipse, Some((north, south)));

    sketch.add_constraint(Constraint::EllipseTangent {
        ellipse,
        segment,
        at: None,
    });
    sketch.resolve(1.0);

    // Measured along the stretch the cut left, not round the whole curve: a
    // trait brushing the half that was taken away would pass `apart` while
    // touching nothing anybody can see.
    let after = along_the_stretch(&sketch, ellipse, segment);
    assert!(
        after < 1e-3,
        "it brushes the stretch it kept, standing {after} off it"
    );
}
