//! Where the chart of a ring torus is cut for the faces a boolean makes on
//! it (#576). A torus's chart is a window one turn wide along the meridian,
//! centred on its `seam` and cut at the antipode, and a face is read in that
//! window: the turn centres it on the arc it lays, so every face it makes
//! sits inside one window. A whole ring is one face with no loop, which the
//! cut may fall anywhere on; what a boolean keeps of it is a band between
//! the rims the other operand's surfaces cross it along, and that band must
//! not straddle the cut, or the chart reads it as two stretches at the
//! window's two ends. So the cut is moved into a band the boolean keeps no
//! face of — one the other operand's matter covers, for a join or for the
//! body a tool is cut from, and one it leaves bare for the tool itself: the
//! kept bands then sit inside the window, however the ring was drawn.

use std::f64::consts::{PI, TAU};

use glam::DVec2;

use super::{Arena, Operands, Operation};
use crate::brep::curve::{Circle, Curve};
use crate::brep::surface::{Surface, Torus};
use crate::brep::topology::{EdgeId, SurfaceId};

/// Each ring torus of the arena one operand alone carries, with no loop on
/// any of its faces, cut along the middle of a band between its rims that
/// `operation` keeps no face of, where there is one.
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
        if operands.lying[owner][rank]
            .iter()
            .any(|face| !body.face(*face).loops.is_empty())
        {
            continue;
        }
        let mut angles: Vec<f64> = arena
            .supports
            .iter()
            .enumerate()
            .filter(|(_, on)| on.binary_search(&surface).is_ok())
            .filter_map(|(edge, _)| {
                match arena.body.curve(arena.body.edge(EdgeId(edge as u32)).curve) {
                    Curve::Circle(circle) if torus.holds(circle, eps) => {
                        Some(along_the_meridian(&torus, circle))
                    }
                    Curve::Circle(_) | Curve::Line(_) | Curve::Conic(_) | Curve::Meet(_) => None,
                }
            })
            .collect();
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
            let seam = cut + PI;
            arena.body.surfaces[rank] = Surface::Torus(Torus {
                seam: seam - TAU * (seam / TAU).round(),
                ..torus
            });
        }
    }
}

/// The angle along the meridian a latitude circle of the torus stands at,
/// in `(−π, π]`.
fn along_the_meridian(torus: &Torus, circle: &Circle) -> f64 {
    let h = (circle.center - torus.origin).dot(torus.axis);
    (h - torus.center.x).atan2(circle.radius - torus.center.y)
}
