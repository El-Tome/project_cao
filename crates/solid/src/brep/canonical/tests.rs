use std::f64::consts::{PI, TAU};

use glam::{DVec2, DVec3};

use super::*;
use crate::brep::curve::{Circle, Curve, Line};
use crate::brep::scale::Scale;
use crate::brep::surface::{Cylinder, Plane, Surface};
use crate::brep::topology::{Body, SurfaceId};
use crate::profile::{Contour, Frame};

fn ground(height: f64) -> Frame {
    Frame {
        origin: DVec3::Z * height,
        u: DVec3::X,
        v: DVec3::Y,
    }
}

fn block(low: [f64; 3], high: [f64; 3]) -> Body {
    let outline = Contour::rectangle(DVec2::new(low[0], low[1]), DVec2::new(high[0], high[1]));
    Body::raised(&outline, &[], ground(low[2]), DVec3::Z * (high[2] - low[2]))
        .expect("a block raises")
}

fn scale() -> Scale {
    Scale::of(60.0)
}

fn plane_at(offset: f64, normal: DVec3) -> Surface {
    Surface::Plane(Plane::through(normal * offset, normal).0)
}

#[test]
fn two_blocks_sharing_a_wall_share_every_plane_but_the_far_wall() {
    let one = block([-20.0, -20.0, 0.0], [20.0, 20.0, 10.0]);
    let other = block([20.0, -20.0, 0.0], [60.0, 20.0, 10.0]);
    let surfaces = Surfaces::of(&one, &other, scale());
    assert_eq!(surfaces.list.len(), one.surfaces.len() + 1);
    assert_eq!(
        &surfaces.list[..one.surfaces.len()],
        one.surfaces.as_slice()
    );
    let new = SurfaceId(one.surfaces.len() as u32);
    let far = surfaces.mapped[1]
        .iter()
        .filter(|(surface, _)| *surface == new)
        .count();
    assert_eq!(far, 1);
    assert!(surfaces.mapped[1].iter().all(|(_, agree)| *agree));
    assert_eq!(surfaces.list[new.0 as usize], plane_at(60.0, DVec3::X));
}

#[test]
fn a_surface_of_the_first_operand_is_never_taken_for_another_of_its_own() {
    let one = block([-20.0, -20.0, 0.0], [20.0, 20.0, 10.0]);
    let surfaces = Surfaces::of(&one, &one, scale());
    assert_eq!(surfaces.list.len(), one.surfaces.len());
    let expected: Vec<(SurfaceId, bool)> = (0..one.surfaces.len() as u32)
        .map(|rank| (SurfaceId(rank), true))
        .collect();
    assert_eq!(surfaces.mapped, [expected.clone(), expected]);
}

#[test]
fn a_line_found_twice_within_the_tolerance_is_one_curve_lying_on_every_surface_it_was_found_on() {
    let eps = scale().eps();
    let mut registry = Registry::new(scale(), Apart::default());
    let edge = Line::through(DVec3::new(20.0, 0.0, 10.0), DVec3::Y);
    let again = Line::through(DVec3::new(20.0 + eps / 2.0, 3.0, 10.0), -DVec3::Y);
    let apart = Line::through(DVec3::new(20.0 + 2.0 * eps, 3.0, 10.0), DVec3::Y);
    let [top, side, wall] = [SurfaceId(1), SurfaceId(4), SurfaceId(7)];
    let first = registry.register(Curve::Line(edge), &[top, side]);
    let second = registry.register(Curve::Line(again), &[wall, top]);
    let third = registry.register(Curve::Line(apart), &[wall]);
    assert_eq!((first, second, third), (0, 0, 1));
    assert_eq!(registry.list[0].support, vec![top, side, wall]);
    assert_eq!(registry.list[0].curve, Curve::Line(edge));
}

#[test]
fn a_circle_found_on_a_cap_and_on_its_hole_is_one_curve() {
    let hole = Cylinder::about(DVec3::new(8.0, 0.0, 0.0), DVec3::Z, 5.0);
    let mut registry = Registry::new(scale(), Apart::default());
    let cap = Circle::on(&hole, 10.0);
    let turned = Circle {
        axis: -cap.axis,
        v: -cap.v,
        ..cap
    };
    assert_eq!(
        registry.register(Curve::Circle(cap), &[SurfaceId(0), SurfaceId(2)]),
        0
    );
    assert_eq!(registry.register(Curve::Circle(turned), &[SurfaceId(5)]), 0);
    assert_eq!(
        registry.register(Curve::Circle(Circle::on(&hole, 0.0)), &[SurfaceId(2)]),
        1
    );
    assert_eq!(
        registry.list[0].support,
        vec![SurfaceId(0), SurfaceId(2), SurfaceId(5)]
    );
}

