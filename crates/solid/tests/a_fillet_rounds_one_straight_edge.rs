//! A fillet rounds one straight edge of an exact body off: given the name the
//! edge answers to (#582) and a radius, it lays a cylinder of that radius
//! tangent to the edge's two faces, which stop where it takes over, set back
//! along each by r / tan(angle / 2) — the radius itself where they meet
//! square. A radius the two faces cannot give is reduced to the largest they
//! can, and the step says both. It is the chamfer's step (#696) with a
//! radius, an operation of the kernel on [`Body`]; the step of the history
//! that calls it comes with the application, in #695.
//!
//! Closes #697.
//! - A fillet step given the name of a straight edge between two planar faces
//!   of a block, and a radius, leaves a closed body where the edge is replaced
//!   by one cylindrical face of that radius, tangent to both faces: its two
//!   straight sides are edges of those two faces, its two ends edges of the
//!   faces at the edge's ends. The body passes the soundness checks —
//!   `a_fillet_on_a_block_s_top_front_edge_lays_one_round_of_the_radius_tangent_to_the_top_and_the_front`;
//!   the one check the public body cannot be asked, that its listing stands
//!   on its geometry, from inside the crate by
//!   `every_body_a_fillet_leaves_lists_what_it_is_made_of_as_its_geometry_stands`
//!   in `src/body/tests/fillet.rs`
//! - The same holds on an edge whose two faces meet at an angle other than a
//!   right one (set-back along each face: r / tan(angle / 2)) —
//!   `a_fillet_on_an_edge_whose_faces_meet_at_a_slant_is_set_back_by_the_radius_over_the_tangent_of_half_the_angle`
//! - The same holds where the edge ends on an oblique wall or on a round, as
//!   for the chamfer (hexagon, trapezoid, slot) —
//!   `a_fillet_on_a_top_edge_of_a_hexagonal_prism_runs_on_past_both_its_ends_to_the_walls_beside_it`,
//!   `a_fillet_on_the_trapezoid_s_slanted_top_edge_is_trimmed_by_the_front_and_the_back_it_meets_at_its_ends`,
//!   `a_fillet_on_a_slot_s_front_runs_on_over_a_round_meeting_it_at_a_few_degrees_trimmed_by_the_round_s_wall`,
//!   the slot's round turning in; turning out, it stands past the front's
//!   line, where no one cylinder takes the matter, and is declined
//!   `Unsupported`, as the issue's "an end the kernel cannot build is declined
//!   with its reason" lets it —
//!   `a_fillet_on_a_slot_s_front_whose_round_end_bulges_out_past_the_front_is_declined_as_unsupported`
//! - A radius larger than the neighbouring faces can give is reduced to the
//!   largest that fits, and the step reports the radius asked and the radius
//!   taken —
//!   `a_fillet_asked_more_than_the_top_can_give_takes_the_whole_top_and_reports_both_radii`,
//!   a face's reach measured along it at a slant by
//!   `a_fillet_asked_more_than_a_slanted_wall_can_give_takes_the_radius_whose_set_back_is_the_wall_s_whole_width`
//! - After a change of size, a move or a turn upstream, the step rounds the
//!   same edge, found by its name —
//!   `a_fillet_rounds_the_edge_its_name_finds_again_after_a_change_of_size_a_move_and_a_turn`
//! - An edge its name no longer finds, gone or split, is a declined step whose
//!   reason says which —
//!   `a_fillet_on_an_edge_a_cut_took_away_is_declined_saying_it_is_gone`,
//!   `a_fillet_on_an_edge_a_cut_parted_in_two_is_declined_saying_it_is_split`
//! - On two blocks joined so that two edges become one, a fillet on that edge
//!   rounds it whole (as #696) —
//!   `a_fillet_on_the_edge_a_join_bridged_from_two_rounds_it_whole`
//! - A concave edge is declined `Unsupported` —
//!   `a_fillet_on_the_inner_corner_of_an_l_where_its_faces_meet_inwards_is_declined_as_unsupported`
//! - The face numbers of every existing part are unchanged; the cylinder takes
//!   the number after the steps' last one —
//!   `a_fillet_leaves_every_other_face_of_the_block_answering_to_the_number_it_had_and_numbers_its_round_after_the_steps_last`;
//!   the parts with no fillet keep theirs by `faces_keep_their_numbers.rs`
//!   and the #682 tests of `edges_and_vertices_keep_their_names.rs`
//! - The exact STEP export of a filleted block writes the cylindrical face on
//!   its true surface —
//!   `a_filleted_block_written_to_step_writes_its_round_on_a_cylinder_of_the_radius`
//! - The chamfer's tests still pass — no test: the chamfer's own suite,
//!   `a_chamfer_cuts_one_straight_edge.rs` and `src/body/tests/chamfer.rs`,
//!   left as it was, is the answer

