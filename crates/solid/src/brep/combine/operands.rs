//! The two bodies a boolean reads, laid on the surfaces they share: which
//! faces of each lie on which surface, the box round each face, and where a
//! point of a surface stands against an operand's faces lying on it — asked of
//! the operand as it was built, never of anything the boolean made.

use std::collections::BTreeMap;

use glam::DVec3;

use crate::brep::Declined;
use crate::brep::canonical::Surfaces;
use crate::brep::domain::Location;
use crate::brep::scale::Scale;
use crate::brep::topology::{Body, FaceId, SurfaceId};

pub(in crate::brep) struct Operands<'a> {
    pub bodies: [&'a Body; 2],
    pub scale: Scale,
    pub surfaces: Surfaces,
    /// For each operand, the faces lying on each of the shared surfaces.
    pub lying: [Vec<Vec<FaceId>>; 2],
    boxes: [Vec<[DVec3; 2]>; 2],
    /// Whether decision 2 moved each shared surface onto a touch.
    moved: Vec<bool>,
    /// The pairs of a wall and a plane decided at the finer tolerance an
    /// operand told the plane from another the wall stands as near.
    parted: BTreeMap<[SurfaceId; 2], Scale>,
}

impl<'a> Operands<'a> {
    pub fn of(first: &'a Body, second: &'a Body, scale: Scale) -> Operands<'a> {
        Operands::laid(first, second, scale, true)
    }

    /// The two operands with every move onto a touch made, a slide of a
    /// wall the second operand drew corners on along an exact touch too:
    /// what decision 2 would move were the corners carried along.
    pub fn sliding(first: &'a Body, second: &'a Body, scale: Scale) -> Operands<'a> {
        Operands::laid(first, second, scale, false)
    }

    /// The two operands laid on their shared surfaces; a wall the second
    /// operand drew corners on is `pinned` against a slide along an exact
    /// touch.
    fn laid(first: &'a Body, second: &'a Body, scale: Scale, pinned: bool) -> Operands<'a> {
        let mut surfaces = Surfaces::of(first, second, scale);
        let bodies = [first, second];
        let lying = [0, 1].map(|operand| {
            let mut lying = vec![Vec::new(); surfaces.list.len()];
            let body = bodies[operand];
            for face in body.face_ids() {
                let (surface, _) = surfaces.mapped[operand][body.face(face).surface.0 as usize];
                lying[surface.0 as usize].push(face);
            }
            lying
        });
        let boxes: [Vec<[DVec3; 2]>; 2] = bodies.map(|body| {
            body.face_ids()
                .map(|face| boxed(body, face, scale.eps()))
                .collect()
        });
        let near = |one: SurfaceId, other: SurfaceId| {
            let (lying, boxes) = (&lying, &boxes);
            let faces = |surface: SurfaceId| {
                (0..2).flat_map(move |operand| {
                    lying[operand][surface.0 as usize]
                        .iter()
                        .map(move |face| boxes[operand][face.0 as usize])
                })
            };
            faces(one).any(|first| faces(other).any(|second| meet(first, second)))
        };
        let mut cornered = vec![false; surfaces.list.len()];
        for vertex in second.vertices.iter().filter(|_| pinned) {
            for own in &vertex.on {
                cornered[surfaces.mapped[1][own.0 as usize].0.0 as usize] = true;
            }
        }
        let moved = surfaces.snapped(
            |operand: usize, surface: SurfaceId| !lying[operand][surface.0 as usize].is_empty(),
            near,
            |surface: SurfaceId| cornered[surface.0 as usize],
            scale,
        );
        let mut operands = Operands {
            bodies,
            scale,
            surfaces,
            lying,
            boxes,
            moved,
            parted: BTreeMap::new(),
        };
        operands.parted = operands.parted();
        operands
    }

    pub fn eps(&self) -> f64 {
        self.scale.eps()
    }

    /// The shared surface an operand's face lies on.
    pub fn surface_of(&self, operand: usize, face: FaceId) -> SurfaceId {
        let own = self.bodies[operand].face(face).surface;
        self.surfaces.mapped[operand][own.0 as usize].0
    }

    /// Whether an operand's face turns its matter to the side the shared
    /// surface's own normal points to.
    pub fn flipped(&self, operand: usize, face: FaceId) -> bool {
        let lying = self.bodies[operand].face(face);
        let (_, agree) = self.surfaces.mapped[operand][lying.surface.0 as usize];
        lying.flipped != !agree
    }

    pub fn carries(&self, operand: usize, surface: SurfaceId) -> bool {
        !self.lying[operand][surface.0 as usize].is_empty()
    }

    /// The scale a pair of shared surfaces is decided at: where one operand
    /// alone carries both, the scale it decided them at, when the later of
    /// the two came into it; for a wall and one of two planes an operand
    /// told apart, the wall within the tolerance of both, the scale it told
    /// them apart at; and the boolean's otherwise.
    pub fn scale_of(&self, pair: [SurfaceId; 2]) -> Scale {
        if let Some(scale) = self
            .parted
            .get(&[pair[0].min(pair[1]), pair[0].max(pair[1])])
        {
            return *scale;
        }
        (0..2)
            .find(|&operand| pair.iter().all(|&surface| self.alone(operand, surface)))
            .map_or(self.scale, |operand| self.decided(operand, pair))
    }

    /// The scale an operand decided a pair of the shared surfaces at, as it
    /// carried them: when the later of the two came into it.
    pub fn decided(&self, operand: usize, pair: [SurfaceId; 2]) -> Scale {
        let [one, other] = pair.map(|surface| self.arrived(operand, surface));
        one.joined(other)
    }

    /// For each shared surface, the scale of the operation that brought it
    /// into the result: the first operand's own where it alone carries it,
    /// unmoved, this one's otherwise — the boolean decides every pair it
    /// makes with the first's surfaces. A pair of the second operand's own surfaces is
    /// read at this scale too, a little coarser than it was decided at: a
    /// second operand is a leaf, whose own surfaces stand far apart.
    pub fn arrivals(&self) -> Vec<Scale> {
        (0..self.surfaces.list.len() as u32)
            .map(|rank| match SurfaceId(rank) {
                surface if self.alone(0, surface) => self.arrived(0, surface),
                _ => self.scale,
            })
            .collect()
    }

    /// Whether an operand alone carries a surface as it decided it: one the
    /// boolean moved onto a touch is the boolean's, whose every pair is
    /// decided again at its scale.
    fn alone(&self, operand: usize, surface: SurfaceId) -> bool {
        self.carries(operand, surface)
            && !self.carries(1 - operand, surface)
            && !self.moved[surface.0 as usize]
    }

    /// The scale of the operation that brought a shared surface into an
    /// operand, the latest of its own surfaces taken for it.
    fn arrived(&self, operand: usize, surface: SurfaceId) -> Scale {
        let body = self.bodies[operand];
        self.surfaces.mapped[operand]
            .iter()
            .enumerate()
            .filter(|(_, (shared, _))| *shared == surface)
            .map(|(own, _)| body.arrived(SurfaceId(own as u32)))
            .fold(Scale::of(1.0), Scale::joined)
    }

    /// Whether the boxes round two faces, one of each operand, meet.
    pub fn near(&self, first: FaceId, second: FaceId) -> bool {
        meet(
            self.boxes[0][first.0 as usize],
            self.boxes[1][second.0 as usize],
        )
    }

    /// Where `point`, on a shared surface, stands against each face of an
    /// operand lying on it, on its boundary within `eps` of it.
    pub fn located(
        &self,
        operand: usize,
        surface: SurfaceId,
        point: DVec3,
        eps: f64,
    ) -> Result<Vec<(FaceId, Location)>, Declined> {
        let body = self.bodies[operand];
        self.lying[operand][surface.0 as usize]
            .iter()
            .map(|&face| {
                let own = body.surface(body.face(face).surface);
                Ok((face, body.locate(face, own.parameters(point), eps)?))
            })
            .collect()
    }

    /// Whether `point` lies inside or on the boundary of a face of an
    /// operand on a shared surface.
    pub fn touched(
        &self,
        operand: usize,
        surface: SurfaceId,
        point: DVec3,
    ) -> Result<bool, Declined> {
        Ok(self
            .located(operand, surface, point, self.eps())?
            .iter()
            .any(|(_, location)| *location != Location::Outside))
    }
}

/// Whether two boxes meet.
fn meet(one: [DVec3; 2], other: [DVec3; 2]) -> bool {
    one[0].cmple(other[1]).all() && other[0].cmple(one[1]).all()
}

/// The box round a face, from the extremes of its edges, grown by `eps`.
fn boxed(body: &Body, face: FaceId, eps: f64) -> [DVec3; 2] {
    let mut low = DVec3::INFINITY;
    let mut high = DVec3::NEG_INFINITY;
    for coedge in body.face(face).loops.iter().flatten() {
        for point in body.extremes(body.edge(coedge.edge)) {
            low = low.min(point);
            high = high.max(point);
        }
    }
    [low - DVec3::splat(eps), high + DVec3::splat(eps)]
}
