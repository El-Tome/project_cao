//! Two points made one: whatever named the point that goes names the point
//! that stays, an arc as a trait does, and the rules held on it too.
//!
//! Closes #539.
//! - (1) an arc end dropped on a corner: moving the corner takes the arc end
//!   with it, there is one area, and the corner is grabbed as one point —
//!   `an_arc_end_dropped_on_a_corner_goes_where_the_corner_goes`
//! - (2) « Coïncidence » between a corner and an arc end leaves one area
//!   whichever was clicked first —
//!   `a_corner_and_an_arc_end_close_the_shape_whichever_was_clicked_first`
//! - (3) « Concentrique »: the arc's centre is the kept one, and its radius,
//!   opening and rotation are unchanged —
//!   `a_circle_made_concentric_carries_the_arc_on_its_centre_whole`
//! - (4) as many points after compacting as before —
//!   `compacting_after_an_arc_end_was_dropped_on_a_corner_lays_no_point_again`
//! - (5) the corner carries `OnArc`, has not moved, and the arc passes through
//!   it — `a_corner_made_one_with_a_point_held_on_an_arc_keeps_it_on_the_arc`;
//!   a point held on a trait hands its `OnSegment` to the corner the same way —
//!   `a_corner_made_one_with_a_point_held_on_a_trait_keeps_it_on_the_trait`
//! - (6) the arc is gone, and a radius value it carried is now a distance
//!   between its centre and the merged point —
//!   `an_arc_end_dragged_round_onto_its_start_takes_the_arc_and_keeps_its_radius`
//! - an arc end made one with its own centre deletes the arc —
//!   `an_arc_end_made_one_with_its_own_centre_takes_the_arc` — and is refused
//!   while a value holds the radius —
//!   `an_arc_end_made_one_with_its_own_centre_is_refused_while_its_radius_is_typed`
//! - an arc of ellipse whose end is made one with a corner ends on that corner —
//!   `an_arc_of_ellipse_whose_end_is_made_one_with_a_corner_ends_on_that_corner`
//! - after any merge, no arc and no rule names a point that no longer exists —
//!   `no_merge_leaves_an_arc_or_a_rule_on_a_point_that_is_gone`, every pair of
//!   points of a drawing holding one of every hold, merged both ways round
//! - the tests under "What must not break" pass unchanged — no test: the gate
//!   runs them, and that none of them was edited is read in the diff
//! - `docs/sketch.md` says all of the above, with its two new rows in "The
//!   cases, against the code" — no test: it is prose
//!
//! Decided while the issue was written, and held here too: a fixed point keeps
//! its place whichever was clicked first —
//! `a_fixed_point_made_one_with_a_corner_clicked_first_keeps_its_place` — and a
//! rule the merge makes impossible refuses it —
//! `a_point_at_the_middle_of_a_trait_made_one_with_its_end_is_refused`.
//!
//! The review of the first version found three ways it went wrong, held here:
//! a curve carried by its centre took a fixed point with it —
//! `a_fixed_point_on_a_curve_carried_by_its_centre_keeps_its_place`; a centre
//! coming to a fixed one left its arc behind —
//! `a_centre_coming_to_a_fixed_one_brings_its_arc_whole`; and a contact point
//! made one with its circle's centre shrank the circle to nothing —
//! `a_contact_point_made_one_with_the_centre_of_its_circle_is_refused`.
//!
//! Case (5) found the solver turning a group back after it had settled, about
//! the origin, onto a rule tying it to a point held still — a turn read to the
//! first order as one that changes nothing. That is held beside the solver, in
//! `an_arc_tied_to_a_held_point_is_not_turned_back_off_the_rule_that_ties_it`.

use cao_part::{Operation, Outcome, PartDocument, PointRef};
use cao_sketch::{
    ArcId, CircleId, Constraint, DimensionTarget, Element, EllipseId, LaidFrom, PointId, Rule,
    RuleIntent, RulePick, SegmentId, Sketch, Support, WorkPlane, rule_intent,
};
use glam::DVec2;

