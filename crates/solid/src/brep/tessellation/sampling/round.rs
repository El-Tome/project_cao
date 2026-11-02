//! The grid of a sphere or a torus (#576): one grid of angles about the axis
//! for the whole surface, cut for the widest circle any face on it reaches,
//! which every circle lying on it is sampled on as a cone's rims are; and a
//! grid of rows along the meridian, on which every meridian arc is sampled —
//! the opening and the closing meridian of a turn on the same rows, whatever
//! circle each lies on — and between which the faces are cut in strips.
//!
//! A doubly curved surface sags both ways: a chord of a latitude circle sags
//! towards the axis, a chord of a meridian towards the tube's centre, and a
//! triangle between two rows and two angles stands off the surface by the
//! sum of both. Each grid therefore takes less than half the tolerance, so
//! that the two together stay within it.

use std::f64::consts::{PI, TAU};

use glam::{DVec2, DVec3};

use crate::brep::curve::Circle;
use crate::brep::domain::floor::{self, Pole};
use crate::brep::surface::{Sphere, Surface, Torus};
use crate::brep::topology::{Body, Edge, Face, SurfaceId};

/// The share of the tolerance each of a round's two grids is cut for: under
/// a half, since the sags add up along the hypotenuse of a triangle of the
/// lattice, and a little more than their sum — a thousandth of the tolerance
/// over the radius — comes of the two curvatures together.
pub(super) const SHARE: f64 = 0.45;

/// How close to a row, in radians along the meridian, a sample must stand to
/// be read as on it: far over the rounding of a point read back from the
/// surface, far under any two rows' gap.
pub(in crate::brep::tessellation) const ON_A_ROW: f64 = 1e-9;

/// A sphere or a torus, read in the chart `(θ, φ)` they share.
#[derive(Clone, Copy, Debug, PartialEq)]
pub(in crate::brep::tessellation) enum Rounded {
    Sphere(Sphere),
    Torus(Torus),
}

impl Rounded {
    pub fn of(surface: &Surface) -> Option<Rounded> {
        match surface {
            Surface::Sphere(sphere) => Some(Rounded::Sphere(*sphere)),
            Surface::Torus(torus) => Some(Rounded::Torus(*torus)),
            Surface::Plane(_) | Surface::Cylinder(_) | Surface::Cone(_) | Surface::Elliptic(_) => {
                None
            }
        }
    }

    pub fn surface(&self) -> Surface {
        match self {
            Rounded::Sphere(sphere) => Surface::Sphere(*sphere),
            Rounded::Torus(torus) => Surface::Torus(*torus),
        }
    }

    pub fn origin(&self) -> DVec3 {
        match self {
            Rounded::Sphere(sphere) => sphere.origin,
            Rounded::Torus(torus) => torus.origin,
        }
    }

    pub fn axis(&self) -> DVec3 {
        match self {
            Rounded::Sphere(sphere) => sphere.axis,
            Rounded::Torus(torus) => torus.axis,
        }
    }

    pub fn u(&self) -> DVec3 {
        match self {
            Rounded::Sphere(sphere) => sphere.u,
            Rounded::Torus(torus) => torus.u,
        }
    }

    pub fn v(&self) -> DVec3 {
        match self {
            Rounded::Sphere(sphere) => sphere.v,
            Rounded::Torus(torus) => torus.v,
        }
    }

    pub fn radial(&self, theta: f64) -> DVec3 {
        match self {
            Rounded::Sphere(sphere) => sphere.radial(theta),
            Rounded::Torus(torus) => torus.radial(theta),
        }
    }

    /// The meridian circle's centre `(h_c, R)` and radius `r`.
    fn meridian_circle(&self) -> (DVec2, f64) {
        match self {
            Rounded::Sphere(sphere) => (DVec2::new(sphere.center, 0.0), sphere.radius),
            Rounded::Torus(torus) => (torus.center, torus.radius),
        }
    }

    /// The tube's radius, the meridian circle's.
    pub fn radius(&self) -> f64 {
        self.meridian_circle().1
    }

