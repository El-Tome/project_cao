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
//!   `a_fillet_on_two_feet_of_a_standing_block_and_the_upright_above_them_joins_the_feet_s_rounds_through_a_piece_of_torus_in_every_order`.
//!   Filleted, "shaped as above" is read as what the body shows: each round
//!   drawn on its cylinder only, the feet's from the walls at their far ends
//!   to the planes through the upright round's axis square to their walls,
//!   the upright's from the standing block's top down to the circle the value
//!   above the lower block's top, where the piece takes over from a round run
//!   down to that top; the piece drawn on its torus only, across the quarter
//!   turn and from the lower block's top to the upright's round, met first by
//!   every ray from its tube's centre circle; and each of the four faces
//!   meeting the faces it is tangent to or ends on, one edge each
//! - The same at a corner that is not square: the trapezoid standing block
//!   #801 already uses, at its obtuse and at its acute corner. The arc the
//!   upright's round leaves then spans 180° less the corner's angle; volumes
//!   held by arithmetic or by the booleans —
//!   `a_chamfer_and_a_fillet_on_the_three_edges_at_a_foot_corner_of_a_standing_block_that_is_not_square_go_round_its_upright_in_every_order`.
//!   Read as: both volumes held by arithmetic, the chamfer's also as the body
//!   two steps build, the upright chamfered then its three feet, as at the
//!   square corner; the shape held as at the square corner, the piece of
//!   torus turning through `π − α`. The booleans hold nothing here: no
//!   boolean builds a piece of torus without the turn the kernel lays it with
//! - The four feet and four uprights of a standing block in one step close
//!   its four foot corners —
//!   `a_chamfer_and_a_fillet_on_the_four_feet_and_four_uprights_of_a_standing_block_close_its_four_foot_corners_in_one_step`.
//!   Read as: chamfered and filleted at one value, the volume exact and the
//!   body sound; each corner's patch on its own corner, a piece of torus on
//!   its own torus about its own upright's axis, so that each of the four
//!   ways round the vertical is reached; each flat with the corners the
//!   octagon gives it, and the chamfer the body two steps build, the four
//!   uprights then the eight feet, as at one corner; each round on its
//!   cylinder between the planes through the two corners' upright axes; and
//!   the same body for the list as given
//!   — the four feet, front, right, back and left, then the four uprights
//!   from the front right round — reversed, and with each foot followed by
//!   the upright at its end
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
//! The chamfer sets back `d` along each wall, as `chamfer` alone sets it
//! back on such an upright: its flat takes `5d²·sin α` and is `2d·sin(α/2)`
//! wide at its foot, and the fills meet it at two corners each turning
//! `(π − α)/2`, each mitre adding `t²·tan((π − α)/4)` at the offset `t`. The
//! round, its axis on the corner's bisector `r` in from both walls, leaves
//! them `r·cot(α/2)` from the corner, takes
//! `10·(r²·cot(α/2) − (π − α)·r²/2)`, and its arc spans `π − α`, which the
//! piece of torus turns the section through: `(π − α)·r³(11/6 − π/2)`. The
//! right foot's far end lies on a wall leaning across it, which #801's
//! arithmetic handles: `+0.4` times the section's first moment from the
//! obtuse corner, its far end on the back wall, and `−0.4` times it from the
//! acute one, its far end on the front wall — `d³/6` among flats,
//! `r³(5/6 − π/4)` among rounds. With `L₁` and `L₂` the two feet's lengths
//! from the corner — 10 and `√116` at the obtuse corner, `√116` and 14 at the
//! acute — and `±0.4` that lean, the chamfer leaves
//! `7200 − 5d²·sin α + (L₁ + L₂ − 2d + 2d·sin(α/2))·d²/2 +
//! 2·tan((π − α)/4)·d³/3 ± 0.4·d³/6`, which the bodies two steps build give
//! to a hair at both corners; and the fillet
//! `7200 − 10·(r²·cot(α/2) − (π − α)·r²/2) +
//! (L₁ + L₂ − 2r·cot(α/2))·r²(1 − π/4) ± 0.4·r³(5/6 − π/4) +
//! (π − α)·r³(11/6 − π/2)`. At a square corner both come back to the
//! formulas above.
//!
//! The four feet and four uprights of [`stacked`] leave an octagon among
//! flats, four sides `10 − 2d` and four `d√2`, eight corners of 135°:
//! `7000 − 20d² + (40 − 8d + 4d√2)·d²/2 + 8·tan(22.5°)·d³/3`; and among
//! rounds four rounds of the uprights, four feet `10 − 2r` long and four
//! pieces of torus: `7000 − 40·r²(1 − π/4) + 4(10 − 2r)·r²(1 − π/4) +
//! 2π·r³(11/6 − π/2)`. The octagon is convex, so the fills are the frustum
//! between it and its offset `d` out, less the standing block: over a height
//! `c` the offset `t = d − c` adds the perimeter times `t` and
//! `t²·tan(22.5°)` at each corner, as at one corner. Each flat lies where
//! that frustum puts it: a fill's two corners on the lower block's top where
//! the offset outline turns, `d·tan(22.5°)` past the ends of the side it
//! offsets, and two `d` up the face it fills against, at that face's ends;
//! an upright's flat from `d` up to the standing block's top, between its
//! two set-backs. Held another way in the test itself: the body two steps
//! build, the four uprights chamfered, then the four feet and the four flats'
//! feet filled, which takes the value whole since a set-back is read to reach
//! only where its blend runs, as
//! `rounds_meeting_through_an_end_face_or_along_a_slant_reduce_the_value.rs`
//! holds it.

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
const STANDING_TOP: usize = 7;
const STANDING_FRONT: usize = 8;
const STANDING_RIGHT: usize = 9;
const STANDING_BACK: usize = 10;
const STANDING_LEFT: usize = 11;

/// The lower block of [`stacked`] with a block standing on it whose outline
/// is not a rectangle, #801's trapezoid: its front from (10, 5) to (20, 5),
/// its right wall leaning out from there to (24, 15), its back running from
/// there to (10, 15) and its left wall upright, numbered as [`stacked`]'s.
/// Its corner at the front right foot holds 111.8° of matter, the one at the
/// back right 68.2°; the two on the left are square.
fn leaning_stacked() -> Body {
    joined(
        &joined(
            &Body::default(),
            Contour::rectangle(DVec2::ZERO, DVec2::new(30.0, 20.0)),
            0.0,
            10.0,
        ),
        Contour::straight(vec![
            DVec2::new(10.0, 5.0),
            DVec2::new(20.0, 5.0),
            DVec2::new(24.0, 15.0),
            DVec2::new(10.0, 15.0),
        ]),
        10.0,
        10.0,
    )
}