const CENTRE: DVec2 = DVec2::new(20.0, 0.0);

fn on_circle(degrees: f64) -> DVec2 {
    CENTRE + DVec2::from_angle(degrees.to_radians()) * 5.0
}

fn point_at(sketch: &Sketch, place: DVec2) -> PointId {
    sketch.nearest_point(place, 1e-6).expect("a point there")
}

fn blank() -> PartDocument {
    let mut document = PartDocument::new("Test", "2026-01-02T09:00:00Z".parse().unwrap());
    document.apply(Operation::CreateSketch {
        plane: WorkPlane::XY,
        on: None,
    });
    document
}

/// The points of a "D" not yet closed: a flat side from top to bottom, and an
/// arc of radius 5 from the bottom corner round to 45°, its end a point of its
/// own.
struct OpenD {
    top: PointId,
    centre: PointId,
    loose_end: PointId,
}

fn an_open_d() -> (PartDocument, OpenD) {
    let mut document = blank();
    document.apply(Operation::AddSegment {
        sketch: 0,
        start: PointRef::New(on_circle(90.0)),
        end: PointRef::New(on_circle(-90.0)),
        construction: false,
    });
    let bottom = point_at(&document.sketches()[0], on_circle(-90.0));
    document.apply(Operation::AddArc {
        sketch: 0,
        center: PointRef::New(CENTRE),
        start: PointRef::Existing(bottom),
        end: PointRef::New(on_circle(45.0)),
        construction: false,
    });
    let sketch = &document.sketches()[0];
    let d = OpenD {
        top: point_at(sketch, on_circle(90.0)),
        centre: point_at(sketch, CENTRE),
        loose_end: point_at(sketch, on_circle(45.0)),
    };
    (document, d)
}

/// « Coïncidence » between two points, clicked in this order.
fn made_to_coincide(
    document: &mut PartDocument,
    first: PointId,
    second: PointId,
) -> (PointId, Option<Outcome>) {
    let picks = [first, second].map(|point| RulePick::Element(Element::Point(point)));
    let Some(RuleIntent::Merge { kept, dropped }) =
        rule_intent(Rule::Coincident, &picks, &document.sketches()[0])
    else {
        panic!("two points made to coincide are a merge");
    };
    let outcome = document.apply(Operation::MergePoints {
        sketch: 0,
        kept,
        dropped,
    });
    (kept, outcome)
}

fn drop_on(document: &mut PartDocument, point: PointId, onto: PointId) {
    let sketch = &document.sketches()[0];
    let landing = sketch.point(onto);
    let scale = document.scale();
    let pull = sketch.pull(point, scale);
    let mut settled = sketch.clone();
    settled.settle_pulled(&pull, landing, scale);
    let merged_into = pull.joins(&settled, landing, 0.1, scale);
    assert_eq!(merged_into, Some(onto), "the drop joins nothing");
    document.apply(Operation::MovePoint {
        sketch: 0,
        point,
        position: landing,
        merged_into,
        on: Vec::new(),
        let_go: false,
    });
}

fn move_to(document: &mut PartDocument, point: PointId, position: DVec2) {
    document.apply(Operation::MovePoint {
        sketch: 0,
        point,
        position,
        merged_into: None,
        on: Vec::new(),
        let_go: false,
    });
}

#[test]
fn an_arc_end_dropped_on_a_corner_goes_where_the_corner_goes() {
    let (mut document, d) = an_open_d();

    drop_on(&mut document, d.loose_end, d.top);
    move_to(&mut document, d.top, on_circle(135.0));

    let sketch = &document.sketches()[0];
    let corner = sketch.point(d.top);
    let arc_end = sketch.point(sketch.arcs()[0].end);
    assert!(
        arc_end.distance(corner) < 1e-9,
        "the arc end stayed {} behind the corner",
        arc_end.distance(corner)
    );
    assert_eq!(sketch.regions().len(), 1, "the D is not closed");
    let grabbed = sketch
        .live_points()
        .filter(|(_, place)| place.distance(corner) < 1e-6)
        .count();
    assert_eq!(grabbed, 1, "the corner is {grabbed} points, not one");
}

