//! How a straight edge between two flat faces is read as a [`Wedge`]: its
//! faces and the numbers they answer to, the way each faces and the way into
//! each, whether they meet inwards, and how far a curved face one of its ends
//! runs into stands out past them. How far each runs from the edge is a
//! step's to read, where the edge's blend runs.

use std::f64::consts::TAU;

use glam::DVec3;

use super::Wedge;
use super::end::end_face;
use crate::brep::{self, Curve, Declined, EdgeId, FaceId, Surface};

impl Wedge {
    /// The wedge of an edge straight between two flat faces, whether or not
    /// it runs on tangentially into the next edge — a step reads that on the
    /// body as given, before it lays anything — its reach left endless, for
    /// the step to read where the edge's blend runs, and, where its faces
    /// meet outwards, its [`dip`]. Declined `Unsupported` where a boundary of
    /// either face holds a curve two walls meet along, whose turning back no
    /// closed form says.
    pub(super) fn read(body: &brep::Body, edge: EdgeId) -> Result<Wedge, Declined> {
        let mut wedge = Wedge::faced(body, edge)?;
        let meeting = (wedge.faces.iter())
            .flat_map(|&face| body.face(face).loops.iter().flatten())
            .any(|coedge| matches!(body.curve(body.edge(coedge.edge).curve), Curve::Meet(_)));
        if meeting {
            return Err(Declined::Unsupported);
        }
        if !wedge.inwards {
            wedge.dip = dip(body, edge, &wedge);
        }
        Ok(wedge)
    }

    /// The wedge of an edge straight between two flat faces, but for how far
    /// its faces reach, left endless, and its dip, left nought: whether the
    /// faces meet outwards or inwards, and nothing of the faces past their
    /// planes.
    pub(super) fn faced(body: &brep::Body, edge: EdgeId) -> Result<Wedge, Declined> {
        let stretch = body.edge(edge);
        let Curve::Line(line) = body.curve(stretch.curve) else {
            return Err(Declined::Unsupported);
        };
        let along = line.direction * (stretch.to - stretch.from).signum();
        let side = |face: FaceId, forward: bool| {
            let face = body.face(face);
            let Surface::Plane(plane) = body.surface(face.surface) else {
                return None;
            };
            let outward = if face.flipped {
                -plane.normal
            } else {
                plane.normal
            };
            let running = if forward { along } else { -along };
            Some((outward, outward.cross(running)))
        };
        let uses = body.uses(edge);
        let [(one, one_way), (other, other_way)] = uses[..] else {
            return Err(Declined::Unsupported);
        };
        let (Some((first, into_first)), Some((second, into_second))) =
            (side(one, one_way), side(other, other_way))
        else {
            return Err(Declined::Unsupported);
        };
        let towards = into_first.dot(second);
        if towards == 0.0 {
            return Err(Declined::Unsupported);
        }
        Ok(Wedge {
            point: line.origin,
            along,
            faces: [one, other],
            numbers: [one, other].map(|face| body.numbers(face).to_vec()),
            outward: [first, second],
            into: [into_first, into_second],
            reach: f64::INFINITY,
            dip: 0.0,
            inwards: towards > 0.0,
        })
    }
}

/// How far a curved face an end of `edge` runs into stands out of the matter
/// past either of the edge's faces, as `wedge` reads them: the furthest any
/// curve bounding that face stands past either face's plane, out from the
/// corner at that end — a round of radius `r` leaving a face outwards at `θ`,
/// `r·(1 − cos θ)` (#713). A cut is carried on past such an end, over that
/// face, and its prism's sides must stand out past the dip or leave a sliver
/// of the round standing (#873). Nought at a flat end, which a cut either
/// stops flush on — a flat face leaving one of the edge's faces outwards
/// faces back along the edge — or meets at a corner the matter stays inside.
fn dip(body: &brep::Body, edge: EdgeId, wedge: &Wedge) -> f64 {
    let ends = body.edge(edge).ends.into_iter().flatten();
    let mut dip: f64 = 0.0;
    for end in ends {
        let Some(face) = end_face(body, end, wedge.faces) else {
            continue;
        };
        let face = body.face(face);
        if matches!(body.surface(face.surface), Surface::Plane(_)) {
            continue;
        }
        let corner = body.vertex(end).point;
        for coedge in face.loops.iter().flatten() {
            for outward in wedge.outward {
                let furthest = furthest(body, coedge.edge, outward);
                dip = dip.max(furthest - outward.dot(corner));
            }
        }
    }
    dip
}

/// How far the edge `edge` reaches along `way`: the most of `way · p` over its
/// points — at an end, or for a circle where it turns back along `way`, or at
/// one of a few points along any other curve.
fn furthest(body: &brep::Body, edge: EdgeId, way: DVec3) -> f64 {
    let stretch = body.edge(edge);
    let (low, high) = (stretch.from.min(stretch.to), stretch.from.max(stretch.to));
    let along = |t: f64| way.dot(body.point_on(edge, t));
    let ends = along(low).max(along(high));
    match body.curve(stretch.curve) {
        Curve::Line(_) => ends,
        Curve::Circle(circle) => {
            let turning = way.dot(circle.v).atan2(way.dot(circle.u));
            let at = turning + ((low - turning) / TAU).ceil() * TAU;
            if at <= high {
                ends.max(along(at))
            } else {
                ends
            }
        }
        _ => (1..SAMPLES)
            .map(|step| along(low + (high - low) * step as f64 / SAMPLES as f64))
            .fold(ends, f64::max),
    }
}

/// How many stretches a curve other than a line or a circle is read in for
/// how far it reaches: a curve bounding a round end face turns once at most
/// over it, so that its furthest stands within a stretch of a sample.
const SAMPLES: usize = 32;
