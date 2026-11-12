//! A full round ending on a pointed end, seen from above, filleted in one step
//! with the two edges of the point. The plate is 30 long, 6 wide and 10 high,
//! its top (0, 0), (30, 0), the tip, (30, 6), (0, 6), raised as `cao_part`
//! builds a step: its floor numbered nought, its top one, then one wall for
//! each run of the outline in order — the front two, the point's front three,
//! the point's back four, the back five, and the square left end six. The
//! top's front and back edges, rounded at half the width, meet across the top
//! in one full round, as #698 lays it; each edge of the point meets one of
//! them at a corner, and the other edge of the point at the tip. Where #803
//! mitres a full round against the one edge across its end, here each point
//! edge's round is laid at the full round's radius and mitred against both its
//! neighbours on the vertical plane bisecting their walls, the three mitres
//! meeting at the crown, where the three cylinders touch. An operation of the
//! kernel on [`Body`], as #698's step on several edges is; the step of the
//! history that calls it comes with the application, in #701.
//!
//! Closes #843.
//! - The pointed end of this issue — tip (33,3) — filleted at 3 on its two
//!   long top edges and its two point edges, is built on the exact kernel and
//!   takes the value whole. The full round and each point round are cylinders
//!   of radius 3. Each point round meets the full round in the vertical plane
//!   through the corner (30,0) or (30,6) and the crown, and the two point
//!   rounds meet in the plane y = 3. The three mitres meet at the crown,
//!   (33 − 3√2, 3, 10). The body holds the volume arithmetic gives; its
//!   triangles are closed, uncrossed, and enclose that volume within the
//!   drawing's tolerance —
//!   `a_fillet_on_a_full_round_and_the_two_edges_of_its_pointed_end_mitres_each_point_round_against_its_neighbours_up_to_the_crown`
//! - The same body whatever the order the list names the four edges in —
//!   `a_fillet_on_a_full_round_and_the_two_edges_of_its_pointed_end_lays_the_same_body_whatever_the_order`.
//!   Read as: each of the 24 orders builds the body the first criterion
//!   holds, and the one the issue's order lays — the same matter, the same
//!   faces meeting by the same edges, each laid face read by the wall it was
//!   laid on, and the plate's flat faces at the same corners
//! - A sharper point, tip (39,3), with the crown at (39 − 3√10, 3, 10), and a
//!   blunter one, tip (32,3), with the crown at (32 − √13, 3, 10): built
//!   likewise —
//!   `a_fillet_on_a_full_round_and_the_two_edges_of_a_sharper_or_a_blunter_point_mitres_each_point_round_against_its_neighbours_up_to_the_crown`.
//!   Read as: every check of the first criterion, in each of the 24 orders,
//!   one body, the one the issue's order lays
//! - The same points chamfered at 3 stay built as on `main`, a hip whose flats
//!   meet at the crown —
//!   `a_chamfer_on_a_full_round_pair_s_edges_and_the_two_edges_of_a_pointed_end_stays_a_hip_whose_flats_meet_at_the_crown`.
//!   Read as: on the tips (33,3), (39,3) and (32,3), in each of the 24
//!   orders, the value taken whole, the body exact, closed and uncrossed, its
//!   volume what arithmetic gives; one flat for each edge, numbered in the
//!   order the list names them, each over its share of the top, facing
//!   halfway between the top's way and its wall's; the front's and the back's
//!   meeting in a ridge along the middle line, the four meeting at one vertex,
//!   the crown; one body, the one the issue's order lays
//! - No body of #698's, #699's or #803's tests changes — no test: those tests
//!   hold their own bodies, in
//!   `a_chamfer_and_a_fillet_take_several_edges_in_one_step.rs` and
//!   `a_chamfer_and_a_fillet_blend_the_corner_their_edges_meet_at.rs`, and run
//!   beside this file
//! - The ends of #894 are still built at the value `main` takes —
//!   `the_blunt_the_lopsided_and_the_bevelled_ends_are_still_filleted_at_the_value_main_takes`.
//!   Read as: at 3, the edges named as the issue names them — the front's top
//!   edge, the end's from front to back, the back's — the value taken as on
//!   `main` at c9fd987d, each body exact, closed and uncrossed: the blunt
//!   point, tip (31,3), 6/√10; the lopsided point, tip (33,2), 2; the bevelled
//!   end (0,0), (29,0), (30,1), (30,5), (29,6), (0,6), 1 with its end edge
//!   named and 1 with it left out. That one order only: on `main` these ends
//!   take other values in other orders, and the bevelled end is declined
//!   `Lost(Gone)` in four, which is #895's
//! - Decided on the way (2026-11-12): admitting an edge that meets one edge
//!   of the pair and a listed edge meeting the other also lets in a V-notched
//!   end, the point turned inwards — tips (27,3) and (28,3), the notch's two
//!   edges meeting at a concave corner, laid beside it as #801 lays two convex
//!   edges beside a concave one: the upright at the notch left sharp, each
//!   notch edge's round running on past its wall's end down to it, mitred
//!   against the other's in the plane y = 3, the crown at (27 − 3√2, 3, 10)
//!   and (28 − √13, 3, 10). `main` declines it in every order; kept, as the
//!   kernel should build every shape, and held in every order — built exact at
//!   3, one body, closed, uncrossed, its volume what arithmetic gives, its
//!   rounds and mitres held as the point's are —
//!   `a_fillet_on_a_full_round_and_the_two_edges_of_a_v_notched_end_mitres_each_notch_edge_s_round_against_its_neighbours_down_to_the_notch_whatever_the_order`
//! - Decided on the way, after the review (2026-11-12): a pointed or V-notched
//!   end whose shared corner — the tip, or the notch's bottom — has a third
//!   edge of the list, its upright, was declined `Unsupported` in every order.
//!   It built one body in 88 of its 120 orders and was declined in the other
//!   32, and one body in every order or a decline in every order is the rule
//!   (#822). #837's third slice builds the point's, its tip closed by a piece
//!   of sphere, below. The notch's stays declined: on the notch (27,3), the
//!   upright at its bottom named with the four top edges, in each of the 120
//!   orders; building it is #914 —
//!   `a_fillet_on_a_full_round_the_two_edges_of_a_v_notched_end_and_the_upright_at_the_notch_s_bottom_is_declined_as_unsupported_whatever_the_order`
//! - Decided on the way, after the review (2026-11-12): what the branch lets
//!   through beside the issue's end is held as the issue's end is — the value
//!   taken whole, every round over its share of the top on its own cylinder,
//!   each mitre in the vertical plane through its corner and its crown, each
//!   crown one vertex, the body exact, closed, uncrossed, its volume what
//!   arithmetic gives, one body in every order. A plate pointed at both ends,
//!   top (0,3), (3,0), (30,0), (33,3), (30,6), (3,6), filleted on its two long
//!   edges and its four point edges, the crowns at (3√2, 3, 10) and
//!   (33 − 3√2, 3, 10). Read as: in 48 of the 720 orders of the six, every
//!   order of each end's four edges — the front, the back and that end's two
//!   point edges — the two ends' point edges named side by side, in both
//!   pairings of the right end's with the left's —
//!   `a_fillet_on_a_full_round_pointed_at_both_ends_mitres_each_point_round_up_to_its_end_s_crown_whatever_the_order`;
//!   and all 720 in a campaign run by hand, out of the gate, which took some
//!   twelve minutes when first run and laid one body —
//!   `a_fillet_on_a_full_round_pointed_at_both_ends_mitres_each_point_round_up_to_its_end_s_crown_in_each_of_the_720_orders_of_its_six_edges`
//! - Decided on the way, after the review (2026-11-12): a plate pointed at one
//!   end, tip (33,3), and square at the other with that end's top edge named
//!   too, as #803 mitres it against the full round, the crowns at (3, 3, 10)
//!   and (33 − 3√2, 3, 10), in each of the 120 orders of the five —
//!   `a_fillet_on_a_full_round_pointed_at_one_end_and_square_at_the_other_mitres_each_end_s_rounds_up_to_its_crown_whatever_the_order`
//! - Decided on the way, after the review (2026-11-12): a lopsided point that
//!   would keep the full round, its long edges 30 and 31 long, top (0,0),
//!   (30,0), (98/3, 2), (31,6), (0,6): both point walls tangent to the circle
//!   of radius 3 about (29, 3), so that every round would reach the one crown,
//!   (29, 3, 10). Its value depended on the order, #895's — the 12 orders
//!   naming the point's front edge before the top's front edge took 3 and laid
//!   the mitred body, the 12 others took 2 — until #901 took for three edges
//!   in a row the least any takes alone: it takes 2 in every order now, the
//!   top kept, short of the full round. Raising it to the value asked is
//!   #894's. Pinned as it is, exact, closed and uncrossed —
//!   `a_fillet_on_a_full_round_and_the_two_edges_of_a_lopsided_point_tangent_to_one_circle_stops_short_of_the_full_round_whatever_the_order`
//! - Decided on the way, after the mutation run (2026-11-12): admitting an
//!   edge at a full round's end holds only an edge of the top meeting both
//!   edges of the pair, or one of them and another such edge meeting the
//!   other. A full round, the edge across its square end and the upright at
//!   one corner of that end — a full round meeting an upright, #837's, the
//!   upright lying off the face the pair meets across — was held here
//!   declined `Unsupported` in every order, until #837's second slice built
//!   it — no test: the decline went with that slice, and the body, its front
//!   corner closed by a piece of sphere and its back corner mitred, is held in
//!   `a_full_round_meets_the_round_of_an_upright_at_its_end_on_an_ellipse.rs`
//!   by
//!   `a_fillet_on_a_full_round_the_edge_across_its_end_and_the_upright_at_its_front_corner_closes_that_corner_with_a_piece_of_sphere_and_mitres_the_other_whatever_the_order`
//! - Found on the way, after the mutation run (2026-11-12): a full round and
//!   one edge of its point alone, tip (33,3), is not declined but stops short
//!   of the full round, at 6 − 3√2 with the top kept, in every order — and
//!   likewise with the edge across the square far end named too: the value is
//!   bounded before any pair forms, so no full round's end is read. Pinned as
//!   it is, exact, closed and uncrossed —
//!   `a_fillet_on_a_full_round_and_one_edge_of_its_point_alone_stops_short_of_the_full_round_whatever_the_order`,
//!   `a_fillet_on_a_full_round_one_edge_of_its_point_alone_and_its_square_far_end_s_edge_stops_short_of_the_full_round_whatever_the_order`
//! - Decided on the way, after slice 2's mutants (2026-11-12): an upright
//!   meeting one edge of a full round's pair at a convex corner is admitted
//!   only where every third edge of the list there is the edge at the pair's
//!   end, bounding the face that is gone and meeting both edges of the pair.
//!   The pointed end, tip (33,3), its four top edges named with the upright
//!   at one of its shoulders, (30,0) or (30,6) — where the third edge is the
//!   point's, meeting one edge of the pair alone — is declined `Unsupported`
//!   in each of the 120 orders of the five, at either shoulder, as one body
//!   in every order or a decline in every order is the rule (#822); building
//!   it is #911 —
//!   `a_fillet_on_a_full_round_the_two_edges_of_its_pointed_end_and_the_upright_at_either_shoulder_is_declined_as_unsupported_whatever_the_order`
//!
//! Closes #837 — its third and last slice, the pointed end's tip. Its first
//! two slices, the mitres and the spheres, are held in
//! `a_full_round_meets_the_round_of_an_upright_at_its_end_on_an_ellipse.rs`.
//! - #843's pointed end, tip (33, 3), with its tip's upright named too,
//!   filleted at 3: built in every order, one body; the tip closed by the
//!   piece of sphere centred (33 − 3√2, 3, 7); the volume arithmetic gives —
//!   `a_fillet_on_a_full_round_the_two_edges_of_its_pointed_end_and_the_upright_at_its_tip_closes_the_tip_with_a_piece_of_sphere_whatever_the_order`.
//!   Read as: (33 − 3√2, 3, 7) one radius from both of the point's walls and
//!   under the top; in each of the 120 orders, the value taken whole; the body
//!   exact, closed, uncrossed, holding `1224 + 36√2 + (75 − 6√2)·9π/4`, which
//!   the booleans give too, and its triangles enclosing that within the
//!   drawing's tolerance; the top gone; the full round one face answering to
//!   both its edges' ranks, each point edge's round and the upright's round
//!   one face answering to its own, and the piece of sphere one face answering
//!   to the patch #699 numbers after every edge's, the next step numbering
//!   past it; the piece drawn on the sphere of 3 about (33 − 3√2, 3, 7) only,
//!   on the tip's side of the planes through that centre square to the point's
//!   two walls and over z = 7, and met there from outside; the full round
//!   drawn on its half cylinder over the front's and the back's shares only,
//!   never past the mitres at the shoulders; each point edge's round on its
//!   quarter cylinder over its share only, from its shoulder's mitre to the
//!   plane through the centre square to its wall; the upright's round on its
//!   cylinder about the vertical through (33 − 3√2, 3) only, turning from
//!   facing out of the point's front wall to facing out of its back wall,
//!   never above z = 7; each round met first from outside; the piece meeting
//!   each point edge's round along the quarter circle of that round's end
//!   section, from where the upright's round leaves its wall at z = 7 up to
//!   the crown, (33 − 3√2, 3, 10), and the upright's round along the arc
//!   z = 7 between its two feet, each drawn through the points between; the
//!   full round meeting each point edge's round on the vertical plane through
//!   their shoulder and the crown, drawn from the shoulder at z = 7 up to the
//!   crown; the full round sharing with the piece the crown and no other
//!   point, and no point with the upright's round; the two point edges' rounds
//!   sharing the crown only, no mitre left between them, and each with the
//!   upright's round its foot only; one vertex at the crown, where the full
//!   round, both point edges' rounds and the piece meet, and one at each foot,
//!   where the point's wall, its edge's round, the upright's round and the
//!   piece meet; the front, the back and the point's walls up to z = 7, the
//!   point's walls cut back to the upright's feet, and the floor given up to
//!   the upright's round along its arc; every face meeting the faces it bounds
//!   by one edge each, the piece the three rounds only; and every order laying
//!   the body the listed order lays
//! - Likewise tips (39, 3) and (32, 3) —
//!   `a_fillet_on_a_full_round_the_two_edges_of_a_sharper_or_a_blunter_point_and_the_upright_at_its_tip_closes_the_tip_with_a_piece_of_sphere_whatever_the_order`.
//!   Read as: every check of the first criterion, in each of the 120 orders,
//!   on the tip (39, 3), the piece about (39 − 3√10, 3, 7), holding
//!   `1152 + 36√10 + 81·atan 3 + (78 − 6√10)·9π/4`, and on the tip (32, 3),
//!   the piece about (32 − √13, 3, 7), holding
//!   `1236 + 12√13 + 81·atan(2/3) + (64 − 2√13)·9π/4`, each which the booleans
//!   give too
//! - Throughout, as far as this slice goes: the lists `main` builds keep their
//!   bodies — the chamfers above but the tongue's, the end edge with both
//!   uprights without the pair, and every one of these at 6.5 wide — no test:
//!   held by the first slice's test, in
//!   `a_full_round_meets_the_round_of_an_upright_at_its_end_on_an_ellipse.rs`,
//!   `the_lists_main_builds_at_the_end_of_a_block_6_or_6_5_wide_keep_their_bodies_in_every_order`,
//!   which this slice leaves as it is
//! - Throughout: no body of #698's, #699's, #803's or #843's tests changes —
//!   no test: those tests hold their own bodies, in
//!   `a_chamfer_and_a_fillet_take_several_edges_in_one_step.rs`,
//!   `a_chamfer_and_a_fillet_blend_the_corner_their_edges_meet_at.rs` and this
//!   file, and run beside the tests above. What changes is a decline, not a
//!   body: the decline of a pointed or V-notched end with the upright at its
//!   tip gives up the point, built above, keeps the notch, and is renamed for
//!   it `a_fillet_on_a_full_round_the_two_edges_of_a_v_notched_end_and_the_upright_at_the_notch_s_bottom_is_declined_as_unsupported_whatever_the_order`
//!
//! The campaign is run by hand, out of the gate, compiled only under the
//! `campaigns` feature: every one of the 720 orders of the plate pointed at
//! both ends, where the gate takes 48 of them, each held as the gate holds its
//! own —
//!
//! ```text
//! cargo test --release -p cao_solid --features campaigns \
//!     --test a_full_round_ends_on_a_pointed_end_mitred_against_each_edge_there -- --ignored
//! ```
//!
//! The volume is arithmetic. Seen from above, the top is shared among the four
//! rounds, each share bounded by its wall, the mitres at the wall's two ends
//! and, for the front and the back, the middle of the left end: the front's
//! and the back's run from the left end to the mitre at their corner, and each
//! point edge's is the triangle its wall makes with the crown. Over its share,
//! a round lays the top at the height a quarter round of radius `r` gives at a
//! point's distance `t` from its wall, `H − r + √(r² − (r − t)²)`. Under the
//! rounds' axes, at `H − r`, the plate keeps its whole outline, of area
//! `2rL + ar` for a tip `a` past the ends of the front and back, both `L` long
//! — `a` negative for a notch, whose triangle is taken away. Over them the
//! matter is `∫ √(r² − (r − t)²)·ℓ(t) dt` from the walls, `t = 0`, up to the
//! axes, `t = r`, `ℓ(t)` the length of the lines at `t` from their walls
//! across the four shares: `L − kt` along the front and along the back, the
//! mitre at the corner standing back by `k = (s − a)/r` for each unit in from
//! the front, the cotangent of half the corner's angle; and `s(r − t)/r` along
//! each point edge, `s = √(a² + r²)` long, shrinking evenly to nothing at the
//! crown. So `ℓ(t)` runs linearly and the integral is
//! `ℓ(r)·πr²/4 + (ℓ(0) − ℓ(r))·r²/3`; `ℓ(r)` is twice the crown's distance
//! from the left end. On the tip (33, 3), `a = r = 3`:
//! `1305 + 36√2 + (66 − 6√2)·9π/4`, 1762.46; on the notch's tip (27, 3),
//! `1215 + 36√2 + (54 − 6√2)·9π/4`, 1587.64, and on (28, 3), 1618.14. At
//! `a = 0` the point is a square end, and the sum is #803's,
//! `(H − r)·2rL + πr²(L − r)/2 + 4r³/3`. The booleans give it too: the full
//! round's prism cut from the plate whole, then each other round's, raised
//! along its wall's line past both its ends and kept on its own side of each
//! mitre it shares with another such round — the tip's, `y = 3` on the point.
//!
//! The same holds on any outline whose named walls' shares close one radius
//! in from every wall they bound: each named wall's share runs along it,
//! shrinking by `t·(c₁ + c₂)` at `t` from it, `c` at each of its ends the
//! cotangent of half the angle there where the wall beside is named too, and
//! of the whole angle where it is not — nought at a square one. Pointed at
//! both ends, the area 180, `ℓ(0) = 54 + 12√2` and `ℓ(r) = 66 − 12√2`:
//! `1224 + 72√2 + (66 − 12√2)·9π/4`, 1672.39. Pointed at one end and square
//! at the other with its edge named: `1341 + 36√2 + (60 − 6√2)·9π/4`,
//! 1756.05, #803's square end taking `πr³/2 − 4r³/3` off the pointed plate's.
//! The lopsided point, laid at the value asked as #894 would lay it: the area
//! 190, `ℓ(0) = 61 + 23/3` and `ℓ(r) = 58`, twice the crown's 29;
//! `1362 + 261π/2`, 1771.98, which the orders that took 3 held before #901. Its tip is where the
//! second tangents from (30, 0) and (31, 6) to the circle about (29, 3) meet:
//! from (30, 0) the wall climbs at twice the angle whose tangent is 1/3, along
//! (4, 3)/5, and from (31, 6) it falls at twice the angle whose tangent is
//! 2/3, along (5, −12)/13; they cross 10/3 and 13/3 from their corners, at
//! (98/3, 2).
//!
//! Chamfered at `r`, a flat lays the top at `H − r + t` over its share, the
//! shares and the lines across them as for the rounds, and the matter over
//! the flats' feet is `∫ t·ℓ(t) dt`, `ℓ(r)·r²/2 + (ℓ(0) − ℓ(r))·r²/6`: on the
//! tips (33, 3), (39, 3) and (32, 3), 1598.27, 1744.54 and 1573.18, the
//! values `main` measures.
//!
//! With the upright at the tip named too, the tip is three convex rounds at a
//! corner, closed as #699 closes one: by a piece of sphere of radius `r` about
//! the centre `c`, the point one radius from the point's two walls and from
//! the top — on the middle line under #843's crown, at `H − r`, `x_c = L + a −
//! s` from the left end, so behind the shoulders on all three tips. The
//! upright's round stands about the vertical through `c` from the floor up to
//! `H − r`, and leaves each point wall at its foot, one radius out from `c`
//! square to the wall, `a` back from the tip: `r` over the tangent of half the
//! tip's angle `α`, which is `r/a`. It turns by `γ = π − α` between the two. Each point edge's round stops on the
//! plane through `c` square to its wall, through its foot, where it meets the
//! sphere along the quarter circle from that foot up to the crown; the
//! upright's round meets it along the arc at `H − r` between the two feet. The
//! full round, mitred against each point edge's round as without the upright,
//! touches the sphere at the crown alone. Seen from above, the kite between
//! `c`, the two feet and the tip, of area `ar`, the tip's own triangle's, is no
//! round's share any more: under `H − r` it keeps the sector of the upright's
//! round, `r²γ/2`, and over it the ball's sector over that cell, `r³γ/3`. Each
//! point edge's share closes square at its foot, so `ℓ(0)` gives up `2a` and
//! `ℓ(r)` stays twice the crown's `x_c`. The body holds
//! `(H − r)(2rL + r²γ/2) + 2x_c·πr²/4 + 4(s − a)·r²/3 + r³γ/3`: on the tip
//! (33, 3), `γ = π/2`, `1224 + 36√2 + (75 − 6√2)·9π/4`, 1745.08, #843's body
//! less `81 − 81π/4`; on (39, 3), `γ = 2·atan 3`,
//! `1152 + 36√10 + 81·atan 3 + (78 − 6√10)·9π/4`, 1784.25; on (32, 3),
//! `γ = 2·atan(2/3)`, `1236 + 12√13 + 81·atan(2/3) + (64 − 2√13)·9π/4`,
//! 1728.31. The booleans give it too, as #699 computes a corner's piece: the
//! full round's prism cut from the plate whole, each point edge's raised
//! along its wall's line past both its ends, the upright's raised whole up the
//! plate; then the cell on the tip's side of the three planes through `c` —
//! square to each point wall, and level — taken away, and the ball's sector
//! over it given back.

