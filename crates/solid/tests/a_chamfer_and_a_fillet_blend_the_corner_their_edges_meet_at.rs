//! A chamfer and a fillet on straight edges of an exact body that meet at a
//! corner, at one value. Where the corner is convex — each of its three edges
//! meeting outwards — two of its edges cut trim each other in a mitre and
//! leave the third sharp, and all three are closed by a patch of their own
//! kind: a triangle among flats, a piece of sphere among rounds. A corner with
//! an edge meeting inwards is declined here, but for those #801 builds, held
//! in `a_chamfer_and_a_fillet_stop_and_mitre_beside_a_concave_edge.rs`. An
//! operation of the kernel on [`Body`], as the step on several edges of #698
//! is; the step of the history that calls it comes with the application, in
//! #701.
//!
//! Closes #699.
//! - Three edges of a convex corner chamfered at one value leave a closed
//!   body with a triangular face at that corner, meeting the three chamfers;
//!   three filleted at one value leave a closed body with a sphere piece
//!   there, meeting the three fillets. Volume exact, soundness checks pass —
//!   on the block's square corner,
//!   `a_chamfer_on_the_three_edges_of_a_corner_closes_it_with_a_triangle_where_the_flats_edges_cross`,
//!   `a_fillet_on_the_three_edges_of_a_corner_closes_it_with_a_piece_of_sphere_one_radius_from_its_three_faces`;
//!   and on a convex corner that is not square, as the third decision asks,
//!   the slanted prism's end, two of its edges square to the third —
//!   `a_chamfer_on_the_three_edges_of_a_corner_that_is_not_square_closes_it_with_a_triangle_where_the_flats_edges_cross`,
//!   `a_fillet_on_the_three_edges_of_a_corner_that_is_not_square_closes_it_with_a_piece_of_sphere_one_radius_from_its_three_faces`;
//!   and on one where no two of the three edges are square, a block whose
//!   top is cut by a plane tilted two ways, its flats closed by their
//!   triangle —
//!   `a_chamfer_on_the_three_edges_of_a_corner_where_no_two_edges_are_square_closes_it_with_a_triangle_where_the_flats_edges_cross`
//!   — and its rounds by their piece of sphere, since #800 draws it; a broken
//!   step with its reason, `Undrawn`, before —
//!   `a_fillet_on_the_three_edges_of_a_corner_where_no_two_edges_are_square_closes_it_with_a_piece_of_sphere_one_radius_from_its_three_faces`;
//!   and several corners closed in one step, the body the same whatever the
//!   order the list names their edges: an upright of the block and the four
//!   edges at its two ends, two corners, and the twelve edges of the block,
//!   eight —
//!   `a_chamfer_and_a_fillet_on_an_upright_and_the_four_edges_at_its_two_ends_close_both_corners_whatever_the_order`,
//!   `a_chamfer_and_a_fillet_on_the_twelve_edges_of_a_block_close_its_eight_corners_into_one_body_whatever_the_order`;
//!   and two corners that are not square, at both ends of the slanted
//!   prism's top edge on its end, where the two cuts on either side cross
//!   that edge apart, not at one point —
//!   `a_chamfer_and_a_fillet_on_the_edges_at_both_ends_of_an_edge_whose_corners_are_not_square_close_both_whatever_the_order`;
//!   and edges named before a later join merged the faces at their corner
//!   with a neighbour's, the names still finding them, mitred or closed in
//!   any order —
//!   `a_chamfer_and_a_fillet_on_edges_named_before_a_join_merged_the_faces_at_their_corner_mitre_or_close_it_whatever_the_order`
//! - The triangle answers to the number after its edges' flats: on a list of
//!   `n` edges from where the steps' numbers ended at `start`, the flats are
//!   `start` to `start + n − 1` in the order the list names their edges, as
//!   #698 numbers them, and the triangle `start + n`, the step ending past it
//!   so that the next step numbers its own faces after the triangle. The
//!   triangle belongs to no one edge, so no edge's rank can name it; placed
//!   after them all, it moves no flat's number, whichever edge the list names
//!   first. The piece of sphere of three rounds is numbered the same way.
//!   Beyond the issue's wording, a choice of this branch —
//!   `a_chamfer_on_the_three_edges_of_a_corner_closes_it_with_a_triangle_where_the_flats_edges_cross`,
//!   `a_fillet_on_the_three_edges_of_a_corner_closes_it_with_a_piece_of_sphere_one_radius_from_its_three_faces`.
//!   Several corners closed in one step number their patches after every
//!   edge's face, one each, in the order of the lowest rank the list gives
//!   one of a corner's three edges, and of the next where two corners share
//!   that edge: the order the user named them in, never the kernel's own
//!   order of its vertices. Two corners share at most one edge, so the order
//!   is whole —
//!   `a_chamfer_and_a_fillet_on_an_upright_and_the_four_edges_at_its_two_ends_close_both_corners_whatever_the_order`,
//!   `a_chamfer_and_a_fillet_on_the_twelve_edges_of_a_block_close_its_eight_corners_into_one_body_whatever_the_order`,
//!   `a_chamfer_and_a_fillet_on_the_edges_at_both_ends_of_an_edge_whose_corners_are_not_square_close_both_whatever_the_order`,
//!   each on lists naming the same edges in different orders
//! - Decision: the triangle of three chamfers has its corners where the
//!   chamfer edges cross on each of the three faces; the sphere piece of three
//!   fillets is centred at the point one value from all three faces, and
//!   meets each fillet along that fillet's end section. Both hold for any
//!   convex trihedral corner, not only a square one. This branch held the
//!   triangle at any convex corner, and the sphere where one edge is square
//!   to the other two, as at the block's corner and the slanted prism's;
//!   where no two edges are square, the sphere's third arc is a great circle
//!   that is neither a meridian nor the equator of its axis, which the
//!   `(θ, φ)` chart of the sphere cannot follow, and the step was a broken
//!   step with its reason, `Undrawn`. Decision of #800: such a piece of
//!   sphere is drawn on a chart of its own, its central projection from the
//!   sphere's centre onto a plane tangent at a direction the whole piece lies
//!   in front of, where every great circle is a straight line, its boundary
//!   sampled as its neighbours share it; a face the `(θ, φ)` chart draws
//!   keeps its path. The tests of the criterion above, which hold the
//!   triangle's three corners and the sphere's centre, radius and the arcs
//!   it shares with the rounds:
//!   `a_chamfer_on_the_three_edges_of_a_corner_closes_it_with_a_triangle_where_the_flats_edges_cross`,
//!   `a_fillet_on_the_three_edges_of_a_corner_closes_it_with_a_piece_of_sphere_one_radius_from_its_three_faces`,
//!   `a_chamfer_on_the_three_edges_of_a_corner_that_is_not_square_closes_it_with_a_triangle_where_the_flats_edges_cross`,
//!   `a_fillet_on_the_three_edges_of_a_corner_that_is_not_square_closes_it_with_a_piece_of_sphere_one_radius_from_its_three_faces`,
//!   `a_chamfer_on_the_three_edges_of_a_corner_where_no_two_edges_are_square_closes_it_with_a_triangle_where_the_flats_edges_cross`,
//!   `a_fillet_on_the_three_edges_of_a_corner_where_no_two_edges_are_square_closes_it_with_a_piece_of_sphere_one_radius_from_its_three_faces`
//! - Two of a convex corner's three edges chamfered or rounded leave a closed
//!   body mitred at that corner, the third edge sharp. Volume exact, soundness
//!   checks pass — on each of the corner's three pairs, named in either order,
//!   `a_chamfer_on_two_of_a_corner_s_three_edges_mitres_their_flats_and_leaves_the_third_edge_sharp_one_distance_short`,
//!   `a_fillet_on_two_of_a_corner_s_three_edges_mitres_their_rounds_on_an_ellipse_and_leaves_the_third_edge_sharp_one_radius_short`;
//!   and on the case the issue opens on, the four top edges of a block, a
//!   mitre at each of its four corners —
//!   `a_chamfer_and_a_fillet_on_the_four_top_edges_of_a_block_mitre_its_four_corners`;
//!   and on a convex corner that is not square, the end of a prism whose
//!   sides slant, every pair of its three edges — the slanted upright among
//!   them — mitred whatever the angles —
//!   `a_chamfer_and_a_fillet_on_two_of_the_three_edges_of_a_corner_that_is_not_square_mitre_them_the_third_left_sharp`;
//!   and where the corner's uncut edge runs at its far end into a round an
//!   earlier step laid, which does not make that edge any less sharp at the
//!   corner —
//!   `a_chamfer_and_a_fillet_mitre_two_edges_whose_corner_s_uncut_edge_runs_into_an_earlier_round_at_its_far_end`
//! - Decision: two of three is a mitre. The two cuts trim each other: a
//!   straight mitre line for chamfers, a planar ellipse arc for fillets,
//!   running from where the two cuts' edges cross on their shared face down to
//!   the third edge, which stays sharp and ends one value short of the old
//!   vertex. No patch is added —
//!   `a_chamfer_on_two_of_a_corner_s_three_edges_mitres_their_flats_and_leaves_the_third_edge_sharp_one_distance_short`,
//!   `a_fillet_on_two_of_a_corner_s_three_edges_mitres_their_rounds_on_an_ellipse_and_leaves_the_third_edge_sharp_one_radius_short`
//! - A corner with a concave edge, and any corner the kernel cannot blend,
//!   gives a broken step with its reason, never an open body. Decision: a
//!   corner is blended only when every one of its edges is convex; the inner
//!   corner of an L-shaped block and the bottom of a pocket are declined —
//!   `a_fillet_and_a_chamfer_on_edges_meeting_at_a_corner_with_a_concave_edge_are_declined_as_unsupported`;
//!   the reason is `Unsupported`, the kernel's word for a shape it does not
//!   build, until #801, the follow-up issue the decision promised, builds
//!   those corners. Its first slice builds every edge at such a corner alone
//!   — the L's top and floor edges at its inner corner, a pocket's rim, a
//!   standing block's uprights, each stopping flush on the face it runs into
//!   — and two convex edges meeting where the third is concave, mitred, the
//!   third left sharp: the L's two top edges at its inner corner and two edges
//!   of a pocket's rim, which this test declined before, held now in
//!   `a_chamfer_and_a_fillet_stop_and_mitre_beside_a_concave_edge.rs`. Its
//!   second slice builds two concave edges meeting where the third is convex,
//!   mitred, the third left sharp: two feet of a block standing on another,
//!   meeting where its upright meets outwards, and its four feet, which this
//!   test declined before, held now in
//!   `a_chamfer_and_a_fillet_fill_concave_edges_meeting_at_a_corner.rs`. Its
//!   third slice builds a pocket's floor edges and uprights alone, each fill
//!   trimmed flush on the faces at its ends, and two of the three concave
//!   edges at a pocket's bottom corner, mitred, the third left sharp: two
//!   edges of its floor, or one and the upright it meets, which this test
//!   declined before, held now in that same file. #829's first slice builds
//!   the three edges at a standing block's foot corner, two feet and the
//!   upright above them, and its second the three at the top of the L's
//!   inner corner, its two top edges and the inner upright below them, which
//!   this test declined before, held now in
//!   `a_chamfer_and_a_fillet_go_round_the_odd_edge_at_a_corner_of_mixed_sense.rs`.
//!   The lists left here are a cut meeting a fill at a corner, declined at
//!   the corner itself, each of their edges laid alone — a foot and the
//!   upright above it, the L's top edge and its inner upright — until #846
//! - A full round, two parallel edges whose rounds meet across the face they
//!   share, meeting a third edge of the list at a corner, was not built here:
//!   the fillet was declined `Unsupported` whichever edge the list names
//!   first, where the pair alone is laid. Since #803 the third edge's round
//!   is mitred against the full round, the body held below in place of the
//!   decline —
//!   `a_fillet_on_a_full_round_pair_and_an_edge_meeting_it_at_a_corner_mitres_that_edge_s_round_against_the_full_round_whatever_the_order`;
//!   a full round meeting an upright at a corner is #837's
//! - A face laid in place of others through `replaced`, as the piece of
//!   sphere is, is verified against the surface it is laid on, so that a
//!   wrong one is a broken step and never a body read as wrong geometry — no
//!   test: held beside the code, in `crates/solid/src/brep/assembly/tests.rs`,
//!   by `a_face_laid_on_a_surface_its_loop_does_not_lie_on_is_unverified`,
//!   `a_face_whose_corners_lie_on_its_surface_but_whose_edges_bow_away_from_it_is_unverified`
//!   and `a_face_laid_again_on_its_own_plane_is_taken`. The check reads each
//!   edge at its two corners and a quarter, half and three quarters along. Two
//!   corners and three samples settle a line or an arc on a plane, a sphere,
//!   a cylinder or a cone, none of which a line or a circle crosses more than
//!   four times without lying on it; a whole circle, which has no corners, is
//!   settled by its three samples only on a plane or a sphere. Sampled
//!   anywhere else along the same curve, the check decides the same there, so
//!   no fixture on those surfaces tells where along the edge the samples stand
//! - Three named edges at a vertex where four edges end are declined
//!   `Unsupported`, for the fillet and the chamfer, whichever three, and the
//!   four as well — `a_fillet_and_a_chamfer_on_three_of_the_four_edges_at_a_pyramid_s_apex_are_declined_as_unsupported`.
//!   The guard that counts the edges ending there is not alone in holding
//!   it: around such a vertex the faces run in a ring, so any three of its
//!   edges hold two that share no face, and the corner's closing, reading
//!   where each two of them cross on the face they share, declines it too
//! - A later edge the step's own earlier cut takes whole at their corner is
//!   laid even so, as the body given reads it, so that the step lays both
//!   cuts whole whichever edge the list names first, as every mitre here
//!   does: on the block whose corner a facet of 0.5 cuts off, the top's front
//!   edge's round is capped at 0.5 by the facet's corner on the front, and
//!   there takes the whole of the facet's edge on the front; the fillet on
//!   the two, at the cap or asked more, is the same body in either order,
//!   the two rounds' prisms cut whole, 5998.38 by the booleans. The chamfer
//!   on them is declined `Unsupported` in either order, as the top's front
//!   edge is alone at that cap, its flat passing through the facet's far
//!   edge and taking the facet whole: a list lays no edge its own step
//!   declines —
//!   `a_fillet_where_one_round_takes_the_next_edge_whole_lays_both_rounds_in_either_order_and_a_chamfer_declines_in_both`
//! - A corner whose piece of sphere would stand behind it along one of its
//!   edges, the centre one radius from the three faces lying past the corner
//!   on that edge's line, is declined `Unsupported` among rounds: the round
//!   of that edge would end on a plane through the centre past the corner.
//!   The same corner closes with its triangle among flats —
//!   `a_fillet_on_the_three_edges_of_a_corner_whose_sphere_would_stand_behind_it_along_an_edge_is_declined_as_unsupported`
//! - An edge of the body just cut on a later edge's line but outside its
//!   stretch, which the step must not take for the later edge — no test: it
//!   cannot be reached through the public API. Such an edge has matter right
//!   beside the line, within the margin the later edge's own prism stands off
//!   its faces, carried past the body's box along the line; the later edge
//!   is declined alone, `Unsupported`, before any corner is read. A slot
//!   across the block's top, parting its front edge in two, does so for each
//!   piece
//! - Any other corner the kernel cannot blend — no test: a corner of more
//!   than three edges, of values that differ, or of a curved edge is out of
//!   the issue's scope, and no list of edges of a block, an L or a pocket
//!   reaches one; the concave corners above are the ones this file can build
//! - The tests of #698 and the single-edge issues still pass — no test: held
//!   by those files, left as they were but for the one test #699 takes back.
//!   `a_fillet_and_a_chamfer_on_two_edges_sharing_a_vertex_are_declined_as_unsupported`
//!   held that two edges meeting at the block's top front right corner are
//!   declined; they now meet in a mitre, and its two pairs are two of the
//!   three the mitre tests above hold
//!
//! Closes #800.
//! - Three fillets at a convex corner where no two edges are square leave a
//!   drawn, closed, uncrossed body with its piece of sphere — on the tilted
//!   block's corner, in every order the list names its three edges,
//!   `a_fillet_on_the_three_edges_of_a_corner_where_no_two_edges_are_square_closes_it_with_a_piece_of_sphere_one_radius_from_its_three_faces`;
//!   and the drawing alone, on a sector of a ball whose piece of sphere is
//!   bounded by three great-circle arcs none of which is a meridian or the
//!   equator of the sphere's axis, built by hand beside the drawing —
//!   `a_sector_of_a_ball_bounded_by_three_great_circle_arcs_none_a_meridian_nor_the_equator_is_drawn_closed_and_encloses_its_volume`,
//!   in `crates/solid/src/brep/tessellation/tests/rounds.rs`
//! - The test #699 wrote on the tilted block (`tilted_block()`, corner
//!   (30, 0, 15), r = 2, volume 4311.32576 by independent booleans) then
//!   expects the body rather than the decline —
//!   `a_fillet_on_the_three_edges_of_a_corner_where_no_two_edges_are_square_closes_it_with_a_piece_of_sphere_one_radius_from_its_three_faces`,
//!   the decline it held turned into a body held through
//!   `closes_with_a_sphere` as the slanted prism's corner is, to the volume
//!   `sphere_by_booleans` gives
//! - What must not break, in the issue's words: "Every sphere, ball and
//!   torus test (#553, #576 and the turned parts); the corner tests of
//!   #699." — no test: each is held by its own test, left as it was, but for
//!   the tilted block's fillet in this file, the one test of #699 that #800
//!   turns from a decline into a body
//!
//! Closes #803.
//! - The fillet on `[top-front, top-back, top-right]` of a block 6 wide at
//!   r = 3 leaves an exact, closed, uncrossed, drawn body of volume
//!   `V − 2r²(1 − π/4)(L − r) − 2r³/3`, whichever edge the list names first.
//!   The full round's face and the right edge's round meet along two
//!   quarter-ellipses from the right face's top corners to the crown. The
//!   uprights at the right end stay sharp, and no face is laid at the
//!   corners.
//!   `a_fillet_on_a_full_round_pair_and_an_edge_meeting_it_at_a_corner_is_declined_as_unsupported_in_either_order`
//!   becomes this body test, in each of the six orders the list can name the
//!   three edges in, the volume held by the booleans as well. Each arc is the
//!   quarter of the ellipse its plane cuts from both cylinders, from the end
//!   of its long axis on the right face to the end of its short one at the
//!   crown: held as each round drawn on its own cylinder and on its own side
//!   of the two planes, both drawn up to the right face's top corners and to
//!   the crown, and met by two edges between them —
//!   `a_fillet_on_a_full_round_pair_and_an_edge_meeting_it_at_a_corner_mitres_that_edge_s_round_against_the_full_round_whatever_the_order`
//! - The four top edges of that block, filleted at 3: the full round mitred
//!   at both ends, in any order the list names them —
//!   `a_fillet_on_the_four_top_edges_of_a_block_6_wide_mitres_its_full_round_against_both_end_edges_whatever_the_order`
//! - A value asked above what fits, 4 on the block 6 wide, is reduced to the
//!   full round's radius, 3, and the end edge is laid at that same value; the
//!   step reports both —
//!   `a_fillet_on_a_full_round_pair_and_an_edge_meeting_it_at_a_corner_asked_more_than_fits_lays_both_at_the_full_round_s_radius_and_reports_both`,
//!   in every order, the end edge alone taking 4 there; green as soon as
//!   it was written, answered by the change of the first criterion, the
//!   value the pair bounds the step to being the full round's radius
//!   exactly. A value asked a hair under 3, within what the kernel takes for
//!   set-backs that meet, lays the full round at its own radius and the end
//!   edge at that same radius, so that the two meet at the crown with no
//!   sliver between them —
//!   `a_fillet_on_a_full_round_pair_and_an_edge_meeting_it_at_a_corner_asked_a_hair_under_the_full_round_s_radius_lays_both_at_that_radius_meeting_at_the_crown`,
//!   in every order, asked 1e-6 and 1e-7 under, the strip of top left two
//!   of those, under four of the kernel's hairs on this block and over one
//!   or not, as #698 asks its full round; the step reports the value asked,
//!   and takes it or the full round's radius, a hair apart
//! - An end that is not square to the pair: a plate whose end face is cut at
//!   an angle, seen from above. The same mitre, exact, closed, uncrossed and
//!   drawn, its volume held by arithmetic or by the booleans, in either
//!   order —
//!   `a_fillet_on_a_full_round_pair_and_an_end_edge_cut_at_an_angle_seen_from_above_mitres_the_end_edge_s_round_against_the_full_round_in_either_order`,
//!   in every order, on a plate whose front is 30 long and its back 27, the
//!   end wall meeting them at 63.4 and 116.6 degrees; its volume held both
//!   ways, the two agreeing to the last digits, and the rest as on the square
//!   block, through the same check, the two arcs on the planes that halve the
//!   angle the slant makes. Green as soon as it was written, answered by the
//!   changes the first three criteria made: the guard of the first lets the
//!   slanted end edge in as it lets in the square one, its faces the top and
//!   the end wall at convex corners
//! - The faces are numbered as #698 and #699 number them: the full round
//!   answers to the earlier edge of the pair, the end edge to its own rank in
//!   the list, and nothing is numbered at the corner. Read as #698 lays a
//!   full round: one face answering to both its edges' numbers, a ray meeting
//!   it naming the earlier, and the step ending past every edge's number —
//!   held by the tests of the first two criteria, in every order they name
//!   the edges in —
//!   `a_fillet_on_a_full_round_pair_and_an_edge_meeting_it_at_a_corner_mitres_that_edge_s_round_against_the_full_round_whatever_the_order`,
//!   `a_fillet_on_the_four_top_edges_of_a_block_6_wide_mitres_its_full_round_against_both_end_edges_whatever_the_order`
//! - What must not break, in the issue's words: "#698's full round and round
//!   bottom, and #699's corner tests, except the decline above, which becomes
//!   a body test." — no test: each is held by its own test, left as it was,
//!   but for the decline the first criterion turns into a body
//! - And: "The chamfer on the same lists, which main already lays: its flats
//!   meet in a ridge, mitred with the end edge's flat (3321 on the block 30 ×
//!   6 × 20 at 3)." No test held it before this branch, which pins it, green
//!   before the fillet is built —
//!   `a_chamfer_on_a_full_round_pair_s_edges_and_an_edge_meeting_them_at_a_corner_mitres_their_ridge_against_that_edge_s_flat_whatever_the_order`
//! - What stays declined, so that the guard letting the end edge in lets in
//!   nothing else: the full round of the top's front and back edges with the
//!   front's right edge, the upright at its end, and with the top's right
//!   edge and both uprights at that end, until #837; a pocket 6 wide whose
//!   floor's front and back edges are filled at 3 in a round bottom, with its
//!   floor's edge on its end wall, until #838; and the four top edges of a
//!   block 6 by 6, two full rounds meeting at its corners. Each is declined
//!   `Unsupported` in every order the list can name its edges in, the pair
//!   alone laid. Green before this branch and after, a pin of the guard rather
//!   than a red test —
//!   `a_fillet_on_a_full_round_meeting_an_upright_or_another_full_round_and_on_a_round_bottom_meeting_the_edge_at_its_end_is_declined_as_unsupported_whatever_the_order`;
//!   and, until #843, a full round ending on a pointed end, a plate 6 wide
//!   whose right end comes to a point at (33, 3) seen from above, filleted at
//!   3 on the top's front and back edges and the two edges of the point, each
//!   of which meets one edge of the pair alone, the pair alone laid as a full
//!   round. No order of the four mitres a full round against the point: the
//!   step is declined `Unsupported` where the value it takes is the full
//!   round's, and elsewhere stops short of it, 6 − 3√2, the top kept, and no
//!   pair laid together. Which of the two an order gets is the order's doing,
//!   a reduction this branch does not touch —
//!   `a_fillet_on_a_full_round_pair_and_the_two_edges_of_a_pointed_end_each_meeting_one_of_the_pair_mitres_no_full_round_against_them_whatever_the_order`.
//!   A bevelled end, its corners cut off upright, does not reach that far:
//!   with the end edge or a bevel edge in the list, the step takes a value
//!   short of the full round, where the rounds would meet through an end
//!   face, and no pair is laid together
//! - What stays declined beside, though no guard of this branch holds it:
//!   the chamfer on the full round's edges, the end edge and both uprights,
//!   `Unsupported` in every order until #837, where on the other lists above
//!   the chamfer is built. Its two flats are laid edge by edge, never as one
//!   pair, so the decline comes from elsewhere in the step —
//!   `a_chamfer_on_a_full_round_pair_s_edges_the_edge_at_their_end_and_both_uprights_there_is_declined_as_unsupported_whatever_the_order`
//!
//! The volumes are arithmetic, measured from the corner along its three
//! edges: a point stands `(a, b, c)` from it, `a` along the edge whose length
//! is `l1`, `b` along `l2`, `c` along `l3`, on a block of volume `V`. At a
//! square corner a chamfer of the first edge alone takes away the prism
//! `b + c < d` along all of it, `d²·l1/2`; a fillet of radius `r`, the strip
//! between the faces and the round, `r²(1 − π/4)·l1`.
//!
//! Two chamfers mitred, the first two edges: the prisms `b + c < d` and
//! `a + c < d` overlap at the corner where both cut, whose section at a depth
//! `c` below their shared face is a square of side `d − c`; it holds the
//! integral from 0 to `d` of `(d − c)²`, `d³/3`, counted twice. The body keeps
//! `V − d²(l1 + l2)/2 + d³/3`.
//!
//! Two fillets mitred: at a depth `c` below the shared face, each round takes
//! the width `w = r − √(r² − (r − c)²)` off its edge's other face; the overlap
//! is a square of side `w`, holding, with `u = r − c`, the integral from 0 to
//! `r` of `(r − √(r² − u²))²`, `2r³ − 2r·πr²/4 − r³/3 = r³(5/3 − π/2)`. The
//! body keeps `V − r²(1 − π/4)(l1 + l2) + r³(5/3 − π/2)`.
//!
//! Three chamfers: outside the cube of side `d` at the corner the three prisms
//! do not overlap — a point of the first's past `a = d` is beyond the other
//! two — each taking `d²(l − d)/2`. Inside it every point goes but those with
//! `a + b + c ≥ 2d`, a tetrahedron of `d³/6` at the cube's far corner, which
//! the triangle through the three crossings bounds: if `a`, `b` and `c` are at
//! most `d`, a sum of `2d` or more leaves each pair `d` or more. The body
//! keeps `V − d²(l1 + l2 + l3)/2 + 3d³/2 − (d³ − d³/6)`, that is
//! `V − d²(l1 + l2 + l3)/2 + 2d³/3`.
//!
//! Three fillets: outside the cube of side `r` the three rounds do not
//! overlap, each taking `r²(1 − π/4)(l − r)`. Inside it every point goes but
//! the eighth of the ball of radius `r` about the cube's far corner, `πr³/6`,
//! which lies inside all three rounds. The body keeps
//! `V − r²(1 − π/4)(l1 + l2 + l3) + 3r³(1 − π/4) − r³(1 − π/6)`, that is
//! `V − r²(1 − π/4)(l1 + l2 + l3) + r³(2 − 7π/12)`: 5888.63 at 3 on the block
//! 30 by 10 by 20. Held another way: the three rounds' prisms alone, cut from
//! the block by the booleans, leave in that cube what lies inside all three
//! cylinders, the eighth of a Steinmetz tricylinder, `(2 − √2)r³`; the sphere
//! leaves of it the eighth of the ball, `πr³/6`, and the two counts agree to
//! the last digit with that body less `(2 − √2 − π/6)r³`. The booleans
//! decline the piece itself, cut as a box less a ball, its sphere tangent to
//! the rounds along their ends: no body of theirs holds every face of it.
//!
//! The four top edges of a block, two of three at each of its four corners:
//! each pair overlaps once, so the chamfered body keeps
//! `V − d²(2L + 2B)/2 + 4d³/3`, and the filleted one
//! `V − r²(1 − π/4)(2L + 2B) + 4r³(5/3 − π/2)`, `L` and `B` its length and
//! breadth: 5676 for the chamfer at 3 on the block 30 by 10 by 20, and
//! 5855.84 for the fillet at 3.
//!
//! The corner that is not square is the end of a prism 30 long whose section
//! is 16 wide at its bottom, 10 at its top and 10 high, its right side
//! slanting in: the end, the top and that side meet at (30, 12, 10). The
//! top's edge on the end, A, and the upright where the side meets the end, C,
//! are square; the top's edge on the side, B, meets at the side's slant, an
//! angle θ with `cos θ = −4/√116`; write `k = sin θ = 10/√116`. Measured
//! from the corner, `x′` in from the end, `w` down from the top and `u` in
//! from the side along Y, the side is `u = 0` and stands `ku` away. Alone, a
//! flat at `d` takes: on A, which runs across the end between two sides that
//! lean out as they go down, a length `10 + 0.6w` at a depth `w`,
//! `5d² + 0.1d³`; on B the triangle `u/d + w/(kd) < 1` along its 30,
//! `15kd²`; on C the square cut `d²/2` along its `10/k`, `5d²/k`. Two flats
//! overlap where both cut: A and B by `kd³(1/2 − k/6)`, `x′ < d − w` and
//! `u < d − w/k` at a depth `w`; B and C by the same; A and C by `d³/(3k)`,
//! `w < d − x′` and `u < (d − x′)/k` at `x′`. A round at `r` alone takes, on
//! A, `10r²(1 − π/4) + 0.6r³(5/6 − π/4)`; on B, `30r²(cot(θ/2) − (π − θ)/2)`;
//! on C, `(10/k)r²(1 − π/4)`; rounds on A and C overlap by
//! `r³(5/3 − π/2)/k`, the square corner's integral stretched by `1/k` across
//! the slant. At 2, the prism of 3900 keeps 3826.06 with flats on A and B,
//! 3860.53 on A and C, 3825.32 on B and C, and 3882.77 with rounds on A and
//! C. Where a round on B meets one on A or C, the overlap is an integral of
//! two arcs at that angle with no short closed form: there the volume
//! promised is the prism's with the two cuts' prisms, each raised whole along
//! its edge's line, taken away by the booleans — the fixture the test also
//! holds every face of the step against.
//!
//! All three edges of the slanted corner cut, in the same coordinates, the
//! flats are `x′ + w < d`, `x′ + s < d` and `w + s < kd`, and they cross on
//! the three faces at `(d, 0, kd)`, `(0, d, d)` and `(d, kd, 0)`: the
//! triangle's plane is `(1 − k/2)x′ + (w + s)/2 = d`. A volume in these
//! coordinates is `k` times the volume it stands for. The three flats overlap
//! in `kd³/2 − k²d³/4` — at `x′`, a square of side `d − x′` cut by
//! `w + s < kd` — and the triangle takes besides the tetrahedron between it
//! and the point the three flats meet at, `(d(1 − k/2), kd/2, kd/2)`, of
//! `kd³(1 − k/2)/6`; at `k = 1` the two are the square corner's `d³/4` and
//! `d³/12`. With the overlaps of pairs above, the prism keeps
//! `V − 5d² − 0.1d³ − 15kd² − 5d²/k + (k + 1/k)d³/3`: 3807.30 at 2.
//!
//! Its three rounds meet the sphere of radius `r` centred `(r, r, r)` in
//! these coordinates, at `(28, 12 − r·cot(θ/2), 8)` for `r = 2`; each ends
//! on the plane through that centre square to its edge, and the three planes
//! bound at the corner a cell whose edges run from the centre the three ways
//! the faces face. Each round alone, less its part on the corner's side of
//! its end section, is laid; the cell is taken away and the ball's sector in
//! it given back. On A's side of its section the round runs on to the side,
//! leaning out: `r·cot(θ/2)·S + 0.4r³(5/6 − π/4)`, with `S = r²(1 − π/4)`,
//! the first moment of the round's section about the top adding the lean; on
//! C's the same, measured from the top to the section along the slant; on
//! B's, `r` along its section `r²(cot(θ/2) − (π − θ)/2)`. The cell is a prism
//! along X, `r` long, whose section holds `r²(1 − 0.6k + 0.232k³)/k`; the
//! three ways the faces face span a solid angle of `π − θ`, and the sector
//! holds a third of `r³` times that. The prism keeps 3873.16 at 2.
//!
//! Both are held a second way, by the booleans, and agree to the last digits:
//! the flats' three prisms and the slab beyond the triangle's plane cut away;
//! the rounds' three prisms cut away, then what is left inside the cell, and
//! the sector added back. On the square corner the same computation gives the
//! formulas' 5748 and 5888.63 exactly. The corner where no two edges are
//! square, the block's top cut by the plane rising 0.1 along X and 0.2 along
//! Y from 12, is measured that way only: 4245.76 with the flats at 2, 4311.33
//! with the rounds.
//!
//! Several corners closed in one step on the block 30 by 10 by 20, each at
//! least the value twice from any other, are each the corner above: every
//! edge's prism along its whole length, and at each corner closed the cube of
//! side the value given back as the corner alone gives it, `2d³/3` among
//! flats and `r³(2 − 7π/12)` among rounds. With `L` the lengths of the edges
//! named summed and `n` the corners closed, the block keeps
//! `V − d²L/2 + 2nd³/3` and `V − r²(1 − π/4)L + nr³(2 − 7π/12)`, where every
//! other end of an edge named meets no other edge named. For an upright and
//! the four edges at its two ends, `L` is 100 and `n` 2: 5586 at 3 with
//! flats, 5815.90 with rounds. For the twelve edges, `L` is 240 and `n` 8:
//! 5064 and 5572.62, the second the box with every edge and corner rounded,
//! `(a − 2r)(b − 2r)(c − 2r)` and its six slabs, twelve quarter-cylinders and
//! eight eighths of a ball, to the last digit.
//!
//! The slanted prism's top edge on its end ends at two corners that are not
//! square: the right side's, `k = 10/√116`, and the left side's, leaning in 1
//! in 5, `k′ = 10/√104`. Its edges on both, chamfered at `d`, close both
//! corners each as above, far enough apart not to meet: the end's top edge
//! alone takes `5d² + 0.1d³`, its two ends leaning out 0.4 and 0.2 a unit of
//! depth; the two top edges on the sides `15kd²` and `15k′d²`; the two
//! slanted uprights `5d²/k` and `5d²/k′`; and each corner gives back
//! `(k + 1/k)d³/3` and `(k′ + 1/k′)d³/3`. The prism keeps 3733.40 at 2, which
//! the booleans give too; the rounds, by the booleans corner by corner,
//! 3849.57.
//!
//! The block whose top's front and right edges and front's right edge are
//! named, then joined to a block 10 by 10 by 20 on its left, flush with its
//! front, top, floor and back, is a block 40 long holding 8000: the top's
//! front edge 40 long, the corner's other two as they were. Mitred, two
//! edges keep 8000 less the mitre's loss on their lengths, 7784 and 7906.02
//! for the top's two edges, 7739 and 7886.70 for the top's front edge and the
//! front's right edge; closed, the three keep 7703 and 7869.32.
//!
//! The block whose floor's front edge was rounded at 3 by an earlier step,
//! its top's front and right edges then mitred at 3, keeps the block less that
//! round's `9(1 − π/4)·30` and less the mitre's own loss, the two far apart:
//! 5771.06 with flats, 5867.39 with rounds, which the booleans give to the
//! last digit.
//!
//! The rounds at the corner where no two edges are square, a broken step
//! until #800, close it with their piece of sphere, held through
//! `closes_with_a_sphere` as at the slanted prism's corner: a body of
//! 4311.33 at 2, the sphere centred at (28, 2, 13.15), 2 in from the right
//! wall, the front and the tilted top, touching each at the foot of the
//! centre on it; each round ending on the plane through that centre square
//! to its edge, along an arc between two of those feet; the three ways the
//! faces face spanning 1.4536 of solid angle for the piece of sphere, whose
//! sector the volume gives back. Its triangles are held at the tolerance it
//! is drawn at, which is not its box's: a cut keeps the larger reach of the
//! part and the tool, and the slab its top is cut off by reaches 90.6, three
//! times the block's 30.
//!
//! At a corner that is not square, the line where the two cuts meet need not
//! run down to the third edge: it can leave by the face the third edge shares
//! with one of the cut ones, and run on along that face. The third edge ends
//! where the cut set back farthest along it crosses it, that cut's set-back
//! over the sine of the angle its edge makes with the third; at a square
//! corner both come to one value.
//!
//! A full round mitred against the edge at its end, on a block `2r` wide: the
//! top's front and back edges rounded at `r` meet across the top in one
//! half-cylinder, which takes `2r² − πr²/2`, that is `2r²(1 − π/4)`, of the
//! section along the length. The end edge's round, raised from its wedge as
//! the body given reads it, is a quarter-cylinder of `r` across the whole
//! width, its axis crossing the full round's one radius in from the end and
//! one radius under the top. Short of the box `r` long, `2r` wide and `r`
//! deep at the end, the full round alone cuts; in that box, of `2r³`, the
//! body keeps what lies inside both cylinders, a quarter of the Steinmetz
//! bicylinder of radius `r`, `4r³/3`. The block keeps
//! `V − 2r²(1 − π/4)(L − r) − 2r³/3`, and with the edge at each end so
//! mitred `V − 2r²(1 − π/4)(L − 2r) − 4r³/3`: 3477.70 and 3471.29 on the
//! block 30 by 6 by 20 at 3. The first is the limit at a width of `2r` of the
//! two mitres a wider block keeps, `V − r²(1 − π/4)(2L + w) + 2r³(5/3 − π/2)`,
//! 3483.68 at 6.01 wide. The booleans give it too, the full round's prism and
//! the end edge's, each raised whole along its line, taken away from the
//! block. Two cylinders of one radius whose axes cross meet along two
//! ellipses, in the two planes through the line square to both axes that
//! halve the angles between them; on the body each is a quarter, from a top
//! corner of the end face, the end of its long axis, up to the crown, the end
//! of its short one, where the two planes cross and the two cylinders touch.
//!
//! An end wall standing at an angle seen from above, meeting the long edges
//! at θ, on a plate whose outline holds the area `A`: the end edge's round's
//! axis, a radius in from the wall and under the top, crosses the full
//! round's at `X` along it. Short of the plane through that crossing
//! parallel to the wall the full round alone cuts, keeping over its axis its
//! half-disc, `πr²/2`, along a length that runs to `X` at the middle of the
//! top and a little more or less either side of it, by as much one way as the
//! other, so that `πr²/2 · X` is what it keeps. Past that plane the body keeps
//! what lies inside both cylinders, the same quarter bicylinder as at a square
//! end, measured by the distance out square to the wall and the distance
//! across the top; an area of the top holds `1/sin θ` times what those two
//! measure, so the quarter bicylinder holds `4r³/(3 sin θ)`. The plate keeps
//! `(H − r)A + πr²X/2 + 4r³/(3 sin θ)`, and at θ = 90° that is the square
//! end's count, 3477.70. On the plate whose front is 30 long and back 27,
//! `A` is 171, `sin θ` is `2/√5` and `X` is `28.5 − 1.5√5`, 25.146: it keeps
//! 3302.74, which the booleans give to the last digits. The two cylinders
//! still meet on two planes through the vertical at the crossing, halving the
//! angles between the two axes, each arc a quarter of an ellipse from a top
//! corner of the end wall up to the crown.
//!
//! The chamfer on the same three edges at `d = r`: the two flats meet in a
//! ridge along the middle of the top, taking `d²` of the section along the
//! length. In the box at the end the body keeps what lies under all three
//! flats, a height over the box's floor at most the least of the distances
//! in from the front, the back and the end, `2d³/3` of the box's `2d³`. The
//! block keeps `V − d²(L − d) − 4d³/3`, 3321 on the block 30 by 6 by 20 at 3.

