//! A full round meeting the round of an upright at its end. The block is 30
//! long, 6 wide and 10 high, standing on the ground's origin, raised as
//! `cao_part` builds a step: its floor numbered nought, its top one, then its
//! front, along y = 0, two; its end, along x = 30, three; its back four; and
//! its left end five. Its top's front and back edges, rounded at half the
//! width, meet across the top in one full round, as #698 lays it: a half
//! cylinder of radius 3 about the line y = 3, z = 7. The upright where the
//! front meets the end, named with them and rounded at the same radius, is a
//! quarter cylinder about the vertical through (27, 3), standing from the
//! floor. The two axes cross at (27, 3, 7), and the two cylinders, of one
//! radius, meet there on a planar ellipse, as #803 mitres a full round against
//! the round of the edge across its end: on the plane z − 7 = x − 27, from
//! where the full round leaves the front, (27, 0, 7), up to the crown,
//! (30, 3, 10). An operation of the kernel on [`Body`], as #803's mitre is; the
//! step of the history that calls it comes with the application, in #701.
//!
//! Closes #837 — its first slice only, the mitres. Its second slice, the
//! spheres, and its third, the pointed end's tip, come on branches of their
//! own, each with its own criteria; the issue stays open until the last one
//! lands.
//! - The block 30 × 6 × 10 filleted at 3 on the two long top edges and the
//!   front-right upright is built on the exact kernel, the value taken whole,
//!   one body in every order. The full round is a half cylinder of radius 3
//!   along y = 3, z = 7; the upright's round a quarter cylinder of radius 3
//!   about the vertical through (27, 3), standing from the floor. They meet on
//!   the plane z − 7 = x − 27, from (27, 0, 7) to (30, 3, 10). The end face
//!   keeps its back half, under the quarter circle the full round leaves on
//!   it, its top edge left sharp; the back upright stays sharp. The body holds
//!   the volume arithmetic gives; its triangles are closed, uncrossed and
//!   enclose it within the drawing's tolerance —
//!   `a_fillet_on_a_full_round_and_the_upright_at_its_end_s_front_corner_mitres_the_upright_s_round_against_the_full_round_on_an_ellipse_whatever_the_order`.
//!   Read as: in each of the six orders, the value taken whole; the body
//!   exact, closed, uncrossed, holding `1215 + 144π`, which the booleans give
//!   too, and its triangles enclosing that within the drawing's tolerance; the
//!   top gone, the full round one face answering to both its edges' ranks and
//!   the upright's round to its own, nothing laid at a corner; the full round
//!   drawn on its half cylinder only and, over the front, never past the
//!   mitre, the upright's round on its quarter cylinder only and never above
//!   the mitre, each met first from outside; both drawn up to (27, 0, 7) and
//!   (30, 3, 10), meeting at one vertex at each, and sharing no point off the
//!   plane; the end face flat, facing along x, its corners (30, 3, 0),
//!   (30, 6, 0), (30, 6, 7) and (30, 3, 10), drawn only under the quarter
//!   circle of radius 3 about (30, 3, 7) and meeting the full round along it
//!   with no face between; the back meeting the end by one edge, its corners
//!   reaching (30, 6, 0) and (30, 6, 7); the front and the floor giving up to
//!   the upright's round what lies outside its quarter circle; every face
//!   meeting the faces it bounds by one edge each; and every order laying the
//!   body the listed order lays, its faces meeting alike, each laid face read
//!   by the edge it was laid on
//! - The same with both uprights: the uprights' rounds one standing half
//!   cylinder about the vertical through (27, 3), mitred against the lying one
//!   on the plane z − 7 = x − 27 from (27, 0, 7) through (30, 3, 10) to
//!   (27, 6, 7); the end face gone; one body in every order; the volume
//!   arithmetic gives —
//!   `a_fillet_on_a_full_round_and_both_uprights_at_its_end_mitres_their_standing_full_round_against_the_lying_one_on_an_ellipse_whatever_the_order`
//! - Throughout, as far as this slice goes: the lists `main` builds keep their
//!   bodies — the chamfers above but the tongue's, the end edge with both
//!   uprights without the pair, and every one of these at 6.5 wide —
//!   `the_lists_main_builds_at_the_end_of_a_block_6_or_6_5_wide_keep_their_bodies_in_every_order`.
//!   Read as: on the block 30 × 6 × 10 at 3, the chamfer on the pair and the
//!   front-right upright, on the pair and both uprights, and on the pair, the
//!   end edge and the front-right upright; the fillet and the chamfer on the
//!   end edge and both uprights; and on the block 30 × 6.5 × 10 at 3, the
//!   fillet and the chamfer on each of the five lists of the issue's table —
//!   each built in every order, as on `main`
//! - Throughout: no body of #698's, #699's, #803's or #843's tests changes —
//!   no test: those tests hold their own bodies, in
//!   `a_chamfer_and_a_fillet_take_several_edges_in_one_step.rs`,
//!   `a_chamfer_and_a_fillet_blend_the_corner_their_edges_meet_at.rs` and
//!   `a_full_round_ends_on_a_pointed_end_mitred_against_each_edge_there.rs`,
//!   and run beside this file. What changes there is a decline, not a body:
//!   in the second file,
//!   `a_fillet_on_a_full_round_meeting_an_upright_or_another_full_round_and_on_a_round_bottom_meeting_the_edge_at_its_end_is_declined_as_unsupported_whatever_the_order`
//!   gives up the list of the pair and the front-right upright, which this
//!   slice builds, and keeps the tongue's for the second slice; in the third,
//!   the decline of the pair, the end edge and the front-right upright stays
//!   until the second slice
//!
//! The volume is arithmetic. The block holds `LWH`, 1800. The full round
//! alone takes, all along the block, the two corners of the top's section
//! outside its half disc, `2r² − πr²/2` each unit, `r²(2 − π/2)L`; the
//! upright's round alone takes, all up the block, the corner of the floor's
//! outline outside its quarter disc, `r²(1 − π/4)H`. What both take, counted
//! twice, lies in the cube of side `r` at the top front right corner: measured
//! from the crossing of the axes, `u = x − 27`, `v = 3 − y`, `w = z − 7`, the
//! cube less what lies inside either cylinder, `u² + v² < r²` or
//! `v² + w² < r²`, `πr³/4` each, inside both `∫ (r² − v²) dv = 2r³/3`:
//! `r³ − πr³/2 + 2r³/3 = r³(5/3 − π/2)`, what #699 gives back at a convex
//! corner. So the body holds
//! `LWH − r²(2 − π/2)L − r²(1 − π/4)H + r³(5/3 − π/2)`, `1215 + 144π`, 1667.39.
//! The two cylinders meet where `(z − 7)² = (x − 27)²`, which on the corner's
//! side is the plane `z − 7 = x − 27`. The booleans give it too: the full
//! round's prism cut from the block whole along its length, then the upright's
//! round's prism, raised whole up its height; each takes what the other takes
//! past the mitre, and the mitre is where their cuts meet.

