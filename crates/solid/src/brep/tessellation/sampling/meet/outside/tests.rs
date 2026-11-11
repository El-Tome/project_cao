use std::collections::BTreeMap;
use std::f64::consts::{FRAC_PI_2, PI, TAU};

use glam::{DVec2, DVec3};

use super::{kept_outside, pierced, sag_at, sampled_angles};
use crate::brep::curve::{Meet, Wall as Carried};
use crate::brep::surface::{Cylinder, Surface};
use crate::brep::tessellation::contact::{self, Wall, Zones};
use crate::brep::tessellation::sampling::steps::{divisions_in, span};
use crate::brep::tessellation::sampling::taken;
use crate::brep::tessellation::tests::fixtures;
use crate::brep::topology::{Body, Coedge, EdgeId, SurfaceId};

const TOLERANCE: f64 = 0.02;

const PROBES: usize = 64;

/// The meet of the stock and a hole of radius 5 bored across it along Y,
/// from its top on the stock's wall at (0, 20, 10) round the hole to the
/// parameter `to`, between two vertices; and a wall along Z of `radius`,
/// inside the stock, touching its wall along the line through the top once
/// moved `off` away from it — or, given a `center`, standing about it.
struct Layout {
    to: f64,
    radius: f64,
    off: f64,
    center: Option<DVec3>,
    /// The heights a face on the wall stands between, if it holds one.
    faced: Option<[f64; 2]>,
    /// Whether the stock holds a face, from its foot to its top.
    stock_faced: bool,
    /// The angles round the wall the top ring of its face is parted at by
    /// vertices, the edge's first end standing at its own.
    parted: Vec<f64>,
}

/// What a [`Layout`] builds.
struct Laid {
    body: Body,
    edge: EdgeId,
    meet: Meet,
    wall: Wall,
    stock: Wall,
}

impl Layout {
    /// A quarter round the hole, down to (-5, 19.4, 5), beside a wall of
    /// `radius` touching the stock, neither holding a face.
    fn quarter(radius: f64) -> Layout {
        Layout {
            to: PI,
            radius,
            off: 0.0,
            center: None,
            faced: None,
            stock_faced: false,
            parted: Vec::new(),
        }
    }

    fn laid(self) -> Laid {
        let mut build = fixtures::Build::new();
        let stock = build.cylinder(DVec3::ZERO, DVec3::Z, fixtures::STOCK_RADIUS);
        let bore = build.cylinder(DVec3::Z * 5.0, DVec3::Y, fixtures::HOLE_RADIUS);
        let meet = build
            .meeting(stock, bore, fixtures::STOCK_RADIUS)
            .components[0];
        let top = meet.point(FRAC_PI_2);
        let ends = [build.vertex(top), build.vertex(meet.point(self.to))];
        let edge = build.meet_edge(meet, Some(ends), FRAC_PI_2, self.to);
        let across = DVec3::new(top.x, top.y, 0.0);
        let center = self
            .center
            .unwrap_or(across * (1.0 - (self.radius + self.off) / across.length()));
        let wall = build.cylinder(center, DVec3::Z, self.radius);
        let face = |build: &mut fixtures::Build, on: SurfaceId, [low, high]: [f64; 2]| {
            let (low, high) = (build.circle(on, low, None), build.circle(on, high, None));
            let use_of = |edge, forward| Coedge { edge, forward };
            build.face(
                on,
                false,
                vec![vec![use_of(low, true)], vec![use_of(high, false)]],
            );
        };
        match self.faced {
            Some([low, high]) if !self.parted.is_empty() => {
                let round = Cylinder::about(center, DVec3::Z, self.radius);
                let corners: Vec<_> = self
                    .parted
                    .iter()
                    .map(|angle| {
                        let point = round.point(DVec2::new(*angle, high));
                        if point.distance(top) < 1e-9 {
                            ends[0]
                        } else {
                            build.vertex(point)
                        }
                    })
                    .collect();
                let arcs: Vec<EdgeId> = (0..corners.len())
                    .map(|at| build.arc(wall, high, corners[at], corners[(at + 1) % corners.len()]))
                    .collect();
                let low = build.circle(wall, low, None);
                let backwards = arcs
                    .iter()
                    .rev()
                    .map(|edge| Coedge {
                        edge: *edge,
                        forward: false,
                    })
                    .collect();
                build.face(
                    wall,
                    false,
                    vec![
                        vec![Coedge {
                            edge: low,
                            forward: true,
                        }],
                        backwards,
                    ],
                );
            }
            Some(heights) => face(&mut build, wall, heights),
            None => {}
        }
        if self.stock_faced {
            face(&mut build, stock, [0.0, fixtures::HEIGHT]);
        }
        let body = build.finish(fixtures::STOCK_RADIUS);
        let round = |id: SurfaceId| match *body.surface(id) {
            Surface::Cylinder(cylinder) => (id, cylinder),
            _ => unreachable!("the walls are round"),
        };
        let (wall, stock) = (round(wall), round(stock));
        Laid {
            body,
            edge,
            meet,
            wall,
            stock,
        }
    }
}

