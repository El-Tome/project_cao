//! What the trim tool makes of a circle: the stretch clicked goes, and what is
//! left is an arc of that very circle.
//!
//! Closes #290.
//! - a click on a circle carrying at least two points takes out the stretch it
//!   fell in, and what is left is an arc of the same circle, ending on those
//!   two points — `a_cut_between_two_points_on_a_circle_leaves_an_arc_of_it`,
//!   `a_click_names_the_stretch_of_the_round_it_fell_in`,
//!   `cutting_a_guide_circle_leaves_a_guide`,
//!   `a_circle_is_never_cut_at_a_point_that_does_not_sit_on_it`,
//!   `naming_the_same_point_twice_cuts_a_circle_nowhere_at_all`
//! - a circle with no point on it, or with only one, is erased whole, and the
//!   single point stays — `a_circle_nothing_sits_on_is_taken_away_whole`,
//!   `a_circle_carrying_one_point_is_taken_away_whole_and_the_point_stays`
//! - a diameter dimension comes back as a radius dimension on the arc —
//!   `a_diameter_typed_on_a_circle_is_read_as_a_reach_on_the_arc`
//! - a radius dimension stays a radius dimension on the arc —
//!   `a_reach_typed_on_a_circle_is_read_on_the_arc_unchanged`
//! - "same radius as another circle" still holds, through a rule for an arc and
//!   a circle of the same radius that this issue brings —
//!   `a_circle_held_the_same_reach_as_another_holds_the_arc_to_it`
//! - a tangency to a trait stays as an arc tangency when the contact is on the
//!   stretch kept, and goes when the contact was on the stretch that went —
//!   `a_trait_brushing_a_circle_goes_on_brushing_the_arc_it_touches`,
//!   `a_cut_counts_the_brush_it_took_away_with_the_stretch_it_touched`
//! - a point held on the circle and standing on the stretch kept stays held on
//!   the arc — `a_point_held_on_a_circle_goes_on_being_held_by_the_arc_it_sits_on`
//! - a point left on nothing after successive trims stays where it is, free —
//!   `a_point_held_on_the_stretch_a_cut_takes_away_loses_what_held_it`
//! - the region walk (#267) encloses what it should around an arc cut from a
//!   circle — `an_arc_cut_from_a_circle_encloses_what_it_bounds`
//! - trimming a straight trait (#277) and trimming an arc (#289) are untouched
//!   — no test: neither path was opened, and `what_a_cut_keeps.rs`,
//!   `what_a_cut_costs.rs` and `what_a_cut_leaves_of_an_arc.rs` already hold
//!   them in the gate

use cao_sketch::{CircleId, Constraint, DimensionTarget, PointId, Sketch, WorkPlane};
use glam::DVec2;

const REACH: f64 = 10.0;
const TOLERANCE: f64 = 1e-9;

fn on_the_rim(sketch: &mut Sketch, degrees: f64) -> PointId {
    sketch.add_point(DVec2::from_angle(degrees.to_radians()) * REACH)
}

/// A whole round about the origin, with two points sitting on it a sixth and a
/// third of the way round.
fn a_round_with_two_points() -> (Sketch, CircleId, [PointId; 2]) {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let circle = sketch.add_circle(Sketch::ORIGIN, REACH);
    let first = on_the_rim(&mut sketch, 60.0);
    let second = on_the_rim(&mut sketch, 120.0);
    (sketch, circle, [first, second])
}

#[test]
fn a_cut_between_two_points_on_a_circle_leaves_an_arc_of_it() {
    let (mut sketch, circle, [first, second]) = a_round_with_two_points();

    let trimmed = sketch
        .trim_circle(circle, Some((first, second)))
        .expect("a cut that can be made");

    assert!(
        sketch.is_erased_circle(circle),
        "the circle the cut was made in is gone",
    );
    let arc = trimmed.arc.expect("an arc of the circle stayed");
    let sweep = sketch.arc_sweep(arc).to_degrees();
    assert!(
        (sweep - 300.0).abs() < TOLERANCE,
        "the whole round less the sixth that went, 300 degrees, got {sweep}",
    );
    let reach = sketch.arc_radius(arc);
    assert!(
        (reach - REACH).abs() < TOLERANCE,
        "a cut takes nothing off the reach, got {reach}",
    );
}

