//! A chamfer and a fillet on a convex straight edge that ends where a concave
//! edge also ends: the top and floor edges at the inner corner of an L, the
//! rim of a pocket, the uprights of a block standing on another. Past such an
//! end lies matter, not air, and the cut's prism, carried past the end along
//! the edge's line as #699 carries it past a block's corner, would take it.
//! It is trimmed flush on the face the edge runs into instead, as #778 trims
//! a fill, and that face grows by the cut's end section and keeps its number.
//! Two such edges meeting at a corner whose third edge is concave, that edge
//! left sharp, run on past the corner into a mitre. An operation of the kernel
//! on [`Body`], as #699's corners are.
//!
//! Part of #801, its first slice: two convex edges with a concave one left
//! sharp. The fills beside a convex edge and the three concave edges of a
//! pocket's bottom corner are its second and third slices, each on a branch
//! of its own; the issue stays open until the last one lands.
//!
//! - A convex straight edge ending where a concave edge also ends leaves a
//!   closed, sound body when chamfered or filleted alone. Its flat or round
//!   stops flush on the face it runs into, which keeps its number. Nothing
//!   past that face is taken. The volume is exact. This holds on the L's top
//!   and floor edges at its inner corner —
//!   `a_chamfer_and_a_fillet_on_a_top_or_floor_edge_at_the_l_s_inner_corner_stop_flush_on_the_side_they_run_into`;
//!   on each rim edge of a pocket —
//!   `a_chamfer_and_a_fillet_on_each_rim_edge_of_a_pocket_stop_flush_on_the_walls_they_run_into_at_both_ends`;
//!   and on each upright of a block standing on another —
//!   `a_chamfer_and_a_fillet_on_each_upright_of_a_block_standing_on_another_stop_flush_on_the_top_it_stands_on`.
//!   The first holds the body where
//!   `a_chamfer_whose_cut_carried_along_its_edge_would_run_into_the_l_s_stem_is_declined_as_unsupported`,
//!   in `a_chamfer_cuts_one_straight_edge.rs`, held the decline of the L's
//!   top edge on the arm's back; that test is gone
//! - Two convex edges meeting at a corner whose third edge is concave and
//!   left sharp leave a closed, sound body when chamfered or filleted at one
//!   value. The two flats or rounds run past the corner into a mitre: a
//!   straight line among flats, a planar ellipse arc among rounds. The mitre
//!   runs from where their set-backs cross on the face they share down to the
//!   concave edge. That edge stays sharp and ends one value short of the old
//!   corner. No patch is added. The volume is exact, and the body is the same
//!   in either order. This holds on the L's two top edges at its inner
//!   corner —
//!   `a_chamfer_on_the_two_top_edges_at_the_l_s_inner_corner_mitres_their_flats_down_to_its_upright_left_sharp_one_distance_short`,
//!   `a_fillet_on_the_two_top_edges_at_the_l_s_inner_corner_mitres_their_rounds_on_an_ellipse_down_to_its_upright_left_sharp_one_radius_short`;
//!   on two rim edges of a pocket above one of its uprights —
//!   `a_chamfer_and_a_fillet_on_two_rim_edges_of_a_pocket_mitre_them_down_to_the_upright_below_left_sharp`;
//!   and the four rim edges of a pocket mitre its four corners —
//!   `a_chamfer_and_a_fillet_on_the_four_rim_edges_of_a_pocket_mitre_its_four_corners`
//! - The same holds at a corner that is not square: an L whose inner corner
//!   is not a right angle —
//!   `a_chamfer_and_a_fillet_at_an_l_s_inner_corner_that_is_not_square_stop_flush_alone_and_mitre_together`,
//!   read as both bullets above: each top edge there alone, stopping flush on
//!   the slanted side it runs into, and the two together, mitred
//! - What must not break: #699's corner tests; the concave fill of one edge
//!   (`a_chamfer_and_a_fillet_fill_a_concave_edge`); the decline test
//!   `a_fillet_and_a_chamfer_on_edges_meeting_at_a_corner_with_a_concave_edge_are_declined_as_unsupported`,
//!   which loses the lists this slice builds, the L's two top edges and the
//!   pocket's two rim edges, and keeps the others — no test: each is held by
//!   its own file, `a_chamfer_and_a_fillet_blend_the_corner_their_edges_meet_at.rs`
//!   and `a_chamfer_and_a_fillet_fill_a_concave_edge.rs`, left as they were
//!   but for the two lists the decline test gives up to the tests above
//!
//! The volumes are arithmetic. Every edge here lies between two faces square
//! to each other, and every face it ends on is square to it. A flat set back
//! `d` along both faces takes a right triangle of legs `d` off the edge's
//! section, `d²/2`; a round of radius `r`, the square of side `r` less the
//! quarter of the disc, `r²(1 − π/4)`. Where air lies past an end, the cut
//! carried past it takes nothing beyond the end's plane; where matter does,
//! the cut stopped flush takes nothing beyond the end face. Either way the
//! cut takes its section along the edge's length, and the part keeps its
//! volume less that: the L, an arm 30 by 6 and a stem 12 by 8 raised 10,
//! holds 2760; the block 30 by 20 by 10 holds 5500 with its pocket 10 by 10
//! sunk 5, and 7000 with the block 10 by 10 by 10 standing on it. Held
//! another way, by the booleans, every edge here at both values agrees to the
//! last digit: the part less the prism over the cut's section, raised along
//! the edge from the face it runs into to past its other end, or from one
//! face run into to the other.
//!
//! Two cuts mitred at a corner whose third edge is concave take each its
//! section along its edge, stopped flush, and besides what lies past both
//! faces they would stop on, which is matter. Measured from the corner, `c`
//! down from the face they share and `p` and `q` past the two end faces, the
//! flats take there what has `max(p, q) + c < d`, the two flats' planes
//! meeting where `p = q`: over the square of side `d` at the corner,
//! `∫∫ (d − max(p, q))`, that is `2∫₀ᵈ u(d − u) du = d³/3`. A round of `r`
//! takes, `u` past the face beside it, the depth `w(u) = r − √(r² − (r − u)²)`,
//! and the two rounds what lies within both depths, `w(max(p, q))`, their
//! cylinders meeting where `p = q` along an ellipse: `2∫₀ʳ u·w(u) du`, with
//! `u = r(1 − t)`, `2r³∫₀¹ (1 − t)(1 − √(1 − t²)) dt`, that is
//! `2r³(1/2 − π/4 + 1/3) = r³(5/3 − π/2)`. They are the overlaps a convex
//! corner gives back in #699, here taken once more. The part keeps its volume
//! less each cut's section along its edge's length and less that once at each
//! corner mitred: on the L, `2760 − 13d² − d³/3` with flats and
//! `2760 − 26r²(1 − π/4) − r³(5/3 − π/2)` with rounds; on the pocket, its rim
//! cut on two walls, `5500 − 10d² − d³/3`, and on all four,
//! `5500 − 20d² − 4d³/3`. Held another way, by the booleans, every mitre here
//! at both values agrees to the last digit: the part less each edge's prism,
//! carried past a mitred end by twice the value and clipped there by the
//! plane halfway between the two edges, on its own side.
//!
//! The L whose inner corner is not square leans its stem's right side from
//! (12, 6) to (18, 14), over its arm, or to (6, 14), away from it: the air
//! between that side and the arm's back is `φ` wide, 53.13° or 126.87°,
//! `sin φ = 0.8` either way, and the L holds 3000 or 2520. Its walls still
//! stand square to its top, so each cut still takes its section square to
//! its edge; what leans is the face each edge runs into. A line along the
//! edge `a` through the section, standing `o` off it, runs from the plane of
//! the face at one end to the plane at the other, `f₀` and `f₁` the ways
//! they face: `L − o·(f₁/(a·f₁) − f₀/(a·f₀))` long. The cut takes the section
//! times `L`, and its first moment about either of the edge's faces, `d³/6`
//! for a flat and `r³(5/6 − π/4)` for a round, times `(n₁ + n₂)·(f₁/(a·f₁) −
//! f₀/(a·f₀))`, `n₁` and `n₂` the ways the edge's faces face. On the arm's
//! back, square to the arm's end and running into the leaning side, that is
//! `cot φ`, ±0.75; on the leaning side, from the stem's back to the arm's
//! back, two parallel faces, nothing. Mitred, the two cuts take besides the
//! square corner's `d³/3` or `r³(5/3 − π/2)` stretched by `1/sin φ`: a unit of
//! `p` past one face by a unit of `q` past the other covers `1/sin φ` of the
//! top. The flats' planes and the rounds' cylinders still meet where
//! `p = q`, on the plane halfway between the two edges, so the mitre still
//! runs from where the two set-backs cross on the top, the value from both
//! walls, down to the concave upright the value below the top, both walls
//! standing square to it: the upright still ends one value short of the
//! corner. Held another way, by the booleans, both L's at both values agree
//! to the last digit, each edge alone and the two mitred: the part less each
//! edge's prism, carried well past both ends and clipped there by the plane
//! of the face it runs into, on the matter's side, or by the plane halfway
//! between the two edges, on the other's.

