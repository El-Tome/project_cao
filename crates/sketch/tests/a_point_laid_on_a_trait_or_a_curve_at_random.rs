//! « Coïncidence » between a point and a trait, or an arc, a circle or an
//! ellipse, laid on drawings made at random: every point against every trait
//! and every curve, both ways round (#548, #554).
//!
//! Closes #548.
//! - a campaign of random drawings, with points placed on them and some of
//!   them fixed, holds after each rule that the first thing clicked has not
//!   moved — or, when the second could not come, the second — that nothing
//!   moved but the two and what they belong to, that the point stands on the
//!   trait's line, or that the rule was refused —
//!   `a_point_and_a_trait_land_as_clicked_on_drawings_made_at_random`
//!
//! Closes #554.
//! - #548's campaign takes curves as well as traits: every point against
//!   every curve of random drawings, both ways round, holding the same
//!   property, the point standing on the curve —
//!   `a_point_and_a_curve_land_as_clicked_on_drawings_made_at_random`
//!
//! The gate plays a fixed range of seeds. A longer campaign is run by hand:
//!
//! ```text
//! CAO_FUZZ_SECONDS=300 cargo test --release -p cao_sketch \
//!     --test a_point_laid_on_a_trait_or_a_curve_at_random -- --ignored --nocapture
//! ```
//!
//! `CAO_FUZZ_SEED` starts it from a given seed rather than from the clock.
//!
//! Each drawing is played as drawn, and again with one trait — or, for the
//! curves, one curve — fixed and a trait's length typed: the generator lays
//! neither.
//!
//! The first one clicked stays, or — when the second could not come — the
//! second stays and the first comes to it. Could not come is read as: the two
//! are one shape, or one of them is anchored (a value, a fixed thing, a point
//! held on an axis, the origin), or the point's foot on the trait's line — or
//! its place on what is drawn of the curve — is already another point's. Between two free shapes apart, anything else is a
//! flaw. A circle that grows or shrinks has moved, though no point of it has.
//!
//! On a curve, two more things hold whichever way it landed: the curve keeps
//! its size, and the point stands on what is drawn of it, not on the rest of
//! the circle or the ellipse an arc is cut from.
//!
//! Seeds 400 to 5199, some 340 000 rules laid, still found three drawings.
//! Two of them, 2792 and 3874, held a point on half an ellipse that was taken
//! back onto the stretch drawn whenever the drawing settled — #531, which put
//! them right, and points held on a cut ellipse are weighed like the rest
//! since. 2783, a single rule, is not understood yet. Run again from 400 to
//! 7955 once #531 had landed, some 530 000 rules, the campaign found 2783 and
//! one more, 7430: the same flaw, the trait clicked first coming to the
//! point, and found on `main` as well. Run from 400 to 17880 with the curves
//! in (#554), some 1 200 000 rules, it found ten drawings: 2783, 7430, 11049,
//! 12692, 13930, 14263, 15093, 16633, 17194 and 17561. Every one of them
//! fails on `main` too, where 14263 breaks some forty rules and here two, by
//! a hair.
//!
//! The curves, from 60 to 7016, some 270 000 rules, found nothing.
//!
//! Two kinds of drawing the generator makes are left out, since no rule can
//! land on them: one holding a trait of no length — a rectangle of no width —
//! and one that does not settle on its own.

// The drawing is the campaign's; this file only lays a rule on it.
#[allow(dead_code, unused_imports)]
mod random_sketches;

use std::f64::consts::{PI, TAU};
use std::time::{Duration, Instant, SystemTime};

use cao_sketch::{
    CLEAR_OF_AN_END, Constraint, DimensionTarget, Element, LaidFrom, LengthOutcome, PointId, Rule,
    RuleIntent, RulePick, SegmentId, Sketch, rule_intent,
};
use glam::DVec2;
use random_sketches::{drawn, laid};

/// The seeds the gate plays: enough to meet every gesture the generator
/// draws with, few enough to take about a second.
const SEEDS: std::ops::Range<u64> = 0..60;

/// How many rules are laid on one drawing, spread over its pairs: a drawing
/// of forty points and twenty traits has sixteen hundred of them.
const PAIRS_PER_DRAWING: usize = 48;