use std::collections::BTreeSet;
use std::f64::consts::PI;

use cao_solid::profile::{Contour, Frame, Profile, Run};
use cao_solid::soundness::{closed, enclosed, uncrossed};
use cao_solid::{Body, Declined, EdgeName, Filleted, Loop};
use glam::{DVec2, DVec3};

/// How far a corner may stand from where the arithmetic puts it, on a plane
/// or on the round: each is placed exactly, so what is left is rounding.
const PLACE: f64 = 1e-9;

/// How near the arithmetic an exact volume comes, as a fraction of it.
const EXACT: f64 = 1e-9;

/// How far the triangles may stand from a round, as a share of the reach the
/// body carries: the drawing's tolerance.
const DRAWN: f64 = 2e-4;

/// The numbers a raised block's faces answer to: its floor nought, its top
/// one, and its sides from the front round to the left, two to five; and the
/// leaning trapezoid's slant and back.
const FLOOR: usize = 0;
const TOP: usize = 1;
const FRONT: usize = 2;
const RIGHT: usize = 3;
const LEFT: usize = 5;
const SLANT: usize = 3;
const BACK: usize = 4;

/// The ground's plane, which a part is drawn on.
fn ground() -> Frame {
    Frame {
        origin: DVec3::ZERO,
        u: DVec3::X,
        v: DVec3::Y,
    }
}

/// An outline with its runs, as the drawing hands it to the exact kernel.
fn profile(outline: Contour) -> Profile<'static> {
    Profile {
        exact: Some((outline, Vec::new())),
        sampled: Loop::straight(&[]),
        sampled_holes: Vec::new(),
        triangles: &[],
    }
}

/// An outline drawn on the ground raised up by `height`, and joined as
/// `cao_part` builds a step: its floor numbered nought, its top one, and its
/// walls from two on, one for each run of the outline.
fn raised(outline: Contour, height: f64) -> Body {
    let part = Body::default();
    let tool = part
        .tool_raised(&profile(outline), ground(), DVec3::Z * height)
        .expect("the tool raises");
    part.union(&tool).expect("the part is built")
}

/// A block of a length, a breadth and a height standing on the ground's
/// origin, numbered as [`raised`] numbers it.
fn block([length, breadth, height]: [f64; 3]) -> Body {
    raised(
        Contour::rectangle(DVec2::ZERO, DVec2::new(length, breadth)),
        height,
    )
}

/// The outline of a right trapezoid: its front 40 long on the X axis, its
/// back 37 long at Y = 9, both from the Y axis, and between their right ends
/// a slant rising 3 for every 1 it comes back. The slant meets the front at
/// an acute corner and the back at an obtuse one, whose cosines are 1/√10
/// and −1/√10: the tangents of their halves, 3/(√10 + 1) and 3/(√10 − 1),
/// are no round numbers, and nor are the set-backs a round asks there.
const LEANING: [DVec2; 4] = [
    DVec2::ZERO,
    DVec2::new(40.0, 0.0),
    DVec2::new(37.0, 9.0),
    DVec2::new(0.0, 9.0),
];

/// The [`LEANING`] trapezoid raised by `height`, its walls numbered from the
/// front round: the front two, the slant three, the back four and the left
/// five.
fn leaning_trapezoid(height: f64) -> Body {
    raised(Contour::straight(LEANING.to_vec()), height)
}

/// The two corners an upright edge of a part raised from the ground stands
/// between, at the point of its outline `at`.
fn upright(at: DVec2, height: f64) -> [DVec3; 2] {
    [at.extend(0.0), at.extend(height)]
}

/// The name of the one edge between the faces numbered `first` and `second`,
/// as the body hands it to whoever points at that edge.
fn edge_between(body: &Body, first: usize, second: usize) -> EdgeName {
    let sides = [vec![first as u32], vec![second as u32]];
    let named: Vec<EdgeName> = body
        .edge_names()
        .into_iter()
        .filter(|name| name.between == sides)
        .collect();
    assert_eq!(
        named.len(),
        1,
        "one edge between {first} and {second}: {named:?}"
    );
    named[0].clone()
}

/// Every number some face of the body answers to: a step's, below where the
/// steps' numbers end, and a piece's, past them.
fn numbers(body: &Body) -> BTreeSet<usize> {
    (0..body.faces_end())
        .chain(body.pieces())
        .filter(|&number| body.has_face(number))
        .collect()
}

/// Holds that a face's corners are the points expected: each of them a
/// corner, and no corner anywhere else.
fn corners_are(body: &Body, face: usize, expected: &[DVec3], what: &str) {
    corners_are_or_on(body, face, expected, |_| false, what);
}

