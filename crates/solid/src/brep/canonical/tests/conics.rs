//! The conic a plane cuts a wall along at a slant (#552) in decisions 3, 5
//! and 6: one curve however it was built from surfaces within the tolerance
//! of each other, read over a stretch up the registered one, carried along a
//! move, standing for the pair it lies on when two corners would be merged,
//! and measured against a line where it parts from it most.

use super::*;
use crate::brep::conic::{Arm, Conic, Round};
use crate::brep::tessellation::tests::fixtures;

fn slanted(through: DVec3, lean: f64) -> Plane {
    Plane::through(through, DVec3::new(lean.sin(), 0.0, lean.cos())).0
}

fn shaft(origin: DVec3, radius: f64) -> Round {
    Round::Cylinder(Cylinder::about(origin, DVec3::Z, radius))
}

/// A point about the Z axis through `origin`, its tip at height 10 and its
/// base of radius `radius` at height 0.
fn point(origin: DVec3, radius: f64) -> Round {
    Round::Cone(Cone::through(
        origin,
        DVec3::Z,
        [DVec2::new(0.0, radius), DVec2::new(10.0, 0.0)],
    ))
}

/// A cone whose meridian line is turned about its foot by `angle`: no
/// further from the origin of its half-plane, but across the box a slope
/// apart.
fn turned_about_its_foot(round: Round, angle: f64) -> Round {
    let Round::Cone(cone) = round else {
        panic!("a cone is turned");
    };
    Round::Cone(Cone {
        ruling: DVec2::from_angle(angle).rotate(cone.ruling),
        ..cone
    })
}

/// The one conic a plane cuts a wall along.
fn one(plane: &Plane, round: &Round) -> Curve {
    let [conic] = Conic::cut(plane, round, scale())[..] else {
        panic!("one conic expected of {plane:?} on {round:?}");
    };
    Curve::Conic(conic)
}

#[test]
fn two_conics_of_one_plane_and_one_wall_are_one_curve() {
    let eps = scale().eps();
    let middle = DVec3::new(0.0, 0.0, 4.0);
    let hair = DVec3::new(eps / 3.0, -eps / 4.0, 0.0);
    for (round, beside, slope) in [
        (
            shaft(DVec3::ZERO, 5.0),
            shaft(hair, 5.0 + eps / 3.0),
            shaft(DVec3::ZERO, 5.0 + 2.0 * eps),
        ),
        (
            point(DVec3::ZERO, 5.0),
            point(hair, 5.0),
            turned_about_its_foot(point(DVec3::ZERO, 5.0), 1e-6),
        ),
    ] {
        let curve = one(&slanted(middle, 0.4), &round);
        let near = one(&slanted(middle + DVec3::Z * (eps / 2.0), 0.4), &beside);
        assert_ne!(curve, near);
        assert!(same(&curve, &near, scale()), "{curve:?} and {near:?}");
        assert!(same(&near, &curve, scale()));
        for other in [
            one(&slanted(middle + DVec3::Z * (3.0 * eps), 0.4), &round),
            one(&slanted(middle, 0.4 + 1e-6), &round),
            one(&slanted(middle, 0.4), &slope),
        ] {
            assert!(!same(&curve, &other, scale()), "{other:?}");
        }
    }
    let ring = Curve::Circle(Circle::on(
        &Cylinder::about(DVec3::ZERO, DVec3::Z, 5.0),
        4.0,
    ));
    let level = one(
        &Plane::through(middle, DVec3::new(1e-6, 0.0, 1.0)).0,
        &shaft(DVec3::ZERO, 5.0),
    );
    assert!(!same(&level, &ring, scale()) && !same(&ring, &level, scale()));
}

#[test]
fn a_conic_and_its_arm_are_not_one_curve() {
    let near_the_axis = Plane::through(DVec3::new(3.0, 0.0, 0.0), DVec3::new(1.0, 0.0, 1e-6)).0;
    let wall = shaft(DVec3::ZERO, 5.0);
    let [whole] = Conic::cut(&near_the_axis, &wall, Scale::of(1e3))[..] else {
        panic!("one ellipse within ten thousand reaches");
    };
    assert_eq!(whole.arm, Arm::Whole);
    let arms = Conic::cut(&near_the_axis, &wall, scale());
    assert_eq!(arms.len(), 2, "{arms:?}");
    for arm in &arms {
        assert!(matches!(arm.arm, Arm::Split { .. }), "{arm:?}");
        assert!(same(&Curve::Conic(*arm), &Curve::Conic(*arm), scale()));
        assert!(!same(&Curve::Conic(*arm), &Curve::Conic(whole), scale()));
    }
    assert!(!same(
        &Curve::Conic(arms[0]),
        &Curve::Conic(arms[1]),
        scale()
    ));
}