/// The points a piece of the drawing stands on.
fn points_of(sketch: &Sketch, element: Element) -> Vec<PointId> {
    match element {
        Element::Point(point) => vec![point],
        Element::Segment(id) => {
            let side = sketch.segments()[id.0];
            vec![side.start, side.end]
        }
        Element::Circle(id) => vec![sketch.circles()[id.0].center],
        Element::Arc(id) => {
            let arc = sketch.arcs()[id.0];
            vec![arc.center, arc.start, arc.end]
        }
        Element::Ellipse(id) => sketch.ellipse_stands_on(id),
    }
}

/// The points a value measures.
fn measured(sketch: &Sketch, target: DimensionTarget) -> Vec<PointId> {
    let elements = match target {
        DimensionTarget::Length(segment) | DimensionTarget::AxisAngle { segment, .. } => {
            vec![Element::Segment(segment)]
        }
        DimensionTarget::Distance { from, to } | DimensionTarget::Projected { from, to, .. } => {
            vec![Element::Point(from), Element::Point(to)]
        }
        DimensionTarget::Angle { first, second, .. }
        | DimensionTarget::AngleBetween { first, second, .. } => {
            vec![Element::Segment(first), Element::Segment(second)]
        }
        DimensionTarget::PointToSegment { point, segment } => {
            vec![Element::Point(point), Element::Segment(segment)]
        }
        DimensionTarget::Radius(circle) | DimensionTarget::Diameter(circle) => {
            vec![Element::Circle(circle)]
        }
        DimensionTarget::ArcRadius(arc) | DimensionTarget::ArcSweep(arc) => {
            vec![Element::Arc(arc)]
        }
    };
    elements
        .into_iter()
        .flat_map(|element| points_of(sketch, element))
        .collect()
}

/// Which shape each point belongs to, as the rank of one point standing for
/// it: points tied by a curve, a rule or a value are one shape.
fn shapes(sketch: &Sketch) -> Vec<usize> {
    let mut parent: Vec<usize> = (0..sketch.points().len()).collect();
    fn root(parent: &mut [usize], mut point: usize) -> usize {
        while parent[point] != point {
            parent[point] = parent[parent[point]];
            point = parent[point];
        }
        point
    }
    let mut tie = |points: &[PointId]| {
        for pair in points.windows(2) {
            let (a, b) = (root(&mut parent, pair[0].0), root(&mut parent, pair[1].0));
            parent[a] = b;
        }
    };
    let of = |element: Element| points_of(sketch, element);
    let mut ties: Vec<Vec<PointId>> = Vec::new();
    for (id, _) in sketch.live_segments() {
        ties.push(of(Element::Segment(id)));
    }
    for (id, _) in sketch.live_arcs() {
        ties.push(of(Element::Arc(id)));
    }
    for (id, _) in sketch.live_ellipses() {
        ties.push(of(Element::Ellipse(id)));
    }
    for rule in sketch.constraints() {
        let elements: Vec<Element> = match *rule {
            Constraint::Perpendicular { first, second }
            | Constraint::Parallel { first, second }
            | Constraint::Equal { first, second }
            | Constraint::Collinear { first, second } => {
                vec![Element::Segment(first), Element::Segment(second)]
            }
            Constraint::EqualRadius { first, second } => {
                vec![Element::Circle(first), Element::Circle(second)]
            }
            Constraint::EqualRadiusArc { first, second } => {
                vec![Element::Arc(first), Element::Arc(second)]
            }
            Constraint::EqualRadiusArcCircle { arc, circle } => {
                vec![Element::Arc(arc), Element::Circle(circle)]
            }
            Constraint::OnSegment { point, segment, .. }
            | Constraint::Midpoint { point, segment } => {
                vec![Element::Point(point), Element::Segment(segment)]
            }
            Constraint::OnCircle { point, circle, .. } => {
                vec![Element::Point(point), Element::Circle(circle)]
            }
            Constraint::OnArc { point, arc, .. } => vec![Element::Point(point), Element::Arc(arc)],
            Constraint::OnEllipse { point, ellipse, .. } => {
                vec![Element::Point(point), Element::Ellipse(ellipse)]
            }
            Constraint::OnAxis { point, .. } => vec![Element::Point(point)],
            Constraint::Tangent {
                circle,
                segment,
                at,
                ..
            } => [Element::Circle(circle), Element::Segment(segment)]
                .into_iter()
                .chain(at.map(Element::Point))
                .collect(),
            Constraint::ArcTangent {
                arc, segment, at, ..
            } => [Element::Arc(arc), Element::Segment(segment)]
                .into_iter()
                .chain(at.map(Element::Point))
                .collect(),
            Constraint::EllipseTangent {
                ellipse,
                segment,
                at,
                ..
            } => [Element::Ellipse(ellipse), Element::Segment(segment)]
                .into_iter()
                .chain(at.map(Element::Point))
                .collect(),
            Constraint::AxisCollinear { segment, .. }
            | Constraint::AxisParallel { segment, .. }
            | Constraint::AxisPerpendicular { segment, .. } => vec![Element::Segment(segment)],
            Constraint::Fixed { element } => vec![element],
        };
        ties.push(elements.into_iter().flat_map(&of).collect());
    }
    for value in sketch.dimensions() {
        ties.push(measured(sketch, value.target));
    }
    for points in &ties {
        tie(points);
    }
    (0..parent.len())
        .map(|point| root(&mut parent, point))
        .collect()
}

