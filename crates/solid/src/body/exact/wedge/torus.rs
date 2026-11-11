//! The piece of torus a fillet joins two feet's rounds through round the
//! round of the upright above them (#829): about the upright round's axis,
//! its tube of the value's radius, its centre circle twice the value from
//! that axis and the value up it, tangent to both feet's rounds, to the face
//! the feet share and to the upright's round. No boolean lays it, as none
//! lays #699's piece of sphere: the corner is first filled up to the planes
//! through the axis square to the feet, from the feet's face to the value up
//! the axis, out to twice the value from it; every edge is then laid against
//! that as alone — each foot's round trimmed flush on the plane it meets, the
//! upright's stopping flush on the top of the filling — which leaves four
//! faces of the filling between four arcs, and those four are replaced by
//! the piece of torus the arcs bound. Where the odd edge is concave, at the
//! top of an L's inner corner, the mirror — a piece cut out about a fill's
//! axis — is still to come.

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
    /// numbers; every edge then followed onto what that leaves, as
    /// [`Exact::following`] finds it, and laid there, as
    /// [`Exact::in_turn`] lays them with `kind`, each face laid on an edge
    /// numbered where the steps' numbers ended plus its rank in the list;
    /// and each corner closed by its piece of torus, as [`Exact::torused`]
    /// lays it, every patch — a piece of torus, or the face closing a corner
    /// three edges of the list meet at — numbered past them in the order of
    /// its corner's ranks, as [`ranked_patches`] numbers them. Declined
    /// `Unsupported` where an odd edge is concave, or any face of a filling
    /// is left once every corner is closed; otherwise as the filling, the
    /// laying and the closing decline.
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
            filled = filled.joined(&filling(corner, &wedges[corner.lone], taken)?)?;
            fillings.push(from..filled.next);
        }
        let followed = (names.iter().zip(wedges))
            .map(|(name, wedge)| filled.following(name, wedge))
            .collect::<Result<Vec<_>, Declined>>()?;
        let step = filled.step(&followed, kind)?;
        let closings: Vec<[usize; 3]> = step.three.iter().map(|&(_, ranks, _)| ranks).collect();
        let (laid, _, pairs) = filled.in_turn(&followed, step, taken, kind)?;
        let lent = filled.next - start;
        let laid_on = filled.next..laid.next;
        let set_aside = laid.renamed_by(|number| {
            let filling = fillings
                .iter()
                .position(|numbers| numbers.contains(&number));
            match filling {
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

    /// The body with the faces answering to `left`, which a filling left at
    /// `corner`, replaced by the piece of torus of the value's radius about the
    /// round of its odd edge, read as `wedge`, numbered where the steps'
    /// numbers end and facing its tube's centre circle, the matter outside the
    /// tube. Declined `Unsupported` where those faces make no one loop of four
    /// edges towards the rest of the body, or the piece left anything but its
    /// face, its every corner a corner of one of the three `rounds`, as
    /// [`Exact::cut_along`] reads it; `Unverified` where the loop strays off
    /// the torus, as [`crate::brep::Body::replaced`] reads it; `Undrawn`
    /// where the piece cannot be cut into triangles.
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
            flipped: true,
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
/// read as `wedge`, crosses the face the two others share: the value in from
/// both its faces, square to the edge from the corner.
fn axis_at(corner: &Odd, wedge: &Wedge, value: f64) -> DVec3 {
    let [first, second] = wedge.outward;
    corner.at - (first + second) * (value / (1.0 + first.dot(second)))
}

/// What fills `corner` before its edges are laid, its odd edge read as
/// `wedge`, for the value `value`: the sector about the axis of that edge's
/// round between the ways its two faces face, out to twice the value from the
/// axis, raised from the face the two others share up the odd edge by the
/// value. Its two flat sides lie on the planes through the axis square to the
/// two others, where their rounds end; its top on the plane where the odd
/// edge's round ends. Declined `Unsupported` where the odd edge is concave,
/// a fill whose mirror is still to come; otherwise as the raise declines.
fn filling(corner: &Odd, wedge: &Wedge, value: f64) -> Result<Exact, Declined> {
    if wedge.inwards {
        return Err(Declined::Unsupported);
    }
    let [first, second] = wedge.outward;
    let frame = Frame {
        origin: axis_at(corner, wedge, value),
        u: first,
        v: corner.up.cross(first),
    };
    let to = DVec2::new(second.dot(frame.u), second.dot(frame.v));
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
