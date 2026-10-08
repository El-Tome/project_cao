use std::f64::consts::{PI, TAU};

use glam::{DVec2, DVec3};

use super::*;
use crate::brep::curve::{Circle, Curve, Line};
use crate::brep::relation::{Relation, relation};
use crate::brep::scale::Scale;
use crate::brep::surface::{Cone, Cylinder, Plane, Surface};
use crate::brep::topology::{Body, SurfaceId};
use crate::profile::{Contour, Frame, Run};

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
fn a_body_carried_along_a_move_keeps_its_faces_their_ranks_and_their_numbers() {
    let far = block([100.0, 100.0, 100.0], [110.0, 110.0, 110.0]);
    let moving = block([0.0, 0.0, 0.0], [10.0, 10.0, 10.0]).renumbered(4);
    let wall = moving
        .surfaces
        .iter()
        .position(|surface| *surface == plane_at(10.0, DVec3::X))
        .expect("the block has a wall on x = 10");
    let hair = DVec3::X * 1e-9;
    let moved = carried_along(&far, &moving, scale(), |_, rank| {
        (rank == wall).then_some(hair)
    })
    .expect("the wall moves");
    assert_ne!(moved.surfaces, moving.surfaces);
    assert_eq!(moved.faces, moving.faces);
}

/// A shaft of radius 10 along Y, 30 long, its far end chamfered by 2.
fn chamfered_shaft() -> Body {
    use crate::turning::{Axis, Corner, Straight, Turn};
    let corners = [
        [0.0, 0.0],
        [30.0, 0.0],
        [30.0, 8.0],
        [28.0, 10.0],
        [0.0, 10.0],
    ];
    let straight = Straight {
        side: -1.0,
        contours: vec![
            (0..corners.len())
                .map(|run| Corner {
                    at: DVec2::from(corners[run]),
                    run: run as u32,
                })
                .collect(),
        ],
        runs: corners.len() as u32,
        last_off_the_axis: Some(4),
    };
    let turn = Turn {
        axis: Axis {
            origin: DVec2::ZERO,
            direction: DVec2::Y,
        },
        angle: TAU,
        resolution: 0.0,
        on_the_axis: 0.0,
    };
    Body::turned(&straight, ground(0.0), &turn).expect("the shaft turns")
}

#[test]
fn a_chamfered_shaft_slid_onto_a_bore_keeps_its_cone_on_its_rim() {
    let far = block([100.0, 100.0, 100.0], [110.0, 110.0, 110.0]);
    let shaft = chamfered_shaft();
    let rank_of = |wanted: fn(&Surface) -> bool| {
        shaft
            .surfaces
            .iter()
            .position(wanted)
            .expect("the shaft has the surface")
    };
    let wall = rank_of(|surface| matches!(surface, Surface::Cylinder(_)));
    let chamfer = rank_of(|surface| matches!(surface, Surface::Cone(_)));
    let hair = DVec3::new(0.5e-8, 0.0, 0.3e-8);
    let moved = carried_along(&far, &shaft, scale(), |_, rank| {
        (rank == wall).then_some(hair)
    })
    .expect("the wall moves");
    let [Surface::Cone(before), Surface::Cone(after)] =
        [&shaft, &moved].map(|body| body.surfaces[chamfer])
    else {
        unreachable!()
    };
    for at in [
        DVec2::new(0.0, 0.5),
        DVec2::new(2.0, 1.5),
        DVec2::new(-3.0, 2.5),
    ] {
        let point = before.point(at) + hair;
        assert!(after.distance(point).abs() < 1e-12, "{point} off {after:?}");
    }
    for face in moved.face_ids() {
        let Surface::Cone(cone) = moved.surface(moved.face(face).surface) else {
            continue;
        };
        for coedge in moved.face(face).loops.iter().flatten() {
            let Curve::Circle(rim) = moved.curve(moved.edge(coedge.edge).curve) else {
                panic!("a chamfer bounded by its rims alone");
            };
            assert!(cone.holds(rim, 1e-12), "{rim:?} off {cone:?}");
        }
    }
}

