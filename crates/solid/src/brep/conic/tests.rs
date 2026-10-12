use std::f64::consts::{PI, TAU};

use glam::{DVec2, DVec3};

use super::built::Section;
use super::*;
use crate::brep::scale::Scale;

const REACH: f64 = 20.0;

fn scale() -> Scale {
    Scale::of(REACH)
}

/// A tenth of the tolerance: how far a point read back may land from where
/// it was.
fn held() -> f64 {
    scale().eps() / 10.0
}

fn plane(point: DVec3, normal: DVec3) -> Plane {
    Plane::through(point, normal).0
}

fn shaft(radius: f64) -> Round {
    Round::Cylinder(Cylinder::about(
        DVec3::new(1.0, -2.0, 0.0),
        DVec3::Z,
        radius,
    ))
}

/// A cone about the Z axis through `(1, −2)` opening at `slope` (`dρ/dh`),
/// at radius 5 at height 0.
fn point_of(slope: f64) -> Round {
    let angle = slope.atan();
    Round::Cone(Cone::through(
        DVec3::new(1.0, -2.0, 0.0),
        DVec3::Z,
        [
            DVec2::new(0.0, 5.0),
            DVec2::new(5.0 * angle.cos(), 5.0 + 5.0 * angle.sin()),
        ],
    ))
}

/// The plane's normal turned from `from` towards `to` by `angle`.
fn leaning(from: DVec3, to: DVec3, angle: f64) -> DVec3 {
    from * angle.cos() + to * angle.sin()
}

fn in_the_box(point: DVec3) -> bool {
    point.abs().max_element() <= REACH
}

/// Points of the cut, each from the ruling at its angle, in the box, with
/// the angle from the plane's normal they stand at: read at evenly spread
/// angles, and where the cut crosses the wall's circles
/// at lengths evenly spread across the box, which is where a long arm passes
/// it within a sliver of angle.
fn cut_points(plane: &Plane, round: &Round, count: usize) -> Vec<(f64, DVec3)> {
    let section = Section::of(plane, round).expect("a circular wall");
    let crossings = (0..=count)
        .map(|step| 4.0 * REACH * (step as f64 / count as f64 - 0.5))
        .filter_map(|length| section.crossing(length))
        .flat_map(|psi| [psi, -psi]);
    (0..count)
        .map(|step| -PI + TAU * (step as f64 + 0.5) / count as f64)
        .chain(crossings)
        .filter(|psi| section.own(*psi))
        .map(|psi| (psi, section.point(section.phi + psi)))
        .filter(|(_, point)| point.is_finite() && in_the_box(*point))
        .collect()
}

/// The conic of `conics` that holds the point at `psi`.
fn holding(conics: &[Conic], psi: f64) -> &Conic {
    conics
        .iter()
        .find(|conic| match conic.arm {
            Arm::Whole => true,
            Arm::Split { side } => f64::from(side) * psi.sin() >= 0.0,
        })
        .expect("a conic holds every point of the cut in the box")
}

/// Every pair of a plane and a wall the sweeps cut: tilts from along the
/// axis and from square to it, planes a few tolerances from touching the
/// wall or from its apex, slopes from a near cylinder to a near disc.
fn sweep() -> Vec<(Plane, Round)> {
    let eps = scale().eps();
    let tilts = [1e-9, 1e-6, 1e-3, 0.3, 1.0];
    let offsets = [1.0, 30.0, 1e3];
    let mut pairs = Vec::new();
    for radius in [0.5, 7.0] {
        let round = shaft(radius);
        for tilt in tilts {
            for (normal, far) in [
                (leaning(DVec3::X, DVec3::Z, tilt), radius),
                (leaning(DVec3::Z, DVec3::X, tilt), 0.0),
            ] {
                for offset in offsets.iter().flat_map(|off| [-off, *off]) {
                    let through = DVec3::new(1.0 + far + offset * eps, -2.0, 0.5);
                    pairs.push((plane(through, normal), round));
                }
            }
        }
    }
    for slope in [1e-8, 1e-4, 1.0, 1e4, 1e8] {
        let round = point_of(slope);
        let Round::Cone(cone) = round else {
            unreachable!()
        };
        let apex = cone.apex();
        for tilt in tilts {
            for normal in [
                leaning(DVec3::X, DVec3::Z, tilt),
                leaning(DVec3::Z, DVec3::X, tilt),
            ] {
                for offset in offsets {
                    pairs.push((plane(apex + normal * (offset * eps), normal), round));
                    let through = DVec3::new(1.0, -2.0, 3.0);
                    pairs.push((plane(through + normal * (offset * eps), normal), round));
                }
            }
        }
    }
    pairs
}

