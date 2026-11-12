//! Two rounds of one step on several edges can meet otherwise than across a
//! side face their edges share: through an end face, where one edge ends on a
//! side face of the other, or across a shared face, where their edges are not
//! parallel. There the step's value is reduced to the largest at which they do
//! not meet, as #698 reduces it for two edges facing each other across a side
//! face, rather than declined. An operation of the kernel on [`Body`], as the
//! step on several edges of #698 is.
//!
//! Closes #792.
//! - The value is reduced to the largest at which those rounds do not meet
//!   either — through an end face: on a block, the top's front edge ends on
//!   the right wall, which is a side face of the right wall's back vertical
//!   edge; asked more than half the wall's width, a hair more or well more,
//!   the two rounds would meet near that corner, and the step takes half, or
//!   a few hairs under, and lays both, each as alone at that value, neither
//!   meeting the other —
//!   `a_fillet_on_two_edges_whose_rounds_would_meet_on_the_wall_one_ends_on_asked_more_than_half_its_width_takes_the_largest_radius_short_of_meeting`,
//!   `a_chamfer_on_two_edges_whose_flats_would_meet_on_the_wall_one_ends_on_asked_more_than_half_its_width_takes_the_largest_distance_short_of_meeting`;
//!   and, beyond the issue's block, the same where the wall leans from square
//!   to the edge that ends on it, the back upright's faces meeting at a sharp
//!   angle: the step takes the value at which the round square to the top's
//!   front edge, reaching further along the leaning wall than across it, and
//!   the back upright's set-back fill the wall's length —
//!   `a_fillet_and_a_chamfer_on_two_edges_whose_rounds_would_meet_on_a_leaning_wall_one_ends_on_take_the_largest_value_short_of_meeting`;
//!   and where one edge's faces meet inwards: the top edge of an L's arm
//!   end, which ends on the arm's back, and the L's inner upright, of which
//!   the arm's back is a side and which is filled in, take half the arm's
//!   back, the round and the fill touching at its top —
//!   `a_fillet_and_a_chamfer_on_the_top_edge_of_an_l_s_arm_end_and_its_inner_upright_asked_more_than_half_the_arm_s_back_take_the_largest_value_short_of_meeting`
//!   — a closed pocket's floor edge and upright, where the two would meet the
//!   same way, are declined each alone, at any value, and cannot hold it;
//!   and on all these pairs the value is the same whichever edge the step
//!   names first —
//!   `a_fillet_and_a_chamfer_on_two_edges_whose_rounds_would_meet_take_the_same_value_whichever_edge_the_step_names_first`
//! - The value is reduced to the largest at which those rounds do not meet
//!   either — across a shared face, two edges that are not parallel:
//!   alternate edges of a hexagonal prism's top, once their set-backs cross;
//!   asked past it, a hair past or well past, the step takes the value at
//!   which the two set-backs touch on the top edge of the wall between them,
//!   a quarter of the side times the square root of three, or a few hairs
//!   under, and lays both, each as alone at that value, neither meeting the
//!   other —
//!   `a_fillet_on_two_alternate_top_edges_of_a_hexagonal_prism_asked_past_where_their_set_backs_cross_takes_the_largest_radius_short_of_meeting`,
//!   `a_chamfer_on_two_alternate_top_edges_of_a_hexagonal_prism_asked_past_where_their_set_backs_cross_takes_the_largest_distance_short_of_meeting`;
//!   whichever of the two the step names first —
//!   `a_fillet_and_a_chamfer_on_two_edges_whose_rounds_would_meet_take_the_same_value_whichever_edge_the_step_names_first`;
//!   and where the edge they would cross on is in the list too, laid after
//!   both, #792 reduced the value to where they touched, as the last
//!   criterion had it; #822 made the value read the layout every edge of the
//!   list leaves, whatever its order, so the middle edge's cut takes away
//!   where they would cross and the value asked is taken —
//!   `a_fillet_and_a_chamfer_on_three_top_edges_in_a_row_of_a_hexagonal_prism_named_with_the_middle_one_last_asked_past_where_the_outer_two_would_cross_take_the_value_asked`,
//!   `a_fillet_and_a_chamfer_on_the_six_top_edges_of_a_hexagonal_prism_in_ring_order_asked_past_where_alternate_edges_would_cross_take_the_value_asked`;
//!   and where that edge is laid before the later of the two but its own
//!   set-back closes between its two mitres first — a trapezoid's short top
//!   edge between its two sides, which converge below it — the step takes
//!   the value at which it closes, the three set-backs meeting in one point,
//!   past which the two sides' would cross on the top beyond its strip,
//!   wherever the trapezoid stands, on the origin or off it along both axes —
//!   `a_fillet_and_a_chamfer_on_a_trapezoid_s_short_top_edge_and_its_two_sides_named_between_them_asked_past_where_its_set_back_closes_take_the_value_at_which_it_closes`;
//!   and with the three walls leaning in under their top edges, where a
//!   fillet sets each edge back along the top by its radius over the tangent
//!   of half the edge's angle, at the radius whose set-back closes the short
//!   edge's, and a chamfer still at the distance that does —
//!   `a_fillet_and_a_chamfer_on_a_drafted_trapezoid_s_short_top_edge_and_its_two_sides_named_between_them_asked_past_where_its_set_back_closes_take_the_value_whose_set_back_closes_it`
//! - Every step #698 lays — held by the suite of #698 in
//!   `a_chamfer_and_a_fillet_take_several_edges_in_one_step.rs`, left as it
//!   was but for the two tests that held these meetings declined. The one on
//!   the wall one edge ends on keeps the step it lays short of the meeting,
//!   renamed
//!   `a_fillet_on_two_edges_whose_rounds_would_meet_on_the_wall_one_ends_on_is_laid_as_asked_short_of_it`,
//!   its decline past it now the reduction above; the one on the hexagonal
//!   prism, where they would meet, goes, its check that each edge alone
//!   takes the value asked carried into the second criterion's tests. Beyond
//!   the issue's wording, two steps #698 lays, the second through #699's
//!   mitres, that no test of its suite reaches and that the reduction could
//!   shrink: two edges whose rounds both end on one wall, side by side across
//!   its width but one at its top and the other at its floor, never meeting
//!   on it, take the value asked —
//!   `a_fillet_and_a_chamfer_on_two_edges_whose_rounds_end_on_one_wall_without_meeting_on_it_take_the_value_asked`;
//!   and three top edges in a row of the hexagonal prism, asked past where
//!   the outer two's set-backs would cross on the middle edge, take the value
//!   asked, the middle edge cut away too between them: each two neighbours
//!   mitred, the outer two meeting nowhere —
//!   `a_fillet_and_a_chamfer_on_three_top_edges_in_a_row_of_a_hexagonal_prism_asked_past_where_the_outer_two_would_cross_take_the_value_asked_the_middle_one_named_before_the_last`.
//!   #698 laid them so with the middle edge named before the last of the
//!   outer two. #792 had the value follow the order the step lays its edges
//!   in, as the owner decided: two set-backs that would meet on an edge of
//!   the list laid before the later of the two did not meet, that edge's cut
//!   taking away the point where they would before the later one was laid;
//!   with the middle edge named last, #698 declined the three `Unsupported`,
//!   and #792 reduced them to where the outer two touch on it, a quarter of
//!   the side times the square root of three, as it did the six top edges
//!   named in ring order, the last named laid after both its neighbours.
//!   #822 took that back: the step still lays its edges in the order named,
//!   but bounds the value by the layout they all leave, a listed edge
//!   between two crossing set-backs bounding them where its own set-back
//!   closes, whatever its rank. The three with the middle edge named last
//!   and the six in ring order take the value asked, each face where it lies
//!   as with the middle edge named before the last —
//!   `a_fillet_and_a_chamfer_on_three_top_edges_in_a_row_of_a_hexagonal_prism_named_with_the_middle_one_last_asked_past_where_the_outer_two_would_cross_take_the_value_asked`,
//!   `a_fillet_and_a_chamfer_on_the_six_top_edges_of_a_hexagonal_prism_in_ring_order_asked_past_where_alternate_edges_would_cross_take_the_value_asked`.
//!   And two steps #698 lays that the reading of
//!   an edge of the list between two others could shrink take the value
//!   asked: where the middle edge's wall overhangs, so steeply that its
//!   fillet's set-back runs ahead of where the outer two's would cross, the
//!   same body whatever the order, the middle edge named last too since
//!   #822 —
//!   `a_fillet_and_a_chamfer_on_three_top_edges_in_a_row_of_a_hexagonal_prism_whose_middle_wall_overhangs_take_the_value_asked_in_every_order_its_set_back_outrunning_where_the_outer_two_would_cross`;
//!   and where the outer two reach the middle edge through its two faces,
//!   one ending on its top face and the other on its wall, a chain of three
//!   on the block and on the trapezoid —
//!   `a_fillet_and_a_chamfer_on_three_edges_in_a_chain_whose_outer_two_reach_the_middle_one_through_its_two_faces_take_the_value_asked`
//!
//! #829 narrowed what a set-back is read to reach: only where its blend runs
//! on its face, up to the plane of the face its edge ends on, or up to the
//! mitre where an edge of the list laid before the later of the two meets
//! it. Read along its whole line, a set-back reached points its blend never
//! does, and two steps it shrank take the value asked since. A standing
//! block's foot, filled, and an edge of the lower block's top beyond the wall
//! the foot ends on, cut, whose set-backs' lines would cross on that top past
//! the foot's end, were declined, the foot's line reaching the far edge at
//! nearly no value; they are laid each as alone, in either order, the fill
//! ending flush on the wall —
//! `a_chamfer_and_a_fillet_on_a_standing_block_s_foot_and_an_edge_of_the_lower_block_s_top_beyond_its_end_take_the_value_asked_the_foot_s_set_back_running_no_further_than_the_wall_it_ends_on`.
//! And the eight feet round a standing block whose four uprights an earlier
//! step chamfered, an octagon of fills meeting in mitres at its eight
//! corners, took `tan(22.5°)` of the value, for lines crossing far out on the
//! lower block's top; they take it whole, the volume the octagon's
//! arithmetic gives, whatever the order — the fills are the frustum between
//! the octagon and its offset out, less the standing block —
//! `a_chamfer_on_the_eight_feet_round_a_standing_block_s_chamfered_uprights_takes_the_value_asked_their_set_backs_running_no_further_than_their_mitres`.
//! Every reduction above is the same with either reading: where two
//! set-backs meet, each runs there. Where a third edge of the list runs into
//! a mitre, the value does not hang on which of the mitre's two edges is
//! laid last: the earlier one runs on past the face it stops on into the
//! mitre once the later is laid, whichever order the list names them in. An
//! L whose corner block is cut under its inner corner by a slanted wall, its
//! two top edges there cut beside the inner upright left out and the
//! upright where the slant meets the front cut with them, takes the value
//! at which the arm's edge's set-back, past the stem's plane, meets the
//! upright's on the slant's top edge — 3.75 chamfered, 5 filleted — in all
//! six orders —
//! `a_fillet_and_a_chamfer_on_the_two_top_edges_at_an_l_s_inner_corner_and_an_upright_their_mitre_runs_into_take_the_value_at_which_they_meet_whatever_the_order`.
//!
//! Closes #822.
//!
//! - A step on several edges takes the same value whichever order its edges
//!   are named in: the largest at which no two set-backs meet once every
//!   edge of the list is laid. The three in a row and the ring of the
//!   hexagon, asked 4.9, take 4.9 in every order — the three in a row in all
//!   six orders,
//!   `a_fillet_and_a_chamfer_on_three_top_edges_in_a_row_of_a_hexagonal_prism_asked_past_where_the_outer_two_would_cross_take_the_value_asked_in_every_order`;
//!   the six as given, reversed and in two orders shuffled,
//!   `a_fillet_and_a_chamfer_on_the_six_top_edges_of_a_hexagonal_prism_asked_past_where_alternate_edges_would_cross_take_the_value_asked_whatever_the_order`.
//!   Read as: chamfered and filleted, the value taken whole, a sound body
//!   holding the arithmetic's volume — one for every order — and the top set
//!   back by the value from every edge of the list
//! - Every step #698, #699 and #792 lay — no test: held by their suites, but
//!   for #792's two tests that held the value reduced with the middle edge
//!   named last and on the ring; each now holds the value asked, every face
//!   where it lies, under a name that says so
//! - Found on the way, not in the issue's wording: three top edges in a row
//!   whose middle wall overhangs, its set-back running ahead of where the
//!   outer two's would cross, take the value asked in all six orders, the
//!   middle edge named last as well as second, the same body in each: the
//!   outer two stop on the middle edge's faces wherever it is ranked —
//!   `a_fillet_and_a_chamfer_on_three_top_edges_in_a_row_of_a_hexagonal_prism_whose_middle_wall_overhangs_take_the_value_asked_in_every_order_its_set_back_outrunning_where_the_outer_two_would_cross`
//!
//! The volumes are #792's: the three in a row's as
//! `a_fillet_and_a_chamfer_on_three_top_edges_in_a_row_of_a_hexagonal_prism_asked_past_where_the_outer_two_would_cross_take_the_value_asked_the_middle_one_named_before_the_last`
//! derives them, the six's as the ring's test does, at the value asked
//! rather than where alternate edges touch.
//!
//! Closes #823.
//!
//! - Where the set-backs of two edges of one step would meet on a curved
//!   edge of the part that is an arc of a circle, the value is reduced to the
//!   largest at which they do not, as #792 reduces it on a straight one. The
//!   arched plate asked 3 takes 2 —
//!   `a_fillet_and_a_chamfer_on_the_two_sides_of_an_arched_plate_asked_past_where_their_set_backs_meet_on_its_arc_take_the_value_at_which_they_touch`.
//!   Read as: the issue's plate, its two sides leaning 30° in from the
//!   upright at 7 from the arc's centre, and the one whose sides lean 10° at
//!   3; both orders, chamfer and fillet; asked past the touch — 3 and 3.158,
//!   1.4635 and 1.91 — the value taken where the two set-backs touch on the
//!   arc, 2 and 1.2635, or at most a few hairs under it; asked short of it,
//!   1.9 and 1.2, the value taken whole; the body sound, drawn within the
//!   tolerance over the arc's wall and the rounds, and holding what the same
//!   step laid directly at the value taken holds
//! - The same where the curved edge is an arc of a conic — no test: not read
//!   yet, its depths along the set-backs being no trigonometric form of one
//!   parameter as a circle's are; tracked as #882
//! - Found on the way, not in the issue's wording: two sides at different
//!   leans and distances, whose set-backs cross off the arc's middle, and the
//!   plate turned so that its arc stands astride its circle's half turn,
//!   take the value at which they touch, both orders, chamfer and fillet —
//!   `a_fillet_and_a_chamfer_on_two_sides_of_an_arched_plate_at_different_leans_take_the_value_at_which_their_set_backs_touch_off_the_arc_s_middle`
//! - Found on the way, not in the issue's wording: a bore through the arched
//!   plate's top, its rim a whole circle bounding the face both sides cut and
//!   read over a whole turn, leaves the value where the two touch on the arc:
//!   inside the face, the rim is reached only after the face's outer edge is
//!   —
//!   `a_fillet_and_a_chamfer_on_the_two_sides_of_an_arched_plate_bored_through_its_top_take_the_value_at_which_their_set_backs_touch_on_the_arc_the_bore_s_rim_read_whole`
//! - Every step #698, #699 and #792 lay — no test: held by their suites, left
//!   as they are
//!
//! Each side's set-back on the top is its line moved in by the value, the
//! fillet's radius over the tangent of half a square, the value again. The
//! two lines `n·p = c − v`, mirrors of each other about the arc's middle,
//! cross on that middle at `(c − v)/sin a` up it, `a` the side's lean, and
//! touch the arc of radius `R` where that is `R`: at `v = c − R·sin a`, 2 for
//! the issue's plate and 1.2635 for the other. Where the two sides differ, on
//! `n₁·p = c₁` and `n₂·p = c₂` with outward normals `n₁` and `n₂`, each
//! depth along the arc is `cᵢ − R·nᵢ·e(θ)`, and the two touch where they are
//! equal, `(n₂ − n₁)·e(θ) = (c₂ − c₁)/R`, at the depth there; the test holds
//! that against the least of the larger depth stepped along the arc.
//!
//! Closes #824.
//!
//! - A round ending on a wall that leans over it is drawn within the
//!   drawing's tolerance of its exact surface, as it is against an upright
//!   wall: the hexagon's front top edge filleted 2, its next wall leaning
//!   over 20° or 40°, encloses the exact body's volume within what the
//!   tolerance allows its round —
//!   `a_fillet_on_a_hexagonal_prism_s_front_top_edge_is_drawn_within_the_drawing_s_tolerance_whether_its_next_wall_stands_upright_or_leans_over_it`.
//!   Read as: upright, 20° and 40°, the hexagon's wall three cut back to
//!   lean in under its top edge as #792's overhanging hexagon is; the body
//!   exact, its triangles closed and uncrossed, the value taken whole, and
//!   the matter they enclose within the drawing's tolerance at the part's
//!   reach over the round's area along its edge, a side long — the 0.063 the
//!   issue names, against the exact body's own volume
//! - Every fillet drawn within the tolerance today — no test: held by every
//!   test that draws one, in this file and the others, left as they are
//!
//! Measured when the issue was taken: the round encloses 0.030 less than the
//! exact body against the upright wall, 0.145 at 20° and 0.272 at 40°. Every
//! corner of the round's triangles lies on its cylinder, but the round is
//! drawn in 52 triangles against the upright wall and in 22 and 16 against
//! the leaning one, the worst sagging 0.0152 off the cylinder where upright
//! it sags 0.0017: its turn is sampled coarser once one end is the leaning
//! wall's trim.
//!
//! Closes #877.
//!
//! - The step takes one value, or declines, whatever the order, where an
//!   outer wall of three edges in a row overhangs —
//!   `a_fillet_and_a_chamfer_on_three_top_edges_in_a_row_whose_outer_edge_alone_reaches_past_the_middle_one_or_whose_outer_wall_overhangs_take_in_every_order_the_least_any_of_the_three_takes_alone`.
//!   Read as decided when the issue was taken, and narrowed once a reading
//!   that lifted the cap split orders that had agreed: one value whatever
//!   the order — each edge's own limit applies whatever its rank, so every
//!   order takes the least any of the three takes alone; never a wrong body.
//!   Beyond the issue's wording, the edges need not overhang: a hexagon
//!   whose third side turns a twelfth of a turn from the second, which alone
//!   sets back along the top past the second's far end, took one value
//!   where the second was named before it and its own alone where named
//!   after, upright walls too. Rows tried: that hexagon 12 tall, its third
//!   wall leaning in 40 degrees, asked 5 and 6; its first wall leaning in 40
//!   degrees, 12 and 20 tall, asked 6, where reading the third edge's cap as
//!   if the second were laid split orders that had agreed; with its third
//!   side 30 long, upright and leaning in 20 degrees, asked 6; and the
//!   regular hexagon 10 tall, its front wall leaning in 50 degrees, asked 8,
//!   past where two chamfers' flats laid whole would meet in a line leaving
//!   through the leaning wall before the middle one; chamfer and fillet, in
//!   all six orders: the least value any of the three takes alone, the same
//!   matter, a sound body, the same body in each. And at
//!   the tie where the second wall closes at the depth the third edge alone
//!   is bounded by — that hexagon's first wall leaning in 60 degrees, a
//!   chamfer asked 6, the third edge alone declining —
//!   `a_chamfer_on_three_top_edges_in_a_row_whose_first_wall_leans_in_60_degrees_takes_one_value_or_declines_in_every_order_where_the_third_edge_alone_declines`
//! - #792's and #822's tests, the hexagon's overhanging middle wall
//!   included — no test: held by their own tests in this file, left as they
//!   are; the overhanging middle wall's comparison of the bodies across
//!   orders moved into `the_same_body_in_every_order`, asserting what it did

use std::collections::BTreeSet;
use std::f64::consts::{FRAC_1_SQRT_2, PI, TAU};

use cao_solid::profile::{Contour, Frame, Profile, Run};
use cao_solid::soundness::{closed, enclosed, uncrossed};
use cao_solid::{Body, Declined, EdgeName, Loop};
use glam::{DVec2, DVec3};

/// How far a corner may stand from where the arithmetic puts it, on a plane
/// or on a round: each is placed exactly, so what is left is rounding.
const PLACE: f64 = 1e-9;

/// How near the arithmetic an exact volume comes, as a fraction of it.
const EXACT: f64 = 1e-9;

/// How far the triangles may stand from a round, as a share of the reach the
/// body carries: the drawing's tolerance.
const DRAWN: f64 = 2e-4;