    /// The furthest from the axis the surface stands: at the crest, `R + r`.
    pub fn widest(&self) -> f64 {
        let (center, radius) = self.meridian_circle();
        center.y + radius
    }

    pub fn point(&self, at: DVec2) -> DVec3 {
        self.surface().point(at)
    }

    /// `(θ, φ)`, `θ` in `(-π, π]` and `φ` in the chart's window.
    pub fn parameters(&self, point: DVec3) -> DVec2 {
        self.surface().parameters(point)
    }

    /// An angle along the meridian moved by whole turns into the window.
    pub fn windowed(&self, phi: f64) -> f64 {
        match self {
            Rounded::Sphere(_) => phi,
            Rounded::Torus(torus) => torus.windowed(phi),
        }
    }

    /// The angle a torus's chart is cut at the antipode of; none for a
    /// sphere, whose chart ends at its poles.
    pub fn seam(&self) -> Option<f64> {
        floor::seam(&self.surface())
    }

    /// The angle along the meridian a torus's chart is cut at, where a face
    /// winding round the tube is closed.
    pub fn cut(&self) -> Option<f64> {
        self.seam().map(|seam| seam + PI)
    }

    /// Whether `circle` is a latitude circle of the surface within `eps`.
    pub fn holds(&self, circle: &Circle, eps: f64) -> bool {
        match self {
            Rounded::Sphere(sphere) => sphere.holds(circle, eps),
            Rounded::Torus(torus) => torus.holds(circle, eps),
        }
    }

    /// Whether `circle` is a meridian circle of the surface within `eps`.
    pub fn meridian(&self, circle: &Circle, eps: f64) -> bool {
        match self {
            Rounded::Sphere(sphere) => sphere.meridian(circle, eps),
            Rounded::Torus(torus) => torus.meridian(circle, eps),
        }
    }

    /// The poles of the surface, the low one first, with the room along the
    /// meridian `eps` leaves at the tube's radius.
    pub fn poles(&self, eps: f64) -> Vec<Pole> {
        floor::poles_of(&self.surface(), eps)
    }

    /// The angle along the meridian a latitude `circle` of the surface stands
    /// at, in the window: one value for the whole circle, and nought with
    /// its sign fixed, so that the equator is one level of the chart.
    pub fn latitude(&self, circle: &Circle) -> f64 {
        let (center, _) = self.meridian_circle();
        let h = (circle.center - self.origin()).dot(self.axis());
        self.windowed((h - center.x).atan2(circle.radius - center.y)) + 0.0
    }

    /// The angle about the axis the arc of the meridian `circle` holding its
    /// point at `at` stands at — a great circle of a sphere stands at two,
    /// half a turn apart — and the circle's own parameter at the crest, `ψ`,
    /// with the way its parameter runs with the angle along the meridian,
    /// `s`: `t = ψ + s·φ`.
    pub fn along_meridian(&self, circle: &Circle, at: f64) -> (f64, f64, f64) {
        let axis = self.axis();
        let from = circle.point(at) - self.origin();
        let radial = (from - axis * axis.dot(from)).normalize();
        let theta = radial.dot(self.v()).atan2(radial.dot(self.u()));
        let psi = radial.dot(circle.v).atan2(radial.dot(circle.u));
        let sign = radial.cross(axis).dot(circle.axis).signum();
        (theta, psi, sign)
    }
}

/// A parameter of a meridian arc's edge whose point stands off the axis: the
/// middle of the arc, or the start of a whole circle.
pub(in crate::brep::tessellation) fn midway(edge: &Edge) -> f64 {
    if edge.ends.is_none() {
        edge.from
    } else {
        (edge.from + edge.to) / 2.0
    }
}

