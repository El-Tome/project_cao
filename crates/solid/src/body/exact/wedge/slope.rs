//! A cut beside a fill (#791): an edge whose faces meet outwards and one
//! whose faces meet inwards, facing each other across the face they share,
//! whose set-backs meet there — a tread's nose and foot set back by half its
//! depth — the face they shared gone, whichever edge is named first (#812).
//! Where their flats lie in one plane, or a hair from it — a riser turned by
//! less than the kernel tells from parallel — they are one slope, the fill
//! laid on the cut's plane; where they lie at an angle — a stair whose front
//! leans — two flats meeting in a crease.

use glam::{DVec2, DVec3};

use super::chain::TANGENT;
use super::flush::Flush;
use super::{Prism, Wedge};
use crate::body::exact::Exact;
use crate::brep::{Declined, EdgeName, Plane, Scale, Surface};
use crate::profile::Contour;

/// Whether the flats of two edges, each on the plane through the lines it
/// is set back to along its two faces — `own` from the one edge, `theirs`
/// from the other — lie in one plane: the other's two lines, where they
/// pass the other edge's point, within `within` of the one's plane.
fn one_plane(one: &Wedge, other: &Wedge, [own, theirs]: [f64; 2], within: f64) -> bool {
    let [near, far] = one.into.map(|into| one.point + into * own);
    let normal = (near - far).cross(one.along).normalize();
    (other.into.iter()).all(|&into| normal.dot(other.point + into * theirs - near).abs() <= within)
}

impl Wedge {
    /// The prism a fill of this wedge is raised over so that its flat lies on
    /// `plane`, the flat of a cut beside it, which holds `along`, the cut's
    /// way: raised along `along` over a section square to it, as
    /// [`Wedge::squared_off`] raises one along the edge's own way, but for its
    /// first run, on `plane`, from a margin behind the face the two edges do
    /// not share to the face of rank `shared`, which they do. That other face
    /// runs along `along` only within what the two edges part by: the join
    /// ends the flat where the plane crosses it, inside the margin.
    fn filled_on(
        &self,
        along: DVec3,
        plane: &Plane,
        shared: usize,
        scale: Scale,
        bounds: (DVec3, DVec3),
    ) -> Prism {
        let square = |way: DVec3| (way - along * along.dot(way)).normalize();
        let straight = Wedge {
            point: self.point,
            along,
            faces: self.faces,
            numbers: self.numbers.clone(),
            outward: self.outward.map(square),
            into: self.into.map(square),
            reach: self.reach,
            dip: self.dip,
            inwards: self.inwards,
        };
        let (across, facing) = straight.square();
        let margin = straight.margin(across, facing, scale);
        let off = margin * straight.off();
        let (frame, travel) = straight.raised_along(across, margin, bounds);
        let height = plane.normal.dot(plane.origin - self.point);
        let on_plane = |from: DVec3, way: DVec3| {
            from + way * ((height - plane.normal.dot(from)) / plane.normal.dot(way))
        };
        let ends = [0, 1].map(|side| {
            let behind = if side == shared { 0.0 } else { off };
            on_plane(straight.outward[side] * behind, straight.into[side])
        });
        let squared =
            |side: usize| (side == shared).then(|| ends[side] + straight.outward[side] * off);
        let mut corners = vec![ends[1], ends[0]];
        corners.extend(squared(0));
        corners.extend(straight.rim(margin));
        corners.extend(squared(1));
        let flat = |at: DVec3| DVec2::new(at.dot(frame.u), at.dot(frame.v));
        let outline = Contour::straight(corners.into_iter().map(flat).collect());
        (outline, frame, travel)
    }
}

impl Exact {
    /// The body with the edges `earlier` and `later` find, one cut off and
    /// the other filled in, where their set-backs meet at `value`: the cut
    /// first, whichever the names give first, by the prism `tool` raises over
    /// it — its prism runs on a margin past its set-back, which would lie
    /// flush on, or cut into, a fill laid before it — its flat numbered where
    /// the steps' numbers ended, then the fill on what the cut left, its flat
    /// numbered past it. Where the two flats lie in one plane, as
    /// [`one_plane`] reads them within what two edges parallel within
    /// [`TANGENT`] part by over the part's reach, the fill is raised on the
    /// cut's plane, as [`Wedge::filled_on`] raises it, so that its flat runs
    /// on from the cut's where the face they shared was, and the two are one
    /// slope; otherwise by the prism `tool` raises over it, the two flats
    /// meeting in a crease. Each flat is then numbered as
    /// [`Exact::paired`] numbers it, by its own edge's rank. Declined as
    /// [`Exact::cut_off`] says, and `Unsupported` where the kernel left the
    /// slope in two faces.
    pub(super) fn sloped(
        &self,
        [earlier, later]: [&EdgeName; 2],
        number: u32,
        value: f64,
        tool: &impl Fn(&Wedge, f64, Scale, (DVec3, DVec3)) -> Result<Prism, Declined>,
    ) -> Result<Exact, Declined> {
        let (first_edge, first) = self.wedge(earlier, Wedge::read)?;
        let (second_edge, second) = self.wedge(later, Wedge::read)?;
        let (edge, outward, inward, name, numbers) = if first.inwards {
            (second_edge, &second, &first, earlier, [self.next, number])
        } else {
            (first_edge, &first, &second, later, [number, self.next])
        };
        let cut = self.cut_or_filled(&Flush::at(&self.brep, edge), outward, value, tool)?;
        let (edge, wedge) = cut.wedge(name, Wedge::read)?;
        let flush = Flush::at(&cut.brep, edge);
        let scale = cut.brep.scale();
        if !one_plane(outward, inward, [value; 2], scale.reach() * TANGENT) {
            let filled = cut.cut_or_filled(&flush, &wedge, value, tool)?;
            return filled.paired(self, [&first, &second], numbers, &[]);
        }
        let made = self.next as usize;
        let plane = (cut.named(made))
            .find_map(|face| match cut.brep.surface(cut.brep.face(face).surface) {
                Surface::Plane(plane) => Some(*plane),
                _ => None,
            })
            .ok_or(Declined::Unsupported)?;
        let shared = (inward.faces.iter())
            .position(|face| outward.faces.contains(face))
            .ok_or(Declined::Unsupported)?;
        let bounds = cut.brep.bounds().ok_or(Declined::Unsupported)?;
        let prism = inward.filled_on(outward.along, &plane, shared, scale, bounds);
        let filled = cut.laid_over(&flush, &wedge, &prism)?;
        if !filled.named(made).eq(filled.named(made + 1)) {
            return Err(Declined::Unsupported);
        }
        filled.paired(self, [&first, &second], numbers, &[])
    }
}
