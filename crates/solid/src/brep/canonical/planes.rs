//! What planes fix by themselves: two across each other share one line, and
//! three whose normals span space share one place. Two curves found on the
//! same two such planes are one curve, and two corners found on the same
//! three one corner, however far apart they were found — each plane taken
//! for another within the tolerance by decision 1, their crossing moves by
//! more than the tolerance. Such a corner stands where the three meet.

use glam::DVec3;

use crate::brep::surface::Surface;
use crate::brep::topology::SurfaceId;

/// How far from each other the normals must stand for their planes to fix a
/// line or a place, in the sine of their angle or the volume they span: well
/// clear of planes the relation of their pair would take for parallel.
const ACROSS: f64 = 1e-3;

#[derive(Clone, Debug, Default, PartialEq)]
pub(in crate::brep) struct Planes {
    /// Each surface's normal and offset, when it is a plane.
    planes: Vec<Option<(DVec3, f64)>>,
}

impl Planes {
    pub fn of(surfaces: &[Surface]) -> Planes {
        Planes {
            planes: surfaces
                .iter()
                .map(|surface| match surface {
                    Surface::Plane(plane) => Some((plane.normal, plane.offset())),
                    Surface::Cylinder(_) => None,
                })
                .collect(),
        }
    }

    pub fn is_plane(&self, surface: SurfaceId) -> bool {
        self.planes
            .get(surface.0 as usize)
            .is_some_and(Option::is_some)
    }

    /// Whether both sets lie on two planes across each other.
    pub fn fix_a_line(&self, one: &[SurfaceId], other: &[SurfaceId]) -> bool {
        let shared = self.shared(one, other);
        shared.iter().enumerate().any(|(index, first)| {
            shared[index + 1..]
                .iter()
                .any(|second| first.0.cross(second.0).length() >= ACROSS)
        })
    }

    /// Whether both sets lie on three planes whose normals span space.
    pub fn fix_a_place(&self, one: &[SurfaceId], other: &[SurfaceId]) -> bool {
        self.spanning(&self.shared(one, other)).is_some()
    }

    /// The place three planes of `support` whose normals span space meet at,
    /// of all such triples the one spanning the most; none where no three do,
    /// nor where `support` holds a cylinder that fixes the place as well:
    /// one `held` does not say the planes hold.
    pub fn place(&self, support: &[SurfaceId], held: impl Fn(SurfaceId) -> bool) -> Option<DVec3> {
        let planes = self.shared(support, support);
        if support
            .iter()
            .any(|&surface| !self.is_plane(surface) && !held(surface))
        {
            return None;
        }
        let [one, other, third] = self.spanning(&planes)?;
        let volume = one.0.dot(other.0.cross(third.0));
        Some(
            (other.0.cross(third.0) * one.1
                + third.0.cross(one.0) * other.1
                + one.0.cross(other.0) * third.1)
                / volume,
        )
    }

    /// The triple of planes whose normals span the most, when some span.
    fn spanning(&self, planes: &[(DVec3, f64)]) -> Option<[(DVec3, f64); 3]> {
        let mut best: Option<(f64, [(DVec3, f64); 3])> = None;
        for first in 0..planes.len() {
            for second in first + 1..planes.len() {
                for third in second + 1..planes.len() {
                    let triple = [planes[first], planes[second], planes[third]];
                    let volume = triple[0].0.dot(triple[1].0.cross(triple[2].0)).abs();
                    if volume >= ACROSS && best.is_none_or(|(most, _)| volume > most) {
                        best = Some((volume, triple));
                    }
                }
            }
        }
        best.map(|(_, triple)| triple)
    }

    /// The planes both sets lie on, as normals and offsets.
    fn shared(&self, one: &[SurfaceId], other: &[SurfaceId]) -> Vec<(DVec3, f64)> {
        one.iter()
            .filter(|surface| other.contains(surface))
            .filter_map(|surface| self.planes.get(surface.0 as usize).copied().flatten())
            .collect()
    }
}
