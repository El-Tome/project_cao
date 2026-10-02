//! Decision 9 of `docs/exact-kernel.md`: the band of two surfaces decided to
//! touch, laid out once, the same on both. A plane touching a cylinder, or
//! two cylinders touching, stand within the tolerance of each other over a
//! band about the root of twice the radius times the tolerance wide; a third
//! surface's line or corner inside it parts one surface into strips thinner
//! than the tolerance and not the other, or ends inside the band, leaving
//! strips no arcs bound in common, and two faces back to back whose
//! triangles lie on each other.
//!
//! So a corner standing within the tolerance of both surfaces, on a face of
//! each, lies on both; the line through it along
//! the line of touch is drawn on both; and wherever a surface crossing the
//! line of touch at a corner of the band crosses one of these lines, a
//! corner is put. Every strip of the band is then parted where the others
//! are, on both surfaces, by arcs both carry, and the strips of the two are
//! twins, decided once (decision 6).

use glam::DVec3;

use super::operands::Operands;
use crate::brep::Declined;
use crate::brep::canonical::{Pool, Registry, distance, same};
use crate::brep::curve::{Curve, Line};
use crate::brep::relation::{Crossings, Relation, Touches, crossings_given, relation};
use crate::brep::surface::Surface;
use crate::brep::topology::SurfaceId;

/// How square to the line of touch a surface must stand, in the cosine of
/// its normal's angle with it, to part the band across: a surface nearly
/// along the band — a wall turning off a plane it continues — crosses a line
/// of the band at a grazing angle, a hair either side of where it stands.
const ACROSS: f64 = 1e-3;

/// Two surfaces decided to touch along a registered line.
struct Touch {
    pair: [SurfaceId; 2],
    line: Line,
}

/// The corners and lines of every band laid on the pool and the registry.
pub(super) fn laid_out(
    operands: &Operands,
    registry: &mut Registry,
    pool: &mut Pool,
) -> Result<(), Declined> {
    let mut supports = pool.supports(registry);
    let mut grown = false;
    for touch in touches(operands, registry) {
        if grown {
            supports = pool.supports(registry);
            grown = false;
        }
        let [on_line, banded] = sorted(operands, registry, pool, &supports, &touch)?;
        if banded.is_empty() {
            continue;
        }
        grown = true;
        let direction = touch.line.direction;
        let touching = registry.register(Curve::Line(touch.line), &touch.pair);
        let mut lengthwise = vec![(touch.line, touching)];
        for &corner in &banded {
            let through = Line::through(pool.corners[corner].point, direction);
            let rank = registry.register(Curve::Line(through), &touch.pair);
            if lengthwise.iter().all(|(_, known)| *known != rank) {
                lengthwise.push((through, rank));
            }
        }
        let corners: Vec<usize> = on_line.into_iter().chain(banded).collect();
        let across = crossing(operands, pool, &supports, &corners, &touch);
        let along: Vec<f64> = corners
            .iter()
            .map(|&corner| pool.corners[corner].point.dot(direction))
            .collect();
        let eps = operands.eps();
        let low = along.iter().copied().fold(f64::INFINITY, f64::min) - eps;
        let high = along.iter().copied().fold(f64::NEG_INFINITY, f64::max) + eps;
        let [one, other] = touch.pair;
        for (line, rank) in &lengthwise {
            for &(surface, at) in &across {
                let geometry = &operands.surfaces.list[surface.0 as usize];
                let Some(node) = nearest(*line, geometry, at, operands) else {
                    continue;
                };
                let place = node.dot(direction);
                if place >= low && place <= high && within(operands, registry, &touch, node)? {
                    pool.add(node, [one, other, surface], [*rank], registry);
                }
            }
        }
    }
    Ok(())
}

/// Every pair decided to touch whose line of touch was registered, its
/// faces' boxes meeting.
fn touches(operands: &Operands, registry: &Registry) -> Vec<Touch> {
    let list = &operands.surfaces.list;
    let mut found = Vec::new();
    for one in 0..list.len() {
        for other in one + 1..list.len() {
            let pair = [SurfaceId(one as u32), SurfaceId(other as u32)];
            if !registry.apart.touch(pair[0], pair[1]) {
                continue;
            }
            let Relation::Tangent(line) =
                relation(&list[one], &list[other], operands.scale_of(pair))
            else {
                continue;
            };
            let registered = registry.list.iter().any(|known| {
                same(&known.curve, &Curve::Line(line), operands.scale)
                    && pair.iter().all(|surface| known.support.contains(surface))
            });
            if registered {
                found.push(Touch { pair, line });
            }
        }
    }
    found
}

