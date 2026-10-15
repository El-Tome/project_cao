//! Where an edge along the conic a plane cuts a wall at a slant is sampled:
//! the meet's way, with one wall.
//!
//! At every grid angle of its wall, each sample the point of the cut on the
//! ruling at that angle, so that on the wall it has a point at every grid
//! angle as a circle has, and the outline cuts a face going round at one of
//! its own samples; at every ray the wall's circles take, the angles of the
//! vertices on it among them, so that a ruling from a vertex meets a sample
//! of it; at its ends. On a cylinder the grid is the cylinder's own, on a
//! cone the cone's, read at its widest. The curve runs one way in its wall's
//! angle, so between two of those it never turns back on the wall. Then,
//! between two of them, wherever the chord would stand further from the
//! curve than the tolerance: seen on its plane the curve bends more than its
//! wall's grid allows for, by as much as the plane leans from square to the
//! axis, and a hyperbola's arms run ever faster.
//!
//! No place on the grid or on a ray is kept where the cylinder all but lies
//! on a wall facing it, or on a plane touching it off the line they touch
//! along, nor a step or less from an end where the curve all but lies on a
//! surface that end lies on, as for a meet.

mod rays;

use std::collections::BTreeMap;
use std::f64::consts::{PI, TAU};

use glam::DVec3;

use super::chords::halved;
use super::cone::Grid;
use super::elliptic::{a_step_apart, between_steps, on_the_wall, reached_by_planes};
use super::ends::{grazed, grazed_near};
use super::{divisions, on_a_plane, touching_planes};
use crate::brep::conic::{Conic, On, Round};
use crate::brep::surface::Surface;
use crate::brep::tessellation::contact::{self, Zones};
use crate::brep::topology::{Body, Edge, SurfaceId};
pub(super) use rays::{rays_about, rays_along};

/// How many turns of its wall a stretch may run before its angles are given
/// up on: a whole ellipse runs one, an arm less.
const TURNS: f64 = 4.0;

/// How near an end's own angle on the wall a grid angle or a ray stands at
/// that end, as the outline reads a sample on a grid angle. An arm all but
/// along the wall's rulings turns by a hair across the box: read back off an
/// angle that close to an end's, the place along it is the end's only to
/// rounding times the arm's length, and may land beyond the end.
const AT_AN_END: f64 = 1e-9;

/// Why a parameter is a sample, the stronger reason first: of two places
/// closer than the kernel's tolerance, the one kept is the stronger.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Reason {
    End,
    Grid,
    Ray,
}

/// The points of an edge along a conic between its ends, as [`on_conic`]
/// places them, on the grid of its wall in `body` and, on a cylinder, on the
/// angles `rays` its circles take.
pub(super) fn of_edge(
    body: &Body,
    edge: &Edge,
    conic: &Conic,
    zones: &Zones,
    grids: &BTreeMap<SurfaceId, Grid>,
    rays: &[f64],
    tolerance: f64,
) -> Vec<DVec3> {
    let eps = body.scale().eps();
    let plane = Surface::Plane(conic.plane);
    match conic.round {
        Round::Cylinder(cylinder) => {
            let steps = divisions(cylinder.radius, tolerance);
            let grid = turn(steps);
            let planes = touching_planes(body, cylinder.origin, cylinder.axis, cylinder.radius);
            let step = TAU * cylinder.radius / steps as f64;
            let own = [plane, Surface::Cylinder(cylinder)];
            let grazing = grazed(body, edge, &own, step);
            let clear = |point: DVec3| {
                !grazing(point, eps * contact::APART)
                    && !zones.crowded(&cylinder, point)
                    && !planes
                        .iter()
                        .any(|plane| on_a_plane(plane, cylinder.origin, cylinder.axis, point, eps))
            };
            on_conic(conic, edge, tolerance, eps, [&grid, rays], &clear)
        }
        Round::Cone(cone) => {
            let (grid, vertices) = match grids.values().find(|grid| grid.cone == cone) {
                Some(grid) => (turn(grid.steps), grid.vertex_angles().to_vec()),
                None => {
                    let widest = conic.widest(edge.from, edge.to);
                    (turn(divisions(widest, tolerance)), Vec::new())
                }
            };
            let step = TAU * conic.widest(edge.from, edge.to) / grid.len() as f64;
            let own = [plane, Surface::Cone(cone)];
            let grazing = grazed(body, edge, &own, step);
            let clear = |point: DVec3| !grazing(point, eps * contact::APART);
            let rays: Vec<f64> = vertices.iter().chain(rays).copied().collect();
            on_conic(conic, edge, tolerance, eps, [&grid, &rays], &clear)
        }
        Round::Elliptic(wall) => {
            let steps = divisions(wall.major, tolerance);
            let grid = turn(steps);
            let own = [plane, Surface::Elliptic(wall)];
            let grazing = grazed_near(body, edge, &own, a_step_apart(wall, steps));
            let clear = |point: DVec3| !grazing(point, eps * contact::APART);
            let rays: Vec<f64> = on_the_wall(body, &wall, steps)
                .into_iter()
                .chain(reached_by_planes(body, &wall))
                .chain(rays.iter().copied())
                .collect();
            let edge = between_steps(conic, edge, steps);
            on_conic(conic, &edge, tolerance, eps, [&grid, &rays], &clear)
        }
    }
}

