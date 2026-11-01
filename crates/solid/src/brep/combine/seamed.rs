//! Where the chart of a ring torus is cut for the faces a boolean makes on
//! it (#576, #623). A torus's chart is a window one turn wide along the
//! meridian, centred on its `seam` and cut at the antipode, and a face is
//! read in that window: the turn centres it on the arc it lays, so every
//! face it makes sits inside one window. A face a boolean makes may reach
//! across the cut, and the selection joins it back across it
//! (`selection/parted/glued.rs`); the cut is still moved where it can be,
//! for the face it leaves whole. A whole ring is one face with no loop, and
//! a ring turned part of the way one bounded by whole meridians alone, which
//! the cut may fall anywhere on: what a boolean keeps of either is a band
//! between the rims the other operand's surfaces cross it along, and the cut
//! is moved into a band the boolean keeps no face of — one the other
//! operand's matter covers, for a join or for the body a tool is cut from,
//! and one it leaves bare for the tool itself ([`seamed`]). And the cut is
//! never left on a rim or through a vertex ([`reseamed`]).

use std::f64::consts::{PI, TAU};

use glam::DVec2;

use super::{Arena, Operands, Operation};
use crate::brep::curve::{Circle, Curve};
use crate::brep::scale::Scale;
use crate::brep::surface::{Surface, Torus};
use crate::brep::topology::{Edge, EdgeId, SurfaceId};

/// Each ring torus of the arena one operand alone carries, every face of it
/// bounded by whole meridian circles or by nothing, cut along the middle of
/// a band between its rims that `operation` keeps no face of, where there
/// is one.
pub(super) fn seamed(arena: &mut Arena, operands: &Operands, operation: Operation) {
    let eps = operands.eps();
    for rank in 0..arena.body.surfaces.len() {
        let Surface::Torus(torus) = arena.body.surfaces[rank] else {
            continue;
        };
        if torus.center.y.abs() <= torus.radius {
            continue;
        }
        let surface = SurfaceId(rank as u32);
        let [owner] = (0..2)
            .filter(|operand| operands.carries(*operand, surface))
            .collect::<Vec<usize>>()[..]
        else {
            continue;
        };
        let body = operands.bodies[owner];
        let whole_meridian = |edge: &Edge| {
            edge.ends.is_none()
                && matches!(body.curve(edge.curve), Curve::Circle(circle) if torus.meridian(circle, eps))
        };
        if operands.lying[owner][rank].iter().any(|face| {
            body.face(*face)
                .loops
                .iter()
                .flatten()
                .any(|coedge| !whole_meridian(body.edge(coedge.edge)))
        }) {
            continue;
        }
        let mut angles = rims(arena, surface, &torus, eps);
        angles.sort_by(f64::total_cmp);
        angles.dedup();
        let other = operands.bodies[1 - owner];
        let kept_inside = operation == Operation::AndNot && owner == 1;
        let middles = (0..angles.len()).map(|at| {
            let next = angles.get(at + 1).copied().unwrap_or(angles[0] + TAU);
            (angles[at] + next) / 2.0
        });
        let cut = middles.into_iter().find(|&middle| {
            other
                .winding(torus.point(DVec2::new(0.0, middle)), eps)
                .is_ok_and(|winding| (winding != 0) != kept_inside)
        });
        if let Some(cut) = cut {
            arena.body.surfaces[rank] = cut_at(torus, cut);
        }
    }
}

/// Each ring torus of the arena whose chart is cut along a latitude circle
/// lying on it, or through a vertex on it — a slab resting on a ring's lower
/// crest when the chart is cut there, a wall of the ring's own radius
/// meeting its end meridians at the crests — cut instead along the middle
/// of the widest band between those circles and vertices (#623). On the
/// cut, such a circle would be read along the window's top, over the floor,
/// and a meridian ending there would leave a stretch of nothing past it;
/// anywhere else a face reaching across the cut is joined back across it
/// as the selection reads it.
pub(super) fn reseamed(arena: &mut Arena, operands: &Operands) {
    let eps = operands.eps();
    for rank in 0..arena.body.surfaces.len() {
        let Surface::Torus(torus) = arena.body.surfaces[rank] else {
            continue;
        };
        if torus.center.y.abs() <= torus.radius {
            continue;
        }
        let surface = SurfaceId(rank as u32);
        let mut angles: Vec<f64> = rims(arena, surface, &torus, eps)
            .into_iter()
            .chain(
                arena
                    .body
                    .vertices
                    .iter()
                    .filter(|vertex| vertex.on.binary_search(&surface).is_ok())
                    .map(|vertex| torus.parameters(vertex.point).y),
            )
            .map(|angle| angle.rem_euclid(TAU))
            .collect();
        let room = Scale::HAIR * eps / torus.radius;
        let cut = torus.seam + PI;
        let on_the_cut = |angle: &f64| ((angle - cut + PI).rem_euclid(TAU) - PI).abs() <= room;
        if !angles.iter().any(on_the_cut) {
            continue;
        }
        angles.sort_by(f64::total_cmp);
        angles.dedup();
        let widest = (0..angles.len())
            .map(|at| {
                let next = angles.get(at + 1).copied().unwrap_or(angles[0] + TAU);
                (next - angles[at], (angles[at] + next) / 2.0)
            })
            .max_by(|one, other| one.0.total_cmp(&other.0));
        if let Some((_, middle)) = widest {
            arena.body.surfaces[rank] = cut_at(torus, middle);
        }
    }
}

/// The angles along the meridian, in `(−π, π]`, of the latitude circles the
/// arena lays on a torus.
fn rims(arena: &Arena, surface: SurfaceId, torus: &Torus, eps: f64) -> Vec<f64> {
    arena
        .supports
        .iter()
        .enumerate()
        .filter(|(_, on)| on.binary_search(&surface).is_ok())
        .filter_map(|(edge, _)| {
            match arena.body.curve(arena.body.edge(EdgeId(edge as u32)).curve) {
                Curve::Circle(circle) if torus.holds(circle, eps) => {
                    Some(along_the_meridian(torus, circle))
                }
                Curve::Circle(_) | Curve::Line(_) | Curve::Conic(_) | Curve::Meet(_) => None,
            }
        })
        .collect()
}

/// The torus with its chart cut at the angle `cut` along the meridian.
fn cut_at(torus: Torus, cut: f64) -> Surface {
    let seam = cut + PI;
    Surface::Torus(Torus {
        seam: seam - TAU * (seam / TAU).round(),
        ..torus
    })
}

/// The angle along the meridian a latitude circle of the torus stands at,
/// in `(−π, π]`.
fn along_the_meridian(torus: &Torus, circle: &Circle) -> f64 {
    let h = (circle.center - torus.origin).dot(torus.axis);
    (h - torus.center.x).atan2(circle.radius - torus.center.y)
}
