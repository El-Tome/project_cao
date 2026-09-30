//! Decision 3: the same curve comes out of several pairs of surfaces and out
//! of both operands' edges; curves within the tolerance over the box are one,
//! and the surfaces each was found on are joined into its support.

use glam::DVec3;

use super::apart::Apart;
use crate::brep::curve::{Circle, Curve, Line, Meet};
use crate::brep::scale::Scale;
use crate::brep::surface::Cylinder;
use crate::brep::topology::SurfaceId;

/// A curve and the surfaces it was decided to lie on, sorted.
#[derive(Clone, Debug, PartialEq)]
pub(in crate::brep) struct Registered {
    pub curve: Curve,
    pub support: Vec<SurfaceId>,
}

pub(in crate::brep) struct Registry {
    pub list: Vec<Registered>,
    pub apart: Apart,
    scale: Scale,
}

impl Registry {
    pub fn new(scale: Scale, apart: Apart) -> Registry {
        Registry {
            list: Vec::new(),
            apart,
            scale,
        }
    }

    /// The rank of the curve, the first registered within the tolerance of
    /// it and lying on no surface apart from `support`, or a new one; either
    /// way lying on `support` from now on.
    pub fn register(&mut self, curve: Curve, support: &[SurfaceId]) -> usize {
        let found = self.list.iter().position(|known| {
            same(&known.curve, &curve, self.scale) && !self.apart.across(&known.support, support)
        });
        let rank = found.unwrap_or_else(|| {
            self.list.push(Registered {
                curve,
                support: Vec::new(),
            });
            self.list.len() - 1
        });
        for surface in support {
            self.join(rank, *surface);
        }
        rank
    }

    /// Whether a curve may lie on `surface` too: it lies on no surface
    /// decided apart from it.
    pub fn admits(&self, curve: usize, surface: SurfaceId) -> bool {
        !self.apart.across(&self.list[curve].support, &[surface])
    }

    pub fn join(&mut self, curve: usize, surface: SurfaceId) {
        let support = &mut self.list[curve].support;
        if let Err(place) = support.binary_search(&surface) {
            support.insert(place, surface);
        }
    }

    /// The stretch of a registered curve that a stretch of `own`, one with it
    /// within the tolerance, covers: from `from` to `to` on `own`, read in the
    /// registered curve's parameter and running up it.
    ///
    /// On the curve two cylinders meet along, an end may be a node the curve
    /// passes twice: the stretch is read from its middle, which is never one.
    pub fn stretch(&self, curve: usize, own: &Curve, from: f64, to: f64) -> [f64; 2] {
        let registered = &self.list[curve].curve;
        if registered == own {
            return [from, to];
        }
        let [start, end] = [from, to].map(|at| registered.parameter(own.point(at)));
        match (registered, own) {
            (Curve::Circle(one), Curve::Circle(other)) => {
                if one.axis.dot(other.axis) > 0.0 {
                    [start, start + (to - from)]
                } else {
                    [end, end + (to - from)]
                }
            }
            (Curve::Meet(_), _) => {
                let half = (to - from) / 2.0;
                let middle = registered.parameter(own.point(from + half));
                [middle - half, middle + half]
            }
            _ => [start.min(end), start.max(end)],
        }
    }
}

/// Whether two curves are one within the tolerance over the whole box.
pub(in crate::brep) fn same(one: &Curve, other: &Curve, scale: Scale) -> bool {
    let eps = scale.eps();
    match (one, other) {
        (Curve::Line(one), Curve::Line(other)) => {
            parallel(one.direction, other.direction, scale)
                && line_distance(one, other.origin) <= eps
        }
        (Curve::Circle(one), Curve::Circle(other)) => {
            parallel(one.axis, other.axis, scale)
                && one.center.distance(other.center) <= eps
                && (one.radius - other.radius).abs() <= eps
        }
        (Curve::Meet(one), Curve::Meet(other)) => {
            one.component == other.component
                && alike(&one.first, &other.first, scale)
                && alike(&one.second, &other.second, scale)
        }
        _ => false,
    }
}

/// How far a point stands from a curve.
pub(in crate::brep) fn distance(curve: &Curve, point: DVec3) -> f64 {
    match curve {
        Curve::Line(line) => line_distance(line, point),
        Curve::Circle(circle) => circle_distance(circle, point),
        Curve::Meet(meet) => meet_distance(meet, point),
    }
}

fn line_distance(line: &Line, point: DVec3) -> f64 {
    let from = point - line.origin;
    (from - line.direction * from.dot(line.direction)).length()
}

fn circle_distance(circle: &Circle, point: DVec3) -> f64 {
    let from = point - circle.center;
    let along = from.dot(circle.axis);
    let across = (from - circle.axis * along).length() - circle.radius;
    along.hypot(across)
}

fn meet_distance(meet: &Meet, point: DVec3) -> f64 {
    meet.point(meet.parameter(point)).distance(point)
}

fn alike(one: &Cylinder, other: &Cylinder, scale: Scale) -> bool {
    parallel(one.axis, other.axis, scale)
        && one.origin.distance(other.origin) <= scale.eps()
        && (one.radius - other.radius).abs() <= scale.eps()
}

/// Two directions part by less than the tolerance across the whole box.
fn parallel(one: DVec3, other: DVec3, scale: Scale) -> bool {
    one.cross(other).length() * 2.0 * scale.reach() <= scale.eps()
}
