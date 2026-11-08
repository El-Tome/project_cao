//! A chamfer and a fillet fill a concave straight edge of an exact body in:
//! where two planar faces meet inwards — the inner corner of an L, the foot of
//! a rib standing on a plate — the steps that cut a convex edge off (#696,
//! #697) join matter in instead. The chamfer lays a flat face set back by its
//! distance along each of the two faces; the fillet a cylinder of its radius
//! tangent to both, set back along each by r / tan(α / 2), α the angle between
//! them outside the matter — the radius itself where they meet square. The new
//! face's two sides are edges of the two faces, its two ends edges of the
//! faces at the edge's ends, and a value the faces cannot give is reduced to
//! the largest they can.
//!
//! Closes #778.
//! - A chamfer and a fillet given the name of the inner edge of an L, between
//!   two planar faces, and a value, leave a closed body with one more face: a
//!   flat set back by the distance along each face, or a cylinder of the
//!   radius tangent to both. The faces at the ends hold its ends, and the body
//!   passes the soundness checks —
//!   `a_chamfer_on_the_inner_corner_of_an_l_fills_it_with_one_flat_face_set_back_by_the_distance_along_each_face`,
//!   `a_fillet_on_the_inner_corner_of_an_l_fills_it_with_one_round_of_the_radius_tangent_to_both_faces`
//! - The same holds on a concave edge whose faces meet at an angle other than
//!   a right one —
//!   `a_chamfer_on_a_concave_edge_whose_faces_meet_at_a_slant_is_set_back_by_the_distance_along_each_face`,
//!   `a_fillet_on_a_concave_edge_whose_faces_meet_at_a_slant_is_set_back_by_the_radius_over_the_tangent_of_half_the_angle`
//! - The same holds where the edge ends on an oblique wall, and where a rib
//!   ends short of its plate's edge (the fill trimmed flush by the rib's end) —
//!   `a_chamfer_and_a_fillet_on_a_concave_edge_ending_on_an_oblique_wall_are_trimmed_flush_by_it`,
//!   `a_chamfer_and_a_fillet_at_the_foot_of_a_rib_ending_short_of_its_plate_s_edge_are_trimmed_flush_by_the_rib_s_end`
//! - A value larger than the neighbouring faces can give is reduced to the
//!   largest that fits, and the step reports the value asked and the value
//!   taken —
//!   `a_chamfer_on_a_concave_edge_asked_more_than_a_face_can_give_takes_the_whole_face_and_reports_both_values`,
//!   `a_fillet_on_a_concave_edge_asked_more_than_a_face_can_give_takes_the_radius_whose_set_back_is_the_whole_face_and_reports_both_radii`
//! - After a change of size, a move or a turn upstream, the step fills the
//!   same edge, found by its name —
//!   `a_chamfer_and_a_fillet_fill_the_concave_edge_their_name_finds_again_after_a_change_of_size_a_move_and_a_turn`
//! - An edge its name no longer finds, gone or split, is declined `Lost` with
//!   which —
//!   `a_chamfer_and_a_fillet_on_a_concave_edge_a_cut_took_away_are_declined_saying_it_is_gone`,
//!   `a_chamfer_and_a_fillet_on_a_concave_edge_a_cut_parted_in_two_are_declined_saying_it_is_split`
//! - The face numbers of every existing part are unchanged; the new face takes
//!   the number after the steps' last one —
//!   `a_chamfer_and_a_fillet_on_a_concave_edge_leave_every_face_of_the_l_its_number_and_number_theirs_after_the_steps_last`;
//!   the parts with neither a chamfer nor a fillet keep theirs by
//!   `faces_keep_their_numbers.rs`, left as it was
//! - The exact STEP export of a filled L writes its new face on its true
//!   surface —
//!   `an_l_filled_at_its_inner_corner_written_to_step_writes_its_new_face_on_its_true_surface`
//! - The convex chamfer's and fillet's tests still pass — no test: the convex
//!   suites, `a_chamfer_cuts_one_straight_edge.rs`,
//!   `a_fillet_rounds_one_straight_edge.rs`, `src/body/tests/chamfer.rs` and
//!   `src/body/tests/fillet.rs`, are the answer, left as they were but for the
//!   two tests that declined the inner corner of an L, which this issue turns
//!   round and this file now speaks for

