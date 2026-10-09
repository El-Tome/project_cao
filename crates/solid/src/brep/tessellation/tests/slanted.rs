//! Bodies a plane cuts at a slant through a wall, built by hand, each face
//! bounded by the conic the kernel builds for its plane and its wall: a
//! point's tip cut off by a slanted plane, a flat milled on a chamfer, and a
//! block with a slanted top two holes are bored square into.

use std::f64::consts::{PI, TAU};

use glam::{DVec2, DVec3};

use super::fixtures::{Build, HALF_SIDE, HEIGHT, HOLE_RADIUS, STOCK_RADIUS};
use crate::brep::conic::{Conic, Round};
use crate::brep::curve::Curve;
use crate::brep::scale::Scale;
use crate::brep::surface::{Cone, Cylinder, Plane, Surface};
use crate::brep::topology::{Body, Coedge, FaceId, VertexId};

/// How high the point's tip stands over its base, and the base's radius.
pub(crate) const TIP: f64 = 20.0;
pub(crate) const BASE: f64 = 10.0;

/// The height the slanted plane crosses the point's axis at.
const CROSSING: f64 = 12.0;

fn point() -> Cone {
    Cone::through(
        DVec3::ZERO,
        DVec3::Z,
        [DVec2::new(0.0, BASE), DVec2::new(TIP, 0.0)],
    )
}

/// The tip of the point standing above the plane `z = 12 + lean·x`, a plane
/// less steep than the point's slope: its wall holds its apex within it and
/// goes round between it and the ellipse the plane cuts, with no vertex.
pub(crate) fn tip_cut_at_a_slant(lean: f64) -> Body {
    point_cut_at_a_slant(lean, None)
}

/// How high the point's tip is cut off square to its axis, leaving a rim of
/// radius 1.
pub(crate) const SQUARE: f64 = 18.0;

/// The tip cut at a slant as above, its tip then cut off square at height
/// 18: its wall goes round between a rim of radius 1 and the ellipse, which
/// stands much further from the axis.
pub(crate) fn frustum_cut_at_a_slant(lean: f64) -> Body {
    point_cut_at_a_slant(lean, Some(SQUARE))
}

/// What the frustum cut at a slant holds: the tip, less the point above the
/// rim.
pub(crate) fn frustum_cut_at_a_slant_volume(lean: f64) -> f64 {
    let rim = BASE * (1.0 - SQUARE / TIP);
    tip_cut_at_a_slant_volume(lean) - PI * rim * rim * (TIP - SQUARE) / 3.0
}

fn point_cut_at_a_slant(lean: f64, square: Option<f64>) -> Body {
    let mut build = Build::new();
    let cone = point();
    let wall = build.surface(Surface::Cone(cone));
    let (slant, flipped) = build.plane(DVec3::Z * CROSSING, DVec3::new(lean, 0.0, -1.0));
    let [ellipse] = build.conic(slant, wall, TIP)[..] else {
        panic!("a plane less steep than the point cuts it along one ellipse");
    };
    let period = ellipse.period().expect("an ellipse closes");
    let ring = build.conic_edge(ellipse, None, 0.0, period);
    let use_of = |edge, forward| Coedge { edge, forward };
    build.face(slant, flipped, vec![vec![use_of(ring, false)]]);
    let Some(height) = square else {
        let apex = build.vertex(cone.apex());
        let face = build.face(wall, true, vec![vec![use_of(ring, true)]]);
        let mut body = build.finish(TIP);
        held_at_its_apex(&mut body, face, apex);
        return body;
    };
    let rim = cone.circle(height, BASE * (1.0 - height / TIP));
    let top = build.edge(Curve::Circle(rim), None, 0.0, TAU);
    build.face(
        wall,
        true,
        vec![vec![use_of(ring, true)], vec![use_of(top, false)]],
    );
    let (cap, cap_flipped) = build.plane(DVec3::Z * height, DVec3::Z);
    build.face(cap, cap_flipped, vec![vec![use_of(top, true)]]);
    build.finish(TIP)
}

