//! The cut a fillet makes: a prism raised along the edge over the corner
//! between the edge's two faces and the round tangent to both, cut from the
//! body. Its wall on the round is the one face the step names.

use glam::{DVec2, DVec3};

use super::Exact;
use super::wedge::{Prism, Side, Wedge};
use crate::brep::{Declined, EdgeName, Scale};
use crate::profile::{Contour, Run};

impl Exact {
    /// The body with the edge a name finds rounded off by a cylinder of
    /// `radius`, or of the largest radius the edge's two faces give, tangent
    /// to both, and the radius taken: each face stops where the round takes
    /// over, set back by the radius over the tangent of half the angle they
    /// meet at. The round is numbered where the steps' numbers ended. What is
    /// declined is what [`Exact::chamfered`] declines.
    pub fn filleted(&self, name: &EdgeName, radius: f64) -> Result<(Exact, f64), Declined> {
        self.cut_off(
            name,
            radius,
            |wedge| wedge.reach * wedge.half_tangent(),
            Wedge::rounded,
        )
    }
}

impl Wedge {
    /// The tangent of half the angle between the edge's two faces inside
    /// the matter: what a radius is over the set-back of its round.
    fn half_tangent(&self) -> f64 {
        let [first, second] = self.into;
        let across = (first - second).normalize();
        let inward = (first + second).normalize();
        first.dot(across) / first.dot(inward)
    }

    /// The prism, raised along the edge: in a section square to it, the
    /// round from where it touches the second face to where it touches the
    /// first, its first run, then square off the first face to a margin
    /// outside it, along that margin to the top [`Side::top`] caps, and back
    /// along the second face's margin and square down to the round. Carried
    /// over the body's box along the edge's line and a margin past either
    /// end.
    fn rounded(&self, radius: f64, scale: Scale, bounds: (DVec3, DVec3)) -> Option<Prism> {
        let [first, second] = self.outward;
        let facing = (first + second).normalize();
        let across = (self.into[0] - self.into[1]).normalize();
        let set_back = radius / self.half_tangent();
        let margin = self.margin(across, facing, set_back, scale);
        let side = |normal: DVec3| Side {
            a: normal.dot(across),
            b: normal.dot(facing),
            offset: margin,
        };
        let top = Side::top(side(first), side(second), margin);
        let [on_first, on_second] = self.into.map(|into| into * set_back);
        let centre = on_first - first * radius;
        let (frame, travel) = self.raised_along(across, margin, bounds);
        let flat = |at: DVec3| DVec2::new(at.dot(frame.u), at.dot(frame.v));
        let (from, to, about) = (flat(on_second), flat(on_first), flat(centre));
        let (start, end) = (from - about, to - about);
        let turn = start.perp_dot(end).atan2(start.dot(end));
        let lifted = |at: DVec2| flat(across * at.x + facing * at.y);
        let mut corners = vec![from, to, flat(on_first + first * margin)];
        corners.extend(top.into_iter().map(lifted));
        corners.push(flat(on_second + second * margin));
        let mut runs = vec![Run::Straight; corners.len()];
        runs[0] = Run::Round {
            center: about,
            turn,
        };
        Some((Contour { corners, runs }, frame, travel))
    }
}
