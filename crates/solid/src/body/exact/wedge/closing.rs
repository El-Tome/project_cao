//! A corner three edges of one step meet at (#699), closed once the three are
//! laid: the corner as the body given reads it, and the closing each kind
//! lays there — a triangle among flats, a piece of sphere among rounds.

use glam::DVec3;

use super::Wedge;
use crate::body::exact::Exact;
use crate::brep::Declined;

/// What closes a corner three edges meet at, on the body the three were laid
/// on, the faces they laid numbered as the three numbers say, its own face
/// numbered where the steps' numbers end: as [`super::cut_across`] closes it
/// among flats, and [`super::rounded_across`] among rounds.
pub(in crate::body::exact) type Closing = fn(&Exact, &Corner, [u32; 3]) -> Result<Exact, Declined>;

/// A corner three edges of a step meet at, read on the body as given: where
/// it stands; where the set-backs of each two cross on the face the two share
/// — the first and the second's, the second and the third's, the third and
/// the first's — and the way each of those faces faces; the way each edge
/// runs from the corner; and the value the step takes.
pub(in crate::body::exact) struct Corner {
    pub at: DVec3,
    pub crossings: [DVec3; 3],
    pub facing: [DVec3; 3],
    pub along: [DVec3; 3],
    pub value: f64,
}

/// Each corner three edges meet at, read as a [`Corner`] for the value
/// `taken`, each edge set back `set_back` along its faces, with the ranks of
/// its edges and what closes it. Declined `Unsupported` where two of a
/// corner's edges share no face, their set-backs never cross, or cross level
/// with the corner along one.
pub(super) fn closings(
    corners: Vec<(DVec3, [usize; 3])>,
    wedges: &[Wedge],
    set_back: impl Fn(&Wedge) -> f64,
    taken: f64,
    closing: Closing,
) -> Result<Vec<(Corner, [usize; 3], Closing)>, Declined> {
    corners
        .into_iter()
        .map(|(at, ranks)| {
            let three = ranks.map(|rank| &wedges[rank]);
            let corner =
                read(at, three, three.map(&set_back), taken).ok_or(Declined::Unsupported)?;
            Ok((corner, ranks, closing))
        })
        .collect()
}

/// The corner at `at` of three edges, each set back `set_back` along its
/// faces, as [`closings`] reads it.
fn read(at: DVec3, wedges: [&Wedge; 3], set_back: [f64; 3], value: f64) -> Option<Corner> {
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
    let crossings = [first, second, third];
    let mut along = [0, 1, 2].map(|edge| wedges[edge].along);
    for (way, crossing) in along.iter_mut().zip(crossings) {
        let off = way.dot(crossing - at);
        if off == 0.0 {
            return None;
        }
        *way *= off.signum();
    }
    Some(Corner {
        at,
        crossings,
        facing: [one, two, three],
        along,
        value,
    })
}

impl Exact {
    /// The body with each corner `closes` holds closed as its closing closes
    /// it, the faces laid on its edges numbered from `start` by their ranks,
    /// in turn.
    pub(super) fn closed(
        self,
        closes: &[(Corner, [usize; 3], Closing)],
        start: u32,
    ) -> Result<Exact, Declined> {
        closes
            .iter()
            .try_fold(self, |body, (corner, ranks, close)| {
                close(&body, corner, ranks.map(|rank| start + rank as u32))
            })
    }
}