/// The numbers a raised block's faces answer to: its floor nought, its top
/// one, and its sides from the front round to the left, two to five.
const FLOOR: usize = 0;
const TOP: usize = 1;
const FRONT: usize = 2;
const RIGHT: usize = 3;
const BACK: usize = 4;
const LEFT: usize = 5;

/// The block the end face is met on: 30 along X, 10 across, 20 high.
const LENGTH: f64 = 30.0;
const BREADTH: f64 = 10.0;
const HEIGHT: f64 = 20.0;
const BLOCK_VOLUME: f64 = LENGTH * BREADTH * HEIGHT;

/// The radius, or the distance, at which the round of the block's top front
/// edge and that of its right wall's back edge touch, at the top of the right
/// wall: the first reaches its value back along that wall from the front, the
/// second its value forward from the back, and the wall is [`BREADTH`] wide.
const TOUCHING: f64 = BREADTH / 2.0;

/// What is asked of both edges, well more than [`TOUCHING`], which either
/// edge alone gives.
const MORE_THAN_HALF: f64 = 8.0;

/// One of the kernel's hairs on the block [`LENGTH`] long: twenty
/// tolerances, a tolerance a billionth of the reach.
const ONE_HAIR: f64 = 20.0 * 1e-9 * LENGTH;

/// How many of the part's hairs under the value at which two rounds touch a
/// reduced step may stop: the kernel keeps no strip between two set-backs
/// shorter than four hairs, which leaves the value two hairs under; ten leave
/// the step room to keep a wider strip, and stand a thousand times under the
/// drawing's tolerance.
const SHORT_BY: f64 = 10.0;

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

/// The part with an outline drawn on the ground raised `height` and joined
/// to it as `cao_part` joins a step: its floor numbered first, past the
/// part's numbers, then its top, then its walls, one for each run of the
/// outline.
fn raised_with(part: &Body, outline: Contour, height: f64) -> Body {
    let tool = part
        .tool_raised(&profile(outline), ground(), DVec3::Z * height)
        .expect("the tool raises");
    part.union(&tool).expect("the join is built")
}

/// The block [`LENGTH`] by [`BREADTH`] by [`HEIGHT`] standing on the
/// ground's origin, its floor numbered nought, its top one, and its sides two
/// to five from the front round to the left.
fn block() -> Body {
    raised_with(
        &Body::default(),
        Contour::rectangle(DVec2::ZERO, DVec2::new(LENGTH, BREADTH)),
        HEIGHT,
    )
}

/// The name of the one edge between the faces numbered `first` and `second`,
/// as the body hands it to whoever points at that edge.
fn edge_between(body: &Body, first: usize, second: usize) -> EdgeName {
    let named: Vec<EdgeName> = body
        .edge_names()
        .into_iter()
        .filter(|name| lies_between(name, first, second))
        .collect();
    assert_eq!(
        named.len(),
        1,
        "one edge between {first} and {second}: {named:?}"
    );
    named[0].clone()
}

/// Whether a name is that of an edge between the faces numbered `first` and
/// `second`, whichever it lists first.
fn lies_between(name: &EdgeName, first: usize, second: usize) -> bool {
    let sides = [vec![first as u32], vec![second as u32]];
    let [one, other] = sides.clone();
    name.between == sides || name.between == [other, one]
}

/// The two edges whose rounds would meet through an end face, in the order
/// the step names them: the top's front edge, which ends on the right wall,
/// and the right wall's back edge, upright, of which that wall is a side.
fn top_front_and_right_back(block: &Body) -> [EdgeName; 2] {
    [
        edge_between(block, TOP, FRONT),
        edge_between(block, RIGHT, BACK),
    ]
}

/// Every number some face of the body answers to: a step's, below where the
/// steps' numbers end, and a piece's, past them.
fn numbers(body: &Body) -> BTreeSet<usize> {
    (0..body.faces_end())
        .chain(body.pieces())
        .filter(|&number| body.has_face(number))
        .collect()
}

/// Holds that a step kept every face of the part it was given and laid one
/// face more for each edge, numbered `laid`, the next step numbering its own
/// past them all.
fn kept_every_face_and_laid(part: &Body, body: &Body, laid: &[usize], what: &str) {
    let mut kept_and_laid = numbers(part);
    kept_and_laid.extend(laid);
    assert_eq!(
        numbers(body),
        kept_and_laid,
        "{what}: every face of the part, and one face laid for each edge, {laid:?}"
    );
    let last = laid.iter().max().expect("a face laid");
    assert_eq!(
        body.faces_end(),
        last + 1,
        "{what}: the next step numbers its faces past all those laid"
    );
}

/// Holds that the faces numbered `first` and `second` share no edge: what one
/// edge's step laid does not meet what the other's did.
fn do_not_meet(body: &Body, [first, second]: [usize; 2], what: &str) {
    let between: Vec<EdgeName> = body
        .edge_names()
        .into_iter()
        .filter(|name| lies_between(name, first, second))
        .collect();
    assert!(
        between.is_empty(),
        "{what}: faces {first} and {second} meet, by {between:?}"
    );
}

/// Holds that a step reports the value it was asked, and took it whole.
fn taken_as_asked(asked: f64, taken: f64, value: f64, what: &str) {
    assert!(
        (asked - value).abs() < PLACE && (taken - value).abs() < PLACE,
        "{what}: asked {asked} and took {taken}, where {value} fits"
    );
}

/// Holds that a step reports the value it was asked, and took no more than
/// the value at which two of its rounds touch, nor less than [`SHORT_BY`] of
/// the part's hairs, `hair` long, under it.
fn reduced_to_touching(
    asked: f64,
    taken: f64,
    [expected_asked, touching]: [f64; 2],
    hair: f64,
    what: &str,
) {
    assert!(
        (asked - expected_asked).abs() < PLACE,
        "{what}: reports {asked} asked, where {expected_asked} was"
    );
    let least = touching - SHORT_BY * hair;
    assert!(
        taken <= touching + PLACE && taken >= least,
        "{what}: took {taken}, where the largest value at which the two do not meet is \
         {touching}, or at most {SHORT_BY} hairs under it, {least}"
    );
}

