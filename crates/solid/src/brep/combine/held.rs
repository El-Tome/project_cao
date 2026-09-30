//! The operands' own edges, laid on the registered curves: each edge's curve
//! registered with the surfaces of the faces beside it (decision 3), and the
//! stretch of the registered curve the edge covers.
//!
//! An edge beside a face whose surface was taken for one of the first
//! operand's lies on that surface only within the tolerance: its curve is
//! registered as the surfaces it now lies on share it, or a hair left off
//! them would be a hair more off at every later operation, and two corners
//! found on it and on its neighbour, one each way, would no longer be one.

use std::collections::BTreeMap;

use super::operands::Operands;
use crate::brep::canonical::{Registry, same};
use crate::brep::curve::Curve;
use crate::brep::relation::relation;
use crate::brep::topology::{EdgeId, SurfaceId, VertexId};

/// An edge of an operand, on a registered curve, over a stretch of that
/// curve's parameter.
#[derive(Clone, Debug, PartialEq)]
pub(in crate::brep) struct Held {
    pub operand: usize,
    pub edge: EdgeId,
    pub curve: usize,
    pub stretch: [f64; 2],
}

/// For each operand's vertex, by the operand and the vertex, the registered
/// curves of the edges ending there.
pub(super) type Ending = BTreeMap<(usize, VertexId), Vec<usize>>;

/// Every edge of the first operand, then every edge of the second, and where
/// each ends.
pub(super) fn held(operands: &Operands, registry: &mut Registry) -> (Vec<Held>, Ending) {
    let mut found = Vec::new();
    let mut ending = BTreeMap::new();
    for (operand, body) in operands.bodies.iter().enumerate() {
        for edge in body.edge_ids() {
            let stretch = body.edge(edge);
            let own = body.curve(stretch.curve);
            let around = operands.around(operand, edge);
            let curve = registry.register(shared(operands, operand, edge, own, &around), &around);
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
            });
        }
    }
    (found, ending)
}

/// An edge's own curve, or, when a face beside it lies on a surface the
/// boolean took for another, the curve two of the surfaces it now lies on
/// share within the tolerance of it.
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
    around
        .iter()
        .enumerate()
        .flat_map(|(index, one)| around[index + 1..].iter().map(move |other| [*one, *other]))
        .flat_map(|[one, other]| {
            relation(
                &list[one.0 as usize],
                &list[other.0 as usize],
                operands.scale,
            )
            .curves()
        })
        .find(|shared| same(shared, own, operands.scale))
        .unwrap_or(*own)
}

impl Held {
    /// Whether the edge covers the parameter `at` of its curve, `slack`
    /// beyond either end included; on a closed curve, `at` taken modulo its
    /// period.
    pub fn covers(&self, at: f64, period: Option<f64>, slack: f64) -> bool {
        let [from, to] = self.stretch;
        let at = match period {
            Some(period) => at - period * ((at - (from - slack)) / period).floor(),
            None => at,
        };
        at >= from - slack && at <= to + slack
    }
}