#[test]
fn a_corner_and_an_arc_end_close_the_shape_whichever_was_clicked_first() {
    for corner_first in [true, false] {
        let (mut document, d) = an_open_d();
        let (first, second) = match corner_first {
            true => (d.top, d.loose_end),
            false => (d.loose_end, d.top),
        };

        let (kept, outcome) = made_to_coincide(&mut document, first, second);

        let sketch = &document.sketches()[0];
        assert_eq!(outcome, None, "the merge was refused");
        assert_eq!(
            sketch.arcs()[0].end,
            kept,
            "the arc ends on a point that went"
        );
        assert_eq!(
            sketch.regions().len(),
            1,
            "the D is not closed, the corner clicked first: {corner_first}"
        );
    }
}

#[test]
fn a_circle_made_concentric_carries_the_arc_on_its_centre_whole() {
    let (mut document, d) = an_open_d();
    for (center, radius) in [
        (PointRef::New(DVec2::new(-20.0, 10.0)), 3.0),
        (PointRef::Existing(d.centre), 1.0),
    ] {
        document.apply(Operation::AddCircle {
            sketch: 0,
            center,
            radius,
            rim: Vec::new(),
            construction: false,
        });
    }
    let picks = [CircleId(0), CircleId(1)].map(|circle| RulePick::Element(Element::Circle(circle)));
    let Some(RuleIntent::Merge { kept, dropped }) =
        rule_intent(Rule::Concentric, &picks, &document.sketches()[0])
    else {
        panic!("two circles made concentric are a merge");
    };

    let outcome = document.apply(Operation::MergePoints {
        sketch: 0,
        kept,
        dropped,
    });

    let sketch = &document.sketches()[0];
    let arc = sketch.arcs()[0];
    let centre = sketch.point(arc.center);
    let turned = (sketch.point(arc.start) - centre).to_angle().to_degrees();
    assert_eq!(outcome, None, "the rule was refused");
    assert_eq!(arc.center, kept, "the arc turns about a point that went");
    assert!(
        centre.distance(DVec2::new(-20.0, 10.0)) < 1e-9,
        "the circle clicked first moved to {centre}"
    );
    assert!(
        (sketch.arc_radius(ArcId(0)) - 5.0).abs() < 1e-6,
        "the arc changed size"
    );
    assert!(
        (sketch.arc_sweep(ArcId(0)).to_degrees() - 135.0).abs() < 1e-6,
        "the arc opens {}° where it opened 135°",
        sketch.arc_sweep(ArcId(0)).to_degrees()
    );
    assert!(
        (turned + 90.0).abs() < 1e-6,
        "the arc turned to start at {turned}°"
    );
}

#[test]
fn compacting_after_an_arc_end_was_dropped_on_a_corner_lays_no_point_again() {
    let (mut document, d) = an_open_d();
    drop_on(&mut document, d.loose_end, d.top);
    let before = document.sketches()[0].live_points().count();

    document.compact_history();

    let after = document.sketches()[0].live_points().count();
    assert_eq!(
        after, before,
        "compacting laid {after} points where there were {before}"
    );
}

/// A free trait standing upright from (40, 0), its foot a corner of its own.
fn a_free_trait(document: &mut PartDocument) -> PointId {
    document.apply(Operation::AddSegment {
        sketch: 0,
        start: PointRef::New(DVec2::new(40.0, 0.0)),
        end: PointRef::New(DVec2::new(40.0, 10.0)),
        construction: false,
    });
    point_at(&document.sketches()[0], DVec2::new(40.0, 0.0))
}

