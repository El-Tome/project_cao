//! A profile laid square to its axis, turned about it into a body (#533): a
//! cylinder for each run parallel to the axis, a plane for each run square to
//! it, a cone for each run slanted to it (#536), and the two planes holding
//! the axis a partial turn ends on.
//!
//! The faces are laid by hand, as a raise lays its walls, then tidied and
//! checked as a boolean's result is: what the kernel cannot verify it does not
//! answer.

mod faces;

use std::f64::consts::{FRAC_PI_2, TAU};

use glam::{DVec2, DVec3};

use super::Declined;
use super::assembly::tidied;
use super::curve::Circle;
use super::piece;
use super::scale::Scale;
use super::surface::{Cone, Cylinder};
use super::topology::{Body, turning_points};
use crate::profile::{Contour, Frame};
use crate::turning::{Straight, Turn};

/// The cosine and sine of a quarter, a half and three quarters of a turn,
/// exactly: computed, the half turn's sine is a rounding off nought, and the
/// planes a turn ends on would stand a hair off the planes of the origin.
const RIGHT_ANGLES: [(f64, f64); 3] = [(0.0, 1.0), (-1.0, 0.0), (0.0, -1.0)];

/// Where a turn stands in the world: its axis through `origin` along `axis`,
/// the side of the axis the profile lies on, `toward`, and the direction the
/// profile sets off in, `across`; how far it turns, and the cosine and sine of
/// that, exact on a right angle.
#[derive(Clone, Copy, Debug)]
struct Placed {
    origin: DVec3,
    axis: DVec3,
    toward: DVec3,
    across: DVec3,
    angle: f64,
    cos: f64,
    sin: f64,
    whole: bool,
    half: bool,
}

impl Placed {
    /// A corner `(h, r)` of the profile turned by nought.
    fn opening(&self, corner: DVec2) -> DVec3 {
        self.origin + self.axis * corner.x + self.toward * corner.y
    }

    /// A corner of the profile turned all the way.
    fn closing(&self, corner: DVec2) -> DVec3 {
        self.turned(corner, self.cos, self.sin)
    }

    /// A corner of the profile turned halfway: a point of every face it
    /// bounds that the ends do not hold.
    fn halfway(&self, corner: DVec2) -> DVec3 {
        let (sin, cos) = (self.angle / 2.0).sin_cos();
        self.turned(corner, cos, sin)
    }

    fn turned(&self, corner: DVec2, cos: f64, sin: f64) -> DVec3 {
        self.origin + self.axis * corner.x + (self.toward * cos + self.across * sin) * corner.y
    }

    /// The point of the axis a corner turns about.
    fn center(&self, corner: DVec2) -> DVec3 {
        self.origin + self.axis * corner.x
    }

    fn cylinder(&self, corner: DVec2) -> Cylinder {
        Cylinder::about(self.origin, self.axis, corner.y)
    }

    /// The cone a run slanted to the axis turns into.
    fn cone(&self, from: DVec2, to: DVec2) -> Cone {
        Cone::through(self.origin, self.axis, [from, to])
    }

    /// The outward direction of the plane the turn ends on.
    fn closing_outward(&self) -> DVec3 {
        self.across * self.cos - self.toward * self.sin
    }
}

impl Body {
    /// The profile turned about the axis of `turn` by its angle, the profile
    /// standing in `frame`.
    ///
    /// A run parallel to the axis turns into a cylinder, a run square to it
    /// into a plane, a run slanted to it into a cone, and a run on the axis
    /// into nothing. A corner on the axis between two runs leaving it would
    /// pinch the body there, and is declined as a profile. Each face answers
    /// to the numbers of the runs it was turned from, the outline's runs first
    /// and then each hole's; a partial turn ends on the sketch's own plane,
    /// numbered `runs`, and on the plane it turns to, numbered `runs + 1`.
    ///
    /// A point turned whole has one vertex, at its tip, which its cone holds
    /// within it and no edge reaches; the scale is read again once it is
    /// there. A profile laid to no area turns into nothing.
    pub fn turned(straight: &Straight, frame: Frame, turn: &Turn) -> Result<Body, Declined> {
        let (placed, backwards) = placed(straight, frame, turn)?;
        if straight.contours.is_empty() {
            return Ok(Body::empty());
        }
        let contours: Vec<Vec<DVec2>> = straight
            .contours
            .iter()
            .map(|contour| {
                contour
                    .iter()
                    .map(|corner| {
                        let h = if backwards { -corner.at.x } else { corner.at.x };
                        DVec2::new(h, corner.at.y)
                    })
                    .collect()
            })
            .collect();
        let corners = || contours.iter().flatten().copied();
        if corners().any(|corner| !(corner.is_finite() && corner.y >= 0.0)) {
            return Err(Declined::Profile);
        }
        let scale = Scale::of(reach(&placed, corners()));
        let eps = scale.eps();
        let furthest = corners().map(|corner| corner.y).fold(0.0, f64::max);
        let placed = squared(placed, furthest, eps);
        if !placed.whole
            && corners()
                .any(|corner| corner.y > 0.0 && 2.0 * corner.y * (placed.angle / 2.0).sin() <= eps)
        {
            return Err(Declined::Travel);
        }
        let mut read = Vec::with_capacity(contours.len());
        for (rank, (corners, laid)) in contours.iter().zip(&straight.contours).enumerate() {
            let numbers: Vec<u32> = laid.iter().map(|corner| corner.run).collect();
            let pieces =
                piece::pieces_numbered(&Contour::straight(corners.clone()), &numbers, eps)?;
            if piece::signed_area(&pieces).abs() <= eps * scale.reach() {
                return Err(Declined::Profile);
            }
            read.push(piece::turned(pieces, rank == 0));
        }
        if !piece::apart(&read, eps) || read.iter().any(|contour| pinched(contour)) {
            return Err(Declined::Profile);
        }
        let (mut body, faces) = faces::laid(&placed, &read, straight.runs, eps)?;
        body.scale = Scale::of(body.reach());
        body.arrivals = vec![body.scale; body.surfaces.len()];
        let mut body = tidied(body, faces)?;
        body.scale = body.scale.joined(Scale::of(body.reach()));
        body.arrivals = vec![body.scale; body.surfaces.len()];
        Ok(body)
    }
}

