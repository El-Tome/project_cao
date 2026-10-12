use glam::{DVec2, DVec3};

use super::*;
use crate::brep::curve::{Circle, Line};
use crate::brep::scale::Scale;

const CLOSE: f64 = 1e-12;

#[test]
fn a_plane_reached_from_either_side_is_the_same_plane_turned_round() {
    let (up, turned_up) = Plane::through(DVec3::new(3.0, -2.0, 10.0), DVec3::Z);
    let (down, turned_down) = Plane::through(DVec3::new(-7.0, 5.0, 10.0), -DVec3::Z);
    assert_eq!(up, down);
    assert!(!turned_up && turned_down);
    assert_eq!(up.origin, DVec3::new(0.0, 0.0, 10.0));
}

#[test]
fn a_plane_tilted_by_a_hair_keeps_the_sign_of_the_plane_it_nearly_is() {
    let (plane, turned) = Plane::through(DVec3::ZERO, DVec3::new(-1e-9, 0.0, 1.0));
    assert!(!turned);
    assert!(plane.normal.z > 0.0);
}

#[test]
fn a_point_of_a_plane_comes_back_from_its_parameters() {
    let (plane, _) = Plane::through(DVec3::new(1.0, 2.0, 3.0), DVec3::new(1.0, 2.0, 2.0));
    let at = DVec2::new(4.5, -7.25);
    let back = plane.parameters(plane.point(at));
    assert!((back - at).length() < CLOSE, "{back}");
    assert!(plane.distance(plane.point(at)).abs() < CLOSE);
}

#[test]
fn parallel_cylinders_start_their_angles_from_the_same_direction() {
    let one = Cylinder::about(DVec3::new(8.0, 0.0, 3.0), DVec3::Z, 5.0);
    let other = Cylinder::about(DVec3::new(-2.0, 7.0, -1.0), -DVec3::Z, 20.0);
    assert_eq!(one.u, other.u);
    assert_eq!(one.v, other.v);
    assert_eq!(one.axis, other.axis);
    assert_eq!(one.origin, DVec3::new(8.0, 0.0, 0.0));
}

#[test]
fn the_angles_a_plane_of_the_origin_touches_a_cylinder_at_are_quarter_turns() {
    for axis in [DVec3::X, DVec3::Y, DVec3::Z] {
        let cylinder = Cylinder::about(DVec3::ZERO, axis, 1.0);
        for quarter in 0..4 {
            let radial = cylinder.radial(f64::from(quarter) * std::f64::consts::FRAC_PI_2);
            let along_an_axis = DVec3::AXES
                .iter()
                .any(|world| (radial.abs() - *world).length() < CLOSE);
            assert!(along_an_axis, "{axis} at quarter {quarter}: {radial}");
        }
    }
}

#[test]
fn a_point_of_a_cylinder_comes_back_from_its_parameters() {
    let cylinder = Cylinder::about(DVec3::new(1.0, -3.0, 2.0), DVec3::new(0.0, 1.0, 1.0), 4.0);
    for at in [
        DVec2::new(0.3, 2.0),
        DVec2::new(-2.9, -5.0),
        DVec2::new(3.1, 0.0),
    ] {
        let point = cylinder.point(at);
        assert!(cylinder.distance(point).abs() < CLOSE);
        let back = cylinder.parameters(point);
        assert!((back - at).length() < CLOSE, "{at} came back as {back}");
    }
}

/// The cone the run from `(0, 4)` to `(6, 10)` turns into about the line
/// through `(1, -2, 3)` along a slanted axis.
fn widening() -> Cone {
    Cone::through(
        DVec3::new(1.0, -2.0, 3.0),
        DVec3::new(0.0, 1.0, 1.0),
        [DVec2::new(0.0, 4.0), DVec2::new(6.0, 10.0)],
    )
}

/// A cone narrowing along its axis, its apex at `h = 12` from the origin.
fn narrowing() -> Cone {
    Cone::through(
        DVec3::ZERO,
        DVec3::Z,
        [DVec2::new(0.0, 6.0), DVec2::new(8.0, 2.0)],
    )
}

#[test]
fn a_cone_turned_from_one_run_either_way_is_the_same_bits() {
    let point = DVec3::new(3.0, 7.0, -2.0);
    let axis = DVec3::new(0.3, -1.0, 0.2);
    let [from, to] = [DVec2::new(1.5, 2.0), DVec2::new(4.25, 9.0)];
    let mirrored = |corner: DVec2| DVec2::new(-corner.x, corner.y);
    let forwards = Cone::through(point, axis, [from, to]);
    let backwards = Cone::through(point, -axis, [mirrored(from), mirrored(to)]);
    assert_eq!(forwards, backwards);
}

#[test]
fn a_cone_read_from_its_corners_in_either_order_is_the_same_bits() {
    let point = DVec3::new(-4.0, 1.0, 9.0);
    let [from, to] = [DVec2::new(10.0, 3.0), DVec2::new(2.0, 7.5)];
    assert_eq!(
        Cone::through(point, DVec3::X, [from, to]),
        Cone::through(point, DVec3::X, [to, from])
    );
}

