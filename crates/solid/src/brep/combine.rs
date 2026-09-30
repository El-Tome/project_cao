//! Joining and cutting: the boolean of `docs/exact-kernel.md`. Both operands
//! are laid on one arena — the surfaces they share, the curves where those
//! surfaces meet, the corners where the curves end — cut into arcs; each
//! surface's arcs part it into regions, and a region is kept as a face where
//! the operation says something different on its two sides.

mod crossed;
mod cut;
mod held;
mod operands;
mod related;

use super::Declined;
use super::assembly::assembled;
use super::canonical::{Apart, Planes, Pool, Registry};
use super::selection::selected;
use super::topology::{Body, SurfaceId};
pub(super) use operands::Operands;
use related::Special;

/// What is kept of the two operands' matter.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum Operation {
    /// Everything in either.
    Or,
    /// Everything in the first and not in the second.
    AndNot,
}

impl Operation {
    pub fn holds(self, first: bool, second: bool) -> bool {
        match self {
            Operation::Or => first || second,
            Operation::AndNot => first && !second,
        }
    }
}

/// Both operands laid on each other: the shared surfaces, every curve with
/// the surfaces it lies on, every corner, and the arcs kept as edges; no face
/// yet.
pub(super) struct Arena {
    pub body: Body,
    pub supports: Vec<Vec<SurfaceId>>,
}

impl Body {
    /// Everything in either body.
    pub fn joined(&self, other: &Body) -> Result<Body, Declined> {
        combine(self, other, Operation::Or)
    }

    /// Everything in this body and not in the tool.
    pub fn cut_by(&self, tool: &Body) -> Result<Body, Declined> {
        combine(self, tool, Operation::AndNot)
    }
}

pub(super) fn combine(first: &Body, second: &Body, operation: Operation) -> Result<Body, Declined> {
    let operands = Operands::of(first, second, first.scale().joined(second.scale()));
    let arena = laid(&operands)?;
    let faces = selected(&operands, &arena, operation)?;
    assembled(arena, faces)
}

/// The arena of two operands, decided once: surfaces, relations, curves,
/// corners and arcs.
pub(super) fn laid(operands: &Operands) -> Result<Arena, Declined> {
    let list = &operands.surfaces.list;
    let apart = Apart::of(list, operands.scale);
    let mut registry = Registry::new(operands.scale, apart, Planes::of(list));
    let (held, ending) = held::held(operands, &mut registry);
    let (special, done) = related::related(operands, &mut registry)?;
    let found = crossed::crossed(operands, &mut registry, &held)?;
    related::completed(operands, &mut registry, &done);
    let pool = pooled(operands, &registry, &ending, &special, &found);
    cut::cut(operands, &registry, &pool, &held)
}

/// Every corner candidate in a fixed order — the first operand's vertices,
/// the second's, the relations' special points, the crossings — merged.
fn pooled(
    operands: &Operands,
    registry: &Registry,
    ending: &held::Ending,
    special: &[Special],
    found: &[crossed::Found],
) -> Pool {
    let mut pool = Pool::new(operands.eps());
    for (operand, body) in operands.bodies.iter().enumerate() {
        for id in body.vertex_ids() {
            let vertex = body.vertex(id);
            let surfaces = vertex
                .on
                .iter()
                .map(|own| operands.surfaces.mapped[operand][own.0 as usize].0);
            let curves = ending.get(&(operand, id)).cloned().unwrap_or_default();
            pool.add(vertex.point, surfaces, curves, registry);
        }
    }
    for point in special {
        pool.add(
            point.point,
            point.surfaces,
            point.curves.iter().copied(),
            registry,
        );
    }
    for crossing in found {
        pool.add(
            crossing.point,
            [crossing.surface],
            [crossing.curve],
            registry,
        );
    }
    pool
}

#[cfg(test)]
mod tests;
