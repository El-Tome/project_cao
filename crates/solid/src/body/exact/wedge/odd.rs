//! A corner three edges of one step meet at, two of one sense and the odd one
//! of the other (#829): the foot corner of a block standing on another, two
//! concave feet and the convex upright above them. A cut and a fill laid in
//! turn there meet in nothing, so the two others go round the outline the
//! odd edge leaves, as the kind says. Among flats the odd edge is laid first,
//! as it is laid alone, stopping flush on the face the two others share; the
//! two others then follow the outline it left, as they would in a later
//! step, the edge where the odd edge's face meets that face laid with them —
//! the foot of a chamfer's flat, filled among the feet and mitred with them,
//! as #801 fills it. Among rounds the corner is filled up to where the odd
//! edge's round leaves the others, every edge is then laid against that as
//! alone, and a piece of torus replaces what the filling left, as
//! [`super::torus`] lays it. Either way each named edge's face is numbered
//! by its rank, and every patch — a fill no name finds, a piece of torus, a
//! corner closed on the way — past them by its corner's ranks. A foot corner
//! whose upright's other end meets an edge of the list laid after it is
//! declined: slice 3 of #829 builds a standing block's top corner, #847 a
//! post whose upright is odd at both ends.

use std::collections::BTreeMap;

use glam::DVec3;

use super::Wedge;
use super::pairs::Pair;
use super::step::{Kind, Step};
use crate::body::exact::Exact;
use crate::brep::{self, Curve, Declined, EdgeName};

/// A corner of mixed sense: where it stands, its three edges' ranks in the
/// list, from the lowest, the rank of its odd edge and the way that edge runs
/// from it, and the numbers of the face its two others share, which the odd
/// edge stops flush on there.
pub(super) struct Odd {
    pub at: DVec3,
    pub ranks: [usize; 3],
    pub lone: usize,
    pub up: DVec3,
    pub on: Vec<u32>,
}

/// What a kind lays round the odd edge of a corner of mixed sense.
#[derive(Clone, Copy)]
pub(in crate::body::exact) enum Around {
    /// The odd edge laid first, alone, then the others with the foot of the
    /// face it laid, an edge of their sense: a chamfer's flat, whose foot is
    /// straight.
    Foot,
    /// The corner filled up to the planes through the odd edge's round's axis
    /// square to the others, every edge then laid against it, and what the
    /// filling left replaced by a piece of torus about that axis, as
    /// [`Exact::through_a_torus`] lays it: a fillet's round, whose foot is
    /// curved.
    Torus,
}

/// The corners of mixed sense among those three of a step's edges meet at,
/// on `body`, in the order [`super::corners::of_three`] reads them. Declined
/// `Unsupported` where the two others at such a corner share no face, an
/// edge is the odd one at one corner and one of the two others at another,
/// or an odd edge meets at its other end an edge of the list that is odd at
/// no corner — a standing block's top corner, which slice 3 of #829 builds,
/// or the far feet of a post odd at both ends, which #847 builds. Odd edges
/// meeting each other there, the slanted edges of a pyramid at its apex, are
/// laid together first.
pub(super) fn corners(body: &brep::Body, step: &Step) -> Result<Vec<Odd>, Declined> {
    let mut odd: Vec<Odd> = Vec::new();
    for &(at, ranks, along) in &step.three {
        let inwards = ranks.map(|rank| step.wedges[rank].inwards);
        let alike = |at: usize| inwards.iter().filter(|&&way| way == inwards[at]).count();
        let Some(lone) = (0..3).find(|&at| alike(at) == 1) else {
            continue;
        };
        let [one, other] = [1, 2].map(|by| &step.wedges[ranks[(lone + by) % 3]]);
        let shared = (0..2)
            .find(|&side| other.faces.contains(&one.faces[side]))
            .ok_or(Declined::Unsupported)?;
        odd.push(Odd {
            at,
            ranks,
            lone: ranks[lone],
            up: along[lone],
            on: one.numbers[shared].clone(),
        });
    }
    let lone = |rank: usize| odd.iter().any(|corner| corner.lone == rank);
    let crossed = odd.iter().any(|corner| {
        (odd.iter()).any(|other| other.lone != corner.lone && other.ranks.contains(&corner.lone))
    });
    let topped = odd.iter().any(|corner| {
        (0..step.edges.len())
            .filter(|&rank| !lone(rank) && !corner.ranks.contains(&rank))
            .any(|rank| super::corners::meet(body, [step.edges[corner.lone], step.edges[rank]]))
    });
    if crossed || topped {
        return Err(Declined::Unsupported);
    }
    Ok(odd)
}