#[test]
fn a_cone_shares_origin_and_reference_with_a_cylinder_of_its_axis() {
    let point = DVec3::new(2.0, -5.0, 1.0);
    let axis = DVec3::new(1.0, 2.0, -0.5);
    let cone = Cone::through(point, axis, [DVec2::new(0.0, 1.0), DVec2::new(3.0, 4.0)]);
    let cylinder = Cylinder::about(point, axis, 4.0);
    assert_eq!(
        [cone.origin, cone.axis, cone.u, cone.v],
        [cylinder.origin, cylinder.axis, cylinder.u, cylinder.v]
    );
    assert_eq!(cone.circle(3.0, 4.0), Circle::on(&cylinder, 3.0));
}

#[test]
fn a_point_of_a_cone_reads_back_its_parameters() {
    for cone in [widening(), narrowing()] {
        for at in [
            DVec2::new(0.3, 2.0),
            DVec2::new(-2.9, 5.0),
            DVec2::new(3.1, 0.5),
        ] {
            let point = cone.point(at);
            assert!(cone.distance(point).abs() < CLOSE, "{at}");
            let back = cone.parameters(point);
            assert!((back - at).length() < CLOSE, "{at} came back as {back}");
        }
    }
}

#[test]
fn a_steep_cone_and_a_near_cylinder_cone_keep_their_points_on_themselves() {
    let steep = Cone::through(
        DVec3::ZERO,
        DVec3::Y,
        [DVec2::new(5.0, 1.0), DVec2::new(5.0 + 1e-7, 11.0)],
    );
    let slender = Cone::through(
        DVec3::ZERO,
        DVec3::Y,
        [DVec2::new(0.0, 5.0), DVec2::new(10.0, 5.0 + 1e-7)],
    );
    for (cone, along) in [(steep, [1.0, 11.0]), (slender, [0.0, 10.0])] {
        for step in 0..=10 {
            let radius = along[0] + (along[1] - along[0]) * f64::from(step) / 10.0;
            for theta in [-3.0, -1.0, 0.5, 2.5] {
                let point = if cone == steep {
                    let l = (radius - cone.foot.y) / cone.ruling.y;
                    cone.point(DVec2::new(theta, l))
                } else {
                    cone.point(DVec2::new(theta, radius))
                };
                assert!(
                    cone.distance(point).abs() <= 1e-12 * 11.0,
                    "{} off at {theta}, {radius}",
                    cone.distance(point)
                );
                let at = cone.parameters(point);
                assert!(
                    cone.point(at).distance(point) <= 1e-12 * 11.0,
                    "{point} came back as {}",
                    cone.point(at)
                );
            }
        }
    }
}

#[test]
fn a_cone_s_parameters_turn_as_its_own_normal() {
    let step = 1e-6;
    for cone in [widening(), narrowing()] {
        for at in [DVec2::new(0.4, 3.0), DVec2::new(-2.0, 1.0)] {
            let here = cone.point(at);
            let round = cone.point(at + DVec2::new(step, 0.0)) - here;
            let along = cone.point(at + DVec2::new(0.0, step)) - here;
            let turned = round.cross(along).normalize();
            assert!(
                turned.distance(cone.normal(at.x)) < 1e-6,
                "{turned} against {}",
                cone.normal(at.x)
            );
        }
    }
}

#[test]
fn a_point_behind_the_apex_stands_its_distance_from_the_apex() {
    let cone = narrowing();
    let apex = cone.apex();
    assert!(apex.distance(DVec3::new(0.0, 0.0, 12.0)) < CLOSE, "{apex}");
    for point in [
        DVec3::new(0.0, 0.0, 15.0),
        DVec3::new(0.5, -0.2, 13.0),
        DVec3::new(1.0, 1.0, 14.0),
    ] {
        let distance = cone.distance(point);
        assert!(
            (distance.abs() - point.distance(apex)).abs() < CLOSE,
            "{distance} at {point}"
        );
    }
}

#[test]
fn a_point_of_the_other_nappe_stands_off_the_cone() {
    for cone in [widening(), narrowing()] {
        let apex = cone.apex();
        let [w_h, w_rho] = cone.ruling.to_array();
        for length in [1.0, 5.0] {
            let point = apex - cone.axis * w_h * length + cone.radial(0.7) * w_rho * length;
            let expected = if w_rho >= w_h.abs() {
                2.0 * length * (w_h * w_rho).abs()
            } else {
                length
            };
            let distance = cone.distance(point).abs();
            assert!(
                (distance - expected).abs() < 1e-9,
                "{distance} against {expected}"
            );
        }
    }
}

