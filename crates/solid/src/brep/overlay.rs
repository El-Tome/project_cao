//! The arrangement at the heart of the boolean, step 5 of
//! `docs/exact-kernel.md`: arcs drawn in a surface's parameters cut it into
//! regions, each handed back with the cycles bounding it and a point well
//! inside it. Plain 2D: it knows nothing of bodies.

mod column;
mod partition;
mod piece;
mod star;

use glam::DVec2;

use super::Declined;
use super::trace::Trace;

/// An arc as the boolean cut it: a trace from one vertex to another, by rank,
/// or none for a closed arc with no vertex. Arcs meet only at shared
/// vertices, where they may touch, and may end at a vertex nothing else
/// reaches.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Arc {
    pub trace: Trace,
    pub ends: Option<[usize; 2]>,
}

/// A piece of the surface the arcs cut out: the cycles bounding it, each as
/// arcs run along their own way or against it with the region on their
/// left, every arc used once each way over all the regions; and a point
/// inside it, as far from its boundary as a vertical chord through it lets.
/// Unbounded on a plane for the region outside everything, on a cylinder
/// for those above and below everything.
#[derive(Clone, Debug, PartialEq)]
pub struct Region {
    pub cycles: Vec<Vec<(usize, bool)>>,
    pub inside: DVec2,
    pub unbounded: bool,
}

/// Every region the arcs cut a surface into, the unbounded ones included.
#[derive(Clone, Debug, PartialEq)]
pub struct Overlay {
    pub regions: Vec<Region>,
}

impl Overlay {
    /// The regions `arcs` cut a surface into, the vertices given by their
    /// parameters. `period` is that of the first parameter on a surface
    /// closing on itself, where positions are read modulo it and each trace
    /// is unwrapped, running past it rather than jumping back.
    pub fn of(vertices: &[DVec2], arcs: &[Arc], period: Option<f64>) -> Result<Overlay, Declined> {
        debug_assert!(
            arcs.iter()
                .filter_map(|arc| arc.ends)
                .flatten()
                .all(|vertex| vertex < vertices.len())
        );
        if arcs.is_empty() {
            return Ok(Overlay {
                regions: vec![Region {
                    cycles: Vec::new(),
                    inside: DVec2::ZERO,
                    unbounded: true,
                }],
            });
        }
        let cycles = star::cycles(arcs, vertices, period)?;
        let pieces = piece::pieces(arcs);
        let columns = column::columns(arcs, &pieces, period);
        let layout = partition::Layout {
            arcs,
            cycles: &cycles,
            pieces: &pieces,
            columns: &columns,
        };
        Ok(Overlay {
            regions: layout.regions()?,
        })
    }
}

#[cfg(test)]
mod tests;
