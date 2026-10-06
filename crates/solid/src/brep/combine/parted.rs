//! Decision 2, for a wall standing within the tolerance of two parallel
//! planes an operand decided apart: decided at the boolean's tolerance, it
//! would touch both, along two lines a hair apart on two surfaces no corner
//! may lie on together, and stand off one of them by the hair. It is
//! decided against each at the tolerance the operand told them apart at,
//! so that it touches the one it touches and crosses or misses the other,
//! as it does.
//!
//! The two planes stand a hair apart on one side of the wall. Two either
//! side of it, a diameter apart, are the move midway between them of
//! `canonical/touches.rs`, or a slot's sides its cap was drawn tangent to:
//! decided finer, a bore's wall a slot's cap was taken for missed one side
//! and crossed the other, and the slot's lines of touch along its sides
//! stood on surfaces decided apart (8575631).

use std::collections::BTreeMap;

use super::Operands;
use crate::brep::scale::Scale;
use crate::brep::surface::{Cylinder, Plane, Surface};
use crate::brep::topology::SurfaceId;

impl Operands<'_> {
    /// Each pair of a wall and one of two parallel planes an operand
    /// carries and decided apart, the wall within the tolerance of both and
    /// not that operand's alone, sorted, and the scale the operand decided
    /// the two planes at.
    pub(super) fn parted(&self) -> BTreeMap<[SurfaceId; 2], Scale> {
        let list = &self.surfaces.list;
        let eps = self.eps();
        let mut finer = BTreeMap::new();
        for (rank, wall) in list.iter().enumerate() {
            let wall = match wall {
                Surface::Cylinder(wall) => wall,
                Surface::Plane(_) | Surface::Cone(_) => continue,
            };
            let wall_id = SurfaceId(rank as u32);
            let planes: Vec<(SurfaceId, Plane)> = list
                .iter()
                .enumerate()
                .filter_map(|(other, surface)| match surface {
                    Surface::Plane(plane) if off(wall, plane, self.scale) <= eps => {
                        Some((SurfaceId(other as u32), *plane))
                    }
                    Surface::Plane(_) | Surface::Cylinder(_) | Surface::Cone(_) => None,
                })
                .collect();
            for (index, &(one, first)) in planes.iter().enumerate() {
                for &(other, second) in &planes[index + 1..] {
                    if !beside(wall, [first, second]) {
                        continue;
                    }
                    if let Some(decided) = self.told_apart([one, other], [first, second], wall_id) {
                        for plane in [one, other] {
                            finer.insert([plane.min(wall_id), plane.max(wall_id)], decided);
                        }
                    }
                }
            }
        }
        finer
    }

    /// The scale an operand carrying both planes, but not the wall alone,
    /// told them apart at, where they are parallel and further apart than
    /// its tolerance.
    fn told_apart(
        &self,
        pair: [SurfaceId; 2],
        [first, second]: [Plane; 2],
        wall: SurfaceId,
    ) -> Option<Scale> {
        let reach = self.scale.reach();
        if first.normal.cross(second.normal).length() * 2.0 * reach > self.eps() {
            return None;
        }
        let gap =
            (first.offset() - first.normal.dot(second.normal).signum() * second.offset()).abs();
        (0..2)
            .filter(|&operand| {
                pair.iter().all(|&plane| self.carries(operand, plane))
                    && !(self.carries(operand, wall) && !self.carries(1 - operand, wall))
            })
            .map(|operand| self.decided(operand, pair))
            .find(|decided| gap > decided.eps())
    }
}

/// How far a wall stands from touching a plane along its axis; infinite
/// where the plane does not run along the axis.
fn off(wall: &Cylinder, plane: &Plane, scale: Scale) -> f64 {
    if wall.axis.dot(plane.normal).abs() * 2.0 * scale.reach() > scale.eps() {
        return f64::INFINITY;
    }
    (plane.distance(wall.origin).abs() - wall.radius).abs()
}

/// Whether two planes stand on one side of a wall's axis, a hair apart
/// there, rather than either side of it, a diameter apart.
fn beside(wall: &Cylinder, [first, second]: [Plane; 2]) -> bool {
    let towards = |plane: Plane| plane.normal * plane.distance(wall.origin);
    towards(first).dot(towards(second)) > 0.0
}
