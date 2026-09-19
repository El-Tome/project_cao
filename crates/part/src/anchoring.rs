//! What a drawing is laid on, and whether the part still has it.
//!
//! A drawing on a plane of the origin is held by nothing and never moves. One
//! laid on a face of the part travels with that face: the design records which
//! face, and the plane is worked out again at every replay from the part as it
//! then stands.

use cao_sketch::{PointId, WorkPlane};
use glam::{DVec2, DVec3};

use crate::history::{FaceAnchor, PointRef};
use crate::state::PartState;

impl PartState {
    /// Where a drawing about to be laid down actually sits.
    ///
    /// `recorded` is the plane it was drawn on the day it was made, kept as
    /// what to fall back on. A face that the part no longer has leaves the
    /// drawing there and marks it: catching the nearest face instead would
    /// move a drawing to somewhere nobody pointed at.
    pub(crate) fn plane_for(&mut self, recorded: WorkPlane, on: &Option<FaceAnchor>) -> WorkPlane {
        let Some(anchor) = on else {
            return recorded;
        };
        match self.face_now(anchor) {
            Some(plane) => plane,
            None => {
                self.adrift.insert(self.sketches.len());
                recorded
            }
        }
    }

    /// The plane the anchored face offers as the part stands now.
    fn face_now(&self, anchor: &FaceAnchor) -> Option<WorkPlane> {
        let normal = self.body.pieces_of(anchor.face).next()?.normal();
        let corners: Vec<DVec3> = self
            .body
            .pieces_of(anchor.face)
            .flat_map(|piece| piece.corners.iter().copied())
            .collect();
        Some(WorkPlane::from_face(&corners, normal, anchor.up))
    }

    /// The point an operation names, laid down in the drawing.
    ///
    /// A point dropped on a corner of the part is put where that corner
    /// stands **now** and held there, so the drawing travels with the part
    /// rather than keeping the place the click happened to fall on.
    ///
    /// A corner the part no longer has leaves the point where it was drawn,
    /// loose, and marks the drawing: catching the nearest corner instead
    /// would move a point to somewhere nobody pointed at.
    pub(crate) fn point_for(&mut self, sketch: usize, named: &PointRef) -> Option<PointId> {
        let plane = self.sketches.get(sketch)?.plane;
        let (at, landed) = match named {
            PointRef::Existing(id) => return Some(*id),
            PointRef::New(at) => (*at, None),
            PointRef::OnCorner { at, faces } => {
                (*at, Some(self.corner_now(faces, plane.to_world(*at))))
            }
        };
        let drawing = self.sketches.get_mut(sketch)?;
        match landed {
            None => Some(drawing.add_point(at)),
            Some(Some(corner)) => {
                let point = drawing.add_point(plane.to_local(corner));
                drawing.hold_on_the_part(point);
                Some(point)
            }
            Some(None) => {
                self.unanchored.insert(sketch);
                Some(self.sketches.get_mut(sketch)?.add_point(at))
            }
        }
    }

    /// Where the named corner stands now.
    ///
    /// A corner answers to the name when every face the name holds meets at
    /// it. It may have more: a boolean can cut a face the corner touches into
    /// pieces, and a corner nothing happened to must not be lost over it.
    /// Fewest of those extras wins, and the place clicked settles a draw —
    /// the same reckoning an area's name gets in #345, for the same reason.
    fn corner_now(&self, faces: &[usize], near: DVec3) -> Option<DVec3> {
        let mut answering: Vec<(usize, f64, DVec3)> = self
            .body
            .corners()
            .into_iter()
            .filter(|corner| faces.iter().all(|face| corner.faces.contains(face)))
            .map(|corner| {
                (
                    corner.faces.len().saturating_sub(faces.len()),
                    corner.at.distance(near),
                    corner.at,
                )
            })
            .collect();
        answering.sort_by(|a, b| a.0.cmp(&b.0).then(a.1.total_cmp(&b.1)));
        answering.first().map(|(_, _, at)| *at)
    }

    /// The corners of the part a drawing can land on: the ones lying on its
    /// own plane, each with where it falls on it and the faces that name it.
    ///
    /// Only those on the plane. A corner standing anywhere else has a place
    /// on the plane too, straight down onto it, but landing a point there
    /// would be drawing against a shadow — projecting the part's geometry is
    /// its own behaviour and is not this one.
    pub fn corners_on(&self, sketch: usize) -> Vec<(DVec2, Vec<usize>)> {
        let Some(plane) = self.sketches.get(sketch).map(|drawing| drawing.plane) else {
            return Vec::new();
        };
        let normal = plane.normal();
        self.body
            .corners()
            .into_iter()
            .filter(|corner| (corner.at - plane.origin).dot(normal).abs() <= ON_THE_PLANE)
            .map(|corner| (plane.to_local(corner.at), corner.faces))
            .collect()
    }
}

/// How far off the plane a corner may stand and still be on it. A corner and
/// a plane both come out of the same replay, so they meet to within rounding.
const ON_THE_PLANE: f64 = 1e-9;
