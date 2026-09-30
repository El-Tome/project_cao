//! The operands' own edges, laid on the registered curves: each edge's curve
//! registered with the surfaces of the faces beside it (decision 3), and the
//! stretch of the registered curve the edge covers.

use std::collections::BTreeMap;

use super::operands::Operands;
use crate::brep::canonical::Registry;
use crate::brep::topology::{EdgeId, VertexId};

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
            let curve = registry.register(*own, &operands.around(operand, edge));
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
