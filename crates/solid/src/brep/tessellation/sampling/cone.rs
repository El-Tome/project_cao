//! The grid of a cone: one for the whole surface, read at its widest, so that
//! every circle lying on it is sampled at the same angles from the `u` it
//! shares with every cylinder of its axis. A sphere's and a torus's grid is
//! read the same way ([`round`]).
//!
//! A circle's own grid is cut for its own radius, and a cone's rims stand at
//! two: sampled each on its own, a narrow rim and a wide one would take
//! angles that never meet, and a triangle between them would span a step of
//! the wide one's from a single sample of the narrow one. On the cone's grid
//! as well, every rim has a sample at every one of its angles, and the strip
//! between two rims at two consecutive angles is a flat trapezoid whose sag is
//! the chord's at the widest radius. Each circle keeps its own grid besides,
//! which the other face along it — a bore's wall, a disc — is cut on; where
//! the cone stands close to another wall, every rim takes the others' too
//! ([`axis`]). A conic cut on the cone counts towards its widest too: an
//! ellipse a plane cuts at a slant has no vertex to read it off, and stands
//! further out than a narrow rim, whose strips to it would otherwise span a
//! step of the rim's grid at the ellipse's radius. A circle lying on no cone
//! is sampled as before, bit for bit.
//!
//! A round's grid is cut for the widest its faces reach, which no circle or
//! vertex need stand at: a zone across the equator, a ring, reach the crest
//! between their rims. It takes less than half the tolerance, and so does
//! the grid of rows along the meridian the round carries besides, since a
//! triangle of the lattice sags both ways.
//!
//! [`axis`]: super::axis
//! [`round`]: super::round

use std::collections::BTreeMap;
use std::f64::consts::TAU;

use glam::DVec3;

use super::round::{Rounded, SHARE};
use super::{MOST, divisions};
use crate::brep::conic::{Conic, Round};
use crate::brep::curve::{Circle, Curve};
use crate::brep::surface::{Cone, Cylinder, Surface};
use crate::brep::topology::{Body, Edge, Face, SurfaceId};
use crate::brep::trace::Trace;

/// A surface of revolution carrying a grid of its own: a cone, a sphere or a
/// torus.
#[derive(Clone, Copy, Debug, PartialEq)]
pub(super) enum Spun {
    Cone(Cone),
    Round(Rounded),
}

impl Spun {
    pub(super) fn origin(&self) -> DVec3 {
        match self {
            Spun::Cone(cone) => cone.origin,
            Spun::Round(round) => round.origin(),
        }
    }

    pub(super) fn axis(&self) -> DVec3 {
        match self {
            Spun::Cone(cone) => cone.axis,
            Spun::Round(round) => round.axis(),
        }
    }

    pub(super) fn u(&self) -> DVec3 {
        match self {
            Spun::Cone(cone) => cone.u,
            Spun::Round(round) => round.u(),
        }
    }

    pub(super) fn v(&self) -> DVec3 {
        match self {
            Spun::Cone(cone) => cone.v,
            Spun::Round(round) => round.v(),
        }
    }

    pub(super) fn radial(&self, theta: f64) -> DVec3 {
        match self {
            Spun::Cone(cone) => cone.radial(theta),
            Spun::Round(round) => round.radial(theta),
        }
    }

    /// Whether `circle` lies on the surface, about its axis, within `eps`.
    pub(super) fn holds(&self, circle: &Circle, eps: f64) -> bool {
        match self {
            Spun::Cone(cone) => cone.holds(circle, eps),
            Spun::Round(round) => round.holds(circle, eps),
        }
    }
}

/// The grid of a cone, a sphere or a torus: how many steps a turn, how many
/// rows a turn of the meridian for a round (none for a cone), and the angles
/// of the vertices lying on it but at its apex or poles, where the angle is
/// rounding.
pub(super) struct Grid {
    pub(super) spun: Spun,
    pub(super) steps: usize,
    pub(super) rows: usize,
    angles: Vec<f64>,
}

