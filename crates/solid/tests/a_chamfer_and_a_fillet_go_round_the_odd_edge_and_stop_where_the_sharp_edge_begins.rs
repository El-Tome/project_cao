//! Closes #846.
//!
//! A convex edge and a concave edge meeting at a corner whose third edge is
//! left sharp, chamfered or filleted in one step. As Tom decided on
//! 2026-11-10, the corner's odd edge — the one whose sense differs from the
//! corner's two others, named or not — is laid first, as #829 lays it; the
//! other named edge then follows the outline it leaves, goes round its arc or
//! flat, and stops flush where the wall of the edge left sharp begins, on the
//! plane square to that edge through the line where the odd edge's blend
//! meets that wall. A small end face stays in sight there.
//!
//! - A foot and the upright above it, the other foot left sharp, chamfered or
//!   filleted at one value, leave a closed, sound, drawn body of exact
//!   volume, the same in either order. This holds at the front right foot
//!   corner of `stacked()` and at the two corners of #801's trapezoid
//!   standing block that are not square. The named edges' faces are numbered
//!   at start + rank, then the patch, then the end face. The sharp foot stays
//!   one edge between the lower top and its wall, from its far end to the end
//!   face — chamfered at the square corner,
//!   `a_chamfer_on_a_foot_and_the_upright_above_it_goes_round_the_upright_s_flat_and_stops_where_the_sharp_foot_begins`;
//!   filleted there,
//!   `a_fillet_on_a_foot_and_the_upright_above_it_goes_round_the_upright_s_round_and_stops_where_the_sharp_foot_begins`;
//!   both at the trapezoid's two corners that are not square,
//!   `a_chamfer_and_a_fillet_on_a_foot_and_the_upright_above_it_at_a_corner_that_is_not_square_stop_where_the_sharp_foot_begins`.
//!   Read as: at 1 and at 2.5, the list as given and reversed; the faces
//!   made exactly those four, the next step's numbers past them, no face of
//!   the part gone; each flat facing the way it lies, the patch — the fill
//!   along the foot of the upright's flat, or the piece of torus — and the
//!   end face where the decision puts them, the end face flat on the plane
//!   square to the sharp foot, with its corners; the sharp foot one straight
//!   edge between the lower top and its wall, running from the face at its
//!   far end to the end face, its wall's corners where that leaves them; the
//!   two orders the same body, face for face. At the trapezoid, each foot of
//!   a corner is named in turn, the other left sharp
//! - The same for a top edge at the L's inner corner and the inner upright,
//!   the other top edge left sharp. Read as the foot corner's mirror: the
//!   upright filled first, flush on the top; the named top edge's cut round
//!   the top of its fill, and its end face cut into the top, on the plane
//!   square to the sharp top edge where the fill meets that edge's wall. The
//!   arm's back top edge is named, as the decline test named it, and the
//!   stem's right top edge in turn — no test: built later on this branch,
//!   the L's pair still declined today
//! - The decline test
//!   `a_fillet_and_a_chamfer_on_edges_meeting_at_a_corner_with_a_concave_edge_are_declined_as_unsupported`
//!   loses its last two lists, which become the body tests above; if nothing
//!   is left in it, it goes — no test: a change to that test, in
//!   `a_chamfer_and_a_fillet_blend_the_corner_their_edges_meet_at.rs`; and in
//!   `wedge/corners/tests.rs`, where the corner's own answer is read, a foot
//!   and the upright above it, and the L's top edge and its inner upright, are
//!   taken with no corner beside an edge, as #829's corners of mixed sense are
//! - What must not break: #699's, #801's and #829's corner tests — no test:
//!   each is held by its own file, left as it is
//!
//! The volumes are arithmetic. At a foot corner holding `α` of matter, its
//! upright `H` high, its named foot running from the corner to the wall at
//! its far end:
//!
//! Chamfered at `d`, the upright's flat takes `H·d²·sin α/2`. The named
//! foot's fill adds `d²/2` along its wall, from its far end to `d` short of
//! the corner, less `d³/6` times how far the lines along it shrink a unit
//! out from its wall where the far end's wall leans across it, as #801
//! reads it. The fill along the flat's foot adds `d²/2` along the flat's
//! width, `2d·sin(α/2)`. The one mitre between the two, where the outline
//! turns `(π − α)/2`, adds `tan((π − α)/4)·d³/3`. The fill's end, on the
//! plane square to the sharp foot through the point `d` from the corner
//! along it, where the flat meets its wall, meets each line of the fill
//! `cot(α/2)` further on per unit out from the flat, and adds
//! `cot(α/2)·d³/6`. The end face is a right triangle `d` high and
//! `d/sin(α/2)` out from the sharp foot's wall: the plane meets the fill at
//! `α/2` to its run, so it is `d√2` out at a square corner, not `d`.
//!
//! Filleted at `r`, the upright's round takes `H·(r·t − (π − α)·r²/2)`,
//! `t = r·cot(α/2)`. The named foot's round adds `r²(1 − π/4)` from its far
//! end to `t` short of the corner, less `r³(5/6 − π/4)` times the same
//! lean. The piece of torus about the upright round's axis adds
//! `(π − α)·r³(11/6 − π/2)` by Pappus. The arc ends on the plane through
//! that axis square to the sharp foot, which is the plane the decision
//! names, so the end face is the section itself, a square of `r` less a
//! quarter disc, and adds nothing.
//!
//! At `stacked()`'s square front right corner, its front foot named and
//! running 10 to the left wall:
//! `7000 − 5d² + (10 − d)·d²/2 + d√2·d²/2 + tan(22.5°)·d³/3 + d³/6`, which is
//! `7000 + (5√2/6 − 2/3)·d³`, and
//! `7000 − 10·r²(1 − π/4) + (10 − r)·r²(1 − π/4) + (π/2)·r³(11/6 − π/2)`.
//!
//! The top of the L's inner corner is the mirror, every term signed the
//! other way: the upright's fill adds, and the cut along the named top edge,
//! round the top of the fill, with its mitre and its end, takes — the cut's
//! end digging past the fill's flat into the top, up to the plane square to
//! the sharp top edge. On #699's L, 2760, its upright 10 high, the arm's back
//! top edge running 18 from the corner to the arm's end:
//! `2760 + 5d² − (18 − d)·d²/2 − d√2·d²/2 − tan(22.5°)·d³/3 − d³/6`, and
//! `2760 + 10·r²(1 − π/4) − (18 − r)·r²(1 − π/4) − (π/2)·r³(11/6 − π/2)`;
//! the stem's right top edge, 8 to the stem's back, in place of 18.