#[test]
fn a_click_names_the_stretch_of_the_round_it_fell_in() {
    let (sketch, circle, [first, second]) = a_round_with_two_points();

    let short_way = sketch
        .circle_stretch_at(circle, DVec2::from_angle(90.0_f64.to_radians()) * REACH)
        .expect("a stretch under the click");
    assert_eq!(
        short_way,
        (first, second),
        "a click between the two points asks for the stretch that runs between them",
    );

    let long_way = sketch
        .circle_stretch_at(circle, DVec2::new(REACH, 0.0))
        .expect("a stretch under the click");
    assert_eq!(
        long_way,
        (second, first),
        "a click the other side asks for the rest of the round, the other way about",
    );
}

#[test]
fn a_circle_nothing_sits_on_is_taken_away_whole() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let circle = sketch.add_circle(Sketch::ORIGIN, REACH);

    assert!(
        sketch
            .circle_stretch_at(circle, DVec2::new(REACH, 0.0))
            .is_none(),
        "a round with nothing on it has no stretch to run between",
    );
    let trimmed = sketch
        .trim_circle(circle, None)
        .expect("a cut that can be made");

    assert!(sketch.is_erased_circle(circle), "the whole round went");
    assert_eq!(trimmed.arc, None, "nothing was left to keep");
}

#[test]
fn a_circle_carrying_one_point_is_taken_away_whole_and_the_point_stays() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let circle = sketch.add_circle(Sketch::ORIGIN, REACH);
    let handle = on_the_rim(&mut sketch, 0.0);
    sketch.add_constraint(Constraint::OnCircle {
        point: handle,
        circle,
    });

    assert!(
        sketch
            .circle_stretch_at(circle, DVec2::from_angle(90.0_f64.to_radians()) * REACH)
            .is_none(),
        "one point is nothing to run between",
    );
    let trimmed = sketch
        .trim_circle(circle, None)
        .expect("a cut that can be made");

    assert_eq!(trimmed.arc, None, "nothing was left to keep");
    assert!(
        !sketch.is_erased_point(handle),
        "the point the round carried stays where it is",
    );
    assert!(
        !sketch
            .constraints()
            .iter()
            .any(|rule| matches!(rule, Constraint::OnCircle { .. })),
        "and is free, with nothing left holding it",
    );
}

#[test]
fn a_diameter_typed_on_a_circle_is_read_as_a_reach_on_the_arc() {
    let (mut sketch, circle, [first, second]) = a_round_with_two_points();
    sketch.set_dimension(DimensionTarget::Diameter(circle), 40.0, false);

    let arc = sketch
        .trim_circle(circle, Some((first, second)))
        .expect("a cut that can be made")
        .arc
        .expect("an arc of the circle stayed");

    let reach = sketch
        .dimension_of(DimensionTarget::ArcRadius(arc))
        .expect("the size the circle was given is read on the arc");
    assert!(
        (reach.value - 20.0).abs() < TOLERANCE,
        "half of the 40 across the round, got {}",
        reach.value,
    );
    assert!(
        sketch
            .dimension_of(DimensionTarget::Diameter(circle))
            .is_none(),
        "and is no longer read across a circle the drawing no longer has",
    );
}