/// A block 30 by 10 by 10, and a slot 20 long and 10 wide standing `hair`
/// across its length from the block's sides, whose half circles reach the
/// block's ends; with the rank in the slot of its far side, along y = 10 +
/// `hair`.
fn a_slot_a_hair_off_a_block() -> (Body, Body, usize, f64) {
    let hair = 1e-8;
    let first = block([0.0, 0.0, 0.0], [30.0, 10.0, 10.0]);
    let [low, high] = [hair, 10.0 + hair];
    let outline = Contour {
        corners: vec![
            DVec2::new(5.0, low),
            DVec2::new(25.0, low),
            DVec2::new(25.0, high),
            DVec2::new(5.0, high),
        ],
        runs: vec![
            Run::Straight,
            Run::Round {
                center: DVec2::new(25.0, 5.0 + hair),
                turn: PI,
            },
            Run::Straight,
            Run::Round {
                center: DVec2::new(5.0, 5.0 + hair),
                turn: PI,
            },
        ],
    };
    let slot = Body::raised(&outline, &[], ground(0.0), DVec3::Z * 10.0)
        .expect("a slot raises")
        .renumbered(6);
    let far = slot
        .surfaces
        .iter()
        .position(|surface| *surface == plane_at(high, DVec3::Y))
        .expect("the slot has a side on y = 10 + hair");
    (first, slot, far, hair)
}

#[test]
fn a_slot_slid_onto_a_block_s_side_brings_its_other_side_onto_the_block_s_other_side() {
    let (block, slot, far, hair) = a_slot_a_hair_off_a_block();
    let moved = carried_along(&block, &slot, scale(), |_, rank| {
        (rank == far).then_some(DVec3::Y * -hair)
    })
    .expect("the slide carries the near side onto the block's");
    let near = moved
        .surfaces
        .iter()
        .find_map(|surface| match surface {
            Surface::Plane(plane) if plane.normal.y.abs() > 0.5 && plane.offset().abs() < 1.0 => {
                Some(plane.offset())
            }
            _ => None,
        })
        .expect("the slot keeps its near side");
    assert!(near.abs() < 1e-12, "the near side stands at y = {near}");
}

#[test]
fn a_slot_slid_away_from_a_block_s_side_is_not_carried_further_off_the_block_s_other_side() {
    let (block, slot, far, hair) = a_slot_a_hair_off_a_block();
    let moved = carried_along(&block, &slot, scale(), |_, rank| {
        (rank == far).then_some(DVec3::Y * hair)
    });
    assert!(moved.is_none(), "the near side was carried off the block's");
}

#[test]
fn a_ruling_through_the_apex_only_touches_the_other_surface() {
    let tip = Surface::Cone(Cone::through(
        DVec3::ZERO,
        DVec3::Z,
        [DVec2::new(0.0, 5.0), DVec2::new(10.0, 0.0)],
    ));
    let holding = Surface::Plane(Plane::through(DVec3::ZERO, DVec3::new(1.0, 1.0, 0.0)).0);
    let square = plane_at(10.0, DVec3::Z);
    let below = plane_at(4.0, DVec3::Z);
    let apart = Apart::of(&[tip, holding, square, below], |_| scale());
    let apex = DVec3::new(0.0, 0.0, 10.0);
    let [cone, holding, square, below] = [0, 1, 2, 3].map(SurfaceId);
    for other in [holding, square] {
        let points = apart.points(cone, other);
        assert_eq!(points.len(), 1, "{points:?}");
        assert!(points[0].distance(apex) < 1e-12, "{points:?}");
        assert!(!apart.pair(cone, other));
    }
    assert!(apart.points(cone, below).is_empty());
}