/// Whether a shape holds anything that may keep it from coming: a value, a
/// fixed thing, a point held on an axis, the origin.
fn anchored(sketch: &Sketch, shapes: &[usize], shape: usize) -> bool {
    let in_it = |point: PointId| shapes[point.0] == shape;
    let fixed = sketch.constraints().iter().any(|rule| match *rule {
        Constraint::OnAxis { point, .. } => in_it(point),
        Constraint::Fixed { element } => match element {
            Element::Point(point) => in_it(point),
            Element::Segment(id) => in_it(sketch.segments()[id.0].start),
            Element::Circle(id) => in_it(sketch.circles()[id.0].center),
            Element::Arc(id) => in_it(sketch.arcs()[id.0].center),
            Element::Ellipse(id) => in_it(sketch.ellipses()[id.0].center),
        },
        _ => false,
    });
    let valued = sketch
        .dimensions()
        .iter()
        .any(|value| measured(sketch, value.target).into_iter().any(in_it));
    fixed || valued || in_it(Sketch::ORIGIN)
}

/// How a rule laid as it should went.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Landed {
    /// The first one clicked stayed.
    InOrder,
    /// The second could not come, and the first came to it.
    Reversed,
    Refused,
    /// The drawing carried the rule already.
    AlreadyThere,
}

/// How far a point stands off a trait's line, or off the whole of a curve —
/// an arc's whole circle.
fn off(sketch: &Sketch, point: PointId, on: Element) -> f64 {
    let place = sketch.point(point);
    match on {
        Element::Segment(segment) => sketch
            .point_to_segment(point, segment)
            .unwrap_or(f64::INFINITY),
        Element::Circle(id) => {
            let round = sketch.circles()[id.0];
            (sketch.point(round.center).distance(place) - round.radius).abs()
        }
        Element::Arc(id) => {
            let centre = sketch.point(sketch.arcs()[id.0].center);
            (centre.distance(place) - sketch.arc_radius(id)).abs()
        }
        Element::Ellipse(id) => {
            let drawn = sketch.ellipse_draft(id);
            drawn.at(drawn.turn_nearest(place)).distance(place)
        }
        Element::Point(_) => f64::INFINITY,
    }
}

/// Where the point comes when it is the one that comes: square onto a
/// trait's line, onto what is drawn of a curve at the nearest place, clear of
/// its ends.
fn landing(sketch: &Sketch, point: PointId, on: Element) -> Option<DVec2> {
    let place = sketch.point(point);
    let clear = |turn: f64, from: f64, sweep: f64| {
        let half = sweep / 2.0;
        let room = half - CLEAR_OF_AN_END.min(half);
        let off = (turn - from - half + PI).rem_euclid(TAU) - PI;
        from + half + off.clamp(-room, room)
    };
    match on {
        Element::Segment(segment) => sketch.foot_on_segment(point, segment),
        Element::Circle(id) => {
            let round = sketch.circles()[id.0];
            let centre = sketch.point(round.center);
            Some(centre + (place - centre).try_normalize()? * round.radius)
        }
        Element::Arc(id) => {
            let drawn = sketch.arc_draft(id);
            let from = (drawn.start - drawn.centre).to_angle();
            let turn = clear(
                (place - drawn.centre).to_angle(),
                from,
                sketch.arc_sweep(id),
            );
            Some(drawn.centre + DVec2::from_angle(turn) * sketch.arc_radius(id))
        }
        Element::Ellipse(id) => {
            let drawn = sketch.ellipse_draft(id);
            let turn = drawn.turn_nearest(place);
            let turn = match sketch.ellipse_ends(id) {
                Some(_) => {
                    let (from, sweep) = sketch.ellipse_run(id);
                    clear(turn, from, sweep)
                }
                None => turn,
            };
            Some(drawn.at(turn))
        }
        Element::Point(_) => None,
    }
}