/// Holds that each of the edges `names` alone takes `asked` whole, which is
/// what makes the limit the two edges' together.
fn alone_each_takes(
    step: impl Fn(&EdgeName) -> Result<f64, Declined>,
    names: &[EdgeName],
    asked: f64,
    what: &str,
) {
    for name in names {
        let taken = step(name)
            .unwrap_or_else(|declined| panic!("{what}, one edge alone: declined, {declined:?}"));
        assert!(
            (taken - asked).abs() < PLACE,
            "{what}: one edge alone takes {taken}, where it was asked {asked}"
        );
    }
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

/// Holds that a face's corners are the points expected: each of them a
/// corner, and no corner anywhere else.
fn corners_are(body: &Body, face: usize, expected: &[DVec3], what: &str) {
    corners_are_or_on(body, face, expected, |_| false, what);
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

/// Holds that a body is exact and keeps the rules every solid must: its
/// surface closed, no face through another, the matter it holds what
/// arithmetic promised, and the matter its triangles enclose within `drawn`
/// of it — the drawing's tolerance over its rounds, where it has any.
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

/// The largest coordinate a body reaches, on either side of the origin.
fn reach_of(body: &Body) -> f64 {
    let (low, high) = body.bounds().expect("a part has a box");
    low.abs().max(high.abs()).max_element()
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

/// Holds that the face numbered `face` is the round expected: every corner
/// of its triangles on it, and every ray aimed square at its axis from
/// outside, a tenth to nine tenths of the way along it, meeting it first,
/// where it faces away from the axis along the ray.
fn is_the_round(body: &Body, face: usize, round: &Round, what: &str) {
    is_the_round_running_on(body, face, [round, round], what);
}

/// Holds that the face numbered `face` is a round running on past both ends
/// of its edge: every corner of its triangles on `running_on`, which reaches
/// past them, and every ray aimed at its axis over the edge, `over_the_edge`,
/// meeting it first, as [`is_the_round`] holds it of a round that does not
/// run on.
fn is_the_round_running_on(
    body: &Body,
    face: usize,
    [running_on, over_the_edge]: [&Round; 2],
    what: &str,
) {
    drawn_on(body, face, running_on, what);
    met_from_outside(body, face, over_the_edge, what);
}

/// Holds that every corner of the triangles of the face numbered `face` lies
/// on the round.
fn drawn_on(body: &Body, face: usize, round: &Round, what: &str) {
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
}

/// Holds that every ray aimed square at the round's axis from outside, a
/// tenth to nine tenths of the way along it, meets the face numbered `face`
/// first, where it faces away from the axis along the ray.
fn met_from_outside(body: &Body, face: usize, round: &Round, what: &str) {
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

/// The rounds a fillet of `radius` lays on the top's front edge, along the
/// whole length of the block, and on the right wall's back edge, up its
/// whole height: each its axis the radius in from both its faces.
fn top_front_and_right_back_rounds(radius: f64) -> [Round; 2] {
    [
        Round {
            start: DVec3::new(0.0, radius, HEIGHT - radius),
            along: DVec3::X,
            length: LENGTH,
            radius,
            facing: [DVec3::Z, -DVec3::Y],
        },
        Round {
            start: DVec3::new(LENGTH - radius, BREADTH - radius, 0.0),
            along: DVec3::Z,
            length: HEIGHT,
            radius,
            facing: [DVec3::X, DVec3::Y],
        },
    ]
}

/// What a step on the top's front edge and the right wall's back edge leaves
/// of the right wall, at `value`, and of the top: the right wall set back by
/// the value from the back and trimmed at its top front corner, the top set
/// back by it from the front and trimmed at its back right corner, a strip of
/// each left between the two cuts where they would otherwise meet. The
/// corners where the top front edge's cut trims the wall, and where the back
/// edge's trims the top, are the ends of what trims them.
fn right_wall_and_top(value: f64) -> [(usize, Vec<DVec3>); 2] {
    let x = LENGTH;
    [
        (
            RIGHT,
            vec![
                DVec3::new(x, 0.0, 0.0),
                DVec3::new(x, BREADTH - value, 0.0),
                DVec3::new(x, BREADTH - value, HEIGHT),
                DVec3::new(x, value, HEIGHT),
                DVec3::new(x, 0.0, HEIGHT - value),
            ],
        ),
        (
            TOP,
            vec![
                DVec3::new(0.0, value, HEIGHT),
                DVec3::new(x, value, HEIGHT),
                DVec3::new(x, BREADTH - value, HEIGHT),
                DVec3::new(x - value, BREADTH, HEIGHT),
                DVec3::new(0.0, BREADTH, HEIGHT),
            ],
        ),
    ]
}

#[test]
fn a_fillet_on_two_edges_whose_rounds_would_meet_on_the_wall_one_ends_on_asked_more_than_half_its_width_takes_the_largest_radius_short_of_meeting()
 {
    let block = block();
    let names = top_front_and_right_back(&block);
    let laid = [block.faces_end(), block.faces_end() + 1];

    for asked in [TOUCHING + ONE_HAIR, MORE_THAN_HALF] {
        alone_each_takes(
            |name| block.fillet(name, asked).map(|single| single.taken),
            &names,
            asked,
            &format!(
                "the fillet on the top's front edge or the right wall's back edge, asked {asked}"
            ),
        );

        let filleted = block
            .fillet_edges(&names, asked)
            .unwrap_or_else(|declined| panic!("asked {asked}: declined, {declined:?}"));

        let what = format!(
            "the fillet on the top's front edge and the right wall's back edge, asked {asked}"
        );
        reduced_to_touching(
            filleted.asked,
            filleted.taken,
            [asked, TOUCHING],
            ONE_HAIR,
            &what,
        );
        let radius = filleted.taken;
        let body = &filleted.body;
        let rounds = top_front_and_right_back_rounds(radius);
        for ((round, number), edge) in rounds
            .iter()
            .zip(laid)
            .zip(["top's front", "right wall's back"])
        {
            is_the_round(
                body,
                number,
                round,
                &format!("{what}: face {number}, the {edge} edge rounded off at {radius}"),
            );
        }
        do_not_meet(body, laid, &what);
        kept_every_face_and_laid(&block, body, &laid, &what);
        for (face, corners) in right_wall_and_top(radius) {
            corners_are_or_on(
                body,
                face,
                &corners,
                |corner| rounds.iter().any(|round| round.holds(corner)),
                &format!("{what}: face {face}, cut by both rounds and left between them"),
            );
        }
        let quarter = radius * radius * (1.0 - PI / 4.0);
        sound_drawn_within(
            body,
            BLOCK_VOLUME - quarter * (LENGTH + HEIGHT),
            DRAWN * reach_of(&block) * rounds.iter().map(Round::area).sum::<f64>(),
            &what,
        );
    }
}

/// The flats a chamfer of `distance` lays on the top's front edge, along the
/// whole length of the block, and on the right wall's back edge, up its
/// whole height: each set back by the distance along both its faces, its
/// corners where it meets the walls at its ends, and facing halfway between
/// its two faces.
fn top_front_and_right_back_flats(distance: f64) -> [([DVec3; 4], DVec3); 2] {
    let (x, y, z) = (LENGTH, BREADTH, HEIGHT);
    [
        (
            [
                DVec3::new(0.0, 0.0, z - distance),
                DVec3::new(x, 0.0, z - distance),
                DVec3::new(x, distance, z),
                DVec3::new(0.0, distance, z),
            ],
            DVec3::new(0.0, -FRAC_1_SQRT_2, FRAC_1_SQRT_2),
        ),
        (
            [
                DVec3::new(x, y - distance, 0.0),
                DVec3::new(x, y - distance, z),
                DVec3::new(x - distance, y, z),
                DVec3::new(x - distance, y, 0.0),
            ],
            DVec3::new(FRAC_1_SQRT_2, FRAC_1_SQRT_2, 0.0),
        ),
    ]
}

#[test]
fn a_chamfer_on_two_edges_whose_flats_would_meet_on_the_wall_one_ends_on_asked_more_than_half_its_width_takes_the_largest_distance_short_of_meeting()
 {
    let block = block();
    let names = top_front_and_right_back(&block);
    let laid = [block.faces_end(), block.faces_end() + 1];

    for asked in [TOUCHING + ONE_HAIR, MORE_THAN_HALF] {
        alone_each_takes(
            |name| block.chamfer(name, asked).map(|single| single.taken),
            &names,
            asked,
            &format!(
                "the chamfer on the top's front edge or the right wall's back edge, asked {asked}"
            ),
        );

        let chamfered = block
            .chamfer_edges(&names, asked)
            .unwrap_or_else(|declined| panic!("asked {asked}: declined, {declined:?}"));

        let what = format!(
            "the chamfer on the top's front edge and the right wall's back edge, asked {asked}"
        );
        reduced_to_touching(
            chamfered.asked,
            chamfered.taken,
            [asked, TOUCHING],
            ONE_HAIR,
            &what,
        );
        let distance = chamfered.taken;
        let body = &chamfered.body;
        for (((corners, facing), number), edge) in top_front_and_right_back_flats(distance)
            .iter()
            .zip(laid)
            .zip(["top's front", "right wall's back"])
        {
            let which = format!("{what}: face {number}, the {edge} edge cut off at {distance}");
            faces_toward(body, number, *facing, &which);
            corners_are(body, number, corners, &which);
        }
        do_not_meet(body, laid, &what);
        kept_every_face_and_laid(&block, body, &laid, &what);
        for (face, corners) in right_wall_and_top(distance) {
            corners_are(
                body,
                face,
                &corners,
                &format!("{what}: face {face}, cut by both flats and left between them"),
            );
        }
        let promised = BLOCK_VOLUME - distance * distance / 2.0 * (LENGTH + HEIGHT);
        sound_drawn_within(body, promised, EXACT * promised, &what);
    }
}

#[test]
fn a_fillet_and_a_chamfer_on_two_edges_whose_rounds_end_on_one_wall_without_meeting_on_it_take_the_value_asked()
 {
    let block = block();
    let names = [
        edge_between(&block, TOP, FRONT),
        edge_between(&block, FLOOR, BACK),
    ];
    let laid = [block.faces_end(), block.faces_end() + 1];
    let value = MORE_THAN_HALF;

    let filleted = block
        .fillet_edges(&names, value)
        .unwrap_or_else(|declined| panic!("the fillet: declined, {declined:?}"));
    let chamfered = block
        .chamfer_edges(&names, value)
        .unwrap_or_else(|declined| panic!("the chamfer: declined, {declined:?}"));

    let what = format!(
        "the fillet on the top's front edge and the floor's back edge, asked {value}: their \
         rounds' ends overlap across the right wall's width, one at its top and the other at its \
         floor"
    );
    taken_as_asked(filleted.asked, filleted.taken, value, &what);
    let body = &filleted.body;
    let rounds = [
        Round {
            start: DVec3::new(0.0, value, HEIGHT - value),
            along: DVec3::X,
            length: LENGTH,
            radius: value,
            facing: [DVec3::Z, -DVec3::Y],
        },
        Round {
            start: DVec3::new(0.0, BREADTH - value, value),
            along: DVec3::X,
            length: LENGTH,
            radius: value,
            facing: [-DVec3::Z, DVec3::Y],
        },
    ];
    for (round, number) in rounds.iter().zip(laid) {
        is_the_round(
            body,
            number,
            round,
            &format!("{what}: face {number}, its edge rounded off as asked"),
        );
    }
    do_not_meet(body, laid, &what);
    kept_every_face_and_laid(&block, body, &laid, &what);
    sound_drawn_within(
        body,
        BLOCK_VOLUME - 2.0 * value * value * (1.0 - PI / 4.0) * LENGTH,
        DRAWN * reach_of(&block) * rounds.iter().map(Round::area).sum::<f64>(),
        &what,
    );

    let what = format!(
        "the chamfer on the top's front edge and the floor's back edge, asked {value}: their \
         flats' ends overlap across the right wall's width, one at its top and the other at its \
         floor"
    );
    taken_as_asked(chamfered.asked, chamfered.taken, value, &what);
    let body = &chamfered.body;
    do_not_meet(body, laid, &what);
    kept_every_face_and_laid(&block, body, &laid, &what);
    let promised = BLOCK_VOLUME - value * value * LENGTH;
    sound_drawn_within(body, promised, EXACT * promised, &what);
}

/// The side of the hexagonal prism, and its height.
const HEXAGON_SIDE: f64 = 10.0;
const HEXAGON_HEIGHT: f64 = 5.0;

/// The numbers the hexagonal prism's walls answer to, its top being one:
/// its front two, along the bottom of the hexagon; four, two walls round,
/// whose top edge turns a third of a turn from the front's; and three,
/// between them, where both their rounds run on past their ends.
const HEXAGON_FRONT: usize = 2;
const HEXAGON_BETWEEN: usize = 3;
const HEXAGON_ACROSS: usize = 4;

/// One of the kernel's hairs on the hexagonal prism, which reaches
/// [`HEXAGON_SIDE`] from the origin: twenty tolerances, a tolerance a
/// billionth of the reach.
const HEXAGON_HAIR: f64 = 20.0 * 1e-9 * HEXAGON_SIDE;

/// What is asked of the two top edges past where their set-backs cross,
/// which either edge alone gives: under the prism's height.
const HEXAGON_ASKED: f64 = 4.9;

/// Half the hexagon's breadth across its flats: from its middle to its
/// front.
fn hexagon_half() -> f64 {
    HEXAGON_SIDE / 2.0 * 3.0f64.sqrt()
}

/// The radius, or the distance, at which the front's top edge and wall
/// four's set-backs on the top touch, on the top edge of wall three between
/// them. Each set-back is a line on the top the value in from its edge,
/// crossing wall three's top edge, which turns a sixth of a turn from both,
/// the value over the sine of that sixth from the corner it starts at:
/// two over the square root of three of it. The two touch where those
/// reaches fill the edge, a side long, at a quarter of the side times the
/// square root of three — half the distance across the equilateral triangle
/// the two edges' lines close on wall three's.
fn hexagon_touching() -> f64 {
    HEXAGON_SIDE * 3.0f64.sqrt() / 4.0
}

/// A regular hexagon of side [`HEXAGON_SIDE`] about the origin, its front the
/// side along the bottom, raised [`HEXAGON_HEIGHT`]: its walls numbered from
/// the front round, two to seven.
fn hexagonal_prism() -> Body {
    raised_with(
        &Body::default(),
        Contour::straight(hexagon_corners().to_vec()),
        HEXAGON_HEIGHT,
    )
}

/// The hexagon's corners, from the front's left end round: wall two runs
/// from the first to the second, and each wall after it from where the one
/// before ends.
fn hexagon_corners() -> [DVec2; 6] {
    let (half, side) = (hexagon_half(), HEXAGON_SIDE);
    [
        DVec2::new(-side / 2.0, -half),
        DVec2::new(side / 2.0, -half),
        DVec2::new(side, 0.0),
        DVec2::new(side / 2.0, half),
        DVec2::new(-side / 2.0, half),
        DVec2::new(-side, 0.0),
    ]
}

/// The two edges whose set-backs cross on the top, in the order the step
/// names them: the front's top edge, and wall four's.
fn alternate_top_edges(prism: &Body) -> [EdgeName; 2] {
    [
        edge_between(prism, TOP, HEXAGON_FRONT),
        edge_between(prism, TOP, HEXAGON_ACROSS),
    ]
}

/// A way turned about the vertical by `turn`.
fn turned(way: DVec3, turn: f64) -> DVec3 {
    DVec3::new(
        way.x * turn.cos() - way.y * turn.sin(),
        way.x * turn.sin() + way.y * turn.cos(),
        way.z,
    )
}

/// The round a fillet of `radius` lays on the hexagon's front top edge,
/// turned about the vertical by `turn` — a third of a turn gives wall four's.
/// The first runs on past both its ends to the walls beside it, to where its
/// top meets them, the radius over the square root of three past each; the
/// second is the same round over the edge alone, between its ends.
fn hexagon_round(radius: f64, turn: f64) -> [Round; 2] {
    [
        HEXAGON_SIDE / 2.0 + radius / 3.0f64.sqrt(),
        HEXAGON_SIDE / 2.0,
    ]
    .map(|past| hexagon_round_to(radius, turn, past))
}

/// The round of [`hexagon_round`], `past` either side of the middle of its
/// edge.
fn hexagon_round_to(radius: f64, turn: f64, past: f64) -> Round {
    let half = hexagon_half();
    Round {
        start: turned(
            DVec3::new(-past, radius - half, HEXAGON_HEIGHT - radius),
            turn,
        ),
        along: turned(DVec3::X, turn),
        length: 2.0 * past,
        radius,
        facing: [DVec3::Z, turned(-DVec3::Y, turn)],
    }
}

/// The matter a step of `value` takes off along one top edge of the
/// hexagonal prism, of section `section` and of first moment `moment` about
/// the edge's wall: the section along the edge, a side long, and on past
/// both its ends, where the walls beside it lean out, the length growing by
/// two over the square root of three of the way in from the wall.
fn taken_off_one_top_edge(section: f64, moment: f64) -> f64 {
    HEXAGON_SIDE * section + 2.0 / 3.0f64.sqrt() * moment
}

/// What is left of the hexagon's top once both edges' set-backs at `value`
/// cut it: the front's set-back running from wall seven to wall three, wall
/// four's from wall three to wall five, a stretch of wall three's top edge
/// left between them, and the three far corners as they were.
fn hexagon_top(value: f64) -> Vec<DVec3> {
    let (half, reach) = (hexagon_half(), value / 3.0f64.sqrt());
    let side = HEXAGON_SIDE;
    [
        DVec2::new(-side / 2.0 - reach, value - half),
        DVec2::new(side / 2.0 + reach, value - half),
        DVec2::new(side - reach, -value),
        DVec2::new(side / 2.0 - 2.0 * reach, half),
        DVec2::new(-side / 2.0, half),
        DVec2::new(-side, 0.0),
    ]
    .map(|corner| corner.extend(HEXAGON_HEIGHT))
    .to_vec()
}

/// What is left of wall three once both edges' steps at `value` end on it:
/// its foot whole, its two upright edges cut down by the value, and its top
/// edge cut back from either end by two over the square root of three of
/// the value, a stretch left between. Between each cut upright and the top
/// edge, the end of what the step laid trims the wall.
fn hexagon_between(value: f64) -> Vec<DVec3> {
    let (half, reach) = (hexagon_half(), value / 3.0f64.sqrt());
    let (side, height) = (HEXAGON_SIDE, HEXAGON_HEIGHT);
    vec![
        DVec3::new(side / 2.0, -half, 0.0),
        DVec3::new(side, 0.0, 0.0),
        DVec3::new(side, 0.0, height - value),
        DVec3::new(side - reach, -value, height),
        DVec3::new(side / 2.0 + reach, value - half, height),
        DVec3::new(side / 2.0, -half, height - value),
    ]
}

/// The hexagonal prism's matter: six equilateral triangles of side
/// [`HEXAGON_SIDE`], raised [`HEXAGON_HEIGHT`].
fn hexagon_volume() -> f64 {
    3.0 * 3.0f64.sqrt() / 2.0 * HEXAGON_SIDE * HEXAGON_SIDE * HEXAGON_HEIGHT
}

#[test]
fn a_fillet_on_two_alternate_top_edges_of_a_hexagonal_prism_asked_past_where_their_set_backs_cross_takes_the_largest_radius_short_of_meeting()
 {
    let prism = hexagonal_prism();
    let names = alternate_top_edges(&prism);
    let laid = [prism.faces_end(), prism.faces_end() + 1];
    let touching = hexagon_touching();

    for asked in [touching + HEXAGON_HAIR, HEXAGON_ASKED] {
        alone_each_takes(
            |name| prism.fillet(name, asked).map(|single| single.taken),
            &names,
            asked,
            &format!("the fillet on the hexagon's front top edge or wall four's, asked {asked}"),
        );

        let filleted = prism
            .fillet_edges(&names, asked)
            .unwrap_or_else(|declined| panic!("asked {asked}: declined, {declined:?}"));

        let what = format!(
            "the fillet on the hexagon's front top edge and the top edge two walls round, asked \
             {asked}"
        );
        reduced_to_touching(
            filleted.asked,
            filleted.taken,
            [asked, touching],
            HEXAGON_HAIR,
            &what,
        );
        let radius = filleted.taken;
        let body = &filleted.body;
        let rounds = [0.0, 2.0 * PI / 3.0].map(|turn| hexagon_round(radius, turn));
        for ([running_on, over_the_edge], number) in rounds.iter().zip(laid) {
            is_the_round_running_on(
                body,
                number,
                [running_on, over_the_edge],
                &format!(
                    "{what}: face {number}, its edge rounded off at {radius}, running on past \
                     both its ends"
                ),
            );
        }
        do_not_meet(body, laid, &what);
        kept_every_face_and_laid(&prism, body, &laid, &what);
        corners_are(
            body,
            TOP,
            &hexagon_top(radius),
            &format!("{what}: the top, set back by both rounds and left between them"),
        );
        corners_are_or_on(
            body,
            HEXAGON_BETWEEN,
            &hexagon_between(radius),
            |corner| {
                rounds
                    .iter()
                    .any(|[running_on, _]| running_on.holds(corner))
            },
            &format!("{what}: wall three, trimmed by both rounds' ends and left between them"),
        );
        let section = radius * radius * (1.0 - PI / 4.0);
        let moment = radius.powi(3) * (5.0 / 6.0 - PI / 4.0);
        sound_drawn_within(
            body,
            hexagon_volume() - 2.0 * taken_off_one_top_edge(section, moment),
            DRAWN
                * reach_of(&prism)
                * rounds
                    .iter()
                    .map(|[running_on, _]| running_on.area())
                    .sum::<f64>(),
            &what,
        );
    }
}

/// The flat a chamfer of `distance` lays on the hexagon's front top edge,
/// turned about the vertical by `turn` — a third of a turn gives wall four's:
/// set back by the distance down the wall, between the edge's ends, and in
/// along the top, running on to the walls beside it, the distance over the
/// square root of three past each end; facing halfway between the top and
/// the wall.
fn hexagon_flat(distance: f64, turn: f64) -> ([DVec3; 4], DVec3) {
    hexagon_flat_to(
        distance,
        turn,
        HEXAGON_SIDE / 2.0 + distance / 3.0f64.sqrt(),
    )
}

/// The flat of [`hexagon_flat`], its side on the top `past` either side of
/// the middle of its edge.
fn hexagon_flat_to(distance: f64, turn: f64, past: f64) -> ([DVec3; 4], DVec3) {
    let half = hexagon_half();
    let end = HEXAGON_SIDE / 2.0;
    let top = HEXAGON_HEIGHT;
    (
        [
            DVec3::new(-end, -half, top - distance),
            DVec3::new(end, -half, top - distance),
            DVec3::new(past, distance - half, top),
            DVec3::new(-past, distance - half, top),
        ]
        .map(|corner| turned(corner, turn)),
        turned(DVec3::new(0.0, -FRAC_1_SQRT_2, FRAC_1_SQRT_2), turn),
    )
}

#[test]
fn a_chamfer_on_two_alternate_top_edges_of_a_hexagonal_prism_asked_past_where_their_set_backs_cross_takes_the_largest_distance_short_of_meeting()
 {
    let prism = hexagonal_prism();
    let names = alternate_top_edges(&prism);
    let laid = [prism.faces_end(), prism.faces_end() + 1];
    let touching = hexagon_touching();

    for asked in [touching + HEXAGON_HAIR, HEXAGON_ASKED] {
        alone_each_takes(
            |name| prism.chamfer(name, asked).map(|single| single.taken),
            &names,
            asked,
            &format!("the chamfer on the hexagon's front top edge or wall four's, asked {asked}"),
        );

        let chamfered = prism
            .chamfer_edges(&names, asked)
            .unwrap_or_else(|declined| panic!("asked {asked}: declined, {declined:?}"));

        let what = format!(
            "the chamfer on the hexagon's front top edge and the top edge two walls round, asked \
             {asked}"
        );
        reduced_to_touching(
            chamfered.asked,
            chamfered.taken,
            [asked, touching],
            HEXAGON_HAIR,
            &what,
        );
        let distance = chamfered.taken;
        let body = &chamfered.body;
        for (turn, number) in [0.0, 2.0 * PI / 3.0].into_iter().zip(laid) {
            let (corners, facing) = hexagon_flat(distance, turn);
            let which = format!("{what}: face {number}, its edge cut off at {distance}");
            faces_toward(body, number, facing, &which);
            corners_are(body, number, &corners, &which);
        }
        do_not_meet(body, laid, &what);
        kept_every_face_and_laid(&prism, body, &laid, &what);
        corners_are(
            body,
            TOP,
            &hexagon_top(distance),
            &format!("{what}: the top, set back by both flats and left between them"),
        );
        corners_are(
            body,
            HEXAGON_BETWEEN,
            &hexagon_between(distance),
            &format!("{what}: wall three, trimmed by both flats' ends and left between them"),
        );
        let section = distance * distance / 2.0;
        let moment = distance.powi(3) / 6.0;
        let promised = hexagon_volume() - 2.0 * taken_off_one_top_edge(section, moment);
        sound_drawn_within(body, promised, EXACT * promised, &what);
    }
}

/// How much further along X the leaning block's right wall stands at the
/// back than at the front.
const LEANING_BY: f64 = 5.0;

/// What is asked of both edges of the leaning block past where they would
/// meet, which either edge alone gives.
const LEANING_ASKED: f64 = 6.0;

/// One of the kernel's hairs on the leaning block, which reaches
/// [`LENGTH`] and [`LEANING_BY`] from the origin: twenty tolerances, a
/// tolerance a billionth of the reach.
const LEANING_HAIR: f64 = 20.0 * 1e-9 * (LENGTH + LEANING_BY);

/// The block [`LENGTH`] long at its front, [`BREADTH`] across and
/// [`HEIGHT`] high, its right wall leaning out from square to the front, to
/// [`LEANING_BY`] further along at the back: its faces numbered as the
/// block's.
fn leaning_block() -> Body {
    raised_with(
        &Body::default(),
        Contour::straight(vec![
            DVec2::ZERO,
            DVec2::new(LENGTH, 0.0),
            DVec2::new(LENGTH + LEANING_BY, BREADTH),
            DVec2::new(0.0, BREADTH),
        ]),
        HEIGHT,
    )
}

/// The leaning wall's length across, the way along it from the front to
/// the back, and the way it faces.
fn leaning_wall() -> (f64, DVec3, DVec3) {
    let length = LEANING_BY.hypot(BREADTH);
    (
        length,
        DVec3::new(LEANING_BY, BREADTH, 0.0) / length,
        DVec3::new(BREADTH, -LEANING_BY, 0.0) / length,
    )
}

/// How far along the leaning wall a value of one sets back the back edge,
/// upright, whose faces meet at the angle whose cosine is the lean over the
/// wall's length: a fillet's radius over the tangent of half that angle, a
/// chamfer's distance as it is.
fn leaning_back_set_back(fillet: bool) -> f64 {
    let (length, _, _) = leaning_wall();
    if fillet {
        (length + LEANING_BY) / BREADTH
    } else {
        1.0
    }
}

/// The value at which the top's front edge's round, or flat, and the back
/// upright's touch at the top of the leaning wall. The first, set back the
/// value along the top square to its edge, reaches along the wall the value
/// times its length over the block's breadth; the second sets the wall back
/// from the back by the value times [`leaning_back_set_back`]. The two fill
/// the wall's length at the value this gives.
fn leaning_touching(fillet: bool) -> f64 {
    let (length, _, _) = leaning_wall();
    length / (length / BREADTH + leaning_back_set_back(fillet))
}

/// What a step at `value` on the top's front edge and the back upright
/// leaves of the leaning wall and of the top, the back upright set back
/// `back` along both its faces: the wall set back from the back and trimmed
/// at its top front corner, the top set back from the front and trimmed at
/// its back right corner, a strip of each left between the two cuts.
fn leaning_wall_and_top(value: f64, back: f64) -> [(usize, Vec<DVec3>); 2] {
    let (length, along, _) = leaning_wall();
    let foot = DVec3::new(LENGTH, 0.0, 0.0);
    let cut = foot + along * (length - back);
    let up = DVec3::Z * HEIGHT;
    let reached = DVec3::new(LENGTH + value * LEANING_BY / BREADTH, value, HEIGHT);
    [
        (
            RIGHT,
            vec![
                foot,
                cut,
                cut + up,
                reached,
                foot + DVec3::Z * (HEIGHT - value),
            ],
        ),
        (
            TOP,
            vec![
                DVec3::new(0.0, value, HEIGHT),
                reached,
                cut + up,
                DVec3::new(LENGTH + LEANING_BY - back, BREADTH, HEIGHT),
                DVec3::new(0.0, BREADTH, HEIGHT),
            ],
        ),
    ]
}

/// The leaning block's matter: its outline, a trapezoid, raised
/// [`HEIGHT`].
fn leaning_volume() -> f64 {
    (LENGTH + LEANING_BY / 2.0) * BREADTH * HEIGHT
}

/// The matter a step takes off along the top's front edge of the leaning
/// block, of section `section` and of first moment `moment` about the front:
/// the section along the front, and on past its end, where the leaning wall
/// stands further along by the lean over the breadth of the way in.
fn leaning_taken_off_the_top_front(section: f64, moment: f64) -> f64 {
    LENGTH * section + LEANING_BY / BREADTH * moment
}

#[test]
fn a_fillet_and_a_chamfer_on_two_edges_whose_rounds_would_meet_on_a_leaning_wall_one_ends_on_take_the_largest_value_short_of_meeting()
 {
    let block = leaning_block();
    let names = top_front_and_right_back(&block);
    let laid = [block.faces_end(), block.faces_end() + 1];
    let (length, _, facing_out) = leaning_wall();
    let angle = (LEANING_BY / length).acos();

    let touching = leaning_touching(true);
    for asked in [touching + LEANING_HAIR, LEANING_ASKED] {
        alone_each_takes(
            |name| block.fillet(name, asked).map(|single| single.taken),
            &names,
            asked,
            &format!(
                "the fillet on the leaning block's top front edge or back upright, asked {asked}"
            ),
        );

        let filleted = block
            .fillet_edges(&names, asked)
            .unwrap_or_else(|declined| panic!("the fillet, asked {asked}: declined, {declined:?}"));

        let what = format!(
            "the fillet on the leaning block's top front edge and back upright, asked {asked}"
        );
        reduced_to_touching(
            filleted.asked,
            filleted.taken,
            [asked, touching],
            LEANING_HAIR,
            &what,
        );
        let radius = filleted.taken;
        let back = radius * leaning_back_set_back(true);
        let body = &filleted.body;
        let along_the_top = |length| Round {
            start: DVec3::new(0.0, radius, HEIGHT - radius),
            along: DVec3::X,
            length,
            radius,
            facing: [DVec3::Z, -DVec3::Y],
        };
        let running_on = along_the_top(LENGTH + radius * LEANING_BY / BREADTH);
        let upright = Round {
            start: DVec3::new(LENGTH + LEANING_BY - back, BREADTH - radius, 0.0),
            along: DVec3::Z,
            length: HEIGHT,
            radius,
            facing: [facing_out, DVec3::Y],
        };
        is_the_round_running_on(
            body,
            laid[0],
            [&running_on, &along_the_top(LENGTH)],
            &format!(
                "{what}: face {}, the top's front edge rounded off at {radius}, running on to \
                 the leaning wall",
                laid[0]
            ),
        );
        is_the_round(
            body,
            laid[1],
            &upright,
            &format!(
                "{what}: face {}, the back upright rounded off at {radius}",
                laid[1]
            ),
        );
        do_not_meet(body, laid, &what);
        kept_every_face_and_laid(&block, body, &laid, &what);
        for (face, corners) in leaning_wall_and_top(radius, back) {
            corners_are_or_on(
                body,
                face,
                &corners,
                |corner| running_on.holds(corner) || upright.holds(corner),
                &format!("{what}: face {face}, cut by both rounds and left between them"),
            );
        }
        let top_front = leaning_taken_off_the_top_front(
            radius * radius * (1.0 - PI / 4.0),
            radius.powi(3) * (5.0 / 6.0 - PI / 4.0),
        );
        let upright_off = HEIGHT * radius * radius * (back / radius - (PI - angle) / 2.0);
        sound_drawn_within(
            body,
            leaning_volume() - top_front - upright_off,
            DRAWN * reach_of(&block) * (running_on.area() + upright.area()),
            &what,
        );
    }

    let touching = leaning_touching(false);
    for asked in [touching + LEANING_HAIR, LEANING_ASKED] {
        alone_each_takes(
            |name| block.chamfer(name, asked).map(|single| single.taken),
            &names,
            asked,
            &format!(
                "the chamfer on the leaning block's top front edge or back upright, asked {asked}"
            ),
        );

        let chamfered = block
            .chamfer_edges(&names, asked)
            .unwrap_or_else(|declined| {
                panic!("the chamfer, asked {asked}: declined, {declined:?}")
            });

        let what = format!(
            "the chamfer on the leaning block's top front edge and back upright, asked {asked}"
        );
        reduced_to_touching(
            chamfered.asked,
            chamfered.taken,
            [asked, touching],
            LEANING_HAIR,
            &what,
        );
        let distance = chamfered.taken;
        let body = &chamfered.body;
        let [(_, wall), _] = leaning_wall_and_top(distance, distance);
        let flats = [
            (
                vec![
                    DVec3::new(0.0, 0.0, HEIGHT - distance),
                    DVec3::new(LENGTH, 0.0, HEIGHT - distance),
                    wall[3],
                    DVec3::new(0.0, distance, HEIGHT),
                ],
                DVec3::new(0.0, -FRAC_1_SQRT_2, FRAC_1_SQRT_2),
            ),
            (
                vec![
                    wall[1],
                    wall[2],
                    DVec3::new(LENGTH + LEANING_BY - distance, BREADTH, HEIGHT),
                    DVec3::new(LENGTH + LEANING_BY - distance, BREADTH, 0.0),
                ],
                (facing_out + DVec3::Y).normalize(),
            ),
        ];
        for ((corners, facing), number) in flats.iter().zip(laid) {
            let which = format!("{what}: face {number}, its edge cut off at {distance}");
            faces_toward(body, number, *facing, &which);
            corners_are(body, number, corners, &which);
        }
        do_not_meet(body, laid, &what);
        kept_every_face_and_laid(&block, body, &laid, &what);
        for (face, corners) in leaning_wall_and_top(distance, distance) {
            corners_are(
                body,
                face,
                &corners,
                &format!("{what}: face {face}, cut by both flats and left between them"),
            );
        }
        let top_front =
            leaning_taken_off_the_top_front(distance * distance / 2.0, distance.powi(3) / 6.0);
        let upright_off = HEIGHT * distance * distance * angle.sin() / 2.0;
        let promised = leaning_volume() - top_front - upright_off;
        sound_drawn_within(body, promised, EXACT * promised, &what);
    }
}

/// The round a fillet lays in a concave corner: a cylinder of `radius` about
/// the line from `start` along `along` for `length`, that line standing in
/// the air the two faces leave between them. The round turns from facing the
/// way the edge's first face does to facing the way its second does, and
/// everywhere faces its axis, which stands a radius along the way it faces.
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
    /// ends, and between its two sides — no further round from the way it
    /// faces halfway than either side is.
    fn holds(&self, point: DVec3) -> bool {
        let off = point - self.start;
        let along = off.dot(self.along);
        let toward_the_axis = self.along * along - off;
        let halfway = self.facing_at(0.5);
        let least = self.radius * self.facing[0].dot(halfway);
        (toward_the_axis.length() - self.radius).abs() < PLACE
            && (-PLACE..=self.length + PLACE).contains(&along)
            && toward_the_axis.dot(halfway) >= least - PLACE
    }

    /// The round's area, its turn times its radius times its length.
    fn area(&self) -> f64 {
        let [first, second] = self.facing;
        first.dot(second).acos() * self.radius * self.length
    }
}

/// Holds that the face numbered `face` is the hollow round expected: every
/// corner of its triangles on it, and every ray aimed from its axis, in the
/// air the round leaves, out to it meeting it first, where it faces back
/// along the ray.
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
            let from = hollow.axis_at(along);
            let hit = body
                .ray_hit(from, -facing)
                .unwrap_or_else(|| panic!("{what}: a ray from {from} on the axis meets nothing"));
            assert_eq!(
                hit.face, face,
                "{what}: a ray from {from} on the axis meets face {} before the round",
                hit.face
            );
            assert!(
                hit.normal.distance(facing) < PLACE,
                "{what}: where a ray from {from} meets it, the round faces {}, not {facing}, \
                 back toward its axis",
                hit.normal
            );
        }
    }
}