/// Two cones of one axis and one apex, each the other's mirror: a ruling of
/// one, read as a whole line, is a ruling of the other, but the two meet at
/// the apex alone, and the ruling one of them shares with a plane holding
/// the axis is not the other's.
#[test]
fn a_ruling_of_a_cone_is_not_laid_on_the_same_line_of_its_mirror() {
    let rising = Surface::Cone(Cone::through(
        DVec3::ZERO,
        DVec3::Z,
        [DVec2::new(15.0, 5.0), DVec2::new(20.0, 10.0)],
    ));
    let falling = Surface::Cone(Cone::through(
        DVec3::ZERO,
        DVec3::Z,
        [DVec2::new(0.0, 10.0), DVec2::new(5.0, 5.0)],
    ));
    let holding = Surface::Plane(Plane::through(DVec3::ZERO, DVec3::X).0);
    let list = [holding, rising, falling];
    let mut registry = Registry::new(scale(), Apart::of(&list, |_| scale()), Planes::default());
    let rulings = |rank: usize| match relation(&list[0], &list[rank], scale()) {
        Relation::Rulings { lines, .. } => lines,
        other => panic!("{other:?}"),
    };
    let [plane, rising, falling] = [0, 1, 2].map(SurfaceId);
    let risen = rulings(1).map(|line| registry.register(Curve::Line(line), &[plane, rising]));
    let fallen = rulings(2).map(|line| registry.register(Curve::Line(line), &[plane, falling]));
    assert!(
        risen.iter().all(|rank| !fallen.contains(rank)),
        "{risen:?} {fallen:?}"
    );
}