use std::collections::BTreeSet;
use std::f64::consts::PI;

use cao_solid::profile::{Contour, Frame, Profile};
use cao_solid::soundness::{closed, enclosed, uncrossed};
use cao_solid::{Body, Declined, EdgeName, Loop};
use glam::{DMat3, DVec2, DVec3};

/// How far a corner may stand from where the arithmetic puts it, on a plane:
/// each is placed exactly, so what is left is rounding.
const PLACE: f64 = 1e-9;

/// How near the arithmetic an exact volume comes, as a fraction of it.
const EXACT: f64 = 1e-9;

/// How far the triangles may stand from a round, as a share of the reach the
/// body carries: the drawing's tolerance.
const DRAWN: f64 = 2e-4;

/// The numbers a raised part's floor and top answer to.
const FLOOR: usize = 0;
const TOP: usize = 1;

/// The values every edge is cut at.
const VALUES: [f64; 2] = [1.0, 2.5];

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

/// An outline drawn on the ground raised `height` as `cao_part` builds a
/// step: its floor numbered nought, its top one, then its walls, one for each
/// run of the outline.
fn raised(outline: Contour, height: f64) -> Body {
    let part = Body::default();
    let tool = part
        .tool_raised(&profile(outline), ground(), DVec3::Z * height)
        .expect("the tool raises");
    part.union(&tool).expect("the part is built")
}

/// A block of a length, a breadth and a height standing on the ground's
/// origin, its sides numbered two to five from the front round to the left.
fn block([length, breadth, height]: [f64; 3]) -> Body {
    raised(
        Contour::rectangle(DVec2::ZERO, DVec2::new(length, breadth)),
        height,
    )
}

/// The block a step raises between the corners `from` and `to`, from
/// `from_height` by `height`: its faces numbered past the part's, its floor
/// first, then its top, then its sides from the front round to the left.
fn tool_between(part: &Body, [from, to]: [DVec2; 2], from_height: f64, height: f64) -> Body {
    part.tool_raised(
        &profile(Contour::rectangle(from, to)),
        Frame {
            origin: DVec3::Z * from_height,
            ..ground()
        },
        DVec3::Z * height,
    )
    .expect("the tool raises")
}

/// The outline of an L: an arm 30 long and 6 deep along the front, and at its
/// left end a stem 12 wide reaching back to 14. Raised 10, its walls are
/// numbered from the front round: the front two, the arm's right end three,
/// the arm's back four, the stem's right side five, the stem's back six and
/// the left seven. The arm's back and the stem's right side meet inwards, in
/// the L's inner corner at (12, 6).
fn ell() -> Body {
    raised(
        Contour::straight(vec![
            DVec2::new(0.0, 0.0),
            DVec2::new(30.0, 0.0),
            DVec2::new(30.0, 6.0),
            DVec2::new(12.0, 6.0),
            DVec2::new(12.0, 14.0),
            DVec2::new(0.0, 14.0),
        ]),
        10.0,
    )
}
const ELL_VOLUME: f64 = (30.0 * 6.0 + 12.0 * 8.0) * 10.0;
const ARM_END: usize = 3;
const ARM_BACK: usize = 4;
const STEM_RIGHT: usize = 5;
const STEM_BACK: usize = 6;

/// Where the pocket is sunk and the standing block stands: 10 by 10 in the
/// middle of the block 30 by 20.
const FOOTPRINT: [DVec2; 2] = [DVec2::new(10.0, 5.0), DVec2::new(20.0, 15.0)];

/// A block 30 by 20 by 10 with a pocket 10 by 10 sunk 5 into the middle of
/// its top, as `cao_part` cuts a step: the pocket's floor numbered six, its
/// walls, front, right, back and left, eight to eleven, and the tool's top,
/// standing above the block, gone. The rim, where the walls meet the block's
/// top, meets outwards; the uprights, where two walls meet, inwards.
fn pocketed() -> Body {
    let block = block([30.0, 20.0, 10.0]);
    block
        .difference(&tool_between(&block, FOOTPRINT, 5.0, 6.0))
        .expect("the pocket is cut")
}
const POCKETED_VOLUME: f64 = 30.0 * 20.0 * 10.0 - 10.0 * 10.0 * 5.0;
const POCKET_FRONT: usize = 8;
const POCKET_RIGHT: usize = 9;
const POCKET_BACK: usize = 10;
const POCKET_LEFT: usize = 11;