/// How big a curve is: the radius of a circle or an arc, the two reaches of
/// an ellipse. Nothing for a trait.
fn how_big(sketch: &Sketch, on: Element) -> Vec<f64> {
    match on {
        Element::Circle(id) => vec![sketch.circles()[id.0].radius],
        Element::Arc(id) => vec![sketch.arc_radius(id)],
        Element::Ellipse(id) => {
            let drawn = sketch.ellipse_draft(id);
            vec![drawn.first.length(), drawn.second]
        }
        Element::Point(_) | Element::Segment(_) => Vec::new(),
    }
}

/// How far a point stands off what is drawn of a curve; nought for a trait,
/// which holds its whole line.
fn off_what_is_drawn(sketch: &Sketch, point: PointId, on: Element) -> f64 {
    let place = sketch.point(point);
    match on {
        Element::Arc(id) => sketch.distance_to_arc(id, place),
        Element::Ellipse(id) => sketch.distance_to_ellipse(id, place),
        Element::Circle(_) => off(sketch, point, on),
        Element::Point(_) | Element::Segment(_) => 0.0,
    }
}

/// How far each circle of a shape grew or shrank.
fn resized(before: &Sketch, after: &Sketch, shapes: &[usize], shape: usize) -> f64 {
    before
        .live_circles()
        .filter(|(_, round)| shapes[round.center.0] == shape)
        .map(|(id, round)| (round.radius - after.circles()[id.0].radius).abs())
        .fold(0.0, f64::max)
}

/// How laying « Coïncidence » on one point and one trait or curve, in this
/// order, went — or what went wrong.
fn landed(
    sketch: &Sketch,
    point: PointId,
    on: Element,
    point_first: bool,
) -> Result<Landed, String> {
    let (picked_point, picked_on) = (
        RulePick::Element(Element::Point(point)),
        RulePick::Element(on),
    );
    let picks = match point_first {
        true => [picked_point, picked_on],
        false => [picked_on, picked_point],
    };
    let Some(RuleIntent::Constrain(rule)) = rule_intent(Rule::Coincident, &picks, sketch) else {
        return Err(format!("a point and {on:?} make no coincidence"));
    };
    if sketch.carries(rule) {
        return Ok(Landed::AlreadyThere);
    }
    let (low, high) = sketch.bounds().ok_or("a drawing with no point")?;
    let size = (high - low).length().max(1.0);
    let (near, still) = (size * 1e-4, size * 1e-7);
    let shapes = shapes(sketch);
    let its_points = points_of(sketch, on);
    let (of_point, of_trait) = (shapes[point.0], shapes[its_points[0].0]);

    let mut after = sketch.clone();
    let free_apart = of_point != of_trait
        && !anchored(sketch, &shapes, of_point)
        && !anchored(sketch, &shapes, of_trait);
    if after.lay_rule(rule, 1.0) == LengthOutcome::BestEffort {
        return match free_apart {
            true => Err("refused between two free shapes apart".into()),
            false => Ok(Landed::Refused),
        };
    }

    let moved = |id: PointId| sketch.point(id).distance(after.point(id));
    let off = off(&after, point, on);
    if off > near {
        return Err(format!("the point stands {off} off {on:?}"));
    }
    // A curve comes whole, its size kept, and a point lands on what is drawn
    // of it, whichever of the two came (#554).
    let grown = how_big(sketch, on)
        .into_iter()
        .zip(how_big(&after, on))
        .map(|(was, is)| (was - is).abs())
        .fold(0.0, f64::max);
    if grown > near {
        return Err(format!("{on:?} changed size by {grown}"));
    }
    let off_drawn = off_what_is_drawn(&after, point, on);
    if off_drawn > near {
        return Err(format!(
            "the point stands {off_drawn} off what is drawn of {on:?}"
        ));
    }
    let shape_moved = |shape: usize| {
        sketch
            .live_points()
            .filter(|(id, _)| shapes[id.0] == shape)
            .map(|(id, _)| moved(id))
            .fold(resized(sketch, &after, &shapes, shape), f64::max)
    };
    if let Some((id, _)) = sketch
        .live_points()
        .find(|(id, _)| shapes[id.0] != of_point && shapes[id.0] != of_trait && moved(*id) > still)
    {
        return Err(format!("{id:?}, in a shape apart, moved by {}", moved(id)));
    }
    if let Some((id, round)) = sketch.live_circles().find(|(_, round)| {
        let shape = shapes[round.center.0];
        shape != of_point && shape != of_trait && resized(sketch, &after, &shapes, shape) > still
    }) {
        return Err(format!(
            "{id:?}, in a shape apart, went from a radius of {} to {}",
            round.radius,
            after.circles()[id.0].radius
        ));
    }
    let from = match rule {
        Constraint::OnSegment { from, .. }
        | Constraint::OnCircle { from, .. }
        | Constraint::OnArc { from, .. }
        | Constraint::OnEllipse { from, .. } => from,
        _ => return Err(format!("a coincidence laid as {rule:?}")),
    };
    let laid_on = match on {
        Element::Segment(_) => LaidFrom::Trait,
        _ => LaidFrom::Curve,
    };
    let apart = of_point != of_trait;
    let point_side = match apart {
        true => shape_moved(of_point),
        false => moved(point),
    };
    let on_side = match (apart, on) {
        (true, _) => shape_moved(of_trait),
        (false, Element::Circle(id)) => moved(its_points[0])
            .max((sketch.circles()[id.0].radius - after.circles()[id.0].radius).abs()),
        (false, _) => its_points.iter().map(|id| moved(*id)).fold(0.0, f64::max),
    };
    let (first, second) = match from {
        LaidFrom::Point => (point_side, on_side),
        LaidFrom::Nowhere => {
            return match on_side > still {
                true => Err(format!("a free point, and {on:?} moved by {on_side}")),
                false => Ok(Landed::InOrder),
            };
        }
        laid if laid == laid_on => (on_side, point_side),
        other => return Err(format!("a coincidence on {on:?} laid from {other:?}")),
    };
    // The first one clicked stays; when the second cannot come, the second
    // stays and the first comes to it — which two free shapes apart always
    // can. Never both.
    // A point whose foot on the trait's line, or whose place on the curve, is
    // already taken by another point cannot come there without squeezing
    // what joins them to nothing.
    let foot_taken = landing(sketch, point, on).is_some_and(|foot| {
        sketch
            .live_points()
            .any(|(id, place)| id != point && place.distance(foot) <= near)
    });
    let could_not_come = !free_apart || (from == laid_on && foot_taken);
    match (first > still, second > still) {
        (false, _) => Ok(Landed::InOrder),
        (true, false) if could_not_come => Ok(Landed::Reversed),
        (true, false) => Err(format!(
            "{from:?} first, between free shapes apart, and the first came to the second"
        )),
        (true, true) => Err(format!(
            "{from:?} first, apart {apart}: both moved, the first by {first}, the second by {second}"
        )),
    }
}

