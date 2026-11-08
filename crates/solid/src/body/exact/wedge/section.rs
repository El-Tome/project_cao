//! The section square to an edge a cut or a fill is raised over: how far it
//! stands off the edge's faces, out of the part for a cut and into it for a
//! fill, which is no more than about the value taken or the margin where
//! that is larger — a result carries the larger reach of its operands, and
//! every later step decides and draws at it — what closes it, and the frame
//! it is drawn in, carried along the edge.

use glam::{DVec2, DVec3};

use super::{Prism, Wedge};
use crate::brep::Scale;
use crate::profile::{Contour, Frame, Run};

/// How far the prism stands off the edge's two faces, past the edge and
/// past the body's box along it, as a share of the distance it is set back:
/// far enough that no face of it lies nearly flush with one of the body's,
/// near enough that the part's reach grows by an eighth of the distance at a
/// square edge.
const MARGIN: f64 = 0.125;

/// The least margin, in square roots of the kernel's relative tolerance
/// over the edge's slant, times the part's reach. A raise refuses a section
/// whose area is under its tolerance times its reach, `RELATIVE · reach²`;
/// however short the distance, a section a margin `m` round the edge holds
/// some `m² · tan γ`, `γ` the angle between either face and the plane
/// through the two lines the cut is set back to — a chamfer's own plane —
/// `tan γ` taken at most one, as at a square edge or a sharper one:
/// at three, nine times what the raise needs. There it grows the part's
/// reach by some 1e-4 of it, and more as the edge flattens.
const SECTION: f64 = 3.0;

/// A line of the section, `a·w + b·h = offset`: `w` across the edge, from
/// the second face's side to the first's, `h` up from the edge along the way
/// between the ways its two faces face.
#[derive(Clone, Copy)]
pub(in crate::body::exact) struct Side {
    pub a: f64,
    pub b: f64,
    pub offset: f64,
}

impl Side {
    pub fn at(self, h: f64) -> DVec2 {
        DVec2::new((self.offset - self.b * h) / self.a, h)
    }

    fn meeting(self, other: Side) -> DVec2 {
        let det = self.a * other.b - other.a * self.b;
        DVec2::new(
            (self.offset * other.b - other.offset * self.b) / det,
            (self.a * other.offset - other.a * self.offset) / det,
        )
    }

    /// The top of a section between the first face's side and the second's,
    /// from the first's: capped a margin above the edge, where the two faces
    /// meet so sharply that the cap is wider than the margin — nothing of
    /// the body near the edge stands higher than the edge does — or else
    /// the corner where the two sides meet.
    pub fn top(first: Side, second: Side, margin: f64) -> Vec<DVec2> {
        let cap = [first.at(margin), second.at(margin)];
        if cap[0].x - cap[1].x > margin {
            cap.to_vec()
        } else {
            vec![first.meeting(second)]
        }
    }
}

impl Wedge {
    /// How far the prism stands off the edge's faces, for a cut set back by
    /// `distance` along them, `across` and `facing` the section's two ways.
    pub(in crate::body::exact) fn margin(
        &self,
        across: DVec3,
        facing: DVec3,
        distance: f64,
        scale: Scale,
    ) -> f64 {
        let first = self.outward[0];
        let slant = (first.dot(across) / first.dot(facing)).abs().min(1.0);
        let least = SECTION * (Scale::RELATIVE / slant).sqrt() * scale.reach();
        (distance * MARGIN).max(least)
    }

    /// Which way the prism's sides stand off the edge's faces, along the
    /// ways they face: out of the matter for a cut, which then meets none of
    /// them but along the face it lays, and into it for a fill, which the
    /// matter then covers but for that face and its ends.
    pub(in crate::body::exact) fn off(&self) -> f64 {
        if self.inwards { -1.0 } else { 1.0 }
    }

    /// What closes a section from the first face's side round to the
    /// second's: the top [`Side::top`] caps a cut with, or, for a fill, the
    /// corner where the two sides meet inside the matter.
    fn closed(&self, first: Side, second: Side, margin: f64) -> Vec<DVec2> {
        if self.inwards {
            vec![first.meeting(second)]
        } else {
            Side::top(first, second, margin)
        }
    }

    /// The prism raised along the edge over a section square to it that
    /// stops at the set-backs: its first run from the second face's set-back
    /// to the first's — round about `centre` where there is one, straight
    /// where there is none — then square off the first face to the margin
    /// outside it, or inside the matter for a fill, along that margin to what
    /// [`Wedge::closed`] closes it with, and back along the second face's
    /// margin and square to the start. Nothing of it runs on along a face
    /// past a set-back, so a set-back at a face's far end leaves nothing
    /// standing past that face. Raised as [`Wedge::raised_along`] raises it.
    pub(in crate::body::exact) fn squared_off(
        &self,
        set_back: f64,
        centre: Option<DVec3>,
        scale: Scale,
        bounds: (DVec3, DVec3),
    ) -> Prism {
        let [first, second] = self.outward;
        let facing = (first + second).normalize();
        let across = (self.into[0] - self.into[1]).normalize();
        let margin = self.margin(across, facing, set_back, scale);
        let off = self.off();
        let side = |normal: DVec3| Side {
            a: normal.dot(across),
            b: normal.dot(facing),
            offset: margin * off,
        };
        let top = self.closed(side(first), side(second), margin);
        let [on_first, on_second] = self.into.map(|into| into * set_back);
        let (frame, travel) = self.raised_along(across, margin, bounds);
        let flat = |at: DVec3| DVec2::new(at.dot(frame.u), at.dot(frame.v));
        let (from, to) = (flat(on_second), flat(on_first));
        let lifted = |at: DVec2| flat(across * at.x + facing * at.y);
        let mut corners = vec![from, to, flat(on_first + first * margin * off)];
        corners.extend(top.into_iter().map(lifted));
        corners.push(flat(on_second + second * margin * off));
        let mut runs = vec![Run::Straight; corners.len()];
        if let Some(centre) = centre {
            let about = flat(centre);
            let (start, end) = (from - about, to - about);
            runs[0] = Run::Round {
                center: about,
                turn: start.perp_dot(end).atan2(start.dot(end)),
            };
        }
        (Contour { corners, runs }, frame, travel)
    }

    /// The frame a section square to the edge is drawn in, `across` its
    /// first axis, and the travel that raises it along the edge's line over
    /// the body's box and `margin` past either end — a fill's, before the
    /// faces at the edge's ends trim it.
    pub(in crate::body::exact) fn raised_along(
        &self,
        across: DVec3,
        margin: f64,
        (low, high): (DVec3, DVec3),
    ) -> (Frame, DVec3) {
        let middle = self.along.dot((low + high) / 2.0 - self.point);
        let half = self.along.abs().dot((high - low) / 2.0) + margin;
        let frame = Frame {
            origin: self.point + self.along * (middle - half),
            u: across,
            v: self.along.cross(across),
        };
        (frame, self.along * 2.0 * half)
    }
}
