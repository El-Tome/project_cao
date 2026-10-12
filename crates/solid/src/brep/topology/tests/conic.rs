//! Faces of a point bounded by the conics a plane cuts it along (#552),
//! built by hand from the arenas before any boolean makes one: the tip a
//! plane at a slant cuts off, held by an ellipse ring alone, and the two
//! faces a flat milled along its axis leaves on its slope, either side of a
//! hyperbola.
//!
//! The point is a cone of base radius ten on the XY plane, its apex at
//! (0, 0, 20).

use std::f64::consts::TAU;

use glam::{DVec2, DVec3};

use super::*;
use crate::brep::conic::Conic;
use crate::brep::surface::Cone;
use crate::brep::tessellation::tests::fixtures::Build;
use crate::brep::topology::Coedge;

pub(in crate::brep) const BASE: f64 = 10.0;
pub(in crate::brep) const TIP: f64 = 20.0;

pub(in crate::brep) fn point_cone() -> Cone {
    Cone::through(
        DVec3::ZERO,
        DVec3::Z,
        [DVec2::new(0.0, BASE), DVec2::new(TIP, 0.0)],
    )
}

/// The faces of a point a plane cut along a conic: the face on the plane,
/// and on the cone the one holding the apex and the one beyond the plane.
pub(in crate::brep) struct Cut {
    pub body: Body,
    pub conic: Conic,
    pub flat: FaceId,
    pub tip: FaceId,
    pub beyond: FaceId,
}

/// The point cut by the plane through (0, 0, 10) leaning along X by `lean`:
/// an ellipse round the axis, with no vertex. The tip it cuts off is held
/// by the ellipse alone; the frustum's slope beneath, by its base ring and
/// the ellipse.
pub(in crate::brep) fn point_cut_at_a_slant(lean: f64) -> Cut {
    let mut build = Build::new();
    let cone = build.surface(Surface::Cone(point_cone()));
    let (plane, flipped) = build.plane(DVec3::new(0.0, 0.0, 10.0), DVec3::new(lean, 0.0, 1.0));
    let [conic] = build.conic(plane, cone, TIP)[..] else {
        panic!("a plane at a slant across a point cuts one ellipse");
    };
    let period = conic.period().expect("an ellipse closes");
    let ring = build.edge(
        Curve::Circle(point_cone().circle(0.0, BASE)),
        None,
        0.0,
        TAU,
    );
    let ellipse = build.conic_edge(conic, None, 0.0, period);
    let use_of = |edge, forward| Coedge { edge, forward };
    let flat = build.face(plane, flipped, vec![vec![use_of(ellipse, true)]]);
    let tip = build.face(cone, true, vec![vec![use_of(ellipse, true)]]);
    let beyond = build.face(
        cone,
        true,
        vec![vec![use_of(ring, true)], vec![use_of(ellipse, false)]],
    );
    Cut {
        body: build.finish(TIP),
        conic,
        flat,
        tip,
        beyond,
    }
}

/// The point with a flat milled along its axis, `x ≤ at` kept: a hyperbola
/// from the base up to its vertex at `(at, 0, 20 − 2·at)` and back down.
/// The flat is bounded by it and the base line under it; the slope left
/// goes round the axis from one end of the hyperbola to the other and holds
/// the apex; the sliver milled off, beyond the plane, does not.
pub(in crate::brep) fn point_with_a_flat(at: f64) -> Cut {
    let mut build = Build::new();
    let cone = build.surface(Surface::Cone(point_cone()));
    let (plane, flipped) = build.plane(DVec3::new(at, 0.0, 0.0), DVec3::X);
    let [conic] = build.conic(plane, cone, TIP)[..] else {
        panic!("a plane along a point's axis cuts one branch of a hyperbola");
    };
    let half = (BASE * BASE - at * at).sqrt();
    let [low, high] = [-half, half].map(|y| build.vertex(DVec3::new(at, y, 0.0)));
    let [start, end] = [-half, half].map(|y| conic.parameter(DVec3::new(at, y, 0.0)));
    assert!(
        start < end,
        "the hyperbola runs the way the cone's angle turns"
    );
    let hyperbola = build.conic_edge(conic, Some([low, high]), start, end);
    let circle = point_cone().circle(0.0, BASE);
    let [from, to] = [-half, half].map(|y| circle.parameter(DVec3::new(at, y, 0.0)));
    let to = from + (to - from).rem_euclid(TAU);
    assert!(
        circle.point((from + to) / 2.0).x > at,
        "the rim runs from one end of the hyperbola to the other beyond the flat"
    );
    let rim = Curve::Circle(circle);
    let near = build.edge(rim, Some([low, high]), from, to);
    let far = build.edge(rim, Some([high, low]), to, from + TAU);
    let floor = build.line(high, low);
    let use_of = |edge, forward| Coedge { edge, forward };
    let flat = build.face(
        plane,
        flipped,
        vec![vec![use_of(hyperbola, true), build.leaving(floor, high)]],
    );
    let tip = build.face(
        cone,
        true,
        vec![vec![use_of(hyperbola, true), use_of(far, true)]],
    );
    let beyond = build.face(
        cone,
        true,
        vec![vec![use_of(near, true), use_of(hyperbola, false)]],
    );
    Cut {
        body: build.finish(TIP),
        conic,
        flat,
        tip,
        beyond,
    }
}

