//! The piece of torus a fillet joins two rounds through round the round of
//! the odd edge between them, at a corner of mixed sense (#829): about the odd
//! round's axis, its tube of the value's radius, its centre circle twice the
//! value from that axis and the value along it from the face the two others
//! share, tangent to both their rounds, to that face and to the odd edge's
//! round. No boolean lays it, as none lays #699's piece of sphere. At a
//! standing block's foot corner the corner is first filled up to the planes
//! through the upright round's axis square to the feet, from the lower top to
//! the value up the axis, out to twice the value from it; every edge is then
//! laid against that as alone — each foot's round trimmed flush on the plane
//! it meets, the upright's stopping flush on the top of the filling — which
//! leaves four faces of the filling between four arcs, replaced by the piece
//! of torus the arcs bound, facing its tube's centre. At the top of an L's
//! inner corner, the mirror: the corner is first propped between the walls
//! and those two planes, from the top down by the value, so that each top
//! edge's round stops flush on its plane and the upright's fill ends below
//! the prop; once every edge is laid, the sector the filling would fill is
//! cut away instead, prop and all, and the four faces it leaves are replaced
//! by the piece of torus facing away from its tube's centre, the matter
//! inside the tube.

use std::f64::consts::FRAC_1_SQRT_2;
use std::ops::Range;

use glam::{DVec2, DVec3};

use super::Wedge;
use super::odd::{Odd, ranked_patches};
use super::overlap::OVERLAPS;
use super::pairs::Pair;
use super::sphere::lap;
use super::step::Kind;
use crate::body::exact::Exact;
use crate::brep::{Declined, EdgeName, Face, FaceId, Surface, SurfaceId, Torus};
use crate::profile::{Contour, Frame, Run};

impl Exact {
    /// The body with the edges `names` find laid at `taken`, each corner of
    /// mixed sense `odd` holds gone round the round of its odd edge, its wedge
    /// on this body by its rank in `wedges`: each corner filled as
    /// [`filling`] fills it, the filling's faces numbered past the steps'
    /// numbers and merged off any face of the body they lie on; every edge
    /// then followed onto what that leaves, as [`Exact::following`] finds it,
    /// and laid there, as [`Exact::in_turn`] lays them with `kind`, each face
    /// laid on an edge numbered where the steps' numbers ended plus its rank
    /// in the list; what [`clearing`] holds then cut from each corner; and
    /// each corner closed by its piece of torus, as [`Exact::torused`] lays
    /// it, every patch — a piece of torus, or the face closing a corner three
    /// edges of the list meet at — numbered past them in the order of its
    /// corner's ranks, as [`ranked_patches`] numbers them. Declined
    /// `Unsupported` where joining the fillings, or cutting the clearings,
    /// takes away a face of the body it is laid on but a filling's — a block
    /// standing where a corner's patch sweeps, which the filling would
    /// swallow or the clearing cut, and the piece of torus clear with its tube
    /// or fill — or any face of a filling or a clearing is left once every
    /// corner is closed; otherwise as the filling, the laying, the clearing
    /// and the closing decline.
    pub(super) fn through_a_torus(
        &self,
        names: &[EdgeName],
        wedges: &[Wedge],
        odd: &[Odd],
        taken: f64,
        kind: &Kind,
    ) -> Result<(Exact, f64, Vec<Pair>), Declined> {
        let start = self.next;
        let mut filled = self.clone();
        let mut fillings: Vec<Range<u32>> = Vec::new();
        for corner in odd {
            let from = filled.next;
            let joined = filled.joined(&filling(corner, &wedges[corner.lone], taken)?)?;
            let next = joined.next;
            let merged = Exact {
                next: from,
                ..joined
            }
            .merged_at(&[])?;
            filled = Exact { next, ..merged };
            fillings.push(from..next);
        }
        if !self.kept_on(&filled, &[]) {
            return Err(Declined::Unsupported);
        }
        let followed = (names.iter().zip(wedges))
            .map(|(name, wedge)| filled.following(name, wedge))
            .collect::<Result<Vec<_>, Declined>>()?;
        let step = filled.step(&followed, kind)?;
        let closings: Vec<[usize; 3]> = step.three.iter().map(|&(_, ranks, _)| ranks).collect();
        let (laid, _, pairs) = filled.in_turn(&followed, step, taken, kind)?;
        let lent = filled.next - start;
        let laid_on = filled.next..laid.next;
        let mut cleared = laid;
        let mut clearings: Vec<Range<u32>> = Vec::new();
        for corner in odd {
            let from = cleared.next;
            if let Some(sector) = clearing(corner, &wedges[corner.lone], taken)? {
                let cut = cleared.cut_by(&sector)?;
                if !cleared.kept_on(&cut, &fillings) {
                    return Err(Declined::Unsupported);
                }
                cleared = cut;
            }
            clearings.push(from..cleared.next);
        }
        let set_aside = cleared.renamed_by(|number| {
            let corner = (0..odd.len())
                .find(|&at| fillings[at].contains(&number) || clearings[at].contains(&number));
            match corner {
                Some(corner) => OVERLAPS + corner as u32,
                None if laid_on.contains(&number) => number - lent,
                None => number,
            }
        });
        let mut closed = Exact {
            next: laid_on.end - lent,
            ..set_aside
        };
        let after = start + names.len() as u32;
        let mut patches: Vec<([usize; 3], u32)> = (closings.into_iter().enumerate())
            .map(|(index, ranks)| (ranks, after + index as u32))
            .collect();
        for (index, corner) in odd.iter().enumerate() {
            let rounds = corner.ranks.map(|rank| start + rank as u32);
            let left = OVERLAPS + index as u32;
            patches.push((corner.ranks, closed.next));
            closed = closed.torused(left, corner, &wedges[corner.lone], taken, rounds)?;
        }
        closed.overlaps_covered()?;
        let moved = ranked_patches(patches, after);
        let renamed = closed.renamed_by(|number| moved.get(&number).copied().unwrap_or(number));
        Ok((renamed, taken, pairs))
    }