#[test]
fn a_corner_made_one_with_a_point_held_on_an_arc_keeps_it_on_the_arc() {
    let mut document = blank();
    document.apply(Operation::AddArc {
        sketch: 0,
        center: PointRef::New(CENTRE),
        start: PointRef::New(on_circle(-90.0)),
        end: PointRef::New(on_circle(90.0)),
        construction: false,
    });
    document.apply(Operation::AddPoint {
        sketch: 0,
        position: on_circle(0.0),
        on: vec![Support::Arc(ArcId(0))],
    });
    let held = point_at(&document.sketches()[0], on_circle(0.0));
    let corner = a_free_trait(&mut document);

    let (kept, outcome) = made_to_coincide(&mut document, corner, held);

    let sketch = &document.sketches()[0];
    let arc = sketch.arcs()[0];
    let off =
        (sketch.point(kept).distance(sketch.point(arc.center)) - sketch.arc_radius(ArcId(0))).abs();
    assert_eq!(outcome, None, "the merge was refused");
    assert!(
        sketch.constraints().contains(&Constraint::OnArc {
            point: kept,
            arc: ArcId(0)
        }),
        "the rule was lost with the point"
    );
    assert!(off < 1e-6, "the corner stands {off} off the arc");
    assert!(
        sketch.point(kept).distance(DVec2::new(40.0, 0.0)) < 1e-6,
        "the corner clicked first moved to {}",
        sketch.point(kept)
    );
}

#[test]
fn a_corner_made_one_with_a_point_held_on_a_trait_keeps_it_on_the_trait() {
    let mut document = blank();
    document.apply(Operation::AddSegment {
        sketch: 0,
        start: PointRef::New(DVec2::new(0.0, -10.0)),
        end: PointRef::New(DVec2::new(0.0, 10.0)),
        construction: false,
    });
    document.apply(Operation::AddPoint {
        sketch: 0,
        position: DVec2::new(0.0, 2.0),
        on: vec![Support::Segment(SegmentId(0))],
    });
    let held = point_at(&document.sketches()[0], DVec2::new(0.0, 2.0));
    let corner = a_free_trait(&mut document);

    let (kept, outcome) = made_to_coincide(&mut document, corner, held);

    let sketch = &document.sketches()[0];
    let off = sketch
        .point_to_segment(kept, SegmentId(0))
        .expect("the trait is still there");
    assert_eq!(outcome, None, "the merge was refused");
    assert!(
        sketch.constraints().contains(&Constraint::OnSegment {
            point: kept,
            segment: SegmentId(0)
        }),
        "the rule was lost with the point"
    );
    assert!(off < 1e-6, "the corner stands {off} off the trait");
    assert!(
        sketch.point(kept).distance(DVec2::new(40.0, 0.0)) < 1e-6,
        "the corner clicked first moved to {}",
        sketch.point(kept)
    );
}

/// An arc of radius 5 alone, from the bottom round to 45°: its centre, start
/// and end.
fn an_arc_alone() -> (PartDocument, [PointId; 3]) {
    let mut document = blank();
    document.apply(Operation::AddArc {
        sketch: 0,
        center: PointRef::New(CENTRE),
        start: PointRef::New(on_circle(-90.0)),
        end: PointRef::New(on_circle(45.0)),
        construction: false,
    });
    let sketch = &document.sketches()[0];
    let points = [CENTRE, on_circle(-90.0), on_circle(45.0)].map(|place| point_at(sketch, place));
    (document, points)
}

fn type_its_radius(document: &mut PartDocument) {
    document.apply(Operation::SetDimension {
        sketch: 0,
        target: DimensionTarget::ArcRadius(ArcId(0)),
        value: 5.0.into(),
        placement: None,
    });
}

