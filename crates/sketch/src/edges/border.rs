//! A stretch two curves of the drawing lie along, kept as the one border it
//! is.
//!
//! Two traits laid along each other are each cut at the other's ends, and come
//! out as two pieces joining the same two vertices. Kept both, they are a
//! sliver of no width the face walk cannot read: the two leave each end the
//! same way and bend alike, so nothing puts them in order there, and whichever
//! order they land in is the same at both ends where a map needs one to mirror
//! the other. The walk then steps across the sliver into the shape next door,
//! and comes back with one loop round both.
//!
//! A shared border is one border, as on a map, and every curve lying along it
//! names it. The curve first in the graph's order lays the edge; each one
//! after it adds its name to that edge instead of an edge of its own, so an
//! area keeps a name whichever of them is later erased.

use std::collections::HashMap;
use std::collections::hash_map::Entry;

use glam::DVec2;

use crate::ellipsing::EllipseDraft;
use crate::naming::CurveId;

use super::half_edge::Bend;
use super::off_by;

/// The pieces already in the graph, each as the vertices it joins and the
/// edge it was laid as.
#[derive(Default)]
pub(super) struct Borders {
    /// Lowest vertex first: a straight piece runs between its ends either
    /// way, and is the same stretch read from either.
    straight: HashMap<(usize, usize), usize>,
    /// In the order the piece runs: a piece of circle or a run of ellipse
    /// always turns counter-clockwise from its start to its end, and the one
    /// from the end back to the start is the rest of the curve.
    curved: HashMap<(usize, usize), Vec<(Bend, usize)>>,
}

impl Borders {
    /// The edge already joining a straight piece's two ends, or nothing when
    /// the piece is a border of its own, noted then as `edge`.
    pub(super) fn along_straight(
        &mut self,
        start: usize,
        end: usize,
        edge: usize,
    ) -> Option<usize> {
        match self.straight.entry((start.min(end), start.max(end))) {
            Entry::Occupied(known) => Some(*known.get()),
            Entry::Vacant(spot) => {
                spot.insert(edge);
                None
            }
        }
    }

    /// The same for a curved piece: one already joining the same two ends in
    /// the same order along the same curve is that border.
    ///
    /// A straight piece and a curved one joining the same two vertices are two
    /// borders, a lens between them, and are never held against each other.
    pub(super) fn along_curved(
        &mut self,
        bend: Bend,
        (start, end): (usize, usize),
        places: &[DVec2],
        edge: usize,
    ) -> Option<usize> {
        let through = places[start];
        let along = self.curved.entry((start, end)).or_default();
        if let Some((_, known)) = along
            .iter()
            .find(|(known, _)| is_the_curve((*known, through), (bend, through)))
        {
            return Some(*known);
        }
        along.push((bend, edge));
        None
    }
}

/// The circles and ellipses nothing cut, each as what it bends along and a
/// place it runs through.
///
/// A whole loop has no vertex, so a second one drawn on the first never meets
/// a piece to be held against: the same curve drawn twice is caught here
/// instead.
#[derive(Default)]
pub(super) struct Loops(Vec<(Bend, DVec2)>);

impl Loops {
    /// Which loop already noted a whole loop is the same curve as, or nothing
    /// when it is a curve of its own, noted then after the others.
    pub(super) fn along(&mut self, bend: Bend, through: DVec2) -> Option<usize> {
        if let Some(known) = self
            .0
            .iter()
            .position(|known| is_the_curve(*known, (bend, through)))
        {
            return Some(known);
        }
        self.0.push((bend, through));
        None
    }
}

/// Adds a curve to the names a border answers to, kept in order and once
/// each, so that the same border reads alike from every area it bounds.
pub(super) fn also_named(names: &mut Vec<CurveId>, curve: CurveId) {
    if let Err(at) = names.binary_search(&curve) {
        names.insert(at, curve);
    }
}

/// The same for an edge of the graph, on both its halves: `edge` is the
/// first of the two, its twin next to it.
pub(super) fn edge_named_too(from: &mut [Vec<CurveId>], edge: usize, curve: CurveId) {
    for half in [edge, edge + 1] {
        also_named(&mut from[half], curve);
    }
}

/// Whether two curves, each given as what it bends along and a place it runs
/// through, are one and the same.
///
/// An ellipse is held against anything by its own reckoning: the same one has
/// several descriptions, either axis first and either way round, and only
/// [`EllipseDraft::is_the_curve`] reads them as one. Two circles are the same
/// when their centres and their radii are each as near as the graph needs two
/// places to be to make them one.
fn is_the_curve(one: (Bend, DVec2), other: (Bend, DVec2)) -> bool {
    match (one, other) {
        ((Bend::Round(near), here), (Bend::Round(far), there)) => {
            near.distance(far) <= off_by(near)
                && (here.distance(near) - there.distance(far)).abs() <= off_by(here)
        }
        _ => ellipse_of(one).is_the_curve(&ellipse_of(other)),
    }
}

/// The ellipse a curve runs along: a circle being one whose two axes both
/// reach its radius.
fn ellipse_of((bend, through): (Bend, DVec2)) -> EllipseDraft {
    match bend {
        Bend::Oval(drawn) => drawn,
        Bend::Round(centre) => EllipseDraft::round(centre, through.distance(centre)),
    }
}

#[cfg(test)]
mod tests;
