//! The prism a chamfer is cut with, raised along the edge over a section
//! that holds the corner the chamfer takes, and stands outside the part by
//! no more than about the distance taken, or the margin where that is
//! larger: a result carries the larger reach of its operands, and every
//! later step decides and draws at it.

use glam::{DVec2, DVec3};

use super::Wedge;
use crate::brep::Scale;
use crate::profile::{Contour, Frame};

/// How far the prism stands off the edge's two faces, past the edge and
/// past the body's box along it, as a share of the distance taken: far
/// enough that no face of it lies nearly flush with one of the body's, near
/// enough that the part's reach grows by an eighth of the distance at a
/// square edge.
const MARGIN: f64 = 0.125;

/// The least margin, in square roots of the kernel's relative tolerance
/// over the edge's slant, times the part's reach. A raise refuses a section
/// whose area is under its tolerance times its reach, `RELATIVE · reach²`;
/// however short the distance, a section a margin `m` round the edge holds
/// some `m² · tan γ`, `γ` the angle between the chamfer's plane and either
/// face, `tan γ` taken at most one, as at a square edge or a sharper one:
/// at three, nine times what the raise needs. There it grows the part's
/// reach by some 1e-4 of it, and more as the edge flattens.
const SECTION: f64 = 3.0;

/// A line of the section, `a·w + b·h = offset`: `w` along the chamfer's
/// plane, from the second face's side to the first's, `h` up from the edge
/// along the way the chamfer faces.
#[derive(Clone, Copy)]
struct Side {
    a: f64,
    b: f64,
    offset: f64,
}

impl Side {
    fn at(self, h: f64) -> DVec2 {
        DVec2::new((self.offset - self.b * h) / self.a, h)
    }

    fn meeting(self, other: Side) -> DVec2 {
        let det = self.a * other.b - other.a * self.b;
        DVec2::new(
            (self.offset * other.b - other.offset * self.b) / det,
            (self.a * other.offset - other.a * self.offset) / det,
        )
    }
}

impl Wedge {
    /// The prism, raised along the edge: in a section square to it, the
    /// corner between the chamfer's plane and the edge's two faces, each
    /// face pushed out by the margin, or by less where the chamfer's plane
    /// lies so near it that its run would end further than the distance, or
    /// the margin, past the set-back; capped a margin above the edge, where
    /// the two faces meet so sharply that the cap is wider than the margin —
    /// nothing of the body near the edge stands higher than the edge does.
    /// Its first run lies on the chamfer's plane. Carried over the body's
    /// box along the edge's line and a margin past either end. None where a
    /// face lies within twice `Scale::HAIR` tolerances of flush with the
    /// chamfer's plane, too near to cut honestly.
    pub(super) fn prism(
        &self,
        distance: f64,
        scale: Scale,
        (low, high): (DVec3, DVec3),
    ) -> Option<(Contour, Frame, DVec3)> {
        let [first, second] = self.outward;
        let facing = (first + second).normalize();
        let [on_first, on_second] = self.into.map(|into| into * distance);
        let across = (on_first - on_second).normalize();
        let height = -facing.dot(on_first);
        let slant = (first.dot(across) / first.dot(facing)).min(1.0);
        let least = SECTION * (Scale::RELATIVE / slant).sqrt() * scale.reach();
        let margin = (distance * MARGIN).max(least);
        let flush = 2.0 * Scale::HAIR * scale.eps();
        let side = |normal: DVec3| {
            let (a, b) = (normal.dot(across), normal.dot(facing));
            let offset = margin.min(distance.max(margin) * a.abs());
            (offset >= flush).then_some(Side { a, b, offset })
        };
        let (first, second) = (side(first)?, side(second)?);
        let ends = [second.at(-height), first.at(-height)];
        let cap = [first.at(margin), second.at(margin)];
        let section = if cap[0].x - cap[1].x > margin {
            vec![ends[0], ends[1], cap[0], cap[1]]
        } else {
            vec![ends[0], ends[1], first.meeting(second)]
        };
        let middle = self.along.dot((low + high) / 2.0 - self.point);
        let half = self.along.abs().dot((high - low) / 2.0) + margin;
        let frame = Frame {
            origin: self.point + self.along * (middle - half),
            u: across,
            v: self.along.cross(across),
        };
        let up = facing.dot(frame.v);
        let outline = Contour::straight(
            section
                .into_iter()
                .map(|at| DVec2::new(at.x, at.y * up))
                .collect(),
        );
        Some((outline, frame, self.along * 2.0 * half))
    }
}
