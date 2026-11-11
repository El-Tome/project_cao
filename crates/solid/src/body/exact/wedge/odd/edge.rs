//! The way round the odd edge of a corner of mixed sense by the edge of the
//! face it lays (#829): the odd edge laid first, with every edge closing a
//! corner of one sense with it, then the others with the edge where its face
//! meets the face they share — a chamfer's flat's foot, or a fill's top —
//! and, where one of the two others is left sharp (#846), the flat along that
//! edge held in the air on the plane square to the sharp edge, its end a face
//! of its own.

use std::collections::BTreeSet;

use glam::DVec3;

use super::{Odd, alike, ranked_patches};
use crate::body::exact::Exact;
use crate::body::exact::wedge::flush::Held;
use crate::body::exact::wedge::overlap::{ENDS, STRIDE};
use crate::body::exact::wedge::step::{Kind, Step};
use crate::brep::{Declined, EdgeId, EdgeName};

/// The ranks of the edges of a step laid first, ascending: each odd edge of
/// `odd`, and every edge closing a corner of three of one sense, as [`alike`]
/// reads them, with one laid first — the top edges of a standing block whose
/// uprights are odd at their feet, their top corners closed among them as
/// #699 closes one, before the feet go round the uprights.
fn laid_first(step: &Step, odd: &[Odd]) -> Vec<usize> {
    let mut first: BTreeSet<usize> = odd.iter().map(|corner| corner.lone).collect();
    let closing = alike(step);
    loop {
        let before = first.len();
        for ranks in &closing {
            if ranks.iter().any(|rank| first.contains(rank)) {
                first.extend(ranks);
            }
        }
        if first.len() == before {
            return first.into_iter().collect();
        }
    }
}

/// What a list of edges laid in turn leaves: the body, and the corners three
/// of its edges meet at, by their ranks in the list, in the order their
/// closing faces are numbered.
type Laid = (Exact, Vec<[usize; 3]>);

impl Exact {
    /// The body with the edges `names` find laid at `taken`, each corner of
    /// mixed sense `odd` holds gone round its odd edge: the odd edges first,
    /// with every edge closing a corner of one sense with them, as
    /// [`laid_first`] reads them on `step`, what the names find on this body;
    /// then the others — each followed onto what that left, as
    /// [`Exact::following`] finds it — with, for each such corner, the edge
    /// where the face its odd edge laid meets the face the two others share,
    /// as [`Exact::in_turn`] lays each of the two lists with `kind`. The face
    /// laid on each named
    /// edge is numbered where the steps' numbers ended plus its rank in the
    /// list, and every patch past them, in the order of its corner's ranks
    /// in the list, as [`ranked_patches`] numbers them: the flat laid on each
    /// edge no name finds, at its corner of mixed sense — the fill along a
    /// flat's foot, the cut along a fill's top — and the face closing
    /// a corner three edges of either list meet at — the apex of a pyramid
    /// whose slanted edges are laid first. Declined `Unsupported` where
    /// either list gives less than `taken`, as [`Step::bound`] reads it on
    /// the body it is laid on, an edge cannot be followed, the odd edge's
    /// face meets the face its two others share along no edge or several, or
    /// an edge no name finds is of a corner the second list closes; otherwise
    /// as [`Exact::cut_off`] declines either list.
    pub(super) fn round_the_edge(
        &self,
        names: &[EdgeName],
        step: &Step,
        odd: &[Odd],
        taken: f64,
        kind: &Kind,
    ) -> Result<(Exact, f64), Declined> {
        let start = self.next;
        let wedges = &step.wedges;
        let first = laid_first(step, odd);
        let rest: Vec<usize> = (0..names.len())
            .filter(|rank| !first.contains(rank))
            .collect();
        let named = |ranks: &[usize]| -> Vec<EdgeName> {
            ranks.iter().map(|&rank| names[rank].clone()).collect()
        };
        let (laid, closed_first) = self.laid_at(&named(&first), taken, kind, &[])?;
        let mut held = Vec::new();
        let mut edges_on = Vec::new();
        for corner in odd {
            let at = first
                .binary_search(&corner.lone)
                .map_err(|_| Declined::Unsupported)?;
            let (name, edge) = laid.edge_on(start + at as u32, &corner.on)?;
            if let Some(away) = corner.sharp {
                held.push(laid.held_where(edge, corner.at, away)?);
            }
            edges_on.push(name);
        }
        let followed = (rest.iter())
            .map(|&rank| laid.following(&names[rank], &wedges[rank]))
            .collect::<Result<Vec<_>, Declined>>()?;
        let following: Vec<EdgeName> = followed.into_iter().chain(edges_on).collect();
        let (round, closed_after) = laid.laid_at(&following, taken, kind, &held)?;
        let back = |ranks: &[usize], corner: [usize; 3]| -> Result<[usize; 3], Declined> {
            let mut back = [0; 3];
            for (into, rank) in back.iter_mut().zip(corner) {
                *into = *ranks.get(rank).ok_or(Declined::Unsupported)?;
            }
            back.sort();
            Ok(back)
        };
        let after_first = start + first.len() as u32;
        let after_rest = laid.next + rest.len() as u32;
        let mut patches = Vec::new();
        for (index, &corner) in closed_first.iter().enumerate() {
            patches.push((back(&first, corner)?, after_first + index as u32));
        }
        for (index, corner) in odd.iter().enumerate() {
            patches.push((corner.ranks, after_rest + index as u32));
        }
        for (index, &corner) in closed_after.iter().enumerate() {
            let number = laid.next + following.len() as u32 + index as u32;
            patches.push((back(&rest, corner)?, number));
        }
        let mut moved = ranked_patches(patches, start + names.len() as u32);
        let laid_on = (first.iter().map(|&rank| (start, rank)))
            .enumerate()
            .chain(rest.iter().map(|&rank| (laid.next, rank)).enumerate());
        for (index, (from, rank)) in laid_on {
            moved.insert(from + index as u32, start + rank as u32);
        }
        let ends = start + names.len() as u32 + (closed_first.len() + odd.len()) as u32;
        let ends = ends + closed_after.len() as u32;
        let renamed = round.renamed_by(|number| match moved.get(&number) {
            Some(&moved) => moved,
            None if number >= ENDS && number < ENDS + held.len() as u32 * STRIDE => {
                ends + (number - ENDS) / STRIDE
            }
            None => number,
        });
        let next = ends + held.len() as u32;
        Ok((Exact { next, ..renamed }, taken))
    }