/// What a list of edges laid in turn leaves: the body, and the corners three
/// of its edges meet at, by their ranks in the list, in the order their
/// closing faces are numbered.
type Laid = (Exact, Vec<[usize; 3]>);

/// The numbers the faces closing a step's corners take, each corner by its
/// three edges' ranks, from the lowest, and the number its face answers to:
/// `first` and on, in the order of those ranks, as #699 numbers its patches.
pub(super) fn ranked_patches(
    mut patches: Vec<([usize; 3], u32)>,
    first: u32,
) -> BTreeMap<u32, u32> {
    patches.sort_by_key(|&(ranks, _)| ranks);
    (patches.into_iter().enumerate())
        .map(|(place, (_, number))| (number, first + place as u32))
        .collect()
}

impl Exact {
    /// The body with the edges `names` find laid at `taken`, each corner of
    /// mixed sense `odd` holds gone round its odd edge as the kind's
    /// `meeting` says: by its foot, as [`Exact::round_the_foot`] lays it, or
    /// through a piece of torus, as [`Exact::through_a_torus`] lays it, each
    /// edge's wedge on this body by its rank in `wedges`. The pairs laid as
    /// one are handed back through a piece of torus alone, where the fillet
    /// reads them; the chamfer, which goes round by the foot, reads none, and
    /// none is handed back that way. Declined as those decline.
    pub(super) fn round_the_odd(
        &self,
        names: &[EdgeName],
        wedges: &[Wedge],
        odd: &[Odd],
        taken: f64,
        kind: &Kind,
    ) -> Result<(Exact, f64, Vec<Pair>), Declined> {
        match kind.meeting.around {
            Around::Foot => (self.round_the_foot(names, wedges, odd, taken, kind))
                .map(|(round, taken)| (round, taken, Vec::new())),
            Around::Torus => self.through_a_torus(names, wedges, odd, taken, kind),
        }
    }

    /// The body with the edges `names` find laid at `taken`, each corner of
    /// mixed sense `odd` holds gone round its odd edge: the odd edges first,
    /// alone, then the others — each followed onto what the odd edges left,
    /// as [`Exact::following`] finds it, its wedge on this body by its rank in
    /// `wedges` — with, for each such corner, the edge where the face its odd
    /// edge laid meets the face the two others share, as [`Exact::in_turn`]
    /// lays each of the two lists with `kind`. The face laid on each named
    /// edge is numbered where the steps' numbers ended plus its rank in the
    /// list, and every patch past them, in the order of its corner's ranks
    /// in the list, as [`ranked_patches`] numbers them: the fill laid on each
    /// edge no name finds, at its corner of mixed sense, and the face closing
    /// a corner three edges of either list meet at — the apex of a pyramid
    /// whose slanted edges are laid first. Declined `Unsupported` where
    /// either list gives less than `taken`, as [`Step::bound`] reads it on
    /// the body it is laid on, an edge cannot be followed, the odd edge's
    /// face meets the face its two others share along no edge or several, or
    /// an edge no name finds is of a corner the second list closes; otherwise
    /// as [`Exact::cut_off`] declines either list.
    fn round_the_foot(
        &self,
        names: &[EdgeName],
        wedges: &[Wedge],
        odd: &[Odd],
        taken: f64,
        kind: &Kind,
    ) -> Result<(Exact, f64), Declined> {
        let start = self.next;
        let (first, rest): (Vec<usize>, Vec<usize>) =
            (0..names.len()).partition(|&rank| odd.iter().any(|corner| corner.lone == rank));
        let named = |ranks: &[usize]| -> Vec<EdgeName> {
            ranks.iter().map(|&rank| names[rank].clone()).collect()
        };
        let (laid, closed_first) = self.laid_at(&named(&first), taken, kind)?;
        let feet = (odd.iter())
            .map(|corner| {
                let at = first
                    .binary_search(&corner.lone)
                    .map_err(|_| Declined::Unsupported)?;
                laid.foot(start + at as u32, &corner.on)
            })
            .collect::<Result<Vec<_>, Declined>>()?;
        let followed = (rest.iter())
            .map(|&rank| laid.following(&names[rank], &wedges[rank]))
            .collect::<Result<Vec<_>, Declined>>()?;
        let following: Vec<EdgeName> = followed.into_iter().chain(feet).collect();
        let (round, closed_after) = laid.laid_at(&following, taken, kind)?;
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
        let renamed = round.renamed_by(|number| moved.get(&number).copied().unwrap_or(number));
        Ok((renamed, taken))
    }

