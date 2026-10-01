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
//! two lying along each other, is that surface's twin: where an operand none
//! of whose faces covers the region has a face there, and a ray from the
//! region's point cannot tell which side of it the point stands on, the face
//! covers the region as it covers the foot of the region's point, read by
//! locating that foot in the face. All across is checked on the region's
//! point and along every arc bounding it: both surfaces are smooth and the
//! region is thin, so a region whose boundary stands within the tolerance of
//! a surface stands within it inside too. Two planes are never twins this
//! way: decision 1 takes them for one or keeps them apart.

use glam::DVec3;

use super::wrapped::{Wrapped, covering};
use crate::brep::Declined;
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

/// How an operand's face on `other` covers a point of a region standing
/// within the tolerance of it all across, seen from the region's surface:
/// as the face covers the point's foot on `other`, its sides turned round
/// where the two surfaces' normals point apart there.
pub(super) fn covering_beside(
    operands: &Operands,
    operand: usize,
    own: &Surface,
    point: DVec3,
    other: SurfaceId,
) -> Result<Option<Wrapped>, Declined> {
    let surface = &operands.surfaces.list[other.0 as usize];
    let foot = surface.point(surface.parameters(point));
    let Some(wrapped) = covering(operands, operand, other, foot)? else {
        return Ok(None);
    };
    let agree = surface
        .normal(surface.parameters(foot))
        .dot(own.normal(own.parameters(point)))
        > 0.0;
    Ok(Some(if agree { wrapped } else { wrapped.turned() }))
}