/// [`Layout::quarter`], its wall holding a face the whole height of the stock.
fn a_quarter_meet_and_a_faced_wall_touching_the_stock_inside_at_its_top(
    radius: f64,
) -> (Body, EdgeId, Meet, Vec<Wall>) {
    let laid = Layout {
        faced: Some([0.0, fixtures::HEIGHT]),
        ..Layout::quarter(radius)
    }
    .laid();
    (laid.body, laid.edge, laid.meet, vec![laid.wall])
}

/// The edge's points from its first end to its last, `between` in the
/// middle.
fn run_of(body: &Body, edge: EdgeId, between: &[DVec3]) -> Vec<DVec3> {
    let ends = body.edge(edge).ends.expect("between two vertices");
    let mut run = vec![body.vertex(ends[0]).point];
    run.extend_from_slice(between);
    run.push(body.vertex(ends[1]).point);
    run
}

/// How deep the chord between `start` and `end` passes inside `wall` at its
/// deepest, probed at [`PROBES`] places along it.
fn deepest_inside(wall: &Cylinder, start: DVec3, end: DVec3) -> f64 {
    (1..PROBES)
        .map(|part| -wall.distance(start.lerp(end, part as f64 / PROBES as f64)))
        .fold(0.0, f64::max)
}

#[test]
fn a_meet_s_chord_passing_inside_a_wall_its_end_stands_on_deeper_than_its_triangles_sag_is_cut_until_it_dips_no_deeper_than_they_sag()
 {
    let (body, edge, meet, walls) =
        a_quarter_meet_and_a_faced_wall_touching_the_stock_inside_at_its_top(10.0);
    let eps = body.scale().eps();
    let wall = &walls[0].1;
    let angles = sampled_angles(
        divisions_in(wall.radius, TOLERANCE, span(&body)),
        Vec::new(),
    );
    assert!(
        deeper_than_the_sag(&meet, wall, &angles, eps, FRAC_PI_2, PI) > 0.0,
        "the fixture's chord dips no deeper than the wall's triangles sag"
    );

    let zones = Zones::of(&body, &walls);
    let contacts = BTreeMap::new();
    let kept = kept_outside(
        &body,
        edge,
        &meet,
        &walls,
        (TOLERANCE, span(&body)),
        Vec::new(),
        (&walls, &zones, &contacts),
    );

    assert!(!kept.is_empty(), "the chord is left whole");
    let mut last = FRAC_PI_2;
    for point in &kept {
        let t = meet.parameter(*point);
        assert!(
            (meet.point(t) - *point).length() < 1e-9,
            "{point} is off the curve"
        );
        assert!(last < t && t < PI, "{t} out of order after {last}");
        last = t;
    }
    let places: Vec<f64> = std::iter::once(FRAC_PI_2)
        .chain(kept.iter().map(|point| meet.parameter(*point)))
        .chain(std::iter::once(PI))
        .collect();
    for pair in places.windows(2) {
        let deeper = deeper_than_the_sag(&meet, wall, &angles, eps, pair[0], pair[1]);
        assert!(
            deeper <= 0.0,
            "the chord from {} to {} dips {deeper} deeper than the wall's triangles sag",
            pair[0],
            pair[1]
        );
    }
}