#[test]
fn a_stretch_of_a_circle_turning_the_other_way_is_read_up_the_registered_one() {
    let hole = Cylinder::about(DVec3::new(8.0, 0.0, 0.0), DVec3::Z, 5.0);
    let cap = Circle::on(&hole, 10.0);
    let turned = Circle {
        axis: -cap.axis,
        v: -cap.v,
        ..cap
    };
    let mut registry = Registry::new(scale(), Apart::default());
    let rank = registry.register(Curve::Circle(cap), &[]);
    let [from, to] = registry.stretch(rank, &Curve::Circle(turned), 0.5, 0.5 + PI / 2.0);
    assert!((from - (-0.5 - PI / 2.0)).abs() < 1e-12, "{from}");
    assert!((to - (-0.5)).abs() < 1e-12, "{to}");
    let [start, end] = registry.stretch(rank, &Curve::Circle(cap), 0.0, TAU);
    assert!(start.abs() < 1e-12 && (end - TAU).abs() < 1e-12);
}

#[test]
fn corners_found_within_the_tolerance_are_one_and_lie_on_every_surface_either_was_found_on() {
    let eps = scale().eps();
    let mut registry = Registry::new(scale(), Apart::default());
    let edge = registry.register(
        Curve::Line(Line::through(DVec3::new(20.0, 0.0, 10.0), DVec3::Y)),
        &[SurfaceId(1), SurfaceId(4)],
    );
    let mut pool = Pool::new(eps);
    let corner = DVec3::new(20.0, 5.0, 10.0);
    let first = pool.add(corner, [SurfaceId(6)], [edge], &registry);
    let second = pool.add(corner + DVec3::X * eps / 2.0, [SurfaceId(9)], [], &registry);
    let third = pool.add(corner + DVec3::X * 2.0 * eps, [SurfaceId(9)], [], &registry);
    assert_eq!((first, second, third), (0, 0, 1));
    assert_eq!(pool.corners[0].point, corner);
    assert_eq!(
        pool.supports(&registry)[0],
        vec![SurfaceId(1), SurfaceId(4), SurfaceId(6), SurfaceId(9)]
    );
}

#[test]
fn a_corner_lies_on_the_nearer_of_the_two_lines_a_plane_cuts_a_cylinder_along() {
    let eps = scale().eps();
    let mut registry = Registry::new(scale(), Apart::default());
    let [plane, cylinder, cap] = [SurfaceId(0), SurfaceId(1), SurfaceId(2)];
    let near = registry.register(
        Curve::Line(Line::through(DVec3::new(20.0, -3.0, 0.0), DVec3::Z)),
        &[plane, cylinder],
    );
    let far = registry.register(
        Curve::Line(Line::through(DVec3::new(20.0, 3.0, 0.0), DVec3::Z)),
        &[plane, cylinder],
    );
    let support = vec![plane, cylinder, cap];
    let corner = DVec3::new(20.0, -3.0, 10.0);
    assert!(lies_on(corner, &support, near, &registry, eps));
    assert!(!lies_on(corner, &support, far, &registry, eps));
    assert!(!lies_on(corner, &[plane, cap], near, &registry, eps));
}

#[test]
fn a_corner_on_the_one_surface_a_curve_was_found_on_lies_on_it_only_where_it_stands() {
    let eps = scale().eps();
    let mut registry = Registry::new(scale(), Apart::default());
    let [plane, other] = [SurfaceId(0), SurfaceId(1)];
    let hole = Cylinder::about(DVec3::new(7.0, 5.0, 0.0), DVec3::Z, 4.0);
    let circle = registry.register(Curve::Circle(Circle::on(&hole, 6.0)), &[plane]);
    let on_it = Circle::on(&hole, 6.0).point(1.0);
    let off_it = DVec3::new(1.0, 3.0, 6.0);
    assert!(lies_on(on_it, &[plane, other], circle, &registry, eps));
    assert!(!lies_on(off_it, &[plane, other], circle, &registry, eps));
}

#[test]
fn a_corner_found_on_two_surfaces_of_a_line_lying_on_three_lies_on_the_line_and_on_the_third() {
    let eps = scale().eps();
    let mut registry = Registry::new(scale(), Apart::default());
    let [side, tangent, cylinder, top] = [SurfaceId(0), SurfaceId(1), SurfaceId(2), SurfaceId(3)];
    let edge = registry.register(
        Curve::Line(Line::through(DVec3::new(2.5, 4.0, 0.0), DVec3::Z)),
        &[side, tangent, cylinder],
    );
    let across = registry.register(
        Curve::Line(Line::through(DVec3::new(2.5, 0.0, 12.5), DVec3::Y)),
        &[side, top],
    );
    let mut pool = Pool::new(eps);
    let corner = DVec3::new(2.5, 4.0, 12.5);
    pool.add(corner, [cylinder], [across], &registry);
    let supports = pool.supports(&registry);
    assert_eq!(supports[0], vec![side, tangent, cylinder, top]);
    assert!(lies_on(corner, &supports[0], edge, &registry, eps));
}

