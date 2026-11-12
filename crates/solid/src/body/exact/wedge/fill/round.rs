//! What a fill is trimmed by at a round end (#788): the solid holding what of
//! the prism stands outside the round past where the prism's long edges
//! leave it, bounded by the round's own surface so that the fill ends flush
//! on it. A cylinder's is raised along its axis, and hugs the prism's sides:
//! a result carries the larger reach of its operands, and a solid standing
//! out of the part's box would draw every later step coarser. A cone cannot
//! be raised: what stands outside a cylinder just outside it is cut as a
//! cylinder's is, and only the stretch between the two is held by a piece
//! turned about the axis, bounded either side by planes through the axis
//! and outside by a cylinder about it (#807).

use std::f64::consts::{PI, TAU};

use glam::{DVec2, DVec3};

use super::Prism;
use crate::body::exact::Exact;
use crate::body::exact::wedge::end::CLEAR;
use crate::brep::{Cone, Cylinder, Declined};
use crate::profile::{Contour, Frame, Run};
use crate::turning::{Axis, ON_THE_AXIS, Straight, Turn};

/// Each long edge of the prism, from its end behind to its end along `past`,
/// and how far a solid holding what of them stands past an end is to stand
/// clear of them: a share [`CLEAR`] of the section's reach.
fn long_edges((outline, frame, travel): &Prism, past: DVec3) -> (Vec<[DVec3; 2]>, f64) {
    let ahead = past.dot(*travel) > 0.0;
    let edges = (outline.corners.iter())
        .map(|corner| {
            let start = frame.origin + frame.u * corner.x + frame.v * corner.y;
            if ahead {
                [start, start + *travel]
            } else {
                [start + *travel, start]
            }
        })
        .collect();
    let reach = (outline.corners.iter()).fold(0.0, |far: f64, corner| far.max(corner.length()));
    (edges, reach * CLEAR)
}

/// The solid holding what of the prism stands outside `cylinder` past where
/// its long edges, running on `past`, leave it, raised along the axis over a
/// section square to it: the rectangle round the long edges, from a line
/// across them where every one that meets the cylinder stands inside it —
/// clear short of where the first leaves it, or the plane through the axis
/// where that is nearer: each edge runs along `past` at one distance aside
/// from the axis, so it crosses the disc evenly about that plane — to clear
/// past their ends, less the cylinder's disc. An edge that misses the
/// cylinder is cut where the rectangle starts: inside the matter where it
/// lies outside the round, as the margin sunk into a slot's wall does, and
/// otherwise leaving a face of the fill standing past the round, which the
/// fill's own check finds stray. Declined `Unsupported` where no edge meets
/// the cylinder, where one leaves it past the prism's end, where the line
/// the rectangle starts on misses an edge between the prism's ends — the
/// prism's own back standing past it, behind a rib shorter than its round
/// end bows back across the fill — and where the disc does not cross the
/// rectangle's rim twice; a solid the kernel cannot raise, as the raise
/// says.
pub(super) fn outside_cylinder(
    prism: &Prism,
    cylinder: &Cylinder,
    past: DVec3,
) -> Result<Exact, Declined> {
    let (axis, radius) = (cylinder.axis, cylinder.radius);
    let towards = (past - axis * axis.dot(past)).normalize();
    let aside = axis.cross(towards);
    let read = |at: DVec3| {
        let from = at - cylinder.origin;
        DVec3::new(from.dot(towards), from.dot(aside), from.dot(axis))
    };
    let (edges, clear) = long_edges(prism, past);
    let lines: Vec<[DVec3; 2]> = edges.iter().map(|edge| edge.map(read)).collect();
    let mut leaving = f64::INFINITY;
    for [_, far] in &lines {
        let half = (radius * radius - far.y * far.y).sqrt();
        if half > 0.0 {
            if half >= far.x {
                return Err(Declined::Unsupported);
            }
            leaving = leaving.min(half);
        }
    }
    if !leaving.is_finite() {
        return Err(Declined::Unsupported);
    }
    let start = (leaving - clear).max(0.0);
    let (mut low, mut high) = (f64::INFINITY, f64::NEG_INFINITY);
    let (mut side, mut end) = ([f64::INFINITY, f64::NEG_INFINITY], f64::NEG_INFINITY);
    for [near, far] in &lines {
        if !(near.x < start && start < far.x) {
            return Err(Declined::Unsupported);
        }
        let at_start = near.z + (start - near.x) * (far.z - near.z) / (far.x - near.x);
        low = low.min(at_start.min(far.z));
        high = high.max(at_start.max(far.z));
        side = [side[0].min(far.y), side[1].max(far.y)];
        end = end.max(far.x);
    }
    let [right, left] = [side[0] - clear, side[1] + clear];
    let rim = [
        DVec2::new(start, right),
        DVec2::new(end + clear, right),
        DVec2::new(end + clear, left),
        DVec2::new(start, left),
    ];
    let section = less_the_disc(&rim, radius).ok_or(Declined::Unsupported)?;
    let frame = Frame {
        origin: cylinder.origin + axis * (low - clear),
        u: towards,
        v: aside,
    };
    Exact::raised(&section, &[], frame, axis * (high - low + 2.0 * clear))
}