use std::collections::{BTreeMap, BTreeSet};
use std::f64::consts::PI;

use cao_solid::profile::{Contour, Frame, Profile, Run};
use cao_solid::soundness::{closed, enclosed, uncrossed};
use cao_solid::{Body, Declined, EdgeName, Filleted, Loop};
use glam::{DVec2, DVec3};

/// How far a point may stand from where the arithmetic puts it: each is placed
/// exactly, so what is left is rounding.
const PLACE: f64 = 1e-9;

/// How near the arithmetic an exact volume comes, as a fraction of it.
const EXACT: f64 = 1e-9;

/// How far the triangles may stand from a round, as a share of the reach the
/// body carries: the drawing's tolerance.
const DRAWN: f64 = 2e-4;

/// How far from the mitre a ray aims, on either side of it, to meet the
/// triangles of the round it aims at rather than the other's: well over the
/// drawing's tolerance on the block, [`DRAWN`] times its reach.
const CLEAR_OF_THE_MITRE: f64 = 0.1;

/// The numbers the block's faces answer to, raised as `cao_part` builds a
/// step: its floor, its top, then a wall for each side of its outline — the
/// front along y = 0, the end along x = [`LENGTH`], the back and the left end.
const FLOOR: usize = 0;
const TOP: usize = 1;
const FRONT: usize = 2;
const END: usize = 3;
const BACK: usize = 4;
const LEFT: usize = 5;

/// The block: 30 long, 6 wide and 10 high.
const LENGTH: f64 = 30.0;
const WIDTH: f64 = 6.0;
const HEIGHT: f64 = 10.0;

/// The value asked, half the width: the radius at which the rounds of the
/// top's front and back edges meet across the top in one full round.
const RADIUS: f64 = WIDTH / 2.0;

/// The edges the list names: the full round's pair, then the upright where
/// the front meets the end, each as the two faces it lies between, and what
/// it is called.
const PAIR_AND_FRONT_UPRIGHT: [([usize; 2], &str); 3] = [
    ([TOP, FRONT], "the top's front edge"),
    ([TOP, BACK], "the top's back edge"),
    ([FRONT, END], "the upright at the end's front corner"),
];

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

/// The block [`LENGTH`] by [`WIDTH`] by [`HEIGHT`], drawn on the ground from
/// its origin and raised as `cao_part` builds a step.
fn block() -> Body {
    let part = Body::default();
    let tool = part
        .tool_raised(
            &profile(Contour::rectangle(DVec2::ZERO, DVec2::new(LENGTH, WIDTH))),
            ground(),
            DVec3::Z * HEIGHT,
        )
        .expect("the tool raises");
    part.union(&tool).expect("the part is built")
}