use std::collections::BTreeSet;
use std::f64::consts::PI;

use cao_solid::profile::{Contour, Frame, Profile};
use cao_solid::soundness::{closed, enclosed, uncrossed};
use cao_solid::{Body, Declined, EdgeName, Loop};
use glam::{DVec2, DVec3};

/// How far a corner may stand from where it is expected, on a plane: each is
/// placed exactly, so what is left is rounding.
const PLACE: f64 = 1e-9;

/// How near the arithmetic an exact volume comes, as a fraction of it.
const EXACT: f64 = 1e-9;

/// How far the triangles may stand from a round, as a share of the reach the
/// body carries: the drawing's tolerance.
const DRAWN: f64 = 2e-4;

/// The number a raised part's top answers to.
const TOP: usize = 1;

/// The values every list is laid at.
const VALUES: [f64; 2] = [1.0, 2.5];

/// The two orders a corner's two named edges can be named in, each by its
/// place in [`SharpCorner::edges`].
const ORDERS_OF_TWO: [[usize; 2]; 2] = [[0, 1], [1, 0]];

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

/// The tool an outline drawn on the ground raises from `from` by `height`,
/// as `cao_part` raises a step: its faces numbered past the part's, its floor
/// first, then its top, then its walls, one for each run of the outline.
fn tool_raised(part: &Body, outline: Contour, from: f64, height: f64) -> Body {
    part.tool_raised(
        &profile(outline),
        Frame {
            origin: DVec3::Z * from,
            ..ground()
        },
        DVec3::Z * height,
    )
    .expect("the tool raises")
}