/// A block 30 by 20 by 10 with another, 10 by 10 by 10, standing on the
/// middle of its top, as `cao_part` joins a step: the upper block's top
/// numbered seven, its walls, front, right, back and left, eight to eleven,
/// its floor gone into the lower block's top. Its uprights meet outwards; its
/// feet, where its walls meet the lower block's top, inwards.
fn stacked() -> Body {
    let block = block([30.0, 20.0, 10.0]);
    block
        .union(&tool_between(&block, FOOTPRINT, 10.0, 10.0))
        .expect("the standing block is joined")
}
const STACKED_VOLUME: f64 = 30.0 * 20.0 * 10.0 + 10.0 * 10.0 * 10.0;
const STANDING_TOP: usize = 7;
const STANDING_FRONT: usize = 8;
const STANDING_RIGHT: usize = 9;
const STANDING_BACK: usize = 10;
const STANDING_LEFT: usize = 11;

/// A convex edge between two faces square to each other: the faces it lies
/// between, the way each faces, out of the matter, and its two ends. Where
/// two edges are mitred, the first face of each is the one they share, and
/// each runs into the other's second face.
struct Convex {
    called: &'static str,
    between: [usize; 2],
    facing: [DVec3; 2],
    ends: [End; 2],
}

/// An end of an edge: where it is, the face the edge runs into there — the
/// one face at that corner that is neither of the edge's two, flat — the way
/// that face faces, out of the matter, and whether matter lies past it along
/// the edge's line, or air.
struct End {
    at: DVec3,
    on: usize,
    facing: DVec3,
    matter_past: bool,
}

/// The point that stands `offsets` from the planes through `through` that
/// face the three ways `facing`, each offset signed the way its plane faces.
fn standing(through: DVec3, facing: [DVec3; 3], offsets: [f64; 3]) -> DVec3 {
    let [one, two, three] = facing;
    through + DMat3::from_cols(one, two, three).transpose().inverse() * DVec3::from(offsets)
}

impl Convex {
    fn length(&self) -> f64 {
        self.ends[0].at.distance(self.ends[1].at)
    }

    fn along(&self) -> DVec3 {
        (self.ends[1].at - self.ends[0].at).normalize()
    }

    /// Where a cut of `value` crosses the face an end runs into: on the
    /// edge's first face, set back by the value from its second, then on its
    /// second, set back by the value from its first.
    fn set_back(&self, end: &End, value: f64) -> [DVec3; 2] {
        let [first, second] = self.facing;
        [
            standing(end.at, [second, end.facing, first], [-value, 0.0, 0.0]),
            standing(end.at, [first, end.facing, second], [-value, 0.0, 0.0]),
        ]
    }

    /// Whether a point lies on the curve a round of `radius` ends in on the
    /// face an end runs into: on that face's plane, a radius from the round's
    /// axis, which stands the radius in from both faces, and on the edge's
    /// side of the axis, between the two set-backs.
    fn on_the_round_s_end(&self, end: &End, radius: f64, point: DVec3) -> bool {
        let [first, second] = self.facing;
        let off = point - (end.at - (first + second) * radius);
        let across = off - self.along() * off.dot(self.along());
        (point - end.at).dot(end.facing).abs() < PLACE
            && (across.length() - radius).abs() < PLACE
            && across.dot(first) >= -PLACE
            && across.dot(second) >= -PLACE
    }

    /// What a cut of `value` takes off the edge alone, each line along the
    /// edge through its section running from the plane of the face at one end
    /// to the plane of the face at the other: the section times the edge's
    /// length, and, where a face at an end leans across the edge, the
    /// section's first moment about the edge times how far the lines grow a
    /// unit in from it, as the module's documentation derives it.
    fn taken(&self, cut: Cut, value: f64) -> f64 {
        let along = self.along();
        let [start, end] = &self.ends;
        let lean = end.facing / along.dot(end.facing) - start.facing / along.dot(start.facing);
        let [first, second] = self.facing;
        cut.section(value) * self.length() + cut.moment(value) * (first + second).dot(lean)
    }
}

/// The L's four edges at its inner corner that meet outwards: the top's and
/// the floor's on the arm's back, running from the arm's right end, where air
/// lies past, to the stem's right side, where the stem does; and the top's
/// and the floor's on the stem's right side, from the stem's back to the
/// arm's back, where the arm does.
fn ell_edges() -> [Convex; 4] {
    [
        Convex {
            called: "the L's top edge on the arm's back",
            between: [TOP, ARM_BACK],
            facing: [DVec3::Z, DVec3::Y],
            ends: [
                End {
                    at: DVec3::new(30.0, 6.0, 10.0),
                    on: ARM_END,
                    facing: DVec3::X,
                    matter_past: false,
                },
                End {
                    at: DVec3::new(12.0, 6.0, 10.0),
                    on: STEM_RIGHT,
                    facing: DVec3::X,
                    matter_past: true,
                },
            ],
        },
        Convex {
            called: "the L's floor edge on the arm's back",
            between: [FLOOR, ARM_BACK],
            facing: [-DVec3::Z, DVec3::Y],
            ends: [
                End {
                    at: DVec3::new(30.0, 6.0, 0.0),
                    on: ARM_END,
                    facing: DVec3::X,
                    matter_past: false,
                },
                End {
                    at: DVec3::new(12.0, 6.0, 0.0),
                    on: STEM_RIGHT,
                    facing: DVec3::X,
                    matter_past: true,
                },
            ],
        },
        Convex {
            called: "the L's top edge on the stem's right side",
            between: [TOP, STEM_RIGHT],
            facing: [DVec3::Z, DVec3::X],
            ends: [
                End {
                    at: DVec3::new(12.0, 14.0, 10.0),
                    on: STEM_BACK,
                    facing: DVec3::Y,
                    matter_past: false,
                },
                End {
                    at: DVec3::new(12.0, 6.0, 10.0),
                    on: ARM_BACK,
                    facing: DVec3::Y,
                    matter_past: true,
                },
            ],
        },
        Convex {
            called: "the L's floor edge on the stem's right side",
            between: [FLOOR, STEM_RIGHT],
            facing: [-DVec3::Z, DVec3::X],
            ends: [
                End {
                    at: DVec3::new(12.0, 14.0, 0.0),
                    on: STEM_BACK,
                    facing: DVec3::Y,
                    matter_past: false,
                },
                End {
                    at: DVec3::new(12.0, 6.0, 0.0),
                    on: ARM_BACK,
                    facing: DVec3::Y,
                    matter_past: true,
                },
            ],
        },
    ]
}