#[test]
fn an_arc_end_dragged_round_onto_its_start_takes_the_arc_and_keeps_its_radius() {
    let (mut document, [centre, start, end]) = an_arc_alone();
    type_its_radius(&mut document);

    drop_on(&mut document, end, start);

    let sketch = &document.sketches()[0];
    let carried = sketch.dimension_of(DimensionTarget::Distance {
        from: centre,
        to: start,
    });
    assert!(sketch.is_erased_arc(ArcId(0)), "the arc is still drawn");
    assert!(
        carried.is_some_and(|value| (value.value - 5.0).abs() < 1e-9 && !value.driven),
        "the radius typed became {carried:?}"
    );
    assert!(
        !sketch.is_erased_point(centre) && !sketch.is_erased_point(start),
        "the arc took its points with it"
    );
}

#[test]
fn an_arc_end_made_one_with_its_own_centre_takes_the_arc() {
    let (mut document, [centre, _, end]) = an_arc_alone();

    let (kept, outcome) = made_to_coincide(&mut document, centre, end);

    let sketch = &document.sketches()[0];
    assert_eq!(outcome, None, "the merge was refused");
    assert!(
        sketch.is_erased_arc(ArcId(0)),
        "an arc of no radius is still drawn"
    );
    assert!(
        !sketch.is_erased_point(kept),
        "the centre went with the arc"
    );
}

#[test]
fn an_arc_end_made_one_with_its_own_centre_is_refused_while_its_radius_is_typed() {
    let (mut document, [centre, _, end]) = an_arc_alone();
    type_its_radius(&mut document);
    let steps = document.history.applied_operations().len();

    let (_, outcome) = made_to_coincide(&mut document, centre, end);

    let sketch = &document.sketches()[0];
    assert_eq!(outcome, Some(Outcome::RuleRefused));
    assert!(
        !sketch.is_erased_arc(ArcId(0)),
        "the arc was taken all the same"
    );
    assert!(
        !sketch.is_erased_point(end),
        "the end was merged all the same"
    );
    assert_eq!(
        document.history.applied_operations().len(),
        steps,
        "the history took a step"
    );
}

#[test]
fn an_arc_of_ellipse_whose_end_is_made_one_with_a_corner_ends_on_that_corner() {
    let mut document = blank();
    let middle = DVec2::new(0.0, 50.0);
    document.apply(Operation::AddEllipse {
        sketch: 0,
        center: PointRef::New(middle),
        first: [-30.0, 30.0].map(|x| PointRef::New(middle + DVec2::new(x, 0.0))),
        second: [-20.0, 20.0].map(|y| PointRef::New(middle + DVec2::new(0.0, y))),
        construction: false,
        drawn: None,
    });
    let on_the_curve = |degrees: f64| {
        let turn = DVec2::from_angle(f64::to_radians(degrees));
        middle + DVec2::new(30.0 * turn.x, 20.0 * turn.y)
    };
    for degrees in [45.0, 135.0] {
        document.apply(Operation::AddPoint {
            sketch: 0,
            position: on_the_curve(degrees),
            on: vec![Support::Ellipse(EllipseId(0))],
        });
    }
    let sketch = &document.sketches()[0];
    let [right, left] = [45.0, 135.0].map(|degrees| point_at(sketch, on_the_curve(degrees)));
    document.apply(Operation::TrimEllipse {
        sketch: 0,
        ellipse: EllipseId(0),
        between: Some((right, left)),
    });
    let corner = a_free_trait(&mut document);

    let (kept, outcome) = made_to_coincide(&mut document, corner, right);

    let sketch = &document.sketches()[0];
    let ends = sketch.ellipse_ends(EllipseId(0));
    let off = sketch.distance_to_ellipse(EllipseId(0), sketch.point(kept));
    assert_eq!(outcome, None, "the merge was refused");
    assert!(
        ends.is_some_and(|(from, to)| from == kept || to == kept),
        "the arc of ellipse ends on {ends:?}, not on the corner {kept:?}"
    );
    assert!(off < 1e-6, "the corner stands {off} off the curve");
}