fn with_surfaces(surfaces: Vec<Surface>) -> Body {
    Body {
        surfaces,
        curves: Vec::new(),
        vertices: Vec::new(),
        edges: Vec::new(),
        faces: Vec::new(),
        scale: Scale::of(1.0),
    }
}

#[test]
fn a_surface_within_the_tolerance_of_two_of_the_first_operand_s_is_the_nearest() {
    let scale = Scale::of(25.0);
    let hair = 0.8 * scale.eps();
    let one = with_surfaces(vec![
        plane_at(7.0, DVec3::Z),
        plane_at(7.0 + hair, DVec3::Z),
        plane_at(7.0 - hair / 2.0, DVec3::Z),
    ]);
    let other = with_surfaces(vec![plane_at(7.0 + hair, DVec3::Z)]);
    let surfaces = Surfaces::of(&one, &other, scale);
    assert_eq!(surfaces.mapped[1], vec![(SurfaceId(1), true)]);
}

/// Seed 243 of the campaign: a slit a tenth of a micron wide, left by a cut
/// at a reach of 95, and a later cut reaching 198, whose tolerance is twice
/// the slit.
fn slit_walls() -> (Vec<Surface>, Scale) {
    let surfaces = vec![
        plane_at(22.5, DVec3::X),
        plane_at(22.500_000_1, DVec3::X),
        plane_at(42.5, DVec3::Y),
        plane_at(-35.0, DVec3::Z),
    ];
    (surfaces, Scale::of(198.0))
}

#[test]
fn two_surfaces_a_body_kept_two_are_apart_whatever_tolerance_a_later_operation_brings() {
    let (walls, scale) = slit_walls();
    assert!(scale.eps() > 1e-7);
    let tube = Cylinder::about(DVec3::ZERO, DVec3::Z, 5.0);
    let bore = Cylinder::about(DVec3::ZERO, DVec3::Z, 3.0);
    let beside = Cylinder::about(DVec3::X * 4.0, DVec3::Z, 3.0);
    let mut surfaces = walls;
    surfaces.extend([tube, bore, beside].map(Surface::Cylinder));
    let apart = Apart::of(&surfaces, scale);
    let [one, other, side, floor] = [0, 1, 2, 3].map(SurfaceId);
    let [tube, bore, beside] = [4, 5, 6].map(SurfaceId);
    assert!(apart.pair(one, other) && apart.pair(other, one));
    assert!(!apart.pair(one, side) && !apart.pair(one, floor));
    assert!(apart.pair(tube, bore) && apart.pair(one, tube));
    assert!(!apart.pair(tube, beside) && !apart.pair(floor, tube));
}

#[test]
fn lines_within_the_tolerance_on_two_surfaces_decided_apart_are_two_curves() {
    let (surfaces, scale) = slit_walls();
    let mut registry = Registry::new(scale, Apart::of(&surfaces, scale));
    let [one, other, side] = [0, 1, 2].map(SurfaceId);
    let first = registry.register(
        Curve::Line(Line::through(DVec3::new(22.5, 42.5, 0.0), DVec3::Z)),
        &[one, side],
    );
    let second = registry.register(
        Curve::Line(Line::through(DVec3::new(22.500_000_1, 42.5, 0.0), DVec3::Z)),
        &[other, side],
    );
    assert_eq!((first, second), (0, 1));
    assert_eq!(registry.list[0].support, vec![one, side]);
    assert!(!registry.admits(first, other));
    assert!(registry.admits(first, SurfaceId(3)));
}

#[test]
fn corners_within_the_tolerance_on_two_surfaces_decided_apart_are_two_corners() {
    let (surfaces, scale) = slit_walls();
    let mut registry = Registry::new(scale, Apart::of(&surfaces, scale));
    let [one, other, side, floor] = [0, 1, 2, 3].map(SurfaceId);
    let edge = registry.register(
        Curve::Line(Line::through(DVec3::new(22.5, 42.5, 0.0), DVec3::Z)),
        &[one, side],
    );
    let mut pool = Pool::new(scale.eps());
    let corner = DVec3::new(22.5, 42.5, -35.0);
    let first = pool.add(corner, [floor], [edge], &registry);
    let second = pool.add(
        corner + DVec3::X * 1e-7,
        [other, side, floor],
        [],
        &registry,
    );
    let third = pool.add(corner + DVec3::Y * 1e-7, [side, floor], [], &registry);
    assert_eq!((first, second, third), (0, 1, 0));
    let supports = pool.supports(&registry);
    assert_eq!(supports[0], vec![one, side, floor]);
    assert_eq!(supports[1], vec![other, side, floor]);
}