/// The angles of a grid of `steps` steps a turn.
fn turn(steps: usize) -> Vec<f64> {
    (0..steps)
        .map(|step| TAU * step as f64 / steps as f64)
        .collect()
}

/// The points of an edge along a conic between its ends, in the way the edge
/// runs. A whole ellipse's points start at its first angle of the grid or of
/// a ray and do not come back to it. `angles` are its wall's grid, then the
/// rays its circles take; a place on either is kept only where `clear` says
/// so.
pub(in crate::brep::tessellation) fn on_conic(
    conic: &Conic,
    edge: &Edge,
    tolerance: f64,
    eps: f64,
    [grid, rays]: [&[f64]; 2],
    clear: &dyn Fn(DVec3) -> bool,
) -> Vec<DVec3> {
    let whole = edge.ends.is_none();
    let (low, high) = match (whole, conic.period()) {
        (true, Some(period)) => (edge.from, edge.from + period),
        (true, None) => return Vec::new(),
        (false, _) => (edge.from.min(edge.to), edge.from.max(edge.to)),
    };
    let angle = |t: f64| conic.seen_on(On::Wall, t)[0].x;
    let (start, end) = (angle(low), angle(high));
    let (first, last) = (start.min(end), start.max(end));
    let mut marks: Vec<(f64, Reason, DVec3)> = if whole {
        Vec::new()
    } else {
        vec![
            (low, Reason::End, conic.point(low)),
            (high, Reason::End, conic.point(high)),
        ]
    };
    let at_an_end = |angle: f64| {
        !whole
            && [start, end]
                .iter()
                .any(|end| ((angle - end + PI).rem_euclid(TAU) - PI).abs() <= AT_AN_END)
    };
    for (reason, angles) in [(Reason::Grid, grid), (Reason::Ray, rays)] {
        for at in angles.iter().filter(|at| !at_an_end(**at)) {
            let (from, to) = (((first - at) / TAU).ceil(), ((last - at) / TAU).floor());
            if !(to - from).is_finite() || to - from > TURNS {
                continue;
            }
            for turns in from as i64..=to as i64 {
                let (t, point) = conic.at_angle(at + TAU * turns as f64);
                let inside = if whole {
                    low <= t && t < high
                } else {
                    low < t && t < high
                };
                if inside && point.is_finite() {
                    marks.push((t, reason, point));
                }
            }
        }
    }
    marks.sort_by(|one, other| one.0.total_cmp(&other.0).then(one.1.cmp(&other.1)));

    let mut kept: Vec<(f64, Reason, DVec3)> = Vec::with_capacity(marks.len());
    for (t, reason, point) in marks {
        if reason >= Reason::Grid && !clear(point) {
            continue;
        }
        match kept.last_mut() {
            Some(last) if (last.2 - point).length() <= eps => {
                if reason < last.1 {
                    *last = (t, reason, point);
                }
            }
            _ => kept.push((t, reason, point)),
        }
    }
    if whole && let (Some(first), Some(period)) = (kept.first().copied(), conic.period()) {
        if kept.len() > 1 && (kept[kept.len() - 1].2 - first.2).length() <= eps {
            kept.pop();
        }
        kept.push((first.0 + period, first.1, first.2));
    }
    if kept.len() < 2 {
        return Vec::new();
    }

    let point = |t: f64| conic.point(t);
    let mut points = vec![kept[0].2];
    for pair in kept.windows(2) {
        let mut places = Vec::new();
        halved(&point, pair[0].0, pair[1].0, tolerance, &mut places);
        points.extend(places.into_iter().map(point));
        points.push(pair[1].2);
    }
    points.pop();
    if !whole {
        points.remove(0);
    }
    if edge.to < edge.from {
        points.reverse();
    }
    points
}