#[test]
fn a_meet_s_chords_ending_within_the_tolerance_of_a_wall_a_hair_inside_its_own_are_left_alone() {
    let hair = 1e-3;
    let (body, edge, meet, walls) =
        a_quarter_meet_and_a_faced_wall_touching_the_stock_inside_at_its_top(
            fixtures::STOCK_RADIUS - hair,
        );
    let eps = body.scale().eps();
    let wall = &walls[0].1;
    let points: Vec<DVec3> = (1..4)
        .map(|step| meet.point(FRAC_PI_2 + FRAC_PI_2 * step as f64 / 4.0))
        .collect();
    let run = run_of(&body, edge, &points);
    for point in &run {
        assert!(
            wall.distance(*point).abs() < TOLERANCE,
            "{point} stands further than the tolerance from the wall"
        );
    }
    assert!(
        run.windows(2)
            .all(|pair| deepest_inside(wall, pair[0], pair[1]) > 1e3 * eps),
        "a chord of the fixture stays outside the wall"
    );
    assert!(
        run.iter().any(|point| wall.distance(*point) > 1e2 * eps),
        "the fixture's curve lies on the wall"
    );

    let zones = Zones::of(&body, &walls);
    let contacts = BTreeMap::new();
    let kept = kept_outside(
        &body,
        edge,
        &meet,
        &walls,
        (TOLERANCE, span(&body)),
        points.clone(),
        (&walls, &zones, &contacts),
    );

    assert_eq!(kept, points);
}

#[test]
fn a_meet_s_own_samples_stand_bit_for_bit_among_those_the_wall_adds() {
    let (body, edge, meet, walls) =
        a_quarter_meet_and_a_faced_wall_touching_the_stock_inside_at_its_top(10.0);
    let points: Vec<DVec3> = [0.5, 0.75]
        .map(|share| meet.point(FRAC_PI_2 + FRAC_PI_2 * share) + DVec3::X * 1e-10)
        .to_vec();

    let zones = Zones::of(&body, &walls);
    let contacts = BTreeMap::new();
    let kept = kept_outside(
        &body,
        edge,
        &meet,
        &walls,
        (TOLERANCE, span(&body)),
        points.clone(),
        (&walls, &zones, &contacts),
    );

    assert!(kept.len() > points.len(), "the wall adds no sample");
    let mut found = kept.iter();
    for point in &points {
        assert!(
            found.any(|kept| kept.to_array() == point.to_array()),
            "{point} is not kept as it was, in its place"
        );
    }
}

/// How far inside a circle of `radius` the chord between its angles `low`
/// and `high` stands on the ray at `angle`.
fn sag_of_the_chord(radius: f64, low: f64, high: f64, angle: f64) -> f64 {
    let (start, end) = (
        DVec2::from_angle(low) * radius,
        DVec2::from_angle(high) * radius,
    );
    let along = end - start;
    radius - start.perp_dot(along) / DVec2::from_angle(angle).perp_dot(along)
}

#[test]
fn a_wall_is_sampled_at_every_step_of_its_grid_and_at_every_ray_brought_within_a_turn_in_order() {
    let angles = sampled_angles(4, vec![-FRAC_PI_2, TAU + 0.1, 1.0]);

    let expected = [
        0.0,
        0.1,
        1.0,
        FRAC_PI_2,
        PI,
        3.0 * FRAC_PI_2,
        3.0 * FRAC_PI_2,
    ];
    assert_eq!(angles.len(), expected.len(), "{angles:?}");
    for (angle, expected) in angles.iter().zip(expected) {
        assert!((angle - expected).abs() < 1e-12, "{angles:?}");
    }
}

const SAMPLED: [f64; 4] = [0.0, 1.0, 2.5, 4.0];

#[test]
fn a_wall_s_triangles_stand_on_it_at_every_angle_it_is_sampled_at() {
    for angle in SAMPLED {
        let sag = sag_at(&SAMPLED, 10.0, angle);
        assert!(sag.abs() < 1e-12, "{sag} at {angle}");
    }
}

#[test]
fn halfway_between_two_angles_a_wall_s_triangles_sag_by_its_radius_less_the_cosine_of_half_the_step()
 {
    let sag = sag_at(&SAMPLED, 10.0, 1.75);

    assert!((sag - 10.0 * (1.0 - 0.75_f64.cos())).abs() < 1e-12, "{sag}");
}

#[test]
fn between_two_angles_a_wall_s_triangles_stand_where_the_chord_between_them_crosses_the_ray() {
    for angle in [1.2, 2.3] {
        let sag = sag_at(&SAMPLED, 10.0, angle);

        let expected = sag_of_the_chord(10.0, 1.0, 2.5, angle);
        assert!(
            (sag - expected).abs() < 1e-12,
            "{sag} at {angle}, not {expected}"
        );
    }
}

