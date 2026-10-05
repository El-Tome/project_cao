//! A body the exact kernel computed, the count its faces are numbered by, and
//! the triangles it is drawn with, made once and only when asked for.

use std::collections::BTreeSet;
use std::sync::OnceLock;

use glam::DVec3;
use serde::{Deserialize, Serialize};

use super::caught::{caught, quietly};
use super::drawn::Drawn;
use super::{FaceHit, FacePlane};
use crate::brep::{self, Declined, FaceId, Surface};
use crate::mesh::Mesh;
use crate::profile::{Contour, Frame};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub(super) struct Exact {
    brep: brep::Body,
    /// The number the next face made is given: every number a step named,
    /// kept or not, stands below it, so that no number is ever given twice.
    next: u32,
    #[serde(skip)]
    drawn: OnceLock<Drawn>,
}

impl Default for Exact {
    fn default() -> Self {
        Exact::of(brep::Body::empty(), 0)
    }
}

/// Two exact bodies are one when their matter and their count are: how far
/// either was drawn yet says nothing of what it is.
impl PartialEq for Exact {
    fn eq(&self, other: &Self) -> bool {
        self.brep == other.brep && self.next == other.next
    }
}

impl Exact {
    fn of(brep: brep::Body, next: u32) -> Exact {
        Exact {
            brep,
            next,
            drawn: OnceLock::new(),
        }
    }

    pub fn raised(
        outline: &Contour,
        holes: &[Contour],
        frame: Frame,
        travel: DVec3,
    ) -> Result<Exact, Declined> {
        let brep = caught(|| brep::Body::raised(outline, holes, frame, travel))?;
        Ok(Exact::of(brep, brep::Body::numbers_raised(outline, holes)))
    }

    pub fn joined(&self, other: &Exact) -> Result<Exact, Declined> {
        self.combined(other, |first, second| {
            if first.face_ids().next().is_none() {
                return Ok(second.clone());
            }
            first.joined(second)
        })
    }

    pub fn cut_by(&self, tool: &Exact) -> Result<Exact, Declined> {
        self.combined(tool, |first, second| {
            if first.face_ids().next().is_none() {
                return Ok(first.clone());
            }
            first.cut_by(second)
        })
    }

    /// The operation on the two bodies, the second's numbers moved above the
    /// first's count, every face a cut left in pieces apart told apart, and
    /// the result drawn: a body the kernel cannot draw is declined, rather
    /// than shown as nothing and handed on as nothing to the flats.
    fn combined(
        &self,
        other: &Exact,
        operation: impl FnOnce(&brep::Body, &brep::Body) -> Result<brep::Body, Declined>,
    ) -> Result<Exact, Declined> {
        caught(|| {
            let second = other.brep.clone().renumbered(self.next);
            let brep = if second.face_ids().next().is_none() {
                self.brep.clone()
            } else {
                operation(&self.brep, &second)?
            };
            let mut next = self.next + other.next;
            let brep = parted(brep, &mut next);
            let drawn = Drawn::of(&brep);
            let exact = Exact::of(brep, next);
            let _ = exact.drawn.set(drawn);
            Ok(exact)
        })
    }

    pub fn count_past(&mut self, outline: &Contour, holes: &[Contour]) {
        self.next += brep::Body::numbers_raised(outline, holes);
    }

    pub fn is_empty(&self) -> bool {
        self.brep.face_ids().next().is_none()
    }

    pub fn faces_end(&self) -> usize {
        self.next as usize
    }

    /// The exact volume, or the volume the body is drawn with should
    /// reading it stop on a bug.
    pub fn volume(&self) -> f64 {
        quietly(|| self.brep.volume()).unwrap_or_else(|| self.flats().volume())
    }

    pub fn bounds(&self) -> Option<(DVec3, DVec3)> {
        quietly(|| self.brep.bounds()).flatten()
    }

    pub fn has_face(&self, face: usize) -> bool {
        self.named(face).next().is_some()
    }

    /// Flat when every face answering to the number lies on one plane and
    /// faces one way.
    pub fn is_flat(&self, face: usize) -> bool {
        let mut named = self.named(face).map(|id| {
            let face = self.brep.face(id);
            (face.surface, face.flipped)
        });
        let Some(first) = named.next() else {
            return false;
        };
        matches!(self.brep.surface(first.0), Surface::Plane(_)) && named.all(|each| each == first)
    }

    /// Read off the exact plane and the exact corners, never off the
    /// triangles: how finely those are cut follows the body's reach, and a
    /// drawing on a face would move when an unrelated part of the body grew.
    pub fn plane_of(&self, face: usize) -> Option<FacePlane> {
        quietly(|| self.exact_plane_of(face)).flatten()
    }

    fn exact_plane_of(&self, face: usize) -> Option<FacePlane> {
        let first = self.brep.face(self.named(face).next()?);
        let Surface::Plane(plane) = self.brep.surface(first.surface) else {
            return None;
        };
        let normal = if first.flipped {
            -plane.normal
        } else {
            plane.normal
        };
        let corners = self
            .named(face)
            .flat_map(|id| self.brep.corners_of(id))
            .collect();
        Some(FacePlane { normal, corners })
    }

    /// The triangles, drawn when first asked for. A body built by joining
    /// or cutting was drawn as it was built; a tool raised alone, or a body
    /// read back, is drawn here, and drawn empty should that stop on a bug.
    pub fn drawn(&self) -> &Drawn {
        self.drawn
            .get_or_init(|| quietly(|| Drawn::of(&self.brep)).unwrap_or_default())
    }

    pub fn ray_hit(&self, origin: DVec3, direction: DVec3) -> Option<FaceHit> {
        quietly(|| self.drawn().ray_hit(&self.brep, origin, direction)).flatten()
    }

    pub fn triangles_of(&self, face: usize) -> impl Iterator<Item = [DVec3; 3]> + '_ {
        self.drawn().of_face(&self.brep, face)
    }

    pub fn flats(&self) -> Mesh {
        self.drawn().flats(&self.brep)
    }

    fn named(&self, face: usize) -> impl Iterator<Item = FaceId> + '_ {
        self.brep
            .face_ids()
            .filter(move |&id| self.brep.numbers(id).iter().any(|&n| n as usize == face))
    }
}

/// The body with each face a cut left in pieces that no longer touch told
/// apart: the first piece keeps the number, each other one takes the next
/// fresh number, as the flats do. A drawing laid on one piece then has no
/// business being measured from the other.
fn parted(mut body: brep::Body, next: &mut u32) -> brep::Body {
    let numbers: BTreeSet<u32> = body
        .face_ids()
        .flat_map(|face| body.numbers(face).to_vec())
        .collect();
    for number in numbers {
        for piece in body.pieces_of(number).iter().skip(1) {
            body.rename(piece, number, *next);
            *next += 1;
        }
    }
    body
}
