//! A curve seen in the parameters of a surface it lies on.

use std::f64::consts::TAU;

use glam::{DVec2, DVec3};

use crate::brep::Declined;
use crate::brep::curve::{Circle, Curve};
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
///
/// A curve a surface does not carry — a line across a cylinder's axis, a
/// circle whose centre stands further than `eps` off it or square to
/// another axis — lies on the surface only because an arc of the surface
/// was taken for it (decision 6), within the tolerance and between two
/// corners: where it stands within `eps` of the chord between them, it is
/// seen as the segment between where its ends stand, and it is declined
/// otherwise, as a circle leaning on a plane is.
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
            Ok(Trace::Graph {
                meet: *meet,
                on_first: parallel(cylinder.axis, meet.first.axis),
                from,
                to,
            })
        }
        _ => chord(curve, surface, from, to, eps),
    }
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