#[test]
fn past_the_last_angle_a_wall_s_triangles_sag_as_the_chord_to_the_first_a_turn_on() {
    let off_the_grid = [0.5, 1.0, 2.5, 4.0];
    for (angles, angle) in [(SAMPLED, 4.5), (SAMPLED, 6.0), (off_the_grid, 5.0)] {
        let sag = sag_at(&angles, 10.0, angle);

        let expected = sag_of_the_chord(10.0, 4.0, angles[0] + TAU, angle);
        assert!(
            (sag - expected).abs() < 1e-12,
            "{sag} at {angle}, not {expected}"
        );
    }
}

/// The walls of `walls` a chord of the laid edge between its two ends
/// pierces, the grid of each cut for `tolerance`.
fn pierced_in(laid: &Laid, walls: &[Wall], tolerance: f64) -> Vec<Wall> {
    let zones = Zones::of(&laid.body, walls);
    let contacts = BTreeMap::new();
    pierced(
        &laid.body,
        laid.edge,
        &laid.meet,
        (walls, &zones, &contacts),
        (tolerance, span(&laid.body)),
        &[],
    )
}

#[test]
fn a_third_wall_a_meet_s_chord_dips_into_deeper_than_its_triangles_sag_where_it_holds_a_face_is_pierced()
 {
    let laid = Layout {
        faced: Some([0.0, fixtures::HEIGHT]),
        ..Layout::quarter(10.0)
    }
    .laid();

    assert_eq!(pierced_in(&laid, &[laid.wall], TOLERANCE), vec![laid.wall]);
}

#[test]
fn a_meet_s_own_wall_is_never_pierced_by_its_chords() {
    let laid = Layout {
        stock_faced: true,
        ..Layout::quarter(10.0)
    }
    .laid();
    let (_, stock) = laid.stock;
    assert_eq!(laid.meet.first, Carried::Cylinder(stock));
    let ends = run_of(&laid.body, laid.edge, &[]);
    assert!(
        deepest_inside(&stock, ends[0], ends[1]) > 0.1,
        "the fixture's chord stays near the stock's wall"
    );

    assert_eq!(pierced_in(&laid, &[laid.stock], TOLERANCE), Vec::new());
}

#[test]
fn a_wall_no_end_of_the_edge_lies_on_is_not_pierced() {
    let laid = Layout {
        off: 1e-6,
        faced: Some([0.0, fixtures::HEIGHT]),
        ..Layout::quarter(10.0)
    }
    .laid();
    let eps = laid.body.scale().eps();
    let ends = run_of(&laid.body, laid.edge, &[]);
    assert!(
        ends.iter()
            .all(|end| laid.wall.1.distance(*end) > 10.0 * eps)
    );
    assert!(deepest_inside(&laid.wall.1, ends[0], ends[1]) > 1e3 * eps);

    assert_eq!(pierced_in(&laid, &[laid.wall], TOLERANCE), Vec::new());
}

#[test]
fn a_wall_bored_away_where_the_chord_dips_into_it_is_not_pierced() {
    let laid = Layout {
        faced: Some([2.0 * fixtures::HEIGHT, 3.0 * fixtures::HEIGHT]),
        ..Layout::quarter(10.0)
    }
    .laid();

    assert_eq!(pierced_in(&laid, &[laid.wall], TOLERANCE), Vec::new());
}

#[test]
fn a_wall_whose_triangles_sag_further_than_the_chord_dips_is_not_pierced() {
    let coarse = 1.0;
    let laid = Layout {
        faced: Some([0.0, fixtures::HEIGHT]),
        ..Layout::quarter(10.0)
    }
    .laid();
    let eps = laid.body.scale().eps();
    let (_, wall) = laid.wall;
    let angles = sampled_angles(
        divisions_in(wall.radius, coarse, span(&laid.body)),
        Vec::new(),
    );
    let ends = run_of(&laid.body, laid.edge, &[]);
    assert!(deepest_inside(&wall, ends[0], ends[1]) > 1e3 * eps);
    for part in 1..PROBES {
        let chord = ends[0].lerp(ends[1], part as f64 / PROBES as f64);
        let angle = wall.parameters(chord).x.rem_euclid(TAU);
        let depth = -wall.distance(chord);
        assert!(
            depth <= sag_at(&angles, wall.radius, angle),
            "the chord dips {depth} at {angle}, below the wall's triangles"
        );
    }

    assert_eq!(pierced_in(&laid, &[laid.wall], coarse), Vec::new());
}

