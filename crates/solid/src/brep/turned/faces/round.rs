//! The surface an arc of the profile turns into, a sphere or a torus, and
//! the meridian edges a partial turn leaves on its two ends (#576).

use glam::{DVec2, DVec3};

use super::super::Placed;
use crate::brep::curve::Circle;
use crate::brep::laying::{Laying, reversed};
use crate::brep::surface::Surface;
use crate::brep::topology::{Coedge, Face, SurfaceId, VertexId};

/// An arc of the profile in `(h, r)`: from `from`, turned about `center` by
/// `sweep`, to `to`, at `radius`.
pub(super) struct Round {
    pub from: DVec2,
    pub to: DVec2,
    pub center: DVec2,
    pub radius: f64,
    pub sweep: f64,
}

impl Round {
    /// The arc's middle: the bulge no corner stands on.
    pub fn middle(&self) -> DVec2 {
        self.center + DVec2::from_angle(self.sweep / 2.0).rotate(self.from - self.center)
    }

    /// Whether the arc turns into a sphere: its centre laid on the axis.
    pub fn is_sphere(&self) -> bool {
        self.center.y == 0.0
    }
}

/// The sphere or the torus an arc turns into, shared with one already laid
/// that its four corners and its middle, turned, stand on; and whether the
/// matter lies on the side the surface's normal points to. The matter lies
/// on the left of the arc as the contour turns: towards its centre where it
/// turns anticlockwise, which is against the normal, and away from it
/// where it turns clockwise.
pub(super) fn swept(laying: &mut Laying, placed: &Placed, round: &Round) -> (SurfaceId, bool) {
    let middle = round.middle();
    let corners = [
        placed.opening(round.from),
        placed.opening(round.to),
        placed.halfway(round.from),
        placed.halfway(round.to),
        placed.halfway(middle),
    ];
    let surface = if round.is_sphere() {
        Surface::Sphere(placed.sphere(round.center.x, round.radius))
    } else {
        Surface::Torus(placed.torus(round.center, round.radius, middle))
    };
    (laying.surface(surface, &corners), round.sweep < 0.0)
}

/// The circle the arc's meridian runs along on the end the turn opens on,
/// or on the one it closes on: about the arc's centre turned there, of the
/// arc's radius, in that plane; with the direction the arc turns
/// anticlockwise about there.
pub(super) fn meridian(placed: &Placed, round: &Round, closing: bool) -> (Circle, DVec3) {
    let (center, about) = if closing {
        (placed.closing(round.center), placed.closing_outward())
    } else {
        (placed.opening(round.center), placed.across)
    };
    (Circle::about(center, about, round.radius), about)
}

/// The meridian edge of an arc between two corners on an end, used the way
/// the arc turns.
pub(super) fn edge(
    laying: &mut Laying,
    placed: &Placed,
    round: &Round,
    ends: [VertexId; 2],
    closing: bool,
) -> Coedge {
    let (circle, about) = meridian(placed, round, closing);
    laying.round(circle, ends, round.sweep, about)
}

/// The face of a whole circle of the profile turned part of the way: a
/// torus bounded by its two meridian circles, one on each end, with no
/// vertex; and the two circles, as the ends use them.
pub(super) fn ring(
    laying: &mut Laying,
    placed: &Placed,
    round: &Round,
    (surface, flipped): (SurfaceId, bool),
    numbers: Vec<u32>,
) -> (Face, Coedge, Coedge) {
    let (opening, about) = meridian(placed, round, false);
    let at_opening = laying.round_ring(opening, round.sweep, about);
    let (closing, about) = meridian(placed, round, true);
    let at_closing = laying.round_ring(closing, round.sweep, about);
    let face = Face {
        surface,
        flipped,
        loops: vec![vec![at_opening], vec![reversed(at_closing)]],
        numbers,
        poles: Vec::new(),
    };
    (face, at_opening, at_closing)
}