/// Holds that a face's corners are the points expected, each of them a
/// corner, and every other corner on the curve `on` says it is on.
fn corners_are_or_on(
    body: &Body,
    face: usize,
    expected: &[DVec3],
    on: impl Fn(DVec3) -> bool,
    what: &str,
) {
    let plane = body
        .plane_of(face)
        .unwrap_or_else(|| panic!("{what}: the body offers no plane on face {face}"));
    for corner in &plane.corners {
        assert!(
            on(*corner) || expected.iter().any(|point| point.distance(*corner) < PLACE),
            "{what}: a corner at {corner}, where the corners are {expected:?}"
        );
    }
    for point in expected {
        assert!(
            plane
                .corners
                .iter()
                .any(|corner| corner.distance(*point) < PLACE),
            "{what}: no corner at {point}, among {:?}",
            plane.corners
        );
    }
}

/// Holds that a body with curved faces is exact and keeps the rules every
/// solid must: its surface closed, no face through another, the matter it
/// holds what arithmetic promised, and the matter its triangles enclose,
/// which stand within the drawing's tolerance of its rounds, within `drawn`
/// of it.
fn sound_drawn_within(body: &Body, promised: f64, drawn: f64, what: &str) {
    assert!(
        body.is_exact(),
        "{what}: the exact kernel computed the body"
    );
    let triangles = body.triangles();
    assert_eq!(closed(&triangles), Ok(()), "{what}");
    assert_eq!(uncrossed(&triangles), Ok(()), "{what}");
    assert!(
        (body.volume() - promised).abs() <= EXACT * promised,
        "{what}: the body holds {}, where arithmetic promised {promised}",
        body.volume()
    );
    let enclosed = enclosed(&triangles);
    assert!(
        (enclosed - promised).abs() <= drawn,
        "{what}: its triangles enclose {enclosed}, where arithmetic promised {promised}"
    );
}

/// Holds that a fillet was given the radius it was asked.
fn taken_whole(filleted: &Filleted, radius: f64, what: &str) {
    assert!(
        (filleted.asked - radius).abs() < PLACE && (filleted.taken - radius).abs() < PLACE,
        "{what}: asked {} and took {}, where {radius} fits",
        filleted.asked,
        filleted.taken
    );
}

/// Holds that a step left every face of the part it was given and made one
/// face more, curved, and gives back the number that face answers to.
fn one_round_more(part: &Body, body: &Body, what: &str) -> usize {
    let (before, after) = (numbers(part), numbers(body));
    assert!(
        before.is_subset(&after),
        "{what}: a face went: {before:?} became {after:?}"
    );
    let made: Vec<usize> = after.difference(&before).copied().collect();
    assert_eq!(
        made.len(),
        1,
        "{what}: one face more: {before:?} became {after:?}"
    );
    assert!(
        !body.is_flat(made[0]) && body.plane_of(made[0]).is_none(),
        "{what}: the face the fillet lays is curved, and offers no plane to draw on"
    );
    made[0]
}

/// The round a fillet is expected to lay: a cylinder of `radius` about the
/// line from `start` along `along` for `length`, turning about it from facing
/// the way the edge's first face does to facing the way its second does.
/// Facing each face's way where it meets it is what tangent to both comes to.
struct Round {
    start: DVec3,
    along: DVec3,
    length: f64,
    radius: f64,
    facing: [DVec3; 2],
}

impl Round {
    /// The point of the axis `share` of the way along it.
    fn axis_at(&self, share: f64) -> DVec3 {
        self.start + self.along * self.length * share
    }

    /// The way the round faces `share` of the way round it, from its first
    /// face's way to its second's.
    fn facing_at(&self, share: f64) -> DVec3 {
        let [first, second] = self.facing;
        let turn = first.dot(second).acos() * share;
        let across = (second - first * first.dot(second)).normalize();
        first * turn.cos() + across * turn.sin()
    }

    /// Whether a point lies on the round: on its cylinder, between its two
    /// ends, and between its two sides.
    fn holds(&self, point: DVec3) -> bool {
        let off = point - self.start;
        let along = off.dot(self.along);
        let across = off - self.along * along;
        let [first, second] = self.facing;
        let least = self.radius * first.dot(second);
        (across.length() - self.radius).abs() < PLACE
            && (-PLACE..=self.length + PLACE).contains(&along)
            && across.dot(first) >= least - PLACE
            && across.dot(second) >= least - PLACE
    }

    /// The two points where the round's sides meet a wall square to it,
    /// `end` of the way along it.
    fn sides_at(&self, end: f64) -> [DVec3; 2] {
        [0.0, 1.0].map(|share| self.axis_at(end) + self.facing_at(share) * self.radius)
    }

    /// The round's area, its turn times its radius times its length.
    fn area(&self) -> f64 {
        let [first, second] = self.facing;
        first.dot(second).acos() * self.radius * self.length
    }
}