/// Every point a rule names.
fn named_by(rule: Constraint) -> Vec<PointId> {
    match rule {
        Constraint::OnSegment { point, .. }
        | Constraint::OnCircle { point, .. }
        | Constraint::OnArc { point, .. }
        | Constraint::OnEllipse { point, .. }
        | Constraint::OnAxis { point, .. }
        | Constraint::Midpoint { point, .. }
        | Constraint::Fixed {
            element: Element::Point(point),
        } => vec![point],
        Constraint::Tangent { at, .. }
        | Constraint::ArcTangent { at, .. }
        | Constraint::EllipseTangent { at, .. } => at.into_iter().collect(),
        Constraint::Fixed { .. }
        | Constraint::Perpendicular { .. }
        | Constraint::Parallel { .. }
        | Constraint::Equal { .. }
        | Constraint::EqualRadius { .. }
        | Constraint::EqualRadiusArc { .. }
        | Constraint::EqualRadiusArcCircle { .. }
        | Constraint::Collinear { .. }
        | Constraint::AxisCollinear { .. }
        | Constraint::AxisParallel { .. }
        | Constraint::AxisPerpendicular { .. } => Vec::new(),
    }
}

/// The open "D", a circle with a point on its rim, a point held on the arc,
/// a point at the middle of a free trait, a fixed point, and an arc of ellipse.
fn a_drawing_of_every_hold() -> PartDocument {
    let (mut document, _) = an_open_d();
    document.apply(Operation::AddCircle {
        sketch: 0,
        center: PointRef::New(DVec2::new(-20.0, 0.0)),
        radius: 4.0,
        rim: vec![PointRef::New(DVec2::new(-16.0, 0.0))],
        construction: false,
    });
    document.apply(Operation::AddPoint {
        sketch: 0,
        position: on_circle(0.0),
        on: vec![Support::Arc(ArcId(0))],
    });
    let foot = a_free_trait(&mut document);
    document.apply(Operation::AddPoint {
        sketch: 0,
        position: DVec2::new(40.0, 5.0),
        on: Vec::new(),
    });
    let middle = point_at(&document.sketches()[0], DVec2::new(40.0, 5.0));
    let free_trait = SegmentId(document.sketches()[0].segments().len() - 1);
    for constraint in [
        Constraint::Midpoint {
            point: middle,
            segment: free_trait,
        },
        Constraint::Fixed {
            element: Element::Point(foot),
        },
    ] {
        document.apply(Operation::Constrain {
            sketch: 0,
            constraint,
        });
    }
    let centre = DVec2::new(0.0, 50.0);
    document.apply(Operation::AddEllipse {
        sketch: 0,
        center: PointRef::New(centre),
        first: [-30.0, 30.0].map(|x| PointRef::New(centre + DVec2::new(x, 0.0))),
        second: [-20.0, 20.0].map(|y| PointRef::New(centre + DVec2::new(0.0, y))),
        construction: false,
        drawn: Some([-30.0, 30.0].map(|x| PointRef::New(centre + DVec2::new(x, 0.0)))),
    });
    document
}

#[test]
fn no_merge_leaves_an_arc_or_a_rule_on_a_point_that_is_gone() {
    let drawing = a_drawing_of_every_hold();
    let points: Vec<PointId> = drawing.sketches()[0]
        .live_points()
        .map(|(point, _)| point)
        .collect();

    for kept in &points {
        for dropped in points.iter().filter(|dropped| *dropped != kept) {
            let mut document = drawing.clone();
            document.apply(Operation::MergePoints {
                sketch: 0,
                kept: *kept,
                dropped: *dropped,
            });
            let sketch = &document.sketches()[0];
            let gone = |point: &PointId| sketch.is_erased_point(*point);
            for (_, arc) in sketch.live_arcs() {
                assert!(
                    ![arc.center, arc.start, arc.end].iter().any(gone),
                    "{kept:?} kept and {dropped:?} dropped leave an arc on a point gone"
                );
            }
            for (id, _) in sketch.live_ellipses() {
                assert!(
                    !sketch.ellipse_stands_on(id).iter().any(gone),
                    "{kept:?} kept and {dropped:?} dropped leave an ellipse on a point gone"
                );
            }
            for rule in sketch.constraints() {
                assert!(
                    !named_by(*rule).iter().any(gone),
                    "{kept:?} kept and {dropped:?} dropped leave {rule:?} on a point gone"
                );
            }
        }
    }
}