    /// Whether every face this body holds still stands on `on` but those
    /// numbered in `fillings`: what a join or a cut took away.
    fn kept_on(&self, on: &Exact, fillings: &[Range<u32>]) -> bool {
        (self.brep.face_ids())
            .flat_map(|face| self.brep.numbers(face))
            .filter(|number| !fillings.iter().any(|numbers| numbers.contains(number)))
            .all(|&number| on.has_face(number as usize))
    }

    /// The body with the faces answering to `left`, which a filling or a
    /// clearing left at `corner`, replaced by the piece of torus of the value's
    /// radius about the round of its odd edge, read as `wedge`, numbered where
    /// the steps' numbers end: facing its tube's centre circle, the matter
    /// outside the tube, where the odd edge is convex and the two others
    /// filled; facing away from it, the matter inside, where the odd edge is
    /// concave and the two others cut. Declined `Unsupported` where those
    /// faces make no one loop of four edges towards the rest of the body, or
    /// the piece left anything but its face, its every corner a corner of one
    /// of the three `rounds`, as [`Exact::cut_along`] reads it; `Unverified`
    /// where the loop strays off the torus, as
    /// [`crate::brep::Body::replaced`] reads it; `Undrawn` where the piece
    /// cannot be cut into triangles.
    fn torused(
        &self,
        left: u32,
        corner: &Odd,
        wedge: &Wedge,
        value: f64,
        rounds: [u32; 3],
    ) -> Result<Exact, Declined> {
        let gone: Vec<FaceId> = self.named(left as usize).collect();
        let lap = lap(&self.brep, &gone)
            .filter(|lap| lap.len() == 4)
            .ok_or(Declined::Unsupported)?;
        let tube = DVec2::new(value, 2.0 * value);
        let middle = tube - DVec2::splat(value * FRAC_1_SQRT_2);
        let axis = axis_at(corner, wedge, value);
        let torus = Torus::through(axis, corner.up, tube, value, middle);
        let face = Face {
            surface: SurfaceId(0),
            flipped: !wedge.inwards,
            loops: vec![lap],
            numbers: vec![self.next],
            poles: Vec::new(),
        };
        let closed = self.patched(&gone, Surface::Torus(torus), face, self.next + 1)?;
        if closed.cut_along(&[], self, 1, &rounds, &[]) {
            Ok(closed)
        } else {
            Err(Declined::Unsupported)
        }
    }
}