/// What [`leaning_stacked`] holds: the lower block, and the trapezoid 10 at
/// its front and 14 at its back, 10 deep, raised 10.
const LEANING_STACKED_VOLUME: f64 = 30.0 * 20.0 * 10.0 + (10.0 + 14.0) / 2.0 * 10.0 * 10.0;

/// The way the trapezoid's leaning right wall faces, out of the matter.
fn leaning_wall_facing() -> DVec3 {
    DVec3::new(10.0, -4.0, 0.0).normalize()
}

/// The way the trapezoid's leaning right wall runs, from its front to its
/// back.
fn leaning_wall_along() -> DVec3 {
    DVec3::new(4.0, 10.0, 0.0).normalize()
}

/// The height of the lower block's top, which the standing block stands on.
const LOWER_TOP: f64 = 10.0;

/// How high the standing block stands over the lower block's top.
const STANDING_HEIGHT: f64 = 10.0;

/// The number the flat a chamfer lays on an upright alone answers to: where
/// the part's numbers ended.
const UPRIGHT_FLAT: usize = 12;

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

/// A foot corner of a block standing on another: what it is called, the part
/// and the matter it holds, where the corner stands on the lower block's top,
/// its two feet, and what the upright above them is called. Its three edges
/// are named, in every list here, by their place in [`FootCorner::edges`]:
/// the first foot, the second, and the upright between their two walls.
struct FootCorner {
    called: &'static str,
    part: fn() -> Body,
    volume: f64,
    at: DVec3,
    feet: [Foot; 2],
    upright: &'static str,
}