    /// Where the fill along the edge `edge` of this body ends at the wall of
    /// an edge left sharp at the corner `at`, running from it `away`: held on
    /// the plane square to that edge through the end of `edge` on its line,
    /// as [`Held`] reads it. Declined `Unsupported` where neither end of
    /// `edge` stands on that line.
    fn held_where(&self, edge: EdgeId, at: DVec3, away: DVec3) -> Result<Held, Declined> {
        let eps = self.brep.scale().eps();
        let ends = self.brep.edge(edge).ends.into_iter().flatten();
        (ends.map(|end| self.brep.vertex(end).point))
            .find(|&point| (point - at).cross(away).length() <= eps)
            .map(|point| Held { at: point, away })
            .ok_or(Declined::Unsupported)
    }

    /// The body with the edges `names` find on it laid in turn at `taken`, as
    /// [`Exact::in_turn`] lays them with `kind`, and the corners three of them
    /// meet at, by their ranks, in the order their closing faces are
    /// numbered. Declined `Unsupported` where they give less than `taken`, as
    /// [`Step::bound`] reads it on this body, by more than the body's
    /// tolerance — a limit the value was taken at, read again through a
    /// mitre, comes back a rounding short of it; otherwise as
    /// [`Exact::cut_off`] declines them.
    fn laid_at(
        &self,
        names: &[EdgeName],
        taken: f64,
        kind: &Kind,
        held: &[Held],
    ) -> Result<Laid, Declined> {
        let mut step = self.step(names, kind)?;
        step.held = held.to_vec();
        if taken - step.bound(&self.brep, taken, kind) > self.brep.scale().eps() {
            return Err(Declined::Unsupported);
        }
        let closed = step.three.iter().map(|&(_, ranks, _)| ranks).collect();
        let (laid, _, _) = self.in_turn(names, step, taken, kind)?;
        Ok((laid, closed))
    }

    /// The name of the one edge where the face laid answering to `laid`
    /// meets the face answering to `on` alone. Declined `Unsupported` where
    /// they meet along no edge, or several.
    fn edge_on(&self, laid: u32, on: &[u32]) -> Result<(EdgeName, EdgeId), Declined> {
        let mut edges = (self.named(laid as usize))
            .flat_map(|face| self.brep.face(face).loops.iter().flatten())
            .map(|coedge| coedge.edge)
            .filter(|&edge| {
                (self.brep.uses(edge).iter()).any(|&(face, _)| self.brep.numbers(face) == on)
            });
        match (edges.next(), edges.next()) {
            (Some(edge), None) => Ok((self.edges[edge.0 as usize].clone(), edge)),
            _ => Err(Declined::Unsupported),
        }
    }
}
