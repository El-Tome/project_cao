//! The matter, as the rest of the workspace is allowed to see it.

use glam::{DVec2, DVec3};
use serde::{Deserialize, Serialize};

use crate::mesh::{Mesh, Polygon};
use crate::sweep::{self, Loop};

/// A solid: one closed surface, its faces numbered.
///
/// What it is made of is this crate's own business — flat pieces today,
/// whatever computes the matter better tomorrow. Everything above raises it,
/// joins it, cuts it, draws it and points at its faces through what is here,
/// and nothing above can reach inside it (#499).
///
/// Written exactly as the pieces it holds, so that a geometry cache written
/// before there was a body reads as one.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Body {
    mesh: Mesh,
}

/// A face of the body a ray met: how far along the ray, which face, and which
/// way the piece of it the ray met is facing.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FaceHit {
    pub distance: f64,
    pub face: usize,
    pub normal: DVec3,
}

/// What a face offers a drawing laid on it: which way its first piece faces,
/// and every corner of every piece of it, in the order the pieces hold them.
#[derive(Clone, Debug, PartialEq)]
pub struct FacePlane {
    pub normal: DVec3,
    pub corners: Vec<DVec3>,
}

impl Body {
    /// A flat area raised into a prism along `direction`.
    pub fn prism(
        outline: Loop<'_>,
        holes: &[Loop<'_>],
        triangles: &[[DVec2; 3]],
        to_world: impl Fn(DVec2) -> DVec3,
        direction: DVec3,
    ) -> Body {
        Body {
            mesh: sweep::prism(outline, holes, triangles, to_world, direction),
        }
    }

    /// A flat area turned about an axis lying in its own plane, or `None` when
    /// the area straddles the axis and would sweep through itself.
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
        Some(Body { mesh })
    }

    /// Everything that is in either body.
    pub fn union(&self, other: &Body) -> Body {
        Body {
            mesh: self.mesh.union(&other.mesh),
        }
    }

    /// Everything that is in this body and not in the other.
    pub fn difference(&self, other: &Body) -> Body {
        Body {
            mesh: self.mesh.difference(&other.mesh),
        }
    }

    /// What is left of the body behind a plane, left open where the plane
    /// went through: a way of looking inside rather than of taking matter away.
    pub fn behind(&self, normal: DVec3, offset: f64) -> Body {
        Body {
            mesh: self.mesh.behind(normal, offset),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.mesh.is_empty()
    }

    /// The triangles the body is drawn with.
    pub fn triangles(&self) -> Vec<[DVec3; 3]> {
        self.mesh.triangles()
    }

    /// The face a ray meets first, if any.
    pub fn ray_hit(&self, origin: DVec3, direction: DVec3) -> Option<FaceHit> {
        let hit = self.mesh.ray_hit(origin, direction)?;
        Some(FaceHit {
            distance: hit.distance,
            face: hit.polygon.face,
            normal: hit.polygon.normal(),
        })
    }

    /// The number no face of this body answers to: a face numbered from it on
    /// is one made since it was read.
    pub fn faces_end(&self) -> usize {
        self.mesh.faces_end()
    }

    /// The triangles one face is drawn with, which is what lighting it whole
    /// under the cursor needs. None when the body has no such face.
    pub fn pieces_of(&self, face: usize) -> impl Iterator<Item = [DVec3; 3]> + '_ {
        self.mesh.pieces_of(face).flat_map(Polygon::triangles)
    }

    /// Whether a face is flat, which is whether a drawing can be laid on it.
    pub fn is_flat(&self, face: usize) -> bool {
        self.mesh.is_flat(face)
    }

    /// The plane a face offers a drawing, or `None` when the body has no such
    /// face. Read off the face whether it is flat or not: which faces may carry
    /// a drawing is for the caller to ask [`Body::is_flat`].
    pub fn plane_of(&self, face: usize) -> Option<FacePlane> {
        let normal = self.mesh.pieces_of(face).next()?.normal();
        let corners = self
            .mesh
            .pieces_of(face)
            .flat_map(|piece| piece.corners.iter().copied())
            .collect();
        Some(FacePlane { normal, corners })
    }

    /// The lowest and the highest corner of the box the body fits in.
    pub fn bounds(&self) -> Option<(DVec3, DVec3)> {
        self.mesh.bounds()
    }

    /// The matter the body encloses.
    pub fn volume(&self) -> f64 {
        self.mesh.volume()
    }
}

#[cfg(test)]
mod tests;