/// What the tip cut by `z = 12 + lean·x` holds: a third of its ellipse's area
/// times the tip's height over the plane, the ellipse's axes read off where
/// the plane crosses the point's slope in the XZ plane and, across, at their
/// middle.
pub(crate) fn tip_cut_at_a_slant_volume(lean: f64) -> f64 {
    let slope = BASE / TIP;
    let at = |side: f64| {
        let radius = (BASE - slope * CROSSING) / (1.0 + side * slope * lean);
        DVec3::new(side * radius, 0.0, CROSSING + lean * side * radius)
    };
    let (near, far) = (at(1.0), at(-1.0));
    let middle = (near + far) / 2.0;
    let across = BASE - slope * middle.z;
    let minor = (across * across - middle.x * middle.x).sqrt();
    let major = near.distance(far) / 2.0;
    let height = (TIP - CROSSING) / (1.0 + lean * lean).sqrt();
    PI * major * minor * height / 3.0
}

fn held_at_its_apex(body: &mut Body, face: FaceId, apex: VertexId) {
    let surface = body.faces[face.0 as usize].surface;
    body.faces[face.0 as usize].apex = Some(apex);
    body.vertices[apex.0 as usize].on = vec![surface];
}

/// The chamfer's radius at its foot and at its top, and its height.
pub(crate) const FOOT: f64 = 10.0;
pub(crate) const TOP: f64 = 5.0;
pub(crate) const RISE: f64 = 20.0;

/// The chamfer — a frustum about Z from radius 10 on the XY plane to radius
/// 5 at height 20 — with a flat milled down it at `x = 6`, parallel to its
/// axis: the flat is bounded by the hyperbola it cuts the cone along and the
/// chord it leaves on the foot, and the cone's wall goes round between its
/// top and the foot's arc and the hyperbola.
pub(crate) fn chamfer_with_a_flat(offset: f64) -> Body {
    let mut build = Build::new();
    let cone = Cone::through(
        DVec3::ZERO,
        DVec3::Z,
        [DVec2::new(0.0, FOOT), DVec2::new(RISE, TOP)],
    );
    let wall = build.surface(Surface::Cone(cone));
    let (flat, flat_flipped) = build.plane(DVec3::X * offset, DVec3::X);
    let (foot, foot_flipped) = build.plane(DVec3::ZERO, DVec3::NEG_Z);
    let (top, top_flipped) = build.plane(DVec3::Z * RISE, DVec3::Z);
    let half = (FOOT * FOOT - offset * offset).sqrt();
    let [left, right] = [-half, half].map(|y| DVec3::new(offset, y, 0.0));
    let [start, end] = [left, right].map(|corner| build.vertex(corner));
    let rim = cone.circle(0.0, FOOT);
    let from = rim.parameter(right);
    let to = from + (rim.parameter(left) - from).rem_euclid(TAU);
    let arc = build.edge(Curve::Circle(rim), Some([end, start]), from, to);
    let chord = build.line(start, end);
    let [hyperbola] = build.conic(flat, wall, RISE)[..] else {
        panic!("a plane parallel to a cone's axis cuts it along one hyperbola");
    };
    let span = [left, right].map(|corner| hyperbola.parameter(corner));
    let bend = build.conic_edge(hyperbola, Some([start, end]), span[0], span[1]);
    let ring = build.edge(Curve::Circle(cone.circle(RISE, TOP)), None, 0.0, TAU);
    let use_of = |edge, forward| Coedge { edge, forward };
    build.face(
        wall,
        true,
        vec![
            vec![use_of(arc, true), use_of(bend, true)],
            vec![use_of(ring, false)],
        ],
    );
    build.face(
        flat,
        flat_flipped,
        vec![vec![build.leaving(chord, start), use_of(bend, false)]],
    );
    build.face(
        foot,
        foot_flipped,
        vec![vec![build.leaving(chord, end), use_of(arc, false)]],
    );
    build.face(top, top_flipped, vec![vec![use_of(ring, true)]]);
    build.finish(RISE)
}