/// What the triangles of a body may enclose apart from its matter, its curved
/// faces `area` in all: the drawing's tolerance at the reach the body carries,
/// over them. That reach is taken at `reach`, twice the part's own, for
/// whatever the round is cut with reaching past the part.
fn drawn_within(reach: f64, area: f64) -> f64 {
    DRAWN * reach * area
}

/// Holds that the face numbered `face` is the round expected: every corner
/// of its triangles on it, and every ray aimed square at its axis from
/// outside meeting it first, where it faces away from the axis along the ray.
fn is_the_round(body: &Body, face: usize, round: &Round, what: &str) {
    let corners: Vec<DVec3> = body.triangles_of(face).flatten().collect();
    assert!(!corners.is_empty(), "{what}: the round is drawn");
    for corner in corners {
        assert!(
            round.holds(corner),
            "{what}: the round is drawn through {corner}, off the cylinder of {} about the line \
             from {} along {}",
            round.radius,
            round.start,
            round.along
        );
    }
    for along in [0.11, 0.47, 0.83] {
        for share in [0.1, 0.3, 0.5, 0.7, 0.9] {
            let facing = round.facing_at(share);
            let from = round.axis_at(along) + facing * (round.radius + 10.0);
            let hit = body.ray_hit(from, -facing).unwrap_or_else(|| {
                panic!("{what}: a ray from {from} toward the axis meets nothing")
            });
            assert_eq!(
                hit.face, face,
                "{what}: a ray from {from} toward the axis meets face {} before the round",
                hit.face
            );
            assert!(
                hit.normal.distance(facing) < PLACE,
                "{what}: where a ray from {from} meets it, the round faces {}, not {facing}, \
                 away from its axis",
                hit.normal
            );
        }
    }
}

/// Holds that a flat wall at one end of the edge is trimmed by the round: its
/// corners the points `expected`, which hold the two where the round's sides
/// meet it, and every other corner on the round's end — an arc of its radius
/// on a wall square to it, of an ellipse on an oblique one.
fn trimmed_by_the_round(body: &Body, wall: usize, expected: &[DVec3], round: &Round, what: &str) {
    corners_are_or_on(body, wall, expected, |corner| round.holds(corner), what);
}

/// The sides of the faces the face numbered `face` meets, one for each edge
/// it has, in order.
fn sides_met(body: &Body, face: usize) -> Vec<Vec<u32>> {
    let own = vec![face as u32];
    let mut met: Vec<Vec<u32>> = body
        .edge_names()
        .into_iter()
        .filter_map(|name| {
            let [first, second] = name.between;
            if first == own {
                Some(second)
            } else if second == own {
                Some(first)
            } else {
                None
            }
        })
        .collect();
    met.sort();
    met
}

#[test]
fn a_fillet_on_a_block_s_top_front_edge_lays_one_round_of_the_radius_tangent_to_the_top_and_the_front()
 {
    let [length, breadth, height] = [30.0, 10.0, 20.0];
    let radius = 3.0;
    let block = block([length, breadth, height]);
    let edge = edge_between(&block, TOP, FRONT);

    let filleted = block.fillet(&edge, radius).expect("the fillet is laid");

    let what = "the top front edge";
    taken_whole(&filleted, radius, what);
    let body = &filleted.body;
    let round = Round {
        start: DVec3::new(0.0, radius, height - radius),
        along: DVec3::X,
        length,
        radius,
        facing: [DVec3::Z, -DVec3::Y],
    };
    sound_drawn_within(
        body,
        length * breadth * height - length * radius * radius * (1.0 - PI / 4.0),
        drawn_within(2.0 * length, round.area()),
        what,
    );
    let fillet = one_round_more(&block, body, what);
    is_the_round(
        body,
        fillet,
        &round,
        "the round, its axis the radius under the top and the radius behind the front",
    );
    assert_eq!(
        sides_met(body, fillet),
        [TOP, FRONT, RIGHT, LEFT].map(|side| vec![side as u32]),
        "the round meets the top and the front along its sides, and the right and the left \
         walls at its ends, by one edge each"
    );
    assert!(
        body.edge_names()
            .iter()
            .all(|name| name.between != [vec![TOP as u32], vec![FRONT as u32]]),
        "the top and the front no longer meet: the round stands between them"
    );
    let on_top = |along: f64| DVec3::new(along, radius, height);
    let on_front = |along: f64| DVec3::new(along, 0.0, height - radius);
    corners_are(
        body,
        TOP,
        &[
            on_top(0.0),
            on_top(length),
            DVec3::new(length, breadth, height),
            DVec3::new(0.0, breadth, height),
        ],
        "what is left of the top, set back by the radius",
    );
    corners_are(
        body,
        FRONT,
        &[
            DVec3::ZERO,
            DVec3::new(length, 0.0, 0.0),
            on_front(length),
            on_front(0.0),
        ],
        "what is left of the front, set back by the radius",
    );
    for (wall, x, end) in [(LEFT, 0.0, 0.0), (RIGHT, length, 1.0)] {
        let kept = [
            DVec3::new(x, 0.0, 0.0),
            DVec3::new(x, breadth, 0.0),
            DVec3::new(x, breadth, height),
        ];
        trimmed_by_the_round(
            body,
            wall,
            &[&kept[..], &round.sides_at(end)].concat(),
            &round,
            &format!("the wall at x = {x}, the round's end on it"),
        );
    }
}

