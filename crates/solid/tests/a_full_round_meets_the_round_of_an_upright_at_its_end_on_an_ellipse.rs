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
//! (30, 3, 10). The upright at the back is its mirror image. With both
//! uprights named, their rounds meet across the end in a full round of their
//! own, standing, and the two full rounds meet on the same plane, the end
//! gone. An operation of the kernel on [`Body`], as #803's mitre is; the step
//! of the history that calls it comes with the application, in #701.
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
//!   by the edge it was laid on. Not in the issue's wording: the same with the
//!   back upright alone, mirrored across the block's middle — the end keeping
//!   its front half, the front upright left sharp —
//!   `a_fillet_on_a_full_round_and_the_upright_at_its_end_s_back_corner_mitres_the_upright_s_round_against_the_full_round_on_an_ellipse_whatever_the_order`
//! - The same with both uprights: the uprights' rounds one standing half
//!   cylinder about the vertical through (27, 3), mitred against the lying one
//!   on the plane z − 7 = x − 27 from (27, 0, 7) through (30, 3, 10) to
//!   (27, 6, 7); the end face gone; one body in every order; the volume
//!   arithmetic gives —
//!   `a_fillet_on_a_full_round_and_both_uprights_at_its_end_mitres_their_standing_full_round_against_the_lying_one_on_an_ellipse_whatever_the_order`.
//!   Read as: in each of the 24 orders, the value taken whole; the body exact,
//!   closed, uncrossed, holding `1170 + 153π`, which the booleans give too, and
//!   its triangles enclosing that within the drawing's tolerance; the top and
//!   the end gone, the lying full round one face answering to both its edges'
//!   ranks and the standing one one face answering to both uprights' ranks,
//!   nothing laid at a corner; the lying full round drawn on its half cylinder
//!   only and never past the mitre, the standing one on its half cylinder
//!   about the vertical through (27, 3) only and never above the mitre, each
//!   met first from outside; both drawn up to (27, 0, 7), (30, 3, 10) and
//!   (27, 6, 7), meeting at one vertex at each of the two ends of the mitre,
//!   and sharing no point off the plane; the front and the back up to
//!   (27, 0, 7) and (27, 6, 7), and the floor up to the half circle of radius
//!   3 about (27, 3); every face meeting the faces it bounds by one edge each,
//!   the two full rounds by the edges the mitre is laid as; and every order
//!   laying the body the listed order lays
//! - Throughout, as far as this slice goes: the lists `main` builds keep their
//!   bodies — the chamfers above but the tongue's, the end edge with both
//!   uprights without the pair, and every one of these at 6.5 wide —
//!   `the_lists_main_builds_at_the_end_of_a_block_6_or_6_5_wide_keep_their_bodies_in_every_order`.
//!   Read as: on the block 30 × 6 × 10 at 3, the chamfer on the pair and the
//!   front-right upright, on the pair and both uprights, and on the pair, the
//!   end edge and the front-right upright; the fillet and the chamfer on the
//!   end edge and both uprights; and on the block 30 × 6.5 × 10 at 3, the
//!   fillet and the chamfer on each of the five lists of the issue's table —
//!   each built in every order, the value taken whole, exact, closed and
//!   uncrossed, holding the volume arithmetic gives, which is the volume
//!   `main` measures, to the digits measured on #843's branch
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
//! `LWH − r²(2 − π/2)L − r²(1 − π/4)H + r³(5/3 − π/2)`, `1215 + 144π`, 1667.39,
//! with either upright. The two cylinders meet where `(z − 7)² = (x − 27)²`,
//! which on the corner's side is the plane `z − 7 = x − 27`. The booleans give
//! it too: the full round's prism cut from the block whole along its length,
//! then the upright's round's prism, raised whole up its height; each takes
//! what the other takes past the mitre, and the mitre is where their cuts
//! meet. With both uprights the standing full round takes `r²(2 − π/2)H`, and
//! the two full rounds share what lies past both in the two cubes at the end's
//! top corners: `LWH − r²(2 − π/2)(L + H) + 2r³(5/3 − π/2)`, `1170 + 153π`,
//! 1650.66, which the booleans give as well, the standing full round's prism
//! raised whole up the block.
//!
//! The lists `main` builds are held to the same arithmetic, on a block `W`
//! wide, at `d = r = 3`. Each named edge's cut alone takes a section all along
//! it — a flat the triangle `d²/2`, a round the strip `r²(1 − π/4)` — the
//! top's front and back edges 30 long, the end edge `W`, each upright 10. At
//! each of the end's two top corners, where the top's front or back edge, the
//! end edge and an upright meet, what the cuts of the named edges there take
//! more than once is given back: for two, mitred, `d³/3` or `r³(5/3 − π/2)`;
//! for three, closed by a triangle or a piece of sphere, `2d³/3` or
//! `r³(2 − 7π/12)` — #699's arithmetic. Nowhere else do two cuts share
//! matter: at 6 wide the front's and the back's meet along the middle of the
//! top, and the two uprights' along the middle of the end, without crossing;
//! at 6.5 they stay a quarter apart. Both criteria's bodies are that sum. The
//! lists `main` builds: at 6 wide, chamfered, the pair and the front upright
//! `1800 − 70d²/2 + d³/3`, 1494; the pair and both uprights 1458; the pair,
//! the end edge and the front upright 1485; the end edge and both uprights,
//! filleted `1656 + 63π/2`, 1754.96, and chamfered 1701. At 6.5 wide, filleted
//! then chamfered: the pair and the front upright `1365 + 144π`, 1817.39, and
//! 1644; the pair, the end edge and the front upright `1360.5 + 1143π/8`,
//! 1809.36, and 1632.75; the pair and both uprights `1320 + 153π`, 1800.66,
//! and 1608; the tongue, the pair, the end edge and both uprights,
//! `1279.5 + 1305π/8`, 1791.97, and 1596.75; the end edge and both uprights
//! `1801.5 + 261π/8`, 1903.99, and 1848.75. Each is the volume `main` measures,
//! to the digits measured on #843's branch, which builds these lists as `main`
//! does.

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

