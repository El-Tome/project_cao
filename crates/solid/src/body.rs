//! The matter, as the rest of the workspace is allowed to see it.

mod drawn;
mod exact;

use std::borrow::Cow;

use glam::{DVec2, DVec3};
use serde::{Deserialize, Serialize};

use crate::brep::Declined;
use crate::mesh::{Mesh, Polygon};
use crate::profile::{Frame, Profile};
use crate::sweep::{self, Loop};
use exact::Exact;

/// A solid: one closed surface, its faces numbered.
///
/// What it is made of is this crate's own business. Everything above raises
/// it, joins it, cuts it, draws it and points at its faces through what is
/// here, and nothing above can reach inside it (#499).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Body {
    matter: Matter,
}

/// Which kernel computed the matter. Exact on planes and cylinders as long as
/// every step was one the exact kernel builds; flat pieces from the first step
/// it does not — a revolution, an ellipse — on, since an exact body joined to
/// flats can only be flats.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
enum Matter {
    Exact(Exact),
    Flats(Flats),
}

/// Flat pieces, and the number the next face made is given: every number a
/// step named stands below it, kept or not, as in an exact body, so that a
/// part going to the flats never gives a number twice.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
struct Flats {
    mesh: Mesh,
    next: usize,
}

impl Default for Body {
    fn default() -> Self {
        Body {
            matter: Matter::Exact(Exact::default()),
        }
    }
}

/// A face of the body a ray met: how far along the ray, which face, and which
/// way the face is facing where the ray met it.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FaceHit {
    pub distance: f64,
    pub face: usize,
    pub normal: DVec3,
}

/// What a face offers a drawing laid on it: which way it faces, and every
/// corner of every piece of it.
#[derive(Clone, Debug, PartialEq)]
pub struct FacePlane {
    pub normal: DVec3,
    pub corners: Vec<DVec3>,
}

impl Body {
    /// A profile raised into a prism along `travel`, square to its frame, in
    /// the kernel this body is computed by: exact when the body is and the
    /// profile has no run the exact kernel cannot raise, flats otherwise.
    /// Declined only by the exact kernel.
    pub fn tool_raised(
        &self,
        profile: &Profile,
        frame: Frame,
        travel: DVec3,
    ) -> Result<Body, Declined> {
        if let (Matter::Exact(_), Some((outline, holes))) = (&self.matter, &profile.exact) {
            return Ok(Body::of_exact(Exact::raised(
                outline, holes, frame, travel,
            )?));
        }
        let mesh = sweep::prism(
            profile.sampled,
            &profile.sampled_holes,
            profile.triangles,
            |point| frame.at(point),
            travel,
        );
        Ok(Body::raised_flats(mesh))
    }

    /// A flat area turned about an axis lying in its own plane, or `None` when
    /// the area straddles the axis and would sweep through itself. Always
    /// flats: the exact kernel has no surface of revolution.
    pub fn revolution(
        outline: Loop<'_>,
        holes: &[Loop<'_>],
        triangles: &[[DVec2; 3]],
        to_world: impl Fn(DVec2) -> DVec3,
        axis_origin: DVec2,
        axis_direction: DVec2,
        turn: f64,
    ) -> Option<Body> {
        let mesh = sweep::revolution(
            outline,
            holes,
            triangles,
            to_world,
            axis_origin,
            axis_direction,
            turn,
        )?;
        Some(Body::raised_flats(mesh))
    }

    /// Everything that is in either body.
    pub fn union(&self, other: &Body) -> Result<Body, Declined> {
        match (&self.matter, &other.matter) {
            (Matter::Exact(first), Matter::Exact(second)) => {
                Ok(Body::of_exact(first.joined(second)?))
            }
            _ => Ok(self.on_flats(other, Mesh::union_counted)),
        }
    }

    /// Everything that is in this body and not in the other.
    pub fn difference(&self, other: &Body) -> Result<Body, Declined> {
        match (&self.matter, &other.matter) {
            (Matter::Exact(first), Matter::Exact(second)) => {
                Ok(Body::of_exact(first.cut_by(second)?))
            }
            _ => Ok(self.on_flats(other, Mesh::difference_counted)),
        }
    }

    /// What is left of the body behind a plane, left open where the plane
    /// went through: a way of looking inside rather than of taking matter away.
    pub fn behind(&self, normal: DVec3, offset: f64) -> Body {
        Body::of_flats(self.as_flats().behind(normal, offset), self.faces_end())
    }

    pub fn is_empty(&self) -> bool {
        match &self.matter {
            Matter::Exact(exact) => exact.is_empty(),
            Matter::Flats(flats) => flats.mesh.is_empty(),
        }
    }

    /// The triangles the body is drawn with.
    pub fn triangles(&self) -> Vec<[DVec3; 3]> {
        match &self.matter {
            Matter::Exact(exact) => exact.drawn().triangles.clone(),
            Matter::Flats(flats) => flats.mesh.triangles(),
        }
    }

    /// The face a ray meets first, if any.
    pub fn ray_hit(&self, origin: DVec3, direction: DVec3) -> Option<FaceHit> {
        match &self.matter {
            Matter::Exact(exact) => exact.ray_hit(origin, direction),
            Matter::Flats(flats) => {
                let hit = flats.mesh.ray_hit(origin, direction)?;
                Some(FaceHit {
                    distance: hit.distance,
                    face: hit.polygon.face,
                    normal: hit.polygon.normal(),
                })
            }
        }
    }

    /// The number no face of this body answers to: a face numbered from it on
    /// is one made since it was read.
    pub fn faces_end(&self) -> usize {
        match &self.matter {
            Matter::Exact(exact) => exact.faces_end(),
            Matter::Flats(flats) => flats.next,
        }
    }

    /// Moves the faces' count past the numbers a raise of `profile` would
    /// have named, for a step the kernel declined: the steps after it then
    /// number their faces as if it had not been, and a drawing laid on one of
    /// them keeps its face once the step is mended. The flats never decline,
    /// and count their faces by the numbers they hold.
    pub fn count_past(&mut self, profile: &Profile) {
        if let (Matter::Exact(exact), Some((outline, holes))) = (&mut self.matter, &profile.exact) {
            exact.count_past(outline, holes);
        }
    }

    /// Whether some face of the body answers to `face`.
    pub fn has_face(&self, face: usize) -> bool {
        match &self.matter {
            Matter::Exact(exact) => exact.has_face(face),
            Matter::Flats(flats) => flats.mesh.pieces_of(face).next().is_some(),
        }
    }

    /// The triangles one face is drawn with, which is what lighting it whole
    /// under the cursor needs: at least one for every face the body holds, and
    /// none when it has no such face.
    pub fn triangles_of(&self, face: usize) -> Box<dyn Iterator<Item = [DVec3; 3]> + '_> {
        match &self.matter {
            Matter::Exact(exact) => Box::new(exact.triangles_of(face)),
            Matter::Flats(flats) => {
                Box::new(flats.mesh.pieces_of(face).flat_map(Polygon::triangles))
            }
        }
    }