#[test]
fn a_ruling_is_told_from_a_line_across_the_cone() {
    let scale = Scale::of(20.0);
    let slender = Cone::through(
        DVec3::ZERO,
        DVec3::Z,
        [DVec2::new(0.0, 5.0), DVec2::new(10.0, 5.0 + 1e-6)],
    );
    for cone in [widening(), narrowing(), slender] {
        let theta = 0.9;
        let foot = cone.point(DVec2::new(theta, 1.0));
        let along = cone.point(DVec2::new(theta, 2.0)) - foot;
        assert!(cone.rules(&Line::through(foot, along), scale));
        let across = cone.radial(theta).cross(cone.axis);
        assert!(!cone.rules(&Line::through(foot, across), scale));
        let leaning = along + across * 1e-6;
        assert!(!cone.rules(&Line::through(foot, leaning), scale));
        let beside = foot + cone.normal(theta) * 20.0 * scale.eps();
        assert!(!cone.rules(&Line::through(beside, along), scale));
    }
}

#[test]
fn a_cone_s_roots_lie_on_its_own_nappe_only() {
    let cone = narrowing();
    let up = cone.roots(DVec3::new(3.0, 0.0, -5.0), DVec3::Z);
    let [Some(t), None] = up else {
        panic!("a line along the axis off it crosses one nappe once: {up:?}");
    };
    let at = DVec3::new(3.0, 0.0, -5.0 + t);
    assert!(cone.distance(at).abs() < CLOSE, "{at}");
    assert!(at.z < 12.0, "{at} is on the other nappe");

    let origin = DVec3::new(-10.0, 1.0, 4.0);
    let direction = DVec3::new(1.0, 0.0, 0.1).normalize();
    let both = cone.roots(origin, direction);
    let [Some(first), Some(second)] = both else {
        panic!("a line across crosses twice: {both:?}");
    };
    assert!(first < second);
    for t in [first, second] {
        assert!(cone.distance(origin + direction * t).abs() < CLOSE);
    }
}

#[test]
fn a_circle_about_the_axis_on_the_nappe_is_held_and_one_off_it_is_not() {
    let eps = 1e-8;
    for cone in [widening(), narrowing()] {
        let l = 2.5;
        let (h, radius) = (cone.height_at(l), cone.radius_at(l));
        assert!(cone.holds(&cone.circle(h, radius), eps));
        assert!((cone.section(h) - radius).abs() < CLOSE);
        assert!(!cone.holds(&cone.circle(h, radius + 10.0 * eps), eps));
        let mut leaning = cone.circle(h, radius);
        leaning.axis = (leaning.axis + leaning.u * 1e-6).normalize();
        assert!(!cone.holds(&leaning, eps));
    }
}

#[test]
fn a_line_crossing_a_cone_a_hair_from_a_disc_is_found_on_it_to_rounding() {
    let cone = Cone::through(
        DVec3::ZERO,
        DVec3::Y,
        [DVec2::new(2.0, 4.5), DVec2::new(2.0000002, 1.5)],
    );
    let origin = DVec3::new(-7.1351913357409575, -2.4514847956860875, -17.65520603895878);
    let direction = DVec3::new(0.2959199195828451, 0.21946291946926036, 0.9296598454123485);
    let roots: Vec<f64> = cone
        .roots(origin, direction)
        .into_iter()
        .flatten()
        .collect();
    assert_eq!(roots.len(), 1, "{roots:?}");
    let off = cone.distance(origin + direction * roots[0]);
    assert!(off.abs() < 1e-14, "the crossing stands {off} off the cone");
}

#[test]
fn a_line_crossing_a_cone_a_hair_from_a_disc_whose_two_roots_rounding_merges_is_found_on_it() {
    let cone = Cone::through(
        DVec3::new(4.0, 0.0, 0.0),
        DVec3::Z,
        [DVec2::new(0.5, 4.5), DVec2::new(0.5000002, 2.0)],
    );
    let direction = DVec3::new(0.2959199195828451, 0.21946291946926036, 0.9296598454123485);
    for origin in [
        DVec3::new(-3.4953630840525207, -3.80127111251585, -16.239702096428253),
        DVec3::new(
            -1.3705863609991313,
            -0.9192521076683491,
            -17.596391930899653,
        ),
    ] {
        let roots: Vec<f64> = cone
            .roots(origin, direction)
            .into_iter()
            .flatten()
            .collect();
        assert_eq!(roots.len(), 1, "{origin}: {roots:?}");
        let off = cone.distance(origin + direction * roots[0]);
        assert!(
            off.abs() < 1e-14,
            "{origin}: the crossing stands {off} off the cone"
        );
    }
}

#[test]
fn a_line_passing_a_cone_by_a_hair_or_crossing_its_other_nappe_never_crosses_it() {
    let cone = narrowing();
    for origin in [
        DVec3::new(-30.0, 4.0 + 1e-9, 4.0),
        DVec3::new(-30.0, 4.0 + 1e-12, 4.0),
        DVec3::new(-30.0, 0.5, 14.0),
    ] {
        assert_eq!(cone.roots(origin, DVec3::X), [None, None], "{origin}");
    }
}
