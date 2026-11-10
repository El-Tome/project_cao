//! A chamfer and a fillet on the three straight edges of an exact body that
//! meet at a corner of mixed sense — two meeting one way, the odd one the
//! other — at one value, in one step. The odd edge is laid first, as it is
//! laid alone, and the other two follow the outline it leaves. An operation
//! of the kernel on [`Body`], as #699's and #801's corners are.
//!
//! Part of #829, its first slice: the foot corner of a block standing on
//! another, two concave feet and the convex upright above them. The upright
//! is chamfered or rounded first, its blend stopping flush on the lower
//! block's top as it does alone; the feet's fills then run along their walls
//! up to where the upright's blend begins and go round it. Chamfered, the
//! foot of the upright's flat is filled too, though no name in the list finds
//! it, and the three fills meet in mitres at the two corners of 135°, as #801
//! fills them when the upright was chamfered in an earlier step. Filleted,
//! the feet's rounds join through a piece of torus about the upright round's
//! axis, its tube of the value's radius and its centre circle twice the value
//! from that axis, tangent to both rounds, to the lower block's top and to
//! the upright's round. Slices 2, the top of the L's inner corner, and 3,
//! every edge of a standing block, are still to come.
//!
//! - The three edges at a standing block's foot corner — two feet and the
//!   upright above them — leave a closed, sound, drawn body when chamfered or
//!   filleted at one value, shaped as above, its volume exact, the same body
//!   in every order the list names them. Chamfered, the body is the one two
//!   steps leave, the upright chamfered alone and then its three feet —
//!   `a_chamfer_on_two_feet_of_a_standing_block_and_the_upright_above_them_fills_the_foot_of_the_upright_s_flat_and_mitres_the_three_fills_in_every_order`;
//!   filleted —
//!   `a_fillet_on_two_feet_of_a_standing_block_and_the_upright_above_them_joins_the_feet_s_rounds_through_a_piece_of_torus_in_every_order`
//! - The same at a corner that is not square: the trapezoid standing block
//!   #801 already uses, at its obtuse and at its acute corner. The arc the
//!   upright's round leaves then spans 180° less the corner's angle; volumes
//!   held by arithmetic or by the booleans —
//!   `a_chamfer_and_a_fillet_on_the_three_edges_at_a_foot_corner_of_a_standing_block_that_is_not_square_go_round_its_upright_in_every_order`
//! - The four feet and four uprights of a standing block in one step close
//!   its four foot corners —
//!   `a_chamfer_and_a_fillet_on_the_four_feet_and_four_uprights_of_a_standing_block_close_its_four_foot_corners_in_one_step`
//! - The patch's faces are numbered as #699 numbers its patches, after the
//!   edges' faces; the fill along the upright chamfer's foot, which no name
//!   in the list finds, is numbered with the patches too — held in
//!   `a_chamfer_on_two_feet_of_a_standing_block_and_the_upright_above_them_fills_the_foot_of_the_upright_s_flat_and_mitres_the_three_fills_in_every_order`,
//!   `a_fillet_on_two_feet_of_a_standing_block_and_the_upright_above_them_joins_the_feet_s_rounds_through_a_piece_of_torus_in_every_order`
//!   and
//!   `a_chamfer_and_a_fillet_on_the_four_feet_and_four_uprights_of_a_standing_block_close_its_four_foot_corners_in_one_step`.
//!   Read as: the face laid on each named edge — the upright's flat or
//!   round, each foot's fill — answers to where the steps' numbers ended
//!   plus the edge's rank in the list; then each corner's patch — the fill
//!   along the chamfer's foot, or the piece of torus — one number each,
//!   corner by corner by the lowest ranks of their edges in the list, then
//!   the next, as `chamfer_edges` numbers its triangles; and the next step
//!   numbers its own faces past them
//! - The decline test
//!   `a_fillet_and_a_chamfer_on_edges_meeting_at_a_corner_with_a_concave_edge_are_declined_as_unsupported`
//!   loses the list of the three edges at the standing block's front right
//!   foot, which becomes a body test above — no test: a change to that test,
//!   in `a_chamfer_and_a_fillet_blend_the_corner_their_edges_meet_at.rs`; and
//!   in `corners/tests.rs`, where the corner's own answer is read, the feet
//!   and upright are taken with no corner beside an edge, as a pocket's
//!   bottom corner is, while the L's three edges stay declined for slice 2
//! - What must not break: #699's and #801's corner tests; the decline test
//!   above keeps its two lists of two edges — a foot and the upright above
//!   it; the L's top edge at its inner corner and the inner upright — for
//!   #846 — no test: each is held by its own file, left as it was but for the
//!   one list the decline test gives up
//!
//! The volumes are arithmetic. The lower block is 30 by 20 by 10 and the
//! block standing on it 10 by 10 by 10, 7000 together. The corner is the
//! standing block's front right foot, (20, 5, 10): its front foot runs to the
//! left wall, its right foot to the back wall, each square to it, and the
//! upright stands 10 above them. A flat set back `d` along two faces square to
//! each other takes or fills a section `d²/2` along its edge; a round of
//! radius `r` the square of side `r` less the quarter of the disc,
//! `r²(1 − π/4)`.
//!
//! Chamfered at `d`, the upright's flat takes `10·d²/2` and leaves the
//! standing block's outline over the lower block's top a side `d√2` long
//! between the front, `10 − d` from the left wall, and the right, `10 − d`
//! to the back wall, at two corners of 135°. At a height `c` over the lower
//! block's top the three fills hold what lies between that outline and the
//! outline offset `t = d − c` out, its two corners mitred:
//! `(20 − 2d + d√2)·t + 2·tan(22.5°)·t²`, each corner turning 45° and its
//! mitre adding `t²·tan(22.5°)`. Over the height they add
//! `(20 − 2d + d√2)·d²/2 + 2·tan(22.5°)·d³/3`, and the body holds
//! `7000 − 5d² + (20 − 2d + d√2)·d²/2 + 2·tan(22.5°)·d³/3`, that is
//! `7000 + 5d² + (7√2/6 − 5/3)·d³`: 7004.98325 at 1, #801's body round its
//! upright chamfered by 1 in an earlier step. Held another way in the test
//! itself: the body those two steps build.
//!
//! Filleted at `r`, the upright's round, its axis `r` in from both walls,
//! takes `10·r²(1 − π/4)`. The feet's rounds run from the walls at their far
//! ends to the two lines where the upright's round leaves its walls, `10 − r`
//! each, and add `2(10 − r)·r²(1 − π/4)`, each ending on the plane through
//! the round's axis square to its wall. The piece of torus between them turns
//! their section a quarter turn about that axis: in a plane through the
//! axis, the section spans `ρ` from `r` to `2r` out from it and `z` from 0 to
//! `r` up from the lower block's top, less the disc of radius `r` centred
//! `(2r, r)`. Its `∫ρ dA` is the square's `3r³/2` less the quarter disc's
//! `(πr²/4)·(2r − 4r/(3π))`, `r³(11/6 − π/2)`, and by Pappus the piece holds
//! `(π/2)·r³(11/6 − π/2)`. The body holds
//! `7000 − 10·r²(1 − π/4) + 2(10 − r)·r²(1 − π/4) + (π/2)·r³(11/6 − π/2)`.
//! The piece meets each foot's round along that round's end section, tangent
//! to it there; the lower block's top along the circle `2r` from the axis,
//! and the upright's round along the circle `r` up, tangent to both.
//!
//! At a corner of the trapezoid #801 stands on the lower block — 10 at its
//! front, 14 at its back, 10 deep, its right wall leaning out 4 in 10, 7200
//! with the block below — the matter's angle `α` at the corner is 111.8° at
//! the front right and 68.2° at the back right, `sin α = 10/√116` at both.
//! The chamfer sets back `d` along each wall: its flat takes `5d²·sin α`
//! and is `2d·sin(α/2)` wide at its foot, and the fills meet it at two
//! corners each turning `(180° − α)/2`. The round, its axis on the corner's
//! bisector `r` in from both walls, leaves them `r·cot(α/2)` from the corner,
//! takes `10·(r²·cot(α/2) − (π − α)·r²/2)`, and its arc spans `π − α`, which
//! the piece of torus turns the section through: `(π − α)·r³(11/6 − π/2)`.
//! The right foot's far end lies on a wall leaning across it, which #801's
//! arithmetic handles: `+0.4` times the section's first moment from the
//! obtuse corner, its far end on the back wall, and `−0.4` times it from the
//! acute one, its far end on the front wall — `d³/6` among flats,
//! `r³(5/6 − π/4)` among rounds.
//!
//! The four feet and four uprights of [`stacked`] leave an octagon among
//! flats, four sides `10 − 2d` and four `d√2`, eight corners of 135°:
//! `7000 − 20d² + (40 − 8d + 4d√2)·d²/2 + 8·tan(22.5°)·d³/3`; and among
//! rounds four rounds of the uprights, four feet `10 − 2r` long and four
//! pieces of torus: `7000 − 40·r²(1 − π/4) + 4(10 − 2r)·r²(1 − π/4) +
//! 2π·r³(11/6 − π/2)`.