/// Whether a contour has a corner on the axis between two pieces that both
/// leave it, where the body turned from it would touch itself at a point.
fn pinched(contour: &[piece::Named]) -> bool {
    let count = contour.len();
    (0..count).any(|index| {
        let (before, after) = (
            &contour[(index + count - 1) % count].piece,
            &contour[index].piece,
        );
        after.from().y == 0.0 && before.from().y > 0.0 && after.to().y > 0.0
    })
}

/// The turn placed in the world, the angle unsquared, and whether it turns
/// backwards: a turn backwards is the turn forwards about the axis turned
/// round, its profile read along it the other way.
fn placed(straight: &Straight, frame: Frame, turn: &Turn) -> Result<(Placed, bool), Declined> {
    let along = turn.axis.direction.normalize_or_zero();
    let origin = frame.at(turn.axis.origin);
    if along == DVec2::ZERO || !origin.is_finite() || !along.is_finite() {
        return Err(Declined::Profile);
    }
    if !(turn.angle.is_finite() && turn.angle != 0.0) {
        return Err(Declined::Travel);
    }
    let backwards = turn.angle < 0.0;
    let axis = (frame.u * along.x + frame.v * along.y).normalize();
    let axis = if backwards { -axis } else { axis };
    let toward = (frame.v * along.x - frame.u * along.y).normalize() * straight.side.signum();
    let whole = turn.is_whole();
    let angle = if whole { TAU } else { turn.angle.abs() };
    let (sin, cos) = angle.sin_cos();
    Ok((
        Placed {
            origin,
            axis,
            toward,
            across: axis.cross(toward),
            angle,
            cos,
            sin,
            whole,
            half: false,
        },
        backwards,
    ))
}

/// The turn on a right angle when it is within a hair of one at the
/// profile's furthest point from the axis: its ends then stand on the planes
/// a raise from the planes of the origin stands on, and a half turn ends on
/// one plane.
fn squared(placed: Placed, furthest: f64, eps: f64) -> Placed {
    if placed.whole {
        return placed;
    }
    for (rank, (cos, sin)) in RIGHT_ANGLES.into_iter().enumerate() {
        let right = (rank + 1) as f64 * FRAC_PI_2;
        if (placed.angle - right).abs() * furthest <= Scale::HAIR * eps {
            return Placed {
                angle: right,
                cos,
                sin,
                half: rank == 1,
                ..placed
            };
        }
    }
    placed
}

/// The largest coordinate the body will reach: every corner where the turn
/// starts and where it ends, and the points where a circle a corner turns
/// along turns back in a coordinate.
fn reach(placed: &Placed, corners: impl Iterator<Item = DVec2>) -> f64 {
    let mut points = Vec::new();
    for corner in corners {
        points.push(placed.opening(corner));
        points.push(placed.closing(corner));
        if corner.y > 0.0 {
            let cylinder = placed.cylinder(corner);
            let circle = Circle::on(
                &cylinder,
                (placed.center(corner) - cylinder.origin).dot(cylinder.axis),
            );
            let (from, to) = if placed.whole {
                (0.0, TAU)
            } else {
                let start = if cylinder.axis.dot(placed.axis) > 0.0 {
                    placed.opening(corner)
                } else {
                    placed.closing(corner)
                };
                let from = circle.parameter(start);
                (from, from + placed.angle)
            };
            points.extend(turning_points(&circle, from, to));
        }
    }
    points
        .iter()
        .map(|point| point.abs().max_element())
        .fold(0.0, f64::max)
}

#[cfg(test)]
mod tests;