#[test]
fn a_wall_no_end_of_the_edge_lies_on_keeps_no_chord_of_it_outside() {
    let laid = Layout {
        off: 1e-6,
        faced: Some([0.0, fixtures::HEIGHT]),
        ..Layout::quarter(10.0)
    }
    .laid();
    let eps = laid.body.scale().eps();
    let ends = run_of(&laid.body, laid.edge, &[]);
    assert!(
        ends.iter()
            .all(|end| laid.wall.1.distance(*end) > 10.0 * eps)
    );
    assert!(deepest_inside(&laid.wall.1, ends[0], ends[1]) > 1e3 * eps);

    let walls = [laid.wall];
    let zones = Zones::of(&laid.body, &walls);
    let contacts = BTreeMap::new();
    let kept = kept_outside(
        &laid.body,
        laid.edge,
        &laid.meet,
        &walls,
        (TOLERANCE, span(&laid.body)),
        Vec::new(),
        (&walls, &zones, &contacts),
    );

    assert_eq!(kept, Vec::new());
}

#[test]
fn a_chord_dipping_into_a_wall_no_deeper_than_the_kernel_s_tolerance_is_left_alone() {
    let laid = Layout {
        to: FRAC_PI_2 + 4.733e-4,
        faced: Some([0.0, fixtures::HEIGHT]),
        ..Layout::quarter(10.0)
    }
    .laid();
    let eps = laid.body.scale().eps();
    let (_, wall) = laid.wall;
    let ends = run_of(&laid.body, laid.edge, &[]);
    assert!(deepest_inside(&wall, ends[0], ends[1]) <= eps);
    assert!(
        wall.distance(
            laid.meet
                .point((FRAC_PI_2 + laid.body.edge(laid.edge).to) / 2.0)
        ) > eps
    );

    let walls = [laid.wall];
    let zones = Zones::of(&laid.body, &walls);
    let contacts = BTreeMap::new();
    let kept = kept_outside(
        &laid.body,
        laid.edge,
        &laid.meet,
        &walls,
        (0.0, span(&laid.body)),
        Vec::new(),
        (&walls, &zones, &contacts),
    );

    assert_eq!(kept, Vec::new());
}

#[test]
fn a_chord_passing_inside_a_wall_the_curve_itself_runs_along_is_left_alone() {
    let laid = Layout {
        to: FRAC_PI_2 + 0.02,
        faced: Some([0.0, fixtures::HEIGHT]),
        ..Layout::quarter(fixtures::STOCK_RADIUS - 1e-3)
    }
    .laid();
    let eps = laid.body.scale().eps();
    let (_, wall) = laid.wall;
    let ends = run_of(&laid.body, laid.edge, &[]);
    assert!(deepest_inside(&wall, ends[0], ends[1]) > 1e3 * eps);
    for part in 0..=PROBES {
        let t = FRAC_PI_2 + 0.02 * part as f64 / PROBES as f64;
        assert!(wall.distance(laid.meet.point(t)).abs() <= eps);
    }
    assert!(wall.distance(ends[1]) > 0.0);

    let walls = [laid.wall];
    let zones = Zones::of(&laid.body, &walls);
    let contacts = BTreeMap::new();
    let kept = kept_outside(
        &laid.body,
        laid.edge,
        &laid.meet,
        &walls,
        (0.0, span(&laid.body)),
        Vec::new(),
        (&walls, &zones, &contacts),
    );

    assert_eq!(kept, Vec::new());
}

/// Half round the hole, from its top down to its foot at (0, 20, 0), both on
/// a wall of radius 10 touching the stock inside, which holds a face; sampled
/// at the parameters `samples`. The chord from the top dips into the wall far
/// deeper than its triangles sag; the chord to the foot from nine tenths of
/// the way dips into it less.
fn a_half_meet_from_a_wall_s_top_to_its_foot(samples: &[f64]) -> (Laid, Vec<DVec3>) {
    let laid = Layout {
        to: 3.0 * FRAC_PI_2,
        faced: Some([0.0, fixtures::HEIGHT]),
        ..Layout::quarter(10.0)
    }
    .laid();
    let points = samples.iter().map(|t| laid.meet.point(*t)).collect();
    (laid, points)
}