/// An upright edge of the leaning trapezoid where two of its walls meet at a
/// slant: the walls, the corner of the outline it stands on, the far end of
/// each wall from it, and the way each wall faces, all in the ground's plane.
struct Upright {
    what: &'static str,
    walls: [usize; 2],
    corner: DVec2,
    far: [DVec2; 2],
    outward: [DVec2; 2],
}

#[test]
fn a_fillet_on_an_edge_whose_faces_meet_at_a_slant_is_set_back_by_the_radius_over_the_tangent_of_half_the_angle()
 {
    let (height, radius) = (5.0, 2.0);
    let slant = DVec2::new(3.0, 1.0) / 10.0f64.sqrt();
    let edges = [
        Upright {
            what: "the acute edge between the front and the slant",
            walls: [FRONT, SLANT],
            corner: LEANING[1],
            far: [LEANING[0], LEANING[2]],
            outward: [-DVec2::Y, slant],
        },
        Upright {
            what: "the obtuse edge between the slant and the back",
            walls: [SLANT, BACK],
            corner: LEANING[2],
            far: [LEANING[1], LEANING[3]],
            outward: [slant, DVec2::Y],
        },
    ];

    for edge in edges {
        let what = edge.what;
        let part = leaning_trapezoid(height);
        let name = edge_between(&part, edge.walls[0], edge.walls[1]);

        let filleted = part
            .fillet(&name, radius)
            .unwrap_or_else(|declined| panic!("{what}: declined, {declined:?}"));

        taken_whole(&filleted, radius, what);
        let body = &filleted.body;
        let into = edge.far.map(|far| (far - edge.corner).normalize());
        let angle = into[0].dot(into[1]).acos();
        let set_back = radius / (angle / 2.0).tan();
        let touching = into.map(|into| edge.corner + into * set_back);
        let round = Round {
            start: (touching[0] - edge.outward[0] * radius).extend(0.0),
            along: DVec3::Z,
            length: height,
            radius,
            facing: edge.outward.map(|outward| outward.extend(0.0)),
        };
        let trapezoid = (40.0 + 37.0) / 2.0 * 9.0 * height;
        let corner_rounded_off =
            (set_back * radius - radius * radius * (PI - angle) / 2.0) * height;
        sound_drawn_within(
            body,
            trapezoid - corner_rounded_off,
            drawn_within(2.0 * 40.0, round.area()),
            what,
        );
        let fillet = one_round_more(&part, body, what);
        is_the_round(
            body,
            fillet,
            &round,
            &format!("the round on {what}, its axis the radius off both walls"),
        );
        let mut sides = [FLOOR, TOP, edge.walls[0], edge.walls[1]];
        sides.sort();
        assert_eq!(
            sides_met(body, fillet),
            sides.map(|side| vec![side as u32]),
            "{what}: the round meets its two walls along its sides, and the floor and the top \
             at its ends, by one edge each"
        );
        assert!(
            body.edge_names().iter().all(
                |name| name.between != [vec![edge.walls[0] as u32], vec![edge.walls[1] as u32]]
            ),
            "{what}: the two walls no longer meet: the round stands between them"
        );
        for ((wall, touching), far) in edge.walls.into_iter().zip(touching).zip(edge.far) {
            corners_are(
                body,
                wall,
                &[upright(touching, height), upright(far, height)].concat(),
                &format!("what is left of wall {wall} at {what}, set back by {set_back}"),
            );
        }
        let others: Vec<DVec2> = LEANING
            .into_iter()
            .filter(|&corner| corner != edge.corner)
            .collect();
        for (wall, z, end) in [(FLOOR, 0.0, 0.0), (TOP, height, 1.0)] {
            let mut expected: Vec<DVec3> = others.iter().map(|corner| corner.extend(z)).collect();
            expected.extend(round.sides_at(end));
            trimmed_by_the_round(
                body,
                wall,
                &expected,
                &round,
                &format!("face {wall} at {what}, the round's end on it"),
            );
        }
    }
}