use std::collections::{BTreeMap, BTreeSet};
use std::f64::consts::{PI, SQRT_2};

use cao_solid::profile::{Contour, Frame, Profile, Run};
use cao_solid::soundness::{closed, enclosed, uncrossed};
use cao_solid::{Body, Chamfered, Declined, EdgeName, Filleted, Loop};
use glam::{DVec2, DVec3};

/// How far a point may stand from where the arithmetic puts it: each is placed
/// exactly, so what is left is rounding.
const PLACE: f64 = 1e-9;

/// How near the arithmetic an exact volume comes, as a fraction of it.
const EXACT: f64 = 1e-9;

/// How far the triangles may stand from a round, as a share of the reach the
/// body carries: the drawing's tolerance.
const DRAWN: f64 = 2e-4;

/// How far inside a round's own share of the top a ray aims, clear of every
/// side of it, to meet that round's triangles rather than a neighbour's: well
/// over the drawing's tolerance on the plate, [`DRAWN`] times its reach.
const CLEAR_OF_THE_MITRE: f64 = 0.1;

/// The numbers a raised plate's floor and top answer to: nought and one, its
/// walls following, one for each run of its outline, from two.
const FLOOR: usize = 0;
const TOP: usize = 1;

/// The numbers of a pointed plate's walls, as [`Plate::pointed`] raises it:
/// its front, the point's two walls, front then back, whose upright stands at
/// the tip, and its back.
const FRONT: usize = 2;
const POINT_FRONT: usize = 3;
const POINT_BACK: usize = 4;
const BACK: usize = 5;

/// The plate: its front and back 30 long, from the left end to the point, 6
/// wide and 10 high.
const LENGTH: f64 = 30.0;
const WIDTH: f64 = 6.0;
const HEIGHT: f64 = 10.0;

/// The value asked, half the width: the radius at which the rounds of the
/// top's front and back edges meet across the top in one full round.
const RADIUS: f64 = WIDTH / 2.0;

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

    /// Whether a point lies on the round however wide its turn: held by
    /// [`Round::holds`], and no further round from halfway between its two
    /// faces' ways than half the turn. Past a quarter turn, the two sides
    /// [`Round::holds`] reads let through the far side of the cylinder too.
    fn holds_within_its_turn(&self, point: DVec3) -> bool {
        let off = point - self.start;
        let across = off - self.along * off.dot(self.along);
        let [first, second] = self.facing;
        let halfway = (first + second).normalize();
        let half_turn = first.dot(second).acos() / 2.0;
        self.holds(point) && across.dot(halfway) >= self.radius * half_turn.cos() - PLACE
    }
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

/// A round's share of the top, seen from above: the corners of the polygon it
/// is laid over, anticlockwise.
struct Share {
    corners: Vec<DVec2>,
}

impl Share {
    /// How far a point stands inside the share, seen from above: its least
    /// distance in from the share's sides, negative outside.
    fn inside_by(&self, point: DVec3) -> f64 {
        let point = point.truncate();
        let next = self.corners.iter().cycle().skip(1);
        self.corners
            .iter()
            .zip(next)
            .map(|(&from, &to)| (to - from).normalize().perp_dot(point - from))
            .fold(f64::INFINITY, f64::min)
    }

    /// Whether one of the share's corners stands at `point`, seen from above.
    fn reaches(&self, point: DVec3) -> bool {
        self.corners
            .iter()
            .any(|corner| corner.distance(point.truncate()) < PLACE)
    }
}

/// A wall of the plate: its number, what its top edge is called, and where it
/// runs from and to, seen from above, the way the outline runs round the
/// plate.
struct Wall {
    number: usize,
    called: String,
    from: DVec2,
    to: DVec2,
}

impl Wall {
    /// The way the wall runs, level.
    fn along(&self) -> DVec3 {
        (self.to - self.from).normalize().extend(0.0)
    }

    /// The way out of the plate, square to the wall, level: the outline runs
    /// anticlockwise seen from above, the plate on its left.
    fn outward(&self) -> DVec3 {
        self.along().cross(DVec3::Z)
    }

    /// The top edge's name on the part.
    fn name(&self, part: &Body) -> EdgeName {
        edge_between(part, TOP, self.number)
    }

    /// The round a fillet of [`RADIUS`] lays on the top edge over `share`: a
    /// quarter round about the line the radius in from the wall and under the
    /// top, as far along as the share reaches — past the wall's end where the
    /// wall meets the next inwards, as a notch's two do — turning from the
    /// top's way to the wall's.
    fn round_over(&self, share: &Share) -> Round {
        let along = self.along();
        let reached = |corner: &DVec2| (*corner - self.from).dot(along.truncate());
        let first = share
            .corners
            .iter()
            .map(reached)
            .fold(f64::INFINITY, f64::min);
        let last = share
            .corners
            .iter()
            .map(reached)
            .fold(f64::NEG_INFINITY, f64::max);
        Round {
            start: self.from.extend(HEIGHT) - (self.outward() + DVec3::Z) * RADIUS + along * first,
            along,
            length: last - first,
            radius: RADIUS,
            facing: [DVec3::Z, self.outward()],
        }
    }

