//! A corner three edges of one step meet at, two of one sense and the odd one
//! of the other (#829), named or one of the two left out and left sharp
//! (#846): the foot corner of a block standing on another, two
//! concave feet and the convex upright above them, and its mirror, the top
//! of an L's inner corner, two convex top edges and the concave upright below
//! them. A cut and a fill laid in turn there meet in nothing, so the two
//! others go round the outline the odd edge leaves, as the kind says. Among
//! flats the odd edge is laid first, as it is laid alone, stopping flush on
//! the face the two others share; the two others then follow the outline it
//! left, as they would in a later step, the edge where the odd edge's face
//! meets that face laid with them — the foot of a chamfer's flat, filled
//! among the feet, or the top of a fill's flat, cut among the top edges —
//! and mitred with them, as #801 lays it. Among rounds a foot corner is
//! filled up to where the upright's round leaves the feet, every edge is then
//! laid against that as alone, and a piece of torus replaces what the filling
//! left, as [`super::torus`] lays it; the top of an L's inner corner, the
//! mirror, is propped, laid, cleared and closed by a piece of torus cut out.
//! An odd edge whose other end closes a corner of one sense with edges of
//! the list, an upright with its block's top edges, is laid with them, that
//! corner closed as #699 closes it. Either way each named edge's face is
//! numbered by its rank, and every patch — a flat no name finds, a piece of
//! torus, a corner closed on the way — past them by its corner's ranks. An
//! odd edge whose other end meets an edge of the list any other way is
//! declined: #847 builds a post whose upright is odd at both ends. Where one
//! of the two others is left sharp, the flat along the odd edge's face goes
//! round it only up to the plane square to that edge where its wall begins,
//! held there in the air, its end a face of its own numbered after the
//! patches; a fillet's piece of torus runs round the whole arc there, and
//! stops on that plane at a flat end section of its own, numbered as the
//! chamfer's end is.

mod edge;

use std::collections::BTreeMap;

use glam::DVec3;

use super::Wedge;
use super::pairs::Pair;
use super::step::{Kind, Step};
use crate::body::exact::Exact;
use crate::brep::{self, Curve, Declined, EdgeId, EdgeName};

/// A corner of mixed sense: where it stands, its edges' ranks in the list,
/// from the lowest — the third [`NONE`] where one of its edges is left out —
/// the rank of its odd edge and the way that edge runs from it, the numbers
/// of the face its two others share, which the odd edge stops flush on there,
/// and, where one of those two is left out and left sharp (#846), the way it
/// runs from the corner.
pub(super) struct Odd {
    pub at: DVec3,
    pub ranks: [usize; 3],
    pub lone: usize,
    pub up: DVec3,
    pub on: Vec<u32>,
    pub sharp: Option<DVec3>,
}

/// The rank a corner of mixed sense gives the edge it leaves out: past every
/// rank, so that its corner is numbered by its two named edges' ranks.
pub(super) const NONE: usize = usize::MAX;

/// What a kind lays round the odd edge of a corner of mixed sense.
#[derive(Clone, Copy)]
pub(in crate::body::exact) enum Around {
    /// The odd edge laid first, alone, then the others with the edge where
    /// the face it laid meets the face they share, an edge of their sense: a
    /// chamfer's flat, whose edge there is straight — its foot, or its top.
    Edge,
    /// The corner filled up to the planes through the odd edge's round's axis
    /// square to the others, every edge then laid against it, and what the
    /// filling left replaced by a piece of torus about that axis, as
    /// [`Exact::through_a_torus`] lays it: a fillet's round, whose edge there
    /// is curved. At a concave odd edge, the mirror: the corner propped, every
    /// edge laid, the sector cut away and a piece of torus cut out about the
    /// fill's axis.
    Torus,
}

