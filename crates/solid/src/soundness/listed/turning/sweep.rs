//! What an edge sweeps in its surface's parameters: the area, and on a
//! surface about an axis the angle it turns round it.

use std::f64::consts::TAU;

use glam::DVec3;

use super::CHORDS;
use crate::brep::{Circle, Curve, ListedEdge, Surface};
use crate::soundness::listed::lying::point;
use crate::soundness::listed::nappe::Nappe;
use crate::soundness::listed::oval;
use crate::soundness::listed::round::Tube;

/// The area an edge run along its own way sweeps in a surface's parameters,
/// and on a cylinder or a cone the angle it turns round the axis.
///
/// On a plane the area is half the moment of the edge about `about`, a
/// place of the face, along the plane's normal: exact for a line and for a
/// circle. On a cylinder it is what the edge sweeps under it, down to the
/// height of `about`, as the angle turns: nothing for a ruling, which turns
/// no angle. On a cone it is the same in `(θ, l)`, `l` along the ruling:
/// nothing for a ruling, the length from that of `about` times the angle for
/// a cross-section.
pub(super) fn sweep(surface: &Surface, edge: &ListedEdge, about: DVec3, room: f64) -> (f64, f64) {
    match (surface, edge.curve) {
        (Surface::Plane(plane), Curve::Line(_)) => {
            let [from, to] = [edge.from, edge.to].map(|at| point(&edge.curve, at) - about);
            (plane.normal.dot(from.cross(to)) / 2.0, 0.0)
        }
        (Surface::Plane(plane), Curve::Circle(circle)) => {
            let chord = point(&edge.curve, edge.to) - point(&edge.curve, edge.from);
            let moment = plane.normal.dot((circle.center - about).cross(chord));
            let turned = circle.radius
                * circle.radius
                * (edge.to - edge.from)
                * plane.normal.dot(circle.u.cross(circle.v));
            ((moment + turned) / 2.0, 0.0)
        }
        (Surface::Plane(plane), _) => chords(edge, |place| {
            let from = place - about;
            (from.dot(plane.u), from.dot(plane.v))
        }),
        (
            Surface::Cylinder(_)
            | Surface::Cone(_)
            | Surface::Elliptic(_)
            | Surface::Sphere(_)
            | Surface::Torus(_),
            Curve::Line(_),
        ) => (0.0, 0.0),
        (Surface::Sphere(sphere), _) => tube(&Tube::of_sphere(sphere), edge, about, room),
        (Surface::Torus(torus), _) => tube(&Tube::of_torus(torus), edge, about, room),
        (Surface::Cylinder(cylinder), Curve::Circle(circle))
            if section(cylinder.origin, cylinder.axis, &circle, room) =>
        {
            let axis = cylinder.axis.normalize();
            let height = (circle.center - about).dot(axis);
            let angle = (edge.to - edge.from) * axis.dot(circle.u.cross(circle.v)).signum();
            (-height * angle, angle)
        }
        (Surface::Cylinder(cylinder), _) => {
            let axis = cylinder.axis.normalize();
            around(edge, |place| {
                let from = place - cylinder.origin;
                (
                    (place - about).dot(axis),
                    from.dot(cylinder.v).atan2(from.dot(cylinder.u)),
                )
            })
        }
        (Surface::Cone(cone), Curve::Circle(circle))
            if section(cone.origin, cone.axis, &circle, room) =>
        {
            let nappe = Nappe::of(cone);
            let length = nappe.length(point(&edge.curve, edge.from)) - nappe.length(about);
            let angle = (edge.to - edge.from)
                * cone.axis.normalize().dot(circle.u.cross(circle.v)).signum();
            (-length * angle, angle)
        }
        (Surface::Cone(cone), _) => {
            let nappe = Nappe::of(cone);
            let from = nappe.length(about);
            around(edge, |place| {
                (nappe.length(place) - from, nappe.angle(place))
            })
        }
        (Surface::Elliptic(wall), _) => {
            let axis = wall.axis.normalize();
            let [u, v] = [wall.u, wall.v].map(DVec3::normalize);
            around(edge, |place| {
                let from = place - wall.origin;
                (
                    (place - about).dot(axis),
                    oval::angle([wall.major, wall.minor], [from.dot(u), from.dot(v)]),
                )
            })
        }
    }
}