    /// Whether a face is flat, which is whether a drawing can be laid on it.
    pub fn is_flat(&self, face: usize) -> bool {
        match &self.matter {
            Matter::Exact(exact) => exact.is_flat(face),
            Matter::Flats(flats) => flats.mesh.is_flat(face),
        }
    }

    /// The plane a face offers a drawing, or `None` when the body has no such
    /// face. The flats read it off the face whether it is flat or not, and
    /// which faces may carry a drawing is for the caller to ask
    /// [`Body::is_flat`]; the exact kernel offers none on a curved face.
    pub fn plane_of(&self, face: usize) -> Option<FacePlane> {
        match &self.matter {
            Matter::Exact(exact) => exact.plane_of(face),
            Matter::Flats(Flats { mesh, .. }) => {
                let normal = mesh.pieces_of(face).next()?.normal();
                let corners = mesh
                    .pieces_of(face)
                    .flat_map(|piece| piece.corners.iter().copied())
                    .collect();
                Some(FacePlane { normal, corners })
            }
        }
    }

    /// The lowest and the highest corner of the box the body fits in.
    pub fn bounds(&self) -> Option<(DVec3, DVec3)> {
        match &self.matter {
            Matter::Exact(exact) => exact.bounds(),
            Matter::Flats(flats) => flats.mesh.bounds(),
        }
    }

    /// The matter the body encloses.
    pub fn volume(&self) -> f64 {
        match &self.matter {
            Matter::Exact(exact) => exact.volume(),
            Matter::Flats(flats) => flats.mesh.volume(),
        }
    }

    fn of_exact(exact: Exact) -> Body {
        Body {
            matter: Matter::Exact(exact),
        }
    }

    fn of_flats(mesh: Mesh, next: usize) -> Body {
        Body {
            matter: Matter::Flats(Flats { mesh, next }),
        }
    }

    /// A tool the flats raised, its faces counted by the numbers they hold.
    fn raised_flats(mesh: Mesh) -> Body {
        let next = mesh.faces_end();
        Body::of_flats(mesh, next)
    }

    /// The two bodies combined as flats, the other's faces numbered past
    /// every number this one gave, and the count past every number either
    /// gave.
    fn on_flats(
        &self,
        other: &Body,
        operation: fn(&Mesh, &Mesh, usize, &mut usize) -> Mesh,
    ) -> Body {
        let floor = self.faces_end();
        let mut next = floor + other.faces_end();
        let mesh = operation(&self.as_flats(), &other.as_flats(), floor, &mut next);
        Body::of_flats(mesh, next)
    }

    /// The body as flat pieces: its own when it is made of them, the
    /// triangles it is drawn with otherwise.
    fn as_flats(&self) -> Cow<'_, Mesh> {
        match &self.matter {
            Matter::Exact(exact) => Cow::Owned(exact.flats()),
            Matter::Flats(flats) => Cow::Borrowed(&flats.mesh),
        }
    }
}

#[cfg(test)]
mod tests;