/// How far the worst point of the cut in the box is read back off itself,
/// and how many were read.
fn read_back(plane: &Plane, round: &Round) -> (f64, usize) {
    let conics = Conic::cut(plane, round, scale());
    let points = cut_points(plane, round, 96);
    let worst = points
        .iter()
        .map(|(psi, point)| {
            let conic = holding(&conics, *psi);
            conic.point(conic.parameter(*point)).distance(*point)
        })
        .fold(0.0, f64::max);
    (worst, points.len())
}

#[test]
fn an_ellipse_a_parabola_and_a_hyperbola_are_one_formula_either_side_of_the_parabola() {
    let curve = |k: f64, bend: f64| Conic {
        plane: plane(DVec3::ZERO, DVec3::Z),
        round: shaft(1.0),
        anchor: DVec3::new(3.0, 0.0, 0.0),
        tangent: DVec3::Y,
        bend: DVec3::X * bend,
        k,
        arm: Arm::Whole,
    };
    let (a, b) = (3.0, 2.0);
    let ellipse = curve(1.0 / (b * b), -a / (b * b));
    let hyperbola = curve(-1.0 / (b * b), a / (b * b));
    for t in [-4.0, -1.0, -0.01, 0.0, 0.3, 2.0, 5.0] {
        let p = ellipse.point(t);
        let on = p.x * p.x / (a * a) + p.y * p.y / (b * b) - 1.0;
        assert!(on.abs() < 1e-14, "an ellipse at {t}: {on}");
        let p = hyperbola.point(t);
        let on = p.x * p.x / (a * a) - p.y * p.y / (b * b) - 1.0;
        assert!(
            on.abs() < 1e-12 * (1.0 + p.length_squared()),
            "a hyperbola at {t}: {on}"
        );
    }
    let semi = 0.5;
    let parabola = curve(0.0, 1.0 / semi);
    for t in [-3.0, -0.2, 0.0, 0.1, 4.0] {
        let p = parabola.point(t) - parabola.anchor;
        let on = p.y * p.y - 2.0 * semi * p.x;
        assert!(on.abs() < 1e-13, "a parabola at {t}: {on}");
    }
    for t in [-3.0, 0.05, 2.5] {
        for k in [1e-9, 1e-6, 1e-3] {
            let [below, at, above] = [-k, 0.0, k].map(|k| curve(k, 1.0 / semi).point(t));
            let step = (above - below) / 2.0;
            assert!(
                (below + step - at).length() <= 1e-3 * step.length() + 1e-15,
                "the parabola is the middle of its neighbours at {t}, k = {k}"
            );
            assert!(
                step.length() <= k * (t.abs().powi(3) + t.powi(4) / semi),
                "a step of {k} moves the curve by {step}"
            );
        }
    }
}