/// What the chamfer holds once the flat at `x = 6` is milled: the frustum,
/// less the slices of its sections beyond the flat, `4 [F(10) − F(6)]` with
/// `F(r) = r³/3 acos(6/r) − 4r √(r² − 36) + 72 ln(r + √(r² − 36))`, whose
/// derivative is the segment `r² acos(6/r) − 6 √(r² − 36)`.
pub(crate) fn chamfer_with_a_flat_volume() -> f64 {
    let frustum = PI * RISE / 3.0 * (FOOT * FOOT + FOOT * TOP + TOP * TOP);
    let primitive = |r: f64| {
        let root = (r * r - 36.0).max(0.0).sqrt();
        r.powi(3) / 3.0 * (6.0 / r).acos() - 4.0 * r * root + 72.0 * (r + root).ln()
    };
    frustum - 4.0 * (primitive(FOOT) - primitive(6.0))
}

/// How much the block's top rises along X, and its height at the middle.
pub(crate) const RISING: f64 = 0.25;
const MIDDLE: f64 = 10.0;

/// The block, its top slanted to `z = 10 + x/4`, with two holes bored square
/// into the top, `apart` from each other along its slope, out through the
/// floor: a circle on the top and an ellipse on the floor each.
pub(crate) fn slanted_block_with_two_holes(apart: f64) -> Body {
    let mut build = Build::new();
    let height = |x: f64| MIDDLE + RISING * x;
    let corner = |index: usize| {
        let pick = |bit: usize| {
            if index & bit == 0 {
                -HALF_SIDE
            } else {
                HALF_SIDE
            }
        };
        let (x, y) = (pick(1), pick(2));
        DVec3::new(x, y, if index & 4 == 0 { 0.0 } else { height(x) })
    };
    let corners: Vec<VertexId> = (0..8).map(|index| build.vertex(corner(index))).collect();
    let quad = |build: &mut Build, picked: [usize; 4], outward: DVec3| {
        build.polygon(&picked.map(|index| corners[index]), outward)
    };
    let normal = DVec3::new(-RISING, 0.0, 1.0);
    let bottom = quad(&mut build, [0, 2, 3, 1], DVec3::NEG_Z);
    let top = quad(&mut build, [4, 5, 7, 6], normal);
    quad(&mut build, [0, 4, 6, 2], DVec3::NEG_X);
    quad(&mut build, [1, 3, 7, 5], DVec3::X);
    quad(&mut build, [0, 1, 5, 4], DVec3::NEG_Y);
    quad(&mut build, [2, 6, 7, 3], DVec3::Y);
    let (floor, _) = Plane::through(DVec3::ZERO, DVec3::NEG_Z);
    for side in [-1.0, 1.0] {
        let along = side * (HOLE_RADIUS + apart / 2.0) / (1.0 + RISING * RISING).sqrt();
        let center = DVec3::new(along, 0.0, height(along));
        let cylinder = Cylinder::about(center, normal, HOLE_RADIUS);
        let wall = build.surface(Surface::Cylinder(cylinder));
        let rim = build.circle(wall, (center - cylinder.origin).dot(cylinder.axis), None);
        let cut = Conic::cut(&floor, &Round::Cylinder(cylinder), Scale::of(HALF_SIDE));
        let [ellipse] = cut[..] else {
            panic!("the floor cuts a slanted bore along one ellipse");
        };
        let period = ellipse.period().expect("an ellipse closes");
        let exit = build.conic_edge(ellipse, None, 0.0, period);
        let use_of = |edge, forward| Coedge { edge, forward };
        build.face(
            wall,
            true,
            vec![vec![use_of(exit, false)], vec![use_of(rim, true)]],
        );
        build.add_loop(top, vec![use_of(rim, false)]);
        build.add_loop(bottom, vec![use_of(exit, true)]);
    }
    build.finish(HALF_SIDE)
}

/// What the slanted block holds: its mean height over its floor, less each
/// bore's section times the length of its axis between the top and the
/// floor.
pub(crate) fn slanted_block_with_two_holes_volume(apart: f64) -> f64 {
    let slope = (1.0 + RISING * RISING).sqrt();
    let block = (2.0 * HALF_SIDE).powi(2) * MIDDLE;
    let bores: f64 = [-1.0, 1.0]
        .into_iter()
        .map(|side| {
            let along = side * (HOLE_RADIUS + apart / 2.0) / slope;
            PI * HOLE_RADIUS * HOLE_RADIUS * (MIDDLE + RISING * along) * slope
        })
        .sum();
    block - bores
}

