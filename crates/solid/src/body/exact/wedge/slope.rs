//! A cut beside a fill (#791): an edge whose faces meet outwards and one
//! whose faces meet inwards, facing each other across the face they share,
//! whose flats meet there in one plane — a tread's nose and foot set back by
//! half its depth — laid as one slope, the face they shared gone.

use glam::DVec3;

use super::flush::Flush;
use super::{Prism, Wedge};
use crate::body::exact::Exact;
use crate::brep::{Declined, EdgeName, Scale};

/// Whether the flats of two edges, each on the plane through the lines it
/// is set back to along its two faces — `own` from the one edge, `theirs`
/// from the other — lie in one plane: the other's two lines, where they
/// pass the other edge's point, within `eps` of the one's plane. The two
/// edges are parallel only as `facing` reads them, within `TANGENT`: flats
/// parting further along are left apart by the kernel, and
/// [`Exact::sloped`] declines them.
pub(super) fn one_plane(one: &Wedge, other: &Wedge, [own, theirs]: [f64; 2], eps: f64) -> bool {
    let [near, far] = one.into.map(|into| one.point + into * own);
    let normal = (near - far).cross(one.along).normalize();
    (other.into.iter()).all(|&into| normal.dot(other.point + into * theirs - near).abs() <= eps)
}

impl Exact {
    /// The body with the edges `earlier` and `later` find, one cut off and
    /// the other filled in, by the prisms `tool` raises over each for
    /// `value`, where their set-backs meet: the cut first, whichever the
    /// names give first — its prism runs on a margin past its set-back, which
    /// would lie flush on a fill laid before it — its flat numbered where the
    /// steps' numbers ended, then the fill on what the cut left, its flat
    /// numbered past it, so that the fill's flat runs on from the cut's where
    /// the face they shared was, and the two are one slope, numbered as
    /// [`Exact::paired`] numbers it. Declined as [`Exact::cut_off`] says, and
    /// `Unsupported` where the kernel left the two flats apart.
    pub(super) fn sloped(
        &self,
        [earlier, later]: [&EdgeName; 2],
        number: u32,
        value: f64,
        tool: &impl Fn(&Wedge, f64, Scale, (DVec3, DVec3)) -> Result<Prism, Declined>,
    ) -> Result<Exact, Declined> {
        let (first_edge, first) = self.wedge(earlier, Wedge::read)?;
        let (second_edge, second) = self.wedge(later, Wedge::read)?;
        let (edge, outward, inward) = if first.inwards {
            (second_edge, &second, earlier)
        } else {
            (first_edge, &first, later)
        };
        let flush = Flush::at(&self.brep, edge, outward);
        let cut = self.cut_or_filled(Some(edge), outward, &flush, value, tool)?;
        let (edge, wedge) = cut.wedge(inward, Wedge::read)?;
        let filled = cut.cut_or_filled(Some(edge), &wedge, &Flush::default(), value, tool)?;
        let [made, lent] = [self.next, self.next + 1].map(|number| number as usize);
        if !filled.named(made).eq(filled.named(lent)) {
            return Err(Declined::Unsupported);
        }
        filled.paired(self, [&first, &second], number)
    }
}
