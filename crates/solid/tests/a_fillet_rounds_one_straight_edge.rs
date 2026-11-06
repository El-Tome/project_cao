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
//!   `a_fillet_on_a_slot_s_front_runs_on_over_a_round_meeting_it_at_a_few_degrees_trimmed_by_the_round_s_wall`
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

use cao_solid::profile::{Contour, Frame, Profile};
use cao_solid::soundness::{closed, enclosed, uncrossed};
use cao_solid::{Body, EdgeName, Filleted, Loop};
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
/// one, and its sides from the front round to the left, two to five.
const TOP: usize = 1;
const FRONT: usize = 2;
const RIGHT: usize = 3;
const LEFT: usize = 5;

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
    let plane = body
        .plane_of(face)
        .unwrap_or_else(|| panic!("{what}: the body offers no plane on face {face}"));
    for corner in &plane.corners {
        assert!(
            expected.iter().any(|point| point.distance(*corner) < PLACE),
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

    /// What the triangles of a body whose one curved face is this round may
    /// enclose apart from its matter: the drawing's tolerance at the reach the
    /// body carries, over the round's area. That reach is taken at `reach`,
    /// twice the part's own, for whatever the round is cut with reaching
    /// past the part.
    fn drawn_within(&self, reach: f64) -> f64 {
        let [first, second] = self.facing;
        DRAWN * reach * first.dot(second).acos() * self.radius * self.length
    }
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

/// Holds that a wall at one end of the edge, `end` of the way along it, is
/// trimmed by the round: its corners the points `kept` and the two where the
/// round's sides meet it, and every other corner on the round's end, an arc of
/// its radius.
fn trimmed_by_the_round(
    body: &Body,
    wall: usize,
    kept: &[DVec3],
    round: &Round,
    end: f64,
    what: &str,
) {
    let plane = body
        .plane_of(wall)
        .unwrap_or_else(|| panic!("{what}: the body offers no plane on face {wall}"));
    let at = round.axis_at(end);
    let sides = [0.0, 1.0].map(|share| at + round.facing_at(share) * round.radius);
    let expected: Vec<DVec3> = kept.iter().copied().chain(sides).collect();
    for corner in &plane.corners {
        let on_the_end = round.holds(*corner) && (*corner - at).dot(round.along).abs() < PLACE;
        assert!(
            on_the_end || expected.iter().any(|point| point.distance(*corner) < PLACE),
            "{what}: a corner at {corner}, neither on the round's end nor among {expected:?}"
        );
    }
    for point in &expected {
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
        round.drawn_within(2.0 * length),
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
        trimmed_by_the_round(
            body,
            wall,
            &[
                DVec3::new(x, 0.0, 0.0),
                DVec3::new(x, breadth, 0.0),
                DVec3::new(x, breadth, height),
            ],
            &round,
            end,
            &format!("the wall at x = {x}, the round's end on it"),
        );
    }
}
