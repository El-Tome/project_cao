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
    let mut registry = Registry::new(scale(), Apart::default(), Planes::default());
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
    let mut registry = Registry::new(scale(), Apart::default(), Planes::default());
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
    let mut registry = Registry::new(scale(), Apart::default(), Planes::default());
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
    let mut registry = Registry::new(scale(), Apart::default(), Planes::default());
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
    let mut registry = Registry::new(scale(), Apart::default(), Planes::default());
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
    let mut registry = Registry::new(scale(), Apart::default(), Planes::default());
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
    let mut registry = Registry::new(scale(), Apart::default(), Planes::default());
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

/// Seed 1008675 of the campaign: two blocks joined with their tops six tenths
/// of a micron apart, more than the tolerance at a reach of 465, then a third
/// whose top stands halfway between, within it of both. Taken for either,
/// the third's top would stand, as the third was made, across the strip of
/// wall between the two: it is taken for neither.
#[test]
fn a_surface_of_the_tool_between_two_of_the_body_s_within_the_tolerance_of_each_is_its_own() {
    let scale = Scale::of(465.0);
    let one = with_surfaces(vec![
        plane_at(150.0, DVec3::Y),
        plane_at(150.000_000_6, DVec3::Y),
    ]);
    let between = with_surfaces(vec![plane_at(150.000_000_3, DVec3::Y)]);
    assert_eq!(
        Surfaces::of(&one, &between, scale).mapped[1],
        vec![(SurfaceId(2), true)]
    );
    let beyond = with_surfaces(vec![plane_at(150.000_000_8, DVec3::Y)]);
    assert_eq!(
        Surfaces::of(&one, &beyond, scale).mapped[1],
        vec![(SurfaceId(1), true)]
    );
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
    let mut registry = Registry::new(scale, Apart::of(&surfaces, scale), Planes::default());
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
    let mut registry = Registry::new(scale, Apart::of(&surfaces, scale), Planes::default());
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

/// Seed 5001541 of the campaign: two blocks whose sides and bottoms stand
/// within the tolerance of each other's, three tenths of a micron apart each
/// way, at a reach of 420. Each plane is one, and the corner of each block
/// where the side meets the bottom stands more than the tolerance from the
/// other's.
fn nearly_one_corner() -> (Registry, [SurfaceId; 3]) {
    let scale = Scale::of(420.0);
    let surfaces = vec![
        plane_at(-120.0, DVec3::Z),
        plane_at(105.0, DVec3::X),
        plane_at(240.0, DVec3::Y),
    ];
    let registry = Registry::new(scale, Apart::of(&surfaces, scale), Planes::of(&surfaces));
    (registry, [0, 1, 2].map(SurfaceId))
}

#[test]
fn two_lines_on_two_planes_across_each_other_are_one_curve_however_far_apart_they_were_found() {
    let (mut registry, [bottom, side, _]) = nearly_one_corner();
    let first = registry.register(
        Curve::Line(Line::through(DVec3::new(105.0, 0.0, -120.0), DVec3::Y)),
        &[bottom, side],
    );
    let apart = DVec3::new(104.999_999_7, 0.0, -119.999_999_7);
    assert!(apart.distance(DVec3::new(105.0, 0.0, -120.0)) > Scale::of(420.0).eps());
    let second = registry.register(Curve::Line(Line::through(apart, DVec3::Y)), &[side, bottom]);
    let alone = registry.register(Curve::Line(Line::through(apart, DVec3::Y)), &[side]);
    assert_eq!((first, second, alone), (0, 0, 1));
}

#[test]
fn two_corners_on_three_planes_whose_normals_span_space_are_one_however_far_apart_they_were_found()
{
    let (registry, [bottom, side, front]) = nearly_one_corner();
    let mut pool = Pool::new(Scale::of(420.0).eps());
    let corner = DVec3::new(105.0, 240.0, -120.0);
    let apart = DVec3::new(104.999_999_7, 240.0, -119.999_999_7);
    let first = pool.add(corner, [bottom, side, front], [], &registry);
    let second = pool.add(apart, [front, side, bottom], [], &registry);
    let third = pool.add(apart, [front, side], [], &registry);
    assert_eq!((first, second, third), (0, 0, 1));
    assert_eq!(pool.corners[0].point, corner);
}

/// Seed 1034172 of the campaign: a block's bottom touches a bore along a
/// line six tenths of a micron short of the block's side, at a reach of 315.
/// The block's corner lies on both within the tolerance, as every place of a
/// band millimetres wide does where two surfaces touch, and is not on the
/// line they touch along: two surfaces touching fix no line the way two
/// crossing do.
#[test]
fn a_corner_on_a_plane_and_a_cylinder_touching_it_lies_on_their_line_only_within_the_tolerance() {
    let scale = Scale::of(315.0);
    let surfaces = vec![
        plane_at(180.0, DVec3::Z),
        Surface::Cylinder(Cylinder::about(
            DVec3::new(0.0, 209.999_999_4, 225.0),
            DVec3::X,
            45.0,
        )),
    ];
    let mut registry = Registry::new(scale, Apart::of(&surfaces, scale), Planes::of(&surfaces));
    let [bottom, bore] = [0, 1].map(SurfaceId);
    let touch = registry.register(
        Curve::Line(Line::through(
            DVec3::new(0.0, 209.999_999_4, 180.0),
            DVec3::X,
        )),
        &[bottom, bore],
    );
    let on = [bottom, bore];
    let corner = DVec3::new(105.0, 210.0, 180.0);
    assert!(!lies_on(corner, &on, touch, &registry, scale.eps()));
    let near = DVec3::new(105.0, 209.999_999_6, 180.0);
    assert!(lies_on(near, &on, touch, &registry, scale.eps()));
}

/// Seed 1026137 of the campaign: a block's side a boss touches a tenth of a
/// micron off, within the tolerance at a reach of 110, and a corner an
/// earlier cut left on the side, where another wall touched it five
/// hundredths of a micron inside. The corner stands within the tolerance of
/// the line the side and the boss touch along, and that line within it of
/// the boss, but the corner not: it is not on the line.
#[test]
fn a_corner_within_the_tolerance_of_a_line_of_touch_but_not_of_both_surfaces_is_off_it() {
    let scale = Scale::of(110.0);
    let surfaces = vec![
        plane_at(42.5, DVec3::Y),
        Surface::Cylinder(Cylinder::about(
            DVec3::new(0.0, 47.5, 25.0),
            DVec3::X,
            4.999_999_9,
        )),
    ];
    let mut registry = Registry::new(scale, Apart::of(&surfaces, scale), Planes::of(&surfaces));
    let [side, boss] = [0, 1].map(SurfaceId);
    let touch = registry.register(
        Curve::Line(Line::through(DVec3::new(0.0, 42.5, 25.0), DVec3::X)),
        &[side, boss],
    );
    let corner = DVec3::new(10.000_000_1, 42.499_999_95, 25.0);
    assert!(!lies_on(
        corner,
        &[side, boss],
        touch,
        &registry,
        scale.eps()
    ));
    let on_both = DVec3::new(10.000_000_1, 42.500_000_05, 25.0);
    assert!(lies_on(
        on_both,
        &[side, boss],
        touch,
        &registry,
        scale.eps()
    ));
}

/// Seed 1019570 of the campaign: a circle of radius 2.5 tangent to a block's
/// bottom side, its centre two hundredths of a micron short of the block's
/// end side, at a reach of 13. From where it touches the side to the corner
/// it runs within a hair of the side, and round the other way it leaves it.
#[test]
fn a_circle_parts_from_a_side_it_touches_by_a_hair_up_to_a_corner_a_hair_along_it() {
    let eps = Scale::of(13.0).eps();
    let cylinder = Cylinder::about(DVec3::new(4.0, 4.499_999_98, 10.5), DVec3::X, 2.5);
    let circle = Circle::on(&cylinder, 4.0);
    let side = Line::through(DVec3::new(4.0, 0.0, 8.0), DVec3::Y);
    let [touch, corner] = [4.499_999_98, 4.5].map(|y| DVec3::new(4.0, y, 8.0));
    let [one, other] = [touch, corner].map(|point| circle.parameter(point));
    let short = [one.min(other), one.max(other)];
    let long = [one.max(other), one.min(other) + TAU];
    let along = [touch, corner].map(|point| side.parameter(point));
    let along = [along[0].min(along[1]), along[0].max(along[1])];
    let [circle, side] = [Curve::Circle(circle), Curve::Line(side)];
    assert!(parting(&circle, short, &side, along) < eps / 1000.0);
    assert!(parting(&side, along, &circle, short) < eps / 1000.0);
    assert!(parting(&circle, long, &side, along) > 1.0);
}

/// Two circles of one radius on one plane, their centres three tenths of a
/// micron apart: they cross where the line between the centres is square to
/// the radius, and part by the whole offset where it runs along it.
#[test]
fn two_circles_of_one_radius_a_hair_apart_part_by_the_offset_where_they_run_across_it() {
    let offset = 3e-7;
    let one = Circle::on(&Cylinder::about(DVec3::ZERO, DVec3::Z, 10.0), 0.0);
    let other = Circle::on(&Cylinder::about(DVec3::X * offset, DVec3::Z, 10.0), 0.0);
    let [one, other] = [one, other].map(Curve::Circle);
    let near = [PI / 2.0 - 1e-3, PI / 2.0 + 1e-3];
    let whole = [0.0, TAU];
    assert!(parting(&one, near, &other, near) < 1e-9);
    let far = parting(&one, whole, &other, whole);
    assert!((far - offset).abs() < 1e-12, "{far}");
}

/// The surfaces of a pair, the first carried by the first operand, the
/// second by the second, and by the first too when `both` says so.
fn snapped_pair(first: Surface, second: Surface, both: bool) -> Vec<Surface> {
    let mut surfaces = Surfaces {
        list: vec![first, second],
        mapped: [Vec::new(), Vec::new()],
    };
    surfaces.snapped(
        |operand, surface| match (operand, surface.0) {
            (0, 0) | (1, 1) => true,
            (0, 1) => both,
            _ => false,
        },
        scale(),
    );
    surfaces.list
}

/// Decision 2: a hole a hair past touching a block's top, within the
/// tolerance, is moved onto the touch, as a surface, so that the line they
/// touch along lies on both; the block's plane stays where it was made.
#[test]
fn a_cylinder_of_the_second_operand_a_hair_past_touching_a_plane_is_moved_onto_it() {
    let top = plane_at(10.0, DVec3::Z);
    let hole = Cylinder::about(
        DVec3::new(0.0, 5.0, 7.0),
        DVec3::X,
        3.0 + scale().eps() / 2.0,
    );
    let [plane, moved] = snapped_pair(top, Surface::Cylinder(hole), false)[..] else {
        panic!("two surfaces");
    };
    assert_eq!(plane, top);
    let Surface::Cylinder(moved) = moved else {
        panic!("a cylinder");
    };
    assert_eq!(moved.radius, hole.radius);
    assert!((10.0 - moved.origin.z - moved.radius).abs() < 1e-14);
}

/// The same with the plane the second operand's: the plane moves.
#[test]
fn a_plane_of_the_second_operand_a_hair_from_touching_a_cylinder_is_moved_onto_it() {
    let post = Cylinder::about(DVec3::new(0.0, 5.0, 7.0), DVec3::X, 3.0);
    let lid = plane_at(10.0 - scale().eps() / 2.0, DVec3::Z);
    let [kept, moved] = snapped_pair(Surface::Cylinder(post), lid, false)[..] else {
        panic!("two surfaces");
    };
    assert_eq!(kept, Surface::Cylinder(post));
    assert_eq!(moved, plane_at(10.0, DVec3::Z));
}

/// Two parallel cylinders a hair inside touching move the second operand's
/// onto the touch; a surface both operands carry is never moved.
#[test]
fn a_bore_a_hair_past_touching_the_stock_inside_is_moved_onto_it_but_not_a_shared_surface() {
    let stock = Cylinder::about(DVec3::ZERO, DVec3::Z, 10.0);
    let bore = Cylinder::about(DVec3::X * 4.0, DVec3::Z, 6.0 + scale().eps() / 2.0);
    let [_, moved] = snapped_pair(Surface::Cylinder(stock), Surface::Cylinder(bore), false)[..]
    else {
        panic!("two surfaces");
    };
    let Surface::Cylinder(moved) = moved else {
        panic!("a cylinder");
    };
    assert!((moved.origin.x + moved.radius - 10.0).abs() < 1e-14);
    let shared = snapped_pair(Surface::Cylinder(stock), Surface::Cylinder(bore), true);
    assert_eq!(shared, [Surface::Cylinder(stock), Surface::Cylinder(bore)]);
}

/// A hole touching one wall of a slot exactly and the other a hair off is
/// left where it is: moved onto the second, it would leave the first.
#[test]
fn a_cylinder_touching_one_plane_exactly_is_not_moved_onto_another() {
    let eps = scale().eps();
    let floor = plane_at(0.0, DVec3::Y);
    let ceiling = plane_at(6.0 + eps / 2.0, DVec3::Y);
    let hole = Cylinder::about(DVec3::new(0.0, 3.0, 0.0), DVec3::Z, 3.0);
    let mut surfaces = Surfaces {
        list: vec![floor, ceiling, Surface::Cylinder(hole)],
        mapped: [Vec::new(), Vec::new()],
    };
    surfaces.snapped(
        |operand, surface| (operand == 1) == (surface.0 == 2),
        scale(),
    );
    assert_eq!(surfaces.list, [floor, ceiling, Surface::Cylinder(hole)]);
}