/// Points read off each conic's own formula between two of the cut's points
/// it holds in the box, and still in it.
fn between(plane: &Plane, round: &Round) -> Vec<DVec3> {
    let conics = Conic::cut(plane, round, scale());
    let points = cut_points(plane, round, 96);
    let mut found = Vec::new();
    for conic in &conics {
        let mut along: Vec<f64> = points
            .iter()
            .filter(|(psi, _)| std::ptr::eq(holding(&conics, *psi), conic))
            .map(|(_, point)| conic.parameter(*point))
            .collect();
        along.sort_by(f64::total_cmp);
        found.extend(
            along
                .windows(2)
                .map(|pair| conic.point((pair[0] + pair[1]) / 2.0))
                .filter(|point| in_the_box(*point)),
        );
    }
    found
}

/// A pair of each kind: an ellipse round a shaft, its arms where the plane
/// lies a hair off along the axis, a hyperbola where a plane along a cone's
/// axis cuts it, a parabola where it lies along a ruling, and an ellipse
/// round a cone.
fn kinds() -> Vec<(Plane, Round)> {
    let ruling = DVec3::new(1.0, 0.0, -1.0);
    vec![
        (
            plane(DVec3::new(1.0, -2.0, 0.0), leaning(DVec3::Z, DVec3::X, 0.5)),
            shaft(5.0),
        ),
        (
            plane(
                DVec3::new(2.0, -2.0, 0.0),
                leaning(DVec3::X, DVec3::Z, 1e-7),
            ),
            shaft(5.0),
        ),
        (plane(DVec3::new(2.0, -2.0, 0.0), DVec3::X), point_of(1.0)),
        (plane(DVec3::new(3.0, -2.0, 0.0), ruling), point_of(1.0)),
        (
            plane(DVec3::new(1.0, -2.0, 3.0), leaning(DVec3::Z, DVec3::X, 0.3)),
            point_of(1.0),
        ),
    ]
}

/// Each conic of each kind, and the stretch of its parameter the cut's
/// points in the box span.
fn stretches() -> Vec<(Conic, [f64; 2])> {
    let mut found = Vec::new();
    for (plane, round) in kinds() {
        let conics = Conic::cut(&plane, &round, scale());
        let points = cut_points(&plane, &round, 96);
        for conic in &conics {
            let along: Vec<f64> = points
                .iter()
                .filter(|(psi, _)| std::ptr::eq(holding(&conics, *psi), conic))
                .map(|(_, point)| conic.parameter(*point))
                .collect();
            let low = along.iter().copied().fold(f64::INFINITY, f64::min);
            let high = along.iter().copied().fold(f64::NEG_INFINITY, f64::max);
            found.push((*conic, [low, high]));
        }
    }
    assert_eq!(found.len(), 6, "every kind is cut, the arms in two");
    found
}

fn evenly(from: f64, to: f64, count: usize) -> impl Iterator<Item = f64> {
    (0..=count).map(move |step| from + (to - from) * step as f64 / count as f64)
}

#[test]
fn a_conic_reads_back_the_parameter_of_every_point_it_passes_through() {
    let mut read = 0;
    for (plane, round) in sweep() {
        let (off, count) = read_back(&plane, &round);
        assert!(
            off <= held(),
            "read back {off} off on {plane:?} against {round:?}"
        );
        read += count;
    }
    assert!(read > 20_000, "the sweep reads back {read} points");
}

#[test]
fn the_conic_a_plane_cuts_a_cylinder_lies_on_both_to_rounding() {
    let mut read = 0;
    for (plane, round) in sweep() {
        let Round::Cylinder(cylinder) = round else {
            continue;
        };
        for point in between(&plane, &round) {
            let [off_plane, off_wall] = [plane.distance(point), cylinder.distance(point)];
            assert!(
                off_plane.abs().max(off_wall.abs()) <= held(),
                "{point} stands {off_plane} off {plane:?} and {off_wall} off {cylinder:?}"
            );
            read += 1;
        }
    }
    assert!(read > 5_000, "the sweep reads {read} points on cylinders");
}