    /// The prism the round of the top edge takes away alone: all that stands
    /// over the edge outside its quarter round, raised along the wall's whole
    /// line and past both its ends.
    fn tool(&self, part: &Body) -> Body {
        let r = RADIUS;
        let far = 2.0 * r + 2.0;
        let foot = HEIGHT - r;
        let corners = vec![
            DVec2::new(0.0, foot),
            DVec2::new(r, HEIGHT),
            DVec2::new(r, HEIGHT + far),
            DVec2::new(-far, HEIGHT + far),
            DVec2::new(-far, foot),
        ];
        let mut runs = vec![Run::Straight; corners.len()];
        runs[0] = Run::Round {
            center: DVec2::new(r, foot),
            turn: -PI / 2.0,
        };
        let along = self.along();
        let past = r + 2.0;
        part.tool_raised(
            &profile(Contour { corners, runs }),
            Frame {
                origin: self.from.extend(0.0) - along * past,
                u: DVec3::Z.cross(along),
                v: DVec3::Z,
            },
            along * (self.from.distance(self.to) + 2.0 * past),
        )
        .expect("the tool raises")
    }
}

/// Where two neighbouring rounds meet: their edges, as indices into
/// [`Plate::edges`], the corner of the outline their walls share, seen from
/// above, the crown the mitre runs up to, and what the corner is called.
struct Mitre {
    edges: [usize; 2],
    corner: DVec2,
    crown: DVec3,
    called: String,
}

impl Mitre {
    /// How far a point stands from the vertical plane through the corner and
    /// the crown.
    fn off(&self, point: DVec3) -> f64 {
        let across = (self.crown.truncate() - self.corner).perp().normalize();
        (point.truncate() - self.corner).dot(across).abs()
    }

    /// The half-space bounded by the mitre's plane that holds `wall`'s share
    /// of the top: its normal, level, and its offset, as [`Body::behind`]
    /// takes them.
    fn kept_side_of(&self, wall: &Wall) -> (DVec3, f64) {
        let mut across = (self.crown.truncate() - self.corner).perp().normalize();
        let far_end = if wall.from.distance(self.corner) < PLACE {
            wall.to
        } else {
            wall.from
        };
        if (far_end - self.corner).dot(across) > 0.0 {
            across = -across;
        }
        (across.extend(0.0), across.dot(self.corner))
    }
}

/// A plate [`HEIGHT`] high raised from an outline, anticlockwise seen from
/// above, as `cao_part` builds a step — its run `k` from corner `k` to the
/// next raising wall `2 + k` — some of whose walls' top edges a list names,
/// filleted at [`RADIUS`]: what the plate is called; its outline; the runs
/// whose top edges are named, each with what it is called, in the order the
/// plate lists them, the issue's where it names one; the two runs, along the
/// front and the back, whose rounds meet across the top in one full round;
/// the crowns its mitres run up to, as the issue or the arithmetic puts them;
/// and the part.
struct Plate {
    called: String,
    outline: Vec<DVec2>,
    edges: Vec<(usize, String)>,
    full_round: [usize; 2],
    crowns: Vec<DVec3>,
    part: Body,
}

impl Plate {
    /// The plate raised from `outline`, the top edges of the runs `edges`
    /// names listed in that order.
    fn raised(
        called: &str,
        outline: Vec<DVec2>,
        edges: &[(usize, &str)],
        full_round: [usize; 2],
        crowns: Vec<DVec3>,
    ) -> Plate {
        Plate {
            called: called.to_string(),
            part: raised(Contour::straight(outline.clone()), HEIGHT),
            outline,
            edges: edges
                .iter()
                .map(|&(run, called)| (run, called.to_string()))
                .collect(),
            full_round,
            crowns,
        }
    }

    /// The plate [`LENGTH`] long and [`WIDTH`] wide whose right end comes to a
    /// point seen from above, its tip at `x` along on the middle line — or,
    /// `x` short of [`LENGTH`], is notched back to it —, the four top edges
    /// named as the issue names them: the front's, the end's from front to
    /// back, the back's; its mitres meeting at `crown`.
    fn pointed(x: f64, crown: DVec3) -> Plate {
        let end = if x < LENGTH { "notch" } else { "point" };
        let [front, back] = ["front", "back"].map(|side| format!("the {end}'s {side} edge"));
        Plate::raised(
            &format!("the plate whose {end}'s tip stands at ({x}, {RADIUS})"),
            vec![
                DVec2::ZERO,
                DVec2::new(LENGTH, 0.0),
                DVec2::new(x, RADIUS),
                DVec2::new(LENGTH, WIDTH),
                DVec2::new(0.0, WIDTH),
            ],
            &[
                (0, "the top's front edge"),
                (1, front.as_str()),
                (2, back.as_str()),
                (3, "the top's back edge"),
            ],
            [0, 3],
            vec![crown],
        )
    }

    /// The corner `corner` of the outline, counted round it.
    fn corner(&self, corner: usize) -> DVec2 {
        self.outline[corner % self.outline.len()]
    }

    /// Where the list names the top edge of run `run`, if it does.
    fn edge_of(&self, run: usize) -> Option<usize> {
        self.edges.iter().position(|&(named, _)| named == run)
    }

    /// The wall run `run` raises.
    fn wall(&self, run: usize) -> Wall {
        Wall {
            number: 2 + run,
            called: self.edge_of(run).map_or_else(
                || format!("the top edge of wall {}", 2 + run),
                |edge| self.edges[edge].1.clone(),
            ),
            from: self.corner(run),
            to: self.corner(run + 1),
        }
    }

    /// The walls whose top edges the list names, in the order the plate lists
    /// them.
    fn walls(&self) -> Vec<Wall> {
        self.edges.iter().map(|&(run, _)| self.wall(run)).collect()
    }

    /// The runs on either side of run `run`, before it and after it.
    fn runs_beside(&self, run: usize) -> [usize; 2] {
        let count = self.outline.len();
        [(run + count - 1) % count, (run + 1) % count]
    }

    /// The angle the outline makes inside the plate at corner `corner`: less
    /// than a half turn where it turns left, more where it turns back in.
    fn angle_at(&self, corner: usize) -> f64 {
        let count = self.outline.len();
        let here = self.corner(corner);
        let coming = here - self.corner(corner + count - 1);
        let going = self.corner(corner + 1) - here;
        PI - coming.perp_dot(going).atan2(coming.dot(going))
    }

    /// The cotangent the side of a share leans at, at the start or the end of
    /// run `run`: of half the angle there where the run beside is named too,
    /// the mitre bisecting the corner; of the whole angle where it is not, the
    /// share running along the wall beside.
    fn lean(&self, run: usize, at_its_end: bool) -> f64 {
        let [before, after] = self.runs_beside(run);
        let (corner, beside) = if at_its_end {
            (run + 1, after)
        } else {
            (run, before)
        };
        let angle = self.angle_at(corner);
        if self.edge_of(beside).is_some() {
            1.0 / (angle / 2.0).tan()
        } else {
            1.0 / angle.tan()
        }
    }

    /// Where the share of run `run` closes at its start or its end, seen from
    /// above: one radius in from its wall, along the side that leans from
    /// that corner.
    fn closing(&self, run: usize, at_its_end: bool) -> DVec2 {
        let wall = self.wall(run);
        let along = (wall.to - wall.from).normalize();
        let inward = along.perp() * RADIUS;
        let lean = along * (RADIUS * self.lean(run, at_its_end));
        if at_its_end {
            wall.to - lean + inward
        } else {
            wall.from + lean + inward
        }
    }

    /// The share of the top the round of the list's edge `edge` is laid over:
    /// its wall, then where it closes at the wall's end and at its start, one
    /// corner where the two meet.
    fn share(&self, edge: usize) -> Share {
        let run = self.edges[edge].0;
        let wall = self.wall(run);
        let [at_the_start, at_the_end] = [false, true].map(|end| self.closing(run, end));
        let mut corners = vec![wall.from, wall.to, at_the_end];
        if at_the_start.distance(at_the_end) > PLACE {
            corners.push(at_the_start);
        }
        Share { corners }
    }

    /// The shares of every edge the list names, in the order the plate lists
    /// them.
    fn shares(&self) -> Vec<Share> {
        (0..self.edges.len()).map(|edge| self.share(edge)).collect()
    }

    /// The mitres: one at each corner of the outline between two walls whose
    /// top edges are named, running up to where both their shares close.
    fn mitres(&self) -> Vec<Mitre> {
        (0..self.outline.len())
            .filter_map(|corner| {
                let [before, _] = self.runs_beside(corner);
                let edges = [self.edge_of(before)?, self.edge_of(corner)?];
                Some(Mitre {
                    edges,
                    corner: self.corner(corner),
                    crown: self.closing(corner, false).extend(HEIGHT),
                    called: format!("the corner at {}", self.corner(corner)),
                })
            })
            .collect()
    }

    /// The faces the rounds make, as the list's edges each answers to: the
    /// full round, both its edges, then every other edge alone.
    fn faces(&self) -> Vec<Vec<usize>> {
        let full: Vec<usize> = self
            .full_round
            .iter()
            .map(|&run| self.edge_of(run).expect("the full round's edges are named"))
            .collect();
        let mut faces = vec![full.clone()];
        faces.extend(
            (0..self.edges.len())
                .filter(|edge| !full.contains(edge))
                .map(|edge| vec![edge]),
        );
        faces
    }

    /// The walls beside the list's edge `edge` whose top edges it does not
    /// name, by their numbers.
    fn unnamed_beside(&self, edge: usize) -> Vec<usize> {
        self.runs_beside(self.edges[edge].0)
            .into_iter()
            .filter(|&run| self.edge_of(run).is_none())
            .map(|run| 2 + run)
            .collect()
    }

    /// The area of the outline.
    fn outline_area(&self) -> f64 {
        (0..self.outline.len())
            .map(|corner| self.corner(corner).perp_dot(self.corner(corner + 1)))
            .sum::<f64>()
            / 2.0
    }

    /// The length of the lines at `t` from their walls across the shares,
    /// seen from above: along each named wall, its length less `t` times the
    /// leans at its two ends.
    fn shares_at(&self, t: f64) -> f64 {
        self.edges
            .iter()
            .map(|&(run, _)| {
                let wall = self.wall(run);
                wall.from.distance(wall.to) - t * (self.lean(run, false) + self.lean(run, true))
            })
            .sum()
    }

    /// What the plate keeps once its named edges are rounded at [`RADIUS`]
    /// and mitred, as the module's documentation derives it: the whole
    /// outline up to the rounds' axes, and over them a quarter round swept
    /// along the shares, from the walls up to the axes.
    fn rounded(&self) -> f64 {
        (HEIGHT - RADIUS) * self.outline_area()
            + quarter_round_swept_along(self.shares_at(0.0), self.shares_at(RADIUS))
    }

    /// What the plate keeps once its named edges are chamfered at [`RADIUS`]
    /// into a hip, as the module's documentation derives it: the whole outline
    /// up to the flats' feet, and over them a flat swept along the shares,
    /// from the walls up to the ridge.
    fn hipped(&self) -> f64 {
        (HEIGHT - RADIUS) * self.outline_area()
            + flat_swept_along(self.shares_at(0.0), self.shares_at(RADIUS))
    }

    /// The names of the list's edges, in `order`, as indices into
    /// [`Plate::edges`].
    fn names(&self, order: &[usize]) -> Vec<EdgeName> {
        order
            .iter()
            .map(|&edge| self.wall(self.edges[edge].0).name(&self.part))
            .collect()
    }

    /// The fillet at [`RADIUS`] on the list's edges, named in `order`.
    fn filleted(&self, order: &[usize]) -> Result<Filleted, Declined> {
        self.part.fillet_edges(&self.names(order), RADIUS)
    }

    /// The chamfer at [`RADIUS`] on the list's edges, named in `order`.
    fn chamfered(&self, order: &[usize]) -> Result<Chamfered, Declined> {
        self.part.chamfer_edges(&self.names(order), RADIUS)
    }

    /// What the `cut` — a fillet or a chamfer — on the edges named in `order`
    /// is called.
    fn called(&self, cut: &str, order: &[usize]) -> String {
        let called: Vec<&str> = order
            .iter()
            .map(|&edge| self.edges[edge].1.as_str())
            .collect();
        format!(
            "the {cut} at {RADIUS} on {}, on {}",
            called.join(", then "),
            self.called
        )
    }
}

/// Holds that the arithmetic of the shares gives the plate's rounded body the
/// volume the module's documentation writes out for it.
fn the_shares_give(plate: &Plate, volume: f64) {
    assert!(
        (plate.rounded() - volume).abs() <= EXACT * volume,
        "{}: the shares give {}, where the documentation writes {volume}",
        plate.called,
        plate.rounded()
    );
}

/// What a right-angled triangle whose legs are [`RADIUS`] holds swept along
/// lines whose length runs evenly from `at_the_walls` up to `at_the_ridge`:
/// half the radius squared along `at_the_ridge`, and a sixth of the radius
/// cubed for each unit the line shrinks per unit climbed.
fn flat_swept_along(at_the_walls: f64, at_the_ridge: f64) -> f64 {
    let r = RADIUS;
    at_the_ridge * r * r / 2.0 + (at_the_walls - at_the_ridge) * r * r / 6.0
}

/// What a quarter round of [`RADIUS`] holds swept along lines whose length
/// runs evenly from `at_the_walls` up to `at_the_axes`: the quarter disc
/// along `at_the_axes`, and a third of the radius cubed for each unit the
/// line shrinks per unit climbed.
fn quarter_round_swept_along(at_the_walls: f64, at_the_axes: f64) -> f64 {
    let r = RADIUS;
    at_the_axes * PI * r * r / 4.0 + (at_the_walls - at_the_axes) / r * r.powi(3) / 3.0
}

/// The prism the full round takes away, raised whole along X past both ends
/// of the plate: all that stands over the front and the back, along y = 0 and
/// y = [`WIDTH`], outside the half-cylinder about the line [`RADIUS`] under
/// the middle of the top.
fn full_round_tool(plate: &Plate) -> Body {
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
    let [left, right] = [f64::min, f64::max].map(|bound| {
        plate
            .outline
            .iter()
            .map(|corner| corner.x)
            .fold(plate.outline[0].x, bound)
    });
    plate
        .part
        .tool_raised(
            &profile(Contour { corners, runs }),
            Frame {
                origin: DVec3::new(left - 2.0, 0.0, 0.0),
                u: DVec3::Y,
                v: DVec3::Z,
            },
            DVec3::X * (right - left + 4.0),
        )
        .expect("the tool raises")
}

