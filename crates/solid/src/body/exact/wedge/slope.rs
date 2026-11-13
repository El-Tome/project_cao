//! A cut beside a fill (#791): an edge whose faces meet outwards and one
//! whose faces meet inwards, facing each other across the face they share,
//! whose set-backs meet there — a tread's nose and foot set back by half its
//! depth — the face they shared gone, whichever edge is named first (#812).
//! Where their flats lie at an angle the kernel tells from none — a riser
//! turned or leaning by a hair among them — they are one slope, the fill
//! raised on the cut's plane and cut back to the riser's; where it tells it
//! — a stair whose front leans — two flats meeting in a crease. A chain of
//! them, each fill between two cuts — a flight's noses and feet, each tread
//! as deep as each riser is high — is one slope across it, every fill raised
//! on the first cut's plane (#813). The front's upright at an end of the
//! nose is mitred against the slope as against the nose's own flat (#814).

use glam::{DVec2, DVec3};

use super::chain::TANGENT;
use super::end::behind;
use super::flush::Flush;
use super::{Prism, SHORTEST, Wedge, raised_over};
use crate::body::exact::Exact;
use crate::brep::{self, Declined, EdgeId, FaceId, Plane, Scale, Surface};
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
    /// The body with the edges of `chain`, each with its wedge on `given`,
    /// the body as given, and its ends read there, as an edge meeting another
    /// laid before it is — cut off where their faces meet outwards and filled
    /// in where they meet inwards, each two facing each other across a face
    /// they share in a chain, their set-backs meeting there at `value` — on
    /// this body, whatever the list laid before them, the cuts first, then
    /// the fills on what the cuts left, each in the order of the numbers its
    /// faces answer to, whatever order the names give, so that the body is
    /// one whatever it is — a cut's prism runs on a margin past its set-back,
    /// which would lie flush on, or cut into, a fill laid before it — each cut
    /// by the prism `tool` raises over it, the flats numbered from where the steps' numbers
    /// ended in the order they are laid, then each by its own edge's number
    /// in `numbers`, as [`Exact::paired`] numbers them. Where every flat lies
    /// at an angle the kernel tells from none to the first cut's, within
    /// [`TANGENT`] — each facing the way between the ways its edge's two
    /// faces face — they are one slope, every fill raised on that cut's
    /// plane, as [`Exact::filled_on_slope`] lays it. Two flats at an angle
    /// the kernel tells meet in a crease, the fill laid by the prism `tool`
    /// raises. Declined as [`Exact::cut_off`] says, and `Unsupported` before
    /// anything is laid where a chain of more than two holds a crease — a
    /// flight whose front leans, until #926 — where the kernel left the
    /// slope in more faces than one — a chain a hair off one plane, a
    /// flight's riser turned or tipped by a hair, until #929 — or where the
    /// crease's fill left something else than its one face, as
    /// [`Exact::cut_along`] reads it.
    pub(super) fn sloped(
        &self,
        given: &brep::Body,
        chain: &[(EdgeId, &Wedge)],
        numbers: &[u32],
        value: f64,
        tool: &impl Fn(&Wedge, f64, Scale, (DVec3, DVec3)) -> Result<Prism, Declined>,
    ) -> Result<Exact, Declined> {
        let (mut cuts, mut fills): (Vec<usize>, Vec<usize>) =
            (0..chain.len()).partition(|&rank| !chain[rank].1.inwards);
        for laying in [&mut cuts, &mut fills] {
            laying.sort_by_key(|&rank| chain[rank].1.numbers.clone());
        }
        let first = chain[*cuts.first().ok_or(Declined::Unsupported)?].1;
        let facing = first.square().1;
        let crease =
            (chain.iter()).any(|(_, wedge)| wedge.square().1.cross(facing).length() > TANGENT);
        if crease && chain.len() > 2 {
            return Err(Declined::Unsupported);
        }
        let mut laid = self.clone();
        for &rank in &cuts {
            let (edge, wedge) = chain[rank];
            let cut = laid.cut_or_filled(&Flush::at(given, edge), wedge, value, tool)?;
            laid = cut;
        }
        let made = self.next as usize;
        let slope = (laid.named(made))
            .find_map(|face| plane_of(&laid.brep, face))
            .ok_or(Declined::Unsupported)?;
        let cut_faces: Vec<FaceId> = cuts.iter().flat_map(|&rank| chain[rank].1.faces).collect();
        for &rank in &fills {
            let (edge, wedge) = chain[rank];
            let flush = Flush::at(given, edge);
            let filled = if crease {
                let filled = laid.cut_or_filled(&flush, wedge, value, tool)?;
                if !filled.cut_along(&[wedge], &laid, 1, &[], &[], &[]) {
                    return Err(Declined::Unsupported);
                }
                filled
            } else {
                laid.filled_on_slope(given, (&flush, wedge), &cut_faces, (first, &slope))?
            };
            laid = filled;
        }
        let one = (1..chain.len()).all(|more| laid.named(made).eq(laid.named(made + more)));
        if !crease && !one {
            return Err(Declined::Unsupported);
        }
        let order: Vec<usize> = cuts.iter().chain(&fills).copied().collect();
        let wedges: Vec<&Wedge> = order.iter().map(|&rank| chain[rank].1).collect();
        let numbers: Vec<u32> = order.iter().map(|&rank| numbers[rank]).collect();
        laid.paired(self, &wedges, &numbers, &[])
    }

    /// The body with the edge `flush` reads on `given`, the body as given,
    /// as `inward`, filled in on `slope`, the plane of the flat `cut` laid
    /// beside it: raised on that plane, as [`Wedge::filled_on`] raises it,
    /// from the face of `inward` among `cut_faces`, those of the cuts it is
    /// laid with, read on `given` too, its run going on
    /// behind its other face by as far as its edge and the cut's part across
    /// the part, and the shortest set-back the kernel keeps, [`SHORTEST`]
    /// hairs, then cut back to that face's own plane, as [`behind`] cuts it —
    /// so that it ends on that face whichever way it strays from the cut's
    /// edge, and never stands past it, over the face beyond, where the slope
    /// takes it whole. Declined `Unsupported` where neither face of `inward`
    /// is a cut's, or the other is not flat; otherwise as the trim and the
    /// join decline.
    fn filled_on_slope(
        &self,
        given: &brep::Body,
        (flush, inward): (&Flush, &Wedge),
        cut_faces: &[FaceId],
        (cut, slope): (&Wedge, &Plane),
    ) -> Result<Exact, Declined> {
        let shared = (inward.faces.iter())
            .position(|face| cut_faces.contains(face))
            .ok_or(Declined::Unsupported)?;
        let other = inward.faces[1 - shared];
        let riser = plane_of(given, other).ok_or(Declined::Unsupported)?;
        let scale = self.brep.scale();
        let bounds = self.brep.bounds().ok_or(Declined::Unsupported)?;
        let part = cut.along.cross(inward.along).length() * farthest(bounds, inward.point);
        let past = part + SHORTEST * Scale::HAIR * scale.eps();
        let prism = inward.filled_on(cut.along, slope, (shared, past), scale, bounds);
        let behind = behind(&prism, &riser, -inward.outward[1 - shared])?;
        let tool = raised_over(&prism)?.cut_by(&behind)?;
        self.laid_flush(flush, inward, &prism, tool)
    }
}
