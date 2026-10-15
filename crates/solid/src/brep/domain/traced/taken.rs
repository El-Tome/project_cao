//! A stretch of a curve a surface does not carry, seen as the curve it was
//! taken for (decision 6): the curve the surface meets the curve's own wall
//! along, the conic it meets the curve's own plane or wall along, or the
//! chord between the stretch's corners.

use glam::DVec2;

use super::unwrapped;
use crate::brep::Declined;
use crate::brep::conic::{Conic, On, Round};
use crate::brep::curve::{Curve, Meet, Wall};
use crate::brep::scale::Scale;
use crate::brep::surface::{Plane, Surface};
use crate::brep::trace::Trace;

/// A stretch of a curve lying on `own`, a wall square to `other`, seen on
/// `other` as the curve the two meet along: the component and the stretch
/// of it the curve's ends and middle stand on, when the middle stands
/// within `eps` of it.
pub(super) fn met(
    curve: &Curve,
    own: Wall,
    other: Wall,
    [from, to]: [f64; 2],
    eps: f64,
) -> Option<Trace> {
    let middle = curve.point((from + to) / 2.0);
    let off = |meet: &Meet, at: f64| meet.point(at).distance(middle);
    let (meet, period, at) = (0..2)
        .filter_map(|component| {
            let meet = Meet {
                first: own,
                second: other,
                component,
            };
            Some((meet, meet.period()?, meet.parameter(middle)))
        })
        .min_by(|(one, _, at), (other, _, then)| off(one, *at).total_cmp(&off(other, *then)))?;
    if off(&meet, at) > eps {
        return None;
    }
    let [start, end] = [from, to].map(|end| {
        let t = meet.parameter(curve.point(end));
        t + period * ((at - t) / period).round()
    });
    ((start - at) * (end - at) < 0.0).then_some(Trace::Graph {
        meet,
        on_first: false,
        beside: None,
        from: start,
        to: end,
    })
}

/// A stretch of a curve lying on a plane and a wall, `own`, seen on a
/// surface a hair beside one of them as the conic that surface meets the
/// other along: on a plane, the conic it cuts the wall along; on a wall,
/// the conic the plane cuts it along. The component and the stretch of it
/// the curve's ends and middle stand on, when the middle stands within
/// `eps` of it; read round the turns a stretch of a closed curve runs, a
/// whole loop of it a whole loop of the conic.
pub(super) fn beside(
    curve: &Curve,
    (plane, round): (&Plane, &Round),
    surface: &Surface,
    [from, to]: [f64; 2],
    eps: f64,
) -> Option<Trace> {
    let scale = Scale::of(eps / Scale::RELATIVE);
    let (conics, on) = match surface {
        Surface::Plane(beside) => (Conic::cut(beside, round, scale), On::Plane),
        Surface::Cylinder(wall) => (Conic::cut(plane, &Round::Cylinder(*wall), scale), On::Wall),
        Surface::Cone(wall) => (Conic::cut(plane, &Round::Cone(*wall), scale), On::Wall),
        Surface::Elliptic(wall) => (Conic::cut(plane, &Round::Elliptic(*wall), scale), On::Wall),
        Surface::Sphere(_) | Surface::Torus(_) => return None,
    };
    let middle = curve.point((from + to) / 2.0);
    let off = |conic: &Conic, at: f64| {
        if conic.holds(middle) {
            conic.point(at).distance(middle)
        } else {
            f64::INFINITY
        }
    };
    let (conic, at) = conics
        .into_iter()
        .map(|conic| (conic, conic.parameter(middle)))
        .min_by(|(one, at), (other, then)| off(one, *at).total_cmp(&off(other, *then)))?;
    if off(&conic, at) > eps {
        return None;
    }
    let [start, end] = [from, to].map(|end| conic.parameter(curve.point(end)));
    let [start, end] = match (conic.period(), curve.period()) {
        (Some(period), Some(turn)) => {
            let way = curve
                .derivative((from + to) / 2.0)
                .dot(conic.derivative(at))
                .signum();
            let run = way * (to - from) / turn * period;
            let start = start + period * ((at - run / 2.0 - start) / period).round();
            [start, end + period * ((start + run - end) / period).round()]
        }
        (Some(period), None) => {
            [start, end].map(|end| end + period * ((at - end) / period).round())
        }
        (None, _) => [start, end],
    };
    ((start - at) * (end - at) < 0.0).then_some(Trace::Conic {
        conic,
        on,
        from: start,
        to: end,
    })
}

/// The segment between the parameters of a stretch's ends, when the stretch
/// stands within `eps` of its chord at its middle.
pub(super) fn chord(
    curve: &Curve,
    surface: &Surface,
    from: f64,
    to: f64,
    eps: f64,
) -> Result<Trace, Declined> {
    let [start, end] = [from, to].map(|at| curve.point(at));
    let middle = curve.point((from + to) / 2.0) - start;
    let along = end - start;
    let share = if along.length_squared() == 0.0 {
        0.0
    } else {
        (middle.dot(along) / along.length_squared()).clamp(0.0, 1.0)
    };
    if (middle - along * share).length() > eps {
        return Err(Declined::Unsupported);
    }
    let [from, to] = [start, end].map(|point| surface.parameters(point));
    Ok(Trace::Segment {
        from,
        to: match surface {
            Surface::Cylinder(_)
            | Surface::Cone(_)
            | Surface::Elliptic(_)
            | Surface::Sphere(_)
            | Surface::Torus(_) => DVec2::new(unwrapped(to.x, from.x), to.y),
            Surface::Plane(_) => to,
        },
    })
}
