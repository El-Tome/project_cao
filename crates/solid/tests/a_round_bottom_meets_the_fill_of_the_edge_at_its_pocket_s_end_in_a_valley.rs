//! A round bottom meeting the fill of the floor edge at its pocket's end. The
//! block is 30 by 10 by 20, standing on the ground's origin, raised as
//! `cao_part` builds a step: its floor numbered nought, its top one, its sides
//! two to five. A pocket 20 long, x from 5 to 25, 6 wide, y from 2 to 8, and
//! 6 deep, its floor at z = 14, is cut into the middle of its top: its floor
//! numbered six, its walls, front, right, back and left, eight to eleven. Its
//! floor's front and back edges, filled at half its width, meet across the
//! floor in one round bottom, as #698 lays it: a hollow half cylinder of
//! radius 3 about the line y = 5, z = 17. The floor's edge on the right wall,
//! named with them and filled at the same radius, is filled as it would be
//! alone: a hollow quarter cylinder about the line x = 22, z = 17. The two axes
//! cross at (22, 5, 17), and the two cylinders, of one radius, meet on the
//! planes x − 22 = ±(y − 5), from (25, 2, 17), where the round bottom leaves
//! the front wall, down to (22, 5, 14), where both touch the floor's plane,
//! and up to (25, 8, 17): a valley, as #801 mitres two fills at a pocket's
//! bottom corner — the concave twin of #803's mitre against a full round. The
//! uprights at the end are left sharp. With both ends' floor edges named, the
//! left end is the mirror image about x = 15. An operation of the kernel on
//! [`Body`], as #803's mitre is; the step of the history that calls it comes
//! with the application, in #701.
//!
//! Closes #838.
//! - `narrow_pocket`, the round bottom's pair and the right end's floor edge,
//!   filleted at 3: built in every order, one body. The round bottom is one
//!   face answering to both its edges' ranks, a half cylinder of radius 3
//!   about y = 5, z = 17. The end edge's fill is a quarter cylinder about
//!   x = 22, z = 17. The two meet on the planes x − 22 = ±(y − 5), from
//!   (25, 2, 17) through (22, 5, 14) to (25, 8, 17), the uprights left sharp.
//!   The body holds the volume the arithmetic gives, which the booleans give
//!   too —
//!   `a_fillet_on_a_pocket_s_round_bottom_and_the_floor_edge_at_its_end_meets_that_edge_s_fill_in_a_valley_whatever_the_order`.
//!   Read as: in each of the six orders, the value taken whole; the body
//!   exact, closed, uncrossed, holding `5604 − 153π/2`, which the booleans
//!   give too, and its triangles enclosing that within the drawing's
//!   tolerance; the pocket's floor gone, the round bottom one face answering
//!   to both its edges' ranks and the end edge's fill to its own, nothing laid
//!   at a corner, the next step numbering its faces past them; the round
//!   bottom drawn on its half cylinder only and never past the valley, the
//!   end edge's fill on its quarter cylinder only and never short of it, each
//!   met first from its axis; both drawn up to (25, 2, 17), (22, 5, 14) and
//!   (25, 8, 17), sharing no point off the two planes, and meeting by two
//!   edges, one on each plane, at one vertex at each of the three points; the
//!   right wall flat, facing back along x, its corners (25, 2, 17),
//!   (25, 8, 17), (25, 8, 20) and (25, 2, 20), meeting the end edge's fill
//!   along z = 17; the front and the back walls kept from z = 17 up to the
//!   top, each meeting the right wall along its upright, left sharp, by one
//!   edge; the left wall kept above the half circle of radius 3 about
//!   (5, 5, 17) the round bottom leaves on it; every face meeting the faces it
//!   bounds by one edge each; and every order laying the body the listed order
//!   lays, its faces meeting alike, each laid face read by the edge it was
//!   laid on. Read as, where the decision's words and the geometry part: each
//!   of the two curves is a quarter of the ellipse its plane cuts from both
//!   cylinders, from the end of its long axis, on a side wall, to the end of
//!   its short one, at (22, 5, 14) — the decision's "two planar half
//!   ellipses" are the valley's two halves, each a quarter ellipse; and the
//!   floor keeps nothing — the round bottom alone takes all of it at 6 wide,
//!   touching its plane only along y = 5, where the decision has it keep what
//!   lies inside a half circle
//! - The same with both ends' floor edges, each end as above: built in every
//!   order, one body, with the volume the arithmetic gives —
//!   `a_fillet_on_a_pocket_s_round_bottom_and_the_floor_edges_at_both_its_ends_meets_each_end_s_fill_in_a_valley_whatever_the_order`.
//!   Read as: in each of the 24 orders, the first criterion's reading at each
//!   end, the left end the mirror image of the right about x = 15 — its fill
//!   a quarter cylinder about x = 8, z = 17, meeting the round bottom on the
//!   planes 8 − x = ±(y − 5) from (5, 2, 17) through (8, 5, 14) to
//!   (5, 8, 17), the left wall flat from z = 17 up; the body holding
//!   `5568 − 63π`, which the booleans give too
//! - Throughout: the lists `main` builds keep their bodies — every chamfer of
//!   the issue's table that `main` builds, and every list at 6.5 wide, at the
//!   volumes in the table —
//!   `the_lists_main_builds_on_a_pocket_6_or_6_5_wide_keep_their_bodies_in_every_order`.
//!   Read as: on `narrow_pocket` at 3, the chamfer on the pair, on the pair and
//!   the right end's floor edge, on the pair and both ends' floor edges, on
//!   the right end's floor edge alone, and on that edge and the two uprights
//!   at its end; and on the pocket 6.5 wide and 6 deep, the fillet and the
//!   chamfer on each of the table's six lists — each built in every order, the
//!   value taken whole, exact, closed and uncrossed, holding the volume
//!   arithmetic gives, which is the table's to its four decimals. Not in the
//!   issue's wording: the two fillets the table's first column has `main`
//!   build at 6 wide, on the pair alone and on the right end's floor edge
//!   alone, are held there too. The pocket 6.5 wide is 6 deep, as the
//!   table's volumes say its probe was: `narrow_pocket`'s depth is its width
//!   only at 6
//! - Throughout: no body of #698's, #801's or #803's tests changes. The decline
//!   test
//!   `a_fillet_on_a_full_round_meeting_another_full_round_and_on_a_round_bottom_meeting_the_edge_at_its_end_is_declined_as_unsupported_whatever_the_order`
//!   loses its round-bottom case and keeps the two full rounds — no test:
//!   those tests hold their own bodies, in
//!   `a_chamfer_and_a_fillet_take_several_edges_in_one_step.rs`,
//!   `a_chamfer_and_a_fillet_stop_and_mitre_beside_a_concave_edge.rs`,
//!   `a_chamfer_and_a_fillet_fill_concave_edges_meeting_at_a_corner.rs` and
//!   `a_chamfer_and_a_fillet_blend_the_corner_their_edges_meet_at.rs`, and run
//!   beside this file. In the last, the decline gives up the round bottom,
//!   built by this file's first test, and is renamed for what it keeps,
//!   `a_fillet_on_a_full_round_meeting_another_full_round_is_declined_as_unsupported_whatever_the_order`;
//!   `narrow_pocket` moved here with the case
//! - Decided on the way (2026-11-12): the fillet on the right end's floor edge
//!   and both uprights there, at 6 wide — the first of the two lists the
//!   issue's "Set aside" gives #919 — is built. It is this issue's shape
//!   turned upright, and the same rule builds it, with no guard written
//!   against it: the two uprights' fills meet across the right wall in one
//!   standing round bottom, as the pair's meet across the floor, and the
//!   floor's edge on that wall, bounding the face that is gone and meeting
//!   both uprights, is filled as it would be alone and meets it in a valley.
//!   Taken from #919 —
//!   `a_fillet_on_the_floor_edge_and_both_uprights_at_a_pocket_s_end_meets_their_standing_round_bottom_in_a_valley_at_its_foot_whatever_the_order`.
//!   Read as: in each of the six orders, the value taken whole; the body
//!   exact, closed, uncrossed, holding `5352 − 27π/2`, which the booleans give
//!   too, and its triangles enclosing that within the drawing's tolerance; the
//!   right wall gone, the standing round bottom — a hollow half cylinder of
//!   radius 3 about the vertical through (22, 5), from the floor up to the top
//!   — one face answering to both uprights' ranks, and the floor edge's fill a
//!   quarter cylinder about x = 22, z = 17, answering to its own, nothing laid
//!   at a corner; each drawn on its own cylinder only and on its own side of
//!   the valley, met first from its axis; the two meeting on the planes
//!   z − 17 = ±(y − 5), not on the lying case's x − 22 = ±(y − 5): the
//!   valley turned upright runs from (22, 2, 14), where the standing round
//!   bottom leaves the front wall at the floor, up to (25, 5, 17), where both
//!   touch the right wall's plane, and down to (22, 8, 14), each half a
//!   quarter ellipse, by two edges, one vertex at each of the three points,
//!   and no shared point off the planes; the floor kept up to x = 22, meeting
//!   the fill along that line, its front and back edges left sharp; the front
//!   and the back walls kept up to x = 22 from the floor to the top; the left
//!   wall whole; the top's rim taken round the half circle of radius 3 about
//!   (22, 5, 20); every face meeting the faces it bounds by one edge each; and
//!   every order laying the body the listed order lays
//! - Not in the issue's wording, from its "Set aside": the second of #919's
//!   lists, the pair, the right end's floor edge and both uprights there —
//!   two round bottoms meeting, the floor corners to be closed by a hollow
//!   quarter of a ball — stays #919's. The fillet and the chamfer on it stay
//!   declined `Unsupported` in every order at 6 wide, as on `main`, so that
//!   what lets a round bottom meet the end edge's fill lets in nothing else,
//!   #822's rule —
//!   `a_fillet_and_a_chamfer_on_the_round_bottom_the_floor_edge_at_its_end_and_both_uprights_there_stay_declined_as_unsupported_whatever_the_order`
//! - Decided on the way, after the review (2026-11-12): what the guard now
//!   lets through but the laying refuses. A cut and a fill meeting where a
//!   slope or a round bottom ends do not commute — the one laid against the
//!   other takes or gives back what the other just gave or took — and the
//!   lists where they do pass `at_their_ends` and are refused by a later
//!   laying check, `cut_along` or `sloped`, alone. Each stays declined
//!   `Unsupported` in every order, its parts laid alone, so that whoever
//!   loosens a laying check sees it go red, #822's rule. #791's stair, its
//!   slope — the tread's nose and foot chamfered at 5, meeting across it —
//!   with the tread's convex edge on the stair's right side; the same stair
//!   between two walls, the tread's edge on the right wall concave, which
//!   declines too; the slot 6 wide open at x = 30, its round bottom filleted
//!   at 3 with the floor's convex edge on the block's right face; and the
//!   pocket's standing round bottom, its right end's two uprights filleted at
//!   3, with the convex rim edge on top of that wall —
//!   `a_cut_and_a_fill_that_meet_where_a_slope_or_a_round_bottom_ends_are_declined_as_unsupported_whatever_the_order`
//! - Decided on the way, after the review (2026-11-12): an end at a slant. A
//!   pocket 6 wide whose outline is (5, 2), (25, 2), (27, 8), (5, 8), its
//!   round bottom and its slanted end's floor edge filleted at 3: built in
//!   every order, one body, the volume the arithmetic gives, which the
//!   booleans give too —
//!   `a_fillet_on_a_pocket_s_round_bottom_and_the_floor_edge_of_its_slanted_end_meets_that_edge_s_fill_in_a_valley_whatever_the_order`.
//!   Read as: in each of the six orders, the value taken whole; the body
//!   exact, closed, uncrossed, holding `5622 − 189π/2 − 12√10 + 9√10π/2`,
//!   5331.8778, and its triangles enclosing that within the drawing's
//!   tolerance; the floor gone, the round bottom one face answering to both
//!   its edges' ranks, on its half cylinder and never past the valley, the end
//!   edge's fill on its quarter cylinder about the line 3 in from the slanted
//!   wall and over the floor and never short of it, each met first from its
//!   axis; the two meeting on the two upright planes through the crossing of
//!   their axes, (26 − √10, 5, 17), that halve the angles between them, from
//!   (25, 2, 17) through (26 − √10, 5, 14) to (27, 8, 17), by two edges, at one
//!   vertex at each of the three points, sharing no point off the planes; the
//!   slanted wall flat, facing back square to itself, kept above z = 17 and
//!   meeting the fill along that line; each face meeting the faces it bounds
//!   by one edge each; and every order laying the body the listed order lays
//! - Decided on the way, after the review (2026-11-12): a slot open at its
//!   right end, cut as the pocket is but out through the block's right face
//!   at x = 30, its round bottom and its closed left end's floor edge filleted
//!   at 3: built as the pocket builds it, the left end's fill meeting the
//!   round bottom in a valley, the round bottom running out to the block's
//!   right face —
//!   `a_fillet_on_a_slot_s_round_bottom_and_the_floor_edge_at_its_closed_end_meets_that_edge_s_fill_in_a_valley_whatever_the_order`.
//!   Read as the first criterion, mirrored to the left end: in each of the six
//!   orders, the body holding `5514 − 99π`, which the booleans give too; the
//!   left fill a quarter cylinder about x = 8, z = 17, meeting the round
//!   bottom on 8 − x = ±(y − 5) from (5, 2, 17) through (8, 5, 14) to
//!   (5, 8, 17); the left wall flat from z = 17 up; the front and the back
//!   walls kept from z = 17 up to the top, out to the block's right face; and
//!   the round bottom meeting that face, where the slot opens, by one edge
//!
//! The volume is arithmetic. The block holds 6000, and a pocket `w` wide and 6
//! deep takes `120w` of it: `narrow_pocket` holds 5280. Each fill alone adds
//! its section along its edge, trimmed flush on the walls at its ends as #801
//! trims it: a round the strip `r²(1 − π/4)` between the two faces and the
//! round, a flat the triangle `d²/2` — the floor's front and back edges 20
//! long, its edges on the end walls `w`, the uprights 6. At 6 wide the pair's
//! two rounds meet along the floor's middle line in the round bottom without
//! overlapping, and the two uprights' flats along the end wall's. Where two
//! named edges meet at one of the pocket's four bottom corners, their fills
//! both hold what lies within both in the cube of side the value there,
//! `r³(5/3 − π/2)` or `d³/3`, which the body counts once; where three do, the
//! corner closed by a hollow piece of sphere or a triangle, the body holds the
//! three fills' sum less `r³(2 − 7π/12)` or `2d³/3` — #801's arithmetic.
//!
//! The round bottom and the right end's fill share what lies at the two
//! corners at x = 25. Measured from the crossing of their axes, `u = x − 22`,
//! `v = y − 5` and `t = 17 − z`, the round bottom holds at a depth `t` the
//! points whose `|v|` is past `√(r² − t²)`, the end edge's fill those whose `u`
//! is: both, `2(r − √(r² − t²))²`, and over the depth `2r³(5/3 − π/2)`, the
//! two corners' share. So the body holds
//! `5280 + 40r²(1 − π/4) + 6r²(1 − π/4) − 2r³(5/3 − π/2)`, that is
//! `5604 − 153π/2`, 5363.67; with both ends,
//! `5280 + 52r²(1 − π/4) − 4r³(5/3 − π/2)`, `5568 − 63π`, 5370.08. The two
//! cylinders meet where `v² = u²`, which past the crossing is the pair of
//! planes `x − 22 = ±(y − 5)`. The booleans give it too: the pocket joined to
//! the round bottom's prism, raised along the whole pocket and into the matter
//! past both its ends, then to each end edge's fill prism, raised across the
//! pocket and into the matter past its side walls; each prism holds what the
//! other holds past the valley, which is where their surfaces meet.
//!
//! Turned upright, the right end's floor edge and the two uprights there: the
//! uprights' fills add `6r²(1 − π/4)` each up the pocket's depth, the floor
//! edge's `6r²(1 − π/4)` across its width, and at each of the two corners at
//! x = 25 the floor edge's fill and an upright's both hold `r³(5/3 − π/2)`,
//! counted once. The body holds `5280 + 18r²(1 − π/4) − 2r³(5/3 − π/2)`, that
//! is `5352 − 27π/2`, 5309.5885. Measured from the crossing of the axes,
//! `u = x − 22`, `v = y − 5` and `t = 17 − z`, the standing round bottom holds
//! the points whose `|v|` is past `√(r² − u²)`, the floor edge's fill those
//! whose `t` is, and the two cylinders meet where `v² = t²`: the planes
//! `z − 17 = ±(y − 5)`. The booleans give it too, the pocket joined to the
//! standing round bottom's prism, raised from under the floor up to the
//! block's top, then to the floor edge's fill prism.
//!
//! The slot open at its right end is 25 long, from x = 5 to the block's right
//! face, and takes 900: it holds 5100. Its round bottom adds its section along
//! the 25, its left end's floor edge along the 6, and the two share at the two
//! corners at x = 5 what the right end's share at x = 25 in the pocket:
//! `5100 + 50r²(1 − π/4) + 6r²(1 − π/4) − 2r³(5/3 − π/2)`, `5514 − 99π`,
//! 5202.98. The booleans give it with the round bottom's prism raised up to
//! that face, past which lies air.
//!
//! The pocket whose right end stands at a slant, from (25, 2) to (27, 8), has
//! the area `(20 + 22)/2 · 6`, 126, and holds 5244. Its round bottom, trimmed
//! flush on the slanted wall, adds its section along the pocket's length at
//! each point of it, `20 + (y − 2)/3`; the section lies evenly either side of
//! y = 5, so the round bottom adds its section along 21, `378(1 − π/4)`. The
//! slanted end's floor edge is `2√10` long, between faces square to each
//! other, the floor and the slanted wall; its fill, trimmed flush on the front
//! and the back walls, runs the same `2√10` along every line through its
//! section, each line rising 6 across the pocket at 3 in `√10`:
//! `2√10 · r²(1 − π/4)`. At each of the two corners where the end edge meets
//! the front's and the back's floor edges, the fills of both share the square
//! corner's `r³(5/3 − π/2)` stretched by one over the sine of the angle the two
//! walls make, a unit off one by a unit off the other covering `1/sin α` of
//! the floor: both angles, 108.4° and 71.6°, have the sine `3/√10`. The body
//! holds `5244 + 378(1 − π/4) + 18√10(1 − π/4) − 2r³(5/3 − π/2)·√10/3`, that is
//! `5622 − 189π/2 − 12√10 + 9√10π/2`, 5331.8778, which the reviewer's probe
//! measured. The two cylinders, of one radius, their axes crossing at
//! (26 − √10, 5, 17), meet on the two upright planes through that crossing
//! that halve the angles between the axes: there the distance out past the
//! crossing square to the slanted wall equals the distance across from the
//! middle line. The booleans give it too, the end edge's fill prism raised
//! along the slanted end from the matter past the back wall to the matter past
//! the front one.
//!
//! The lists `main` builds are held to the same arithmetic. At 6 wide,
//! chamfered at 3: the pair 5460; the pair and the right end's floor edge,
//! `5460 + 27 − 2·9`, 5469; the pair and both ends' floor edges 5478; the right
//! end's floor edge alone 5307; that edge and the two uprights at its end,
//! `5280 + 27 + 54 − 2·9`, 5343; filleted, the pair `5640 − 90π`, 5357.2566,
//! and the right end's floor edge alone `5334 − 27π/2`, 5291.5885. The pocket
//! 6.5 wide and 6 deep holds 5220: the pair 5297.2566 and 5400; the pair and
//! the right end's floor edge 5304.6338 and 5411.25; the pair and both ends'
//! floor edges 5312.0110 and 5422.5; the right end's floor edge alone
//! 5232.5542 and 5249.25; that edge and the two uprights 5250.5542 and
//! 5285.25; the pair, that edge and the two uprights, each corner at x = 25
//! closed, 5323.9480 and 5447.25, filleted then chamfered — the table's
//! volumes, to its four decimals.