/// The outline of an L: an arm [`LENGTH`] long and 6 deep along the front,
/// and at its left end a stem 12 wide reaching back to 20. Raised
/// [`ELL_HEIGHT`], its walls are numbered from the front round: the front
/// two, the arm's end three, the arm's back four, the stem's right side
/// five, the stem's back six and the left seven. The arm's back and the
/// stem's right side meet inwards, in the L's inner corner.
const ELL_OUTLINE: [DVec2; 6] = [
    DVec2::new(0.0, 0.0),
    DVec2::new(LENGTH, 0.0),
    DVec2::new(LENGTH, 6.0),
    DVec2::new(12.0, 6.0),
    DVec2::new(12.0, 20.0),
    DVec2::new(0.0, 20.0),
];
const ELL_HEIGHT: f64 = 20.0;
const ARM_END: usize = 3;
const ARM_BACK: usize = 4;
const STEM_RIGHT: usize = 5;

/// What is asked of the L's two edges past where they would meet, which
/// either alone gives: under the arm's back, the stem's right side and the
/// L's height.
const ELL_ASKED: f64 = 11.0;

/// The L of [`ELL_OUTLINE`], raised [`ELL_HEIGHT`].
fn ell() -> Body {
    raised_with(
        &Body::default(),
        Contour::straight(ELL_OUTLINE.to_vec()),
        ELL_HEIGHT,
    )
}

/// The two edges of the L that meet through an end face, in the order the
/// step names them: the top edge of the arm's end, which ends on the arm's
/// back, and the L's inner upright, of which the arm's back is a side —
/// filled in, its faces meeting inwards.
fn arm_end_top_and_inner_upright(ell: &Body) -> [EdgeName; 2] {
    [
        edge_between(ell, TOP, ARM_END),
        edge_between(ell, ARM_BACK, STEM_RIGHT),
    ]
}

/// The value at which the arm's end's round, or flat, and the inner
/// upright's fill touch at the top of the arm's back: the first sets the
/// arm's back back from its end by the value, the second fills it in from
/// the inner corner by as much, and the arm's back runs from one to the
/// other.
fn ell_touching() -> f64 {
    let [_, _, end, corner, _, _] = ELL_OUTLINE;
    (end.x - corner.x) / 2.0
}

/// What a step at `value` on the arm's end's top edge and the inner upright
/// leaves of the arm's back and of the top: the arm's back filled in from
/// the inner corner and trimmed at its top end, the top set back from the
/// arm's end and grown into the inner corner, a strip of each left between
/// the two.
fn arm_back_and_top(value: f64) -> [(usize, Vec<DVec3>); 2] {
    let [front, _, end, corner, stem, back] = ELL_OUTLINE;
    let up = |at: DVec2, z: f64| at.extend(z);
    let top = ELL_HEIGHT;
    [
        (
            ARM_BACK,
            vec![
                up(corner + DVec2::X * value, 0.0),
                up(end, 0.0),
                up(end, top - value),
                up(end - DVec2::X * value, top),
                up(corner + DVec2::X * value, top),
            ],
        ),
        (
            TOP,
            vec![
                up(front, top),
                DVec3::new(end.x - value, 0.0, top),
                up(end - DVec2::X * value, top),
                up(corner + DVec2::X * value, top),
                up(corner + DVec2::Y * value, top),
                up(stem, top),
                up(back, top),
            ],
        ),
    ]
}

/// The L's matter: its arm and its stem, raised [`ELL_HEIGHT`].
fn ell_volume() -> f64 {
    (LENGTH * 6.0 + 12.0 * 14.0) * ELL_HEIGHT
}

#[test]
fn a_fillet_and_a_chamfer_on_the_top_edge_of_an_l_s_arm_end_and_its_inner_upright_asked_more_than_half_the_arm_s_back_take_the_largest_value_short_of_meeting()
 {
    let ell = ell();
    let names = arm_end_top_and_inner_upright(&ell);
    let laid = [ell.faces_end(), ell.faces_end() + 1];
    let touching = ell_touching();
    let [_, start, end, corner, _, _] = ELL_OUTLINE;
    let top = ELL_HEIGHT;
    let filled_less_cut = top - (end.y - start.y);

    for asked in [touching + ONE_HAIR, ELL_ASKED] {
        alone_each_takes(
            |name| ell.fillet(name, asked).map(|single| single.taken),
            &names,
            asked,
            &format!("the fillet on the L's arm end's top edge or inner upright, asked {asked}"),
        );

        let filleted = ell
            .fillet_edges(&names, asked)
            .unwrap_or_else(|declined| panic!("the fillet, asked {asked}: declined, {declined:?}"));

        let what =
            format!("the fillet on the L's arm end's top edge and inner upright, asked {asked}");
        reduced_to_touching(
            filleted.asked,
            filleted.taken,
            [asked, touching],
            ONE_HAIR,
            &what,
        );
        let radius = filleted.taken;
        let body = &filleted.body;
        let round = Round {
            start: DVec3::new(end.x - radius, 0.0, top - radius),
            along: DVec3::Y,
            length: 6.0,
            radius,
            facing: [DVec3::Z, DVec3::X],
        };
        let hollow = Hollow {
            start: (corner + DVec2::splat(radius)).extend(0.0),
            along: DVec3::Z,
            length: top,
            radius,
            facing: [DVec3::Y, DVec3::X],
        };
        is_the_round(
            body,
            laid[0],
            &round,
            &format!(
                "{what}: face {}, the arm end's top edge rounded off at {radius}",
                laid[0]
            ),
        );
        is_the_hollow(
            body,
            laid[1],
            &hollow,
            &format!(
                "{what}: face {}, the inner upright filled in at {radius}",
                laid[1]
            ),
        );
        do_not_meet(body, laid, &what);
        kept_every_face_and_laid(&ell, body, &laid, &what);
        for (face, corners) in arm_back_and_top(radius) {
            corners_are_or_on(
                body,
                face,
                &corners,
                |corner| round.holds(corner) || hollow.holds(corner),
                &format!("{what}: face {face}, cut by the round, filled by the fill, a strip left"),
            );
        }
        sound_drawn_within(
            body,
            ell_volume() + radius * radius * (1.0 - PI / 4.0) * filled_less_cut,
            DRAWN * reach_of(&ell) * (round.area() + hollow.area()),
            &what,
        );
    }

    for asked in [touching + ONE_HAIR, ELL_ASKED] {
        alone_each_takes(
            |name| ell.chamfer(name, asked).map(|single| single.taken),
            &names,
            asked,
            &format!("the chamfer on the L's arm end's top edge or inner upright, asked {asked}"),
        );

        let chamfered = ell.chamfer_edges(&names, asked).unwrap_or_else(|declined| {
            panic!("the chamfer, asked {asked}: declined, {declined:?}")
        });

        let what =
            format!("the chamfer on the L's arm end's top edge and inner upright, asked {asked}");
        reduced_to_touching(
            chamfered.asked,
            chamfered.taken,
            [asked, touching],
            ONE_HAIR,
            &what,
        );
        let distance = chamfered.taken;
        let body = &chamfered.body;
        let filled = corner + DVec2::X * distance;
        let flats = [
            (
                [
                    DVec3::new(end.x - distance, 0.0, top),
                    DVec3::new(end.x, 0.0, top - distance),
                    DVec3::new(end.x, end.y, top - distance),
                    DVec3::new(end.x - distance, end.y, top),
                ],
                DVec3::new(FRAC_1_SQRT_2, 0.0, FRAC_1_SQRT_2),
            ),
            (
                [
                    filled.extend(0.0),
                    filled.extend(top),
                    (corner + DVec2::Y * distance).extend(top),
                    (corner + DVec2::Y * distance).extend(0.0),
                ],
                DVec3::new(FRAC_1_SQRT_2, FRAC_1_SQRT_2, 0.0),
            ),
        ];
        for (((corners, facing), number), edge) in flats
            .iter()
            .zip(laid)
            .zip(["arm end's top edge cut off", "inner upright filled in"])
        {
            let which = format!("{what}: face {number}, the {edge} at {distance}");
            faces_toward(body, number, *facing, &which);
            corners_are(body, number, corners, &which);
        }
        do_not_meet(body, laid, &what);
        kept_every_face_and_laid(&ell, body, &laid, &what);
        for (face, corners) in arm_back_and_top(distance) {
            corners_are(
                body,
                face,
                &corners,
                &format!("{what}: face {face}, cut by one flat, filled by the other, a strip left"),
            );
        }
        let promised = ell_volume() + distance * distance / 2.0 * filled_less_cut;
        sound_drawn_within(body, promised, EXACT * promised, &what);
    }
}

/// Three top edges in a row of the hexagonal prism: the front's, wall
/// three's and wall four's, each turned about the vertical from the front's
/// by a sixth of a turn more, and the walls they stand on.
const IN_A_ROW: [usize; 3] = [HEXAGON_FRONT, HEXAGON_BETWEEN, HEXAGON_ACROSS];

/// What a step at `value` on the three top edges in a row leaves of the
/// hexagon's top: set back by the value from each, the front's and wall
/// four's set-backs running on to the walls beyond their outer ends, and
/// each two of the three meeting in a mitre at the corner they share.
fn in_a_row_top(value: f64) -> Vec<DVec3> {
    let (half, reach) = (hexagon_half(), value / 3.0f64.sqrt());
    let side = HEXAGON_SIDE;
    [
        DVec2::new(-side / 2.0 - reach, value - half),
        DVec2::new(side / 2.0 - reach, value - half),
        DVec2::new(side - 2.0 * reach, 0.0),
        DVec2::new(side / 2.0 - 2.0 * reach, half),
        DVec2::new(-side / 2.0, half),
        DVec2::new(-side, 0.0),
    ]
    .map(|corner| corner.extend(HEXAGON_HEIGHT))
    .to_vec()
}

/// What a step at `value` leaves of the wall under one of the three edges,
/// the edge turned by `turn` from the front's: the wall cut down by the
/// value along its whole length, between the uprights at its ends.
fn in_a_row_wall(value: f64, turn: f64) -> Vec<DVec3> {
    let (half, end) = (hexagon_half(), HEXAGON_SIDE / 2.0);
    let cut = HEXAGON_HEIGHT - value;
    [(-end, 0.0), (end, 0.0), (end, cut), (-end, cut)]
        .map(|(x, z)| turned(DVec3::new(x, -half, z), turn))
        .to_vec()
}

/// The matter a step at `value` on the three top edges in a row takes off
/// the hexagonal prism, a fillet's if `fillet`, a chamfer's if not. At a
/// height where the cuts stand `w` in from the walls, the section loses a
/// strip `w` wide along each of the three edges, less, at each of the two
/// corners where two strips meet in a mitre, the `w² / √3` they share, plus,
/// at each outer end, the `w² / (2√3)` its strip runs on along the wall
/// beyond: `3 s w − w² / √3`, for the side `s`. A chamfer stands in by its
/// rise `t` above where it begins on the walls, which sums up its height to
/// `3 s d² / 2 − d³ / (3√3)`; a fillet by `r − √(r² − t²)`, whose height
/// sums `w` to `r² (1 − π / 4)` and `w²` to `r³ (5/3 − π / 2)`.
fn taken_off_in_a_row(value: f64, fillet: bool) -> f64 {
    let (side, root) = (HEXAGON_SIDE, 3.0f64.sqrt());
    let (first, second) = if fillet {
        (
            value * value * (1.0 - PI / 4.0),
            value.powi(3) * (5.0 / 3.0 - PI / 2.0),
        )
    } else {
        (value * value / 2.0, value.powi(3) / 3.0)
    };
    3.0 * side * first - second / root
}

#[test]
fn a_fillet_and_a_chamfer_on_three_top_edges_in_a_row_of_a_hexagonal_prism_asked_past_where_the_outer_two_would_cross_take_the_value_asked_the_middle_one_named_before_the_last()
 {
    let prism = hexagonal_prism();
    let value = HEXAGON_ASKED;
    let end = HEXAGON_SIDE / 2.0;
    let mitred = end - value / 3.0f64.sqrt();
    let run_on = end + value / 3.0f64.sqrt();
    assert!(
        value > hexagon_touching() && value < 2.0 * hexagon_touching(),
        "asked {value}: past where the outer two's set-backs would cross on the middle edge, \
         {}, and short of where they would cross within the middle one's own set-back, {}",
        hexagon_touching(),
        2.0 * hexagon_touching()
    );

    for order in [[0, 1, 2], [2, 1, 0], [1, 0, 2], [1, 2, 0]] {
        let names: Vec<EdgeName> = order
            .iter()
            .map(|&rank| edge_between(&prism, TOP, IN_A_ROW[rank]))
            .collect();
        let laid: Vec<usize> = (0..order.len())
            .map(|named| prism.faces_end() + named)
            .collect();
        let number_of = |rank: usize| {
            laid[order
                .iter()
                .position(|&named| named == rank)
                .expect("named")]
        };
        let turn_of = |rank: usize| rank as f64 * PI / 3.0;

        let filleted = prism
            .fillet_edges(&names, value)
            .unwrap_or_else(|declined| {
                panic!("the fillet, named {order:?}: declined, {declined:?}")
            });
        let chamfered = prism
            .chamfer_edges(&names, value)
            .unwrap_or_else(|declined| {
                panic!("the chamfer, named {order:?}: declined, {declined:?}")
            });

        let what = format!(
            "the fillet on three top edges in a row of the hexagon, named {order:?}, asked \
             {value}: the middle one cut too, the outer two never cross on it"
        );
        taken_as_asked(filleted.asked, filleted.taken, value, &what);
        let body = &filleted.body;
        for (rank, &wall) in IN_A_ROW.iter().enumerate() {
            let (number, turn) = (number_of(rank), turn_of(rank));
            let which = format!("{what}: face {number}, wall {wall}'s top edge rounded off");
            let reach = if rank == 1 { end } else { run_on };
            drawn_on(body, number, &hexagon_round_to(value, turn, reach), &which);
            met_from_outside(body, number, &hexagon_round_to(value, turn, mitred), &which);
            corners_are(
                body,
                wall,
                &in_a_row_wall(value, turn),
                &format!("{what}: wall {wall}, cut down by the value"),
            );
        }
        do_not_meet(body, [number_of(0), number_of(2)], &what);
        kept_every_face_and_laid(&prism, body, &laid, &what);
        corners_are(
            body,
            TOP,
            &in_a_row_top(value),
            &format!("{what}: the top, set back by the value from all three edges"),
        );
        let area = IN_A_ROW.len() as f64 * PI / 2.0 * value * (HEXAGON_SIDE + value);
        sound_drawn_within(
            body,
            hexagon_volume() - taken_off_in_a_row(value, true),
            DRAWN * reach_of(&prism) * area,
            &what,
        );

        let what = format!(
            "the chamfer on three top edges in a row of the hexagon, named {order:?}, asked \
             {value}: the middle one cut too, the outer two never cross on it"
        );
        taken_as_asked(chamfered.asked, chamfered.taken, value, &what);
        let body = &chamfered.body;
        for (rank, &wall) in IN_A_ROW.iter().enumerate() {
            let (number, turn) = (number_of(rank), turn_of(rank));
            let which = format!("{what}: face {number}, wall {wall}'s top edge cut off");
            let (_, facing) = hexagon_flat_to(value, turn, mitred);
            faces_toward(body, number, facing, &which);
            corners_are(
                body,
                wall,
                &in_a_row_wall(value, turn),
                &format!("{what}: wall {wall}, cut down by the value"),
            );
        }
        do_not_meet(body, [number_of(0), number_of(2)], &what);
        kept_every_face_and_laid(&prism, body, &laid, &what);
        corners_are(
            body,
            TOP,
            &in_a_row_top(value),
            &format!("{what}: the top, set back by the value from all three edges"),
        );
        let promised = hexagon_volume() - taken_off_in_a_row(value, false);
        sound_drawn_within(body, promised, EXACT * promised, &what);
    }
}

