//! Decision 2, once per pair of surfaces whose faces, one of each operand,
//! stand in boxes that meet: the curves the pair shares, registered as lying
//! on both, and the points where such a curve crosses itself, where two
//! cylinders only touch, or where a cone's apex stands on the other surface.
//! A pair at a slant, whose curve the kernel does not build, declines the
//! operation unless its faces stand clear of each other.
//!
//! The boxes only spare the pairs that cannot meet; they decide nothing
//! else. A box round a face at a slant is loose, which costs a pair related
//! for nothing where the relation is built, and is decided on the faces
//! themselves where it is not.

use std::collections::BTreeSet;

use glam::DVec3;

use super::clear::clear;
use super::operands::Operands;
use crate::brep::Declined;
use crate::brep::canonical::Registry;
use crate::brep::relation::{Relation, relation};
use crate::brep::topology::{FaceId, SurfaceId};

/// A point a relation says must be a corner, on both surfaces of its pair
/// and on the registered curves passing through it.
#[derive(Clone, Debug, PartialEq)]
pub(super) struct Special {
    pub point: DVec3,
    pub surfaces: [SurfaceId; 2],
    pub curves: Vec<usize>,
}

/// The special points, and the pairs related.
pub(super) fn related(
    operands: &Operands,
    registry: &mut Registry,
) -> Result<(Vec<Special>, BTreeSet<[SurfaceId; 2]>), Declined> {
    let list = &operands.surfaces.list;
    let mut special = Vec::new();
    let mut done = BTreeSet::new();
    for one in 0..list.len() {
        for other in one + 1..list.len() {
            let pair = [SurfaceId(one as u32), SurfaceId(other as u32)];
            if !facing(operands, pair) {
                continue;
            }
            done.insert(pair);
            let found = relation(&list[one], &list[other], operands.scale_of(pair));
            if found == Relation::Unsupported {
                if clear(operands, pair) {
                    continue;
                }
                return Err(Declined::Unsupported);
            }
            let ranks: Vec<usize> = found
                .curves()
                .into_iter()
                .map(|curve| registry.register(curve, &pair))
                .collect();
            match &found {
                Relation::Meet(meeting) => {
                    for node in &meeting.nodes {
                        special.push(Special {
                            point: node.point,
                            surfaces: pair,
                            curves: node
                                .on
                                .iter()
                                .map(|(component, _)| ranks[*component as usize])
                                .collect(),
                        });
                    }
                    if let Some(point) = meeting.contact {
                        special.push(Special {
                            point,
                            surfaces: pair,
                            curves: Vec::new(),
                        });
                    }
                }
                Relation::Rulings {
                    apex: Some(point), ..
                } => special.push(Special {
                    point: *point,
                    surfaces: pair,
                    curves: ranks.clone(),
                }),
                Relation::Apex(point) => special.push(Special {
                    point: *point,
                    surfaces: pair,
                    curves: Vec::new(),
                }),
                Relation::Apart
                | Relation::Same { .. }
                | Relation::Line(_)
                | Relation::Lines(_)
                | Relation::Tangent(_)
                | Relation::Circle(_)
                | Relation::Rulings { apex: None, .. }
                | Relation::Unsupported => {}
            }
        }
    }
    Ok((special, done))
}

/// Every other pair of surfaces a registered curve lies on both of, related
/// too, so that every curve such a pair shares is registered — the other line
/// where a plane cuts a cylinder, though no edge runs along it — and a corner
/// lying on both is told which of them it stands on. A curve a pair shares
/// may come to lie on a third surface, making pairs of its own: they are
/// related in turn, until none is left. A pair of one operand alone is not
/// the boolean's to decline.
pub(super) fn completed(
    operands: &Operands,
    registry: &mut Registry,
    done: &BTreeSet<[SurfaceId; 2]>,
) {
    let mut related = done.clone();
    let list = &operands.surfaces.list;
    loop {
        let mut pairs = BTreeSet::new();
        for registered in &registry.list {
            let support = &registered.support;
            for (index, &one) in support.iter().enumerate() {
                for &other in &support[index + 1..] {
                    if !related.contains(&[one, other]) {
                        pairs.insert([one, other]);
                    }
                }
            }
        }
        if pairs.is_empty() {
            return;
        }
        for [one, other] in pairs {
            related.insert([one, other]);
            let found = relation(
                &list[one.0 as usize],
                &list[other.0 as usize],
                operands.scale_of([one, other]),
            );
            for curve in found.curves() {
                registry.register(curve, &[one, other]);
            }
        }
    }
}

/// Whether a face of one operand on one surface of the pair and a face of
/// the other on the other surface stand in boxes that meet.
fn facing(operands: &Operands, [one, other]: [SurfaceId; 2]) -> bool {
    let on = |operand: usize, surface: SurfaceId| -> &[FaceId] {
        &operands.lying[operand][surface.0 as usize]
    };
    let near = |firsts: &[FaceId], seconds: &[FaceId]| {
        firsts
            .iter()
            .any(|first| seconds.iter().any(|second| operands.near(*first, *second)))
    };
    near(on(0, one), on(1, other)) || near(on(0, other), on(1, one))
}
