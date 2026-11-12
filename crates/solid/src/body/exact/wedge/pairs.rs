//! Two edges of one step facing each other across a face they share
//! (#698): the bound they set on the one value, so that their set-backs do
//! not cross on it, the pairs whose set-backs meet across it, which are laid
//! as one, and the edges of the list such a pair is mitred against at a
//! corner (#803): each edge at its end, bounding the face that is gone, one
//! across it or the two of a pointed or a V-notched end (#843), the upright
//! at a point's tip with them, and any upright meeting one of the pair, at
//! either end, alone or with the edge at their end (#837); and the edge at a
//! round bottom's end, which its fill meets in a valley (#838).

use std::collections::{BTreeMap, BTreeSet};

use glam::DVec3;

use super::chain::TANGENT;
use super::corners::{self, Beside, Three};
use super::{MADE, Prism, Wedge, too_short};
use crate::body::exact::Exact;
use crate::brep::{self, Declined, EdgeId, FaceId, Scale};

/// What raises the one prism two edges whose set-backs meet are taken off
/// with, the earlier edge's wedge first, at the value whose set-backs meet
/// across the face they share: as [`Wedge::rounded_with`] raises it.
pub(in crate::body::exact) type Raise =
    fn(&Wedge, &Wedge, Scale, (DVec3, DVec3)) -> Result<Prism, Declined>;

/// Which two edges whose set-backs meet across the face they share a kind
/// lays as one, and how.
#[derive(Clone, Copy)]
pub(in crate::body::exact) enum Together {
    /// Two whose faces meet the same way, taken off or filled in as one by
    /// the prism this raises — a full round, a round bottom. A cut beside a
    /// fill is laid edge by edge.
    Raised(Raise),
    /// A cut beside a fill, as [`Exact::sloped`] lays them at the value
    /// their set-backs meet at: one slope where their flats lie in one
    /// plane, two flats meeting in a crease where they lie at an angle. Two
    /// whose faces meet the same way are laid edge by edge.
    Sloped,
}

/// Two edges of a step by their ranks in its list, the earlier first, and
/// the value whose set-backs meet across the face they share, as [`bound`]
/// reads it of them alone.
pub(super) type Pair = (usize, usize, f64);

/// Each two edges parallel within [`TANGENT`] that face each other across a
/// face they share, the earlier first, and the distance across it from one
/// to the other. Two that do not face each other across it, or are not
/// parallel, bound nothing here: [`super::set_backs::bound`] reads where
/// their set-backs would meet, and they are laid in turn.
pub(super) fn facing(wedges: &[Wedge]) -> Vec<(usize, usize, f64)> {
    let sides = |wedge: &Wedge| wedge.faces.into_iter().zip(wedge.into);
    let mut facing = Vec::new();
    for (rank, one) in wedges.iter().enumerate() {
        for (later, other) in wedges.iter().enumerate().skip(rank + 1) {
            for (face, into) in sides(one) {
                for (_, towards) in sides(other).filter(|&(theirs, _)| theirs == face) {
                    let parallel = one.along.cross(other.along).length() <= TANGENT;
                    let across = into.dot(other.point - one.point);
                    if parallel && across > 0.0 && into.dot(towards) < 0.0 {
                        facing.push((rank, later, across));
                    }
                }
            }
        }
    }
    facing
}

/// The most value at which no two set-backs cross on a face two of the
/// edges share: for each two that face each other across it, the distance
/// between them over the set-backs a value of one makes from either.
pub(super) fn bound(
    wedges: &[Wedge],
    facing: &[(usize, usize, f64)],
    set_back: impl Fn(&Wedge) -> f64,
) -> f64 {
    facing
        .iter()
        .map(|&(one, other, across)| across / (set_back(&wedges[one]) + set_back(&wedges[other])))
        .fold(f64::INFINITY, f64::min)
}

/// The two edges facing each other, the earlier first, whose set-backs at
/// `taken` meet across the face they share, leaving none of it the kernel
/// could keep, and which `together` lays as one, the face gone, each with
/// the value it is laid at.
pub(super) fn meeting(
    wedges: &[Wedge],
    facing: &[(usize, usize, f64)],
    taken: f64,
    set_back: impl Fn(&Wedge) -> f64,
    together: Together,
    scale: Scale,
) -> Result<Vec<Pair>, Declined> {
    let pairs: Vec<Pair> = facing
        .iter()
        .filter_map(|&(one, other, across)| {
            let [first, second] = [&wedges[one], &wedges[other]];
            let left = across - taken * (set_back(first) + set_back(second));
            let value = bound(wedges, &[(one, other, across)], &set_back);
            let laid = match together {
                Together::Raised(_) => first.inwards == second.inwards,
                Together::Sloped => first.inwards != second.inwards,
            };
            (laid && too_short(left, scale)).then_some((one, other, value))
        })
        .collect();
    Ok(pairs)
}

