//! Faces bounded by the conics a plane cuts a wall along (#552), read about
//! their reach exactly and about an axis conservatively.

use glam::DVec3;

use super::{flat, plate};
use crate::brep::combine::clear::{apart, farthest, radii, reach};
use crate::brep::conic::Conic;
use crate::brep::curve::Curve;
use crate::brep::surface::Surface;
use crate::brep::tessellation::tests::fixtures;
use crate::brep::topology::tests::conic::{
    BASE, point_cone, point_cut_at_a_slant, point_with_a_flat,
};
use crate::brep::topology::{Body, FaceId};

const SAMPLES: usize = 200_000;

/// The least and the largest of `direction · p` along a conic's stretch,
/// read off samples finer than any tolerance asked of it here.
fn sampled(conic: &Conic, [from, to]: [f64; 2], direction: DVec3) -> [f64; 2] {
    (0..=SAMPLES)
        .map(|step| {
            let t = from + (to - from) * step as f64 / SAMPLES as f64;
            direction.dot(conic.point(t))
        })
        .fold([f64::INFINITY, f64::NEG_INFINITY], |[low, high], along| {
            [low.min(along), high.max(along)]
        })
}

fn slanted_top(body: &Body) -> FaceId {
    body.face_ids()
        .find(|&face| match body.surface(body.face(face).surface) {
            Surface::Plane(plane) => plane.normal.x != 0.0,
            Surface::Cylinder(_)
            | Surface::Cone(_)
            | Surface::Elliptic(_)
            | Surface::Sphere(_)
            | Surface::Torus(_) => false,
        })
        .expect("the slanted top")
}

#[test]
fn a_face_bounded_by_an_ellipse_reaches_along_a_direction_as_far_as_its_conic_does() {
    let body = fixtures::stock_cut_at_a_slant(0.25);
    let top = slanted_top(&body);
    let edge = body.edge(body.face(top).loops[0][0].edge);
    let Curve::Conic(conic) = *body.curve(edge.curve) else {
        panic!("the top is bounded by its ellipse");
    };
    for direction in [
        DVec3::Z,
        DVec3::X,
        DVec3::new(1.0, 1.0, 1.0).normalize(),
        DVec3::new(0.3, -1.0, 0.2).normalize(),
    ] {
        let [low, high] =
            reach((&body, top), direction).expect("a face bounded by a conic reaches");
        let [least, most] = sampled(&conic, [edge.from, edge.to], direction);
        assert!(
            low <= least && least - low <= 1e-9 && high >= most && high - most <= 1e-9,
            "along {direction} the top reaches {low}..{high}, its ellipse {least}..{most}"
        );
    }
}

#[test]
fn a_cone_face_bounded_by_a_conic_stands_from_its_axis_out_to_its_widest() {
    let cone = point_cone();
    let off = |point: DVec3| point.truncate().length();
    let cut = point_cut_at_a_slant(0.3);
    let period = cut.conic.period().expect("an ellipse closes");
    let widest = (0..=SAMPLES)
        .map(|step| off(cut.conic.point(period * step as f64 / SAMPLES as f64)))
        .fold(0.0, f64::max);
    let [inner, outer] = radii(&cone, (&cut.body, cut.tip)).expect("read off its ellipse");
    assert!(
        inner == 0.0 && outer >= widest && outer - widest <= 1e-9,
        "the tip spans {inner}..{outer} about its axis, its ellipse out to {widest}"
    );
    let farthest =
        farthest((&cut.body, cut.tip), cone.origin, cone.axis).expect("read off its ellipse's box");
    assert!(farthest >= widest, "{farthest} short of {widest}");
    let flat = point_with_a_flat(4.0);
    let [inner, outer] =
        radii(&cone, (&flat.body, flat.beyond)).expect("read off its hyperbola and rim");
    assert!(
        inner == 0.0 && (outer - BASE).abs() <= 1e-12,
        "the sliver spans {inner}..{outer}"
    );
}

#[test]
fn a_face_bounded_by_a_conic_stands_clear_of_a_face_beyond_its_reach_and_not_of_one_it_crosses() {
    let body = fixtures::stock_cut_at_a_slant(0.25);
    let top = slanted_top(&body);
    let normal = DVec3::new(1.0, 0.0, -1.0);
    for (origin, expected) in [
        (DVec3::new(0.0, 0.0, 40.0), true),
        (DVec3::new(0.0, 60.0, 10.0), true),
        (DVec3::new(0.0, 0.0, 10.0), false),
    ] {
        let slab = plate(origin, normal, 20.0);
        assert_eq!(
            apart((&body, top), (&slab, flat(&slab, normal)), 1e-8),
            expected,
            "a slab through {origin}"
        );
    }
}
