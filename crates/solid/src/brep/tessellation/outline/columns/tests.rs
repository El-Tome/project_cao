use glam::{DVec2, DVec3};

use super::unrolled;
use crate::brep::surface::Cone;

const TOLERANCE: f64 = 1e-12;

/// The ruling of the column the cone is rolled out about.
const MIDDLE: f64 = 0.7;

/// A cone about Z opening from radius 1 at the foot to 3 at 4 above, its
/// ruling at `w_ρ = 1/√5`: far enough from 1 and from ½ that a factor of
/// it, or of two, dropped or doubled, shows.
fn cone() -> Cone {
    Cone::through(
        DVec3::ZERO,
        DVec3::Z,
        [DVec2::new(0.0, 1.0), DVec2::new(4.0, 3.0)],
    )
}

/// How far a length `l` along the ruling stands from the apex.
fn from_the_apex(cone: &Cone, l: f64) -> f64 {
    cone.radius_at(l) / cone.ruling.y
}

#[test]
fn a_point_on_the_middle_ruling_is_rolled_out_onto_the_second_axis_at_its_own_length() {
    let cone = cone();
    for l in [-0.5, 0.0, 1.0, 3.5] {
        let rolled = unrolled(&cone, DVec2::new(MIDDLE, l), MIDDLE);
        assert!(
            rolled.distance(DVec2::new(0.0, l)) < TOLERANCE,
            "l = {l}: rolled out to {rolled}"
        );
    }
}

#[test]
fn the_apex_is_rolled_out_onto_the_second_axis_from_every_ruling() {
    let cone = cone();
    let apex = DVec2::new(0.0, cone.apex_at());
    for theta in [MIDDLE - 1.2, MIDDLE - 0.3, MIDDLE + 0.4, MIDDLE + 1.5] {
        let rolled = unrolled(&cone, DVec2::new(theta, cone.apex_at()), MIDDLE);
        assert!(
            rolled.distance(apex) < TOLERANCE,
            "θ = {theta}: the apex rolled out to {rolled}, not {apex}"
        );
    }
}

#[test]
fn a_ruling_rolled_out_keeps_every_length_along_it() {
    let cone = cone();
    let apex = DVec2::new(0.0, cone.apex_at());
    for theta in [MIDDLE - 1.2, MIDDLE - 0.3, MIDDLE + 0.4, MIDDLE + 1.5] {
        let (near, far) = (0.5, 3.0);
        let [one, other] = [near, far].map(|l| unrolled(&cone, DVec2::new(theta, l), MIDDLE));
        assert!(
            (one.distance(other) - (far - near)).abs() < TOLERANCE,
            "θ = {theta}: {near} and {far} along the ruling rolled out {} apart",
            one.distance(other)
        );
        for (rolled, l) in [(one, near), (other, far)] {
            assert!(
                (rolled.distance(apex) - from_the_apex(&cone, l)).abs() < TOLERANCE,
                "θ = {theta}, l = {l}: rolled out {} from the apex, not {}",
                rolled.distance(apex),
                from_the_apex(&cone, l)
            );
        }
    }
}

#[test]
fn two_points_at_one_length_either_side_of_the_middle_ruling_stand_their_chord_apart() {
    let cone = cone();
    for l in [0.0, 2.5] {
        for off in [0.05, 0.6, 1.4] {
            let [left, right] = [MIDDLE - off, MIDDLE + off]
                .map(|theta| unrolled(&cone, DVec2::new(theta, l), MIDDLE));
            let chord = 2.0 * from_the_apex(&cone, l) * (cone.ruling.y * off).sin();
            assert!(
                (left.distance(right) - chord).abs() < TOLERANCE,
                "l = {l}, ±{off}: {} apart, not {chord}",
                left.distance(right)
            );
            assert!(
                (left.y - right.y).abs() < TOLERANCE && left.x < right.x,
                "l = {l}, ±{off}: {left} and {right} not level, left of right"
            );
        }
    }
}

#[test]
fn a_triangle_turning_anticlockwise_in_the_chart_turns_anticlockwise_rolled_out() {
    let cone = cone();
    let step = 1e-3;
    for theta in [MIDDLE - 1.2, MIDDLE, MIDDLE + 0.9] {
        for l in [0.0, 1.5, 3.5] {
            let [a, b, c] = [
                DVec2::new(theta, l),
                DVec2::new(theta + step, l),
                DVec2::new(theta, l + step),
            ]
            .map(|at| unrolled(&cone, at, MIDDLE));
            let twice_the_area = (b - a).perp_dot(c - a);
            let expected = from_the_apex(&cone, l) * cone.ruling.y * step * step;
            assert!(
                twice_the_area > 0.0 && (twice_the_area / expected - 1.0).abs() < 1e-2,
                "θ = {theta}, l = {l}: twice the area {twice_the_area}, about {expected} expected"
            );
        }
    }
}