/// Declined `Unsupported` where an edge of `pairs` would meet two others,
/// every face round it gone, but where they close on themselves, as
/// [`super::ring::closed`] reads them.
pub(super) fn apart(pairs: &[Pair]) -> Result<(), Declined> {
    let ranks: BTreeSet<usize> = pairs
        .iter()
        .flat_map(|&(one, other, _)| [one, other])
        .collect();
    if ranks.len() < 2 * pairs.len() {
        return Err(Declined::Unsupported);
    }
    Ok(())
}

/// The ranks of the edges laid as one round with the edge of rank `rank`,
/// itself among them, as `pairs` join them one to the next, ascending.
pub(in crate::body::exact) fn together(rank: usize, pairs: &[Pair]) -> Vec<usize> {
    let mut joined = BTreeSet::from([rank]);
    loop {
        let more: Vec<usize> = (pairs.iter())
            .filter_map(
                |&(one, other, _)| match (joined.contains(&one), joined.contains(&other)) {
                    (true, false) => Some(other),
                    (false, true) => Some(one),
                    _ => None,
                },
            )
            .collect();
        if more.is_empty() {
            return joined.into_iter().collect();
        }
        joined.extend(more);
    }
}

/// Each edge of `edges` bounding the face a pair of `pairs` whose faces meet
/// outwards meet across, of no pair itself, that meets both of the pair at a
/// corner — the edge at their end — or one of them and another such edge that
/// meets the other — each edge of a pointed or a notched end (#843) — with
/// the third edge of the list ending where those two meet, as `three` holds
/// the corners three of them meet at — the upright at a point's tip, which
/// meets neither of the pair, the three closed there (#837) — and each edge
/// whose faces meet outwards, not bounding that face, that meets one of the
/// pair at a corner where no third edge of the list ends but the edge at
/// their end, the three closed there, convex, `beside` holding the corners
/// that are not — an upright at either end, one of a standing full round
/// among them (#837) — and, of a pair not laid outwards, a round bottom, a
/// slope or a crease, each edge bounding the face they meet across that
/// meets both of the pair at a corner, whatever its own sense — whose cut or
/// fill is laid in turn against the one prism they are taken off or filled
/// in with and against its neighbours, by its rank, with the pair's value,
/// the radius that prism is raised at, so that it meets theirs where they
/// meet across the face even where the value taken stands a hair short of
/// it. Of a pair not laid outwards, only an end edge meeting inwards is
/// built: the floor's edge on a pocket's end wall, or the edge at the foot of
/// a standing round bottom of two uprights at that end, its fill joined to
/// theirs in a valley (#838). A list mixing senses there — a slope or a
/// crease with the tread's end edge, a round bottom with a convex end edge
/// such as an open slot's end, a standing round bottom with the rim above
/// it — is let in here and refused
/// by the laying, as [`Exact::cut_along`] and [`Exact::sloped`] refuse it: a
/// cut and a fill meeting there do not commute. Declined `Unsupported` where
/// an edge of a pair meets another of `edges` at a corner any other way: a
/// full round meeting an edge bounding its face at one of its edges alone,
/// or an upright where a third edge of the list ends but the edge at their
/// end (#837), a notched end with the upright at its bottom, a corner of
/// mixed sense gone round that upright first (#914), a round bottom, a slope
/// or a crease meeting any other edge — the uprights at a round bottom's end
/// (#919), the edges of a pointed or a notched one (#921).
pub(super) fn at_their_ends(
    body: &brep::Body,
    edges: &[EdgeId],
    wedges: &[Wedge],
    pairs: &[Pair],
    three: &[Three],
    beside: &[Beside],
) -> Result<BTreeMap<usize, f64>, Declined> {
    let members: BTreeSet<usize> = pairs
        .iter()
        .flat_map(|&(one, other, _)| [one, other])
        .collect();
    let meets = |member: usize, rank: usize| {
        rank != member && corners::meet(body, [edges[member], edges[rank]])
    };
    let mut ends = BTreeMap::new();
    for &(one, other, value) in pairs {
        let [first, second] = [&wedges[one], &wedges[other]];
        let outwards = !first.inwards && !second.inwards;
        let across = first.faces.iter().find(|face| second.faces.contains(face));
        let bounding = |rank: usize| {
            !members.contains(&rank) && across.is_some_and(|face| wedges[rank].faces.contains(face))
        };
        let thirds = |rank: usize, tip: usize| -> Vec<usize> {
            (three.iter())
                .filter(|(_, at, _)| at.contains(&rank) && at.contains(&tip))
                .flat_map(|(_, at, _)| at.iter().copied())
                .filter(|third| ![rank, tip].contains(third))
                .collect()
        };
        let tips = |rank: usize, far: usize| {
            (0..edges.len())
                .filter(move |&tip| bounding(tip) && meets(rank, tip) && meets(far, tip))
        };
        let reaching =
            |rank: usize, far: usize| meets(far, rank) || tips(rank, far).next().is_some();
        let at_the_end = |rank: usize| bounding(rank) && meets(one, rank) && meets(other, rank);
        let upright = |rank: usize| {
            let member = if meets(one, rank) { one } else { other };
            meets(one, rank) != meets(other, rank)
                && !wedges[rank].inwards
                && !across.is_some_and(|face| wedges[rank].faces.contains(face))
                && (three.iter())
                    .filter(|(_, at, _)| at.contains(&rank) && at.contains(&member))
                    .all(|(_, at, _)| {
                        (at.iter())
                            .all(|&third| [rank, member].contains(&third) || at_the_end(third))
                    })
                && !(beside.iter()).any(|(_, at)| at.contains(&rank) && at.contains(&member))
        };
        for rank in (0..edges.len()).filter(|&rank| meets(one, rank) || meets(other, rank)) {
            let at_their_end = bounding(rank) && reaching(rank, one) && reaching(rank, other);
            let laid = if outwards {
                at_their_end || upright(rank)
            } else {
                at_the_end(rank)
            };
            if !laid {
                return Err(Declined::Unsupported);
            }
            ends.insert(rank, value);
            let at_the_tip = [one, other].into_iter().flat_map(|far| tips(rank, far));
            ends.extend(
                at_the_tip
                    .flat_map(|tip| thirds(rank, tip))
                    .map(|third| (third, value)),
            );
        }
    }
    Ok(ends)
}