/// The point's slope between two flats milled along its axis, `|x| ≤ at`
/// kept, as a screwdriver's tip: two hyperbolas and the two stretches of
/// the rim between them, the rim going less than half way round.
pub(in crate::brep) fn slope_between_two_flats(at: f64) -> (Body, FaceId) {
    let mut build = Build::new();
    let cone = build.surface(Surface::Cone(point_cone()));
    let half = (BASE * BASE - at * at).sqrt();
    let circle = point_cone().circle(0.0, BASE);
    let mut hyperbolas = Vec::new();
    for side in [1.0, -1.0] {
        let (plane, _) = build.plane(DVec3::new(side * at, 0.0, 0.0), DVec3::X * side);
        let [conic] = build.conic(plane, cone, TIP)[..] else {
            panic!("a plane along a point's axis cuts one branch of a hyperbola");
        };
        let ends = [-side * half, side * half].map(|y| DVec3::new(side * at, y, 0.0));
        let [start, end] = ends.map(|point| conic.parameter(point));
        assert!(
            start < end,
            "the hyperbola runs the way the cone's angle turns"
        );
        let vertices = ends.map(|point| build.vertex(point));
        let edge = build.conic_edge(conic, Some(vertices), start, end);
        hyperbolas.push((edge, vertices, ends));
    }
    let mut lap = Vec::new();
    for (rank, (edge, [_, high], [_, from])) in hyperbolas.iter().enumerate() {
        let (_, [low, _], [to, _]) = hyperbolas[(rank + 1) % 2];
        let start = circle.parameter(*from);
        let end = start + (circle.parameter(to) - start).rem_euclid(TAU);
        let rim = build.edge(Curve::Circle(circle), Some([*high, low]), start, end);
        lap.push(Coedge {
            edge: *edge,
            forward: true,
        });
        lap.push(Coedge {
            edge: rim,
            forward: true,
        });
    }
    let slope = build.face(cone, true, vec![lap]);
    (build.finish(TIP), slope)
}

/// The apex each face is given, as a boolean gives it to the faces it keeps.
fn apexes(cut: &Cut) -> Vec<Option<DVec3>> {
    let mut body = cut.body.clone();
    let mut faces = body.faces.clone();
    body.hold_apexes(&mut faces);
    faces
        .iter()
        .map(|face| face.apex.map(|vertex| body.vertex(vertex).point))
        .collect()
}

#[test]
fn a_cone_face_held_by_an_ellipse_round_its_axis_holds_its_apex() {
    for lean in [0.3, -0.45, 1e-6] {
        let cut = point_cut_at_a_slant(lean);
        let held = apexes(&cut);
        assert_eq!(
            held[cut.tip.0 as usize],
            Some(DVec3::new(0.0, 0.0, TIP)),
            "the tip leaning by {lean}"
        );
        assert_eq!(
            held[cut.beyond.0 as usize], None,
            "the slope leaning by {lean}"
        );
        assert_eq!(held[cut.flat.0 as usize], None);
    }
}

#[test]
fn a_cone_face_cut_along_a_hyperbola_does_not_hold_its_apex() {
    for at in [4.0, 0.5, 9.5] {
        let cut = point_with_a_flat(at);
        let held = apexes(&cut);
        assert_eq!(
            held[cut.beyond.0 as usize], None,
            "the sliver beyond x = {at}"
        );
        assert_eq!(
            held[cut.tip.0 as usize],
            Some(DVec3::new(0.0, 0.0, TIP)),
            "the slope kept short of x = {at} goes round the axis"
        );
    }
}

#[test]
fn a_cone_face_between_two_hyperbolas_goes_round_its_axis_and_holds_its_apex() {
    for at in [4.0, 1.0, 9.0] {
        let (mut body, slope) = slope_between_two_flats(at);
        let mut faces = body.faces.clone();
        body.hold_apexes(&mut faces);
        let apex = faces[slope.0 as usize]
            .apex
            .map(|vertex| body.vertex(vertex).point);
        assert_eq!(
            apex,
            Some(DVec3::new(0.0, 0.0, TIP)),
            "the slope between flats {at} off the axis, its rim short of half a turn"
        );
    }
}