/// Two edges of a part whose rounds would meet, the part, what is asked of
/// them past it, the value at which a fillet's rounds and a chamfer's flats
/// touch, and the part's hair.
type Meeting = (&'static str, [EdgeName; 2], Body, f64, [f64; 2], f64);

/// Every pair this file reduces the value for: through an end face on the
/// block, on the leaning block and on the L, and across the hexagon's top.
fn meetings() -> Vec<Meeting> {
    let block = block();
    let leaning = leaning_block();
    let prism = hexagonal_prism();
    let ell = ell();
    vec![
        (
            "the block's top front edge and right wall's back edge",
            top_front_and_right_back(&block),
            block,
            MORE_THAN_HALF,
            [TOUCHING, TOUCHING],
            ONE_HAIR,
        ),
        (
            "the leaning block's top front edge and back upright",
            top_front_and_right_back(&leaning),
            leaning,
            LEANING_ASKED,
            [leaning_touching(true), leaning_touching(false)],
            LEANING_HAIR,
        ),
        (
            "the hexagon's front top edge and the top edge two walls round",
            alternate_top_edges(&prism),
            prism,
            HEXAGON_ASKED,
            [hexagon_touching(), hexagon_touching()],
            HEXAGON_HAIR,
        ),
        (
            "the L's arm end's top edge and inner upright",
            arm_end_top_and_inner_upright(&ell),
            ell,
            ELL_ASKED,
            [ell_touching(), ell_touching()],
            ONE_HAIR,
        ),
    ]
}

#[test]
fn a_fillet_and_a_chamfer_on_two_edges_whose_rounds_would_meet_take_the_same_value_whichever_edge_the_step_names_first()
 {
    for (pair, [first, second], part, asked, touching, hair) in meetings() {
        for (kind, fillet) in [("fillet", true), ("chamfer", false)] {
            let step = |names: &[EdgeName]| -> (f64, f64, f64) {
                let laid = if fillet {
                    part.fillet_edges(names, asked)
                        .map(|done| (done.asked, done.taken, done.body.volume()))
                } else {
                    part.chamfer_edges(names, asked)
                        .map(|done| (done.asked, done.taken, done.body.volume()))
                };
                laid.unwrap_or_else(|declined| {
                    panic!("the {kind} on {pair}, asked {asked}: declined, {declined:?}")
                })
            };
            let touching = touching[usize::from(!fillet)];

            let named_first = step(&[first.clone(), second.clone()]);
            let named_second = step(&[second.clone(), first.clone()]);

            for ((reported, taken, _), order) in [
                (named_first, "in the order given"),
                (named_second, "the other way round"),
            ] {
                reduced_to_touching(
                    reported,
                    taken,
                    [asked, touching],
                    hair,
                    &format!("the {kind} on {pair}, named {order}"),
                );
            }
            assert!(
                (named_first.1 - named_second.1).abs() < PLACE,
                "the {kind} on {pair}: took {} named in the order given and {} the other way \
                 round",
                named_first.1,
                named_second.1
            );
            assert!(
                (named_first.2 - named_second.2).abs() <= EXACT * named_first.2,
                "the {kind} on {pair}: holds {} named in the order given and {} the other way \
                 round",
                named_first.2,
                named_second.2
            );
        }
    }
}

/// A step on edges of a part, a fillet's if `fillet`, a chamfer's if not:
/// the value it reports asked, the one it took, and the body it laid.
fn stepped(
    prism: &Body,
    names: &[EdgeName],
    asked: f64,
    fillet: bool,
    what: &str,
) -> (f64, f64, Body) {
    let laid = if fillet {
        prism
            .fillet_edges(names, asked)
            .map(|done| (done.asked, done.taken, done.body))
    } else {
        prism
            .chamfer_edges(names, asked)
            .map(|done| (done.asked, done.taken, done.body))
    };
    laid.unwrap_or_else(|declined| panic!("{what}: declined, {declined:?}"))
}

/// Holds that the face numbered `number` is what a step at `value` laid on
/// the top edge turned by `turn` from the front's: for a fillet, its round,
/// drawn within `reach` either side of the edge's middle and met by rays
/// between the mitres; for a chamfer, a flat facing halfway between the top
/// and the wall.
fn laid_on_the_top_edge(
    body: &Body,
    number: usize,
    [value, turn, reach]: [f64; 3],
    fillet: bool,
    what: &str,
) {
    let mitred = HEXAGON_SIDE / 2.0 - value / 3.0f64.sqrt();
    if fillet {
        drawn_on(body, number, &hexagon_round_to(value, turn, reach), what);
        met_from_outside(body, number, &hexagon_round_to(value, turn, mitred), what);
    } else {
        let (_, facing) = hexagon_flat_to(value, turn, mitred);
        faces_toward(body, number, facing, what);
    }
}

#[test]
fn a_fillet_and_a_chamfer_on_three_top_edges_in_a_row_of_a_hexagonal_prism_named_with_the_middle_one_last_asked_past_where_the_outer_two_would_cross_take_the_value_asked()
 {
    let prism = hexagonal_prism();
    let asked = HEXAGON_ASKED;
    let end = HEXAGON_SIDE / 2.0;

    for order in [[0, 2, 1], [2, 0, 1]] {
        let names: Vec<EdgeName> = order
            .iter()
            .map(|&rank| edge_between(&prism, TOP, IN_A_ROW[rank]))
            .collect();
        let laid: Vec<usize> = (0..order.len())
            .map(|named| prism.faces_end() + named)
            .collect();
        let number_of = |rank: usize| {
            laid[order
                .iter()
                .position(|&named| named == rank)
                .expect("named")]
        };
        for (kind, fillet) in [("fillet", true), ("chamfer", false)] {
            let what = format!(
                "the {kind} on three top edges in a row of the hexagon, named {order:?}, asked \
                 {asked}: the middle one laid after both outer ones, its cut taking away where \
                 they would cross on it"
            );

            let (reported, taken, body) = stepped(&prism, &names, asked, fillet, &what);

            taken_as_asked(reported, taken, asked, &what);
            for (rank, &wall) in IN_A_ROW.iter().enumerate() {
                let turn = rank as f64 * PI / 3.0;
                let reach = if rank == 1 {
                    end
                } else {
                    end + taken / 3.0f64.sqrt()
                };
                laid_on_the_top_edge(
                    &body,
                    number_of(rank),
                    [taken, turn, reach],
                    fillet,
                    &format!("{what}: face {}, wall {wall}'s top edge", number_of(rank)),
                );
                corners_are(
                    &body,
                    wall,
                    &in_a_row_wall(taken, turn),
                    &format!("{what}: wall {wall}, cut down by the value taken"),
                );
            }
            do_not_meet(&body, [number_of(0), number_of(2)], &what);
            kept_every_face_and_laid(&prism, &body, &laid, &what);
            corners_are(
                &body,
                TOP,
                &in_a_row_top(taken),
                &format!("{what}: the top, set back by the value taken from all three edges"),
            );
            let promised = hexagon_volume() - taken_off_in_a_row(taken, fillet);
            let drawn = if fillet {
                DRAWN * reach_of(&prism) * 3.0 * PI / 2.0 * taken * (HEXAGON_SIDE + taken)
            } else {
                EXACT * promised
            };
            sound_drawn_within(&body, promised, drawn, &what);
        }
    }
}

/// The numbers of the hexagonal prism's six walls in ring order, from the
/// front round: the top edge of each turned about the vertical from the
/// front's by a sixth of a turn for each wall before it.
const HEXAGON_WALLS: [usize; 6] = [2, 3, 4, 5, 6, 7];

/// What a step at `value` on all six top edges of the hexagonal prism
/// leaves of its top: the hexagon set back by the value from every edge, a
/// hexagon of the same middle, its corners mitred by two edges' cuts each.
fn ring_top(value: f64) -> Vec<DVec3> {
    let shrink = 1.0 - value / hexagon_half();
    hexagon_corners()
        .map(|corner| (corner * shrink).extend(HEXAGON_HEIGHT))
        .to_vec()
}

/// The matter a step at `value` on all six top edges takes off the
/// hexagonal prism, a fillet's if `fillet`, a chamfer's if not. Every edge
/// cut alike and their cuts mitred, the section at a rise `t` above where
/// the cuts begin on the walls is the hexagon set back by what the cut
/// stands in there, δ — `t` for a chamfer, `r − √(r² − t²)` for a fillet —,
/// of apothem `a − δ` and holding `2√3 (a − δ)²`. Summed up the height the
/// cuts take and from the hexagon whole there, that leaves
/// `2√3 (a² d − (a³ − (a − d)³) / 3)` for a chamfer of `d`, and
/// `2√3 (a² r − (a − r)² r − (a − r) π r² / 2 − 2 r³ / 3)` for a fillet of `r`.
fn taken_off_the_ring(value: f64, fillet: bool) -> f64 {
    let (a, v) = (hexagon_half(), value);
    let root = 2.0 * 3.0f64.sqrt();
    if fillet {
        root * (a * a * v
            - (a - v).powi(2) * v
            - (a - v) * PI * v * v / 2.0
            - 2.0 * v.powi(3) / 3.0)
    } else {
        root * (a * a * v - (a.powi(3) - (a - v).powi(3)) / 3.0)
    }
}

/// Holds that no two of the faces `laid` around a ring, but those next to
/// each other in it, share an edge: each meets only its neighbours.
fn only_neighbours_meet(body: &Body, laid: &[usize], what: &str) {
    for (rank, &one) in laid.iter().enumerate() {
        for (later, &other) in laid.iter().enumerate().skip(rank + 1) {
            let apart = later - rank;
            if apart != 1 && apart != laid.len() - 1 {
                do_not_meet(body, [one, other], what);
            }
        }
    }
}

#[test]
fn a_fillet_and_a_chamfer_on_the_six_top_edges_of_a_hexagonal_prism_in_ring_order_asked_past_where_alternate_edges_would_cross_take_the_value_asked()
 {
    let prism = hexagonal_prism();
    let asked = HEXAGON_ASKED;
    let names: Vec<EdgeName> = HEXAGON_WALLS
        .iter()
        .map(|&wall| edge_between(&prism, TOP, wall))
        .collect();
    let laid: Vec<usize> = (0..HEXAGON_WALLS.len())
        .map(|rank| prism.faces_end() + rank)
        .collect();

    for (kind, fillet) in [("fillet", true), ("chamfer", false)] {
        let what = format!(
            "the {kind} on the hexagon's six top edges in ring order, asked {asked}: the last \
             laid after both its neighbours, its cut taking away where they would cross on it"
        );

        let (reported, taken, body) = stepped(&prism, &names, asked, fillet, &what);

        taken_as_asked(reported, taken, asked, &what);
        for ((rank, &wall), &number) in HEXAGON_WALLS.iter().enumerate().zip(&laid) {
            let turn = rank as f64 * PI / 3.0;
            laid_on_the_top_edge(
                &body,
                number,
                [taken, turn, HEXAGON_SIDE / 2.0],
                fillet,
                &format!("{what}: face {number}, wall {wall}'s top edge"),
            );
            corners_are(
                &body,
                wall,
                &in_a_row_wall(taken, turn),
                &format!("{what}: wall {wall}, cut down by the value taken"),
            );
        }
        only_neighbours_meet(&body, &laid, &what);
        kept_every_face_and_laid(&prism, &body, &laid, &what);
        corners_are(
            &body,
            TOP,
            &ring_top(taken),
            &format!("{what}: the top, set back by the value taken from every edge"),
        );
        let promised = hexagon_volume() - taken_off_the_ring(taken, fillet);
        let drawn = if fillet {
            DRAWN * reach_of(&prism) * 6.0 * PI / 2.0 * taken * HEXAGON_SIDE
        } else {
            EXACT * promised
        };
        sound_drawn_within(&body, promised, drawn, &what);
    }
}

/// The trapezoid's short side, its two sides, the angle they lean from the
/// short side's line — so that each meets it at 95 degrees, and the two
/// converge 10 degrees apart below it — and its height.
const TRAPEZOID_SHORT: f64 = 10.0;
const TRAPEZOID_SIDE: f64 = 20.0;
const TRAPEZOID_LEAN_DEGREES: f64 = 85.0;
const TRAPEZOID_HEIGHT: f64 = 10.0;

/// The numbers the trapezoid's walls answer to, from its short side round:
/// the short side two, the right side three, the long side four, the left
/// side five.
const TRAPEZOID_SHORT_WALL: usize = 2;
const TRAPEZOID_RIGHT: usize = 3;
const TRAPEZOID_LEFT: usize = 5;

/// What is asked of the trapezoid's three top edges past where the short
/// one's set-back closes, which each alone gives.
const TRAPEZOID_ASKED: f64 = 6.0;

/// Where the trapezoid is drawn off the origin as well as on it: along
/// both axes, so that no point the step reads lies square to the ways it
/// reads them along.
const TRAPEZOID_MOVED: DVec2 = DVec2::new(3.0, 7.0);

/// The trapezoid's corners, from the short side's left end round, the
/// first at `at`.
fn trapezoid_corners_at(at: DVec2) -> [DVec2; 4] {
    let lean = TRAPEZOID_LEAN_DEGREES.to_radians();
    let side = DVec2::new(lean.cos(), lean.sin()) * TRAPEZOID_SIDE;
    [
        DVec2::ZERO,
        DVec2::new(TRAPEZOID_SHORT, 0.0),
        DVec2::new(TRAPEZOID_SHORT, 0.0) + side,
        DVec2::new(-side.x, side.y),
    ]
    .map(|corner| corner + at)
}

/// The trapezoid of [`trapezoid_corners_at`] `at`, raised
/// [`TRAPEZOID_HEIGHT`].
fn trapezoid_at(at: DVec2) -> Body {
    raised_with(
        &Body::default(),
        Contour::straight(trapezoid_corners_at(at).to_vec()),
        TRAPEZOID_HEIGHT,
    )
}

/// The three top edges, in the order the step names them: the left side's,
/// the short side's between, and the right side's.
fn trapezoid_edges(part: &Body) -> [EdgeName; 3] {
    [
        edge_between(part, TOP, TRAPEZOID_LEFT),
        edge_between(part, TOP, TRAPEZOID_SHORT_WALL),
        edge_between(part, TOP, TRAPEZOID_RIGHT),
    ]
}

/// The value at which the short top edge's set-back closes between its two
/// mitres. Each side's set-back crosses the short edge's the value times the
/// cotangent of half the corner, 95 degrees, in from its end; the two fill
/// the short edge at half its length times the tangent of half the corner.
/// There the three set-backs meet in one point; past it the sides' cross on
/// the top beyond the short edge's strip.
fn trapezoid_closing() -> f64 {
    let corner = 180.0 - TRAPEZOID_LEAN_DEGREES;
    TRAPEZOID_SHORT / 2.0 * (corner / 2.0).to_radians().tan()
}

/// A top edge of the trapezoid, from `from` to `to` round the outline: the
/// way along it and the way its wall faces, out of the matter.
fn trapezoid_edge(from: DVec2, to: DVec2) -> (DVec2, DVec2) {
    let along = (to - from).normalize();
    (along, DVec2::new(along.y, -along.x))
}

/// Where two lines cross, each a point and a way along it.
fn crossing((point, way): (DVec2, DVec2), (other, other_way): (DVec2, DVec2)) -> DVec2 {
    let off = other - point;
    let along =
        (off.x * other_way.y - off.y * other_way.x) / (way.x * other_way.y - way.y * other_way.x);
    point + way * along
}

/// What is left of the top of the trapezoid of `corners` once its left,
/// short and right top edges are set back by `inset` along it: the long
/// edge as it was, and each corner where two of the four lines cross.
fn trapezoid_top(corners: [DVec2; 4], inset: f64) -> [DVec3; 4] {
    let [c0, c1, c2, c3] = corners;
    let line = |from: DVec2, to: DVec2, by: f64| {
        let (along, out) = trapezoid_edge(from, to);
        (from - out * by, along)
    };
    let [left, short, right, long] = [
        line(c3, c0, inset),
        line(c0, c1, inset),
        line(c1, c2, inset),
        line(c2, c3, 0.0),
    ];
    [
        crossing(left, short),
        crossing(short, right),
        crossing(right, long),
        crossing(long, left),
    ]
    .map(|corner| corner.extend(TRAPEZOID_HEIGHT))
}

/// Holds what a step at `value` on the trapezoid's three top edges, numbered
/// `laid` in the order of [`trapezoid_edges`], the trapezoid drawn `at`,
/// leaves: each edge's round on
/// its cylinder, or its flat facing halfway between the top and its wall;
/// each of the three walls cut down by the value; the top set back by it
/// from all three, mitred at the short edge's ends and running on to the
/// long one; the two sides' faces sharing no edge; every face kept; and the
/// matter, at a height where the cuts stand `w` in, losing a strip along the
/// three edges, `50 w`, less `cot 47.5° w²` at each mitre and
/// `cot 85° w² / 2` where each side's strip runs on to the long edge.
fn trapezoid_laid(
    part: &Body,
    body: &Body,
    (laid, at): ([usize; 3], DVec2),
    value: f64,
    fillet: bool,
    what: &str,
) {
    let corners = trapezoid_corners_at(at);
    let [c0, c1, c2, c3] = corners;
    let top = TRAPEZOID_HEIGHT;
    let edges = [
        (c3, c0, TRAPEZOID_LEFT),
        (c0, c1, TRAPEZOID_SHORT_WALL),
        (c1, c2, TRAPEZOID_RIGHT),
    ];
    for ((from, to, wall), number) in edges.into_iter().zip(laid) {
        let (along, out) = trapezoid_edge(from, to);
        let which = format!("{what}: face {number}, wall {wall}'s top edge");
        if fillet {
            let round = |past: f64, middle: bool| {
                let length = (to - from).length();
                let start = if middle {
                    from + along * (length / 2.0 - past)
                } else {
                    from - along * past
                };
                Round {
                    start: (start - out * value).extend(top - value),
                    along: along.extend(0.0),
                    length: if middle {
                        2.0 * past
                    } else {
                        length + 2.0 * past
                    },
                    radius: value,
                    facing: [DVec3::Z, out.extend(0.0)],
                }
            };
            drawn_on(body, number, &round(2.0 * value, false), &which);
            let middle = if wall == TRAPEZOID_SHORT_WALL {
                0.5
            } else {
                2.0
            };
            met_from_outside(body, number, &round(middle, true), &which);
        } else {
            faces_toward(
                body,
                number,
                (out.extend(0.0) + DVec3::Z).normalize(),
                &which,
            );
        }
        corners_are(
            body,
            wall,
            &[
                from.extend(0.0),
                to.extend(0.0),
                to.extend(top - value),
                from.extend(top - value),
            ],
            &format!("{what}: wall {wall}, cut down by the value"),
        );
    }
    do_not_meet(body, [laid[0], laid[2]], what);
    kept_every_face_and_laid(part, body, &laid, what);
    corners_are(
        body,
        TOP,
        &trapezoid_top(corners, value),
        &format!("{what}: the top, set back by the value from all three edges"),
    );
    let mitre = (90.0 - TRAPEZOID_LEAN_DEGREES / 2.0)
        .to_radians()
        .tan()
        .recip();
    let run_on = TRAPEZOID_LEAN_DEGREES.to_radians().tan().recip();
    let (first, second) = if fillet {
        (
            value * value * (1.0 - PI / 4.0),
            value.powi(3) * (5.0 / 3.0 - PI / 2.0),
        )
    } else {
        (value * value / 2.0, value.powi(3) / 3.0)
    };
    let strips = TRAPEZOID_SHORT + 2.0 * TRAPEZOID_SIDE;
    let taken_off = strips * first - (2.0 * mitre + run_on) * second;
    let area = (c1.x - c0.x + c2.x - c3.x) / 2.0 * (c2.y - c1.y);
    let promised = area * top - taken_off;
    let drawn = if fillet {
        DRAWN * reach_of(part) * PI / 2.0 * value * (strips + 4.0 * value)
    } else {
        EXACT * promised
    };
    sound_drawn_within(body, promised, drawn, what);
}

#[test]
fn a_fillet_and_a_chamfer_on_a_trapezoid_s_short_top_edge_and_its_two_sides_named_between_them_asked_past_where_its_set_back_closes_take_the_value_at_which_it_closes()
 {
    let closing = trapezoid_closing();

    for ((kind, fillet), at) in [("fillet", true), ("chamfer", false)]
        .into_iter()
        .flat_map(|kind| [DVec2::ZERO, TRAPEZOID_MOVED].map(|at| (kind, at)))
    {
        let part = trapezoid_at(at);
        let names = trapezoid_edges(&part);
        let laid = [0, 1, 2].map(|rank| part.faces_end() + rank);
        let hair = 20.0 * 1e-9 * reach_of(&part);
        for asked in [TRAPEZOID_ASKED, closing + hair] {
            alone_each_takes(
                |name| {
                    if fillet {
                        part.fillet(name, asked).map(|single| single.taken)
                    } else {
                        part.chamfer(name, asked).map(|single| single.taken)
                    }
                },
                &names,
                asked,
                &format!("the {kind} on one of the trapezoid's three top edges, asked {asked}"),
            );
            let what = format!(
                "the {kind} on the trapezoid's left, short and right top edges, drawn at {at}, \
                 asked {asked}: the short one laid before the right, the two sides' set-backs \
                 crossing past its own once it closes"
            );

            let (reported, taken, body) = stepped(&part, &names, asked, fillet, &what);

            reduced_to_touching(reported, taken, [asked, closing], hair, &what);
            trapezoid_laid(&part, &body, (laid, at), taken, fillet, &what);
        }
    }
}

/// The part with the wall under the top edge from `from` to `to`, facing
/// `out`, cut back to lean in under that edge by `degrees` from upright, as
/// a part's next step cuts it: a box drawn on a sketch lying on the leaning
/// plane, wider than the part, raised off it out of the matter and taken
/// away. Gives the number the leaning wall answers to, the box's floor
/// numbered past the part's faces.
fn leaning_in(part: &Body, [from, to]: [DVec3; 2], out: DVec3, degrees: f64) -> (Body, usize) {
    let tilt = degrees.to_radians();
    let facing = out * tilt.cos() - DVec3::Z * tilt.sin();
    let u = (to - from).normalize();
    let frame = Frame {
        origin: (from + to) / 2.0,
        u,
        v: facing.cross(u),
    };
    let wider = Contour::rectangle(DVec2::splat(-60.0), DVec2::splat(60.0));
    let tool = part
        .tool_raised(&profile(wider), frame, facing * 40.0)
        .expect("the tool raises");
    let wall = part.faces_end();
    (part.difference(&tool).expect("the cut is built"), wall)
}

/// How far the drafted trapezoid's three walls lean in under its top edges.
const DRAFT_DEGREES: f64 = 10.0;

/// The trapezoid of [`trapezoid_corners_at`] the origin, its left, short and
/// right walls each leaning in by [`DRAFT_DEGREES`] under its top edge, its
/// top and its long wall as they were: the part, and the numbers its three
/// leaning walls answer to, in that order.
fn drafted_trapezoid() -> (Body, [usize; 3]) {
    let [c0, c1, c2, c3] = trapezoid_corners_at(DVec2::ZERO);
    let mut part = trapezoid_at(DVec2::ZERO);
    let mut walls = [0; 3];
    for ((from, to), wall) in [(c3, c0), (c0, c1), (c1, c2)].into_iter().zip(&mut walls) {
        let (_, out) = trapezoid_edge(from, to);
        let up = |at: DVec2| at.extend(TRAPEZOID_HEIGHT);
        (part, *wall) = leaning_in(&part, [up(from), up(to)], out.extend(0.0), DRAFT_DEGREES);
    }
    (part, walls)
}

/// How far along the top a fillet of one sets back an edge of the drafted
/// trapezoid's top: the cotangent of half the edge's angle, 90 degrees less
/// the draft, which is `(1 + sin δ) / cos δ`; a chamfer's is one.
fn drafted_per(fillet: bool) -> f64 {
    let draft = DRAFT_DEGREES.to_radians();
    if fillet {
        (1.0 + draft.sin()) / draft.cos()
    } else {
        1.0
    }
}

/// The drafted trapezoid's matter, and what a step at `value` on its three
/// top edges takes off it. At a depth `t` under the top, the section is the
/// trapezoid with its three drafted sides set in by `s = t tan δ`, holding
/// `A(s) = A₀ − 50 s + β s²` — the strips along the three sides, less
/// `cot 47.5°` at each mitre and `cot 85° / 2` where a strip runs on to the
/// long side, `β` their sum. The cut sets the three sides in by a width `w`
/// more, taking `A(s) − A(s + w) = 50 w − β (2 s w + w²)`. A chamfer's flat
/// leaves the top `d` in and the wall `d` down it, so `w = d − t / cos δ`
/// down to `d cos δ`; a fillet's round, its axis `r p` in and `r` down,
/// leaves `w = r p − √(r² − (r − t)²) − s` down to `r (1 + sin δ)`, summed
/// over its turn, `t = r (1 − cos θ)`, by Simpson's rule.
fn drafted_matter(value: f64, fillet: bool) -> (f64, f64) {
    let [c0, c1, c2, c3] = trapezoid_corners_at(DVec2::ZERO);
    let draft = DRAFT_DEGREES.to_radians();
    let lean = TRAPEZOID_LEAN_DEGREES.to_radians();
    let beta = 2.0
        * (90.0 - TRAPEZOID_LEAN_DEGREES / 2.0)
            .to_radians()
            .tan()
            .recip()
        + lean.tan().recip();
    let strips = TRAPEZOID_SHORT + 2.0 * TRAPEZOID_SIDE;
    let area = (c1.x - c0.x + c2.x - c3.x) / 2.0 * c2.y;
    let height = TRAPEZOID_HEIGHT;
    let whole = area * height - strips * draft.tan() * height * height / 2.0
        + beta * draft.tan().powi(2) * height.powi(3) / 3.0;
    let lost = |s: f64, w: f64| strips * w - beta * (2.0 * s * w + w * w);
    let simpson = |f: &dyn Fn(f64) -> f64, end: f64| {
        let steps = 2000;
        let step = end / steps as f64;
        (0..=steps)
            .map(|rank| {
                let weight = match rank {
                    0 => 1.0,
                    last if last == steps => 1.0,
                    odd if odd % 2 == 1 => 4.0,
                    _ => 2.0,
                };
                weight * f(rank as f64 * step)
            })
            .sum::<f64>()
            * step
            / 3.0
    };
    let taken = if fillet {
        let (r, p) = (value, drafted_per(true));
        simpson(
            &|turn: f64| {
                let t = r * (1.0 - turn.cos());
                let s = t * draft.tan();
                let w = r * p - r * turn.sin() - s;
                lost(s, w) * r * turn.sin()
            },
            PI / 2.0 + draft,
        )
    } else {
        simpson(
            &|t: f64| lost(t * draft.tan(), value - t / draft.cos()),
            value * draft.cos(),
        )
    };
    (whole, taken)
}

#[test]
fn a_fillet_and_a_chamfer_on_a_drafted_trapezoid_s_short_top_edge_and_its_two_sides_named_between_them_asked_past_where_its_set_back_closes_take_the_value_whose_set_back_closes_it()
 {
    let (part, walls) = drafted_trapezoid();
    let names = walls.map(|wall| edge_between(&part, TOP, wall));
    let laid = [0, 1, 2].map(|rank| part.faces_end() + rank);
    let hair = 20.0 * 1e-9 * reach_of(&part);
    let [c0, c1, c2, c3] = trapezoid_corners_at(DVec2::ZERO);
    let draft = DRAFT_DEGREES.to_radians();
    let top = TRAPEZOID_HEIGHT;
    let (whole, _) = drafted_matter(0.0, false);
    assert!(
        (part.volume() - whole).abs() <= EXACT * whole,
        "the drafted trapezoid holds {}, where arithmetic promised {whole}",
        part.volume()
    );

    for (kind, fillet) in [("fillet", true), ("chamfer", false)] {
        let per = drafted_per(fillet);
        let closing = trapezoid_closing() / per;
        for asked in [TRAPEZOID_ASKED, closing + hair] {
            alone_each_takes(
                |name| {
                    if fillet {
                        part.fillet(name, asked).map(|single| single.taken)
                    } else {
                        part.chamfer(name, asked).map(|single| single.taken)
                    }
                },
                &names,
                asked,
                &format!("the {kind} on one of the drafted trapezoid's top edges, asked {asked}"),
            );
            let what = format!(
                "the {kind} on the drafted trapezoid's left, short and right top edges, asked \
                 {asked}: the short one's set-back, {per} of the value along the top, closes at \
                 {closing}"
            );

            let (reported, value, body) = stepped(&part, &names, asked, fillet, &what);

            reduced_to_touching(reported, value, [asked, closing], hair, &what);
            for ((from, to), number) in [(c3, c0), (c0, c1), (c1, c2)].into_iter().zip(laid) {
                let (along, out) = trapezoid_edge(from, to);
                let leaning = out.extend(0.0) * draft.cos() - DVec3::Z * draft.sin();
                let which = format!("{what}: face {number}");
                if fillet {
                    let length = (to - from).length();
                    let round = |start: DVec2, length: f64| Round {
                        start: (start - out * value * per).extend(top - value),
                        along: along.extend(0.0),
                        length,
                        radius: value,
                        facing: [DVec3::Z, leaning],
                    };
                    let past = 2.0 * value;
                    drawn_on(
                        &body,
                        number,
                        &round(from - along * past, length + 2.0 * past),
                        &which,
                    );
                    let middle = if number == laid[1] { 0.5 } else { 2.0 };
                    met_from_outside(
                        &body,
                        number,
                        &round(from + along * (length / 2.0 - middle), 2.0 * middle),
                        &which,
                    );
                } else {
                    faces_toward(&body, number, (DVec3::Z + leaning).normalize(), &which);
                }
            }
            do_not_meet(&body, [laid[0], laid[2]], &what);
            kept_every_face_and_laid(&part, &body, &laid, &what);
            corners_are(
                &body,
                TOP,
                &trapezoid_top([c0, c1, c2, c3], value * per),
                &format!(
                    "{what}: the top, set back {} along it from all three edges",
                    value * per
                ),
            );
            let (whole, taken_off) = drafted_matter(value, fillet);
            let promised = whole - taken_off;
            let drawn = if fillet {
                DRAWN * reach_of(&part) * PI / 2.0 * value * (60.0 + 4.0 * value)
            } else {
                EXACT * promised
            };
            sound_drawn_within(&body, promised, drawn, &what);
        }
    }
}

/// How far the overhanging hexagon's middle wall leans in under its top
/// edge: steep enough that a fillet's set-back along the top, the radius
/// over the tangent of half the edge's angle, outruns where the two edges
/// beside it would cross.
const OVERHANG_DEGREES: f64 = 40.0;

/// What is asked of the three top edges in a row of the overhanging
/// hexagon: short of what the overhanging wall alone gives.
const OVERHANG_ASKED: f64 = 2.0;

/// Holds that a body is exact, its surface closed and no face through
/// another: the rules every solid keeps, short of the matter its triangles
/// enclose. A round ending on a wall that overhangs is drawn further from
/// its surface than the drawing's tolerance — a fillet of 2 on the
/// hexagon's front top edge alone, its next wall leaning in 20 degrees,
/// encloses 0.145 less than it holds, past the 0.063 the tolerance allows
/// its round — which is the drawing's business (#824), not this step's.
fn closed_and_uncrossed(body: &Body, what: &str) {
    assert!(
        body.is_exact(),
        "{what}: the exact kernel computed the body"
    );
    let triangles = body.triangles();
    assert_eq!(closed(&triangles), Ok(()), "{what}");
    assert_eq!(uncrossed(&triangles), Ok(()), "{what}");
}

/// The corners of the polygon `corners` once its sides are set in by
/// `insets`, the side from each corner to the next by the inset of that
/// rank: where each two set-in sides next to each other cross.
fn set_in(corners: &[DVec2], insets: &[f64]) -> Vec<DVec2> {
    let count = corners.len();
    let side = |rank: usize| {
        let (from, to) = (corners[rank], corners[(rank + 1) % count]);
        let (along, out) = trapezoid_edge(from, to);
        (from - out * insets[rank], along)
    };
    (0..count)
        .map(|rank| crossing(side((rank + count - 1) % count), side(rank)))
        .collect()
}

#[test]
fn a_fillet_and_a_chamfer_on_three_top_edges_in_a_row_of_a_hexagonal_prism_whose_middle_wall_overhangs_take_the_value_asked_in_every_order_its_set_back_outrunning_where_the_outer_two_would_cross()
 {
    let [_, from, to, _, _, _] = hexagon_corners();
    let (_, out) = trapezoid_edge(from, to);
    let (part, wall) = leaning_in(
        &hexagonal_prism(),
        [from, to].map(|at| at.extend(HEXAGON_HEIGHT)),
        out.extend(0.0),
        OVERHANG_DEGREES,
    );
    let edges = [
        edge_between(&part, TOP, HEXAGON_FRONT),
        edge_between(&part, TOP, wall),
        edge_between(&part, TOP, HEXAGON_ACROSS),
    ];
    let value = OVERHANG_ASKED;

    for (kind, fillet) in [("fillet", true), ("chamfer", false)] {
        let per = if fillet {
            (45.0 - OVERHANG_DEGREES / 2.0).to_radians().tan().recip()
        } else {
            1.0
        };
        let mut laid_in_every_order: Vec<(String, Body, [usize; 3])> = Vec::new();
        for order in ORDERS_OF_THREE {
            let names: Vec<EdgeName> = order.iter().map(|&rank| edges[rank].clone()).collect();
            let laid: Vec<usize> = (0..order.len())
                .map(|named| part.faces_end() + named)
                .collect();
            let number_of = |rank: usize| {
                laid[order
                    .iter()
                    .position(|&named| named == rank)
                    .expect("named")]
            };
            let what = format!(
                "the {kind} on three top edges in a row of the hexagon whose middle wall \
                 overhangs {OVERHANG_DEGREES} degrees, named {order:?}, asked {value}: the \
                 middle one's set-back along the top, {per} of the value, runs ahead of where \
                 the outer two's set-backs would cross"
            );

            let (reported, taken, body) = stepped(&part, &names, value, fillet, &what);

            taken_as_asked(reported, taken, value, &what);
            for rank in [0, 2] {
                let (number, turn) = (number_of(rank), rank as f64 * PI / 3.0);
                let which = format!("{what}: face {number}");
                if fillet {
                    let reach = HEXAGON_SIDE / 2.0 + value / 3.0f64.sqrt();
                    drawn_on(&body, number, &hexagon_round_to(value, turn, reach), &which);
                } else {
                    let (_, facing) = hexagon_flat_to(value, turn, 0.0);
                    faces_toward(&body, number, facing, &which);
                }
            }
            do_not_meet(&body, [number_of(0), number_of(2)], &what);
            kept_every_face_and_laid(&part, &body, &laid, &what);
            let insets = [value, value * per, value, 0.0, 0.0, 0.0];
            corners_are(
                &body,
                TOP,
                &set_in(&hexagon_corners(), &insets)
                    .into_iter()
                    .map(|corner| corner.extend(HEXAGON_HEIGHT))
                    .collect::<Vec<_>>(),
                &format!("{what}: the top, set back from the three edges"),
            );
            closed_and_uncrossed(&body, &what);
            laid_in_every_order.push((what, body, [0, 1, 2].map(number_of)));
        }
        the_same_body_in_every_order(&laid_in_every_order);
    }
}

/// Holds that the bodies a step on three edges in a row laid in several
/// orders are one body: the same matter, within [`EXACT`] of it, and on each
/// edge of the row a face flat in all of them or in none, a flat with the
/// same corners, within [`PLACE`]. Each body comes with what it is and the
/// numbers of the faces laid on the row's edges, by their rank in the row.
fn the_same_body_in_every_order(laid_in_every_order: &[(String, Body, [usize; 3])]) {
    let (first_what, first, first_numbers) = &laid_in_every_order[0];
    for (what, body, numbers) in &laid_in_every_order[1..] {
        let which = format!("{first_what}, and {what}");
        assert!(
            (body.volume() - first.volume()).abs() <= EXACT * first.volume(),
            "{which}: the two hold {} and {}",
            first.volume(),
            body.volume()
        );
        for rank in 0..3 {
            let [mine, theirs] = [numbers[rank], first_numbers[rank]];
            assert_eq!(
                body.is_flat(mine),
                first.is_flat(theirs),
                "{which}: the face laid on the edge of rank {rank} in the row is flat in one \
                 and not in the other"
            );
            if let (Some(plane), Some(other)) = (body.plane_of(mine), first.plane_of(theirs)) {
                let among = |corners: &[DVec3], point: &DVec3| {
                    corners.iter().any(|corner| corner.distance(*point) < PLACE)
                };
                assert!(
                    plane
                        .corners
                        .iter()
                        .all(|point| among(&other.corners, point))
                        && other
                            .corners
                            .iter()
                            .all(|point| among(&plane.corners, point)),
                    "{which}: the flat laid on the edge of rank {rank} in the row has the \
                     corners {:?} in one and {:?} in the other",
                    plane.corners,
                    other.corners
                );
            }
        }
    }
}

/// A prism of the outline `corners`, raised `height`: its floor numbered
/// nought, its top one, and its walls from two, the first from the first
/// corner to the second, each after it from where the one before ends.
fn prism_of(corners: &[DVec2], height: f64) -> Body {
    raised_with(
        &Body::default(),
        Contour::straight(corners.to_vec()),
        height,
    )
}

/// A hexagon whose first two sides are ten long and meet as the regular
/// one's do, but whose third side turns only a twelfth of a turn from the
/// second, where the regular one turns a sixth, and runs `third` long; closed
/// by a wall up to a back side at `back` across the y axis, and that side.
/// Its second side runs along the x axis from the origin.
fn uneven_hexagon(third: f64, back: f64) -> [DVec2; 6] {
    let root = 3.0f64.sqrt();
    [
        DVec2::new(-5.0, 5.0 * root),
        DVec2::ZERO,
        DVec2::new(10.0, 0.0),
        DVec2::new(10.0 + third * root / 2.0, third / 2.0),
        DVec2::new(10.0 + third / 2.0, back),
        DVec2::new(-5.0, back),
    ]
}

/// Three top edges in a row of a prism — on its walls two, three and four —
/// stepped in every order: what the row is, the prism's outline and height,
/// the outer wall leaning in under its top edge and by how many degrees,
/// where one does, by its rank in the row, and what is asked.
struct Row {
    called: &'static str,
    corners: [DVec2; 6],
    height: f64,
    leaning: Option<(usize, f64)>,
    asked: f64,
}

/// What a step gives: the value it took and the body it laid, or why it
/// declined.
type Outcome = Result<(f64, Body), Declined>;

/// What each order a step's edges are named in gives, by that order.
type InEveryOrder = Vec<([usize; 3], Outcome)>;

/// The step of a chamfer, or a fillet if `fillet`, on one edge or several of
/// `part`, at `asked`.
fn step_on(part: &Body, names: &[EdgeName], asked: f64, fillet: bool) -> Outcome {
    if fillet {
        (part.fillet_edges(names, asked)).map(|done| (done.taken, done.body))
    } else {
        (part.chamfer_edges(names, asked)).map(|done| (done.taken, done.body))
    }
}

/// What each order of [`ORDERS_OF_THREE`] gives for a chamfer, or a fillet if
/// `fillet`, on the three top edges of `row`; the part they are laid on, and
/// the three edges by their rank in the row.
fn stepped_in_every_order(row: &Row, fillet: bool) -> (Body, [EdgeName; 3], InEveryOrder) {
    let raised = prism_of(&row.corners, row.height);
    let mut walls = IN_A_ROW;
    let part = match row.leaning {
        Some((outer, degrees)) => {
            let (from, to) = (row.corners[outer], row.corners[outer + 1]);
            let (_, out) = trapezoid_edge(from, to);
            let (part, wall) = leaning_in(
                &raised,
                [from, to].map(|at| at.extend(row.height)),
                out.extend(0.0),
                degrees,
            );
            walls[outer] = wall;
            part
        }
        None => raised,
    };
    let edges = walls.map(|wall| edge_between(&part, TOP, wall));
    let outcomes = (ORDERS_OF_THREE.iter())
        .map(|&order| {
            let names: Vec<EdgeName> = order.iter().map(|&rank| edges[rank].clone()).collect();
            (order, step_on(&part, &names, row.asked, fillet))
        })
        .collect();
    (part, edges, outcomes)
}

/// Says, for the chamfer or the fillet on `row`, whether every order gave one
/// outcome — the same value and the same matter, within [`PLACE`] and
/// [`EXACT`], or the same decline — listing what each order gave where they
/// do not, or where `at_the_least_alone` and they did not all build at the
/// least value any of the three edges takes alone; and where they agree and
/// the step is built, holds each body sound and all of them one body.
fn one_outcome_in_every_order(row: &Row, fillet: bool, at_the_least_alone: bool) -> Option<String> {
    let kind = if fillet { "fillet" } else { "chamfer" };
    let called = format!("the {kind} of {} on {}", row.asked, row.called);
    let (part, edges, outcomes) = stepped_in_every_order(row, fillet);
    let told = |outcomes: &[([usize; 3], Outcome)]| -> String {
        (outcomes.iter())
            .map(|(order, laid)| match laid {
                Ok((taken, body)) => {
                    format!("  named {order:?}, took {taken}, holding {}", body.volume())
                }
                Err(declined) => format!("  named {order:?}, declined {declined:?}"),
            })
            .collect::<Vec<_>>()
            .join("\n")
    };
    let agree = |one: &Outcome, other: &Outcome| match (one, other) {
        (Ok((one, body)), Ok((other, theirs))) => {
            (one - other).abs() <= PLACE
                && (body.volume() - theirs.volume()).abs() <= EXACT * theirs.volume()
        }
        (Err(one), Err(other)) => one == other,
        _ => false,
    };
    if !(outcomes.iter()).all(|(_, laid)| agree(laid, &outcomes[0].1)) {
        return Some(format!("{called}:\n{}", told(&outcomes)));
    }
    if at_the_least_alone {
        let mut least = f64::INFINITY;
        for edge in &edges {
            match step_on(&part, std::slice::from_ref(edge), row.asked, fillet) {
                Ok((taken, _)) => least = least.min(taken),
                Err(declined) => {
                    return Some(format!("{called}: {edge:?} alone declined, {declined:?}"));
                }
            }
        }
        let at_least = |(_, laid): &([usize; 3], Outcome)| {
            laid.as_ref()
                .is_ok_and(|(taken, _)| (taken - least).abs() <= PLACE)
        };
        if !outcomes.iter().all(at_least) {
            return Some(format!(
                "{called}: every order builds at {least}, the least any of the three takes \
                 alone, each edge's own limit holding whatever its rank:\n{}",
                told(&outcomes)
            ));
        }
    }
    let mut laid_in_every_order: Vec<(String, Body, [usize; 3])> = Vec::new();
    for (order, laid) in outcomes {
        let Ok((_, body)) = laid else {
            return None;
        };
        let laid: Vec<usize> = (0..order.len())
            .map(|named| part.faces_end() + named)
            .collect();
        let number_of = |rank: usize| {
            laid[order
                .iter()
                .position(|&named| named == rank)
                .expect("named")]
        };
        let what = format!("{called}, named {order:?}");
        closed_and_uncrossed(&body, &what);
        laid_in_every_order.push((what, body, [0, 1, 2].map(number_of)));
    }
    the_same_body_in_every_order(&laid_in_every_order);
    None
}

#[test]
fn a_fillet_and_a_chamfer_on_three_top_edges_in_a_row_whose_outer_edge_alone_reaches_past_the_middle_one_or_whose_outer_wall_overhangs_take_in_every_order_the_least_any_of_the_three_takes_alone()
 {
    let rows = [
        Row {
            called: "three top edges in a row of a hexagon whose third side turns a twelfth \
                     of a turn, 12 tall, that side's wall leaning in 40 degrees",
            corners: uneven_hexagon(10.0, 20.0),
            height: 12.0,
            leaning: Some((2, 40.0)),
            asked: 5.0,
        },
        Row {
            called: "three top edges in a row of a hexagon whose third side turns a twelfth \
                     of a turn and runs 30 long, 12 tall, every wall upright",
            corners: uneven_hexagon(30.0, 40.0),
            height: 12.0,
            leaning: None,
            asked: 6.0,
        },
        Row {
            called: "three top edges in a row of a hexagon whose third side turns a twelfth \
                     of a turn and runs 30 long, 12 tall, that side's wall leaning in 20 \
                     degrees",
            corners: uneven_hexagon(30.0, 40.0),
            height: 12.0,
            leaning: Some((2, 20.0)),
            asked: 6.0,
        },
        Row {
            called: "three top edges in a row of the regular hexagon, 10 tall, its front wall \
                     leaning in 50 degrees, asked past where two chamfers' flats laid whole \
                     would meet in a line leaving through the leaning wall before the middle \
                     one",
            corners: hexagon_corners(),
            height: 2.0 * HEXAGON_HEIGHT,
            leaning: Some((0, 50.0)),
            asked: 8.0,
        },
        Row {
            called: "three top edges in a row of a hexagon whose third side turns a twelfth \
                     of a turn, 12 tall, its first wall leaning in 40 degrees",
            corners: uneven_hexagon(10.0, 20.0),
            height: 12.0,
            leaning: Some((0, 40.0)),
            asked: 6.0,
        },
        Row {
            called: "three top edges in a row of a hexagon whose third side turns a twelfth \
                     of a turn, 20 tall, its first wall leaning in 40 degrees",
            corners: uneven_hexagon(10.0, 20.0),
            height: 20.0,
            leaning: Some((0, 40.0)),
            asked: 6.0,
        },
        Row {
            called: "three top edges in a row of a hexagon whose third side turns a twelfth \
                     of a turn, 12 tall, that side's wall leaning in 40 degrees",
            corners: uneven_hexagon(10.0, 20.0),
            height: 12.0,
            leaning: Some((2, 40.0)),
            asked: 6.0,
        },
    ];
    let wrong: Vec<String> = (rows.iter())
        .flat_map(|row| [true, false].map(|fillet| one_outcome_in_every_order(row, fillet, true)))
        .flatten()
        .collect();
    assert!(
        wrong.is_empty(),
        "one value whatever the order the three edges are named in, the least any of them \
         takes alone:\n{}",
        wrong.join("\n")
    );
}

#[test]
fn a_chamfer_on_three_top_edges_in_a_row_whose_first_wall_leans_in_60_degrees_takes_one_value_or_declines_in_every_order_where_the_third_edge_alone_declines()
 {
    let row = Row {
        called: "three top edges in a row of a hexagon whose third side turns a twelfth of a \
                 turn, 12 tall, its first wall leaning in 60 degrees",
        corners: uneven_hexagon(10.0, 20.0),
        height: 12.0,
        leaning: Some((0, 60.0)),
        asked: 6.0,
    };
    let wrong = one_outcome_in_every_order(&row, false, false);
    assert!(
        wrong.is_none(),
        "one value, or one decline, whatever the order the three edges are named in:\n{}",
        wrong.unwrap_or_default()
    );
}

#[test]
fn a_fillet_and_a_chamfer_on_three_edges_in_a_chain_whose_outer_two_reach_the_middle_one_through_its_two_faces_take_the_value_asked()
 {
    let block = block();
    let trapezoid = trapezoid_at(DVec2::ZERO);
    let chains = [
        (
            "the block's top front edge, its top right edge and its right wall's back edge",
            &block,
            [
                edge_between(&block, TOP, FRONT),
                edge_between(&block, TOP, RIGHT),
                edge_between(&block, RIGHT, BACK),
            ],
            MORE_THAN_HALF,
        ),
        (
            "the trapezoid's left top edge, its short top edge and the upright at the short \
             edge's right end",
            &trapezoid,
            [
                edge_between(&trapezoid, TOP, TRAPEZOID_LEFT),
                edge_between(&trapezoid, TOP, TRAPEZOID_SHORT_WALL),
                edge_between(&trapezoid, TRAPEZOID_SHORT_WALL, TRAPEZOID_RIGHT),
            ],
            TRAPEZOID_ASKED,
        ),
    ];

    for (chain, part, names, value) in chains {
        let laid = [0, 1, 2].map(|rank| part.faces_end() + rank);
        for (kind, fillet) in [("fillet", true), ("chamfer", false)] {
            let what = format!(
                "the {kind} on {chain}, asked {value}: the first ends on the middle one's top \
                 face, the last on its wall"
            );

            let (reported, taken, body) = stepped(part, &names, value, fillet, &what);

            taken_as_asked(reported, taken, value, &what);
            do_not_meet(&body, [laid[0], laid[2]], &what);
            kept_every_face_and_laid(part, &body, &laid, &what);
            let promised = body.volume();
            sound_drawn_within(
                &body,
                promised,
                DRAWN * reach_of(part) * 3.0 * PI * value * 30.0,
                &what,
            );
        }
    }
}

/// A block 30 by 20 by 10 with another, 10 by 10 by 10, standing on the
/// middle of its top: the lower block's top numbered one, its walls, front,
/// right, back and left, two to five, and the upper block's walls eight to
/// eleven in the same turn, its floor gone into the lower block's top. Its
/// feet, where its walls meet that top, meet inwards, and each ends on the
/// walls beside its own.
fn stacked() -> Body {
    let lower = raised_with(
        &Body::default(),
        Contour::rectangle(DVec2::ZERO, DVec2::new(30.0, 20.0)),
        10.0,
    );
    let tool = lower
        .tool_raised(
            &profile(Contour::rectangle(
                DVec2::new(10.0, 5.0),
                DVec2::new(20.0, 15.0),
            )),
            Frame {
                origin: DVec3::Z * 10.0,
                ..ground()
            },
            DVec3::Z * 10.0,
        )
        .expect("the standing block raises");
    lower.union(&tool).expect("the standing block joins")
}
const STACKED_VOLUME: f64 = 30.0 * 20.0 * 10.0 + 10.0 * 10.0 * 10.0;

/// [`stacked`] with its four uprights chamfered by `value` in an earlier
/// step, each flat stopping flush on the lower block's top: the four flats,
/// front right, back right, back left and front left, numbered twelve to
/// fifteen. Its eight feet, the four walls' and the four flats', meet inwards
/// round an octagon on the lower block's top, four sides `10 − 2·value` and
/// four `value·√2`, its eight corners of 135°.
fn stacked_with_its_uprights_chamfered(value: f64) -> Body {
    let stacked = stacked();
    let uprights: Vec<EdgeName> = [[8, 9], [9, 10], [10, 11], [11, 8]]
        .iter()
        .map(|&[first, second]| edge_between(&stacked, first, second))
        .collect();
    let chamfered = stacked
        .chamfer_edges(&uprights, value)
        .unwrap_or_else(|declined| panic!("the four uprights are chamfered: {declined:?}"));
    taken_as_asked(
        chamfered.asked,
        chamfered.taken,
        value,
        "the four uprights chamfered",
    );
    chamfered.body
}

/// The eight feet round the octagon of [`stacked_with_its_uprights_chamfered`],
/// by the faces each lies between: the four walls' feet, front, right, back
/// and left, then the four flats' feet, front right, back right, back left
/// and front left.
const OCTAGON_FEET: [[usize; 2]; 8] = [
    [TOP, 8],
    [TOP, 9],
    [TOP, 10],
    [TOP, 11],
    [TOP, 12],
    [TOP, 13],
    [TOP, 14],
    [TOP, 15],
];

/// The orders the octagon's eight feet are named in, by their places in
/// [`OCTAGON_FEET`]: as given, reversed, and in turn round the octagon from
/// the front, each wall's foot followed by the flat's at its end.
const OCTAGON_ORDERS: [[usize; 8]; 3] = [
    [0, 1, 2, 3, 4, 5, 6, 7],
    [7, 6, 5, 4, 3, 2, 1, 0],
    [0, 4, 1, 5, 2, 6, 3, 7],
];

/// What [`stacked_with_its_uprights_chamfered`] at `value` holds once the
/// octagon's eight feet are filled at `value`: 7000 less the four flats'
/// `10·value²/2`, and the frustum between the octagon and its offset `value`
/// out, less the standing block — the octagon's length times `value²/2`, and
/// `tan(22.5°)·value³/3` at each of its eight corners.
fn octagon_filled(value: f64) -> f64 {
    let d = value;
    let around = 40.0 - 8.0 * d + 4.0 * 2f64.sqrt() * d;
    STACKED_VOLUME - 4.0 * 10.0 * d * d / 2.0
        + around * d * d / 2.0
        + 8.0 * (PI / 8.0).tan() * d.powi(3) / 3.0
}

#[test]
fn a_chamfer_on_the_eight_feet_round_a_standing_block_s_chamfered_uprights_takes_the_value_asked_their_set_backs_running_no_further_than_their_mitres()
 {
    for value in [1.0, 2.5] {
        let part = stacked_with_its_uprights_chamfered(value);
        let names = OCTAGON_FEET.map(|[first, second]| edge_between(&part, first, second));
        let laid: Vec<usize> = (0..8).map(|rank| part.faces_end() + rank).collect();
        for order in OCTAGON_ORDERS {
            let what = format!(
                "the chamfer of {value} on the eight feet round the standing block's chamfered \
                 uprights, named {order:?}"
            );
            let list: Vec<EdgeName> = order.iter().map(|&foot| names[foot].clone()).collect();

            let step = part
                .chamfer_edges(&list, value)
                .unwrap_or_else(|declined| panic!("{what}: declined, {declined:?}"));

            taken_as_asked(step.asked, step.taken, value, &what);
            kept_every_face_and_laid(&part, &step.body, &laid, &what);
            let promised = octagon_filled(value);
            sound_drawn_within(&step.body, promised, EXACT * promised, &what);
        }
    }
}

/// A pair of [`stacked`]'s edges that meet at no corner and whose
/// set-backs' lines would cross on the lower block's top past where one of
/// them ends: a foot of the standing block, filled, and an edge of the lower
/// block's top beyond the wall that foot ends on, cut. The faces each lies
/// between, in the order the step names them, how long each is, which of
/// the two is the foot, and what to call them.
struct FootAndFarEdge {
    named: [[usize; 2]; 2],
    lengths: [f64; 2],
    foot: usize,
    called: &'static str,
}

const FOOT_AND_FAR_EDGE: [FootAndFarEdge; 4] = [
    FootAndFarEdge {
        named: [[TOP, 8], [TOP, RIGHT]],
        lengths: [10.0, 20.0],
        foot: 0,
        called: "the standing block's front foot and the lower block's top right edge",
    },
    FootAndFarEdge {
        named: [[TOP, RIGHT], [TOP, 8]],
        lengths: [20.0, 10.0],
        foot: 1,
        called: "the lower block's top right edge and the standing block's front foot",
    },
    FootAndFarEdge {
        named: [[TOP, 8], [TOP, LEFT]],
        lengths: [10.0, 20.0],
        foot: 0,
        called: "the standing block's front foot and the lower block's top left edge",
    },
    FootAndFarEdge {
        named: [[TOP, 9], [TOP, FRONT]],
        lengths: [10.0, 30.0],
        foot: 0,
        called: "the standing block's right foot and the lower block's top front edge",
    },
];

#[test]
fn a_chamfer_and_a_fillet_on_a_standing_block_s_foot_and_an_edge_of_the_lower_block_s_top_beyond_its_end_take_the_value_asked_the_foot_s_set_back_running_no_further_than_the_wall_it_ends_on()
 {
    let part = stacked();
    let laid = [0, 1].map(|rank| part.faces_end() + rank);
    for FootAndFarEdge {
        named,
        lengths,
        foot,
        called,
    } in FOOT_AND_FAR_EDGE
    {
        let names = named.map(|[first, second]| edge_between(&part, first, second));
        for value in [1.0, 2.5] {
            for (kind, fillet) in [("fillet", true), ("chamfer", false)] {
                let what = format!("the {kind} of {value} on {called}");

                let (reported, taken, body) = stepped(&part, &names, value, fillet, &what);

                taken_as_asked(reported, taken, value, &what);
                kept_every_face_and_laid(&part, &body, &laid, &what);
                do_not_meet(&body, laid, &what);
                let section = if fillet {
                    value * value * (1.0 - PI / 4.0)
                } else {
                    value * value / 2.0
                };
                let promised =
                    STACKED_VOLUME + section * lengths[foot] - section * lengths[1 - foot];
                let drawn = if fillet {
                    DRAWN * reach_of(&part) * PI / 2.0 * value * (lengths[0] + lengths[1])
                } else {
                    EXACT * promised
                };
                sound_drawn_within(&body, promised, drawn, &what);
            }
        }
    }
}

/// An L 10 high whose corner block under its inner corner is cut away by a
/// slanted wall: an arm from (12, 0) to (30, 6) along the front, a stem from
/// (0, 6) to (12, 14) at its left, and between them the slant from (0, 9) to
/// (12, 0), its line `3x + 4y = 36`. Its walls are numbered from the arm's
/// front round: the front two, the arm's right end three, the arm's back
/// four, the stem's right side five, the stem's back six, its left seven and
/// the slant eight. The arm's back and the stem's right side meet inwards in
/// the L's inner corner, (12, 6); the slant and the front meet outwards, at
/// 143.13°, along the upright at (12, 0).
fn slanted_ell() -> Body {
    raised_with(
        &Body::default(),
        Contour::straight(vec![
            DVec2::new(12.0, 0.0),
            DVec2::new(30.0, 0.0),
            DVec2::new(30.0, 6.0),
            DVec2::new(12.0, 6.0),
            DVec2::new(12.0, 14.0),
            DVec2::new(0.0, 14.0),
            DVec2::new(0.0, 9.0),
        ]),
        10.0,
    )
}
const SLANTED_ELL_ARM_BACK: usize = 4;
const SLANTED_ELL_STEM_RIGHT: usize = 5;
const SLANTED_ELL_SLANT: usize = 8;

/// The three edges of [`slanted_ell`] named together, by the faces each lies
/// between: the top edge of the arm's back, the upright where the slant meets
/// the front, and the top edge of the stem's right side.
const ARM_BACK_UPRIGHT_STEM_RIGHT: [[usize; 2]; 3] = [
    [TOP, SLANTED_ELL_ARM_BACK],
    [FRONT, SLANTED_ELL_SLANT],
    [TOP, SLANTED_ELL_STEM_RIGHT],
];

/// Every order three edges can be named in.
const ORDERS_OF_THREE: [[usize; 3]; 6] = [
    [0, 1, 2],
    [0, 2, 1],
    [1, 0, 2],
    [1, 2, 0],
    [2, 0, 1],
    [2, 1, 0],
];

/// For a chamfer and a fillet on [`ARM_BACK_UPRIGHT_STEM_RIGHT`]: a value
/// short of where the blends meet, which every order takes whole; the value
/// at which they meet; and two values asked past it. The two top edges, cut
/// beside the concave inner upright left out, each stop flush on the other's
/// wall and run on past it into their mitre, which reaches the slant. Past
/// the stem's plane the arm's back edge's set-back, `6 − v` across the top,
/// meets on the slant's top edge the upright's, which reaches `0.6·s` up
/// that edge for a set-back `s` along it from the front: a chamfer's `s` is
/// its distance, meeting at `0.6·d = 6 − d`, 3.75; a fillet's is
/// `r·cot(α/2)`, a third of the radius where the slant and the front meet at
/// 143.13°, meeting at `0.2·r = 6 − r`, 5.
const SLANTED_ELL_VALUES: [(&str, bool, f64, f64, [f64; 2]); 2] = [
    ("chamfer", false, 3.5, 3.75, [3.8, 4.0]),
    ("fillet", true, 4.8, 5.0, [5.2, 5.5]),
];

#[test]
fn a_fillet_and_a_chamfer_on_the_two_top_edges_at_an_l_s_inner_corner_and_an_upright_their_mitre_runs_into_take_the_value_at_which_they_meet_whatever_the_order()
 {
    let ell = slanted_ell();
    let names =
        ARM_BACK_UPRIGHT_STEM_RIGHT.map(|[first, second]| edge_between(&ell, first, second));
    let hair = 20.0 * 1e-9 * 30.0;
    for (kind, fillet, short, meeting, past) in SLANTED_ELL_VALUES {
        for asked in std::iter::once(short).chain(past) {
            let laid: Vec<(String, f64, f64, Body)> = ORDERS_OF_THREE
                .iter()
                .map(|order| {
                    let what = format!(
                        "the {kind} of {asked} on the arm's back top edge, the upright where \
                         the slant meets the front and the stem's right top edge, named \
                         {order:?}"
                    );
                    let list: Vec<EdgeName> =
                        order.iter().map(|&edge| names[edge].clone()).collect();
                    let (reported, taken, body) = stepped(&ell, &list, asked, fillet, &what);
                    (what, reported, taken, body)
                })
                .collect();
            for (what, reported, taken, _) in &laid {
                if asked < meeting {
                    taken_as_asked(*reported, *taken, asked, what);
                } else {
                    reduced_to_touching(*reported, *taken, [asked, meeting], hair, what);
                }
            }
            let promised = laid[0].3.volume();
            let drawn = if fillet {
                DRAWN * reach_of(&ell) * PI / 2.0 * asked * (18.0 + 8.0 + 10.0 + 2.0 * asked)
            } else {
                EXACT * promised
            };
            for (what, _, _, body) in &laid {
                sound_drawn_within(body, promised, drawn, what);
            }
        }
    }
}

/// Holds, for each order of `orders` and each kind, that the step on the
/// hexagon's top edges on the walls `walls`, named in that order, takes
/// `value` whole and leaves a sound body holding `promised` of its kind, its
/// top's corners `top`; or says, among `wrong`, what it took or why it
/// declined. A fillet is drawn within the tolerance over one round for each
/// edge, a side and a value long.
fn take_the_value_asked_in_every_order(
    walls: &[usize],
    orders: &[Vec<usize>],
    value: f64,
    (promised, top): (&dyn Fn(bool) -> f64, &[DVec3]),
    called: &str,
) -> Vec<String> {
    let prism = hexagonal_prism();
    let mut wrong = Vec::new();
    for order in orders {
        let names: Vec<EdgeName> = (order.iter())
            .map(|&rank| edge_between(&prism, TOP, walls[rank]))
            .collect();
        for (kind, fillet) in [("fillet", true), ("chamfer", false)] {
            let what = format!("the {kind} of {value} on {called}, named {order:?}");
            let laid = if fillet {
                prism
                    .fillet_edges(&names, value)
                    .map(|done| (done.asked, done.taken, done.body))
            } else {
                prism
                    .chamfer_edges(&names, value)
                    .map(|done| (done.asked, done.taken, done.body))
            };
            let (asked, taken, body) = match laid {
                Ok(laid) => laid,
                Err(reason) => {
                    wrong.push(format!("{what}: declined, {reason:?}"));
                    continue;
                }
            };
            if (asked - value).abs() > PLACE || (taken - value).abs() > PLACE {
                wrong.push(format!("{what}: asked {asked} and took {taken}"));
                continue;
            }
            let promised = promised(fillet);
            let drawn = if fillet {
                let area = walls.len() as f64 * PI / 2.0 * value * (HEXAGON_SIDE + value);
                DRAWN * reach_of(&prism) * area
            } else {
                EXACT * promised
            };
            sound_drawn_within(&body, promised, drawn, &what);
            corners_are(
                &body,
                TOP,
                top,
                &format!("{what}: the top, set back by the value from every edge of the list"),
            );
        }
    }
    wrong
}

#[test]
fn a_fillet_and_a_chamfer_on_three_top_edges_in_a_row_of_a_hexagonal_prism_asked_past_where_the_outer_two_would_cross_take_the_value_asked_in_every_order()
 {
    let value = HEXAGON_ASKED;
    let orders: Vec<Vec<usize>> = ORDERS_OF_THREE.iter().map(|order| order.to_vec()).collect();
    let wrong = take_the_value_asked_in_every_order(
        &IN_A_ROW,
        &orders,
        value,
        (
            &|fillet| hexagon_volume() - taken_off_in_a_row(value, fillet),
            &in_a_row_top(value),
        ),
        "three top edges in a row of the hexagon",
    );
    assert!(
        wrong.is_empty(),
        "three top edges in a row of the hexagon, asked {value}, past where the outer two's \
         set-backs would cross on the middle edge, {}: the value asked whatever the order, as \
         no two set-backs meet once all three are laid:\n{}",
        hexagon_touching(),
        wrong.join("\n")
    );
}

#[test]
fn a_fillet_and_a_chamfer_on_the_six_top_edges_of_a_hexagonal_prism_asked_past_where_alternate_edges_would_cross_take_the_value_asked_whatever_the_order()
 {
    let value = HEXAGON_ASKED;
    let orders = vec![
        vec![0, 1, 2, 3, 4, 5],
        vec![5, 4, 3, 2, 1, 0],
        vec![0, 3, 1, 4, 2, 5],
        vec![2, 0, 4, 1, 5, 3],
    ];
    let top = ring_top(value);
    let wrong = take_the_value_asked_in_every_order(
        &HEXAGON_WALLS,
        &orders,
        value,
        (
            &|fillet| hexagon_volume() - taken_off_the_ring(value, fillet),
            &top,
        ),
        "the hexagon's six top edges",
    );
    assert!(
        wrong.is_empty(),
        "the hexagon's six top edges, asked {value}, past where alternate edges' set-backs would \
         cross, {}: the value asked whatever the order, as no two set-backs meet once all six are \
         laid:\n{}",
        hexagon_touching(),
        wrong.join("\n")
    );
}

/// A plate raised 10 whose top is bounded by two sides on the lines
/// `−x·cos a + y·sin a = c` and its mirror, converging up the plan to an arc
/// of [`ARCH_RADIUS`] about the origin between them, and by a straight bottom
/// at [`ARCH_BOTTOM`]: the issue's fixture at `c` 7 and `a` 30°, and the one
/// nearer to diameters at `c` 3 and `a` 10°. Asked `below` the touch and
/// each of `past` beyond it.
struct Arched {
    distance: f64,
    degrees: f64,
    below: f64,
    past: [f64; 2],
}

const ARCH_RADIUS: f64 = 10.0;
const ARCH_BOTTOM: f64 = -5.0;
const ARCH_HEIGHT: f64 = 10.0;

/// The arched plate's faces, as [`raised_with`] numbers them: its bottom
/// wall two, its right side three, its arc four and its left side five.
const ARCH_RIGHT: usize = 3;
const ARCH_WALL: usize = 4;
const ARCH_LEFT: usize = 5;

const ARCHED: [Arched; 2] = [
    Arched {
        distance: 7.0,
        degrees: 30.0,
        below: 1.9,
        past: [3.0, 3.158],
    },
    Arched {
        distance: 3.0,
        degrees: 10.0,
        below: 1.2,
        past: [1.4635, 1.91],
    },
];

impl Arched {
    /// Where the two sides' set-backs touch on the arc: `c − R·sin a`, as the
    /// module's documentation derives it.
    fn touching(&self) -> f64 {
        self.distance - ARCH_RADIUS * self.degrees.to_radians().sin()
    }

    /// The plate's corners, from the bottom's left end round: the bottom's
    /// two ends, then where the right side and the left side meet the arc.
    fn corners(&self) -> [DVec2; 4] {
        let lean = self.degrees.to_radians();
        let (c, s) = (lean.cos(), lean.sin());
        let up_the_side = (ARCH_RADIUS * ARCH_RADIUS - self.distance * self.distance).sqrt();
        let on_the_arc = DVec2::new(-c, s) * self.distance + DVec2::new(s, c) * up_the_side;
        let bottom = -(self.distance - ARCH_BOTTOM * s) / c;
        [
            DVec2::new(bottom, ARCH_BOTTOM),
            DVec2::new(-bottom, ARCH_BOTTOM),
            DVec2::new(-on_the_arc.x, on_the_arc.y),
            on_the_arc,
        ]
    }

    /// The turn the arc makes about the origin, from the right side's end to
    /// the left side's.
    fn arc_turn(&self) -> f64 {
        let [_, _, right, _] = self.corners();
        2.0 * right.x.atan2(right.y)
    }

    fn plate(&self) -> Body {
        let corners = self.corners();
        raised_with(
            &Body::default(),
            Contour {
                corners: corners.to_vec(),
                runs: vec![
                    Run::Straight,
                    Run::Straight,
                    Run::Round {
                        center: DVec2::ZERO,
                        turn: self.arc_turn(),
                    },
                    Run::Straight,
                ],
            },
            ARCH_HEIGHT,
        )
    }

    /// The curved faces a step of `value` leaves on the plate, which the
    /// drawing's tolerance is reckoned over: the arc's wall, and among
    /// rounds the two sides' rounds along their length and a radius past.
    fn curved(&self, value: f64, fillet: bool) -> f64 {
        let [low, _, _, high] = self.corners();
        let side = (high - low).length() + value;
        ARCH_RADIUS * self.arc_turn() * ARCH_HEIGHT
            + if fillet {
                2.0 * PI / 2.0 * value * side
            } else {
                0.0
            }
    }
}

/// The step of `value` on `names`, a fillet's if `fillet`, a chamfer's if not:
/// what it reports asked and taken, and the body it lays, or why it declined.
fn laid_on(
    part: &Body,
    names: &[EdgeName],
    value: f64,
    fillet: bool,
) -> Result<(f64, f64, Body), Declined> {
    if fillet {
        part.fillet_edges(names, value)
            .map(|done| (done.asked, done.taken, done.body))
    } else {
        part.chamfer_edges(names, value)
            .map(|done| (done.asked, done.taken, done.body))
    }
}

#[test]
fn a_fillet_and_a_chamfer_on_the_two_sides_of_an_arched_plate_asked_past_where_their_set_backs_meet_on_its_arc_take_the_value_at_which_they_touch()
 {
    let mut wrong = Vec::new();
    for arched in ARCHED {
        let part = arched.plate();
        assert!(
            part.has_face(ARCH_WALL),
            "the arched plate has its arc's wall, face {ARCH_WALL}"
        );
        let sides = [
            edge_between(&part, TOP, ARCH_LEFT),
            edge_between(&part, TOP, ARCH_RIGHT),
        ];
        let touching = arched.touching();
        let hair = 20.0 * 1e-9 * reach_of(&part);
        for (kind, fillet) in [("fillet", true), ("chamfer", false)] {
            for order in [[0, 1], [1, 0]] {
                let names: Vec<EdgeName> = order.iter().map(|&side| sides[side].clone()).collect();
                for asked in [arched.below].into_iter().chain(arched.past) {
                    let what = format!(
                        "the {kind} of {asked} on the two sides of the arched plate whose sides \
                         lean {} degrees at {}, named {order:?}, their set-backs touching on the \
                         arc at {touching}",
                        arched.degrees, arched.distance
                    );
                    let (reported, taken, body) = match laid_on(&part, &names, asked, fillet) {
                        Ok(laid) => laid,
                        Err(reason) => {
                            wrong.push(format!("{what}: declined, {reason:?}"));
                            continue;
                        }
                    };
                    let expected = asked.min(touching);
                    let least = expected
                        - if asked > touching {
                            SHORT_BY * hair
                        } else {
                            PLACE
                        };
                    if (reported - asked).abs() > PLACE || taken > expected + PLACE || taken < least
                    {
                        wrong.push(format!(
                            "{what}: reports {reported} asked and took {taken}, where {expected} \
                             fits"
                        ));
                        continue;
                    }
                    let (_, _, direct) =
                        laid_on(&part, &sides, taken, fillet).unwrap_or_else(|declined| {
                            panic!(
                                "{what}: the step laid directly at {taken}: declined, {declined:?}"
                            )
                        });
                    sound_drawn_within(
                        &body,
                        direct.volume(),
                        DRAWN * reach_of(&part) * arched.curved(taken, fillet),
                        &format!("{what}, and the same step laid directly at {taken}"),
                    );
                }
            }
        }
    }
    assert!(
        wrong.is_empty(),
        "the two sides of an arched plate whose set-backs would meet on its arc take the value \
         asked short of where they touch, and that value past it:\n{}",
        wrong.join("\n")
    );
}

/// A plate raised [`ARCH_HEIGHT`] whose top is bounded by a straight bottom
/// at [`ARCH_BOTTOM`], a right side on `n₂·p = c₂`, an arc of [`ARCH_RADIUS`]
/// about the origin and a left side on `n₁·p = c₁`, each side's outward
/// normal leaning `a` from the upright — the two sides at different leans
/// and distances, so that their set-backs cross off the arc's middle — the
/// whole turned by `turn` about the origin, so that the arc may straddle any
/// angle of its circle. Asked `below` the touch and each of `past` beyond.
struct Askew {
    left: (f64, f64),
    right: (f64, f64),
    turn: f64,
    called: &'static str,
}

impl Askew {
    /// Each side's outward normal on the plan, unturned, and its distance
    /// from the origin: the left first.
    fn sides(&self) -> [(DVec2, f64); 2] {
        let [(c1, a1), (c2, a2)] = [self.left, self.right].map(|(c, a)| (c, a.to_radians()));
        [
            (DVec2::new(-a1.cos(), a1.sin()), c1),
            (DVec2::new(a2.cos(), a2.sin()), c2),
        ]
    }

    /// Where each side, unturned, meets the circle above the origin, the
    /// left first.
    fn on_the_arc(&self) -> [DVec2; 2] {
        self.sides().map(|(normal, c)| {
            let up = DVec2::new(-normal.y, normal.x) * normal.x.signum().max(-1.0);
            let along = if up.y < 0.0 { -up } else { up };
            normal * c + along * (ARCH_RADIUS * ARCH_RADIUS - c * c).sqrt()
        })
    }

    /// The plate's corners, unturned, from the bottom's left end round.
    fn corners_unturned(&self) -> [DVec2; 4] {
        let [(n1, c1), (n2, c2)] = self.sides();
        let on_bottom = |normal: DVec2, c: f64| (c - normal.y * ARCH_BOTTOM) / normal.x;
        let [left, right] = self.on_the_arc();
        [
            DVec2::new(on_bottom(n1, c1), ARCH_BOTTOM),
            DVec2::new(on_bottom(n2, c2), ARCH_BOTTOM),
            right,
            left,
        ]
    }

    fn turned(&self, at: DVec2) -> DVec2 {
        DVec2::from_angle(self.turn.to_radians()).rotate(at)
    }

    fn plate(&self) -> Body {
        let corners = self.corners_unturned();
        let [_, _, right, left] = corners;
        let arc = left.to_angle() - right.to_angle();
        raised_with(
            &Body::default(),
            Contour {
                corners: corners.map(|at| self.turned(at)).to_vec(),
                runs: vec![
                    Run::Straight,
                    Run::Straight,
                    Run::Round {
                        center: DVec2::ZERO,
                        turn: arc,
                    },
                    Run::Straight,
                ],
            },
            ARCH_HEIGHT,
        )
    }

    /// Where the two set-backs touch on the arc, as the module's
    /// documentation derives it: at the point of the arc as deep behind both
    /// sides' set-backs, `(n₂ − n₁)·e(θ) = (c₂ − c₁)/R`, the depth there.
    fn touching(&self) -> f64 {
        let [(n1, c1), (n2, c2)] = self.sides();
        let [left, right] = self.on_the_arc();
        let [from, to] = [right.to_angle(), left.to_angle()];
        let way = n2 - n1;
        let spread = ((c2 - c1) / (ARCH_RADIUS * way.length())).acos();
        let at = way.to_angle();
        let theta = [
            at - spread,
            at + spread,
            at - spread + TAU,
            at + spread - TAU,
        ]
        .into_iter()
        .find(|theta| (from..=to).contains(theta))
        .expect("the two depths are equal somewhere on the arc");
        c1 - ARCH_RADIUS * n1.dot(DVec2::from_angle(theta))
    }

    /// The least, along the arc, of the larger of the two depths, read by
    /// stepping along it: what [`Askew::touching`] derives, held another way.
    fn touching_by_steps(&self) -> f64 {
        let [(n1, c1), (n2, c2)] = self.sides();
        let [left, right] = self.on_the_arc();
        let [from, to] = [right.to_angle(), left.to_angle()];
        (0..=200_000)
            .map(|step| {
                let e = DVec2::from_angle(from + (to - from) * step as f64 / 200_000.0);
                (c1 - ARCH_RADIUS * n1.dot(e)).max(c2 - ARCH_RADIUS * n2.dot(e))
            })
            .fold(f64::INFINITY, f64::min)
    }

    /// The curved faces a step of `value` leaves, as [`Arched::curved`]
    /// reckons them.
    fn curved(&self, value: f64, fillet: bool) -> f64 {
        let [low, low_right, right, left] = self.corners_unturned();
        let arc = left.to_angle() - right.to_angle();
        let sides = (left - low).length() + (right - low_right).length() + 2.0 * value;
        ARCH_RADIUS * arc * ARCH_HEIGHT
            + if fillet {
                PI / 2.0 * value * sides
            } else {
                0.0
            }
    }
}

const ASKEW: [Askew; 4] = [
    Askew {
        left: (7.0, 30.0),
        right: (6.0, 20.0),
        turn: 0.0,
        called: "sides at 7 leaning 30° and at 6 leaning 20°",
    },
    Askew {
        left: (5.0, 15.0),
        right: (7.0, 35.0),
        turn: 0.0,
        called: "sides at 5 leaning 15° and at 7 leaning 35°",
    },
    Askew {
        left: (7.0, 30.0),
        right: (7.0, 30.0),
        turn: 90.0,
        called: "sides at 7 leaning 30°, turned a quarter, the arc astride its circle's half turn",
    },
    Askew {
        left: (7.0, 30.0),
        right: (6.0, 20.0),
        turn: 95.0,
        called: "sides at 7 leaning 30° and at 6 leaning 20°, turned 95°",
    },
];

#[test]
fn a_fillet_and_a_chamfer_on_two_sides_of_an_arched_plate_at_different_leans_take_the_value_at_which_their_set_backs_touch_off_the_arc_s_middle()
 {
    let mut wrong = Vec::new();
    for askew in ASKEW {
        let touching = askew.touching();
        assert!(
            (touching - askew.touching_by_steps()).abs() < 1e-4,
            "{}: the touch derived, {touching}, is the least of the larger depth along the arc, {}",
            askew.called,
            askew.touching_by_steps()
        );
        let part = askew.plate();
        let sides = [
            edge_between(&part, TOP, ARCH_LEFT),
            edge_between(&part, TOP, ARCH_RIGHT),
        ];
        let hair = 20.0 * 1e-9 * reach_of(&part);
        for (kind, fillet) in [("fillet", true), ("chamfer", false)] {
            for order in [[0, 1], [1, 0]] {
                let names: Vec<EdgeName> = order.iter().map(|&side| sides[side].clone()).collect();
                for asked in [touching - 0.3, touching + 0.5, touching + 1.0] {
                    let what = format!(
                        "the {kind} of {asked} on the two sides of the arched plate with {}, \
                         named {order:?}, their set-backs touching on the arc at {touching}",
                        askew.called
                    );
                    let (reported, taken, body) = match laid_on(&part, &names, asked, fillet) {
                        Ok(laid) => laid,
                        Err(reason) => {
                            wrong.push(format!("{what}: declined, {reason:?}"));
                            continue;
                        }
                    };
                    let expected = asked.min(touching);
                    let least = expected
                        - if asked > touching {
                            SHORT_BY * hair
                        } else {
                            PLACE
                        };
                    if (reported - asked).abs() > PLACE || taken > expected + PLACE || taken < least
                    {
                        wrong.push(format!(
                            "{what}: reports {reported} asked and took {taken}, where {expected} \
                             fits"
                        ));
                        continue;
                    }
                    let (_, _, direct) =
                        laid_on(&part, &sides, taken, fillet).unwrap_or_else(|declined| {
                            panic!(
                                "{what}: the step laid directly at {taken}: declined, {declined:?}"
                            )
                        });
                    sound_drawn_within(
                        &body,
                        direct.volume(),
                        DRAWN * reach_of(&part) * askew.curved(taken, fillet),
                        &format!("{what}, and the same step laid directly at {taken}"),
                    );
                }
            }
        }
    }
    assert!(
        wrong.is_empty(),
        "two sides of an arched plate at different leans take the value at which their set-backs \
         touch on the arc, off its middle:\n{}",
        wrong.join("\n")
    );
}

#[test]
fn a_fillet_and_a_chamfer_on_the_two_sides_of_an_arched_plate_bored_through_its_top_take_the_value_at_which_their_set_backs_touch_on_the_arc_the_bore_s_rim_read_whole()
 {
    let arched = &ARCHED[0];
    let plain = arched.plate();
    let bore = Contour::circle(DVec2::new(0.0, 2.0), 1.5);
    let bored = plain
        .difference(
            &plain
                .tool_raised(
                    &profile(bore),
                    Frame {
                        origin: DVec3::Z * -1.0,
                        ..ground()
                    },
                    DVec3::Z * (ARCH_HEIGHT + 2.0),
                )
                .expect("the bore raises"),
        )
        .expect("the bore is cut");
    let touching = arched.touching();
    let mut wrong = Vec::new();
    for (kind, fillet) in [("fillet", true), ("chamfer", false)] {
        for asked in [arched.below].into_iter().chain(arched.past) {
            let what = format!(
                "the {kind} of {asked} on the two sides of the arched plate bored through its top, \
                 the bore's rim a whole circle bounding the top both sides cut, their set-backs \
                 touching on the arc at {touching}"
            );
            let laid = [&plain, &bored].map(|part| {
                let sides = [
                    edge_between(part, TOP, ARCH_LEFT),
                    edge_between(part, TOP, ARCH_RIGHT),
                ];
                laid_on(part, &sides, asked, fillet)
            });
            let [Ok((_, plain_taken, _)), Ok((_, taken, body))] = laid else {
                wrong.push(format!("{what}: declined, {:?}", laid.map(|one| one.err())));
                continue;
            };
            if (taken - plain_taken).abs() > PLACE {
                wrong.push(format!(
                    "{what}: took {taken}, where the plate unbored takes {plain_taken}: the rim, \
                     inside the top, is reached by the two set-backs only after the arc is"
                ));
                continue;
            }
            let sides = [
                edge_between(&bored, TOP, ARCH_LEFT),
                edge_between(&bored, TOP, ARCH_RIGHT),
            ];
            let (_, _, direct) =
                laid_on(&bored, &sides, taken, fillet).unwrap_or_else(|declined| {
                    panic!("{what}: laid directly at {taken}: {declined:?}")
                });
            let bore_wall = 2.0 * PI * 1.5 * (ARCH_HEIGHT + 2.0);
            sound_drawn_within(
                &body,
                direct.volume(),
                DRAWN * reach_of(&bored) * (arched.curved(taken, fillet) + bore_wall),
                &what,
            );
        }
    }
    assert!(
        wrong.is_empty(),
        "a bore through the arched plate's top, its rim read whole, leaves the value where the \
         two set-backs touch on the arc:\n{}",
        wrong.join("\n")
    );
}

/// The hexagonal prism with its wall three, between the front and wall four,
/// standing upright as raised or cut back to lean in under its top edge by
/// `degrees`, as [`leaning_in`] cuts it.
fn hexagon_with_its_wall_three_leaning(degrees: f64) -> Body {
    if degrees == 0.0 {
        return hexagonal_prism();
    }
    let [_, from, to, _, _, _] = hexagon_corners();
    let (_, out) = trapezoid_edge(from, to);
    let (part, _) = leaning_in(
        &hexagonal_prism(),
        [from, to].map(|at| at.extend(HEXAGON_HEIGHT)),
        out.extend(0.0),
        degrees,
    );
    part
}

#[test]
fn a_fillet_on_a_hexagonal_prism_s_front_top_edge_is_drawn_within_the_drawing_s_tolerance_whether_its_next_wall_stands_upright_or_leans_over_it()
 {
    let radius = 2.0;
    let mut wrong = Vec::new();
    for degrees in [0.0, 20.0, 40.0] {
        let part = hexagon_with_its_wall_three_leaning(degrees);
        let what = format!(
            "the fillet of {radius} on the hexagon's front top edge alone, its wall three leaning \
             in {degrees} degrees under its top edge"
        );
        let step = part
            .fillet_edges(&[edge_between(&part, TOP, HEXAGON_FRONT)], radius)
            .unwrap_or_else(|declined| panic!("{what}: declined, {declined:?}"));
        taken_as_asked(step.asked, step.taken, radius, &what);
        let body = &step.body;
        assert!(
            body.is_exact(),
            "{what}: the exact kernel computed the body"
        );
        let triangles = body.triangles();
        assert_eq!(closed(&triangles), Ok(()), "{what}");
        assert_eq!(uncrossed(&triangles), Ok(()), "{what}");
        let allowed =
            DRAWN * reach_of(&part) * hexagon_round_to(radius, 0.0, HEXAGON_SIDE / 2.0).area();
        let short = body.volume() - enclosed(&triangles);
        if short.abs() > allowed {
            wrong.push(format!(
                "{what}: its triangles enclose {short} less than the exact body's {}, where the \
                 drawing's tolerance allows its round {allowed}",
                body.volume()
            ));
        }
    }
    assert!(
        wrong.is_empty(),
        "a round is drawn within the drawing's tolerance of its exact surface, against an upright \
         wall or one leaning over it:\n{}",
        wrong.join("\n")
    );
}
