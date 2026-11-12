//! A cut beside a fill (#791): an edge whose faces meet outwards and one
//! whose faces meet inwards, facing each other across the face they share,
//! whose set-backs meet there — a tread's nose and foot set back by half its
//! depth — the face they shared gone, whichever edge is named first (#812).
//! Where their flats lie at an angle the kernel tells from none — a riser
//! turned or leaning by a hair among them — they are one slope; where it
//! tells it — a stair whose front leans — two flats meeting in a crease.

use glam::{DVec2, DVec3};

use super::chain::TANGENT;
use super::flush::Flush;
use super::{Prism, Wedge};
use crate::body::exact::Exact;
use crate::brep::{Declined, EdgeName, Plane, Scale, Surface};
use crate::profile::Contour;

/// The way the flat a chamfer lays on `wedge` faces, whatever its value:
/// square to the plane through the lines it is set back to along its two
/// faces, which runs along the edge.
fn normal(wedge: &Wedge) -> DVec3 {
    (wedge.into[0] - wedge.into[1])
        .cross(wedge.along)
        .normalize()
}

/// Whether the flats of two edges set back by `value` lie in one plane: the
/// other's two lines, where they pass the other edge's point, within
/// `within` of the one's plane.
fn one_plane(one: &Wedge, other: &Wedge, value: f64, within: f64) -> bool {
    let near = one.point + one.into[0] * value;
    let normal = normal(one);
    (other.into.iter()).all(|&into| normal.dot(other.point + into * value - near).abs() <= within)
}

/// How far a point of the box from `low` to `high` may stand from `point`.
fn farthest((low, high): (DVec3, DVec3), point: DVec3) -> f64 {
    (((low + high) / 2.0 - point).abs() + (high - low) / 2.0).length()
}

impl Wedge {
    /// The prism a fill of this wedge is raised over so that its flat lies on
    /// `plane`, the flat of a cut beside it, which holds `along`, the cut's
    /// way: raised along `along` over a section square to it, as
    /// [`Wedge::squared_off`] raises one along the edge's own way, its first
    /// run on `plane` from the face the two edges do not share to the face of
    /// rank `shared`, which they do. That other face runs along `along` only
    /// within what the two edges part by: the run goes on `past` behind the
    /// line it meets that face on at this edge's point, and the join ends the
    /// flat where the plane crosses the face.
    fn filled_on(
        &self,
        along: DVec3,
        plane: &Plane,
        (shared, past): (usize, f64),
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
        let off = straight.off();
        let (frame, travel) = straight.raised_along(across, margin, bounds);
        let height = plane.normal.dot(plane.origin - self.point);
        let on_plane = |from: DVec3, way: DVec3| {
            from + way * ((height - plane.normal.dot(from)) / plane.normal.dot(way))
        };
        let [first, second] = [0, 1].map(|side| {
            let behind = if side == shared { 0.0 } else { past };
            let outward = straight.outward[side] * off;
            let end = on_plane(outward * behind, straight.into[side]);
            [end, end + outward * (margin - behind)]
        });
        let mut corners = vec![second[0], first[0], first[1]];
        corners.extend(straight.rim(margin));
        corners.push(second[1]);
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
    /// numbered past it, each flat then numbered by its own edge's rank, as
    /// [`Exact::paired`] numbers them. Where the two flats lie at an angle
    /// the kernel tells from none, within [`TANGENT`], they are one slope:
    /// the fill by the prism `tool` raises over it where its flat lies in the
    /// cut's plane, as [`one_plane`] reads it, within the kernel's tolerance
    /// all along the part, and otherwise raised on the cut's plane, as
    /// [`Wedge::filled_on`] raises it, its run going on behind the face the
    /// two do not share by as far as their edges part across the part, and a
    /// hair. Anywhere else the fill is laid by the prism `tool` raises, and
    /// the two flats meet in a crease. Declined as [`Exact::cut_off`] says,
    /// and `Unsupported` where the kernel left the slope in two faces, or the
    /// crease's fill something else than its one face, as
    /// [`Exact::cut_along`] reads it.
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
        if normal(outward).cross(normal(inward)).length() > TANGENT {
            let filled = cut.cut_or_filled(&flush, &wedge, value, tool)?;
            if !filled.cut_along(&[&wedge], &cut, 1, &[], &[], &[]) {
                return Err(Declined::Unsupported);
            }
            return filled.paired(self, [&first, &second], numbers, &[]);
        }
        let scale = cut.brep.scale();
        let eps = scale.eps();
        let bounds = cut.brep.bounds().ok_or(Declined::Unsupported)?;
        let part = outward.along.cross(inward.along).length() * farthest(bounds, inward.point);
        let made = self.next as usize;
        let filled = if part <= eps && one_plane(outward, inward, value, eps) {
            cut.cut_or_filled(&flush, &wedge, value, tool)?
        } else {
            let plane = (cut.named(made))
                .find_map(|face| match cut.brep.surface(cut.brep.face(face).surface) {
                    Surface::Plane(plane) => Some(*plane),
                    _ => None,
                })
                .ok_or(Declined::Unsupported)?;
            let shared = (inward.faces.iter())
                .position(|face| outward.faces.contains(face))
                .ok_or(Declined::Unsupported)?;
            let past = part + Scale::HAIR * eps;
            let prism = inward.filled_on(outward.along, &plane, (shared, past), scale, bounds);
            cut.laid_over(&flush, &wedge, &prism)?
        };
        if !filled.named(made).eq(filled.named(made + 1)) {
            return Err(Declined::Unsupported);
        }
        filled.paired(self, [&first, &second], numbers, &[])
    }
}
