//! A piece of sphere its `(θ, φ)` chart cannot lay out (#800) — bounded by a
//! great circle tilted across the axis, neither a latitude nor a meridian —
//! laid out on a chart of its own: its central projection, from the sphere's
//! centre onto the plane tangent at a direction the whole piece lies in
//! front of.
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

mod refinement;

use glam::{DVec2, DVec3};

use super::super::sampling::Samples;
use super::super::sweep;
use super::Outline;
use crate::brep::surface::Sphere;

/// How far in front of the chart's direction every sample of the outline
/// must stand, as the cosine of the angle from it: some 75°, where the chart
/// stretches the sphere sixteen times. A piece reaching further — a
/// hemisphere or more — is not laid out.
const FRONT: f64 = 0.25;

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
    /// mean of the ways they stand from its centre. None when one of them
    /// stands less than [`FRONT`] in front of it.
    fn of(sphere: &Sphere, samples: &Samples, laps: &[Vec<usize>]) -> Option<Chart> {
        let centre = sphere.middle();
        let ways: Vec<DVec3> = laps
            .iter()
            .flatten()
            .map(|&id| (samples.point(id) - centre).try_normalize())
            .collect::<Option<_>>()?;
        let ahead = ways.iter().sum::<DVec3>().try_normalize()?;
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
    /// face does not lie in front of one direction, or cannot be cut.
    pub(super) fn central(
        sphere: &Sphere,
        samples: &mut Samples,
        laps: &[Vec<usize>],
        tolerance: f64,
    ) -> Option<(Outline, Vec<[usize; 3]>)> {
        let chart = Chart::of(sphere, samples, laps)?;
        let mut outline = Outline::default();
        for lap in laps {
            let placed: Vec<(DVec2, usize)> = lap
                .iter()
                .map(|&id| (chart.laid(samples.point(id)), id))
                .collect();
            outline.close(&placed);
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