/// The angles along the meridian that bound the faces of `face`'s surface
/// where `face` lies, which no row of the grid may stand within the room of:
/// its latitude circles, the poles of the surface and, on a torus, the cut
/// of the chart. A meridian arc and the face it bounds read the same list,
/// so that the arc's samples stand exactly where the face's rows are.
pub(in crate::brep::tessellation) fn levels(
    body: &Body,
    face: &Face,
    round: &Rounded,
    eps: f64,
) -> Vec<f64> {
    let mut levels: Vec<f64> = round.poles(eps).iter().map(|pole| pole.at).collect();
    levels.extend(round.cut());
    for coedge in face.loops.iter().flatten() {
        let edge = body.edge(coedge.edge);
        if let crate::brep::curve::Curve::Circle(circle) = body.curve(edge.curve)
            && round.holds(circle, eps)
        {
            levels.push(round.latitude(circle));
        }
    }
    levels
}

/// The angles along the meridian bounding every face of `surface` using the
/// edge `edge`.
pub(super) fn levels_along(
    body: &Body,
    surface: SurfaceId,
    edge: &Edge,
    round: &Rounded,
    eps: f64,
) -> Vec<f64> {
    body.face_ids()
        .map(|id| body.face(id))
        .filter(|face| {
            face.surface == surface
                && face
                    .loops
                    .iter()
                    .flatten()
                    .any(|coedge| std::ptr::eq(body.edge(coedge.edge), edge))
        })
        .flat_map(|face| levels(body, face, round, eps))
        .collect()
}

/// The rows of a grid of `steps` rows a turn strictly between `low` and
/// `high` along the meridian, none within `room` of an angle of `levels` a
/// whole number of turns on, ascending.
pub(in crate::brep::tessellation) fn rows(
    steps: usize,
    low: f64,
    high: f64,
    levels: &[f64],
    room: f64,
) -> Vec<f64> {
    let step = TAU / steps as f64;
    ((low / step).floor() as i64..=(high / step).ceil() as i64)
        .map(|rank| TAU * rank as f64 / steps as f64)
        .filter(|&row| row > low + room && row < high - room)
        .filter(|&row| !levels.iter().any(|level| near(row, *level, room)))
        .collect()
}

/// Whether `row` stands within `room` of `level`, a whole number of turns
/// on.
fn near(row: f64, level: f64, room: f64) -> bool {
    ((row - level + PI).rem_euclid(TAU) - PI).abs() <= room
}

/// The points of a meridian arc's edge between its ends, in the way the edge
/// runs: at every row of the round's grid of `steps` rows a turn strictly
/// inside the arc, none within the round's room of an end or of a level of
/// the faces it bounds. A whole meridian circle — a ring turned part of the
/// way — takes every row of the window and the cut of the chart besides,
/// where the face winding round the tube is closed.
pub(super) fn on_meridian(
    circle: &Circle,
    edge: &Edge,
    round: &Rounded,
    steps: usize,
    levels: &[f64],
    eps: f64,
) -> Vec<DVec3> {
    let (_, psi, sign) = round.along_meridian(circle, midway(edge));
    let phi_of = |t: f64| sign * (t - psi);
    let room = eps / round.radius();
    let mut taken: Vec<f64> = if edge.ends.is_none() {
        let mut levels = levels.to_vec();
        levels.extend(round.cut());
        let mut taken: Vec<f64> = (0..steps)
            .map(|rank| TAU * rank as f64 / steps as f64)
            .filter(|row| !levels.iter().any(|level| near(*row, *level, room)))
            .collect();
        taken.extend(round.cut());
        taken
    } else {
        let (one, other) = (phi_of(edge.from), phi_of(edge.to));
        let mut levels = levels.to_vec();
        levels.extend([one, other]);
        rows(steps, one.min(other), one.max(other), &levels, room)
    };
    let low = edge.from.min(edge.to);
    let mut places: Vec<f64> = taken
        .drain(..)
        .map(|phi| {
            let t = psi + sign * phi;
            low + (t - low).rem_euclid(TAU)
        })
        .collect();
    places.sort_by(f64::total_cmp);
    if edge.to < edge.from {
        places.reverse();
    }
    places.into_iter().map(|t| circle.point(t)).collect()
}