/// The edge's points once its chords are kept outside the walls they pierce,
/// the two steps taken as the sampling of an edge takes them.
fn kept_outside_in(laid: &Laid, tolerance: f64, points: Vec<DVec3>) -> Vec<DVec3> {
    let walls = [laid.wall];
    let zones = Zones::of(&laid.body, &walls);
    let contacts = BTreeMap::new();
    let pierced = pierced(
        &laid.body,
        laid.edge,
        &laid.meet,
        (&walls, &zones, &contacts),
        (tolerance, span(&laid.body)),
        &points,
    );
    assert_eq!(pierced, vec![laid.wall], "the edge's chords pierce no wall");
    kept_outside(
        &laid.body,
        laid.edge,
        &laid.meet,
        &pierced,
        (tolerance, span(&laid.body)),
        points,
        (&walls, &zones, &contacts),
    )
}

/// How much deeper the chord of `meet` from `from` to `to` dips into `wall`
/// than the wall's triangles sag, sampled at `angles`, at its worst, where
/// the curve stands outside the wall: below nought, nowhere.
fn deeper_than_the_sag(
    meet: &Meet,
    wall: &Cylinder,
    angles: &[f64],
    eps: f64,
    from: f64,
    to: f64,
) -> f64 {
    let (start, end) = (meet.point(from), meet.point(to));
    (1..PROBES)
        .filter_map(|part| {
            let share = part as f64 / PROBES as f64;
            let chord = start.lerp(end, share);
            let depth = -wall.distance(chord);
            let sag = sag_at(
                angles,
                wall.radius,
                wall.parameters(chord).x.rem_euclid(TAU),
            );
            let outside = wall.distance(meet.point(from + (to - from) * share)) > eps;
            (outside && depth > eps).then_some(depth - sag)
        })
        .fold(f64::NEG_INFINITY, f64::max)
}

fn parameters_between(meet: &Meet, kept: &[DVec3], from: f64, to: f64) -> Vec<f64> {
    kept.iter()
        .map(|point| meet.parameter(*point))
        .filter(|t| from + 1e-9 < *t && *t < to - 1e-9)
        .collect()
}

#[test]
fn a_chord_dipping_into_a_pierced_wall_no_deeper_than_its_triangles_sag_is_left_whole() {
    let last = FRAC_PI_2 + 0.9 * PI;
    let (laid, points) = a_half_meet_from_a_wall_s_top_to_its_foot(&[PI, last]);
    let eps = laid.body.scale().eps();
    let (_, wall) = laid.wall;
    let angles = sampled_angles(
        divisions_in(wall.radius, TOLERANCE, span(&laid.body)),
        Vec::new(),
    );
    let foot = 3.0 * FRAC_PI_2;
    let (start, end) = (laid.meet.point(last), laid.meet.point(foot));
    assert!(
        deepest_inside(&wall, start, end) > 1e2 * eps,
        "the chord to the foot stays outside the wall"
    );
    assert!(
        deeper_than_the_sag(&laid.meet, &wall, &angles, eps, last, foot) <= 0.0,
        "the chord to the foot dips deeper than the wall's triangles sag"
    );
    assert!(
        wall.distance(start) > TOLERANCE,
        "the chord to the foot stays within the tolerance of the wall"
    );

    let kept = kept_outside_in(&laid, TOLERANCE, points);

    let added = parameters_between(&laid.meet, &kept, last, foot);
    assert!(
        added.is_empty(),
        "the chord to the foot is cut at {added:?}"
    );
}

#[test]
fn a_chord_dipping_into_a_wall_deeper_than_its_triangles_sag_is_halved_only_until_it_dips_no_deeper_than_they_sag()
 {
    let (laid, points) = a_half_meet_from_a_wall_s_top_to_its_foot(&[PI, FRAC_PI_2 + 0.9 * PI]);
    let eps = laid.body.scale().eps();
    let (_, wall) = laid.wall;
    let angles = sampled_angles(
        divisions_in(wall.radius, TOLERANCE, span(&laid.body)),
        Vec::new(),
    );
    let meet = laid.meet;
    let to = meet.parameter(points[0]);

    let kept = kept_outside_in(&laid, TOLERANCE, points);

    let mut stretches = vec![(FRAC_PI_2, to)];
    while let Some((from, to)) = stretches.pop() {
        let added = parameters_between(&meet, &kept, from, to);
        let deeper = deeper_than_the_sag(&meet, &wall, &angles, eps, from, to);
        if deeper > 0.0 {
            let middle = (from + to) / 2.0;
            assert!(
                added.iter().any(|t| (t - middle).abs() <= 1e-9),
                "the chord from {from} to {to} dips {deeper} deeper than the wall's triangles sag, and is not halved: {added:?}"
            );
            stretches.extend([(from, middle), (middle, to)]);
        } else {
            assert!(
                added.is_empty(),
                "the chord from {from} to {to} dips no deeper than the wall's triangles sag, and is cut at {added:?}"
            );
        }
    }
}

