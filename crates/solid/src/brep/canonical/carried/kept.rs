//! What a move must keep: every surface it carries standing with the first
//! operand's as it stood, and apart from its own operand's left behind.

use std::collections::BTreeSet;

use crate::brep::relation::{Relation, relation};
use crate::brep::scale::Scale;
use crate::brep::surface::Surface;
use crate::brep::topology::Body;

/// Whether every surface the move carries along with the surface of rank
/// `taken` stands with each of the first operand's as it stood before:
/// touching it still, and one with it only where the move brings it no
/// further off — the first's is the surface read, so a surface carried off
/// it by less than the tolerance would stay where it was while the corners
/// and the walls it was built with went. A hundredth of a degree's turn
/// lying on a block's side, slid onto a round its other end grazes, took the
/// line its two ends meet along ninety microns along the side (5365230254).
/// A rounded block a hair off a block's sides, slid onto one of them, brings
/// its other side onto the block's other one, and is kept a hair off
/// otherwise (5366611312). And a cone, or a surface beside one,
/// decided with it about one axis still (#536): moved a hair off, the pair
/// would meet along a curve the kernel does not build, and a bore of a
/// pocket's radius moved onto it took its cone off the wall it widens onto
/// (5361127255). The surface itself goes where its move was decided: one
/// with the first's wall, about its axis, or onto a touch. And whether every
/// surface carried stays apart from those of its own operand left behind,
/// which that operand built apart: a rounded rectangle a hair taller than its
/// two corners, its top moved down onto a face, would take its upper corners'
/// walls onto its lower ones (8528992).
pub(super) fn kept(
    first: &Body,
    before: &Body,
    after: &Body,
    carried: &BTreeSet<usize>,
    taken: usize,
    scale: Scale,
) -> bool {
    let same = |one: &Surface, other: &Surface| {
        matches!(relation(one, other, scale), Relation::Same { .. })
    };
    let parted = carried.iter().all(|&rank| {
        (0..before.surfaces.len())
            .filter(|left| !carried.contains(left))
            .all(|left| {
                same(&before.surfaces[rank], &before.surfaces[left])
                    || !same(&after.surfaces[rank], &after.surfaces[left])
            })
    });
    parted
        && carried.iter().filter(|&&rank| rank != taken).all(|&rank| {
            first.surfaces.iter().all(|known| {
                let held = |surface: &Surface| match relation(known, surface, scale) {
                    Relation::Same { .. } | Relation::Tangent(_) => true,
                    Relation::Parallel(rulings) if rulings.iter().any(|ruled| ruled.touching) => {
                        true
                    }
                    Relation::Conic(_) | Relation::Ruling { .. } | Relation::Unsupported => false,
                    Relation::Apart
                    | Relation::Line(_)
                    | Relation::Lines(_)
                    | Relation::Parallel(_)
                    | Relation::Circle(_)
                    | Relation::Meet(_)
                    | Relation::Rulings { .. }
                    | Relation::Apex(_) => [known, surface]
                        .iter()
                        .any(|one| matches!(one, Surface::Cone(_))),
                };
                (!same(known, &before.surfaces[rank])
                    || onto(known, &before.surfaces[rank], &after.surfaces[rank], scale))
                    && (!held(&before.surfaces[rank]) || held(&after.surfaces[rank]))
            })
        })
}

/// Whether a surface one with `known` before the move is one with it after,
/// and no further from it: a plane by its offset along the normal, a
/// cylinder by its axis and its radius. Its corners and its walls then go
/// towards the surface read rather than away from it. Two cones are not
/// weighed: a move carrying a cone one with the first's is not made.
fn onto(known: &Surface, before: &Surface, after: &Surface, scale: Scale) -> bool {
    let off = |surface: &Surface| match (known, surface) {
        (Surface::Plane(known), Surface::Plane(plane)) => Some(known.distance(plane.origin).abs()),
        (Surface::Cylinder(known), Surface::Cylinder(cylinder)) => {
            let between = cylinder.origin - known.origin;
            let across = between - known.axis * known.axis.dot(between);
            Some(across.length() + (cylinder.radius - known.radius).abs())
        }
        _ => None,
    };
    matches!(relation(known, after, scale), Relation::Same { .. })
        && matches!((off(before), off(after)), (Some(before), Some(after)) if after <= before)
}