#[test]
fn a_reach_typed_on_a_circle_is_read_on_the_arc_unchanged() {
    let (mut sketch, circle, [first, second]) = a_round_with_two_points();
    sketch.set_dimension(DimensionTarget::Radius(circle), 20.0, false);

    let arc = sketch
        .trim_circle(circle, Some((first, second)))
        .expect("a cut that can be made")
        .arc
        .expect("an arc of the circle stayed");

    let reach = sketch
        .dimension_of(DimensionTarget::ArcRadius(arc))
        .expect("the reach the circle was given is read on the arc");
    assert!(
        (reach.value - 20.0).abs() < TOLERANCE,
        "the very reach that was typed, got {}",
        reach.value,
    );
}

#[test]
fn a_circle_held_the_same_reach_as_another_holds_the_arc_to_it() {
    let (mut sketch, circle, [first, second]) = a_round_with_two_points();
    let elsewhere = sketch.add_point(DVec2::new(100.0, 0.0));
    let other = sketch.add_circle(elsewhere, REACH);
    sketch.add_constraint(Constraint::EqualRadius {
        first: circle,
        second: other,
    });

    let arc = sketch
        .trim_circle(circle, Some((first, second)))
        .expect("a cut that can be made")
        .arc
        .expect("an arc of the circle stayed");

    assert!(
        sketch
            .constraints()
            .contains(&Constraint::EqualRadiusArcCircle { arc, circle: other }.normalised()),
        "the arc is held to the other round's reach, as the circle was: {:?}",
        sketch.constraints(),
    );
}

/// A round about the origin brushed from below by a trait, with two points
/// sitting on it wherever the caller asks.
fn a_round_a_trait_brushes(degrees: [f64; 2]) -> (Sketch, CircleId, [PointId; 2]) {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let circle = sketch.add_circle(Sketch::ORIGIN, REACH);
    let west = sketch.add_point(DVec2::new(-20.0, -REACH));
    let east = sketch.add_point(DVec2::new(20.0, -REACH));
    let below = sketch.add_segment(west, east);
    sketch.add_constraint(Constraint::Tangent {
        circle,
        segment: below,
        at: None,
    });
    let first = on_the_rim(&mut sketch, degrees[0]);
    let second = on_the_rim(&mut sketch, degrees[1]);
    (sketch, circle, [first, second])
}

#[test]
fn a_trait_brushing_a_circle_goes_on_brushing_the_arc_it_touches() {
    let (mut sketch, circle, [first, second]) = a_round_a_trait_brushes([60.0, 120.0]);

    let trimmed = sketch
        .trim_circle(circle, Some((first, second)))
        .expect("a cut that can be made");
    let arc = trimmed.arc.expect("an arc of the circle stayed");

    assert!(
        sketch.constraints().iter().any(
            |rule| matches!(rule, Constraint::ArcTangent { arc: brushed, .. } if *brushed == arc)
        ),
        "the trait brushes the stretch that stayed, so it brushes the arc: {:?}",
        sketch.constraints(),
    );
    assert_eq!(trimmed.rules_dropped, 0, "nothing was lost on the way");
}

#[test]
fn a_cut_counts_the_brush_it_took_away_with_the_stretch_it_touched() {
    let (mut sketch, circle, [first, second]) = a_round_a_trait_brushes([240.0, 300.0]);

    let trimmed = sketch
        .trim_circle(circle, Some((first, second)))
        .expect("a cut that can be made");

    assert!(
        !sketch
            .constraints()
            .iter()
            .any(|rule| matches!(rule, Constraint::ArcTangent { .. })),
        "the trait brushed the stretch that went, so it brushes nothing now: {:?}",
        sketch.constraints(),
    );
    assert_eq!(
        trimmed.rules_dropped, 1,
        "and the drawing says it lost a rule on the way",
    );
}

/// The round with its two points, and a third held on it wherever asked.
fn a_round_holding_a_point(degrees: f64) -> (Sketch, CircleId, [PointId; 2], PointId) {
    let (mut sketch, circle, ends) = a_round_with_two_points();
    let held = on_the_rim(&mut sketch, degrees);
    sketch.add_constraint(Constraint::OnCircle {
        point: held,
        circle,
    });
    (sketch, circle, ends, held)
}