/// The pocket's four rim edges, each running between the two walls beside
/// its own, with the block's matter past both.
fn rim_edges() -> [Convex; 4] {
    let [front_left, front_right, back_right, back_left] = [
        DVec3::new(10.0, 5.0, 10.0),
        DVec3::new(20.0, 5.0, 10.0),
        DVec3::new(20.0, 15.0, 10.0),
        DVec3::new(10.0, 15.0, 10.0),
    ];
    let flush = |at: DVec3, on: usize, facing: DVec3| End {
        at,
        on,
        facing,
        matter_past: true,
    };
    [
        Convex {
            called: "the pocket's rim on its front wall",
            between: [TOP, POCKET_FRONT],
            facing: [DVec3::Z, DVec3::Y],
            ends: [
                flush(front_left, POCKET_LEFT, DVec3::X),
                flush(front_right, POCKET_RIGHT, -DVec3::X),
            ],
        },
        Convex {
            called: "the pocket's rim on its right wall",
            between: [TOP, POCKET_RIGHT],
            facing: [DVec3::Z, -DVec3::X],
            ends: [
                flush(front_right, POCKET_FRONT, DVec3::Y),
                flush(back_right, POCKET_BACK, -DVec3::Y),
            ],
        },
        Convex {
            called: "the pocket's rim on its back wall",
            between: [TOP, POCKET_BACK],
            facing: [DVec3::Z, -DVec3::Y],
            ends: [
                flush(back_right, POCKET_RIGHT, -DVec3::X),
                flush(back_left, POCKET_LEFT, DVec3::X),
            ],
        },
        Convex {
            called: "the pocket's rim on its left wall",
            between: [TOP, POCKET_LEFT],
            facing: [DVec3::Z, DVec3::X],
            ends: [
                flush(back_left, POCKET_BACK, -DVec3::Y),
                flush(front_left, POCKET_FRONT, DVec3::Y),
            ],
        },
    ]
}

/// The standing block's four uprights, each running from its top, where air
/// lies past, down to the lower block's top, where that block does.
fn uprights() -> [Convex; 4] {
    let upright =
        |called: &'static str, between: [usize; 2], facing: [DVec3; 2], [x, y]: [f64; 2]| Convex {
            called,
            between,
            facing,
            ends: [
                End {
                    at: DVec3::new(x, y, 20.0),
                    on: STANDING_TOP,
                    facing: DVec3::Z,
                    matter_past: false,
                },
                End {
                    at: DVec3::new(x, y, 10.0),
                    on: TOP,
                    facing: DVec3::Z,
                    matter_past: true,
                },
            ],
        };
    [
        upright(
            "the standing block's front right upright",
            [STANDING_FRONT, STANDING_RIGHT],
            [-DVec3::Y, DVec3::X],
            [20.0, 5.0],
        ),
        upright(
            "the standing block's back right upright",
            [STANDING_RIGHT, STANDING_BACK],
            [DVec3::X, DVec3::Y],
            [20.0, 15.0],
        ),
        upright(
            "the standing block's back left upright",
            [STANDING_BACK, STANDING_LEFT],
            [DVec3::Y, -DVec3::X],
            [10.0, 15.0],
        ),
        upright(
            "the standing block's front left upright",
            [STANDING_LEFT, STANDING_FRONT],
            [-DVec3::X, -DVec3::Y],
            [10.0, 5.0],
        ),
    ]
}