use std::collections::BTreeSet;
use std::f64::consts::PI;

use cao_solid::profile::{Contour, Frame, Profile};
use cao_solid::soundness::{closed, enclosed, uncrossed};
use cao_solid::{Body, EdgeName, Loop};
use glam::{DVec2, DVec3};

/// How far a corner may stand from where the arithmetic puts it, on a plane
/// or on the round: each is placed exactly, so what is left is rounding.
const PLACE: f64 = 1e-9;

/// How near the arithmetic an exact volume comes, as a fraction of it.
const EXACT: f64 = 1e-9;

/// How far the triangles may stand from a round, as a share of the reach the
/// body carries: the drawing's tolerance.
const DRAWN: f64 = 2e-4;

/// The numbers a raised part's floor and top answer to.
const FLOOR: usize = 0;
const TOP: usize = 1;

/// The numbers an L's arm's back and its stem's right side answer to: the
/// two faces that meet inwards at its inner corner.
const ARM_BACK: usize = 4;
const STEM_RIGHT: usize = 5;

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

/// An outline drawn on a frame raised along its normal by `height`, and
/// joined as `cao_part` builds a step: its floor numbered nought, its top
/// one, and its walls from two on, one for each run of the outline.
fn raised_on(frame: Frame, outline: Contour, height: f64) -> Body {
    let part = Body::default();
    let tool = part
        .tool_raised(&profile(outline), frame, frame.normal() * height)
        .expect("the tool raises");
    part.union(&tool).expect("the part is built")
}

/// The two corners an upright edge of a part raised from the ground stands
/// between, at the point of its outline `at`.
fn upright(at: DVec2, height: f64) -> [DVec3; 2] {
    [at.extend(0.0), at.extend(height)]
}

/// The corners of a part's floor or top, the points of its outline lifted to
/// `height`.
fn lifted_to(outline: &[DVec2], height: f64) -> Vec<DVec3> {
    outline.iter().map(|at| at.extend(height)).collect()
}

/// An L lying on the ground: an arm `length` long and `depth` deep along X,
/// and at the arm's left end a stem `stem` wide reaching back to `back`,
/// raised by `height`. Its walls are numbered from the front round: the front
/// two, the arm's right end three, the arm's back four, the stem's right side
/// five, the stem's back six and the left seven. The arm's back and the
/// stem's right side meet inwards, at its inner corner.
#[derive(Clone, Copy)]
struct Ell {
    length: f64,
    depth: f64,
    stem: f64,
    back: f64,
    height: f64,
}

const ELL: Ell = Ell {
    length: 30.0,
    depth: 6.0,
    stem: 12.0,
    back: 14.0,
    height: 10.0,
};

impl Ell {
    /// The L's outline, from the front's left end round, its inner corner
    /// fourth.
    fn outline(&self) -> Vec<DVec2> {
        let Ell {
            length,
            depth,
            stem,
            back,
            ..
        } = *self;
        vec![
            DVec2::ZERO,
            DVec2::new(length, 0.0),
            DVec2::new(length, depth),
            DVec2::new(stem, depth),
            DVec2::new(stem, back),
            DVec2::new(0.0, back),
        ]
    }

    /// The L's outline with its inner corner given up for the two points
    /// `set_back` along the arm's back and along the stem's right side.
    fn outline_filled(&self, set_back: [DVec2; 2]) -> Vec<DVec2> {
        let mut outline = self.outline();
        outline.splice(3..4, set_back);
        outline
    }

    fn on(&self, frame: Frame) -> Body {
        raised_on(frame, Contour::straight(self.outline()), self.height)
    }

