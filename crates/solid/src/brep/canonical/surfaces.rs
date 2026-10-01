//! Decision 1: each surface of the second operand is compared with the
//! first's, never a body's surface with its own, and one within the
//! tolerance is the first's — the nearest of them, since the tolerance grows
//! with the reach of the operands and two surfaces of the first a hair apart,
//! told apart when it was made, may both stand within it now.

use crate::brep::relation::{Relation, relation};
use crate::brep::scale::Scale;
use crate::brep::surface::Surface;
use crate::brep::topology::{Body, SurfaceId};

/// The surfaces both operands stand on, the first's first under their own
/// ranks, and where each operand's own surfaces went.
pub(in crate::brep) struct Surfaces {
    pub list: Vec<Surface>,
    /// For each operand, each of its surfaces as one of `list`, and whether
    /// their own normals point the same way.
    pub mapped: [Vec<(SurfaceId, bool)>; 2],
}

impl Surfaces {
    pub fn of(first: &Body, second: &Body, scale: Scale) -> Surfaces {
        let mut list = first.surfaces.clone();
        let own: Vec<(SurfaceId, bool)> = (0..list.len() as u32)
            .map(|rank| (SurfaceId(rank), true))
            .collect();
        let shared = list.len();
        let mut other = Vec::with_capacity(second.surfaces.len());
        for surface in &second.surfaces {
            let alike: Vec<(f64, SurfaceId, bool)> = list[..shared]
                .iter()
                .enumerate()
                .filter_map(|(rank, known)| match relation(known, surface, scale) {
                    Relation::Same { agree } => {
                        Some((gap(known, surface), SurfaceId(rank as u32), agree))
                    }
                    _ => None,
                })
                .collect();
            let same = alike
                .iter()
                .copied()
                .min_by(|one, other| one.0.total_cmp(&other.0).then(one.1.cmp(&other.1)))
                .filter(|_| !between(&list, &alike, surface, scale))
                .map(|(_, rank, agree)| (rank, agree));
            other.push(same.unwrap_or_else(|| {
                list.push(*surface);
                (SurfaceId(list.len() as u32 - 1), true)
            }));
        }
        Surfaces {
            list,
            mapped: [own, other],
        }
    }
}

/// Whether a plane stands strictly between two planes of the first operand it
/// is within the tolerance of: taken for either, it would stand, as its own
/// operand was made, across the strip of wall between the two, where the
/// boolean asks that operand which side a point is on.
fn between(
    list: &[Surface],
    alike: &[(f64, SurfaceId, bool)],
    surface: &Surface,
    scale: Scale,
) -> bool {
    let Surface::Plane(plane) = surface else {
        return false;
    };
    let rounding = scale.eps() * ROUNDING;
    let sides: Vec<f64> = alike
        .iter()
        .filter_map(|(_, rank, _)| match &list[rank.0 as usize] {
            Surface::Plane(known) => {
                let facing = known.normal.dot(plane.normal).signum();
                Some(facing * known.offset() - plane.offset())
            }
            Surface::Cylinder(_) => None,
        })
        .collect();
    sides.iter().any(|side| *side > rounding) && sides.iter().any(|side| *side < -rounding)
}

/// Under this share of the tolerance, two offsets are one.
const ROUNDING: f64 = 1e-6;

/// How far apart two surfaces one within the tolerance stand: their offsets
/// along the normal, or their axes and radii.
fn gap(one: &Surface, other: &Surface) -> f64 {
    match (one, other) {
        (Surface::Plane(one), Surface::Plane(other)) => {
            let facing = one.normal.dot(other.normal).signum();
            (one.offset() - facing * other.offset()).abs()
        }
        (Surface::Cylinder(one), Surface::Cylinder(other)) => {
            let between = other.origin - one.origin;
            let across = between - one.axis * one.axis.dot(between);
            across.length() + (one.radius - other.radius).abs()
        }
        _ => f64::INFINITY,
    }
}
