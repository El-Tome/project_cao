//! Decision 7 of `docs/exact-kernel.md`: the band two surfaces stand within
//! the tolerance of each other over, decided once. A plane touching a
//! cylinder, or two cylinders of one radius crossing at a grazing angle,
//! stand within the tolerance of each other over a band about as wide as the
//! square root of twice the radius times the tolerance, far wider than it; a
//! third surface or line within that band parts each surface into strips
//! thinner than the tolerance, whose point stands femtometres off the other
//! surface, where no ray tells which side it is on.
//!
//! A region standing within the tolerance of another surface all across, the
//! two lying along each other, is decided with that surface once: an
//! operand none of whose faces on the region's surface covers it, but one of
//! whose faces lies over it on the other, wraps it once where the region
//! stands on that face's matter side and not at all on the other — and
//! which side of the face the region stands on is read off how the arena
//! decided the two surfaces, never off a ray from a point femtometres off
//! the face. All across is checked on the region's point and along every arc
//! bounding it: both surfaces are smooth and the region is thin, so a region
//! whose boundary stands within the tolerance of a surface stands within it
//! inside too. Two planes are never read this way: decision 1 takes them for
//! one or keeps them apart.

use glam::DVec3;

use super::Place;
use super::wrapped::{covering, lies_above};
use crate::brep::combine::Operands;
use crate::brep::overlay::Region;
use crate::brep::surface::Surface;
use crate::brep::topology::{Body, EdgeId, SurfaceId};

/// How many stretches each arc bounding a region is measured over.
const STRETCHES: usize = 8;

/// The sine of the widest angle two surfaces lying along each other part by
/// over a band: at its edge a cylinder turns from a plane it touches by the
/// root of twice the tolerance over its radius, a thousandth for a radius a
/// millionth of the reach. A surface crossing a region steeper than this
/// stands within the tolerance of it only because the region is a sliver,
/// which it does not lie along.
const ALONG: f64 = 1e-2;

/// The surfaces among `others` a region of `own` stands within `eps` of all
/// across.
pub(super) fn beside(
    body: &Body,
    own: &Surface,
    region: &Region,
    edges: &[EdgeId],
    others: impl Iterator<Item = SurfaceId>,
    eps: f64,
) -> Vec<SurfaceId> {
    let inside = own.point(region.inside);
    let mut samples: Option<Vec<DVec3>> = None;
    others
        .filter(|&other| {
            let surface = body.surface(other);
            let curved =
                matches!(own, Surface::Cylinder(_)) || matches!(surface, Surface::Cylinder(_));
            if !curved || surface.distance(inside).abs() > eps {
                return false;
            }
            let along = own
                .normal(own.parameters(inside))
                .cross(surface.normal(surface.parameters(inside)))
                .length();
            if along > ALONG {
                return false;
            }
            samples
                .get_or_insert_with(|| bounding(body, region, edges))
                .iter()
                .all(|sample| surface.distance(*sample).abs() <= eps)
        })
        .collect()
}

/// Points along every arc bounding a region, its ends among them.
fn bounding(body: &Body, region: &Region, edges: &[EdgeId]) -> Vec<DVec3> {
    region
        .cycles
        .iter()
        .flatten()
        .flat_map(|&(arc, _)| {
            let edge = edges[arc];
            let stretch = body.edge(edge);
            (0..=STRETCHES).map(move |step| {
                let share = step as f64 / STRETCHES as f64;
                body.point_on(edge, stretch.from + (stretch.to - stretch.from) * share)
            })
        })
        .collect()
}

/// How many times an operand none of whose faces on a region's surface
/// covers the region wraps it, read off its faces the region stands within
/// the tolerance of all across. Stacked along the region's normal, the
/// nearest such face on either side tells: the region is in the operand's
/// matter where it stands on that face's matter side. Which side of a face
/// the region stands on, and which of two faces on one side is nearer, is
/// read off how the arena decided the surfaces — a cylinder touching a
/// plane lies on its axis's side of it, the inner of two touching cylinders
/// inside the outer — never off where the region's point stands, within
/// rounding of them. None where no face of the operand lies over the
/// region's point, where its foot lands on a face's boundary, or where the
/// order cannot be read or the two sides disagree: a ray is cast instead.
pub(super) fn wound_beside(operands: &Operands, operand: usize, places: &[Place]) -> Option<i32> {
    let mut found = None;
    for place in places {
        let winding = wound_at(operands, operand, place)?;
        if winding.is_some_and(|winding| found.is_some_and(|known| known != winding)) {
            return None;
        }
        found = winding.or(found);
    }
    found
}

/// A face of the operand over a place, on another surface: whether its
/// matter and the surface itself stand on the side the place's normal
/// points to, and whether its surface's normal points that way too.
struct Over {
    surface: SurfaceId,
    matter_above: bool,
    lying_above: bool,
    agree: bool,
}

/// The winding at one place, or none where no face of the operand lies over
/// it; `None` outright where the faces cannot be read.
fn wound_at(operands: &Operands, operand: usize, place: &Place) -> Option<Option<i32>> {
    let own = place.geometry;
    let normal = own.normal(own.parameters(place.point));
    let mut over = Vec::new();
    for &other in place.beside {
        let surface = &operands.surfaces.list[other.0 as usize];
        let foot = surface.point(surface.parameters(place.point));
        let wrapped = match covering(operands, operand, other, foot) {
            Ok(Some(wrapped)) => wrapped,
            Ok(None) => continue,
            Err(_) => return None,
        };
        let agree = surface.normal(surface.parameters(foot)).dot(normal) > 0.0;
        let pair = [place.surface, other];
        over.push(Over {
            surface: other,
            matter_above: (wrapped.above == 1) == agree,
            lying_above: lies_above(operands, operands.scale_of(pair), pair, place.point)?,
            agree,
        });
    }
    let mut winding = None;
    for side in [true, false] {
        let mut nearest: Option<&Over> = None;
        for face in over.iter().filter(|face| face.lying_above == side) {
            nearest = match nearest {
                None => Some(face),
                Some(known) => {
                    let pair = [known.surface, face.surface];
                    let higher = lies_above(operands, operands.scale_of(pair), pair, place.point)?
                        == known.agree;
                    Some(if higher != side { face } else { known })
                }
            };
        }
        if let Some(face) = nearest {
            let inside = i32::from(face.matter_above != side);
            if winding.is_some_and(|known| known != inside) {
                return None;
            }
            winding = Some(inside);
        }
    }
    Some(winding)
}