    /// The body with the edges `names` find on it laid in turn at `taken`, as
    /// [`Exact::in_turn`] lays them with `kind`, and the corners three of them
    /// meet at, by their ranks, in the order their closing faces are
    /// numbered. Declined `Unsupported` where they give less than `taken`, as
    /// [`Step::bound`] reads it on this body, by more than the body's
    /// tolerance — a limit the value was taken at, read again through a
    /// mitre, comes back a rounding short of it; otherwise as
    /// [`Exact::cut_off`] declines them.
    fn laid_at(&self, names: &[EdgeName], taken: f64, kind: &Kind) -> Result<Laid, Declined> {
        let step = self.step(names, kind)?;
        if taken - step.bound(&self.brep, taken, kind) > self.brep.scale().eps() {
            return Err(Declined::Unsupported);
        }
        let closed = step.three.iter().map(|&(_, ranks, _)| ranks).collect();
        let (laid, _, _) = self.in_turn(names, step, taken, kind)?;
        Ok((laid, closed))
    }

    /// The name on this body of the edge `name` found as `wedge` on a body
    /// laid on since, which may have moved both its ends: the one straight
    /// edge between the sides the name records, its every end on the line
    /// `wedge` runs along. Declined `Unsupported` where no edge or several lie
    /// there.
    pub(super) fn following(&self, name: &EdgeName, wedge: &Wedge) -> Result<EdgeName, Declined> {
        let eps = self.brep.scale().eps();
        let [one, other] = &name.between;
        let on_line = |at: DVec3| (at - wedge.point).cross(wedge.along).length() <= eps;
        let mut found = self
            .brep
            .edge_ids()
            .zip(&self.edges)
            .filter(|&(edge, found)| {
                let stretch = self.brep.edge(edge);
                let ends = stretch.ends.into_iter().flatten();
                (found.between == [one.clone(), other.clone()]
                    || found.between == [other.clone(), one.clone()])
                    && matches!(self.brep.curve(stretch.curve), Curve::Line(_))
                    && ends.map(|end| self.brep.vertex(end).point).all(on_line)
            });
        match (found.next(), found.next()) {
            (Some((_, found)), None) => Ok(found.clone()),
            _ => Err(Declined::Unsupported),
        }
    }

    /// The name of the one edge where the face laid answering to `laid`
    /// meets the face answering to `on` alone. Declined `Unsupported` where
    /// they meet along no edge, or several.
    fn foot(&self, laid: u32, on: &[u32]) -> Result<EdgeName, Declined> {
        let mut feet = (self.named(laid as usize))
            .flat_map(|face| self.brep.face(face).loops.iter().flatten())
            .map(|coedge| coedge.edge)
            .filter(|&edge| {
                (self.brep.uses(edge).iter()).any(|&(face, _)| self.brep.numbers(face) == on)
            });
        match (feet.next(), feet.next()) {
            (Some(edge), None) => Ok(self.edges[edge.0 as usize].clone()),
            _ => Err(Declined::Unsupported),
        }
    }

    /// The body with each face answering to what `moved` makes of its
    /// numbers, in whatever order it leaves them, and each piece parted
    /// from what it makes of the number it was parted from.
    pub(super) fn renamed_by(self, moved: impl Fn(u32) -> u32) -> Exact {
        let Exact {
            brep,
            next,
            pieces,
            roots,
            drawn,
            ..
        } = self;
        let roots = (roots.into_iter())
            .map(|(piece, replaced)| (piece, moved(replaced)))
            .collect();
        let renamed = Exact::of(brep.renamed_by(&moved), next, pieces, roots);
        if let Some(drawn) = drawn.into_inner() {
            let _ = renamed.drawn.set(drawn);
        }
        renamed
    }
}