/// The name of the one edge between the faces numbered `first` and `second`,
/// as the body hands it to whoever points at that edge.
fn edge_between(body: &Body, first: usize, second: usize) -> EdgeName {
    let sides = [vec![first as u32], vec![second as u32]];
    let [one, other] = sides.clone();
    let named: Vec<EdgeName> = body
        .edge_names()
        .into_iter()
        .filter(|name| name.between == sides || name.between == [other.clone(), one.clone()])
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

/// Whether every point of `some` stands at a point of `others`, and every
/// point of `others` at one of `some`.
fn same_points(some: &[DVec3], others: &[DVec3]) -> bool {
    let within =
        |points: &[DVec3], point: &DVec3| points.iter().any(|other| other.distance(*point) < PLACE);
    some.iter().all(|point| within(others, point)) && others.iter().all(|point| within(some, point))
}

/// Holds that a face's corners are the points expected: each of them a
/// corner, and no corner anywhere else.
fn corners_are(body: &Body, face: usize, expected: &[DVec3], what: &str) {
    let plane = body
        .plane_of(face)
        .unwrap_or_else(|| panic!("{what}: the body offers no plane on face {face}"));
    assert!(
        same_points(&plane.corners, expected),
        "{what}: face {face} has its corners at {:?}, where they are {expected:?}",
        plane.corners
    );
}

/// Holds that a face one of whose edges runs along an arc has its corners at
/// the points expected and along that arc: each point expected a corner, and
/// every other corner — the points the arc is drawn through — one `on_the_arc`
/// takes.
fn corners_are_along_an_arc(
    body: &Body,
    face: usize,
    expected: &[DVec3],
    on_the_arc: impl Fn(DVec3) -> bool,
    what: &str,
) {
    let plane = body
        .plane_of(face)
        .unwrap_or_else(|| panic!("{what}: the body offers no plane on face {face}"));
    let off: Vec<&DVec3> = plane
        .corners
        .iter()
        .filter(|&&corner| !drawn_through(expected, corner) && !on_the_arc(corner))
        .collect();
    let missing: Vec<&DVec3> = expected
        .iter()
        .filter(|&&point| !drawn_through(&plane.corners, point))
        .collect();
    assert!(
        off.is_empty() && missing.is_empty(),
        "{what}: face {face} has corners at {off:?}, neither one of {expected:?} nor on its \
         arc, and none at {missing:?}"
    );
}

/// Whether a point lies on the circle of `radius` about `center` and within
/// the quarter of it between the ways `from` and `to` point from the centre.
fn on_the_quarter_circle(point: DVec3, center: DVec3, radius: f64, [from, to]: [DVec3; 2]) -> bool {
    let off = point - center;
    (off.length() - radius).abs() < PLACE && off.dot(from) >= -PLACE && off.dot(to) >= -PLACE
}

/// Holds that the face numbered `face` is flat and faces the way `facing`
/// points.
fn faces_toward(body: &Body, face: usize, facing: DVec3, what: &str) {
    let plane = body
        .plane_of(face)
        .unwrap_or_else(|| panic!("{what}: the body offers no plane on face {face}"));
    assert!(
        plane.normal.distance(facing) < PLACE,
        "{what}: face {face} faces {}, not {facing}",
        plane.normal
    );
}

/// The sides of the faces the face answering to all of `numbers`, and to
/// nothing else, meets, one for each edge it has, in order.
fn sides_met_by(body: &Body, numbers: &[usize]) -> Vec<Vec<u32>> {
    let own: Vec<u32> = numbers.iter().map(|&number| number as u32).collect();
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

/// The sides a face is expected to meet, one edge each, in the order
/// [`sides_met_by`] lists them, each side every number the face across it
/// answers to: a full round answers to both its edges'.
fn sides_answering(faces: &[&[usize]]) -> Vec<Vec<u32>> {
    let mut sorted: Vec<Vec<u32>> = faces
        .iter()
        .map(|numbers| numbers.iter().map(|&number| number as u32).collect())
        .collect();
    sorted.sort();
    sorted
}

/// Holds that a body is exact and keeps the rules every solid must: its
/// surface closed, no face through another, the matter it holds what
/// arithmetic promised, and the matter its triangles enclose within `drawn`
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

/// Holds that a step reports the value it was asked, and took it whole.
fn taken_as_asked(asked: f64, taken: f64, value: f64, what: &str) {
    assert!(
        (asked - value).abs() < PLACE && (taken - value).abs() < PLACE,
        "{what}: asked {asked} and took {taken}, where {value} fits"
    );
}

/// The largest coordinate a body reaches, on either side of the origin.
fn reach_of(body: &Body) -> f64 {
    let (low, high) = body.bounds().expect("a part has a box");
    low.abs().max(high.abs()).max_element()
}

/// Holds that the numbers `laid` answer to one and the same face, which is
/// curved: drawn with the same triangles whichever is asked.
fn one_curved_face_answers_to(body: &Body, laid: &[usize], what: &str) {
    let drawn: Vec<Vec<[DVec3; 3]>> = laid
        .iter()
        .map(|&number| body.triangles_of(number).collect())
        .collect();
    assert!(
        !drawn[0].is_empty() && drawn.iter().all(|each| *each == drawn[0]),
        "{what}: faces {laid:?} are one face, drawn with the same triangles"
    );
    for &number in laid {
        assert!(
            !body.is_flat(number) && body.plane_of(number).is_none(),
            "{what}: face {number} is curved, and offers no plane to draw on"
        );
    }
}

/// Whether some corner of the triangles `drawn` stands at `point`.
fn drawn_through(drawn: &[DVec3], point: DVec3) -> bool {
    drawn.iter().any(|corner| corner.distance(point) < PLACE)
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

    /// The round's area, its turn times its radius times its length.
    fn area(&self) -> f64 {
        let [first, second] = self.facing;
        first.dot(second).acos() * self.radius * self.length
    }
}

/// The full round, whole: two quarter rounds about the line [`RADIUS`] under
/// the middle of the top, from the front up to the top and from the top down
/// to the back, all along the block.
fn full_round() -> [Round; 2] {
    [[-DVec3::Y, DVec3::Z], [DVec3::Z, DVec3::Y]].map(|facing| Round {
        start: DVec3::new(0.0, RADIUS, HEIGHT - RADIUS),
        along: DVec3::X,
        length: LENGTH,
        radius: RADIUS,
        facing,
    })
}

/// The round of the upright at the end's front corner, whole: a quarter round
/// about the vertical [`RADIUS`] in from the front and from the end, from the
/// floor up to the top, turning from the front's way to the end's.
fn front_upright_round() -> Round {
    Round {
        start: DVec3::new(LENGTH - RADIUS, RADIUS, 0.0),
        along: DVec3::Z,
        length: HEIGHT,
        radius: RADIUS,
        facing: [-DVec3::Y, DVec3::X],
    }
}

/// Where the full round's axis and the upright's round's cross: [`RADIUS`] in
/// from the end and from the front, and under the top.
fn crossing() -> DVec3 {
    DVec3::new(LENGTH - RADIUS, RADIUS, HEIGHT - RADIUS)
}

/// How far a point stands past the mitre, on the side of the upright's round:
/// how far it stands out along the block past the crossing, less how far up
/// from it. Nought on the plane z − 7 = x − 27, where the two cylinders meet.
fn past_the_mitre(point: DVec3) -> f64 {
    let off = point - crossing();
    off.x - off.z
}

/// Holds that every ray aimed square at the round's axis from outside, over
/// a grid along it and round it, meets the face numbered `face` first,
/// facing away from the axis along the ray, wherever `shows` says the round
/// shows at the point the ray aims at; and that some ray was aimed.
fn met_from_outside_where(
    body: &Body,
    face: usize,
    round: &Round,
    shows: impl Fn(DVec3) -> bool,
    what: &str,
) {
    let mut aimed = 0;
    for tenth in 0..10 {
        let along = (f64::from(tenth) + 0.5) / 10.0;
        for share in [0.1, 0.3, 0.5, 0.7, 0.9] {
            let facing = round.facing_at(share);
            let aim = round.axis_at(along) + facing * round.radius;
            if !shows(aim) {
                continue;
            }
            aimed += 1;
            let from = aim + facing * 10.0;
            let hit = body.ray_hit(from, -facing).unwrap_or_else(|| {
                panic!("{what}: a ray from {from} toward the axis meets nothing")
            });
            assert_eq!(
                hit.face, face,
                "{what}: a ray from {from} toward {aim} meets face {} before the round",
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
    assert!(aimed > 0, "{what}: some ray aims where the round shows");
}

/// What the block keeps once its full round and the round of the upright at
/// its end's front corner are laid at [`RADIUS`], as the module's
/// documentation derives it: the block, less what the full round takes all
/// along it and what the upright's round takes all up it, given back what
/// both take at the corner.
fn full_round_and_front_upright() -> f64 {
    let r = RADIUS;
    LENGTH * WIDTH * HEIGHT - r * r * (2.0 - PI / 2.0) * LENGTH - r * r * (1.0 - PI / 4.0) * HEIGHT
        + r.powi(3) * (5.0 / 3.0 - PI / 2.0)
}

/// The prism the full round takes away, raised whole along X past both ends
/// of the block: all that stands over the front and the back outside the half
/// cylinder about the line [`RADIUS`] under the middle of the top.
fn full_round_tool(part: &Body) -> Body {
    let far = 2.0 * RADIUS + 2.0;
    let foot = HEIGHT - RADIUS;
    let corners = vec![
        DVec2::new(-far, foot),
        DVec2::new(0.0, foot),
        DVec2::new(WIDTH, foot),
        DVec2::new(WIDTH + far, foot),
        DVec2::new(WIDTH + far, HEIGHT + far),
        DVec2::new(-far, HEIGHT + far),
    ];
    let mut runs = vec![Run::Straight; corners.len()];
    runs[1] = Run::Round {
        center: DVec2::new(RADIUS, foot),
        turn: -PI,
    };
    part.tool_raised(
        &profile(Contour { corners, runs }),
        Frame {
            origin: DVec3::new(-2.0, 0.0, 0.0),
            u: DVec3::Y,
            v: DVec3::Z,
        },
        DVec3::X * (LENGTH + 4.0),
    )
    .expect("the tool raises")
}

/// The prism the round of the upright at the end's front corner takes away,
/// raised whole up past the floor and the top: all that stands outside the
/// front and the end beyond the quarter round about the vertical [`RADIUS`] in
/// from both.
fn front_upright_tool(part: &Body) -> Body {
    let far = RADIUS + 2.0;
    let center = DVec2::new(LENGTH - RADIUS, RADIUS);
    let corners = vec![
        DVec2::new(LENGTH - RADIUS, 0.0),
        DVec2::new(LENGTH - RADIUS, -far),
        DVec2::new(LENGTH + far, -far),
        DVec2::new(LENGTH + far, RADIUS),
        DVec2::new(LENGTH, RADIUS),
    ];
    let mut runs = vec![Run::Straight; corners.len()];
    runs[4] = Run::Round {
        center,
        turn: -PI / 2.0,
    };
    part.tool_raised(
        &profile(Contour { corners, runs }),
        Frame {
            origin: DVec3::new(0.0, 0.0, -2.0),
            u: DVec3::X,
            v: DVec3::Y,
        },
        DVec3::Z * (HEIGHT + 4.0),
    )
    .expect("the tool raises")
}

/// What the block keeps once its full round and the round of the upright at
/// its end's front corner are laid at [`RADIUS`], computed apart from the
/// step: the full round's prism, then the upright round's, taken away by the
/// booleans.
fn full_round_and_front_upright_by_booleans(part: &Body) -> f64 {
    let rounded = part
        .difference(&full_round_tool(part))
        .expect("the full round's prism cuts the block");
    rounded
        .difference(&front_upright_tool(&rounded))
        .expect("the upright round's prism cuts the block")
        .volume()
}

/// Every order `count` things can be named in.
fn every_order(count: usize) -> Vec<Vec<usize>> {
    if count == 0 {
        return vec![Vec::new()];
    }
    every_order(count - 1)
        .into_iter()
        .flat_map(|shorter| {
            (0..count).map(move |at| {
                let mut order = shorter.clone();
                order.insert(at, count - 1);
                order
            })
        })
        .collect()
}

/// Each face of a body and the sides of the faces it meets, one for each edge
/// it has, every face laid on the edges named in `order` read by the edge it
/// was laid on, as the rank the listed order gives that edge: what tells two
/// bodies the same face for face, whatever the order their edges were named
/// in.
fn meetings(body: &Body, start: usize, order: &[usize]) -> BTreeMap<Vec<u32>, Vec<Vec<u32>>> {
    let as_listed = |side: Vec<u32>| -> Vec<u32> {
        let mut read: Vec<u32> = side
            .into_iter()
            .map(|number| {
                let number = number as usize;
                if number < start {
                    number as u32
                } else {
                    (start + order[number - start]) as u32
                }
            })
            .collect();
        read.sort();
        read
    };
    let mut met: BTreeMap<Vec<u32>, Vec<Vec<u32>>> = BTreeMap::new();
    for name in body.edge_names() {
        let [first, second] = name.between.map(as_listed);
        met.entry(first.clone()).or_default().push(second.clone());
        met.entry(second).or_default().push(first);
    }
    for sides in met.values_mut() {
        sides.sort();
    }
    met
}

/// What the faces of `body`, laid on the edges named in `order`, meet
/// otherwise than those of `reference`, laid on them named in
/// `reference_order`, each laid face read by the edge it was laid on: one
/// line for each face that does.
fn met_otherwise(
    start: usize,
    order: &[usize],
    body: &Body,
    reference_order: &[usize],
    reference: &Body,
) -> Vec<String> {
    let own = meetings(body, start, order);
    let theirs = meetings(reference, start, reference_order);
    let faces: BTreeSet<&Vec<u32>> = own.keys().chain(theirs.keys()).collect();
    faces
        .into_iter()
        .filter(|face| own.get(*face) != theirs.get(*face))
        .map(|face| {
            format!(
                "face {face:?} meets {:?}, where the listed order lays it meeting {:?}",
                own.get(face),
                theirs.get(face)
            )
        })
        .collect()
}

/// How many vertices of the body stand where exactly the faces `faces` meet,
/// each given as every number it answers to.
fn vertices_among(body: &Body, faces: &[&[usize]]) -> usize {
    let sides = sides_answering(faces);
    body.vertex_names()
        .into_iter()
        .filter(|vertex| vertex.faces == sides)
        .count()
}

/// Holds that the fillet at [`RADIUS`] on the full round's pair and the
/// upright at the end's front corner, named in `order` and laid as
/// `filleted`, takes the value whole and mitres the upright's round against
/// the full round, as [`shaped_as_the_mitre`] holds it, its faces numbered as
/// #698 and #803 number them: the full round answering to both its edges'
/// numbers and the upright's round to its own rank, the top gone, nothing laid
/// at a corner, and the next step numbering its faces past them.
fn mitres_the_upright_s_round_against_the_full_round(
    part: &Body,
    order: &[usize],
    filleted: &Filleted,
    what: &str,
) {
    let start = part.faces_end();
    let count = order.len();
    taken_as_asked(filleted.asked, filleted.taken, RADIUS, what);
    let body = &filleted.body;

    let rank_of = |edge: usize| {
        start
            + order
                .iter()
                .position(|&named| named == edge)
                .expect("every edge is named")
    };
    let mut pair = [rank_of(0), rank_of(1)];
    pair.sort();
    let full: &[usize] = &pair;
    let standing = rank_of(2);

    assert!(
        !body.has_face(TOP),
        "{what}: the top is gone, the full round over it"
    );
    let mut kept_and_laid = numbers(part);
    kept_and_laid.remove(&TOP);
    kept_and_laid.extend(start..start + count);
    assert_eq!(
        numbers(body),
        kept_and_laid,
        "{what}: every face of the block but its top, the full round answering to {pair:?}, the \
         upright's round to {standing}, and no face laid at a corner"
    );
    assert_eq!(
        body.faces_end(),
        start + count,
        "{what}: the next step numbers its faces past the {count} the list gave its edges"
    );
    one_curved_face_answers_to(body, full, &format!("{what}: the full round"));
    shaped_as_the_mitre(part, body, full, standing, what);
}

/// Holds that `body` is the block's full round mitred against the round of
/// the upright at its end's front corner, on the plane through their crossing
/// axes, the full round answering to the numbers `full` and the upright's
/// round to `standing`: the body exact, closed, uncrossed and drawn, holding
/// what arithmetic promises; both rounds curved, each drawn on its own
/// cylinder and on its own side of the mitre and met there from outside, both
/// drawn up to the mitre's two ends and sharing no point off its plane, one
/// vertex at each end; the end face keeping its back half under the quarter
/// circle the full round leaves on it, its top edge sharp; the back upright
/// sharp; the front and the floor giving up what lies outside the upright's
/// quarter circle; and every face meeting the faces it bounds, one edge each.
fn shaped_as_the_mitre(part: &Body, body: &Body, full: &[usize], standing: usize, what: &str) {
    let r = RADIUS;
    let [front_half, back_half] = full_round();
    let upright = front_upright_round();
    let area = front_half.area() + back_half.area() + upright.area();
    sound_drawn_within(
        body,
        full_round_and_front_upright(),
        DRAWN * reach_of(part) * area,
        what,
    );
    for number in full.iter().chain([&standing]) {
        assert!(
            !body.is_flat(*number) && body.plane_of(*number).is_none(),
            "{what}: face {number}, a round, is curved, and offers no plane to draw on"
        );
    }

    let full_drawn: Vec<DVec3> = body.triangles_of(full[0]).flatten().collect();
    for &point in &full_drawn {
        assert!(
            front_half.holds(point) || back_half.holds(point),
            "{what}: the full round is drawn through {point}, off its half cylinder of {r} about \
             the line {r} under the middle of the top"
        );
        assert!(
            point.y >= r - PLACE || past_the_mitre(point) <= PLACE,
            "{what}: the full round is drawn through {point}, over the front past the mitre where \
             the upright's round meets it"
        );
    }
    let standing_drawn: Vec<DVec3> = body.triangles_of(standing).flatten().collect();
    assert!(
        !standing_drawn.is_empty(),
        "{what}: the upright's round is drawn"
    );
    for &point in &standing_drawn {
        assert!(
            upright.holds(point),
            "{what}: the upright's round is drawn through {point}, off its quarter cylinder of \
             {r} about the vertical through {}",
            crossing().truncate()
        );
        assert!(
            past_the_mitre(point) >= -PLACE,
            "{what}: the upright's round is drawn through {point}, above the mitre where the \
             full round meets it"
        );
    }
    met_from_outside_where(
        body,
        full[0],
        &front_half,
        |aim| past_the_mitre(aim) < -CLEAR_OF_THE_MITRE,
        &format!("{what}: the full round over the front, short of the mitre"),
    );
    met_from_outside_where(
        body,
        full[0],
        &back_half,
        |_| true,
        &format!("{what}: the full round over the back"),
    );
    met_from_outside_where(
        body,
        standing,
        &upright,
        |aim| past_the_mitre(aim) > CLEAR_OF_THE_MITRE,
        &format!("{what}: the upright's round, under the mitre"),
    );

    let leaving_the_front = DVec3::new(LENGTH - r, 0.0, HEIGHT - r);
    let crown = DVec3::new(LENGTH, r, HEIGHT);
    for (point, at) in [
        (leaving_the_front, "where the full round leaves the front"),
        (crown, "the crown"),
    ] {
        for (drawn, which) in [
            (&full_drawn, "the full round"),
            (&standing_drawn, "the upright's round"),
        ] {
            assert!(
                drawn_through(drawn, point),
                "{what}: {which} is drawn up to {at}, {point}, where the mitre ends"
            );
        }
    }
    for &point in full_drawn
        .iter()
        .filter(|&&point| drawn_through(&standing_drawn, point))
    {
        assert!(
            past_the_mitre(point).abs() < PLACE,
            "{what}: the full round and the upright's round meet at {point}, off the plane \
             z − 7 = x − 27"
        );
    }
    for (faces, at) in [
        (
            [&[FRONT][..], full, &[standing]],
            "where the full round leaves the front",
        ),
        ([&[END][..], full, &[standing]], "the crown"),
    ] {
        assert_eq!(
            vertices_among(body, &faces),
            1,
            "{what}: the full round, the upright's round and the wall there meet at one vertex, \
             {at}"
        );
    }

    let end_top_back = DVec3::new(LENGTH, WIDTH, HEIGHT - r);
    assert!(
        drawn_through(&full_drawn, end_top_back),
        "{what}: the full round is drawn up to the end's top back corner, {end_top_back}, where \
         the quarter circle it leaves on the end starts"
    );
    faces_toward(body, END, DVec3::X, &format!("{what}: the end"));
    let quarter_circle_center = DVec3::new(LENGTH, r, HEIGHT - r);
    corners_are_along_an_arc(
        body,
        END,
        &[
            DVec3::new(LENGTH, r, 0.0),
            DVec3::new(LENGTH, WIDTH, 0.0),
            end_top_back,
            crown,
        ],
        |corner| on_the_quarter_circle(corner, quarter_circle_center, r, [DVec3::Y, DVec3::Z]),
        &format!(
            "{what}: the end, its back half kept from the floor up to the quarter circle the \
             full round leaves on it"
        ),
    );
    for point in body.triangles_of(END).flatten() {
        assert!(
            (point.x - LENGTH).abs() < PLACE
                && (r - PLACE..=WIDTH + PLACE).contains(&point.y)
                && point.z >= -PLACE
                && (point.z <= HEIGHT - r + PLACE
                    || point.distance(quarter_circle_center) <= r + PLACE),
            "{what}: the end is drawn through {point}, off its back half under the quarter \
             circle of {r} about {quarter_circle_center}"
        );
    }
    assert_eq!(
        sides_met_by(body, &[END]),
        sides_answering(&[&[FLOOR], &[BACK], full, &[standing]]),
        "{what}: the end meets the floor, the back along the back upright, left sharp, the full \
         round along the quarter circle, its top edge left sharp, and the upright's round, by \
         one edge each"
    );

    corners_are(
        body,
        BACK,
        &[
            DVec3::new(0.0, WIDTH, 0.0),
            DVec3::new(LENGTH, WIDTH, 0.0),
            end_top_back,
            DVec3::new(0.0, WIDTH, HEIGHT - r),
        ],
        &format!("{what}: the back, up to where the full round takes over and out to the end"),
    );
    assert_eq!(
        sides_met_by(body, &[BACK]),
        sides_answering(&[&[FLOOR], &[LEFT], &[END], full]),
        "{what}: the back meets the floor, the left end, the end along the back upright, left \
         sharp, and the full round, by one edge each"
    );
    corners_are(
        body,
        FRONT,
        &[
            DVec3::ZERO,
            DVec3::new(LENGTH - r, 0.0, 0.0),
            leaving_the_front,
            DVec3::new(0.0, 0.0, HEIGHT - r),
        ],
        &format!(
            "{what}: the front, up to where the full round takes over and out to where the \
             upright's round does"
        ),
    );
    assert_eq!(
        sides_met_by(body, &[FRONT]),
        sides_answering(&[&[FLOOR], &[LEFT], full, &[standing]]),
        "{what}: the front meets the floor, the left end, the full round and the upright's \
         round, by one edge each"
    );
    corners_are_along_an_arc(
        body,
        FLOOR,
        &[
            DVec3::ZERO,
            DVec3::new(LENGTH - r, 0.0, 0.0),
            DVec3::new(LENGTH, r, 0.0),
            DVec3::new(LENGTH, WIDTH, 0.0),
            DVec3::new(0.0, WIDTH, 0.0),
        ],
        |corner| {
            on_the_quarter_circle(
                corner,
                crossing().truncate().extend(0.0),
                r,
                [-DVec3::Y, DVec3::X],
            )
        },
        &format!(
            "{what}: the floor, its front right corner given up to the upright's round along \
             the quarter circle of {r} about {}",
            crossing().truncate()
        ),
    );
    assert_eq!(
        sides_met_by(body, &[FLOOR]),
        sides_answering(&[&[FRONT], &[END], &[BACK], &[LEFT], &[standing]]),
        "{what}: the floor meets the four walls and the upright's round, by one edge each"
    );
    assert_eq!(
        sides_met_by(body, full),
        sides_answering(&[&[FRONT], &[BACK], &[LEFT], &[END], &[standing]]),
        "{what}: the full round meets the front and the back along its sides, the left end, the \
         end along the quarter circle, and the upright's round along the mitre, by one edge each"
    );
    assert_eq!(
        sides_met_by(body, &[standing]),
        sides_answering(&[&[FLOOR], &[FRONT], &[END], full]),
        "{what}: the upright's round meets the floor, the front and the end along its sides, and \
         the full round along the mitre, by one edge each"
    );
}

#[test]
fn a_fillet_on_a_full_round_and_the_upright_at_its_end_s_front_corner_mitres_the_upright_s_round_against_the_full_round_on_an_ellipse_whatever_the_order()
 {
    let part = block();
    let promised = full_round_and_front_upright();
    assert!(
        (promised - (1215.0 + 144.0 * PI)).abs() <= EXACT * promised,
        "the arithmetic gives {promised}, where the documentation writes 1215 + 144π"
    );
    let by_booleans = full_round_and_front_upright_by_booleans(&part);
    assert!(
        (by_booleans - promised).abs() <= EXACT * promised,
        "the full round's prism and the upright round's, cut from the block by the booleans, \
         leave {by_booleans}, where arithmetic promises {promised}"
    );

    let start = part.faces_end();
    let named: Vec<EdgeName> = PAIR_AND_FRONT_UPRIGHT
        .iter()
        .map(|&([one, other], _)| edge_between(&part, one, other))
        .collect();
    let called = |order: &[usize]| {
        let called: Vec<&str> = order
            .iter()
            .map(|&edge| PAIR_AND_FRONT_UPRIGHT[edge].1)
            .collect();
        format!("the fillet at {RADIUS} on {}", called.join(", then "))
    };
    let laid: Vec<(Vec<usize>, Result<Filleted, Declined>)> = every_order(named.len())
        .into_iter()
        .map(|order| {
            let names: Vec<EdgeName> = order.iter().map(|&edge| named[edge].clone()).collect();
            let filleted = part.fillet_edges(&names, RADIUS);
            (order, filleted)
        })
        .collect();

    let declined: Vec<String> = laid
        .iter()
        .filter_map(|(order, laid)| {
            laid.as_ref()
                .err()
                .map(|declined| format!("{}: declined, {declined:?}", called(order)))
        })
        .collect();
    assert!(
        declined.is_empty(),
        "the fillet at {RADIUS} on the top's front and back edges of a block {WIDTH} wide, a full \
         round, and on the upright at its end's front corner mitres the upright's round against \
         the full round, whichever edge the list names first; declined:\n{}",
        declined.join("\n")
    );

    let listed: Vec<usize> = (0..named.len()).collect();
    let reference = laid
        .iter()
        .find(|(order, _)| *order == listed)
        .and_then(|(_, laid)| laid.as_ref().ok())
        .expect("the listed order is laid");
    for (order, laid) in &laid {
        let what = called(order);
        let filleted = laid.as_ref().expect("declines are held above");
        mitres_the_upright_s_round_against_the_full_round(&part, order, filleted, &what);
        let otherwise = met_otherwise(start, order, &filleted.body, &listed, &reference.body);
        assert!(
            otherwise.is_empty(),
            "{what}: the body is the one the listed order lays, its faces meeting alike; not \
             so:\n{}",
            otherwise.join("\n")
        );
    }
}
