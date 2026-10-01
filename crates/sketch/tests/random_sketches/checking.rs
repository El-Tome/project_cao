//! A drawing played gesture by gesture, and held to every rule after each one.

use cao_sketch::{PointId, Sketch, WorkPlane};
use glam::DVec2;

use super::Gesture;
use super::areas::areas_of;
use super::enclosing::{Curve, Enclosure, curves_of};
use super::random::Random;
use super::rules::{
    Area, Flaw, Place, Relaying, laid_alike, nothing_extra, nothing_missing, reach, tint_is_measure,
};

/// Whether a drawing keeps every rule after every one of its gestures.
pub fn check(gestures: &[Gesture]) -> Result<(), Flaw> {
    played(gestures).map_err(|(_, flaw)| flaw)
}

/// The rule a drawing broke, and how many of its gestures had been laid when
/// it broke it.
pub fn played(gestures: &[Gesture]) -> Result<(), (usize, Flaw)> {
    let mut sketch = Sketch::new(WorkPlane::XY);
    for (index, gesture) in gestures.iter().enumerate() {
        gesture.lay(&mut sketch);
        keeps_its_areas(&sketch).map_err(|flaw| (index + 1, flaw))?;
    }
    Ok(())
}

/// Plays a drawing and ends the test on the first rule it breaks, saying
/// after which gesture.
pub fn holds(gestures: &[Gesture]) {
    if let Err((after, flaw)) = played(gestures) {
        panic!(
            "{:?} broke after gesture {after} of {}: {flaw:?}",
            flaw.rule(),
            gestures.len()
        );
    }
}

/// Whether the areas a sketch has now keep every rule.
pub fn keeps_its_areas(sketch: &Sketch) -> Result<(), Flaw> {
    let areas = areas_of(sketch);
    tint_is_measure(&areas)?;
    let curves = curves_of(sketch);
    let enclosure = Enclosure::of(&curves);
    let looked_at = places_to_look_at(&curves, &areas);
    let places: Vec<Place> = looked_at
        .iter()
        .filter_map(|at| {
            Some(Place {
                at: *at,
                enclosed: enclosure.encloses(*at)?,
            })
        })
        .collect();
    nothing_missing(&areas, &places)?;
    nothing_extra(&areas, &places)?;
    for how in relayings(sketch, &curves).into_iter().filter(|how| {
        *how == Relaying::Reordered || !enclosure.may_read_otherwise(|at| carried(*how, at))
    }) {
        let again = areas_of(&relaid(sketch, how));
        laid_alike(how, &areas, &again, &looked_at, |at| carried(how, at))?;
    }
    Ok(())
}

/// How far from a curve a place has to stand to be looked at, as a share of
/// how far the drawing reaches: well clear of the rounding a vertex is welded
/// with, and of the band where the reckoning and the tint could each see the
/// place on another side.
const CLEAR: f64 = 1e-5;

/// How far a curve sampled into steps of a forty-eighth of a turn strays
/// from its steps, as a share of how far it bends: `1 − cos(π / 48)`.
const SAGGING: f64 = 0.002_141_1;

/// How many places a drawing is looked at, each way, across its box.
const ACROSS: usize = 16;

/// How many corners of tint a drawing is looked at, at most: one per
/// triangle, at its middle, so that every area is looked at where it claims
/// to be.
const TINTED: usize = 400;

/// The places the rules are asked about: a grid across the drawing, the
/// middle of every triangle of tint, and a few places either side of every
/// curve — none of them nearer a curve than the tint could stray from it.
fn places_to_look_at(curves: &[Curve], areas: &[Area]) -> Vec<DVec2> {
    let mut seen: Vec<DVec2> = curves
        .iter()
        .flat_map(|curve| (0..=16).map(move |step| curve.at(step as f64 / 16.0)))
        .collect();
    seen.extend(areas.iter().flat_map(|area| area.outline.iter().copied()));
    let Some((low, high)) = bounds(&seen) else {
        return Vec::new();
    };
    let reach = reach(&seen);
    let margin = (high - low) * 0.1 + DVec2::ONE;
    let (low, high) = (low - margin, high + margin);

    let mut random = Random::seeded(500);
    let mut places = Vec::new();
    for row in 0..ACROSS {
        for column in 0..ACROSS {
            let cell =
                DVec2::new(column as f64, row as f64) + DVec2::new(random.unit(), random.unit());
            places.push(low + (high - low) * cell / ACROSS as f64);
        }
    }
    let triangles: Vec<&[DVec2; 3]> = areas.iter().flat_map(|area| &area.triangles).collect();
    let every = triangles.len().div_ceil(TINTED).max(1);
    places.extend(
        triangles
            .iter()
            .step_by(every)
            .map(|[a, b, c]| (*a + *b + *c) / 3.0),
    );
    for curve in curves {
        for along in [0.37, 0.71] {
            let at = curve.at(along);
            let across = normal(curve, along) * 3.0 * band(curve, reach);
            places.extend([at + across, at - across]);
        }
    }
    places.retain(|at| {
        curves
            .iter()
            .all(|curve| curve.distance(*at) > band(curve, reach))
    });
    places
}

fn band(curve: &Curve, reach: f64) -> f64 {
    CLEAR * (1.0 + reach) + 2.0 * SAGGING * curve.bend()
}