/// What the rules laid on one drawing came to: every flaw, and how many
/// landed each way.
#[derive(Default)]
struct Tally {
    flaws: Vec<String>,
    reversed: usize,
    refused: usize,
    laid: usize,
}

impl Tally {
    fn add(&mut self, other: Tally) {
        self.flaws.extend(other.flaws);
        self.reversed += other.reversed;
        self.refused += other.refused;
        self.laid += other.laid;
    }
}

/// What the points of a drawing are laid on.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum On {
    Traits,
    Curves,
}

/// The traits, or the curves, of a drawing.
fn targets(sketch: &Sketch, on: On) -> Vec<Element> {
    match on {
        On::Traits => sketch
            .live_segments()
            .map(|(id, _)| Element::Segment(id))
            .collect(),
        On::Curves => sketch
            .live_circles()
            .map(|(id, _)| Element::Circle(id))
            .chain(sketch.live_arcs().map(|(id, _)| Element::Arc(id)))
            .chain(sketch.live_ellipses().map(|(id, _)| Element::Ellipse(id)))
            .collect(),
    }
}

/// The drawing as the generator drew it, and the same with one of what the
/// points are laid on fixed and a trait's length typed: the generator lays
/// neither, and a fixed trait or curve is the case where the second one
/// cannot come.
fn variants(seed: u64, on: On) -> Vec<(&'static str, Sketch)> {
    let drawn = laid(&drawn(seed));
    let traits: Vec<SegmentId> = drawn.live_segments().map(|(id, _)| id).collect();
    let targets = targets(&drawn, on);
    if traits.is_empty() || targets.is_empty() {
        return vec![("as drawn", drawn)];
    }
    let mut held = drawn.clone();
    let fixed = targets[seed as usize % targets.len()];
    let typed = traits[(seed as usize / 3 + 1) % traits.len()];
    held.add_constraint(Constraint::Fixed { element: fixed });
    held.set_dimension(
        DimensionTarget::Length(typed),
        held.segment_length(typed),
        false,
    );
    let variant = match on {
        On::Traits => "one trait fixed, one typed",
        On::Curves => "one curve fixed, a trait typed",
    };
    vec![("as drawn", drawn), (variant, held)]
}