/// The corners of a band, those on its line of touch and the others: on
/// either surface, within the tolerance of both, on a face of each.
fn sorted(
    operands: &Operands,
    registry: &Registry,
    pool: &Pool,
    supports: &[Vec<SurfaceId>],
    touch: &Touch,
) -> Result<[Vec<usize>; 2], Declined> {
    let line = Curve::Line(touch.line);
    let mut found = [Vec::new(), Vec::new()];
    for (rank, (corner, support)) in pool.corners.iter().zip(supports).enumerate() {
        let point = corner.point;
        if !touch.pair.iter().any(|surface| support.contains(surface))
            || registry.apart.across(support, &touch.pair)
            || !within(operands, registry, touch, point)?
        {
            continue;
        }
        let off = distance(&line, point) > operands.eps();
        found[usize::from(off)].push(rank);
    }
    Ok(found)
}

/// Whether a place stands within the tolerance of both surfaces of a band,
/// on a face of each.
fn within(
    operands: &Operands,
    registry: &Registry,
    touch: &Touch,
    point: DVec3,
) -> Result<bool, Declined> {
    let eps = operands.eps();
    let [one, other] = touch.pair;
    Ok(registry.apart.near(one, point, eps)
        && registry.apart.near(other, point, eps)
        && faced(operands, one, point)?
        && faced(operands, other, point)?)
}

/// The surfaces a line along the line of touch crosses at a corner of the
/// band, each with the corner: those that part the band across.
fn crossing(
    operands: &Operands,
    pool: &Pool,
    supports: &[Vec<SurfaceId>],
    corners: &[usize],
    touch: &Touch,
) -> Vec<(SurfaceId, DVec3)> {
    let eps = operands.eps();
    let mut across: Vec<(SurfaceId, DVec3)> = Vec::new();
    for &corner in corners {
        let point = pool.corners[corner].point;
        let through = Line::through(point, touch.line.direction);
        for &surface in &supports[corner] {
            if touch.pair.contains(&surface)
                || across
                    .iter()
                    .any(|(known, at)| *known == surface && at.distance(point) <= eps)
            {
                continue;
            }
            let geometry = &operands.surfaces.list[surface.0 as usize];
            let normal = geometry.normal(geometry.parameters(point));
            if normal.dot(touch.line.direction).abs() >= ACROSS
                && nearest(through, geometry, point, operands)
                    .is_some_and(|found| found.distance(point) <= eps)
            {
                across.push((surface, point));
            }
        }
    }
    across
}

/// Where a line crosses a surface nearest a place.
fn nearest(line: Line, surface: &Surface, place: DVec3, operands: &Operands) -> Option<DVec3> {
    let alone = Touches {
        along: false,
        at: Vec::new(),
    };
    match crossings_given(&Curve::Line(line), surface, operands.scale, &alone) {
        Crossings::At(list) => list
            .into_iter()
            .map(|crossing| crossing.point)
            .min_by(|one, other| one.distance(place).total_cmp(&other.distance(place))),
        Crossings::Along | Crossings::Unsupported => None,
    }
}

/// Whether a place on `own` lies on `other` too, as its band lays it: the
/// two decided to touch, the place within the tolerance of `other` and on
/// a face of it.
pub(super) fn beside(
    operands: &Operands,
    registry: &Registry,
    own: SurfaceId,
    other: SurfaceId,
    point: DVec3,
) -> Result<bool, Declined> {
    if !registry.apart.touch(own, other) || !registry.apart.near(other, point, operands.eps()) {
        return Ok(false);
    }
    faced(operands, other, point)
}

/// Whether a place lies inside or on the boundary of a face of either
/// operand on a surface: a plane whose face is far off, or ends short of
/// the place, makes no band there.
fn faced(operands: &Operands, surface: SurfaceId, point: DVec3) -> Result<bool, Declined> {
    for operand in 0..2 {
        if operands.carries(operand, surface) && operands.touched(operand, surface, point)? {
            return Ok(true);
        }
    }
    Ok(false)
}