/// The convex outline `rim`, turning anticlockwise, less the disc of
/// `radius` about the origin, where the disc's circle crosses the rim twice:
/// from where the rim leaves the disc round its corners outside it to where
/// it comes back, then back along the circle, the disc on the right. None
/// where the circle crosses it otherwise.
fn less_the_disc(rim: &[DVec2], radius: f64) -> Option<Contour> {
    let mut crossings = Vec::new();
    for (rank, &from) in rim.iter().enumerate() {
        let run = rim[(rank + 1) % rim.len()] - from;
        let (square, half) = (run.length_squared(), from.dot(run));
        let room = half * half - square * (from.length_squared() - radius * radius);
        if room <= 0.0 {
            continue;
        }
        for sign in [-1.0, 1.0] {
            let t = (sign * room.sqrt() - half) / square;
            if t > 0.0 && t < 1.0 {
                crossings.push((rank, from + run * t, sign > 0.0));
            }
        }
    }
    let &[one, other] = crossings.as_slice() else {
        return None;
    };
    let ((out_rank, out, _), (back_rank, back, _)) =
        if one.2 { (one, other) } else { (other, one) };
    let mut corners = vec![out];
    let mut rank = out_rank;
    loop {
        rank = (rank + 1) % rim.len();
        corners.push(rim[rank]);
        if rank == back_rank {
            break;
        }
    }
    corners.push(back);
    let sweep = (back.to_angle() - out.to_angle()).rem_euclid(TAU);
    let mut runs = vec![Run::Straight; corners.len()];
    runs[corners.len() - 1] = Run::Round {
        center: DVec2::ZERO,
        turn: -sweep,
    };
    Some(Contour { corners, runs })
}

/// How far along each of `edges` it leaves the round about `axis` through
/// `origin` whose radius at a height `h` along the axis is `radius(h)`,
/// running straight: the larger root of the quadratic its distance from the
/// axis squared less that radius squared makes along the edge — the inside
/// of a cone or a cylinder is convex, so an edge leaves it once. None for an
/// edge that misses it. Declined `Unsupported` where an edge runs nearer the
/// axis's way than the round's meridian does, and where one leaves it
/// outside the prism's two ends or where the radius is no length.
fn leaving(
    edges: &[[DVec3; 2]],
    (origin, axis): (DVec3, DVec3),
    radius: impl Fn(f64) -> f64,
) -> Result<Vec<Option<f64>>, Declined> {
    let mut leaving = Vec::new();
    for &[near, far] in edges {
        let (from, run) = (near - origin, far - near);
        let (h, rise) = (from.dot(axis), run.dot(axis));
        let (off, drift) = (from - axis * h, run - axis * rise);
        let (at_near, growth) = (radius(h), radius(h + rise) - radius(h));
        let square = drift.length_squared() - growth * growth;
        let half = off.dot(drift) - at_near * growth;
        let room = half * half - square * (off.length_squared() - at_near * at_near);
        if square <= 0.0 {
            return Err(Declined::Unsupported);
        }
        if room <= 0.0 {
            leaving.push(None);
            continue;
        }
        let at = (room.sqrt() - half) / square;
        if !(at > 0.0 && at < 1.0 && at_near + growth * at > 0.0) {
            return Err(Declined::Unsupported);
        }
        leaving.push(Some(at));
    }
    Ok(leaving)
}

