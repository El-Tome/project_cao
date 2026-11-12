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
//!   in the second file, the decline now named
//!   `a_fillet_on_a_full_round_meeting_another_full_round_and_on_a_round_bottom_meeting_the_edge_at_its_end_is_declined_as_unsupported_whatever_the_order`
//!   gives up the list of the pair and the front-right upright, which this
//!   slice builds, and kept the tongue's for the second slice, which builds it
//!   below; in the third, the decline of the pair, the end edge and the
//!   front-right upright stayed until the second slice, which builds that list
//!   below too
//! - Decided on the way, after the review (2026-11-12): what the branch lets
//!   in beside the issue's end, each list declined on `main`, is held to the
//!   shape the distance to the named walls gives — built exact in every
//!   order, one body, the value taken whole, closed, uncrossed, its triangles
//!   enclosing the volume arithmetic gives within the drawing's tolerance,
//!   that volume given by the booleans too. The upright at the block's other
//!   end, at its front corner, with the pair; and one upright at each end,
//!   the front one at x = 30 and the back one at x = 0 —
//!   `a_fillet_on_a_full_round_and_an_upright_at_its_far_end_or_one_at_each_end_mitres_each_upright_s_round_against_the_full_round_whatever_the_order`
//! - Decided on the way, after the review (2026-11-12): the pair, the edge
//!   across the end at x = 30 and the front upright at x = 0 — #803's mitre
//!   and this issue's in one step —
//!   `a_fillet_on_a_full_round_the_edge_across_one_end_and_an_upright_at_the_other_mitres_both_against_the_full_round_whatever_the_order`
//! - Decided on the way, after the review (2026-11-12): a standing full round
//!   alone with one lying edge — both uprights at x = 30 and the top's front
//!   edge alone, and likewise the floor's front edge alone — that edge's round
//!   mitred against the standing full round as an upright's is against a
//!   lying one —
//!   `a_fillet_on_both_uprights_at_an_end_and_one_long_edge_of_the_top_or_the_floor_mitres_that_edge_s_round_against_their_standing_full_round_whatever_the_order`
//! - Decided on the way, after the review (2026-11-12): full rounds along the
//!   top and along the floor, the top's front and back edges and the floor's,
//!   with the front upright at x = 30, which meets both: its round mitred
//!   against each, on z − 7 = x − 27 and on z − 3 = 27 − x —
//!   `a_fillet_on_full_rounds_along_the_top_and_the_floor_and_the_upright_at_their_end_mitres_the_upright_s_round_against_both_whatever_the_order`
//! - Decided on the way, after the review (2026-11-12): #843's pointed plate,
//!   top (0, 0), (30, 0), (33, 3), (30, 6), (0, 6), 10 high, with the pair and
//!   the upright at the corner (30, 0), the point's edges not named: the
//!   upright's round, a corner of 135°'s, about the vertical through
//!   (33 − 3√2, 3), mitred against the full round on z − 7 = x − 33 + 3√2,
//!   the mitre ending on the point's front wall at
//!   (33 − 3√2/2, 3 − 3√2/2, 7 + 3√2/2), short of the crown —
//!   `a_fillet_on_a_full_round_and_the_upright_at_a_corner_of_its_pointed_end_mitres_the_upright_s_round_against_it_up_to_the_point_s_wall_whatever_the_order`
//! - Decided on the way, after the review (2026-11-12): an end slanted seen
//!   from above, top (0, 0), (30, 0), (32, 6), (0, 6), with the pair and the
//!   upright at (30, 0): its round, a corner of 108.4°'s, about the vertical
//!   through (31 − √10, 3), mitred against the full round, the mitre ending on
//!   the end wall at (31 − √10 + 9/√10, 3 − 3/√10, 7 + 9/√10), away from the
//!   crown's (31, 3, 10) —
//!   `a_fillet_on_a_full_round_and_the_upright_at_a_corner_of_its_slanted_end_mitres_the_upright_s_round_against_it_up_to_the_end_wall_whatever_the_order`
//! - Decided on the way, after the review (2026-11-12): the hair — the pair,
//!   the front upright at x = 30 and the floor's front edge, on a block a hair
//!   wider than 6, 6 + 10⁻¹², at 3: the pair's set-backs stop a hair short of
//!   meeting and are laid as one full round at its own radius, the upright's
//!   round laid with the pair's value, and its lower end meets the floor's
//!   front edge's round laid at the value taken — the body the block 6 wide
//!   would lay, to the hair —
//!   `a_fillet_on_a_full_round_a_hair_short_the_upright_at_its_end_and_the_floor_s_front_edge_takes_the_value_whole_whatever_the_order`
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
//!
//! What the review let in is held to that arithmetic too, taken at all eight
//! corners of the block: a corner where two named edges meet gives back
//! `r³(5/3 − π/2)`, one where three do `r³(2 − 7π/12)`. The upright at the far
//! end with the pair, `1215 + 144π`, as at x = 30; one upright at each end,
//! `1800 − 80r²(1 − π/4) + 2r³(5/3 − π/2)`, `1170 + 153π`, 1650.66; the pair,
//! the end edge and the far front upright, three corners mitred,
//! `1800 − 76r²(1 − π/4) + 3r³(5/3 − π/2)`, `1251 + 261π/2`, 1660.98; both
//! uprights at x = 30 with the top's or the floor's front edge,
//! `1800 − 50r²(1 − π/4) + r³(5/3 − π/2)`, `1395 + 99π`, 1706.02; the full
//! rounds along the top and the floor with the front upright,
//! `1800 − 130r²(1 − π/4) + 2r³(5/3 − π/2)`, `720 + 531π/2`, 1554.09, the two
//! full rounds sharing nothing, 4 apart; the hair,
//! `LWH − 100r²(1 − π/4) + 2r³(5/3 − π/2)`, `990 + 198π` on the block 6 wide,
//! 1612.04, the hair's own share, `3·10⁻¹⁰` of the block, far under the
//! exact kernel's tolerance on the volume.
//!
//! An upright at a corner that is not square, on a plate whose full round
//! runs along its front at y = 0, the corner's other wall turning by `β` from
//! the front: its round, tangent to both walls, stands `t = r·tan(β/2)` back
//! from the corner along each, and alone takes up the plate's whole height the
//! floor's corner outside it, `rt − r²β/2`. The full round alone takes the
//! matter over its half cylinder all over the plate, `(r − s(y))` over each
//! point, `s(y) = √(r² − (r − y)²)`. What both take, counted twice, is that
//! over the corner the upright's round takes: at `u = r − y`, from the arc,
//! `x_c + √(r² − u²)`, its axis at `x_c = x₀ − t` for the corner at `x₀`, to
//! the wall, `x₀ + y·cot β`, a width
//! `w = t + (r − u)cot β − √(r² − u²)`, from `u = r` down to `u = r·cos β`,
//! where the round leaves the wall. With `u = r·sin φ`, the integral
//! `∫ w·(r − √(r² − u²)) du` is `r²∫ (K − r·cot β·sin φ − r·cos φ)(cos φ −
//! cos² φ) dφ` from `π/2 − β` to `π/2`, `K = t + r·cot β`, each term an
//! elementary one. At a square corner it is `r³(5/3 − π/2)` again. On #843's
//! pointed plate, `β = π/4`: the plate holds `189H`, the full round takes
//! `30r²(2 − π/2)` along its length and `45 − 27π/2` over the point, the
//! upright `H(9(√2 − 1) − 9π/8)`, and they share
//! `99√2/4 − 9 − 27π(1 + √2)/8`, 0.4042; the body holds
//! `1386 + 1251π/8 − 261√2/4 − 27√2π/8`, 1769.99. On the slanted end,
//! `β = atan 3`: the plate holds `186H`, the full round takes
//! `31r²(2 − π/2)`, the upright `H(3(√10 − 1) − 9β/2)`, and they share
//! `9 + 171√10/20 − 9(√10 + 3)β/2`, 1.4011; the body holds
//! `1341 + 279π/2 − 429√10/20 + 9(7 − √10)β/2`, 1732.99. Both were checked
//! against the volume integrated over the plate point by point, to 3·10⁻⁹.
//! The mitre is still where the two cylinders cross, on the plane through
//! their crossing axes, `z − 7 = x − x_c`; it ends where the upright's round
//! leaves the corner's other wall, at `z = 7 + r·sin β`. The booleans give
//! each volume as well, every round's prism cut whole from the plate, the
//! upright's raised up its height.
//!
//! The tip of a thin tongue rounded all round: the pair, the edge across the
//! end and both uprights there, filleted at 3 on the block 6 wide. Each of the
//! end's two top corners, where three named edges meet, is closed by a piece
//! of sphere one radius from its three faces, as #699 closes it; at 6 wide
//! both are centred at the crossing of the two full rounds' axes,
//! (27, 3, 7), and the end edge's round between them closes to nothing. The
//! tip is one quarter of a ball, past the planes x = 27 and z = 7, tangent to
//! the lying full round along the half circle x = 27 and to the standing one
//! along z = 7; the two full rounds touch only where those half circles end,
//! (27, 0, 7) and (27, 6, 7). With the front upright alone, the front corner
//! is closed by the piece of sphere about that same centre and the back corner
//! mitred as #803 mitres it. Chamfered, the end edge's flat closes to nothing
//! between the two corners' triangles, which meet on a ridge.
//!
//! Closes #837 — its second slice only, the spheres. Its third, the pointed
//! end's tip, comes on a branch of its own.
//! - The tongue — the pair, the end edge and both uprights — filleted at 3:
//!   built in every order, one body; the tip a quarter of a ball of radius 3
//!   centred (27, 3, 7), meeting the lying full round on the half circle
//!   x = 27 and the standing one on z = 7; the volume arithmetic gives —
//!   `a_fillet_on_a_full_round_the_edge_across_its_end_and_both_uprights_there_rounds_the_tongue_s_tip_into_a_quarter_of_a_ball_whatever_the_order`.
//!   Read as: in each of the 120 orders, the value taken whole; the body
//!   exact, closed, uncrossed, holding `1134 + 162π`, which the booleans give
//!   too, and its triangles enclosing that within the drawing's tolerance; the
//!   top and the end gone; the lying full round one face answering to both
//!   its edges' ranks, the standing one one face answering to both uprights'
//!   ranks, and the quarter ball one face answering to the end edge's rank
//!   and to the two numbers #699 gives, after every edge's, the patches of the
//!   two corners it closes, a ray meeting it naming the end edge's rank; the
//!   quarter ball drawn on its sphere only, past both planes, and met there
//!   from outside; the lying full round drawn on its half cylinder only and
//!   never past x = 27, the standing one on its own and never above z = 7,
//!   each met first from outside; each meeting the quarter ball along its half
//!   circle, drawn from (27, 0, 7) to (27, 6, 7) through the points between,
//!   and the two sharing no other point; at each of those two points one
//!   vertex, where the wall there, both full rounds and the quarter ball meet;
//!   the front and the back up to (27, 0, 7) and (27, 6, 7), and the floor up
//!   to the half circle of radius 3 about (27, 3); every face meeting the
//!   faces it bounds by one edge each, the quarter ball the two full rounds
//!   only; and every order laying the body the listed order lays. Not in the
//!   issue's wording, a choice of this test: the quarter ball answers to what
//!   merged into it, as the full round answers to both its edges — the end
//!   edge's round, of no length at 6 wide, and the two corners' pieces of
//!   sphere, three faces numbered so at 6.5 wide
//! - The pair, the end edge and the front-right upright: built in every
//!   order, one body; the front corner closed by the piece of sphere centred
//!   (27, 3, 7), the back corner mitred against the end edge's round as #803
//!   does; the volume arithmetic gives —
//!   `a_fillet_on_a_full_round_the_edge_across_its_end_and_the_upright_at_its_front_corner_closes_that_corner_with_a_piece_of_sphere_and_mitres_the_other_whatever_the_order`
//! - The tongue chamfered at 3: built in every order, one body; the end
//!   edge's flat gone, the two corners' triangles meeting on the ridge from
//!   (27, 3, 10) to (30, 3, 7); the volume arithmetic gives —
//!   `a_chamfer_on_a_full_round_the_edge_across_its_end_and_both_uprights_there_closes_the_end_edge_s_flat_to_a_ridge_between_the_corners_triangles_whatever_the_order`
//! - Throughout, as far as this slice goes: the lists `main` builds keep their
//!   bodies — the chamfers above but the tongue's, the end edge with both
//!   uprights without the pair, and every one of these at 6.5 wide — held by
//!   the first slice's test, which this slice leaves as it is —
//!   `the_lists_main_builds_at_the_end_of_a_block_6_or_6_5_wide_keep_their_bodies_in_every_order`
//! - Throughout: no body of #698's, #699's, #803's or #843's tests changes —
//!   no test: those tests hold their own bodies and run beside this file.
//!   What changes there is a decline, not a body: in
//!   `a_chamfer_and_a_fillet_blend_the_corner_their_edges_meet_at.rs`, the
//!   fillet's decline gives up the tongue, built above, keeps the round bottom
//!   and the block 6 by 6, and is renamed for them
//!   `a_fillet_on_a_full_round_meeting_another_full_round_and_on_a_round_bottom_meeting_the_edge_at_its_end_is_declined_as_unsupported_whatever_the_order`;
//!   the chamfer's decline on the tongue there,
//!   `a_chamfer_on_a_full_round_pair_s_edges_the_edge_at_their_end_and_both_uprights_there_is_declined_as_unsupported_whatever_the_order`,
//!   gives way to the third criterion's test; and in
//!   `a_full_round_ends_on_a_pointed_end_mitred_against_each_edge_there.rs`,
//!   `a_fillet_on_a_full_round_its_square_end_s_edge_and_the_upright_at_one_corner_of_that_end_is_declined_as_unsupported_whatever_the_order`
//!   goes, the list it declined built by the second criterion's test
//!
//! The tongue's volume is the arithmetic above, its five edges' rounds each
//! taken whole, the top's front and back edges 30 long, the end edge 6 and
//! each upright 10, and `r³(2 − 7π/12)` given back at each of the end's two
//! top corners, where three named edges meet:
//! `LWH − 86r²(1 − π/4) + 2r³(2 − 7π/12)`, `1134 + 162π`, 1642.94. Read the
//! other way, it is the full round's section, `42 + 9π/2`, along the 27 the
//! lying full round runs; the standing half disc, `9π/2`, up the 7 the
//! standing one rises; and the quarter ball, `πr³/3`. The booleans give it as
//! #699's give a corner's piece of sphere: the prisms of the lying full round,
//! the standing one and the end edge's round cut whole from the block; then,
//! of what is left, the cell past both planes x = 27 and z = 7 — where the
//! three prisms leave a quarter of the solid common to the three cylinders —
//! taken away,
//! and the ball's sector over it given back, `r³Ω/3` for the quarter's solid
//! angle `Ω = π`.