impl Grid {
    /// The directions from the axis a circle of the surface is sampled along:
    /// the grid's, then the vertices'.
    pub(super) fn rays(&self) -> impl Iterator<Item = DVec3> + '_ {
        (0..self.steps)
            .map(|step| TAU * step as f64 / self.steps as f64)
            .chain(self.angles.iter().copied())
            .map(|angle| self.spun.radial(angle))
    }

    /// The angles of the vertices lying on the surface but at its apex or
    /// poles.
    pub(super) fn vertex_angles(&self) -> &[f64] {
        &self.angles
    }
}

/// The grid of every cone, sphere and torus of the body, read at the
/// furthest any circle lying on it, conic cut on it, vertex on it or face on
/// it stands from the axis: a ring a plane cuts at a slant has no vertex to
/// read its width off, and may stand further out than every rim; a face of a
/// round across its crest reaches further than its rims.
pub(super) fn grids(body: &Body, tolerance: f64) -> BTreeMap<SurfaceId, Grid> {
    let eps = body.scale().eps();
    let circles: Vec<&Circle> = body
        .edge_ids()
        .filter_map(|id| match body.curve(body.edge(id).curve) {
            Curve::Circle(circle) => Some(circle),
            Curve::Line(_) | Curve::Meet(_) | Curve::Conic(_) => None,
        })
        .collect();
    let conics: Vec<(&Conic, &Edge)> = body
        .edge_ids()
        .map(|id| body.edge(id))
        .filter_map(|edge| match body.curve(edge.curve) {
            Curve::Conic(conic) => Some((conic, edge)),
            Curve::Line(_) | Curve::Circle(_) | Curve::Meet(_) => None,
        })
        .collect();
    (0..body.surfaces.len() as u32)
        .map(SurfaceId)
        .filter_map(|id| match body.surface(id) {
            Surface::Cone(cone) => Some((id, Spun::Cone(*cone))),
            Surface::Sphere(sphere) => Some((id, Spun::Round(Rounded::Sphere(*sphere)))),
            Surface::Torus(torus) => Some((id, Spun::Round(Rounded::Torus(*torus)))),
            Surface::Plane(_) | Surface::Cylinder(_) | Surface::Elliptic(_) => None,
        })
        .map(|(id, spun)| {
            let mut widest = circles
                .iter()
                .filter(|circle| spun.holds(circle, eps))
                .map(|circle| circle.radius)
                .fold(0.0, f64::max);
            if let Spun::Cone(cone) = spun {
                for (conic, edge) in &conics {
                    if conic.round == Round::Cone(cone) {
                        widest = widest.max(conic.widest(edge.from, edge.to));
                    }
                }
            }
            let mut angles = Vec::new();
            for vertex in body.vertex_ids().map(|vertex| body.vertex(vertex)) {
                if !vertex.on.contains(&id) {
                    continue;
                }
                let from = vertex.point - spun.origin();
                let radial = from - spun.axis() * from.dot(spun.axis());
                widest = widest.max(radial.length());
                if radial.length() > eps {
                    angles.push(radial.dot(spun.v()).atan2(radial.dot(spun.u())));
                }
            }
            let (steps, rows) = match spun {
                Spun::Cone(_) => (divisions(widest, tolerance), 0),
                Spun::Round(round) => {
                    let faces = body
                        .face_ids()
                        .map(|face| body.face(face))
                        .filter(|face| face.surface == id);
                    if faces
                        .into_iter()
                        .any(|face| across_the_crest(body, face, &round, eps))
                    {
                        widest = widest.max(round.widest());
                    }
                    let steps = divisions(widest, tolerance * SHARE);
                    (
                        kept_by_walls(body, &round, steps, tolerance, eps),
                        divisions(round.radius(), tolerance * SHARE),
                    )
                }
            };
            (
                id,
                Grid {
                    spun,
                    steps,
                    rows,
                    angles,
                },
            )
        })
        .collect()
}

