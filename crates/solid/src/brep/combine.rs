//! Joining and cutting: the boolean of `docs/exact-kernel.md`. Both operands
//! are laid on one arena — the surfaces they share, the curves where those
//! surfaces meet, the corners where the curves end — cut into arcs; each
//! surface's arcs part it into regions, and a region is kept as a face where
//! the operation says something different on its two sides.

mod band;
mod clear;
mod crossed;
mod cut;
mod held;
mod identified;
mod operands;
mod parted;
mod related;
mod slid;

use glam::DVec3;

use super::Declined;
use super::assembly::assembled;
use super::canonical::{Apart, Place, Planes, Pool, Registry, closed, flush};
use super::selection::selected;
use super::topology::{Body, SurfaceId};
pub(super) use operands::Operands;
use related::Special;
use slid::slid;

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

/// Both operands laid on each other: the shared surfaces, every curve, every
/// corner, and the arcs kept as edges, each with the surfaces it lies on; no
/// face yet.
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
    let scale = first.scale().joined(second.scale());
    let closed = closed(first, second, scale);
    let second = closed.as_ref().unwrap_or(second);
    let flush = flush(first, second, scale);
    let second = flush.as_ref().unwrap_or(second);
    let slid = slid(first, second, scale);
    let operands = Operands::of(first, slid.as_ref().unwrap_or(second), scale);
    let arena = laid(&operands)?;
    let faces = selected(&operands, &arena, operation)?;
    assembled(arena, faces)
}

/// The arena of two operands, decided once: surfaces, relations, curves,
/// corners and arcs.
pub(super) fn laid(operands: &Operands) -> Result<Arena, Declined> {
    let list = &operands.surfaces.list;
    let apart = Apart::of(list, |pair| operands.scale_of(pair));
    let mut registry = Registry::new(operands.scale, apart, Planes::of(list));
    let (held, ending) = held::held(operands, &mut registry);
    let (special, done) = related::related(operands, &mut registry)?;
    let found = crossed::crossed(operands, &mut registry, &held)?;
    related::completed(operands, &mut registry, &done);
    let candidates = candidates(operands, &registry, &ending, &special, &found);
    let places: Vec<Place> = candidates
        .iter()
        .map(|(point, surfaces, curves)| {
            let mut on = surfaces.clone();
            for curve in curves {
                on.extend(registry.list[*curve].support.iter().copied());
            }
            Place {
                point: *point,
                on,
                along: curves.clone(),
            }
        })
        .collect();
    registry.planes = std::mem::take(&mut registry.planes).grazing(
        |pair| operands.meeting(pair),
        &places,
        operands.eps(),
    );
    let mut pool = Pool::new(operands.eps());
    for (point, surfaces, curves) in candidates {
        pool.add(point, surfaces, curves, &registry);
    }
    let bands = band::laid_out(operands, &mut registry, &mut pool)?;
    cut::cut(operands, &registry, &pool, &held, &bands)
}

/// Every corner candidate, the surfaces it was found on and the curves it
/// was found along, in the fixed order they are merged in — the first
/// operand's vertices, the second's, the relations' special points, the
/// crossings three planes fix, the other crossings. A place three planes
/// fix is exact, and the crossings a curve makes nearby are merged into it
/// rather than into each other: a post's rim crossing the two sides of a
/// box whose corner line stands a hair inside the wall crosses them past
/// the tolerance apart, each within it of the corner.
fn candidates(
    operands: &Operands,
    registry: &Registry,
    ending: &held::Ending,
    special: &[Special],
    found: &[crossed::Found],
) -> Vec<(DVec3, Vec<SurfaceId>, Vec<usize>)> {
    let mut candidates = Vec::new();
    for (operand, body) in operands.bodies.iter().enumerate() {
        for id in body.vertex_ids() {
            let vertex = body.vertex(id);
            let surfaces = vertex
                .on
                .iter()
                .map(|own| operands.surfaces.mapped[operand][own.0 as usize].0)
                .collect();
            let curves = ending.get(&(operand, id)).cloned().unwrap_or_default();
            let point = held::laid(operands, registry, operand, id, &curves);
            candidates.push((point, surfaces, curves));
        }
    }
    for point in special {
        candidates.push((point.point, point.surfaces.to_vec(), point.curves.to_vec()));
    }
    let fixed = |crossing: &&crossed::Found| {
        let mut on = registry.list[crossing.curve].support.clone();
        on.push(crossing.surface);
        on.sort();
        registry.planes.fix_a_place(&on, &on)
    };
    let (by_planes, others): (Vec<&crossed::Found>, Vec<&crossed::Found>) =
        found.iter().partition(fixed);
    for crossing in by_planes.into_iter().chain(others) {
        let mut surfaces = crossing.partly.clone();
        surfaces.push(crossing.surface);
        candidates.push((crossing.point, surfaces, vec![crossing.curve]));
    }
    candidates
}

#[cfg(test)]
mod tests;