#[test]
fn a_chord_dipping_into_a_pierced_wall_deeper_than_its_triangles_sag_where_it_is_bored_away_is_left_whole()
 {
    let face_from = 6.0;
    let laid = Layout {
        to: 3.0 * FRAC_PI_2,
        faced: Some([face_from, fixtures::HEIGHT]),
        ..Layout::quarter(10.0)
    }
    .laid();
    let points = vec![laid.meet.point(PI)];
    let eps = laid.body.scale().eps();
    let (_, wall) = laid.wall;
    let angles = sampled_angles(
        divisions_in(wall.radius, TOLERANCE, span(&laid.body)),
        Vec::new(),
    );
    let (top, middle, foot) = (FRAC_PI_2, PI, 3.0 * FRAC_PI_2);
    for (from, to) in [(top, middle), (middle, foot)] {
        assert!(
            deeper_than_the_sag(&laid.meet, &wall, &angles, eps, from, to) > 0.0,
            "the chord from {from} to {to} dips no deeper than the wall's triangles sag"
        );
    }
    let (start, end) = (laid.meet.point(top), laid.meet.point(middle));
    assert!(
        (1..PROBES).any(|part| {
            let chord = start.lerp(end, part as f64 / PROBES as f64);
            let sag = sag_at(
                &angles,
                wall.radius,
                wall.parameters(chord).x.rem_euclid(TAU),
            );
            -wall.distance(chord) > sag && chord.z > face_from
        }),
        "the chord from the top dips deeper than the wall's triangles sag nowhere its face holds"
    );
    for t in [middle, foot] {
        assert!(
            laid.meet.point(t).z < face_from,
            "the chord to the foot reaches the wall's face"
        );
    }

    let kept = kept_outside_in(&laid, TOLERANCE, points);

    assert!(
        !parameters_between(&laid.meet, &kept, top, middle).is_empty(),
        "the chord from the top, dipping where the wall holds a face, is left whole"
    );
    let cut = parameters_between(&laid.meet, &kept, middle, foot);
    assert!(
        cut.is_empty(),
        "the chord to the foot, dipping where the wall is bored away, is cut at {cut:?}"
    );
}

#[test]
fn a_chord_dipping_into_a_wall_only_where_the_curve_itself_runs_inside_it_is_left_whole() {
    let (to, radius) = (FRAC_PI_2 + 0.3 * PI, 3.0);
    let (sin, cos) = 115f64.to_radians().sin_cos();
    let laid = Layout {
        center: Some(DVec3::new(
            radius * cos,
            fixtures::STOCK_RADIUS + radius * sin,
            0.0,
        )),
        faced: Some([0.0, fixtures::HEIGHT]),
        to,
        ..Layout::quarter(radius)
    }
    .laid();
    let eps = laid.body.scale().eps();
    let (_, wall) = laid.wall;
    let meet = laid.meet;
    let ends = run_of(&laid.body, laid.edge, &[]);
    assert!(
        wall.distance(ends[0]).abs() <= eps,
        "the top stands off the wall"
    );
    assert!(
        wall.distance(ends[1]) > TOLERANCE,
        "the far end stands within the tolerance of the wall"
    );
    let angles = sampled_angles(
        divisions_in(wall.radius, TOLERANCE, span(&laid.body)),
        Vec::new(),
    );
    assert!(
        (1..PROBES).any(|part| {
            let chord = ends[0].lerp(ends[1], part as f64 / PROBES as f64);
            let sag = sag_at(
                &angles,
                wall.radius,
                wall.parameters(chord).x.rem_euclid(TAU),
            );
            -wall.distance(chord) > sag.max(eps)
        }),
        "the chord dips no deeper than the wall's triangles sag"
    );
    let deeper = deeper_than_the_sag(&meet, &wall, &angles, eps, FRAC_PI_2, to);
    assert!(
        deeper <= 0.0,
        "the chord dips {deeper} deeper than the wall's triangles sag where the curve stands outside"
    );

    let walls = [laid.wall];
    let zones = Zones::of(&laid.body, &walls);
    let contacts = BTreeMap::new();
    let kept = kept_outside(
        &laid.body,
        laid.edge,
        &meet,
        &walls,
        (TOLERANCE, span(&laid.body)),
        Vec::new(),
        (&walls, &zones, &contacts),
    );

    assert_eq!(kept, Vec::new());
}

