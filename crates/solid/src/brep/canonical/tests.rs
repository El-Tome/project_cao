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
    let mut registry = Registry::new(scale());
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
    let mut registry = Registry::new(scale());
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
    let mut registry = Registry::new(scale());
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
    let mut registry = Registry::new(scale());
    let edge = registry.register(
        Curve::Line(Line::through(DVec3::new(20.0, 0.0, 10.0), DVec3::Y)),
        &[SurfaceId(1), SurfaceId(4)],
    );
    let mut pool = Pool::new(eps);
    let corner = DVec3::new(20.0, 5.0, 10.0);
    let first = pool.add(corner, [SurfaceId(6)], [edge]);
    let second = pool.add(corner + DVec3::X * eps / 2.0, [SurfaceId(9)], []);
    let third = pool.add(corner + DVec3::X * 2.0 * eps, [SurfaceId(9)], []);
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
    let mut registry = Registry::new(scale());
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
    let mut registry = Registry::new(scale());
    let [plane, other] = [SurfaceId(0), SurfaceId(1)];
    let hole = Cylinder::about(DVec3::new(7.0, 5.0, 0.0), DVec3::Z, 4.0);
    let circle = registry.register(Curve::Circle(Circle::on(&hole, 6.0)), &[plane]);
    let on_it = Circle::on(&hole, 6.0).point(1.0);
    let off_it = DVec3::new(1.0, 3.0, 6.0);
    assert!(lies_on(on_it, &[plane, other], circle, &registry, eps));
    assert!(!lies_on(off_it, &[plane, other], circle, &registry, eps));
}