    fn volume(&self) -> f64 {
        (self.length * self.depth + self.stem * (self.back - self.depth)) * self.height
    }
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

/// Holds that the faces numbered `first` and `second` meet along no edge.
fn no_longer_meet(body: &Body, first: usize, second: usize, what: &str) {
    let (first, second) = (vec![first as u32], vec![second as u32]);
    let meeting: Vec<EdgeName> = body
        .edge_names()
        .into_iter()
        .filter(|name| name.between.contains(&first) && name.between.contains(&second))
        .collect();
    assert!(meeting.is_empty(), "{what}: {meeting:?}");
}

/// Every number some face of the body answers to: a step's, below where the
/// steps' numbers end, and a piece's, past them.
fn numbers(body: &Body) -> BTreeSet<usize> {
    (0..body.faces_end())
        .chain(body.pieces())
        .filter(|&number| body.has_face(number))
        .collect()
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

/// Holds that a body is exact and keeps the rules every solid must: its
/// surface closed, no face through another, and the matter its triangles
/// enclose, as the matter it holds, what arithmetic promised.
fn sound(body: &Body, promised: f64, what: &str) {
    sound_drawn_within(body, promised, EXACT * promised, what);
}

/// Holds what [`sound`] holds of a body with curved faces, whose triangles
/// stand within the drawing's tolerance of them: the matter they enclose
/// within `drawn` of what arithmetic promised, the matter the body holds
/// exactly that.
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

/// Holds that nothing of the body stands past the box of the part it was
/// made from: a fill held flush by the faces at its edge's ends.
fn within_the_box_of(part: &Body, body: &Body, what: &str) {
    let (low, high) = part.bounds().expect("a part has a box");
    let (reached_low, reached_high) = body.bounds().expect("a part has a box");
    assert!(
        reached_low.distance(low) < PLACE && reached_high.distance(high) < PLACE,
        "{what}: the body reaches from {reached_low} to {reached_high}, where the part it was \
         made from reached from {low} to {high}"
    );
}

/// Holds that a step was given the value it was asked.
fn taken_whole(asked: f64, taken: f64, value: f64, what: &str) {
    assert!(
        (asked - value).abs() < PLACE && (taken - value).abs() < PLACE,
        "{what}: asked {asked} and took {taken}, where {value} fits"
    );
}

/// Holds that a step left every face of the part it was given and made one
/// face more, and gives back the number that face answers to.
fn one_face_more(part: &Body, body: &Body, what: &str) -> usize {
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
    made[0]
}

/// Holds what [`one_face_more`] holds, the face made flat.
fn one_flat_face_more(part: &Body, body: &Body, what: &str) -> usize {
    let made = one_face_more(part, body, what);
    assert!(
        body.is_flat(made),
        "{what}: the face the chamfer lays is flat"
    );
    made
}

/// Holds what [`one_face_more`] holds, the face made curved.
fn one_round_more(part: &Body, body: &Body, what: &str) -> usize {
    let made = one_face_more(part, body, what);
    assert!(
        !body.is_flat(made) && body.plane_of(made).is_none(),
        "{what}: the face the fillet lays is curved, and offers no plane to draw on"
    );
    made
}

/// Holds that a flat face faces the way `toward` points.
fn faces_toward(body: &Body, face: usize, toward: DVec3, what: &str) {
    let facing = body
        .plane_of(face)
        .expect("a flat face offers a plane")
        .normal;
    assert!(
        facing.distance(toward.normalize()) < PLACE,
        "{what}: the chamfer faces {facing}, where it faces {}",
        toward.normalize()
    );
}

/// The largest coordinate a body reaches, on either side of the origin.
fn reach_of(body: &Body) -> f64 {
    let (low, high) = body.bounds().expect("a part has a box");
    low.abs().max(high.abs()).max_element()
}

/// What the triangles of the body a step left on `part` may enclose apart
/// from its matter, its curved faces `area` in all: the drawing's tolerance
/// at the part's own reach, over them.
fn drawn_within(part: &Body, area: f64) -> f64 {
    DRAWN * reach_of(part) * area
}

/// The round a fillet lays in a concave corner: a cylinder of `radius` about
/// the line from `start` along `along` for `length`, that line standing in
/// the air the two faces leave between them. The round turns from facing the
/// way the edge's first face does to facing the way its second does, and
/// everywhere faces its axis, a radius away against the way it faces. Facing
/// each face's way where it meets it is what tangent to both comes to.
struct Hollow {
    start: DVec3,
    along: DVec3,
    length: f64,
    radius: f64,
    facing: [DVec3; 2],
}

impl Hollow {
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
        let toward_the_axis = self.along * along - off;
        let [first, second] = self.facing;
        let least = self.radius * first.dot(second);
        (toward_the_axis.length() - self.radius).abs() < PLACE
            && (-PLACE..=self.length + PLACE).contains(&along)
            && toward_the_axis.dot(first) >= least - PLACE
            && toward_the_axis.dot(second) >= least - PLACE
    }

    /// The round's area, its turn times its radius times its length.
    fn area(&self) -> f64 {
        let [first, second] = self.facing;
        first.dot(second).acos() * self.radius * self.length
    }
}

/// Holds that the face numbered `face` is the hollow round expected: every
/// corner of its triangles on it, and every ray aimed from the air beyond its
/// axis back across it meeting it first, where it faces back along the ray.
fn is_the_hollow(body: &Body, face: usize, hollow: &Hollow, what: &str) {
    let corners: Vec<DVec3> = body.triangles_of(face).flatten().collect();
    assert!(!corners.is_empty(), "{what}: the round is drawn");
    for corner in corners {
        assert!(
            hollow.holds(corner),
            "{what}: the round is drawn through {corner}, off the cylinder of {} about the line \
             from {} along {}",
            hollow.radius,
            hollow.start,
            hollow.along
        );
    }
    for along in [0.11, 0.47, 0.83] {
        for share in [0.1, 0.3, 0.5, 0.7, 0.9] {
            let facing = hollow.facing_at(share);
            let from = hollow.axis_at(along) + facing * 10.0;
            let hit = body.ray_hit(from, -facing).unwrap_or_else(|| {
                panic!("{what}: a ray from {from} across the axis meets nothing")
            });
            assert_eq!(
                hit.face, face,
                "{what}: a ray from {from} across the axis meets face {} before the round",
                hit.face
            );
            assert!(
                hit.normal.distance(facing) < PLACE,
                "{what}: where a ray from {from} meets it, the round faces {}, not {facing}, \
                 toward its axis",
                hit.normal
            );
        }
    }
}

#[test]
fn a_chamfer_on_the_inner_corner_of_an_l_fills_it_with_one_flat_face_set_back_by_the_distance_along_each_face()
 {
    let distance = 3.0;
    let ell = ELL.on(ground());
    let inner = edge_between(&ell, ARM_BACK, STEM_RIGHT);

    let chamfered = ell
        .chamfer(&inner, distance)
        .unwrap_or_else(|declined| panic!("the inner corner of the L: declined, {declined:?}"));

    let what = "the inner corner of the L";
    taken_whole(chamfered.asked, chamfered.taken, distance, what);
    let body = &chamfered.body;
    let Ell {
        length,
        depth,
        stem,
        back,
        height,
    } = ELL;
    sound(
        body,
        ELL.volume() + distance * distance / 2.0 * height,
        what,
    );
    let chamfer = one_flat_face_more(&ell, body, what);
    faces_toward(
        body,
        chamfer,
        DVec3::new(1.0, 1.0, 0.0),
        "the chamfer, halfway between the arm's back and the stem's right side, out of the matter",
    );
    let on_arm_back = DVec2::new(stem + distance, depth);
    let on_stem_right = DVec2::new(stem, depth + distance);
    corners_are(
        body,
        chamfer,
        &[upright(on_arm_back, height), upright(on_stem_right, height)].concat(),
        "the chamfer, set back by the distance along the arm's back and the stem's right side",
    );
    corners_are(
        body,
        ARM_BACK,
        &[
            upright(on_arm_back, height),
            upright(DVec2::new(length, depth), height),
        ]
        .concat(),
        "what is left of the arm's back",
    );
    corners_are(
        body,
        STEM_RIGHT,
        &[
            upright(on_stem_right, height),
            upright(DVec2::new(stem, back), height),
        ]
        .concat(),
        "what is left of the stem's right side",
    );
    let filled = ELL.outline_filled([on_arm_back, on_stem_right]);
    for (cap, at, whose) in [(FLOOR, 0.0, "the floor"), (TOP, height, "the top")] {
        corners_are(
            body,
            cap,
            &lifted_to(&filled, at),
            &format!("{whose}, holding the chamfer's end flush, the inner corner gone"),
        );
    }
    assert_eq!(
        sides_met(body, chamfer),
        [FLOOR, TOP, ARM_BACK, STEM_RIGHT].map(|side| vec![side as u32]),
        "the chamfer meets the arm's back and the stem's right side along its sides, and the \
         floor and the top at its ends, by one edge each"
    );
    no_longer_meet(
        body,
        ARM_BACK,
        STEM_RIGHT,
        "the arm's back and the stem's right side no longer meet: the chamfer stands between them",
    );
    within_the_box_of(&ell, body, what);
}

#[test]
fn a_fillet_on_the_inner_corner_of_an_l_fills_it_with_one_round_of_the_radius_tangent_to_both_faces()
 {
    let radius = 3.0;
    let ell = ELL.on(ground());
    let inner = edge_between(&ell, ARM_BACK, STEM_RIGHT);

    let filleted = ell
        .fillet(&inner, radius)
        .unwrap_or_else(|declined| panic!("the inner corner of the L: declined, {declined:?}"));

    let what = "the inner corner of the L";
    taken_whole(filleted.asked, filleted.taken, radius, what);
    let body = &filleted.body;
    let Ell {
        length,
        depth,
        stem,
        back,
        height,
    } = ELL;
    let hollow = Hollow {
        start: DVec3::new(stem + radius, depth + radius, 0.0),
        along: DVec3::Z,
        length: height,
        radius,
        facing: [DVec3::Y, DVec3::X],
    };
    sound_drawn_within(
        body,
        ELL.volume() + radius * radius * (1.0 - PI / 4.0) * height,
        drawn_within(&ell, hollow.area()),
        what,
    );
    let fillet = one_round_more(&ell, body, what);
    is_the_hollow(
        body,
        fillet,
        &hollow,
        "the round, its axis the radius out from the arm's back and from the stem's right side",
    );
    let on_arm_back = DVec2::new(stem + radius, depth);
    let on_stem_right = DVec2::new(stem, depth + radius);
    corners_are(
        body,
        ARM_BACK,
        &[
            upright(on_arm_back, height),
            upright(DVec2::new(length, depth), height),
        ]
        .concat(),
        "what is left of the arm's back, set back by the radius",
    );
    corners_are(
        body,
        STEM_RIGHT,
        &[
            upright(on_stem_right, height),
            upright(DVec2::new(stem, back), height),
        ]
        .concat(),
        "what is left of the stem's right side, set back by the radius",
    );
    let filled = ELL.outline_filled([on_arm_back, on_stem_right]);
    for (cap, at, whose) in [(FLOOR, 0.0, "the floor"), (TOP, height, "the top")] {
        corners_are_or_on(
            body,
            cap,
            &lifted_to(&filled, at),
            |corner| hollow.holds(corner),
            &format!("{whose}, holding the round's end flush, the inner corner gone"),
        );
    }
    assert_eq!(
        sides_met(body, fillet),
        [FLOOR, TOP, ARM_BACK, STEM_RIGHT].map(|side| vec![side as u32]),
        "the round meets the arm's back and the stem's right side along its sides, and the floor \
         and the top at its ends, by one edge each"
    );
    no_longer_meet(
        body,
        ARM_BACK,
        STEM_RIGHT,
        "the arm's back and the stem's right side no longer meet: the round stands between them",
    );
    within_the_box_of(&ell, body, what);
}
