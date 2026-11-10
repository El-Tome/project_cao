//! A face of a sphere its `(θ, φ)` chart cannot lay out (#800), whatever
//! the reason — a great circle tilted across the axis, neither a latitude
//! nor a meridian, is the case it was made for, but a loop the strips
//! cannot follow comes here too — laid out on a chart of its own: its
//! central projection, from the sphere's centre onto the plane tangent at
//! the axis of the narrowest cap holding its samples ([`cap`]).
//!
//! A central projection takes every straight line of space to a straight
//! line, so the chord between two samples of a boundary — the very chord the
//! face beyond draws — lies on the segment between their projections: the
//! outline is the polygon of the edges' samples, whatever their curves, and
//! no point is ever added on a boundary. The polygon is swept, then refined
//! inside ([`refinement`]) until every triangle stands within the tolerance
//! of the sphere, each point added lifted onto the sphere along its ray from
//! the centre. A triangulation of the plane lifted along rays from one point
//! cannot cross itself.
//!
//! Left open still: a face some sample of which stands less than [`FRONT`]
//! in front of that axis, as a hemisphere or more does; one whose region on
//! the chart is not the bounded side of its outer loop, which would reach
//! behind the plane and be drawn inside out; and one the sweep or the
//! refinement cannot cut. A face of a torus its chart refuses is left open
//! too.

mod cap;
mod refinement;

use std::cmp::Ordering;

use glam::{DVec2, DVec3};

use super::super::sampling::Samples;
use super::super::sweep;
use super::Outline;
use crate::brep::surface::Sphere;
use crate::orientation::rotation;

/// How far in front of the chart's axis every sample of the outline must
/// stand, as the cosine of the angle from it: some 87°, a cap 174° across.
/// The chart stretches the sphere four hundred times along a ray there and
/// lays its points twenty radii out, which costs no soundness: every sag is
/// measured on the sphere and every turn decided exactly, and a triangle
/// stretched on the chart is short on the sphere all the same. Nearer a
/// hemisphere, the chart would run off to infinity.
const FRONT: f64 = 0.05;

/// The plane tangent to the unit sphere at `ahead`, read along `x` and `y`,
/// square to it and to each other, `x × y = ahead`: a turn counterclockwise
/// there is one counterclockwise seen from outside the sphere, along its own
/// normal.
struct Chart {
    centre: DVec3,
    radius: f64,
    ahead: DVec3,
    x: DVec3,
    y: DVec3,
}

impl Chart {
    /// The chart of the samples `laps` of a face of `sphere`, ahead along the
    /// axis of the narrowest cap holding the ways they stand from its centre.
    /// None when one of them stands less than [`FRONT`] in front of it.
    fn of(sphere: &Sphere, samples: &Samples, laps: &[Vec<usize>]) -> Option<Chart> {
        let centre = sphere.middle();
        let ways: Vec<DVec3> = laps
            .iter()
            .flatten()
            .map(|&id| (samples.point(id) - centre).try_normalize())
            .collect::<Option<_>>()?;
        let ahead = cap::axis(&ways)?;
        if ways.iter().any(|way| way.dot(ahead) < FRONT) {
            return None;
        }
        let x = ahead.any_orthonormal_vector();
        Some(Chart {
            centre,
            radius: sphere.radius,
            ahead,
            x,
            y: ahead.cross(x),
        })
    }

    /// Where a point, seen from the centre, stands on the chart.
    fn laid(&self, point: DVec3) -> DVec2 {
        let from = point - self.centre;
        DVec2::new(from.dot(self.x), from.dot(self.y)) / from.dot(self.ahead)
    }

    /// The point of the sphere the chart's point `at` stands for, from the
    /// centre.
    fn lifted(&self, at: DVec2) -> DVec3 {
        (self.ahead + self.x * at.x + self.y * at.y).normalize() * self.radius
    }
}

impl Outline {
    /// Lays out the loops `laps` of a face of `sphere` on their central
    /// chart and cuts them, refined within `tolerance`: the outline, its
    /// points inside added to `samples`, and its triangles. None when the
    /// face does not lie in front of one direction, reaches behind it, or
    /// cannot be cut.
    pub(super) fn central(
        sphere: &Sphere,
        samples: &mut Samples,
        laps: &[Vec<usize>],
        tolerance: f64,
    ) -> Option<(Outline, Vec<[usize; 3]>)> {
        let chart = Chart::of(sphere, samples, laps)?;
        let laid: Vec<Vec<(DVec2, usize)>> = laps
            .iter()
            .map(|lap| {
                lap.iter()
                    .map(|&id| (chart.laid(samples.point(id)), id))
                    .collect()
            })
            .collect();
        if !bounded(&laid) {
            return None;
        }
        let mut outline = Outline::default();
        for placed in &laid {
            outline.close(placed);
        }
        let swept = sweep::triangles(&outline.points, &outline.segments)?;
        let places: Vec<DVec3> = outline
            .samples
            .iter()
            .map(|&id| samples.point(id) - chart.centre)
            .collect();
        let (added, triangles) = refinement::refined(
            &chart,
            (&outline.points, &places),
            &outline.segments,
            swept,
            tolerance,
        )?;
        for at in added {
            outline.points.push(at);
            outline
                .samples
                .push(samples.added(chart.centre + chart.lifted(at)));
        }
        Some((outline, triangles))
    }
}

/// Whether the loops `laid` on the chart hold their region on its bounded
/// side: the widest turning counterclockwise, every other clockwise. A face
/// holding points behind the chart's plane has the outside of its loops for
/// its region there, which a sweep would cut inside out.
fn bounded(laid: &[Vec<(DVec2, usize)>]) -> bool {
    let loops: Vec<Vec<DVec2>> = laid
        .iter()
        .map(|placed| placed.iter().map(|(at, _)| *at).collect())
        .collect();
    let area = |points: &[DVec2]| {
        let pairs = points.iter().zip(points.iter().cycle().skip(1));
        pairs
            .map(|(from, to)| from.perp_dot(*to))
            .sum::<f64>()
            .abs()
    };
    let Some(outer) =
        (0..loops.len()).max_by(|&one, &other| area(&loops[one]).total_cmp(&area(&loops[other])))
    else {
        return false;
    };
    loops.iter().enumerate().all(|(at, points)| {
        let way = if at == outer {
            Ordering::Greater
        } else {
            Ordering::Less
        };
        rotation(points) == way
    })
}