/// A regular hexagon of side 10 raised 5, its front the side along the
/// bottom, at y = −5√3: the top one, the front two, and the walls beside the
/// front three on its right and seven on its left, each meeting it at 120
/// degrees.
fn hexagonal_prism() -> Body {
    let half = 5.0 * 3.0f64.sqrt();
    raised(
        Contour::straight(vec![
            DVec2::new(-5.0, -half),
            DVec2::new(5.0, -half),
            DVec2::new(10.0, 0.0),
            DVec2::new(5.0, half),
            DVec2::new(-5.0, half),
            DVec2::new(-10.0, 0.0),
        ]),
        5.0,
    )
}

#[test]
fn a_fillet_on_a_top_edge_of_a_hexagonal_prism_runs_on_past_both_its_ends_to_the_walls_beside_it() {
    let (half, root) = (5.0 * 3.0f64.sqrt(), 3.0f64.sqrt());
    let (radius, right, left) = (1.0, 3, 7);
    let part = hexagonal_prism();
    let edge = edge_between(&part, TOP, FRONT);

    let filleted = part
        .fillet(&edge, radius)
        .unwrap_or_else(|declined| panic!("declined, {declined:?}"));

    let what = "1 asked on the top edge of a hexagon's front 10 wide";
    taken_whole(&filleted, radius, what);
    let body = &filleted.body;
    let past = 5.0 + radius / root;
    let round = Round {
        start: DVec3::new(-past, radius - half, 5.0 - radius),
        along: DVec3::X,
        length: 2.0 * past,
        radius,
        facing: [DVec3::Z, -DVec3::Y],
    };
    let hexagon = 1.5 * root * 10.0 * 10.0 * 5.0;
    let rounded_off = 10.0 * radius * radius * (1.0 - PI / 4.0)
        + 2.0 / root * radius.powi(3) * (5.0 / 6.0 - PI / 4.0);
    sound_drawn_within(
        body,
        hexagon - rounded_off,
        drawn_within(2.0 * 10.0, round.area()),
        what,
    );
    let fillet = one_round_more(&part, body, what);
    is_the_round(
        body,
        fillet,
        &round,
        "the round, from the wall on the front's left to the one on its right",
    );
    assert_eq!(
        sides_met(body, fillet),
        [TOP, FRONT, right, left].map(|side| vec![side as u32]),
        "the round meets the top and the front along its sides, and the walls beside the front \
         at its ends, by one edge each"
    );
    corners_are(
        body,
        TOP,
        &[
            DVec3::new(-past, radius - half, 5.0),
            DVec3::new(past, radius - half, 5.0),
            DVec3::new(10.0, 0.0, 5.0),
            DVec3::new(5.0, half, 5.0),
            DVec3::new(-5.0, half, 5.0),
            DVec3::new(-10.0, 0.0, 5.0),
        ],
        "the top, its line along the round longer than the edge",
    );
    corners_are(
        body,
        FRONT,
        &[
            DVec3::new(-5.0, -half, 0.0),
            DVec3::new(5.0, -half, 0.0),
            DVec3::new(5.0, -half, 5.0 - radius),
            DVec3::new(-5.0, -half, 5.0 - radius),
        ],
        "the front, its line along the round the front's width",
    );
    for (wall, side) in [(right, 1.0), (left, -1.0)] {
        let at = |x: f64, y: f64, z: f64| DVec3::new(side * x, y, z);
        trimmed_by_the_round(
            body,
            wall,
            &[
                at(5.0, -half, 0.0),
                at(10.0, 0.0, 0.0),
                at(10.0, 0.0, 5.0),
                at(past, radius - half, 5.0),
                at(5.0, -half, 5.0 - radius),
            ],
            &round,
            &format!("wall {wall}, beside the front, the round's end on it an arc of an ellipse"),
        );
    }
}

/// A right trapezoid raised by `height`: its front 40 long on the X axis, its
/// back 34 long at Y = 8, both from the Y axis, and between their right ends
/// a slanted wall 10 long, 6 back and 8 across, facing (0.8, 0.6). Its walls
/// are numbered from the front round: the front two, the slant three, the
/// back four and the left five.
fn trapezoid(height: f64) -> Body {
    raised(
        Contour::straight(vec![
            DVec2::ZERO,
            DVec2::new(40.0, 0.0),
            DVec2::new(34.0, 8.0),
            DVec2::new(0.0, 8.0),
        ]),
        height,
    )
}