/// The part joined to an outline raised from `from` by `height`, numbered as
/// [`tool_raised`] numbers it.
fn joined(part: &Body, outline: Contour, from: f64, height: f64) -> Body {
    part.union(&tool_raised(part, outline, from, height))
        .expect("the part is built")
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
const STANDING_TOP: usize = 7;
const STANDING_FRONT: usize = 8;
const STANDING_RIGHT: usize = 9;
const STANDING_BACK: usize = 10;
const STANDING_LEFT: usize = 11;

/// The height of the lower block's top, which the standing block stands on.
const LOWER_TOP: f64 = 10.0;

/// How high the standing block stands over the lower block's top.
const STANDING_HEIGHT: f64 = 10.0;

/// The wall a foot runs into at its far end: its number, where the foot ends
/// on it, and the way it faces, out of the matter.
struct FarEnd {
    on: usize,
    at: DVec3,
    facing: DVec3,
}

/// A foot at a foot corner: what it is called, the standing block's wall it
/// runs along, the way that wall faces, out of the matter, the way the foot
/// runs from the corner, and the wall its far end runs into.
struct Foot {
    called: &'static str,
    wall: usize,
    facing: DVec3,
    along: DVec3,
    far: FarEnd,
}

/// A foot corner of a block standing on another, one of its feet and the
/// upright above it named and the other foot left sharp: what it is called,
/// the part and the matter it holds, where the corner stands on the lower
/// block's top, the foot named and the foot left sharp, and what the upright
/// between their walls is called. Its two named edges are named, in every
/// list here, by their place in [`SharpCorner::edges`]: the foot, then the
/// upright.
struct SharpCorner {
    called: &'static str,
    part: fn() -> Body,
    volume: f64,
    at: DVec3,
    named: Foot,
    sharp: Foot,
    upright: &'static str,
}

/// Where a chamfer of a value on a [`SharpCorner`] lays its flats out, as the
/// module's documentation does: where the upright's flat leaves the named
/// foot's wall and where it leaves the sharp foot's, on the lower block's
/// top; the outer corner of the mitre between the named foot's fill and the
/// fill along the flat's foot; and the outer corner of that fill's end, on the
/// plane square to the sharp foot.
struct RoundTheFlat {
    leaves_named: DVec3,
    leaves_sharp: DVec3,
    mitre: DVec3,
    end: DVec3,
}

impl SharpCorner {
    /// What each of the two named edges is called, and the two faces it lies
    /// between: the foot, then the upright.
    fn edges(&self) -> [(&'static str, [usize; 2]); 2] {
        [
            (self.named.called, [TOP, self.named.wall]),
            (self.upright, [self.named.wall, self.sharp.wall]),
        ]
    }

    /// The angle of matter the two walls hold at the corner, `α`.
    fn angle(&self) -> f64 {
        self.named.along.dot(self.sharp.along).acos()
    }

    /// How far round the outline turns at the corner, `π − α`: the flat a
    /// chamfer lays on the upright turns it half of that at each of its ends.
    fn turn(&self) -> f64 {
        PI - self.angle()
    }

    /// The way the upright's flat faces, out of the matter: halfway between
    /// its two walls' ways.
    fn flat_facing(&self) -> DVec3 {
        (self.named.facing + self.sharp.facing).normalize()
    }

    /// How far the lines along the named foot shrink a unit out from its wall,
    /// where the wall at its far end leans across it.
    fn lean(&self) -> f64 {
        let foot = &self.named;
        foot.facing.dot(foot.far.facing) / foot.along.dot(foot.far.facing)
    }

    /// How long the named foot runs from the corner to the wall at its far
    /// end, along its own wall.
    fn named_length(&self) -> f64 {
        (self.named.far.at - self.at).dot(self.named.along)
    }

    /// Where a chamfer of `value` lays its flats out at the corner.
    fn round_the_flat(&self, value: f64) -> RoundTheFlat {
        let d = value;
        let [a, b] = [self.named.along, self.sharp.along];
        let half = self.angle() / 2.0;
        let leaves_named = self.at + a * d;
        let leaves_sharp = self.at + b * d;
        let run = (b - a).normalize();
        RoundTheFlat {
            leaves_named,
            leaves_sharp,
            mitre: leaves_named + (self.named.facing - a * (self.turn() / 4.0).tan()) * d,
            end: leaves_sharp + (self.flat_facing() + run / half.tan()) * d,
        }
    }

    /// What the part holds once the named foot and the upright are chamfered
    /// at `value`, as the module's documentation derives it.
    fn chamfered(&self, value: f64) -> f64 {
        let d = value;
        let half = self.angle() / 2.0;
        self.volume - STANDING_HEIGHT * d * d * self.angle().sin() / 2.0
            + (self.named_length() - d) * d * d / 2.0
            - self.lean() * d.powi(3) / 6.0
            + 2.0 * d * half.sin() * d * d / 2.0
            + (self.turn() / 4.0).tan() * d.powi(3) / 3.0
            + d.powi(3) / 6.0 / half.tan()
    }

    /// How far from the corner along each wall a round of `radius` on the
    /// upright leaves it: `r·cot(α/2)`.
    fn tangent(&self, radius: f64) -> f64 {
        radius / (self.angle() / 2.0).tan()
    }

    /// Where the axis of a round of `radius` on the upright stands on the
    /// lower block's top: `radius` in from both walls.
    fn axis(&self, radius: f64) -> DVec3 {
        self.at + self.named.along * self.tangent(radius) - self.named.facing * radius
    }

    /// What the part holds once the named foot and the upright are filleted
    /// at `radius`, as the module's documentation derives it.
    fn filleted(&self, radius: f64) -> f64 {
        let r = radius;
        let tangent = self.tangent(r);
        self.volume - STANDING_HEIGHT * (r * tangent - self.turn() * r * r / 2.0)
            + (self.named_length() - tangent) * r * r * (1.0 - PI / 4.0)
            - self.lean() * r.powi(3) * (5.0 / 6.0 - PI / 4.0)
            + self.turn() * r.powi(3) * (11.0 / 6.0 - PI / 2.0)
    }

    /// The rounds a fillet of `radius` on the two named edges leaves, in the
    /// order of [`SharpCorner::edges`]: the foot's, from the plane through the
    /// upright round's axis square to its wall to the plane of the face at its
    /// far end; and the upright's, from the circle where the piece of torus
    /// meets it, `radius` over the lower block's top, to the standing block's
    /// top.
    fn rounds(&self, radius: f64) -> [Cylinder; 2] {
        let r = radius;
        let foot = &self.named;
        let axis = self.axis(r);
        [
            Cylinder {
                called: foot.called,
                through: self.at + (foot.facing + DVec3::Z) * r,
                along: foot.along,
                radius: r,
                toward: [-DVec3::Z, -foot.facing],
                ends: [
                    (self.at + foot.along * self.tangent(r), -foot.along),
                    (foot.far.at, foot.far.facing),
                ],
                meets: vec![TOP, foot.wall, foot.far.on],
            },
            Cylinder {
                called: self.upright,
                through: axis,
                along: DVec3::Z,
                radius: r,
                toward: [foot.facing, self.sharp.facing],
                ends: [
                    (axis + DVec3::Z * r, -DVec3::Z),
                    (axis + DVec3::Z * STANDING_HEIGHT, DVec3::Z),
                ],
                meets: vec![STANDING_TOP, foot.wall, self.sharp.wall],
            },
        ]
    }

    /// The piece of torus a fillet of `radius` on the two named edges lays
    /// round the upright's round, over the whole of its arc: from the named
    /// foot's round to the plane through its axis square to the sharp foot.
    fn torus(&self, radius: f64) -> Torus {
        Torus {
            axis: self.axis(radius),
            radius,
            from: self.named.facing,
            to: self.sharp.facing,
            sense: 1.0,
            shared: TOP,
        }
    }

    /// What the triangles of a fillet of `radius` on the two named edges may
    /// enclose apart from its matter: the drawing's tolerance at the part's
    /// own reach, over the curved faces — the foot's round along its foot and
    /// a radius past it, the upright's round along the standing block's
    /// height, and the piece of torus, `θ·r²(π − 1)`.
    fn drawn_within(&self, part: &Body, radius: f64) -> f64 {
        let r = radius;
        let foot = PI / 2.0 * r * (self.named_length() + r);
        let upright = self.turn() * r * STANDING_HEIGHT;
        let piece = self.turn() * r * r * (PI - 1.0);
        DRAWN * reach_of(part) * (foot + upright + piece)
    }
}

/// [`stacked`]'s front right foot corner, (20, 5, 10), square: its front foot
/// named, running to the left wall, and its right foot left sharp, running to
/// the back wall.
fn stacked_s_front_right() -> SharpCorner {
    SharpCorner {
        called: "the standing block's front right foot corner",
        part: stacked,
        volume: STACKED_VOLUME,
        at: DVec3::new(20.0, 5.0, LOWER_TOP),
        named: Foot {
            called: "the standing block's front foot",
            wall: STANDING_FRONT,
            facing: -DVec3::Y,
            along: -DVec3::X,
            far: FarEnd {
                on: STANDING_LEFT,
                at: DVec3::new(10.0, 5.0, LOWER_TOP),
                facing: -DVec3::X,
            },
        },
        sharp: Foot {
            called: "the standing block's right foot",
            wall: STANDING_RIGHT,
            facing: DVec3::X,
            along: DVec3::Y,
            far: FarEnd {
                on: STANDING_BACK,
                at: DVec3::new(20.0, 15.0, LOWER_TOP),
                facing: DVec3::Y,
            },
        },
        upright: "the standing block's front right upright",
    }
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
fn taken_as_asked(asked: f64, taken: f64, value: f64, what: &str) {
    assert!(
        (asked - value).abs() < PLACE && (taken - value).abs() < PLACE,
        "{what}: asked {asked} and took {taken}, where {value} fits"
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

/// Holds that a flat face's corners are the points expected, each of them a
/// corner, and no other.
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
            (plane.corners.iter()).any(|corner| corner.distance(*point) < PLACE),
            "{what}: no corner at {point}, among {:?}",
            plane.corners
        );
    }
}

/// A round's cylinder, as the body is to show it: its axis through `through`
/// along `along`, its radius, the two ways from the axis to the lines where
/// it touches the faces it is tangent to, the arc it keeps between them; the
/// planes it ends on, each a point of it and the way out of the round across
/// it; and the faces of the part it meets besides the pieces of torus, one
/// edge each.
struct Cylinder {
    called: &'static str,
    through: DVec3,
    along: DVec3,
    radius: f64,
    toward: [DVec3; 2],
    ends: [(DVec3, DVec3); 2],
    meets: Vec<usize>,
}

impl Cylinder {
    /// Whether a point lies on the cylinder, on the arc between the two
    /// lines where it touches its faces, and nowhere past the planes it ends
    /// on.
    fn holds(&self, point: DVec3) -> bool {
        let off = point - self.through;
        let across = off - self.along * off.dot(self.along);
        let [one, other] = self.toward;
        let least = self.radius * one.dot(other);
        (across.length() - self.radius).abs() < PLACE
            && self
                .toward
                .iter()
                .all(|&way| across.dot(way) >= least - PLACE)
            && (self.ends.iter()).all(|&(at, out)| (point - at).dot(out) <= PLACE)
    }

    /// The four corners of the round: where each of the two lines it touches
    /// its faces along crosses each of the planes it ends on.
    fn corners(&self) -> Vec<DVec3> {
        self.ends
            .iter()
            .flat_map(|&(at, out)| {
                self.toward.map(|way| {
                    let line = self.through + way * self.radius;
                    line + self.along * ((at - line).dot(out) / self.along.dot(out))
                })
            })
            .collect()
    }
}

/// The piece of torus a fillet joins a corner's two like edges' rounds
/// through: about the odd edge round's axis, standing at `axis` on the face
/// the two share, its tube of `radius`, its centre circle twice that from the
/// axis and `radius` over that face where `sense` is one — a fill, the
/// matter between the piece and the walls — or under it where it is minus
/// one — a cut, the matter round the tube's centre — turning from the way
/// `from` out from the axis, where it meets the first edge's round, to the
/// way `to`, where it meets the second's; and the number of the face the two
/// share, which it meets too.
struct Torus {
    axis: DVec3,
    radius: f64,
    from: DVec3,
    to: DVec3,
    sense: f64,
    shared: usize,
}

impl Torus {
    /// The way out from the axis `share` of the way round the piece.
    fn out(&self, share: f64) -> DVec3 {
        let across = (self.to - self.from * self.from.dot(self.to)).normalize();
        let turn = share * self.from.dot(self.to).acos();
        self.from * turn.cos() + across * turn.sin()
    }

    /// The centre of the tube's section the way `out` from the axis.
    fn tube_centre(&self, out: DVec3) -> DVec3 {
        self.axis + out * 2.0 * self.radius + DVec3::Z * self.radius * self.sense
    }

    /// Whether a point lies on the torus, on the quarter of its tube that
    /// faces the axis and the face the two edges share — from `radius` out
    /// from the axis at the tube centre's height to twice that on that face —
    /// and within the turn between the two planes through the axis it ends
    /// on.
    fn holds(&self, point: DVec3) -> bool {
        let off = point - self.axis;
        let level = DVec3::new(off.x, off.y, 0.0);
        let out = level.length();
        let up = off.z * self.sense;
        let r = self.radius;
        let least = out * self.from.dot(self.to);
        ((out - 2.0 * r).hypot(up - r) - r).abs() < PLACE
            && out <= 2.0 * r + PLACE
            && up <= r + PLACE
            && level.dot(self.from) >= least - PLACE
            && level.dot(self.to) >= least - PLACE
    }

    /// The four corners of the piece: where it touches the face the two
    /// edges share and where it touches the odd edge's round, on each of the
    /// two planes it ends on.
    fn corners(&self) -> Vec<DVec3> {
        let r = self.radius;
        [0.0, 1.0]
            .into_iter()
            .flat_map(|share| {
                let out = self.out(share);
                [
                    self.axis + out * 2.0 * r,
                    self.axis + out * r + DVec3::Z * r * self.sense,
                ]
            })
            .collect()
    }
}

/// Holds that a face is curved and drawn, every corner of its triangles
/// where `on` says it lies, and `corners` among them.
fn curved_and_drawn_on(
    body: &Body,
    face: usize,
    on: impl Fn(DVec3) -> bool,
    corners: &[DVec3],
    what: &str,
) {
    assert!(
        !body.is_flat(face) && body.plane_of(face).is_none(),
        "{what} is curved, and offers no plane to draw on"
    );
    let drawn: Vec<DVec3> = body.triangles_of(face).flatten().collect();
    assert!(!drawn.is_empty(), "{what} is drawn");
    for point in &drawn {
        assert!(
            on(*point),
            "{what} is drawn through {point}, off where it lies"
        );
    }
    for corner in corners {
        assert!(
            drawn.iter().any(|point| point.distance(*corner) < PLACE),
            "{what}: no corner at {corner}"
        );
    }
}

/// Holds that the face numbered `face` is the round `round` names: drawn on
/// its cylinder and only there, its four corners among its triangles', and
/// meeting the faces of the part the round names and the pieces of torus
/// `pieces`, one edge each.
fn round_held(body: &Body, face: usize, round: &Cylinder, pieces: &[usize], what: &str) {
    let which = format!("{what}: the round on {}, face {face}", round.called);
    curved_and_drawn_on(
        body,
        face,
        |point| round.holds(point),
        &round.corners(),
        &which,
    );
    let met: Vec<usize> = round.meets.iter().chain(pieces).copied().collect();
    assert_eq!(
        sides_met(body, face),
        sides(&met),
        "{which} meets the faces it lies between, the face at each end and the pieces of torus \
         there, one edge each"
    );
}

/// Holds that the face numbered `piece` is the piece of torus `torus` names:
/// drawn on its torus and only there, its four corners among its triangles',
/// met first by every ray from its tube's centre circle out to it, facing
/// out of the matter, and meeting the face the two like edges share and the
/// faces `meets`, one edge each.
fn piece_held(body: &Body, piece: usize, torus: &Torus, meets: &[usize], what: &str) {
    let which = format!("{what}: the piece of torus, face {piece}");
    curved_and_drawn_on(
        body,
        piece,
        |point| torus.holds(point),
        &torus.corners(),
        &which,
    );
    for around in [0.1, 0.3, 0.5, 0.7, 0.9] {
        let out = torus.out(around);
        let from = torus.tube_centre(out);
        for across in [0.1, 0.3, 0.5, 0.7, 0.9] {
            let turn = across * PI / 2.0;
            let toward = -(out * turn.cos() + DVec3::Z * turn.sin() * torus.sense);
            let facing = -toward * torus.sense;
            let hit = body.ray_hit(from, toward).unwrap_or_else(|| {
                panic!("{which}: a ray from {from}, on its tube's centre circle, meets nothing")
            });
            assert_eq!(
                hit.face, piece,
                "{which}: a ray from {from}, on its tube's centre circle, meets face {} before it",
                hit.face
            );
            assert!(
                hit.normal.distance(facing) < PLACE,
                "{which}: where a ray from {from} meets it, it faces {}, not {facing}, out of the \
                 matter: back toward its tube's centre circle where it fills, away from it where \
                 it cuts",
                hit.normal
            );
        }
    }
    let met: Vec<usize> = [torus.shared].iter().chain(meets).copied().collect();
    assert_eq!(
        sides_met(body, piece),
        sides(&met),
        "{which} meets the face the two like edges share, the rounds round it and the end face, \
         one edge each"
    );
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

/// The sides a face is expected to meet, one edge each, in the order
/// [`sides_met`] lists them.
fn sides(faces: &[usize]) -> Vec<Vec<u32>> {
    let mut sorted: Vec<Vec<u32>> = faces.iter().map(|&face| vec![face as u32]).collect();
    sorted.sort();
    sorted
}

/// The largest coordinate a body reaches, on either side of the origin.
fn reach_of(body: &Body) -> f64 {
    let (low, high) = body.bounds().expect("a part has a box");
    low.abs().max(high.abs()).max_element()
}

/// The two blends a step lays on the corner's edges.
#[derive(Clone, Copy, PartialEq)]
enum Cut {
    Flat,
    Round,
}

/// The body a step left, the value it was asked and the value it took.
struct Laid {
    body: Body,
    asked: f64,
    taken: f64,
}

impl Cut {
    fn called(self) -> &'static str {
        match self {
            Cut::Flat => "chamfer",
            Cut::Round => "fillet",
        }
    }

    /// The step on the edges the names find, in one step, at `value`.
    fn laid_on(self, part: &Body, edges: &[EdgeName], value: f64) -> Result<Laid, Declined> {
        match self {
            Cut::Flat => part.chamfer_edges(edges, value).map(|step| Laid {
                body: step.body,
                asked: step.asked,
                taken: step.taken,
            }),
            Cut::Round => part.fillet_edges(edges, value).map(|step| Laid {
                body: step.body,
                asked: step.asked,
                taken: step.taken,
            }),
        }
    }
}

/// A step on a corner's two named edges: the order the list named them in,
/// by their place in [`SharpCorner::edges`], the value, what the step left,
/// and what to call it.
struct Case {
    order: [usize; 2],
    value: f64,
    step: Laid,
    what: String,
}

impl Case {
    /// The number the face laid on the edge `edge` answers to: where the
    /// steps' numbers of `part` ended, plus the edge's rank in the list.
    fn laid_on(&self, part: &Body, edge: usize) -> usize {
        part.faces_end()
            + self
                .order
                .iter()
                .position(|&named| named == edge)
                .expect("every edge is named")
    }
}

/// Holds that the foot `foot`, left sharp, is still one straight edge between
/// the lower block's top and its wall, from the face at its far end to the
/// face numbered `end`: the one edge between those two faces, the faces met
/// at its ends those two, and its wall's corners where the step leaves them —
/// its own two at its far end, and from `from` up, where the step left the
/// wall, the corner at `from` itself, the one `up` above it where the blend
/// beside it starts, and the one at the wall's top.
fn left_sharp(body: &Body, foot: &Foot, end: usize, (from, up): (DVec3, f64), what: &str) {
    let between = edges_between(body, TOP, foot.wall);
    assert_eq!(
        between.len(),
        1,
        "{what}: {} is one edge between the lower block's top and its wall: {between:?}",
        foot.called
    );
    let mut ends = between[0].ends.clone();
    ends.sort();
    let mut expected = [vec![vec![foot.far.on as u32]], vec![vec![end as u32]]];
    expected.sort();
    assert_eq!(
        ends, expected,
        "{what}: {} runs from the face at its far end to the end face",
        foot.called
    );
    let top = DVec3::Z * STANDING_HEIGHT;
    corners_are(
        body,
        foot.wall,
        &[
            from,
            from + DVec3::Z * up,
            from + top,
            foot.far.at,
            foot.far.at + top,
        ],
        &format!("{what}: the wall of {}", foot.called),
    );
}

/// Holds what a chamfer of its value on a corner's named foot and upright,
/// named in its order, leaves of the part:
///
/// - the value taken whole; a sound body holding what
///   [`SharpCorner::chamfered`] promises;
/// - one flat for each named edge, answering to where the part's numbers
///   ended plus the edge's rank in the list, then the fill along the foot of
///   the upright's flat, then the fill's end face, and the next step's
///   numbers past them; no face of the part gone;
/// - each of those flats facing the way it lies, the fill along the flat's
///   foot and the end face with the corners [`SharpCorner::round_the_flat`]
///   lays out, the end face on the plane square to the sharp foot;
/// - and the sharp foot left one edge, as [`left_sharp`] holds it.
fn stopped_where_the_sharp_foot_begins(corner: &SharpCorner, part: &Body, case: &Case) {
    let Case {
        value, step, what, ..
    } = case;
    let d = *value;
    let start = part.faces_end();
    let patch = start + 2;
    let end = start + 3;
    taken_as_asked(step.asked, step.taken, d, what);
    let body = &step.body;
    let promised = corner.chamfered(d);
    sound_drawn_within(body, promised, EXACT * promised, what);
    assert_eq!(
        faces_made(part, body),
        (start..=end).collect::<Vec<_>>(),
        "{what}: one flat for each named edge, numbered in the order the step names them, then \
         the fill along the foot of the upright's flat, then the fill's end face: {:?}",
        numbers(body)
    );
    assert_eq!(
        body.faces_end(),
        end + 1,
        "{what}: the next step numbers its own faces past the end face"
    );
    assert!(
        numbers(part).is_subset(&numbers(body)),
        "{what}: a face of the part went: {:?}",
        numbers(body)
    );
    let flat = corner.flat_facing();
    let facing = [(DVec3::Z + corner.named.facing).normalize(), flat];
    for (edge, (called, _)) in corner.edges().iter().enumerate() {
        faces_toward(
            body,
            case.laid_on(part, edge),
            facing[edge],
            &format!("{what}: the flat laid on {called}"),
        );
    }
    let laid = corner.round_the_flat(d);
    let up = DVec3::Z * d;
    faces_toward(
        body,
        patch,
        (DVec3::Z + flat).normalize(),
        &format!("{what}: the fill along the foot of the upright's flat"),
    );
    corners_are(
        body,
        patch,
        &[
            laid.leaves_named + up,
            laid.mitre,
            laid.end,
            laid.leaves_sharp + up,
        ],
        &format!(
            "{what}: the fill along the foot of the upright's flat, mitred to the named foot's \
             and ending on the plane square to the sharp foot"
        ),
    );
    faces_toward(
        body,
        end,
        corner.sharp.along,
        &format!(
            "{what}: the fill's end face, square to {}",
            corner.sharp.called
        ),
    );
    corners_are(
        body,
        end,
        &[laid.leaves_sharp, laid.end, laid.leaves_sharp + up],
        &format!(
            "{what}: the fill's end face, where the wall of {} begins",
            corner.sharp.called
        ),
    );
    left_sharp(body, &corner.sharp, end, (laid.leaves_sharp, d), what);
}

/// Holds, at every one of [`VALUES`], the list as given and reversed, what
/// `held` holds of what the blend `cut` leaves of the corner's named foot and
/// upright, and that the two orders leave the same body, face for face.
fn in_both_orders(corner: &SharpCorner, cut: Cut, held: fn(&SharpCorner, &Body, &Case)) {
    let part = (corner.part)();
    assert!(
        (part.volume() - corner.volume).abs() <= EXACT * corner.volume,
        "{} holds {}",
        corner.called,
        part.volume()
    );
    let names = corner
        .edges()
        .map(|(_, [first, second])| edge_between(&part, first, second));
    for value in VALUES {
        let mut laid: Vec<Case> = Vec::new();
        for order in ORDERS_OF_TWO {
            let what = format!(
                "the {} of {value} on {} and {}, {} left sharp, named {order:?}",
                cut.called(),
                corner.named.called,
                corner.upright,
                corner.sharp.called
            );
            let list: Vec<EdgeName> = order.iter().map(|&edge| names[edge].clone()).collect();
            let step = cut
                .laid_on(&part, &list, value)
                .unwrap_or_else(|declined| panic!("{what}: declined, {declined:?}"));
            let case = Case {
                order,
                value,
                step,
                what,
            };
            held(corner, &part, &case);
            laid.push(case);
        }
        let [first, other] = [&laid[0], &laid[1]];
        let to_first: Vec<(usize, usize)> = (0..2)
            .map(|edge| (other.laid_on(&part, edge), first.laid_on(&part, edge)))
            .collect();
        the_same_bodies(
            &first.step.body,
            &other.step.body,
            &to_first,
            &format!("{} and {}", first.what, other.what),
        );
    }
}

#[test]
fn a_chamfer_on_a_foot_and_the_upright_above_it_goes_round_the_upright_s_flat_and_stops_where_the_sharp_foot_begins()
 {
    in_both_orders(
        &stacked_s_front_right(),
        Cut::Flat,
        stopped_where_the_sharp_foot_begins,
    );
}

/// Holds that the face numbered `end` is the end section a fillet of
/// `radius` leaves where its piece of torus ends at the start of the sharp
/// foot's wall: flat, facing along the sharp foot, on the plane square to it
/// through `from`, where the upright's round leaves that wall; its corners
/// on a square of `radius` standing on the lower block's top and out from
/// that wall, less the quarter disc about the square's far top corner — on
/// its foot along that top, on its side up that wall, or on the arc — with
/// the three ends of those among them; and meeting the lower block's top,
/// that wall and the piece of torus `piece`, one edge each.
fn section_held(
    body: &Body,
    end: usize,
    (sharp, from): (&Foot, DVec3),
    (radius, piece): (f64, usize),
    what: &str,
) {
    let which = format!("{what}: the end section, face {end}");
    faces_toward(body, end, sharp.along, &which);
    let plane = body
        .plane_of(end)
        .unwrap_or_else(|| panic!("{which}: the body offers no plane on it"));
    let r = radius;
    for corner in &plane.corners {
        let off = *corner - from;
        let [along, out, up] = [sharp.along, sharp.facing, DVec3::Z].map(|way| off.dot(way));
        let within = |at: f64| (-PLACE..=r + PLACE).contains(&at);
        assert!(
            along.abs() < PLACE
                && ((up.abs() < PLACE && within(out))
                    || (out.abs() < PLACE && within(up))
                    || (((out - r).hypot(up - r) - r).abs() < PLACE && within(out) && within(up))),
            "{which}: a corner at {corner}, off the square of {r} less its quarter disc on the \
             plane square to {} where its wall begins",
            sharp.called
        );
    }
    for point in [from, from + sharp.facing * r, from + DVec3::Z * r] {
        assert!(
            (plane.corners.iter()).any(|corner| corner.distance(point) < PLACE),
            "{which}: no corner at {point}, among {:?}",
            plane.corners
        );
    }
    assert_eq!(
        sides_met(body, end),
        sides(&[TOP, sharp.wall, piece]),
        "{which} meets the lower block's top, the wall of {} and the piece of torus, one edge each",
        sharp.called
    );
}

/// Holds what a fillet of its value on a corner's named foot and upright,
/// named in its order, leaves of the part:
///
/// - the value taken whole; a sound body holding what
///   [`SharpCorner::filleted`] promises;
/// - one round for each named edge, answering to where the part's numbers
///   ended plus the edge's rank in the list, then the piece of torus, then
///   the end section, and the next step's numbers past them; no face of the
///   part gone;
/// - each round on its cylinder and only there, as [`SharpCorner::rounds`]
///   lays them out, meeting the faces of the part it lies between, the one
///   at its far end, and the piece;
/// - the piece of torus on its torus and only there, over the whole arc of
///   the upright's round, from the named foot's round to the plane through
///   its axis square to the sharp foot; met first, facing back, by every ray
///   from its tube's centre circle out to it; meeting the lower block's top,
///   the two rounds and the end section, one edge each;
/// - the end section on that plane, as [`section_held`] holds it;
/// - and the sharp foot left one edge, as [`left_sharp`] holds it.
fn stopped_round_the_round_where_the_sharp_foot_begins(
    corner: &SharpCorner,
    part: &Body,
    case: &Case,
) {
    let Case {
        value, step, what, ..
    } = case;
    let r = *value;
    let start = part.faces_end();
    let piece = start + 2;
    let end = start + 3;
    taken_as_asked(step.asked, step.taken, r, what);
    let body = &step.body;
    let promised = corner.filleted(r);
    sound_drawn_within(body, promised, corner.drawn_within(part, r), what);
    assert_eq!(
        faces_made(part, body),
        (start..=end).collect::<Vec<_>>(),
        "{what}: one round for each named edge, numbered in the order the step names them, then \
         the piece of torus, then the end section: {:?}",
        numbers(body)
    );
    assert_eq!(
        body.faces_end(),
        end + 1,
        "{what}: the next step numbers its own faces past the end section"
    );
    assert!(
        numbers(part).is_subset(&numbers(body)),
        "{what}: a face of the part went: {:?}",
        numbers(body)
    );
    for (edge, round) in corner.rounds(r).iter().enumerate() {
        round_held(body, case.laid_on(part, edge), round, &[piece], what);
    }
    let rounds = [case.laid_on(part, 0), case.laid_on(part, 1)];
    piece_held(
        body,
        piece,
        &corner.torus(r),
        &[rounds[0], rounds[1], end],
        what,
    );
    let from = corner.at + corner.sharp.along * corner.tangent(r);
    section_held(body, end, (&corner.sharp, from), (r, piece), what);
    left_sharp(body, &corner.sharp, end, (from, r), what);
}

#[test]
fn a_fillet_on_a_foot_and_the_upright_above_it_goes_round_the_upright_s_round_and_stops_where_the_sharp_foot_begins()
 {
    in_both_orders(
        &stacked_s_front_right(),
        Cut::Round,
        stopped_round_the_round_where_the_sharp_foot_begins,
    );
}