/// What an edge sweeps on a sphere or a torus, in `(θ, φ)` with `φ`
/// weighed by the area form from that of `about`: a latitude circle in
/// closed form, a meridian circle nothing, since it turns no angle about
/// the axis — never read along chords, which at a pole read an angle that
/// is rounding — and anything else along chords.
fn tube(tube: &Tube, edge: &ListedEdge, about: DVec3, room: f64) -> (f64, f64) {
    let from = tube.phi(about);
    let height = |place: DVec3| tube.rise(from, tube.phi(place));
    if let Curve::Circle(circle) = edge.curve {
        if tube.holds_meridian(&circle, room) {
            return (0.0, 0.0);
        }
        let (origin, axis) = tube.frame();
        if section(origin, axis, &circle, room) {
            let angle = (edge.to - edge.from) * axis.dot(circle.u.cross(circle.v)).signum();
            let height = height(point(&edge.curve, edge.from));
            return (-height * angle, angle);
        }
    }
    around(edge, |place| (height(place), tube.angle(place)))
}

/// Whether a circle is a cross-section of a surface about the axis through
/// `origin` along `axis`, its angle the surface's own: square to the axis,
/// about a place of it within `room`. A circle about another axis lies on
/// the surface only over a stretch a hair long, an arc of the surface's
/// taken for it — the rim of a wall all but touching this one, whose angle
/// may run the other way round.
fn section(origin: DVec3, axis: DVec3, circle: &Circle, room: f64) -> bool {
    let axis = axis.normalize();
    let from = circle.center - origin;
    axis.cross(circle.axis).length() <= SQUARE && (from - axis * from.dot(axis)).length() <= room
}

/// How far from square to the axis, in the sine of the angle between the
/// two axes, a circle may stand and still be a cross-section.
const SQUARE: f64 = 1e-9;

/// What a curve sweeps on a plane, along chords in the plane's parameters,
/// each with the sliver between it and the curve, read through the place
/// half way along it as a parabola's is: four thirds of the triangle the
/// chord makes with that place. The chords alone fall short by the
/// slivers, as much as a whole crescent a hair wide between an arc of an
/// ellipse and one of a circle touching it at both ends sweeps, which then
/// read as turning the wrong way.
fn chords(edge: &ListedEdge, flat: impl Fn(DVec3) -> (f64, f64)) -> (f64, f64) {
    let places: Vec<(f64, f64)> = (0..=2 * CHORDS)
        .map(|half| {
            let at = edge.from + (edge.to - edge.from) * half as f64 / (2 * CHORDS) as f64;
            flat(point(&edge.curve, at))
        })
        .collect();
    let cross = |one: (f64, f64), other: (f64, f64)| one.0 * other.1 - other.0 * one.1;
    let area = places
        .windows(3)
        .step_by(2)
        .map(|piece| {
            let (start, middle, end) = (piece[0], piece[1], piece[2]);
            let along = (end.0 - start.0, end.1 - start.1);
            let bulge = (middle.0 - start.0, middle.1 - start.1);
            cross(start, end) / 2.0 + 2.0 / 3.0 * cross(bulge, along)
        })
        .sum();
    (area, 0.0)
}

/// What a curve sweeps under a surface about an axis, along chords in the
/// surface's parameters as `chart` reads them at each place — the height
/// from that of `about`, then the angle — the angle followed across the
/// turn rather than read afresh.
fn around(edge: &ListedEdge, chart: impl Fn(DVec3) -> (f64, f64)) -> (f64, f64) {
    let mut last: Option<(f64, f64)> = None;
    let (mut area, mut angle) = (0.0, 0.0);
    for piece in 0..=CHORDS {
        let at = edge.from + (edge.to - edge.from) * piece as f64 / CHORDS as f64;
        let (height, theta) = chart(point(&edge.curve, at));
        if let Some((before_height, before)) = last {
            let step = (theta - before + TAU / 2.0).rem_euclid(TAU) - TAU / 2.0;
            area -= (height + before_height) / 2.0 * step;
            angle += step;
        }
        last = Some((height, theta));
    }
    (area, angle)
}