/// How far a volume `main` measured may stand from the arithmetic: half the
/// last of the four decimals it was written down with.
const AS_MEASURED: f64 = 5e-5;

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

/// A hair wider than twice the value asked, where no full round forms.
const WIDER: f64 = 6.5;

/// The value asked, half the width: the radius at which the rounds of the
/// top's front and back edges meet across the top in one full round.
const RADIUS: f64 = WIDTH / 2.0;

/// The edges the lists name, each as the two faces it lies between: the top's
/// front and back edges, the full round's pair; the top's edge across the
/// end; and the uprights where the front and the back meet the end.
const TOP_FRONT: [usize; 2] = [TOP, FRONT];
const TOP_BACK: [usize; 2] = [TOP, BACK];
const TOP_END: [usize; 2] = [TOP, END];
const FRONT_UPRIGHT: [usize; 2] = [FRONT, END];
const BACK_UPRIGHT: [usize; 2] = [BACK, END];

/// The full round's pair, and what each edge is called.
const PAIR: [([usize; 2], &str); 2] = [
    (TOP_FRONT, "the top's front edge"),
    (TOP_BACK, "the top's back edge"),
];

/// The edge across the end, and what it is called.
const END_EDGE: ([usize; 2], &str) = (TOP_END, "the top's edge across the end");

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

