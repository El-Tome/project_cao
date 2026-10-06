//! The operands' own edges, laid on the registered curves: each edge's curve
//! registered with the surfaces of the faces beside it (decision 3), and the
//! stretch of the registered curve the edge covers.
//!
//! An edge beside a face whose surface was taken for one of the first
//! operand's lies on that surface only within the tolerance: its curve is
//! registered as the surfaces it now lies on share it, or a hair left off
//! them would be a hair more off at every later operation, and two corners
//! found on it and on its neighbour, one each way, would no longer be one.
//!
//! Two edges of one operand on two of its curves are never laid on one: the
//! operation that made it kept them apart, at a tolerance this one's may have
//! grown past.
//!
//! An edge an earlier operation took for an arc of another curve (decision
//! 6) lies on that curve's surfaces only between its two corners — a line
//! across a cylinder's axis, a circle off it: its curve is registered on the
//! surfaces that carry it all along, and the others go with the stretch the
//! edge covers.

use std::collections::BTreeMap;

use glam::DVec3;

use super::operands::Operands;
use crate::brep::canonical::{Registry, distance, within};
use crate::brep::curve::Curve;
use crate::brep::relation::relation;
use crate::brep::scale::Scale;
use crate::brep::surface::Surface;
use crate::brep::topology::{CurveId, EdgeId, SurfaceId, VertexId};

/// An edge of an operand, on a registered curve, over a stretch of that
/// curve's parameter, and the surfaces it lies on over that stretch alone.
/// `whole` when the edge is a closed curve with no vertex, a ring: it
/// covers its curve all round, whatever rounding left of where its stretch
/// starts.
#[derive(Clone, Debug, PartialEq)]
pub(in crate::brep) struct Held {
    pub operand: usize,
    pub edge: EdgeId,
    pub curve: usize,
    pub stretch: [f64; 2],
    pub whole: bool,
    pub partly: Vec<SurfaceId>,
}

/// For each operand's vertex, by the operand and the vertex, the registered
/// curves of the edges ending there.
pub(super) type Ending = BTreeMap<(usize, VertexId), Vec<usize>>;

/// Every edge of the first operand, then every edge of the second, and where
/// each ends.
pub(super) fn held(operands: &Operands, registry: &mut Registry) -> (Vec<Held>, Ending) {
    let mut found = Vec::new();
    let mut ending = BTreeMap::new();
    let mut claimed: BTreeMap<usize, (usize, CurveId)> = BTreeMap::new();
    for (operand, body) in operands.bodies.iter().enumerate() {
        for edge in body.edge_ids() {
            let stretch = body.edge(edge);
            let own = body.curve(stretch.curve);
            let (around, partly) = beside(operands, operand, edge, own);
            let theirs = (operand, stretch.curve);
            let curve = registry.register_beside(
                shared(operands, operand, edge, own, &around),
                &around,
                |rank| {
                    claimed
                        .get(&rank)
                        .is_some_and(|&(by, of)| by == operand && of != stretch.curve)
                },
            );
            claimed.entry(curve).or_insert(theirs);
            for end in stretch.ends.iter().flatten() {
                ending
                    .entry((operand, *end))
                    .or_insert_with(Vec::new)
                    .push(curve);
            }
            found.push(Held {
                operand,
                edge,
                curve,
                stretch: registry.stretch(curve, own, stretch.from, stretch.to),
                whole: stretch.ends.is_none(),
                partly,
            });
        }
    }
    (found, ending)
}

/// The shared surfaces of the faces beside an operand's edge, parted into
/// those that carry its curve all along and the others, as the operand was
/// built: a surface the boolean took for another or moved onto a touch
/// stands up to the tolerance off the curve, and a rim of a bore moved onto
/// a side would no longer be its wall's.
fn beside(
    operands: &Operands,
    operand: usize,
    edge: EdgeId,
    own: &Curve,
) -> (Vec<SurfaceId>, Vec<SurfaceId>) {
    let body = operands.bodies[operand];
    let mut around = Vec::new();
    let mut partly = Vec::new();
    for (face, _) in body.uses(edge) {
        let lying = body.surface(body.face(face).surface);
        let shared = operands.surface_of(operand, face);
        if carries(lying, own, operands.scale) {
            around.push(shared);
        } else {
            partly.push(shared);
        }
    }
    around.sort();
    around.dedup();
    partly.sort();
    partly.dedup();
    partly.retain(|surface| around.binary_search(surface).is_err());
    (around, partly)
}

