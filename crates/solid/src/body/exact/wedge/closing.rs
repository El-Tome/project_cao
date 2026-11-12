//! A corner three edges of one step meet at (#699), closed once the three are
//! laid: the corner as the body given reads it, and the closing each kind
//! lays there — a triangle among flats, a piece of sphere among rounds, cut
//! where the three meet outwards and joined, hollow, where they meet inwards,
//! at a pocket's bottom corner (#801).

use glam::{DVec2, DVec3};

use super::Wedge;
use super::corners::Three;
use crate::body::exact::Exact;
use crate::brep::Declined;
use crate::profile::{Contour, Frame};

/// What closes a corner three edges meet at, on the body the three were laid
/// on, the faces they laid numbered as the three numbers say, its own face
/// numbered where the steps' numbers end: as [`super::cut_across`] closes it
/// among flats, and [`super::rounded_across`] among rounds.
pub(in crate::body::exact) type Close = fn(&Exact, &Corner, [u32; 3]) -> Result<Exact, Declined>;

/// What a kind closes a corner with: `outwards` where its three edges meet
/// outwards, `inwards` where they meet inwards.
#[derive(Clone, Copy)]
pub(in crate::body::exact) struct Closing {
    pub outwards: Close,
    pub inwards: Close,
}

/// How far what fills a concave corner reaches into the matter past the
/// corner's three faces, as a share of the value: far enough that no face of
/// it lies nearly flush with theirs, near enough to stay inside a wall or a
/// floor thicker than that.
const SUNK: f64 = 0.125;

/// A corner three edges of a step meet at, read on the body as given: where
/// it stands; where the set-backs of each two cross on the face the two share
/// — the first and the second's, the second and the third's, the third and
/// the first's — and the way each of those faces faces; the way each edge
/// runs from the corner; the value its first edge is laid at; and whether
/// each edge runs from it to another corner the step closes.
pub(in crate::body::exact) struct Corner {
    pub at: DVec3,
    pub crossings: [DVec3; 3],
    pub facing: [DVec3; 3],
    pub along: [DVec3; 3],
    pub value: f64,
    pub between: [bool; 3],
}

/// Each corner three edges meet at, read as a [`Corner`] for the value
/// `laid_at` says each edge is laid at, the edge set back `set_back` of a
/// value of one along its faces, with the ranks of its edges and what of
/// `closing` closes it, by the way its edges meet. Declined `Unsupported`
/// where two of a corner's edges share no face, their set-backs never cross,
/// or the three do not all meet the same way.
pub(super) fn closings(
    corners: Vec<Three>,
    wedges: &[Wedge],
    laid_at: impl Fn(usize) -> f64,
    set_back: impl Fn(&Wedge) -> f64,
    closing: Closing,
) -> Result<Vec<(Corner, [usize; 3], Close)>, Declined> {
    let closed: Vec<[usize; 3]> = corners.iter().map(|&(_, ranks, _)| ranks).collect();
    corners
        .into_iter()
        .map(|(at, ranks, along)| {
            let three = ranks.map(|rank| &wedges[rank]);
            let backs = ranks.map(|rank| laid_at(rank) * set_back(&wedges[rank]));
            let between = ranks.map(|rank| {
                (closed.iter())
                    .filter(|other| other.contains(&rank))
                    .count()
                    > 1
            });
            let corner = read((at, along), three, backs, laid_at(ranks[0]), between)
                .ok_or(Declined::Unsupported)?;
            let close = match three.map(|wedge| wedge.inwards) {
                [false, false, false] => closing.outwards,
                [true, true, true] => closing.inwards,
                _ => return Err(Declined::Unsupported),
            };
            Ok((corner, ranks, close))
        })
        .collect()
}

/// The corner at `at` of three edges running from it `along`, each set back
/// `set_back` along its faces, as [`closings`] reads it.
fn read(
    (at, along): (DVec3, [DVec3; 3]),
    wedges: [&Wedge; 3],
    set_back: [f64; 3],
    value: f64,
    between: [bool; 3],
) -> Option<Corner> {
    let crossing = |one: usize, other: usize| {
        let [first, second] = [wedges[one], wedges[other]];
        let (on_first, on_second) = (0..2)
            .flat_map(|i| (0..2).map(move |j| (i, j)))
            .find(|&(i, j)| first.faces[i] == second.faces[j])?;
        let from = first.point + first.into[on_first] * set_back[one];
        let to = second.point + second.into[on_second] * set_back[other];
        let normal = first.along.cross(second.along);
        let along = (to - from).cross(second.along).dot(normal) / normal.length_squared();
        along
            .is_finite()
            .then(|| (from + first.along * along, first.outward[on_first]))
    };
    let [(first, one), (second, two), (third, three)] =
        [crossing(0, 1)?, crossing(1, 2)?, crossing(2, 0)?];
    Some(Corner {
        at,
        crossings: [first, second, third],
        facing: [one, two, three],
        along,
        value,
        between,
    })
}

impl Corner {
    /// `solid` less what of it stands deeper in the matter than [`SUNK`] of
    /// the value past the corner's three faces, every point of it within
    /// `reach` of the corner: what fills a concave corner, kept inside its
    /// walls.
    pub fn sunk(&self, solid: Exact, reach: f64) -> Result<Exact, Declined> {
        let depth = self.value * SUNK;
        (self.facing.iter()).try_fold(solid, |kept, &facing| {
            kept.cut_by(&slab(-facing, self.at - facing * depth, reach)?)
        })
    }
}

/// A box standing on the plane through `from` square to `way`, `half` either
/// side of `from` across it, raised along `way` twice that.
pub(super) fn slab(way: DVec3, from: DVec3, half: f64) -> Result<Exact, Declined> {
    let u = way.any_orthonormal_vector();
    let frame = Frame {
        origin: from,
        u,
        v: way.cross(u),
    };
    let square = Contour::rectangle(DVec2::splat(-half), DVec2::splat(half));
    Exact::raised(&square, &[], frame, way * 2.0 * half)
}

impl Exact {
    /// The body with each corner `closes` holds closed as its closing closes
    /// it, the faces laid on its edges numbered from `start` by their ranks,
    /// in turn.
    pub(super) fn closed(
        self,
        closes: &[(Corner, [usize; 3], Close)],
        start: u32,
    ) -> Result<Exact, Declined> {
        closes
            .iter()
            .try_fold(self, |body, (corner, ranks, close)| {
                close(&body, corner, ranks.map(|rank| start + rank as u32))
            })
    }
}
