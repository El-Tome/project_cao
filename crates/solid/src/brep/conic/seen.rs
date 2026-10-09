//! A conic seen in the parameters of its own plane, `(s, t)`, and of its own
//! wall, `(θ, h)` on a cylinder and `(θ, l)` on a cone, with `θ` unwrapped
//! from the anchor so that it never jumps by a turn along the curve.
//!
//! The curve is a graph over its wall's angle, which grows with `τ`: on a
//! cylinder exactly as `θ_A + √k·τ`, the eccentric angle of an affine circle;
//! on a cone it is read off the point, and unwrapped by the side of the anchor
//! the point stands on, since the angle turns less than half a turn either
//! way from a vertex or from within an arm.

use std::f64::consts::{PI, TAU};

use glam::{DVec2, DVec3};

use super::built::Section;
use super::{Conic, On, Round};

impl Conic {
    /// The point at `t` in the parameters of the plane or of the wall, with
    /// its first and second derivatives with respect to `t`.
    pub fn seen_on(&self, on: On, t: f64) -> [DVec2; 3] {
        match on {
            On::Plane => self.on_plane(t),
            On::Wall => self.on_wall(t),
        }
    }

    fn on_plane(&self, t: f64) -> [DVec2; 3] {
        let plane = &self.plane;
        let [speed, bend] = [self.derivative(t), self.bending(t)];
        let flat = |v: DVec3| DVec2::new(v.dot(plane.u), v.dot(plane.v));
        [plane.parameters(self.point(t)), flat(speed), flat(bend)]
    }

    fn on_wall(&self, t: f64) -> [DVec2; 3] {
        let [point, speed, bend] = [self.point(t), self.derivative(t), self.bending(t)];
        match self.round {
            Round::Cylinder(cylinder) => {
                let root = self.k.sqrt();
                let start = cylinder.parameters(self.anchor).x;
                let axis = cylinder.axis;
                [
                    DVec2::new(start + root * t, (point - cylinder.origin).dot(axis)),
                    DVec2::new(root, speed.dot(axis)),
                    DVec2::new(0.0, bend.dot(axis)),
                ]
            }
            Round::Cone(cone) => {
                let from = point - cone.origin;
                let height = from.dot(cone.axis);
                let across = |v: DVec3| DVec2::new(v.dot(cone.u), v.dot(cone.v));
                let [at, run, sway] = [across(from), across(speed), across(bend)];
                let square = at.length_squared();
                let rho = square.sqrt();
                let turn = at.perp_dot(run) / square;
                let sway_turn = at.perp_dot(sway) / square - 2.0 * turn * at.dot(run) / square;
                let out = at.dot(run) / rho;
                let out_sway = (run.length_squared() + at.dot(sway) - out * out) / rho;
                let [w_h, w_rho] = cone.ruling.to_array();
                let length = (DVec2::new(height, rho) - cone.foot).dot(cone.ruling);
                [
                    DVec2::new(self.angle(t, at.y.atan2(at.x)), length),
                    DVec2::new(turn, w_h * speed.dot(cone.axis) + w_rho * out),
                    DVec2::new(sway_turn, w_h * bend.dot(cone.axis) + w_rho * out_sway),
                ]
            }
        }
    }

    /// The wall's angle at `t`, from its principal value `raw`: on the side
    /// of the anchor's angle that `t` stands on, and as many turns on as the
    /// periods `t` runs.
    fn angle(&self, t: f64, raw: f64) -> f64 {
        let start = self.anchor_angle();
        let (turns, within) = match self.period() {
            Some(period) => {
                let turns = (t / period).round();
                (turns, t - turns * period)
            }
            None => (0.0, t),
        };
        let mut turned = raw - start;
        turned -= TAU * (turned / TAU).round();
        if turned * within < 0.0 && turned.abs() > PI / 2.0 {
            turned += TAU * within.signum();
        }
        start + turned + TAU * turns
    }

    fn anchor_angle(&self) -> f64 {
        match self.round {
            Round::Cylinder(cylinder) => cylinder.parameters(self.anchor).x,
            Round::Cone(cone) => cone.parameters(self.anchor).x,
        }
    }

    /// The point of the cut on the ruling at the wall's angle `theta`, its
    /// part across the axis that angle's radial direction times a length, so
    /// that it stands at that very angle to rounding, and its parameter: on a
    /// cylinder `(θ − θ_A)/√k`, on a cone read back off the point, as many
    /// periods on as `theta` turns.
    pub fn at_angle(&self, theta: f64) -> (f64, DVec3) {
        let point = Section::of(&self.plane, &self.round).point(theta);
        let t = match self.round {
            Round::Cylinder(_) => (theta - self.anchor_angle()) / self.k.sqrt(),
            Round::Cone(_) => {
                let t = self.parameter(point);
                let seen = self.seen_on(super::On::Wall, t)[0].x;
                self.period()
                    .map_or(t, |period| t + period * ((theta - seen) / TAU).round())
            }
        };
        (t, point)
    }

    /// The wall's angle a stretch turns through, signed as it goes.
    pub fn swept(&self, from: f64, to: f64) -> f64 {
        self.on_wall(to)[0].x - self.on_wall(from)[0].x
    }

    /// How far a stretch stands from its wall's axis at most: a cylinder's
    /// radius; on a cone, at an end, or at the far vertex of an ellipse where
    /// the stretch passes it, `|C|/|D|`, `D` least there.
    pub fn widest(&self, from: f64, to: f64) -> f64 {
        let cone = match self.round {
            Round::Cylinder(cylinder) => return cylinder.radius,
            Round::Cone(cone) => cone,
        };
        let ends = [from, to].map(|t| cone.off_axis(self.point(t)));
        let widest = ends[0].max(ends[1]);
        let section = Section::of(&self.plane, &self.round);
        if self.k <= 0.0 {
            return widest;
        }
        let far = if section.level(0.0).abs() < section.level(PI).abs() {
            0.0
        } else {
            PI
        };
        let [start, end] = [from, to].map(|t| self.on_wall(t)[0].x);
        let (low, high) = (start.min(end), start.max(end));
        let vertex = section.phi + far;
        let passes = vertex + TAU * ((low - vertex) / TAU).ceil() <= high;
        if passes {
            widest.max((section.c / section.level(far)).abs())
        } else {
            widest
        }
    }
}