/// The corners of mixed sense among those three of a step's edges meet at,
/// on `body`, in the order [`super::corners::of_three`] reads them. Declined
/// `Unsupported` where the two others at such a corner share no face, an
/// edge is the odd one at one corner and one of the two others at another,
/// or an odd edge meets at its other end an edge of the list that is odd at
/// no corner without closing a corner of three edges of one sense with it,
/// as [`laid_first`] lays them: the far feet of a post odd at both ends,
/// which #847 builds. Odd edges meeting each other there, the slanted edges
/// of a pyramid at its apex, and the top edges of a standing block meeting
/// its uprights at its top corners, are laid with them first.
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
            sharp: None,
        });
    }
    for (at, pair, third) in super::corners::sharp(body, &step.edges) {
        let left = Wedge::faced(body, third)?;
        let [named, lone] = if step.wedges[pair[0]].inwards == left.inwards {
            pair
        } else {
            [pair[1], pair[0]]
        };
        let away = |edge: EdgeId| {
            let ends = body.edge(edge).ends.into_iter().flatten();
            let far = ends.map(|end| body.vertex(end).point);
            far.max_by(|one, other| one.distance(at).total_cmp(&other.distance(at)))
                .map(|point| (point - at).normalize())
                .ok_or(Declined::Unsupported)
        };
        let foot = &step.wedges[named];
        let shared = (0..2)
            .find(|&side| left.faces.contains(&foot.faces[side]))
            .ok_or(Declined::Unsupported)?;
        odd.push(Odd {
            at,
            ranks: [pair[0], pair[1], NONE],
            lone,
            up: away(step.edges[lone])?,
            on: foot.numbers[shared].clone(),
            sharp: Some(away(third)?),
        });
    }
    let lone = |rank: usize| odd.iter().any(|corner| corner.lone == rank);
    let crossed = odd.iter().any(|corner| {
        (odd.iter()).any(|other| other.lone != corner.lone && other.ranks.contains(&corner.lone))
    });
    let closing = |one: usize, other: usize| {
        (alike(step).iter()).any(|ranks| ranks.contains(&one) && ranks.contains(&other))
    };
    let topped = odd.iter().any(|corner| {
        (0..step.edges.len())
            .filter(|&rank| !lone(rank) && !corner.ranks.contains(&rank))
            .filter(|&rank| !closing(corner.lone, rank))
            .any(|rank| super::corners::meet(body, [step.edges[corner.lone], step.edges[rank]]))
    });
    if crossed || topped {
        return Err(Declined::Unsupported);
    }
    Ok(odd)
}

/// The corners three of a step's edges meet at, all of one sense, by their
/// ranks: those #699 closes, and #801 at a pocket's bottom.
fn alike(step: &Step) -> Vec<[usize; 3]> {
    (step.three.iter())
        .map(|&(_, ranks, _)| ranks)
        .filter(|ranks| {
            let sense = step.wedges[ranks[0]].inwards;
            ranks.iter().all(|&rank| step.wedges[rank].inwards == sense)
        })
        .collect()
}

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
    /// `meeting` says: by the edge of the face it laid, as
    /// [`Exact::round_the_edge`] lays it, or through a piece of torus, as
    /// [`Exact::through_a_torus`] lays it, `step` what the names find on this
    /// body. The pairs laid as
    /// one are handed back through a piece of torus alone, where the fillet
    /// reads them; the chamfer, which goes round by that edge, reads none, and
    /// none is handed back that way. Declined as those decline.
    pub(super) fn round_the_odd(
        &self,
        names: &[EdgeName],
        step: &Step,
        odd: &[Odd],
        taken: f64,
        kind: &Kind,
    ) -> Result<(Exact, f64, Vec<Pair>), Declined> {
        match kind.meeting.around {
            Around::Edge => (self.round_the_edge(names, step, odd, taken, kind))
                .map(|(round, taken)| (round, taken, Vec::new())),
            Around::Torus => self.through_a_torus(names, &step.wedges, odd, taken, kind),
        }
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