/// Whether the segment from `start` to `end` passes through the triangle
/// `corners`, clear of its edges and of its own ends by more than `room`.
fn passes_through(start: DVec3, end: DVec3, corners: &[DVec3; 3], room: f64) -> bool {
    let [a, b, c] = *corners;
    let (along, ab, ac) = (end - start, b - a, c - a);
    let normal = ab.cross(ac);
    let across = normal.dot(along);
    if across.abs() <= f64::EPSILON * normal.length() * along.length() {
        return false;
    }
    let share = normal.dot(a - start) / across;
    let point = start + along * share;
    let doubled = normal.length_squared();
    let u = (point - a).cross(ac).dot(normal) / doubled;
    let v = ab.cross(point - a).dot(normal) / doubled;
    let clear = room / along.length();
    share > clear && share < 1.0 - clear && u > room && v > room && u + v < 1.0 - room
}

#[test]
fn a_wall_whose_face_is_parted_closer_than_its_grid_near_the_vertex_takes_a_ray_at_each_vertex_and_no_chord_of_the_meet_passes_through_its_triangles()
 {
    let to = FRAC_PI_2 + 0.3;
    let closer = 0.01;
    let parted: Vec<f64> = (-3..=20)
        .map(|step| FRAC_PI_2 + closer * step as f64)
        .collect();
    let laid = Layout {
        to,
        faced: Some([0.0, fixtures::HEIGHT]),
        parted: parted.clone(),
        ..Layout::quarter(10.0)
    }
    .laid();
    let eps = laid.body.scale().eps();
    let (surface, wall) = laid.wall;
    let steps = divisions_in(wall.radius, TOLERANCE, span(&laid.body));
    assert!(
        closer < TAU / steps as f64 / 4.0,
        "the face's top ring is parted no closer than a quarter of the wall's grid"
    );
    let grid = sampled_angles(steps, Vec::new());
    let ends = run_of(&laid.body, laid.edge, &[]);
    assert!(
        deepest_inside(&wall, ends[0], ends[1]) > 1e3 * eps,
        "the chord stays outside the wall"
    );
    assert!(
        deeper_than_the_sag(&laid.meet, &wall, &grid, eps, FRAC_PI_2, to) <= 0.0,
        "the chord dips deeper than the wall's grid sags"
    );

    let fineness = (TOLERANCE, span(&laid.body));
    let all = contact::walls(&laid.body);
    let zones = Zones::of(&laid.body, &all);
    let meets = vec![Vec::new(); laid.body.edge_ids().count()];
    let contacts = contact::contacts(
        &laid.body,
        &all,
        &zones,
        &meets,
        &BTreeMap::new(),
        &BTreeMap::new(),
        fineness,
    );
    let rays = taken(&Carried::Cylinder(wall), &wall, &all, &contacts);
    for angle in &parted {
        assert!(
            rays.iter()
                .any(|ray| ((ray - angle + PI).rem_euclid(TAU) - PI).abs() < 1e-9),
            "the wall takes no ray at its vertex at {angle}: {rays:?}"
        );
    }

    let kept = kept_outside(
        &laid.body,
        laid.edge,
        &laid.meet,
        &[laid.wall],
        fineness,
        Vec::new(),
        (&all, &zones, &contacts),
    );

    let cut = laid.body.triangles_by_face(TOLERANCE);
    let triangles: Vec<&[DVec3; 3]> = cut
        .triangles
        .iter()
        .zip(&cut.faces)
        .filter(|(_, face)| laid.body.face(**face).surface == surface)
        .map(|(corners, _)| corners)
        .collect();
    assert!(!triangles.is_empty(), "the wall's face is cut into nothing");
    let run = run_of(&laid.body, laid.edge, &kept);
    for pair in run.windows(2) {
        let through = triangles
            .iter()
            .filter(|corners| passes_through(pair[0], pair[1], corners, 1e-9))
            .count();
        assert_eq!(
            through, 0,
            "the chord from {} to {} passes through {through} of the wall's triangles",
            pair[0], pair[1]
        );
    }
}
