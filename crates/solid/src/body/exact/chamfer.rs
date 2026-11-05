//! The cut a chamfer makes: a slab standing on the plane set back by the
//! distance along each of the edge's two faces, raised past the edge and
//! cut from the body. Its floor is the one face the step names.

use glam::{DVec2, DVec3};

use super::{Exact, PIECES, root};
use crate::body::caught::caught;
use crate::brep::{self, Curve, Declined, EdgeId, EdgeName, FaceId, Surface};
use crate::profile::{Contour, Frame};

/// A straight edge between two flat faces meeting outwards: a point of it,
/// the way it runs, for each face the way it faces and the way into it,
/// square to the edge, and how far the nearer face runs that way to its
/// nearest corner off the edge — the most a chamfer can set back.
struct Wedge {
    point: DVec3,
    along: DVec3,
    outward: [DVec3; 2],
    into: [DVec3; 2],
    reach: f64,
}

impl Exact {
    /// The body with the edge a name finds cut off, set back by `distance`
    /// along each of its two faces or by the most they give, and the
    /// distance taken; the face the cut lays numbered where the steps'
    /// numbers ended. Unsupported: an edge that is not straight
    /// between two flat faces meeting outwards, and a cut that, carried along
    /// the edge's line, takes matter beyond it — a face of the slab but its
    /// floor left on the body, the floor in two pieces, or a corner of it on
    /// neither of the edge's faces.
    pub fn chamfered(&self, name: &EdgeName, distance: f64) -> Result<(Exact, f64), Declined> {
        caught(|| {
            let edge = self
                .brep
                .find_rooted(name, |number| root(&self.roots, number))
                .map_err(Declined::Lost)?;
            let wedge = Wedge::of(&self.brep, edge)?;
            let bounds = self.brep.bounds().ok_or(Declined::Unsupported)?;
            let taken = distance.clamp(0.0, wedge.reach);
            let (outline, frame, travel) = wedge.slab(taken, bounds);
            let tool = Exact {
                next: 1,
                ..Exact::raised(&outline, &[], frame, travel)?
            };
            let made = self.next;
            let cut = self.cut_by(&tool)?;
            if cut.cut_along(&wedge, made) {
                Ok((cut, taken))
            } else {
                Err(Declined::Unsupported)
            }
        })
    }

    /// Whether the slab left its floor alone on the body, numbered `made`,
    /// in one piece, every corner of it on one of the edge's two faces.
    fn cut_along(&self, wedge: &Wedge, made: u32) -> bool {
        let eps = self.brep.scale().eps();
        let stray = self
            .brep
            .face_ids()
            .flat_map(|face| self.brep.numbers(face))
            .any(|&number| number > made && number < PIECES);
        let parted = self.roots.values().any(|&replaced| replaced == made);
        let on_a_side = self
            .named(made as usize)
            .flat_map(|face| self.brep.face(face).loops.iter().flatten())
            .flat_map(|coedge| self.brep.edge(coedge.edge).ends.into_iter().flatten())
            .all(|corner| {
                let at = self.brep.vertex(corner).point - wedge.point;
                wedge.outward.iter().any(|side| side.dot(at).abs() <= eps)
            });
        !stray && !parted && self.has_face(made as usize) && on_a_side
    }
}

impl Wedge {
    fn of(body: &brep::Body, edge: EdgeId) -> Result<Wedge, Declined> {
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
        if into_first.dot(second) >= 0.0 {
            return Err(Declined::Unsupported);
        }
        let eps = body.scale().eps();
        let reach = [(one, into_first), (other, into_second)]
            .into_iter()
            .flat_map(|(face, into)| {
                let corners = body.corners_of(face);
                corners
                    .into_iter()
                    .map(move |at| into.dot(at - line.origin))
            })
            .filter(|&far| far > eps)
            .fold(f64::INFINITY, f64::min);
        Ok(Wedge {
            point: line.origin,
            along,
            outward: [first, second],
            into: [into_first, into_second],
            reach,
        })
    }

    /// The slab whose floor is the chamfer's plane: as long as the body's box
    /// along the edge and a distance more at each end, twice the chamfer's
    /// width across, and twice the distance high, which clears the edge.
    /// Nothing of it but its floor meets the two faces.
    fn slab(&self, distance: f64, (low, high): (DVec3, DVec3)) -> (Contour, Frame, DVec3) {
        let [into_first, into_second] = self.into;
        let facing = (self.outward[0] + self.outward[1]).normalize();
        let frame = Frame {
            origin: self.point + (into_first + into_second) * distance / 2.0,
            u: self.along,
            v: facing.cross(self.along),
        };
        let middle = self.along.dot((low + high) / 2.0 - frame.origin);
        let length = self.along.abs().dot((high - low) / 2.0) + distance;
        let width = (into_first - into_second).length() * distance;
        let outline = Contour::rectangle(
            DVec2::new(middle - length, -width),
            DVec2::new(middle + length, width),
        );
        (outline, frame, frame.normal() * 2.0 * distance)
    }
}