#[test]
fn the_conic_a_plane_cuts_a_cone_lies_on_its_own_nappe_to_rounding() {
    let mut read = 0;
    for (plane, round) in sweep() {
        let Round::Cone(cone) = round else {
            continue;
        };
        for point in between(&plane, &round) {
            let [off_plane, off_wall] = [plane.distance(point), cone.distance(point)];
            assert!(
                off_plane.abs().max(off_wall.abs()) <= held(),
                "{point} stands {off_plane} off {plane:?} and {off_wall} off {cone:?}"
            );
            read += 1;
        }
    }
    assert!(read > 5_000, "the sweep reads {read} points on cones");
}

#[test]
fn a_conic_built_twice_from_the_same_surfaces_has_the_same_bits() {
    for (plane, round) in sweep() {
        let [once, twice] = [0, 1].map(|_| Conic::cut(&plane, &round, scale()));
        assert!(
            once.len() == twice.len()
                && once
                    .iter()
                    .zip(&twice)
                    .all(|(one, other)| format!("{one:?}") == format!("{other:?}")),
            "{once:?} built again is {twice:?}"
        );
    }
    let point = DVec3::new(3.0, -1.0, 2.0);
    let normal = leaning(DVec3::Z, DVec3::X, 0.4);
    let [up, down] = [normal, -normal].map(|normal| plane(point, normal));
    assert_eq!(
        Conic::cut(&up, &shaft(5.0), scale()),
        Conic::cut(&down, &shaft(5.0), scale()),
        "a plane is the same whichever way its normal was given"
    );
}

#[test]
fn a_conic_runs_with_the_angle_of_its_wall() {
    for (plane, round) in sweep() {
        let conics = Conic::cut(&plane, &round, scale());
        for (psi, point) in cut_points(&plane, &round, 24) {
            let conic = holding(&conics, psi);
            let t = conic.parameter(point);
            let [seen, speed, _] = conic.seen_on(On::Wall, t);
            assert!(
                speed.x > 0.0,
                "the wall's angle runs back at {t} on {conic:?}"
            );
            let (own, off_axis) = match round {
                Round::Cylinder(cylinder) => (cylinder.parameters(point).x, cylinder.radius),
                Round::Cone(cone) => (cone.parameters(point).x, cone.off_axis(point)),
                Round::Elliptic(wall) => (wall.parameters(point).x, wall.major),
            };
            let apart = (seen.x - own).rem_euclid(TAU);
            let apart = apart.min(TAU - apart);
            assert!(
                apart * off_axis <= held(),
                "seen at the angle {} where the point stands at {own}",
                seen.x
            );
        }
    }
}

#[test]
fn a_conic_whose_vertices_stand_beyond_ten_thousand_reaches_is_built_as_two_arms_held_to_the_tolerance()
 {
    let round = shaft(5.0);
    let plane = plane(
        DVec3::new(2.0, -2.0, 0.0),
        leaning(DVec3::X, DVec3::Z, 1e-7),
    );
    let section = Section::of(&plane, &round).expect("a circular wall");
    for psi in [0.0, PI] {
        assert!(section.point(section.phi + psi).length() > LONG * REACH);
    }
    let conics = Conic::cut(&plane, &round, scale());
    assert_eq!(
        conics.iter().map(|conic| conic.arm).collect::<Vec<_>>(),
        [Arm::Split { side: 1 }, Arm::Split { side: -1 }]
    );
    assert!(
        conics
            .iter()
            .all(|conic| conic.anchor.distance(DVec3::new(1.0, -2.0, 0.0)) <= 5.0 + held()),
        "each arm is anchored on the wall's foot circle"
    );
    let (off, count) = read_back(&plane, &round);
    assert!(
        count > 50 && off <= held(),
        "{count} points read back {off} off"
    );
}

