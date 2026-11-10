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
//! [`super::torus`] lays it.

use glam::DVec3;

use super::pairs::Pair;
use super::step::Step;
use super::{Meeting, Prism, Wedge};
use crate::body::exact::Exact;
use crate::brep::{Curve, Declined, EdgeName, Scale};

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
/// in the order [`super::corners::of_three`] reads them. Declined
/// `Unsupported` where the two others at such a corner share no face, or an
/// edge is the odd one at one corner and one of the two others at another.
pub(super) fn corners(step: &Step) -> Result<Vec<Odd>, Declined> {
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
    let crossed = odd.iter().any(|corner| {
        (odd.iter()).any(|other| other.lone != corner.lone && other.ranks.contains(&corner.lone))
    });
    if crossed {
        return Err(Declined::Unsupported);
    }
    Ok(odd)
}

impl Exact {
    /// The body with the edges `names` find laid at `taken`, each corner of
    /// mixed sense `odd` holds gone round its odd edge as `meeting` says: by
    /// its foot, as [`Exact::round_the_foot`] lays it, or through a piece of
    /// torus, as [`Exact::through_a_torus`] lays it, each edge's wedge on this
    /// body by its rank in `wedges`. Declined as those decline.
    pub(super) fn round_the_odd(
        &self,
        names: &[EdgeName],
        wedges: &[Wedge],
        odd: &[Odd],
        taken: f64,
        kind: (
            (&impl Fn(&Wedge) -> f64, &impl Fn(&Wedge) -> f64),
            &impl Fn(&Wedge, f64, Scale, (DVec3, DVec3)) -> Result<Prism, Declined>,
            Meeting,
        ),
    ) -> Result<(Exact, f64, Vec<Pair>), Declined> {
        let (_, _, meeting) = kind;
        match meeting.around {
            Around::Foot => self.round_the_foot(names, wedges, odd, taken, kind),
            Around::Torus => self.through_a_torus(names, wedges, odd, taken, kind),
        }
    }

    /// The body with the edges `names` find laid at `taken`, each corner of
    /// mixed sense `odd` holds gone round its odd edge: the odd edges first,
    /// alone, then the others — each followed onto what the odd edges left,
    /// as [`Exact::following`] finds it, its wedge on this body by its rank in
    /// `wedges` — with, for each such corner, the edge where the face its odd
    /// edge laid meets the face the two others share, as [`Exact::in_turn`]
    /// lays each of the two lists with what `kind` holds — `most`,
    /// `set_back`, `tool` and `meeting` as [`Exact::cut_off`] takes them; the
    /// face laid on each named edge numbered where the steps' numbers ended
    /// plus its rank in the list, and those laid on the edges no name finds
    /// past them, corner by corner. Declined `Unsupported` where either list
    /// gives less than `taken`, as [`Step::bound`] reads it on the body it is
    /// laid on, an edge cannot be followed, the odd edge's face meets the face
    /// its two others share along no edge or several, or one of two edges
    /// laid together is an edge no name finds; otherwise as [`Exact::cut_off`]
    /// declines either list.
    fn round_the_foot(
        &self,
        names: &[EdgeName],
        wedges: &[Wedge],
        odd: &[Odd],
        taken: f64,
        kind: (
            (&impl Fn(&Wedge) -> f64, &impl Fn(&Wedge) -> f64),
            &impl Fn(&Wedge, f64, Scale, (DVec3, DVec3)) -> Result<Prism, Declined>,
            Meeting,
        ),
    ) -> Result<(Exact, f64, Vec<Pair>), Declined> {
        let start = self.next;
        let (first, rest): (Vec<usize>, Vec<usize>) =
            (0..names.len()).partition(|&rank| odd.iter().any(|corner| corner.lone == rank));
        let named = |ranks: &[usize]| -> Vec<EdgeName> {
            ranks.iter().map(|&rank| names[rank].clone()).collect()
        };
        let (laid, before) = self.laid_at(&named(&first), taken, kind)?;
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
        let (round, after) = laid.laid_at(&following, taken, kind)?;
        let order: Vec<u32> = (first.iter().chain(&rest))
            .map(|&rank| start + rank as u32)
            .collect();
        let pairs = [ranked_back(&first, before)?, ranked_back(&rest, after)?].concat();
        let renamed = round.renamed_by(|number| {
            let at = number.checked_sub(start).map(|at| at as usize);
            at.and_then(|at| order.get(at)).copied().unwrap_or(number)
        });
        Ok((renamed, taken, pairs))
    }

    /// The body with the edges `names` find on it laid in turn at `taken`, as
    /// [`Exact::in_turn`] lays them with what `kind` holds, and the pairs it
    /// laid as one. Declined `Unsupported` where they give less than `taken`,
    /// as [`Step::bound`] reads it on this body; otherwise as
    /// [`Exact::cut_off`] declines them.
    fn laid_at(
        &self,
        names: &[EdgeName],
        taken: f64,
        ((most, set_back), tool, meeting): (
            (&impl Fn(&Wedge) -> f64, &impl Fn(&Wedge) -> f64),
            &impl Fn(&Wedge, f64, Scale, (DVec3, DVec3)) -> Result<Prism, Declined>,
            Meeting,
        ),
    ) -> Result<(Exact, Vec<Pair>), Declined> {
        let step = self.step(names)?;
        if step.bound(&self.brep, taken, most, set_back) < taken {
            return Err(Declined::Unsupported);
        }
        let (laid, _, pairs) = self.in_turn(names, step, taken, set_back, tool, meeting)?;
        Ok((laid, pairs))
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

/// The pairs a list laid, each edge by the rank `ranks` gives it in the
/// step's list rather than its own. Declined `Unsupported` where an edge of a
/// pair is past `ranks`: an edge no name finds.
fn ranked_back(ranks: &[usize], pairs: Vec<Pair>) -> Result<Vec<Pair>, Declined> {
    let back = |rank: usize| ranks.get(rank).copied().ok_or(Declined::Unsupported);
    (pairs.into_iter())
        .map(|(one, other, value)| Ok((back(one)?, back(other)?, value)))
        .collect()
}