use std::collections::{BTreeMap, BTreeSet};
use std::f64::consts::PI;

use cao_solid::profile::{Contour, Frame, Profile, Run};
use cao_solid::soundness::{closed, enclosed, uncrossed};
use cao_solid::{Body, Declined, EdgeName, Loop};
use glam::{DVec2, DVec3};

/// How far a corner may stand from where the arithmetic puts it: each is
/// placed exactly, so what is left is rounding.
const PLACE: f64 = 1e-9;

/// How near the arithmetic an exact volume comes, as a fraction of it.
const EXACT: f64 = 1e-9;

/// How far the triangles may stand from a round, as a share of the reach the
/// body carries: the drawing's tolerance.
const DRAWN: f64 = 2e-4;

/// How far from the valley a ray aims, on either side of it, to meet the
/// triangles of the fill it aims at rather than the other's: well over the
/// drawing's tolerance on the block, [`DRAWN`] times its reach.
const CLEAR_OF_THE_VALLEY: f64 = 0.1;

/// How near the volumes the issue's table writes, to four decimals, the
/// arithmetic comes.
const AS_MEASURED: f64 = 5e-5;

/// The numbers a raised block's top and right face answer to.
const TOP: usize = 1;
const BLOCK_RIGHT: usize = 3;

/// The numbers the pocket's faces answer to: its floor six, its walls, front,
/// right, back and left, eight to eleven, the tool's top, above the block,
/// gone.
const POCKET_FLOOR: usize = 6;
const POCKET_FRONT: usize = 8;
const POCKET_RIGHT: usize = 9;
const POCKET_BACK: usize = 10;
const POCKET_LEFT: usize = 11;

/// The block the pocket is cut into: 30 along X, 10 across, 20 high.
const LENGTH: f64 = 30.0;
const BREADTH: f64 = 10.0;
const HEIGHT: f64 = 20.0;

/// Where the pocket's walls stand: the left at x = 5, the right at x = 25,
/// the front at y = 2; the back stands its width further on.
const LEFT_WALL_AT: f64 = 5.0;
const RIGHT_WALL_AT: f64 = 25.0;
const FRONT_WALL_AT: f64 = 2.0;

/// The pocket's floor, 6 under the block's top, whatever its width.
const FLOOR_AT: f64 = 14.0;
const DEPTH: f64 = HEIGHT - FLOOR_AT;
const POCKET_LENGTH: f64 = RIGHT_WALL_AT - LEFT_WALL_AT;

/// The width of [`narrow_pocket`], twice [`RADIUS`]: the fills of its floor's
/// front and back edges meet across the floor.
const WIDTH: f64 = 6.0;

/// The width of the pocket the issue's table measures beside it, where the
/// two fills stay a quarter apart.
const WIDER: f64 = 6.5;

/// The value every list is cut at: half [`WIDTH`].
const RADIUS: f64 = WIDTH / 2.0;

/// The pocket's edges, each as the two faces it lies between: the floor's
/// on each wall, and the uprights at the right end.
const FLOOR_FRONT: [usize; 2] = [POCKET_FLOOR, POCKET_FRONT];
const FLOOR_BACK: [usize; 2] = [POCKET_FLOOR, POCKET_BACK];
const FLOOR_RIGHT: [usize; 2] = [POCKET_FLOOR, POCKET_RIGHT];
const FLOOR_LEFT: [usize; 2] = [POCKET_FLOOR, POCKET_LEFT];
const FRONT_RIGHT: [usize; 2] = [POCKET_FRONT, POCKET_RIGHT];
const BACK_RIGHT: [usize; 2] = [POCKET_BACK, POCKET_RIGHT];