#[test]
fn a_fixed_point_made_one_with_a_corner_clicked_first_keeps_its_place() {
    let mut document = blank();
    document.apply(Operation::AddPoint {
        sketch: 0,
        position: DVec2::new(10.0, 30.0),
        on: Vec::new(),
    });
    let fixed = point_at(&document.sketches()[0], DVec2::new(10.0, 30.0));
    document.apply(Operation::Constrain {
        sketch: 0,
        constraint: Constraint::Fixed {
            element: Element::Point(fixed),
        },
    });
    let corner = a_free_trait(&mut document);

    let (kept, outcome) = made_to_coincide(&mut document, corner, fixed);

    let sketch = &document.sketches()[0];
    assert_eq!(outcome, None, "the merge was refused");
    assert!(
        sketch.point(kept).distance(DVec2::new(10.0, 30.0)) < 1e-9,
        "the fixed point moved to {}",
        sketch.point(kept)
    );
    assert!(
        sketch.is_held(Element::Point(kept)),
        "the point is no longer fixed"
    );
}

#[test]
fn a_point_at_the_middle_of_a_trait_made_one_with_its_end_is_refused() {
    let mut document = blank();
    let foot = a_free_trait(&mut document);
    document.apply(Operation::AddPoint {
        sketch: 0,
        position: DVec2::new(40.0, 5.0),
        on: Vec::new(),
    });
    let middle = point_at(&document.sketches()[0], DVec2::new(40.0, 5.0));
    document.apply(Operation::Constrain {
        sketch: 0,
        constraint: Constraint::Midpoint {
            point: middle,
            segment: SegmentId(0),
        },
    });

    let (_, outcome) = made_to_coincide(&mut document, foot, middle);

    let sketch = &document.sketches()[0];
    assert_eq!(outcome, Some(Outcome::RuleRefused));
    assert!(
        !sketch.is_erased_point(middle),
        "the middle was merged all the same"
    );
    assert!(
        sketch.segment_length(SegmentId(0)) > 9.0,
        "the trait shrank to {}",
        sketch.segment_length(SegmentId(0))
    );
}

/// The open "D", a circle away from it clicked first, and a small circle drawn
/// on the arc's own centre.
fn a_d_and_two_circles(document: &mut PartDocument, d: &OpenD) -> (PointId, PointId) {
    for (center, radius) in [
        (PointRef::New(DVec2::new(-20.0, 10.0)), 3.0),
        (PointRef::Existing(d.centre), 1.0),
    ] {
        document.apply(Operation::AddCircle {
            sketch: 0,
            center,
            radius,
            rim: Vec::new(),
            construction: false,
        });
    }
    let picks = [CircleId(0), CircleId(1)].map(|circle| RulePick::Element(Element::Circle(circle)));
    let Some(RuleIntent::Merge { kept, dropped }) =
        rule_intent(Rule::Concentric, &picks, &document.sketches()[0])
    else {
        panic!("two circles made concentric are a merge");
    };
    (kept, dropped)
}

fn fix(document: &mut PartDocument, point: PointId) {
    document.apply(Operation::Constrain {
        sketch: 0,
        constraint: Constraint::Fixed {
            element: Element::Point(point),
        },
    });
}