#[test]
fn a_fillet_on_the_trapezoid_s_slanted_top_edge_is_trimmed_by_the_front_and_the_back_it_meets_at_its_ends()
 {
    let (height, radius) = (5.0, 1.0);
    let part = trapezoid(height);
    let edge = edge_between(&part, TOP, SLANT);

    let filleted = part
        .fillet(&edge, radius)
        .unwrap_or_else(|declined| panic!("declined, {declined:?}"));

    let what = "1 asked on the top edge of the slant, which meets the back at 127 degrees";
    taken_whole(&filleted, radius, what);
    let body = &filleted.body;
    let (slant, up_the_slant) = (DVec3::new(0.8, 0.6, 0.0), DVec3::new(-0.6, 0.8, 0.0));
    let start = DVec3::new(40.0, 0.0, height) - (slant + DVec3::Z) * radius;
    let round = Round {
        start,
        along: up_the_slant,
        length: (8.0 - start.y) / up_the_slant.y,
        radius,
        facing: [DVec3::Z, slant],
    };
    let trapezoid = (40.0 + 34.0) / 2.0 * 8.0 * height;
    let rounded_off = 10.0 * radius * radius * (1.0 - PI / 4.0);
    sound_drawn_within(
        body,
        trapezoid - rounded_off,
        drawn_within(2.0 * 40.0, round.area()),
        what,
    );
    let fillet = one_round_more(&part, body, what);
    is_the_round(
        body,
        fillet,
        &round,
        "the round, its axis the radius under the top and the radius inside the slant",
    );
    assert_eq!(
        sides_met(body, fillet),
        [TOP, FRONT, SLANT, BACK].map(|side| vec![side as u32]),
        "the round meets the top and the slant along its sides, and the front and the back at \
         its ends, by one edge each"
    );
    let on_top = [
        DVec3::new(40.0 - 1.25 * radius, 0.0, height),
        DVec3::new(34.0 - 1.25 * radius, 8.0, height),
    ];
    let on_slant = [
        DVec3::new(40.0, 0.0, height - radius),
        DVec3::new(34.0, 8.0, height - radius),
    ];
    corners_are(
        body,
        TOP,
        &[
            DVec3::new(0.0, 0.0, height),
            on_top[0],
            on_top[1],
            DVec3::new(0.0, 8.0, height),
        ],
        "the top, its line along the round stopping short of the front's end and running on \
         past the slant's end along the back",
    );
    corners_are(
        body,
        SLANT,
        &[
            DVec3::new(40.0, 0.0, 0.0),
            DVec3::new(34.0, 8.0, 0.0),
            on_slant[1],
            on_slant[0],
        ],
        "the slant, set back by the radius under the top",
    );
    for (wall, y, far, end) in [(FRONT, 0.0, 40.0, 0), (BACK, 8.0, 34.0, 1)] {
        trimmed_by_the_round(
            body,
            wall,
            &[
                DVec3::new(0.0, y, 0.0),
                DVec3::new(far, y, 0.0),
                DVec3::new(0.0, y, height),
                on_top[end],
                on_slant[end],
            ],
            &round,
            &format!("wall {wall}, the round's end on it an arc of an ellipse"),
        );
    }
}

/// A slot 4 wide raised 20: a front 28 long on the X axis, a round end of
/// half a turn about `(28 + off, 2)` from the front's end, and a straight back
/// to the Y axis. Off by nought, the round leaves the front tangentially;
/// moved off, it leaves it at a corner of about `off / 2` radians, outwards
/// when `off` is positive and inwards when it is negative. Its top one, its
/// front two, its round three, its back four and its left five.
fn slot_off_tangent_by(off: f64) -> Body {
    let (end, centre) = (DVec2::new(28.0, 0.0), DVec2::new(28.0 + off, 2.0));
    raised(
        Contour {
            corners: vec![DVec2::ZERO, end, 2.0 * centre - end, DVec2::new(0.0, 4.0)],
            runs: vec![
                Run::Straight,
                Run::Round {
                    center: centre,
                    turn: PI,
                },
                Run::Straight,
                Run::Straight,
            ],
        },
        20.0,
    )
}

/// The slot's own matter: the quadrilateral from the front to the round's
/// chord, the half disc beyond the chord, raised 20.
fn slot_volume(off: f64) -> f64 {
    let radius_squared = 4.0 + off * off;
    ((56.0 + 2.0 * off) / 2.0 * 4.0 + PI * radius_squared / 2.0) * 20.0
}

/// How many steps [`slot_rounded_off`] is summed over: the slot's round,
/// read along the fillet's, has its nearest singularity some 0.07 off the
/// quarter turn of the sum, and Simpson's rule over this many steps comes
/// within a millionth of the body's tolerance of it.
const STEPS: usize = 1 << 16;