#[test]
fn a_circle_of_a_cone_is_laid_on_the_same_circle_of_a_cone_of_its_apex_a_hair_apart_in_slope() {
    let one = Surface::Cone(Cone::through(
        DVec3::ZERO,
        DVec3::Z,
        [DVec2::new(5.0, 5.0), DVec2::new(10.0, 0.0)],
    ));
    let other = Surface::Cone(Cone::through(
        DVec3::ZERO,
        DVec3::Z,
        [DVec2::new(5.0, 5.00000002), DVec2::new(10.0, 0.0)],
    ));
    assert!(matches!(relation(&one, &other, scale()), Relation::Apex(_)));
    let level = plane_at(5.0, DVec3::Z);
    let list = [level, one, other];
    let mut registry = Registry::new(scale(), Apart::of(&list, |_| scale()), Planes::default());
    let circle = |rank: usize| match relation(&list[0], &list[rank], scale()) {
        Relation::Circle(circle) => Curve::Circle(circle),
        found => panic!("{found:?}"),
    };
    let [level, one, other] = [0, 1, 2].map(SurfaceId);
    let first = registry.register(circle(1), &[level, one]);
    assert_eq!(registry.register(circle(2), &[level, other]), first);
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
        arrivals: Vec::new(),
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

#[test]
fn a_cone_within_the_tolerance_of_two_of_the_first_operand_s_is_the_nearest() {
    let scale = Scale::of(25.0);
    let along = 0.8 * scale.eps() * 5.0_f64.sqrt();
    let tip = |height: f64| {
        Surface::Cone(Cone::through(
            DVec3::Z * height,
            DVec3::Z,
            [DVec2::new(0.0, 5.0), DVec2::new(10.0, 0.0)],
        ))
    };
    let one = with_surfaces(vec![tip(0.0), tip(along), tip(-along / 2.0)]);
    let other = with_surfaces(vec![tip(along)]);
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

/// Seed 8537287 of the profile draw: a plane of the tool standing where one
/// of the body's three floors a hair apart stands is that floor, though it
/// stands between the two others within the tolerance of each.
#[test]
fn a_surface_of_the_tool_on_one_of_the_body_s_between_two_others_is_that_one() {
    let scale = Scale::of(465.0);
    let one = with_surfaces(vec![
        plane_at(150.0, DVec3::Y),
        plane_at(150.000_000_6, DVec3::Y),
        plane_at(150.000_000_3, DVec3::Y),
    ]);
    let on = with_surfaces(vec![plane_at(150.000_000_3, DVec3::Y)]);
    assert_eq!(
        Surfaces::of(&one, &on, scale).mapped[1],
        vec![(SurfaceId(2), true)]
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
    let apart = Apart::of(&surfaces, |_| scale);
    let [one, other, side, floor] = [0, 1, 2, 3].map(SurfaceId);
    let [tube, bore, beside] = [4, 5, 6].map(SurfaceId);
    assert!(apart.pair(one, other) && apart.pair(other, one));
    assert!(!apart.pair(one, side) && !apart.pair(one, floor));
    assert!(apart.pair(tube, bore) && apart.pair(one, tube));
    assert!(!apart.pair(tube, beside) && !apart.pair(floor, tube));
}

/// Two walls of one radius whose axes stand a hair apart, beyond decision
/// 8's, cross along two rulings at an angle of the hair over the radius:
/// they stand within the tolerance of each other microns from either, as
/// two walls touching do, and the pair is decided so once. Two walls of one
/// radius crossing steeply, or of two radii crossing a hair from touching,
/// are not.
#[test]
fn two_walls_of_one_radius_a_hair_apart_cross_at_a_grazing_angle_and_no_other_pair_does() {
    let scale = Scale::of(55.0);
    let surfaces = [
        Cylinder::about(DVec3::new(0.0, 30.0, 0.0), DVec3::Z, 7.5),
        Cylinder::about(DVec3::new(1e-5, 30.0, 0.0), DVec3::Z, 7.5),
        Cylinder::about(DVec3::new(7.5, 30.0, 0.0), DVec3::Z, 7.5),
        Cylinder::about(DVec3::new(0.0, 39.5 - 1e-5, 0.0), DVec3::Z, 2.0),
    ]
    .map(Surface::Cylinder);
    let apart = Apart::of(&surfaces, |_| scale);
    let [own, hair, steep, near] = [0, 1, 2, 3].map(SurfaceId);
    assert!(apart.graze(own, hair) && apart.graze(hair, own));
    assert!(!apart.touch(own, hair) && !apart.pair(own, hair));
    assert!(!apart.graze(own, steep), "crossing at sixty degrees");
    assert!(
        !apart.graze(own, near),
        "of two radii, a hair from touching"
    );
}

#[test]
fn lines_within_the_tolerance_on_two_surfaces_decided_apart_are_two_curves() {
    let (surfaces, scale) = slit_walls();
    let mut registry = Registry::new(scale, Apart::of(&surfaces, |_| scale), Planes::default());
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
    let mut registry = Registry::new(scale, Apart::of(&surfaces, |_| scale), Planes::default());
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
    let registry = Registry::new(
        scale,
        Apart::of(&surfaces, |_| scale),
        Planes::of(&surfaces),
    );
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
    let along = DVec3::new(104.999_999_7, 240.0, -119.999_999);
    let third = pool.add(along, [front, side], [], &registry);
    assert_eq!((first, second, third), (0, 0, 1));
    assert_eq!(pool.corners[0].point, corner);
}

#[test]
fn a_corner_on_two_planes_across_each_other_stands_on_the_line_they_share_nearest_where_it_was_found()
 {
    let (registry, [bottom, side, _]) = nearly_one_corner();
    let found = DVec3::new(104.999_999_7, 17.0, -119.999_999_7);
    let placed = standing(found, &[bottom, side], &registry, Scale::of(420.0).eps());
    assert!(placed.distance(DVec3::new(105.0, 17.0, -120.0)) < 1e-12);
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
    let mut registry = Registry::new(
        scale,
        Apart::of(&surfaces, |_| scale),
        Planes::of(&surfaces),
    );
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
    let mut registry = Registry::new(
        scale,
        Apart::of(&surfaces, |_| scale),
        Planes::of(&surfaces),
    );
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

/// Seed 1008037 of the campaign: two circles of one radius two hundredths
/// of a micron apart cross where the line between their centres is square
/// to the radius. The half of one on the one side and the half of the other
/// on the other side run between the same two crossings, each within a hair
/// of the other's circle all along, and still part by the diameter: what
/// is measured is how far each stands from the other's arc, not its curve.
#[test]
fn opposite_halves_of_two_circles_a_hair_apart_part_by_the_diameter() {
    let offset = 2e-8;
    let one = Circle::on(&Cylinder::about(DVec3::ZERO, DVec3::Z, 4.5), 0.0);
    let other = Circle::on(&Cylinder::about(DVec3::X * offset, DVec3::Z, 4.5), 0.0);
    let [one, other] = [one, other].map(Curve::Circle);
    let right = [-PI / 2.0, PI / 2.0];
    let left = [PI / 2.0, 3.0 * PI / 2.0];
    assert!(parting(&one, right, &other, left) > 4.0);
    assert!(parting(&one, right, &other, right) < 1e-7);
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
        |_, _| true,
        |_| false,
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

/// A hole touching one wall of a slot exactly and the other a hair off was
/// left where it was, since moved onto the second it would leave the first.
/// Failure 4-1 changed that: a cylinder touching two parallel planes on
/// opposite sides is moved midway between them, its radius half their gap,
/// and touches both exactly.
#[test]
fn a_cylinder_touching_two_parallel_planes_on_opposite_sides_is_moved_midway_and_touches_both() {
    let eps = scale().eps();
    let floor = plane_at(0.0, DVec3::Y);
    let ceiling = plane_at(6.0 + eps / 2.0, DVec3::Y);
    let hole = Cylinder::about(DVec3::new(0.0, 3.0, 0.0), DVec3::Z, 3.0);
    let mut surfaces = Surfaces {
        list: vec![floor, ceiling, Surface::Cylinder(hole)],
        mapped: [Vec::new(), Vec::new()],
    };
    let moved = surfaces.snapped(
        |operand, surface| (operand == 1) == (surface.0 == 2),
        |_, _| true,
        |_| false,
        scale(),
    );
    assert_eq!(moved, [false, false, true]);
    let [first, second, Surface::Cylinder(wall)] = surfaces.list[..] else {
        panic!("the hole stays a cylinder");
    };
    assert_eq!([first, second], [floor, ceiling]);
    assert_eq!(wall.radius, (3.0 + eps / 4.0));
    assert!((wall.origin.y - (3.0 + eps / 4.0)).abs() < 1e-15);
}

/// A hole, a slot's floor and a ceiling `ceiling` past the hole's top, the
/// hole the second operand's, as decision 2 leaves them.
fn hole_under(ceiling: f64) -> Surface {
    let floor = plane_at(0.0, DVec3::Y);
    let ceiling = plane_at(6.0 + ceiling, DVec3::Y);
    let hole = Cylinder::about(DVec3::new(0.0, 3.0, 0.0), DVec3::Z, 3.0);
    let mut surfaces = Surfaces {
        list: vec![floor, ceiling, Surface::Cylinder(hole)],
        mapped: [Vec::new(), Vec::new()],
    };
    surfaces.snapped(
        |operand, surface| (operand == 1) == (surface.0 == 2),
        |_, _| true,
        |_| false,
        scale(),
    );
    surfaces.list[2]
}

/// Seed 8554448 of the profile draw: a hole touching a slot's floor within
/// the tolerance and its ceiling a few tolerances off, past it. Left
/// touching the floor alone, it stood a hair under the ceiling, a skin no
/// ray tells the side of: it is moved midway as it would be within the
/// tolerance of both.
#[test]
fn a_cylinder_touching_one_of_two_parallel_planes_a_hair_from_the_other_is_moved_midway() {
    let eps = scale().eps();
    let Surface::Cylinder(wall) = hole_under(3.0 * eps) else {
        unreachable!("the hole stays a cylinder");
    };
    assert!((wall.radius - (3.0 + 1.5 * eps)).abs() < 1e-14);
    assert!((wall.origin.y - (3.0 + 1.5 * eps)).abs() < 1e-14);
}

/// Further than decision 8's hair from the ceiling, it is left touching the
/// floor.
#[test]
fn a_cylinder_touching_one_of_two_parallel_planes_far_from_the_other_keeps_its_radius() {
    let hole = Cylinder::about(DVec3::new(0.0, 3.0, 0.0), DVec3::Z, 3.0);
    assert_eq!(hole_under(30.0 * scale().eps()), Surface::Cylinder(hole));
}

/// A hole a hair from touching a plane whose faces stand far from its own
/// is not moved: two surfaces touch only where faces of theirs meet.
#[test]
fn a_cylinder_a_hair_from_a_plane_whose_faces_stand_far_is_not_moved() {
    let top = plane_at(10.0, DVec3::Z);
    let hole = Cylinder::about(
        DVec3::new(0.0, 5.0, 7.0),
        DVec3::X,
        3.0 + scale().eps() / 2.0,
    );
    let mut surfaces = Surfaces {
        list: vec![top, Surface::Cylinder(hole)],
        mapped: [Vec::new(), Vec::new()],
    };
    surfaces.snapped(
        |operand, surface| operand == surface.0 as usize,
        |_, _| false,
        |_| false,
        scale(),
    );
    assert_eq!(surfaces.list, [top, Surface::Cylinder(hole)]);
}

/// A hole touching a plane exactly is moved onto a second touch only along
/// the plane: a bar touching a floor and a ceiling exactly is moved along
/// them onto a post it touches inside a hair off, and a cylinder touching a
/// side exactly is not moved across it onto a parallel wall a hair off.
#[test]
fn a_cylinder_touching_a_plane_exactly_moves_onto_another_touch_only_along_the_plane() {
    let eps = scale().eps();
    let floor = plane_at(0.0, DVec3::Z);
    let bar = Cylinder::about(DVec3::new(0.0, 15.0 - eps / 2.0, 3.0), DVec3::X, 3.0);
    let post = Cylinder::about(DVec3::ZERO, DVec3::Z, 18.0);
    let mut along = Surfaces {
        list: vec![floor, Surface::Cylinder(post), Surface::Cylinder(bar)],
        mapped: [Vec::new(), Vec::new()],
    };
    along.snapped(
        |operand, surface| (operand == 1) == (surface.0 == 2),
        |_, _| true,
        |_| false,
        scale(),
    );
    let Surface::Cylinder(moved) = along.list[2] else {
        panic!("the bar stays a cylinder");
    };
    assert!((moved.origin.y - 15.0).abs() < 1e-14 && moved.origin.z == 3.0);

    let side = plane_at(0.0, DVec3::X);
    let wall = Cylinder::about(DVec3::new(3.0, 0.0, 0.0), DVec3::Z, 3.0);
    let stock = Cylinder::about(DVec3::new(6.0 + eps / 2.0, 0.0, 0.0), DVec3::Z, 6.0);
    let mut across = Surfaces {
        list: vec![side, Surface::Cylinder(stock), Surface::Cylinder(wall)],
        mapped: [Vec::new(), Vec::new()],
    };
    across.snapped(
        |operand, surface| (operand == 1) == (surface.0 == 2),
        |_, _| true,
        |_| false,
        scale(),
    );
    assert_eq!(across.list[2], Surface::Cylinder(wall));
}

/// A cylinder touching two parallel planes on one side, a hair into the
/// nearer and a hair off the further, is not moved between them: there is
/// no between, and half their gap would be no radius. Moved onto either
/// touch, it would leave the other, so it stays where it was drawn.
#[test]
fn a_cylinder_touching_two_parallel_planes_on_one_side_keeps_its_radius() {
    let eps = scale().eps();
    let low = plane_at(0.0, DVec3::Y);
    let high = plane_at(1.6 * eps, DVec3::Y);
    let post = Cylinder::about(DVec3::new(0.0, 3.0 + 0.8 * eps, 0.0), DVec3::Z, 3.0);
    let mut surfaces = Surfaces {
        list: vec![low, high, Surface::Cylinder(post)],
        mapped: [Vec::new(), Vec::new()],
    };
    let moved = surfaces.snapped(
        |operand, surface| (operand == 1) == (surface.0 == 2),
        |_, _| true,
        |_| false,
        scale(),
    );
    assert_eq!(moved, [false, false, false]);
    assert_eq!(surfaces.list, [low, high, Surface::Cylinder(post)]);
}

/// A rectangle from `low` to `high` with its corners rounded to `radius`,
/// raised from the ground by `height`.
fn rounded(low: [f64; 2], high: [f64; 2], radius: f64, height: f64) -> Body {
    let [low, high] = [DVec2::from(low), DVec2::from(high)];
    let corner = |x: f64, y: f64| DVec2::new(x, y);
    let round = |x: f64, y: f64| Run::Round {
        center: corner(x, y),
        turn: PI / 2.0,
    };
    let outline = Contour {
        corners: vec![
            corner(low.x + radius, low.y),
            corner(high.x - radius, low.y),
            corner(high.x, low.y + radius),
            corner(high.x, high.y - radius),
            corner(high.x - radius, high.y),
            corner(low.x + radius, high.y),
            corner(low.x, high.y - radius),
            corner(low.x, low.y + radius),
        ],
        runs: vec![
            Run::Straight,
            round(high.x - radius, low.y + radius),
            Run::Straight,
            round(high.x - radius, high.y - radius),
            Run::Straight,
            round(low.x + radius, high.y - radius),
            Run::Straight,
            round(low.x + radius, low.y + radius),
        ],
    };
    Body::raised(&outline, &[], ground(0.0), DVec3::Z * height).expect("a profile raises")
}

/// The offsets of the planes of `body` square to X.
fn sides(body: &Body) -> Vec<f64> {
    body.surfaces
        .iter()
        .filter_map(|surface| match surface {
            Surface::Plane(plane) if plane.normal.x.abs() > 0.5 => Some(plane.offset()),
            _ => None,
        })
        .collect()
}

/// Seed 8540801 of the profile draw: a rounded rectangle drawn a hair and a
/// half beside a block, its sides beyond the tolerance of the block's, its
/// corners' walls touching its own sides and crossing the block's at a
/// grazing angle. Decision 10 takes its sides for the block's, and moves
/// what it built on them along.
#[test]
fn a_rounded_rectangle_a_hair_beside_a_block_is_moved_flush_with_it() {
    let block = block([1.0, 3.0, 0.0], [8.0, 9.0, 5.0]);
    let scale = Scale::of(9.0);
    let hair = 1.5 * scale.eps();
    let beside = rounded([1.0 - hair, 3.0], [8.0 - hair, 8.5], 1.0, 5.0);
    let moved = flush(&block, &beside, scale).expect("the rectangle is moved");
    for offset in sides(&moved) {
        assert!(
            (offset - 1.0).abs() < 1e-12 || (offset - 8.0).abs() < 1e-12,
            "a side at {offset}"
        );
    }
    for surface in &moved.surfaces {
        if let Surface::Cylinder(wall) = surface {
            let off = (wall.origin.x - 2.0).abs().min((wall.origin.x - 7.0).abs());
            assert!(off < 1e-12, "a wall about {:?}", wall.origin);
        }
    }
}

/// Two blocks whose sides stand a hair apart, no wall touching either, are
/// left two: the skin between them is well defined, and case 8 keeps its
/// floor of a ten-millionth.
#[test]
fn a_block_a_hair_beside_another_with_no_wall_touching_its_side_is_left_where_it_is() {
    let block = block([1.0, 3.0, 0.0], [8.0, 9.0, 5.0]);
    let scale = Scale::of(9.0);
    let hair = 1.5 * scale.eps();
    let other = Body::raised(
        &Contour::rectangle(DVec2::new(1.0 - hair, 3.0), DVec2::new(8.0 - hair, 8.5)),
        &[],
        ground(0.0),
        DVec3::Z * 5.0,
    )
    .expect("a block raises");
    assert!(flush(&block, &other, scale).is_none());
}