#[test]
fn a_fixed_point_on_a_curve_carried_by_its_centre_keeps_its_place() {
    let (mut document, d) = an_open_d();
    let bottom = point_at(&document.sketches()[0], on_circle(-90.0));
    fix(&mut document, bottom);
    let (kept, dropped) = a_d_and_two_circles(&mut document, &d);

    let outcome = document.apply(Operation::MergePoints {
        sketch: 0,
        kept,
        dropped,
    });

    let sketch = &document.sketches()[0];
    assert_eq!(outcome, None, "the merge was refused");
    assert!(
        sketch.point(bottom).distance(on_circle(-90.0)) < 1e-9,
        "the fixed end moved to {}",
        sketch.point(bottom)
    );
}

#[test]
fn a_centre_coming_to_a_fixed_one_brings_its_arc_whole() {
    let (mut document, d) = an_open_d();
    document.apply(Operation::AddCircle {
        sketch: 0,
        center: PointRef::Existing(d.centre),
        radius: 1.0,
        rim: Vec::new(),
        construction: false,
    });
    document.apply(Operation::AddCircle {
        sketch: 0,
        center: PointRef::New(DVec2::new(-20.0, 10.0)),
        radius: 3.0,
        rim: Vec::new(),
        construction: false,
    });
    let elsewhere = point_at(&document.sketches()[0], DVec2::new(-20.0, 10.0));
    fix(&mut document, elsewhere);
    let picks = [CircleId(0), CircleId(1)].map(|circle| RulePick::Element(Element::Circle(circle)));
    let Some(RuleIntent::Merge { kept, dropped }) =
        rule_intent(Rule::Concentric, &picks, &document.sketches()[0])
    else {
        panic!("two circles made concentric are a merge");
    };

    let outcome = document.apply(Operation::MergePoints {
        sketch: 0,
        kept,
        dropped,
    });

    let sketch = &document.sketches()[0];
    let arc = sketch.arcs()[0];
    let centre = sketch.point(arc.center);
    let turned = (sketch.point(arc.start) - centre).to_angle().to_degrees();
    assert_eq!(outcome, None, "the merge was refused");
    assert!(
        centre.distance(DVec2::new(-20.0, 10.0)) < 1e-9,
        "the arc turns about {centre}, not the fixed centre"
    );
    assert!(
        (sketch.arc_radius(ArcId(0)) - 5.0).abs() < 1e-6,
        "the arc changed size"
    );
    assert!(
        (sketch.arc_sweep(ArcId(0)).to_degrees() - 135.0).abs() < 1e-6,
        "the arc opens {}° where it opened 135°",
        sketch.arc_sweep(ArcId(0)).to_degrees()
    );
    assert!(
        (turned + 90.0).abs() < 1e-6,
        "the arc turned to start at {turned}°"
    );
}

#[test]
fn a_contact_point_made_one_with_the_centre_of_its_circle_is_refused() {
    let mut document = blank();
    document.apply(Operation::AddCircle {
        sketch: 0,
        center: PointRef::New(DVec2::new(0.0, 20.0)),
        radius: 5.0,
        rim: Vec::new(),
        construction: false,
    });
    document.apply(Operation::AddSegment {
        sketch: 0,
        start: PointRef::New(DVec2::new(-20.0, 10.0)),
        end: PointRef::New(DVec2::new(20.0, 10.0)),
        construction: false,
    });
    document.apply(Operation::Constrain {
        sketch: 0,
        constraint: Constraint::Tangent {
            circle: CircleId(0),
            segment: SegmentId(0),
            at: None,
            from: LaidFrom::Nowhere,
        },
    });
    let sketch = &document.sketches()[0];
    let contact = sketch
        .constraints()
        .iter()
        .find_map(|rule| match rule {
            Constraint::Tangent { at, .. } => *at,
            _ => None,
        })
        .expect("a tangency laid with its contact point");
    let centre = sketch.circles()[0].center;

    let (_, outcome) = made_to_coincide(&mut document, centre, contact);

    let sketch = &document.sketches()[0];
    assert_eq!(outcome, Some(Outcome::RuleRefused));
    assert!(
        sketch.circles()[0].radius > 1.0,
        "the circle shrank to {}",
        sketch.circles()[0].radius
    );
}