impl FootCorner {
    /// What each of the corner's three edges is called, and the two faces it
    /// lies between.
    fn edges(&self) -> [(&'static str, [usize; 2]); 3] {
        let [one, other] = &self.feet;
        [
            (one.called, [TOP, one.wall]),
            (other.called, [TOP, other.wall]),
            (self.upright, [one.wall, other.wall]),
        ]
    }

    /// The angle of matter the two walls hold at the corner, `α`.
    fn angle(&self) -> f64 {
        self.feet[0].along.dot(self.feet[1].along).acos()
    }

    /// How far round the upright's round turns, and the piece of torus about
    /// its axis: 180° less the corner's angle.
    fn turn(&self) -> f64 {
        PI - self.angle()
    }

    /// How far from the corner along each wall a round of `radius` on the
    /// upright leaves it: `r·cot(α/2)`.
    fn tangent(&self, radius: f64) -> f64 {
        radius / (self.angle() / 2.0).tan()
    }

    /// Where the axis of a round of `radius` on the upright stands on the
    /// lower block's top: `radius` in from both walls.
    fn axis(&self, radius: f64) -> DVec3 {
        let foot = &self.feet[0];
        self.at + foot.along * self.tangent(radius) - foot.facing * radius
    }

    /// The ways the flats a chamfer lays face, out of the matter, in the order
    /// of [`FootCorner::edges`] — each foot's fill halfway between up and its
    /// wall's way, the upright's flat halfway between its two walls' ways —
    /// and the way the fill along the foot of that flat faces, halfway between
    /// up and the flat's way.
    fn flats_facing(&self) -> ([DVec3; 3], DVec3) {
        let [one, other] = &self.feet;
        let upright = (one.facing + other.facing).normalize();
        (
            [
                (DVec3::Z + one.facing).normalize(),
                (DVec3::Z + other.facing).normalize(),
                upright,
            ],
            (DVec3::Z + upright).normalize(),
        )
    }

    /// What a fill whose section is `section`, its first moment about either
    /// face `moment`, adds along the two feet, from `from` along each from the
    /// corner to the wall at its far end: the section times that length, less,
    /// where that wall leans across the foot, the moment times how far the
    /// lines along the foot shrink a unit out from its wall, as #801's
    /// arithmetic reads it.
    fn along_the_feet(&self, from: f64, section: f64, moment: f64) -> f64 {
        self.feet
            .iter()
            .map(|foot| {
                let length = (foot.far.at - self.at).dot(foot.along) - from;
                let lean = foot.facing.dot(foot.far.facing) / foot.along.dot(foot.far.facing);
                section * length - moment * lean
            })
            .sum()
    }

    /// What the part holds once the corner's three edges are chamfered at
    /// `value`, as the module's documentation derives it: the upright's flat
    /// taken, and at each height the outline it leaves offset out over the
    /// lower block's top, its two corners mitred.
    fn chamfered(&self, value: f64) -> f64 {
        let d = value;
        let half = self.angle() / 2.0;
        self.volume - STANDING_HEIGHT * d * d * self.angle().sin() / 2.0
            + self.along_the_feet(d, d * d / 2.0, d.powi(3) / 6.0)
            + 2.0 * d * half.sin() * d * d / 2.0
            + 2.0 * (self.turn() / 4.0).tan() * d.powi(3) / 3.0
    }

    /// What the part holds once the corner's three edges are filleted at
    /// `radius`, as the module's documentation derives it: the upright's
    /// round taken, the feet's rounds added from where it leaves their walls,
    /// and the piece of torus between them added by Pappus.
    fn filleted(&self, radius: f64) -> f64 {
        let r = radius;
        let tangent = self.tangent(r);
        self.volume - STANDING_HEIGHT * (r * tangent - self.turn() * r * r / 2.0)
            + self.along_the_feet(
                tangent,
                r * r * (1.0 - PI / 4.0),
                r.powi(3) * (5.0 / 6.0 - PI / 4.0),
            )
            + self.turn() * r.powi(3) * (11.0 / 6.0 - PI / 2.0)
    }
}

/// [`stacked`]'s front right foot corner, (20, 5, 10), square: its front
/// foot, running to the left wall, and its right foot, to the back wall.
fn stacked_s_front_right() -> FootCorner {
    FootCorner {
        called: "the standing block's front right foot corner",
        part: stacked,
        volume: STACKED_VOLUME,
        at: DVec3::new(20.0, 5.0, LOWER_TOP),
        feet: [
            Foot {
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
            Foot {
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
        ],
        upright: "the standing block's front right upright",
    }
}

/// [`leaning_stacked`]'s front right foot corner, (20, 5, 10), its obtuse
/// one, 111.8° of matter: its front foot, running to the left wall, and its
/// right foot, along the leaning wall to the back wall, which leans across it.
fn leaning_s_obtuse_corner() -> FootCorner {
    FootCorner {
        called: "the leaning block's front right foot corner, 111.8° of matter",
        part: leaning_stacked,
        volume: LEANING_STACKED_VOLUME,
        at: DVec3::new(20.0, 5.0, LOWER_TOP),
        feet: [
            Foot {
                called: "the leaning block's front foot",
                wall: STANDING_FRONT,
                facing: -DVec3::Y,
                along: -DVec3::X,
                far: FarEnd {
                    on: STANDING_LEFT,
                    at: DVec3::new(10.0, 5.0, LOWER_TOP),
                    facing: -DVec3::X,
                },
            },
            Foot {
                called: "the leaning block's right foot",
                wall: STANDING_RIGHT,
                facing: leaning_wall_facing(),
                along: leaning_wall_along(),
                far: FarEnd {
                    on: STANDING_BACK,
                    at: DVec3::new(24.0, 15.0, LOWER_TOP),
                    facing: DVec3::Y,
                },
            },
        ],
        upright: "the leaning block's front right upright",
    }
}

/// [`leaning_stacked`]'s back right foot corner, (24, 15, 10), its acute one,
/// 68.2° of matter: its right foot, along the leaning wall to the front wall,
/// which leans across it, and its back foot, running to the left wall.
fn leaning_s_acute_corner() -> FootCorner {
    FootCorner {
        called: "the leaning block's back right foot corner, 68.2° of matter",
        part: leaning_stacked,
        volume: LEANING_STACKED_VOLUME,
        at: DVec3::new(24.0, 15.0, LOWER_TOP),
        feet: [
            Foot {
                called: "the leaning block's right foot",
                wall: STANDING_RIGHT,
                facing: leaning_wall_facing(),
                along: -leaning_wall_along(),
                far: FarEnd {
                    on: STANDING_FRONT,
                    at: DVec3::new(20.0, 5.0, LOWER_TOP),
                    facing: -DVec3::Y,
                },
            },
            Foot {
                called: "the leaning block's back foot",
                wall: STANDING_BACK,
                facing: DVec3::Y,
                along: -DVec3::X,
                far: FarEnd {
                    on: STANDING_LEFT,
                    at: DVec3::new(10.0, 15.0, LOWER_TOP),
                    facing: -DVec3::X,
                },
            },
        ],
        upright: "the leaning block's back right upright",
    }
}

/// The corner's part chamfered at `value` in two steps, as #801 builds it:
/// the upright alone first, the flat laid on it answering to
/// [`UPRIGHT_FLAT`] and stopping flush on the lower block's top; then, in one
/// step, the two feet and the foot of that flat, numbered in that order past
/// it, as [`IN_TWO_STEPS`] says.
fn chamfered_in_two_steps(corner: &FootCorner, value: f64) -> Body {
    let part = (corner.part)();
    let [one, other] = corner.feet.each_ref().map(|foot| foot.wall);
    let upright = part
        .chamfer(&edge_between(&part, one, other), value)
        .unwrap_or_else(|declined| panic!("the upright alone is chamfered: {declined:?}"));
    taken_as_asked(
        upright.asked,
        upright.taken,
        value,
        "the upright chamfered alone",
    );
    let chamfered = upright.body;
    let feet = [
        edge_between(&chamfered, TOP, one),
        edge_between(&chamfered, TOP, other),
        edge_between(&chamfered, TOP, UPRIGHT_FLAT),
    ];
    let filled = chamfered
        .chamfer_edges(&feet, value)
        .unwrap_or_else(|declined| {
            panic!("the three feet round the chamfered upright are filled: {declined:?}")
        });
    taken_as_asked(
        filled.asked,
        filled.taken,
        value,
        "the three feet round the chamfered upright",
    );
    filled.body
}

/// The numbers the flats on a corner's three edges answer to in the body
/// [`chamfered_in_two_steps`] leaves, in the order of [`FootCorner::edges`],
/// and the number of the fill along the foot of the upright's flat.
const IN_TWO_STEPS: ([usize; 3], usize) = (
    [UPRIGHT_FLAT + 1, UPRIGHT_FLAT + 2, UPRIGHT_FLAT],
    UPRIGHT_FLAT + 3,
);

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

/// A step on a list of edges: the order the list named them in, by their
/// place in [`FootCorner::edges`] or in [`FEET_AND_UPRIGHTS`], the blend, the
/// value, what the step left, and what to call it.
struct Case {
    order: Vec<usize>,
    cut: Cut,
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

    /// The numbers the patches at `corners` answer to, each corner given by
    /// its three edges' places: past every edge's face, one each, in the order
    /// of the lowest rank the list gives one of a corner's edges, then the
    /// next, as #699 numbers its patches.
    fn patches(&self, part: &Body, corners: &[[usize; 3]]) -> Vec<usize> {
        let rank = |edge: usize| self.laid_on(part, edge) - part.faces_end();
        let mut by_rank: Vec<(usize, [usize; 3])> = corners
            .iter()
            .enumerate()
            .map(|(index, edges)| {
                let mut ranks = edges.map(rank);
                ranks.sort();
                (index, ranks)
            })
            .collect();
        by_rank.sort_by_key(|&(_, ranks)| ranks);
        let mut numbers = vec![0; corners.len()];
        for (place, (index, _)) in by_rank.into_iter().enumerate() {
            numbers[index] = part.faces_end() + self.order.len() + place;
        }
        numbers
    }
}

/// Holds what a chamfer of its value on a corner's three edges, named in its
/// order, leaves of the part:
///
/// - the value taken whole; a sound body holding what
///   [`FootCorner::chamfered`] promises;
/// - one flat for each edge, answering to where the part's numbers ended
///   plus the edge's rank in the list, then the fill along the foot of the
///   upright's flat, which no name finds, and the next step's numbers past
///   it; no face of the part gone;
/// - each of those flats facing the way it lies;
/// - and `two_steps`, the body the upright chamfered and then its three feet
///   leave, the same body, face for face.
fn went_round_the_upright(corner: &FootCorner, part: &Body, case: &Case, two_steps: &Body) {
    let Case {
        value, step, what, ..
    } = case;
    let start = part.faces_end();
    let patch = start + 3;
    taken_as_asked(step.asked, step.taken, *value, what);
    let body = &step.body;
    let promised = corner.chamfered(*value);
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
    let (facing, foot_facing) = corner.flats_facing();
    for (edge, (called, _)) in corner.edges().iter().enumerate() {
        faces_toward(
            body,
            case.laid_on(part, edge),
            facing[edge],
            &format!("{what}: the flat laid on {called}"),
        );
    }
    faces_toward(
        body,
        patch,
        foot_facing,
        &format!("{what}: the fill along the foot of the upright's flat"),
    );
    let (edges_in_two_steps, foot_in_two_steps) = IN_TWO_STEPS;
    let to_two_steps: Vec<(usize, usize)> = (0..3)
        .map(|edge| (case.laid_on(part, edge), edges_in_two_steps[edge]))
        .chain([(patch, foot_in_two_steps)])
        .collect();
    the_same_bodies(
        two_steps,
        body,
        &to_two_steps,
        &format!("{what}, and the body two steps leave, the upright chamfered then its three feet"),
    );
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

/// The piece of torus a fillet joins a corner's feet's rounds through: about
/// the upright round's axis, standing at `axis` on the lower block's top, its
/// tube of `radius`, its centre circle twice that from the axis and `radius`
/// above the lower block's top, turning from the way `from` out from the
/// axis, where it meets the first foot's round, to the way `to`, where it
/// meets the second's.
struct Torus {
    axis: DVec3,
    radius: f64,
    from: DVec3,
    to: DVec3,
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
        self.axis + out * 2.0 * self.radius + DVec3::Z * self.radius
    }

    /// Whether a point lies on the torus, on the quarter of its tube that
    /// faces the axis and the lower block's top — from `radius` out from the
    /// axis at the tube centre's height to twice that on the lower block's
    /// top — and within the turn between the two planes through the axis it
    /// ends on.
    fn holds(&self, point: DVec3) -> bool {
        let off = point - self.axis;
        let level = DVec3::new(off.x, off.y, 0.0);
        let out = level.length();
        let up = off.z;
        let r = self.radius;
        let least = out * self.from.dot(self.to);
        ((out - 2.0 * r).hypot(up - r) - r).abs() < PLACE
            && out <= 2.0 * r + PLACE
            && up <= r + PLACE
            && level.dot(self.from) >= least - PLACE
            && level.dot(self.to) >= least - PLACE
    }

    /// The four corners of the piece: where it touches the lower block's top
    /// and where it touches the upright's round, on each of the two planes it
    /// ends on.
    fn corners(&self) -> Vec<DVec3> {
        let r = self.radius;
        [0.0, 1.0]
            .into_iter()
            .flat_map(|share| {
                let out = self.out(share);
                [
                    self.axis + out * 2.0 * r,
                    self.axis + out * r + DVec3::Z * r,
                ]
            })
            .collect()
    }
}

impl FootCorner {
    /// The rounds a fillet of `radius` on the corner's three edges leaves, in
    /// the order of [`FootCorner::edges`]: each foot's, from the plane through
    /// the upright round's axis square to its wall to the plane of the wall
    /// at its far end; and the upright's, from the circle where the piece of
    /// torus meets it, `radius` above the lower block's top, to the standing
    /// block's top.
    fn rounds(&self, radius: f64) -> [Cylinder; 3] {
        let r = radius;
        let tangent = self.tangent(r);
        let [one, other] = &self.feet;
        let foot = |foot: &Foot| Cylinder {
            called: foot.called,
            through: self.at + foot.facing * r + DVec3::Z * r,
            along: foot.along,
            radius: r,
            toward: [-DVec3::Z, -foot.facing],
            ends: [
                (self.at + foot.along * tangent, -foot.along),
                (foot.far.at, foot.far.facing),
            ],
            meets: vec![TOP, foot.wall, foot.far.on],
        };
        let axis = self.axis(r);
        [
            foot(one),
            foot(other),
            Cylinder {
                called: self.upright,
                through: axis,
                along: DVec3::Z,
                radius: r,
                toward: [one.facing, other.facing],
                ends: [
                    (axis + DVec3::Z * r, -DVec3::Z),
                    (axis + DVec3::Z * STANDING_HEIGHT, DVec3::Z),
                ],
                meets: vec![STANDING_TOP, one.wall, other.wall],
            },
        ]
    }

    /// The piece of torus a fillet of `radius` on the corner's three edges
    /// joins the feet's rounds through.
    fn torus(&self, radius: f64) -> Torus {
        Torus {
            axis: self.axis(radius),
            radius,
            from: self.feet[0].facing,
            to: self.feet[1].facing,
        }
    }

    /// What the triangles of a fillet of `radius` on the corner's three edges
    /// may enclose apart from its matter: the drawing's tolerance at the
    /// part's own reach, over the curved faces — the feet's rounds along their
    /// feet and a radius past them, the upright's round along the standing
    /// block's height, and the piece of torus, `θ·r²(π − 1)`.
    fn drawn_within(&self, part: &Body, radius: f64) -> f64 {
        let r = radius;
        let feet: f64 = (self.feet.iter())
            .map(|foot| PI / 2.0 * r * ((foot.far.at - self.at).length() + r))
            .sum();
        let upright = self.turn() * r * STANDING_HEIGHT;
        let piece = self.turn() * r * r * (PI - 1.0);
        DRAWN * reach_of(part) * (feet + upright + piece)
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
/// met first, facing back, by every ray from its tube's centre circle out to
/// it, and meeting the lower block's top and the rounds `rounds`, one edge
/// each.
fn piece_held(body: &Body, piece: usize, torus: &Torus, rounds: &[usize], what: &str) {
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
            let toward = -(out * turn.cos() + DVec3::Z * turn.sin());
            let hit = body.ray_hit(from, toward).unwrap_or_else(|| {
                panic!("{which}: a ray from {from}, on its tube's centre circle, meets nothing")
            });
            assert_eq!(
                hit.face, piece,
                "{which}: a ray from {from}, on its tube's centre circle, meets face {} before it",
                hit.face
            );
            assert!(
                hit.normal.distance(-toward) < PLACE,
                "{which}: where a ray from {from} meets it, it faces {}, not {}, back toward its \
                 tube's centre circle",
                hit.normal,
                -toward
            );
        }
    }
    let met: Vec<usize> = [TOP].iter().chain(rounds).copied().collect();
    assert_eq!(
        sides_met(body, piece),
        sides(&met),
        "{which} meets the lower block's top and the rounds round it, one edge each"
    );
}

/// Holds what a fillet of its value on a corner's three edges, named in its
/// order, leaves of the part:
///
/// - the value taken whole; a sound body holding what
///   [`FootCorner::filleted`] promises;
/// - one round for each edge, answering to where the part's numbers ended
///   plus the edge's rank in the list, then the piece of torus, and the next
///   step's numbers past it; nothing else made, no face of the part gone;
/// - each round on its cylinder and only there, as [`FootCorner::rounds`]
///   lays them out; each meeting the faces of the part it lies between, the
///   one at its far end, and the piece;
/// - the piece of torus on its torus and only there, from where it touches
///   the lower block's top to where it touches the upright's round, across
///   the turn between the feet's rounds; met first, facing back, by every ray
///   from its tube's centre circle out to it; meeting the lower block's top
///   and the three rounds, one edge each.
fn joined_through_a_piece_of_torus(corner: &FootCorner, part: &Body, case: &Case) {
    let Case {
        value, step, what, ..
    } = case;
    let r = *value;
    taken_as_asked(step.asked, step.taken, r, what);
    let body = &step.body;
    let promised = corner.filleted(r);
    sound_drawn_within(body, promised, corner.drawn_within(part, r), what);
    let start = part.faces_end();
    let piece = start + 3;
    assert_eq!(
        faces_made(part, body),
        (start..=piece).collect::<Vec<_>>(),
        "{what}: one round for each edge, numbered in the order the step names them, and the \
         piece of torus after them: {:?}",
        numbers(body)
    );
    assert_eq!(
        body.faces_end(),
        piece + 1,
        "{what}: the next step numbers its own faces past the piece of torus"
    );
    assert!(
        numbers(part).is_subset(&numbers(body)),
        "{what}: a face of the part went: {:?}",
        numbers(body)
    );

    for (edge, round) in corner.rounds(r).iter().enumerate() {
        round_held(body, case.laid_on(part, edge), round, &[piece], what);
    }
    let laid: Vec<usize> = (0..3).map(|edge| case.laid_on(part, edge)).collect();
    piece_held(body, piece, &corner.torus(r), &laid, what);
}

/// Holds that the three edges of each of `corners`, named in every one of
/// [`ORDERS_OF_THREE`] and blended by each of `cuts` at every one of
/// [`VALUES`] in one step, go round the upright: a chamfer as
/// [`went_round_the_upright`] holds it, against the body two steps leave,
/// whose matter the arithmetic gives too; a fillet as
/// [`joined_through_a_piece_of_torus`] holds it; and the same body whatever
/// the order, the face laid on each edge answering to the number its rank
/// gives it. Every list is laid before any body is read, so that a decline
/// names every case that declines.
fn round_the_upright_in_every_order(corners: &[FootCorner], cuts: &[Cut], called: &str) {
    let parts: Vec<Body> = corners.iter().map(|corner| (corner.part)()).collect();
    let mut in_two_steps = Vec::new();
    for corner in corners.iter().filter(|_| cuts.contains(&Cut::Flat)) {
        for value in VALUES {
            let promised = corner.chamfered(value);
            let two_steps = chamfered_in_two_steps(corner, value);
            assert!(
                (two_steps.volume() - promised).abs() <= EXACT * promised,
                "{}, the chamfer of {value} in two steps, the upright then its three feet: the \
                 body holds {}, where arithmetic promised {promised}",
                corner.called,
                two_steps.volume()
            );
            in_two_steps.push((corner.called, value, two_steps));
        }
    }
    let mut laid = Vec::new();
    let mut declined = Vec::new();
    for (corner, part) in corners.iter().zip(&parts) {
        let edges = corner.edges();
        let names = edges.map(|(_, [first, second])| edge_between(part, first, second));
        for &cut in cuts {
            for value in VALUES {
                for order in ORDERS_OF_THREE {
                    let list: Vec<EdgeName> =
                        order.iter().map(|&edge| names[edge].clone()).collect();
                    let what = format!(
                        "the {} of {value} on {}, named {:?}",
                        cut.called(),
                        corner.called,
                        order.map(|edge| edges[edge].0)
                    );
                    let order = order.to_vec();
                    match cut.laid_on(part, &list, value) {
                        Ok(step) => laid.push((
                            corner,
                            part,
                            Case {
                                order,
                                cut,
                                value,
                                step,
                                what,
                            },
                        )),
                        Err(reason) => declined.push(format!("{what}: {reason:?}")),
                    }
                }
            }
        }
    }
    assert!(
        declined.is_empty(),
        "{called}: the three edges at the corner laid in one step, the upright first and the feet \
         round it; declined:\n{}",
        declined.join("\n")
    );
    for (corner, part, case) in &laid {
        match case.cut {
            Cut::Flat => {
                let (_, _, two_steps) = in_two_steps
                    .iter()
                    .find(|(at, value, _)| *at == corner.called && *value == case.value)
                    .expect("the body two steps leave is built at every value");
                went_round_the_upright(corner, part, case, two_steps);
            }
            Cut::Round => joined_through_a_piece_of_torus(corner, part, case),
        }
    }
    for (corner, part, case) in &laid {
        let (_, _, first) = laid
            .iter()
            .find(|(at, _, other)| {
                at.called == corner.called && other.cut == case.cut && other.value == case.value
            })
            .expect("the case itself is among them");
        if std::ptr::eq(first, case) {
            continue;
        }
        let to_first: Vec<(usize, usize)> = (0..3)
            .map(|edge| (case.laid_on(part, edge), first.laid_on(part, edge)))
            .collect();
        the_same_bodies(
            &first.step.body,
            &case.step.body,
            &to_first,
            &format!("{} and {}", first.what, case.what),
        );
    }
}

#[test]
fn a_chamfer_on_two_feet_of_a_standing_block_and_the_upright_above_them_fills_the_foot_of_the_upright_s_flat_and_mitres_the_three_fills_in_every_order()
 {
    round_the_upright_in_every_order(
        &[stacked_s_front_right()],
        &[Cut::Flat],
        "the standing block's front right foot corner chamfered",
    );
}

#[test]
fn a_fillet_on_two_feet_of_a_standing_block_and_the_upright_above_them_joins_the_feet_s_rounds_through_a_piece_of_torus_in_every_order()
 {
    round_the_upright_in_every_order(
        &[stacked_s_front_right()],
        &[Cut::Round],
        "the standing block's front right foot corner filleted",
    );
}

#[test]
fn a_chamfer_and_a_fillet_on_the_three_edges_at_a_foot_corner_of_a_standing_block_that_is_not_square_go_round_its_upright_in_every_order()
 {
    round_the_upright_in_every_order(
        &[leaning_s_obtuse_corner(), leaning_s_acute_corner()],
        &[Cut::Flat, Cut::Round],
        "the leaning block's obtuse and acute foot corners chamfered and filleted",
    );
}

/// The eight edges at [`stacked`]'s standing block's feet, by the two faces
/// each lies between, in the order the list as given names them: its four
/// feet, front, right, back and left, then its four uprights, front right,
/// back right, back left and front left.
const FEET_AND_UPRIGHTS: [[usize; 2]; 8] = [
    [TOP, STANDING_FRONT],
    [TOP, STANDING_RIGHT],
    [TOP, STANDING_BACK],
    [TOP, STANDING_LEFT],
    [STANDING_FRONT, STANDING_RIGHT],
    [STANDING_RIGHT, STANDING_BACK],
    [STANDING_BACK, STANDING_LEFT],
    [STANDING_LEFT, STANDING_FRONT],
];

/// For each corner of [`stacked_s_foot_corners`], its three edges' places in
/// [`FEET_AND_UPRIGHTS`], in the order of [`FootCorner::edges`]: its first
/// foot, its second, its upright.
const AT_THE_CORNERS: [[usize; 3]; 4] = [[0, 1, 4], [1, 2, 5], [2, 3, 6], [3, 0, 7]];

/// For each foot of [`FEET_AND_UPRIGHTS`], the two corners of
/// [`stacked_s_foot_corners`] it runs between, the one it runs from first.
const FEET_BETWEEN: [[usize; 2]; 4] = [[3, 0], [0, 1], [1, 2], [2, 3]];

/// The orders the eight edges are named in, by their places in
/// [`FEET_AND_UPRIGHTS`]: as given, the four feet then the four uprights;
/// reversed; and each foot followed by the upright at its end, the feet and
/// uprights alternating.
const ALL_ROUND_ORDERS: [[usize; 8]; 3] = [
    [0, 1, 2, 3, 4, 5, 6, 7],
    [7, 6, 5, 4, 3, 2, 1, 0],
    [0, 4, 1, 5, 2, 6, 3, 7],
];

/// [`stacked`]'s four foot corners, in turn round its standing block from the
/// front right, each with the foot before it in that turn first, as
/// [`stacked_s_front_right`] has them, and each foot's far end at the next
/// corner along it.
fn stacked_s_foot_corners() -> [FootCorner; 4] {
    let at = |x: f64, y: f64| DVec3::new(x, y, LOWER_TOP);
    let foot = |called, wall, facing, along, far: (usize, DVec3, DVec3)| Foot {
        called,
        wall,
        facing,
        along,
        far: FarEnd {
            on: far.0,
            at: far.1,
            facing: far.2,
        },
    };
    let corner = |called, at, feet, upright| FootCorner {
        called,
        part: stacked,
        volume: STACKED_VOLUME,
        at,
        feet,
        upright,
    };
    [
        stacked_s_front_right(),
        corner(
            "the standing block's back right foot corner",
            at(20.0, 15.0),
            [
                foot(
                    "the standing block's right foot",
                    STANDING_RIGHT,
                    DVec3::X,
                    -DVec3::Y,
                    (STANDING_FRONT, at(20.0, 5.0), -DVec3::Y),
                ),
                foot(
                    "the standing block's back foot",
                    STANDING_BACK,
                    DVec3::Y,
                    -DVec3::X,
                    (STANDING_LEFT, at(10.0, 15.0), -DVec3::X),
                ),
            ],
            "the standing block's back right upright",
        ),
        corner(
            "the standing block's back left foot corner",
            at(10.0, 15.0),
            [
                foot(
                    "the standing block's back foot",
                    STANDING_BACK,
                    DVec3::Y,
                    DVec3::X,
                    (STANDING_RIGHT, at(20.0, 15.0), DVec3::X),
                ),
                foot(
                    "the standing block's left foot",
                    STANDING_LEFT,
                    -DVec3::X,
                    -DVec3::Y,
                    (STANDING_FRONT, at(10.0, 5.0), -DVec3::Y),
                ),
            ],
            "the standing block's back left upright",
        ),
        corner(
            "the standing block's front left foot corner",
            at(10.0, 5.0),
            [
                foot(
                    "the standing block's left foot",
                    STANDING_LEFT,
                    -DVec3::X,
                    DVec3::Y,
                    (STANDING_BACK, at(10.0, 15.0), DVec3::Y),
                ),
                foot(
                    "the standing block's front foot",
                    STANDING_FRONT,
                    -DVec3::Y,
                    DVec3::X,
                    (STANDING_RIGHT, at(20.0, 5.0), DVec3::X),
                ),
            ],
            "the standing block's front left upright",
        ),
    ]
}

/// What [`stacked`] holds once its four feet and four uprights are chamfered
/// at `value` in one step, as the module's documentation derives it: the
/// four uprights' flats taken, and at each height the octagon they leave
/// offset out over the lower block's top, its eight corners of 135° mitred.
fn chamfered_all_round(value: f64) -> f64 {
    let d = value;
    STACKED_VOLUME - 4.0 * STANDING_HEIGHT * d * d / 2.0
        + (40.0 - 8.0 * d + 4.0 * 2f64.sqrt() * d) * d * d / 2.0
        + 8.0 * (PI / 8.0).tan() * d.powi(3) / 3.0
}

/// What [`stacked`] holds once its four feet and four uprights are filleted
/// at `radius` in one step, as the module's documentation derives it: the
/// four uprights' rounds taken, the four feet's added between them, and the
/// four pieces of torus, each a quarter turn.
fn filleted_all_round(radius: f64) -> f64 {
    let r = radius;
    let section = r * r * (1.0 - PI / 4.0);
    STACKED_VOLUME - 4.0 * STANDING_HEIGHT * section
        + 4.0 * (10.0 - 2.0 * r) * section
        + 4.0 * PI / 2.0 * r.powi(3) * (11.0 / 6.0 - PI / 2.0)
}

/// [`stacked`] chamfered at `value` in two steps, as #801 builds it: its four
/// uprights first, each flat stopping flush on the lower block's top and
/// numbered from [`UPRIGHT_FLAT`] in the order of [`FEET_AND_UPRIGHTS`]; then,
/// in one step, its four feet and the feet of those four flats, numbered past
/// them in that order, as [`in_two_steps_all_round`] says.
fn chamfered_all_round_in_two_steps(value: f64) -> Body {
    let part = stacked();
    let uprights: Vec<EdgeName> = FEET_AND_UPRIGHTS[4..]
        .iter()
        .map(|&[first, second]| edge_between(&part, first, second))
        .collect();
    let upright = part
        .chamfer_edges(&uprights, value)
        .unwrap_or_else(|declined| panic!("the four uprights are chamfered: {declined:?}"));
    taken_as_asked(
        upright.asked,
        upright.taken,
        value,
        "the four uprights chamfered",
    );
    let chamfered = upright.body;
    let feet: Vec<EdgeName> = FEET_AND_UPRIGHTS[..4]
        .iter()
        .map(|&[first, second]| edge_between(&chamfered, first, second))
        .chain((0..4).map(|flat| edge_between(&chamfered, TOP, UPRIGHT_FLAT + flat)))
        .collect();
    let filled = chamfered
        .chamfer_edges(&feet, value)
        .unwrap_or_else(|declined| {
            panic!("the eight feet round the chamfered uprights are filled: {declined:?}")
        });
    taken_as_asked(
        filled.asked,
        filled.taken,
        value,
        "the eight feet round the chamfered uprights",
    );
    filled.body
}

/// The number the flat on the edge at `edge` in [`FEET_AND_UPRIGHTS`]
/// answers to in the body [`chamfered_all_round_in_two_steps`] leaves: a
/// foot's in the second step, an upright's in the first.
fn in_two_steps_all_round(edge: usize) -> usize {
    match edge {
        foot @ 0..4 => UPRIGHT_FLAT + 4 + foot,
        upright => UPRIGHT_FLAT + upright - 4,
    }
}

/// The number the fill along the foot of the flat at the corner `corner` of
/// [`stacked_s_foot_corners`] answers to in the body
/// [`chamfered_all_round_in_two_steps`] leaves: the last four of its second
/// step.
fn flat_s_foot_in_two_steps_all_round(corner: usize) -> usize {
    UPRIGHT_FLAT + 8 + corner
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

/// The corners of the twelve flats a chamfer of `value` on [`stacked`]'s four
/// feet and four uprights lays, as the module's documentation lays them out:
/// for each edge of [`FEET_AND_UPRIGHTS`], then for the fill along the foot
/// of the flat at each corner of `corners`. Each fill's corners are two on
/// the lower block's top, where the outline the uprights' flats leave,
/// offset the value out, turns, `value·tan(22.5°)` past its own ends; and two
/// the value up the face it fills against, at that face's ends. Each
/// upright's flat runs from the value up to the standing block's top,
/// between the two set-backs on its walls.
fn flats_all_round(corners: &[FootCorner; 4], value: f64) -> ([Vec<DVec3>; 8], [Vec<DVec3>; 4]) {
    let d = value;
    let past = d * (PI / 8.0).tan();
    let up = DVec3::Z * d;
    let fill = |from: DVec3, to: DVec3, out: DVec3| {
        let along = (to - from).normalize();
        vec![
            from + out * d - along * past,
            to + out * d + along * past,
            from + up,
            to + up,
        ]
    };
    let set_backs = |corner: &FootCorner| {
        corner
            .feet
            .each_ref()
            .map(|foot| corner.at + foot.along * d)
    };
    let mut edges: [Vec<DVec3>; 8] = Default::default();
    for (foot, [from, to]) in FEET_BETWEEN.into_iter().enumerate() {
        let wall = FEET_AND_UPRIGHTS[foot][1];
        let [start, end] = [from, to].map(|corner| corners[corner].at);
        let along = (end - start).normalize();
        let out = (corners[from].feet.iter())
            .find(|at| at.wall == wall)
            .expect("a foot runs from the corner it starts at")
            .facing;
        edges[foot] = fill(start + along * d, end - along * d, out);
    }
    let mut feet: [Vec<DVec3>; 4] = Default::default();
    for (index, corner) in corners.iter().enumerate() {
        let [one, other] = set_backs(corner);
        let top = DVec3::Z * STANDING_HEIGHT;
        edges[AT_THE_CORNERS[index][2]] = vec![one + up, other + up, one + top, other + top];
        let (facing, _) = corner.flats_facing();
        feet[index] = fill(one, other, facing[2]);
    }
    (edges, feet)
}

/// Holds what a step of its blend and value on [`stacked`]'s four feet and
/// four uprights, named in its order, leaves of the part:
///
/// - the value taken whole; a sound body holding what
///   [`chamfered_all_round`] or [`filleted_all_round`] promises;
/// - one face for each edge, answering to where the part's numbers ended
///   plus the edge's rank in the list, then one patch at each of the four
///   corners — the fill along the foot of the upright's flat, or the piece of
///   torus — numbered as [`Case::patches`] says; the next step's numbers past
///   them; nothing else made, no face of the part gone;
/// - among flats, each flat facing the way it lies, its corners where
///   [`flats_all_round`] puts them and nowhere else; and `two_steps`, the
///   body the four uprights chamfered and then the eight feet leave, the same
///   body, face for face;
/// - among rounds, each upright's round on its cylinder as
///   [`FootCorner::rounds`] lays it out, meeting its corner's piece of torus;
///   each foot's round on its cylinder from the plane through one corner's
///   upright round's axis to the other's, meeting the pieces of torus at both
///   ends; and each corner's piece of torus on its own torus, as
///   [`piece_held`] holds it.
fn closed_all_round(part: &Body, corners: &[FootCorner; 4], case: &Case, two_steps: &Body) {
    let Case {
        cut,
        value,
        step,
        what,
        ..
    } = case;
    let r = *value;
    taken_as_asked(step.asked, step.taken, r, what);
    let body = &step.body;
    let start = part.faces_end();
    let patches = case.patches(part, &AT_THE_CORNERS);
    let laid = |edge: usize| case.laid_on(part, edge);
    match cut {
        Cut::Flat => {
            let promised = chamfered_all_round(r);
            sound_drawn_within(body, promised, EXACT * promised, what);
        }
        Cut::Round => {
            let promised = filleted_all_round(r);
            let curved = PI / 2.0 * r * (4.0 * 10.0 + 4.0 * STANDING_HEIGHT)
                + 4.0 * PI / 2.0 * r * r * (PI - 1.0);
            sound_drawn_within(body, promised, DRAWN * reach_of(part) * curved, what);
        }
    }
    assert_eq!(
        faces_made(part, body),
        (start..start + 12).collect::<Vec<_>>(),
        "{what}: one face for each edge, numbered in the order the step names them, and a patch \
         at each of the four corners after them: {:?}",
        numbers(body)
    );
    assert_eq!(
        body.faces_end(),
        start + 12,
        "{what}: the next step numbers its own faces past the four patches"
    );
    assert!(
        numbers(part).is_subset(&numbers(body)),
        "{what}: a face of the part went: {:?}",
        numbers(body)
    );
    let foot_wall = |foot: usize| {
        let [corner, _] = FEET_BETWEEN[foot];
        let wall = FEET_AND_UPRIGHTS[foot][1];
        corners[corner]
            .feet
            .iter()
            .find(|at| at.wall == wall)
            .expect("a foot runs from the corner it starts at")
            .facing
    };
    match cut {
        Cut::Flat => {
            let (edges, feet) = flats_all_round(corners, r);
            for (edge, expected) in edges.iter().enumerate() {
                let which = format!("{what}: the flat on {:?}", FEET_AND_UPRIGHTS[edge]);
                let facing = match edge {
                    foot @ 0..4 => (DVec3::Z + foot_wall(foot)).normalize(),
                    upright => {
                        let corner = (0..4)
                            .find(|&corner| AT_THE_CORNERS[corner][2] == upright)
                            .expect("each upright stands at a corner");
                        corners[corner].flats_facing().0[2]
                    }
                };
                faces_toward(body, laid(edge), facing, &which);
                corners_are(body, laid(edge), expected, &which);
            }
            for (index, corner) in corners.iter().enumerate() {
                let which = format!(
                    "{what}: the fill along the foot of the flat on {}, face {}",
                    corner.upright, patches[index]
                );
                faces_toward(body, patches[index], corner.flats_facing().1, &which);
                corners_are(body, patches[index], &feet[index], &which);
            }
            let to_two_steps: Vec<(usize, usize)> =
                (0..8)
                    .map(|edge| (laid(edge), in_two_steps_all_round(edge)))
                    .chain((0..4).map(|corner| {
                        (patches[corner], flat_s_foot_in_two_steps_all_round(corner))
                    }))
                    .collect();
            the_same_bodies(
                two_steps,
                body,
                &to_two_steps,
                &format!(
                    "{what}, and the body two steps leave, the four uprights chamfered then the \
                     eight feet"
                ),
            );
        }
        Cut::Round => {
            for (index, corner) in corners.iter().enumerate() {
                let [one, other, upright] = AT_THE_CORNERS[index];
                let [_, _, round] = corner.rounds(r);
                round_held(body, laid(upright), &round, &[patches[index]], what);
                piece_held(
                    body,
                    patches[index],
                    &corner.torus(r),
                    &[laid(one), laid(other), laid(upright)],
                    &format!("{what}, at {}", corner.called),
                );
            }
            for (foot, [from, to]) in FEET_BETWEEN.into_iter().enumerate() {
                let facing = foot_wall(foot);
                let [start_at, end_at] = [from, to].map(|corner| corners[corner].at);
                let along = (end_at - start_at).normalize();
                let [before, after] = [from, to].map(|corner| corners[corner].tangent(r));
                let round = Cylinder {
                    called: corners[from]
                        .feet
                        .iter()
                        .find(|at| at.wall == FEET_AND_UPRIGHTS[foot][1])
                        .expect("a foot runs from the corner it starts at")
                        .called,
                    through: start_at + facing * r + DVec3::Z * r,
                    along,
                    radius: r,
                    toward: [-DVec3::Z, -facing],
                    ends: [
                        (start_at + along * before, -along),
                        (end_at - along * after, along),
                    ],
                    meets: vec![TOP, FEET_AND_UPRIGHTS[foot][1]],
                };
                round_held(
                    body,
                    laid(foot),
                    &round,
                    &[patches[from], patches[to]],
                    what,
                );
            }
        }
    }
}

#[test]
fn a_chamfer_and_a_fillet_on_the_four_feet_and_four_uprights_of_a_standing_block_close_its_four_foot_corners_in_one_step()
 {
    let part = stacked();
    let corners = stacked_s_foot_corners();
    let names = FEET_AND_UPRIGHTS.map(|[first, second]| edge_between(&part, first, second));
    let in_two_steps = VALUES.map(|value| {
        let promised = chamfered_all_round(value);
        let two_steps = chamfered_all_round_in_two_steps(value);
        assert!(
            (two_steps.volume() - promised).abs() <= EXACT * promised,
            "the chamfer of {value} in two steps, the four uprights then the eight feet: the body \
             holds {}, where arithmetic promised {promised}",
            two_steps.volume()
        );
        two_steps
    });
    let mut laid = Vec::new();
    let mut declined = Vec::new();
    for cut in [Cut::Flat, Cut::Round] {
        for value in VALUES {
            for order in ALL_ROUND_ORDERS {
                let list: Vec<EdgeName> = order.iter().map(|&edge| names[edge].clone()).collect();
                let what = format!(
                    "the {} of {value} on the standing block's four feet and four uprights, \
                     named {order:?}",
                    cut.called()
                );
                match cut.laid_on(&part, &list, value) {
                    Ok(step) => laid.push(Case {
                        order: order.to_vec(),
                        cut,
                        value,
                        step,
                        what,
                    }),
                    Err(reason) => declined.push(format!("{what}: {reason:?}")),
                }
            }
        }
    }
    assert!(
        declined.is_empty(),
        "the standing block's four feet and four uprights laid in one step, each upright first and \
         the feet round it; declined:\n{}",
        declined.join("\n")
    );
    for case in &laid {
        let at = VALUES
            .iter()
            .position(|&value| value == case.value)
            .expect("every case is laid at one of the values");
        closed_all_round(&part, &corners, case, &in_two_steps[at]);
    }
    for case in &laid {
        let first = laid
            .iter()
            .find(|other| other.cut == case.cut && other.value == case.value)
            .expect("the case itself is among them");
        if std::ptr::eq(first, case) {
            continue;
        }
        let [mine, theirs] = [case, first].map(|one| one.patches(&part, &AT_THE_CORNERS));
        let to_first: Vec<(usize, usize)> = (0..8)
            .map(|edge| (case.laid_on(&part, edge), first.laid_on(&part, edge)))
            .chain(mine.into_iter().zip(theirs))
            .collect();
        the_same_bodies(
            &first.step.body,
            &case.step.body,
            &to_first,
            &format!("{} and {}", first.what, case.what),
        );
    }
}
