//! How a straight edge between two flat faces is read as a [`Wedge`]: its
//! faces and the numbers they answer to, the way each faces and the way into
//! each, whether they meet inwards, and how far each runs from the edge.

use glam::DVec3;

use super::{Wedge, reach};
use crate::brep::{self, Curve, Declined, EdgeId, FaceId, Surface};

impl Wedge {
    /// The wedge of an edge straight between two flat faces, whether or not
    /// it runs on tangentially into the next edge: a step reads that on the
    /// body as given, before it lays anything.
    pub(super) fn read(body: &brep::Body, edge: EdgeId) -> Result<Wedge, Declined> {
        let wedge = Wedge::faced(body, edge)?;
        let reach_of = |face: FaceId, into: DVec3| reach::reach(body, face, into, wedge.point);
        let [(one, into_one), (other, into_other)] =
            [0, 1].map(|side| (wedge.faces[side], wedge.into[side]));
        let (Some(first_reach), Some(second_reach)) =
            (reach_of(one, into_one), reach_of(other, into_other))
        else {
            return Err(Declined::Unsupported);
        };
        Ok(Wedge {
            reach: first_reach.min(second_reach),
            ..wedge
        })
    }

    /// The wedge of an edge straight between two flat faces, but for how far
    /// its faces reach, left endless: whether the faces meet outwards or
    /// inwards, and nothing of the faces past their planes.
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
            inwards: towards > 0.0,
        })
    }
}