/// What the plate keeps once its named edges are rounded at [`RADIUS`],
/// computed apart from the step: the full round's prism, then each other
/// round's, raised whole along its wall's line and kept on its own side of
/// each mitre it shares with another such round, taken away by the booleans.
fn rounded_by_booleans(plate: &Plate) -> f64 {
    let walls = plate.walls();
    let faces = plate.faces();
    let full = &faces[0];
    let mitres = plate.mitres();
    let rounded = plate
        .part
        .difference(&full_round_tool(plate))
        .expect("the full round's prism cuts the plate");
    (0..walls.len())
        .filter(|edge| !full.contains(edge))
        .fold(rounded, |cut, edge| {
            let prism = mitres
                .iter()
                .filter(|mitre| {
                    mitre.edges.contains(&edge) && mitre.edges.iter().all(|one| !full.contains(one))
                })
                .fold(walls[edge].tool(&cut), |prism, mitre| {
                    let (normal, offset) = mitre.kept_side_of(&walls[edge]);
                    prism.behind(normal, offset)
                });
            cut.difference(&prism)
                .expect("a round's prism cuts the plate")
        })
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

/// The orders of a plate pointed at both ends whose six edges are listed as
/// the front, the right point's front and back, the back, the left point's
/// back and front: every order of each end's four edges — the front, the
/// back and that end's two point edges — the two ends' point edges named side
/// by side, the right's first, in both pairings of the right's with the
/// left's. All 720 orders take minutes, and are the campaign's; these hold
/// every order the four edges at either end can be named in, twice.
fn each_end_in_every_order(count: usize) -> Vec<Vec<usize>> {
    assert_eq!(count, 6, "a plate pointed at both ends lists six edges");
    let [front, right_front, right_back, back, left_back, left_front] = [0, 1, 2, 3, 4, 5];
    let pairings = [
        [[right_front, left_front], [right_back, left_back]],
        [[right_front, left_back], [right_back, left_front]],
    ];
    pairings
        .into_iter()
        .flat_map(|[first, second]| {
            every_order(4).into_iter().map(move |at_one_end| {
                at_one_end
                    .into_iter()
                    .flat_map(|place| match place {
                        0 => vec![front],
                        1 => first.to_vec(),
                        2 => second.to_vec(),
                        _ => vec![back],
                    })
                    .collect()
            })
        })
        .collect()
}

/// The one order the plate lists its `count` edges in, the issue's where it
/// names one.
fn the_listed_order(count: usize) -> Vec<Vec<usize>> {
    vec![(0..count).collect()]
}

/// Each face of a body and the sides of the faces it meets, one for each edge
/// it has, every face laid on the edges named in `order` read by the edge it
/// was laid on, as the rank the plate's listed order gives that edge, and a
/// patch numbered past every edge's rank read as itself — the one corner a
/// plate here closes with a patch, its tip with the upright named, has the
/// same number whatever the order: what tells two bodies the same face for
/// face, whatever the order their edges were named in.
fn meetings(body: &Body, start: usize, order: &[usize]) -> BTreeMap<Vec<u32>, Vec<Vec<u32>>> {
    let as_listed = |side: Vec<u32>| -> Vec<u32> {
        let mut read: Vec<u32> = side
            .into_iter()
            .map(|number| {
                let number = number as usize;
                if number < start || number >= start + order.len() {
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
                "face {face:?} meets {:?}, where the order it is read against lays it \
                 meeting {:?}",
                own.get(face),
                theirs.get(face)
            )
        })
        .collect()
}

/// Holds that a body whose faces meet as those of `reference`, the one laid
/// with the edges named in the plate's listed order, is that body: the same
/// matter, and the plate's flat faces at the same corners.
fn same_body_as(plate: &Plate, body: &Body, reference: &Body, what: &str) {
    assert!(
        (body.volume() - reference.volume()).abs() <= EXACT * reference.volume(),
        "{what}: the body holds {}, where the listed order lays one of {}",
        body.volume(),
        reference.volume()
    );
    for face in numbers(&plate.part) {
        let [own, theirs] =
            [body, reference].map(|laid| laid.plane_of(face).map(|plane| plane.corners));
        let alike = match (&own, &theirs) {
            (Some(own), Some(theirs)) => same_points(own, theirs),
            (None, None) => true,
            _ => false,
        };
        assert!(
            alike,
            "{what}: face {face} has its corners at {own:?}, where the listed order leaves them \
             at {theirs:?}"
        );
    }
}

/// Holds that the fillet at [`RADIUS`] on the plate's listed edges, named in
/// `order` and laid as `filleted`, takes the value whole and lays every round
/// at that radius, each mitred against its neighbours up to its crown: the
/// body exact, closed, uncrossed and drawn, holding what the arithmetic
/// promises; the full round answering to both its edges' numbers and every
/// other round to its own, the top gone and nothing laid at a crown; each
/// round drawn on its own cylinder over its own share of the top, and met
/// there from outside; each mitre running up to one of the plate's crowns,
/// its two rounds drawn up to both its ends, from the corner their walls share
/// up to the crown, and meeting nowhere off the vertical plane through the
/// two; the rounds whose shares reach a crown meeting at one vertex there;
/// each round meeting its walls, its neighbours across its mitres and the
/// walls not named it runs into, by one edge each; and the walls set back by
/// the radius under them.
fn mitres_each_round_against_its_neighbours(
    plate: &Plate,
    order: &[usize],
    filleted: &Filleted,
    what: &str,
) {
    let r = RADIUS;
    let part = &plate.part;
    let start = part.faces_end();
    let walls = plate.walls();
    let count = walls.len();
    let shares = plate.shares();
    let mitres = plate.mitres();
    let faces = plate.faces();
    let rounds: Vec<Round> = walls
        .iter()
        .zip(&shares)
        .map(|(wall, share)| wall.round_over(share))
        .collect();

    taken_as_asked(filleted.asked, filleted.taken, r, what);
    let body = &filleted.body;
    let area: f64 = rounds.iter().map(Round::area).sum();
    sound_drawn_within(body, plate.rounded(), DRAWN * reach_of(part) * area, what);

    let rank_of = |edge: usize| {
        start
            + order
                .iter()
                .position(|&named| named == edge)
                .expect("every edge is named")
    };
    let face_holding = |edge: usize| {
        faces
            .iter()
            .find(|face| face.contains(&edge))
            .expect("every edge's round is one face")
    };
    let side_of = |edge: usize| -> Vec<usize> {
        let mut side: Vec<usize> = face_holding(edge)
            .iter()
            .map(|&each| rank_of(each))
            .collect();
        side.sort();
        side
    };
    let number_of = |edge: usize| side_of(edge)[0];
    let round_called = |edge: usize| {
        if face_holding(edge).len() > 1 {
            "the full round".to_string()
        } else {
            format!("the round of {}", walls[edge].called)
        }
    };

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
        "{what}: every face of the plate but its top, the full round answering to both its \
         edges' ranks and every other round to its own, and no face laid at a crown"
    );
    assert_eq!(
        body.faces_end(),
        start + count,
        "{what}: the next step numbers its faces past the {count} the list gave its edges"
    );
    for face in &faces {
        let laid = side_of(face[0]);
        if face.len() > 1 {
            one_curved_face_answers_to(body, &laid, &format!("{what}: the full round"));
        } else {
            assert!(
                !body.is_flat(laid[0]) && body.plane_of(laid[0]).is_none(),
                "{what}: {}, face {}, is curved, and offers no plane to draw on",
                round_called(face[0]),
                laid[0]
            );
        }
    }

    let drawn_of =
        |edge: usize| -> Vec<DVec3> { body.triangles_of(number_of(edge)).flatten().collect() };
    for face in &faces {
        let which = round_called(face[0]);
        let drawn = drawn_of(face[0]);
        assert!(!drawn.is_empty(), "{what}: {which} is drawn");
        for &point in &drawn {
            assert!(
                face.iter()
                    .any(|&edge| rounds[edge].holds(point)
                        && shares[edge].inside_by(point) >= -PLACE),
                "{what}: {which} is drawn through {point}, off the quarter round of {r} over \
                 the share of the top of each of its walls"
            );
        }
        for &edge in face {
            met_from_outside_where(
                body,
                number_of(edge),
                &rounds[edge],
                |aim| shares[edge].inside_by(aim) > CLEAR_OF_THE_MITRE,
                &format!(
                    "{what}: {which}, over the wall under {}, clear of its mitres",
                    walls[edge].called
                ),
            );
        }
    }

    for mitre in &mitres {
        assert!(
            plate
                .crowns
                .iter()
                .any(|crown| crown.distance(mitre.crown) < PLACE),
            "{what}: the arithmetic runs the mitre at {} up to {}, none of the crowns {:?}",
            mitre.called,
            mitre.crown,
            plate.crowns
        );
        let [one, other] = mitre.edges;
        let [one_drawn, other_drawn] = [drawn_of(one), drawn_of(other)];
        let foot = mitre.corner.extend(HEIGHT - r);
        for (point, at) in [(foot, mitre.called.as_str()), (mitre.crown, "the crown")] {
            for (drawn, which) in [
                (&one_drawn, round_called(one)),
                (&other_drawn, round_called(other)),
            ] {
                assert!(
                    drawn_through(drawn, point),
                    "{what}: {which} is drawn up to {at}, {point}, where the mitre at {} ends",
                    mitre.called
                );
            }
        }
        let between_the_two: Vec<&Mitre> = mitres
            .iter()
            .filter(|each| {
                let [first, second] = each.edges.map(face_holding);
                (first, second) == (face_holding(one), face_holding(other))
                    || (second, first) == (face_holding(one), face_holding(other))
            })
            .collect();
        for &point in one_drawn
            .iter()
            .filter(|&&point| drawn_through(&other_drawn, point))
        {
            assert!(
                between_the_two.iter().any(|each| each.off(point) < PLACE),
                "{what}: {} and {} meet at {point}, off the vertical plane through the corner \
                 and the crown of every mitre between them, the one at {} among them",
                round_called(one),
                round_called(other),
                mitre.called
            );
        }
    }

    for &crown in &plate.crowns {
        let mut at_the_crown: Vec<Vec<u32>> = faces
            .iter()
            .filter(|face| face.iter().any(|&edge| shares[edge].reaches(crown)))
            .map(|face| {
                side_of(face[0])
                    .into_iter()
                    .map(|number| number as u32)
                    .collect()
            })
            .collect();
        at_the_crown.sort();
        let vertices = body
            .vertex_names()
            .into_iter()
            .filter(|vertex| vertex.faces == at_the_crown)
            .count();
        assert_eq!(
            vertices, 1,
            "{what}: the rounds whose shares reach the crown {crown} meet at one vertex there"
        );
    }

    for face in &faces {
        let mut met: Vec<Vec<usize>> = face.iter().map(|&edge| vec![walls[edge].number]).collect();
        for mitre in &mitres {
            let [one, other] = mitre.edges;
            for (own, beside) in [(one, other), (other, one)] {
                if face.contains(&own) {
                    met.push(side_of(beside));
                }
            }
        }
        let unnamed: BTreeSet<usize> = face
            .iter()
            .flat_map(|&edge| plate.unnamed_beside(edge))
            .collect();
        met.extend(unnamed.into_iter().map(|number| vec![number]));
        let met: Vec<&[usize]> = met.iter().map(Vec::as_slice).collect();
        assert_eq!(
            sides_met_by(body, &side_of(face[0])),
            sides_answering(&met),
            "{what}: {} meets its walls along their feet, each round beside it along their \
             mitre, and each wall not named it runs into, by one edge each",
            round_called(face[0])
        );
    }

    for (edge, wall) in walls.iter().enumerate() {
        let [from, to] = [wall.from, wall.to];
        corners_are(
            body,
            wall.number,
            &[
                from.extend(0.0),
                to.extend(0.0),
                to.extend(HEIGHT - r),
                from.extend(HEIGHT - r),
            ],
            &format!(
                "{what}: the wall under {}, set back by the radius from the top",
                wall.called
            ),
        );
        let [before, after] = plate.runs_beside(plate.edges[edge].0).map(|run| 2 + run);
        assert_eq!(
            sides_met_by(body, &[wall.number]),
            sides_answering(&[&[FLOOR], &[before], &[after], &side_of(edge)]),
            "{what}: the wall under {} meets the floor, the walls on either side along its \
             uprights, and its round along its top",
            wall.called
        );
    }
}

/// What the fillet at [`RADIUS`] laid on plates' listed edges, order by
/// order: the orders declined, those taking less than the value asked, those
/// whose faces meet otherwise than the first order that took it whole.
struct Laid {
    declined: Vec<String>,
    short: Vec<String>,
    laid_otherwise: Vec<String>,
}

/// The fillet at [`RADIUS`] on each of `plates`' listed edges, in each order
/// `orders` gives for that many edges, after the booleans have been held to
/// give the matter the arithmetic does: every order taking the value whole
/// whose faces meet as the first such order's do held whole — the mitred
/// rounds, and the same body as that order's — and every other order read
/// into [`Laid`].
fn laid_in_each_order(plates: &[Plate], orders: fn(usize) -> Vec<Vec<usize>>) -> Laid {
    let mut laid = Laid {
        declined: Vec::new(),
        short: Vec::new(),
        laid_otherwise: Vec::new(),
    };
    for plate in plates {
        let count = plate.edges.len();
        let start = plate.part.faces_end();
        let promised = plate.rounded();
        let by_booleans = rounded_by_booleans(plate);
        assert!(
            (by_booleans - promised).abs() <= EXACT * promised,
            "{}: each round's prism, cut from the plate by the booleans, leaves {by_booleans}, \
             where arithmetic promises {promised}",
            plate.called
        );
        let mut reference: Option<(Vec<usize>, Filleted)> = None;
        for order in orders(count) {
            let what = plate.called("fillet", &order);
            let filleted = match plate.filleted(&order) {
                Ok(filleted) => filleted,
                Err(reason) => {
                    laid.declined.push(format!("{what}: declined, {reason:?}"));
                    continue;
                }
            };
            if (filleted.taken - RADIUS).abs() >= PLACE {
                laid.short.push(format!("{what}: took {}", filleted.taken));
                continue;
            }
            if let Some((reference_order, reference)) = &reference {
                let otherwise = met_otherwise(
                    start,
                    &order,
                    &filleted.body,
                    reference_order,
                    &reference.body,
                );
                if !otherwise.is_empty() {
                    laid.laid_otherwise
                        .push(format!("{what}:\n  {}", otherwise.join("\n  ")));
                    continue;
                }
            }
            mitres_each_round_against_its_neighbours(plate, &order, &filleted, &what);
            match &reference {
                Some((_, reference)) => {
                    same_body_as(plate, &filleted.body, &reference.body, &what);
                }
                None => reference = Some((order, filleted)),
            }
        }
    }
    laid
}

/// Holds, for each of `plates`, that the fillet at [`RADIUS`] on its listed
/// edges lays in each order `orders` gives for that many edges one body, the
/// mitred rounds, as [`laid_in_each_order`] holds it: no order declined, none
/// taking less than the value asked, none whose faces meet otherwise.
fn lays_one_mitred_body_in_each_order(plates: &[Plate], orders: fn(usize) -> Vec<Vec<usize>>) {
    let laid = laid_in_each_order(plates, orders);
    assert!(
        laid.declined.is_empty() && laid.short.is_empty() && laid.laid_otherwise.is_empty(),
        "the fillet at {RADIUS} on a full round and on the edges of the plate's ends, each \
         mitred against its neighbours, lays one body whichever edge the list names first; \
         declined:\n{}\ntaking less than the value asked:\n{}\nits faces meeting otherwise \
         than in the first order taking the value whole:\n{}",
        laid.declined.join("\n"),
        laid.short.join("\n"),
        laid.laid_otherwise.join("\n")
    );
}

/// Holds that every order of `listed`'s edges, each the edge between the two
/// faces numbered, named in a fillet at [`RADIUS`] on `part`, is a broken
/// step and its reason, `Unsupported`; `what` says what the list is.
fn declined_in_every_order(part: &Body, listed: &[([usize; 2], &str)], what: &str) {
    let not_declined: Vec<String> = every_order(listed.len())
        .into_iter()
        .filter_map(|order| {
            let names: Vec<EdgeName> = order
                .iter()
                .map(|&edge| {
                    let [one, other] = listed[edge].0;
                    edge_between(part, one, other)
                })
                .collect();
            let called: Vec<&str> = order.iter().map(|&edge| listed[edge].1).collect();
            let called = called.join(", then ");
            match part.fillet_edges(&names, RADIUS) {
                Err(Declined::Unsupported) => None,
                Ok(filleted) => Some(format!(
                    "the fillet on {called}: a body of {}, taken {}",
                    filleted.body.volume(),
                    filleted.taken
                )),
                Err(declined) => Some(format!("the fillet on {called}: declined, {declined:?}")),
            }
        })
        .collect();
    assert!(
        not_declined.is_empty(),
        "the fillet at {RADIUS} on {what} is a broken step and its reason, `Unsupported`, \
         whatever the order; not so:\n{}",
        not_declined.join("\n")
    );
}

/// Holds that the chamfer at [`RADIUS`] on the plate's listed edges, named in
/// `order` and laid as `chamfered`, takes the value whole and lays a hip: the
/// body exact, closed and uncrossed, holding what the arithmetic promises;
/// one flat for each edge, numbered in the order the list names them, the top
/// gone; each flat over its own share of the top, from its wall's two ends one
/// radius under the top up to where its share closes, and facing halfway
/// between the top's way and its wall's; each flat meeting its wall, its
/// neighbours across their mitres, the other flat of the front and back
/// across the ridge, and the walls not named it runs into, by one edge each;
/// the flats whose shares reach a crown meeting at one vertex there; and the
/// walls set back by the distance under them.
fn flattens_into_a_hip(plate: &Plate, order: &[usize], chamfered: &Chamfered, what: &str) {
    let r = RADIUS;
    let part = &plate.part;
    let start = part.faces_end();
    let walls = plate.walls();
    let count = walls.len();
    let shares = plate.shares();
    let mitres = plate.mitres();
    let faces = plate.faces();
    let ridge = &faces[0];

    taken_as_asked(chamfered.asked, chamfered.taken, r, what);
    let body = &chamfered.body;
    let promised = plate.hipped();
    sound_drawn_within(body, promised, EXACT * promised, what);

    let flat_of = |edge: usize| {
        start
            + order
                .iter()
                .position(|&named| named == edge)
                .expect("every edge is named")
    };
    assert!(
        !body.has_face(TOP),
        "{what}: the top is gone, the front's and the back's flats meeting in a ridge over it"
    );
    let mut kept_and_laid = numbers(part);
    kept_and_laid.remove(&TOP);
    kept_and_laid.extend(start..start + count);
    assert_eq!(
        numbers(body),
        kept_and_laid,
        "{what}: every face of the plate but its top, one flat for each edge, numbered in the \
         order the list names them, and no face laid at a crown"
    );
    assert_eq!(
        body.faces_end(),
        start + count,
        "{what}: the next step numbers its faces past the {count} the list gave its edges"
    );

    for (edge, wall) in walls.iter().enumerate() {
        let flat = flat_of(edge);
        let which = format!("{what}: the flat of {}", wall.called);
        let inward = -wall.outward().truncate();
        let lifted: Vec<DVec3> = shares[edge]
            .corners
            .iter()
            .map(|&corner| corner.extend(HEIGHT - r + (corner - wall.from).dot(inward)))
            .collect();
        corners_are(
            body,
            flat,
            &lifted,
            &format!("{which}, over its share of the top"),
        );
        faces_toward(body, flat, (wall.outward() + DVec3::Z).normalize(), &which);
        let mut met: Vec<Vec<usize>> = vec![vec![wall.number]];
        for mitre in &mitres {
            let [one, other] = mitre.edges;
            for (own, beside) in [(one, other), (other, one)] {
                if own == edge {
                    met.push(vec![flat_of(beside)]);
                }
            }
        }
        if ridge.contains(&edge) {
            met.extend(
                ridge
                    .iter()
                    .filter(|&&other| other != edge)
                    .map(|&other| vec![flat_of(other)]),
            );
        }
        met.extend(
            plate
                .unnamed_beside(edge)
                .into_iter()
                .map(|number| vec![number]),
        );
        let met: Vec<&[usize]> = met.iter().map(Vec::as_slice).collect();
        assert_eq!(
            sides_met_by(body, &[flat]),
            sides_answering(&met),
            "{which} meets its wall along its foot, the flats beside it — across the ridge \
             along the middle line and the hips at the corners — and the walls not named it runs \
             into, by one edge each"
        );

        let [from, to] = [wall.from, wall.to];
        corners_are(
            body,
            wall.number,
            &[
                from.extend(0.0),
                to.extend(0.0),
                to.extend(HEIGHT - r),
                from.extend(HEIGHT - r),
            ],
            &format!(
                "{what}: the wall under {}, set back by the distance from the top",
                wall.called
            ),
        );
        let [before, after] = plate.runs_beside(plate.edges[edge].0).map(|run| 2 + run);
        assert_eq!(
            sides_met_by(body, &[wall.number]),
            sides_answering(&[&[FLOOR], &[before], &[after], &[flat]]),
            "{what}: the wall under {} meets the floor, the walls on either side along its \
             uprights, and its flat along its top",
            wall.called
        );
    }

    for &crown in &plate.crowns {
        let mut at_the_crown: Vec<Vec<u32>> = (0..count)
            .filter(|&edge| shares[edge].reaches(crown))
            .map(|edge| vec![flat_of(edge) as u32])
            .collect();
        at_the_crown.sort();
        let vertices = body
            .vertex_names()
            .into_iter()
            .filter(|vertex| vertex.faces == at_the_crown)
            .count();
        assert_eq!(
            vertices, 1,
            "{what}: the flats whose shares reach the crown {crown} meet at one vertex there"
        );
    }
}

/// Holds, for each of `plates`, that the chamfer at [`RADIUS`] on its listed
/// edges lays in each order `orders` gives for that many edges the one hip
/// the listed order lays: every body whose faces meet as that order's do held
/// whole, and its flats at the corners of that order's, each read by the edge
/// it was laid on; then every order declined, and every one whose faces meet
/// otherwise, listed together.
fn chamfers_into_one_hip_in_each_order(plates: &[Plate], orders: fn(usize) -> Vec<Vec<usize>>) {
    let mut declined = Vec::new();
    let mut laid_otherwise = Vec::new();
    for plate in plates {
        let count = plate.edges.len();
        let start = plate.part.faces_end();
        let listed: Vec<usize> = (0..count).collect();
        let reference = plate.chamfered(&listed);
        for order in orders(count) {
            let what = plate.called("chamfer", &order);
            let chamfered = match plate.chamfered(&order) {
                Ok(chamfered) => chamfered,
                Err(reason) => {
                    declined.push(format!("{what}: declined, {reason:?}"));
                    continue;
                }
            };
            if let Ok(reference) = &reference {
                let otherwise =
                    met_otherwise(start, &order, &chamfered.body, &listed, &reference.body);
                if !otherwise.is_empty() {
                    laid_otherwise.push(format!("{what}:\n  {}", otherwise.join("\n  ")));
                    continue;
                }
            }
            flattens_into_a_hip(plate, &order, &chamfered, &what);
            if let Ok(reference) = &reference {
                same_body_as(plate, &chamfered.body, &reference.body, &what);
                for edge in 0..count {
                    let laid = start
                        + order
                            .iter()
                            .position(|&named| named == edge)
                            .expect("every edge is named");
                    let [own, theirs] = [(&chamfered.body, laid), (&reference.body, start + edge)]
                        .map(|(body, flat)| body.plane_of(flat).map(|plane| plane.corners));
                    assert!(
                        matches!((&own, &theirs), (Some(own), Some(theirs)) if same_points(own, theirs)),
                        "{what}: the flat of {} has its corners at {own:?}, where the listed \
                         order lays them at {theirs:?}",
                        plate.edges[edge].1
                    );
                }
            }
        }
    }
    assert!(
        declined.is_empty() && laid_otherwise.is_empty(),
        "the chamfer at {RADIUS} on the top's front and back edges and on the two edges of the \
         point lays one hip whichever edge the list names first, the one the listed order \
         lays; declined:\n{}\nits faces meeting otherwise than in the listed order:\n{}",
        declined.join("\n"),
        laid_otherwise.join("\n")
    );
}

#[test]
fn a_fillet_on_a_full_round_and_the_two_edges_of_its_pointed_end_mitres_each_point_round_against_its_neighbours_up_to_the_crown()
 {
    let plate = Plate::pointed(33.0, DVec3::new(33.0 - 3.0 * SQRT_2, 3.0, 10.0));
    the_shares_give(
        &plate,
        1305.0 + 36.0 * SQRT_2 + (66.0 - 6.0 * SQRT_2) * 9.0 * PI / 4.0,
    );
    lays_one_mitred_body_in_each_order(&[plate], the_listed_order);
}

#[test]
fn a_fillet_on_a_full_round_and_the_two_edges_of_its_pointed_end_lays_the_same_body_whatever_the_order()
 {
    lays_one_mitred_body_in_each_order(
        &[Plate::pointed(
            33.0,
            DVec3::new(33.0 - 3.0 * SQRT_2, 3.0, 10.0),
        )],
        every_order,
    );
}

#[test]
fn a_fillet_on_a_full_round_and_the_two_edges_of_a_v_notched_end_mitres_each_notch_edge_s_round_against_its_neighbours_down_to_the_notch_whatever_the_order()
 {
    let notched = Plate::pointed(27.0, DVec3::new(27.0 - 3.0 * SQRT_2, 3.0, 10.0));
    the_shares_give(
        &notched,
        1215.0 + 36.0 * SQRT_2 + (54.0 - 6.0 * SQRT_2) * 9.0 * PI / 4.0,
    );
    lays_one_mitred_body_in_each_order(
        &[
            notched,
            Plate::pointed(28.0, DVec3::new(28.0 - 13f64.sqrt(), 3.0, 10.0)),
        ],
        every_order,
    );
}

#[test]
fn a_fillet_on_a_full_round_and_the_two_edges_of_a_sharper_or_a_blunter_point_mitres_each_point_round_against_its_neighbours_up_to_the_crown()
 {
    lays_one_mitred_body_in_each_order(
        &[
            Plate::pointed(39.0, DVec3::new(39.0 - 3.0 * 10f64.sqrt(), 3.0, 10.0)),
            Plate::pointed(32.0, DVec3::new(32.0 - 13f64.sqrt(), 3.0, 10.0)),
        ],
        every_order,
    );
}

#[test]
fn a_chamfer_on_a_full_round_pair_s_edges_and_the_two_edges_of_a_pointed_end_stays_a_hip_whose_flats_meet_at_the_crown()
 {
    chamfers_into_one_hip_in_each_order(
        &[
            Plate::pointed(33.0, DVec3::new(33.0 - 3.0 * SQRT_2, 3.0, 10.0)),
            Plate::pointed(39.0, DVec3::new(39.0 - 3.0 * 10f64.sqrt(), 3.0, 10.0)),
            Plate::pointed(32.0, DVec3::new(32.0 - 13f64.sqrt(), 3.0, 10.0)),
        ],
        every_order,
    );
}

#[test]
fn the_blunt_the_lopsided_and_the_bevelled_ends_are_still_filleted_at_the_value_main_takes() {
    let ends_of = |end: &[DVec2]| {
        let mut outline = vec![DVec2::ZERO];
        outline.extend_from_slice(end);
        outline.push(DVec2::new(0.0, WIDTH));
        outline
    };
    let pointed = |tip: DVec2| ends_of(&[DVec2::new(LENGTH, 0.0), tip, DVec2::new(LENGTH, WIDTH)]);
    let bevelled = ends_of(&[
        DVec2::new(29.0, 0.0),
        DVec2::new(30.0, 1.0),
        DVec2::new(30.0, 5.0),
        DVec2::new(29.0, WIDTH),
    ]);
    let cases = [
        (
            "the blunt point, its tip at (31, 3)",
            pointed(DVec2::new(31.0, 3.0)),
            vec![2, 3, 4, 5],
            6.0 / 10f64.sqrt(),
        ),
        (
            "the lopsided point, its tip at (33, 2)",
            pointed(DVec2::new(33.0, 2.0)),
            vec![2, 3, 4, 5],
            2.0,
        ),
        (
            "the bevelled end, its end edge named",
            bevelled.clone(),
            vec![2, 3, 4, 5, 6],
            1.0,
        ),
        (
            "the bevelled end, its end edge left out",
            bevelled,
            vec![2, 3, 5, 6],
            1.0,
        ),
    ];

    for (called, outline, walls, value) in cases {
        let part = raised(Contour::straight(outline), HEIGHT);
        let what = format!(
            "the fillet at {RADIUS} on {called}, the top edges of walls {walls:?} named in turn"
        );
        let names: Vec<EdgeName> = walls
            .iter()
            .map(|&wall| edge_between(&part, TOP, wall))
            .collect();

        let filleted = part
            .fillet_edges(&names, RADIUS)
            .unwrap_or_else(|declined| panic!("{what}: declined, {declined:?}"));

        assert!(
            (filleted.asked - RADIUS).abs() < PLACE && (filleted.taken - value).abs() < PLACE,
            "{what}: asked {} and took {}, where main takes {value}",
            filleted.asked,
            filleted.taken
        );
        let body = &filleted.body;
        assert!(
            body.is_exact(),
            "{what}: the exact kernel computed the body"
        );
        let triangles = body.triangles();
        assert_eq!(closed(&triangles), Ok(()), "{what}");
        assert_eq!(uncrossed(&triangles), Ok(()), "{what}");
    }
}

#[test]
fn a_fillet_on_a_full_round_the_two_edges_of_a_v_notched_end_and_the_upright_at_the_notch_s_bottom_is_declined_as_unsupported_whatever_the_order()
 {
    let plate = Plate::pointed(27.0, DVec3::ZERO);
    declined_in_every_order(
        &plate.part,
        &[
            ([TOP, FRONT], "the top's front edge"),
            ([TOP, POINT_FRONT], "the notch's front edge"),
            ([TOP, POINT_BACK], "the notch's back edge"),
            ([TOP, BACK], "the top's back edge"),
            (
                [POINT_FRONT, POINT_BACK],
                "the upright at the notch's bottom",
            ),
        ],
        &format!(
            "a full round, the two edges of the notch and the upright where they meet, on {}, \
             until #914 builds it",
            plate.called
        ),
    );
}

#[test]
fn a_fillet_on_a_full_round_the_two_edges_of_its_pointed_end_and_the_upright_at_either_shoulder_is_declined_as_unsupported_whatever_the_order()
 {
    let plate = Plate::pointed(33.0, DVec3::ZERO);
    let top_edges = [
        ([TOP, FRONT], "the top's front edge"),
        ([TOP, POINT_FRONT], "the point's front edge"),
        ([TOP, POINT_BACK], "the point's back edge"),
        ([TOP, BACK], "the top's back edge"),
    ];
    let shoulders = [
        (
            [FRONT, POINT_FRONT],
            "the upright at the front shoulder, (30, 0)",
        ),
        (
            [POINT_BACK, BACK],
            "the upright at the back shoulder, (30, 6)",
        ),
    ];
    for shoulder in shoulders {
        let listed: Vec<([usize; 2], &str)> = top_edges.into_iter().chain([shoulder]).collect();
        declined_in_every_order(
            &plate.part,
            &listed,
            &format!(
                "a full round, the two edges of the point and {}, whose third edge is the \
                 point's and meets one edge of the pair alone, on {}",
                shoulder.1, plate.called
            ),
        );
    }
}

/// Where the list of a [`Tip`] names the upright at the tip: after the four
/// top edges, as [`Plate::pointed`] lists them.
const AT_THE_TIP: usize = 4;

/// #843's pointed plate with the upright at its tip named after its four top
/// edges: the plate, and the centre of the piece of sphere closing the tip,
/// where the issue puts it. The point's two top edges and that upright meet at
/// the tip, three convex rounds, and #699 closes such a corner by a piece of
/// sphere of [`RADIUS`] about the point one radius from its three faces.
struct Tip {
    plate: Plate,
    centre: DVec3,
}

impl Tip {
    /// The plate whose point's tip stands at `x` on the middle line, the piece
    /// of sphere at its tip about `centre`, one radius under the crown #843's
    /// mitres meet at.
    fn at(x: f64, centre: DVec3) -> Tip {
        Tip {
            plate: Plate::pointed(x, centre + DVec3::Z * RADIUS),
            centre,
        }
    }

    /// The list's five edges, each as the two faces it lies between, with
    /// what it is called.
    fn listed(&self) -> Vec<([usize; 2], String)> {
        self.plate
            .edges
            .iter()
            .map(|(run, called)| ([TOP, 2 + run], called.clone()))
            .chain([(
                [POINT_FRONT, POINT_BACK],
                "the upright at the tip".to_string(),
            )])
            .collect()
    }

    /// The fillet at [`RADIUS`] on the list's edges, named in `order`.
    fn filleted(&self, order: &[usize]) -> Result<Filleted, Declined> {
        let listed = self.listed();
        let names: Vec<EdgeName> = order
            .iter()
            .map(|&edge| {
                let [one, other] = listed[edge].0;
                edge_between(&self.plate.part, one, other)
            })
            .collect();
        self.plate.part.fillet_edges(&names, RADIUS)
    }

    /// What the fillet on the edges named in `order` is called.
    fn called(&self, order: &[usize]) -> String {
        let listed = self.listed();
        let called: Vec<&str> = order.iter().map(|&edge| listed[edge].1.as_str()).collect();
        format!(
            "the fillet at {RADIUS} on {}, on {}",
            called.join(", then "),
            self.plate.called
        )
    }

    /// The point's two walls, the front one then the back one.
    fn point_walls(&self) -> [Wall; 2] {
        [self.plate.wall(1), self.plate.wall(2)]
    }

    /// The ways out of the point's two walls, front then back, level: the
    /// ways the upright's round faces where it leaves them.
    fn out_of_the_point(&self) -> [DVec3; 2] {
        self.point_walls().map(|wall| wall.outward())
    }

    /// Where the upright's round leaves each of the point's walls, front then
    /// back, at the centre's height: one radius out from the centre, square to
    /// the wall.
    fn feet(&self) -> [DVec3; 2] {
        self.out_of_the_point()
            .map(|out| self.centre + out * RADIUS)
    }

    /// The crown, one radius over the centre, where the full round and both
    /// point edges' rounds reach the piece of sphere.
    fn crown(&self) -> DVec3 {
        self.centre + DVec3::Z * RADIUS
    }

    /// The shares of the top the four top edges' rounds are laid over, in the
    /// order the list names them: the front's and the back's as #843 lays
    /// them, from the left end to the mitre at their shoulder; each point
    /// edge's from its shoulder's mitre to the plane through the centre square
    /// to its wall, which closes it at its foot.
    fn shares(&self) -> Vec<Share> {
        let centre = self.centre.truncate();
        let [front_foot, back_foot] = self.feet().map(DVec3::truncate);
        let [point_front, point_back] = self.point_walls();
        vec![
            self.plate.share(0),
            Share {
                corners: vec![point_front.from, front_foot, centre],
            },
            Share {
                corners: vec![back_foot, point_back.to, centre],
            },
            self.plate.share(3),
        ]
    }

    /// The turn of the upright's round, from facing out of the point's front
    /// wall to facing out of its back wall: a half turn less the tip's angle.
    fn turn(&self) -> f64 {
        let [front, back] = self.out_of_the_point();
        front.dot(back).acos()
    }

    /// The upright's round: a cylinder of [`RADIUS`] about the vertical
    /// through the centre, from the floor up to the centre's height, turning
    /// from facing out of the point's front wall to facing out of its back
    /// wall.
    fn upright_round(&self) -> Round {
        Round {
            start: self.centre.truncate().extend(0.0),
            along: DVec3::Z,
            length: self.centre.z,
            radius: RADIUS,
            facing: self.out_of_the_point(),
        }
    }

    /// Whether a point stands in the cell the piece of sphere fills: on the
    /// tip's side of the planes through the centre square to the point's two
    /// walls, and over the level one.
    fn in_the_cell(&self, point: DVec3) -> bool {
        let off = point - self.centre;
        let [front, back] = self.point_walls().map(|wall| wall.along());
        off.dot(front) >= -PLACE && off.dot(back) <= PLACE && off.z >= -PLACE
    }

    /// What the plate keeps once the list's five edges are rounded at
    /// [`RADIUS`] and the tip closed, as the module's documentation derives
    /// it: under the rounds' axes, the outline less the kite between the
    /// centre, the two feet and the tip, given back the upright's round's
    /// sector of it; over them, a quarter round swept along the four shares,
    /// from the walls — the point's from its shoulder to its foot — up to the
    /// axes, where the front's and the back's run from the left end to the
    /// crown and the point's close to nothing; and over the kite, the ball's
    /// sector over the cell.
    fn kept(&self) -> f64 {
        let r = RADIUS;
        let turn = self.turn();
        let [point_front, _] = self.point_walls();
        let [front_foot, _] = self.feet().map(DVec3::truncate);
        let kite = r * front_foot.distance(point_front.to);
        let at_the_walls = 2.0 * LENGTH + 2.0 * point_front.from.distance(front_foot);
        let at_the_axes = 2.0 * self.centre.x;
        (HEIGHT - r) * (self.plate.outline_area() - kite + r * r * turn / 2.0)
            + quarter_round_swept_along(at_the_walls, at_the_axes)
            + r.powi(3) * turn / 3.0
    }

    /// What the plate keeps once the list's five edges are rounded at
    /// [`RADIUS`] and the tip closed, computed apart from the step as #699
    /// computes a corner's piece of sphere: the full round's prism, then each
    /// point edge's round's, raised whole along its wall's line, then the
    /// upright's round's, raised whole up the plate, cut away by the booleans;
    /// then, of what is left, the cell the piece of sphere fills taken away,
    /// and the ball's sector over that cell given back, a third of the radius
    /// cubed times the cell's solid angle, the upright's round's turn.
    fn by_booleans(&self) -> f64 {
        let plate = &self.plate;
        let walls = plate.walls();
        let [point_front, point_back] = self.point_walls();
        let rounded = plate
            .part
            .difference(&full_round_tool(plate))
            .expect("the full round's prism cuts the plate");
        let rounded = [1, 2].into_iter().fold(rounded, |cut, edge| {
            cut.difference(&walls[edge].tool(&cut))
                .expect("a point edge's round's prism cuts the plate")
        });
        let rounded = rounded
            .difference(&upright_tool_at(
                &rounded,
                point_front.to,
                point_front.along().truncate(),
                point_back.along().truncate(),
            ))
            .expect("the upright's round's prism cuts the plate");
        let cell = [-point_front.along(), point_back.along(), -DVec3::Z]
            .into_iter()
            .fold(rounded.clone(), |left, away| {
                left.difference(&beyond(&left, self.centre, away, 100.0))
                    .expect("a plane through the centre cuts the plate")
            });
        rounded.volume() - cell.volume() + RADIUS.powi(3) * self.turn() / 3.0
    }
}

/// What a round of [`RADIUS`] at a convex corner of a section takes away,
/// drawn in the section's plane, anticlockwise: all that stands outside both
/// sides beyond the arc tangent to them, out past them. The section's outline
/// runs anticlockwise too, arriving at `corner` along `arriving` and leaving
/// it along `leaving`, both of unit length; the round stands back from the
/// corner along each by `r·tan(β/2)`, `β` the turn between them, written
/// `r(1 − cos β)/sin β` so that a square corner's comes out exact.
fn outside_the_round_at(corner: DVec2, arriving: DVec2, leaving: DVec2) -> Contour {
    let r = RADIUS;
    let far = r + 2.0;
    let (sin, cos) = (arriving.perp_dot(leaving), arriving.dot(leaving));
    let set_back = r * (1.0 - cos) / sin;
    let [out_of_arriving, out_of_leaving] =
        [arriving, leaving].map(|way| DVec2::new(way.y, -way.x));
    let on_arriving = corner - arriving * set_back;
    let on_leaving = corner + leaving * set_back;
    let corners = vec![
        on_arriving,
        on_arriving + out_of_arriving * far,
        corner + (out_of_arriving + out_of_leaving) * far,
        on_leaving + out_of_leaving * far,
        on_leaving,
    ];
    let mut runs = vec![Run::Straight; corners.len()];
    runs[4] = Run::Round {
        center: on_arriving - out_of_arriving * r,
        turn: -sin.atan2(cos),
    };
    Contour { corners, runs }
}

/// The prism the round of the upright at `corner` of the outline seen from
/// above takes away, the outline arriving there along `arriving` and leaving
/// along `leaving`, raised whole up past the floor and the top.
fn upright_tool_at(part: &Body, corner: DVec2, arriving: DVec2, leaving: DVec2) -> Body {
    part.tool_raised(
        &profile(outside_the_round_at(corner, arriving, leaving)),
        Frame {
            origin: DVec3::new(0.0, 0.0, -2.0),
            u: DVec3::X,
            v: DVec3::Y,
        },
        DVec3::Z * (HEIGHT + 4.0),
    )
    .expect("the tool raises")
}

/// The slab beyond the plane through `through` square to `away`, on the side
/// `away` points to: `size` each way across it, and `size` deep.
fn beyond(part: &Body, through: DVec3, away: DVec3, size: f64) -> Body {
    let away = away.normalize();
    let u = away.any_orthonormal_vector();
    part.tool_raised(
        &profile(Contour::rectangle(DVec2::splat(-size), DVec2::splat(size))),
        Frame {
            origin: through,
            u,
            v: away.cross(u),
        },
        away * size,
    )
    .expect("the slab raises")
}

/// Every point of `some` that stands at a point of `others`, each once.
fn shared_points(some: &[DVec3], others: &[DVec3]) -> Vec<DVec3> {
    let mut shared: Vec<DVec3> = Vec::new();
    for &point in some {
        if drawn_through(others, point) && !drawn_through(&shared, point) {
            shared.push(point);
        }
    }
    shared
}

/// Whether a point lies on the arc of `radius` about `centre` from the way
/// `from` points to the way `to` does, the shorter way round: on the circle,
/// in the plane the two ways span, and no further round from halfway between
/// them than half the turn.
fn on_the_arc(point: DVec3, centre: DVec3, radius: f64, [from, to]: [DVec3; 2]) -> bool {
    let off = point - centre;
    let square = from.cross(to).normalize();
    let halfway = (from + to).normalize();
    let half_turn = from.dot(to).acos() / 2.0;
    (off.length() - radius).abs() < PLACE
        && off.dot(square).abs() < PLACE
        && off.dot(halfway) >= radius * half_turn.cos() - PLACE
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

/// Holds that the fillet at [`RADIUS`] on the tip's five edges, named in
/// `order` and laid as `filleted`, takes the value whole and closes the tip
/// with a piece of sphere, its faces numbered as #698 and #699 number them:
/// the body exact, closed, uncrossed and drawn, holding what the arithmetic
/// promises; the full round one face answering to both its edges' ranks, each
/// other round to its own, the piece to the patch after every edge's, the top
/// gone and the next step numbering past the patch; each round drawn on its
/// own cylinder over its own share, the upright's within its turn and under
/// the centre, the piece on its sphere in its cell, each met there from
/// outside; the piece meeting each point edge's round along its end section's
/// quarter circle and the upright's round along the level arc between its
/// feet; the full round meeting each point edge's round on its shoulder's
/// mitre; the rounds sharing no other point than the crown and the feet; one
/// vertex at each of those; the walls and the floor cut back to the rounds;
/// and every face meeting the faces it bounds by one edge each.
fn closes_the_tip_with_a_piece_of_sphere(
    tip: &Tip,
    order: &[usize],
    filleted: &Filleted,
    what: &str,
) {
    let r = RADIUS;
    let plate = &tip.plate;
    let part = &plate.part;
    let start = part.faces_end();
    let piece = start + order.len();
    let centre = tip.centre;
    let crown = tip.crown();
    let walls = plate.walls();
    let shares = tip.shares();
    let rounds: Vec<Round> = walls
        .iter()
        .zip(&shares)
        .map(|(wall, share)| wall.round_over(share))
        .collect();
    let upright = tip.upright_round();
    let turn = tip.turn();
    let [point_front, point_back] = tip.point_walls();
    let [out_front, out_back] = tip.out_of_the_point();
    let [front_foot, back_foot] = tip.feet();
    let left_end = plate.wall(4).number;

    taken_as_asked(filleted.asked, filleted.taken, r, what);
    let body = &filleted.body;
    let area = rounds.iter().map(Round::area).sum::<f64>() + upright.area() + r * r * turn;
    sound_drawn_within(body, tip.kept(), DRAWN * reach_of(part) * area, what);

    let full = ranks_of(start, order, &[0, 3]);
    let [on_the_front, on_the_back, standing] =
        [1, 2, AT_THE_TIP].map(|edge| ranks_of(start, order, &[edge])[0]);
    let [front, back, upright_face, patch] = [[on_the_front], [on_the_back], [standing], [piece]];
    assert!(
        !body.has_face(TOP),
        "{what}: the top is gone, the full round over it"
    );
    let mut kept_and_laid = numbers(part);
    kept_and_laid.remove(&TOP);
    kept_and_laid.extend(start..=piece);
    assert_eq!(
        numbers(body),
        kept_and_laid,
        "{what}: every face of the plate but its top, the full round answering to {full:?}, the \
         point edges' rounds to {on_the_front} and {on_the_back}, the upright's round to \
         {standing}, and the piece of sphere to {piece}"
    );
    assert_eq!(
        body.faces_end(),
        piece + 1,
        "{what}: the next step numbers its faces past the {} the list gave its edges and the \
         patch of the corner it closes",
        order.len()
    );
    one_curved_face_answers_to(body, &full, &format!("{what}: the full round"));
    for (number, which) in [
        (on_the_front, "the round of the point's front edge"),
        (on_the_back, "the round of the point's back edge"),
        (standing, "the upright's round"),
        (piece, "the piece of sphere"),
    ] {
        assert!(
            !body.is_flat(number) && body.plane_of(number).is_none(),
            "{what}: {which}, face {number}, is curved, and offers no plane to draw on"
        );
    }

    let drawn_of = |number: usize| -> Vec<DVec3> { body.triangles_of(number).flatten().collect() };
    let [
        full_drawn,
        front_drawn,
        back_drawn,
        standing_drawn,
        piece_drawn,
    ] = [full[0], on_the_front, on_the_back, standing, piece].map(drawn_of);
    let over_its_share = |edge: usize, point: DVec3| {
        rounds[edge].holds(point) && shares[edge].inside_by(point) >= -PLACE
    };
    let drawn_only_where =
        |drawn: &[DVec3], on_it: &dyn Fn(DVec3) -> bool, which: &str, where_it_lies: &str| {
            assert!(!drawn.is_empty(), "{what}: {which} is drawn");
            let off: Vec<&DVec3> = drawn.iter().filter(|&&point| !on_it(point)).collect();
            assert!(
                off.is_empty(),
                "{what}: {which} is drawn through {off:?}, off {where_it_lies}"
            );
        };
    drawn_only_where(
        &full_drawn,
        &|point| over_its_share(0, point) || over_its_share(3, point),
        "the full round",
        "its half cylinder over the front's and the back's shares",
    );
    drawn_only_where(
        &front_drawn,
        &|point| over_its_share(1, point),
        "the round of the point's front edge",
        "its quarter cylinder over its share",
    );
    drawn_only_where(
        &back_drawn,
        &|point| over_its_share(2, point),
        "the round of the point's back edge",
        "its quarter cylinder over its share",
    );
    drawn_only_where(
        &standing_drawn,
        &|point| upright.holds_within_its_turn(point),
        "the upright's round",
        "its cylinder about the vertical through the centre, within its turn and under the \
         centre",
    );
    drawn_only_where(
        &piece_drawn,
        &|point| (point.distance(centre) - r).abs() < PLACE && tip.in_the_cell(point),
        "the piece of sphere",
        "the sphere about the centre, in the cell on the tip's side of its three planes",
    );
    for (edge, number) in [
        (0, full[0]),
        (3, full[0]),
        (1, on_the_front),
        (2, on_the_back),
    ] {
        met_from_outside_where(
            body,
            number,
            &rounds[edge],
            |aim| shares[edge].inside_by(aim) > CLEAR_OF_THE_MITRE,
            &format!(
                "{what}: the round over the wall under {}, clear of its share's sides",
                walls[edge].called
            ),
        );
    }
    met_from_outside_where(
        body,
        standing,
        &upright,
        |aim| aim.z < centre.z - CLEAR_OF_THE_MITRE,
        &format!("{what}: the upright's round, under the piece of sphere"),
    );
    for way in [
        DVec3::Z + out_front + out_back,
        DVec3::Z * 2.0 + out_front + out_back,
        DVec3::Z + out_front * 2.0 + out_back,
        DVec3::Z + out_front + out_back * 2.0,
    ] {
        let facing = way.normalize();
        let from = centre + facing * (r + 10.0);
        let hit = body.ray_hit(from, -facing).unwrap_or_else(|| {
            panic!("{what}: a ray from {from} toward the piece of sphere's centre meets nothing")
        });
        assert_eq!(
            hit.face, piece,
            "{what}: a ray from {from} toward the piece of sphere's centre meets face {} before it",
            hit.face
        );
        assert!(
            hit.normal.distance(facing) < PLACE,
            "{what}: where a ray from {from} meets it, the piece of sphere faces {}, not {facing}, \
             away from its centre",
            hit.normal
        );
    }

    for (drawn, arc, ends, called) in [
        (
            &front_drawn,
            [out_front, DVec3::Z],
            [front_foot, crown],
            "the round of the point's front edge, along its end section",
        ),
        (
            &back_drawn,
            [out_back, DVec3::Z],
            [back_foot, crown],
            "the round of the point's back edge, along its end section",
        ),
        (
            &standing_drawn,
            [out_front, out_back],
            [front_foot, back_foot],
            "the upright's round, along its top",
        ),
    ] {
        let seam = shared_points(drawn, &piece_drawn);
        let off: Vec<&DVec3> = seam
            .iter()
            .filter(|&&point| !on_the_arc(point, centre, r, arc))
            .collect();
        assert!(
            off.is_empty(),
            "{what}: {called}, meets the piece of sphere at {off:?}, off the arc of {r} about \
             {centre} from {} to {}",
            ends[0],
            ends[1]
        );
        assert!(
            seam.len() > 2 && ends.iter().all(|&end| drawn_through(&seam, end)),
            "{what}: {called}, meets the piece of sphere along the arc drawn from {} to {} \
             through the points between: {seam:?}",
            ends[0],
            ends[1]
        );
    }
    for (drawn, shoulder, called) in [
        (&front_drawn, point_front.from, "the point's front edge"),
        (&back_drawn, point_back.to, "the point's back edge"),
    ] {
        let mitre = shared_points(&full_drawn, drawn);
        let across = (centre.truncate() - shoulder).perp().normalize();
        let off: Vec<&DVec3> = mitre
            .iter()
            .filter(|&&point| (point.truncate() - shoulder).dot(across).abs() >= PLACE)
            .collect();
        assert!(
            off.is_empty(),
            "{what}: the full round and the round of {called} meet at {off:?}, off the vertical \
             plane through the shoulder {shoulder} and the crown"
        );
        let foot = shoulder.extend(HEIGHT - r);
        assert!(
            mitre.len() > 2 && drawn_through(&mitre, foot) && drawn_through(&mitre, crown),
            "{what}: the full round and the round of {called} meet along their mitre, drawn from \
             {foot} to the crown, {crown}, through the points between: {mitre:?}"
        );
    }
    for (one, other, only, called) in [
        (
            &full_drawn,
            &piece_drawn,
            vec![crown],
            "the full round and the piece of sphere",
        ),
        (
            &front_drawn,
            &back_drawn,
            vec![crown],
            "the two point edges' rounds, no mitre left between them",
        ),
        (
            &front_drawn,
            &standing_drawn,
            vec![front_foot],
            "the round of the point's front edge and the upright's round",
        ),
        (
            &back_drawn,
            &standing_drawn,
            vec![back_foot],
            "the round of the point's back edge and the upright's round",
        ),
        (
            &full_drawn,
            &standing_drawn,
            Vec::new(),
            "the full round and the upright's round",
        ),
    ] {
        let shared = shared_points(one, other);
        assert!(
            same_points(&shared, &only),
            "{what}: {called} share no point but {only:?}: {shared:?}"
        );
    }
    for (faces, at) in [
        ([&full[..], &front, &back, &patch], crown),
        (
            [&[POINT_FRONT][..], &front, &upright_face, &patch],
            front_foot,
        ),
        ([&[POINT_BACK][..], &back, &upright_face, &patch], back_foot),
    ] {
        assert_eq!(
            vertices_among(body, &faces),
            1,
            "{what}: one vertex at {at}, where faces {faces:?} meet"
        );
    }

    let at = |point: DVec2, height: f64| point.extend(height);
    let low = HEIGHT - r;
    let [front_foot_on_the_floor, back_foot_on_the_floor] =
        [front_foot, back_foot].map(|foot| foot.truncate().extend(0.0));
    for (wall, corners, called) in [
        (
            FRONT,
            [
                at(DVec2::ZERO, 0.0),
                at(point_front.from, 0.0),
                at(point_front.from, low),
                at(DVec2::ZERO, low),
            ],
            "the front, up to the full round",
        ),
        (
            POINT_FRONT,
            [
                at(point_front.from, 0.0),
                front_foot_on_the_floor,
                front_foot,
                at(point_front.from, low),
            ],
            "the point's front wall, up to its edge's round and out to the upright's",
        ),
        (
            POINT_BACK,
            [
                back_foot_on_the_floor,
                at(point_back.to, 0.0),
                at(point_back.to, low),
                back_foot,
            ],
            "the point's back wall, up to its edge's round and out to the upright's",
        ),
        (
            BACK,
            [
                at(point_back.to, 0.0),
                at(DVec2::new(0.0, WIDTH), 0.0),
                at(DVec2::new(0.0, WIDTH), low),
                at(point_back.to, low),
            ],
            "the back, up to the full round",
        ),
    ] {
        corners_are(body, wall, &corners, &format!("{what}: {called}"));
    }
    corners_are_along_an_arc(
        body,
        FLOOR,
        &[
            at(DVec2::ZERO, 0.0),
            at(point_front.from, 0.0),
            front_foot_on_the_floor,
            back_foot_on_the_floor,
            at(point_back.to, 0.0),
            at(DVec2::new(0.0, WIDTH), 0.0),
        ],
        |corner| {
            on_the_arc(
                corner,
                centre.truncate().extend(0.0),
                r,
                [out_front, out_back],
            )
        },
        &format!(
            "{what}: the floor, its tip given up to the upright's round along the arc of {r} \
             about {}",
            centre.truncate()
        ),
    );

    for (face, met, called) in [
        (
            &full[..],
            vec![&[FRONT][..], &[BACK], &[left_end], &front, &back],
            "the full round meets the front and the back along its sides, the left end, and each \
             point edge's round along its shoulder's mitre, and the piece of sphere at the crown \
             only",
        ),
        (
            &front[..],
            vec![&[POINT_FRONT][..], &full, &patch],
            "the round of the point's front edge meets its wall, the full round along its \
             shoulder's mitre and the piece of sphere along its end section",
        ),
        (
            &back[..],
            vec![&[POINT_BACK][..], &full, &patch],
            "the round of the point's back edge meets its wall, the full round along its \
             shoulder's mitre and the piece of sphere along its end section",
        ),
        (
            &upright_face[..],
            vec![&[FLOOR][..], &[POINT_FRONT], &[POINT_BACK], &patch],
            "the upright's round meets the floor, the point's two walls and the piece of sphere \
             along its top",
        ),
        (
            &patch[..],
            vec![&front[..], &back, &upright_face],
            "the piece of sphere meets the three rounds, and nothing else",
        ),
        (
            &[FRONT][..],
            vec![&[FLOOR][..], &[left_end], &[POINT_FRONT], &full],
            "the front meets the floor, the left end, the point's front wall along the shoulder's \
             upright, left sharp, and the full round",
        ),
        (
            &[POINT_FRONT][..],
            vec![&[FLOOR][..], &[FRONT], &front, &upright_face],
            "the point's front wall meets the floor, the front, its edge's round and the \
             upright's round",
        ),
        (
            &[POINT_BACK][..],
            vec![&[FLOOR][..], &[BACK], &back, &upright_face],
            "the point's back wall meets the floor, the back, its edge's round and the upright's \
             round",
        ),
        (
            &[BACK][..],
            vec![&[FLOOR][..], &[left_end], &[POINT_BACK], &full],
            "the back meets the floor, the left end, the point's back wall along the shoulder's \
             upright, left sharp, and the full round",
        ),
        (
            &[FLOOR][..],
            vec![
                &[FRONT][..],
                &[POINT_FRONT],
                &upright_face,
                &[POINT_BACK],
                &[BACK],
                &[left_end],
            ],
            "the floor meets the five walls and the upright's round",
        ),
        (
            &[left_end][..],
            vec![&[FLOOR][..], &[FRONT], &[BACK], &full],
            "the left end meets the floor, the front, the back and the full round",
        ),
    ] {
        assert_eq!(
            sides_met_by(body, face),
            sides_answering(&met),
            "{what}: {called}, by one edge each"
        );
    }
}

/// Holds that the arithmetic of [`Tip::kept`] gives the volume the module's
/// documentation writes out for the tip.
fn the_tip_s_arithmetic_gives(tip: &Tip, volume: f64) {
    assert!(
        (tip.kept() - volume).abs() <= EXACT * volume,
        "{}, its tip closed: the arithmetic gives {}, where the documentation writes {volume}",
        tip.plate.called,
        tip.kept()
    );
}

/// Holds, for each of `tips` before any is laid, that its centre stands one
/// radius in from both of the point's walls and under the top, and that the
/// booleans give what the arithmetic does; then, for each, that the fillet at
/// [`RADIUS`] on its five edges, in each of the 120 orders they can be named
/// in, is laid, closing the tip with a piece of sphere as
/// [`closes_the_tip_with_a_piece_of_sphere`] holds it, each order laying the
/// body the listed order lays: the same matter, the same faces meeting by the
/// same edges, each laid face read by the edge it was laid on, and the plate's
/// flat faces at the same corners.
fn closes_the_tip_with_a_piece_of_sphere_in_every_order(tips: &[Tip]) {
    let r = RADIUS;
    for tip in tips {
        let called = &tip.plate.called;
        for wall in tip.point_walls() {
            let in_from_it = (wall.from - tip.centre.truncate()).dot(wall.outward().truncate());
            assert!(
                (in_from_it - r).abs() < PLACE && (tip.centre.z - (HEIGHT - r)).abs() < PLACE,
                "{called}: the centre {} stands one radius in from the wall under {} and under \
                 the top, where it stands {in_from_it} in",
                tip.centre,
                wall.called
            );
        }
        let promised = tip.kept();
        let by_booleans = tip.by_booleans();
        assert!(
            (by_booleans - promised).abs() <= EXACT * promised,
            "{called}: the prisms of the full round, of the point edges' rounds and of the \
             upright's round, cut from the plate by the booleans, the cell at the tip taken away \
             and the piece of sphere given back, leave {by_booleans}, where arithmetic promises \
             {promised}"
        );
    }
    for tip in tips {
        let start = tip.plate.part.faces_end();
        let listed_order: Vec<usize> = (0..=AT_THE_TIP).collect();
        let laid: Vec<(Vec<usize>, Result<Filleted, Declined>)> = every_order(listed_order.len())
            .into_iter()
            .map(|order| {
                let laid = tip.filleted(&order);
                (order, laid)
            })
            .collect();
        let declined: Vec<String> = laid
            .iter()
            .filter_map(|(order, laid)| {
                laid.as_ref()
                    .err()
                    .map(|declined| format!("{}: declined, {declined:?}", tip.called(order)))
            })
            .collect();
        assert!(
            declined.is_empty(),
            "{} closes the tip whichever edge the list names first; declined:\n{}",
            tip.called(&listed_order),
            declined.join("\n")
        );
        let reference = &laid
            .iter()
            .find(|(order, _)| *order == listed_order)
            .and_then(|(_, laid)| laid.as_ref().ok())
            .expect("the listed order is laid")
            .body;
        for (order, laid) in &laid {
            let what = tip.called(order);
            let filleted = laid.as_ref().expect("declines are held above");
            closes_the_tip_with_a_piece_of_sphere(tip, order, filleted, &what);
            let otherwise = met_otherwise(start, order, &filleted.body, &listed_order, reference);
            assert!(
                otherwise.is_empty(),
                "{what}: the body is the one the listed order lays, its faces meeting alike; not \
                 so:\n{}",
                otherwise.join("\n")
            );
            same_body_as(&tip.plate, &filleted.body, reference, &what);
        }
    }
}

#[test]
fn a_fillet_on_a_full_round_the_two_edges_of_its_pointed_end_and_the_upright_at_its_tip_closes_the_tip_with_a_piece_of_sphere_whatever_the_order()
 {
    let tip = Tip::at(33.0, DVec3::new(33.0 - 3.0 * SQRT_2, 3.0, 7.0));
    the_tip_s_arithmetic_gives(
        &tip,
        1224.0 + 36.0 * SQRT_2 + (75.0 - 6.0 * SQRT_2) * 9.0 * PI / 4.0,
    );
    closes_the_tip_with_a_piece_of_sphere_in_every_order(&[tip]);
}

#[test]
fn a_fillet_on_a_full_round_the_two_edges_of_a_sharper_or_a_blunter_point_and_the_upright_at_its_tip_closes_the_tip_with_a_piece_of_sphere_whatever_the_order()
 {
    let [root_ten, root_thirteen] = [10f64.sqrt(), 13f64.sqrt()];
    let sharper = Tip::at(39.0, DVec3::new(39.0 - 3.0 * root_ten, 3.0, 7.0));
    the_tip_s_arithmetic_gives(
        &sharper,
        1152.0 + 36.0 * root_ten + 81.0 * 3f64.atan() + (78.0 - 6.0 * root_ten) * 9.0 * PI / 4.0,
    );
    let blunter = Tip::at(32.0, DVec3::new(32.0 - root_thirteen, 3.0, 7.0));
    the_tip_s_arithmetic_gives(
        &blunter,
        1236.0
            + 12.0 * root_thirteen
            + 81.0 * (2.0f64 / 3.0).atan()
            + (64.0 - 2.0 * root_thirteen) * 9.0 * PI / 4.0,
    );
    closes_the_tip_with_a_piece_of_sphere_in_every_order(&[sharper, blunter]);
}

/// The plate pointed at both ends, top (0, 3), (3, 0), (30, 0), (33, 3),
/// (30, 6), (3, 6), its six top edges listed from the front round to the
/// left point's front edge, its crowns one radius in from both tips' walls.
fn plate_pointed_at_both_ends() -> Plate {
    Plate::raised(
        "the plate pointed at both ends, its tips at (0, 3) and (33, 3)",
        vec![
            DVec2::new(0.0, 3.0),
            DVec2::new(3.0, 0.0),
            DVec2::new(30.0, 0.0),
            DVec2::new(33.0, 3.0),
            DVec2::new(30.0, 6.0),
            DVec2::new(3.0, 6.0),
        ],
        &[
            (1, "the top's front edge"),
            (2, "the right point's front edge"),
            (3, "the right point's back edge"),
            (4, "the top's back edge"),
            (5, "the left point's back edge"),
            (0, "the left point's front edge"),
        ],
        [1, 4],
        vec![
            DVec3::new(3.0 * SQRT_2, 3.0, 10.0),
            DVec3::new(33.0 - 3.0 * SQRT_2, 3.0, 10.0),
        ],
    )
}

#[test]
fn a_fillet_on_a_full_round_pointed_at_both_ends_mitres_each_point_round_up_to_its_end_s_crown_whatever_the_order()
 {
    let plate = plate_pointed_at_both_ends();
    the_shares_give(
        &plate,
        1224.0 + 72.0 * SQRT_2 + (66.0 - 12.0 * SQRT_2) * 9.0 * PI / 4.0,
    );
    lays_one_mitred_body_in_each_order(&[plate], each_end_in_every_order);
}

#[test]
fn a_fillet_on_a_full_round_pointed_at_one_end_and_square_at_the_other_mitres_each_end_s_rounds_up_to_its_crown_whatever_the_order()
 {
    let plate = Plate::raised(
        "the plate pointed at (33, 3), its left end square",
        vec![
            DVec2::ZERO,
            DVec2::new(LENGTH, 0.0),
            DVec2::new(33.0, 3.0),
            DVec2::new(LENGTH, WIDTH),
            DVec2::new(0.0, WIDTH),
        ],
        &[
            (0, "the top's front edge"),
            (1, "the point's front edge"),
            (2, "the point's back edge"),
            (3, "the top's back edge"),
            (4, "the top's left edge"),
        ],
        [0, 3],
        vec![
            DVec3::new(3.0, 3.0, 10.0),
            DVec3::new(33.0 - 3.0 * SQRT_2, 3.0, 10.0),
        ],
    );
    the_shares_give(
        &plate,
        1341.0 + 36.0 * SQRT_2 + (60.0 - 6.0 * SQRT_2) * 9.0 * PI / 4.0,
    );
    lays_one_mitred_body_in_each_order(&[plate], every_order);
}

#[test]
fn a_fillet_on_a_full_round_and_the_two_edges_of_a_lopsided_point_tangent_to_one_circle_stops_short_of_the_full_round_whatever_the_order()
 {
    let lopsided = raised(
        Contour::straight(vec![
            DVec2::ZERO,
            DVec2::new(LENGTH, 0.0),
            DVec2::new(98.0 / 3.0, 2.0),
            DVec2::new(31.0, WIDTH),
            DVec2::new(0.0, WIDTH),
        ]),
        HEIGHT,
    );
    stops_short_in_every_order(
        &lopsided,
        &[
            ([TOP, FRONT], "the top's front edge"),
            ([TOP, POINT_FRONT], "the point's front edge"),
            ([TOP, POINT_BACK], "the point's back edge"),
            ([TOP, BACK], "the top's back edge"),
        ],
        2.0,
        "the full round's pair and the two edges of the lopsided point whose tip stands at \
         (98/3, 2), both tangent to the circle of radius 3 about (29, 3), its back 31 long",
    );
}

/// Holds that the fillet at [`RADIUS`] on `listed`'s edges of `part`, each
/// the edge between the two faces numbered, takes `value` and keeps the top in
/// every order, short of a full round, the body exact, closed and uncrossed;
/// `what` says what the list is.
fn stops_short_in_every_order(part: &Body, listed: &[([usize; 2], &str)], value: f64, what: &str) {
    for order in every_order(listed.len()) {
        let names: Vec<EdgeName> = order
            .iter()
            .map(|&edge| {
                let [one, other] = listed[edge].0;
                edge_between(part, one, other)
            })
            .collect();
        let called: Vec<&str> = order.iter().map(|&edge| listed[edge].1).collect();
        let what = format!(
            "the fillet at {RADIUS} on {what}, {}",
            called.join(", then ")
        );
        let filleted = part
            .fillet_edges(&names, RADIUS)
            .unwrap_or_else(|declined| panic!("{what}: declined, {declined:?}"));
        assert!(
            (filleted.asked - RADIUS).abs() < PLACE && (filleted.taken - value).abs() < PLACE,
            "{what}: asked {} and took {}, where the list stops short of the full round at \
             {value}",
            filleted.asked,
            filleted.taken
        );
        let body = &filleted.body;
        assert!(
            body.has_face(TOP),
            "{what}: the top is kept, no full round laid over it"
        );
        assert!(
            body.is_exact(),
            "{what}: the exact kernel computed the body"
        );
        let triangles = body.triangles();
        assert_eq!(closed(&triangles), Ok(()), "{what}");
        assert_eq!(uncrossed(&triangles), Ok(()), "{what}");
    }
}

#[test]
fn a_fillet_on_a_full_round_and_one_edge_of_its_point_alone_stops_short_of_the_full_round_whatever_the_order()
 {
    let plate = Plate::pointed(33.0, DVec3::ZERO);
    stops_short_in_every_order(
        &plate.part,
        &[
            ([TOP, FRONT], "the top's front edge"),
            ([TOP, BACK], "the top's back edge"),
            ([TOP, POINT_FRONT], "the point's front edge"),
        ],
        6.0 - 3.0 * SQRT_2,
        &format!(
            "the full round's pair and the point's front edge alone, on {}",
            plate.called
        ),
    );
}

#[test]
fn a_fillet_on_a_full_round_one_edge_of_its_point_alone_and_its_square_far_end_s_edge_stops_short_of_the_full_round_whatever_the_order()
 {
    let plate = Plate::pointed(33.0, DVec3::ZERO);
    let left_end = 6;
    stops_short_in_every_order(
        &plate.part,
        &[
            ([TOP, FRONT], "the top's front edge"),
            ([TOP, BACK], "the top's back edge"),
            ([TOP, POINT_FRONT], "the point's front edge"),
            ([TOP, left_end], "the top's left edge"),
        ],
        6.0 - 3.0 * SQRT_2,
        &format!(
            "the full round's pair, the point's front edge alone and the edge across the square \
             far end, on {}",
            plate.called
        ),
    );
}

#[cfg(feature = "campaigns")]
mod campaign {
    use super::*;

    #[test]
    #[ignore = "a campaign, run by hand: see the head of this file"]
    fn a_fillet_on_a_full_round_pointed_at_both_ends_mitres_each_point_round_up_to_its_end_s_crown_in_each_of_the_720_orders_of_its_six_edges()
     {
        lays_one_mitred_body_in_each_order(&[plate_pointed_at_both_ends()], every_order);
    }
}
