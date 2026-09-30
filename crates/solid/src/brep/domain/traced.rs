//! A curve seen in the parameters of a surface it lies on.

use glam::{DVec2, DVec3};

use crate::brep::Declined;
use crate::brep::curve::Curve;
use crate::brep::scale::Scale;
use crate::brep::surface::Surface;
use crate::brep::trace::Trace;

/// The stretch of `curve` from parameter `from` to `to`, seen in the
/// parameters of `surface`: declined where the surface would see an ellipse,
/// or a curve it cannot carry.
pub(in crate::brep) fn traced(
    curve: &Curve,
    surface: &Surface,
    from: f64,
    to: f64,
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
            let start = circle.u.dot(cylinder.v).atan2(circle.u.dot(cylinder.u));
            let height = (circle.center - cylinder.origin).dot(cylinder.axis);
            Ok(Trace::Segment {
                from: DVec2::new(start + turn * from, height),
                to: DVec2::new(start + turn * to, height),
            })
        }
        (Surface::Cylinder(cylinder), Curve::Meet(meet))
            if *cylinder == meet.first || *cylinder == meet.second =>
        {
            Ok(Trace::Graph {
                meet: *meet,
                on_first: *cylinder == meet.first,
                from,
                to,
            })
        }
        _ => Err(Declined::Unsupported),
    }
}

/// Parallel either way, closely enough that over the reach of a body the
/// two directions part by less than its tolerance.
fn parallel(one: DVec3, other: DVec3) -> bool {
    one.cross(other).length() <= Scale::RELATIVE
}