fn normal(curve: &Curve, along: f64) -> DVec2 {
    let step = 1e-4;
    (curve.at(along + step) - curve.at(along - step))
        .perp()
        .normalize_or_zero()
}

fn bounds(places: &[DVec2]) -> Option<(DVec2, DVec2)> {
    let first = *places.first()?;
    Some(places.iter().fold((first, first), |(low, high), at| {
        (low.min(*at), high.max(*at))
    }))
}

/// The ways a drawing is laid again to be held against itself: in another
/// order, moved by a step of its own size, turned a quarter turn, and turned
/// by an angle that lands nothing on the lattice.
///
/// A sketch laid again has an origin of its own, which a move leaves where
/// the drawing had nothing: the step is chosen so that it lands clear of
/// every curve and every point, or the drawing laid again would hold a point
/// the first one did not.
fn relayings(sketch: &Sketch, curves: &[Curve]) -> [Relaying; 4] {
    let seen: Vec<DVec2> = sketch.live_points().map(|(_, at)| at).collect();
    let reach = reach(&seen);
    let unit = (reach / 10.0).max(1.0).log2().round().exp2();
    let clear = |by: DVec2| {
        let origin = -by;
        curves
            .iter()
            .all(|curve| curve.distance(origin) > band(curve, reach))
            && seen
                .iter()
                .all(|at| at.distance(origin) > CLEAR * (1.0 + reach))
    };
    let by = [
        (3.0, -2.0),
        (-5.0, 3.0),
        (7.0, 5.0),
        (-2.0, -7.0),
        (0.5, 0.25),
    ]
    .map(|(x, y)| DVec2::new(x, y) * unit)
    .into_iter()
    .find(|by| clear(*by))
    .unwrap_or(DVec2::new(0.25, 0.125) * unit);
    [
        Relaying::Reordered,
        Relaying::Moved(by),
        Relaying::Turned(90.0),
        Relaying::Turned(30.0),
    ]
}

fn ordered<T>(mut items: Vec<T>, reverse: bool) -> Vec<T> {
    if reverse {
        items.reverse();
    }
    items
}

/// Where a place of the drawing lands once it is laid again.
pub fn carried(how: Relaying, at: DVec2) -> DVec2 {
    match how {
        Relaying::Reordered => at,
        Relaying::Moved(by) => at + by,
        Relaying::Turned(90.0) => DVec2::new(-at.y, at.x),
        Relaying::Turned(degrees) => DVec2::from_angle(degrees.to_radians()).rotate(at),
    }
}

/// The same points and curves laid on a fresh sketch, carried as `how` says,
/// and in the reverse of the order they were drawn in when it says so — each
/// trait then drawn from its other end. Rules and values are left behind: no
/// area reads them.
pub fn relaid(sketch: &Sketch, how: Relaying) -> Sketch {
    let reverse = how == Relaying::Reordered;
    let mut again = Sketch::new(WorkPlane::XY);
    let mut points: Vec<Option<PointId>> = vec![None; sketch.points().len()];
    let mut live: Vec<(PointId, DVec2)> = sketch.live_points().collect();
    if reverse {
        live.reverse();
    }
    for (id, at) in live {
        let to = carried(how, at);
        points[id.0] = Some(match sketch.is_origin(id) && to == at {
            true => id,
            false => again.add_point(to),
        });
    }
    let mut point = |again: &mut Sketch, id: PointId| match points[id.0] {
        Some(laid) => laid,
        None => {
            let laid = again.add_point(carried(how, sketch.point(id)));
            points[id.0] = Some(laid);
            laid
        }
    };
    for (id, segment) in ordered(sketch.live_segments().collect(), reverse) {
        if sketch.ellipse_of_axis(id).is_some() {
            continue;
        }
        let (start, end) = (
            point(&mut again, segment.start),
            point(&mut again, segment.end),
        );
        let (start, end) = if reverse { (end, start) } else { (start, end) };
        match segment.construction {
            true => again.add_construction_segment(start, end),
            false => again.add_segment(start, end),
        };
    }
    for (_, circle) in ordered(sketch.live_circles().collect(), reverse) {
        let centre = point(&mut again, circle.center);
        match circle.construction {
            true => again.add_construction_circle(centre, circle.radius),
            false => again.add_circle(centre, circle.radius),
        };
    }
    for (_, arc) in ordered(sketch.live_arcs().collect(), reverse) {
        let [centre, start, end] = [arc.center, arc.start, arc.end].map(|id| point(&mut again, id));
        match arc.construction {
            true => again.add_construction_arc(centre, start, end),
            false => again.add_arc(centre, start, end),
        };
    }
    for (id, ellipse) in ordered(sketch.live_ellipses().collect(), reverse) {
        let ends = |axis: cao_sketch::SegmentId| {
            let segment = sketch.segments()[axis.0];
            [segment.start, segment.end]
        };
        let centre = point(&mut again, ellipse.center);
        let first = ends(ellipse.first).map(|end| point(&mut again, end));
        let second = ends(ellipse.second).map(|end| point(&mut again, end));
        let laid = match ellipse.construction {
            true => again.add_construction_ellipse(centre, first, second),
            false => again.add_ellipse(centre, first, second),
        };
        if let Some((from, to)) = sketch.ellipse_ends(id) {
            let (from, to) = (point(&mut again, from), point(&mut again, to));
            again.draw_the_stretch(laid, from, to);
        }
    }
    again
}