/// The block [`LENGTH`] long, `width` wide and [`HEIGHT`] high, drawn on the
/// ground from its origin and raised as `cao_part` builds a step.
fn block(width: f64) -> Body {
    let part = Body::default();
    let tool = part
        .tool_raised(
            &profile(Contour::rectangle(DVec2::ZERO, DVec2::new(LENGTH, width))),
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

/// Whether a point lies on the circle of `radius` about `center`, on the side
/// of it each of `ways` points to from the centre: a quarter of it for two
/// ways square to each other, a half for one.
fn on_the_circle_toward(point: DVec3, center: DVec3, radius: f64, ways: &[DVec3]) -> bool {
    let off = point - center;
    (off.length() - radius).abs() < PLACE && ways.iter().all(|way| off.dot(*way) >= -PLACE)
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

/// One half of the full round lying along the block, whole: a quarter round
/// about the line [`RADIUS`] under the middle of the top, all along the
/// block, from facing `side`, the front's way or the back's, to the top's.
fn lying_half(side: DVec3) -> Round {
    Round {
        start: DVec3::new(0.0, RADIUS, HEIGHT - RADIUS),
        along: DVec3::X,
        length: LENGTH,
        radius: RADIUS,
        facing: [side, DVec3::Z],
    }
}

/// One half of the full round standing at the end, whole — an upright's
/// round alone: a quarter round about the vertical [`RADIUS`] in from the end
/// and under the middle of the top, from the floor up to the top, from facing
/// `side`, the front's way or the back's, to the end's.
fn standing_half(side: DVec3) -> Round {
    Round {
        start: DVec3::new(LENGTH - RADIUS, RADIUS, 0.0),
        along: DVec3::Z,
        length: HEIGHT,
        radius: RADIUS,
        facing: [side, DVec3::X],
    }
}

/// Where the full round's axis and the uprights' rounds' cross: [`RADIUS`] in
/// from the end, under the middle of the top.
fn crossing() -> DVec3 {
    DVec3::new(LENGTH - RADIUS, RADIUS, HEIGHT - RADIUS)
}

/// How far a point stands past the mitre, on the side of the uprights'
/// rounds: how far it stands out along the block past the crossing, less how
/// far up from it. Nought on the plane z − 7 = x − 27, where the cylinders
/// meet.
fn past_the_mitre(point: DVec3) -> f64 {
    let off = point - crossing();
    off.x - off.z
}

/// One of the two uprights at the block's end: where the front meets the
/// end, or where the back does.
#[derive(Clone, Copy, Debug, PartialEq)]
enum Upright {
    Front,
    Back,
}

impl Upright {
    /// The upright, as the two faces it lies between, and what it is called.
    fn edge(self) -> ([usize; 2], &'static str) {
        match self {
            Upright::Front => (FRONT_UPRIGHT, "the upright at the end's front corner"),
            Upright::Back => (BACK_UPRIGHT, "the upright at the end's back corner"),
        }
    }

    /// The wall the upright stands on beside the end, then the wall across
    /// the block from it.
    fn walls(self) -> [usize; 2] {
        match self {
            Upright::Front => [FRONT, BACK],
            Upright::Back => [BACK, FRONT],
        }
    }

    /// The way that points toward this upright's wall as `way` points toward
    /// the front's: itself at the front, mirrored across the block's middle at
    /// the back.
    fn way(self, way: DVec3) -> DVec3 {
        match self {
            Upright::Front => way,
            Upright::Back => DVec3::new(way.x, -way.y, way.z),
        }
    }

    /// The point standing at this upright's corner where `point` stands at the
    /// front's: itself at the front, mirrored across the block's middle at the
    /// back.
    fn at(self, point: DVec3) -> DVec3 {
        match self {
            Upright::Front => point,
            Upright::Back => DVec3::new(point.x, WIDTH - point.y, point.z),
        }
    }

    /// The upright's round, whole: the half of the standing full round on its
    /// wall's side.
    fn round(self) -> Round {
        standing_half(self.way(-DVec3::Y))
    }

    /// The prism the upright's round takes away, raised whole up past the
    /// floor and the top: all that stands outside its wall and the end beyond
    /// the quarter round about the vertical [`RADIUS`] in from both. The back's
    /// outline is the front's mirrored, its corners taken in the reverse order
    /// so that it still runs anticlockwise, its arc still last.
    fn tool(self, part: &Body) -> Body {
        let far = RADIUS + 2.0;
        let mut corners: Vec<DVec2> = [
            DVec2::new(LENGTH - RADIUS, 0.0),
            DVec2::new(LENGTH - RADIUS, -far),
            DVec2::new(LENGTH + far, -far),
            DVec2::new(LENGTH + far, RADIUS),
            DVec2::new(LENGTH, RADIUS),
        ]
        .iter()
        .map(|corner| self.at(corner.extend(0.0)).truncate())
        .collect();
        if self == Upright::Back {
            corners.reverse();
        }
        let mut runs = vec![Run::Straight; corners.len()];
        runs[4] = Run::Round {
            center: crossing().truncate(),
            turn: -PI / 2.0,
        };
        raised_up_the_block(part, Contour { corners, runs })
    }
}

/// A tool drawn on the ground and raised whole up past the block's floor and
/// top.
fn raised_up_the_block(part: &Body, outline: Contour) -> Body {
    part.tool_raised(
        &profile(outline),
        Frame {
            origin: DVec3::new(0.0, 0.0, -2.0),
            u: DVec3::X,
            v: DVec3::Y,
        },
        DVec3::Z * (HEIGHT + 4.0),
    )
    .expect("the tool raises")
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

/// A fillet or a chamfer, at [`RADIUS`].
#[derive(Clone, Copy, Debug)]
enum Cut {
    Fillet,
    Chamfer,
}

/// What a cut laid: the value asked, the value taken, and the body.
struct Laid {
    asked: f64,
    taken: f64,
    body: Body,
}

impl Cut {
    /// What the cut is called.
    fn called(self) -> &'static str {
        match self {
            Cut::Fillet => "fillet",
            Cut::Chamfer => "chamfer",
        }
    }

    /// What one edge's cut alone takes away for each unit of its length,
    /// between two faces square to each other: the strip between them and the
    /// round, `r²(1 − π/4)`, or the triangle `d²/2`.
    fn section(self) -> f64 {
        let r = RADIUS;
        match self {
            Cut::Fillet => r * r * (1.0 - PI / 4.0),
            Cut::Chamfer => r * r / 2.0,
        }
    }

    /// What the cuts of `count` named edges meeting at a square corner take
    /// more than once there, given back: nothing for one; for two, mitred,
    /// `r³(5/3 − π/2)` or `d³/3`; for three, closed by a piece of sphere or a
    /// triangle, `r³(2 − 7π/12)` or `2d³/3`.
    fn given_back(self, count: usize) -> f64 {
        let cube = RADIUS.powi(3);
        match (self, count) {
            (_, 0 | 1) => 0.0,
            (Cut::Fillet, 2) => cube * (5.0 / 3.0 - PI / 2.0),
            (Cut::Chamfer, 2) => cube / 3.0,
            (Cut::Fillet, _) => cube * (2.0 - 7.0 * PI / 12.0),
            (Cut::Chamfer, _) => 2.0 * cube / 3.0,
        }
    }

    /// The cut at [`RADIUS`] on the edges `names` finds on `part`.
    fn laid_on(self, part: &Body, names: &[EdgeName]) -> Result<Laid, Declined> {
        match self {
            Cut::Fillet => part.fillet_edges(names, RADIUS).map(|laid| Laid {
                asked: laid.asked,
                taken: laid.taken,
                body: laid.body,
            }),
            Cut::Chamfer => part.chamfer_edges(names, RADIUS).map(|laid| Laid {
                asked: laid.asked,
                taken: laid.taken,
                body: laid.body,
            }),
        }
    }
}

/// What the block `width` wide keeps once the edges `listed` are cut at
/// [`RADIUS`], as the module's documentation derives it: each edge's cut
/// taken whole all along it, and given back at each of the end's two top
/// corners what the cuts of the named edges there take more than once.
fn kept(cut: Cut, width: f64, listed: &[[usize; 2]]) -> f64 {
    let length = |edge: &[usize; 2]| match *edge {
        TOP_FRONT | TOP_BACK => LENGTH,
        TOP_END => width,
        _ => HEIGHT,
    };
    let named_at =
        |corner: [[usize; 2]; 3]| corner.iter().filter(|edge| listed.contains(edge)).count();
    LENGTH * width * HEIGHT - cut.section() * listed.iter().map(length).sum::<f64>()
        + cut.given_back(named_at([TOP_FRONT, TOP_END, FRONT_UPRIGHT]))
        + cut.given_back(named_at([TOP_BACK, TOP_END, BACK_UPRIGHT]))
}

/// Holds that the arithmetic of [`kept`] gives the volume the module's
/// documentation writes out for the fillet on `listed`.
fn the_arithmetic_gives(listed: &[[usize; 2]], volume: f64, written: &str) {
    let promised = kept(Cut::Fillet, WIDTH, listed);
    assert!(
        (promised - volume).abs() <= EXACT * volume,
        "the arithmetic gives {promised}, where the documentation writes {written}"
    );
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

/// The prism the standing full round takes away, raised whole up past the
/// floor and the top: all that stands past the end, the front and the back
/// beyond the half cylinder about the vertical [`RADIUS`] in from the end and
/// under the middle of the top.
fn standing_full_round_tool(part: &Body) -> Body {
    let far = RADIUS + 2.0;
    let corners = vec![
        DVec2::new(LENGTH - RADIUS, 0.0),
        DVec2::new(LENGTH - RADIUS, -far),
        DVec2::new(LENGTH + far, -far),
        DVec2::new(LENGTH + far, WIDTH + far),
        DVec2::new(LENGTH - RADIUS, WIDTH + far),
        DVec2::new(LENGTH - RADIUS, WIDTH),
    ];
    let mut runs = vec![Run::Straight; corners.len()];
    runs[5] = Run::Round {
        center: crossing().truncate(),
        turn: -PI,
    };
    raised_up_the_block(part, Contour { corners, runs })
}

/// What the block keeps once its full round and the round of `upright` are
/// laid at [`RADIUS`], computed apart from the step: the full round's prism,
/// then the upright round's, taken away by the booleans.
fn full_round_and_upright_by_booleans(part: &Body, upright: Upright) -> f64 {
    let rounded = part
        .difference(&full_round_tool(part))
        .expect("the full round's prism cuts the block");
    rounded
        .difference(&upright.tool(&rounded))
        .expect("the upright round's prism cuts the block")
        .volume()
}

/// What the block keeps once its full round and the standing full round of
/// both uprights are laid at [`RADIUS`], computed apart from the step: the
/// lying full round's prism, then the standing one's, taken away by the
/// booleans.
fn two_full_rounds_by_booleans(part: &Body) -> f64 {
    let rounded = part
        .difference(&full_round_tool(part))
        .expect("the full round's prism cuts the block");
    rounded
        .difference(&standing_full_round_tool(&rounded))
        .expect("the standing full round's prism cuts the block")
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

/// The ranks the faces laid on the list's edges `edges` answer to, named in
/// `order`, from `start`, ascending.
fn ranks_of(start: usize, order: &[usize], edges: &[usize]) -> Vec<usize> {
    let mut ranks: Vec<usize> = edges
        .iter()
        .map(|edge| {
            start
                + order
                    .iter()
                    .position(|named| named == edge)
                    .expect("every edge is named")
        })
        .collect();
    ranks.sort();
    ranks
}

/// Holds that the fillet at [`RADIUS`] on `listed`'s edges of `part`, in
/// every order the list can name them, is laid, each order's body held by
/// `holds` — given the order, the step's answer and what it is called — and
/// each the body the listed order lays, its faces meeting alike, each laid
/// face read by the edge it was laid on.
fn filleted_in_every_order(
    part: &Body,
    listed: &[([usize; 2], &str)],
    holds: impl Fn(&[usize], &Filleted, &str),
) {
    let start = part.faces_end();
    let named: Vec<EdgeName> = listed
        .iter()
        .map(|&([one, other], _)| edge_between(part, one, other))
        .collect();
    let called = |order: &[usize]| {
        let called: Vec<&str> = order.iter().map(|&edge| listed[edge].1).collect();
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
    let listed_order: Vec<usize> = (0..named.len()).collect();
    assert!(
        declined.is_empty(),
        "{}, on the block {WIDTH} wide, is laid whichever edge the list names first; \
         declined:\n{}",
        called(&listed_order),
        declined.join("\n")
    );

    let reference = laid
        .iter()
        .find(|(order, _)| *order == listed_order)
        .and_then(|(_, laid)| laid.as_ref().ok())
        .expect("the listed order is laid");
    for (order, laid) in &laid {
        let what = called(order);
        let filleted = laid.as_ref().expect("declines are held above");
        holds(order, filleted, &what);
        let otherwise = met_otherwise(start, order, &filleted.body, &listed_order, &reference.body);
        assert!(
            otherwise.is_empty(),
            "{what}: the body is the one the listed order lays, its faces meeting alike; not \
             so:\n{}",
            otherwise.join("\n")
        );
    }
}

/// Holds that the fillet at [`RADIUS`] on the full round's pair and
/// `upright`, listed in that order, named in `order` and laid as `filleted`,
/// takes the value whole and mitres the upright's round against the full
/// round, as [`shaped_as_the_mitre`] holds it, its faces numbered as #698 and
/// #803 number them: the full round answering to both its edges' numbers and
/// the upright's round to its own rank, the top gone, nothing laid at a
/// corner, and the next step numbering its faces past them.
fn mitres_the_upright_s_round_against_the_full_round(
    part: &Body,
    upright: Upright,
    order: &[usize],
    filleted: &Filleted,
    what: &str,
) {
    let start = part.faces_end();
    let count = order.len();
    taken_as_asked(filleted.asked, filleted.taken, RADIUS, what);
    let body = &filleted.body;
    let full = ranks_of(start, order, &[0, 1]);
    let standing = ranks_of(start, order, &[2])[0];

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
        "{what}: every face of the block but its top, the full round answering to {full:?}, the \
         upright's round to {standing}, and no face laid at a corner"
    );
    assert_eq!(
        body.faces_end(),
        start + count,
        "{what}: the next step numbers its faces past the {count} the list gave its edges"
    );
    one_curved_face_answers_to(body, &full, &format!("{what}: the full round"));
    shaped_as_the_mitre(part, body, &full, standing, upright, what);
}

/// Holds that `body` is the block's full round mitred against the round of
/// `upright`, on the plane through their crossing axes, the full round
/// answering to the numbers `full` and the upright's round to `standing`: the
/// body exact, closed, uncrossed and drawn, holding what arithmetic promises;
/// both rounds curved, each drawn on its own cylinder and on its own side of
/// the mitre and met there from outside, both drawn up to the mitre's two ends
/// and sharing no point off its plane, one vertex at each end; the end face
/// keeping the half across from the upright, under the quarter circle the full
/// round leaves on it, its top edge sharp; the other upright sharp; the
/// upright's wall and the floor giving up what lies outside its quarter
/// circle; and every face meeting the faces it bounds, one edge each.
fn shaped_as_the_mitre(
    part: &Body,
    body: &Body,
    full: &[usize],
    standing: usize,
    upright: Upright,
    what: &str,
) {
    let r = RADIUS;
    let [near, far] = upright.walls();
    let [near_half, far_half] = [-DVec3::Y, DVec3::Y].map(|side| lying_half(upright.way(side)));
    let round = upright.round();
    let area = near_half.area() + far_half.area() + round.area();
    sound_drawn_within(
        body,
        kept(Cut::Fillet, WIDTH, &[TOP_FRONT, TOP_BACK, upright.edge().0]),
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
            near_half.holds(point) || far_half.holds(point),
            "{what}: the full round is drawn through {point}, off its half cylinder of {r} about \
             the line {r} under the middle of the top"
        );
        assert!(
            upright.at(point).y >= r - PLACE || past_the_mitre(point) <= PLACE,
            "{what}: the full round is drawn through {point}, on the upright's side past the \
             mitre where the upright's round meets it"
        );
    }
    let standing_drawn: Vec<DVec3> = body.triangles_of(standing).flatten().collect();
    assert!(
        !standing_drawn.is_empty(),
        "{what}: the upright's round is drawn"
    );
    for &point in &standing_drawn {
        assert!(
            round.holds(point),
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
        &near_half,
        |aim| past_the_mitre(aim) < -CLEAR_OF_THE_MITRE,
        &format!("{what}: the full round on the upright's side, short of the mitre"),
    );
    met_from_outside_where(
        body,
        full[0],
        &far_half,
        |_| true,
        &format!("{what}: the full round across from the upright"),
    );
    met_from_outside_where(
        body,
        standing,
        &round,
        |aim| past_the_mitre(aim) > CLEAR_OF_THE_MITRE,
        &format!("{what}: the upright's round, under the mitre"),
    );

    let leaving_the_wall = upright.at(DVec3::new(LENGTH - r, 0.0, HEIGHT - r));
    let crown = DVec3::new(LENGTH, r, HEIGHT);
    for (point, at) in [
        (
            leaving_the_wall,
            "where the full round leaves the upright's wall",
        ),
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
            [&[near][..], full, &[standing]],
            "where the full round leaves the upright's wall",
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

    let end_top_far = upright.at(DVec3::new(LENGTH, WIDTH, HEIGHT - r));
    assert!(
        drawn_through(&full_drawn, end_top_far),
        "{what}: the full round is drawn up to the end's top corner across from the upright, \
         {end_top_far}, where the quarter circle it leaves on the end starts"
    );
    faces_toward(body, END, DVec3::X, &format!("{what}: the end"));
    let quarter_circle_center = DVec3::new(LENGTH, r, HEIGHT - r);
    corners_are_along_an_arc(
        body,
        END,
        &[
            upright.at(DVec3::new(LENGTH, r, 0.0)),
            upright.at(DVec3::new(LENGTH, WIDTH, 0.0)),
            end_top_far,
            crown,
        ],
        |corner| {
            on_the_circle_toward(
                corner,
                quarter_circle_center,
                r,
                &[upright.way(DVec3::Y), DVec3::Z],
            )
        },
        &format!(
            "{what}: the end, its half across from the upright kept from the floor up to the \
             quarter circle the full round leaves on it"
        ),
    );
    for point in body.triangles_of(END).flatten() {
        assert!(
            (point.x - LENGTH).abs() < PLACE
                && (r - PLACE..=WIDTH + PLACE).contains(&upright.at(point).y)
                && point.z >= -PLACE
                && (point.z <= HEIGHT - r + PLACE
                    || point.distance(quarter_circle_center) <= r + PLACE),
            "{what}: the end is drawn through {point}, off its half across from the upright, \
             under the quarter circle of {r} about {quarter_circle_center}"
        );
    }
    assert_eq!(
        sides_met_by(body, &[END]),
        sides_answering(&[&[FLOOR], &[far], full, &[standing]]),
        "{what}: the end meets the floor, the wall across from the upright along its own \
         upright, left sharp, the full round along the quarter circle, its top edge left sharp, \
         and the upright's round, by one edge each"
    );

    corners_are(
        body,
        far,
        &[
            upright.at(DVec3::new(0.0, WIDTH, 0.0)),
            upright.at(DVec3::new(LENGTH, WIDTH, 0.0)),
            end_top_far,
            upright.at(DVec3::new(0.0, WIDTH, HEIGHT - r)),
        ],
        &format!(
            "{what}: the wall across from the upright, up to where the full round takes over and \
             out to the end"
        ),
    );
    assert_eq!(
        sides_met_by(body, &[far]),
        sides_answering(&[&[FLOOR], &[LEFT], &[END], full]),
        "{what}: the wall across from the upright meets the floor, the left end, the end along \
         its upright, left sharp, and the full round, by one edge each"
    );
    corners_are(
        body,
        near,
        &[
            upright.at(DVec3::ZERO),
            upright.at(DVec3::new(LENGTH - r, 0.0, 0.0)),
            leaving_the_wall,
            upright.at(DVec3::new(0.0, 0.0, HEIGHT - r)),
        ],
        &format!(
            "{what}: the upright's wall, up to where the full round takes over and out to where \
             the upright's round does"
        ),
    );
    assert_eq!(
        sides_met_by(body, &[near]),
        sides_answering(&[&[FLOOR], &[LEFT], full, &[standing]]),
        "{what}: the upright's wall meets the floor, the left end, the full round and the \
         upright's round, by one edge each"
    );
    corners_are_along_an_arc(
        body,
        FLOOR,
        &[
            DVec3::ZERO,
            DVec3::new(0.0, WIDTH, 0.0),
            upright.at(DVec3::new(LENGTH - r, 0.0, 0.0)),
            upright.at(DVec3::new(LENGTH, r, 0.0)),
            upright.at(DVec3::new(LENGTH, WIDTH, 0.0)),
        ],
        |corner| {
            on_the_circle_toward(
                corner,
                crossing().truncate().extend(0.0),
                r,
                &[upright.way(-DVec3::Y), DVec3::X],
            )
        },
        &format!(
            "{what}: the floor, its corner under the upright given up to the upright's round \
             along the quarter circle of {r} about {}",
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
        sides_answering(&[&[FLOOR], &[near], &[END], full]),
        "{what}: the upright's round meets the floor, its wall and the end along its sides, and \
         the full round along the mitre, by one edge each"
    );
}

/// Holds, for `upright`, that the fillet at [`RADIUS`] on the full round's
/// pair and that upright is laid in every order as
/// [`mitres_the_upright_s_round_against_the_full_round`] holds it, after the
/// arithmetic has been held to give `1215 + 144π`, and the booleans to give
/// what the arithmetic does.
fn mitred_against_the_full_round_whatever_the_order(upright: Upright) {
    let part = block(WIDTH);
    let listed = [PAIR[0], PAIR[1], upright.edge()];
    let edges = listed.map(|(edge, _)| edge);
    the_arithmetic_gives(&edges, 1215.0 + 144.0 * PI, "1215 + 144π");
    let promised = kept(Cut::Fillet, WIDTH, &edges);
    let by_booleans = full_round_and_upright_by_booleans(&part, upright);
    assert!(
        (by_booleans - promised).abs() <= EXACT * promised,
        "the full round's prism and the round's of {}, cut from the block by the booleans, \
         leave {by_booleans}, where arithmetic promises {promised}",
        upright.edge().1
    );
    filleted_in_every_order(&part, &listed, |order, filleted, what| {
        mitres_the_upright_s_round_against_the_full_round(&part, upright, order, filleted, what);
    });
}

#[test]
fn a_fillet_on_a_full_round_and_the_upright_at_its_end_s_front_corner_mitres_the_upright_s_round_against_the_full_round_on_an_ellipse_whatever_the_order()
 {
    mitred_against_the_full_round_whatever_the_order(Upright::Front);
}

#[test]
fn a_fillet_on_a_full_round_and_the_upright_at_its_end_s_back_corner_mitres_the_upright_s_round_against_the_full_round_on_an_ellipse_whatever_the_order()
 {
    mitred_against_the_full_round_whatever_the_order(Upright::Back);
}

/// Holds that the fillet at [`RADIUS`] on the full round's pair and both
/// uprights, listed in that order, named in `order` and laid as `filleted`,
/// takes the value whole and mitres the standing full round against the lying
/// one, as [`shaped_as_two_full_rounds_mitred`] holds it, its faces numbered
/// as #698 numbers a full round: each full round one face answering to both
/// its edges' numbers, the top and the end gone, nothing laid at a corner, and
/// the next step numbering its faces past them.
fn mitres_the_standing_full_round_against_the_lying_one(
    part: &Body,
    order: &[usize],
    filleted: &Filleted,
    what: &str,
) {
    let start = part.faces_end();
    let count = order.len();
    taken_as_asked(filleted.asked, filleted.taken, RADIUS, what);
    let body = &filleted.body;
    let lying = ranks_of(start, order, &[0, 1]);
    let standing = ranks_of(start, order, &[2, 3]);

    for (gone, which) in [(TOP, "the top"), (END, "the end")] {
        assert!(
            !body.has_face(gone),
            "{what}: {which} is gone, a full round over it"
        );
    }
    let mut kept_and_laid = numbers(part);
    kept_and_laid.remove(&TOP);
    kept_and_laid.remove(&END);
    kept_and_laid.extend(start..start + count);
    assert_eq!(
        numbers(body),
        kept_and_laid,
        "{what}: every face of the block but its top and its end, the lying full round \
         answering to {lying:?}, the standing one to {standing:?}, and no face laid at a corner"
    );
    assert_eq!(
        body.faces_end(),
        start + count,
        "{what}: the next step numbers its faces past the {count} the list gave its edges"
    );
    one_curved_face_answers_to(body, &lying, &format!("{what}: the lying full round"));
    one_curved_face_answers_to(body, &standing, &format!("{what}: the standing full round"));
    shaped_as_two_full_rounds_mitred(part, body, &lying, &standing, what);
}

/// Holds that `body` is the block's lying full round mitred against the
/// standing full round of both uprights, on the plane through their crossing
/// axes, the lying one answering to the numbers `lying` and the standing one
/// to `standing`: the body exact, closed, uncrossed and drawn, holding what
/// arithmetic promises; the end gone; each full round drawn on its own half
/// cylinder and on its own side of the mitre and met there from outside, both
/// drawn up to the mitre's two ends and through the crown, sharing no point off
/// its plane, one vertex at each end; the front, the back and the floor giving
/// up what lies outside the standing half circle; and every face meeting the
/// faces it bounds, one edge each, the two full rounds by the edges the
/// mitre is laid as.
fn shaped_as_two_full_rounds_mitred(
    part: &Body,
    body: &Body,
    lying: &[usize],
    standing: &[usize],
    what: &str,
) {
    let r = RADIUS;
    let lying_halves = [-DVec3::Y, DVec3::Y].map(lying_half);
    let standing_halves = [-DVec3::Y, DVec3::Y].map(standing_half);
    let area: f64 = lying_halves
        .iter()
        .chain(&standing_halves)
        .map(Round::area)
        .sum();
    sound_drawn_within(
        body,
        kept(
            Cut::Fillet,
            WIDTH,
            &[TOP_FRONT, TOP_BACK, FRONT_UPRIGHT, BACK_UPRIGHT],
        ),
        DRAWN * reach_of(part) * area,
        what,
    );

    let lying_drawn: Vec<DVec3> = body.triangles_of(lying[0]).flatten().collect();
    for &point in &lying_drawn {
        assert!(
            lying_halves.iter().any(|half| half.holds(point)),
            "{what}: the lying full round is drawn through {point}, off its half cylinder of {r} \
             about the line {r} under the middle of the top"
        );
        assert!(
            past_the_mitre(point) <= PLACE,
            "{what}: the lying full round is drawn through {point}, past the mitre where the \
             standing one meets it"
        );
    }
    let standing_drawn: Vec<DVec3> = body.triangles_of(standing[0]).flatten().collect();
    for &point in &standing_drawn {
        assert!(
            standing_halves.iter().any(|half| half.holds(point)),
            "{what}: the standing full round is drawn through {point}, off its half cylinder of \
             {r} about the vertical through {}",
            crossing().truncate()
        );
        assert!(
            past_the_mitre(point) >= -PLACE,
            "{what}: the standing full round is drawn through {point}, above the mitre where the \
             lying one meets it"
        );
    }
    for half in &lying_halves {
        met_from_outside_where(
            body,
            lying[0],
            half,
            |aim| past_the_mitre(aim) < -CLEAR_OF_THE_MITRE,
            &format!("{what}: the lying full round, short of the mitre"),
        );
    }
    for half in &standing_halves {
        met_from_outside_where(
            body,
            standing[0],
            half,
            |aim| past_the_mitre(aim) > CLEAR_OF_THE_MITRE,
            &format!("{what}: the standing full round, under the mitre"),
        );
    }

    let [leaving_the_front, leaving_the_back] =
        [0.0, WIDTH].map(|y| DVec3::new(LENGTH - r, y, HEIGHT - r));
    let crown = DVec3::new(LENGTH, r, HEIGHT);
    for (point, at) in [
        (
            leaving_the_front,
            "where the lying full round leaves the front",
        ),
        (crown, "the crown"),
        (
            leaving_the_back,
            "where the lying full round leaves the back",
        ),
    ] {
        for (drawn, which) in [
            (&lying_drawn, "the lying full round"),
            (&standing_drawn, "the standing full round"),
        ] {
            assert!(
                drawn_through(drawn, point),
                "{what}: {which} is drawn through {at}, {point}, on the mitre"
            );
        }
    }
    for &point in lying_drawn
        .iter()
        .filter(|&&point| drawn_through(&standing_drawn, point))
    {
        assert!(
            past_the_mitre(point).abs() < PLACE,
            "{what}: the two full rounds meet at {point}, off the plane z − 7 = x − 27"
        );
    }
    for (wall, at) in [
        (FRONT, "where the lying full round leaves the front"),
        (BACK, "where the lying full round leaves the back"),
    ] {
        assert_eq!(
            vertices_among(body, &[&[wall], lying, standing]),
            1,
            "{what}: the two full rounds and the wall there meet at one vertex, {at}"
        );
    }

    for (wall, y, which) in [(FRONT, 0.0, "the front"), (BACK, WIDTH, "the back")] {
        corners_are(
            body,
            wall,
            &[
                DVec3::new(0.0, y, 0.0),
                DVec3::new(LENGTH - r, y, 0.0),
                DVec3::new(LENGTH - r, y, HEIGHT - r),
                DVec3::new(0.0, y, HEIGHT - r),
            ],
            &format!(
                "{what}: {which}, up to where the lying full round takes over and out to where \
                 the standing one does"
            ),
        );
        assert_eq!(
            sides_met_by(body, &[wall]),
            sides_answering(&[&[FLOOR], &[LEFT], lying, standing]),
            "{what}: {which} meets the floor, the left end and the two full rounds, by one edge \
             each"
        );
    }
    corners_are_along_an_arc(
        body,
        FLOOR,
        &[
            DVec3::ZERO,
            DVec3::new(LENGTH - r, 0.0, 0.0),
            DVec3::new(LENGTH - r, WIDTH, 0.0),
            DVec3::new(0.0, WIDTH, 0.0),
        ],
        |corner| on_the_circle_toward(corner, crossing().truncate().extend(0.0), r, &[DVec3::X]),
        &format!(
            "{what}: the floor, its end given up to the standing full round along the half \
             circle of {r} about {}",
            crossing().truncate()
        ),
    );
    assert_eq!(
        sides_met_by(body, &[FLOOR]),
        sides_answering(&[&[FRONT], &[BACK], &[LEFT], standing]),
        "{what}: the floor meets the front, the back, the left end and the standing full round, \
         by one edge each"
    );
    assert_eq!(
        sides_met_by(body, lying),
        sides_answering(&[&[FRONT], &[BACK], &[LEFT], standing]),
        "{what}: the lying full round meets the front and the back along its sides, the left \
         end, and the standing full round along the mitre"
    );
    assert_eq!(
        sides_met_by(body, standing),
        sides_answering(&[&[FLOOR], &[FRONT], &[BACK], lying]),
        "{what}: the standing full round meets the floor, the front and the back along its \
         sides, and the lying full round along the mitre"
    );
}

#[test]
fn a_fillet_on_a_full_round_and_both_uprights_at_its_end_mitres_their_standing_full_round_against_the_lying_one_on_an_ellipse_whatever_the_order()
 {
    let part = block(WIDTH);
    let listed = [
        PAIR[0],
        PAIR[1],
        Upright::Front.edge(),
        Upright::Back.edge(),
    ];
    let edges = listed.map(|(edge, _)| edge);
    the_arithmetic_gives(&edges, 1170.0 + 153.0 * PI, "1170 + 153π");
    let promised = kept(Cut::Fillet, WIDTH, &edges);
    let by_booleans = two_full_rounds_by_booleans(&part);
    assert!(
        (by_booleans - promised).abs() <= EXACT * promised,
        "the lying and the standing full rounds' prisms, cut from the block by the booleans, \
         leave {by_booleans}, where arithmetic promises {promised}"
    );
    filleted_in_every_order(&part, &listed, |order, filleted, what| {
        mitres_the_standing_full_round_against_the_lying_one(&part, order, filleted, what);
    });
}

/// A list of edges at the block's end, each as the two faces it lies between,
/// and what it is called.
type Listed = Vec<([usize; 2], &'static str)>;

#[test]
fn the_lists_main_builds_at_the_end_of_a_block_6_or_6_5_wide_keep_their_bodies_in_every_order() {
    let [front_upright, back_upright] = [Upright::Front, Upright::Back].map(Upright::edge);
    let pair_and = |others: &[([usize; 2], &'static str)]| -> Listed {
        let mut list = PAIR.to_vec();
        list.extend_from_slice(others);
        list
    };
    let end_and_both_uprights = vec![END_EDGE, front_upright, back_upright];
    let cases: Vec<(f64, Cut, Listed, f64)> = vec![
        (WIDTH, Cut::Chamfer, pair_and(&[front_upright]), 1494.0),
        (
            WIDTH,
            Cut::Chamfer,
            pair_and(&[front_upright, back_upright]),
            1458.0,
        ),
        (
            WIDTH,
            Cut::Chamfer,
            pair_and(&[END_EDGE, front_upright]),
            1485.0,
        ),
        (WIDTH, Cut::Fillet, end_and_both_uprights.clone(), 1754.9602),
        (WIDTH, Cut::Chamfer, end_and_both_uprights.clone(), 1701.0),
        (WIDER, Cut::Fillet, pair_and(&[front_upright]), 1817.3893),
        (WIDER, Cut::Chamfer, pair_and(&[front_upright]), 1644.0),
        (
            WIDER,
            Cut::Fillet,
            pair_and(&[END_EDGE, front_upright]),
            1809.3551,
        ),
        (
            WIDER,
            Cut::Chamfer,
            pair_and(&[END_EDGE, front_upright]),
            1632.75,
        ),
        (
            WIDER,
            Cut::Fillet,
            pair_and(&[front_upright, back_upright]),
            1800.6637,
        ),
        (
            WIDER,
            Cut::Chamfer,
            pair_and(&[front_upright, back_upright]),
            1608.0,
        ),
        (
            WIDER,
            Cut::Fillet,
            pair_and(&[END_EDGE, front_upright, back_upright]),
            1791.9723,
        ),
        (
            WIDER,
            Cut::Chamfer,
            pair_and(&[END_EDGE, front_upright, back_upright]),
            1596.75,
        ),
        (WIDER, Cut::Fillet, end_and_both_uprights.clone(), 1903.9945),
        (WIDER, Cut::Chamfer, end_and_both_uprights, 1848.75),
    ];

    let mut not_kept = Vec::new();
    for (width, cut, listed, measured) in &cases {
        let part = block(*width);
        let edges: Vec<[usize; 2]> = listed.iter().map(|&(edge, _)| edge).collect();
        let promised = kept(*cut, *width, &edges);
        let called: Vec<&str> = listed.iter().map(|&(_, called)| called).collect();
        let list = format!(
            "the {} at {RADIUS} on {}, on the block {width} wide",
            cut.called(),
            called.join(", ")
        );
        assert!(
            (promised - measured).abs() <= AS_MEASURED,
            "{list}: the arithmetic gives {promised}, where main measures {measured}"
        );
        let named: Vec<EdgeName> = edges
            .iter()
            .map(|&[one, other]| edge_between(&part, one, other))
            .collect();
        for order in every_order(named.len()) {
            let names: Vec<EdgeName> = order.iter().map(|&edge| named[edge].clone()).collect();
            let what = format!("{list}, named in the order {order:?}");
            let laid = match cut.laid_on(&part, &names) {
                Ok(laid) => laid,
                Err(declined) => {
                    not_kept.push(format!("{what}: declined, {declined:?}"));
                    continue;
                }
            };
            if (laid.asked - RADIUS).abs() >= PLACE || (laid.taken - RADIUS).abs() >= PLACE {
                not_kept.push(format!(
                    "{what}: asked {} and took {}",
                    laid.asked, laid.taken
                ));
                continue;
            }
            let body = &laid.body;
            let triangles = body.triangles();
            if !body.is_exact() {
                not_kept.push(format!("{what}: not computed by the exact kernel"));
            } else if let Err(fault) = closed(&triangles) {
                not_kept.push(format!("{what}: not closed, {fault:?}"));
            } else if let Err(fault) = uncrossed(&triangles) {
                not_kept.push(format!("{what}: crossed, {fault:?}"));
            } else if (body.volume() - promised).abs() > EXACT * promised {
                not_kept.push(format!(
                    "{what}: holds {}, where arithmetic and main give {promised}",
                    body.volume()
                ));
            }
        }
    }
    assert!(
        not_kept.is_empty(),
        "every list main builds at the end of a block 6 or 6.5 wide is built in every order, \
         the value taken whole, exact, closed and uncrossed, holding what arithmetic and main \
         give; not so:\n{}",
        not_kept.join("\n")
    );
}
