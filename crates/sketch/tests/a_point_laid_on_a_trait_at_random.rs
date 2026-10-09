//! « Coïncidence » between a point and a trait, laid on drawings made at
//! random: every point against every trait, both ways round (#548).
//!
//! Closes #548.
//! - a campaign of random drawings, with points placed on them and some of
//!   them fixed, holds after each rule that the first thing clicked has not
//!   moved — or, when the second could not come, the second — that nothing
//!   moved but the two and what they belong to, that the point stands on the
//!   trait's line, or that the rule was refused —
//!   `a_point_and_a_trait_land_as_clicked_on_drawings_made_at_random`
//!
//! The gate plays a fixed range of seeds. A longer campaign is run by hand:
//!
//! ```text
//! CAO_FUZZ_SECONDS=300 cargo test --release -p cao_sketch \
//!     --test a_point_laid_on_a_trait_at_random -- --ignored --nocapture
//! ```
//!
//! `CAO_FUZZ_SEED` starts it from a given seed rather than from the clock.
//!
//! Each drawing is played as drawn, and again with one trait fixed and
//! another one's length typed: the generator lays neither.
//!
//! The first one clicked stays, or — when the second could not come — the
//! second stays and the first comes to it. Could not come is read as: the two
//! are one shape, or one of them is anchored (a value, a fixed thing, a point
//! held on an axis, the origin), or the point's foot on the trait's line is
//! already another point's. Between two free shapes apart, anything else is a
//! flaw.
//!
//! Seeds 400 to 5199, some 340 000 rules laid, still found three drawings.
//! Two of them, 2792 and 3874, held a point on half an ellipse that was taken
//! back onto the stretch drawn whenever the drawing settled — #531, which put
//! them right, and points held on a cut ellipse are weighed like the rest
//! since. 2783, a single rule, is not understood yet. Run again from 400 to
//! 7955 once #531 had landed, some 530 000 rules, the campaign found 2783 and
//! one more, 7430: the same flaw, the trait clicked first coming to the
//! point, and found on `main` as well.
//!
//! Two kinds of drawing the generator makes are left out, since no rule can
//! land on them: one holding a trait of no length — a rectangle of no width —
//! and one that does not settle on its own.

// The drawing is the campaign's; this file only lays a rule on it.
#[allow(dead_code, unused_imports)]
mod random_sketches;

use std::time::{Duration, Instant, SystemTime};