use std::collections::{BTreeMap, BTreeSet};
use std::f64::consts::{PI, SQRT_2};

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
/// triangles of the round it aims at rather than the other's — and as far
/// from the seam where a full round meets the tip's quarter ball: well over
/// the drawing's tolerance on the block, [`DRAWN`] times its reach.
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

/// The edges the review's lists name besides: the floor's front and back
/// edges, the pair of a full round along the floor; and the uprights where
/// the front and the back meet the left end, at x = 0.
const FLOOR_FRONT: [usize; 2] = [FLOOR, FRONT];
const FLOOR_BACK: [usize; 2] = [FLOOR, BACK];
const FRONT_LEFT_UPRIGHT: [usize; 2] = [FRONT, LEFT];
const BACK_LEFT_UPRIGHT: [usize; 2] = [BACK, LEFT];

/// The full round's pair, and what each edge is called.
const PAIR: [([usize; 2], &str); 2] = [
    (TOP_FRONT, "the top's front edge"),
    (TOP_BACK, "the top's back edge"),
];

/// What the block [`WIDTH`] wide is called.
const ON_THE_BLOCK: &str = "the block 6 wide";

/// The edge across the end, and what it is called.
const END_EDGE: ([usize; 2], &str) = (TOP_END, "the top's edge across the end");

