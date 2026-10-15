//! A curve seen in the parameters of a surface it lies on.

mod square;
mod taken;

use std::f64::consts::TAU;

use glam::{DVec2, DVec3};

use crate::brep::Declined;
use crate::brep::conic::{Conic, On, Round};
use crate::brep::curve::{Circle, Curve, Wall};
use crate::brep::scale::Scale;
use crate::brep::surface::{Cylinder, Plane, Surface};
use crate::brep::trace::Trace;
use taken::{beside, chord, met};

/// The stretch of `curve` from parameter `from` to `to`, seen in the
/// parameters of `surface`: declined where the surface would see a curve it
/// cannot carry.
///
/// That the curve lies on the surface was decided before; what is read here
/// is only how. The curve two cylinders meet along is seen on the one whose
/// axis the surface's runs along — the second moved, by less than the
/// tolerance, onto a touch their pair decided, and carried so by the curve.
/// A wall parallel to that one and further than `eps` from it carries the
/// curve only because it was taken for the curve the wall meets the other
/// cylinder along (decision 6), and sees it as that curve; where no stretch
/// of that curve stands within `eps` of it, it sees the curve in its own
/// angles, not in its own wall's. A conic is seen on its own plane and its
/// own wall as the graph it is there (#552). The curve two walls of one
/// radius whose axes meet square meet along lies on a plane at 45° through
/// both axes, and is seen there as the conic that plane cuts its first
/// wall along (5552120815).
///
/// A curve a surface does not carry — a line across a cylinder's axis, a
/// circle whose centre stands further than `eps` off it or square to
/// another axis, a circle leaning on a plane, a conic on a surface not its
/// own — lies on the surface only because an arc of the surface was taken
/// for it (decision 6), within the tolerance and between two corners. A
/// conic, and a circle leaning on a plane by less than the tolerance across
/// its rim, are seen as the conic the surface meets the curve's other own
/// surface along, the circle's own wall a cylinder: its stretch the curve's
/// ends stand on, where its middle stands within `eps` of it. Where a
/// stretch stands within `eps` of the chord between its corners, it is
/// seen as the segment between where its ends stand — first, for a circle,
/// as it was before conics were built. A circle square to a cylinder's axis
/// that bulges off its chord there — a rim passing a hair from a ruling it
/// crosses twice, taken for the curve its own wall meets that cylinder
/// along — is seen as that curve, traced the way the curve's own arc is.
/// Anything else is declined.
pub(in crate::brep) fn traced(
    curve: &Curve,
    surface: &Surface,
    from: f64,
    to: f64,
    eps: f64,
) -> Result<Trace, Declined> {
    match (surface, curve) {
        (Surface::Sphere(_) | Surface::Torus(_), _) => Err(Declined::Unfinished),
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
        (Surface::Plane(_), Curve::Circle(circle)) => {
            let (own, _) = Plane::through(circle.center, circle.axis);
            let round = Round::Cylinder(Cylinder::about(circle.center, circle.axis, circle.radius));
            chord(curve, surface, from, to, eps).or_else(|declined| {
                beside(curve, (&own, &round), surface, [from, to], eps).ok_or(declined)
            })
        }
        (Surface::Plane(plane), Curve::Conic(conic)) if *plane == conic.plane => {
            Ok(own(conic, On::Plane, from, to))
        }
        (Surface::Cylinder(cylinder), Curve::Conic(conic))
            if conic.round == Round::Cylinder(*cylinder) =>
        {
            Ok(own(conic, On::Wall, from, to))
        }
        (Surface::Cone(cone), Curve::Conic(conic)) if conic.round == Round::Cone(*cone) => {
            Ok(own(conic, On::Wall, from, to))
        }
        (Surface::Elliptic(wall), Curve::Conic(conic)) if conic.round == Round::Elliptic(*wall) => {
            Ok(own(conic, On::Wall, from, to))
        }
        (_, Curve::Conic(conic)) => {
            match beside(
                curve,
                (&conic.plane, &conic.round),
                surface,
                [from, to],
                eps,
            ) {
                Some(trace) => Ok(trace),
                None => chord(curve, surface, from, to, eps).or_else(|declined| {
                    square::rim(curve, conic, surface, [from, to], eps).ok_or(declined)
                }),
            }
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
        (Surface::Elliptic(wall), Curve::Line(line)) if parallel(line.direction, wall.axis) => {
            let start = wall.parameters(line.point(from));
            let height = (line.point(to) - wall.origin).dot(wall.axis);
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
            let [start, end] = if off_axis(circle, cylinder.origin, cylinder.axis) > eps {
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
            if parallel(cylinder.axis, meet.first.axis())
                || parallel(cylinder.axis, meet.second.axis()) =>
        {
            let on_first = parallel(cylinder.axis, meet.first.axis());
            let (own, other) = if on_first {
                (meet.first, meet.second)
            } else {
                (meet.second, meet.first)
            };
            let beside = meet.own(&Wall::Cylinder(*cylinder), eps).is_none();
            if beside && matches!(own, Wall::Elliptic(_)) {
                return chord(curve, surface, from, to, eps);
            }
            let graph = Trace::Graph {
                meet: *meet,
                on_first,
                beside: beside.then_some(*cylinder),
                from,
                to,
            };
            Ok(if beside {
                met(curve, other, Wall::Cylinder(*cylinder), [from, to], eps).unwrap_or(graph)
            } else {
                graph
            })
        }
        (Surface::Cylinder(cylinder), Curve::Circle(circle))
            if square(circle.axis, cylinder.axis) =>
        {
            let own = Wall::Cylinder(Cylinder::about(circle.center, circle.axis, circle.radius));
            let other = Wall::Cylinder(*cylinder);
            chord(curve, surface, from, to, eps)
                .or_else(|declined| met(curve, own, other, [from, to], eps).ok_or(declined))
        }
        (Surface::Elliptic(oval), Curve::Circle(circle)) if square(circle.axis, oval.axis) => {
            chord(curve, surface, from, to, eps).or_else(|declined| {
                square::circle(curve, circle, oval, [from, to], eps).ok_or(declined)
            })
        }
        (Surface::Cone(cone), Curve::Line(line))
            if cone.rules(line, Scale::of(eps / Scale::RELATIVE)) =>
        {
            let angle = cone.ruling_angle(line, from, to);
            let [from, to] =
                [from, to].map(|at| DVec2::new(angle, cone.parameters(line.point(at)).y));
            Ok(Trace::Segment { from, to })
        }
        (Surface::Cone(cone), Curve::Circle(circle))
            if parallel(circle.axis, cone.axis)
                && off_axis(circle, cone.origin, cone.axis) <= eps =>
        {
            let turn = circle.axis.dot(cone.axis).signum();
            let start = circle.u.dot(cone.v).atan2(circle.u.dot(cone.u));
            let height = (circle.center - cone.origin).dot(cone.axis);
            let length = (DVec2::new(height, circle.radius) - cone.foot).dot(cone.ruling);
            Ok(Trace::Segment {
                from: DVec2::new(start + turn * from, length),
                to: DVec2::new(start + turn * to, length),
            })
        }
        (Surface::Elliptic(oval), Curve::Meet(meet))
            if let Some(on_first) = meet.own(&Wall::Elliptic(*oval), eps) =>
        {
            Ok(Trace::Graph {
                meet: *meet,
                on_first,
                beside: None,
                from,
                to,
            })
        }
        (Surface::Plane(plane), Curve::Meet(meet)) => {
            let round = square::round(meet, plane);
            chord(curve, surface, from, to, eps).or_else(|declined| {
                beside(curve, (plane, &round), surface, [from, to], eps).ok_or(declined)
            })
        }
        (Surface::Cylinder(_) | Surface::Cone(_) | Surface::Elliptic(_), _) => {
            chord(curve, surface, from, to, eps)
        }
    }
}

fn own(conic: &Conic, on: On, from: f64, to: f64) -> Trace {
    Trace::Conic {
        conic: *conic,
        on,
        from,
        to,
    }
}

/// How far a circle's centre stands off the axis through `origin`.
fn off_axis(circle: &Circle, origin: DVec3, axis: DVec3) -> f64 {
    let from = circle.center - origin;
    (from - axis * from.dot(axis)).length()
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