/// The matter a fillet of 1 on the slot's front takes, trimmed by the slot's
/// own walls: over every place of the slot's plan less than 1 back from the
/// front's line, the height the round stands under the top, 1 − √(1 − (1 −
/// y)²). Up to 1 back, the slot runs from the Y axis to its round,
/// `x = c + √(r² − (y − 2)²)`. Read with `y = 1 − cos θ`, under which the
/// height is `1 − sin θ` and the product has no closed form, so it is summed
/// by Simpson's rule.
fn slot_rounded_off(off: f64) -> f64 {
    let (centre, radius) = (28.0 + off, (4.0 + off * off).sqrt());
    let reach = |y: f64| centre + (radius * radius - (y - 2.0).powi(2)).max(0.0).sqrt();
    let at = |theta: f64| {
        let y = 1.0 - f64::cos(theta);
        reach(y) * (1.0 - theta.sin()) * theta.sin()
    };
    let step = PI / 2.0 / STEPS as f64;
    let inner: f64 = (1..STEPS)
        .map(|index| at(step * index as f64) * if index % 2 == 1 { 4.0 } else { 2.0 })
        .sum();
    (at(0.0) + inner + at(PI / 2.0)) * step / 3.0
}

#[test]
fn a_fillet_on_a_slot_s_front_runs_on_over_a_round_meeting_it_at_a_few_degrees_trimmed_by_the_round_s_wall()
 {
    let (radius, round_end) = (1.0, 3);
    let off = -0.1;
    let what = format!(
        "the round turning in from the front's end at {:.1} degrees",
        (-off / 2.0f64).atan().to_degrees()
    );
    let slot = slot_off_tangent_by(off);
    let edge = edge_between(&slot, TOP, FRONT);

    let filleted = slot
        .fillet(&edge, radius)
        .unwrap_or_else(|declined| panic!("{what}: declined, {declined:?}"));

    taken_whole(&filleted, radius, &what);
    let body = &filleted.body;
    let (centre, wall) = (DVec2::new(28.0 + off, 2.0), (4.0 + off * off).sqrt());
    let reach = |y: f64| centre.x + (wall * wall - (y - 2.0).powi(2)).sqrt();
    let round = Round {
        start: DVec3::new(0.0, radius, 20.0 - radius),
        along: DVec3::X,
        length: reach(radius),
        radius,
        facing: [DVec3::Z, -DVec3::Y],
    };
    sound_drawn_within(
        body,
        slot_volume(off) - slot_rounded_off(off),
        drawn_within(2.0 * 31.0, round.area() + PI * wall * 20.0),
        &what,
    );
    let fillet = one_round_more(&slot, body, &what);
    is_the_round(body, fillet, &round, &what);
    let drawn: Vec<DVec3> = body.triangles_of(fillet).flatten().collect();
    for corner in &drawn {
        assert!(
            corner.x <= centre.x || corner.truncate().distance(centre) <= wall + PLACE,
            "{what}: the round is drawn through {corner}, past the slot's round wall"
        );
    }
    let furthest = drawn.iter().map(|corner| corner.x).fold(0.0, f64::max);
    assert!(
        (furthest - reach(radius)).abs() < PLACE,
        "{what}: the round runs on to {furthest}, where its side on the top meets the slot's \
         round wall at {}",
        reach(radius)
    );
    assert_eq!(
        sides_met(body, fillet),
        [TOP, FRONT, round_end, LEFT].map(|side| vec![side as u32]),
        "{what}: the round meets the top and the front along its sides, and the left and \
         the slot's round wall at its ends, by one edge each"
    );
    corners_are(
        body,
        FRONT,
        &[
            DVec3::ZERO,
            DVec3::new(28.0, 0.0, 0.0),
            DVec3::new(28.0, 0.0, 20.0 - radius),
            DVec3::new(0.0, 0.0, 20.0 - radius),
        ],
        &format!("the front, {what}"),
    );
    corners_are_or_on(
        body,
        TOP,
        &[
            DVec3::new(0.0, radius, 20.0),
            DVec3::new(reach(radius), radius, 20.0),
            DVec3::new(28.0 + 2.0 * off, 4.0, 20.0),
            DVec3::new(0.0, 4.0, 20.0),
        ],
        |corner| {
            (corner.truncate().distance(centre) - wall).abs() < PLACE && corner.y >= radius - PLACE
        },
        &format!("the top, {what}, its line along the round running on to the round wall"),
    );
    let kept = [
        DVec3::ZERO,
        DVec3::new(0.0, 4.0, 0.0),
        DVec3::new(0.0, 4.0, 20.0),
    ];
    trimmed_by_the_round(
        body,
        LEFT,
        &[&kept[..], &round.sides_at(0.0)].concat(),
        &round,
        &format!("the left, {what}"),
    );
}

#[test]
fn a_fillet_on_a_slot_s_front_whose_round_end_bulges_out_past_the_front_is_declined_as_unsupported()
{
    let slot = slot_off_tangent_by(0.1);
    let edge = edge_between(&slot, TOP, FRONT);

    let declined = slot.fillet(&edge, 1.0).err();

    assert_eq!(
        declined,
        Some(Declined::Unsupported),
        "the round leaves the front's end 2.9 degrees outwards and stands 0.0025 past its line \
         from 28 to 28.2: the matter there above the round's side is neither the round nor the \
         front, and no one cylinder takes it"
    );
}