impl Exact {
    /// The body with the two edges of `pair`, each with its wedge as read on
    /// `on`, the body as given, the earlier first, whose set-backs meet
    /// across the face they share, cut off as one by the prism `raise` raises
    /// over both at the value they meet at — or filled in as one, where their
    /// faces meet inwards, as [`Exact::filled`] fills: its first two walls,
    /// the later edge's then the earlier's, laid as one face where they lie on
    /// one surface, numbered as [`Exact::paired`] numbers it, meeting the
    /// faces `mitred` numbers in a mitre. The edges laid before move none of
    /// their planes, and the rounds at their ends may have moved both ends of
    /// each, so that no name finds them. Declined as [`Exact::cut_off`] says.
    pub(super) fn laid_together(
        &self,
        on: &brep::Body,
        pair: [(EdgeId, &Wedge); 2],
        number: u32,
        raise: Raise,
        mitred: &[u32],
    ) -> Result<Exact, Declined> {
        let [(_, first), (_, second)] = pair;
        let bounds = self.brep.bounds().ok_or(Declined::Unsupported)?;
        let prism = raise(first, second, self.brep.scale(), bounds)?;
        let (outline, frame, travel) = &prism;
        let raised = Exact::raised(outline, &[], *frame, *travel)?;
        let tool = Exact {
            brep: raised.brep.renumbered_by(|wall| match wall {
                0 | 1 => wall + MADE,
                2 | 3 => wall - MADE,
                other => other,
            }),
            ..raised
        };
        let cut = if first.inwards {
            self.filled(on, &pair, (&[], &[]), &prism, tool)?
        } else {
            self.cut_by(&Exact { next: 2, ..tool })?
        };
        cut.paired(self, [first, second], [self.next, number], mitred)
    }

    /// The body two edges of `before` were laid on as one, its faces left
    /// alone as [`Exact::cut_along`] reads it of both their wedges and of the
    /// faces laid before it that `mitred` numbers: what answered to the number
    /// where the steps' numbers of `before` ended — the first wall laid —
    /// answering to `numbers[0]` instead, and what answered to the one past it
    /// to `numbers[1]`. Declined `Unsupported` where that reading fails.
    pub(super) fn paired(
        self,
        before: &Exact,
        wedges: [&Wedge; 2],
        numbers: [u32; 2],
        mitred: &[u32],
    ) -> Result<Exact, Declined> {
        if !self.cut_along(&wedges, before, 2, mitred, &[], &[]) {
            return Err(Declined::Unsupported);
        }
        let Exact {
            mut brep,
            next,
            pieces,
            roots,
            drawn,
            ..
        } = self;
        let laid = [before.next, before.next + 1];
        let faces = laid.map(|number| {
            (brep.face_ids())
                .filter(|&face| brep.numbers(face).contains(&number))
                .collect::<Vec<FaceId>>()
        });
        for ((faces, from), to) in faces.iter().zip(laid).zip(numbers) {
            brep.rename(faces, from, to);
        }
        let renamed = Exact::of(brep, next, pieces, roots);
        if let Some(drawn) = drawn.into_inner() {
            let _ = renamed.drawn.set(drawn);
        }
        Ok(renamed)
    }
}
