//! A corner three edges of one step meet at, two of one sense and the odd one
//! of the other (#829): the foot corner of a block standing on another, two
//! concave feet and the convex upright above them. A cut and a fill laid in
//! turn there meet in nothing, so the odd edge is laid first, as it is laid
//! alone, stopping flush on the face the two others share; the two others
//! then follow the outline it left, as they would in a later step, the edge
//! where the odd edge's face meets that face laid with them — the foot of a
//! chamfer's flat, filled among the feet and mitred with them, as #801 fills
//! it. A fillet's round leaves a curved foot, which the feet run on into
//! tangentially, and the step is declined there as any tangent chain is,
//! until a piece of torus joins the feet's rounds round it.

use glam::DVec3;

use super::pairs::Pair;
use super::step::Step;
use super::{Meeting, Prism, Wedge};
use crate::body::exact::Exact;
use crate::brep::{Declined, EdgeName, Scale};

/// A corner of mixed sense: the rank of its odd edge in the list, and the
/// numbers of the face its two others share, which the odd edge stops flush
/// on there.
pub(super) type Odd = (usize, Vec<u32>);

/// The corners of mixed sense among those three of a step's edges meet at,
/// in the order [`super::corners::of_three`] reads them. Declined
/// `Unsupported` where the two others at such a corner share no face, or an
/// edge is the odd one at one corner and one of the two others at another.
pub(super) fn corners(step: &Step) -> Result<Vec<Odd>, Declined> {
    let mut odd = Vec::new();
    for &(_, ranks, _) in &step.three {
        let inwards = ranks.map(|rank| step.wedges[rank].inwards);
        let alike = |at: usize| inwards.iter().filter(|&&way| way == inwards[at]).count();
        let Some(lone) = (0..3).find(|&at| alike(at) == 1) else {
            continue;
        };
        let [one, other] = [1, 2].map(|by| &step.wedges[ranks[(lone + by) % 3]]);
        let shared = (0..2)
            .find(|&side| other.faces.contains(&one.faces[side]))
            .ok_or(Declined::Unsupported)?;
        odd.push((ranks[lone], ranks, one.numbers[shared].clone()));
    }
    let crossed = odd.iter().any(|&(lone, _, _)| {
        (odd.iter()).any(|&(other, ranks, _)| other != lone && ranks.contains(&lone))
    });
    if crossed {
        return Err(Declined::Unsupported);
    }
    Ok(odd.into_iter().map(|(lone, _, on)| (lone, on)).collect())
}

impl Exact {
    /// The body with the edges `names` find laid at `taken`, each corner of
    /// mixed sense `odd` holds gone round its odd edge: the odd edges first,
    /// alone, then the others with, for each such corner, the edge where the
    /// face its odd edge laid meets the face the two others share, as
    /// [`Exact::in_turn`] lays each of the two lists with what `kind` holds —
    /// `most`, `set_back`, `tool` and `meeting` as [`Exact::cut_off`] takes
    /// them; the face laid on each named edge numbered where the steps'
    /// numbers ended plus its rank in the list, and those laid on the edges
    /// no name finds past them, corner by corner. Declined
    /// `Unsupported` where either list gives less than `taken`, as
    /// [`Step::bound`] reads it on the body it is laid on, the odd edge's face
    /// meets the face its two others share along no edge or several, or one
    /// of two edges laid together is an edge no name finds; otherwise as
    /// [`Exact::cut_off`] declines either list.
    pub(super) fn round_the_odd(
        &self,
        names: &[EdgeName],
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
            (0..names.len()).partition(|&rank| odd.iter().any(|&(lone, _)| lone == rank));
        let named = |ranks: &[usize]| -> Vec<EdgeName> {
            ranks.iter().map(|&rank| names[rank].clone()).collect()
        };
        let (laid, before) = self.laid_at(&named(&first), taken, kind)?;
        let feet = (odd.iter())
            .map(|(lone, on)| {
                let at = first
                    .binary_search(lone)
                    .map_err(|_| Declined::Unsupported)?;
                laid.foot(start + at as u32, on)
            })
            .collect::<Result<Vec<_>, Declined>>()?;
        let following: Vec<EdgeName> = named(&rest).into_iter().chain(feet).collect();
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
    fn renamed_by(self, moved: impl Fn(u32) -> u32) -> Exact {
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