/// An edge's own curve, or, when a face beside it lies on a surface the
/// boolean took for another or moved onto a touch, the curve two of the
/// surfaces it now lies on share nearest it. Each of the two may stand up to
/// the tolerance from the edge's own surface, so the curve they share may
/// stand further than the tolerance from the edge: a rim of a bore moved onto
/// a side, its floor taken for one a hair off, stands √2 hairs from the
/// circle the moved wall and that floor share.
fn shared(
    operands: &Operands,
    operand: usize,
    edge: EdgeId,
    own: &Curve,
    around: &[SurfaceId],
) -> Curve {
    let body = operands.bodies[operand];
    let moved = body.uses(edge).into_iter().any(|(face, _)| {
        let lying = body.face(face).surface;
        operands.surfaces.list[operands.surface_of(operand, face).0 as usize]
            != *body.surface(lying)
    });
    if !moved {
        return *own;
    }
    let list = &operands.surfaces.list;
    let stretch = body.edge(edge);
    let middle = own.point((stretch.from + stretch.to) / 2.0);
    around
        .iter()
        .enumerate()
        .flat_map(|(index, one)| around[index + 1..].iter().map(move |other| [*one, *other]))
        .flat_map(|[one, other]| {
            relation(
                &list[one.0 as usize],
                &list[other.0 as usize],
                operands.scale_of([one, other]),
            )
            .curves()
        })
        .filter(|shared| within(shared, own, operands.scale, MOVED * operands.eps()))
        .min_by(|one, other| distance(one, middle).total_cmp(&distance(other, middle)))
        .unwrap_or(*own)
}

/// How far, in tolerances, the curve two surfaces share moves when each is
/// taken for another or moved onto a touch by up to the tolerance: √2 where
/// they stand square, as two planes of the origin or a cap and its wall do.
const MOVED: f64 = 2.0;

/// Whether a surface carries a curve all along it, within the tolerance: a
/// line on a cylinder runs along its axis, a circle on a plane or a cylinder
/// stands square to it and about its axis, the curve two cylinders meet along
/// runs round one of its own two — not round a wall of one radius a hair
/// beside one of them, which carries it only over the stretch an arc of it
/// was taken for (decision 6).
fn carries(surface: &Surface, curve: &Curve, scale: Scale) -> bool {
    let eps = scale.eps();
    let parallel =
        |one: DVec3, other: DVec3| one.cross(other).length() * 2.0 * scale.reach() <= eps;
    match (surface, curve) {
        (Surface::Plane(_), Curve::Line(_)) => true,
        (Surface::Plane(plane), Curve::Circle(circle)) => {
            parallel(circle.axis, plane.normal) && plane.distance(circle.center).abs() <= eps
        }
        (Surface::Cylinder(cylinder), Curve::Line(line)) => parallel(line.direction, cylinder.axis),
        (Surface::Cylinder(cylinder), Curve::Circle(circle)) => {
            let from = circle.center - cylinder.origin;
            parallel(circle.axis, cylinder.axis)
                && (from - cylinder.axis * from.dot(cylinder.axis)).length() <= eps
                && (circle.radius - cylinder.radius).abs() <= eps
        }
        (Surface::Cylinder(cylinder), Curve::Meet(meet)) => meet.own(cylinder, eps).is_some(),
        (Surface::Plane(_), Curve::Meet(_)) => false,
        (Surface::Cone(_), _) => false,
    }
}

/// Where an operand's corner stands in the arena: where the operand built
/// it, or, where a surface it lies on was taken for another or moved onto a
/// touch, where a line of the edges ending there now crosses a plane it lies
/// on, square enough to it to fix the place. Its edges are laid on the
/// curves the surfaces now share; left where it was built, the corner would
/// stand a hair off them, and off the place the next operation finds them
/// crossing at.
pub(super) fn laid(
    operands: &Operands,
    registry: &Registry,
    operand: usize,
    vertex: VertexId,
    curves: &[usize],
) -> DVec3 {
    let body = operands.bodies[operand];
    let own = body.vertex(vertex);
    let list = &operands.surfaces.list;
    let shared: Vec<SurfaceId> = own
        .on
        .iter()
        .map(|surface| operands.surfaces.mapped[operand][surface.0 as usize].0)
        .collect();
    let moved = own
        .on
        .iter()
        .zip(&shared)
        .any(|(surface, taken)| list[taken.0 as usize] != *body.surface(*surface));
    if !moved {
        return own.point;
    }
    let mut crossings = curves.iter().flat_map(|&curve| {
        let registered = &registry.list[curve];
        let Curve::Line(line) = registered.curve else {
            return Vec::new();
        };
        shared
            .iter()
            .filter(|surface| registered.support.binary_search(surface).is_err())
            .filter_map(|surface| match list[surface.0 as usize] {
                Surface::Plane(plane) if plane.normal.dot(line.direction).abs() >= SQUARE => {
                    let at = -plane.distance(line.origin) / plane.normal.dot(line.direction);
                    Some(line.point(at))
                }
                _ => None,
            })
            .collect()
    });
    crossings
        .find(|place| place.distance(own.point) <= MOVED * operands.eps())
        .unwrap_or(own.point)
}

/// How square to a plane a line must stand, in the cosine of their angle,
/// for the place it crosses the plane at to be fixed by the two.
const SQUARE: f64 = 0.1;

impl Held {
    /// Whether the edge covers the parameter `at` of its curve, `slack`
    /// beyond either end included; on a closed curve, `at` taken modulo its
    /// period. A ring covers every parameter: measured against its stretch,
    /// the middle of an arc running from a corner round to it again, a
    /// rounding past the stretch's end, would fall off it.
    pub fn covers(&self, at: f64, period: Option<f64>, slack: f64) -> bool {
        if self.whole {
            return true;
        }
        let [from, to] = self.stretch;
        let at = match period {
            Some(period) => at - period * ((at - (from - slack)) / period).floor(),
            None => at,
        };
        at >= from - slack && at <= to + slack
    }
}