use std::collections::BTreeSet;
use std::f64::consts::PI;

use cao_solid::profile::{Contour, Frame, Profile};
use cao_solid::soundness::{closed, enclosed, uncrossed};
use cao_solid::{Body, Chamfered, EdgeName, Loop};
use glam::{DVec2, DVec3};

/// How far a corner may stand from where it is expected, on a plane: each is
/// placed exactly, so what is left is rounding.
const PLACE: f64 = 1e-9;

/// How near the arithmetic an exact volume comes, as a fraction of it.
const EXACT: f64 = 1e-9;

/// The number a raised part's top answers to.
const TOP: usize = 1;

/// The values every list is laid at.
const VALUES: [f64; 2] = [1.0, 2.5];

/// Every order three edges can be named in.
const ORDERS_OF_THREE: [[usize; 3]; 6] = [
    [0, 1, 2],
    [0, 2, 1],
    [1, 0, 2],
    [1, 2, 0],
    [2, 0, 1],
    [2, 1, 0],
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

/// The part joined to an outline raised from `from` by `height`, as
/// `cao_part` raises a step: its faces numbered past the part's, its floor
/// first, then its top, then its walls, one for each run of the outline.
fn joined(part: &Body, outline: Contour, from: f64, height: f64) -> Body {
    let tool = part
        .tool_raised(
            &profile(outline),
            Frame {
                origin: DVec3::Z * from,
                ..ground()
            },
            DVec3::Z * height,
        )
        .expect("the tool raises");
    part.union(&tool).expect("the part is built")
}

/// A block 30 by 20 by 10 with another, 10 by 10 by 10, standing on the
/// middle of its top: the upper block's top numbered seven, its walls, front,
/// right, back and left, eight to eleven, its floor gone into the lower
/// block's top. Its feet, where its walls meet that top, meet inwards; its
/// uprights outwards.
fn stacked() -> Body {
    joined(
        &joined(
            &Body::default(),
            Contour::rectangle(DVec2::ZERO, DVec2::new(30.0, 20.0)),
            0.0,
            10.0,
        ),
        Contour::rectangle(DVec2::new(10.0, 5.0), DVec2::new(20.0, 15.0)),
        10.0,
        10.0,
    )
}
const STACKED_VOLUME: f64 = 30.0 * 20.0 * 10.0 + 10.0 * 10.0 * 10.0;
const STANDING_FRONT: usize = 8;
const STANDING_RIGHT: usize = 9;

/// The number the flat a chamfer lays on [`stacked`]'s front right upright
/// alone answers to: where the part's numbers ended.
const UPRIGHT_FLAT: usize = 12;

/// An edge at the standing block's front right foot corner: what it is
/// called, the two faces it lies between, the way the flat a chamfer lays on
/// it faces, out of the matter, and the number that flat answers to in the
/// body [`chamfered_in_two_steps`] leaves.
struct AtTheCorner {
    called: &'static str,
    between: [usize; 2],
    flat_facing: DVec3,
    in_two_steps: usize,
}

/// The three edges at [`stacked`]'s front right foot corner, (20, 5, 10):
/// its front foot, its right foot, and the upright above them.
fn foot_corner() -> [AtTheCorner; 3] {
    [
        AtTheCorner {
            called: "the standing block's front foot",
            between: [TOP, STANDING_FRONT],
            flat_facing: DVec3::new(0.0, -1.0, 1.0).normalize(),
            in_two_steps: UPRIGHT_FLAT + 1,
        },
        AtTheCorner {
            called: "the standing block's right foot",
            between: [TOP, STANDING_RIGHT],
            flat_facing: DVec3::new(1.0, 0.0, 1.0).normalize(),
            in_two_steps: UPRIGHT_FLAT + 2,
        },
        AtTheCorner {
            called: "the standing block's front right upright",
            between: [STANDING_FRONT, STANDING_RIGHT],
            flat_facing: DVec3::new(1.0, -1.0, 0.0).normalize(),
            in_two_steps: UPRIGHT_FLAT,
        },
    ]
}

/// The way the fill along the foot of the upright's flat faces, out of the
/// matter: halfway between up and the way that flat faces.
fn the_flat_s_foot_facing() -> DVec3 {
    (DVec3::new(1.0, -1.0, 0.0).normalize() + DVec3::Z).normalize()
}

/// The number the fill along the foot of the upright's flat answers to in
/// the body [`chamfered_in_two_steps`] leaves: the last of its second step.
const THE_FLAT_S_FOOT_IN_TWO_STEPS: usize = UPRIGHT_FLAT + 3;

/// [`stacked`] chamfered at `value` in two steps, as #801 builds it: its
/// front right upright alone first, the flat laid on it answering to
/// [`UPRIGHT_FLAT`] and stopping flush on the lower block's top; then, in one
/// step, its front foot, its right foot and the foot of that flat, numbered
/// in that order past it.
fn chamfered_in_two_steps(value: f64) -> Body {
    let part = stacked();
    let upright = part
        .chamfer(&edge_between(&part, STANDING_FRONT, STANDING_RIGHT), value)
        .unwrap_or_else(|declined| panic!("the upright alone is chamfered: {declined:?}"));
    taken_as_asked(&upright, value, "the upright chamfered alone");
    let chamfered = upright.body;
    let feet = [
        edge_between(&chamfered, TOP, STANDING_FRONT),
        edge_between(&chamfered, TOP, STANDING_RIGHT),
        edge_between(&chamfered, TOP, UPRIGHT_FLAT),
    ];
    let filled = chamfered
        .chamfer_edges(&feet, value)
        .unwrap_or_else(|declined| {
            panic!("the three feet round the chamfered upright are filled: {declined:?}")
        });
    taken_as_asked(&filled, value, "the three feet round the chamfered upright");
    filled.body
}

/// What [`stacked`] holds once the three edges at its front right foot
/// corner are chamfered at `value`, as the module's documentation derives
/// it: the upright's flat taken, and at each height the outline it leaves
/// offset out over the lower block's top, its two corners of 135° mitred.
fn chamfered_round_the_upright(value: f64) -> f64 {
    let along = 20.0 - 2.0 * value + 2f64.sqrt() * value;
    STACKED_VOLUME - 10.0 * value * value / 2.0
        + along * value * value / 2.0
        + 2.0 * (PI / 8.0).tan() * value.powi(3) / 3.0
}

/// The names of every edge between the faces numbered `first` and `second`.
fn edges_between(body: &Body, first: usize, second: usize) -> Vec<EdgeName> {
    let sides = [vec![first as u32], vec![second as u32]];
    let [one, other] = sides.clone();
    body.edge_names()
        .into_iter()
        .filter(|name| name.between == sides || name.between == [other.clone(), one.clone()])
        .collect()
}

/// The name of the one edge between the faces numbered `first` and `second`,
/// as the body hands it to whoever points at that edge.
fn edge_between(body: &Body, first: usize, second: usize) -> EdgeName {
    let named = edges_between(body, first, second);
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

/// The numbers of the faces a step made: those the body answers to that the
/// part it was given did not.
fn faces_made(part: &Body, body: &Body) -> Vec<usize> {
    numbers(body).difference(&numbers(part)).copied().collect()
}

/// Holds that a step reports the value it was asked, and took it whole.
fn taken_as_asked(step: &Chamfered, value: f64, what: &str) {
    assert!(
        (step.asked - value).abs() < PLACE && (step.taken - value).abs() < PLACE,
        "{what}: asked {} and took {}, where {value} fits",
        step.asked,
        step.taken
    );
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

/// Holds that a face is flat and faces the way `toward` points.
fn faces_toward(body: &Body, face: usize, toward: DVec3, what: &str) {
    let facing = body
        .plane_of(face)
        .unwrap_or_else(|| panic!("{what}: the body offers no plane on face {face}"))
        .normal;
    assert!(
        facing.distance(toward) < PLACE,
        "{what}: face {face} faces {facing}, where it faces {toward}"
    );
}

/// Holds that two bodies are the same, each face of `other` that `to_one`
/// names answering in `one` to the number it maps to: the same matter; the
/// same faces, each flat or curved alike and a flat one with the same
/// corners; and the same edges between the same faces.
fn the_same_bodies(one: &Body, other: &Body, to_one: &[(usize, usize)], what: &str) {
    let as_one = |number: usize| {
        to_one
            .iter()
            .find(|(from, _)| *from == number)
            .map_or(number, |&(_, to)| to)
    };
    assert!(
        (one.volume() - other.volume()).abs() <= EXACT * one.volume(),
        "{what}: the two hold {} and {}",
        one.volume(),
        other.volume()
    );
    let renumbered: BTreeSet<usize> = numbers(other).into_iter().map(as_one).collect();
    assert_eq!(numbers(one), renumbered, "{what}: the same faces");
    for number in numbers(other) {
        let face = as_one(number);
        assert_eq!(
            one.is_flat(face),
            other.is_flat(number),
            "{what}: face {face} is flat in one and not in the other"
        );
        let (Some(mine), Some(theirs)) = (one.plane_of(face), other.plane_of(number)) else {
            continue;
        };
        let within = |corners: &[DVec3], corner: &DVec3| {
            corners.iter().any(|point| point.distance(*corner) < PLACE)
        };
        assert!(
            mine.corners
                .iter()
                .all(|corner| within(&theirs.corners, corner))
                && theirs
                    .corners
                    .iter()
                    .all(|corner| within(&mine.corners, corner)),
            "{what}: face {face} has the corners {:?} in one and {:?} in the other",
            mine.corners,
            theirs.corners
        );
    }
    let between = |body: &Body, number: &dyn Fn(usize) -> usize| {
        let mut sides: Vec<[usize; 2]> = body
            .edge_names()
            .into_iter()
            .map(|name| {
                let [first, second] = name.between.map(|side| number(side[0] as usize));
                [first.min(second), first.max(second)]
            })
            .collect();
        sides.sort();
        sides
    };
    assert_eq!(
        between(one, &|number| number),
        between(other, &as_one),
        "{what}: the same edges between the same faces"
    );
}

/// A step on the three edges at the foot corner: the order the list named
/// them in, the value, which of the bodies two steps leave is built at that
/// value, and what the step left.
struct Case {
    order: [usize; 3],
    value: f64,
    two_steps: usize,
    step: Chamfered,
    what: String,
}

/// Holds what a chamfer of its value on the three edges at [`stacked`]'s
/// front right foot corner, named in its order, leaves of the part:
///
/// - the value taken whole; a sound body holding what
///   [`chamfered_round_the_upright`] promises;
/// - one flat for each edge, answering to where the part's numbers ended
///   plus the edge's rank in the list, then the fill along the foot of the
///   upright's flat, which no name finds, and the next step's numbers past
///   it; no face of the part gone;
/// - each of those flats facing the way it lies;
/// - and `two_steps`, the body the upright chamfered and then its three feet
///   leave, the same body, face for face.
fn went_round_the_upright(part: &Body, case: &Case, two_steps: &Body) {
    let Case {
        order,
        value,
        step,
        what,
        ..
    } = case;
    let edges = foot_corner();
    let start = part.faces_end();
    let patch = start + edges.len();
    let laid_on = |edge: usize| {
        start
            + order
                .iter()
                .position(|&named| named == edge)
                .expect("every edge of the corner is named")
    };
    taken_as_asked(step, *value, what);
    let body = &step.body;
    let promised = chamfered_round_the_upright(*value);
    sound_drawn_within(body, promised, EXACT * promised, what);
    assert_eq!(
        faces_made(part, body),
        (start..=patch).collect::<Vec<_>>(),
        "{what}: one flat for each edge, numbered in the order the step names them, and the fill \
         along the foot of the upright's flat after them: {:?}",
        numbers(body)
    );
    assert_eq!(
        body.faces_end(),
        patch + 1,
        "{what}: the next step numbers its own faces past the fill along the foot of the \
         upright's flat"
    );
    assert!(
        numbers(part).is_subset(&numbers(body)),
        "{what}: a face of the part went: {:?}",
        numbers(body)
    );
    for (index, edge) in edges.iter().enumerate() {
        faces_toward(
            body,
            laid_on(index),
            edge.flat_facing,
            &format!("{what}: the flat laid on {}", edge.called),
        );
    }
    faces_toward(
        body,
        patch,
        the_flat_s_foot_facing(),
        &format!("{what}: the fill along the foot of the upright's flat"),
    );
    let to_two_steps: Vec<(usize, usize)> = edges
        .iter()
        .enumerate()
        .map(|(index, edge)| (laid_on(index), edge.in_two_steps))
        .chain([(patch, THE_FLAT_S_FOOT_IN_TWO_STEPS)])
        .collect();
    the_same_bodies(
        two_steps,
        body,
        &to_two_steps,
        &format!("{what}, and the body two steps leave, the upright chamfered then its three feet"),
    );
}

#[test]
fn a_chamfer_on_two_feet_of_a_standing_block_and_the_upright_above_them_fills_the_foot_of_the_upright_s_flat_and_mitres_the_three_fills_in_every_order()
 {
    let part = stacked();
    let edges = foot_corner();
    let names = edges
        .each_ref()
        .map(|edge| edge_between(&part, edge.between[0], edge.between[1]));
    let mut in_two_steps = Vec::new();
    let mut laid = Vec::new();
    let mut declined = Vec::new();
    for value in VALUES {
        let promised = chamfered_round_the_upright(value);
        let two_steps = chamfered_in_two_steps(value);
        assert!(
            (two_steps.volume() - promised).abs() <= EXACT * promised,
            "the chamfer of {value} in two steps, the upright then its three feet: the body \
             holds {}, where arithmetic promised {promised}",
            two_steps.volume()
        );
        in_two_steps.push(two_steps);
        for order in ORDERS_OF_THREE {
            let list: Vec<EdgeName> = order.iter().map(|&edge| names[edge].clone()).collect();
            let what = format!(
                "the chamfer of {value} on {:?}",
                order.map(|edge| edges[edge].called)
            );
            match part.chamfer_edges(&list, value) {
                Ok(step) => laid.push(Case {
                    order,
                    value,
                    two_steps: in_two_steps.len() - 1,
                    step,
                    what,
                }),
                Err(reason) => declined.push(format!("{what}: {reason:?}")),
            }
        }
    }
    assert!(
        declined.is_empty(),
        "the three edges at the standing block's front right foot chamfered in one step, the \
         upright first and the feet round it; declined:\n{}",
        declined.join("\n")
    );
    for case in &laid {
        went_round_the_upright(&part, case, &in_two_steps[case.two_steps]);
    }
}
