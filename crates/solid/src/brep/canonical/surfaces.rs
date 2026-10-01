//! Decision 1: each surface of the second operand is compared with the
//! first's, never a body's surface with its own, and one within the
//! tolerance is the first's — the nearest of them, since the tolerance grows
//! with the reach of the operands and two surfaces of the first a hair apart,
//! told apart when it was made, may both stand within it now.

use crate::brep::meet::moved;
use crate::brep::relation::{Relation, relation};
use crate::brep::scale::Scale;
use crate::brep::surface::{Cylinder, Plane, Surface};
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

    /// Decision 2 makes two surfaces of the two operands a hair from touching
    /// touch: two perpendicular cylinders, inside, outside or at a node; a
    /// plane and a cylinder, or two parallel cylinders, decided to touch
    /// along a line. The move is the surface's, made once here before any
    /// curve or corner is found on either, so that the curve they share and
    /// the corners on it stand on both. Two perpendicular cylinders move the
    /// one their pair puts second; a line of touch moves the surface the
    /// second operand alone carries. Only pairs of a surface `carried` by
    /// each operand are moved, a surface once, and never one another was
    /// moved against, nor one that touches another exactly already.
    pub fn snapped(&mut self, carried: impl Fn(usize, SurfaceId) -> bool, scale: Scale) {
        let own = |surface: usize| {
            let surface = SurfaceId(surface as u32);
            carried(1, surface) && !carried(0, surface)
        };
        let pairs: Vec<[usize; 2]> = (0..self.list.len())
            .flat_map(|one| (one + 1..self.list.len()).map(move |other| [one, other]))
            .filter(|&[one, other]| {
                let [first, second] = [one, other].map(|rank| SurfaceId(rank as u32));
                carried(0, first) && carried(1, second) || carried(1, first) && carried(0, second)
            })
            .collect();
        let mut settled = vec![false; self.list.len()];
        for &[one, other] in &pairs {
            if matches!(
                relation(&self.list[one], &self.list[other], scale),
                Relation::Tangent(_)
            ) {
                for (moving, fixed) in [(other, one), (one, other)] {
                    if own(moving) && touching(&self.list[fixed], &self.list[moving]).is_none() {
                        settled[moving] = true;
                    }
                }
            }
        }
        for [one, other] in pairs {
            let found = match relation(&self.list[one], &self.list[other], scale) {
                Relation::Meet(_) => perpendicular(&self.list[one], &self.list[other], scale)
                    .map(|(rank, cylinder)| ([one, other][rank], cylinder)),
                Relation::Tangent(_) if own(other) => {
                    touching(&self.list[one], &self.list[other]).map(|moved| (other, moved))
                }
                Relation::Tangent(_) if own(one) => {
                    touching(&self.list[other], &self.list[one]).map(|moved| (one, moved))
                }
                _ => None,
            };
            let Some((shifted, surface)) = found else {
                continue;
            };
            if settled[shifted] {
                continue;
            }
            self.list[shifted] = surface;
            settled[one] = true;
            settled[other] = true;
        }
    }
}

/// Which of two perpendicular cylinders their pair moves onto the touch, by
/// its rank in the pair, and where to.
fn perpendicular(one: &Surface, other: &Surface, scale: Scale) -> Option<(usize, Surface)> {
    let (Surface::Cylinder(first), Surface::Cylinder(second)) = (one, other) else {
        return None;
    };
    let (rank, cylinder) = moved(first, second, scale)?;
    Some((rank, Surface::Cylinder(cylinder)))
}

/// `moving` moved onto the line it was decided to touch `fixed` along: a
/// cylinder along a plane's normal, a plane along its own, a cylinder
/// towards or away from a parallel one, by the gap the relation found within
/// the tolerance. None where the touch is exact already.
fn touching(fixed: &Surface, moving: &Surface) -> Option<Surface> {
    let moved = match (fixed, moving) {
        (Surface::Plane(plane), Surface::Cylinder(cylinder)) => {
            let away = plane.distance(cylinder.origin);
            let gap = away.abs() - cylinder.radius;
            Surface::Cylinder(Cylinder::about(
                cylinder.origin - plane.normal * away.signum() * gap,
                cylinder.axis,
                cylinder.radius,
            ))
        }
        (Surface::Cylinder(cylinder), Surface::Plane(plane)) => {
            let away = plane.distance(cylinder.origin);
            let gap = away.abs() - cylinder.radius;
            let (plane, _) = Plane::through(
                plane.origin + plane.normal * away.signum() * gap,
                plane.normal,
            );
            Surface::Plane(plane)
        }
        (Surface::Cylinder(kept), Surface::Cylinder(cylinder)) => {
            let between = cylinder.origin - kept.origin;
            let across = between - kept.axis * kept.axis.dot(between);
            let distance = across.length();
            if distance == 0.0 {
                return None;
            }
            let outside = distance - (kept.radius + cylinder.radius);
            let inside = (kept.radius - cylinder.radius).abs() - distance;
            let by = if outside.abs() <= inside.abs() {
                -outside
            } else {
                inside
            };
            Surface::Cylinder(Cylinder::about(
                cylinder.origin + across / distance * by,
                cylinder.axis,
                cylinder.radius,
            ))
        }
        (Surface::Plane(_), Surface::Plane(_)) => return None,
    };
    (moved != *moving).then_some(moved)
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