use std::collections::BTreeSet;
use std::f64::consts::PI;

use cao_solid::profile::{Contour, Frame, Profile, Run};
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

/// The numbers a raised block's faces answer to: its floor nought, its top
/// one, and its sides from the front round to the left, two to five.
const FLOOR: usize = 0;
const TOP: usize = 1;
const FRONT: usize = 2;
const RIGHT: usize = 3;
const BACK: usize = 4;
const LEFT: usize = 5;

/// The block the corners are held on: 30 along X, 10 across, 20 high — three
/// lengths apart, so that each edge of a corner is told by its length.
const LENGTH: f64 = 30.0;
const BREADTH: f64 = 10.0;
const HEIGHT: f64 = 20.0;
const BLOCK_VOLUME: f64 = LENGTH * BREADTH * HEIGHT;

/// The two edges a step names among a corner's three, and the third it leaves,
/// as indices into [`Corner`]'s edges: each of the three pairs, in either
/// order.
const TWO_OF_THREE: [[usize; 3]; 6] = [
    [0, 1, 2],
    [1, 0, 2],
    [0, 2, 1],
    [2, 0, 1],
    [1, 2, 0],
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

/// A corner where three edges of a block meet, square: where it stands, and
/// for each of its three edges the way it runs from the corner, how long it
/// is, the face across from it — the one of the corner's three faces it does
/// not lie on —, the face it ends on at its far end, and what it is called.
struct Corner {
    at: DVec3,
    along: [DVec3; 3],
    length: [f64; 3],
    across: [usize; 3],
    far: [usize; 3],
    called: [&'static str; 3],
}

/// The block's top front right corner: the top's front edge running 30 to the
/// left wall, the top's right edge 10 to the back, and the front's right edge
/// 20 down to the floor.
fn top_front_right() -> Corner {
    Corner {
        at: DVec3::new(LENGTH, 0.0, HEIGHT),
        along: [-DVec3::X, DVec3::Y, -DVec3::Z],
        length: [LENGTH, BREADTH, HEIGHT],
        across: [RIGHT, FRONT, TOP],
        far: [LEFT, BACK, FLOOR],
        called: [
            "the top's front edge",
            "the top's right edge",
            "the front's right edge",
        ],
    }
}

impl Corner {
    /// The point `by` from the corner along the edges `order` names: the first
    /// length along the first, the second along the second, the third along
    /// the third.
    fn point(&self, order: [usize; 3], by: [f64; 3]) -> DVec3 {
        order
            .into_iter()
            .zip(by)
            .fold(self.at, |point, (edge, length)| {
                point + self.along[edge] * length
            })
    }

    /// How far a point stands from the corner along the edge `edge`.
    fn along_edge(&self, point: DVec3, edge: usize) -> f64 {
        (point - self.at).dot(self.along[edge])
    }

    /// The round a fillet of `radius` lays on the edge `edge`, whole: its axis
    /// the radius in from the edge's two faces — the one it shares with the
    /// edge `beside` and the one across from `beside` — and running from the
    /// corner's plane to the edge's far end, turning from facing the way the
    /// shared face does to facing the way the other does.
    fn round(&self, edge: usize, [beside, shared]: [usize; 2], radius: f64) -> Round {
        Round {
            start: self.at + (self.along[beside] + self.along[shared]) * radius,
            along: self.along[edge],
            length: self.length[edge],
            radius,
            facing: [-self.along[shared], -self.along[beside]],
        }
    }

    /// The name of the corner's edge `edge` on the body: the edge between the
    /// two faces across from the corner's two other edges.
    fn name(&self, body: &Body, edge: usize) -> EdgeName {
        let [one, other] = [(edge + 1) % 3, (edge + 2) % 3];
        edge_between(body, self.across[one], self.across[other])
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
    body.edge_names()
        .into_iter()
        .filter(|name| lies_between(name, first, second))
        .collect()
}

/// Whether a name is that of an edge between the faces numbered `first` and
/// `second`, whichever it lists first.
fn lies_between(name: &EdgeName, first: usize, second: usize) -> bool {
    let sides = [vec![first as u32], vec![second as u32]];
    let [one, other] = sides.clone();
    name.between == sides || name.between == [other, one]
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

/// Holds that a face's corners are the points expected: each of them a
/// corner, and no corner anywhere else.
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
            plane
                .corners
                .iter()
                .any(|corner| corner.distance(*point) < PLACE),
            "{what}: no corner at {point}, among {:?}",
            plane.corners
        );
    }
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

/// The sides of the faces the face numbered `face` meets, one for each edge
/// it has, in order.
fn sides_met(body: &Body, face: usize) -> Vec<Vec<u32>> {
    sides_met_by(body, &[face])
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
/// [`sides_met`] lists them.
fn sides(faces: &[usize]) -> Vec<Vec<u32>> {
    let mut sorted: Vec<Vec<u32>> = faces.iter().map(|&face| vec![face as u32]).collect();
    sorted.sort();
    sorted
}

/// The sides a face is expected to meet, one edge each, as [`sides`] lists
/// them, each side every number the face across it answers to: a full round
/// answers to both its edges'.
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

/// What a block keeps once two of a square corner's edges, `lengths` long,
/// are chamfered at `distance` and mitred: each flat's prism taken whole, and
/// the pyramid where both cut given back once, as the module's documentation
/// derives it.
fn mitred_by_flats(lengths: [f64; 2], distance: f64) -> f64 {
    let [first, second] = lengths;
    BLOCK_VOLUME - distance * distance * (first + second) / 2.0 + distance.powi(3) / 3.0
}

#[test]
fn a_chamfer_on_two_of_a_corner_s_three_edges_mitres_their_flats_and_leaves_the_third_edge_sharp_one_distance_short()
 {
    let distance = 3.0;
    let block = block([LENGTH, BREADTH, HEIGHT]);
    let corner = top_front_right();
    let steps_end = block.faces_end();

    for [first, second, third] in TWO_OF_THREE {
        let what = format!(
            "the chamfer on {} then {}, {} left",
            corner.called[first], corner.called[second], corner.called[third]
        );
        let names = [corner.name(&block, first), corner.name(&block, second)];

        let chamfered = block
            .chamfer_edges(&names, distance)
            .unwrap_or_else(|declined| panic!("{what}: declined, {declined:?}"));

        taken_as_asked(chamfered.asked, chamfered.taken, distance, &what);
        let body = &chamfered.body;
        let [long_first, long_second, long_third] =
            [first, second, third].map(|edge| corner.length[edge]);
        let promised = mitred_by_flats([long_first, long_second], distance);
        sound_drawn_within(body, promised, EXACT * promised, &what);
        let [flat_first, flat_second] = [steps_end, steps_end + 1];
        assert_eq!(
            faces_made(&block, body),
            vec![flat_first, flat_second],
            "{what}: one flat for each edge, numbered in the order the step names them, and no \
             face laid at the corner: {:?}",
            numbers(body)
        );
        assert!(
            numbers(&block).is_subset(&numbers(body)),
            "{what}: a face of the block went: {:?}",
            numbers(body)
        );
        let at = |by: [f64; 3]| corner.point([first, second, third], by);
        let d = distance;
        let [across_first, across_second, shared] =
            [first, second, third].map(|edge| corner.across[edge]);
        let [far_first, far_second, far_third] =
            [first, second, third].map(|edge| corner.far[edge]);
        let crossing = at([d, d, 0.0]);
        let short_of_the_corner = at([0.0, 0.0, d]);

        corners_are(
            body,
            flat_first,
            &[
                at([long_first, d, 0.0]),
                at([long_first, 0.0, d]),
                short_of_the_corner,
                crossing,
            ],
            &format!(
                "{what}: the flat of the first edge, from its far end to the mitre, which runs \
                 from where the two flats cross the shared face down to the third edge"
            ),
        );
        faces_toward(
            body,
            flat_first,
            -(corner.along[second] + corner.along[third]).normalize(),
            &format!("{what}: the flat of the first edge"),
        );
        assert_eq!(
            sides_met(body, flat_first),
            sides(&[shared, across_second, far_first, flat_second]),
            "{what}: the flat of the first edge meets its edge's two faces along its sides, the \
             face its edge ends on, and the other flat along the mitre, by one edge each"
        );
        corners_are(
            body,
            flat_second,
            &[
                at([d, long_second, 0.0]),
                at([0.0, long_second, d]),
                short_of_the_corner,
                crossing,
            ],
            &format!("{what}: the flat of the second edge, from its far end to the mitre"),
        );
        faces_toward(
            body,
            flat_second,
            -(corner.along[first] + corner.along[third]).normalize(),
            &format!("{what}: the flat of the second edge"),
        );
        assert_eq!(
            sides_met(body, flat_second),
            sides(&[shared, across_first, far_second, flat_first]),
            "{what}: the flat of the second edge meets its edge's two faces along its sides, the \
             face its edge ends on, and the other flat along the mitre, by one edge each"
        );
        corners_are(
            body,
            shared,
            &[
                crossing,
                at([long_first, d, 0.0]),
                at([long_first, long_second, 0.0]),
                at([d, long_second, 0.0]),
            ],
            &format!(
                "{what}: the face the two edges share, set back by the distance from each, its \
                 corner at the crossing"
            ),
        );
        assert_eq!(
            sides_met(body, shared),
            sides(&[flat_first, flat_second, far_first, far_second]),
            "{what}: the face the two edges share meets the two flats, and no longer the third \
             edge's faces"
        );
        let third_s_far_end = at([0.0, 0.0, long_third]);
        for (face, other, [beside, beyond], far, flat, which) in [
            (
                across_second,
                across_first,
                [at([long_first, 0.0, d]), at([long_first, 0.0, long_third])],
                far_first,
                flat_first,
                "the face the first edge and the third share",
            ),
            (
                across_first,
                across_second,
                [
                    at([0.0, long_second, d]),
                    at([0.0, long_second, long_third]),
                ],
                far_second,
                flat_second,
                "the face the second edge and the third share",
            ),
        ] {
            corners_are(
                body,
                face,
                &[short_of_the_corner, beside, beyond, third_s_far_end],
                &format!(
                    "{what}: {which}, set back by the distance from the flat, the third edge \
                     along it ending one distance short of the corner"
                ),
            );
            assert_eq!(
                sides_met(body, face),
                sides(&[flat, far, far_third, other]),
                "{what}: {which} meets its flat, the faces the edges end on, and the third \
                 edge's other face, sharp"
            );
        }
        assert_eq!(
            edges_between(body, across_first, across_second).len(),
            1,
            "{what}: {} is still one sharp edge, from where it ends one distance short of the \
             corner to its far end",
            corner.called[third]
        );
    }
}

/// What a block keeps once two of a square corner's edges, `lengths` long,
/// are rounded at `radius` and mitred: each round's strip taken whole, and the
/// overlap where both take given back once, as the module's documentation
/// derives it.
fn mitred_by_rounds(lengths: [f64; 2], radius: f64) -> f64 {
    let [first, second] = lengths;
    BLOCK_VOLUME - radius * radius * (1.0 - PI / 4.0) * (first + second)
        + radius.powi(3) * (5.0 / 3.0 - PI / 2.0)
}

/// What a block keeps once the three edges of a square corner, `lengths`
/// long, are chamfered at `distance` and closed by a triangle: each flat's
/// prism taken whole, less what they share inside the cube of side
/// `distance` at the corner, as the module's documentation derives it.
fn closed_by_a_triangle(lengths: [f64; 3], distance: f64) -> f64 {
    let along: f64 = lengths.iter().sum();
    BLOCK_VOLUME - distance * distance * along / 2.0 + 2.0 * distance.powi(3) / 3.0
}

/// What a block keeps once the three edges of a square corner, `lengths`
/// long, are rounded at `radius` and closed by a piece of sphere: each
/// round's strip taken whole, less what they share inside the cube of side
/// `radius` at the corner but the eighth of the ball, as the module's
/// documentation derives it.
fn closed_by_a_sphere(lengths: [f64; 3], radius: f64) -> f64 {
    let along: f64 = lengths.iter().sum();
    BLOCK_VOLUME - radius * radius * (1.0 - PI / 4.0) * along
        + radius.powi(3) * (2.0 - 7.0 * PI / 12.0)
}

/// Every order three things can be named in.
const ORDERS_OF_THREE: [[usize; 3]; 6] = [
    [0, 1, 2],
    [0, 2, 1],
    [1, 0, 2],
    [1, 2, 0],
    [2, 0, 1],
    [2, 1, 0],
];

/// What the block keeps once its four top edges are chamfered at `distance`,
/// mitred at its four top corners: each flat's prism taken whole, and at each
/// corner the pyramid where two cut given back once, as the module's
/// documentation derives it.
fn ringed_by_flats(distance: f64) -> f64 {
    BLOCK_VOLUME - distance * distance * (2.0 * LENGTH + 2.0 * BREADTH) / 2.0
        + 4.0 * distance.powi(3) / 3.0
}

/// What the block keeps once its four top edges are rounded at `radius`,
/// mitred at its four top corners, as [`ringed_by_flats`] counts it.
fn ringed_by_rounds(radius: f64) -> f64 {
    BLOCK_VOLUME - radius * radius * (1.0 - PI / 4.0) * (2.0 * LENGTH + 2.0 * BREADTH)
        + 4.0 * radius.powi(3) * (5.0 / 3.0 - PI / 2.0)
}

/// An edge of the block's top: the wall it runs along and the way that wall
/// faces, the corner it starts at, the way it runs from it — round the top
/// anticlockwise, seen from above — and how long it is.
struct TopEdge {
    wall: usize,
    facing: DVec3,
    start: DVec3,
    along: DVec3,
    length: f64,
}

/// The block's four top edges, round the top from the front: the front, the
/// right, the back and the left.
fn top_edges() -> [TopEdge; 4] {
    [
        TopEdge {
            wall: FRONT,
            facing: -DVec3::Y,
            start: DVec3::new(0.0, 0.0, HEIGHT),
            along: DVec3::X,
            length: LENGTH,
        },
        TopEdge {
            wall: RIGHT,
            facing: DVec3::X,
            start: DVec3::new(LENGTH, 0.0, HEIGHT),
            along: DVec3::Y,
            length: BREADTH,
        },
        TopEdge {
            wall: BACK,
            facing: DVec3::Y,
            start: DVec3::new(LENGTH, BREADTH, HEIGHT),
            along: -DVec3::X,
            length: LENGTH,
        },
        TopEdge {
            wall: LEFT,
            facing: -DVec3::X,
            start: DVec3::new(0.0, BREADTH, HEIGHT),
            along: -DVec3::Y,
            length: BREADTH,
        },
    ]
}

impl TopEdge {
    /// The corner it ends at.
    fn end(&self) -> DVec3 {
        self.start + self.along * self.length
    }

    /// The edge's name on the part.
    fn name(&self, part: &Body) -> EdgeName {
        edge_between(part, TOP, self.wall)
    }

    /// Where a cut set back by `value` on the top from this edge crosses the
    /// cut of the edge before it, at its start, and of the edge after it, at
    /// its end.
    fn crossings(&self, value: f64) -> [DVec3; 2] {
        let into = -self.facing * value;
        [
            self.start + self.along * value + into,
            self.end() - self.along * value + into,
        ]
    }

    /// The flat a chamfer of `distance` lays on the edge, mitred at both its
    /// ends: from the two crossings on the top down to the two uprights, one
    /// distance below the top, and the way it faces.
    fn flat(&self, distance: f64) -> ([DVec3; 4], DVec3) {
        let [at_start, at_end] = self.crossings(distance);
        let down = DVec3::Z * distance;
        (
            [at_start, at_end, self.end() - down, self.start - down],
            (self.facing + DVec3::Z).normalize(),
        )
    }

    /// The round a fillet of `radius` lays on the edge, whole: its axis the
    /// radius in from the top and from the wall, along the whole edge.
    fn round(&self, radius: f64) -> Round {
        Round {
            start: self.start - (self.facing + DVec3::Z) * radius,
            along: self.along,
            length: self.length,
            radius,
            facing: [DVec3::Z, self.facing],
        }
    }

    /// Whether a point stands on this edge's side of the plane halfway between
    /// it and the edge before it, at its start, and of the plane halfway
    /// between it and the edge after it, at its end: no nearer either corner
    /// along this edge than in from this edge's wall.
    fn between_its_mitres(&self, point: DVec3) -> bool {
        let inwards = |corner: DVec3| (point - corner).dot(-self.facing);
        (point - self.start).dot(self.along) >= inwards(self.start) - PLACE
            && (self.end() - point).dot(self.along) >= inwards(self.end()) - PLACE
    }
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

    /// The part of the round from `from` along its axis to `to`.
    fn part(&self, from: f64, to: f64) -> Round {
        Round {
            start: self.start + self.along * from,
            length: to - from,
            ..*self
        }
    }

    /// The round's area, its turn times its radius times its length.
    fn area(&self) -> f64 {
        let [first, second] = self.facing;
        first.dot(second).acos() * self.radius * self.length
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

/// The part with a block between the corners `from` and `to`, raised from
/// `from_height` by `height`, joined to it as `cao_part` joins a step: the
/// tool's faces numbered past the part's, its floor first, then its top, then
/// its sides from the front round to the left.
fn joined_to(part: &Body, [from, to]: [DVec2; 2], from_height: f64, height: f64) -> Body {
    part.union(&tool_between(part, [from, to], from_height, height))
        .expect("the join is built")
}

/// The block a step raises between the corners `from` and `to`, from
/// `from_height` by `height`.
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

/// A block 30 by 20 by 10 with another, 10 by 10 by 10, standing on the
/// middle of its top: the upper block's walls, front, right, back and left,
/// numbered eight to eleven, its top seven, its floor gone into the lower
/// block's top. Its four feet, where its walls meet that top, meet inwards;
/// two meet at each of its four corners, where its upright edge meets
/// outwards.
fn stacked() -> Body {
    joined_to(
        &block([30.0, 20.0, 10.0]),
        [DVec2::new(10.0, 5.0), DVec2::new(20.0, 15.0)],
        10.0,
        10.0,
    )
}

/// The numbers of the walls of the block standing on the other, from the
/// front round to the left.
const STANDING: [usize; 4] = [8, 9, 10, 11];

/// The outline of an L: an arm 30 long and 6 deep along the front, and at its
/// left end a stem 12 wide reaching back to 14. Raised 10, its walls are
/// numbered from the front round: the front two, the arm's right end three,
/// the arm's back four, the stem's right side five, the stem's back six and
/// the left seven. The arm's back and the stem's right side meet inwards, in
/// the L's inner corner.
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
const ARM_BACK: usize = 4;
const STEM_RIGHT: usize = 5;

#[test]
fn a_fillet_and_a_chamfer_on_edges_meeting_at_a_corner_with_a_concave_edge_are_declined_as_unsupported()
 {
    let value = 1.0;
    let stacked = stacked();
    let feet: Vec<EdgeName> = STANDING
        .iter()
        .map(|&wall| edge_between(&stacked, TOP, wall))
        .collect();
    for (foot, name) in STANDING.iter().zip(&feet) {
        let which = format!("the foot of the standing block's wall {foot}, alone");
        stacked
            .fillet(name, value)
            .unwrap_or_else(|declined| panic!("{which}: the fillet declined, {declined:?}"));
        stacked
            .chamfer(name, value)
            .unwrap_or_else(|declined| panic!("{which}: the chamfer declined, {declined:?}"));
    }
    let ell = ell();
    let names = |body: &Body, pairs: &[[usize; 2]]| -> Vec<EdgeName> {
        pairs
            .iter()
            .map(|&[first, second]| edge_between(body, first, second))
            .collect()
    };

    for (what, body, list) in [
        (
            "a foot of the standing block and the upright above it",
            &stacked,
            vec![
                feet[0].clone(),
                edge_between(&stacked, STANDING[0], STANDING[1]),
            ],
        ),
        (
            "a top edge at the L's inner corner and the L's inner upright",
            &ell,
            names(&ell, &[[TOP, ARM_BACK], [ARM_BACK, STEM_RIGHT]]),
        ),
    ] {
        let filleted = body.fillet_edges(&list, value).err();
        let chamfered = body.chamfer_edges(&list, value).err();

        assert_eq!(
            filleted,
            Some(Declined::Unsupported),
            "the fillet on {what}: a broken step and its reason, and no body"
        );
        assert_eq!(
            chamfered,
            Some(Declined::Unsupported),
            "the chamfer on {what}: a broken step and its reason, and no body"
        );
    }
}

#[test]
fn a_fillet_on_two_of_a_corner_s_three_edges_mitres_their_rounds_on_an_ellipse_and_leaves_the_third_edge_sharp_one_radius_short()
 {
    let radius = 3.0;
    let block = block([LENGTH, BREADTH, HEIGHT]);
    let corner = top_front_right();
    let steps_end = block.faces_end();

    for [first, second, third] in TWO_OF_THREE {
        let what = format!(
            "the fillet on {} then {}, {} left",
            corner.called[first], corner.called[second], corner.called[third]
        );
        let names = [corner.name(&block, first), corner.name(&block, second)];

        let filleted = block
            .fillet_edges(&names, radius)
            .unwrap_or_else(|declined| panic!("{what}: declined, {declined:?}"));

        taken_as_asked(filleted.asked, filleted.taken, radius, &what);
        let body = &filleted.body;
        let [long_first, long_second, long_third] =
            [first, second, third].map(|edge| corner.length[edge]);
        let rounds = [
            corner.round(first, [second, third], radius),
            corner.round(second, [first, third], radius),
        ];
        let area: f64 = rounds.iter().map(Round::area).sum();
        sound_drawn_within(
            body,
            mitred_by_rounds([long_first, long_second], radius),
            DRAWN * reach_of(&block) * area,
            &what,
        );
        let [round_first, round_second] = [steps_end, steps_end + 1];
        assert_eq!(
            faces_made(&block, body),
            vec![round_first, round_second],
            "{what}: one round for each edge, numbered in the order the step names them, and no \
             face laid at the corner: {:?}",
            numbers(body)
        );
        assert!(
            numbers(&block).is_subset(&numbers(body)),
            "{what}: a face of the block went: {:?}",
            numbers(body)
        );
        let at = |by: [f64; 3]| corner.point([first, second, third], by);
        let r = radius;
        let [across_first, across_second, shared] =
            [first, second, third].map(|edge| corner.across[edge]);
        let [far_first, far_second, far_third] =
            [first, second, third].map(|edge| corner.far[edge]);

        for (face, round, own, other, sides_expected, which) in [
            (
                round_first,
                &rounds[0],
                first,
                second,
                sides(&[shared, across_second, far_first, round_second]),
                "the round of the first edge",
            ),
            (
                round_second,
                &rounds[1],
                second,
                first,
                sides(&[shared, across_first, far_second, round_first]),
                "the round of the second edge",
            ),
        ] {
            assert!(
                !body.is_flat(face) && body.plane_of(face).is_none(),
                "{what}: {which} is curved, and offers no plane to draw on"
            );
            let corners: Vec<DVec3> = body.triangles_of(face).flatten().collect();
            assert!(!corners.is_empty(), "{what}: {which} is drawn");
            for point in corners {
                assert!(
                    round.holds(point),
                    "{what}: {which} is drawn through {point}, off its cylinder of {r} about the \
                     line from {} along {}",
                    round.start,
                    round.along
                );
                assert!(
                    corner.along_edge(point, own) >= corner.along_edge(point, other) - PLACE,
                    "{what}: {which} is drawn through {point}, past the plane halfway between the \
                     two edges, where the other round meets it in the mitre"
                );
            }
            met_from_outside(
                body,
                face,
                &round.part(r, round.length),
                &format!("{what}: {which}, past the mitre"),
            );
            assert_eq!(
                sides_met(body, face),
                sides_expected,
                "{what}: {which} meets its edge's two faces along its sides, the face its edge \
                 ends on, and the other round along the mitre, by one edge each"
            );
        }
        let short_of_the_corner = at([0.0, 0.0, r]);
        corners_are(
            body,
            shared,
            &[
                at([r, r, 0.0]),
                at([long_first, r, 0.0]),
                at([long_first, long_second, 0.0]),
                at([r, long_second, 0.0]),
            ],
            &format!(
                "{what}: the face the two edges share, set back by the radius from each, its \
                 corner where the two rounds' sides cross"
            ),
        );
        assert_eq!(
            sides_met(body, shared),
            sides(&[round_first, round_second, far_first, far_second]),
            "{what}: the face the two edges share meets the two rounds, and no longer the third \
             edge's faces"
        );
        let third_s_far_end = at([0.0, 0.0, long_third]);
        for (face, other, [beside, beyond], far, round, which) in [
            (
                across_second,
                across_first,
                [at([long_first, 0.0, r]), at([long_first, 0.0, long_third])],
                far_first,
                round_first,
                "the face the first edge and the third share",
            ),
            (
                across_first,
                across_second,
                [
                    at([0.0, long_second, r]),
                    at([0.0, long_second, long_third]),
                ],
                far_second,
                round_second,
                "the face the second edge and the third share",
            ),
        ] {
            corners_are(
                body,
                face,
                &[short_of_the_corner, beside, beyond, third_s_far_end],
                &format!(
                    "{what}: {which}, set back by the radius from the round, the third edge \
                     along it ending one radius short of the corner"
                ),
            );
            assert_eq!(
                sides_met(body, face),
                sides(&[round, far, far_third, other]),
                "{what}: {which} meets its round, the faces the edges end on, and the third \
                 edge's other face, sharp"
            );
        }
        assert_eq!(
            edges_between(body, across_first, across_second).len(),
            1,
            "{what}: {} is still one sharp edge, from where it ends one radius short of the \
             corner to its far end",
            corner.called[third]
        );
    }
}

#[test]
fn a_chamfer_and_a_fillet_on_the_four_top_edges_of_a_block_mitre_its_four_corners() {
    let value = 3.0;
    let block = block([LENGTH, BREADTH, HEIGHT]);
    let edges = top_edges();
    let steps_end = block.faces_end();

    for (order, called) in [
        ([0, 1, 2, 3], "named round the top"),
        ([0, 2, 1, 3], "named the front and the back first"),
    ] {
        let names: Vec<EdgeName> = order.iter().map(|&edge| edges[edge].name(&block)).collect();

        let chamfered = block
            .chamfer_edges(&names, value)
            .unwrap_or_else(|declined| panic!("the chamfer, {called}: declined, {declined:?}"));
        let filleted = block
            .fillet_edges(&names, value)
            .unwrap_or_else(|declined| panic!("the fillet, {called}: declined, {declined:?}"));

        let laid_for = |edge: usize| {
            steps_end
                + order
                    .iter()
                    .position(|&named| named == edge)
                    .expect("every top edge is named")
        };
        let laid: Vec<usize> = (0..4).map(laid_for).collect();
        let rounds: Vec<Round> = edges.iter().map(|edge| edge.round(value)).collect();
        let area: f64 = rounds.iter().map(Round::area).sum();
        for (what, asked, taken, body, promised, drawn) in [
            (
                format!("the chamfer on the four top edges, {called}"),
                chamfered.asked,
                chamfered.taken,
                &chamfered.body,
                ringed_by_flats(value),
                EXACT * ringed_by_flats(value),
            ),
            (
                format!("the fillet on the four top edges, {called}"),
                filleted.asked,
                filleted.taken,
                &filleted.body,
                ringed_by_rounds(value),
                DRAWN * reach_of(&block) * area,
            ),
        ] {
            taken_as_asked(asked, taken, value, &what);
            sound_drawn_within(body, promised, drawn, &what);
            let mut made = laid.clone();
            made.sort();
            assert_eq!(
                faces_made(&block, body),
                made,
                "{what}: one face for each edge, numbered in the order the step names them, and \
                 no face laid at a corner: {:?}",
                numbers(body)
            );
            assert!(
                numbers(&block).is_subset(&numbers(body)),
                "{what}: a face of the block went: {:?}",
                numbers(body)
            );
            let top: Vec<DVec3> = edges.iter().map(|edge| edge.crossings(value)[0]).collect();
            corners_are(
                body,
                TOP,
                &top,
                &format!("{what}: the top, set back by the value from its four edges"),
            );
            assert_eq!(
                sides_met(body, TOP),
                sides(&laid),
                "{what}: the top meets the four faces laid, and no wall"
            );
            for (index, edge) in edges.iter().enumerate() {
                let [before, after] = [(index + 3) % 4, (index + 1) % 4];
                let down = DVec3::Z * value;
                let floor = DVec3::Z * HEIGHT;
                corners_are(
                    body,
                    edge.wall,
                    &[
                        edge.start - down,
                        edge.end() - down,
                        edge.end() - floor,
                        edge.start - floor,
                    ],
                    &format!(
                        "{what}: wall {}, set back by the value from the top, its uprights \
                         ending one value below it",
                        edge.wall
                    ),
                );
                assert_eq!(
                    sides_met(body, edge.wall),
                    sides(&[FLOOR, laid[index], edges[before].wall, edges[after].wall]),
                    "{what}: wall {} meets the floor, the face laid on its top edge, and the \
                     two walls beside it along their uprights, sharp",
                    edge.wall
                );
                assert_eq!(
                    sides_met(body, laid[index]),
                    sides(&[TOP, edge.wall, laid[before], laid[after]]),
                    "{what}: the face laid on wall {}'s top edge meets the top and the wall \
                     along its sides, and the faces laid beside it along the mitres at its two \
                     ends",
                    edge.wall
                );
            }
        }
        for (index, edge) in edges.iter().enumerate() {
            let which = format!(
                "the chamfer on the four top edges, {called}: the flat on wall {}'s top edge",
                edge.wall
            );
            let (corners, facing) = edge.flat(value);
            corners_are(&chamfered.body, laid[index], &corners, &which);
            faces_toward(&chamfered.body, laid[index], facing, &which);
        }
        for (index, (edge, round)) in edges.iter().zip(&rounds).enumerate() {
            let which = format!(
                "the fillet on the four top edges, {called}: the round on wall {}'s top edge",
                edge.wall
            );
            let body = &filleted.body;
            let face = laid[index];
            assert!(
                !body.is_flat(face) && body.plane_of(face).is_none(),
                "{which} is curved, and offers no plane to draw on"
            );
            let corners: Vec<DVec3> = body.triangles_of(face).flatten().collect();
            assert!(!corners.is_empty(), "{which} is drawn");
            for point in corners {
                assert!(
                    round.holds(point),
                    "{which} is drawn through {point}, off its cylinder of {value} about the \
                     line from {} along {}",
                    round.start,
                    round.along
                );
                assert!(
                    edge.between_its_mitres(point),
                    "{which} is drawn through {point}, past a plane halfway between its edge and \
                     the one beside it, where the round beside it meets it in a mitre"
                );
            }
            met_from_outside(
                body,
                face,
                &round.part(value, round.length - value),
                &format!("{which}, between its mitres"),
            );
        }
    }
}

/// The section of a prism lying along X, drawn on the plane of Y and Z: its
/// bottom 16 wide, its top 10 wide, from 2 to 12 at a height of 10, and its
/// two sides slanting in. Raised along X for [`LENGTH`], its numbers are its
/// start at x = 0 nought, its end one, then its bottom two, its right side
/// three, its top four and its left side five. A prism of a convex section,
/// every edge of it meets outwards.
const SLANTED: [DVec2; 4] = [
    DVec2::new(0.0, 0.0),
    DVec2::new(16.0, 0.0),
    DVec2::new(12.0, 10.0),
    DVec2::new(2.0, 10.0),
];
const SLANTED_START: usize = 0;
const SLANTED_END: usize = 1;
const SLANTED_BOTTOM: usize = 2;
const SLANTED_RIGHT: usize = 3;
const SLANTED_TOP: usize = 4;
const SLANTED_LEFT: usize = 5;

/// The slanted prism's matter: its section's area along its length.
const SLANTED_VOLUME: f64 = (16.0 + 10.0) / 2.0 * 10.0 * LENGTH;

/// The corner where the slanted prism's end, its top and its right side meet.
const SLANTED_CORNER: DVec3 = DVec3::new(LENGTH, 12.0, 10.0);

/// The [`SLANTED`] section raised along X for [`LENGTH`].
fn slanted_prism() -> Body {
    let part = Body::default();
    let tool = part
        .tool_raised(
            &profile(Contour::straight(SLANTED.to_vec())),
            Frame {
                origin: DVec3::ZERO,
                u: DVec3::Y,
                v: DVec3::Z,
            },
            DVec3::X * LENGTH,
        )
        .expect("the tool raises");
    part.union(&tool).expect("the part is built")
}

/// The way the slanted prism's right side faces.
fn side_facing() -> DVec3 {
    DVec3::new(0.0, 10.0, 4.0).normalize()
}

/// The way the slanted prism's right side runs down from its top.
fn side_down() -> DVec3 {
    DVec3::new(0.0, 4.0, -10.0).normalize()
}

/// What a step lays on an edge: a flat or a round.
#[derive(Clone, Copy, Debug, PartialEq)]
enum Cut {
    Flat,
    Round,
}

/// An edge at a corner: the corner it runs from, the two faces it lies
/// between, the way it runs from the corner, for each face the way into it
/// from the edge, square to the edge, and the way it faces, and how far before
/// and past the corner a prism raised along it must reach to take all a cut of
/// it takes.
struct EdgeAt {
    called: &'static str,
    at: DVec3,
    faces: [usize; 2],
    along: DVec3,
    into: [DVec3; 2],
    outward: [DVec3; 2],
    reach: [f64; 2],
}

/// The three edges at [`SLANTED_CORNER`]: the top's edge on the end, square,
/// the top's edge on the right side, at the side's slant, and the slanted
/// upright where the side meets the end, square.
fn slanted_corner_edges() -> [EdgeAt; 3] {
    [
        EdgeAt {
            called: "the top's edge on the end",
            at: SLANTED_CORNER,
            faces: [SLANTED_TOP, SLANTED_END],
            along: -DVec3::Y,
            into: [-DVec3::X, -DVec3::Z],
            outward: [DVec3::Z, DVec3::X],
            reach: [-8.0, 18.0],
        },
        EdgeAt {
            called: "the top's edge on the right side",
            at: SLANTED_CORNER,
            faces: [SLANTED_TOP, SLANTED_RIGHT],
            along: -DVec3::X,
            into: [-DVec3::Y, side_down()],
            outward: [DVec3::Z, side_facing()],
            reach: [-2.0, LENGTH + 2.0],
        },
        EdgeAt {
            called: "the slanted upright where the side meets the end",
            at: SLANTED_CORNER,
            faces: [SLANTED_END, SLANTED_RIGHT],
            along: side_down(),
            into: [-side_facing(), -DVec3::X],
            outward: [DVec3::X, side_facing()],
            reach: [-3.0, 116f64.sqrt() + 3.0],
        },
    ]
}

impl EdgeAt {
    /// The edge's name on the part.
    fn name(&self, part: &Body) -> EdgeName {
        let [first, second] = self.faces;
        edge_between(part, first, second)
    }

    /// The angle its two faces meet at, inside the matter.
    fn angle(&self) -> f64 {
        let [first, second] = self.into;
        first.dot(second).acos()
    }

    /// How far a cut of `value` sets back from the edge along each of its
    /// faces: a flat by the value, a round where it is tangent to the face.
    fn set_back(&self, cut: Cut, value: f64) -> f64 {
        match cut {
            Cut::Flat => value,
            Cut::Round => value / (self.angle() / 2.0).tan(),
        }
    }

    /// The round a fillet of `radius` lays on the edge, whole: its axis on the
    /// line halfway between the two faces, the radius from each, as far as
    /// the edge's prism reaches.
    fn round(&self, radius: f64) -> Round {
        let [first, second] = self.into;
        let halfway = (first + second).normalize();
        let [before, past] = self.reach;
        Round {
            start: self.at + halfway * radius / (self.angle() / 2.0).sin() + self.along * before,
            along: self.along,
            length: past - before,
            radius,
            facing: self.outward,
        }
    }

    /// The prism a cut of `value` on this edge alone takes away, raised along
    /// the edge's whole line and past its faces: cut from the part, it takes
    /// what the arithmetic says the cut takes.
    fn tool(&self, part: &Body, cut: Cut, value: f64) -> Body {
        let u = self.into[0];
        let v = self.along.cross(u);
        let flat = |point: DVec3| DVec2::new(point.dot(u), point.dot(v));
        let [mut into_first, mut into_second] = self.into.map(flat);
        let [mut out_first, mut out_second] = self.outward.map(flat);
        if into_first.perp_dot(into_second) > 0.0 {
            std::mem::swap(&mut into_first, &mut into_second);
            std::mem::swap(&mut out_first, &mut out_second);
        }
        let halfway = (into_first + into_second).normalize();
        let far = 2.0 * value + 2.0;
        let set_back = self.set_back(cut, value);
        let [on_first, on_second] = [into_first * set_back, into_second * set_back];
        let corners = vec![
            on_first,
            on_first + out_first * far,
            -halfway * far,
            on_second + out_second * far,
            on_second,
        ];
        let mut runs = vec![Run::Straight; corners.len()];
        if cut == Cut::Round {
            runs[4] = Run::Round {
                center: halfway * value / (self.angle() / 2.0).sin(),
                turn: -(PI - self.angle()),
            };
        }
        let [before, past] = self.reach;
        part.tool_raised(
            &profile(Contour { corners, runs }),
            Frame {
                origin: self.at + self.along * before,
                u,
                v,
            },
            self.along * (past - before),
        )
        .expect("the tool raises")
    }
}

/// The sine of the angle the slanted prism's top meets its right side at.
fn slant() -> f64 {
    10.0 / 116f64.sqrt()
}

/// What the slanted prism keeps once two edges at [`SLANTED_CORNER`] are cut
/// at `value` and mitred, where the module's documentation derives it by
/// hand: every pair of flats, and the pair of rounds on the two square edges.
/// None for the two pairs of rounds taking the top's edge on the side.
fn slanted_mitre(cut: Cut, [first, second]: [usize; 2], value: f64) -> Option<f64> {
    let k = slant();
    let flat_alone = [
        5.0 * value * value + 0.1 * value.powi(3),
        15.0 * k * value * value,
        5.0 * value * value / k,
    ];
    let round_alone = [
        10.0 * value * value * (1.0 - PI / 4.0) + 0.6 * value.powi(3) * (5.0 / 6.0 - PI / 4.0),
        0.0,
        10.0 / k * value * value * (1.0 - PI / 4.0),
    ];
    let pair = (first.min(second), first.max(second));
    match (cut, pair) {
        (Cut::Flat, (0, 2)) => Some(value.powi(3) / (3.0 * k)),
        (Cut::Flat, _) => Some(k * value.powi(3) * (0.5 - k / 6.0)),
        (Cut::Round, (0, 2)) => Some(value.powi(3) * (5.0 / 3.0 - PI / 2.0) / k),
        (Cut::Round, _) => None,
    }
    .map(|overlap| {
        let alone = match cut {
            Cut::Flat => flat_alone,
            Cut::Round => round_alone,
        };
        SLANTED_VOLUME - alone[first] - alone[second] + overlap
    })
}

/// The sides each face of a body meets, as [`sides_met`] lists them, every
/// number taken through `renumbered` first.
fn sides_renumbered(
    body: &Body,
    face: usize,
    renumbered: impl Fn(usize) -> usize,
) -> Vec<Vec<u32>> {
    let mut met: Vec<Vec<u32>> = sides_met(body, face)
        .into_iter()
        .map(|side| {
            side.into_iter()
                .map(|number| renumbered(number as usize) as u32)
                .collect()
        })
        .collect();
    met.sort();
    met
}

#[test]
fn a_chamfer_and_a_fillet_on_two_of_the_three_edges_of_a_corner_that_is_not_square_mitre_them_the_third_left_sharp()
 {
    let value = 2.0;
    let prism = slanted_prism();
    let edges = slanted_corner_edges();
    let steps_end = prism.faces_end();
    let mut cases = Vec::new();
    for order in TWO_OF_THREE {
        for cut in [Cut::Flat, Cut::Round] {
            cases.push((order, cut));
        }
    }

    let laid: Vec<_> = cases
        .iter()
        .map(|&([first, second, _], cut)| {
            let names = [edges[first].name(&prism), edges[second].name(&prism)];
            match cut {
                Cut::Flat => prism
                    .chamfer_edges(&names, value)
                    .map(|chamfered| (chamfered.body, chamfered.asked, chamfered.taken)),
                Cut::Round => prism
                    .fillet_edges(&names, value)
                    .map(|filleted| (filleted.body, filleted.asked, filleted.taken)),
            }
        })
        .collect();

    let declined: Vec<String> = cases
        .iter()
        .zip(&laid)
        .filter_map(|(&([first, second, _], cut), laid)| {
            laid.as_ref().err().map(|declined| {
                format!(
                    "{cut:?} on {} then {}: {declined:?}",
                    edges[first].called, edges[second].called
                )
            })
        })
        .collect();
    assert!(
        declined.is_empty(),
        "every pair of a convex corner's edges is mitred, the corner square or not; declined:\n{}",
        declined.join("\n")
    );
    for ((order, cut), laid) in cases.into_iter().zip(laid) {
        let [first, second, third] = order;
        let what = format!(
            "the {} on {} then {}, {} left",
            match cut {
                Cut::Flat => "chamfer",
                Cut::Round => "fillet",
            },
            edges[first].called,
            edges[second].called,
            edges[third].called
        );
        let (body, asked, taken) = laid.expect("declines are held above");
        taken_as_asked(asked, taken, value, &what);
        let fixture = prism
            .difference(&edges[first].tool(&prism, cut, value))
            .and_then(|cut_once| cut_once.difference(&edges[second].tool(&prism, cut, value)))
            .expect("the two prisms cut the fixture");
        let promised = slanted_mitre(cut, [first, second], value).unwrap_or(fixture.volume());
        let drawn = match cut {
            Cut::Flat => EXACT * promised,
            Cut::Round => {
                let area: f64 = [first, second]
                    .iter()
                    .map(|&edge| edges[edge].round(value).area())
                    .sum();
                DRAWN * reach_of(&prism) * area
            }
        };
        sound_drawn_within(&body, promised, drawn, &what);
        let laid_faces = [steps_end, steps_end + 1];
        assert_eq!(
            faces_made(&prism, &body),
            laid_faces.to_vec(),
            "{what}: one face for each edge, numbered in the order the step names them, and no \
             face laid at the corner: {:?}",
            numbers(&body)
        );
        assert!(
            numbers(&prism).is_subset(&numbers(&body)),
            "{what}: a face of the prism went: {:?}",
            numbers(&body)
        );
        for face in 0..steps_end {
            let plane = body
                .plane_of(face)
                .unwrap_or_else(|| panic!("{what}: face {face} of the prism is still flat"));
            assert!(
                plane
                    .corners
                    .iter()
                    .all(|corner| corner.distance(SLANTED_CORNER) > PLACE),
                "{what}: face {face} still has a corner at the old vertex {SLANTED_CORNER}"
            );
        }
        let third_edge = &edges[third];
        let short = [first, second]
            .iter()
            .map(|&edge| {
                edges[edge].set_back(cut, value)
                    / edges[edge].along.cross(third_edge.along).length()
            })
            .fold(0.0, f64::max);
        let end = SLANTED_CORNER + third_edge.along * short;
        let [one, other] = third_edge.faces;
        assert_eq!(
            edges_between(&body, one, other).len(),
            1,
            "{what}: {} is still one sharp edge",
            third_edge.called
        );
        for face in third_edge.faces {
            let plane = body
                .plane_of(face)
                .unwrap_or_else(|| panic!("{what}: face {face} is still flat"));
            assert!(
                plane
                    .corners
                    .iter()
                    .any(|corner| corner.distance(end) < PLACE),
                "{what}: {} ends at {end}, {short} from the corner, where the cut set back \
                 farthest along it crosses it — no corner of face {face} there, among {:?}",
                third_edge.called,
                plane.corners
            );
        }
        for (face, &edge) in laid_faces.iter().zip(&[first, second]) {
            let which = format!("{what}: the face laid on {}", edges[edge].called);
            match cut {
                Cut::Flat => {
                    let [one, other] = edges[edge].outward;
                    faces_toward(&body, *face, (one + other).normalize(), &which);
                }
                Cut::Round => {
                    assert!(
                        !body.is_flat(*face) && body.plane_of(*face).is_none(),
                        "{which} is curved, and offers no plane to draw on"
                    );
                    let round = edges[edge].round(value);
                    let corners: Vec<DVec3> = body.triangles_of(*face).flatten().collect();
                    assert!(!corners.is_empty(), "{which} is drawn");
                    for point in corners {
                        assert!(
                            round.holds(point),
                            "{which} is drawn through {point}, off its cylinder of {value} about \
                             the line from {} along {}",
                            round.start,
                            round.along
                        );
                    }
                }
            }
        }
        let made_by_fixture = faces_made(&prism, &fixture);
        let to_fixture = |number: usize| match laid_faces.iter().position(|&face| face == number) {
            Some(rank) => made_by_fixture[rank],
            None => number,
        };
        for face in numbers(&body) {
            assert_eq!(
                sides_renumbered(&body, face, to_fixture),
                sides_met(&fixture, to_fixture(face)),
                "{what}: face {face} meets the faces it meets on the prism with the two cuts' \
                 prisms taken away, whole"
            );
            if body.plane_of(face).is_some() {
                let expected = fixture
                    .plane_of(to_fixture(face))
                    .expect("the fixture's face is flat too");
                corners_are(
                    &body,
                    face,
                    &expected.corners,
                    &format!(
                        "{what}: face {face}, as on the prism with the two cuts' prisms taken \
                         away"
                    ),
                );
            }
        }
    }
}

#[test]
fn a_chamfer_on_the_three_edges_of_a_corner_closes_it_with_a_triangle_where_the_flats_edges_cross()
{
    let distance = 3.0;
    let block = block([LENGTH, BREADTH, HEIGHT]);
    let corner = top_front_right();
    let steps_end = block.faces_end();
    let triangle = steps_end + 3;

    for order in ORDERS_OF_THREE {
        let what = format!(
            "the chamfer on {}, {} and {}",
            corner.called[order[0]], corner.called[order[1]], corner.called[order[2]]
        );
        let names = order.map(|edge| corner.name(&block, edge));

        let chamfered = block
            .chamfer_edges(&names, distance)
            .unwrap_or_else(|declined| panic!("{what}: declined, {declined:?}"));

        taken_as_asked(chamfered.asked, chamfered.taken, distance, &what);
        let body = &chamfered.body;
        sound_drawn_within(
            body,
            closed_by_a_triangle(corner.length, distance),
            EXACT * closed_by_a_triangle(corner.length, distance),
            &what,
        );
        let flat_of = |edge: usize| {
            steps_end
                + order
                    .iter()
                    .position(|&named| named == edge)
                    .expect("every edge of the corner is named")
        };
        assert_eq!(
            faces_made(&block, body),
            vec![steps_end, steps_end + 1, steps_end + 2, triangle],
            "{what}: one flat for each edge, numbered in the order the step names them, and the \
             triangle after them: {:?}",
            numbers(body)
        );
        assert_eq!(
            body.faces_end(),
            triangle + 1,
            "{what}: the next step numbers its own faces past the triangle"
        );
        assert!(
            numbers(&block).is_subset(&numbers(body)),
            "{what}: a face of the block went: {:?}",
            numbers(body)
        );
        let d = distance;
        let in_order = [0, 1, 2];
        let at = |by: [f64; 3]| corner.point(in_order, by);
        let crossings = [at([d, d, 0.0]), at([d, 0.0, d]), at([0.0, d, d])];

        corners_are(
            body,
            triangle,
            &crossings,
            &format!(
                "{what}: the triangle, its corners where the flats' edges cross on the three faces"
            ),
        );
        faces_toward(
            body,
            triangle,
            -(corner.along[0] + corner.along[1] + corner.along[2]).normalize(),
            &format!(
                "{what}: the triangle, facing away from the corner, square to none of its faces"
            ),
        );
        assert_eq!(
            sides_met(body, triangle),
            sides(&[flat_of(0), flat_of(1), flat_of(2)]),
            "{what}: the triangle meets the three flats, one side each, and nothing else"
        );
        for edge in 0..3 {
            let [one, other] = [(edge + 1) % 3, (edge + 2) % 3];
            let frame = [edge, one, other];
            let at = |by: [f64; 3]| corner.point(frame, by);
            let long = corner.length[edge];
            let flat = flat_of(edge);
            let which = format!("{what}: the flat of {}", corner.called[edge]);
            corners_are(
                body,
                flat,
                &[
                    at([long, d, 0.0]),
                    at([long, 0.0, d]),
                    at([d, 0.0, d]),
                    at([d, d, 0.0]),
                ],
                &format!(
                    "{which}, from its far end to the straight side it shares with the triangle, \
                     one distance from the corner"
                ),
            );
            faces_toward(
                body,
                flat,
                -(corner.along[one] + corner.along[other]).normalize(),
                &which,
            );
            assert_eq!(
                sides_met(body, flat),
                sides(&[
                    corner.across[one],
                    corner.across[other],
                    corner.far[edge],
                    triangle
                ]),
                "{which} meets its edge's two faces along its sides, the face its edge ends on, \
                 and the triangle, by one edge each, and no other flat"
            );
            let face = corner.across[edge];
            let at = |by: [f64; 3]| corner.point([one, other, edge], by);
            let [long_one, long_other] = [corner.length[one], corner.length[other]];
            corners_are(
                body,
                face,
                &[
                    at([d, d, 0.0]),
                    at([long_one, d, 0.0]),
                    at([long_one, long_other, 0.0]),
                    at([d, long_other, 0.0]),
                ],
                &format!(
                    "{what}: face {face}, set back by the distance from both its edges at the \
                     corner, touching the triangle at one of its corners"
                ),
            );
            assert_eq!(
                sides_met(body, face),
                sides(&[
                    flat_of(one),
                    flat_of(other),
                    corner.far[one],
                    corner.far[other]
                ]),
                "{what}: face {face} meets the flats of its two edges at the corner and the faces \
                 they end on, and not the triangle along a side"
            );
        }
        for face in numbers(body) {
            if let Some(plane) = body.plane_of(face) {
                assert!(
                    plane
                        .corners
                        .iter()
                        .all(|point| point.distance(corner.at) > PLACE),
                    "{what}: face {face} still has a corner at the old vertex {}",
                    corner.at
                );
            }
        }
    }
}

#[test]
fn a_fillet_on_the_three_edges_of_a_corner_closes_it_with_a_piece_of_sphere_one_radius_from_its_three_faces()
 {
    let radius = 3.0;
    let block = block([LENGTH, BREADTH, HEIGHT]);
    let corner = top_front_right();
    let steps_end = block.faces_end();
    let piece = steps_end + 3;
    let r = radius;
    let in_order = [0, 1, 2];
    let centre = corner.point(in_order, [r, r, r]);
    let rounds = [0, 1, 2].map(|edge| corner.round(edge, [(edge + 1) % 3, (edge + 2) % 3], r));
    let area: f64 = rounds.iter().map(Round::area).sum::<f64>() + PI * r * r / 2.0;

    for order in ORDERS_OF_THREE {
        let what = format!(
            "the fillet on {}, {} and {}",
            corner.called[order[0]], corner.called[order[1]], corner.called[order[2]]
        );
        let names = order.map(|edge| corner.name(&block, edge));

        let filleted = block
            .fillet_edges(&names, radius)
            .unwrap_or_else(|declined| panic!("{what}: declined, {declined:?}"));

        taken_as_asked(filleted.asked, filleted.taken, radius, &what);
        let body = &filleted.body;
        sound_drawn_within(
            body,
            closed_by_a_sphere(corner.length, r),
            DRAWN * reach_of(&block) * area,
            &what,
        );
        let round_of = |edge: usize| {
            steps_end
                + order
                    .iter()
                    .position(|&named| named == edge)
                    .expect("every edge of the corner is named")
        };
        assert_eq!(
            faces_made(&block, body),
            vec![steps_end, steps_end + 1, steps_end + 2, piece],
            "{what}: one round for each edge, numbered in the order the step names them, and the \
             piece of sphere after them: {:?}",
            numbers(body)
        );
        assert_eq!(
            body.faces_end(),
            piece + 1,
            "{what}: the next step numbers its own faces past the piece of sphere"
        );
        assert!(
            numbers(&block).is_subset(&numbers(body)),
            "{what}: a face of the block went: {:?}",
            numbers(body)
        );

        let which = format!("{what}: the piece of sphere");
        assert!(
            !body.is_flat(piece) && body.plane_of(piece).is_none(),
            "{which} is curved, and offers no plane to draw on"
        );
        let drawn: Vec<DVec3> = body.triangles_of(piece).flatten().collect();
        assert!(!drawn.is_empty(), "{which} is drawn");
        for point in &drawn {
            assert!(
                (point.distance(centre) - r).abs() < PLACE,
                "{which} is drawn through {point}, off the sphere of {r} about {centre}, one \
                 radius from each of the corner's three faces"
            );
            for edge in 0..3 {
                assert!(
                    corner.along_edge(*point, edge) <= r + PLACE,
                    "{which} is drawn through {point}, past the end section of the round on {}, \
                     one radius from the corner along it",
                    corner.called[edge]
                );
            }
        }
        for (face, point) in [
            (corner.across[0], corner.point(in_order, [0.0, r, r])),
            (corner.across[1], corner.point(in_order, [r, 0.0, r])),
            (corner.across[2], corner.point(in_order, [r, r, 0.0])),
        ] {
            assert!(
                drawn.iter().any(|corner| corner.distance(point) < PLACE),
                "{which}: a corner at {point}, where it touches face {face}"
            );
        }
        for weights in [
            [1.0, 1.0, 1.0],
            [2.0, 1.0, 1.0],
            [1.0, 2.0, 1.0],
            [1.0, 1.0, 2.0],
        ] {
            let facing = -(0..3)
                .map(|edge| corner.along[edge] * weights[edge])
                .sum::<DVec3>()
                .normalize();
            let from = centre + facing * (r + 10.0);
            let hit = body.ray_hit(from, -facing).unwrap_or_else(|| {
                panic!("{which}: a ray from {from} toward its centre meets nothing")
            });
            assert_eq!(
                hit.face, piece,
                "{which}: a ray from {from} toward its centre meets face {} before it",
                hit.face
            );
            assert!(
                hit.normal.distance(facing) < PLACE,
                "{which}: where a ray from {from} meets it, it faces {}, not {facing}, away from \
                 its centre",
                hit.normal
            );
        }
        assert_eq!(
            sides_met(body, piece),
            sides(&[round_of(0), round_of(1), round_of(2)]),
            "{which} meets the three rounds, one side each, and nothing else"
        );

        for (edge, round) in rounds.iter().enumerate() {
            let [one, other] = [(edge + 1) % 3, (edge + 2) % 3];
            let face = round_of(edge);
            let which = format!("{what}: the round of {}", corner.called[edge]);
            assert!(
                !body.is_flat(face) && body.plane_of(face).is_none(),
                "{which} is curved, and offers no plane to draw on"
            );
            let corners: Vec<DVec3> = body.triangles_of(face).flatten().collect();
            assert!(!corners.is_empty(), "{which} is drawn");
            for point in &corners {
                assert!(
                    round.holds(*point),
                    "{which} is drawn through {point}, off its cylinder of {r} about the line \
                     from {} along {}",
                    round.start,
                    round.along
                );
                assert!(
                    corner.along_edge(*point, edge) >= r - PLACE,
                    "{which} is drawn through {point}, short of its end section, one radius from \
                     the corner along its edge"
                );
            }
            let section: Vec<&DVec3> = corners
                .iter()
                .filter(|point| (corner.along_edge(**point, edge) - r).abs() < PLACE)
                .collect();
            let at = |by: [f64; 3]| corner.point([edge, one, other], by);
            for end in [at([r, 0.0, r]), at([r, r, 0.0])] {
                assert!(
                    section.iter().any(|point| point.distance(end) < PLACE),
                    "{which}: its end section, an arc in the plane through the sphere's centre \
                     square to its edge, ends at {end}, among {section:?}"
                );
            }
            assert!(
                section.len() > 2,
                "{which}: its end section is drawn along the arc between its two ends: \
                 {section:?}"
            );
            met_from_outside(
                body,
                face,
                &round.part(r, round.length),
                &format!("{which}, past its end section"),
            );
            assert_eq!(
                sides_met(body, face),
                sides(&[
                    corner.across[one],
                    corner.across[other],
                    corner.far[edge],
                    piece
                ]),
                "{which} meets its edge's two faces along its sides, the face its edge ends on, \
                 and the piece of sphere along its end section, by one edge each, and no other \
                 round"
            );

            let across = corner.across[edge];
            let at = |by: [f64; 3]| corner.point([one, other, edge], by);
            let [long_one, long_other] = [corner.length[one], corner.length[other]];
            corners_are(
                body,
                across,
                &[
                    at([r, r, 0.0]),
                    at([long_one, r, 0.0]),
                    at([long_one, long_other, 0.0]),
                    at([r, long_other, 0.0]),
                ],
                &format!(
                    "{what}: face {across}, set back by the radius from both its edges at the \
                     corner, touching the piece of sphere at one of its corners"
                ),
            );
            assert_eq!(
                sides_met(body, across),
                sides(&[
                    round_of(one),
                    round_of(other),
                    corner.far[one],
                    corner.far[other]
                ]),
                "{what}: face {across} meets the rounds of its two edges at the corner and the \
                 faces they end on, and not the piece of sphere along a side"
            );
        }
        for face in numbers(body) {
            if let Some(plane) = body.plane_of(face) {
                assert!(
                    plane
                        .corners
                        .iter()
                        .all(|point| point.distance(corner.at) > PLACE),
                    "{what}: face {face} still has a corner at the old vertex {}",
                    corner.at
                );
            }
        }
    }
}

/// A convex corner where three flat faces meet: where it stands, each face's
/// number and the way it faces, and for the edge across from each face — the
/// one the two others meet along — what it is called, how long it is and the
/// face it ends on.
struct Trihedral {
    at: DVec3,
    faces: [usize; 3],
    outward: [DVec3; 3],
    called: [&'static str; 3],
    length: [f64; 3],
    far: [usize; 3],
}

impl Trihedral {
    /// The edge across from face `n`, running from the corner along the two
    /// other faces, into the matter.
    fn edge(&self, n: usize) -> EdgeAt {
        let [one, other] = [(n + 1) % 3, (n + 2) % 3];
        let mut along = self.outward[one].cross(self.outward[other]).normalize();
        if along.dot(self.outward[n]) > 0.0 {
            along = -along;
        }
        let into = |face: usize, beside: usize| {
            let way = self.outward[face].cross(along).normalize();
            if way.dot(self.outward[beside]) > 0.0 {
                -way
            } else {
                way
            }
        };
        EdgeAt {
            called: self.called[n],
            at: self.at,
            faces: [self.faces[one], self.faces[other]],
            along,
            into: [into(one, other), into(other, one)],
            outward: [self.outward[one], self.outward[other]],
            reach: [-3.0, self.length[n] + 3.0],
        }
    }

    /// Where the flats a chamfer of `distance` lays on the two edges of face
    /// `n` cross on it: the distance from each edge, square to it in the face,
    /// so the distance over the sine of the angle the two edges meet at along
    /// each of them.
    fn crossing(&self, n: usize, distance: f64) -> DVec3 {
        let [one, other] = [(n + 1) % 3, (n + 2) % 3].map(|edge| self.edge(edge).along);
        self.at + (one + other) * distance / one.cross(other).length()
    }

    /// The point `radius` in from each of the three faces.
    fn centre(&self, radius: f64) -> DVec3 {
        let [first, second, third] = self.outward;
        let facing = DMat3::from_cols(first, second, third).transpose();
        self.at + facing.inverse() * DVec3::splat(-radius)
    }

    /// The solid angle between the three ways the faces face: what a piece
    /// of sphere tangent to the three, seen from its centre, covers.
    fn spread(&self) -> f64 {
        let [a, b, c] = self.outward;
        2.0 * a
            .dot(b.cross(c))
            .abs()
            .atan2(1.0 + a.dot(b) + b.dot(c) + c.dot(a))
    }
}

/// The slanted prism's corner at [`SLANTED_CORNER`], its faces the right side,
/// the end and the top: across from them the top's edge on the end, ending on
/// the left side, the top's edge on the side, ending on the start, and the
/// slanted upright, ending on the bottom.
fn slanted_trihedral() -> Trihedral {
    Trihedral {
        at: SLANTED_CORNER,
        faces: [SLANTED_RIGHT, SLANTED_END, SLANTED_TOP],
        outward: [side_facing(), DVec3::X, DVec3::Z],
        called: [
            "the top's edge on the end",
            "the top's edge on the right side",
            "the slanted upright where the side meets the end",
        ],
        length: [10.0, LENGTH, 116f64.sqrt()],
        far: [SLANTED_LEFT, SLANTED_START, SLANTED_BOTTOM],
    }
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

/// The way the block's top is tilted, in two directions at once: rising 0.1
/// along X and 0.2 along Y.
fn tilted_up() -> DVec3 {
    DVec3::new(-0.1, -0.2, 1.0).normalize()
}

/// The block 30 by 10 by 20 with its top cut off by the plane rising from 12
/// at its origin, 0.1 along X and 0.2 along Y: its walls and floor numbered as
/// the block's, the tilted top six. A convex solid, every edge of it meets
/// outwards.
fn tilted_block() -> Body {
    let block = block([LENGTH, BREADTH, HEIGHT]);
    block
        .difference(&tilted_slab(&block))
        .expect("the tilted top is cut")
}

/// The slab the tilted block's top is cut off by: beyond the tilted plane,
/// 60 each way across it and 60 deep.
fn tilted_slab(block: &Body) -> Body {
    beyond(block, DVec3::new(15.0, 5.0, 14.5), tilted_up(), 60.0)
}

/// The reach the tilted block is drawn at: a cut keeps the larger reach of
/// the part and of the tool cut from it, and the slab's far corners reach
/// past the block's box, three times as far.
fn tilted_reach() -> f64 {
    let block = block([LENGTH, BREADTH, HEIGHT]);
    reach_of(&block).max(reach_of(&tilted_slab(&block)))
}

/// The tilted block's corner at (30, 0, 15), where no two of the three edges
/// are square: its faces the right wall, the front and the tilted top; across
/// from them the tilted top's edge on the front, ending on the left wall, its
/// edge on the right, ending on the back, and the upright, ending on the
/// floor.
fn tilted_trihedral() -> Trihedral {
    Trihedral {
        at: DVec3::new(LENGTH, 0.0, 15.0),
        faces: [RIGHT, FRONT, 6],
        outward: [DVec3::X, -DVec3::Y, tilted_up()],
        called: [
            "the tilted top's edge on the front",
            "the tilted top's edge on the right",
            "the upright where the front meets the right",
        ],
        length: [909f64.sqrt(), 104f64.sqrt(), 15.0],
        far: [LEFT, BACK, FLOOR],
    }
}

/// What a body keeps once the three edges at a convex corner are chamfered
/// at `distance` and the corner closed by the triangle through the crossings,
/// computed apart from the step: the three flats' prisms, raised whole along
/// each edge's line, and the slab beyond the triangle's plane on the corner's
/// side, cut away by the booleans. The body being convex, nothing of it but
/// the corner stands on that side.
fn triangle_by_booleans(body: &Body, corner: &Trihedral, distance: f64) -> f64 {
    let cut = (0..3).fold(body.clone(), |part, edge| {
        part.difference(&corner.edge(edge).tool(&part, Cut::Flat, distance))
            .expect("a flat's prism cuts the body")
    });
    let crossings = [0, 1, 2].map(|face| corner.crossing(face, distance));
    let mut toward = (crossings[1] - crossings[0])
        .cross(crossings[2] - crossings[0])
        .normalize();
    if toward.dot(corner.at - crossings[0]) < 0.0 {
        toward = -toward;
    }
    cut.difference(&beyond(&cut, crossings[0], toward, 100.0))
        .expect("the triangle's slab cuts the corner")
        .volume()
}

/// What a body keeps once the three edges at a convex corner are rounded at
/// `radius` and the corner closed by a piece of sphere, computed apart from
/// the step: the three rounds' prisms, raised whole, cut away by the
/// booleans; then, of what is left, the part on the corner's side of the
/// three planes through the centre square to the edges — the cell the piece
/// of sphere fills — taken away, and the ball's sector over that cell, a
/// third of the radius cubed times the solid angle the three faces' ways
/// span, given back.
fn sphere_by_booleans(body: &Body, corner: &Trihedral, radius: f64) -> f64 {
    spheres_by_booleans(
        body,
        &[corner],
        &[0, 1, 2].map(|edge| corner.edge(edge)),
        radius,
    )
}

/// What a body keeps once `edges` are rounded at `radius` and each of
/// `corners`, three of them meeting there, closed by a piece of sphere,
/// computed as [`sphere_by_booleans`] computes one, each corner's cell taken
/// apart from the others'.
fn spheres_by_booleans(body: &Body, corners: &[&Trihedral], edges: &[EdgeAt], radius: f64) -> f64 {
    let rounded = edges.iter().fold(body.clone(), |part, edge| {
        part.difference(&edge.tool(&part, Cut::Round, radius))
            .expect("a round's prism cuts the body")
    });
    corners.iter().fold(rounded.volume(), |kept, corner| {
        let centre = corner.centre(radius);
        let cell = (0..3).fold(rounded.clone(), |part, edge| {
            part.difference(&beyond(&part, centre, corner.edge(edge).along, 100.0))
                .expect("a plane through the centre cuts the body")
        });
        kept - cell.volume() + radius.powi(3) * corner.spread() / 3.0
    })
}

/// What the slanted prism keeps once the three edges at [`SLANTED_CORNER`]
/// are chamfered at `distance` and the corner closed by a triangle, as the
/// module's documentation derives it by hand.
fn slanted_triangle(distance: f64) -> f64 {
    let k = slant();
    let d = distance;
    SLANTED_VOLUME - 5.0 * d * d - 0.1 * d.powi(3) - 15.0 * k * d * d - 5.0 * d * d / k
        + (k + 1.0 / k) * d.powi(3) / 3.0
}

/// What the slanted prism keeps once the three edges at [`SLANTED_CORNER`]
/// are rounded at `radius` and the corner closed by a piece of sphere, as the
/// module's documentation derives it by hand: each round whole, less its part
/// on the corner's side of its end section, and the cell those sections bound
/// at the corner, less the ball's sector in it.
fn slanted_sphere(radius: f64) -> f64 {
    let k = slant();
    let r = radius;
    let set_back = 1.0 / k - 0.4;
    let square = r * r * (1.0 - PI / 4.0);
    let slanted = r * r * (set_back - 2.5f64.atan() / 2.0);
    let moment = r.powi(3) * (5.0 / 6.0 - PI / 4.0);
    let whole = (10.0 * square + 0.6 * moment) + LENGTH * slanted + 10.0 / k * square;
    let short_of_the_corner = 2.0 * (r * set_back * square + 0.4 * moment) + r * slanted;
    let cell = r.powi(3) * (1.0 - 0.6 * k + 0.232 * k.powi(3)) / k;
    let sector = r.powi(3) * 2.5f64.atan() / 3.0;
    SLANTED_VOLUME - whole + short_of_the_corner - cell + sector
}

/// Holds that the chamfer on the three edges at `corner`, named in `order`,
/// at `distance`, closes the corner with a triangle where the flats' edges
/// cross, and leaves `promised`.
fn closes_with_a_triangle(
    part: &Body,
    corner: &Trihedral,
    order: [usize; 3],
    distance: f64,
    promised: f64,
) {
    let what = format!(
        "the chamfer on {}, {} and {}",
        corner.called[order[0]], corner.called[order[1]], corner.called[order[2]]
    );
    let names = order.map(|edge| corner.edge(edge).name(part));

    let chamfered = part
        .chamfer_edges(&names, distance)
        .unwrap_or_else(|declined| panic!("{what}: declined, {declined:?}"));

    taken_as_asked(chamfered.asked, chamfered.taken, distance, &what);
    let body = &chamfered.body;
    sound_drawn_within(body, promised, EXACT * promised, &what);
    let steps_end = part.faces_end();
    let triangle = steps_end + 3;
    let flat_of = |edge: usize| {
        steps_end
            + order
                .iter()
                .position(|&named| named == edge)
                .expect("every edge of the corner is named")
    };
    assert_eq!(
        faces_made(part, body),
        vec![steps_end, steps_end + 1, steps_end + 2, triangle],
        "{what}: one flat for each edge, numbered in the order the step names them, and the \
         triangle after them: {:?}",
        numbers(body)
    );
    assert_eq!(
        body.faces_end(),
        triangle + 1,
        "{what}: the next step numbers its own faces past the triangle"
    );
    assert!(
        numbers(part).is_subset(&numbers(body)),
        "{what}: a face of the part went: {:?}",
        numbers(body)
    );
    let crossings = [0, 1, 2].map(|face| corner.crossing(face, distance));
    corners_are(
        body,
        triangle,
        &crossings,
        &format!(
            "{what}: the triangle, its corners where the flats' edges cross on the three faces"
        ),
    );
    let mut away = (crossings[1] - crossings[0])
        .cross(crossings[2] - crossings[0])
        .normalize();
    if away.dot(corner.at - crossings[0]) < 0.0 {
        away = -away;
    }
    faces_toward(
        body,
        triangle,
        away,
        &format!("{what}: the triangle, facing the old corner"),
    );
    assert_eq!(
        sides_met(body, triangle),
        sides(&[flat_of(0), flat_of(1), flat_of(2)]),
        "{what}: the triangle meets the three flats, one side each, and nothing else"
    );
    for edge in 0..3 {
        let [one, other] = [(edge + 1) % 3, (edge + 2) % 3];
        let at = corner.edge(edge);
        let flat = flat_of(edge);
        let which = format!("{what}: the flat of {}", at.called);
        let [first, second] = at.outward;
        faces_toward(body, flat, (first + second).normalize(), &which);
        let plane = body
            .plane_of(flat)
            .unwrap_or_else(|| panic!("{which} offers no plane"));
        for crossing in [crossings[one], crossings[other]] {
            assert!(
                plane
                    .corners
                    .iter()
                    .any(|point| point.distance(crossing) < PLACE),
                "{which} ends on the triangle along a straight side between the crossings on its \
                 two faces: no corner at {crossing}, among {:?}",
                plane.corners
            );
        }
        let [face_one, face_other] = at.faces;
        assert_eq!(
            sides_met(body, flat),
            sides(&[face_one, face_other, corner.far[edge], triangle]),
            "{which} meets its edge's two faces along its sides, the face its edge ends on, and \
             the triangle, by one edge each, and no other flat"
        );
        let face = corner.faces[edge];
        let plane = body
            .plane_of(face)
            .unwrap_or_else(|| panic!("{what}: face {face} is still flat"));
        assert!(
            plane
                .corners
                .iter()
                .any(|point| point.distance(crossings[edge]) < PLACE),
            "{what}: face {face} has a corner where the two flats on it cross, {}",
            crossings[edge]
        );
        let met = sides_met(body, face);
        for laid in [flat_of(one), flat_of(other)] {
            assert!(
                met.contains(&vec![laid as u32]),
                "{what}: face {face} meets the flat {laid} of one of its edges at the corner: \
                 {met:?}"
            );
        }
        assert!(
            !met.contains(&vec![triangle as u32]),
            "{what}: face {face} touches the triangle at a corner only: {met:?}"
        );
    }
    for face in numbers(body) {
        if let Some(plane) = body.plane_of(face) {
            assert!(
                plane
                    .corners
                    .iter()
                    .all(|point| point.distance(corner.at) > PLACE),
                "{what}: face {face} still has a corner at the old vertex {}",
                corner.at
            );
        }
    }
}

/// Holds that the fillet on the three edges at `corner`, named in `order`, at
/// `radius`, closes the corner with a piece of sphere one radius from its
/// three faces, each round ending on the plane through the sphere's centre
/// square to its edge, and leaves `promised`, its triangles within the
/// drawing's tolerance at `drawn_at`, the reach the part is drawn at.
fn closes_with_a_sphere(
    part: &Body,
    corner: &Trihedral,
    order: [usize; 3],
    radius: f64,
    promised: f64,
    drawn_at: f64,
) {
    let what = format!(
        "the fillet on {}, {} and {}",
        corner.called[order[0]], corner.called[order[1]], corner.called[order[2]]
    );
    let names = order.map(|edge| corner.edge(edge).name(part));

    let filleted = part
        .fillet_edges(&names, radius)
        .unwrap_or_else(|declined| panic!("{what}: declined, {declined:?}"));

    taken_as_asked(filleted.asked, filleted.taken, radius, &what);
    let body = &filleted.body;
    let r = radius;
    let centre = corner.centre(r);
    let edges = [0, 1, 2].map(|edge| corner.edge(edge));
    let rounds = [0, 1, 2].map(|edge| Round {
        start: centre,
        along: edges[edge].along,
        length: corner.length[edge] + 2.0 * r,
        radius: r,
        facing: edges[edge].outward,
    });
    let area: f64 = [0, 1, 2]
        .iter()
        .map(|&edge| edges[edge].round(r).area())
        .sum::<f64>()
        + corner.spread() * r * r;
    sound_drawn_within(body, promised, DRAWN * drawn_at * area, &what);
    let steps_end = part.faces_end();
    let piece = steps_end + 3;
    let round_of = |edge: usize| {
        steps_end
            + order
                .iter()
                .position(|&named| named == edge)
                .expect("every edge of the corner is named")
    };
    assert_eq!(
        faces_made(part, body),
        vec![steps_end, steps_end + 1, steps_end + 2, piece],
        "{what}: one round for each edge, numbered in the order the step names them, and the \
         piece of sphere after them: {:?}",
        numbers(body)
    );
    assert_eq!(
        body.faces_end(),
        piece + 1,
        "{what}: the next step numbers its own faces past the piece of sphere"
    );
    assert!(
        numbers(part).is_subset(&numbers(body)),
        "{what}: a face of the part went: {:?}",
        numbers(body)
    );

    let which = format!("{what}: the piece of sphere");
    assert!(
        !body.is_flat(piece) && body.plane_of(piece).is_none(),
        "{which} is curved, and offers no plane to draw on"
    );
    let drawn: Vec<DVec3> = body.triangles_of(piece).flatten().collect();
    assert!(!drawn.is_empty(), "{which} is drawn");
    for point in &drawn {
        assert!(
            (point.distance(centre) - r).abs() < PLACE,
            "{which} is drawn through {point}, off the sphere of {r} about {centre}, one radius \
             from each of the corner's three faces"
        );
        for edge in &edges {
            assert!(
                (*point - centre).dot(edge.along) <= PLACE,
                "{which} is drawn through {point}, past the end section of the round on {}",
                edge.called
            );
        }
    }
    for (face, way) in corner.faces.iter().zip(corner.outward) {
        let foot = centre + way * r;
        assert!(
            drawn.iter().any(|point| point.distance(foot) < PLACE),
            "{which}: a corner at {foot}, where it touches face {face}"
        );
    }
    for weights in [
        [1.0, 1.0, 1.0],
        [2.0, 1.0, 1.0],
        [1.0, 2.0, 1.0],
        [1.0, 1.0, 2.0],
    ] {
        let facing = (0..3)
            .map(|face| corner.outward[face] * weights[face])
            .sum::<DVec3>()
            .normalize();
        let from = centre + facing * (r + 10.0);
        let hit = body.ray_hit(from, -facing).unwrap_or_else(|| {
            panic!("{which}: a ray from {from} toward its centre meets nothing")
        });
        assert_eq!(
            hit.face, piece,
            "{which}: a ray from {from} toward its centre meets face {} before it",
            hit.face
        );
        assert!(
            hit.normal.distance(facing) < PLACE,
            "{which}: where a ray from {from} meets it, it faces {}, not {facing}, away from its \
             centre",
            hit.normal
        );
    }
    assert_eq!(
        sides_met(body, piece),
        sides(&[round_of(0), round_of(1), round_of(2)]),
        "{which} meets the three rounds, one side each, and nothing else"
    );

    for (edge, round) in rounds.iter().enumerate() {
        let [one, other] = [(edge + 1) % 3, (edge + 2) % 3];
        let at = &edges[edge];
        let face = round_of(edge);
        let which = format!("{what}: the round of {}", at.called);
        assert!(
            !body.is_flat(face) && body.plane_of(face).is_none(),
            "{which} is curved, and offers no plane to draw on"
        );
        let corners: Vec<DVec3> = body.triangles_of(face).flatten().collect();
        assert!(!corners.is_empty(), "{which} is drawn");
        for point in &corners {
            assert!(
                round.holds(*point),
                "{which} is drawn through {point}, off its cylinder of {r} about the line from \
                 {centre} along {}",
                at.along
            );
        }
        let section: Vec<&DVec3> = corners
            .iter()
            .filter(|point| ((**point - centre).dot(at.along)).abs() < PLACE)
            .collect();
        for way in at.outward {
            let foot = centre + way * r;
            assert!(
                section.iter().any(|point| point.distance(foot) < PLACE),
                "{which}: its end section, an arc in the plane through the sphere's centre square \
                 to its edge, ends at {foot}, among {section:?}"
            );
        }
        assert!(
            section.len() > 2,
            "{which}: its end section is drawn along the arc between its two ends: {section:?}"
        );
        met_from_outside(
            body,
            face,
            &round.part(r, 0.6 * corner.length[edge]),
            &format!("{which}, past its end section"),
        );
        let [face_one, face_other] = at.faces;
        assert_eq!(
            sides_met(body, face),
            sides(&[face_one, face_other, corner.far[edge], piece]),
            "{which} meets its edge's two faces along its sides, the face its edge ends on, and \
             the piece of sphere along its end section, by one edge each, and no other round"
        );
        let own = corner.faces[edge];
        let foot = centre + corner.outward[edge] * r;
        let plane = body
            .plane_of(own)
            .unwrap_or_else(|| panic!("{what}: face {own} is still flat"));
        assert!(
            plane
                .corners
                .iter()
                .any(|point| point.distance(foot) < PLACE),
            "{what}: face {own} has a corner where the piece of sphere touches it, {foot}"
        );
        let met = sides_met(body, own);
        for laid in [round_of(one), round_of(other)] {
            assert!(
                met.contains(&vec![laid as u32]),
                "{what}: face {own} meets the round {laid} of one of its edges at the corner: \
                 {met:?}"
            );
        }
        assert!(
            !met.contains(&vec![piece as u32]),
            "{what}: face {own} touches the piece of sphere at a corner only: {met:?}"
        );
    }
    for face in numbers(body) {
        if let Some(plane) = body.plane_of(face) {
            assert!(
                plane
                    .corners
                    .iter()
                    .all(|point| point.distance(corner.at) > PLACE),
                "{what}: face {face} still has a corner at the old vertex {}",
                corner.at
            );
        }
    }
}

#[test]
fn a_chamfer_on_the_three_edges_of_a_corner_that_is_not_square_closes_it_with_a_triangle_where_the_flats_edges_cross()
 {
    let prism = slanted_prism();
    let corner = slanted_trihedral();
    for order in ORDERS_OF_THREE {
        closes_with_a_triangle(&prism, &corner, order, 2.0, slanted_triangle(2.0));
    }
}

#[test]
fn a_fillet_on_the_three_edges_of_a_corner_that_is_not_square_closes_it_with_a_piece_of_sphere_one_radius_from_its_three_faces()
 {
    let prism = slanted_prism();
    let corner = slanted_trihedral();
    for order in ORDERS_OF_THREE {
        closes_with_a_sphere(
            &prism,
            &corner,
            order,
            2.0,
            slanted_sphere(2.0),
            reach_of(&prism),
        );
    }
}

#[test]
fn a_chamfer_on_the_three_edges_of_a_corner_where_no_two_edges_are_square_closes_it_with_a_triangle_where_the_flats_edges_cross()
 {
    let block = tilted_block();
    let corner = tilted_trihedral();
    let promised = triangle_by_booleans(&block, &corner, 2.0);
    for order in ORDERS_OF_THREE {
        closes_with_a_triangle(&block, &corner, order, 2.0, promised);
    }
}

#[test]
fn a_fillet_on_the_three_edges_of_a_corner_where_no_two_edges_are_square_closes_it_with_a_piece_of_sphere_one_radius_from_its_three_faces()
 {
    let block = tilted_block();
    let corner = tilted_trihedral();
    let promised = sphere_by_booleans(&block, &corner, 2.0);
    for order in ORDERS_OF_THREE {
        closes_with_a_sphere(&block, &corner, order, 2.0, promised, tilted_reach());
    }
}

/// The block's eight corners, each with its faces square to X, to Y and to
/// Z: across from them its edges along X, along Y and along Z.
fn block_corners() -> Vec<Trihedral> {
    let mut corners = Vec::new();
    for (x, x_face, x_far, x_way) in [
        (0.0, LEFT, RIGHT, -DVec3::X),
        (LENGTH, RIGHT, LEFT, DVec3::X),
    ] {
        for (y, y_face, y_far, y_way) in [
            (0.0, FRONT, BACK, -DVec3::Y),
            (BREADTH, BACK, FRONT, DVec3::Y),
        ] {
            for (z, z_face, z_far, z_way) in
                [(0.0, FLOOR, TOP, -DVec3::Z), (HEIGHT, TOP, FLOOR, DVec3::Z)]
            {
                corners.push(Trihedral {
                    at: DVec3::new(x, y, z),
                    faces: [x_face, y_face, z_face],
                    outward: [x_way, y_way, z_way],
                    called: ["its edge along X", "its edge along Y", "its edge along Z"],
                    length: [LENGTH, BREADTH, HEIGHT],
                    far: [x_far, y_far, z_far],
                });
            }
        }
    }
    corners
}

/// The two faces an edge lies between, the smaller first.
fn sorted([one, other]: [usize; 2]) -> [usize; 2] {
    [one.min(other), one.max(other)]
}

/// The corners among `corners` three edges of `list` meet at, each with the
/// ranks of its three edges in the list, from the lowest: in the order a step
/// numbers the patches closing them, by the lowest rank, then the next.
fn closed_corners<'a>(
    list: &[[usize; 2]],
    corners: &'a [Trihedral],
) -> Vec<(&'a Trihedral, [usize; 3])> {
    let rank = |edge: [usize; 2]| list.iter().position(|&named| sorted(named) == edge);
    let mut closed: Vec<(&Trihedral, [usize; 3])> = corners
        .iter()
        .filter_map(|corner| {
            let [Some(one), Some(two), Some(three)] =
                [0, 1, 2].map(|edge| rank(sorted(corner.edge(edge).faces)))
            else {
                return None;
            };
            let mut ranks = [one, two, three];
            ranks.sort();
            Some((corner, ranks))
        })
        .collect();
    closed.sort_by_key(|(_, ranks)| *ranks);
    closed
}

/// What the block keeps once the edges of a list, `along` long summed, are
/// cut at `value` and `closed` corners closed, every other end of them meeting
/// no other edge of the list, as the module's documentation derives it.
fn rounded_box(cut: Cut, along: f64, closed: f64, value: f64) -> f64 {
    match cut {
        Cut::Flat => {
            BLOCK_VOLUME - value * value * along / 2.0 + closed * 2.0 * value.powi(3) / 3.0
        }
        Cut::Round => {
            BLOCK_VOLUME - value * value * (1.0 - PI / 4.0) * along
                + closed * value.powi(3) * (2.0 - 7.0 * PI / 12.0)
        }
    }
}

/// Holds that a chamfer at `value`, and a fillet, on the edges each of
/// `lists` names on `part` — the same edges, `along` long summed, in
/// different orders — close every one of `corners` three of them meet at,
/// leave what `kept` promises for the cut and the corners closed, number each
/// corner's patch after the edges' faces by the lowest ranks of its edges in
/// the list, and leave the same body whatever the order, face for face.
fn closes_its_corners_whatever_the_order(
    part: &Body,
    corners: &[Trihedral],
    lists: &[Vec<[usize; 2]>],
    along: f64,
    value: f64,
    kept: impl Fn(Cut, usize) -> f64,
    called: &str,
) {
    let block = part;
    let start = block.faces_end();
    for cut in [Cut::Flat, Cut::Round] {
        let kind = match cut {
            Cut::Flat => "chamfer",
            Cut::Round => "fillet",
        };
        let laid: Vec<Result<Body, Declined>> = lists
            .iter()
            .map(|list| {
                let names: Vec<EdgeName> = list
                    .iter()
                    .map(|&[one, other]| edge_between(block, one, other))
                    .collect();
                match cut {
                    Cut::Flat => block
                        .chamfer_edges(&names, value)
                        .map(|chamfered| chamfered.body),
                    Cut::Round => block
                        .fillet_edges(&names, value)
                        .map(|filleted| filleted.body),
                }
            })
            .collect();
        let declined: Vec<String> = lists
            .iter()
            .zip(&laid)
            .filter_map(|(list, laid)| {
                laid.as_ref()
                    .err()
                    .map(|declined| format!("named {list:?}: {declined:?}"))
            })
            .collect();
        assert!(
            declined.is_empty(),
            "the {kind} on {called} closes every corner whatever the order; declined:\n{}",
            declined.join("\n")
        );
        let bodies: Vec<Body> = laid.into_iter().flatten().collect();
        for (list, body) in lists.iter().zip(&bodies) {
            let what = format!("the {kind} on {called}, named {list:?}");
            let closed = closed_corners(list, corners);
            let promised = kept(cut, closed.len());
            let drawn = match cut {
                Cut::Flat => EXACT * promised,
                Cut::Round => {
                    DRAWN
                        * reach_of(block)
                        * (PI / 2.0 * value * (along + value * closed.len() as f64))
                }
            };
            sound_drawn_within(body, promised, drawn, &what);
            let laid_end = start + list.len() + closed.len();
            assert_eq!(
                faces_made(block, body),
                (start..laid_end).collect::<Vec<_>>(),
                "{what}: one face for each edge, by its rank in the list, then one patch for \
                 each corner closed"
            );
            assert_eq!(
                body.faces_end(),
                laid_end,
                "{what}: the next step numbers its own faces past the last patch"
            );
            for (index, (corner, ranks)) in closed.iter().enumerate() {
                let patch = start + list.len() + index;
                let which = format!(
                    "{what}: patch {patch}, closing the corner at {}, its edges ranked {ranks:?} \
                     — the {}th corner by the lowest ranks of its edges",
                    corner.at,
                    index + 1
                );
                match cut {
                    Cut::Flat => corners_are(
                        body,
                        patch,
                        &[0, 1, 2].map(|face| corner.crossing(face, value)),
                        &format!("{which}, a triangle where the flats' edges cross"),
                    ),
                    Cut::Round => {
                        let centre = corner.centre(value);
                        let drawn: Vec<DVec3> = body.triangles_of(patch).flatten().collect();
                        assert!(!drawn.is_empty(), "{which} is drawn");
                        for point in drawn {
                            assert!(
                                (point.distance(centre) - value).abs() < PLACE,
                                "{which}: drawn through {point}, off the sphere of {value} about \
                                 {centre}"
                            );
                        }
                    }
                }
                assert_eq!(
                    sides_met(body, patch),
                    sides(&ranks.map(|rank| start + rank)),
                    "{which} meets the three faces laid on its edges, and nothing else"
                );
            }
        }
        let (first_list, first) = (&lists[0], &bodies[0]);
        let first_closed = closed_corners(first_list, corners);
        for (list, body) in lists.iter().zip(&bodies).skip(1) {
            let what = format!(
                "the {kind} on {called}, named {list:?}, against the same named {first_list:?}"
            );
            let closed = closed_corners(list, corners);
            let to_this = |number: usize| -> usize {
                if number < start {
                    return number;
                }
                let rank = number - start;
                if rank < first_list.len() {
                    let edge = sorted(first_list[rank]);
                    return start
                        + list
                            .iter()
                            .position(|&named| sorted(named) == edge)
                            .expect("the lists name the same edges");
                }
                let at = first_closed[rank - first_list.len()].0.at;
                start
                    + list.len()
                    + closed
                        .iter()
                        .position(|(corner, _)| corner.at == at)
                        .expect("the lists close the same corners")
            };
            for face in numbers(first) {
                assert_eq!(
                    sides_renumbered(first, face, to_this),
                    sides_met(body, to_this(face)),
                    "{what}: face {face} meets what it meets in the first order"
                );
                if let Some(plane) = first.plane_of(face) {
                    corners_are(
                        body,
                        to_this(face),
                        &plane.corners,
                        &format!("{what}: face {face}, as in the first order"),
                    );
                }
            }
        }
    }
}

/// An edge of the block as the two faces it lies between.
const TOP_FRONT: [usize; 2] = [TOP, FRONT];
const TOP_RIGHT: [usize; 2] = [TOP, RIGHT];
const TOP_BACK: [usize; 2] = [TOP, BACK];
const TOP_LEFT: [usize; 2] = [TOP, LEFT];
const FLOOR_FRONT: [usize; 2] = [FLOOR, FRONT];
const FLOOR_RIGHT: [usize; 2] = [FLOOR, RIGHT];
const FLOOR_BACK: [usize; 2] = [FLOOR, BACK];
const FLOOR_LEFT: [usize; 2] = [FLOOR, LEFT];
const FRONT_LEFT: [usize; 2] = [FRONT, LEFT];
const FRONT_RIGHT: [usize; 2] = [FRONT, RIGHT];
const BACK_RIGHT: [usize; 2] = [BACK, RIGHT];
const BACK_LEFT: [usize; 2] = [BACK, LEFT];

#[test]
fn a_chamfer_and_a_fillet_on_an_upright_and_the_four_edges_at_its_two_ends_close_both_corners_whatever_the_order()
 {
    closes_its_corners_whatever_the_order(
        &block([LENGTH, BREADTH, HEIGHT]),
        &block_corners(),
        &[
            vec![TOP_FRONT, TOP_RIGHT, FLOOR_FRONT, FLOOR_RIGHT, FRONT_RIGHT],
            vec![TOP_FRONT, FRONT_RIGHT, TOP_RIGHT, FLOOR_FRONT, FLOOR_RIGHT],
            vec![FLOOR_FRONT, FRONT_RIGHT, TOP_FRONT, FLOOR_RIGHT, TOP_RIGHT],
        ],
        2.0 * LENGTH + 2.0 * BREADTH + HEIGHT,
        3.0,
        |cut, closed| {
            rounded_box(
                cut,
                2.0 * LENGTH + 2.0 * BREADTH + HEIGHT,
                closed as f64,
                3.0,
            )
        },
        "the front right upright and the four edges at its two ends",
    );
}

#[test]
fn a_chamfer_and_a_fillet_on_the_twelve_edges_of_a_block_close_its_eight_corners_into_one_body_whatever_the_order()
 {
    closes_its_corners_whatever_the_order(
        &block([LENGTH, BREADTH, HEIGHT]),
        &block_corners(),
        &[
            vec![
                TOP_FRONT,
                TOP_RIGHT,
                TOP_BACK,
                TOP_LEFT,
                FLOOR_FRONT,
                FLOOR_RIGHT,
                FLOOR_BACK,
                FLOOR_LEFT,
                FRONT_LEFT,
                FRONT_RIGHT,
                BACK_RIGHT,
                BACK_LEFT,
            ],
            vec![
                FRONT_RIGHT,
                FLOOR_BACK,
                TOP_LEFT,
                TOP_FRONT,
                BACK_LEFT,
                FLOOR_FRONT,
                TOP_RIGHT,
                FLOOR_LEFT,
                BACK_RIGHT,
                TOP_BACK,
                FRONT_LEFT,
                FLOOR_RIGHT,
            ],
        ],
        4.0 * (LENGTH + BREADTH + HEIGHT),
        3.0,
        |cut, closed| rounded_box(cut, 4.0 * (LENGTH + BREADTH + HEIGHT), closed as f64, 3.0),
        "the twelve edges of the block",
    );
}

/// The breadth of the block a full round is laid on, twice [`FULL_ROUND`]:
/// the rounds of its top's front and back edges meet across the top.
const NARROW: f64 = 6.0;

/// The radius the rounds of the top's front and back edges of the block
/// [`NARROW`] wide meet across its top at: half its breadth.
const FULL_ROUND: f64 = NARROW / 2.0;

/// The full round's pair on the block [`NARROW`] wide, the top's front and
/// back edges, and what each is called.
const PAIR: [([usize; 2], &str); 2] = [
    (TOP_FRONT, "the top's front edge"),
    (TOP_BACK, "the top's back edge"),
];

/// How far from a mitre a ray aims, on either side of it, to meet the
/// triangles of the round it aims at rather than the other's: well over the
/// drawing's tolerance on the block [`NARROW`] wide, [`DRAWN`] times its
/// reach.
const CLEAR_OF_THE_MITRE: f64 = 0.1;

/// What a plate keeps once its top's front and back edges are rounded at
/// [`FULL_ROUND`] into one full round, and the top's edges at the ends
/// `named` rounded at the same radius and mitred against it, as the module's
/// documentation derives it: the whole outline up to the full round's axis;
/// over it, the full round's half-disc along its axis from one end to the
/// other — to where the two axes cross at an end named, to the middle of the
/// wall at one not — and at each end named, what lies inside both cylinders
/// past that crossing, `4r³/3` stretched across the slant by one over the
/// sine of the angle the end wall meets the long edges at.
fn mitred_against_a_full_round(plate: &Plate, named: &[&End]) -> f64 {
    let r = FULL_ROUND;
    let reach = |end: &End| {
        if named.iter().any(|one| one.wall == end.wall) {
            end.crossing().x
        } else {
            (end.corners[0].x + end.corners[1].x) / 2.0
        }
    };
    let [right, left] = &plate.ends;
    let past_the_crossings: f64 = named
        .iter()
        .map(|end| 4.0 * r.powi(3) / (3.0 * end.outward.x.abs()))
        .sum();
    (HEIGHT - r) * plate.area()
        + PI * r * r / 2.0 * (reach(right) - reach(left))
        + past_the_crossings
}

/// The prism the full round on the top's front and back edges of the block
/// [`NARROW`] wide takes away, raised whole along X past both its ends: all
/// that stands over the top's two edges outside the half-cylinder about the
/// line [`FULL_ROUND`] under the middle of the top.
fn full_round_tool(part: &Body) -> Body {
    let far = 2.0 * FULL_ROUND + 2.0;
    let foot = HEIGHT - FULL_ROUND;
    let corners = vec![
        DVec2::new(-far, foot),
        DVec2::new(0.0, foot),
        DVec2::new(NARROW, foot),
        DVec2::new(NARROW + far, foot),
        DVec2::new(NARROW + far, HEIGHT + far),
        DVec2::new(-far, HEIGHT + far),
    ];
    let mut runs = vec![Run::Straight; corners.len()];
    runs[1] = Run::Round {
        center: DVec2::new(FULL_ROUND, foot),
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

/// A plate [`NARROW`] wide and [`HEIGHT`] high whose top's front and back
/// edges, rounded at [`FULL_ROUND`], meet across the top in one full round:
/// the part, and its two ends, the right then the left. Its left end is
/// square, at the origin; its right end's wall may stand at an angle, seen
/// from above. Raised as `cao_part` raises a step, its floor is numbered
/// nought, its top one, then its front, its right end, its back and its left
/// end, two to five, as the block's are.
struct Plate {
    part: Body,
    ends: [End; 2],
}

impl Plate {
    /// The plate whose right end's wall runs from `front`, on the front, to
    /// `back`, on the back, seen from above.
    fn with_right_end([front, back]: [DVec2; 2]) -> Plate {
        let left = [DVec2::ZERO, DVec2::new(0.0, NARROW)];
        Plate {
            part: raised(
                Contour::straight(vec![left[0], front, back, left[1]]),
                HEIGHT,
            ),
            ends: [
                End::between(RIGHT, "the top's right edge", [front, back]),
                End::between(LEFT, "the top's left edge", left),
            ],
        }
    }

    /// The area of its outline, between its two ends.
    fn area(&self) -> f64 {
        let [right, left] = &self.ends;
        let along = |end: &End| end.corners[0].x + end.corners[1].x;
        NARROW * (along(right) - along(left)) / 2.0
    }
}

/// The block 30 by [`NARROW`] by 20, both its ends square.
fn narrow_block() -> Plate {
    Plate::with_right_end([DVec2::new(LENGTH, 0.0), DVec2::new(LENGTH, NARROW)])
}

/// How long the back of [`slanted_plate`] is, its front being [`LENGTH`]:
/// its right end's wall leans back by half the plate's breadth across it, its
/// two corners at 63.4 and 116.6 degrees, both convex.
const SLANTED_BACK: f64 = LENGTH - NARROW / 2.0;

/// The plate [`NARROW`] wide whose right end is cut at an angle seen from
/// above: its front [`LENGTH`] long and its back [`SLANTED_BACK`].
fn slanted_plate() -> Plate {
    Plate::with_right_end([DVec2::new(LENGTH, 0.0), DVec2::new(SLANTED_BACK, NARROW)])
}

/// An end of a [`Plate`] whose top edge a list may name with the full round's
/// pair: the wall there, its two corners seen from above, front then back,
/// the way out of the plate square to it, the top's edge on it, and that
/// edge's round, whole.
struct End {
    wall: usize,
    corners: [DVec2; 2],
    outward: DVec3,
    edge: EdgeAt,
    round: Round,
}

impl End {
    /// The end whose wall `wall` runs from `front` to `back`, seen from above:
    /// the top's edge on it, `called`, from its front corner to its back one,
    /// its prism reaching past the front and the back; and its round, a
    /// quarter round about the line [`FULL_ROUND`] in from the wall and under
    /// the top, from the top's way to the wall's.
    fn between(wall: usize, called: &'static str, [front, back]: [DVec2; 2]) -> End {
        let r = FULL_ROUND;
        let at = front.extend(HEIGHT);
        let run = (back - front).extend(0.0);
        let along = run.normalize();
        let middle = DVec3::new(LENGTH / 2.0, NARROW / 2.0, HEIGHT);
        let mut outward = along.cross(DVec3::Z);
        if outward.dot(at - middle) < 0.0 {
            outward = -outward;
        }
        End {
            wall,
            corners: [front, back],
            outward,
            edge: EdgeAt {
                called,
                at,
                faces: [TOP, wall],
                along,
                into: [-outward, -DVec3::Z],
                outward: [DVec3::Z, outward],
                reach: [-2.0 - r, run.length() + 2.0 + r],
            },
            round: Round {
                start: at - (outward + DVec3::Z) * r,
                along,
                length: run.length(),
                radius: r,
                facing: [DVec3::Z, outward],
            },
        }
    }

    /// Where the end edge's round's axis crosses the full round's, under the
    /// middle of the top.
    fn crossing(&self) -> DVec3 {
        let start = self.round.start;
        start + self.round.along * ((FULL_ROUND - start.y) / self.round.along.y)
    }

    /// How far a point stands past the mitre at this end, on the side of the
    /// end edge's round: how far it stands out past the crossing, square to
    /// the end wall, less its distance from the middle of the top across it —
    /// at the crossing, the two distances from the two axes in the plane of
    /// the top. Nought on the two planes the two rounds meet on, through the
    /// line square to both axes where they cross, halving the angles between
    /// them.
    fn past_the_mitre(&self, point: DVec3) -> f64 {
        (point - self.crossing()).dot(self.outward) - (point.y - FULL_ROUND).abs()
    }

    /// The crown, where the two arcs of the mitre meet: on the top's middle
    /// line, over the crossing.
    fn crown(&self) -> DVec3 {
        self.crossing() + DVec3::Z * FULL_ROUND
    }

    /// The end wall's two top corners, front then back, one radius under the
    /// top, where the two arcs of the mitre start.
    fn top_corners(&self) -> [DVec3; 2] {
        self.corners
            .map(|corner| corner.extend(HEIGHT - FULL_ROUND))
    }

    /// The end wall's two corners on the floor, front then back.
    fn foot_corners(&self) -> [DVec3; 2] {
        self.corners.map(|corner| corner.extend(0.0))
    }
}

/// What the block [`NARROW`] wide keeps once the full round and the rounds of
/// the top's edges at `ends` are laid at [`FULL_ROUND`], computed apart from
/// the step: the full round's prism, then each end edge's, raised whole along
/// its line, taken away by the booleans.
fn mitred_against_a_full_round_by_booleans(part: &Body, ends: &[&End]) -> f64 {
    let rounded = part
        .difference(&full_round_tool(part))
        .expect("the full round's prism cuts the block");
    ends.iter()
        .fold(rounded, |cut, end| {
            cut.difference(&end.edge.tool(&cut, Cut::Round, FULL_ROUND))
                .expect("an end edge's round's prism cuts the body")
        })
        .volume()
}

/// The full round of the block [`NARROW`] wide, whole: two quarter rounds
/// about the line [`FULL_ROUND`] under the middle of its top, from the front
/// up to the top and from the top down to the back.
fn full_round() -> [Round; 2] {
    [[-DVec3::Y, DVec3::Z], [DVec3::Z, DVec3::Y]].map(|facing| Round {
        start: DVec3::new(0.0, FULL_ROUND, HEIGHT - FULL_ROUND),
        along: DVec3::X,
        length: LENGTH,
        radius: FULL_ROUND,
        facing,
    })
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

/// Holds that the fillet asked `asked` on the full round's pair of `plate`
/// and on the top's edges at its `ends`, in every order the list can name
/// them in, reports what `reported` holds of the value asked and the value
/// taken, and mitres each end edge's round against the full round, both at
/// [`FULL_ROUND`]: the body exact, closed, uncrossed and drawn, holding what
/// the arithmetic and the booleans promise; the full round answering to both
/// its edges' numbers and each end edge's round to its own, the top gone and
/// nothing laid at a corner; each round drawn on its own cylinder and on its
/// own side of the mitre, the two meeting by two edges, from the end wall's
/// top corners up to the crown; and the walls set back by the radius, their
/// uprights sharp.
fn mitres_each_end_against_the_full_round_whatever_the_order(
    plate: &Plate,
    ends: &[&End],
    asked: f64,
    reported: impl Fn(f64, f64, &str),
) {
    let r = FULL_ROUND;
    let narrow = &plate.part;
    let start = narrow.faces_end();
    let count = PAIR.len() + ends.len();
    let promised = mitred_against_a_full_round(plate, ends);
    let by_booleans = mitred_against_a_full_round_by_booleans(narrow, ends);
    assert!(
        (by_booleans - promised).abs() <= EXACT * promised,
        "the full round's prism and the end edges' rounds', cut from the block by the \
         booleans, leave {by_booleans}, where arithmetic promises {promised}"
    );
    let named: Vec<(EdgeName, &str)> = PAIR
        .iter()
        .map(|&([one, other], called)| (edge_between(narrow, one, other), called))
        .chain(
            ends.iter()
                .map(|end| (end.edge.name(narrow), end.edge.called)),
        )
        .collect();
    let orders = every_order(count);
    let called = |order: &[usize]| {
        order
            .iter()
            .map(|&edge| named[edge].1)
            .collect::<Vec<_>>()
            .join(", then ")
    };

    let laid: Vec<_> = orders
        .iter()
        .map(|order| {
            let names: Vec<EdgeName> = order.iter().map(|&edge| named[edge].0.clone()).collect();
            narrow.fillet_edges(&names, asked)
        })
        .collect();

    let declined: Vec<String> = orders
        .iter()
        .zip(&laid)
        .filter_map(|(order, laid)| {
            laid.as_ref()
                .err()
                .map(|declined| format!("{}: {declined:?}", called(order)))
        })
        .collect();
    assert!(
        declined.is_empty(),
        "the fillet asked {asked} on the top's front and back edges of a plate {NARROW} wide, a \
         full round, and on the top's edge at {} of its ends mitres each end edge's round \
         against the full round, whichever edge the list names first; declined:\n{}",
        ends.len(),
        declined.join("\n")
    );
    let [front_half, back_half] = full_round();
    let area =
        front_half.area() + back_half.area() + ends.iter().map(|end| end.round.area()).sum::<f64>();
    for (order, laid) in orders.iter().zip(laid) {
        let what = format!("the fillet asked {asked} on {}", called(order));
        let filleted = laid.expect("declines are held above");
        reported(filleted.asked, filleted.taken, &what);
        let body = &filleted.body;
        sound_drawn_within(body, promised, DRAWN * reach_of(narrow) * area, &what);
        let rank_of = |edge: usize| {
            start
                + order
                    .iter()
                    .position(|&named| named == edge)
                    .expect("every edge is named")
        };
        let mut pair = [rank_of(0), rank_of(1)];
        pair.sort();
        let laid_at: Vec<usize> = (PAIR.len()..count).map(rank_of).collect();
        assert!(
            !body.has_face(TOP),
            "{what}: the top is gone, the full round over it"
        );
        let mut kept_and_laid = numbers(narrow);
        kept_and_laid.remove(&TOP);
        kept_and_laid.extend(start..start + count);
        assert_eq!(
            numbers(body),
            kept_and_laid,
            "{what}: every face of the block but its top, the full round answering to {pair:?}, \
             each end edge's round to its own rank, {laid_at:?}, and no face laid at the corners"
        );
        assert_eq!(
            body.faces_end(),
            start + count,
            "{what}: the next step numbers its faces past the {count} the list gave its edges"
        );
        one_curved_face_answers_to(body, &pair, &format!("{what}: the full round"));

        let full_drawn: Vec<DVec3> = body.triangles_of(pair[0]).flatten().collect();
        for &point in &full_drawn {
            assert!(
                front_half.holds(point) || back_half.holds(point),
                "{what}: the full round is drawn through {point}, off its half-cylinder of {r} \
                 about the line {r} under the middle of the top"
            );
            for end in ends {
                assert!(
                    end.past_the_mitre(point) <= PLACE,
                    "{what}: the full round is drawn through {point}, past the mitre where the \
                     round of {} meets it",
                    end.edge.called
                );
            }
        }
        for half in [&front_half, &back_half] {
            met_from_outside_where(
                body,
                pair[0],
                half,
                |aim| {
                    ends.iter()
                        .all(|end| end.past_the_mitre(aim) < -CLEAR_OF_THE_MITRE)
                },
                &format!("{what}: the full round, short of the mitres"),
            );
        }
        let full: &[usize] = &pair;
        let mut met_by_full: Vec<Vec<usize>> = vec![vec![FRONT], vec![BACK]];
        for wall in [RIGHT, LEFT] {
            match ends.iter().position(|end| end.wall == wall) {
                Some(at) => met_by_full.extend([vec![laid_at[at]], vec![laid_at[at]]]),
                None => met_by_full.push(vec![wall]),
            }
        }
        let met_by_full: Vec<&[usize]> = met_by_full.iter().map(Vec::as_slice).collect();
        assert_eq!(
            sides_met_by(body, full),
            sides_answering(&met_by_full),
            "{what}: the full round meets the front and the back along its sides, and at each \
             end the wall there, or the end edge's round along the two arcs of the mitre, by one \
             edge each"
        );

        for (end, &number) in ends.iter().zip(&laid_at) {
            let which = format!("the round of {}", end.edge.called);
            assert!(
                !body.is_flat(number) && body.plane_of(number).is_none(),
                "{what}: {which}, face {number}, is curved, and offers no plane to draw on"
            );
            let end_drawn: Vec<DVec3> = body.triangles_of(number).flatten().collect();
            assert!(!end_drawn.is_empty(), "{what}: {which} is drawn");
            for &point in &end_drawn {
                assert!(
                    end.round.holds(point),
                    "{what}: {which} is drawn through {point}, off its quarter-cylinder of {r} \
                     about the line {r} in from the end and under the top"
                );
                assert!(
                    end.past_the_mitre(point) >= -PLACE,
                    "{what}: {which} is drawn through {point}, past the mitre where the full \
                     round meets it"
                );
            }
            let [front_corner, back_corner] = end.top_corners();
            for (point, at) in [
                (front_corner, "the end wall's top front corner"),
                (end.crown(), "the crown"),
                (back_corner, "the end wall's top back corner"),
            ] {
                for (drawn, round) in [(&full_drawn, "the full round"), (&end_drawn, &*which)] {
                    assert!(
                        drawn.iter().any(|corner| corner.distance(point) < PLACE),
                        "{what}: {round} is drawn up to {at}, {point}, where an arc of the \
                         mitre ends"
                    );
                }
            }
            met_from_outside_where(
                body,
                number,
                &end.round,
                |aim| end.past_the_mitre(aim) > CLEAR_OF_THE_MITRE,
                &format!("{what}: {which}, past the mitre"),
            );
            assert_eq!(
                sides_met(body, number),
                sides_answering(&[&[end.wall], full, full]),
                "{what}: {which} meets the end wall along its foot and the full round along the \
                 two arcs of the mitre, and no other face"
            );
            let [front_foot, back_foot] = end.foot_corners();
            corners_are(
                body,
                end.wall,
                &[front_foot, back_foot, back_corner, front_corner],
                &format!(
                    "{what}: the end wall under {which}, set back by the radius from the top, \
                     its two top corners where the arcs of the mitre start"
                ),
            );
            assert_eq!(
                sides_met(body, end.wall),
                sides_answering(&[&[FLOOR], &[FRONT], &[BACK], &[number]]),
                "{what}: the end wall under {which} meets the floor, the front and the back \
                 along its uprights, and that round"
            );
        }
        for (wall, side, which) in [(FRONT, 0, "the front"), (BACK, 1, "the back")] {
            let [right, left] = &plate.ends;
            let [right_foot, left_foot] = [right, left].map(|end| end.foot_corners()[side]);
            let [right_top, left_top] = [right, left].map(|end| end.top_corners()[side]);
            corners_are(
                body,
                wall,
                &[left_foot, right_foot, right_top, left_top],
                &format!("{what}: {which}, up to where the full round takes over"),
            );
            assert_eq!(
                sides_met(body, wall),
                sides_answering(&[&[FLOOR], &[LEFT], &[RIGHT], full]),
                "{what}: {which} meets the floor, the two walls and the full round, by one edge \
                 each, and no end edge's round"
            );
            for end in ends {
                assert_eq!(
                    edges_between(body, wall, end.wall).len(),
                    1,
                    "{what}: the upright where {which} meets the wall under the round of {} is \
                     still one sharp edge, from the floor up to the mitre",
                    end.edge.called
                );
            }
        }
    }
}

#[test]
fn a_fillet_on_a_full_round_pair_and_an_edge_meeting_it_at_a_corner_mitres_that_edge_s_round_against_the_full_round_whatever_the_order()
 {
    let plate = narrow_block();
    let [right, _] = &plate.ends;
    mitres_each_end_against_the_full_round_whatever_the_order(
        &plate,
        &[right],
        FULL_ROUND,
        |asked, taken, what| taken_as_asked(asked, taken, FULL_ROUND, what),
    );
}

#[test]
fn a_fillet_on_the_four_top_edges_of_a_block_6_wide_mitres_its_full_round_against_both_end_edges_whatever_the_order()
 {
    let plate = narrow_block();
    let [right, left] = &plate.ends;
    mitres_each_end_against_the_full_round_whatever_the_order(
        &plate,
        &[right, left],
        FULL_ROUND,
        |asked, taken, what| taken_as_asked(asked, taken, FULL_ROUND, what),
    );
}

/// What is asked of the full round's pair and the edge at its end on the
/// block [`NARROW`] wide, more than [`FULL_ROUND`], where the pair's
/// set-backs meet: less than the end edge alone takes.
const MORE_THAN_FITS: f64 = 4.0;

#[test]
fn a_fillet_on_a_full_round_pair_and_an_edge_meeting_it_at_a_corner_asked_more_than_fits_lays_both_at_the_full_round_s_radius_and_reports_both()
 {
    let plate = narrow_block();
    let [right, _] = &plate.ends;
    let alone = plate
        .part
        .fillet(&right.edge.name(&plate.part), MORE_THAN_FITS)
        .unwrap_or_else(|declined| panic!("the top's right edge alone: declined, {declined:?}"));
    taken_as_asked(
        alone.asked,
        alone.taken,
        MORE_THAN_FITS,
        "the fillet on the top's right edge alone, whose round the full round's radius does not \
         bound",
    );

    mitres_each_end_against_the_full_round_whatever_the_order(
        &plate,
        &[right],
        MORE_THAN_FITS,
        |asked, taken, what| {
            assert!(
                (asked - MORE_THAN_FITS).abs() < PLACE,
                "{what}: reports {asked} asked, where {MORE_THAN_FITS} was"
            );
            assert!(
                (taken - FULL_ROUND).abs() < PLACE,
                "{what}: took {taken}, where the full round's pair gives {FULL_ROUND}, and the \
                 edge at its end is laid at that one value"
            );
        },
    );
}

/// One of the kernel's hairs on the block [`LENGTH`] long: twenty
/// tolerances, a tolerance a billionth of the reach.
const ONE_HAIR: f64 = 20.0 * 1e-9 * LENGTH;

/// How short a strip left between two set-backs on a face may be and still
/// read as none to the kernel, on the block [`LENGTH`] long: four of its
/// hairs.
const MEETING_HAIR: f64 = 4.0 * ONE_HAIR;

#[test]
fn a_fillet_on_a_full_round_pair_and_an_edge_meeting_it_at_a_corner_asked_a_hair_under_the_full_round_s_radius_lays_both_at_that_radius_meeting_at_the_crown()
 {
    let plate = narrow_block();
    let [right, _] = &plate.ends;

    for (under, past_one_hair) in [(1e-6, true), (1e-7, false)] {
        let asked = FULL_ROUND - under;
        let strip = NARROW - 2.0 * asked;
        assert!(
            strip < MEETING_HAIR && (strip > ONE_HAIR) == past_one_hair,
            "asked {asked}: the strip of top the pair's rounds leave, {strip}, is shorter than the \
             kernel keeps, {MEETING_HAIR}, and longer than one of its hairs, {ONE_HAIR}, or not, \
             as {past_one_hair} says"
        );

        mitres_each_end_against_the_full_round_whatever_the_order(
            &plate,
            &[right],
            asked,
            |reported, taken, what| {
                assert!(
                    (reported - asked).abs() < PLACE && (taken - asked).abs() <= MEETING_HAIR,
                    "{what}: asked {reported} and took {taken}, where {asked} is a hair from the \
                     full round's radius, {FULL_ROUND}"
                );
            },
        );
    }
}

#[test]
fn a_fillet_on_a_full_round_pair_and_an_end_edge_cut_at_an_angle_seen_from_above_mitres_the_end_edge_s_round_against_the_full_round_in_either_order()
 {
    let plate = slanted_plate();
    let [right, _] = &plate.ends;
    mitres_each_end_against_the_full_round_whatever_the_order(
        &plate,
        &[right],
        FULL_ROUND,
        |asked, taken, what| taken_as_asked(asked, taken, FULL_ROUND, what),
    );
}

/// A block 30 by 10 by 20 with a pocket 20 long, [`NARROW`] wide and
/// [`NARROW`] deep cut into the middle of its top: its floor numbered six,
/// its walls, front, right, back and left, eight to eleven, the tool's top,
/// above the block, gone. The floor's front and back edges meet inwards,
/// facing each other across the floor, and filled at [`FULL_ROUND`] meet in
/// one round bottom.
fn narrow_pocket() -> Body {
    let solid = block([LENGTH, BREADTH, HEIGHT]);
    let tool = tool_between(
        &solid,
        [DVec2::new(5.0, 2.0), DVec2::new(25.0, 2.0 + NARROW)],
        HEIGHT - NARROW,
        NARROW + 1.0,
    );
    solid.difference(&tool).expect("the pocket is cut")
}

/// The numbers [`narrow_pocket`]'s floor and the walls whose floor edges a
/// round bottom and its end take answer to.
const POCKET_FLOOR: usize = 6;
const POCKET_FRONT: usize = 8;
const POCKET_RIGHT: usize = 9;
const POCKET_BACK: usize = 10;

#[test]
fn a_fillet_on_a_full_round_meeting_an_upright_or_another_full_round_and_on_a_round_bottom_meeting_the_edge_at_its_end_is_declined_as_unsupported_whatever_the_order()
 {
    let narrow = block([LENGTH, NARROW, HEIGHT]);
    let square = block([NARROW, NARROW, HEIGHT]);
    let pocket = narrow_pocket();
    let cases = [
        (
            "the full round of the top's front and back edges and the front's right edge, the \
             upright at its end, until #837",
            &narrow,
            [TOP_FRONT, TOP_BACK],
            vec![FRONT_RIGHT],
        ),
        (
            "the full round of the top's front and back edges, the top's right edge and the two \
             uprights at that end, until #837",
            &narrow,
            [TOP_FRONT, TOP_BACK],
            vec![TOP_RIGHT, FRONT_RIGHT, BACK_RIGHT],
        ),
        (
            "the round bottom of the pocket's floor edges on its front and back walls, and its \
             floor edge on its right wall, until #838",
            &pocket,
            [[POCKET_FLOOR, POCKET_FRONT], [POCKET_FLOOR, POCKET_BACK]],
            vec![[POCKET_FLOOR, POCKET_RIGHT]],
        ),
        (
            "the four top edges of a block 6 by 6, two full rounds meeting at its four corners",
            &square,
            [TOP_FRONT, TOP_BACK],
            vec![TOP_RIGHT, TOP_LEFT],
        ),
    ];

    let mut laid = Vec::new();
    for (called, part, pair, others) in &cases {
        let name = |[one, other]: [usize; 2]| edge_between(part, one, other);
        part.fillet_edges(&pair.map(name), FULL_ROUND)
            .unwrap_or_else(|declined| {
                panic!("{called}: the pair alone is laid, a full round or a round bottom; declined, {declined:?}")
            });
        let list: Vec<[usize; 2]> = pair.iter().chain(others).copied().collect();
        for order in every_order(list.len()) {
            let faces: Vec<[usize; 2]> = order.iter().map(|&edge| list[edge]).collect();
            let names: Vec<EdgeName> = faces.iter().map(|&edge| name(edge)).collect();
            let filleted = part.fillet_edges(&names, FULL_ROUND).map(|_| ());
            laid.push((format!("{called}, the fillet on {faces:?}"), filleted));
        }
    }

    let not_declined: Vec<String> = laid
        .iter()
        .filter(|(_, result)| *result != Err(Declined::Unsupported))
        .map(|(what, result)| match result {
            Ok(()) => format!("{what}: a body"),
            Err(declined) => format!("{what}: declined, {declined:?}"),
        })
        .collect();
    assert!(
        not_declined.is_empty(),
        "a full round meeting an edge at a corner other than the end edge on the face it took, \
         a round bottom meeting any, and a full round meeting another, are each a broken step \
         and its reason, whatever the order, the pair alone laid; not so:\n{}",
        not_declined.join("\n")
    );
}

#[test]
fn a_chamfer_on_a_full_round_pair_s_edges_the_edge_at_their_end_and_both_uprights_there_is_declined_as_unsupported_whatever_the_order()
 {
    let narrow = block([LENGTH, NARROW, HEIGHT]);
    let name = |[one, other]: [usize; 2]| edge_between(&narrow, one, other);
    narrow
        .chamfer_edges(&[TOP_FRONT, TOP_BACK].map(name), FULL_ROUND)
        .unwrap_or_else(|declined| {
            panic!("the top's front and back edges alone are chamfered, their flats meeting in a ridge; declined, {declined:?}")
        });
    let list = [TOP_FRONT, TOP_BACK, TOP_RIGHT, FRONT_RIGHT, BACK_RIGHT];

    let not_declined: Vec<String> = every_order(list.len())
        .into_iter()
        .filter_map(|order| {
            let faces: Vec<[usize; 2]> = order.iter().map(|&edge| list[edge]).collect();
            let names: Vec<EdgeName> = faces.iter().map(|&edge| name(edge)).collect();
            match narrow.chamfer_edges(&names, FULL_ROUND) {
                Err(Declined::Unsupported) => None,
                Ok(_) => Some(format!("the chamfer on {faces:?}: a body")),
                Err(declined) => Some(format!("the chamfer on {faces:?}: declined, {declined:?}")),
            }
        })
        .collect();
    assert!(
        not_declined.is_empty(),
        "the chamfer at {FULL_ROUND} on the top's front and back edges of a block {NARROW} wide, \
         its top's right edge and the two uprights at that end is a broken step and its reason, \
         whatever the order, until #837; not so:\n{}",
        not_declined.join("\n")
    );
}

/// The face numbers of [`pointed_plate`]: its front, the two walls of its
/// point, front then back, and its back.
const POINTED_FRONT: usize = 2;
const POINT_FRONT: usize = 3;
const POINT_BACK: usize = 4;
const POINTED_BACK: usize = 5;

/// A plate [`NARROW`] wide and [`HEIGHT`] high whose right end comes to a
/// point seen from above, from the ends of its front and back, 30 long, to
/// its tip at (33, 3): its floor numbered nought, its top one, then its
/// front, the two walls of its point and its back, two to five, and its left
/// end six. Each edge of the point's top meets one of the top's front and
/// back edges, whose rounds at [`FULL_ROUND`] meet across the top in one full
/// round, and the other edge of the point.
fn pointed_plate() -> Body {
    raised(
        Contour::straight(vec![
            DVec2::new(0.0, 0.0),
            DVec2::new(LENGTH, 0.0),
            DVec2::new(LENGTH + FULL_ROUND, FULL_ROUND),
            DVec2::new(LENGTH, NARROW),
            DVec2::new(0.0, NARROW),
        ]),
        HEIGHT,
    )
}

#[test]
fn a_fillet_on_a_full_round_pair_and_the_two_edges_of_a_pointed_end_each_meeting_one_of_the_pair_mitres_no_full_round_against_them_whatever_the_order()
 {
    let pointed = pointed_plate();
    let name = |[one, other]: [usize; 2]| edge_between(&pointed, one, other);
    let pair = [[TOP, POINTED_FRONT], [TOP, POINTED_BACK]];
    let alone = pointed
        .fillet_edges(&pair.map(name), FULL_ROUND)
        .unwrap_or_else(|declined| {
            panic!("the top's front and back edges alone make a full round; declined, {declined:?}")
        });
    assert!(
        !alone.body.has_face(TOP) && (alone.taken - FULL_ROUND).abs() < PLACE,
        "the top's front and back edges alone make a full round of {FULL_ROUND}, the top gone: \
         took {}",
        alone.taken
    );
    let list = [pair[0], pair[1], [TOP, POINT_FRONT], [TOP, POINT_BACK]];

    let mitred: Vec<String> = every_order(list.len())
        .into_iter()
        .filter_map(|order| {
            let faces: Vec<[usize; 2]> = order.iter().map(|&edge| list[edge]).collect();
            let names: Vec<EdgeName> = faces.iter().map(|&edge| name(edge)).collect();
            match pointed.fillet_edges(&names, FULL_ROUND) {
                Err(Declined::Unsupported) => None,
                Ok(filleted)
                    if filleted.body.has_face(TOP)
                        && filleted.taken < FULL_ROUND - MEETING_HAIR =>
                {
                    None
                }
                Ok(filleted) => Some(format!(
                    "the fillet on {faces:?}: a body of {}, taken {}, the top {}",
                    filleted.body.volume(),
                    filleted.taken,
                    if filleted.body.has_face(TOP) {
                        "kept"
                    } else {
                        "gone"
                    }
                )),
                Err(declined) => Some(format!("the fillet on {faces:?}: declined, {declined:?}")),
            }
        })
        .collect();
    assert!(
        mitred.is_empty(),
        "the fillet at {FULL_ROUND} on the top's front and back edges of a plate {NARROW} wide and \
         on the two edges of its pointed end, each meeting one edge of the pair, mitres no full \
         round against them, whatever the order, until #843: it is a broken step and its reason, \
         or a body whose value stops short of the full round, the top kept; not so:\n{}",
        mitred.join("\n")
    );
}

/// What the block [`NARROW`] wide keeps once its top's front and back edges
/// are chamfered at [`FULL_ROUND`], their flats meeting in a ridge along the
/// middle of the top, and its top's right edge chamfered at the same distance
/// and mitred against both, as the module's documentation derives it.
fn ridge_mitred_against_the_end_flat() -> f64 {
    let d = FULL_ROUND;
    LENGTH * NARROW * HEIGHT - d * d * (LENGTH - d) - 4.0 * d.powi(3) / 3.0
}

#[test]
fn a_chamfer_on_a_full_round_pair_s_edges_and_an_edge_meeting_them_at_a_corner_mitres_their_ridge_against_that_edge_s_flat_whatever_the_order()
 {
    let d = FULL_ROUND;
    let plate = narrow_block();
    let narrow = &plate.part;
    let start = narrow.faces_end();
    let [right, _] = &plate.ends;
    let named: Vec<(EdgeName, &str)> = PAIR
        .iter()
        .map(|&([one, other], called)| (edge_between(narrow, one, other), called))
        .chain([(right.edge.name(narrow), right.edge.called)])
        .collect();
    let promised = ridge_mitred_against_the_end_flat();
    let ridge_s_end = DVec3::new(LENGTH - d, d, HEIGHT);
    let [front_corner, back_corner] = right.top_corners();

    for order in every_order(3) {
        let called: Vec<&str> = order.iter().map(|&edge| named[edge].1).collect();
        let what = format!("the chamfer at {d} on {}", called.join(", then "));
        let names: Vec<EdgeName> = order.iter().map(|&edge| named[edge].0.clone()).collect();

        let chamfered = narrow
            .chamfer_edges(&names, d)
            .unwrap_or_else(|declined| panic!("{what}: declined, {declined:?}"));

        taken_as_asked(chamfered.asked, chamfered.taken, d, &what);
        let body = &chamfered.body;
        sound_drawn_within(body, promised, EXACT * promised, &what);
        let rank_of = |edge: usize| {
            start
                + order
                    .iter()
                    .position(|&named| named == edge)
                    .expect("every edge is named")
        };
        let [front, back, end] = [0, 1, 2].map(rank_of);
        assert!(
            !body.has_face(TOP),
            "{what}: the top is gone, the ridge along its middle"
        );
        let mut kept_and_laid = numbers(narrow);
        kept_and_laid.remove(&TOP);
        kept_and_laid.extend(start..start + 3);
        assert_eq!(
            numbers(body),
            kept_and_laid,
            "{what}: every face of the block but its top, one flat for each edge, numbered in the \
             order the list names them, and no face laid at the corners"
        );
        for (flat, corners, facing, met, which) in [
            (
                front,
                vec![
                    DVec3::new(0.0, 0.0, HEIGHT - d),
                    front_corner,
                    ridge_s_end,
                    DVec3::new(0.0, d, HEIGHT),
                ],
                (DVec3::Z - DVec3::Y).normalize(),
                [FRONT, LEFT, back, end],
                "the front edge's flat, from the front up to the ridge",
            ),
            (
                back,
                vec![
                    DVec3::new(0.0, NARROW, HEIGHT - d),
                    back_corner,
                    ridge_s_end,
                    DVec3::new(0.0, d, HEIGHT),
                ],
                (DVec3::Z + DVec3::Y).normalize(),
                [BACK, LEFT, front, end],
                "the back edge's flat, from the back up to the ridge",
            ),
        ] {
            corners_are(body, flat, &corners, &format!("{what}: {which}"));
            faces_toward(body, flat, facing, &format!("{what}: {which}"));
            assert_eq!(
                sides_met(body, flat),
                sides(&met),
                "{what}: {which} meets its wall, the left wall, the other flat along the ridge, \
                 and the right edge's flat along the mitre, by one edge each"
            );
        }
        corners_are(
            body,
            end,
            &[front_corner, back_corner, ridge_s_end],
            &format!(
                "{what}: the right edge's flat, a triangle from the right wall's two top corners \
                 up to the ridge's end"
            ),
        );
        faces_toward(
            body,
            end,
            (DVec3::Z + DVec3::X).normalize(),
            &format!("{what}: the right edge's flat"),
        );
        assert_eq!(
            sides_met(body, end),
            sides(&[RIGHT, front, back]),
            "{what}: the right edge's flat meets the right wall and the two flats of the ridge, \
             by one edge each"
        );
        corners_are(
            body,
            RIGHT,
            &[
                DVec3::new(LENGTH, 0.0, 0.0),
                DVec3::new(LENGTH, NARROW, 0.0),
                back_corner,
                front_corner,
            ],
            &format!("{what}: the right wall, set back by the distance from the top"),
        );
        for (wall, y, flat, which) in [
            (FRONT, 0.0, front, "the front"),
            (BACK, NARROW, back, "the back"),
        ] {
            corners_are(
                body,
                wall,
                &[
                    DVec3::new(0.0, y, 0.0),
                    DVec3::new(LENGTH, y, 0.0),
                    DVec3::new(LENGTH, y, HEIGHT - d),
                    DVec3::new(0.0, y, HEIGHT - d),
                ],
                &format!("{what}: {which}, up to its edge's flat"),
            );
            assert_eq!(
                sides_met(body, wall),
                sides(&[FLOOR, LEFT, RIGHT, flat]),
                "{what}: {which} meets the floor, the two walls and its edge's flat, by one edge \
                 each"
            );
            assert_eq!(
                edges_between(body, wall, RIGHT).len(),
                1,
                "{what}: the upright where {which} meets the right wall is still one sharp edge"
            );
        }
    }
}

#[test]
fn a_chamfer_and_a_fillet_mitre_two_edges_whose_corner_s_uncut_edge_runs_into_an_earlier_round_at_its_far_end()
 {
    let value = 3.0;
    let block = block([LENGTH, BREADTH, HEIGHT]);
    let rounded = block
        .fillet(&edge_between(&block, FLOOR, FRONT), value)
        .unwrap_or_else(|declined| panic!("the floor's front edge alone: declined, {declined:?}"))
        .body;
    let earlier = value * value * (1.0 - PI / 4.0) * LENGTH;
    let start = rounded.faces_end();
    let names = [TOP_FRONT, TOP_RIGHT].map(|[one, other]| edge_between(&rounded, one, other));

    for (cut, promised) in [
        (
            Cut::Flat,
            mitred_by_flats([LENGTH, BREADTH], value) - earlier,
        ),
        (
            Cut::Round,
            mitred_by_rounds([LENGTH, BREADTH], value) - earlier,
        ),
    ] {
        let what = format!(
            "the {} on the top's front and right edges, the floor's front edge rounded before",
            match cut {
                Cut::Flat => "chamfer",
                Cut::Round => "fillet",
            }
        );

        let laid = match cut {
            Cut::Flat => rounded
                .chamfer_edges(&names, value)
                .map(|chamfered| chamfered.body),
            Cut::Round => rounded
                .fillet_edges(&names, value)
                .map(|filleted| filleted.body),
        }
        .unwrap_or_else(|declined| panic!("{what}: declined, {declined:?}"));

        let drawn = match cut {
            Cut::Flat => DRAWN * reach_of(&block) * PI / 2.0 * value * LENGTH,
            Cut::Round => DRAWN * reach_of(&block) * PI / 2.0 * value * (2.0 * LENGTH + BREADTH),
        };
        sound_drawn_within(&laid, promised, drawn, &what);
        assert_eq!(
            faces_made(&rounded, &laid),
            vec![start, start + 1],
            "{what}: one face for each edge, and none at the corner"
        );
        assert_eq!(
            edges_between(&laid, FRONT, RIGHT).len(),
            1,
            "{what}: the front's right edge is still one sharp edge"
        );
        corners_are(
            &laid,
            FRONT,
            &[
                DVec3::new(0.0, 0.0, value),
                DVec3::new(LENGTH, 0.0, value),
                DVec3::new(LENGTH, 0.0, HEIGHT - value),
                DVec3::new(0.0, 0.0, HEIGHT - value),
            ],
            &format!(
                "{what}: the front, from the earlier round up to the cut, its right edge ending \
                 one value short of the corner"
            ),
        );
    }
}

/// The slanted prism's corner where its end, its top and its left side
/// meet, at (30, 2, 10): across from those faces the top's edge on the end,
/// running to the right side, the top's edge on the left side, and the
/// slanted upright where the left side meets the end.
fn left_slanted_trihedral() -> Trihedral {
    Trihedral {
        at: DVec3::new(LENGTH, 2.0, 10.0),
        faces: [SLANTED_LEFT, SLANTED_END, SLANTED_TOP],
        outward: [DVec3::new(0.0, -10.0, 2.0).normalize(), DVec3::X, DVec3::Z],
        called: [
            "the top's edge on the end",
            "the top's edge on the left side",
            "the slanted upright where the left side meets the end",
        ],
        length: [10.0, LENGTH, 104f64.sqrt()],
        far: [SLANTED_RIGHT, SLANTED_START, SLANTED_BOTTOM],
    }
}

/// What the slanted prism keeps once its end's top edge and the two edges at
/// each of its ends are chamfered at `distance`, both corners closed by a
/// triangle, as the module's documentation derives it.
fn slanted_two_triangles(distance: f64) -> f64 {
    let d = distance;
    let [right, left] = [slant(), 10.0 / 104f64.sqrt()];
    SLANTED_VOLUME
        - (5.0 * d * d
            + 0.1 * d.powi(3)
            + 15.0 * (right + left) * d * d
            + 5.0 * d * d * (1.0 / right + 1.0 / left)
            - ((right + 1.0 / right) + (left + 1.0 / left)) * d.powi(3) / 3.0)
}

#[test]
fn a_chamfer_and_a_fillet_on_the_edges_at_both_ends_of_an_edge_whose_corners_are_not_square_close_both_whatever_the_order()
 {
    let value = 2.0;
    let prism = slanted_prism();
    let right = slanted_trihedral();
    let left = left_slanted_trihedral();
    let edges = [
        right.edge(0),
        right.edge(1),
        right.edge(2),
        left.edge(1),
        left.edge(2),
    ];
    let rounds = spheres_by_booleans(&prism, &[&right, &left], &edges, value);
    let [top_right, end_right, top_left, end_left, top_end] = [
        [SLANTED_TOP, SLANTED_RIGHT],
        [SLANTED_END, SLANTED_RIGHT],
        [SLANTED_TOP, SLANTED_LEFT],
        [SLANTED_END, SLANTED_LEFT],
        [SLANTED_TOP, SLANTED_END],
    ];

    closes_its_corners_whatever_the_order(
        &prism,
        &[right, left],
        &[
            vec![top_right, end_right, top_left, end_left, top_end],
            vec![top_end, top_right, end_right, top_left, end_left],
            vec![top_left, top_end, top_right, end_left, end_right],
        ],
        10.0 + 2.0 * LENGTH + 116f64.sqrt() + 104f64.sqrt(),
        value,
        |cut, _| match cut {
            Cut::Flat => slanted_two_triangles(value),
            Cut::Round => rounds,
        },
        "the slanted prism's end's top edge and the two edges at each of its ends",
    );
}

#[test]
fn a_chamfer_and_a_fillet_on_edges_named_before_a_join_merged_the_faces_at_their_corner_mitre_or_close_it_whatever_the_order()
 {
    let value = 3.0;
    let drawn_block = block([LENGTH, BREADTH, HEIGHT]);
    let [top_front, top_right, upright] = [TOP_FRONT, TOP_RIGHT, FRONT_RIGHT]
        .map(|[one, other]| edge_between(&drawn_block, one, other));
    let joined = joined_to(
        &drawn_block,
        [DVec2::new(-10.0, 0.0), DVec2::new(0.0, BREADTH)],
        0.0,
        HEIGHT,
    );
    let added = 10.0 * BREADTH * HEIGHT;
    let long = LENGTH + 10.0;
    let start = joined.faces_end();
    let mut cases = Vec::new();
    for cut in [Cut::Flat, Cut::Round] {
        let mitre = |lengths: [f64; 2]| match cut {
            Cut::Flat => mitred_by_flats(lengths, value) + added,
            Cut::Round => mitred_by_rounds(lengths, value) + added,
        };
        let closed = rounded_box(cut, long + BREADTH + HEIGHT, 1.0, value) + added;
        for (what, names, promised, patches) in [
            (
                "the top's front and right edges",
                vec![top_front.clone(), top_right.clone()],
                mitre([long, BREADTH]),
                0,
            ),
            (
                "the top's right and front edges",
                vec![top_right.clone(), top_front.clone()],
                mitre([long, BREADTH]),
                0,
            ),
            (
                "the top's front edge and the front's right edge",
                vec![top_front.clone(), upright.clone()],
                mitre([long, HEIGHT]),
                0,
            ),
            (
                "the front's right edge and the top's front edge",
                vec![upright.clone(), top_front.clone()],
                mitre([long, HEIGHT]),
                0,
            ),
            (
                "the three edges at the corner",
                vec![top_front.clone(), top_right.clone(), upright.clone()],
                closed,
                1,
            ),
            (
                "the three edges at the corner, the front's right edge first",
                vec![upright.clone(), top_front.clone(), top_right.clone()],
                closed,
                1,
            ),
        ] {
            cases.push((cut, what, names, promised, patches));
        }
    }

    let laid: Vec<Result<Body, Declined>> = cases
        .iter()
        .map(|(cut, _, names, _, _)| match cut {
            Cut::Flat => joined
                .chamfer_edges(names, value)
                .map(|chamfered| chamfered.body),
            Cut::Round => joined
                .fillet_edges(names, value)
                .map(|filleted| filleted.body),
        })
        .collect();

    let declined: Vec<String> = cases
        .iter()
        .zip(&laid)
        .filter_map(|((cut, what, ..), laid)| {
            laid.as_ref()
                .err()
                .map(|declined| format!("{cut:?} on {what}: {declined:?}"))
        })
        .collect();
    assert!(
        declined.is_empty(),
        "names taken before the join still find the corner's edges, whatever the order; \
         declined:\n{}",
        declined.join("\n")
    );
    for ((cut, what, names, promised, patches), laid) in cases.into_iter().zip(laid) {
        let what = format!("{cut:?} on {what}, named before the join");
        let body = laid.expect("declines are held above");
        let drawn = match cut {
            Cut::Flat => EXACT * promised,
            Cut::Round => {
                DRAWN * reach_of(&joined) * PI / 2.0 * value * (long + BREADTH + HEIGHT + value)
            }
        };
        sound_drawn_within(&body, promised, drawn, &what);
        assert_eq!(
            faces_made(&joined, &body),
            (start..start + names.len() + patches).collect::<Vec<_>>(),
            "{what}: one face for each edge, and a patch where the three are closed"
        );
    }
}

/// The block 10 by 10 by 10 cut by four planes through the middle of its top,
/// each leaning 45 degrees, down to its walls' halfway: a square pyramid 5
/// high on a box 5 high, holding `500 + 500/3`. The four planes' faces are
/// numbered six, twelve, eighteen and twenty-four, front, back, left and
/// right; four edges end at the apex, one between each two faces beside each
/// other.
fn pyramid() -> Body {
    let apex = DVec3::new(5.0, 5.0, 10.0);
    [
        DVec3::new(0.0, -1.0, 1.0),
        DVec3::new(0.0, 1.0, 1.0),
        DVec3::new(-1.0, 0.0, 1.0),
        DVec3::new(1.0, 0.0, 1.0),
    ]
    .into_iter()
    .fold(
        raised(Contour::rectangle(DVec2::ZERO, DVec2::splat(10.0)), 10.0),
        |part, away| {
            part.difference(&beyond(&part, apex, away, 50.0))
                .expect("a plane through the apex cuts the cube")
        },
    )
}

#[test]
fn a_fillet_and_a_chamfer_on_three_of_the_four_edges_at_a_pyramid_s_apex_are_declined_as_unsupported()
 {
    let pyramid = pyramid();
    assert!(
        (pyramid.volume() - (500.0 + 500.0 / 3.0)).abs() < EXACT * 1000.0,
        "the pyramid on its box holds {}",
        pyramid.volume()
    );
    let [front, back, left, right] = [6, 12, 18, 24];
    let hips = [[front, left], [front, right], [back, right], [back, left]]
        .map(|[one, other]| edge_between(&pyramid, one, other));

    for (what, list) in [
        ("three hips in turn round the apex", vec![0, 1, 2]),
        ("three hips, the last first", vec![3, 0, 1]),
        ("three hips round the other way", vec![2, 1, 3]),
        ("the four hips", vec![0, 1, 2, 3]),
    ] {
        let names: Vec<EdgeName> = list.iter().map(|&hip| hips[hip].clone()).collect();

        let filleted = pyramid.fillet_edges(&names, 1.0).err();
        let chamfered = pyramid.chamfer_edges(&names, 1.0).err();

        assert_eq!(
            filleted,
            Some(Declined::Unsupported),
            "the fillet on {what}, at a vertex four edges end at: a broken step and its \
             reason, and no body"
        );
        assert_eq!(
            chamfered,
            Some(Declined::Unsupported),
            "the chamfer on {what}, at a vertex four edges end at: a broken step and its \
             reason, and no body"
        );
    }
}

/// The block 30 by 10 by 20 with the corner at its top's front right cut off
/// by the plane through the points 0.5 from it along its three edges: a facet,
/// numbered six, an equilateral triangle of side √0.5.
fn small_facet() -> Body {
    let block = block([LENGTH, BREADTH, HEIGHT]);
    block
        .difference(&beyond(
            &block,
            DVec3::new(LENGTH - 0.5, 0.0, HEIGHT),
            DVec3::new(1.0, -1.0, 1.0),
            50.0,
        ))
        .expect("the facet is cut")
}

/// The small facet's corner on the top's front edge, at (29.5, 0, 20): its
/// faces the facet, the front and the top; across from them the top's front
/// edge, running to the left wall, the facet's edge on the top and its edge
/// on the front, both running to the right wall.
fn small_facet_trihedral() -> Trihedral {
    Trihedral {
        at: DVec3::new(LENGTH - 0.5, 0.0, HEIGHT),
        faces: [6, FRONT, TOP],
        outward: [DVec3::new(1.0, -1.0, 1.0).normalize(), -DVec3::Y, DVec3::Z],
        called: [
            "the top's front edge",
            "the facet's edge on the top",
            "the facet's edge on the front",
        ],
        length: [LENGTH - 0.5, 0.5f64.sqrt(), 0.5f64.sqrt()],
        far: [LEFT, RIGHT, RIGHT],
    }
}

#[test]
fn a_fillet_where_one_round_takes_the_next_edge_whole_lays_both_rounds_in_either_order_and_a_chamfer_declines_in_both()
 {
    let cap = 0.5;
    let part = small_facet();
    let corner = small_facet_trihedral();
    let [top_front, facet_front] = [corner.edge(0), corner.edge(2)];
    let promised = [&top_front, &facet_front]
        .iter()
        .fold(part.clone(), |cut, edge| {
            cut.difference(&edge.tool(&cut, Cut::Round, cap))
                .expect("a round's prism cuts the part")
        })
        .volume();
    let names = [top_front.name(&part), facet_front.name(&part)];
    let start = part.faces_end();

    for asked in [cap, 2.0] {
        let orders = [
            ("the top's front edge first", [0, 1]),
            ("the facet's edge first", [1, 0]),
        ];
        let laid: Vec<Body> = orders
            .iter()
            .map(|(what, order)| {
                let list = order.map(|edge| names[edge].clone());
                let filleted = part.fillet_edges(&list, asked).unwrap_or_else(|declined| {
                    panic!("the fillet asked {asked}, {what}: declined, {declined:?}")
                });
                assert!(
                    (filleted.taken - cap).abs() < PLACE,
                    "the fillet asked {asked}, {what}: took {}, where the facet's corner allows \
                     {cap}",
                    filleted.taken
                );
                filleted.body
            })
            .collect();
        for ((what, order), body) in orders.iter().zip(&laid) {
            let what = format!("the fillet asked {asked}, {what}");
            let area = PI / 2.0 * cap * (LENGTH + 1.0);
            sound_drawn_within(body, promised, DRAWN * reach_of(&part) * area, &what);
            assert_eq!(
                faces_made(&part, body),
                vec![start, start + 1],
                "{what}: one round for each edge, by its rank in the list"
            );
            for (rank, &edge) in order.iter().enumerate() {
                assert!(
                    !body.is_flat(start + rank) && body.plane_of(start + rank).is_none(),
                    "{what}: face {} is the round of {}, curved",
                    start + rank,
                    corner.called[[0, 2][edge]]
                );
            }
        }
        let swapped = |number: usize| match number {
            number if number == start => start + 1,
            number if number == start + 1 => start,
            number => number,
        };
        let [first, second] = [&laid[0], &laid[1]];
        for face in numbers(first) {
            let what = format!(
                "the fillet asked {asked}: face {face} named the top's front edge first, against \
                 the facet's edge first"
            );
            assert_eq!(
                sides_renumbered(first, face, swapped),
                sides_met(second, swapped(face)),
                "{what}: it meets the same faces"
            );
            if let Some(plane) = first.plane_of(face) {
                corners_are(second, swapped(face), &plane.corners, &what);
            }
        }

        for (what, order) in orders {
            let list = order.map(|edge| names[edge].clone());

            let chamfered = part.chamfer_edges(&list, asked).err();

            assert_eq!(
                chamfered,
                Some(Declined::Unsupported),
                "the chamfer asked {asked}, {what}: the top's front edge's flat at the cap takes \
                 the facet whole, as it does alone"
            );
        }
    }
}

/// The block 30 by 10 by 20 with the corner at its top's front right cut off
/// steeply: the facet, numbered six, through the points 2 along the top's two
/// edges and 6 down the upright. At its corner on the upright, (30, 0, 14),
/// the facet's two edges rise at wide angles to the upright, 161.6 degrees on
/// the front and on the right.
fn steep_facet() -> Body {
    let block = block([LENGTH, BREADTH, HEIGHT]);
    block
        .difference(&beyond(
            &block,
            DVec3::new(LENGTH - 2.0, 0.0, HEIGHT),
            DVec3::new(0.5, -0.5, 1.0 / 6.0),
            50.0,
        ))
        .expect("the facet is cut")
}

/// The steep facet's corner on the upright: its faces the right wall, the
/// front and the facet; across from them the facet's edge on the front, its
/// edge on the right, both rising to the top, and the upright, running down
/// to the floor.
fn steep_trihedral() -> Trihedral {
    Trihedral {
        at: DVec3::new(LENGTH, 0.0, HEIGHT - 6.0),
        faces: [RIGHT, FRONT, 6],
        outward: [
            DVec3::X,
            -DVec3::Y,
            DVec3::new(0.5, -0.5, 1.0 / 6.0).normalize(),
        ],
        called: [
            "the facet's edge on the front",
            "the facet's edge on the right",
            "the upright",
        ],
        length: [40f64.sqrt(), 40f64.sqrt(), HEIGHT - 6.0],
        far: [TOP, TOP, FLOOR],
    }
}

#[test]
fn a_fillet_on_the_three_edges_of_a_corner_whose_sphere_would_stand_behind_it_along_an_edge_is_declined_as_unsupported()
 {
    let value = 0.5;
    let part = steep_facet();
    let corner = steep_trihedral();
    let behind = (corner.centre(value) - corner.at).dot(corner.edge(2).along);
    let gradient = (0.25 + 0.25 + 1.0 / 36.0f64).sqrt();
    assert!(
        (behind - 6.0 * value * (gradient - 1.0)).abs() < PLACE && behind < 0.0,
        "the sphere's centre, one radius in from the three faces, stands {behind} along the \
         upright from the corner: behind it, where the arithmetic puts it"
    );
    let promised = triangle_by_booleans(&part, &corner, value);

    for order in ORDERS_OF_THREE {
        let names = order.map(|edge| corner.edge(edge).name(&part));

        let filleted = part.fillet_edges(&names, value).err();

        assert_eq!(
            filleted,
            Some(Declined::Unsupported),
            "the fillet on {}, {} and {}, the piece of sphere standing behind the corner along \
             the upright: a broken step and its reason, and no body",
            corner.called[order[0]],
            corner.called[order[1]],
            corner.called[order[2]]
        );
        closes_with_a_triangle(&part, &corner, order, value, promised);
    }
}
