//! Decision 3: the same curve comes out of several pairs of surfaces and out
//! of both operands' edges; curves within the tolerance over the box are one,
//! and the surfaces each was found on are joined into its support.

use glam::{DVec2, DVec3};

use super::apart::Apart;
use super::planes::Planes;
use crate::brep::conic::{Arm, Conic, Round};
use crate::brep::curve::{Circle, Curve, Line, Meet};
use crate::brep::relation::apart_by;
use crate::brep::scale::Scale;
use crate::brep::surface::{Cone, Cylinder};
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
    pub planes: Planes,
    scale: Scale,
}

impl Registry {
    pub fn new(scale: Scale, apart: Apart, planes: Planes) -> Registry {
        Registry {
            list: Vec::new(),
            apart,
            planes,
            scale,
        }
    }

    /// The rank of the curve, the first registered within the tolerance of
    /// it — or a line on two planes across each other `support` lies on —
    /// and lying on no surface apart from `support`, nor, a line, meeting one
    /// of it at a cone's apex alone, or a new one; either way lying on
    /// `support` from now on.
    pub fn register(&mut self, curve: Curve, support: &[SurfaceId]) -> usize {
        self.register_beside(curve, support, |_| false)
    }

    /// As `register`, but never onto a curve `other` says the curve was
    /// already decided apart from.
    pub fn register_beside(
        &mut self,
        curve: Curve,
        support: &[SurfaceId],
        other: impl Fn(usize) -> bool,
    ) -> usize {
        let found = self.list.iter().enumerate().position(|(rank, known)| {
            let fixed = matches!((&known.curve, &curve), (Curve::Line(_), Curve::Line(_)))
                && self.planes.fix_a_line(&known.support, support);
            (fixed || same(&known.curve, &curve, self.scale))
                && !self.apart.across(&known.support, support)
                && !ruled_apart(&self.apart, &curve, &known.support, support)
                && !other(rank)
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
    /// decided apart from it, nor, a line, decided to meet it at a cone's
    /// apex alone.
    pub fn admits(&self, curve: usize, surface: SurfaceId) -> bool {
        let Registered { curve, support } = &self.list[curve];
        !self.apart.across(support, &[surface])
            && !ruled_apart(&self.apart, curve, support, &[surface])
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
    /// So is a stretch of a whole ellipse, whose ends may stand either side
    /// of where its parameter wraps round.
    pub fn stretch(&self, curve: usize, own: &Curve, from: f64, to: f64) -> [f64; 2] {
        let registered = &self.list[curve].curve;
        if registered == own {
            return [from, to];
        }
        let [start, end] = [from, to].map(|at| registered.parameter(own.point(at)));
        let from_the_middle = || {
            let half = (to - from) / 2.0;
            let middle = registered.parameter(own.point(from + half));
            [middle - half, middle + half]
        };
        match (registered, own) {
            (Curve::Circle(one), Curve::Circle(other)) => {
                if one.axis.dot(other.axis) > 0.0 {
                    [start, start + (to - from)]
                } else {
                    [end, end + (to - from)]
                }
            }
            (Curve::Meet(_), _) => from_the_middle(),
            (Curve::Conic(conic), _) if conic.period().is_some() => from_the_middle(),
            _ => [start.min(end), start.max(end)],
        }
    }
}

/// Whether `curve`, a line, lies on a surface of `one` and one of `other`
/// decided to meet at a cone's apex alone: a ruling of one, read as a whole
/// line, may be one of the other's, two cones of one apex each the other's
/// mirror, its two halves one on each nappe. A circle lies on a cone's own
/// nappe or not at all: within the tolerance of both, it is on both — two
/// cones of one apex a hair apart in slope, which stand within the
/// tolerance of each other all along their faces (5365211513).
fn ruled_apart(apart: &Apart, curve: &Curve, one: &[SurfaceId], other: &[SurfaceId]) -> bool {
    matches!(curve, Curve::Line(_)) && apart.pointed(one, other)
}

/// Whether two curves are one within the tolerance over the whole box.
pub(in crate::brep) fn same(one: &Curve, other: &Curve, scale: Scale) -> bool {
    within(one, other, scale, scale.eps())
}

/// Whether two curves are one within `eps` over the whole box.
pub(in crate::brep) fn within(one: &Curve, other: &Curve, scale: Scale, eps: f64) -> bool {
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
                && alike(&one.first, &other.first, scale, eps)
                && alike(&one.second, &other.second, scale, eps)
        }
        (Curve::Conic(one), Curve::Conic(other)) => conics_within(one, other, scale, eps),
        (Curve::Meet(meet), Curve::Conic(conic)) | (Curve::Conic(conic), Curve::Meet(meet)) => {
            meet_within_conic(meet, conic, eps)
        }
        _ => false,
    }
}

/// How many places along the curve two cylinders meet along are read to
/// tell whether it lies in a conic's plane.
const PLACES: usize = 16;

/// A component of the curve two walls meet along is a conic when that conic
/// is cut whole on one of the two walls and the component lies in its plane
/// all round: a plane cuts a wall along one curve. Two walls of one radius
/// whose axes meet square meet along two ellipses, each in a plane at 45°.
fn meet_within_conic(meet: &Meet, conic: &Conic, eps: f64) -> bool {
    let (Round::Cylinder(wall), Arm::Whole, Some(period)) = (conic.round, conic.arm, meet.period())
    else {
        return false;
    };
    meet.own(&wall, eps).is_some()
        && (0..PLACES).all(|place| {
            let at = meet.point(period * place as f64 / PLACES as f64);
            (at - conic.plane.origin).dot(conic.plane.normal).abs() <= eps
        })
}

/// Two conics are one when their planes are one, their walls are one, and
/// they are the same stretch of the cut: the whole of it, or the arm on the
/// same side of the plane of symmetry — a side read off the plane's normal
/// and the wall's axis, which two surfaces one within the tolerance may turn
/// either way.
fn conics_within(one: &Conic, other: &Conic, scale: Scale, eps: f64) -> bool {
    let facing = |mine: DVec3, theirs: DVec3| if mine.dot(theirs) > 0.0 { 1 } else { -1 };
    let (mine, theirs) = (&one.plane, &other.plane);
    let normals = facing(mine.normal, theirs.normal);
    let level = parallel(mine.normal, theirs.normal, scale)
        && (mine.offset() - f64::from(normals) * theirs.offset()).abs() <= eps;
    let (round, axes) = match (one.round, other.round) {
        (Round::Cylinder(mine), Round::Cylinder(theirs)) => (
            alike(&mine, &theirs, scale, eps),
            facing(mine.axis, theirs.axis),
        ),
        (Round::Cone(mine), Round::Cone(theirs)) => (
            cones_alike(&mine, &theirs, scale, eps),
            facing(mine.axis, theirs.axis),
        ),
        (Round::Elliptic(mine), Round::Elliptic(theirs)) => (
            parallel(mine.axis, theirs.axis, scale) && mine.within(&theirs, eps),
            facing(mine.axis, theirs.axis),
        ),
        (Round::Cylinder(_), Round::Cone(_) | Round::Elliptic(_))
        | (Round::Cone(_), Round::Cylinder(_) | Round::Elliptic(_))
        | (Round::Elliptic(_), Round::Cylinder(_) | Round::Cone(_)) => {
            return false;
        }
    };
    let arm = match (one.arm, other.arm) {
        (Arm::Whole, Arm::Whole) => true,
        (Arm::Split { side: mine }, Arm::Split { side: theirs }) => mine * normals * axes == theirs,
        (Arm::Whole, Arm::Split { .. }) | (Arm::Split { .. }, Arm::Whole) => false,
    };
    level && round && arm
}

/// Two cones of one axis whose meridian lines are one across the box: as
/// near each other as `apart_by` reads, and of one slope.
fn cones_alike(one: &Cone, other: &Cone, scale: Scale, eps: f64) -> bool {
    let turned = one.axis.dot(other.axis).signum();
    let theirs = DVec2::new(turned * other.ruling.x, other.ruling.y);
    parallel(one.axis, other.axis, scale)
        && one.ruling.perp_dot(theirs).abs() * 2.0 * scale.reach() <= eps
        && apart_by(one, other) <= eps
}

/// How far a point stands from a curve.
pub(in crate::brep) fn distance(curve: &Curve, point: DVec3) -> f64 {
    match curve {
        Curve::Line(line) => line_distance(line, point),
        Curve::Circle(circle) => circle_distance(circle, point),
        Curve::Meet(meet) => meet_distance(meet, point),
        Curve::Conic(conic) => conic.distance(point),
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

fn alike(one: &Cylinder, other: &Cylinder, scale: Scale, eps: f64) -> bool {
    parallel(one.axis, other.axis, scale)
        && one.origin.distance(other.origin) <= eps
        && (one.radius - other.radius).abs() <= eps
}

/// Two directions part by less than the tolerance across the whole box.
fn parallel(one: DVec3, other: DVec3, scale: Scale) -> bool {
    one.cross(other).length() * 2.0 * scale.reach() <= scale.eps()
}