#[test]
fn a_hairpin_ellipse_is_anchored_at_its_vertex_in_the_box() {
    let round = shaft(5.0);
    let vertex = DVec3::new(6.0, -2.0, 3.0);
    let plane = plane(vertex, leaning(DVec3::X, DVec3::Z, 1e-6));
    let conics = Conic::cut(&plane, &round, scale());
    assert_eq!(conics.len(), 1);
    assert_eq!(conics[0].arm, Arm::Whole);
    let conic = conics[0];
    assert!(
        conic.anchor.distance(vertex) <= 1e-6 && conic.tangent.dot(DVec3::Z).abs() <= 1e-12,
        "anchored at {} rather than at the vertex, where the curve runs square to the axis",
        conic.anchor
    );
    let (off, count) = read_back(&plane, &round);
    assert!(
        count > 30 && off <= held(),
        "{count} points read back {off} off"
    );
}

#[test]
fn a_hyperbola_a_hair_from_the_apex_reads_its_arms_back_to_rounding() {
    let apex = DVec3::new(1.0, -2.0, 0.0);
    let round = Round::Cone(Cone::through(
        apex,
        DVec3::Z,
        [DVec2::ZERO, DVec2::new(10.0, 5.0)],
    ));
    for hair in [1.0, 10.0, 1e3] {
        let plane = plane(apex + DVec3::X * (hair * scale().eps()), DVec3::X);
        let conics = Conic::cut(&plane, &round, scale());
        assert_eq!(conics.len(), 1);
        assert!(conics[0].k < 0.0 && conics[0].arm == Arm::Whole);
        let points = cut_points(&plane, &round, 96);
        assert!(
            points.iter().any(|(psi, _)| *psi > 0.0) && points.iter().any(|(psi, _)| *psi < 0.0),
            "both arms of the branch stand in the box"
        );
        let (off, count) = read_back(&plane, &round);
        assert!(
            count > 50 && off <= held(),
            "{count} points read back {off} off"
        );
    }
}

#[test]
fn a_cut_through_an_apex_outside_the_box_is_built_as_its_ruling_without_dividing_by_nought() {
    let round = point_of(1e-4);
    let Round::Cone(cone) = round else {
        unreachable!()
    };
    let apex = cone.apex();
    assert!(!in_the_box(apex));
    for tilt in [1e-9, 1e-6] {
        let plane = plane(apex, leaning(DVec3::X, DVec3::Z, tilt));
        let conics = Conic::cut(&plane, &round, scale());
        assert!(!conics.is_empty());
        for conic in &conics {
            assert!(
                [conic.anchor, conic.tangent, conic.bend]
                    .iter()
                    .all(|v| v.is_finite())
                    && conic.k.is_finite(),
                "{conic:?}"
            );
        }
        let normal = plane.normal;
        let rise = normal.dot(cone.axis);
        let lean = normal.dot(cone.u).hypot(normal.dot(cone.v));
        let facing = normal.dot(cone.v).atan2(normal.dot(cone.u));
        let opening = (-rise * cone.ruling.x / (cone.ruling.y * lean)).acos();
        let mut read = 0;
        for side in [-1.0, 1.0] {
            let angle = facing + side * opening;
            for length in evenly(-2.0 * REACH, 2.0 * REACH, 64) {
                let point = cone.point(DVec2::new(angle, length));
                if !in_the_box(point) {
                    continue;
                }
                let off = conics
                    .iter()
                    .map(|conic| conic.point(conic.parameter(point)).distance(point))
                    .fold(f64::INFINITY, f64::min);
                assert!(
                    off <= held(),
                    "{point} of the ruling is read back {off} off"
                );
                read += 1;
            }
        }
        assert!(read > 50, "{read} points of the rulings read");
    }
    let round = Round::Cone(Cone::through(
        DVec3::ZERO,
        DVec3::Z,
        [DVec2::ZERO, DVec2::new(10.0, 5.0)],
    ));
    let Round::Cone(cone) = round else {
        unreachable!()
    };
    let plane = plane(DVec3::ZERO, leaning(DVec3::X, DVec3::Z, 0.2));
    assert_eq!(Section::of(&plane, &round).expect("a circular wall").c, 0.0);
    let conics = Conic::cut(&plane, &round, scale());
    assert_eq!(conics.len(), 2, "two rulings");
    for conic in &conics {
        assert!(conic.k == 0.0 && conic.bend == DVec3::ZERO);
        let far = conic.point(10.0);
        assert!(plane.distance(far).abs() <= held() && cone.distance(far).abs() <= held());
    }
}