/// Every rule laid on one drawing, as many pairs as it has up to the limit,
/// spread evenly over them.
fn tally(seed: u64, on: On) -> Tally {
    let mut tally = Tally::default();
    for (variant, sketch) in variants(seed, on) {
        // A drawing the generator left unable to settle on its own — a
        // rectangle of no width, whose sides of no length no settle will
        // take — refuses every rule laid on it, and says nothing about this
        // one.
        let collapsed = sketch
            .live_segments()
            .any(|(segment, _)| sketch.segment_length(segment) < 1e-9);
        if collapsed || sketch.clone().resolve(1.0) == LengthOutcome::BestEffort {
            continue;
        }
        let points: Vec<PointId> = sketch
            .live_points()
            .map(|(id, _)| id)
            .filter(|id| !sketch.is_origin(*id))
            .collect();
        let mut pairs = Vec::new();
        for target in targets(&sketch, on) {
            let its_own = points_of(&sketch, target);
            for point in points.iter().filter(|point| !its_own.contains(point)) {
                for point_first in [true, false] {
                    pairs.push((*point, target, point_first));
                }
            }
        }
        let step = pairs.len().div_ceil(PAIRS_PER_DRAWING).max(1);
        for (point, target, point_first) in pairs.into_iter().step_by(step) {
            tally.laid += 1;
            match landed(&sketch, point, target, point_first) {
                Ok(Landed::Reversed) => tally.reversed += 1,
                Ok(Landed::Refused) => tally.refused += 1,
                Ok(Landed::InOrder | Landed::AlreadyThere) => {}
                Err(flaw) => tally.flaws.push(format!(
                    "seed {seed}, {variant}: {point:?} and {target:?}, point first {point_first}: {flaw}"
                )),
            }
        }
    }
    tally
}

/// Every rule laid on the drawings of the seeds the gate plays.
fn the_gate_s_seeds(on: On) -> Tally {
    let mut all = Tally::default();
    for seed in SEEDS {
        all.add(tally(seed, on));
    }
    all
}

#[test]
fn a_point_and_a_trait_land_as_clicked_on_drawings_made_at_random() {
    let all = the_gate_s_seeds(On::Traits);

    assert!(
        all.flaws.is_empty(),
        "{} rules laid wrongly:\n{}",
        all.flaws.len(),
        all.flaws.join("\n")
    );
}

#[test]
fn a_point_and_a_curve_land_as_clicked_on_drawings_made_at_random() {
    let all = the_gate_s_seeds(On::Curves);

    assert!(
        all.flaws.is_empty(),
        "{} rules laid wrongly:\n{}",
        all.flaws.len(),
        all.flaws.join("\n")
    );
}

#[test]
#[ignore]
fn a_long_campaign_of_points_laid_on_traits_lands_as_clicked() {
    long_campaign(On::Traits);
}

#[test]
#[ignore]
fn a_long_campaign_of_points_laid_on_curves_lands_as_clicked() {
    long_campaign(On::Curves);
}

/// Rules laid on drawings from a seed on until the time given runs out.
fn long_campaign(on: On) {
    let seconds = std::env::var("CAO_FUZZ_SECONDS")
        .ok()
        .and_then(|text| text.parse().ok())
        .unwrap_or(60);
    let start = std::env::var("CAO_FUZZ_SEED")
        .ok()
        .and_then(|text| text.parse().ok())
        .unwrap_or_else(|| {
            SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .map_or(0, |since| since.as_secs())
        });
    let deadline = Instant::now() + Duration::from_secs(seconds);
    let mut all = Tally::default();
    let mut seed = start;
    while Instant::now() < deadline {
        all.add(tally(seed, on));
        seed += 1;
    }
    println!(
        "seeds {start}..{seed}: {} rules laid, {} the other way round, {} refused, {} wrongly",
        all.laid,
        all.reversed,
        all.refused,
        all.flaws.len()
    );
    for flaw in &all.flaws {
        println!("{flaw}");
    }
    assert!(all.flaws.is_empty());
}