/// The piece of a ring turned about `cone`'s axis that holds what of the
/// prism stands outside the cone — the side of its meridian line away from
/// the axis — past where the prism's long edges leave it, as [`leaving`]
/// reads it, and the clearance [`long_edges`] gives round that, up to a
/// cylinder about the axis standing clear outside the cone over that
/// stretch; and what of the prism stands outside that cylinder, as
/// [`outside_cylinder`] raises it, hugging the prism, so that the ring only
/// holds the stretch from where the edges leave the cone to where they
/// leave the cylinder, and neither stands out of the prism further than its
/// clearance. An edge that misses the cone, buried in matter as the margin
/// a slot's wall sinks into is, is left to the cylinder's cut, which takes
/// it where its rectangle starts. The ring is bounded inside by the cone,
/// outside by a cylinder round
/// the points the edges leave the two at, either side by a plane through
/// the axis turned clear of them, and along the axis by planes clear of
/// them. Declined `Unsupported` where the edge, running on `past` at
/// `point`, runs into the cone rather than out of it, where no edge leaves
/// it, where one leaves the cylinder where the cone stands out past it,
/// where those points spread round the axis over half a turn or more, and
/// where the meridian line stands on or across the axis at either end of
/// the piece; as [`leaving`] and [`outside_cylinder`] decline; a piece the
/// kernel cannot lay or turn, as the turn says.
pub(super) fn outside_cone(
    prism: &Prism,
    cone: &Cone,
    point: DVec3,
    past: DVec3,
    reach: f64,
) -> Result<Vec<Exact>, Declined> {
    let (origin, axis, foot, ruling) = (cone.origin, cone.axis, cone.foot, cone.ruling);
    let off_the_axis = |at: DVec3| {
        let from = at - origin;
        from - axis * from.dot(axis)
    };
    let out = ruling.perp() * ruling.x.signum();
    if (off_the_axis(point).normalize() * out.y + axis * out.x).dot(past) <= 0.0 {
        return Err(Declined::Unsupported);
    }
    let (edges, clear) = long_edges(prism, past);
    let slope = ruling.y / ruling.x;
    let on_the_cone = |h: f64| foot.y + (h - foot.x) * slope;
    let along = |[near, far]: [DVec3; 2], at: f64| near + (far - near) * at;
    let height = |at: DVec3| (at - origin).dot(axis);
    let left = leaving(&edges, (origin, axis), on_the_cone)?;
    let met: Vec<([DVec3; 2], f64)> = (edges.iter().zip(&left))
        .filter_map(|(&edge, at)| at.map(|at| (edge, at)))
        .collect();
    if met.is_empty() {
        return Err(Declined::Unsupported);
    }
    let exits: Vec<DVec3> = met.iter().map(|&(edge, at)| along(edge, at)).collect();
    let (low, high) = (exits.iter())
        .fold((f64::INFINITY, f64::NEG_INFINITY), |(low, high), &at| {
            (low.min(height(at)), high.max(height(at)))
        });
    let snug = clear / 4.0;
    let radius = on_the_cone(low).max(on_the_cone(high)) + snug;
    let wall = Cylinder {
        origin,
        axis,
        radius,
        u: cone.u,
        v: cone.v,
    };
    let outside = outside_cylinder(prism, &wall, past)?;
    let edges: Vec<[DVec3; 2]> = met.iter().map(|&(edge, _)| edge).collect();
    let mut held = exits;
    for (&edge, at) in edges
        .iter()
        .zip(leaving(&edges, (origin, axis), |_| radius)?)
    {
        let at = along(edge, at.ok_or(Declined::Unsupported)?);
        if on_the_cone(height(at)) >= radius {
            return Err(Declined::Unsupported);
        }
        held.push(at);
    }
    let towards = (past - axis * axis.dot(past)).normalize();
    let aside = axis.cross(towards);
    let (mut low, mut high, mut outer) = (f64::INFINITY, f64::NEG_INFINITY, 0.0_f64);
    let (mut first, mut last) = (f64::INFINITY, f64::NEG_INFINITY);
    for at in held {
        let (flat, h) = (off_the_axis(at), (at - origin).dot(axis));
        let off = flat.length();
        let turn = flat.dot(aside).atan2(flat.dot(towards));
        let spread = (snug / off).min(1.0).asin();
        first = first.min(turn - spread);
        last = last.max(turn + spread);
        low = low.min(h);
        high = high.max(h);
        outer = outer.max(off);
    }
    if !(first.is_finite() && last - first < PI) {
        return Err(Declined::Unsupported);
    }
    let ends = [low - snug, high + snug];
    let on_the_line = ends.map(|h| DVec2::new(h, foot.y + (h - foot.x) * ruling.y / ruling.x));
    if on_the_line
        .iter()
        .any(|at| !(at.y.is_finite() && at.y > 0.0))
    {
        return Err(Declined::Unsupported);
    }
    let outer = on_the_line.iter().fold(outer, |far, at| far.max(at.y)) + snug;
    let section = Contour::straight(vec![
        on_the_line[0],
        DVec2::new(ends[0], outer),
        DVec2::new(ends[1], outer),
        on_the_line[1],
    ]);
    let (sin, cos) = first.sin_cos();
    let frame = Frame {
        origin,
        u: axis,
        v: towards * cos + aside * sin,
    };
    let turn = Turn {
        axis: Axis {
            origin: DVec2::ZERO,
            direction: DVec2::X,
        },
        angle: last - first,
        resolution: 0.0,
        on_the_axis: ON_THE_AXIS * outer,
    };
    let straight = Straight::of(&section, &[], frame, &turn, reach).ok_or(Declined::Unsupported)?;
    let ring = Exact::turned(&straight, frame, &turn, straight.numbers(false))?;
    Ok(vec![outside, ring])
}