#[test]
fn a_conic_s_stretch_is_read_on_the_registered_conic_round_its_period() {
    let eps = scale().eps();
    let wall = shaft(DVec3::new(2.0, 1.0, 0.0), 5.0);
    let registered = one(&slanted(DVec3::new(0.0, 0.0, 4.0), 0.5), &wall);
    let own = one(&slanted(DVec3::new(0.0, 0.0, 4.0 + eps / 4.0), 0.5), &wall);
    let period = registered.period().expect("an ellipse closes");
    let mut registry = Registry::new(scale(), Apart::default(), Planes::default());
    let rank = registry.register(registered, &[]);
    for [from, to] in [[0.4, 0.6], [-0.1, 0.3], [0.45, 1.3]].map(|ends| ends.map(|at| at * period))
    {
        let [start, end] = registry.stretch(rank, &own, from, to);
        assert!(
            ((end - start) - (to - from)).abs() <= 1e-9 * period,
            "{from}..{to} read as {start}..{end}"
        );
        for (mine, theirs) in [(from, start), (to, end)] {
            let off = own.point(mine).distance(registered.point(theirs));
            assert!(off <= eps, "{mine} read at {theirs}, {off} away");
        }
    }
}

#[test]
fn a_conic_moved_with_its_operand_is_built_again_on_the_moved_surfaces() {
    let cut = fixtures::stock_cut_at_a_slant(0.3);
    let far = block([100.0, 100.0, 100.0], [110.0, 110.0, 110.0]);
    let wall = cut
        .surfaces
        .iter()
        .position(|surface| matches!(surface, Surface::Cylinder(_)))
        .expect("the stock has a wall");
    let hair = DVec3::X * 1e-8;
    let moved = carried_along(&far, &cut, cut.scale(), |_, rank| {
        (rank == wall).then_some(hair)
    })
    .expect("the wall moves with the slant it carries");
    let Surface::Cylinder(cylinder) = moved.surfaces[wall] else {
        unreachable!()
    };
    let (rank, Curve::Conic(conic)) = moved
        .curves
        .iter()
        .enumerate()
        .find_map(|(rank, curve)| matches!(curve, Curve::Conic(_)).then_some((rank, *curve)))
        .expect("the slant's ellipse")
    else {
        unreachable!()
    };
    let Curve::Conic(before) = cut.curves[rank] else {
        unreachable!()
    };
    let plane = moved
        .surfaces
        .iter()
        .find_map(|surface| match surface {
            Surface::Plane(plane) if *plane == conic.plane => Some(*plane),
            _ => None,
        })
        .expect("the conic's plane is one of the body's");
    assert_ne!(plane, before.plane, "the slant moves with the wall");
    assert_eq!(conic.round, Round::Cylinder(cylinder));
    let rebuilt = Conic::cut(&plane, &Round::Cylinder(cylinder), cut.scale());
    assert_eq!(rebuilt, vec![conic]);
}

#[test]
fn a_merge_putting_a_corner_on_a_plane_and_a_wall_far_from_their_conic_is_refused() {
    let eps = scale().eps();
    let plane = slanted(DVec3::new(0.0, 0.0, 4.0), 0.3);
    let curve = one(&plane, &shaft(DVec3::ZERO, 5.0));
    let mut registry = Registry::new(scale(), Apart::default(), Planes::default());
    let [on_the_plane, on_the_wall] = [SurfaceId(0), SurfaceId(1)];
    registry.register(curve, &[on_the_plane, on_the_wall]);
    let far = DVec3::new(0.0, 0.0, 4.0);
    let near = curve.point(0.7);
    for (place, merged) in [(far, false), (near, true)] {
        let mut pool = Pool::new(eps);
        pool.add(place, [on_the_plane], [], &registry);
        let second = pool.add(place + DVec3::Y * (eps / 2.0), [on_the_wall], [], &registry);
        assert_eq!(second == 0, merged, "{place}");
    }
}

#[test]
fn an_ellipse_arc_is_parted_from_its_chord_where_it_parts_most() {
    let ellipse = one(
        &slanted(DVec3::new(0.0, 0.0, 4.0), 0.6),
        &shaft(DVec3::new(1.0, 0.0, 0.0), 5.0),
    );
    let [from, to] = [0.13, 0.71];
    let [start, end] = [from, to].map(|at| ellipse.point(at));
    let chord = Curve::Line(Line::through(start, end - start));
    let along = [start, end].map(|point| chord.parameter(point));
    let along = [along[0].min(along[1]), along[0].max(along[1])];
    let sag = |at: f64| distance(&chord, ellipse.point(at));
    let [mut low, mut high] = [from, to];
    for _ in 0..200 {
        let [left, right] = [low + (high - low) / 3.0, high - (high - low) / 3.0];
        if sag(left) < sag(right) {
            low = left;
        } else {
            high = right;
        }
    }
    let most = sag((low + high) / 2.0);
    let found = parting(&ellipse, [from, to], &chord, along);
    assert!(
        (found - most).abs() <= 1e-9 * most,
        "{found} measured, {most} at most"
    );
}