/// Where the axis of the round of value `value` on the odd edge of `corner`,
/// read as `wedge`, crosses the face the two others share: the value from
/// both its faces, square to the edge from the corner — inside the matter for
/// a cut, in the air for a fill.
fn axis_at(corner: &Odd, wedge: &Wedge, value: f64) -> DVec3 {
    let [first, second] = wedge.outward;
    corner.at - (first + second) * (value * wedge.off() / (1.0 + first.dot(second)))
}

/// The ways from the axis of the odd edge's round, as [`axis_at`] reads it,
/// towards where it touches its two faces: the ways they face for a cut,
/// against them for a fill.
fn ways(wedge: &Wedge) -> [DVec3; 2] {
    wedge.outward.map(|outward| outward * wedge.off())
}

/// What fills `corner` before its edges are laid, its odd edge read as
/// `wedge`, for the value `value`, raised from the face the two others share
/// along the odd edge by the value. Where the odd edge is convex, the sector
/// [`sector`] raises: its two flat sides on the planes through the axis
/// square to the two others, where their rounds end, its top where the odd
/// edge's round ends. Where it is concave, what props the corner between its
/// faces and those two planes, from the corner to the axis: each of the two
/// others stops flush on its plane, the odd edge's fill ends under it, and
/// the clearing takes it away. Declined as the raise declines.
fn filling(corner: &Odd, wedge: &Wedge, value: f64) -> Result<Exact, Declined> {
    if !wedge.inwards {
        return sector(corner, wedge, value);
    }
    let (frame, to) = framed(corner, wedge, value);
    let at = corner.at - frame.origin;
    let outline = Contour::straight(vec![
        DVec2::new(at.dot(frame.u), at.dot(frame.v)),
        DVec2::X * value,
        DVec2::ZERO,
        to * value,
    ]);
    Exact::raised(&outline, &[], frame, corner.up * value)
}

/// What is cut from `corner` once its edges are laid: where its odd edge,
/// read as `wedge`, is concave, the sector [`sector`] raises for `value`,
/// which the piece of torus then fills back within its tube; nothing where
/// it is convex, its sector filled first.
fn clearing(corner: &Odd, wedge: &Wedge, value: f64) -> Result<Option<Exact>, Declined> {
    if wedge.inwards {
        sector(corner, wedge, value).map(Some)
    } else {
        Ok(None)
    }
}

/// The sector about the axis of the round of value `value` on the odd edge
/// of `corner`, read as `wedge`, between the [`ways`] towards its two faces,
/// out to twice the value from the axis, raised from the face the two others
/// share along the odd edge by the value. Declined as the raise declines.
fn sector(corner: &Odd, wedge: &Wedge, value: f64) -> Result<Exact, Declined> {
    let (frame, to) = framed(corner, wedge, value);
    let outline = Contour {
        corners: vec![DVec2::ZERO, DVec2::X * 2.0 * value, to * 2.0 * value],
        runs: vec![
            Run::Straight,
            Run::Round {
                center: DVec2::ZERO,
                turn: to.y.atan2(to.x),
            },
            Run::Straight,
        ],
    };
    Exact::raised(&outline, &[], frame, corner.up * value)
}

/// The frame on the face the two others share at the axis of the odd edge's
/// round of value `value`, its first axis the first of the [`ways`], and the
/// second of them in it.
fn framed(corner: &Odd, wedge: &Wedge, value: f64) -> (Frame, DVec2) {
    let [first, second] = ways(wedge);
    let frame = Frame {
        origin: axis_at(corner, wedge, value),
        u: first,
        v: corner.up.cross(first),
    };
    (frame, DVec2::new(second.dot(frame.u), second.dot(frame.v)))
}