/// The two cuts a step lays on an edge.
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

    /// The step on the edge a name finds, alone, at `value`.
    fn laid(self, part: &Body, edge: &EdgeName, value: f64) -> Result<Laid, Declined> {
        match self {
            Cut::Flat => part.chamfer(edge, value).map(|step| Laid {
                body: step.body,
                asked: step.asked,
                taken: step.taken,
            }),
            Cut::Round => part.fillet(edge, value).map(|step| Laid {
                body: step.body,
                asked: step.asked,
                taken: step.taken,
            }),
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

    /// The section the cut takes off an edge whose faces meet square.
    fn section(self, value: f64) -> f64 {
        match self {
            Cut::Flat => value * value / 2.0,
            Cut::Round => value * value * (1.0 - PI / 4.0),
        }
    }

    /// The first moment of the section the cut takes off an edge whose faces
    /// meet square, about either of those faces.
    fn moment(self, value: f64) -> f64 {
        match self {
            Cut::Flat => value.powi(3) / 6.0,
            Cut::Round => value.powi(3) * (5.0 / 6.0 - PI / 4.0),
        }
    }

    /// What two cuts mitred at a square corner whose third edge is concave
    /// take past the faces they would each stop flush on, as the module's
    /// documentation derives it.
    fn past_the_corner(self, value: f64) -> f64 {
        match self {
            Cut::Flat => value.powi(3) / 3.0,
            Cut::Round => value.powi(3) * (5.0 / 3.0 - PI / 2.0),
        }
    }
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

/// The names of every edge between the faces numbered `first` and `second`.
fn edges_between(body: &Body, first: usize, second: usize) -> Vec<EdgeName> {
    let sides = [vec![first as u32], vec![second as u32]];
    let [one, other] = sides.clone();
    body.edge_names()
        .into_iter()
        .filter(|name| name.between == sides || name.between == [other.clone(), one.clone()])
        .collect()
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

/// Holds that each of `edges`, chamfered and filleted alone at every one of
/// [`VALUES`], leaves what [`stopped_flush`] holds, the part holding `volume`
/// before the step. Every edge is laid before any body is read, so that a
/// decline names every case that declines.
fn each_stops_flush(part: &Body, volume: f64, edges: &[Convex], called: &str) {
    assert!(
        (part.volume() - volume).abs() <= EXACT * volume,
        "{called}: the part holds {}, where arithmetic gives {volume}",
        part.volume()
    );
    let mut laid = Vec::new();
    let mut declined = Vec::new();
    for edge in edges {
        let name = edge_between(part, edge.between[0], edge.between[1]);
        for cut in [Cut::Flat, Cut::Round] {
            for value in VALUES {
                let what = format!("the {} of {value} on {}", cut.called(), edge.called);
                match cut.laid(part, &name, value) {
                    Ok(step) => laid.push((edge, cut, value, step, what)),
                    Err(reason) => declined.push(format!("{what}: {reason:?}")),
                }
            }
        }
    }
    assert!(
        declined.is_empty(),
        "{called}: each edge cut alone, its cut stopping flush where matter lies past its end; \
         declined:\n{}",
        declined.join("\n")
    );
    for (edge, cut, value, step, what) in &laid {
        stopped_flush(part, volume, edge, *cut, *value, step, what);
    }
}

/// Holds what a cut of `value` on a convex edge, alone, leaves of a part
/// holding `volume`: the value taken whole; a sound body holding the part's
/// volume less what the cut takes between the faces at the edge's ends, and so
/// nothing past the faces the edge runs into taken; one face laid, numbered where the
/// steps' numbers ended, flat or curved as the cut is, a flat running from
/// the face at one end of the edge to the face at the other, and either cut
/// meeting the edge's two faces and the face at each of its ends by one edge
/// each. At each end where matter lies past, the face the edge runs into
/// answers to its number still: its outline the part's with the corner the
/// edge ended at given up for the cut's end section, and meeting each of the
/// edge's two faces along one edge still, for a name to find.
fn stopped_flush(
    part: &Body,
    volume: f64,
    edge: &Convex,
    cut: Cut,
    value: f64,
    step: &Laid,
    what: &str,
) {
    taken_as_asked(step.asked, step.taken, value, what);
    let body = &step.body;
    let promised = volume - edge.taken(cut, value);
    let drawn = match cut {
        Cut::Flat => EXACT * promised,
        Cut::Round => rounds_drawn_within(part, std::slice::from_ref(edge), value),
    };
    sound_drawn_within(body, promised, drawn, what);
    let made = part.faces_end();
    assert_eq!(
        faces_made(part, body),
        vec![made],
        "{what}: one face laid, numbered where the steps' numbers ended, and the faces the edge \
         runs into answering to theirs still: {:?}",
        numbers(body)
    );
    assert!(
        numbers(part).is_subset(&numbers(body)),
        "{what}: a face of the part went: {:?}",
        numbers(body)
    );
    let [start, end] = &edge.ends;
    match cut {
        Cut::Flat => {
            assert!(body.is_flat(made), "{what}: the chamfer lays a flat face");
            corners_are_or_on(
                body,
                made,
                &[edge.set_back(start, value), edge.set_back(end, value)].concat(),
                |_| false,
                &format!("{what}: the flat, from the face at one end of the edge to the other's"),
            );
        }
        Cut::Round => assert!(
            !body.is_flat(made) && body.plane_of(made).is_none(),
            "{what}: the fillet lays a curved face, which offers no plane to draw on"
        ),
    }
    let [one, other] = edge.between;
    assert_eq!(
        sides_met(body, made),
        sides(&[one, other, start.on, end.on]),
        "{what}: the cut meets the edge's two faces along its sides, and the face at each end of \
         the edge across its ends, by one edge each"
    );
    for end in edge.ends.iter().filter(|end| end.matter_past) {
        let face = end.on;
        let kept = part
            .plane_of(face)
            .unwrap_or_else(|| panic!("{what}: the part offers no plane on face {face}"))
            .corners
            .into_iter()
            .filter(|corner| corner.distance(end.at) >= PLACE);
        let expected: Vec<DVec3> = kept.chain(edge.set_back(end, value)).collect();
        corners_are_or_on(
            body,
            face,
            &expected,
            |corner| matches!(cut, Cut::Round) && edge.on_the_round_s_end(end, value, corner),
            &format!(
                "{what}: face {face}, which the edge runs into, grown by the cut's end section in \
                 place of the corner the edge ended at"
            ),
        );
        for side in edge.between {
            let meeting = edges_between(body, face, side);
            assert_eq!(
                meeting.len(),
                1,
                "{what}: face {face}, which the edge runs into, meets face {side} along one edge \
                 still: {meeting:?}"
            );
        }
    }
}

#[test]
fn a_chamfer_and_a_fillet_on_a_top_or_floor_edge_at_the_l_s_inner_corner_stop_flush_on_the_side_they_run_into()
 {
    each_stops_flush(
        &ell(),
        ELL_VOLUME,
        &ell_edges(),
        "the L's top and floor edges at its inner corner",
    );
}

#[test]
fn a_chamfer_and_a_fillet_on_each_rim_edge_of_a_pocket_stop_flush_on_the_walls_they_run_into_at_both_ends()
 {
    each_stops_flush(
        &pocketed(),
        POCKETED_VOLUME,
        &rim_edges(),
        "the pocket's rim edges",
    );
}

#[test]
fn a_chamfer_and_a_fillet_on_each_upright_of_a_block_standing_on_another_stop_flush_on_the_top_it_stands_on()
 {
    each_stops_flush(
        &stacked(),
        STACKED_VOLUME,
        &uprights(),
        "the standing block's uprights",
    );
}

/// The L's two top edges at its inner corner, the arm's back's and the stem's
/// right side's, meeting at (12, 6, 10) above the L's inner upright.
fn ell_top_edges() -> [Convex; 2] {
    let [arm_back, _, stem_right, _] = ell_edges();
    [arm_back, stem_right]
}

/// The pocket's rim on its front wall and on its right wall, meeting at
/// (20, 5, 10) above the pocket's front right upright.
fn front_and_right_rim_edges() -> [Convex; 2] {
    let [front, right, _, _] = rim_edges();
    [front, right]
}

/// The two orders a list can name two edges in.
const BOTH_ORDERS: [&[usize]; 2] = [&[0, 1], &[1, 0]];

/// Two of the orders a list can name the pocket's four rim edges in: round
/// the rim, and the front and the back first.
const ROUND_AND_ACROSS: [&[usize]; 2] = [&[0, 1, 2, 3], &[0, 2, 1, 3]];

/// The edge of `edges` other than `edge` that ends where the end `end` of
/// `edge` is, the two meeting there in a mitre, and which of its ends that
/// is; none where no other edge of the list ends there.
fn mitred_with(edges: &[Convex], edge: usize, end: usize) -> Option<(usize, usize)> {
    let at = edges[edge].ends[end].at;
    (0..edges.len())
        .filter(|&other| other != edge)
        .find_map(|other| {
            edges[other]
                .ends
                .iter()
                .position(|far| far.at.distance(at) < PLACE)
                .map(|far| (other, far))
        })
}

/// The way an edge runs from its end `end` to its other end.
fn from_end(edge: &Convex, end: usize) -> DVec3 {
    (edge.ends[1 - end].at - edge.ends[end].at).normalize()
}

/// The round a fillet is expected to lay: a cylinder of `radius` about the
/// line from `start` along `along` for `length`, turning about it from facing
/// the way the edge's first face does to facing the way its second does.
struct Round {
    start: DVec3,
    along: DVec3,
    length: f64,
    radius: f64,
    facing: [DVec3; 2],
}

impl Round {
    /// The round of `radius` on `edge`: its axis the radius in from the
    /// edge's two faces, from the edge's first end to its last.
    fn on(edge: &Convex, radius: f64) -> Round {
        let [first, second] = edge.facing;
        Round {
            start: edge.ends[0].at - (first + second) * radius,
            along: edge.along(),
            length: edge.length(),
            radius,
            facing: edge.facing,
        }
    }

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

    /// Whether a point lies on the round's cylinder, between its two sides;
    /// how far along it the round runs is for the faces at its ends to say.
    fn holds(&self, point: DVec3) -> bool {
        let off = point - self.start;
        let across = off - self.along * off.dot(self.along);
        let [first, second] = self.facing;
        let least = self.radius * first.dot(second);
        (across.length() - self.radius).abs() < PLACE
            && across.dot(first) >= least - PLACE
            && across.dot(second) >= least - PLACE
    }

    /// The part of the round from `from` along its axis to `to`.
    fn part(&self, from: f64, to: f64) -> Round {
        Round {
            start: self.start + self.along * from,
            length: to - from,
            ..*self
        }
    }
}

/// How far a round of `radius` on the edge `edge` of `edges` can stand past
/// each end of the edge's own length, or short of it, along the edge: past
/// a mitre, as far as the plane halfway between the two edges reaches across
/// the round, the radius over the tangent of half their angle; where the face
/// at an end leans across the edge, as far as that face's plane reaches
/// across the radius; nothing where that face is square to the edge.
fn overhang(edges: &[Convex], edge: usize, radius: f64) -> [f64; 2] {
    let convex = &edges[edge];
    [0, 1].map(|end| match mitred_with(edges, edge, end) {
        Some((other, far)) => {
            let cos = from_end(convex, end).dot(from_end(&edges[other], far));
            radius * (1.0 + cos) / (1.0 - cos * cos).sqrt()
        }
        None => leaning_by(convex, end, radius),
    })
}

/// How far along the edge the plane of the face at its end `end` stands off
/// square across `radius` of it: nothing where that face is square to the
/// edge.
fn leaning_by(edge: &Convex, end: usize, radius: f64) -> f64 {
    let cos = edge.along().dot(edge.ends[end].facing).abs();
    radius * (1.0 - cos * cos).sqrt() / cos
}

/// What the triangles of a fillet on `edges` may enclose apart from its
/// matter: the drawing's tolerance at the part's own reach, over the rounds,
/// each counted along its edge and as far past its ends as it can stand.
fn rounds_drawn_within(part: &Body, edges: &[Convex], radius: f64) -> f64 {
    let area: f64 = (0..edges.len())
        .map(|edge| {
            let [before, after] = overhang(edges, edge, radius);
            PI / 2.0 * radius * (edges[edge].length() + before + after)
        })
        .sum();
    DRAWN * reach_of(part) * area
}

/// The two corners a cut of `value` on the edge `edge` of `edges` has at its
/// end `end`, on the face the edges share and on the edge's other face. Where
/// it is mitred, the first is where the two cuts' set-backs cross on the face
/// they share, and the second where they meet the concave edge between the
/// two other faces, the value below the face they share; elsewhere, where the
/// cut crosses the face its edge runs into.
fn cut_corners(edges: &[Convex], edge: usize, end: usize, value: f64) -> [DVec3; 2] {
    let convex = &edges[edge];
    let [shared, own] = convex.facing;
    match mitred_with(edges, edge, end) {
        Some((other, _)) => {
            let facing = [own, edges[other].facing[1], shared];
            let at = convex.ends[end].at;
            [
                standing(at, facing, [-value, -value, 0.0]),
                standing(at, facing, [0.0, 0.0, -value]),
            ]
        }
        None => convex.set_back(&convex.ends[end], value),
    }
}

/// Holds that every ray aimed square at the round's axis from a unit outside
/// it — nearer than any other face of these parts stands across the air from
/// it — a tenth to nine tenths of the way along it, meets the face numbered
/// `face` first, where it faces away from the axis along the ray.
fn met_from_outside(body: &Body, face: usize, round: &Round, what: &str) {
    for along in [0.11, 0.47, 0.83] {
        for share in [0.1, 0.3, 0.5, 0.7, 0.9] {
            let facing = round.facing_at(share);
            let from = round.axis_at(along) + facing * (round.radius + 1.0);
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

/// Holds that a flat face faces the way `toward` points.
fn faces_toward(body: &Body, face: usize, toward: DVec3, what: &str) {
    let facing = body
        .plane_of(face)
        .unwrap_or_else(|| panic!("{what}: the body offers no plane on face {face}"))
        .normal;
    assert!(
        facing.distance(toward.normalize()) < PLACE,
        "{what}: face {face} faces {facing}, where it faces {}",
        toward.normalize()
    );
}

/// The sides a face met on the part, each face numbered `from` in `renamed`
/// met now as the face numbered `to`, in the order [`sides_met`] lists them.
fn renamed(met: Vec<Vec<u32>>, renamed: &[(usize, usize)]) -> Vec<Vec<u32>> {
    let mut sides: Vec<Vec<u32>> = met
        .into_iter()
        .map(|side| {
            renamed
                .iter()
                .find(|(from, _)| side == [*from as u32])
                .map_or(side, |&(_, to)| vec![to as u32])
        })
        .collect();
    sides.sort();
    sides
}

/// Holds that `edges`, named in each of `orders` and cut by each of `cuts` at
/// every one of [`VALUES`] in one step, leave what [`mitred`] holds, the part
/// holding `volume` before the step, and the same body whatever the order,
/// as [`the_same_body`] holds it. Every list is laid before any body is read,
/// so that a decline names every case that declines.
fn mitre_in_every_order(
    part: &Body,
    volume: f64,
    edges: &[Convex],
    orders: &[&[usize]],
    cuts: &[Cut],
    called: &str,
) {
    assert!(
        (part.volume() - volume).abs() <= EXACT * volume,
        "{called}: the part holds {}, where arithmetic gives {volume}",
        part.volume()
    );
    let names: Vec<EdgeName> = edges
        .iter()
        .map(|edge| edge_between(part, edge.between[0], edge.between[1]))
        .collect();
    let mut laid = Vec::new();
    let mut declined = Vec::new();
    for &order in orders {
        let list: Vec<EdgeName> = order.iter().map(|&edge| names[edge].clone()).collect();
        let named: Vec<&str> = order.iter().map(|&edge| edges[edge].called).collect();
        for &cut in cuts {
            for value in VALUES {
                let what = format!(
                    "the {} of {value} on {called}, named {named:?}",
                    cut.called()
                );
                match cut.laid_on(part, &list, value) {
                    Ok(step) => laid.push(Case {
                        order,
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
        "{called}: the edges cut in one step, mitred where two of them meet; declined:\n{}",
        declined.join("\n")
    );
    for case in &laid {
        mitred(part, volume, edges, case);
    }
    for case in &laid {
        let first = laid
            .iter()
            .find(|other| other.cut == case.cut && other.value == case.value)
            .expect("the case itself is among them");
        if !std::ptr::eq(first, case) {
            the_same_body(part, edges, first, case);
        }
    }
}

/// A step laid on a list of edges: the order the list names them in, the cut,
/// the value, what the step gave, and what to call it.
struct Case<'a> {
    order: &'a [usize],
    cut: Cut,
    value: f64,
    step: Laid,
    what: String,
}

impl Case<'_> {
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

/// Holds that two steps naming the same edges in different orders left the
/// same body, the face laid on each edge answering to the number its rank
/// gives it in each: the same matter; the same faces, each flat or curved
/// alike and a flat one with the same corners; and the same edges between
/// the same faces.
fn the_same_body(part: &Body, edges: &[Convex], first: &Case, second: &Case) {
    let what = format!("{} and {}", first.what, second.what);
    let [one, other] = [&first.step.body, &second.step.body];
    let to_first: Vec<(usize, usize)> = (0..edges.len())
        .map(|edge| (second.laid_on(part, edge), first.laid_on(part, edge)))
        .collect();
    let as_first = |number: usize| {
        to_first
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
    let renumbered: BTreeSet<usize> = numbers(other).into_iter().map(as_first).collect();
    assert_eq!(numbers(one), renumbered, "{what}: the same faces");
    for number in numbers(other) {
        let face = as_first(number);
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
        between(other, &as_first),
        "{what}: the same edges between the same faces"
    );
}

/// Holds what a step's cut of its value on `edges`, named in its order,
/// leaves of a part holding `volume`, the edges' first faces one face they
/// share, and two of the edges meeting at a corner whose third edge is
/// concave wherever two of them end at one point:
///
/// - the value taken whole; a sound body holding the part's volume less what
///   each cut takes alone, stopped on the face its edge runs into, and less,
///   at each corner, what the two cuts take there past both faces they would
///   each stop on;
/// - one face laid for each edge, numbered in the order the list names them,
///   and none at a corner;
/// - each face laid meeting its edge's two faces along its sides, and at each
///   end the face laid on the edge it is mitred with there, or the face its
///   edge runs into, one edge each;
/// - a flat running from end to end, its mitre from where the two flats cross
///   on the face they share down to the concave edge, the value below that
///   face; a round on its cylinder from end to end, past a mitred end as far
///   as the plane halfway between the two edges and no farther, where the
///   other round meets it along an ellipse, and nowhere past the face its
///   edge runs into at another end;
/// - the face the edges share set back by the value from each, its corner at
///   each mitre where the two set-backs cross, and meeting the faces laid in
///   place of the edges' other faces;
/// - each edge's other face cut down by the value along it, and the concave
///   edge at each corner still one sharp edge, ending where the two cuts meet
///   it.
fn mitred(part: &Body, volume: f64, edges: &[Convex], case: &Case) {
    let Case {
        cut,
        value,
        step,
        what,
        ..
    } = case;
    let (cut, value) = (*cut, *value);
    taken_as_asked(step.asked, step.taken, value, what);
    let body = &step.body;
    let laid: Vec<usize> = (0..edges.len())
        .map(|edge| case.laid_on(part, edge))
        .collect();
    let mitres: Vec<(usize, usize, usize, usize)> = (0..edges.len())
        .flat_map(|edge| {
            (0..2).filter_map(move |end| {
                mitred_with(edges, edge, end).map(|(other, far)| (edge, end, other, far))
            })
        })
        .filter(|&(edge, _, other, _)| edge < other)
        .collect();
    let alone: f64 = edges.iter().map(|edge| edge.taken(cut, value)).sum();
    let at_the_corners: f64 = mitres
        .iter()
        .map(|&(edge, _, other, _)| {
            let sine = edges[edge].facing[1].cross(edges[other].facing[1]).length();
            cut.past_the_corner(value) / sine
        })
        .sum();
    let promised = volume - alone - at_the_corners;
    let drawn = match cut {
        Cut::Flat => EXACT * promised,
        Cut::Round => rounds_drawn_within(part, edges, value),
    };
    sound_drawn_within(body, promised, drawn, what);
    let mut made = laid.clone();
    made.sort();
    assert_eq!(
        faces_made(part, body),
        made,
        "{what}: one face for each edge, numbered in the order the step names them, and no \
         face laid at a corner: {:?}",
        numbers(body)
    );
    assert!(
        numbers(part).is_subset(&numbers(body)),
        "{what}: a face of the part went: {:?}",
        numbers(body)
    );
    let shared = edges[0].between[0];
    let shared_facing = edges[0].facing[0];
    let corners_of = |face: usize| {
        part.plane_of(face)
            .unwrap_or_else(|| panic!("{what}: the part offers no plane on face {face}"))
            .corners
    };
    let end_at = |edge: usize, corner: DVec3| {
        (0..2).find(|&end| edges[edge].ends[end].at.distance(corner) < PLACE)
    };
    for (index, edge) in edges.iter().enumerate() {
        let face = laid[index];
        let [_, own] = edge.between;
        let [_, own_facing] = edge.facing;
        let partners = [0, 1].map(|end| mitred_with(edges, index, end));
        let [met_first, met_last] =
            [0, 1].map(|end| partners[end].map_or(edge.ends[end].on, |(other, _)| laid[other]));
        assert_eq!(
            sides_met(body, face),
            sides(&[shared, own, met_first, met_last]),
            "{what}: the face laid on {} meets its edge's two faces along its sides, and at each \
             end the face laid on the edge it is mitred with, or the face it runs into, by one \
             edge each",
            edge.called
        );
        match cut {
            Cut::Flat => {
                let corners: Vec<DVec3> = [0, 1]
                    .into_iter()
                    .flat_map(|end| cut_corners(edges, index, end, value))
                    .collect();
                corners_are_or_on(
                    body,
                    face,
                    &corners,
                    |_| false,
                    &format!(
                        "{what}: the flat on {}, from end to end, its mitre at a corner running \
                         from where the two flats cross on the face they share down to the \
                         concave edge, the value below that face",
                        edge.called
                    ),
                );
                faces_toward(
                    body,
                    face,
                    shared_facing + own_facing,
                    &format!("{what}: the flat on {}", edge.called),
                );
            }
            Cut::Round => {
                let round = Round::on(edge, value);
                let which = format!("{what}: the round on {}", edge.called);
                assert!(
                    !body.is_flat(face) && body.plane_of(face).is_none(),
                    "{which} is curved, and offers no plane to draw on"
                );
                let drawn: Vec<DVec3> = body.triangles_of(face).flatten().collect();
                assert!(!drawn.is_empty(), "{which} is drawn");
                for point in drawn {
                    assert!(
                        round.holds(point),
                        "{which} is drawn through {point}, off its cylinder of {value} about the \
                         line from {} along {}",
                        round.start,
                        round.along
                    );
                    for (end, partner) in partners.iter().enumerate() {
                        let at = &edge.ends[end];
                        let inward = from_end(edge, end);
                        match *partner {
                            Some((other, far)) => assert!(
                                (point - at.at).dot(inward)
                                    >= (point - at.at).dot(from_end(&edges[other], far)) - PLACE,
                                "{which} is drawn through {point}, past the plane halfway between \
                                 it and {}, where the other round meets it in the mitre",
                                edges[other].called
                            ),
                            None => assert!(
                                (point - at.at).dot(at.facing) * inward.dot(at.facing).signum()
                                    >= -PLACE,
                                "{which} is drawn through {point}, past face {}, which its edge \
                                 runs into there",
                                at.on
                            ),
                        }
                    }
                }
                let [before, after] = [0, 1].map(|end| leaning_by(edge, end, value));
                met_from_outside(
                    body,
                    face,
                    &round.part(before, edge.length() - after),
                    &format!("{which}, between its edge's ends"),
                );
            }
        }
        let cut_down: Vec<DVec3> = corners_of(own)
            .into_iter()
            .map(|corner| {
                end_at(index, corner).map_or(corner, |end| cut_corners(edges, index, end, value)[1])
            })
            .collect();
        corners_are_or_on(
            body,
            own,
            &cut_down,
            |_| false,
            &format!(
                "{what}: face {own}, the other face of {}, cut down by the value along it",
                edge.called
            ),
        );
        assert_eq!(
            sides_met(body, own),
            renamed(sides_met(part, own), &[(shared, face)]),
            "{what}: face {own}, the other face of {}, meets the face laid on it in place of face \
             {shared}, and every other face it met",
            edge.called
        );
    }
    let set_back: Vec<DVec3> = corners_of(shared)
        .into_iter()
        .map(|corner| {
            (0..edges.len())
                .find_map(|edge| {
                    end_at(edge, corner).map(|end| cut_corners(edges, edge, end, value)[0])
                })
                .unwrap_or(corner)
        })
        .collect();
    corners_are_or_on(
        body,
        shared,
        &set_back,
        |_| false,
        &format!(
            "{what}: face {shared}, which the edges share, set back by the value from each, its \
             corner at each mitre where the two set-backs cross"
        ),
    );
    let replaced: Vec<(usize, usize)> = edges
        .iter()
        .zip(&laid)
        .map(|(edge, &face)| (edge.between[1], face))
        .collect();
    assert_eq!(
        sides_met(body, shared),
        renamed(sides_met(part, shared), &replaced),
        "{what}: face {shared}, which the edges share, meets the faces laid in place of the \
         edges' other faces, and every other face it met"
    );
    for &(edge, end, other, _) in &mitres {
        let [one, two] = [edges[edge].between[1], edges[other].between[1]];
        assert_eq!(
            edges_between(body, one, two).len(),
            1,
            "{what}: the concave edge between faces {one} and {two}, below the mitre at {}, is \
             still one sharp edge, ending where the two cuts meet it",
            edges[edge].ends[end].at
        );
    }
}

#[test]
fn a_chamfer_on_the_two_top_edges_at_the_l_s_inner_corner_mitres_their_flats_down_to_its_upright_left_sharp_one_distance_short()
 {
    mitre_in_every_order(
        &ell(),
        ELL_VOLUME,
        &ell_top_edges(),
        &BOTH_ORDERS,
        &[Cut::Flat],
        "the L's two top edges at its inner corner",
    );
}

#[test]
fn a_fillet_on_the_two_top_edges_at_the_l_s_inner_corner_mitres_their_rounds_on_an_ellipse_down_to_its_upright_left_sharp_one_radius_short()
 {
    mitre_in_every_order(
        &ell(),
        ELL_VOLUME,
        &ell_top_edges(),
        &BOTH_ORDERS,
        &[Cut::Round],
        "the L's two top edges at its inner corner",
    );
}

#[test]
fn a_chamfer_and_a_fillet_on_two_rim_edges_of_a_pocket_mitre_them_down_to_the_upright_below_left_sharp()
 {
    mitre_in_every_order(
        &pocketed(),
        POCKETED_VOLUME,
        &front_and_right_rim_edges(),
        &BOTH_ORDERS,
        &[Cut::Flat, Cut::Round],
        "the pocket's rim on its front and right walls, above its front right upright",
    );
}

#[test]
fn a_chamfer_and_a_fillet_on_the_four_rim_edges_of_a_pocket_mitre_its_four_corners() {
    mitre_in_every_order(
        &pocketed(),
        POCKETED_VOLUME,
        &rim_edges(),
        &ROUND_AND_ACROSS,
        &[Cut::Flat, Cut::Round],
        "the pocket's four rim edges",
    );
}

/// An L whose inner corner is not square: its arm [`ell`]'s, 30 long and 6
/// deep along the front, and its stem's right side leaning from the inner
/// corner at (12, 6) to (`top`, 14), the stem's back running from there to
/// the left. Raised 10, its faces are numbered as [`ell`]'s.
fn slanted_ell(top: f64) -> Body {
    raised(
        Contour::straight(vec![
            DVec2::new(0.0, 0.0),
            DVec2::new(30.0, 0.0),
            DVec2::new(30.0, 6.0),
            DVec2::new(12.0, 6.0),
            DVec2::new(top, 14.0),
            DVec2::new(0.0, 14.0),
        ]),
        10.0,
    )
}

/// What the L of [`slanted_ell`] holds: its arm, and its stem a trapezoid 8
/// deep, 12 wide at the arm and `top` at its back.
fn slanted_ell_volume(top: f64) -> f64 {
    (30.0 * 6.0 + 8.0 * (12.0 + top) / 2.0) * 10.0
}

/// The two top edges at the inner corner of the L of [`slanted_ell`]: the
/// arm's back's, from the arm's right end to the stem's right side, which
/// leans across it, and the stem's right side's, from the stem's back, which
/// leans across it too, to the arm's back.
fn slanted_ell_top_edges(top: f64) -> [Convex; 2] {
    let side = DVec2::new(top - 12.0, 8.0);
    let stem_right = DVec3::new(side.y, -side.x, 0.0).normalize();
    let corner = DVec3::new(12.0, 6.0, 10.0);
    [
        Convex {
            called: "the L's top edge on the arm's back",
            between: [TOP, ARM_BACK],
            facing: [DVec3::Z, DVec3::Y],
            ends: [
                End {
                    at: DVec3::new(30.0, 6.0, 10.0),
                    on: ARM_END,
                    facing: DVec3::X,
                    matter_past: false,
                },
                End {
                    at: corner,
                    on: STEM_RIGHT,
                    facing: stem_right,
                    matter_past: true,
                },
            ],
        },
        Convex {
            called: "the L's top edge on the stem's leaning right side",
            between: [TOP, STEM_RIGHT],
            facing: [DVec3::Z, stem_right],
            ends: [
                End {
                    at: DVec3::new(top, 14.0, 10.0),
                    on: STEM_BACK,
                    facing: DVec3::Y,
                    matter_past: false,
                },
                End {
                    at: corner,
                    on: ARM_BACK,
                    facing: DVec3::Y,
                    matter_past: true,
                },
            ],
        },
    ]
}

#[test]
fn a_chamfer_and_a_fillet_at_an_l_s_inner_corner_that_is_not_square_stop_flush_alone_and_mitre_together()
 {
    for (top, called) in [
        (
            18.0,
            "an L whose stem's right side leans over its arm, the air between them 53 degrees wide",
        ),
        (
            6.0,
            "an L whose stem's right side leans away from its arm, the air between them 127 \
             degrees wide",
        ),
    ] {
        let part = slanted_ell(top);
        let volume = slanted_ell_volume(top);
        let edges = slanted_ell_top_edges(top);
        each_stops_flush(
            &part,
            volume,
            &edges,
            &format!("the top edges at the inner corner of {called}, each alone"),
        );
        mitre_in_every_order(
            &part,
            volume,
            &edges,
            &BOTH_ORDERS,
            &[Cut::Flat, Cut::Round],
            &format!("the two top edges at the inner corner of {called}"),
        );
    }
}
