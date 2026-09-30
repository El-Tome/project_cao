//! What planes fix by themselves: two across each other share one line, and
//! three whose normals span space share one place. Two curves found on the
//! same two such planes are one curve, and two corners found on the same
//! three one corner, however far apart they were found — each plane taken
//! for another within the tolerance by decision 1, their crossing moves by
//! more than the tolerance.

use glam::DVec3;

use crate::brep::surface::Surface;
use crate::brep::topology::SurfaceId;

/// How far from each other the normals must stand for their planes to fix a
/// line or a place, in the sine of their angle or the volume they span: well
/// clear of planes the relation of their pair would take for parallel.
const ACROSS: f64 = 1e-3;

#[derive(Clone, Debug, Default, PartialEq)]
pub(in crate::brep) struct Planes {
    normals: Vec<Option<DVec3>>,
}

impl Planes {
    pub fn of(surfaces: &[Surface]) -> Planes {
        Planes {
            normals: surfaces
                .iter()
                .map(|surface| match surface {
                    Surface::Plane(plane) => Some(plane.normal),
                    Surface::Cylinder(_) => None,
                })
                .collect(),
        }
    }

    /// Whether both sets lie on two planes across each other.
    pub fn fix_a_line(&self, one: &[SurfaceId], other: &[SurfaceId]) -> bool {
        let shared = self.shared(one, other);
        shared.iter().enumerate().any(|(index, first)| {
            shared[index + 1..]
                .iter()
                .any(|second| first.cross(*second).length() >= ACROSS)
        })
    }

    /// Whether both sets lie on three planes whose normals span space.
    pub fn fix_a_place(&self, one: &[SurfaceId], other: &[SurfaceId]) -> bool {
        let shared = self.shared(one, other);
        (0..shared.len()).any(|first| {
            (first + 1..shared.len()).any(|second| {
                (second + 1..shared.len()).any(|third| {
                    shared[first].dot(shared[second].cross(shared[third])).abs() >= ACROSS
                })
            })
        })
    }

    /// The normals of the planes both sets lie on.
    fn shared(&self, one: &[SurfaceId], other: &[SurfaceId]) -> Vec<DVec3> {
        one.iter()
            .filter(|surface| other.contains(surface))
            .filter_map(|surface| self.normals.get(surface.0 as usize).copied().flatten())
            .collect()
    }
}