/// A round's `steps` a turn, made a multiple of the grid of every cylinder
/// of the body that stands on its axis and touches it along an equator — a
/// ring round a post of its inner radius or in a bore of its outer one, a
/// ball in a bore of its own — where the two grids start the same way round.
/// A circle another surface only touches a round along parts neither face
/// (#576), so neither face samples it, and a chord of the round along it
/// cut across the cylinder's between two of its steps: the two crossed
/// beside the circle they share (5576855166, 5585606550). Left as they are
/// where no such multiple stands under the most a turn is cut into.
fn kept_by_walls(body: &Body, round: &Rounded, steps: usize, tolerance: f64, eps: f64) -> usize {
    let mut every = 1;
    for surface in &body.surfaces {
        let Surface::Cylinder(wall) = surface else {
            continue;
        };
        if !touching(round, wall, eps) {
            continue;
        }
        let theirs = divisions(wall.radius, tolerance);
        let turn = wall.u.dot(round.v()).atan2(wall.u.dot(round.u()));
        let ranks = turn / TAU * theirs as f64;
        if (ranks - ranks.round()).abs() <= ALIGNED {
            every = every / greatest_common(every, theirs) * theirs;
        }
    }
    let kept = steps.div_ceil(every) * every;
    if kept > MOST { steps } else { kept }
}

/// Under this share of a step, two grids start at one angle.
const ALIGNED: f64 = 1e-9;

/// Whether `wall` stands on the axis of `round` and touches it along an
/// equator: its radius the round's furthest from the axis or nearest.
fn touching(round: &Rounded, wall: &Cylinder, eps: f64) -> bool {
    let from = round.origin() - wall.origin;
    let off = (from - wall.axis * wall.axis.dot(from)).length();
    let leaning = wall.axis.cross(round.axis()).length() * wall.radius;
    let (outer, inner) = (round.widest(), round.widest() - 2.0 * round.radius());
    off <= eps
        && leaning <= eps
        && [outer, inner]
            .iter()
            .any(|equator| (equator - wall.radius).abs() <= eps)
}

fn greatest_common(one: usize, other: usize) -> usize {
    if other == 0 {
        one
    } else {
        greatest_common(other, one % other)
    }
}

/// Whether a face of `round` reaches its crest, the circle furthest from the
/// axis: its latitude circles and its vertices stand on either side of it
/// along the meridian, a meridian bounding it runs across it — a face
/// winding round the tube whose rims all stand inside, beside a shaft
/// touching its tube there (#623) — or nothing bounds it there, a whole
/// surface or a face between two whole meridians.
fn across_the_crest(body: &Body, face: &Face, round: &Rounded, eps: f64) -> bool {
    let mut along: Vec<f64> = face
        .poles
        .iter()
        .map(|vertex| round.parameters(body.vertex(*vertex).point).y)
        .collect();
    for coedge in face.loops.iter().flatten() {
        let edge = body.edge(coedge.edge);
        if let Curve::Circle(circle) = body.curve(edge.curve)
            && round.holds(circle, eps)
        {
            along.push(round.latitude(circle));
        }
        if let Curve::Circle(circle) = body.curve(edge.curve)
            && round.meridian(circle, eps)
            && let Ok(Trace::Segment { from, to }) = body.trace(coedge.edge, face.surface)
            && (from.y.max(to.y) / TAU).floor() * TAU >= from.y.min(to.y)
        {
            return true;
        }
        for end in edge.ends.into_iter().flatten() {
            along.push(round.parameters(body.vertex(end).point).y);
        }
    }
    along.is_empty()
        || along.iter().copied().fold(f64::INFINITY, f64::min) <= 0.0
            && along.iter().copied().fold(f64::NEG_INFINITY, f64::max) >= 0.0
}
