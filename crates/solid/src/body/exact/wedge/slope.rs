//! A cut beside a fill (#791): an edge whose faces meet outwards and one
//! whose faces meet inwards, facing each other across the face they share,
//! whose set-backs meet there — a tread's nose and foot set back by half its
//! depth — the face they shared gone, whichever edge is named first (#812).
//! Where their flats lie at an angle the kernel tells from none — a riser
//! turned or leaning by a hair among them — they are one slope, the fill
//! raised on the cut's plane and cut back to the riser's; where it tells it
//! — a stair whose front leans — two flats meeting in a crease.

use glam::{DVec2, DVec3};

use super::chain::TANGENT;
use super::end::behind;
use super::flush::Flush;
use super::{Prism, SHORTEST, Wedge, raised_over};
use crate::body::exact::Exact;
use crate::brep::{self, Declined, EdgeName, FaceId, Plane, Scale, Surface};
use crate::profile::Contour;

/// The plane of the face `id`, where it is flat.
fn plane_of(body: &brep::Body, id: FaceId) -> Option<Plane> {
    match body.surface(body.face(id).surface) {
        Surface::Plane(plane) => Some(*plane),
        _ => None,
    }
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
    /// line it meets that face on at this edge's point, for the prism to be
    /// cut back to the face's own plane, as [`Exact::sloped`] cuts it.
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
    /// the kernel tells from none, within [`TANGENT`] — each facing the way
    /// between the ways its edge's two faces face — they are one slope: the
    /// fill raised on the cut's plane, as [`Wedge::filled_on`] raises it, its
    /// run going on behind the face the two do not share by as far as their
    /// edges part across the part, and the shortest set-back the kernel
    /// keeps, [`SHORTEST`] hairs, then cut back to that face's own plane, as
    /// [`behind`] cuts it — so that it ends on that face whichever way it
    /// strays from the cut's edge, and never stands past it, over the face
    /// beyond, where the slope takes it whole. Anywhere else the fill is laid
    /// by the prism `tool` raises, and the two flats meet in a crease.
    /// Declined as [`Exact::cut_off`] says, and `Unsupported` where the
    /// kernel left the slope in two faces, or the crease's fill something
    /// else than its one face, as [`Exact::cut_along`] reads it.
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
        let [cut_facing, fill_facing] = [outward, inward].map(|wedge| wedge.square().1);
        if cut_facing.cross(fill_facing).length() > TANGENT {
            let filled = cut.cut_or_filled(&flush, &wedge, value, tool)?;
            if !filled.cut_along(&[&wedge], &cut, 1, &[], &[], &[]) {
                return Err(Declined::Unsupported);
            }
            return filled.paired(self, [&first, &second], numbers, &[]);
        }
        let made = self.next as usize;
        let slope = (cut.named(made))
            .find_map(|face| plane_of(&cut.brep, face))
            .ok_or(Declined::Unsupported)?;
        let shared = (inward.faces.iter())
            .position(|face| outward.faces.contains(face))
            .ok_or(Declined::Unsupported)?;
        let riser = plane_of(&self.brep, inward.faces[1 - shared]).ok_or(Declined::Unsupported)?;
        let scale = cut.brep.scale();
        let bounds = cut.brep.bounds().ok_or(Declined::Unsupported)?;
        let part = outward.along.cross(inward.along).length() * farthest(bounds, inward.point);
        let past = part + SHORTEST * Scale::HAIR * scale.eps();
        let prism = inward.filled_on(outward.along, &slope, (shared, past), scale, bounds);
        let behind = behind(&prism, &riser, -inward.outward[1 - shared])?;
        let tool = raised_over(&prism)?.cut_by(&behind)?;
        let filled = cut.laid_flush(&flush, &wedge, &prism, tool)?;
        if !filled.named(made).eq(filled.named(made + 1)) {
            return Err(Declined::Unsupported);
        }
        filled.paired(self, [&first, &second], numbers, &[])
    }
}