/// The height on the stock's wall a sliver is cut from.
const SLIVER_FOOT: f64 = 5.0;

/// The stock, a sliver cut off its side above half its height by a plane
/// leaning `tilt` from the plane touching its wall along `x = 20`, through
/// the point of that line at half the height: the plane cuts the wall along
/// a hairpin, the vertex of an ellipse whose centre stands `20 / tilt` away,
/// from one corner on the top's rim down to that point and back up to the
/// other.
pub(crate) fn stock_with_a_sliver_cut_at_a_slant(tilt: f64) -> Body {
    let mut build = Build::new();
    let wall = build.cylinder(DVec3::ZERO, DVec3::Z, STOCK_RADIUS);
    let foot = DVec3::new(STOCK_RADIUS, 0.0, SLIVER_FOOT);
    let (slant, slant_flipped) = build.plane(foot, DVec3::new(tilt.cos(), 0.0, tilt.sin()));
    let across = STOCK_RADIUS - (HEIGHT - SLIVER_FOOT) * tilt.tan();
    let half = ((STOCK_RADIUS - across) * (STOCK_RADIUS + across)).sqrt();
    let [left, right] = [-half, half].map(|y| DVec3::new(across, y, HEIGHT));
    let [start, end] = [left, right].map(|corner| build.vertex(corner));
    let low = build.circle(wall, 0.0, None);
    let rim = build.arc(wall, HEIGHT, end, start);
    let chord = build.line(start, end);
    let [hairpin] = build.conic(slant, wall, STOCK_RADIUS)[..] else {
        panic!("a plane at a slant cuts a shaft along one ellipse");
    };
    let span = [left, right].map(|corner| hairpin.parameter(corner));
    let bend = build.conic_edge(hairpin, Some([start, end]), span[0], span[1]);
    let use_of = |edge, forward| Coedge { edge, forward };
    build.face(
        wall,
        false,
        vec![
            vec![use_of(low, true)],
            vec![use_of(rim, false), use_of(bend, false)],
        ],
    );
    let (top, top_flipped) = build.plane(DVec3::Z * HEIGHT, DVec3::Z);
    build.face(
        top,
        top_flipped,
        vec![vec![use_of(rim, true), build.leaving(chord, start)]],
    );
    build.face(
        slant,
        slant_flipped,
        vec![vec![use_of(bend, true), build.leaving(chord, end)]],
    );
    let (bottom, bottom_flipped) = build.plane(DVec3::ZERO, DVec3::NEG_Z);
    build.face(bottom, bottom_flipped, vec![vec![use_of(low, false)]]);
    build.finish(STOCK_RADIUS)
}

/// What the stock holds less the sliver: the segments of its section beyond
/// the slanted plane, `R² acos(c/R) − c √(R² − c²)` at the plane's distance
/// `c` from the axis, summed up the sliver by Simpson's rule in the square
/// root of the height over its foot, where the segment opens as the height
/// to the power three halves.
pub(crate) fn stock_with_a_sliver_cut_at_a_slant_volume(tilt: f64) -> f64 {
    let radius = STOCK_RADIUS;
    let segment = |z: f64| {
        let c = radius - (z - SLIVER_FOOT) * tilt.tan();
        radius * radius * (c / radius).clamp(-1.0, 1.0).acos()
            - c * ((radius - c) * (radius + c)).max(0.0).sqrt()
    };
    const PANELS: usize = 2000;
    let top = (HEIGHT - SLIVER_FOOT).sqrt();
    let width = top / PANELS as f64;
    let at = |w: f64| segment(SLIVER_FOOT + w * w) * 2.0 * w;
    let sliver: f64 = (0..PANELS)
        .map(|panel| {
            let low = width * panel as f64;
            width / 6.0 * (at(low) + 4.0 * at(low + width / 2.0) + at(low + width))
        })
        .sum();
    PI * radius * radius * HEIGHT - sliver
}