#[test]
fn a_conic_s_derivatives_are_its_speed_and_bend() {
    for (conic, [from, to]) in stretches() {
        let step = 1e-4 * (to - from);
        for t in evenly(from, to, 7) {
            let [before, here, after] = [t - step, t, t + step].map(|t| conic.point(t));
            let speed = (after - before) / (2.0 * step);
            let bend = (after - 2.0 * here + before) / (step * step);
            let [exact_speed, exact_bend] = [conic.derivative(t), conic.bending(t)];
            assert!(
                (speed - exact_speed).length() <= 1e-6 * exact_speed.length(),
                "speed {exact_speed} where the points give {speed} on {conic:?}"
            );
            assert!(
                (bend - exact_bend).length() <= 1e-3 * exact_bend.length() + 1e-6,
                "bend {exact_bend} where the points give {bend} on {conic:?}"
            );
        }
    }
}

#[test]
fn a_conic_s_box_holds_its_stretch_and_touches_its_extremes() {
    for (conic, [from, to]) in stretches() {
        let [low, high] = conic.bounds(from, to);
        let mut reached = [DVec3::INFINITY, DVec3::NEG_INFINITY];
        for t in evenly(from, to, 4000) {
            let point = conic.point(t);
            assert!(
                point.cmpge(low).all() && point.cmple(high).all(),
                "{point} leaves [{low}, {high}]"
            );
            reached = [reached[0].min(point), reached[1].max(point)];
        }
        let room = 1e-5 * (high - low).max_element();
        assert!(
            (reached[0] - low).max_element() <= room && (high - reached[1]).max_element() <= room,
            "[{low}, {high}] is wider than the stretch's [{}, {}]",
            reached[0],
            reached[1]
        );
    }
}

#[test]
fn a_conic_turns_back_along_a_direction_where_its_closed_form_says() {
    let directions = [
        DVec3::X,
        DVec3::Y,
        DVec3::Z,
        DVec3::new(1.0, 2.0, -3.0).normalize(),
    ];
    for (conic, [from, to]) in stretches() {
        for direction in directions {
            let turning = conic.turning(direction, from, to);
            for t in &turning {
                let speed = conic.derivative(*t);
                assert!(
                    speed.dot(direction).abs() <= 1e-12 * speed.length(),
                    "{conic:?} runs along {direction} at {t}"
                );
            }
            let signs: Vec<bool> = evenly(from, to, 2000)
                .map(|t| conic.derivative(t).dot(direction) > 0.0)
                .collect();
            let changes = signs.windows(2).filter(|pair| pair[0] != pair[1]).count();
            assert_eq!(
                changes,
                turning.len(),
                "{conic:?} turns back along {direction} at {turning:?}"
            );
        }
    }
}

#[test]
fn a_conic_crosses_a_plane_where_its_closed_form_says() {
    let normals = [
        DVec3::X,
        DVec3::Y,
        DVec3::Z,
        DVec3::new(1.0, 2.0, -3.0).normalize(),
    ];
    for (conic, [from, to]) in stretches() {
        for t in evenly(from, to, 7) {
            let point = conic.point(t);
            for normal in normals {
                let speed = conic.derivative(t).normalize();
                if speed.dot(normal).abs() < 1e-3 {
                    continue;
                }
                let across = plane(point, normal);
                let roots = conic.across(&across);
                assert!(
                    roots
                        .iter()
                        .any(|root| (root - t).abs() <= 1e-9 * (1.0 + t.abs())),
                    "{conic:?} crosses {normal} through its point at {t}, not at {roots:?}"
                );
                for root in roots.iter().filter(|root| in_the_box(conic.point(**root))) {
                    assert!(
                        across.distance(conic.point(*root)).abs() <= held(),
                        "{conic:?} at {root} stands off the plane it was said to cross"
                    );
                }
            }
        }
    }
}