/// The edges of the review's lists besides, and what each is called.
const FLOOR_FRONT_EDGE: ([usize; 2], &str) = (FLOOR_FRONT, "the floor's front edge");
const FLOOR_BACK_EDGE: ([usize; 2], &str) = (FLOOR_BACK, "the floor's back edge");
const FRONT_LEFT_EDGE: ([usize; 2], &str) = (
    FRONT_LEFT_UPRIGHT,
    "the upright at the left end's front corner",
);
const BACK_LEFT_EDGE: ([usize; 2], &str) = (
    BACK_LEFT_UPRIGHT,
    "the upright at the left end's back corner",
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

/// A plate [`HEIGHT`] high raised from `outline`, anticlockwise seen from
/// above, as `cao_part` builds a step: its floor nought, its top one, then a
/// wall for each run of the outline, from two.
fn plate(outline: Vec<DVec2>) -> Body {
    let part = Body::default();
    let tool = part
        .tool_raised(
            &profile(Contour::straight(outline)),
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
    raised_up_the_block(part, outside_the_round_at(corner, arriving, leaving))
}

/// A tool drawn in the block's section square to X, `u` along Y and `v` up,
/// and raised whole along X past both ends of the stretch `along` covers.
fn raised_along_the_block(part: &Body, outline: Contour, [from, to]: [f64; 2]) -> Body {
    part.tool_raised(
        &profile(outline),
        Frame {
            origin: DVec3::new(from - 2.0, 0.0, 0.0),
            u: DVec3::Y,
            v: DVec3::Z,
        },
        DVec3::X * (to - from + 4.0),
    )
    .expect("the tool raises")
}

/// The prism the round of the top's front edge alone takes away, raised
/// whole along the block: the section's outline, seen with Y to the right and
/// Z up, arrives at the top's front corner along the top, from the back, and
/// leaves it down the front.
fn top_front_tool(part: &Body) -> Body {
    raised_along_the_block(
        part,
        outside_the_round_at(DVec2::new(0.0, HEIGHT), -DVec2::X, -DVec2::Y),
        [0.0, LENGTH],
    )
}

/// The prism the round of the floor's front edge alone takes away, raised
/// whole along the block: the section's outline arrives at the floor's front
/// corner down the front and leaves it along the floor.
fn floor_front_tool(part: &Body) -> Body {
    raised_along_the_block(
        part,
        outside_the_round_at(DVec2::ZERO, -DVec2::Y, DVec2::X),
        [0.0, LENGTH],
    )
}

/// The prism the round of the top's edge across the end at x = [`LENGTH`]
/// takes away, raised whole along Y past the front and the back: drawn in the
/// section square to Y, `u` along X and `v` down, whose outline arrives at the
/// end's top corner along the top and leaves it up the end.
fn end_edge_tool(part: &Body) -> Body {
    part.tool_raised(
        &profile(outside_the_round_at(
            DVec2::new(LENGTH, -HEIGHT),
            DVec2::X,
            DVec2::Y,
        )),
        Frame {
            origin: DVec3::new(0.0, -RADIUS - 2.0, 0.0),
            u: DVec3::X,
            v: -DVec3::Z,
        },
        DVec3::Y * (WIDTH + 2.0 * RADIUS + 4.0),
    )
    .expect("the tool raises")
}

/// The prism the full round along the floor takes away, raised whole along X
/// past both ends of the block: all that stands under the front and the back
/// outside the half cylinder about the line [`RADIUS`] over the middle of the
/// floor.
fn floor_full_round_tool(part: &Body) -> Body {
    let far = 2.0 * RADIUS + 2.0;
    let corners = vec![
        DVec2::new(-far, RADIUS),
        DVec2::new(-far, -far),
        DVec2::new(WIDTH + far, -far),
        DVec2::new(WIDTH + far, RADIUS),
        DVec2::new(WIDTH, RADIUS),
        DVec2::new(0.0, RADIUS),
    ];
    let mut runs = vec![Run::Straight; corners.len()];
    runs[4] = Run::Round {
        center: DVec2::new(RADIUS, RADIUS),
        turn: -PI,
    };
    raised_along_the_block(part, Contour { corners, runs }, [0.0, LENGTH])
}

/// A prism a round takes away, raised on the body it is to be cut from.
type Tool = dyn Fn(&Body) -> Body;

/// What is left of `part` once each of `tools`, raised on what is left so
/// far, is cut from it by the booleans, in turn.
fn left_by_booleans(part: &Body, tools: &[&Tool]) -> f64 {
    tools
        .iter()
        .fold(part.clone(), |left, tool| {
            left.difference(&tool(&left))
                .expect("a round's prism cuts the part")
        })
        .volume()
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
/// taken whole all along it, and given back at each of the block's eight
/// corners what the cuts of the named edges there take more than once. Each
/// edge is written as the constants above write it: the top or the floor
/// first, then the front or the back, then the end or the left end.
fn kept(cut: Cut, width: f64, listed: &[[usize; 2]]) -> f64 {
    let ends = [END, LEFT];
    let sides = [FRONT, BACK];
    let levels = [TOP, FLOOR];
    let length = |edge: &[usize; 2]| {
        let [one, other] = *edge;
        match (levels.contains(&one), ends.contains(&other)) {
            (true, true) => width,
            (false, true) => HEIGHT,
            _ => LENGTH,
        }
    };
    let mut given_back = 0.0;
    for end in ends {
        for side in sides {
            for level in levels {
                let corner = [[level, side], [level, end], [side, end]];
                let named = corner.iter().filter(|edge| listed.contains(edge)).count();
                given_back += cut.given_back(named);
            }
        }
    }
    LENGTH * width * HEIGHT - cut.section() * listed.iter().map(length).sum::<f64>() + given_back
}

/// What the full round and the round of an upright at a convex corner of
/// the front both take, counted twice, the corner's other wall turning by
/// `turn` from the front, as the module's documentation derives it: the
/// integral over the corner the upright's round takes, of the height the full
/// round takes there, in `φ` from `π/2 − turn` to `π/2`.
fn shared_at_a_corner_turning_by(turn: f64) -> f64 {
    let r = RADIUS;
    let (sin, cos) = turn.sin_cos();
    let set_back = r * (1.0 - cos) / sin;
    let lean = cos / sin;
    let k = set_back + r * lean;
    let at = |phi: f64| {
        let (sin, cos) = phi.sin_cos();
        let of_cos = sin;
        let of_cos_squared = phi / 2.0 + sin * cos / 2.0;
        let of_sin_cos = sin * sin / 2.0;
        let of_sin_cos_squared = -cos.powi(3) / 3.0;
        let of_cos_cubed = sin - sin.powi(3) / 3.0;
        k * (of_cos - of_cos_squared)
            - r * lean * (of_sin_cos - of_sin_cos_squared)
            - r * (of_cos_squared - of_cos_cubed)
    };
    r * r * (at(PI / 2.0) - at(PI / 2.0 - turn))
}

/// What a plate of `area` seen from above and [`HEIGHT`] high keeps once its
/// full round, which alone takes `full_round_takes`, and the round of the
/// upright at a convex corner of its front, the corner's other wall turning
/// by `turn` from the front, are laid at [`RADIUS`], as the module's
/// documentation derives it: the plate, less what each takes alone, given
/// back what both take.
fn full_round_and_a_leaning_upright(area: f64, full_round_takes: f64, turn: f64) -> f64 {
    let r = RADIUS;
    let set_back = r * (1.0 - turn.cos()) / turn.sin();
    let upright_takes = HEIGHT * (r * set_back - r * r * turn / 2.0);
    area * HEIGHT - full_round_takes - upright_takes + shared_at_a_corner_turning_by(turn)
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
    full_round_tool_along(part, [0.0, LENGTH])
}

/// The prism the full round takes away, raised whole along X past both ends
/// of the stretch `along` covers: all that stands over the front and the back
/// outside the half cylinder about the line [`RADIUS`] under the middle of the
/// top.
fn full_round_tool_along(part: &Body, along: [f64; 2]) -> Body {
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
    raised_along_the_block(part, Contour { corners, runs }, along)
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
/// in. A patch closing a corner, numbered past every edge's rank, is read as
/// its own number: the lists here close no corner or one, or close both with
/// one face answering to both patches.
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

/// Holds that the fillet at [`RADIUS`] on `listed`'s edges of `part`, called
/// `on`, in every order the list can name them, is laid, each order's body
/// held by `holds` — given the order, the step's answer and what it is called
/// — and each the body the listed order lays, its faces meeting alike, each
/// laid face read by the edge it was laid on.
fn filleted_in_every_order(
    part: &Body,
    on: &str,
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
        format!(
            "the fillet at {RADIUS} on {}, on {on}",
            called.join(", then ")
        )
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
    filleted_in_every_order(&part, ON_THE_BLOCK, &listed, |order, filleted, what| {
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
    filleted_in_every_order(&part, ON_THE_BLOCK, &listed, |order, filleted, what| {
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

/// Holds that the fillet at [`RADIUS`] on `listed`'s edges of `part`, called
/// `on`, lays in every order the list can name them the body the arithmetic
/// gives, holding `promised`: one body, the value taken whole, exact, closed
/// and uncrossed, its triangles enclosing `promised` within the drawing's
/// tolerance over rounds no longer than the part reaches; then that
/// `by_booleans`, cutting the rounds' prisms from the part, leaves `promised`
/// too.
fn laid_as_the_arithmetic_gives_in_every_order(
    part: &Body,
    on: &str,
    listed: &[([usize; 2], &str)],
    promised: f64,
    by_booleans: impl FnOnce() -> f64,
) {
    let reach = reach_of(part);
    let rounds = PI / 2.0 * RADIUS * reach * listed.len() as f64;
    filleted_in_every_order(part, on, listed, |_, filleted, what| {
        taken_as_asked(filleted.asked, filleted.taken, RADIUS, what);
        sound_drawn_within(&filleted.body, promised, DRAWN * reach * rounds, what);
    });
    let left = by_booleans();
    assert!(
        (left - promised).abs() <= EXACT * promised,
        "the prisms of the rounds on {on}, cut from it by the booleans, leave {left}, where \
         arithmetic promises {promised}"
    );
}

/// The prism the round of the upright at the block's front corner at
/// x = [`LENGTH`] takes away.
fn front_right_tool(part: &Body) -> Body {
    Upright::Front.tool(part)
}

/// The prism the round of the upright at the left end's front corner takes
/// away: the outline arrives at the origin down the left end and leaves it
/// along the front.
fn front_left_tool(part: &Body) -> Body {
    upright_tool_at(part, DVec2::ZERO, -DVec2::Y, DVec2::X)
}

/// The prism the round of the upright at the left end's back corner takes
/// away: the outline arrives there along the back and leaves it down the left
/// end.
fn back_left_tool(part: &Body) -> Body {
    upright_tool_at(part, DVec2::new(0.0, WIDTH), -DVec2::X, -DVec2::Y)
}

#[test]
fn a_fillet_on_a_full_round_and_an_upright_at_its_far_end_or_one_at_each_end_mitres_each_upright_s_round_against_the_full_round_whatever_the_order()
 {
    let part = block(WIDTH);

    let at_the_far_end = [PAIR[0], PAIR[1], FRONT_LEFT_EDGE];
    let edges = at_the_far_end.map(|(edge, _)| edge);
    the_arithmetic_gives(&edges, 1215.0 + 144.0 * PI, "1215 + 144π");
    let tools: [&Tool; 2] = [&full_round_tool, &front_left_tool];
    laid_as_the_arithmetic_gives_in_every_order(
        &part,
        ON_THE_BLOCK,
        &at_the_far_end,
        kept(Cut::Fillet, WIDTH, &edges),
        || left_by_booleans(&part, &tools),
    );

    let one_at_each_end = [PAIR[0], PAIR[1], Upright::Front.edge(), BACK_LEFT_EDGE];
    let edges = one_at_each_end.map(|(edge, _)| edge);
    the_arithmetic_gives(&edges, 1170.0 + 153.0 * PI, "1170 + 153π");
    let tools: [&Tool; 3] = [&full_round_tool, &front_right_tool, &back_left_tool];
    laid_as_the_arithmetic_gives_in_every_order(
        &part,
        ON_THE_BLOCK,
        &one_at_each_end,
        kept(Cut::Fillet, WIDTH, &edges),
        || left_by_booleans(&part, &tools),
    );
}

#[test]
fn a_fillet_on_a_full_round_the_edge_across_one_end_and_an_upright_at_the_other_mitres_both_against_the_full_round_whatever_the_order()
 {
    let part = block(WIDTH);
    let listed = [PAIR[0], PAIR[1], END_EDGE, FRONT_LEFT_EDGE];
    let edges = listed.map(|(edge, _)| edge);
    the_arithmetic_gives(&edges, 1251.0 + 261.0 * PI / 2.0, "1251 + 261π/2");
    let tools: [&Tool; 3] = [&full_round_tool, &end_edge_tool, &front_left_tool];
    laid_as_the_arithmetic_gives_in_every_order(
        &part,
        ON_THE_BLOCK,
        &listed,
        kept(Cut::Fillet, WIDTH, &edges),
        || left_by_booleans(&part, &tools),
    );
}

#[test]
fn a_fillet_on_both_uprights_at_an_end_and_one_long_edge_of_the_top_or_the_floor_mitres_that_edge_s_round_against_their_standing_full_round_whatever_the_order()
 {
    let part = block(WIDTH);
    let standing = [Upright::Front.edge(), Upright::Back.edge()];
    let lying: [(([usize; 2], &str), &Tool); 2] = [
        (PAIR[0], &top_front_tool),
        (FLOOR_FRONT_EDGE, &floor_front_tool),
    ];
    for (edge, tool) in lying {
        let listed = [standing[0], standing[1], edge];
        let edges = listed.map(|(edge, _)| edge);
        the_arithmetic_gives(&edges, 1395.0 + 99.0 * PI, "1395 + 99π");
        let tools: [&Tool; 2] = [&standing_full_round_tool, tool];
        laid_as_the_arithmetic_gives_in_every_order(
            &part,
            ON_THE_BLOCK,
            &listed,
            kept(Cut::Fillet, WIDTH, &edges),
            || left_by_booleans(&part, &tools),
        );
    }
}

#[test]
fn a_fillet_on_full_rounds_along_the_top_and_the_floor_and_the_upright_at_their_end_mitres_the_upright_s_round_against_both_whatever_the_order()
 {
    let part = block(WIDTH);
    let listed = [
        PAIR[0],
        PAIR[1],
        FLOOR_FRONT_EDGE,
        FLOOR_BACK_EDGE,
        Upright::Front.edge(),
    ];
    let edges = listed.map(|(edge, _)| edge);
    the_arithmetic_gives(&edges, 720.0 + 531.0 * PI / 2.0, "720 + 531π/2");
    let tools: [&Tool; 3] = [&full_round_tool, &floor_full_round_tool, &front_right_tool];
    laid_as_the_arithmetic_gives_in_every_order(
        &part,
        ON_THE_BLOCK,
        &listed,
        kept(Cut::Fillet, WIDTH, &edges),
        || left_by_booleans(&part, &tools),
    );
}

/// Holds that the arithmetic of [`full_round_and_a_leaning_upright`] gives
/// the volume the module's documentation writes out for a plate.
fn the_leaning_arithmetic_gives(promised: f64, volume: f64, written: &str) {
    assert!(
        (promised - volume).abs() <= EXACT * volume,
        "the arithmetic gives {promised}, where the documentation writes {written}"
    );
}

#[test]
fn a_fillet_on_a_full_round_and_the_upright_at_a_corner_of_its_pointed_end_mitres_the_upright_s_round_against_it_up_to_the_point_s_wall_whatever_the_order()
 {
    let r = RADIUS;
    let tip = 33.0;
    let part = plate(vec![
        DVec2::ZERO,
        DVec2::new(LENGTH, 0.0),
        DVec2::new(tip, r),
        DVec2::new(LENGTH, WIDTH),
        DVec2::new(0.0, WIDTH),
    ]);
    let [point_front, back] = [3, 5];
    let listed = [
        ([TOP, FRONT], "the top's front edge"),
        ([TOP, back], "the top's back edge"),
        (
            [FRONT, point_front],
            "the upright where the front meets the point",
        ),
    ];
    let promised = full_round_and_a_leaning_upright(
        189.0,
        30.0 * r * r * (2.0 - PI / 2.0) + 45.0 - 27.0 * PI / 2.0,
        PI / 4.0,
    );
    the_leaning_arithmetic_gives(
        promised,
        1386.0 + 1251.0 * PI / 8.0 - 261.0 * SQRT_2 / 4.0 - 27.0 * SQRT_2 * PI / 8.0,
        "1386 + 1251π/8 − 261√2/4 − 27√2π/8",
    );
    let full_round = move |left: &Body| full_round_tool_along(left, [0.0, tip]);
    let upright = move |left: &Body| {
        upright_tool_at(
            left,
            DVec2::new(LENGTH, 0.0),
            DVec2::X,
            DVec2::new(1.0, 1.0).normalize(),
        )
    };
    let tools: [&Tool; 2] = [&full_round, &upright];
    laid_as_the_arithmetic_gives_in_every_order(
        &part,
        "#843's plate pointed at (33, 3)",
        &listed,
        promised,
        || left_by_booleans(&part, &tools),
    );
}

#[test]
fn a_fillet_on_a_full_round_and_the_upright_at_a_corner_of_its_slanted_end_mitres_the_upright_s_round_against_it_up_to_the_end_wall_whatever_the_order()
 {
    let r = RADIUS;
    let back_end = 32.0;
    let part = plate(vec![
        DVec2::ZERO,
        DVec2::new(LENGTH, 0.0),
        DVec2::new(back_end, WIDTH),
        DVec2::new(0.0, WIDTH),
    ]);
    let back = 4;
    let listed = [
        ([TOP, FRONT], "the top's front edge"),
        ([TOP, back], "the top's back edge"),
        (
            [FRONT, END],
            "the upright where the front meets the slanted end",
        ),
    ];
    let turn = 3.0_f64.atan();
    let promised = full_round_and_a_leaning_upright(186.0, 31.0 * r * r * (2.0 - PI / 2.0), turn);
    let root_ten = 10.0_f64.sqrt();
    the_leaning_arithmetic_gives(
        promised,
        1341.0 + 279.0 * PI / 2.0 - 429.0 * root_ten / 20.0 + 9.0 * (7.0 - root_ten) / 2.0 * turn,
        "1341 + 279π/2 − 429√10/20 + 9(7 − √10)·atan 3/2",
    );
    let full_round = move |left: &Body| full_round_tool_along(left, [0.0, back_end]);
    let upright = move |left: &Body| {
        upright_tool_at(
            left,
            DVec2::new(LENGTH, 0.0),
            DVec2::X,
            DVec2::new(back_end - LENGTH, WIDTH).normalize(),
        )
    };
    let tools: [&Tool; 2] = [&full_round, &upright];
    laid_as_the_arithmetic_gives_in_every_order(
        &part,
        "the plate whose end runs from (30, 0) to (32, 6)",
        &listed,
        promised,
        || left_by_booleans(&part, &tools),
    );
}

#[test]
fn a_fillet_on_a_full_round_a_hair_short_the_upright_at_its_end_and_the_floor_s_front_edge_takes_the_value_whole_whatever_the_order()
 {
    let hair_wide = WIDTH + 1e-12;
    let part = block(hair_wide);
    let listed = [PAIR[0], PAIR[1], Upright::Front.edge(), FLOOR_FRONT_EDGE];
    let edges = listed.map(|(edge, _)| edge);
    the_arithmetic_gives(&edges, 990.0 + 198.0 * PI, "990 + 198π");
    let tools: [&Tool; 3] = [&full_round_tool, &front_right_tool, &floor_front_tool];
    laid_as_the_arithmetic_gives_in_every_order(
        &part,
        "the block 6 + 10⁻¹² wide",
        &listed,
        kept(Cut::Fillet, hair_wide, &edges),
        || left_by_booleans(&block(WIDTH), &tools),
    );
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

/// What the block keeps once the tongue's five edges are rounded at
/// [`RADIUS`] and its tip closed by the quarter ball, computed apart from the
/// step as #699 computes a corner's piece of sphere: the prisms of the lying
/// full round, of the standing one and of the end edge's round, each raised
/// whole, cut away by the booleans; then, of what is left, the part past both
/// planes through the crossing square to the block and to the uprights,
/// x = 27 and z = 7 — the cell the quarter ball fills — taken away, and the
/// ball's sector over that cell given back, a third of the radius cubed times
/// the quarter's solid angle, π.
fn tip_by_booleans(part: &Body) -> f64 {
    let tools: [&Tool; 3] = [&full_round_tool, &standing_full_round_tool, &end_edge_tool];
    let rounded = tools.iter().fold(part.clone(), |left, tool| {
        left.difference(&tool(&left))
            .expect("a round's prism cuts the block")
    });
    let cell = [-DVec3::X, -DVec3::Z]
        .into_iter()
        .fold(rounded.clone(), |left, away| {
            left.difference(&beyond(&left, crossing(), away, 100.0))
                .expect("a plane through the crossing cuts the block")
        });
    rounded.volume() - cell.volume() + RADIUS.powi(3) * PI / 3.0
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

/// Holds that the fillet at [`RADIUS`] on the tongue's five edges — the full
/// round's pair, the edge across the end and both uprights, listed in that
/// order — named in `order` and laid as `filleted`, takes the value whole and
/// rounds the tip into a quarter of a ball, as [`shaped_as_the_quarter_ball`]
/// holds it, its faces numbered as #698 and #699 number them: each full round
/// one face answering to both its edges' ranks; the quarter ball one face
/// answering to the end edge's rank and to the two patches #699 numbers after
/// every edge's, one for each corner it closes; the top and the end gone; and
/// the next step numbering its faces past the patches.
fn rounds_the_tip_into_a_quarter_ball(
    part: &Body,
    order: &[usize],
    filleted: &Filleted,
    what: &str,
) {
    let start = part.faces_end();
    let count = order.len();
    let laid_end = start + count + 2;
    taken_as_asked(filleted.asked, filleted.taken, RADIUS, what);
    let body = &filleted.body;
    let lying = ranks_of(start, order, &[0, 1]);
    let standing = ranks_of(start, order, &[3, 4]);
    let mut ball = ranks_of(start, order, &[2]);
    ball.extend(start + count..laid_end);

    for (gone, which) in [(TOP, "the top"), (END, "the end")] {
        assert!(
            !body.has_face(gone),
            "{what}: {which} is gone, the tip rounded over it"
        );
    }
    let mut kept_and_laid = numbers(part);
    kept_and_laid.remove(&TOP);
    kept_and_laid.remove(&END);
    kept_and_laid.extend(start..laid_end);
    assert_eq!(
        numbers(body),
        kept_and_laid,
        "{what}: every face of the block but its top and its end, the lying full round \
         answering to {lying:?}, the standing one to {standing:?}, and the quarter ball to \
         {ball:?}, the end edge's rank and a patch for each corner it closes"
    );
    assert_eq!(
        body.faces_end(),
        laid_end,
        "{what}: the next step numbers its faces past the {count} the list gave its edges and \
         the two patches of the corners the quarter ball closes"
    );
    one_curved_face_answers_to(body, &lying, &format!("{what}: the lying full round"));
    one_curved_face_answers_to(body, &standing, &format!("{what}: the standing full round"));
    one_curved_face_answers_to(body, &ball, &format!("{what}: the quarter ball"));
    shaped_as_the_quarter_ball(part, body, &lying, &standing, &ball, what);
}

/// Holds that `body` is the block's tongue rounded all round, the lying full
/// round answering to the numbers `lying`, the standing one to `standing` and
/// the quarter ball to `ball`: the body exact, closed, uncrossed and drawn,
/// holding what arithmetic promises; the quarter ball drawn on the sphere of
/// [`RADIUS`] about the crossing of the two full rounds' axes, past both
/// planes x = 27 and z = 7 only, and met there from outside; each full round
/// drawn on its own half cylinder and on its own side of its plane, and met
/// there from outside; the lying one meeting the quarter ball along the half
/// circle x = 27, the standing one along the half circle z = 7, both drawn
/// between their two ends; the two full rounds sharing those two ends only,
/// each one vertex where the wall there, both full rounds and the quarter ball
/// meet; the front, the back and the floor giving up what lies outside the
/// standing half circle; and every face meeting the faces it bounds, one edge
/// each, the quarter ball the two full rounds only.
fn shaped_as_the_quarter_ball(
    part: &Body,
    body: &Body,
    lying: &[usize],
    standing: &[usize],
    ball: &[usize],
    what: &str,
) {
    let r = RADIUS;
    let centre = crossing();
    let lying_halves = [-DVec3::Y, DVec3::Y].map(lying_half);
    let standing_halves = [-DVec3::Y, DVec3::Y].map(standing_half);
    let area: f64 = lying_halves
        .iter()
        .chain(&standing_halves)
        .map(Round::area)
        .sum::<f64>()
        + PI * r * r;
    sound_drawn_within(
        body,
        kept(
            Cut::Fillet,
            WIDTH,
            &[TOP_FRONT, TOP_BACK, TOP_END, FRONT_UPRIGHT, BACK_UPRIGHT],
        ),
        DRAWN * reach_of(part) * area,
        what,
    );

    let ball_drawn: Vec<DVec3> = body.triangles_of(ball[0]).flatten().collect();
    for &point in &ball_drawn {
        assert!(
            (point.distance(centre) - r).abs() < PLACE
                && point.x >= centre.x - PLACE
                && point.z >= centre.z - PLACE,
            "{what}: the quarter ball is drawn through {point}, off the quarter of the sphere of \
             {r} about {centre} past the planes x = {} and z = {}",
            centre.x,
            centre.z
        );
    }
    for way in [
        DVec3::new(1.0, 0.0, 1.0),
        DVec3::new(2.0, 0.0, 1.0),
        DVec3::new(1.0, 0.0, 2.0),
        DVec3::new(1.0, -2.0, 1.0),
        DVec3::new(1.0, 2.0, 1.0),
        DVec3::new(2.0, -1.0, 2.0),
        DVec3::new(2.0, 1.0, 2.0),
    ] {
        let facing = way.normalize();
        let from = centre + facing * (r + 10.0);
        let hit = body.ray_hit(from, -facing).unwrap_or_else(|| {
            panic!("{what}: a ray from {from} toward the quarter ball's centre meets nothing")
        });
        assert_eq!(
            hit.face, ball[0],
            "{what}: a ray from {from} toward the quarter ball's centre meets face {} before it",
            hit.face
        );
        assert!(
            hit.normal.distance(facing) < PLACE,
            "{what}: where a ray from {from} meets it, the quarter ball faces {}, not {facing}, \
             away from its centre",
            hit.normal
        );
    }

    let lying_drawn: Vec<DVec3> = body.triangles_of(lying[0]).flatten().collect();
    for &point in &lying_drawn {
        assert!(
            lying_halves.iter().any(|half| half.holds(point)) && point.x <= centre.x + PLACE,
            "{what}: the lying full round is drawn through {point}, off its half cylinder of {r} \
             about the line {r} under the middle of the top, or past x = {}, where the quarter \
             ball takes over",
            centre.x
        );
    }
    let standing_drawn: Vec<DVec3> = body.triangles_of(standing[0]).flatten().collect();
    for &point in &standing_drawn {
        assert!(
            standing_halves.iter().any(|half| half.holds(point)) && point.z <= centre.z + PLACE,
            "{what}: the standing full round is drawn through {point}, off its half cylinder of \
             {r} about the vertical through {}, or above z = {}, where the quarter ball takes over",
            centre.truncate(),
            centre.z
        );
    }
    for half in &lying_halves {
        met_from_outside_where(
            body,
            lying[0],
            half,
            |aim| aim.x < centre.x - CLEAR_OF_THE_MITRE,
            &format!("{what}: the lying full round, short of the quarter ball"),
        );
    }
    for half in &standing_halves {
        met_from_outside_where(
            body,
            standing[0],
            half,
            |aim| aim.z < centre.z - CLEAR_OF_THE_MITRE,
            &format!("{what}: the standing full round, under the quarter ball"),
        );
    }

    let ends = [0.0, WIDTH].map(|y| DVec3::new(centre.x, y, centre.z));
    for end in ends {
        for (drawn, which) in [
            (&lying_drawn, "the lying full round"),
            (&standing_drawn, "the standing full round"),
            (&ball_drawn, "the quarter ball"),
        ] {
            assert!(
                drawn_through(drawn, end),
                "{what}: {which} is drawn up to {end}, where the quarter ball's two half circles \
                 meet"
            );
        }
    }
    for (seam, plane, way, called) in [
        (
            shared_points(&lying_drawn, &ball_drawn),
            DVec3::X,
            DVec3::Z,
            "the lying full round",
        ),
        (
            shared_points(&standing_drawn, &ball_drawn),
            DVec3::Z,
            DVec3::X,
            "the standing full round",
        ),
    ] {
        for &point in &seam {
            assert!(
                ((point - centre).dot(plane)).abs() < PLACE
                    && on_the_circle_toward(point, centre, r, &[way]),
                "{what}: {called} and the quarter ball meet at {point}, off the half circle of {r} \
                 about {centre} square to {plane}"
            );
        }
        assert!(
            seam.len() > 2 && ends.iter().all(|&end| drawn_through(&seam, end)),
            "{what}: {called} and the quarter ball meet along the half circle square to {plane}, \
             drawn from {} to {} through the points between: {seam:?}",
            ends[0],
            ends[1]
        );
    }
    let full_rounds_meet = shared_points(&lying_drawn, &standing_drawn);
    assert!(
        same_points(&full_rounds_meet, &ends),
        "{what}: the two full rounds share no point but {} and {}, where both meet the quarter \
         ball: {full_rounds_meet:?}",
        ends[0],
        ends[1]
    );
    for (wall, end) in [(FRONT, ends[0]), (BACK, ends[1])] {
        assert_eq!(
            vertices_among(body, &[&[wall], lying, standing, ball]),
            1,
            "{what}: the wall, the two full rounds and the quarter ball meet at one vertex, {end}"
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
             each, and the quarter ball at a point only"
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
        |corner| on_the_circle_toward(corner, centre.truncate().extend(0.0), r, &[DVec3::X]),
        &format!(
            "{what}: the floor, its end given up to the standing full round along the half \
             circle of {r} about {}",
            centre.truncate()
        ),
    );
    assert_eq!(
        sides_met_by(body, &[FLOOR]),
        sides_answering(&[&[FRONT], &[BACK], &[LEFT], standing]),
        "{what}: the floor meets the front, the back, the left end and the standing full round, \
         by one edge each"
    );
    assert_eq!(
        sides_met_by(body, &[LEFT]),
        sides_answering(&[&[FLOOR], &[FRONT], &[BACK], lying]),
        "{what}: the left end meets the floor, the front, the back and the lying full round, by \
         one edge each"
    );
    assert_eq!(
        sides_met_by(body, lying),
        sides_answering(&[&[FRONT], &[BACK], &[LEFT], ball]),
        "{what}: the lying full round meets the front and the back along its sides, the left \
         end, and the quarter ball along the half circle x = 27, by one edge each"
    );
    assert_eq!(
        sides_met_by(body, standing),
        sides_answering(&[&[FLOOR], &[FRONT], &[BACK], ball]),
        "{what}: the standing full round meets the floor, the front and the back along its \
         sides, and the quarter ball along the half circle z = 7, by one edge each"
    );
    assert_eq!(
        sides_met_by(body, ball),
        sides_answering(&[lying, standing]),
        "{what}: the quarter ball meets the two full rounds, by one edge each, and nothing else"
    );
}

#[test]
fn a_fillet_on_a_full_round_the_edge_across_its_end_and_both_uprights_there_rounds_the_tongue_s_tip_into_a_quarter_of_a_ball_whatever_the_order()
 {
    let part = block(WIDTH);
    let listed = [
        PAIR[0],
        PAIR[1],
        END_EDGE,
        Upright::Front.edge(),
        Upright::Back.edge(),
    ];
    let edges = listed.map(|(edge, _)| edge);
    the_arithmetic_gives(&edges, 1134.0 + 162.0 * PI, "1134 + 162π");
    let promised = kept(Cut::Fillet, WIDTH, &edges);
    let by_booleans = tip_by_booleans(&part);
    assert!(
        (by_booleans - promised).abs() <= EXACT * promised,
        "the prisms of the lying and the standing full rounds and of the end edge's round, cut \
         from the block by the booleans, the cell past x = 27 and z = 7 taken away and the \
         quarter ball given back, leave {by_booleans}, where arithmetic promises {promised}"
    );
    filleted_in_every_order(&part, ON_THE_BLOCK, &listed, |order, filleted, what| {
        rounds_the_tip_into_a_quarter_ball(&part, order, filleted, what);
    });
}
