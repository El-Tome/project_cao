//! A curve seen in the parameters of a surface it lies on.

use std::f64::consts::TAU;

use glam::{DVec2, DVec3};

use crate::brep::Declined;
use crate::brep::curve::{Circle, Curve, Meet};
use crate::brep::scale::Scale;
use crate::brep::surface::{Cylinder, Surface};
use crate::brep::trace::Trace;

/// The stretch of `curve` from parameter `from` to `to`, seen in the
/// parameters of `surface`: declined where the surface would see an ellipse,
/// or a curve it cannot carry.
///
/// That the curve lies on the surface was decided before; what is read here
/// is only how. The curve two cylinders meet along is seen on the one whose
/// axis the surface's runs along — the second moved, by less than the
/// tolerance, onto a touch their pair decided, and carried so by the curve.
/// A wall parallel to that one and further than `eps` from it carries the
/// curve only because it was taken for the curve the wall meets the other
/// cylinder along (decision 6), and sees it as that curve; where no stretch
/// of that curve stands within `eps` of it, it sees the curve in its own
/// angles, not in its own wall's.
///
/// A curve a surface does not carry — a line across a cylinder's axis, a
/// circle whose centre stands further than `eps` off it or square to
/// another axis — lies on the surface only because an arc of the surface
/// was taken for it (decision 6), within the tolerance and between two
/// corners: where it stands within `eps` of the chord between them, it is
/// seen as the segment between where its ends stand. A circle square to a
/// cylinder's axis that bulges off its chord there — a rim passing a hair
/// from a ruling it crosses twice, taken for the curve its own wall meets
/// that cylinder along — is seen as that curve, traced the way the curve's
/// own arc is. Anything else is declined, as a circle leaning on a plane is.
pub(in crate::brep) fn traced(
    curve: &Curve,
    surface: &Surface,
    from: f64,
    to: f64,
    eps: f64,
) -> Result<Trace, Declined> {
    match (surface, curve) {
        (Surface::Plane(plane), Curve::Line(line)) => Ok(Trace::Segment {
            from: plane.parameters(line.point(from)),
            to: plane.parameters(line.point(to)),
        }),
        (Surface::Plane(plane), Curve::Circle(circle)) if parallel(circle.axis, plane.normal) => {
            let turn = circle.axis.dot(plane.normal).signum();
            let start = circle.u.dot(plane.v).atan2(circle.u.dot(plane.u));
            Ok(Trace::Round {
                center: plane.parameters(circle.center),
                radius: circle.radius,
                start: start + turn * from,
                sweep: turn * (to - from),
            })
        }
        (Surface::Cylinder(cylinder), Curve::Line(line))
            if parallel(line.direction, cylinder.axis) =>
        {
            let start = cylinder.parameters(line.point(from));
            let height = (line.point(to) - cylinder.origin).dot(cylinder.axis);
            Ok(Trace::Segment {
                from: start,
                to: DVec2::new(start.x, height),
            })
        }
        (Surface::Cylinder(cylinder), Curve::Circle(circle))
            if parallel(circle.axis, cylinder.axis) =>
        {
            let turn = circle.axis.dot(cylinder.axis).signum();
            let height = (circle.center - cylinder.origin).dot(cylinder.axis);
            let [start, end] = if off_axis(circle, cylinder) > eps {
                let start = cylinder.parameters(circle.point(from)).x;
                let end = cylinder.parameters(circle.point(to)).x;
                [start, unwrapped(end, start + turn * (to - from))]
            } else {
                let start = circle.u.dot(cylinder.v).atan2(circle.u.dot(cylinder.u));
                [start + turn * from, start + turn * to]
            };
            Ok(Trace::Segment {
                from: DVec2::new(start, height),
                to: DVec2::new(end, height),
            })
        }
        (Surface::Cylinder(cylinder), Curve::Meet(meet))
            if parallel(cylinder.axis, meet.first.axis)
                || parallel(cylinder.axis, meet.second.axis) =>
        {
            let on_first = parallel(cylinder.axis, meet.first.axis);
            let other = if on_first { meet.second } else { meet.first };
            let beside = meet.own(cylinder, eps).is_none();
            let graph = Trace::Graph {
                meet: *meet,
                on_first,
                beside: beside.then_some(*cylinder),
                from,
                to,
            };
            Ok(if beside {
                met(curve, other, cylinder, [from, to], eps).unwrap_or(graph)
            } else {
                graph
            })
        }
        (Surface::Cylinder(cylinder), Curve::Circle(circle))
            if square(circle.axis, cylinder.axis) =>
        {
            let own = Cylinder::about(circle.center, circle.axis, circle.radius);
            chord(curve, surface, from, to, eps)
                .or_else(|declined| met(curve, own, cylinder, [from, to], eps).ok_or(declined))
        }
        (Surface::Plane(_) | Surface::Cylinder(_), _) => chord(curve, surface, from, to, eps),
    }
}

/// A stretch of a curve lying on `own`, a cylinder square to `cylinder`,
/// seen on `cylinder` as the curve the two meet along: the component and the
/// stretch of it the curve's ends and middle stand on, when the middle
/// stands within `eps` of it.
fn met(
    curve: &Curve,
    own: Cylinder,
    cylinder: &Cylinder,
    [from, to]: [f64; 2],
    eps: f64,
) -> Option<Trace> {
    let middle = curve.point((from + to) / 2.0);
    let off = |meet: &Meet, at: f64| meet.point(at).distance(middle);
    let (meet, period, at) = (0..2)
        .filter_map(|component| {
            let meet = Meet {
                first: own,
                second: *cylinder,
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

/// The segment between the parameters of a stretch's ends, when the stretch
/// stands within `eps` of its chord at its middle.
fn chord(
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
            Surface::Cylinder(_) => DVec2::new(unwrapped(to.x, from.x), to.y),
            Surface::Plane(_) => to,
        },
    })
}

/// How far a circle's centre stands off a cylinder's axis.
fn off_axis(circle: &Circle, cylinder: &Cylinder) -> f64 {
    let from = circle.center - cylinder.origin;
    (from - cylinder.axis * from.dot(cylinder.axis)).length()
}

/// An angle moved by whole turns to stand within half a turn of `near`, and
/// left as it is when it already does.
fn unwrapped(angle: f64, near: f64) -> f64 {
    angle + TAU * ((near - angle) / TAU).round()
}

/// Parallel either way, closely enough that over the reach of a body the
/// two directions part by less than its tolerance.
fn parallel(one: DVec3, other: DVec3) -> bool {
    one.cross(other).length() <= Scale::RELATIVE
}

/// Square to each other, closely enough that over the reach of a body the
/// two directions stand off square by less than its tolerance.
fn square(one: DVec3, other: DVec3) -> bool {
    one.dot(other).abs() <= Scale::RELATIVE
}