#[test]
fn a_plane_a_few_tolerances_past_a_hairpin_s_vertex_crosses_it_twice_where_it_stands() {
    let round = shaft(5.0);
    let vertex = DVec3::new(6.0, -2.0, 3.0);
    let lean = 1e-7;
    let conic = Conic::cut(
        &plane(vertex, leaning(DVec3::X, DVec3::Z, lean)),
        &round,
        scale(),
    )[0];
    for tolerances in [3.0, 30.0, 300.0] {
        let past = conic.anchor.z + tolerances * scale().eps();
        let floor = plane(DVec3::new(0.0, 0.0, past), DVec3::Z);
        let roots = conic.across(&floor);
        assert_eq!(roots.len(), 2, "{tolerances} tolerances past the vertex");
        let half = (2.0 * 5.0 * lean * tolerances * scale().eps()).sqrt();
        for root in roots {
            let point = conic.point(root);
            assert!(
                (point.z - past).abs() <= held()
                    && ((point.y - conic.anchor.y).abs() - half).abs() <= 1e-6 * half,
                "{tolerances} tolerances past the vertex, crossed at {point} rather than {half} \
                 either side of it"
            );
        }
    }
}

#[test]
fn a_conic_seen_on_its_wall_is_a_graph_over_the_wall_s_angle() {
    for (conic, [from, to]) in stretches() {
        let step = 1e-4 * (to - from);
        for on in [On::Wall, On::Plane] {
            let mut before = f64::NEG_INFINITY;
            for t in evenly(from, to, 9) {
                let [here, first, second] = conic.seen_on(on, t);
                let [back, ahead] = [t - step, t + step].map(|t| conic.seen_on(on, t)[0]);
                let speed = (ahead - back) / (2.0 * step);
                let bend = (ahead - 2.0 * here + back) / (step * step);
                assert!(
                    (speed - first).length() <= 1e-6 * first.length(),
                    "{first} where the points give {speed} on {on:?}"
                );
                assert!(
                    (bend - second).length() <= 1e-3 * second.length() + 1e-6,
                    "{second} where the points give {bend} on {on:?}"
                );
                if on == On::Wall {
                    assert!(here.x > before, "the angle runs back at {t}");
                    before = here.x;
                }
            }
        }
    }
}

#[test]
fn the_point_of_a_conic_at_a_grid_angle_stands_at_that_angle_within_the_outline_s_room() {
    const CLEAR: f64 = 1e-9;
    const STEPS: f64 = 48.0;
    let mut read = 0;
    for (conic, [from, to]) in stretches() {
        let [start, end] = [from, to].map(|t| conic.seen_on(On::Wall, t)[0].x);
        let first = (start * STEPS / TAU).ceil() as i64;
        let last = (end * STEPS / TAU).floor() as i64;
        for grid in first..=last {
            let angle = TAU * grid as f64 / STEPS;
            let (t, point) = conic.at_angle(angle);
            let own = match conic.round {
                Round::Cylinder(cylinder) => cylinder.parameters(point).x,
                Round::Cone(cone) => cone.parameters(point).x,
                Round::Elliptic(wall) => wall.parameters(point).x,
            };
            let apart = (own - angle).rem_euclid(TAU);
            assert!(
                apart.min(TAU - apart) <= CLEAR,
                "{point} stands at {own}, not {angle}"
            );
            assert!(
                (conic.seen_on(On::Wall, t)[0].x - angle).abs() <= CLEAR,
                "its parameter {t} is seen at another angle"
            );
            assert!(conic.point(t).distance(point) <= held());
            read += 1;
        }
    }
    assert!(read > 20, "{read} grid angles read");
}

mod elliptic;