use cao_sketch::{
    Constraint, DimensionTarget, Element, LaidFrom, LengthOutcome, PointId, Rule, RuleIntent,
    RulePick, SegmentId, Sketch, rule_intent,
};
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
        DimensionTarget::Angle { first, second }
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
            Constraint::OnCircle { point, circle } => {
                vec![Element::Point(point), Element::Circle(circle)]
            }
            Constraint::OnArc { point, arc } => vec![Element::Point(point), Element::Arc(arc)],
            Constraint::OnEllipse { point, ellipse } => {
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

/// How laying « Coïncidence » on one point and one trait, in this order,
/// went — or what went wrong.
fn landed(
    sketch: &Sketch,
    point: PointId,
    segment: SegmentId,
    point_first: bool,
) -> Result<Landed, String> {
    let (picked_point, picked_trait) = (
        RulePick::Element(Element::Point(point)),
        RulePick::Element(Element::Segment(segment)),
    );
    let picks = match point_first {
        true => [picked_point, picked_trait],
        false => [picked_trait, picked_point],
    };
    let Some(RuleIntent::Constrain(rule)) = rule_intent(Rule::Coincident, &picks, sketch) else {
        return Err("a point and a trait make no coincidence".into());
    };
    if sketch.carries(rule) {
        return Ok(Landed::AlreadyThere);
    }
    let (low, high) = sketch.bounds().ok_or("a drawing with no point")?;
    let size = (high - low).length().max(1.0);
    let (near, still) = (size * 1e-4, size * 1e-7);
    let shapes = shapes(sketch);
    let side = sketch.segments()[segment.0];
    let (of_point, of_trait) = (shapes[point.0], shapes[side.start.0]);

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
    let off = after
        .point_to_segment(point, segment)
        .unwrap_or(f64::INFINITY);
    if off > near {
        return Err(format!("the point stands {off} off the trait"));
    }
    let shape_moved = |shape: usize| {
        sketch
            .live_points()
            .filter(|(id, _)| shapes[id.0] == shape)
            .map(|(id, _)| moved(id))
            .fold(0.0, f64::max)
    };
    if let Some((id, _)) = sketch
        .live_points()
        .find(|(id, _)| shapes[id.0] != of_point && shapes[id.0] != of_trait && moved(*id) > still)
    {
        return Err(format!("{id:?}, in a shape apart, moved by {}", moved(id)));
    }
    let Constraint::OnSegment { from, .. } = rule else {
        return Err(format!("a coincidence laid as {rule:?}"));
    };
    let apart = of_point != of_trait;
    let point_side = match apart {
        true => shape_moved(of_point),
        false => moved(point),
    };
    let trait_side = match apart {
        true => shape_moved(of_trait),
        false => moved(side.start).max(moved(side.end)),
    };
    let (first, second) = match from {
        LaidFrom::Point => (point_side, trait_side),
        LaidFrom::Trait => (trait_side, point_side),
        LaidFrom::Nowhere => {
            return match trait_side > still {
                true => Err(format!("a free point, and the trait moved by {trait_side}")),
                false => Ok(Landed::InOrder),
            };
        }
        LaidFrom::Curve => return Err("a coincidence laid from a curve".into()),
    };
    // The first one clicked stays; when the second cannot come, the second
    // stays and the first comes to it — which two free shapes apart always
    // can. Never both.
    // A point whose foot on the trait's line is already taken by another
    // point cannot come there without squeezing what joins them to nothing.
    let foot_taken = sketch.foot_on_segment(point, segment).is_some_and(|foot| {
        sketch
            .live_points()
            .any(|(id, place)| id != point && place.distance(foot) <= near)
    });
    let could_not_come = !free_apart || (from == LaidFrom::Trait && foot_taken);
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

/// The drawing as the generator drew it, and the same with one trait fixed
/// and another one's length typed: the generator lays neither, and a fixed
/// trait is the case where the second one cannot come.
fn variants(seed: u64) -> Vec<(&'static str, Sketch)> {
    let drawn = laid(&drawn(seed));
    let traits: Vec<SegmentId> = drawn.live_segments().map(|(id, _)| id).collect();
    if traits.is_empty() {
        return vec![("as drawn", drawn)];
    }
    let mut held = drawn.clone();
    let fixed = traits[seed as usize % traits.len()];
    let typed = traits[(seed as usize / 3 + 1) % traits.len()];
    held.add_constraint(Constraint::Fixed {
        element: Element::Segment(fixed),
    });
    held.set_dimension(
        DimensionTarget::Length(typed),
        held.segment_length(typed),
        false,
    );
    vec![("as drawn", drawn), ("one trait fixed, one typed", held)]
}

/// Every rule laid on one drawing, as many pairs as it has up to the limit,
/// spread evenly over them.
fn tally(seed: u64) -> Tally {
    let mut tally = Tally::default();
    for (variant, sketch) in variants(seed) {
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
        for (segment, side) in sketch.live_segments() {
            for point in points
                .iter()
                .filter(|point| **point != side.start && **point != side.end)
            {
                for point_first in [true, false] {
                    pairs.push((*point, segment, point_first));
                }
            }
        }
        let step = pairs.len().div_ceil(PAIRS_PER_DRAWING).max(1);
        for (point, segment, point_first) in pairs.into_iter().step_by(step) {
            tally.laid += 1;
            match landed(&sketch, point, segment, point_first) {
                Ok(Landed::Reversed) => tally.reversed += 1,
                Ok(Landed::Refused) => tally.refused += 1,
                Ok(Landed::InOrder | Landed::AlreadyThere) => {}
                Err(flaw) => tally.flaws.push(format!(
                    "seed {seed}, {variant}: {point:?} and {segment:?}, point first {point_first}: {flaw}"
                )),
            }
        }
    }
    tally
}

#[test]
fn a_point_and_a_trait_land_as_clicked_on_drawings_made_at_random() {
    let mut all = Tally::default();
    for seed in SEEDS {
        all.add(tally(seed));
    }

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
        all.add(tally(seed));
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
