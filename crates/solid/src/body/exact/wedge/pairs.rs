//! Two edges of one step facing each other across a face they share
//! (#698): the bound they set on the one value, so that their set-backs do
//! not cross on it, and the pairs whose set-backs meet across it, which one
//! prism takes off as one.

use std::collections::BTreeSet;

use glam::DVec3;

use super::chain::TANGENT;
use super::{MADE, Prism, Wedge, too_short};
use crate::body::exact::Exact;
use crate::brep::{Declined, EdgeName, FaceId, Scale};

/// What raises the one prism two edges whose set-backs meet are taken off
/// with, the earlier edge's wedge first, at the value whose set-backs meet
/// across the face they share: as [`Wedge::rounded_with`] raises it.
pub(in crate::body::exact) type Together =
    fn(&Wedge, &Wedge, Scale, (DVec3, DVec3)) -> Result<Prism, Declined>;

/// Two edges of a step by their ranks in its list, the earlier first.
pub(super) type Pair = (usize, usize);

/// Each two edges parallel within [`TANGENT`] that face each other across a
/// face they share, the earlier first, and the distance across it from one
/// to the other. Two that do not face each other across it, or are not
/// parallel, bound nothing here past each one's own reach: they are laid in
/// turn, and the laying declines them where their set-backs meet.
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
/// could keep: the face goes, and one prism cuts both edges off, or fills
/// both in. Only two edges whose faces meet the same way: a cut beside a
/// fill is laid edge by edge. Declined `Unsupported` where an edge would
/// meet two others, every face round it gone.
pub(super) fn meeting(
    wedges: &[Wedge],
    facing: &[(usize, usize, f64)],
    taken: f64,
    set_back: impl Fn(&Wedge) -> f64,
    scale: Scale,
) -> Result<Vec<Pair>, Declined> {
    let pairs: Vec<Pair> = facing
        .iter()
        .filter(|&&(one, other, across)| {
            let [one, other] = [&wedges[one], &wedges[other]];
            let left = across - taken * (set_back(one) + set_back(other));
            one.inwards == other.inwards && too_short(left, scale)
        })
        .map(|&(one, other, _)| (one, other))
        .collect();
    let ranks: BTreeSet<usize> = pairs
        .iter()
        .flat_map(|&(one, other)| [one, other])
        .collect();
    if ranks.len() < 2 * pairs.len() {
        return Err(Declined::Unsupported);
    }
    Ok(pairs)
}

impl Exact {
    /// The body with the edges `earlier` and `later` find, whose set-backs
    /// meet across the face they share, cut off as one by the prism
    /// `together` raises over both at the value they meet at — or filled in
    /// as one, where their faces meet inwards, as [`Exact::filled`] fills:
    /// its first two walls, the later edge's then the earlier's, laid as one
    /// face where they lie on one surface, answering to the number where the
    /// steps' numbers ended — the later edge's — and to `number`, the
    /// earlier's. Declined as [`Exact::cut_off`] says.
    pub(super) fn laid_together(
        &self,
        [earlier, later]: [&EdgeName; 2],
        number: u32,
        together: Together,
    ) -> Result<Exact, Declined> {
        let (first_edge, first) = self.wedge(earlier, Wedge::read)?;
        let (second_edge, second) = self.wedge(later, Wedge::read)?;
        let bounds = self.brep.bounds().ok_or(Declined::Unsupported)?;
        let prism = together(&first, &second, self.brep.scale(), bounds)?;
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
            let edges = [(first_edge, &first), (second_edge, &second)];
            self.filled(&edges, &prism, tool)?
        } else {
            self.cut_by(&Exact { next: 2, ..tool })?
        };
        if !cut.cut_along(&[&first, &second], self, 2) {
            return Err(Declined::Unsupported);
        }
        let Exact {
            mut brep,
            next,
            pieces,
            roots,
            drawn,
            ..
        } = cut;
        let lent = self.next + 1;
        let faces: Vec<FaceId> = brep
            .face_ids()
            .filter(|&face| brep.numbers(face).contains(&lent))
            .collect();
        brep.rename(&faces, lent, number);
        let renamed = Exact::of(brep, next, pieces, roots);
        if let Some(drawn) = drawn.into_inner() {
            let _ = renamed.drawn.set(drawn);
        }
        Ok(renamed)
    }
}