/// A list of edges, each as the two faces it lies between, and what it is
/// called.
type Listed = Vec<([usize; 2], &'static str)>;

/// The round bottom's pair, the floor's front and back edges, and what each
/// is called.
const PAIR: [([usize; 2], &str); 2] = [
    (FLOOR_FRONT, "the floor's edge on the front wall"),
    (FLOOR_BACK, "the floor's edge on the back wall"),
];

/// The two uprights at the right end, where the front and the back walls meet
/// the right wall.
const FRONT_UPRIGHT: ([usize; 2], &str) = (
    FRONT_RIGHT,
    "the upright where the front wall meets the right wall",
);
const BACK_UPRIGHT: ([usize; 2], &str) = (
    BACK_RIGHT,
    "the upright where the back wall meets the right wall",
);

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

/// The tool an outline raises from `base` by `height`, for `part` to number
/// its faces after its own, as `cao_part` raises a step: its floor first, then
/// its top, then its walls, one for each run of the outline.
fn raised_on(part: &Body, outline: Contour, base: f64, height: f64) -> Body {
    part.tool_raised(
        &profile(outline),
        Frame {
            origin: DVec3::Z * base,
            ..ground()
        },
        DVec3::Z * height,
    )
    .expect("the tool raises")
}

/// The part joined to the block between `from` and `to`, seen from above,
/// standing on the ground `height` high.
fn joined_to(part: &Body, [from, to]: [DVec2; 2], height: f64) -> Body {
    part.union(&raised_on(part, Contour::rectangle(from, to), 0.0, height))
        .expect("the join is built")
}

/// The block 30 by 10 by 20 with `outline` cut into its top down to a floor
/// at z = 14, as `cao_part` cuts a step: the tool's faces numbered past the
/// block's, its floor six, its top, above the block, seven and gone, its walls
/// from eight, one for each run of the outline.
fn cut_into_the_block(outline: Contour) -> Body {
    let block = joined_to(
        &Body::default(),
        [DVec2::ZERO, DVec2::new(LENGTH, BREADTH)],
        HEIGHT,
    );
    block
        .difference(&raised_on(&block, outline, FLOOR_AT, DEPTH + 1.0))
        .expect("the cut is made")
}

/// The block with a pocket `width` wide cut into the middle of its top, from
/// x = 5 to x = 25 and from y = 2: its walls, front, right, back and left,
/// eight to eleven.
fn pocket(width: f64) -> Body {
    cut_into_the_block(Contour::rectangle(
        DVec2::new(LEFT_WALL_AT, FRONT_WALL_AT),
        DVec2::new(RIGHT_WALL_AT, FRONT_WALL_AT + width),
    ))
}

/// The pocket [`WIDTH`] wide and as deep: its floor's front and back edges
/// meet inwards, facing each other across the floor, and filled at
/// [`RADIUS`] meet in one round bottom.
fn narrow_pocket() -> Body {
    pocket(WIDTH)
}

/// A slot [`WIDTH`] wide cut as [`narrow_pocket`] is but running out through
/// the block's right face: its tool reaches a unit past it, its right wall
/// outside the block and gone. Its left end is closed by its left wall, and
/// the round bottom along its floor meets the block's right face, where the
/// slot opens.
fn open_slot() -> Body {
    cut_into_the_block(Contour::rectangle(
        DVec2::new(LEFT_WALL_AT, FRONT_WALL_AT),
        DVec2::new(LENGTH + 1.0, FRONT_WALL_AT + WIDTH),
    ))
}

/// A pocket [`WIDTH`] wide whose right end stands at a slant seen from above,
/// from (25, 2) on the front wall to (27, 8) on the back, its corners there
/// 108.4° and 71.6°: its walls, front, slanted end, back and left, eight to
/// eleven.
fn slanted_pocket() -> Body {
    cut_into_the_block(Contour::straight(vec![
        DVec2::new(LEFT_WALL_AT, FRONT_WALL_AT),
        DVec2::new(RIGHT_WALL_AT, FRONT_WALL_AT),
        slanted_end_at_the_back(),
        DVec2::new(LEFT_WALL_AT, FRONT_WALL_AT + WIDTH),
    ]))
}

/// Where the slanted end of [`slanted_pocket`] meets its back wall, seen
/// from above: 2 further along X than where it meets the front wall.
fn slanted_end_at_the_back() -> DVec2 {
    DVec2::new(RIGHT_WALL_AT + 2.0, FRONT_WALL_AT + WIDTH)
}

/// #791's stair: a block 30 long from y = 10 to 20 and 20 high, and in front
/// of it a block from y = 0 to 10, as long and 10 high, joined to it. The
/// lower block's top, seven, is the tread: its nose, along the lower block's
/// front, eight, meets outwards, and its foot, against the riser, the taller
/// block's front, two, meets inwards. The two blocks' right sides merge into
/// one face, and the tread's edge on it meets outwards.
fn stair() -> Body {
    let taller = joined_to(
        &Body::default(),
        [DVec2::new(0.0, 10.0), DVec2::new(LENGTH, 20.0)],
        HEIGHT,
    );
    joined_to(&taller, [DVec2::ZERO, DVec2::new(LENGTH, 10.0)], 10.0)
}

/// The stair between two walls 5 thick, standing as deep and as high as the
/// stair against its right and its left sides: the tread's edge on the right
/// wall, whose face against the stair is seventeen, meets inwards.
fn stair_between_two_walls() -> Body {
    let walled = joined_to(
        &stair(),
        [DVec2::new(LENGTH, 0.0), DVec2::new(LENGTH + 5.0, 20.0)],
        HEIGHT,
    );
    joined_to(
        &walled,
        [DVec2::new(-5.0, 0.0), DVec2::new(0.0, 20.0)],
        HEIGHT,
    )
}

/// The name of the one edge between a face answering to `one` and a face
/// answering to `other`, whatever else either answers to: an edge on a face
/// merged from several.
fn edge_touching(body: &Body, one: usize, other: usize) -> EdgeName {
    let [one, other] = [one as u32, other as u32];
    let named: Vec<EdgeName> = body
        .edge_names()
        .into_iter()
        .filter(|name| {
            let [first, second] = &name.between;
            (first.contains(&one) && second.contains(&other))
                || (first.contains(&other) && second.contains(&one))
        })
        .collect();
    assert_eq!(
        named.len(),
        1,
        "one edge between a face answering to {one} and one answering to {other}: {named:?}"
    );
    named[0].clone()
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

/// Whether some point of `points` stands at `point`.
fn drawn_through(points: &[DVec3], point: DVec3) -> bool {
    points.iter().any(|other| other.distance(point) < PLACE)
}

/// Whether every point of `some` stands at a point of `others`, and every
/// point of `others` at one of `some`.
fn same_points(some: &[DVec3], others: &[DVec3]) -> bool {
    some.iter().all(|point| drawn_through(others, *point))
        && others.iter().all(|point| drawn_through(some, *point))
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
/// of it each of `ways` points to from the centre.
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
/// answers to: a round bottom answers to both its edges'.
fn sides_answering(faces: &[&[usize]]) -> Vec<Vec<u32>> {
    let mut sorted: Vec<Vec<u32>> = faces
        .iter()
        .map(|numbers| numbers.iter().map(|&number| number as u32).collect())
        .collect();
    sorted.sort();
    sorted
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

/// The round a fillet lays in a concave corner: a cylinder of `radius` about
/// the line from `start` along `along` for `length`, standing in the air the
/// two faces leave between them, the round facing its axis, from the way the
/// edge's first face faces to the way its second does.
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
    /// face's way to its second's: toward its axis.
    fn facing_at(&self, share: f64) -> DVec3 {
        let [first, second] = self.facing;
        let turn = first.dot(second).acos() * share;
        let across = (second - first * first.dot(second)).normalize();
        first * turn.cos() + across * turn.sin()
    }

    /// Whether a point lies on the round: on its cylinder, between its two
    /// ends, and on the edge's side of its axis, between its two sides.
    fn holds(&self, point: DVec3) -> bool {
        let off = point - self.start;
        let along = off.dot(self.along);
        let across = off - self.along * along;
        let [first, second] = self.facing;
        let least = self.radius * first.dot(second);
        (across.length() - self.radius).abs() < PLACE
            && (-PLACE..=self.length + PLACE).contains(&along)
            && across.dot(-first) >= least - PLACE
            && across.dot(-second) >= least - PLACE
    }

    /// The round's area, its turn times its radius times its length.
    fn area(&self) -> f64 {
        let [first, second] = self.facing;
        first.dot(second).acos() * self.radius * self.length
    }
}

/// The round bottom of a pocket or a slot [`WIDTH`] wide, whole: two hollow
/// quarter rounds about the line [`RADIUS`] over the middle of the floor, from
/// the left wall `length` along X, from facing the front wall's way to the
/// floor's, and from the floor's to the back wall's.
fn round_bottom_along(length: f64) -> [Hollow; 2] {
    [[DVec3::Y, DVec3::Z], [DVec3::Z, -DVec3::Y]].map(|facing| Hollow {
        start: DVec3::new(LEFT_WALL_AT, FRONT_WALL_AT + RADIUS, FLOOR_AT + RADIUS),
        along: DVec3::X,
        length,
        radius: RADIUS,
        facing,
    })
}

/// A pocket or a slot [`WIDTH`] wide cut into the block, the round bottom
/// along its floor running from the left wall to `reaches_to` along X: what
/// it is called, and the face its round bottom meets at its right end where
/// that end's floor edge is not named — the pocket's right wall, or the
/// block's right face where the slot opens.
struct Pocketed {
    part: Body,
    called: &'static str,
    reaches_to: f64,
    right_face: usize,
}

impl Pocketed {
    /// [`narrow_pocket`], closed at both ends.
    fn narrow() -> Pocketed {
        Pocketed {
            part: narrow_pocket(),
            called: "the pocket 6 wide",
            reaches_to: RIGHT_WALL_AT,
            right_face: POCKET_RIGHT,
        }
    }

    /// [`open_slot`], closed at its left end only.
    fn open_slot() -> Pocketed {
        Pocketed {
            part: open_slot(),
            called: "the slot 6 wide open at its right end",
            reaches_to: LENGTH,
            right_face: BLOCK_RIGHT,
        }
    }

    /// Whether a wall closes the end `end`.
    fn closed_at(&self, end: End) -> bool {
        end == End::Left || self.right_face == POCKET_RIGHT
    }

    /// The face at the end `end`: its wall, or where the slot opens, the
    /// block's right face.
    fn face_at(&self, end: End) -> usize {
        match end {
            End::Right => self.right_face,
            End::Left => POCKET_LEFT,
        }
    }

    /// The round bottom along its floor, whole.
    fn round_bottom(&self) -> [Hollow; 2] {
        round_bottom_along(self.reaches_to - LEFT_WALL_AT)
    }

    /// The prism the round bottom adds, raised along X from the matter past
    /// the left wall into the matter past the right one, or, where the slot
    /// opens, up to the block's right face.
    fn round_bottom_tool(&self) -> Body {
        let to = if self.closed_at(End::Right) {
            RIGHT_WALL_AT + RADIUS
        } else {
            LENGTH
        };
        round_bottom_tool(&self.part, to)
    }

    /// What it holds once the edges `listed` are cut at [`RADIUS`], as
    /// [`kept_along`] reckons it.
    fn kept(&self, cut: Cut, listed: &[[usize; 2]]) -> f64 {
        kept_along(cut, WIDTH, self.reaches_to - LEFT_WALL_AT, listed)
    }
}

/// Holds that every ray aimed from the round's axis out to it, over a grid
/// along it and round it, meets the face numbered `face` first, facing back
/// along the ray, toward the axis, wherever `shows` says the round shows at
/// the point the ray aims at; and that some ray was aimed.
fn met_from_its_axis_where(
    body: &Body,
    face: usize,
    hollow: &Hollow,
    shows: impl Fn(DVec3) -> bool,
    what: &str,
) {
    let mut aimed = 0;
    for tenth in 0..10 {
        let along = (f64::from(tenth) + 0.5) / 10.0;
        for share in [0.1, 0.3, 0.5, 0.7, 0.9] {
            let facing = hollow.facing_at(share);
            let from = hollow.axis_at(along);
            let aim = from - facing * hollow.radius;
            if !shows(aim) {
                continue;
            }
            aimed += 1;
            let hit = body
                .ray_hit(from, -facing)
                .unwrap_or_else(|| panic!("{what}: a ray from {from} on the axis meets nothing"));
            assert_eq!(
                hit.face, face,
                "{what}: a ray from {from} on the axis toward {aim} meets face {} before the \
                 round",
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
    assert!(aimed > 0, "{what}: some ray aims where the round shows");
}

/// One of the pocket's two ends, whose floor edge a list may name with the
/// round bottom's pair.
#[derive(Clone, Copy, Debug, PartialEq)]
enum End {
    Right,
    Left,
}

impl End {
    /// The floor's edge on the end's wall, as the two faces it lies between,
    /// and what it is called.
    fn edge(self) -> ([usize; 2], &'static str) {
        match self {
            End::Right => (FLOOR_RIGHT, "the floor's edge on the right wall"),
            End::Left => (FLOOR_LEFT, "the floor's edge on the left wall"),
        }
    }

    /// The end's wall.
    fn wall(self) -> usize {
        match self {
            End::Right => POCKET_RIGHT,
            End::Left => POCKET_LEFT,
        }
    }

    /// Which way along X the end's wall stands from the pocket's middle.
    fn outward(self) -> f64 {
        match self {
            End::Right => 1.0,
            End::Left => -1.0,
        }
    }

    /// Where the end's wall stands along X.
    fn wall_at(self) -> f64 {
        match self {
            End::Right => RIGHT_WALL_AT,
            End::Left => LEFT_WALL_AT,
        }
    }

    /// Where the axis of the end edge's fill stands along X: [`RADIUS`] in
    /// from the wall, where it crosses the round bottom's.
    fn axis_at(self) -> f64 {
        self.wall_at() - RADIUS * self.outward()
    }

    /// The end edge's fill on [`narrow_pocket`], whole: a hollow quarter round
    /// about the line [`RADIUS`] in from the wall and over the floor, across
    /// the pocket from its front wall to its back, from facing the floor's way
    /// to facing the wall's.
    fn fill(self) -> Hollow {
        Hollow {
            start: DVec3::new(self.axis_at(), FRONT_WALL_AT, FLOOR_AT + RADIUS),
            along: DVec3::Y,
            length: WIDTH,
            radius: RADIUS,
            facing: [DVec3::Z, DVec3::X * -self.outward()],
        }
    }

    /// How far a point stands past the valley at this end, on the side of the
    /// end edge's fill: how far it stands out along X past the crossing of
    /// the two axes, less how far across from the pocket's middle line. Nought
    /// on the two planes the fills meet on.
    fn past_the_valley(self, point: DVec3) -> f64 {
        (point.x - self.axis_at()) * self.outward() - (point.y - FRONT_WALL_AT - RADIUS).abs()
    }

    /// The valley's three points: where the round bottom leaves the front
    /// wall at the end wall, where both fills touch the floor's plane, and
    /// where the round bottom leaves the back wall.
    fn valley(self) -> [DVec3; 3] {
        let top = FLOOR_AT + RADIUS;
        [
            DVec3::new(self.wall_at(), FRONT_WALL_AT, top),
            DVec3::new(self.axis_at(), FRONT_WALL_AT + RADIUS, FLOOR_AT),
            DVec3::new(self.wall_at(), FRONT_WALL_AT + WIDTH, top),
        ]
    }

    /// The prism the end edge's fill adds on [`narrow_pocket`], raised across
    /// the pocket and a unit into the matter past its front and back walls:
    /// what lies under the quarter round, from its axis out past the wall and
    /// down under the floor, all past the pocket in the matter.
    fn tool(self, part: &Body) -> Body {
        let r = RADIUS;
        let axis = self.axis_at();
        let wall = self.wall_at();
        let beyond = wall + r * self.outward();
        let [floor, round_top, below] = [FLOOR_AT, FLOOR_AT + r, FLOOR_AT - r];
        let (corners, arc) = match self {
            End::Right => (
                vec![
                    DVec2::new(axis, below),
                    DVec2::new(beyond, below),
                    DVec2::new(beyond, round_top),
                    DVec2::new(wall, round_top),
                    DVec2::new(axis, floor),
                ],
                3,
            ),
            End::Left => (
                vec![
                    DVec2::new(beyond, below),
                    DVec2::new(axis, below),
                    DVec2::new(axis, floor),
                    DVec2::new(wall, round_top),
                    DVec2::new(beyond, round_top),
                ],
                2,
            ),
        };
        let mut runs = vec![Run::Straight; corners.len()];
        runs[arc] = Run::Round {
            center: DVec2::new(axis, round_top),
            turn: -PI / 2.0,
        };
        part.tool_raised(
            &profile(Contour { corners, runs }),
            Frame {
                origin: DVec3::Y * (FRONT_WALL_AT + WIDTH + 1.0),
                u: DVec3::X,
                v: DVec3::Z,
            },
            -DVec3::Y * (WIDTH + 2.0),
        )
        .expect("the tool raises")
    }
}

/// The prism the round bottom adds on a pocket or a slot [`WIDTH`] wide,
/// raised along X from the matter past its left wall up to `to`: what lies
/// under the half round about the line [`RADIUS`] over the middle of the
/// floor, out a unit into the matter past the front and back walls and down
/// under the floor.
fn round_bottom_tool(part: &Body, to: f64) -> Body {
    let r = RADIUS;
    let [near, far] = [FRONT_WALL_AT, FRONT_WALL_AT + WIDTH];
    let [round_top, below] = [FLOOR_AT + r, FLOOR_AT - r];
    let corners = vec![
        DVec2::new(near - 1.0, below),
        DVec2::new(far + 1.0, below),
        DVec2::new(far + 1.0, round_top),
        DVec2::new(far, round_top),
        DVec2::new(near, round_top),
        DVec2::new(near - 1.0, round_top),
    ];
    let mut runs = vec![Run::Straight; corners.len()];
    runs[3] = Run::Round {
        center: DVec2::new(near + r, round_top),
        turn: -PI,
    };
    part.tool_raised(
        &profile(Contour { corners, runs }),
        Frame {
            origin: DVec3::X * (LEFT_WALL_AT - r),
            u: DVec3::Y,
            v: DVec3::Z,
        },
        DVec3::X * (to - LEFT_WALL_AT + r),
    )
    .expect("the tool raises")
}

/// What a pocket or a slot holds once the round bottom and the fills of the
/// floor edges at `ends` are laid at [`RADIUS`], computed apart from the
/// step: the part joined to the round bottom's prism, then to each end edge's,
/// by the booleans.
fn valley_by_booleans(pocketed: &Pocketed, ends: &[End]) -> f64 {
    let filled = pocketed
        .part
        .union(&pocketed.round_bottom_tool())
        .expect("the round bottom's prism joins the part");
    ends.iter()
        .fold(filled, |body, end| {
            body.union(&end.tool(&body))
                .expect("an end edge's fill prism joins the body")
        })
        .volume()
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

    /// What one edge's fill at `value` alone adds for each unit of its
    /// length, between two faces square to each other: the strip between them
    /// and the round, `r²(1 − π/4)`, or the triangle `d²/2`.
    fn section(self, value: f64) -> f64 {
        match self {
            Cut::Fillet => value * value * (1.0 - PI / 4.0),
            Cut::Chamfer => value * value / 2.0,
        }
    }

    /// What the fills at `value` of `count` named edges meeting at a pocket's
    /// bottom corner hold more than once there, which the body counts once:
    /// nothing for one; for two, mitred in a valley, `r³(5/3 − π/2)` or
    /// `d³/3`; for three, closed by a hollow piece of sphere or a triangle,
    /// `r³(2 − 7π/12)` or `2d³/3`.
    fn held_twice(self, count: usize, value: f64) -> f64 {
        let cube = value.powi(3);
        match (self, count) {
            (_, 0 | 1) => 0.0,
            (Cut::Fillet, 2) => cube * (5.0 / 3.0 - PI / 2.0),
            (Cut::Chamfer, 2) => cube / 3.0,
            (Cut::Fillet, _) => cube * (2.0 - 7.0 * PI / 12.0),
            (Cut::Chamfer, _) => 2.0 * cube / 3.0,
        }
    }

    /// The cut at `value` on the edges `names` finds on `part`.
    fn laid_on(self, part: &Body, names: &[EdgeName], value: f64) -> Result<Laid, Declined> {
        match self {
            Cut::Fillet => part.fillet_edges(names, value).map(|laid| Laid {
                asked: laid.asked,
                taken: laid.taken,
                body: laid.body,
            }),
            Cut::Chamfer => part.chamfer_edges(names, value).map(|laid| Laid {
                asked: laid.asked,
                taken: laid.taken,
                body: laid.body,
            }),
        }
    }
}

/// Whether two edges, each as the two faces it lies between, are one,
/// whichever face each lists first.
fn same_edge([one, other]: [usize; 2], edge: [usize; 2]) -> bool {
    edge == [one, other] || edge == [other, one]
}

/// What the pocket `width` wide holds once the edges `listed` are cut at
/// [`RADIUS`], as the module's documentation derives it: each edge's fill
/// added whole along it, and at each of the pocket's four bottom corners what
/// the fills of the named edges there hold more than once counted once.
fn kept(cut: Cut, width: f64, listed: &[[usize; 2]]) -> f64 {
    kept_along(cut, width, POCKET_LENGTH, listed)
}

/// What a pocket or a slot `width` wide and `length` long holds once the
/// edges `listed` are cut at [`RADIUS`], as [`kept`] reckons it: the floor's
/// front and back edges `length` long; a slot's open end, where no wall
/// stands, has no edge on its wall to name.
fn kept_along(cut: Cut, width: f64, length: f64, listed: &[[usize; 2]]) -> f64 {
    let value = RADIUS;
    let long = length;
    let ends = [POCKET_RIGHT, POCKET_LEFT];
    let sides = [POCKET_FRONT, POCKET_BACK];
    let length = |&[one, other]: &[usize; 2]| match (one == POCKET_FLOOR, ends.contains(&other)) {
        (true, true) => width,
        (true, false) => long,
        (false, _) => DEPTH,
    };
    let mut held_twice = 0.0;
    for end in ends {
        for side in sides {
            let corner = [[POCKET_FLOOR, side], [POCKET_FLOOR, end], [side, end]];
            let named = corner
                .iter()
                .filter(|&&edge| listed.iter().any(|&named| same_edge(named, edge)))
                .count();
            held_twice += cut.held_twice(named, value);
        }
    }
    LENGTH * BREADTH * HEIGHT - long * width * DEPTH
        + cut.section(value) * listed.iter().map(length).sum::<f64>()
        - held_twice
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
                if (start..start + order.len()).contains(&number) {
                    (start + order[number - start]) as u32
                } else {
                    number as u32
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
/// otherwise than those of `reference`, laid on them in the listed order,
/// each laid face read by the edge it was laid on: one line for each face
/// that does.
fn met_otherwise(start: usize, order: &[usize], body: &Body, reference: &Body) -> Vec<String> {
    let listed_order: Vec<usize> = (0..order.len()).collect();
    let own = meetings(body, start, order);
    let theirs = meetings(reference, start, &listed_order);
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

/// Holds that the fillet at [`RADIUS`] on `listed`'s edges of `part`, in
/// every order the list can name them, is laid, each order's body held by
/// `holds` — given the order, the step's answer and what it is called — and
/// each the body the listed order lays, its faces meeting alike, each laid
/// face read by the edge it was laid on.
fn filleted_in_every_order(
    part: &Body,
    on: &str,
    listed: &[([usize; 2], &str)],
    holds: impl Fn(&[usize], &Laid, &str),
) {
    let start = part.faces_end();
    let named: Vec<EdgeName> = listed
        .iter()
        .map(|&([one, other], _)| edge_between(part, one, other))
        .collect();
    let called = |order: &[usize]| {
        let called: Vec<&str> = order.iter().map(|&edge| listed[edge].1).collect();
        format!(
            "the fillet at {RADIUS} on {}, on {on}",
            called.join(", then ")
        )
    };
    let laid: Vec<(Vec<usize>, Result<Laid, Declined>)> = every_order(named.len())
        .into_iter()
        .map(|order| {
            let names: Vec<EdgeName> = order.iter().map(|&edge| named[edge].clone()).collect();
            let laid = Cut::Fillet.laid_on(part, &names, RADIUS);
            (order, laid)
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
        "{} is laid whichever edge the list names first; declined:\n{}",
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
        let laid = laid.as_ref().expect("declines are held above");
        holds(order, laid, &what);
        let otherwise = met_otherwise(start, order, &laid.body, &reference.body);
        assert!(
            otherwise.is_empty(),
            "{what}: the body is the one the listed order lays, its faces meeting alike; not \
             so:\n{}",
            otherwise.join("\n")
        );
    }
}

/// Holds that the fillet at [`RADIUS`] on the round bottom's pair and the
/// floor edges at `ends`, listed in that order, named in `order` and laid as
/// `filleted`, takes the value whole and meets each end edge's fill in a
/// valley: the body exact, closed, uncrossed and drawn, holding `promised`;
/// the floor gone, the round bottom answering to both its edges' ranks and
/// each end edge's fill to its own, nothing laid at a corner; each fill drawn
/// on its own cylinder and on its own side of the valley, met there from its
/// axis, both drawn up to the valley's three points and sharing no point off
/// its planes, one vertex at each; each named end's wall kept above its fill,
/// a wall at an end not named keeping what stands above the round bottom's
/// half circle; the front and the back kept above the round bottom, their
/// uprights at both ends sharp; and every face meeting the faces it bounds,
/// one edge each.
fn meets_each_end_s_fill_in_a_valley(
    pocketed: &Pocketed,
    ends: &[End],
    order: &[usize],
    filleted: &Laid,
    promised: f64,
    what: &str,
) {
    let r = RADIUS;
    let part = &pocketed.part;
    let start = part.faces_end();
    let count = order.len();
    taken_as_asked(filleted.asked, filleted.taken, RADIUS, what);
    let body = &filleted.body;
    let bottom = ranks_of(start, order, &[0, 1]);
    let fills: Vec<usize> = (0..ends.len())
        .map(|at| ranks_of(start, order, &[PAIR.len() + at])[0])
        .collect();
    let [front_half, back_half] = pocketed.round_bottom();
    let area = front_half.area()
        + back_half.area()
        + ends.iter().map(|end| end.fill().area()).sum::<f64>();
    sound_drawn_within(body, promised, DRAWN * reach_of(part) * area, what);

    assert!(
        !body.has_face(POCKET_FLOOR),
        "{what}: the pocket's floor is gone, wholly under the round bottom, which touches its \
         plane only along the middle line"
    );
    let mut kept_and_laid = numbers(part);
    kept_and_laid.remove(&POCKET_FLOOR);
    kept_and_laid.extend(start..start + count);
    assert_eq!(
        numbers(body),
        kept_and_laid,
        "{what}: every face of the pocketed block but the pocket's floor, the round bottom \
         answering to {bottom:?}, each end edge's fill to its own rank, {fills:?}, and no face \
         laid at a corner"
    );
    assert_eq!(
        body.faces_end(),
        start + count,
        "{what}: the next step numbers its faces past the {count} the list gave its edges"
    );
    one_curved_face_answers_to(body, &bottom, &format!("{what}: the round bottom"));

    let bottom_drawn: Vec<DVec3> = body.triangles_of(bottom[0]).flatten().collect();
    for &point in &bottom_drawn {
        assert!(
            front_half.holds(point) || back_half.holds(point),
            "{what}: the round bottom is drawn through {point}, off its half cylinder of {r} \
             about the line {r} over the middle of the floor"
        );
        for end in ends {
            assert!(
                end.past_the_valley(point) <= PLACE,
                "{what}: the round bottom is drawn through {point}, past the valley where the \
                 fill of {} meets it",
                end.edge().1
            );
        }
    }
    for half in [&front_half, &back_half] {
        met_from_its_axis_where(
            body,
            bottom[0],
            half,
            |aim| {
                ends.iter()
                    .all(|end| end.past_the_valley(aim) < -CLEAR_OF_THE_VALLEY)
            },
            &format!("{what}: the round bottom, short of the valleys"),
        );
    }

    for (&end, &fill) in ends.iter().zip(&fills) {
        let which = format!("the fill of {}", end.edge().1);
        let round = end.fill();
        assert!(
            !body.is_flat(fill) && body.plane_of(fill).is_none(),
            "{what}: {which}, face {fill}, is curved, and offers no plane to draw on"
        );
        let fill_drawn: Vec<DVec3> = body.triangles_of(fill).flatten().collect();
        assert!(!fill_drawn.is_empty(), "{what}: {which} is drawn");
        for &point in &fill_drawn {
            assert!(
                round.holds(point),
                "{what}: {which} is drawn through {point}, off its quarter cylinder of {r} \
                 about the line {r} in from the wall and over the floor"
            );
            assert!(
                end.past_the_valley(point) >= -PLACE,
                "{what}: {which} is drawn through {point}, short of the valley where the round \
                 bottom meets it"
            );
        }
        met_from_its_axis_where(
            body,
            fill,
            &round,
            |aim| end.past_the_valley(aim) > CLEAR_OF_THE_VALLEY,
            &format!("{what}: {which}, past the valley"),
        );
        let [front_top, deepest, back_top] = end.valley();
        for (point, at) in [
            (front_top, "where the round bottom leaves the front wall"),
            (deepest, "where both touch the floor's plane"),
            (back_top, "where the round bottom leaves the back wall"),
        ] {
            for (drawn, of) in [(&bottom_drawn, "the round bottom"), (&fill_drawn, &*which)] {
                assert!(
                    drawn_through(drawn, point),
                    "{what}: {of} is drawn up to {at}, {point}, where a curve of the valley ends"
                );
            }
        }
        for &point in bottom_drawn
            .iter()
            .filter(|&&point| drawn_through(&fill_drawn, point))
        {
            assert!(
                end.past_the_valley(point).abs() < PLACE,
                "{what}: the round bottom and {which} meet at {point}, off the two planes of \
                 the valley"
            );
        }
        for (faces, at) in [
            (
                vec![&[POCKET_FRONT][..], &[end.wall()], &bottom, &[fill]],
                front_top,
            ),
            (vec![&bottom[..], &[fill]], deepest),
            (
                vec![&[POCKET_BACK][..], &[end.wall()], &bottom, &[fill]],
                back_top,
            ),
        ] {
            assert_eq!(
                vertices_among(body, &faces),
                1,
                "{what}: the faces {faces:?} meet at one vertex, {at}, an end of a curve of the \
                 valley"
            );
        }
        assert_eq!(
            sides_met_by(body, &[fill]),
            sides_answering(&[&[end.wall()], &bottom, &bottom]),
            "{what}: {which} meets its wall along its top and the round bottom along the two \
             curves of the valley, and no other face"
        );

        let wall = end.wall_at();
        faces_toward(
            body,
            end.wall(),
            DVec3::X * -end.outward(),
            &format!("{what}: the wall under {which}"),
        );
        corners_are(
            body,
            end.wall(),
            &[
                front_top,
                back_top,
                DVec3::new(wall, FRONT_WALL_AT + WIDTH, HEIGHT),
                DVec3::new(wall, FRONT_WALL_AT, HEIGHT),
            ],
            &format!(
                "{what}: the wall over {which}, kept from where the fill leaves it up to the top"
            ),
        );
        assert_eq!(
            sides_met_by(body, &[end.wall()]),
            sides_answering(&[&[TOP], &[POCKET_FRONT], &[POCKET_BACK], &[fill]]),
            "{what}: the wall over {which} meets the top, the front and the back along its \
             uprights, left sharp, and the fill, by one edge each"
        );
    }

    for end in [End::Right, End::Left]
        .into_iter()
        .filter(|end| !ends.contains(end) && pocketed.closed_at(*end))
    {
        let wall = end.wall_at();
        let half_circle_center = DVec3::new(wall, FRONT_WALL_AT + r, FLOOR_AT + r);
        corners_are_along_an_arc(
            body,
            end.wall(),
            &[
                DVec3::new(wall, FRONT_WALL_AT, FLOOR_AT + r),
                DVec3::new(wall, FRONT_WALL_AT + WIDTH, FLOOR_AT + r),
                DVec3::new(wall, FRONT_WALL_AT + WIDTH, HEIGHT),
                DVec3::new(wall, FRONT_WALL_AT, HEIGHT),
            ],
            |corner| on_the_circle_toward(corner, half_circle_center, r, &[-DVec3::Z]),
            &format!(
                "{what}: the wall at the end not named, kept above the half circle of {r} about \
                 {half_circle_center} the round bottom leaves on it"
            ),
        );
        assert_eq!(
            sides_met_by(body, &[end.wall()]),
            sides_answering(&[&[TOP], &[POCKET_FRONT], &[POCKET_BACK], &bottom]),
            "{what}: the wall at the end not named meets the top, the front and the back along \
             its uprights and the round bottom along the half circle, by one edge each"
        );
    }

    for (wall, across, which) in [
        (POCKET_FRONT, FRONT_WALL_AT, "the front wall"),
        (POCKET_BACK, FRONT_WALL_AT + WIDTH, "the back wall"),
    ] {
        corners_are(
            body,
            wall,
            &[
                DVec3::new(LEFT_WALL_AT, across, FLOOR_AT + r),
                DVec3::new(pocketed.reaches_to, across, FLOOR_AT + r),
                DVec3::new(pocketed.reaches_to, across, HEIGHT),
                DVec3::new(LEFT_WALL_AT, across, HEIGHT),
            ],
            &format!("{what}: {which}, kept from where the round bottom leaves it up to the top"),
        );
        assert_eq!(
            sides_met_by(body, &[wall]),
            sides_answering(&[&[TOP], &[pocketed.right_face], &[POCKET_LEFT], &bottom]),
            "{what}: {which} meets the top, the faces at both ends along its uprights and the \
             round bottom, by one edge each, and no end edge's fill"
        );
        for end in [End::Right, End::Left] {
            assert_eq!(
                edges_between(body, wall, pocketed.face_at(end)).len(),
                1,
                "{what}: the upright where {which} meets the face at the {end:?} end is one sharp \
                 edge, from the top down to where the round bottom leaves the walls"
            );
        }
    }

    let mut met_by_bottom: Vec<Vec<usize>> = vec![vec![POCKET_FRONT], vec![POCKET_BACK]];
    for end in [End::Right, End::Left] {
        match ends.iter().position(|named| *named == end) {
            Some(at) => met_by_bottom.extend([vec![fills[at]], vec![fills[at]]]),
            None => met_by_bottom.push(vec![pocketed.face_at(end)]),
        }
    }
    let met_by_bottom: Vec<&[usize]> = met_by_bottom.iter().map(Vec::as_slice).collect();
    assert_eq!(
        sides_met_by(body, &bottom),
        sides_answering(&met_by_bottom),
        "{what}: the round bottom meets the front and the back walls along its sides, and at each \
         end the face there, or the end edge's fill along the two curves of the valley, by one \
         edge each"
    );
    assert_eq!(
        sides_met_by(body, &[TOP]),
        sides_met_by(part, &[TOP]),
        "{what}: the block's top meets what it met before the step"
    );
}

/// Holds, for the floor edges at `ends` of `pocketed`, that the fillet at
/// [`RADIUS`] on the round bottom's pair and those edges is laid in every
/// order as [`meets_each_end_s_fill_in_a_valley`] holds it, after the
/// arithmetic has been held to give the volume the module's documentation
/// writes, and the booleans to give what the arithmetic does.
fn meets_each_end_s_fill_in_a_valley_whatever_the_order(
    pocketed: &Pocketed,
    ends: &[End],
    volume: f64,
    written: &str,
) {
    let listed: Listed = PAIR
        .iter()
        .copied()
        .chain(ends.iter().map(|end| end.edge()))
        .collect();
    let edges: Vec<[usize; 2]> = listed.iter().map(|&(edge, _)| edge).collect();
    let promised = pocketed.kept(Cut::Fillet, &edges);
    assert!(
        (promised - volume).abs() <= EXACT * volume,
        "the arithmetic gives {promised}, where the documentation writes {written}"
    );
    let by_booleans = valley_by_booleans(pocketed, ends);
    assert!(
        (by_booleans - promised).abs() <= EXACT * promised,
        "the round bottom's prism and the fill prisms of the floor edges at {ends:?}, joined to \
         {} by the booleans, leave {by_booleans}, where arithmetic promises {promised}",
        pocketed.called
    );
    filleted_in_every_order(
        &pocketed.part,
        pocketed.called,
        &listed,
        |order, filleted, what| {
            meets_each_end_s_fill_in_a_valley(pocketed, ends, order, filleted, promised, what);
        },
    );
}

#[test]
fn a_fillet_on_a_pocket_s_round_bottom_and_the_floor_edge_at_its_end_meets_that_edge_s_fill_in_a_valley_whatever_the_order()
 {
    meets_each_end_s_fill_in_a_valley_whatever_the_order(
        &Pocketed::narrow(),
        &[End::Right],
        5604.0 - 153.0 * PI / 2.0,
        "5604 − 153π/2",
    );
}

#[test]
fn a_fillet_on_a_pocket_s_round_bottom_and_the_floor_edges_at_both_its_ends_meets_each_end_s_fill_in_a_valley_whatever_the_order()
 {
    meets_each_end_s_fill_in_a_valley_whatever_the_order(
        &Pocketed::narrow(),
        &[End::Right, End::Left],
        5568.0 - 63.0 * PI,
        "5568 − 63π",
    );
}

#[test]
fn a_fillet_on_a_slot_s_round_bottom_and_the_floor_edge_at_its_closed_end_meets_that_edge_s_fill_in_a_valley_whatever_the_order()
 {
    meets_each_end_s_fill_in_a_valley_whatever_the_order(
        &Pocketed::open_slot(),
        &[End::Left],
        5514.0 - 99.0 * PI,
        "5514 − 99π",
    );
}

/// The round bottom's pair followed by `others`.
fn pair_and(others: &[([usize; 2], &'static str)]) -> Listed {
    let mut list = PAIR.to_vec();
    list.extend_from_slice(others);
    list
}

#[test]
fn the_lists_main_builds_on_a_pocket_6_or_6_5_wide_keep_their_bodies_in_every_order() {
    let [right, left] = [End::Right, End::Left].map(End::edge);
    let end_and_uprights = vec![right, FRONT_UPRIGHT, BACK_UPRIGHT];
    let everything = pair_and(&[right, FRONT_UPRIGHT, BACK_UPRIGHT]);
    let cases: Vec<(f64, Cut, Listed, f64)> = vec![
        (WIDTH, Cut::Fillet, pair_and(&[]), 5357.2567),
        (WIDTH, Cut::Chamfer, pair_and(&[]), 5460.0),
        (WIDTH, Cut::Chamfer, pair_and(&[right]), 5469.0),
        (WIDTH, Cut::Chamfer, pair_and(&[right, left]), 5478.0),
        (WIDTH, Cut::Fillet, vec![right], 5291.5885),
        (WIDTH, Cut::Chamfer, vec![right], 5307.0),
        (WIDTH, Cut::Chamfer, end_and_uprights.clone(), 5343.0),
        (WIDER, Cut::Fillet, pair_and(&[]), 5297.2567),
        (WIDER, Cut::Chamfer, pair_and(&[]), 5400.0),
        (WIDER, Cut::Fillet, pair_and(&[right]), 5304.6339),
        (WIDER, Cut::Chamfer, pair_and(&[right]), 5411.25),
        (WIDER, Cut::Fillet, pair_and(&[right, left]), 5312.0111),
        (WIDER, Cut::Chamfer, pair_and(&[right, left]), 5422.5),
        (WIDER, Cut::Fillet, vec![right], 5232.5542),
        (WIDER, Cut::Chamfer, vec![right], 5249.25),
        (WIDER, Cut::Fillet, end_and_uprights.clone(), 5250.5542),
        (WIDER, Cut::Chamfer, end_and_uprights, 5285.25),
        (WIDER, Cut::Fillet, everything.clone(), 5323.9480),
        (WIDER, Cut::Chamfer, everything, 5447.25),
    ];

    let mut not_kept = Vec::new();
    for (width, cut, listed, measured) in &cases {
        let part = pocket(*width);
        let edges: Vec<[usize; 2]> = listed.iter().map(|&(edge, _)| edge).collect();
        let promised = kept(*cut, *width, &edges);
        let called: Vec<&str> = listed.iter().map(|&(_, called)| called).collect();
        let list = format!(
            "the {} at {RADIUS} on {}, on the pocket {width} wide",
            cut.called(),
            called.join(", ")
        );
        assert!(
            (promised - measured).abs() <= AS_MEASURED,
            "{list}: the arithmetic gives {promised}, where the issue's table writes {measured}"
        );
        let named: Vec<EdgeName> = edges
            .iter()
            .map(|&[one, other]| edge_between(&part, one, other))
            .collect();
        for order in every_order(named.len()) {
            let names: Vec<EdgeName> = order.iter().map(|&edge| named[edge].clone()).collect();
            let what = format!("{list}, named in the order {order:?}");
            let laid = match cut.laid_on(&part, &names, RADIUS) {
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
        "every list main builds on a pocket 6 or 6.5 wide is built in every order, the value \
         taken whole, exact, closed and uncrossed, holding what arithmetic and main give; not \
         so:\n{}",
        not_kept.join("\n")
    );
}

/// The standing round bottom of the two uprights at the right end, whole:
/// two hollow quarter rounds about the vertical [`RADIUS`] in from the right
/// wall and over the middle of the floor, from the floor up to the top, from
/// facing the front wall's way to the right wall's, and from the back wall's
/// to the right wall's.
fn standing_round_bottom() -> [Hollow; 2] {
    [[DVec3::Y, -DVec3::X], [-DVec3::Y, -DVec3::X]].map(|facing| Hollow {
        start: DVec3::new(End::Right.axis_at(), FRONT_WALL_AT + RADIUS, FLOOR_AT),
        along: DVec3::Z,
        length: DEPTH,
        radius: RADIUS,
        facing,
    })
}

/// How far a point stands past the valley at the foot of the standing round
/// bottom, on the side of the floor edge's fill: how far it stands under the
/// crossing of the two axes, less how far across from the pocket's middle
/// line. Nought on the two planes the fills meet on, z − 17 = ±(y − 5).
fn past_the_foot_s_valley(point: DVec3) -> f64 {
    (FLOOR_AT + RADIUS - point.z) - (point.y - FRONT_WALL_AT - RADIUS).abs()
}

/// The three points of the valley at the foot of the standing round bottom:
/// where the standing round bottom leaves the front wall at the floor, where
/// both fills touch the right wall's plane, and where the standing round
/// bottom leaves the back wall at the floor.
fn foot_s_valley() -> [DVec3; 3] {
    let axis = End::Right.axis_at();
    [
        DVec3::new(axis, FRONT_WALL_AT, FLOOR_AT),
        DVec3::new(RIGHT_WALL_AT, FRONT_WALL_AT + RADIUS, FLOOR_AT + RADIUS),
        DVec3::new(axis, FRONT_WALL_AT + WIDTH, FLOOR_AT),
    ]
}

/// The prism the standing round bottom adds on [`narrow_pocket`], raised from
/// under the floor up to the block's top: what lies outside the half round
/// about the vertical [`RADIUS`] in from the right wall, out past that wall
/// and a unit past the front and back walls into the matter.
fn standing_round_bottom_tool(part: &Body) -> Body {
    let r = RADIUS;
    let axis = End::Right.axis_at();
    let [near, far] = [FRONT_WALL_AT, FRONT_WALL_AT + WIDTH];
    let beyond = RIGHT_WALL_AT + r;
    let corners = vec![
        DVec2::new(axis, near - 1.0),
        DVec2::new(beyond, near - 1.0),
        DVec2::new(beyond, far + 1.0),
        DVec2::new(axis, far + 1.0),
        DVec2::new(axis, far),
        DVec2::new(axis, near),
    ];
    let mut runs = vec![Run::Straight; corners.len()];
    runs[4] = Run::Round {
        center: DVec2::new(axis, near + r),
        turn: -PI,
    };
    part.tool_raised(
        &profile(Contour { corners, runs }),
        Frame {
            origin: DVec3::Z * (FLOOR_AT - r),
            ..ground()
        },
        DVec3::Z * (HEIGHT - FLOOR_AT + r),
    )
    .expect("the tool raises")
}

/// Holds that the fillet at [`RADIUS`] on the right end's floor edge and the
/// two uprights there, listed in that order, named in `order` and laid as
/// `filleted`, takes the value whole and lays the uprights' fills as one
/// standing round bottom, meeting the floor edge's fill at its foot in a
/// valley: the body exact, closed, uncrossed and drawn, holding `promised`;
/// the right wall gone, the standing round bottom answering to both uprights'
/// ranks and the floor edge's fill to its own, nothing laid at a corner; each
/// fill drawn on its own cylinder and on its own side of the valley, met there
/// from its axis, both drawn up to the valley's three points and sharing no
/// point off its planes, one vertex at each; the floor kept up to the line the
/// floor edge's fill leaves on it, the front and the back walls up to the
/// lines the standing round bottom leaves on them, the floor's front and back
/// edges sharp; the left wall and the top kept, the top up to the half circle
/// the standing round bottom leaves on it; and every face meeting the faces it
/// bounds, one edge each.
fn meets_the_standing_round_bottom_in_a_valley_at_its_foot(
    part: &Body,
    order: &[usize],
    filleted: &Laid,
    promised: f64,
    what: &str,
) {
    let r = RADIUS;
    let start = part.faces_end();
    let count = order.len();
    taken_as_asked(filleted.asked, filleted.taken, RADIUS, what);
    let body = &filleted.body;
    let foot = ranks_of(start, order, &[0])[0];
    let standing = ranks_of(start, order, &[1, 2]);
    let [front_half, back_half] = standing_round_bottom();
    let fill = End::Right.fill();
    sound_drawn_within(
        body,
        promised,
        DRAWN * reach_of(part) * (front_half.area() + back_half.area() + fill.area()),
        what,
    );

    assert!(
        !body.has_face(POCKET_RIGHT),
        "{what}: the right wall is gone, wholly behind the standing round bottom, which touches \
         its plane only along the vertical through its middle"
    );
    let mut kept_and_laid = numbers(part);
    kept_and_laid.remove(&POCKET_RIGHT);
    kept_and_laid.extend(start..start + count);
    assert_eq!(
        numbers(body),
        kept_and_laid,
        "{what}: every face of the pocketed block but its right wall, the standing round bottom \
         answering to {standing:?}, the floor edge's fill to {foot}, and no face laid at a corner"
    );
    assert_eq!(
        body.faces_end(),
        start + count,
        "{what}: the next step numbers its faces past the {count} the list gave its edges"
    );
    one_curved_face_answers_to(
        body,
        &standing,
        &format!("{what}: the standing round bottom"),
    );
    assert!(
        !body.is_flat(foot) && body.plane_of(foot).is_none(),
        "{what}: the floor edge's fill, face {foot}, is curved, and offers no plane to draw on"
    );

    let standing_drawn: Vec<DVec3> = body.triangles_of(standing[0]).flatten().collect();
    for &point in &standing_drawn {
        assert!(
            front_half.holds(point) || back_half.holds(point),
            "{what}: the standing round bottom is drawn through {point}, off its half cylinder of \
             {r} about the vertical {r} in from the right wall and over the middle of the floor"
        );
        assert!(
            past_the_foot_s_valley(point) <= PLACE,
            "{what}: the standing round bottom is drawn through {point}, past the valley where \
             the floor edge's fill meets it"
        );
    }
    for half in [&front_half, &back_half] {
        met_from_its_axis_where(
            body,
            standing[0],
            half,
            |aim| past_the_foot_s_valley(aim) < -CLEAR_OF_THE_VALLEY,
            &format!("{what}: the standing round bottom, above the valley"),
        );
    }
    let foot_drawn: Vec<DVec3> = body.triangles_of(foot).flatten().collect();
    assert!(
        !foot_drawn.is_empty(),
        "{what}: the floor edge's fill is drawn"
    );
    for &point in &foot_drawn {
        assert!(
            fill.holds(point),
            "{what}: the floor edge's fill is drawn through {point}, off its quarter cylinder of \
             {r} about the line {r} in from the right wall and over the floor"
        );
        assert!(
            past_the_foot_s_valley(point) >= -PLACE,
            "{what}: the floor edge's fill is drawn through {point}, above the valley where the \
             standing round bottom meets it"
        );
    }
    met_from_its_axis_where(
        body,
        foot,
        &fill,
        |aim| past_the_foot_s_valley(aim) > CLEAR_OF_THE_VALLEY,
        &format!("{what}: the floor edge's fill, under the valley"),
    );

    let [front_foot, deepest, back_foot] = foot_s_valley();
    for (point, at) in [
        (
            front_foot,
            "where the standing round bottom leaves the front wall at the floor",
        ),
        (deepest, "where both touch the right wall's plane"),
        (
            back_foot,
            "where the standing round bottom leaves the back wall at the floor",
        ),
    ] {
        for (drawn, of) in [
            (&standing_drawn, "the standing round bottom"),
            (&foot_drawn, "the floor edge's fill"),
        ] {
            assert!(
                drawn_through(drawn, point),
                "{what}: {of} is drawn up to {at}, {point}, where a curve of the valley ends"
            );
        }
    }
    for &point in standing_drawn
        .iter()
        .filter(|&&point| drawn_through(&foot_drawn, point))
    {
        assert!(
            past_the_foot_s_valley(point).abs() < PLACE,
            "{what}: the standing round bottom and the floor edge's fill meet at {point}, off the \
             two planes z − 17 = ±(y − 5)"
        );
    }
    for (faces, at) in [
        (
            vec![&[POCKET_FLOOR][..], &[POCKET_FRONT], &standing, &[foot]],
            front_foot,
        ),
        (vec![&standing[..], &[foot]], deepest),
        (
            vec![&[POCKET_FLOOR][..], &[POCKET_BACK], &standing, &[foot]],
            back_foot,
        ),
    ] {
        assert_eq!(
            vertices_among(body, &faces),
            1,
            "{what}: the faces {faces:?} meet at one vertex, {at}, an end of a curve of the valley"
        );
    }
    assert_eq!(
        sides_met_by(body, &[foot]),
        sides_answering(&[&[POCKET_FLOOR], &standing, &standing]),
        "{what}: the floor edge's fill meets the floor along its foot and the standing round \
         bottom along the two curves of the valley, and no other face"
    );
    assert_eq!(
        sides_met_by(body, &standing),
        sides_answering(&[&[TOP], &[POCKET_FRONT], &[POCKET_BACK], &[foot], &[foot]]),
        "{what}: the standing round bottom meets the top along the half circle, the front and \
         the back walls along its sides, and the floor edge's fill along the two curves of the \
         valley, by one edge each"
    );

    let axis = End::Right.axis_at();
    let [near, far] = [FRONT_WALL_AT, FRONT_WALL_AT + WIDTH];
    faces_toward(body, POCKET_FLOOR, DVec3::Z, &format!("{what}: the floor"));
    corners_are(
        body,
        POCKET_FLOOR,
        &[
            DVec3::new(LEFT_WALL_AT, near, FLOOR_AT),
            front_foot,
            back_foot,
            DVec3::new(LEFT_WALL_AT, far, FLOOR_AT),
        ],
        &format!("{what}: the floor, kept up to where the floor edge's fill leaves it"),
    );
    assert_eq!(
        sides_met_by(body, &[POCKET_FLOOR]),
        sides_answering(&[&[POCKET_FRONT], &[POCKET_BACK], &[POCKET_LEFT], &[foot]]),
        "{what}: the floor meets the front, the back and the left walls along its edges, left \
         sharp, and the floor edge's fill, by one edge each"
    );
    for (wall, across, which) in [
        (POCKET_FRONT, near, "the front wall"),
        (POCKET_BACK, far, "the back wall"),
    ] {
        corners_are(
            body,
            wall,
            &[
                DVec3::new(LEFT_WALL_AT, across, FLOOR_AT),
                DVec3::new(axis, across, FLOOR_AT),
                DVec3::new(axis, across, HEIGHT),
                DVec3::new(LEFT_WALL_AT, across, HEIGHT),
            ],
            &format!(
                "{what}: {which}, kept up to where the standing round bottom leaves it, from the \
                 floor to the top"
            ),
        );
        assert_eq!(
            sides_met_by(body, &[wall]),
            sides_answering(&[&[TOP], &[POCKET_FLOOR], &[POCKET_LEFT], &standing]),
            "{what}: {which} meets the top, the floor along its edge, left sharp, the left wall \
             along its upright and the standing round bottom, by one edge each, and not the floor \
             edge's fill"
        );
    }
    corners_are(
        body,
        POCKET_LEFT,
        &[
            DVec3::new(LEFT_WALL_AT, near, FLOOR_AT),
            DVec3::new(LEFT_WALL_AT, far, FLOOR_AT),
            DVec3::new(LEFT_WALL_AT, far, HEIGHT),
            DVec3::new(LEFT_WALL_AT, near, HEIGHT),
        ],
        &format!("{what}: the left wall, whole"),
    );
    assert_eq!(
        sides_met_by(body, &[POCKET_LEFT]),
        sides_answering(&[&[TOP], &[POCKET_FLOOR], &[POCKET_FRONT], &[POCKET_BACK]]),
        "{what}: the left wall meets what it met before the step, by one edge each"
    );
    let rim_center = DVec3::new(axis, near + r, HEIGHT);
    corners_are_along_an_arc(
        body,
        TOP,
        &[
            DVec3::new(0.0, 0.0, HEIGHT),
            DVec3::new(LENGTH, 0.0, HEIGHT),
            DVec3::new(LENGTH, BREADTH, HEIGHT),
            DVec3::new(0.0, BREADTH, HEIGHT),
            DVec3::new(LEFT_WALL_AT, near, HEIGHT),
            DVec3::new(axis, near, HEIGHT),
            DVec3::new(axis, far, HEIGHT),
            DVec3::new(LEFT_WALL_AT, far, HEIGHT),
        ],
        |corner| on_the_circle_toward(corner, rim_center, r, &[DVec3::X]),
        &format!(
            "{what}: the top, its pocket's rim taken round the half circle of {r} about \
             {rim_center} the standing round bottom leaves on it"
        ),
    );
    let mut met_by_top = sides_met_by(part, &[TOP]);
    met_by_top.retain(|side| *side != [POCKET_RIGHT as u32]);
    met_by_top.push(standing.iter().map(|&number| number as u32).collect());
    met_by_top.sort();
    assert_eq!(
        sides_met_by(body, &[TOP]),
        met_by_top,
        "{what}: the top meets what it met before the step, the standing round bottom where the \
         right wall was"
    );
}

#[test]
fn a_fillet_on_the_floor_edge_and_both_uprights_at_a_pocket_s_end_meets_their_standing_round_bottom_in_a_valley_at_its_foot_whatever_the_order()
 {
    let part = narrow_pocket();
    let listed: Listed = vec![End::Right.edge(), FRONT_UPRIGHT, BACK_UPRIGHT];
    let edges: Vec<[usize; 2]> = listed.iter().map(|&(edge, _)| edge).collect();
    let promised = kept(Cut::Fillet, WIDTH, &edges);
    let written = 5352.0 - 27.0 * PI / 2.0;
    assert!(
        (promised - written).abs() <= EXACT * written,
        "the arithmetic gives {promised}, where the documentation writes 5352 − 27π/2"
    );
    let by_booleans = part
        .union(&standing_round_bottom_tool(&part))
        .expect("the standing round bottom's prism joins the pocket")
        .union(&End::Right.tool(&part))
        .expect("the floor edge's fill prism joins the body")
        .volume();
    assert!(
        (by_booleans - promised).abs() <= EXACT * promised,
        "the standing round bottom's prism and the floor edge's fill prism, joined to the pocket \
         by the booleans, leave {by_booleans}, where arithmetic promises {promised}"
    );
    filleted_in_every_order(
        &part,
        "the pocket 6 wide",
        &listed,
        |order, filleted, what| {
            meets_the_standing_round_bottom_in_a_valley_at_its_foot(
                &part, order, filleted, promised, what,
            );
        },
    );
}

#[test]
fn a_fillet_and_a_chamfer_on_the_round_bottom_the_floor_edge_at_its_end_and_both_uprights_there_stay_declined_as_unsupported_whatever_the_order()
 {
    let part = narrow_pocket();
    let everything = pair_and(&[End::Right.edge(), FRONT_UPRIGHT, BACK_UPRIGHT]);

    let mut not_declined = Vec::new();
    for cut in [Cut::Fillet, Cut::Chamfer] {
        let named: Vec<EdgeName> = everything
            .iter()
            .map(|&([one, other], _)| edge_between(&part, one, other))
            .collect();
        for order in every_order(named.len()) {
            let names: Vec<EdgeName> = order.iter().map(|&edge| named[edge].clone()).collect();
            let called: Vec<&str> = order.iter().map(|&edge| everything[edge].1).collect();
            let what = format!(
                "the {} at {RADIUS} on {}, on the pocket {WIDTH} wide",
                cut.called(),
                called.join(", then ")
            );
            match cut.laid_on(&part, &names, RADIUS) {
                Err(Declined::Unsupported) => {}
                Err(declined) => not_declined.push(format!("{what}: declined, {declined:?}")),
                Ok(_) => not_declined.push(format!("{what}: a body")),
            }
        }
    }
    assert!(
        not_declined.is_empty(),
        "the round bottom, the floor edge at its end and both uprights there, two round bottoms \
         meeting, are #919's: each cut is a broken step and its reason, Unsupported, whatever the \
         order; not so:\n{}",
        not_declined.join("\n")
    );
}

/// The way out of [`slanted_pocket`] through its slanted end, square to it,
/// seen from above.
fn slant_outward() -> DVec3 {
    DVec3::new(3.0, -1.0, 0.0) / 10f64.sqrt()
}

/// The way the slanted end runs, from the front wall to the back.
fn slant_along() -> DVec3 {
    DVec3::new(1.0, 3.0, 0.0) / 10f64.sqrt()
}

/// The point of the axis of the slanted end edge's fill level with `y`
/// across the pocket: [`RADIUS`] in from the slanted wall, which stands
/// `√10` further along X at that level, and over the floor.
fn slanted_axis_at(y: f64) -> DVec3 {
    DVec3::new(
        RIGHT_WALL_AT + (y - FRONT_WALL_AT) / 3.0 - 10f64.sqrt(),
        y,
        FLOOR_AT + RADIUS,
    )
}

/// The slanted end edge's fill, its axis from level with `from` across the
/// pocket to level with `to`, from facing the floor's way to facing the
/// slanted wall's.
fn slanted_fill(from: f64, to: f64) -> Hollow {
    Hollow {
        start: slanted_axis_at(from),
        along: slant_along(),
        length: (to - from) / slant_along().y,
        radius: RADIUS,
        facing: [DVec3::Z, -slant_outward()],
    }
}

/// Where the slanted end edge's fill's axis crosses the round bottom's,
/// over the middle of the floor.
fn slanted_crossing() -> DVec3 {
    slanted_axis_at(FRONT_WALL_AT + RADIUS)
}

/// How far a point stands past the valley at the slanted end, on the side of
/// that end edge's fill: how far it stands out past the crossing of the two
/// axes square to the slanted wall, less how far across from the pocket's
/// middle line. Nought on the two upright planes through the crossing that
/// halve the angles between the two axes, where the two cylinders meet.
fn past_the_slanted_valley(point: DVec3) -> f64 {
    let crossing = slanted_crossing();
    (point - crossing).dot(slant_outward()) - (point.y - crossing.y).abs()
}

/// The valley's three points at the slanted end: where the round bottom
/// leaves the front wall there, where both fills touch the floor's plane,
/// and where the round bottom leaves the back wall.
fn slanted_valley() -> [DVec3; 3] {
    let top = FLOOR_AT + RADIUS;
    [
        DVec3::new(RIGHT_WALL_AT, FRONT_WALL_AT, top),
        slanted_crossing() - DVec3::Z * RADIUS,
        slanted_end_at_the_back().extend(top),
    ]
}

/// The prism the slanted end edge's fill adds on [`slanted_pocket`], raised
/// along the slanted end from the matter past the back wall to the matter
/// past the front wall: what lies under the quarter round, from its axis out
/// a unit past the slanted wall and down under the floor.
fn slanted_end_tool(part: &Body) -> Body {
    let r = RADIUS;
    let [from, to] = [1.5, FRONT_WALL_AT + WIDTH + 1.0];
    let corners = vec![
        DVec2::new(0.0, FLOOR_AT - r),
        DVec2::new(r + 1.0, FLOOR_AT - r),
        DVec2::new(r + 1.0, FLOOR_AT + r),
        DVec2::new(r, FLOOR_AT + r),
        DVec2::new(0.0, FLOOR_AT),
    ];
    let mut runs = vec![Run::Straight; corners.len()];
    runs[3] = Run::Round {
        center: DVec2::new(0.0, FLOOR_AT + r),
        turn: -PI / 2.0,
    };
    let start = slanted_axis_at(to);
    part.tool_raised(
        &profile(Contour { corners, runs }),
        Frame {
            origin: DVec3::new(start.x, start.y, 0.0),
            u: slant_outward(),
            v: DVec3::Z,
        },
        -slant_along() * ((to - from) / slant_along().y),
    )
    .expect("the tool raises")
}

/// What [`slanted_pocket`] holds once the round bottom and the slanted end's
/// floor edge are filleted at [`RADIUS`], as the module's documentation
/// derives it: the pocket; the round bottom's section along its mean length,
/// 21, where the pocket's middle line meets the slanted wall; the end edge's
/// section along its `2√10`, trimmed flush on the front and back walls; less,
/// at each of the two corners the end edge makes with them, the square
/// corner's share stretched by one over the sine of their angle, `3/√10` at
/// both.
fn kept_with_the_slanted_end() -> f64 {
    let r = RADIUS;
    let strip = r * r * (1.0 - PI / 4.0);
    let sine = 3.0 / 10f64.sqrt();
    let area = (POCKET_LENGTH + slanted_end_at_the_back().x - LEFT_WALL_AT) / 2.0 * WIDTH;
    let mean_length = RIGHT_WALL_AT + RADIUS / 3.0 - LEFT_WALL_AT;
    let end_length = 2.0 * 10f64.sqrt();
    LENGTH * BREADTH * HEIGHT - area * DEPTH + 2.0 * strip * mean_length + strip * end_length
        - 2.0 * r.powi(3) * (5.0 / 3.0 - PI / 2.0) / sine
}

#[test]
fn a_fillet_on_a_pocket_s_round_bottom_and_the_floor_edge_of_its_slanted_end_meets_that_edge_s_fill_in_a_valley_whatever_the_order()
 {
    let r = RADIUS;
    let part = slanted_pocket();
    let slanted_end = POCKET_RIGHT;
    let listed = pair_and(&[(FLOOR_RIGHT, "the floor's edge on the slanted end")]);
    let promised = kept_with_the_slanted_end();
    let root = 10f64.sqrt();
    let written = 5622.0 - 94.5 * PI - 12.0 * root + 4.5 * root * PI;
    assert!(
        (promised - written).abs() <= EXACT * written,
        "the arithmetic gives {promised}, where the documentation writes \
         5622 − 189π/2 − 12√10 + 9√10π/2"
    );
    let by_booleans = part
        .union(&round_bottom_tool(&part, RIGHT_WALL_AT + r))
        .expect("the round bottom's prism joins the pocket")
        .union(&slanted_end_tool(&part))
        .expect("the slanted end edge's fill prism joins the body")
        .volume();
    assert!(
        (by_booleans - promised).abs() <= EXACT * promised,
        "the round bottom's prism and the slanted end edge's fill prism, joined to the pocket by \
         the booleans, leave {by_booleans}, where arithmetic promises {promised}"
    );

    let start = part.faces_end();
    let [front_half, back_half] = round_bottom_along(slanted_end_at_the_back().x - LEFT_WALL_AT);
    let fill = slanted_fill(FRONT_WALL_AT - 1.0, FRONT_WALL_AT + WIDTH + 1.0);
    let across = slanted_fill(FRONT_WALL_AT, FRONT_WALL_AT + WIDTH);
    let area = front_half.area() + back_half.area() + across.area();
    filleted_in_every_order(
        &part,
        "the pocket 6 wide whose right end is slanted",
        &listed,
        |order, filleted, what| {
            taken_as_asked(filleted.asked, filleted.taken, RADIUS, what);
            let body = &filleted.body;
            let bottom = ranks_of(start, order, &[0, 1]);
            let end = ranks_of(start, order, &[2])[0];
            sound_drawn_within(body, promised, DRAWN * reach_of(&part) * area, what);
            assert!(
                !body.has_face(POCKET_FLOOR),
                "{what}: the pocket's floor is gone, wholly under the round bottom"
            );
            let mut kept_and_laid = numbers(&part);
            kept_and_laid.remove(&POCKET_FLOOR);
            kept_and_laid.extend(start..start + order.len());
            assert_eq!(
                numbers(body),
                kept_and_laid,
                "{what}: every face of the pocketed block but the pocket's floor, the round \
                 bottom answering to {bottom:?}, the slanted end edge's fill to {end}, and no \
                 face laid at a corner"
            );
            one_curved_face_answers_to(body, &bottom, &format!("{what}: the round bottom"));
            assert!(
                !body.is_flat(end) && body.plane_of(end).is_none(),
                "{what}: the slanted end edge's fill, face {end}, is curved"
            );

            let bottom_drawn: Vec<DVec3> = body.triangles_of(bottom[0]).flatten().collect();
            for &point in &bottom_drawn {
                assert!(
                    front_half.holds(point) || back_half.holds(point),
                    "{what}: the round bottom is drawn through {point}, off its half cylinder of \
                     {r} about the line {r} over the middle of the floor"
                );
                assert!(
                    past_the_slanted_valley(point) <= PLACE,
                    "{what}: the round bottom is drawn through {point}, past the valley where the \
                     slanted end edge's fill meets it"
                );
            }
            for half in [&front_half, &back_half] {
                met_from_its_axis_where(
                    body,
                    bottom[0],
                    half,
                    |aim| past_the_slanted_valley(aim) < -CLEAR_OF_THE_VALLEY,
                    &format!("{what}: the round bottom, short of the valley"),
                );
            }
            let end_drawn: Vec<DVec3> = body.triangles_of(end).flatten().collect();
            assert!(
                !end_drawn.is_empty(),
                "{what}: the slanted end edge's fill is drawn"
            );
            for &point in &end_drawn {
                assert!(
                    fill.holds(point),
                    "{what}: the slanted end edge's fill is drawn through {point}, off its quarter \
                     cylinder of {r} about the line {r} in from the slanted wall and over the floor"
                );
                assert!(
                    past_the_slanted_valley(point) >= -PLACE,
                    "{what}: the slanted end edge's fill is drawn through {point}, short of the \
                     valley where the round bottom meets it"
                );
            }
            met_from_its_axis_where(
                body,
                end,
                &across,
                |aim| past_the_slanted_valley(aim) > CLEAR_OF_THE_VALLEY,
                &format!("{what}: the slanted end edge's fill, past the valley"),
            );

            let [front_top, deepest, back_top] = slanted_valley();
            for point in [front_top, deepest, back_top] {
                for (drawn, of) in [
                    (&bottom_drawn, "the round bottom"),
                    (&end_drawn, "the slanted end edge's fill"),
                ] {
                    assert!(
                        drawn_through(drawn, point),
                        "{what}: {of} is drawn up to {point}, where a curve of the valley ends"
                    );
                }
            }
            for &point in bottom_drawn
                .iter()
                .filter(|&&point| drawn_through(&end_drawn, point))
            {
                assert!(
                    past_the_slanted_valley(point).abs() < PLACE,
                    "{what}: the round bottom and the slanted end edge's fill meet at {point}, off \
                     the two planes of the valley"
                );
            }
            for (faces, at) in [
                (
                    vec![&[POCKET_FRONT][..], &[slanted_end], &bottom, &[end]],
                    front_top,
                ),
                (vec![&bottom[..], &[end]], deepest),
                (
                    vec![&[POCKET_BACK][..], &[slanted_end], &bottom, &[end]],
                    back_top,
                ),
            ] {
                assert_eq!(
                    vertices_among(body, &faces),
                    1,
                    "{what}: the faces {faces:?} meet at one vertex, {at}, an end of a curve of \
                     the valley"
                );
            }
            assert_eq!(
                sides_met_by(body, &[end]),
                sides_answering(&[&[slanted_end], &bottom, &bottom]),
                "{what}: the slanted end edge's fill meets its wall along its top and the round \
                 bottom along the two curves of the valley, and no other face"
            );
            assert_eq!(
                sides_met_by(body, &bottom),
                sides_answering(&[
                    &[POCKET_FRONT],
                    &[POCKET_BACK],
                    &[POCKET_LEFT],
                    &[end],
                    &[end]
                ]),
                "{what}: the round bottom meets the front and the back walls along its sides, the \
                 left wall along its half circle, and the slanted end edge's fill along the two \
                 curves of the valley, by one edge each"
            );
            faces_toward(
                body,
                slanted_end,
                -slant_outward(),
                &format!("{what}: the slanted wall"),
            );
            corners_are(
                body,
                slanted_end,
                &[
                    front_top,
                    back_top,
                    DVec3::new(back_top.x, back_top.y, HEIGHT),
                    DVec3::new(front_top.x, front_top.y, HEIGHT),
                ],
                &format!(
                    "{what}: the slanted wall, kept from where its edge's fill leaves it up to the \
                     top"
                ),
            );
            assert_eq!(
                sides_met_by(body, &[slanted_end]),
                sides_answering(&[&[TOP], &[POCKET_FRONT], &[POCKET_BACK], &[end]]),
                "{what}: the slanted wall meets the top, the front and the back along its \
                 uprights, left sharp, and its edge's fill, by one edge each"
            );
        },
    );
}

/// A list the step refuses: what it is called, the part, the cut and its
/// value, the edges of its slope or its round bottom, and the edge at their
/// end.
type Refused<'a> = (&'a str, &'a Body, Cut, f64, Vec<EdgeName>, Vec<EdgeName>);

#[test]
fn a_cut_and_a_fill_that_meet_where_a_slope_or_a_round_bottom_ends_are_declined_as_unsupported_whatever_the_order()
 {
    let stair = stair();
    let walled = stair_between_two_walls();
    let slot = open_slot();
    let pocket = narrow_pocket();
    let named = |part: &Body, edges: &[[usize; 2]]| -> Vec<EdgeName> {
        edges
            .iter()
            .map(|&[one, other]| edge_touching(part, one, other))
            .collect()
    };
    let [tread, nose, riser] = [7, 8, 2];
    let slope = [[tread, nose], [riser, tread]];
    let cases: Vec<Refused> = vec![
        (
            "the stair's slope, its tread's nose and foot chamfered at 5, with the tread's convex \
             edge on the stair's right side",
            &stair,
            Cut::Chamfer,
            5.0,
            named(&stair, &slope),
            named(&stair, &[[tread, BLOCK_RIGHT]]),
        ),
        (
            "the slope of the stair between two walls, with the tread's concave edge on the right \
             wall",
            &walled,
            Cut::Chamfer,
            5.0,
            named(&walled, &slope),
            named(&walled, &[[tread, 17]]),
        ),
        (
            "the round bottom of the slot open at its right end, filleted at 3, with the floor's \
             convex edge on the block's right face",
            &slot,
            Cut::Fillet,
            RADIUS,
            named(&slot, &[FLOOR_FRONT, FLOOR_BACK]),
            named(&slot, &[[POCKET_FLOOR, BLOCK_RIGHT]]),
        ),
        (
            "the standing round bottom of the pocket's two uprights at its right end, filleted at \
             3, with the convex rim edge on top of that wall",
            &pocket,
            Cut::Fillet,
            RADIUS,
            named(&pocket, &[FRONT_RIGHT, BACK_RIGHT]),
            named(&pocket, &[[TOP, POCKET_RIGHT]]),
        ),
    ];

    let mut not_declined = Vec::new();
    for (called, part, cut, value, pair, others) in &cases {
        for (alone, which) in [(pair, "the pair"), (others, "the edge at its end")] {
            if let Err(declined) = cut.laid_on(part, alone, *value) {
                panic!("{called}: {which} alone is laid; declined, {declined:?}");
            }
        }
        let list: Vec<EdgeName> = pair.iter().chain(others).cloned().collect();
        for order in every_order(list.len()) {
            let names: Vec<EdgeName> = order.iter().map(|&edge| list[edge].clone()).collect();
            let what = format!(
                "{called}, the {} named in the order {order:?}",
                cut.called()
            );
            match cut.laid_on(part, &names, *value) {
                Err(Declined::Unsupported) => {}
                Err(declined) => not_declined.push(format!("{what}: declined, {declined:?}")),
                Ok(_) => not_declined.push(format!("{what}: a body")),
            }
        }
    }
    assert!(
        not_declined.is_empty(),
        "a cut and a fill meeting where a slope or a round bottom ends do not commute, each laid \
         against the other: each list is a broken step and its reason, Unsupported, whatever the \
         order, its parts laid alone; not so:\n{}",
        not_declined.join("\n")
    );
}
