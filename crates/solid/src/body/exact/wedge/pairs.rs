//! Two edges of one step facing each other across a face they share
//! (#698): the bound they set on the one value, so that their set-backs do
//! not cross on it, and the pairs whose set-backs meet across it, which are
//! laid as one.

use std::collections::BTreeSet;

use glam::DVec3;

use super::chain::TANGENT;
use super::slope::one_plane;
use super::{MADE, Prism, Wedge, too_short};
use crate::body::exact::Exact;
use crate::brep::{Declined, EdgeName, FaceId, Scale};

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
    /// A cut beside a fill whose flats, at the value their set-backs meet
    /// at, lie in one plane: one slope, as [`Exact::sloped`] lays it. Two
    /// whose faces meet the same way, or a cut and a fill whose flats would
    /// meet at an angle, are laid edge by edge.
    Sloped,
}

/// Two edges of a step by their ranks in its list, the earlier first, and
/// the value whose set-backs meet across the face they share, as [`bound`]
/// reads it of them alone.
pub(super) type Pair = (usize, usize, f64);

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
/// could keep, and which `together` lays as one, the face gone, each with
/// the value it is laid at. Declined `Unsupported` where an edge would meet
/// two others, every face round it gone.
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
                Together::Sloped => {
                    let backs = [first, second].map(|wedge| value * set_back(wedge));
                    first.inwards != second.inwards && one_plane(first, second, backs, scale.eps())
                }
            };
            (laid && too_short(left, scale)).then_some((one, other, value))
        })
        .collect();
    let ranks: BTreeSet<usize> = pairs
        .iter()
        .flat_map(|&(one, other, _)| [one, other])
        .collect();
    if ranks.len() < 2 * pairs.len() {
        return Err(Declined::Unsupported);
    }
    Ok(pairs)
}

impl Exact {
    /// The body with the edges `earlier` and `later` find, whose set-backs
    /// meet across the face they share, cut off as one by the prism `raise`
    /// raises over both at the value they meet at — or filled in as one,
    /// where their faces meet inwards, as [`Exact::filled`] fills: its first
    /// two walls, the later edge's then the earlier's, laid as one face where
    /// they lie on one surface, numbered as [`Exact::paired`] numbers it.
    /// Declined as [`Exact::cut_off`] says.
    pub(super) fn laid_together(
        &self,
        [earlier, later]: [&EdgeName; 2],
        number: u32,
        raise: Raise,
    ) -> Result<Exact, Declined> {
        let (first_edge, first) = self.wedge(earlier, Wedge::read)?;
        let (second_edge, second) = self.wedge(later, Wedge::read)?;
        let bounds = self.brep.bounds().ok_or(Declined::Unsupported)?;
        let prism = raise(&first, &second, self.brep.scale(), bounds)?;
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
        cut.paired(self, [&first, &second], number)
    }

    /// The body two edges of `before` were laid on as one, its face left
    /// alone as [`Exact::cut_along`] reads it of both their wedges, answering
    /// to the number where the steps' numbers of `before` ended — the later
    /// edge's — and to `number`, the earlier's, in place of the one past it.
    /// Declined `Unsupported` where that reading fails.
    pub(super) fn paired(
        self,
        before: &Exact,
        wedges: [&Wedge; 2],
        number: u32,
    ) -> Result<Exact, Declined> {
        if !self.cut_along(&wedges, before, 2, &[]) {
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
        let lent = before.next + 1;
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