#[test]
fn a_point_held_on_a_circle_goes_on_being_held_by_the_arc_it_sits_on() {
    let (mut sketch, circle, [first, second], held) = a_round_holding_a_point(270.0);

    let arc = sketch
        .trim_circle(circle, Some((first, second)))
        .expect("a cut that can be made")
        .arc
        .expect("an arc of the circle stayed");

    assert!(
        sketch
            .constraints()
            .contains(&Constraint::OnArc { point: held, arc }),
        "the point stands on the stretch that stayed, so the arc holds it: {:?}",
        sketch.constraints(),
    );
}

#[test]
fn a_point_held_on_the_stretch_a_cut_takes_away_loses_what_held_it() {
    let (mut sketch, circle, [first, second], held) = a_round_holding_a_point(90.0);

    sketch
        .trim_circle(circle, Some((first, second)))
        .expect("a cut that can be made");

    assert!(!sketch.is_erased_point(held), "the point stays where it is",);
    assert!(
        !sketch
            .constraints()
            .iter()
            .any(|rule| matches!(rule, Constraint::OnArc { point, .. } if *point == held)),
        "free, with nothing left holding it: {:?}",
        sketch.constraints(),
    );
}

#[test]
fn an_arc_cut_from_a_circle_encloses_what_it_bounds() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let circle = sketch.add_circle(Sketch::ORIGIN, REACH);
    let east = on_the_rim(&mut sketch, 0.0);
    let west = on_the_rim(&mut sketch, 180.0);
    sketch.add_segment(east, west);

    let whole = enclosed(&sketch);
    sketch
        .trim_circle(circle, Some((east, west)))
        .expect("a cut that can be made");
    let half = enclosed(&sketch);

    let round = std::f64::consts::PI * REACH * REACH;
    assert!(
        (whole - round).abs() < round * 0.01,
        "a round split by a chord encloses the whole disc, got {whole}",
    );
    assert!(
        (half - round / 2.0).abs() < round * 0.01,
        "the stretch that stayed, closed by the chord, encloses half of it, got {half}",
    );
}

/// Everything the drawing shuts in, added up.
fn enclosed(sketch: &Sketch) -> f64 {
    sketch
        .regions()
        .iter()
        .flat_map(|region| region.triangles.iter())
        .map(|[a, b, c]| (*b - *a).perp_dot(*c - *a).abs() * 0.5)
        .sum()
}

#[test]
fn cutting_a_guide_circle_leaves_a_guide() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let guide = sketch.add_construction_circle(Sketch::ORIGIN, REACH);
    let first = on_the_rim(&mut sketch, 60.0);
    let second = on_the_rim(&mut sketch, 120.0);

    let arc = sketch
        .trim_circle(guide, Some((first, second)))
        .expect("a cut that can be made")
        .arc
        .expect("an arc of the circle stayed");

    assert!(
        sketch.arc(arc).construction,
        "a round that only helps build the drawing still only helps once cut",
    );
}

#[test]
fn a_circle_is_never_cut_at_a_point_that_does_not_sit_on_it() {
    let (mut sketch, circle, [first, _]) = a_round_with_two_points();
    let beside = sketch.add_point(DVec2::new(REACH * 2.0, 0.0));

    assert_eq!(
        sketch.trim_circle(circle, Some((first, beside))),
        None,
        "a cut can only end on the rim, whatever was recorded",
    );
    assert!(
        !sketch.is_erased_circle(circle),
        "and the round is left standing",
    );
}

#[test]
fn naming_the_same_point_twice_cuts_a_circle_nowhere_at_all() {
    let (mut sketch, circle, [first, _]) = a_round_with_two_points();

    assert_eq!(
        sketch.trim_circle(circle, Some((first, first))),
        None,
        "one point is no stretch, and the round has no two ends to keep",
    );
}
