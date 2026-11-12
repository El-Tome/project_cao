//! A chamfer and a fillet take several straight edges of an exact body in one
//! step, at one value: each edge of the list cut off, or filled in where its
//! faces meet inwards, as the step of that one edge would (#696, #697, #778),
//! and the value reduced once for the whole step, to the largest at which
//! every round or flat fits with the others laid. Two rounds whose set-backs
//! meet across the face they share are one face, a full round: their edges
//! are parallel, so the two share one axis — edges that are not parallel are
//! out of this issue's reach. An operation of the kernel on [`Body`]; the
//! step of the history that calls it comes with the application, in #701.
//!
//! Closes #698.
//! - A fillet step given the names of several straight edges of a block that
//!   share no vertex, and one radius, leaves a closed body where each edge is
//!   rounded as the fillet of that one edge alone would round it. The body
//!   passes the soundness checks. The same holds for a chamfer and one
//!   distance —
//!   `a_fillet_on_the_four_long_edges_of_a_block_rounds_each_as_its_own_fillet_alone_would`,
//!   `a_chamfer_on_the_four_long_edges_of_a_block_cuts_each_as_its_own_chamfer_alone_would`
//! - Each round, or flat, is numbered on its own, in the order the step names
//!   its edge, from where the steps' numbers ended —
//!   `each_round_and_each_flat_of_a_step_on_several_edges_is_numbered_in_the_order_the_step_names_its_edge_from_where_the_steps_numbers_ended`
//! - The value is reduced once for the whole step, to the largest at which
//!   every round fits with the others laid. The step reports the value asked
//!   and the one value taken —
//!   `a_fillet_on_several_edges_asked_more_than_one_of_them_gives_lays_every_round_at_the_one_radius_they_all_fit_and_reports_both`,
//!   `a_chamfer_on_several_edges_asked_more_than_one_of_them_gives_cuts_every_flat_at_the_one_distance_they_all_fit_and_reports_both`,
//!   each on a list whose limit is one edge's own faces, and on a list where
//!   that edge limits two set-backs on a shared face before they meet;
//!   a value that fits is taken as asked — the two tests of the first
//!   criterion, which ask a value every edge gives and hold that it is the one
//!   taken, and
//!   `a_fillet_and_a_chamfer_on_the_two_edges_either_side_of_a_face_take_a_value_that_fits_as_asked`;
//!   and, beyond the issue's wording, an outward edge and an inward one
//!   facing each other across a tread they share: the fillet takes half its
//!   depth, a round and a hollow of two axes meeting tangentially in an S —
//!   `a_fillet_on_a_tread_s_nose_and_foot_asked_more_than_half_its_depth_takes_half_a_round_and_a_hollow_meeting_in_an_s`;
//!   the chamfer, whose two flats lie in one plane there, was declined
//!   `Unsupported` here, and is one slope since #791, below;
//!   and two rounds that share no face but would meet on the wall one of
//!   them ends on are laid as asked where they do not meet —
//!   `a_fillet_on_two_edges_whose_rounds_would_meet_on_the_wall_one_ends_on_is_laid_as_asked_short_of_it`;
//!   where they would, the step was declined `Unsupported` here, and is
//!   reduced since #792, in
//!   `rounds_meeting_through_an_end_face_or_along_a_slant_reduce_the_value.rs`
//! - On a block 10 wide, rounding the two edges on either side of that face,
//!   asked more than 5, takes 5, where either alone would take more: the face
//!   is gone, and the two rounds, of one axis and one radius, are one
//!   half-cylinder tangent to both sides — a **full round** — answering to
//!   both edges' numbers. The body passes the soundness checks. A chamfer
//!   takes 5 too, and its two flats meet in a ridge —
//!   `a_fillet_on_the_two_edges_either_side_of_a_face_10_wide_asked_more_than_5_takes_5_in_one_full_round`,
//!   `a_chamfer_on_the_two_edges_either_side_of_a_face_10_wide_asked_more_than_5_takes_5_its_flats_meeting_in_a_ridge`;
//!   the same, and the round bottom of the next criterion, whichever of its
//!   two faces each edge holds the shared one as, and whichever edge the step
//!   names first: on a top, a floor and a front along X, a top along Y, a side
//!   wall upright, a slot's floor and a notch's back —
//!   `a_fillet_lays_one_full_round_or_round_bottom_whichever_of_its_two_faces_each_edge_shares_and_in_either_order`
//!   — and, beyond the issue's block, the same full round where the two sides
//!   are not square to the face and meet it at two angles, the edges still
//!   parallel: their set-backs sum to its width, and the two axes are one,
//!   whichever edge the step names first —
//!   `a_fillet_on_the_two_top_edges_of_a_prism_whose_sides_slant_takes_the_radius_whose_set_backs_meet_in_one_full_round`
//!   — also beyond its wording: a meeting pair named apart in the list still
//!   lays one full round, answering to the first's number and the third's,
//!   the edge named between them numbered between —
//!   `a_fillet_on_a_meeting_pair_named_apart_in_the_list_lays_one_full_round_answering_to_both_their_numbers_the_edge_between_numbered_between`,
//!   and a pair whose earlier edge is not named first, the edge before it
//!   numbered first —
//!   `a_fillet_on_a_meeting_pair_whose_earlier_edge_is_not_named_first_numbers_the_edge_before_it_first_and_the_full_round_after`;
//!   and an edge whose set-backs would meet two others', the four long edges
//!   of a square bar asked more than half its side, declined the fillet
//!   `Unsupported` here — rounds that close on themselves were out of the
//!   issue's scope — and is one closed cylinder since #790, below; less than
//!   half lays all four —
//!   `a_fillet_and_a_chamfer_on_the_four_long_edges_of_a_square_bar_asked_less_than_half_its_side_lay_each_at_the_value_asked`;
//!   the chamfer there takes half its side, its four flats meeting in four
//!   ridges, a diamond —
//!   `a_chamfer_on_the_four_long_edges_of_a_square_bar_asked_half_its_side_or_more_takes_half_a_diamond`;
//!   and asked a hair under 5, where the strip left between the two rounds is
//!   shorter than the kernel keeps, the fillet still lays one full round —
//!   `a_fillet_on_the_two_edges_either_side_of_a_face_10_wide_asked_a_hair_under_5_lays_one_full_round`
//! - In a slot 10 wide, filleting the two edges of its floor, asked more than
//!   5, takes 5: the floor is gone, and the two fills, of one axis and one
//!   radius, are one hollow half-cylinder — a round-bottomed slot — answering
//!   to both edges' numbers. The body passes the soundness checks. A chamfer
//!   takes 5 too, and its two flats meet in a valley —
//!   `a_fillet_on_the_two_floor_edges_of_a_slot_10_wide_asked_5_or_more_takes_5_in_one_round_bottom`,
//!   `a_chamfer_on_the_two_floor_edges_of_a_slot_10_wide_asked_more_than_5_takes_5_its_flats_meeting_in_a_valley`;
//!   beyond the issue's wording, a slot that widens two thirds of the way
//!   along, its two floor edges ending on different faces, declines the
//!   fillet `Unsupported` where their set-backs would meet —
//!   `a_fillet_on_the_two_floor_edges_of_a_slot_that_widens_past_the_end_of_one_is_declined_as_unsupported`;
//!   the chamfer there takes 5, its flats meeting in a valley along the
//!   narrow part and the floor left beyond the widening —
//!   `a_chamfer_on_the_two_floor_edges_of_a_slot_that_widens_past_the_end_of_one_takes_5_a_valley_along_its_narrow_part`
//! - A list mixing an edge whose faces meet outwards and one whose faces meet
//!   inwards cuts the first off and fills the second in, at one value —
//!   `a_fillet_and_a_chamfer_on_an_outward_edge_and_an_inward_one_cut_the_first_off_and_fill_the_second_in_at_one_value`,
//!   at a value both fit, and at one each kind limits for the other
//! - Two edges of the list that share a vertex decline the step
//!   `Unsupported`, and nothing of it is laid — no test: #699 took this
//!   back. Where the vertex is a convex corner the two now meet in a mitre,
//!   and where an edge of the corner meets inwards they are still declined,
//!   both held in `a_chamfer_and_a_fillet_blend_the_corner_their_edges_meet_at.rs`;
//!   the test that stood here, on the block's top front right corner, went
//!   with the decline. Nothing laid is what a declined step is: the step
//!   hands back its reason and no body, and the part it was given is left as
//!   it was. Two edges sharing a vertex share a face wherever three faces
//!   meet there; sharing none, they meet where four faces or more do, and
//!   each alone is declined there already — on a block with another standing
//!   on its top, over its corner, both edges at the corner where they cross —
//!   so no list reaches past the step of one edge to that case.
//! - Beyond the issue's wording: a list of no edge is declined `Unsupported` —
//!   `a_fillet_and_a_chamfer_on_a_list_of_no_edge_are_declined_as_unsupported`;
//!   and two edges that share a face but no vertex and are not parallel are
//!   laid, each as alone, where their set-backs do not meet — well short of
//!   it and just short of it —
//!   `a_fillet_and_a_chamfer_on_two_alternate_top_edges_of_a_hexagonal_prism_lay_each_as_alone_where_they_do_not_meet`;
//!   where they would, the step was declined `Unsupported` here, and is
//!   reduced since #792, in
//!   `rounds_meeting_through_an_end_face_or_along_a_slant_reduce_the_value.rs`
//! - An edge of the list its name no longer finds declines the whole step
//!   with that edge's reason (`Lost`), as for one edge —
//!   `a_fillet_and_a_chamfer_on_a_list_holding_an_edge_its_name_no_longer_finds_are_declined_with_that_edge_s_reason`.
//!   Two names of the list finding the same edge decline it `Unsupported` —
//!   `a_fillet_and_a_chamfer_on_two_names_finding_the_same_edge_are_declined_as_unsupported`
//! - After a change of size upstream, the step rounds the same edges, found by
//!   their names —
//!   `a_fillet_and_a_chamfer_on_several_edges_take_the_edges_their_names_find_again_after_a_change_of_size`
//! - A list of one edge gives the body and the value the single-edge fillet
//!   and chamfer give; their tests pass unchanged — held by those three
//!   files, left as they were: `a_fillet_rounds_one_straight_edge.rs`,
//!   `a_chamfer_cuts_one_straight_edge.rs` and
//!   `a_chamfer_and_a_fillet_fill_a_concave_edge.rs`. The step of one edge
//!   is now the step on a list of one, so they run through the list. What
//!   stands here cannot fail while that is so: it guards the day the two
//!   come apart —
//!   `a_fillet_and_a_chamfer_on_a_list_of_one_edge_give_the_body_and_the_value_the_step_of_that_edge_gives`
//!
//! Closes #791.
//! - Where a chamfer on an outward edge and one on an inward edge facing it
//!   across the face they share meet at the shared limit, the two flats are
//!   one slope answering to both edges' numbers, the shared face gone — on a
//!   stair's tread 10 deep, its nose and its foot asked half its depth and
//!   asked more, which takes half: one flat face from the nose's front to
//!   the riser, meeting those two and the stair's two ends by one edge each,
//!   the matter the nose gives up what the foot takes in, whichever of the
//!   two edges the step names first —
//!   `a_chamfer_on_a_tread_s_nose_and_foot_asked_half_its_depth_or_more_takes_half_in_one_slope_answering_to_both_their_numbers`;
//!   and asked a hair under half, where the strip of tread left between the
//!   two flats is shorter than the kernel keeps, the same slope, never a
//!   sliver of tread, as the full round lays one —
//!   `a_chamfer_on_a_tread_s_nose_and_foot_asked_a_hair_under_half_its_depth_lays_one_slope`;
//!   with an edge far from both on the same step, the top's back edge, named
//!   before them, between them or after them, the slope answers to the
//!   nose's and the foot's numbers by their ranks in the list and to no
//!   other, the far edge's flat to its own number alone, and the next step
//!   numbers its faces past all three —
//!   `a_chamfer_on_a_tread_s_nose_and_foot_and_an_edge_far_from_both_numbers_the_slope_by_their_ranks_and_the_far_flat_by_its_own_wherever_the_far_edge_is_named`;
//!   where the riser is turned by a hair about the upright, the foot
//!   parallel to the nose only as the kernel reads it, the two flats lie in
//!   one plane where it reads them and part along the tread, a sliver of it
//!   left between them, and the step was declined `Unsupported` here rather
//!   than laid so, and is one slope since #812 —
//!   `a_chamfer_on_a_tread_s_nose_and_foot_whose_riser_is_turned_by_a_hair_asked_half_its_depth_or_a_hair_under_lays_one_slope_whichever_is_named_first`;
//!   and, beyond the issue's wording, three cases left as they were, each
//!   pinned until the issue that builds it: on a stair whose front leans,
//!   where the two flats meet at an angle, the nose named first laid two
//!   flats meeting in a crease, the tread gone, each answering to its own
//!   number, and the foot named first was declined `Unsupported` here; both
//!   orders lay the crease since #812 —
//!   `a_chamfer_on_the_nose_and_foot_of_a_tread_whose_front_leans_asked_half_its_depth_or_more_takes_half_in_two_flats_meeting_in_a_crease_whichever_is_named_first`;
//!   on a flight whose treads are as deep as its risers are high, the first
//!   nose, the foot between and the second nose, the foot meeting both, were
//!   declined `Unsupported` here, and are one slope across the flight since
//!   #813 —
//!   `a_chamfer_on_a_flight_s_first_nose_the_foot_between_and_the_second_nose_where_its_treads_are_as_deep_as_its_risers_are_high_takes_half_in_one_slope_across_it_in_every_order`;
//!   and the nose and the foot with the front's upright edge at one end of
//!   the nose, which meets the slope at a corner, are declined `Unsupported`
//!   until #814 —
//!   `a_chamfer_on_a_tread_s_nose_and_foot_and_the_front_s_upright_edge_at_one_end_of_the_nose_asked_half_its_depth_or_more_is_declined_as_unsupported`
//! - Every step #698 lays, the full round and the round-bottomed slot among
//!   them — the tests of #698 above, left as they were: the full round and
//!   the round bottom in either order and on every face they are laid on —
//!   `a_fillet_lays_one_full_round_or_round_bottom_whichever_of_its_two_faces_each_edge_shares_and_in_either_order`,
//!   `a_fillet_on_the_two_edges_either_side_of_a_face_10_wide_asked_more_than_5_takes_5_in_one_full_round`,
//!   `a_fillet_on_the_two_floor_edges_of_a_slot_10_wide_asked_5_or_more_takes_5_in_one_round_bottom`;
//!   the chamfers whose flats meet in a ridge and in a valley —
//!   `a_chamfer_on_the_two_edges_either_side_of_a_face_10_wide_asked_more_than_5_takes_5_its_flats_meeting_in_a_ridge`,
//!   `a_chamfer_on_the_two_floor_edges_of_a_slot_10_wide_asked_more_than_5_takes_5_its_flats_meeting_in_a_valley`;
//!   the fillet on the same tread, an S —
//!   `a_fillet_on_a_tread_s_nose_and_foot_asked_more_than_half_its_depth_takes_half_a_round_and_a_hollow_meeting_in_an_s`;
//!   and the chamfer on it short of the limit, its two flats laid apart and
//!   each answering to its own number, a strip of tread left between —
//!   `a_chamfer_on_a_tread_s_nose_and_foot_asked_less_than_half_its_depth_lays_two_flats_apart_a_strip_of_tread_between`
//!
//! Closes #790.
//! - Where every round of a step meets its neighbours across every face they
//!   share, the rounds are one closed cylinder: the bar's four long faces are
//!   gone, its two end walls are discs, and the cylinder answers to every
//!   edge's number — on the four long edges of a bar 10 square, asked half
//!   its side and asked more, named in each of their 24 orders: one curved
//!   face, the cylinder of half the side about the bar's axis, each end a
//!   flat face bounded by one circle meeting the cylinder alone, and the
//!   matter of that cylinder —
//!   `a_fillet_on_the_four_long_edges_of_a_square_bar_asked_half_its_side_or_more_takes_half_in_one_closed_cylinder_in_every_order`;
//!   and the same on a bar standing askew, away from the origin, so that its
//!   axis is found from the bar rather than from the frame it is drawn on —
//!   `a_fillet_on_the_four_long_edges_of_a_square_bar_standing_askew_away_from_the_origin_takes_half_in_one_closed_cylinder_about_its_own_axis`;
//!   and, beyond the issue's wording, any prism whose long faces all stand
//!   tangent to one circle closes the same way, at that circle's radius, in
//!   every order: a regular triangle, a rhombus of 60 degrees, a regular
//!   hexagon of six edges, and the square bar with one end cut at 20 degrees,
//!   that end an ellipse and the matter the circle's area times the length
//!   along the axis —
//!   `a_fillet_on_the_three_long_edges_of_a_regular_triangular_prism_asked_its_inradius_or_more_takes_it_in_one_closed_cylinder`,
//!   `a_fillet_on_the_four_long_edges_of_a_60_degree_rhombic_prism_asked_its_inradius_or_more_takes_it_in_one_closed_cylinder`,
//!   `a_fillet_on_the_six_long_edges_of_a_regular_hexagonal_bar_asked_its_inradius_or_more_takes_it_in_one_closed_cylinder`,
//!   `a_fillet_on_the_four_long_edges_of_a_square_bar_with_one_end_cut_at_20_degrees_takes_half_in_one_closed_cylinder_its_slanted_end_an_ellipse`;
//!   where an end is not the rounds' alone, the step is still declined
//!   `Unsupported` from half the side, in every order, and less lays four
//!   rounds, as before #790 — a post standing on a plate, a bar with a boss
//!   on one end, and a tube bored through —
//!   `a_fillet_on_the_four_upright_edges_of_a_square_post_standing_on_a_plate_asked_half_its_side_or_more_is_declined_as_unsupported_and_less_lays_four_rounds`,
//!   `a_fillet_on_the_four_long_edges_of_a_square_bar_with_a_boss_on_one_end_asked_half_its_side_or_more_is_declined_as_unsupported_and_less_lays_four_rounds`,
//!   `a_fillet_on_the_four_long_edges_of_a_square_tube_asked_half_its_side_or_more_is_declined_as_unsupported_and_less_lays_four_rounds`;
//!   a closed square cavity, whose edges meet inwards and whose ends are its
//!   own, is declined the same way, and less fills four hollows —
//!   `a_fillet_on_the_four_long_edges_of_a_square_cavity_closed_inside_a_block_asked_half_its_side_or_more_is_declined_as_unsupported_and_less_fills_four_hollows`;
//!   and matter standing apart from the bar, a little past each end and well
//!   away beside it, is left whole by the cut that closes the round —
//!   `a_fillet_on_the_four_long_edges_of_a_square_bar_with_blocks_standing_apart_past_its_ends_and_beside_it_takes_half_and_leaves_them_whole`
//!   — and does not change the round itself, each end still bounded by one
//!   circle —
//!   `a_fillet_on_the_four_long_edges_of_a_square_bar_among_blocks_standing_apart_closes_into_one_cylinder_each_end_bounded_by_one_circle`
//! - The value is reduced to that half side as #698 reduces it, rather than
//!   the step being declined — the two tests above, which ask more than half
//!   and hold that half is taken and the asked value reported —
//!   `a_fillet_on_the_four_long_edges_of_a_square_bar_asked_half_its_side_or_more_takes_half_in_one_closed_cylinder_in_every_order`,
//!   `a_fillet_on_the_four_long_edges_of_a_square_bar_standing_askew_away_from_the_origin_takes_half_in_one_closed_cylinder_about_its_own_axis`
//! - Every step #698 lays, and the full round of two edges — the tests of
//!   #698 above, left as they were, among them
//!   `a_fillet_lays_one_full_round_or_round_bottom_whichever_of_its_two_faces_each_edge_shares_and_in_either_order`,
//!   `a_fillet_and_a_chamfer_on_the_four_long_edges_of_a_square_bar_asked_less_than_half_its_side_lay_each_at_the_value_asked`
//!   and
//!   `a_chamfer_on_the_four_long_edges_of_a_square_bar_asked_half_its_side_or_more_takes_half_a_diamond`
//!
//! Six mutants of `wedge/ring.rs` live on, and no body tells them apart. In
//! `closed`, the guard `count < 3 || pairs.len() != count || !in_two ||
//! inwards` reads one thing twice and another three times: every edge in
//! two pairs makes as many pairs as edges, two ends each, so turning either
//! of its first two `||` into `&&` changes nothing a drawable list reaches —
//! a list where it would, as many pairs as edges and one edge in three, needs
//! a face holding three of the edges, the other faces of all three tangent
//! to the one circle `about` asks for, which no solid has; and an edge that
//! meets inwards has its round's axis on the outer side of both its faces,
//! which `about` refuses on its own, so `||` into `&&` before `inwards`
//! changes nothing either — the closed cavity above runs through it. Adding
//! the base in `flat`, or the centre in the offset of each corner along the
//! axis, rather than taking it away, changes nothing, because a wedge's
//! point is its line's origin, the point of the line nearest the world's
//! origin, so that the base and the centre lie at nought along the axis.
//! And numbering the tool's floor, top and square sides by a product rather
//! than past the edges' numbers renames faces that stand outside the body,
//! past its ends by the margin and off its corners by more, which a cut the
//! step accepts never keeps.
//!
//! Closes #812.
//! - The leaning stair, nose and foot chamfered at 4: built in both orders,
//!   one body, exact, closed and uncrossed. The tread is gone. The two flats
//!   meet in the crease y = 6, z = 10. The body holds
//!   30·(290 − 80/√104 + 8). The same body asked 6, taken 4 —
//!   `a_chamfer_on_the_nose_and_foot_of_a_tread_whose_front_leans_asked_half_its_depth_or_more_takes_half_in_two_flats_meeting_in_a_crease_whichever_is_named_first`.
//!   Read as: each edge alone, asked 6, taking more than 4; in both orders,
//!   asked 4 and asked 6, the value asked reported and 4 taken; the tread
//!   gone and no other face of the stair, a flat for each edge, the next step
//!   numbering past both; the nose's flat answering to the nose's rank in the
//!   list alone, facing out of its section's line from (6, 10) to
//!   (2, 10) + 4·(−2, −10)/√104, those two its corners at either end of the
//!   stair; the foot's flat answering to the foot's rank alone, on
//!   z − y = 4, from (10, 14) to (6, 10); no face answering to both, the two
//!   flats meeting by one edge, the crease, both drawn up to (0, 6, 10) and
//!   (30, 6, 10); each flat meeting its edge's other face, the other flat
//!   and the stair's two ends by one edge each; what is left of the leaning
//!   front and of the riser, and the stair's two ends, their corners where
//!   the arithmetic puts them; the body exact, closed, uncrossed, holding
//!   30·(290 − 80/√104 + 8), 8704.660638, which the booleans give too — the
//!   nose's triangle cut away and the foot's joined on — and its triangles
//!   enclosing that: the triangle the nose gives up has two sides 4 long
//!   and the sine 10/√104 between them, 80/√104
//! - The leaning stair asked 3.999999: the same crease body in both orders,
//!   taken within a hair of 4, no strip of tread kept —
//!   `a_chamfer_on_the_nose_and_foot_of_a_tread_whose_front_leans_asked_a_hair_under_half_its_depth_lays_the_same_crease_whichever_is_named_first`.
//!   Read as: asked 4 − 1e-6, the strip of tread the set-backs would leave,
//!   2e-6, shorter than the kernel keeps and longer than one of its hairs;
//!   in both orders, the value asked reported and taken within that hair of
//!   4; every check of the first criterion's body, its corners at 4 to the
//!   place's tolerance, the tread gone; and, beyond the wording, the same
//!   asked 4 − 1e-7, the strip under one hair, as #791's slope is asked both
//! - The riser turned by 1e-7, asked 5 and 4.999999: built in both orders,
//!   one slope. It is one face answering to both edges' numbers, the tread
//!   gone, exact, closed and uncrossed, holding the volume the straight
//!   stair's slope holds within the drawing's tolerance —
//!   `a_chamfer_on_a_tread_s_nose_and_foot_whose_riser_is_turned_by_a_hair_asked_half_its_depth_or_a_hair_under_lays_one_slope_whichever_is_named_first`.
//!   Read as: in both orders, asked 5, 5 − 1e-6 and, beyond the wording, 8,
//!   as the decline it replaces was asked: the value asked reported and
//!   taken within a hair of 5; the tread gone and no other face of the
//!   stair, the next step numbering past both edges' numbers; one flat face
//!   answering to both, on one plane, drawn with the same triangles, no edge
//!   between the two numbers; the slope meeting the nose's front, the riser
//!   and the stair's two ends by one edge each; its four corners, one where
//!   each end meets the nose's front and one where each meets the turned
//!   riser, on those faces' planes to the place's tolerance and nowhere
//!   else, on one plane, each set back 5 from its edge within
//!   [`turned_stray`] — a hair, and how far the two flats part along the
//!   tread; the same corners in both orders, to the place's tolerance; the
//!   body exact, closed, uncrossed, holding exactly the matter the
//!   arithmetic gives under those corners, its triangles enclosing that, and
//!   that matter 9000, the straight stair's slope's, within what a slope
//!   whose corners stray that far can change, its footprint times twice the
//!   stray: far inside the drawing's tolerance, which over a flat body would
//!   let a slope a whole set-back off pass. Tightened after the review
//!   (2026-11-12): the corners were held within [`turned_stray`] of points
//!   rather than on the faces' planes
//! - Throughout: the lists `main` builds keep their bodies — the leaning
//!   stair at 3 and 2, the turned riser at 4, and #791's straight stair at 5
//!   and 4.999999, in both orders —
//!   `a_chamfer_on_the_nose_and_foot_of_a_tread_whose_front_leans_or_whose_riser_is_turned_short_of_the_limit_keeps_the_body_main_lays_whichever_is_named_first`,
//!   and for the straight stair #791's own tests, left as they were, which
//!   ask 5, 8, 5 − 1e-6 and 5 − 1e-7 in both orders —
//!   `a_chamfer_on_a_tread_s_nose_and_foot_asked_half_its_depth_or_more_takes_half_in_one_slope_answering_to_both_their_numbers`,
//!   `a_chamfer_on_a_tread_s_nose_and_foot_asked_a_hair_under_half_its_depth_lays_one_slope`.
//!   Read as: on the leaning stair at 3 and at 2, in both orders, the value
//!   taken as asked; every face of the stair kept, the tread a strip from
//!   2 + d to 10 − d, and a flat for each edge, each answering to its rank
//!   alone, facing out of its section's line, its corners where the
//!   arithmetic puts them, meeting its edge's other face, the tread and the
//!   stair's two ends by one edge each, the two not meeting; the leaning
//!   front, the riser and the two ends cut back to them; the body exact,
//!   closed, uncrossed, holding 30·(290 − 5d²/√104 + d²/2), 8702.621609 at 3
//!   and 8701.165159 at 2 as a probe on `main` measured them, which the
//!   booleans give too, and its triangles enclosing that. On the turned
//!   riser at 4, in both orders, the value taken as asked; the tread kept, a
//!   strip from y = 4 to 4 short of the riser, and a flat for each edge,
//!   each answering to its rank alone, the nose's from (x, 0, 6) to
//!   (x, 4, 10), the foot's from 4 up the riser to 4 short of it on the
//!   tread, each meeting its edge's other face, the tread and the stair's
//!   two ends by one edge each, the two not meeting; the body exact, closed,
//!   uncrossed, holding 9000, and its triangles enclosing that
//! - Throughout: no body of #791's or #698's tests changes — no test: those
//!   tests hold their own bodies, above in this file, and run beside the
//!   tests that answer #812, left as they were but #791's two pinned tests,
//!   which the next criterion flips; the fillet's S on the tread among them,
//!   `a_fillet_on_a_tread_s_nose_and_foot_asked_more_than_half_its_depth_takes_half_a_round_and_a_hollow_meeting_in_an_s`
//! - Throughout: #791's two pinned tests flip to the above:
//!   `…whose_riser_is_turned_by_a_hair_is_declined_as_unsupported…` and
//!   `…whose_front_leans_lays_two_flats_meeting_in_a_crease_named_nose_first_and_is_declined_named_foot_first`
//!   — the turned riser's decline is now
//!   `a_chamfer_on_a_tread_s_nose_and_foot_whose_riser_is_turned_by_a_hair_asked_half_its_depth_or_a_hair_under_lays_one_slope_whichever_is_named_first`,
//!   and the leaning stair's crease named nose first and decline named foot
//!   first are now
//!   `a_chamfer_on_the_nose_and_foot_of_a_tread_whose_front_leans_asked_half_its_depth_or_more_takes_half_in_two_flats_meeting_in_a_crease_whichever_is_named_first`;
//!   #791's criterion above names them in their stead
//! - Decided on the way, after the review (2026-11-12): a riser as high as
//!   half the tread's depth, or a little higher, keeps the one slope `main`
//!   lays on the nose and the foot, the riser gone where it is as high as
//!   the set-back and a strip of it kept where it is higher —
//!   `a_chamfer_on_a_tread_s_nose_and_foot_under_a_riser_as_high_as_half_its_depth_or_a_little_higher_takes_half_in_one_slope_the_riser_gone_or_a_strip_of_it_kept_whichever_is_named_first`.
//!   Read as: the stair with its taller block 15, 15.001 and 15.01 high, a
//!   riser 5, 5.001 and 5.01 high over the tread 10 deep, in both orders,
//!   asked 5 and 8: the value asked reported and 5 taken; the tread gone,
//!   the riser gone too at 5 and kept at 5.001 and 5.01, every other face of
//!   the stair kept, the next step numbering past both edges' numbers; one
//!   flat face answering to both, facing (0, −1, 1)/√2, from (x, 0, 5) to
//!   (x, 10, 15), meeting the nose's front, the stair's two ends and the
//!   riser by one edge each, or the taller block's top where the riser is
//!   gone; what is left of the riser a strip from 15 up to the top; the
//!   stair's two walls cut back to the slope; the body exact, closed,
//!   uncrossed, holding 30·10·(10 + h), h the taller block's height — 7500,
//!   7500.3 and 7503, as a probe on `main` measured them — and its triangles
//!   enclosing that. The test passes on `main`
//! - Decided on the way, after the review (2026-11-12): a riser tipped back
//!   by a hair about its bottom, its section from (10, 0) to (10 + 2e-6, 20)
//!   — the foot parallel to the nose as the kernel reads two edges, the two
//!   flats about 5e-8 rad apart and some 3.5e-7 off one another, more than
//!   the kernel's tolerance — is one slope in both orders, where `main`
//!   leaves a strip of tread 1e-6 wide between two flats —
//!   `a_chamfer_on_a_tread_s_nose_and_foot_whose_riser_tips_back_by_a_hair_asked_half_its_depth_or_more_lays_one_slope_whichever_is_named_first`.
//!   Read as: the lower block reaching in under the taller, as the turned
//!   riser's does; in both orders, asked 5 and 8, the value asked reported
//!   and taken within a hair of 5; the tread gone and no other face of the
//!   stair, the next step numbering past both edges' numbers; one flat face
//!   answering to both, drawn with the same triangles, no edge between the
//!   two numbers, meeting the nose's front, the riser and the stair's two
//!   ends by one edge each; its four corners on those faces' planes to the
//!   place's tolerance and nowhere else, alike at either end, each set back
//!   5 from its edge within a hair; the same corners in both orders; the
//!   body exact, closed, uncrossed, holding exactly the matter its section
//!   gives through those corners along the stair, its triangles enclosing
//!   that, and that matter the stair's own, 30·(300 − 7.5·2e-6), within what
//!   a slope a hair off can change over the tread
//! - Decided on the way, after the review (2026-11-12): the crease counts as
//!   a pair, as #791's slope does, so a list naming a third edge that meets
//!   it is declined in every order until #926 — on a flight whose front
//!   leans, its riser as high as its tread is deep, the first nose, the foot
//!   and the second nose, which `main` laid in four of their six orders —
//!   `a_chamfer_on_a_leaning_flight_s_first_nose_the_foot_between_and_the_second_nose_where_its_treads_are_as_deep_as_its_risers_are_high_is_declined_as_unsupported_in_every_order`.
//!   Read as: the first nose and the foot, and the foot and the second nose,
//!   each pair laid in both orders at 4; the three declined `Unsupported` in
//!   each of their six orders, asked 4 and asked 6. The front's upright edge
//!   at one end of the leaning stair's nose, named with the nose and the
//!   foot, never reaches the crease: the upright's set-back across the
//!   stair's end reaches the tread √104/10 along it for each unit set back,
//!   and meets the foot's at 80/(10 + √104), 3.960781, short of 4. `main` and
//!   this branch lay it in every order, one body, and it keeps that body —
//!   `a_chamfer_on_the_nose_and_foot_of_a_tread_whose_front_leans_and_the_front_s_upright_edge_at_one_end_of_the_nose_is_reduced_short_of_the_crease_and_lays_the_body_main_lays_in_every_order`.
//!   Read as: in each of the six orders, asked 4 and asked 6, the value
//!   asked reported and 80/(10 + √104) taken; a strip of the tread kept, the
//!   nose's flat and the foot's not meeting, the next step numbering past
//!   the three flats; the body exact, closed, uncrossed, holding
//!   8645.084326, as a probe on `main` measured it, the mitre at the corner
//!   reckoned by no arithmetic here, and its triangles enclosing that; and
//!   each edge's flat, read by its rank, with the corners the first order
//!   gave it, to the place's tolerance
//! - Decided on the way, after the re-review (2026-11-12): a riser turned by
//!   a billionth of a radian, its flats parting by about the kernel's own
//!   tolerance along the stair, is one slope in both orders, as the riser
//!   turned by 1e-7 is — the riser gone too where it is as high as half the
//!   tread's depth, its 10 high control keeping it —
//!   `a_chamfer_on_a_tread_s_nose_and_foot_whose_riser_is_turned_by_a_billionth_lays_one_slope_whichever_is_named_first_the_riser_gone_where_it_is_as_high_as_half_its_depth`.
//!   Read as: the [`stair_with_its_riser_turned_by`] 1e-9, its taller block
//!   20 high and 15 high, in both orders, asked 5 and 8: the value asked
//!   reported and taken within a hair of 5; the tread gone, and the riser
//!   too at 15, every other face of the stair kept, the next step numbering
//!   past both edges' numbers; one flat face answering to both, drawn with
//!   the same triangles, no edge between the two numbers, meeting the nose's
//!   front, the stair's two ends and the riser by one edge each, or the
//!   taller block's top where the riser is gone; its corners within a hair
//!   of (x, 0, 5) and of (x, r, 15) at either end, r where the riser stands
//!   there, and none anywhere else; the body exact, closed, uncrossed,
//!   holding 30·10·(10 + h), h the taller block's height, 9000 and 7500, and
//!   its triangles enclosing that, both within the slope's footprint times
//!   twice a hair. Held to a hair rather than to the place's tolerance: the
//!   flats part by about the kernel's tolerance, and where the riser is gone
//!   the slope's corners may stand that far off its line. The review read
//!   `main` as laying this slope; a probe on `main` declines every case of
//!   the turned riser, the control at 20 too, where the straight riser at 15
//!   and at 20 is one slope
//! - Decided on the way, after the mutants (2026-11-12): the riser turned by
//!   1e-7 and as high as half the tread's depth, the taller block 15 high,
//!   is declined `Unsupported` in both orders until #928. The slope would
//!   take the riser whole, and its top line parts from the riser's top edge
//!   by up to 3e-6 along the stair, more than the hair the billionth turn
//!   parts it by —
//!   `a_chamfer_on_a_tread_s_nose_and_foot_whose_riser_is_turned_by_a_ten_millionth_and_as_high_as_half_its_depth_is_declined_as_unsupported_whichever_is_named_first`.
//!   Read as: the same turn with the taller block 20 high laid in both
//!   orders at 5, one slope; at 15 high, the nose and the foot declined
//!   `Unsupported` in both orders, asked 5 and asked 8
//! - Decided on the way, after the mutants (2026-11-12): a riser turned by
//!   nearly what the kernel reads as parallel, 5e-7 and 9e-7 rad, its flats
//!   parting by up to 2.7e-5 along the stair, far more than any strip the
//!   kernel reads as none, is still one slope in both orders. The fill's run
//!   past the riser must reach as far as the two edges part over the part,
//!   or it stops short of the riser at one end —
//!   `a_chamfer_on_a_tread_s_nose_and_foot_whose_riser_is_turned_by_nearly_what_the_kernel_reads_as_parallel_lays_one_slope_whichever_is_named_first`.
//!   Read as: at each turn, the foot turning from the nose by that turn as
//!   the stair's faces give their corners, within a millionth; in both
//!   orders, asked 5, the value asked reported and taken within
//!   [`turned_stray`] of 5; every check the riser turned by 1e-7 is held
//!   to, the stray scaled to the turn: one flat face answering to both
//!   numbers, the tread gone, meeting the nose's front, the riser and the
//!   stair's two ends by one edge each; its four corners on those faces'
//!   planes to the place's tolerance, on one plane, set back 5 within the
//!   stray; the body exact, closed, uncrossed, holding exactly the matter
//!   under those corners, and that within the stray's reach of 9000; and the
//!   same corners in both orders
//!
//! Closes #813.
//! - The flight of two steps: `stair()` with its first nose, the foot and
//!   the second nose, chamfered at 5: built in every order, one body. One
//!   face answers to all three edges' numbers, on z = y + 5, from (x, 0, 5)
//!   to (x, 15, 20). The tread and the riser are gone. The body holds 8625,
//!   which the booleans give too. Asked 8, the same body, taken 5 —
//!   `a_chamfer_on_a_flight_s_first_nose_the_foot_between_and_the_second_nose_where_its_treads_are_as_deep_as_its_risers_are_high_takes_half_in_one_slope_across_it_in_every_order`.
//!   Read as: each edge alone, asked 8, taking more than 5, and the first
//!   nose and the foot, and the foot and the second nose, each pair laid
//!   alone at 5; in each of the six orders, asked 5 and asked 8, the value
//!   asked reported and 5 taken;
//!   the tread and the riser gone and no other face of the stair, the next
//!   step numbering past the three edges' numbers; one flat face answering
//!   to all three, on one plane, drawn with the same triangles, no edge
//!   between any two of the numbers, facing (0, −1, 1)/√2, its corners
//!   (x, 0, 5) and (x, 15, 20) at x = 0 and x = 30 and none anywhere else,
//!   meeting the nose's front below it, the taller block's top and the
//!   stair's two ends by one edge each; what is left of the nose's front,
//!   from the floor up to 5, and of the top, from 15 back; the stair's two
//!   ends cut back to the slope; the body exact, closed, uncrossed, holding
//!   30·(300 − 12.5), 8625 — each section losing the two noses' triangles
//!   and gaining the foot's, each 12.5 — which the booleans give too, a box
//!   over the foot joined on and a wedge above z = y + 5 cut away, and its
//!   triangles enclosing that. The same body asked 8 is every check above
//!   holding of it as of the body asked 5, in every order
//! - A flight of three steps (noses and feet alternating, each tread 10 deep
//!   and each riser 10 high): its five edges, chamfered at 5, built in every
//!   order as one slope answering to all five, with the volume the
//!   arithmetic gives —
//!   `a_chamfer_on_the_three_noses_and_two_feet_of_a_flight_of_three_steps_whose_treads_are_as_deep_as_its_risers_are_high_takes_half_in_one_slope_across_it_in_every_order`.
//!   Read as: the [`flight_of`] three steps, its blocks 10, 20 and 30 high;
//!   in each of the 120 orders asked 5, and asked 8 named up the flight and
//!   down it, the value asked reported and 5 taken; every check of the
//!   first criterion's body, the two treads and the two risers between
//!   gone, one flat face answering to all five numbers, its corners
//!   (x, 0, 5) and (x, 25, 30), meeting the first nose's front, the third
//!   block's top and the flight's two ends by one edge each; the body
//!   holding 30·(600 − 12.5), 17625 — three noses' triangles lost and two
//!   feet's gained — which the booleans give too
//! - Throughout: the lists `main` builds keep their bodies — a flight whose
//!   tread and riser differ —
//!   `a_chamfer_on_a_flight_s_first_nose_the_foot_between_and_the_second_nose_where_its_tread_and_riser_differ_keeps_the_body_main_lays_one_slope_across_the_shorter_and_a_strip_of_the_other_in_every_order`,
//!   and every two-edge slope of #791, held by #791's own tests, left as
//!   they were —
//!   `a_chamfer_on_a_tread_s_nose_and_foot_asked_half_its_depth_or_more_takes_half_in_one_slope_answering_to_both_their_numbers`,
//!   `a_chamfer_on_a_tread_s_nose_and_foot_asked_a_hair_under_half_its_depth_lays_one_slope`,
//!   `a_chamfer_on_a_tread_s_nose_and_foot_and_an_edge_far_from_both_numbers_the_slope_by_their_ranks_and_the_far_flat_by_its_own_wherever_the_far_edge_is_named`.
//!   Read as: the stair with its tread 10 deep under a riser 6 high, and
//!   with its tread 6 deep under a riser 10 high, the taller block's top 10
//!   deep in both; its first nose, foot and second nose in each of the six
//!   orders, asked 3 and asked 8: the value asked reported and 3 taken, half
//!   the shorter of the tread and the riser; under the shorter riser, the
//!   foot and the second nose one slope across it, the riser gone, from 3
//!   short of the foot on the tread to 3 back on the top, the first nose's
//!   flat apart and a strip of the tread kept from 3 to 7 between the two;
//!   on the shorter tread, the first nose and the foot one slope across it,
//!   the tread gone, from 3 down the nose's front to 3 up the riser, the
//!   second nose's flat apart and a strip of the riser kept from 13 to 17
//!   between the two; every other face of the stair kept, the next step
//!   numbering past the three; the slope answering to its two edges' ranks
//!   in the list, the flat to its own alone, each facing (0, −1, 1)/√2,
//!   its corners where the arithmetic puts them, and meeting the faces
//!   either side of it and the stair's two ends by one edge each; what is
//!   left of the nose's front, of the top and of the stair's two ends; the
//!   body exact, closed, uncrossed, holding 30·(260 − 4.5), 7665, in both —
//!   each section losing two noses' triangles of 4.5 and gaining the
//!   foot's — and its triangles enclosing that
//! - Throughout: no body of #791's, #698's or #812's tests changes — no
//!   test: those tests hold their own bodies, above in this file, and run
//!   beside the tests that answer #813, left as they were but #791's pinned
//!   decline, which the next criterion flips; #812's leaning flight among
//!   them, whose first nose, foot and second nose stay declined in every
//!   order until #926, the crease counting as a pair —
//!   `a_chamfer_on_a_leaning_flight_s_first_nose_the_foot_between_and_the_second_nose_where_its_treads_are_as_deep_as_its_risers_are_high_is_declined_as_unsupported_in_every_order`
//! - Throughout: #791's pinned decline
//!   `…where_its_treads_are_as_deep_as_its_risers_are_high_is_declined_as_unsupported`
//!   flips to the above: it is now
//!   `a_chamfer_on_a_flight_s_first_nose_the_foot_between_and_the_second_nose_where_its_treads_are_as_deep_as_its_risers_are_high_takes_half_in_one_slope_across_it_in_every_order`,
//!   and #791's criterion above names it in its stead
//! - Decided on the way, after the review (2026-11-12): a chain holding a
//!   crease is declined in every order until #926, whether its other flats
//!   meet in a crease too or lie a hair off one plane — the leaning flight
//!   with its top rising 2 over its depth, three flats in two creases, and
//!   rising 1e-5, the foot's flat and the second nose's 5e-7 rad apart, under
//!   the kernel's tangent, which the step laid in every order —
//!   `a_chamfer_on_a_leaning_flight_s_first_nose_the_foot_between_and_the_second_nose_whose_top_rises_meeting_in_two_creases_or_in_a_crease_and_a_slope_a_hair_off_is_declined_as_unsupported_in_every_order`.
//!   Read as: the [`LEANING_FLIGHT`] with its back corner of the top raised
//!   2 and 1e-5; the angle between the foot's flat and the second nose's
//!   read off the section, over a thousandth of a radian at 2 and 5e-7 at
//!   1e-5, under a millionth; the first nose and the foot, and the foot and
//!   the second nose, each pair laid alone at 4 in both orders; the three
//!   declined `Unsupported` in each of their six orders, asked 4 and asked 6
//! - Decided on the way, after the review (2026-11-12): the first nose, the
//!   foot and the second nose of a stair whose riser strays from the
//!   flight's one plane by a hair are one outcome whatever the order they
//!   are named in (#822) — declined `Unsupported` in every order, as none of
//!   the six built the slope, where the riser turned by a billionth was
//!   declined `Unverified` in three orders and `Unsupported` in the other
//!   three —
//!   `a_chamfer_on_the_first_nose_the_foot_and_the_second_nose_of_a_stair_whose_riser_strays_from_the_flight_s_one_plane_by_a_hair_is_declined_as_unsupported_in_every_order`.
//!   Read as: the [`stair_with_its_riser_turned_by`] 1e-9 and 1e-7, the
//!   [`stair_with_its_riser_tipped_back`], and the stair whose riser stands
//!   1e-7 higher than its tread is deep; in each of the six orders, asked 5
//!   and asked 8, declined `Unsupported`. A probe of this branch found the
//!   last three so in every order already; laying one slope across a hair
//!   chain, the same corners in every order, would replace this pin
//! - Decided on the way, after the review (2026-11-12): a chain that starts
//!   and ends on feet is one slope too — on the [`flight_of`] three steps,
//!   its first foot, its second nose and its second foot, the second tread
//!   and the riser under it gone, the first tread and the last riser kept —
//!   `a_chamfer_on_a_flight_of_three_steps_s_first_foot_second_nose_and_second_foot_takes_half_in_one_slope_from_the_first_tread_to_the_last_riser_in_every_order`.
//!   Read as: in each of the six orders, asked 5 and asked 8, the value asked
//!   reported and 5 taken; the second tread and the riser under it gone and
//!   no other face of the flight, the next step numbering past the three;
//!   one flat face answering to all three, no edge between any two of the
//!   numbers, facing (0, −1, 1)/√2, on z = y + 5, its corners (x, 5, 10) and
//!   (x, 20, 25) and none anywhere else, meeting the first tread, the last
//!   riser and the flight's two ends by one edge each; what is left of the
//!   first tread, from y = 0 to 5, and of the last riser, from z = 25 to 30;
//!   the flight's two ends cut back to the slope; the body exact, closed,
//!   uncrossed, holding 30·(600 + 12.5), 18375 — the two feet's triangles
//!   gained and the nose's lost — which the booleans give too, the nose's
//!   triangle cut away by a wedge and each foot's joined on by a prism, and
//!   its triangles enclosing that. The step laid this body before the review:
//!   the test pins it

use std::collections::BTreeSet;
use std::f64::consts::PI;

use cao_solid::profile::{Contour, Frame, Profile};
use cao_solid::soundness::{closed, enclosed, uncrossed};
use cao_solid::{Body, Declined, EdgeName, Loop, Lost};
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

/// The block the first criterion is held on: 30 along X, 10 across, 20 high.
const LENGTH: f64 = 30.0;
const BREADTH: f64 = 10.0;
const HEIGHT: f64 = 20.0;

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

/// A block of a length, a breadth and a height standing on the ground's
/// origin, joined as `cao_part` builds a step: its floor numbered nought, its
/// top one, and its sides two to five from the front round to the left.
fn block([length, breadth, height]: [f64; 3]) -> Body {
    joined_to(
        &Body::default(),
        [DVec2::ZERO, DVec2::new(length, breadth)],
        height,
    )
}

/// The part with a block standing on the ground between the corners `from`
/// and `to`, `height` high, joined to it as `cao_part` joins a step: the
/// block's faces numbered past the part's, its floor first, then its top,
/// then its sides from the front round to the left.
fn joined_to(part: &Body, [from, to]: [DVec2; 2], height: f64) -> Body {
    raised_with(part, Contour::rectangle(from, to), height)
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

/// Holds that a step of one edge left every face of the part it was given and
/// made one face more, and gives back the number that face answers to.
fn made_one(part: &Body, body: &Body, what: &str) -> usize {
    assert!(
        numbers(part).is_subset(&numbers(body)),
        "{what}, alone: a face of the part went: {:?}",
        numbers(body)
    );
    let made = faces_made(part, body);
    assert_eq!(made.len(), 1, "{what}, alone: one face more: {made:?}");
    made[0]
}

/// Holds that a face's corners are the points expected: each of them a
/// corner, and no corner anywhere else.
fn corners_are(body: &Body, face: usize, expected: &[DVec3], what: &str) {
    corners_are_or_on(body, face, expected, |_| false, what);
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

/// Holds that a step reports the value it was asked, and took it whole.
fn taken_as_asked(asked: f64, taken: f64, value: f64, what: &str) {
    assert!(
        (asked - value).abs() < PLACE && (taken - value).abs() < PLACE,
        "{what}: asked {asked} and took {taken}, where {value} fits"
    );
}

/// Holds that a step reports the value it was asked, and the one smaller
/// value every edge of its list gives.
fn reduced(asked: f64, taken: f64, [expected_asked, expected_taken]: [f64; 2], what: &str) {
    assert!(
        (asked - expected_asked).abs() < PLACE,
        "{what}: reports {asked} asked, where {expected_asked} was"
    );
    assert!(
        (taken - expected_taken).abs() < PLACE,
        "{what}: took {taken}, where every edge gives {expected_taken}"
    );
}

/// Holds that a body took away from the part it was given what each of the
/// steps of one edge `alone` takes away from it, all of it and no more.
fn takes_what_each_alone_takes(part: &Body, body: &Body, alone: &[Body], what: &str) {
    let each: f64 = alone
        .iter()
        .map(|single| part.volume() - single.volume())
        .sum();
    let taken = part.volume() - body.volume();
    assert!(
        (taken - each).abs() <= EXACT * part.volume(),
        "{what}: the step took {taken} away, where the steps of each edge alone take {each}"
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
    let sorted: BTreeSet<usize> = faces.iter().copied().collect();
    sorted.into_iter().map(|face| vec![face as u32]).collect()
}

/// The largest coordinate a body reaches, on either side of the origin.
fn reach_of(body: &Body) -> f64 {
    let (low, high) = body.bounds().expect("a part has a box");
    low.abs().max(high.abs()).max_element()
}

/// An edge of a block [`LENGTH`] long that runs along its length, from its
/// left wall to its right: the two faces it lies between, the corner of the
/// left wall it starts at, and the ways the two faces face.
#[derive(Clone)]
struct LongEdge {
    faces: [usize; 2],
    start: DVec3,
    facing: [DVec3; 2],
}

/// The block's four long edges, in the order the steps name them — not the
/// block's own: the top's front, the floor's back, the top's back and the
/// floor's front. No two share a vertex.
fn long_edges() -> [LongEdge; 4] {
    long_edges_of(BREADTH, HEIGHT)
}

/// The four long edges of a block [`LENGTH`] long, `breadth` across and
/// `height` high, in the order of [`long_edges`].
fn long_edges_of(breadth: f64, height: f64) -> [LongEdge; 4] {
    [
        LongEdge {
            faces: [TOP, FRONT],
            start: DVec3::new(0.0, 0.0, height),
            facing: [DVec3::Z, -DVec3::Y],
        },
        LongEdge {
            faces: [FLOOR, BACK],
            start: DVec3::new(0.0, breadth, 0.0),
            facing: [-DVec3::Z, DVec3::Y],
        },
        LongEdge {
            faces: [TOP, BACK],
            start: DVec3::new(0.0, breadth, height),
            facing: [DVec3::Z, DVec3::Y],
        },
        LongEdge {
            faces: [FLOOR, FRONT],
            start: DVec3::new(0.0, 0.0, 0.0),
            facing: [-DVec3::Z, -DVec3::Y],
        },
    ]
}

impl LongEdge {
    /// The edge's name on the part.
    fn name(&self, part: &Body) -> EdgeName {
        let [first, second] = self.faces;
        edge_between(part, first, second)
    }

    /// The same edge on the block moved, whole, by `by`.
    fn moved(&self, by: DVec3) -> LongEdge {
        LongEdge {
            start: self.start + by,
            ..self.clone()
        }
    }

    /// The round a fillet of `radius` lays on the edge: its axis the radius in
    /// from each face, along the whole length of the block.
    fn round(&self, radius: f64) -> Round {
        let [first, second] = self.facing;
        Round {
            start: self.start - (first + second) * radius,
            along: DVec3::X,
            length: LENGTH,
            radius,
            facing: self.facing,
        }
    }

    /// The flat a chamfer of `distance` lays on the edge: set back by it along
    /// each face, its corners at the left wall and at the right, facing
    /// halfway between the two faces.
    fn flat(&self, distance: f64) -> ([DVec3; 4], DVec3) {
        let [first, second] = self.facing;
        let on_first = self.start - second * distance;
        let on_second = self.start - first * distance;
        let along = DVec3::X * LENGTH;
        (
            [on_first, on_first + along, on_second + along, on_second],
            (first + second).normalize(),
        )
    }
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
        self.holds_within(point, PLACE)
    }

    /// Whether a point lies on the round, as [`Round::holds`] says, to
    /// `within` rather than to [`PLACE`].
    fn holds_within(&self, point: DVec3, within: f64) -> bool {
        let off = point - self.start;
        let along = off.dot(self.along);
        let across = off - self.along * along;
        let [first, second] = self.facing;
        let least = self.radius * first.dot(second);
        (across.length() - self.radius).abs() < within
            && (-within..=self.length + within).contains(&along)
            && across.dot(first) >= least - within
            && across.dot(second) >= least - within
    }

    /// The two points where the round's sides meet a wall square to it,
    /// `end` of the way along it: on its first face, then on its second.
    fn sides_at(&self, end: f64) -> [DVec3; 2] {
        [0.0, 1.0].map(|share| self.axis_at(end) + self.facing_at(share) * self.radius)
    }

    /// The round's area, its turn times its radius times its length.
    fn area(&self) -> f64 {
        let [first, second] = self.facing;
        first.dot(second).acos() * self.radius * self.length
    }
}

/// Holds that the face numbered `face` is the round expected: every corner
/// of its triangles on it, and every ray aimed square at its axis from
/// outside meeting it first, where it faces away from the axis along the ray.
fn is_the_round(body: &Body, face: usize, round: &Round, what: &str) {
    is_made_of(body, face, &[round], what);
}

/// Holds that the face numbered `face` is the rounds `parts` laid end to end
/// about one axis, as [`is_the_round`] holds it of one: every corner of its
/// triangles on one of them, and every ray aimed square at the axis from
/// outside, over each, meeting the face first.
fn is_made_of(body: &Body, face: usize, parts: &[&Round], what: &str) {
    is_made_of_within(body, face, parts, PLACE, what);
}

/// Holds what [`is_made_of`] holds, every corner and every normal to
/// `within` rather than to [`PLACE`].
fn is_made_of_within(body: &Body, face: usize, parts: &[&Round], within: f64, what: &str) {
    drawn_on(body, face, parts, within, what);
    for round in parts {
        met_from_outside(body, face, round, within, what);
    }
}

/// Holds that every corner of the triangles of the face numbered `face` lies
/// within `within` of one of the rounds `parts`.
fn drawn_on(body: &Body, face: usize, parts: &[&Round], within: f64, what: &str) {
    let corners: Vec<DVec3> = body.triangles_of(face).flatten().collect();
    assert!(!corners.is_empty(), "{what}: the round is drawn");
    for corner in corners {
        assert!(
            parts.iter().any(|round| round.holds_within(corner, within)),
            "{what}: the round is drawn through {corner}, off the cylinder of {} about the line \
             from {} along {}",
            parts[0].radius,
            parts[0].start,
            parts[0].along
        );
    }
}

/// Holds that every ray aimed square at the round's axis from outside, a
/// tenth to nine tenths of the way along it, meets the face numbered `face`
/// first, where it faces away from the axis along the ray, to `within`.
fn met_from_outside(body: &Body, face: usize, round: &Round, within: f64, what: &str) {
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
                hit.normal.distance(facing) < within.max(PLACE),
                "{what}: where a ray from {from} meets it, the round faces {}, not {facing}, \
                 away from its axis",
                hit.normal
            );
        }
    }
}

/// Holds that the face numbered `face` is a round running on past both ends
/// of its edge: every corner of its triangles on `running_on`, which reaches
/// past them, and every ray aimed at its axis over the edge, `over_the_edge`,
/// meeting it first.
fn is_the_round_running_on(
    body: &Body,
    face: usize,
    [running_on, over_the_edge]: [&Round; 2],
    what: &str,
) {
    drawn_on(body, face, &[running_on], PLACE, what);
    met_from_outside(body, face, over_the_edge, PLACE, what);
}

/// The one face among `made` whose triangles all lie on the round.
fn face_on(body: &Body, made: &[usize], round: &Round, what: &str) -> usize {
    let on: Vec<usize> = made
        .iter()
        .copied()
        .filter(|&face| {
            body.triangles_of(face)
                .flatten()
                .all(|corner| round.holds(corner))
        })
        .collect();
    assert_eq!(
        on.len(),
        1,
        "{what}: one face of {made:?} lies on the round about the line from {}",
        round.start
    );
    on[0]
}

/// The one flat face among `made` facing the way `facing` points.
fn face_facing(body: &Body, made: &[usize], facing: DVec3, what: &str) -> usize {
    let toward: Vec<usize> = made
        .iter()
        .copied()
        .filter(|&face| {
            body.plane_of(face)
                .is_some_and(|plane| plane.normal.distance(facing) < PLACE)
        })
        .collect();
    assert_eq!(
        toward.len(),
        1,
        "{what}: one face of {made:?} is flat and faces {facing}"
    );
    toward[0]
}

/// The corners of what is left of a long face of the block once two cuts set
/// it back to the line from `first` and the line from `second`, both points
/// on the left wall: from each, the length of the block along X.
fn strip(first: DVec3, second: DVec3) -> [DVec3; 4] {
    let along = DVec3::X * LENGTH;
    [first, first + along, second + along, second]
}

/// Holds that no edge of the body lies between the two faces of an edge the
/// step took: the new face stands between them.
fn no_longer_meet(body: &Body, [first, second]: [usize; 2], what: &str) {
    assert!(
        body.edge_names()
            .iter()
            .all(|name| !lies_between(name, first, second)),
        "{what}: faces {first} and {second} no longer meet, the new face stands between them"
    );
}

#[test]
fn a_fillet_on_the_four_long_edges_of_a_block_rounds_each_as_its_own_fillet_alone_would() {
    let radius = 3.0;
    let block = block([LENGTH, BREADTH, HEIGHT]);
    let edges = long_edges();
    let names: Vec<EdgeName> = edges.iter().map(|edge| edge.name(&block)).collect();

    let filleted = block
        .fillet_edges(&names, radius)
        .unwrap_or_else(|declined| panic!("declined, {declined:?}"));

    let what = "the block's four long edges rounded in one step";
    taken_as_asked(filleted.asked, filleted.taken, radius, what);
    let body = &filleted.body;
    let rounds: Vec<Round> = edges.iter().map(|edge| edge.round(radius)).collect();
    let area: f64 = rounds.iter().map(Round::area).sum();
    sound_drawn_within(
        body,
        LENGTH * BREADTH * HEIGHT - 4.0 * LENGTH * radius * radius * (1.0 - PI / 4.0),
        DRAWN * reach_of(&block) * area,
        what,
    );
    let made = faces_made(&block, body);
    assert_eq!(
        made.len(),
        4,
        "{what}: every face of the block kept, and one round more for each edge: {:?}",
        numbers(body)
    );
    assert!(
        numbers(&block).is_subset(&numbers(body)),
        "{what}: a face of the block went: {:?}",
        numbers(body)
    );
    let mut alone = Vec::new();
    for ((edge, name), round) in edges.iter().zip(&names).zip(&rounds) {
        let which = format!("the round between faces {:?}", edge.faces);
        let single = block
            .fillet(name, radius)
            .unwrap_or_else(|declined| panic!("{which}, alone: declined, {declined:?}"));
        let laid_alone = made_one(&block, &single.body, &which);
        is_the_round(
            &single.body,
            laid_alone,
            round,
            &format!("{which}, laid by its fillet alone"),
        );
        let laid = face_on(body, &made, round, &which);
        assert!(
            !body.is_flat(laid) && body.plane_of(laid).is_none(),
            "{which}: the face the fillet lays is curved, and offers no plane to draw on"
        );
        is_the_round(
            body,
            laid,
            round,
            &format!("{which}, laid with the three others"),
        );
        let [first, second] = edge.faces;
        assert_eq!(
            sides_met(body, laid),
            sides(&[first, second, RIGHT, LEFT]),
            "{which}: it meets its two faces along its sides, and the right and the left walls \
             at its ends, by one edge each, and no other round"
        );
        no_longer_meet(body, edge.faces, &which);
        alone.push(single.body);
    }
    takes_what_each_alone_takes(&block, body, &alone, what);
    for (face, first, second, which) in [
        (
            TOP,
            DVec3::new(0.0, radius, HEIGHT),
            DVec3::new(0.0, BREADTH - radius, HEIGHT),
            "what is left of the top, set back by the radius from the front and the back",
        ),
        (
            FLOOR,
            DVec3::new(0.0, radius, 0.0),
            DVec3::new(0.0, BREADTH - radius, 0.0),
            "what is left of the floor, set back by the radius from the front and the back",
        ),
        (
            FRONT,
            DVec3::new(0.0, 0.0, radius),
            DVec3::new(0.0, 0.0, HEIGHT - radius),
            "what is left of the front, set back by the radius from the floor and the top",
        ),
        (
            BACK,
            DVec3::new(0.0, BREADTH, radius),
            DVec3::new(0.0, BREADTH, HEIGHT - radius),
            "what is left of the back, set back by the radius from the floor and the top",
        ),
    ] {
        corners_are(body, face, &strip(first, second), which);
    }
    for (wall, end) in [(LEFT, 0.0), (RIGHT, 1.0)] {
        let ends: Vec<DVec3> = rounds
            .iter()
            .flat_map(|round| round.sides_at(end))
            .collect();
        corners_are_or_on(
            body,
            wall,
            &ends,
            |corner| rounds.iter().any(|round| round.holds(corner)),
            &format!("the wall at x = {}, its four corners rounded", end * LENGTH),
        );
    }
}

#[test]
fn a_chamfer_on_the_four_long_edges_of_a_block_cuts_each_as_its_own_chamfer_alone_would() {
    let distance = 3.0;
    let block = block([LENGTH, BREADTH, HEIGHT]);
    let edges = long_edges();
    let names: Vec<EdgeName> = edges.iter().map(|edge| edge.name(&block)).collect();

    let chamfered = block
        .chamfer_edges(&names, distance)
        .unwrap_or_else(|declined| panic!("declined, {declined:?}"));

    let what = "the block's four long edges cut in one step";
    taken_as_asked(chamfered.asked, chamfered.taken, distance, what);
    let body = &chamfered.body;
    let promised = LENGTH * BREADTH * HEIGHT - 4.0 * LENGTH * distance * distance / 2.0;
    sound_drawn_within(body, promised, EXACT * promised, what);
    let made = faces_made(&block, body);
    assert_eq!(
        made.len(),
        4,
        "{what}: every face of the block kept, and one flat more for each edge: {:?}",
        numbers(body)
    );
    assert!(
        numbers(&block).is_subset(&numbers(body)),
        "{what}: a face of the block went: {:?}",
        numbers(body)
    );
    let mut alone = Vec::new();
    for (edge, name) in edges.iter().zip(&names) {
        let which = format!("the flat between faces {:?}", edge.faces);
        let (corners, facing) = edge.flat(distance);
        let single = block
            .chamfer(name, distance)
            .unwrap_or_else(|declined| panic!("{which}, alone: declined, {declined:?}"));
        let cut_alone = made_one(&block, &single.body, &which);
        face_facing(
            &single.body,
            &[cut_alone],
            facing,
            &format!("{which}, cut by its chamfer alone"),
        );
        corners_are(
            &single.body,
            cut_alone,
            &corners,
            &format!("{which}, cut by its chamfer alone"),
        );
        let cut = face_facing(body, &made, facing, &which);
        corners_are(
            body,
            cut,
            &corners,
            &format!("{which}, cut with the three others"),
        );
        let [first, second] = edge.faces;
        assert_eq!(
            sides_met(body, cut),
            sides(&[first, second, RIGHT, LEFT]),
            "{which}: it meets its two faces along its sides, and the right and the left walls \
             at its ends, by one edge each, and no other flat"
        );
        no_longer_meet(body, edge.faces, &which);
        alone.push(single.body);
    }
    takes_what_each_alone_takes(&block, body, &alone, what);
    for (face, first, second, which) in [
        (
            TOP,
            DVec3::new(0.0, distance, HEIGHT),
            DVec3::new(0.0, BREADTH - distance, HEIGHT),
            "what is left of the top, set back by the distance from the front and the back",
        ),
        (
            FLOOR,
            DVec3::new(0.0, distance, 0.0),
            DVec3::new(0.0, BREADTH - distance, 0.0),
            "what is left of the floor, set back by the distance from the front and the back",
        ),
        (
            FRONT,
            DVec3::new(0.0, 0.0, distance),
            DVec3::new(0.0, 0.0, HEIGHT - distance),
            "what is left of the front, set back by the distance from the floor and the top",
        ),
        (
            BACK,
            DVec3::new(0.0, BREADTH, distance),
            DVec3::new(0.0, BREADTH, HEIGHT - distance),
            "what is left of the back, set back by the distance from the floor and the top",
        ),
    ] {
        corners_are(body, face, &strip(first, second), which);
    }
    for (wall, x) in [(LEFT, 0.0), (RIGHT, LENGTH)] {
        let octagon: Vec<DVec3> = edges
            .iter()
            .flat_map(|edge| {
                let (corners, _) = edge.flat(distance);
                [corners[0], corners[3]].map(|corner| corner + DVec3::X * x)
            })
            .collect();
        corners_are(
            body,
            wall,
            &octagon,
            &format!("the wall at x = {x}, its four corners cut"),
        );
    }
}

#[test]
fn each_round_and_each_flat_of_a_step_on_several_edges_is_numbered_in_the_order_the_step_names_its_edge_from_where_the_steps_numbers_ended()
 {
    let value = 3.0;
    let part = joined_to(
        &block([LENGTH, BREADTH, HEIGHT]),
        [DVec2::new(0.0, 40.0), DVec2::new(LENGTH, 40.0 + BREADTH)],
        HEIGHT,
    );
    let steps_end = 12;
    assert_eq!(
        part.faces_end(),
        steps_end,
        "the two blocks' steps numbered their twelve faces nought to eleven"
    );
    let edges = long_edges();
    let names: Vec<EdgeName> = edges.iter().map(|edge| edge.name(&part)).collect();

    let filleted = part
        .fillet_edges(&names, value)
        .unwrap_or_else(|declined| panic!("the fillet: declined, {declined:?}"));
    let chamfered = part
        .chamfer_edges(&names, value)
        .unwrap_or_else(|declined| panic!("the chamfer: declined, {declined:?}"));

    let laid = steps_end..steps_end + edges.len();
    for (what, body) in [
        ("the fillet", &filleted.body),
        ("the chamfer", &chamfered.body),
    ] {
        let mut kept_and_laid = numbers(&part);
        kept_and_laid.extend(laid.clone());
        assert_eq!(
            numbers(body),
            kept_and_laid,
            "{what}: every number the two blocks' faces answered to, and one for each edge from \
             {steps_end}, where the steps' numbers ended"
        );
        assert_eq!(
            body.faces_end(),
            laid.end,
            "{what}: the step names one face for each edge, and the next step numbers its own past \
             them"
        );
        let then = joined_to(body, [DVec2::new(0.0, -40.0), DVec2::new(10.0, -30.0)], 5.0);
        let mut after_the_next_step = kept_and_laid.clone();
        after_the_next_step.extend(laid.end..laid.end + 6);
        assert_eq!(
            numbers(&then),
            after_the_next_step,
            "{what}: a block joined after the step numbers its six faces past the last it laid"
        );
    }
    for (edge, number) in edges.iter().zip(laid) {
        let which = format!(
            "face {number}, laid for the edge between faces {:?}",
            edge.faces
        );
        is_the_round(
            &filleted.body,
            number,
            &edge.round(value),
            &format!("{which}, the fillet's round"),
        );
        let (corners, facing) = edge.flat(value);
        face_facing(
            &chamfered.body,
            &[number],
            facing,
            &format!("{which}, the chamfer's flat"),
        );
        corners_are(
            &chamfered.body,
            number,
            &corners,
            &format!("{which}, the chamfer's flat"),
        );
    }
}

/// The one flat face among `made` whose corners are the points expected.
fn flat_at(body: &Body, made: &[usize], corners: &[DVec3], what: &str) -> usize {
    let at: Vec<usize> = made
        .iter()
        .copied()
        .filter(|&face| {
            body.plane_of(face).is_some_and(|plane| {
                let near = |point: &DVec3, among: &[DVec3]| {
                    among.iter().any(|other| other.distance(*point) < PLACE)
                };
                plane.corners.iter().all(|corner| near(corner, corners))
                    && corners.iter().all(|point| near(point, &plane.corners))
            })
        })
        .collect();
    assert_eq!(
        at.len(),
        1,
        "{what}: one face of {made:?} is flat with its corners at {corners:?}"
    );
    at[0]
}

/// How wide the narrow block's top is: the most its top front edge gives.
const NARROW: f64 = 4.0;

/// The numbers the narrow block's top and front answer to, past the wide
/// block's six.
const NARROW_TOP: usize = TOP + 6;
const NARROW_FRONT: usize = FRONT + 6;

/// The part the one value is held on: the block of the first criterion, 10
/// across, and beside it, off the line of its edges, a block as long and as
/// high but [`NARROW`] across, from 40 to 44 along Y.
fn wide_and_narrow() -> Body {
    joined_to(
        &block([LENGTH, BREADTH, HEIGHT]),
        [DVec2::new(0.0, 40.0), DVec2::new(LENGTH, 40.0 + NARROW)],
        HEIGHT,
    )
}

/// The matter the two blocks of [`wide_and_narrow`] hold.
const WIDE_AND_NARROW: f64 = LENGTH * (BREADTH + NARROW) * HEIGHT;

/// The lists the one value is held on, each with what it is and the two
/// corners, on the left wall, of what is left of the wide block's top once
/// every edge of the list is set back by [`NARROW`]. The narrow block's top
/// front edge gives no more than that, its top taken whole; the wide block's
/// edges give more. With the wide block's top front edge, its top is left 6
/// wide; with both edges of its top, 2 wide, the two set-backs stopped short
/// of meeting by the narrow edge.
fn narrow_lists() -> [(&'static str, Vec<LongEdge>, [DVec3; 2]); 2] {
    let [top_front, _, top_back, _] = long_edges();
    let narrow = LongEdge {
        faces: [NARROW_TOP, NARROW_FRONT],
        start: DVec3::new(0.0, 40.0, HEIGHT),
        facing: [DVec3::Z, -DVec3::Y],
    };
    [
        (
            "the wide block's top front edge and the narrow block's",
            vec![top_front.clone(), narrow.clone()],
            [
                DVec3::new(0.0, NARROW, HEIGHT),
                DVec3::new(0.0, BREADTH, HEIGHT),
            ],
        ),
        (
            "both edges of the wide block's top and the narrow block's top front edge",
            vec![top_front, top_back, narrow],
            [
                DVec3::new(0.0, NARROW, HEIGHT),
                DVec3::new(0.0, BREADTH - NARROW, HEIGHT),
            ],
        ),
    ]
}

#[test]
fn a_fillet_on_several_edges_asked_more_than_one_of_them_gives_lays_every_round_at_the_one_radius_they_all_fit_and_reports_both()
 {
    let asked = 6.0;
    let part = wide_and_narrow();
    for (list, edges, [first, second]) in narrow_lists() {
        let names: Vec<EdgeName> = edges.iter().map(|edge| edge.name(&part)).collect();

        let filleted = part
            .fillet_edges(&names, asked)
            .unwrap_or_else(|declined| panic!("{list}: declined, {declined:?}"));

        let what = format!("the fillet on {list}, asked {asked}");
        let body = &filleted.body;
        let made = faces_made(&part, body);
        let rounds: Vec<Round> = edges.iter().map(|edge| edge.round(NARROW)).collect();
        for (edge, round) in edges.iter().zip(&rounds) {
            let which = format!(
                "{what}: the round between faces {:?}, of the one radius {NARROW}",
                edge.faces
            );
            let laid = face_on(body, &made, round, &which);
            is_the_round(body, laid, round, &which);
        }
        assert_eq!(
            made.len(),
            edges.len(),
            "{what}: one round for each edge: {made:?}"
        );
        reduced(filleted.asked, filleted.taken, [asked, NARROW], &what);
        let area: f64 = rounds.iter().map(Round::area).sum();
        sound_drawn_within(
            body,
            WIDE_AND_NARROW - edges.len() as f64 * LENGTH * NARROW * NARROW * (1.0 - PI / 4.0),
            DRAWN * reach_of(&part) * area,
            &what,
        );
        corners_are(
            body,
            TOP,
            &strip(first, second),
            &format!("{what}: what is left of the wide block's top, set back by {NARROW}"),
        );
        corners_are(
            body,
            FRONT,
            &strip(DVec3::ZERO, DVec3::new(0.0, 0.0, HEIGHT - NARROW)),
            &format!("{what}: what is left of the wide block's front, set back by {NARROW}"),
        );
        assert!(
            !body.has_face(NARROW_TOP),
            "{what}: the narrow block's top, {NARROW} wide, is taken whole"
        );
    }
}

#[test]
fn a_chamfer_on_several_edges_asked_more_than_one_of_them_gives_cuts_every_flat_at_the_one_distance_they_all_fit_and_reports_both()
 {
    let asked = 6.0;
    let part = wide_and_narrow();
    for (list, edges, [first, second]) in narrow_lists() {
        let names: Vec<EdgeName> = edges.iter().map(|edge| edge.name(&part)).collect();

        let chamfered = part
            .chamfer_edges(&names, asked)
            .unwrap_or_else(|declined| panic!("{list}: declined, {declined:?}"));

        let what = format!("the chamfer on {list}, asked {asked}");
        let body = &chamfered.body;
        let made = faces_made(&part, body);
        for edge in &edges {
            let (corners, facing) = edge.flat(NARROW);
            let which = format!(
                "{what}: the flat between faces {:?}, set back by the one distance {NARROW}",
                edge.faces
            );
            let cut = flat_at(body, &made, &corners, &which);
            face_facing(body, &[cut], facing, &which);
        }
        assert_eq!(
            made.len(),
            edges.len(),
            "{what}: one flat for each edge: {made:?}"
        );
        reduced(chamfered.asked, chamfered.taken, [asked, NARROW], &what);
        let promised = WIDE_AND_NARROW - edges.len() as f64 * LENGTH * NARROW * NARROW / 2.0;
        sound_drawn_within(body, promised, EXACT * promised, &what);
        corners_are(
            body,
            TOP,
            &strip(first, second),
            &format!("{what}: what is left of the wide block's top, set back by {NARROW}"),
        );
        corners_are(
            body,
            FRONT,
            &strip(DVec3::ZERO, DVec3::new(0.0, 0.0, HEIGHT - NARROW)),
            &format!("{what}: what is left of the wide block's front, set back by {NARROW}"),
        );
        assert!(
            !body.has_face(NARROW_TOP),
            "{what}: the narrow block's top, {NARROW} wide, is taken whole"
        );
    }
}

#[test]
fn a_fillet_and_a_chamfer_on_the_two_edges_either_side_of_a_face_take_a_value_that_fits_as_asked() {
    let value = 4.9;
    let block = block([LENGTH, BREADTH, HEIGHT]);
    let [top_front, _, top_back, _] = long_edges();
    let edges = [top_front, top_back];
    let names: Vec<EdgeName> = edges.iter().map(|edge| edge.name(&block)).collect();

    let filleted = block
        .fillet_edges(&names, value)
        .unwrap_or_else(|declined| panic!("the fillet: declined, {declined:?}"));
    let chamfered = block
        .chamfer_edges(&names, value)
        .unwrap_or_else(|declined| panic!("the chamfer: declined, {declined:?}"));

    let top_left = strip(
        DVec3::new(0.0, value, HEIGHT),
        DVec3::new(0.0, BREADTH - value, HEIGHT),
    );
    let what = format!("the fillet on both edges of a top {BREADTH} wide, asked {value}");
    taken_as_asked(filleted.asked, filleted.taken, value, &what);
    let body = &filleted.body;
    let made = faces_made(&block, body);
    let rounds: Vec<Round> = edges.iter().map(|edge| edge.round(value)).collect();
    for (edge, round) in edges.iter().zip(&rounds) {
        let which = format!("{what}: the round between faces {:?}", edge.faces);
        let laid = face_on(body, &made, round, &which);
        is_the_round(body, laid, round, &which);
    }
    let area: f64 = rounds.iter().map(Round::area).sum();
    sound_drawn_within(
        body,
        LENGTH * BREADTH * HEIGHT - 2.0 * LENGTH * value * value * (1.0 - PI / 4.0),
        DRAWN * reach_of(&block) * area,
        &what,
    );
    corners_are(
        body,
        TOP,
        &top_left,
        &format!("{what}: what is left of the top, a strip between the two rounds"),
    );

    let what = format!("the chamfer on both edges of a top {BREADTH} wide, asked {value}");
    taken_as_asked(chamfered.asked, chamfered.taken, value, &what);
    let body = &chamfered.body;
    let made = faces_made(&block, body);
    for edge in &edges {
        let (corners, _) = edge.flat(value);
        flat_at(
            body,
            &made,
            &corners,
            &format!("{what}: the flat between faces {:?}", edge.faces),
        );
    }
    let promised = LENGTH * BREADTH * HEIGHT - 2.0 * LENGTH * value * value / 2.0;
    sound_drawn_within(body, promised, EXACT * promised, &what);
    corners_are(
        body,
        TOP,
        &top_left,
        &format!("{what}: what is left of the top, a strip between the two flats"),
    );
}

/// What is asked of both edges of the top of the block 10 wide: more than
/// half its width, which either edge alone gives.
const MORE_THAN_HALF: f64 = 8.0;

/// Holds that each of the edges `names` alone takes more than `most`, asked
/// `asked`, which is what makes the limit the two edges' together.
fn alone_each_takes_more(
    step: impl Fn(&EdgeName) -> Result<f64, Declined>,
    names: &[EdgeName],
    most: f64,
    what: &str,
) {
    for name in names {
        let taken = step(name)
            .unwrap_or_else(|declined| panic!("{what}, one edge alone: declined, {declined:?}"));
        assert!(
            taken > most + PLACE,
            "{what}: one edge alone takes {taken}, more than {most}"
        );
    }
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

#[test]
fn a_fillet_on_the_two_edges_either_side_of_a_face_10_wide_asked_more_than_5_takes_5_in_one_full_round()
 {
    let block = block([LENGTH, BREADTH, HEIGHT]);
    let [top_front, _, top_back, _] = long_edges();
    let names = [top_front.name(&block), top_back.name(&block)];
    let half = BREADTH / 2.0;
    alone_each_takes_more(
        |name| {
            block
                .fillet(name, MORE_THAN_HALF)
                .map(|single| single.taken)
        },
        &names,
        half,
        "the fillet on the top's front or back edge",
    );

    let filleted = block
        .fillet_edges(&names, MORE_THAN_HALF)
        .unwrap_or_else(|declined| panic!("declined, {declined:?}"));

    let what = format!("the fillet on both edges of a top {BREADTH} wide, asked {MORE_THAN_HALF}");
    reduced(
        filleted.asked,
        filleted.taken,
        [MORE_THAN_HALF, half],
        &what,
    );
    let body = &filleted.body;
    let laid = [block.faces_end(), block.faces_end() + 1];
    assert!(!body.has_face(TOP), "{what}: the top is gone");
    let mut kept_and_laid = numbers(&block);
    kept_and_laid.remove(&TOP);
    kept_and_laid.extend(laid);
    assert_eq!(
        numbers(body),
        kept_and_laid,
        "{what}: every face of the block but its top, and the round answering to {laid:?}"
    );
    assert_eq!(
        body.faces_end(),
        laid[1] + 1,
        "{what}: the next step numbers its faces past both the round answers to"
    );
    one_curved_face_answers_to(body, &laid, &what);
    let axis = DVec3::new(0.0, half, HEIGHT - half);
    let halves = [[-DVec3::Y, DVec3::Z], [DVec3::Z, DVec3::Y]].map(|facing| Round {
        start: axis,
        along: DVec3::X,
        length: LENGTH,
        radius: half,
        facing,
    });
    is_made_of(
        body,
        laid[0],
        &[&halves[0], &halves[1]],
        &format!(
            "{what}: the full round, a half-cylinder of {half} about the line {half} under the \
             middle of the top, tangent to the front and the back"
        ),
    );
    assert_eq!(
        sides_met_by(body, &laid),
        sides(&[FRONT, RIGHT, BACK, LEFT]),
        "{what}: the full round meets the front and the back along its sides, and the right and \
         the left walls at its ends, by one edge each"
    );
    corners_are(
        body,
        FRONT,
        &strip(DVec3::ZERO, DVec3::new(0.0, 0.0, HEIGHT - half)),
        &format!("{what}: what is left of the front, up to where the round takes over"),
    );
    corners_are(
        body,
        BACK,
        &strip(
            DVec3::new(0.0, BREADTH, 0.0),
            DVec3::new(0.0, BREADTH, HEIGHT - half),
        ),
        &format!("{what}: what is left of the back, up to where the round takes over"),
    );
    for (wall, x) in [(LEFT, 0.0), (RIGHT, LENGTH)] {
        corners_are_or_on(
            body,
            wall,
            &[
                DVec3::new(x, 0.0, 0.0),
                DVec3::new(x, BREADTH, 0.0),
                DVec3::new(x, 0.0, HEIGHT - half),
                DVec3::new(x, BREADTH, HEIGHT - half),
            ],
            |corner| halves.iter().any(|round| round.holds(corner)),
            &format!("{what}: the wall at x = {x}, its top a half-circle"),
        );
    }
    sound_drawn_within(
        body,
        LENGTH * BREADTH * HEIGHT - LENGTH * (BREADTH * half - PI * half * half / 2.0),
        DRAWN * reach_of(&block) * PI * half * LENGTH,
        &what,
    );
}

#[test]
fn a_chamfer_on_the_two_edges_either_side_of_a_face_10_wide_asked_more_than_5_takes_5_its_flats_meeting_in_a_ridge()
 {
    let block = block([LENGTH, BREADTH, HEIGHT]);
    let [top_front, _, top_back, _] = long_edges();
    let names = [top_front.name(&block), top_back.name(&block)];
    let half = BREADTH / 2.0;
    alone_each_takes_more(
        |name| {
            block
                .chamfer(name, MORE_THAN_HALF)
                .map(|single| single.taken)
        },
        &names,
        half,
        "the chamfer on the top's front or back edge",
    );

    let chamfered = block
        .chamfer_edges(&names, MORE_THAN_HALF)
        .unwrap_or_else(|declined| panic!("declined, {declined:?}"));

    let what = format!("the chamfer on both edges of a top {BREADTH} wide, asked {MORE_THAN_HALF}");
    reduced(
        chamfered.asked,
        chamfered.taken,
        [MORE_THAN_HALF, half],
        &what,
    );
    let body = &chamfered.body;
    let laid = [block.faces_end(), block.faces_end() + 1];
    assert!(!body.has_face(TOP), "{what}: the top is gone");
    let mut kept_and_laid = numbers(&block);
    kept_and_laid.remove(&TOP);
    kept_and_laid.extend(laid);
    assert_eq!(
        numbers(body),
        kept_and_laid,
        "{what}: every face of the block but its top, and a flat for each edge"
    );
    for (edge, number) in [&top_front, &top_back].into_iter().zip(laid) {
        let (corners, facing) = edge.flat(half);
        let which = format!(
            "{what}: face {number}, the flat between faces {:?}",
            edge.faces
        );
        face_facing(body, &[number], facing, &which);
        corners_are(body, number, &corners, &which);
    }
    edge_between(body, laid[0], laid[1]);
    assert_eq!(
        sides_met(body, laid[0]),
        sides(&[FRONT, RIGHT, LEFT, laid[1]]),
        "{what}: the front flat meets the front, the two walls, and the back flat along the \
         ridge, by one edge each"
    );
    assert_eq!(
        sides_met(body, laid[1]),
        sides(&[RIGHT, BACK, LEFT, laid[0]]),
        "{what}: the back flat meets the back, the two walls, and the front flat along the \
         ridge, by one edge each"
    );
    corners_are(
        body,
        FRONT,
        &strip(DVec3::ZERO, DVec3::new(0.0, 0.0, HEIGHT - half)),
        &format!("{what}: what is left of the front"),
    );
    corners_are(
        body,
        BACK,
        &strip(
            DVec3::new(0.0, BREADTH, 0.0),
            DVec3::new(0.0, BREADTH, HEIGHT - half),
        ),
        &format!("{what}: what is left of the back"),
    );
    for (wall, x) in [(LEFT, 0.0), (RIGHT, LENGTH)] {
        corners_are(
            body,
            wall,
            &[
                DVec3::new(x, 0.0, 0.0),
                DVec3::new(x, BREADTH, 0.0),
                DVec3::new(x, BREADTH, HEIGHT - half),
                DVec3::new(x, half, HEIGHT),
                DVec3::new(x, 0.0, HEIGHT - half),
            ],
            &format!("{what}: the wall at x = {x}, a pentagon with the ridge at its peak"),
        );
    }
    let promised = LENGTH * BREADTH * HEIGHT - 2.0 * LENGTH * half * half / 2.0;
    sound_drawn_within(body, promised, EXACT * promised, &what);
}

/// The section of a prism lying along X, drawn on the plane of Y and Z: its
/// bottom 16 wide, its top 10 wide, from 2 to 12 at a height of 10, and its
/// two sides slanting in at two angles, so that a round asks a different
/// set-back along the top at either end. Raised along X, its numbers are its
/// start at x = 0 nought, its end one, then its bottom two, its right side
/// three, its top four and its left side five.
const SLANTED: [DVec2; 4] = [
    DVec2::new(0.0, 0.0),
    DVec2::new(16.0, 0.0),
    DVec2::new(12.0, 10.0),
    DVec2::new(2.0, 10.0),
];
const SLANTED_START: usize = 0;
const SLANTED_END: usize = 1;
const SLANTED_RIGHT: usize = 3;
const SLANTED_TOP: usize = 4;
const SLANTED_LEFT: usize = 5;

/// A point of the [`SLANTED`] section's plane, across and up, `x` along the
/// prism.
fn on_section(at: DVec2, x: f64) -> DVec3 {
    DVec3::new(x, at.x, at.y)
}

/// The [`SLANTED`] section raised along X for [`LENGTH`].
fn slanted_prism() -> Body {
    let section = Frame {
        origin: DVec3::ZERO,
        u: DVec3::Y,
        v: DVec3::Z,
    };
    let part = Body::default();
    let tool = part
        .tool_raised(
            &profile(Contour::straight(SLANTED.to_vec())),
            section,
            DVec3::X * LENGTH,
        )
        .expect("the tool raises");
    part.union(&tool).expect("the part is built")
}

/// One end of the slanted prism's top: the corner of the section it stands
/// on, the far end of its side, and the way that side faces.
struct SlantedEnd {
    corner: DVec2,
    foot: DVec2,
    outward: DVec2,
}

impl SlantedEnd {
    /// The tangent of half the angle the top and the side meet at, inside
    /// the matter: what a round's radius is divided by to give its set-back
    /// along either.
    fn half_tangent(&self, along_top: DVec2) -> f64 {
        let down = (self.foot - self.corner).normalize();
        let cosine = down.dot(along_top);
        (1.0 - cosine * cosine).sqrt() / (1.0 + cosine)
    }

    /// The angle the top and the side meet at, inside the matter.
    fn angle(&self, along_top: DVec2) -> f64 {
        (self.foot - self.corner).normalize().dot(along_top).acos()
    }
}

#[test]
fn a_fillet_on_the_two_top_edges_of_a_prism_whose_sides_slant_takes_the_radius_whose_set_backs_meet_in_one_full_round()
 {
    let asked = 20.0;
    let prism = slanted_prism();
    let [left_edge, right_edge] = [
        edge_between(&prism, SLANTED_TOP, SLANTED_LEFT),
        edge_between(&prism, SLANTED_RIGHT, SLANTED_TOP),
    ];
    let left = SlantedEnd {
        corner: SLANTED[3],
        foot: SLANTED[0],
        outward: DVec2::new(-10.0, 2.0).normalize(),
    };
    let right = SlantedEnd {
        corner: SLANTED[2],
        foot: SLANTED[1],
        outward: DVec2::new(10.0, 4.0).normalize(),
    };
    let width = SLANTED[2].x - SLANTED[3].x;
    let (left_tangent, right_tangent) =
        (left.half_tangent(DVec2::X), right.half_tangent(-DVec2::X));
    let radius = width / (1.0 / left_tangent + 1.0 / right_tangent);
    for (order, names) in [
        (
            "the left edge named first",
            [left_edge.clone(), right_edge.clone()],
        ),
        (
            "the right edge named first",
            [right_edge.clone(), left_edge.clone()],
        ),
    ] {
        alone_each_takes_more(
            |name| prism.fillet(name, asked).map(|single| single.taken),
            &names,
            radius,
            "the fillet on one top edge of the slanted prism",
        );

        let filleted = prism
            .fillet_edges(&names, asked)
            .unwrap_or_else(|declined| panic!("declined, {declined:?}"));

        let what = format!(
            "the fillet on both top edges of the slanted prism, asked {asked}, its top {width} wide, \
         {order}"
        );
        reduced(filleted.asked, filleted.taken, [asked, radius], &what);
        let body = &filleted.body;
        let laid = [prism.faces_end(), prism.faces_end() + 1];
        assert!(!body.has_face(SLANTED_TOP), "{what}: the top is gone");
        one_curved_face_answers_to(body, &laid, &what);
        let left_set_back = radius / left_tangent;
        let right_set_back = radius / right_tangent;
        let axis = SLANTED[3] + DVec2::new(left_set_back, -radius);
        assert!(
            axis.distance(SLANTED[2] + DVec2::new(-right_set_back, -radius)) < PLACE,
            "the two rounds' axes, {radius} under the top where each set-back ends, are one"
        );
        let facing = |way: DVec2| DVec3::new(0.0, way.x, way.y);
        let parts = [
            [facing(left.outward), DVec3::Z],
            [DVec3::Z, facing(right.outward)],
        ]
        .map(|facing| Round {
            start: on_section(axis, 0.0),
            along: DVec3::X,
            length: LENGTH,
            radius,
            facing,
        });
        is_made_of(
            body,
            laid[0],
            &[&parts[0], &parts[1]],
            &format!(
                "{what}: the full round, of {radius} about one axis, tangent to both sides and \
             turning from one to the other over where the top was"
            ),
        );
        assert_eq!(
            sides_met_by(body, &laid),
            sides(&[SLANTED_START, SLANTED_END, SLANTED_RIGHT, SLANTED_LEFT]),
            "{what}: the full round meets the two sides along its own, and the start and the end at \
         its ends, by one edge each"
        );
        let left_tangent_point =
            left.corner + (left.foot - left.corner).normalize() * left_set_back;
        let right_tangent_point =
            right.corner + (right.foot - right.corner).normalize() * right_set_back;
        for (side, foot, tangent_point) in [
            (SLANTED_LEFT, left.foot, left_tangent_point),
            (SLANTED_RIGHT, right.foot, right_tangent_point),
        ] {
            corners_are(
                body,
                side,
                &strip(on_section(foot, 0.0), on_section(tangent_point, 0.0)),
                &format!("{what}: what is left of side {side}, up to where the round takes over"),
            );
        }
        for (cap, x) in [(SLANTED_START, 0.0), (SLANTED_END, LENGTH)] {
            corners_are_or_on(
                body,
                cap,
                &[
                    left.foot,
                    right.foot,
                    left_tangent_point,
                    right_tangent_point,
                ]
                .map(|at| on_section(at, x)),
                |corner| parts.iter().any(|round| round.holds(corner)),
                &format!("{what}: the end at x = {x}, its top an arc"),
            );
        }
        let section = (16.0 + width) / 2.0 * 10.0;
        let taken: f64 = [
            (&left, DVec2::X, left_tangent),
            (&right, -DVec2::X, right_tangent),
        ]
        .into_iter()
        .map(|(end, along_top, tangent)| {
            radius * radius * (1.0 / tangent - (PI - end.angle(along_top)) / 2.0)
        })
        .sum();
        let turn = (PI - left.angle(DVec2::X)) + (PI - right.angle(-DVec2::X));
        sound_drawn_within(
            body,
            (section - taken) * LENGTH,
            DRAWN * reach_of(&prism) * turn * radius * LENGTH,
            &what,
        );
    }
}

/// The outline of an L: an arm 30 long and 6 deep along the front, and at its
/// left end a stem 12 wide reaching back to 14. Raised by [`ELL_HEIGHT`], its
/// walls are numbered from the front round: the front two, the arm's right
/// end three, the arm's back four, the stem's right side five, the stem's
/// back six and the left seven. The arm's back and the stem's right side meet
/// inwards at [`INNER_CORNER`], and a fill set back along both can take no
/// more than the stem's right side, [`STEM_RIGHT_LENGTH`] long.
const ELL_OUTLINE: [DVec2; 6] = [
    DVec2::new(0.0, 0.0),
    DVec2::new(30.0, 0.0),
    DVec2::new(30.0, 6.0),
    DVec2::new(12.0, 6.0),
    DVec2::new(12.0, 14.0),
    DVec2::new(0.0, 14.0),
];
const ELL_HEIGHT: f64 = 10.0;
const ARM_BACK: usize = 4;
const STEM_RIGHT: usize = 5;
const INNER_CORNER: DVec2 = DVec2::new(12.0, 6.0);
const STEM_RIGHT_LENGTH: f64 = 8.0;
const ELL_VOLUME: f64 = (30.0 * 6.0 + 12.0 * 8.0) * ELL_HEIGHT;

/// The numbers the block beside the L answers to for its top and its front,
/// past the L's eight.
const BESIDE_TOP: usize = TOP + 8;
const BESIDE_FRONT: usize = FRONT + 8;

/// The L, and beside it, off the line of every edge it has, a block
/// [`LENGTH`] long, `breadth` across from 40 along Y, and [`HEIGHT`] high,
/// whose top front edge meets outwards.
fn ell_and_block(breadth: f64) -> Body {
    let ell = raised_with(
        &Body::default(),
        Contour::straight(ELL_OUTLINE.to_vec()),
        ELL_HEIGHT,
    );
    joined_to(
        &ell,
        [DVec2::new(0.0, 40.0), DVec2::new(LENGTH, 40.0 + breadth)],
        HEIGHT,
    )
}

/// The block's top front edge, beside the L.
fn beside_top_front() -> LongEdge {
    LongEdge {
        faces: [BESIDE_TOP, BESIDE_FRONT],
        start: DVec3::new(0.0, 40.0, HEIGHT),
        facing: [DVec3::Z, -DVec3::Y],
    }
}

/// The round a fillet lays in a concave corner: a cylinder of `radius` about
/// the line from `start` along `along` for `length`, that line standing in
/// the air the two faces leave between them. The round turns from facing the
/// way the edge's first face does to facing the way its second does, and
/// everywhere faces its axis, a radius away against the way it faces.
struct Hollow {
    start: DVec3,
    along: DVec3,
    length: f64,
    radius: f64,
    facing: [DVec3; 2],
}

impl Hollow {
    /// The round of `radius` in the L's inner corner, the radius out from
    /// the arm's back and from the stem's right side, from the floor to the
    /// top.
    fn in_the_ell(radius: f64) -> Hollow {
        Hollow {
            start: (INNER_CORNER + DVec2::splat(radius)).extend(0.0),
            along: DVec3::Z,
            length: ELL_HEIGHT,
            radius,
            facing: [DVec3::Y, DVec3::X],
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
    is_hollow_made_of(body, face, &[hollow], what);
}

/// Holds that the face numbered `face` is the hollow rounds `parts` laid end
/// to end about one axis, as [`is_the_hollow`] holds it of one: every corner
/// of its triangles on one of them, and every ray aimed from the axis out to
/// each meeting the face first.
fn is_hollow_made_of(body: &Body, face: usize, parts: &[&Hollow], what: &str) {
    let corners: Vec<DVec3> = body.triangles_of(face).flatten().collect();
    assert!(!corners.is_empty(), "{what}: the round is drawn");
    for corner in corners {
        assert!(
            parts.iter().any(|hollow| hollow.holds(corner)),
            "{what}: the round is drawn through {corner}, off the cylinder of {} about the line \
             from {} along {}",
            parts[0].radius,
            parts[0].start,
            parts[0].along
        );
    }
    for hollow in parts {
        for along in [0.11, 0.47, 0.83] {
            for share in [0.1, 0.3, 0.5, 0.7, 0.9] {
                let facing = hollow.facing_at(share);
                let from = hollow.axis_at(along);
                let hit = body.ray_hit(from, -facing).unwrap_or_else(|| {
                    panic!("{what}: a ray from {from} on the axis meets nothing")
                });
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
}

/// The lists mixing the block's outward edge and the L's inward one, each
/// with what it shows, how wide the block is, the value asked, what the
/// outward edge and the inward one each take alone, and the one value every
/// edge of the list gives.
fn mixed_lists() -> [(&'static str, f64, f64, [f64; 2], f64); 3] {
    [
        ("at a value both fit", BREADTH, 3.0, [3.0, 3.0], 3.0),
        (
            "the outward edge limiting the inward one, the block's top 5 wide taken whole",
            5.0,
            7.0,
            [5.0, 7.0],
            5.0,
        ),
        (
            "the inward edge limiting the outward one, the stem's right side taken whole",
            BREADTH,
            9.0,
            [9.0, STEM_RIGHT_LENGTH],
            STEM_RIGHT_LENGTH,
        ),
    ]
}

#[test]
fn a_fillet_and_a_chamfer_on_an_outward_edge_and_an_inward_one_cut_the_first_off_and_fill_the_second_in_at_one_value()
 {
    for (case, breadth, asked, each_alone, taken) in mixed_lists() {
        let part = ell_and_block(breadth);
        let outward = beside_top_front();
        let names = [
            outward.name(&part),
            edge_between(&part, ARM_BACK, STEM_RIGHT),
        ];
        let laid = [part.faces_end(), part.faces_end() + 1];
        let block_volume = LENGTH * breadth * HEIGHT;

        let filleted = part
            .fillet_edges(&names, asked)
            .unwrap_or_else(|declined| panic!("the fillet {case}: declined, {declined:?}"));
        let chamfered = part
            .chamfer_edges(&names, asked)
            .unwrap_or_else(|declined| panic!("the chamfer {case}: declined, {declined:?}"));

        let what = format!("the fillet {case}, asked {asked}");
        let alone: Vec<f64> = names
            .iter()
            .map(|name| part.fillet(name, asked).expect("each edge alone").taken)
            .collect();
        assert!(
            alone
                .iter()
                .zip(each_alone)
                .all(|(took, expected)| (took - expected).abs() < PLACE),
            "{what}: the outward edge and the inward one alone take {alone:?}, where they give \
             {each_alone:?}"
        );
        reduced(filleted.asked, filleted.taken, [asked, taken], &what);
        let body = &filleted.body;
        let round = outward.round(taken);
        let hollow = Hollow::in_the_ell(taken);
        is_the_round(
            body,
            laid[0],
            &round,
            &format!("{what}: the block's top front edge rounded off at {taken}"),
        );
        is_the_hollow(
            body,
            laid[1],
            &hollow,
            &format!("{what}: the L's inner corner filled in at {taken}"),
        );
        let quarter = taken * taken * (1.0 - PI / 4.0);
        sound_drawn_within(
            body,
            ELL_VOLUME + block_volume - quarter * LENGTH + quarter * ELL_HEIGHT,
            DRAWN * reach_of(&part) * (round.area() + hollow.area()),
            &what,
        );

        let what = format!("the chamfer {case}, asked {asked}");
        let alone: Vec<f64> = names
            .iter()
            .map(|name| part.chamfer(name, asked).expect("each edge alone").taken)
            .collect();
        assert!(
            alone
                .iter()
                .zip(each_alone)
                .all(|(took, expected)| (took - expected).abs() < PLACE),
            "{what}: the outward edge and the inward one alone take {alone:?}, where they give \
             {each_alone:?}"
        );
        reduced(chamfered.asked, chamfered.taken, [asked, taken], &what);
        let body = &chamfered.body;
        let (cut, facing) = outward.flat(taken);
        let which = format!("{what}: the block's top front edge cut off at {taken}");
        face_facing(body, &[laid[0]], facing, &which);
        corners_are(body, laid[0], &cut, &which);
        let on_arm_back = INNER_CORNER + DVec2::X * taken;
        let on_stem_right = INNER_CORNER + DVec2::Y * taken;
        let filled = [on_arm_back, on_stem_right]
            .map(|at| [at.extend(0.0), at.extend(ELL_HEIGHT)])
            .concat();
        let which = format!("{what}: the L's inner corner filled in at {taken}");
        face_facing(
            body,
            &[laid[1]],
            DVec3::new(1.0, 1.0, 0.0).normalize(),
            &which,
        );
        corners_are(body, laid[1], &filled, &which);
        let triangle = taken * taken / 2.0;
        let promised = ELL_VOLUME + block_volume - triangle * LENGTH + triangle * ELL_HEIGHT;
        sound_drawn_within(body, promised, EXACT * promised, &what);
    }
}

#[test]
fn a_fillet_on_a_meeting_pair_named_apart_in_the_list_lays_one_full_round_answering_to_both_their_numbers_the_edge_between_numbered_between()
 {
    let block = block([LENGTH, BREADTH, HEIGHT]);
    let [top_front, floor_back, top_back, _] = long_edges();
    let names = [
        top_front.name(&block),
        floor_back.name(&block),
        top_back.name(&block),
    ];
    let half = BREADTH / 2.0;

    let filleted = block
        .fillet_edges(&names, MORE_THAN_HALF)
        .unwrap_or_else(|declined| panic!("declined, {declined:?}"));

    let what = format!(
        "the fillet on the top's front edge, the floor's back edge and the top's back edge, asked \
         {MORE_THAN_HALF}"
    );
    reduced(
        filleted.asked,
        filleted.taken,
        [MORE_THAN_HALF, half],
        &what,
    );
    let body = &filleted.body;
    let end = block.faces_end();
    let (pair, between) = ([end, end + 2], end + 1);
    let mut kept_and_laid = numbers(&block);
    kept_and_laid.remove(&TOP);
    kept_and_laid.extend(end..end + 3);
    assert_eq!(
        numbers(body),
        kept_and_laid,
        "{what}: every face of the block but its top, the full round answering to {pair:?} and \
         the floor's back round to {between}"
    );
    assert_eq!(
        body.faces_end(),
        end + 3,
        "{what}: the next step numbers its faces past the three the edges were given"
    );
    one_curved_face_answers_to(body, &pair, &what);
    let axis = DVec3::new(0.0, half, HEIGHT - half);
    let halves = [[-DVec3::Y, DVec3::Z], [DVec3::Z, DVec3::Y]].map(|facing| Round {
        start: axis,
        along: DVec3::X,
        length: LENGTH,
        radius: half,
        facing,
    });
    is_made_of(
        body,
        pair[0],
        &[&halves[0], &halves[1]],
        &format!("{what}: the full round over where the top was"),
    );
    let floor_round = floor_back.round(half);
    assert!(
        !body.is_flat(between) && body.plane_of(between).is_none(),
        "{what}: face {between} is curved"
    );
    is_the_round(
        body,
        between,
        &floor_round,
        &format!("{what}: face {between}, the floor's back edge rounded off at {half}"),
    );
    assert_eq!(
        sides_met_by(body, &pair),
        sides(&[FRONT, RIGHT, BACK, LEFT]),
        "{what}: the full round meets the front, the back and the two walls, by one edge each"
    );
    assert_eq!(
        sides_met(body, between),
        sides(&[FLOOR, RIGHT, BACK, LEFT]),
        "{what}: the floor's back round meets the floor, the back and the two walls, by one edge \
         each"
    );
    corners_are(
        body,
        BACK,
        &strip(
            DVec3::new(0.0, BREADTH, half),
            DVec3::new(0.0, BREADTH, HEIGHT - half),
        ),
        &format!("{what}: what is left of the back, between its two rounds"),
    );
    sound_drawn_within(
        body,
        LENGTH * BREADTH * HEIGHT
            - LENGTH * (BREADTH * half - PI * half * half / 2.0)
            - LENGTH * half * half * (1.0 - PI / 4.0),
        DRAWN * reach_of(&block) * (PI * half * LENGTH + floor_round.area()),
        &what,
    );
}

/// The side of the square bar: its four long edges' set-backs meet across
/// every long face at half of it.
const BAR: f64 = 10.0;

/// A prism whose long edges' rounds close on themselves, as the closed round
/// is held on it: the part, the names of its long edges, the radius of the
/// circle its long faces stand tangent to, the point its axis starts from on
/// its first end, the way it runs and how far to the second end, how far
/// past either end the cylinder reaches where an end is slanted, two ways
/// square to the axis and to each other, its long faces, its two ends, the
/// first then the second, each with the way it faces, and the faces and the
/// matter of whatever stands apart from it, which the step leaves whole.
struct ClosingPrism {
    part: Body,
    edges: Vec<EdgeName>,
    radius: f64,
    start: DVec3,
    along: DVec3,
    length: f64,
    overhang: f64,
    across: [DVec3; 2],
    sides: Vec<usize>,
    ends: [(usize, DVec3); 2],
    apart: (Vec<usize>, f64),
}

/// The bar lying on the ground from the origin along X: a block
/// [`LENGTH`] long, [`BAR`] across and [`BAR`] high, its left wall its first
/// end.
fn lying_bar() -> ClosingPrism {
    let part = block([LENGTH, BAR, BAR]);
    let edges = long_edges_of(BAR, BAR)
        .iter()
        .map(|edge| edge.name(&part))
        .collect();
    ClosingPrism {
        part,
        edges,
        radius: BAR / 2.0,
        start: DVec3::new(0.0, BAR / 2.0, BAR / 2.0),
        along: DVec3::X,
        length: LENGTH,
        overhang: 0.0,
        across: [DVec3::Y, DVec3::Z],
        sides: vec![FLOOR, TOP, FRONT, BACK],
        ends: [(LEFT, -DVec3::X), (RIGHT, DVec3::X)],
        apart: (Vec::new(), 0.0),
    }
}

/// The bar raised from a square drawn on a plane that leans every way and
/// stands away from the origin: its floor, numbered nought, its first end,
/// its top the second, and its walls two to five, each long edge between
/// two walls that follow each other.
fn askew_bar() -> ClosingPrism {
    let frame = Frame {
        origin: DVec3::new(7.0, -3.0, 4.0),
        u: DVec3::new(2.0, 1.0, 2.0) / 3.0,
        v: DVec3::new(-1.0, 2.0, 0.0) / 5.0_f64.sqrt(),
    };
    let along = frame.normal();
    let tool = Body::default()
        .tool_raised(
            &profile(Contour::rectangle(DVec2::ZERO, DVec2::splat(BAR))),
            frame,
            along * LENGTH,
        )
        .expect("the tool raises");
    let part = Body::default().union(&tool).expect("the join is built");
    let edges = [[2, 3], [3, 4], [4, 5], [5, 2]]
        .iter()
        .map(|&[first, second]| edge_between(&part, first, second))
        .collect();
    ClosingPrism {
        part,
        edges,
        radius: BAR / 2.0,
        start: frame.at(DVec2::splat(BAR / 2.0)),
        along,
        length: LENGTH,
        overhang: 0.0,
        across: [frame.u, frame.v],
        sides: vec![2, 3, 4, 5],
        ends: [(0, -along), (1, along)],
        apart: (Vec::new(), 0.0),
    }
}

/// A prism raised [`LENGTH`] from the ground on the outline through
/// `corners`, counter-clockwise, whose sides all stand tangent to the circle
/// of `radius` about `centre`: its floor its first end, its top its second,
/// and each long edge between two walls that follow each other.
fn upright_prism(corners: Vec<DVec2>, centre: DVec2, radius: f64) -> ClosingPrism {
    let count = corners.len();
    let part = raised_with(&Body::default(), Contour::straight(corners), LENGTH);
    let sides: Vec<usize> = (2..2 + count).collect();
    let edges = (0..count)
        .map(|at| edge_between(&part, sides[at], sides[(at + 1) % count]))
        .collect();
    ClosingPrism {
        part,
        edges,
        radius,
        start: centre.extend(0.0),
        along: DVec3::Z,
        length: LENGTH,
        overhang: 0.0,
        across: [DVec3::X, DVec3::Y],
        sides,
        ends: [(FLOOR, -DVec3::Z), (TOP, DVec3::Z)],
        apart: (Vec::new(), 0.0),
    }
}

/// The corners of a regular polygon of `count` sides `side` long about
/// `centre`, counter-clockwise from the one at angle `first`, and the radius
/// of the circle its sides stand tangent to.
fn regular(count: usize, side: f64, centre: DVec2, first: f64) -> (Vec<DVec2>, f64) {
    let turn = 2.0 * PI / count as f64;
    let reach = side / 2.0 / (turn / 2.0).sin();
    let corners = (0..count)
        .map(|at| centre + DVec2::from_angle(first + turn * at as f64) * reach)
        .collect();
    (corners, side / 2.0 / (turn / 2.0).tan())
}

/// How far the slanted end of the bar of
/// [`a_fillet_on_the_four_long_edges_of_a_square_bar_with_one_end_cut_at_20_degrees_takes_half_in_one_closed_cylinder_its_slanted_end_an_ellipse`]
/// leans off square to its axis.
const SLANT: f64 = 20.0 * PI / 180.0;

/// The lying bar with its right end cut away by a plane leaning [`SLANT`]
/// about Y, through its axis [`BAR`] short of the right wall, so that no
/// part of that wall is left: the slanted face is its second end.
fn slanted_bar() -> ClosingPrism {
    let square = lying_bar();
    let half = BAR / 2.0;
    let facing = DVec3::new(SLANT.cos(), 0.0, SLANT.sin());
    let frame = Frame {
        origin: DVec3::new(LENGTH - half, half, half),
        u: DVec3::Y,
        v: facing.cross(DVec3::Y),
    };
    let slab = square
        .part
        .tool_raised(
            &profile(Contour::rectangle(
                DVec2::splat(-2.0 * BAR),
                DVec2::splat(2.0 * BAR),
            )),
            frame,
            facing * BAR,
        )
        .expect("the slab raises");
    let part = square
        .part
        .difference(&slab)
        .expect("the slanted cut is built");
    let slanted: Vec<usize> = faces_made(&square.part, &part)
        .into_iter()
        .filter(|&face| {
            part.plane_of(face)
                .is_some_and(|plane| plane.normal.distance(facing) < PLACE)
        })
        .collect();
    assert_eq!(slanted.len(), 1, "one slanted end: {slanted:?}");
    let edges = long_edges_of(BAR, BAR)
        .iter()
        .map(|edge| edge.name(&part))
        .collect();
    ClosingPrism {
        part,
        edges,
        length: LENGTH - half,
        overhang: half * SLANT.tan(),
        ends: [(LEFT, -DVec3::X), (slanted[0], facing)],
        ..square
    }
}

/// Every order a list may be named in.
fn every_order<T: Clone>(items: &[T]) -> Vec<Vec<T>> {
    if items.is_empty() {
        return vec![Vec::new()];
    }
    (0..items.len())
        .flat_map(|first| {
            let mut rest = items.to_vec();
            let head = rest.remove(first);
            every_order(&rest).into_iter().map(move |mut tail| {
                tail.insert(0, head.clone());
                tail
            })
        })
        .collect()
}

/// Holds that a fillet on the prism's long edges, named in the order
/// `order`, asked `asked`, took the radius of the circle its sides stand
/// tangent to and closed its rounds into one cylinder about its axis, each
/// end a flat face bounded by one curve on that cylinder.
fn closes_into_one_cylinder(prism: &ClosingPrism, order: &[usize], asked: f64, what: &str) {
    let radius = prism.radius;
    let names: Vec<EdgeName> = order.iter().map(|&at| prism.edges[at].clone()).collect();
    let filleted = prism
        .part
        .fillet_edges(&names, asked)
        .unwrap_or_else(|declined| panic!("{what}: declined, {declined:?}"));

    reduced(filleted.asked, filleted.taken, [asked, radius], what);
    let body = &filleted.body;
    let first = prism.part.faces_end();
    let laid: Vec<usize> = (first..first + prism.edges.len()).collect();
    let mut ends_and_laid: BTreeSet<usize> = prism.ends.iter().map(|&(end, _)| end).collect();
    ends_and_laid.extend(laid.iter().copied());
    let (apart, matter_apart) = &prism.apart;
    ends_and_laid.extend(apart.iter().copied());
    assert_eq!(
        numbers(body),
        ends_and_laid,
        "{what}: the long faces {:?} gone, the two ends, the cylinder answering to {laid:?} and \
         the faces {apart:?} of what stands apart left",
        prism.sides
    );
    assert_eq!(
        body.faces_end(),
        first + laid.len(),
        "{what}: the next step numbers its faces past every number the cylinder answers to"
    );
    one_curved_face_answers_to(body, &laid, what);
    let quarters = |start: DVec3, length: f64| {
        let [one, other] = prism.across;
        [[one, other], [other, -one], [-one, -other], [-other, one]].map(|facing| Round {
            start,
            along: prism.along,
            length,
            radius,
            facing,
        })
    };
    let overhang = prism.along * prism.overhang;
    let reaching = quarters(prism.start - overhang, prism.length + 2.0 * prism.overhang);
    let between_the_ends = quarters(prism.start + overhang, prism.length - 2.0 * prism.overhang);
    let cylinder = format!(
        "{what}: the closed round, a cylinder of {radius} about the line from {} along {}",
        prism.start, prism.along
    );
    drawn_on(body, laid[0], &reaching.each_ref(), PLACE, &cylinder);
    for round in &between_the_ends {
        met_from_outside(body, laid[0], round, PLACE, &cylinder);
    }
    for (end, (face, facing)) in prism.ends.iter().copied().enumerate() {
        is_an_end(
            body,
            face,
            [
                prism.start + prism.along * prism.length * end as f64,
                facing,
            ],
            prism,
            &laid,
            &format!("{what}: the end numbered {face}"),
        );
    }
    sound_drawn_within(
        body,
        PI * radius * radius * prism.length + matter_apart,
        DRAWN * reach_of(&prism.part) * 2.0 * PI * radius * (prism.length + prism.overhang),
        what,
    );
}

/// Holds that the face numbered `face` is an end of the closed round: flat
/// on the plane through `centre` facing `facing`, every corner on the
/// cylinder of the prism's radius about its axis, its triangles covering
/// what the cylinder cuts from the plane — a disc where the end is square
/// to the axis, an ellipse where it leans — and one edge only, shared with
/// the cylinder answering to the numbers `rim`.
fn is_an_end(
    body: &Body,
    face: usize,
    [centre, facing]: [DVec3; 2],
    prism: &ClosingPrism,
    rim: &[usize],
    what: &str,
) {
    let radius = prism.radius;
    let plane = body
        .plane_of(face)
        .unwrap_or_else(|| panic!("{what}: flat, a plane to draw on"));
    assert!(
        plane.normal.distance(facing) < PLACE,
        "{what}: faces {}, where the end faces {facing}",
        plane.normal
    );
    assert!(!plane.corners.is_empty(), "{what}: has a rim");
    for corner in &plane.corners {
        let off = *corner - centre;
        let from_the_axis = off - prism.along * off.dot(prism.along);
        assert!(
            off.dot(facing).abs() < PLACE && (from_the_axis.length() - radius).abs() < PLACE,
            "{what}: a corner at {corner}, off the cylinder of {radius} about the line from {} \
             along {}",
            prism.start,
            prism.along
        );
    }
    let mut met: Vec<u32> = rim.iter().map(|&number| number as u32).collect();
    met.sort();
    assert_eq!(
        sides_met(body, face),
        vec![met],
        "{what}: one edge, shared with the cylinder and nothing else"
    );
    let area: f64 = body
        .triangles_of(face)
        .map(|[a, b, c]| (b - a).cross(c - a).length() / 2.0)
        .sum();
    let cut = PI * radius * radius / facing.dot(prism.along).abs();
    assert!(
        (area - cut).abs()
            <= DRAWN * reach_of(body) * 2.0 * PI * radius / facing.dot(prism.along).abs(),
        "{what}: its triangles cover {area}, where the cylinder cuts {cut} from its plane"
    );
}

/// Holds that a fillet on the prism's long edges closes into one cylinder,
/// as [`closes_into_one_cylinder`] holds it, asked the radius its sides
/// stand tangent to and asked more, named in each order of `orders`.
fn closes_in_each_order(prism: &ClosingPrism, orders: &[Vec<usize>], shape: &str) {
    for asked in [prism.radius, MORE_THAN_HALF] {
        for order in orders {
            closes_into_one_cylinder(
                prism,
                order,
                asked,
                &format!(
                    "the fillet on the long edges of {shape}, named in the order {order:?}, \
                     asked {asked}"
                ),
            );
        }
    }
}

#[test]
fn a_fillet_on_the_four_long_edges_of_a_square_bar_asked_half_its_side_or_more_takes_half_in_one_closed_cylinder_in_every_order()
 {
    closes_in_each_order(
        &lying_bar(),
        &every_order(&[0, 1, 2, 3]),
        &format!("a bar {BAR} square lying along X"),
    );
}

#[test]
fn a_fillet_on_the_four_long_edges_of_a_square_bar_standing_askew_away_from_the_origin_takes_half_in_one_closed_cylinder_about_its_own_axis()
 {
    let bar = askew_bar();
    let shape = format!("a bar {BAR} square standing askew from {}", bar.start);
    closes_in_each_order(&bar, &every_order(&[0, 1, 2, 3]), &shape);
}

#[test]
fn a_fillet_on_the_three_long_edges_of_a_regular_triangular_prism_asked_its_inradius_or_more_takes_it_in_one_closed_cylinder()
 {
    let (corners, inradius) = regular(3, 12.0, DVec2::new(3.0, 4.0), -PI / 2.0);
    assert!(
        (inradius - 2.0 * 3.0_f64.sqrt()).abs() < PLACE && inradius < MORE_THAN_HALF,
        "a triangle 12 on a side stands tangent to a circle of 2√3, which 8 is more than"
    );
    closes_in_each_order(
        &upright_prism(corners, DVec2::new(3.0, 4.0), inradius),
        &every_order(&[0, 1, 2]),
        "a regular triangular prism 12 on a side",
    );
}

#[test]
fn a_fillet_on_the_four_long_edges_of_a_60_degree_rhombic_prism_asked_its_inradius_or_more_takes_it_in_one_closed_cylinder()
 {
    let side = 10.0;
    let slant = DVec2::from_angle(PI / 3.0) * side;
    let corners = vec![DVec2::ZERO, DVec2::X * side, DVec2::X * side + slant, slant];
    let inradius = side * (PI / 3.0).sin() / 2.0;
    closes_in_each_order(
        &upright_prism(corners, (DVec2::X * side + slant) / 2.0, inradius),
        &every_order(&[0, 1, 2, 3]),
        "a prism on a rhombus 10 on a side, its corners at 60 and 120 degrees",
    );
}

#[test]
fn a_fillet_on_the_six_long_edges_of_a_regular_hexagonal_bar_asked_its_inradius_or_more_takes_it_in_one_closed_cylinder()
 {
    let centre = DVec2::new(-4.0, 9.0);
    let (corners, inradius) = regular(6, 6.0, centre, 0.3);
    assert!(
        (inradius - 3.0 * 3.0_f64.sqrt()).abs() < PLACE && inradius < MORE_THAN_HALF,
        "a hexagon 6 on a side stands tangent to a circle of 3√3, which 8 is more than"
    );
    closes_in_each_order(
        &upright_prism(corners, centre, inradius),
        &every_order(&[0, 1, 2, 3, 4, 5]),
        "a regular hexagonal bar 6 on a side",
    );
}

#[test]
fn a_fillet_on_the_four_long_edges_of_a_square_bar_with_one_end_cut_at_20_degrees_takes_half_in_one_closed_cylinder_its_slanted_end_an_ellipse()
 {
    closes_in_each_order(
        &slanted_bar(),
        &every_order(&[0, 1, 2, 3]),
        &format!("a bar {BAR} square lying along X, its right end cut at 20 degrees"),
    );
}

/// The lying bar with a block standing apart a little past each of its ends,
/// across its axis, and one standing well away beside it, along it: none of
/// them touches the bar, so its ends are its rounds' alone.
fn bar_among_blocks() -> ClosingPrism {
    let bar = lying_bar();
    let (beside, past_the_left, past_the_right) = (
        [DVec2::new(5.0, 25.0), DVec2::new(25.0, 30.0)],
        [DVec2::new(-6.0, 3.0), DVec2::new(-1.0, 7.0)],
        [DVec2::new(LENGTH + 1.0, 3.0), DVec2::new(LENGTH + 6.0, 7.0)],
    );
    let part = [past_the_left, past_the_right, beside]
        .iter()
        .fold(bar.part.clone(), |part, &corners| {
            joined_to(&part, corners, BAR)
        });
    let edges = long_edges_of(BAR, BAR)
        .iter()
        .map(|edge| edge.name(&part))
        .collect();
    let apart = (bar.part.faces_end()..part.faces_end()).collect();
    ClosingPrism {
        part,
        edges,
        apart: (apart, (2.0 * 5.0 * 4.0 + 20.0 * 5.0) * BAR),
        ..bar
    }
}

#[test]
fn a_fillet_on_the_four_long_edges_of_a_square_bar_with_blocks_standing_apart_past_its_ends_and_beside_it_takes_half_and_leaves_them_whole()
 {
    let among = bar_among_blocks();
    let half = BAR / 2.0;
    let laid: Vec<usize> = (among.part.faces_end()..among.part.faces_end() + 4).collect();
    let (apart, matter_apart) = &among.apart;
    for asked in [half, MORE_THAN_HALF] {
        for order in every_order(&[0, 1, 2, 3]) {
            let what = format!(
                "the fillet on the long edges of a bar {BAR} square lying along X, a block \
                 standing 1 past each end and one 15 beside it, named in the order {order:?}, \
                 asked {asked}"
            );
            let names: Vec<EdgeName> = order.iter().map(|&at| among.edges[at].clone()).collect();
            let filleted = among
                .part
                .fillet_edges(&names, asked)
                .unwrap_or_else(|declined| panic!("{what}: declined, {declined:?}"));
            reduced(filleted.asked, filleted.taken, [asked, half], &what);
            let body = &filleted.body;
            let mut left: BTreeSet<usize> = [LEFT, RIGHT].into();
            left.extend(laid.iter().copied());
            left.extend(apart.iter().copied());
            assert_eq!(
                numbers(body),
                left,
                "{what}: the bar's ends, the cylinder answering to {laid:?}, and every face of \
                 the blocks apart, {apart:?}"
            );
            one_curved_face_answers_to(body, &laid, &what);
            sound_drawn_within(
                body,
                PI * half * half * LENGTH + matter_apart,
                DRAWN * reach_of(&among.part) * 2.0 * PI * half * LENGTH,
                &format!("{what}: the cylinder and the three blocks whole"),
            );
        }
    }
}

#[test]
fn a_fillet_on_the_four_long_edges_of_a_square_bar_among_blocks_standing_apart_closes_into_one_cylinder_each_end_bounded_by_one_circle()
 {
    closes_in_each_order(
        &bar_among_blocks(),
        &every_order(&[0, 1, 2, 3]),
        &format!(
            "a bar {BAR} square lying along X, a block standing 1 past each end and one 15 \
             beside it"
        ),
    );
}

/// Holds that a fillet on the four edges `edges` of `part`, each `length`
/// long, is declined `Unsupported` asked half the side [`BAR`] or more, in
/// every order, and asked less lays four rounds, each taking from the part
/// what a round of that value takes along a straight edge between square
/// faces.
fn declined_from_half_and_laid_below(part: &Body, edges: &[EdgeName], length: f64, what: &str) {
    for asked in [BAR / 2.0, MORE_THAN_HALF] {
        for order in every_order(&[0, 1, 2, 3]) {
            let names: Vec<EdgeName> = order.iter().map(|&at| edges[at].clone()).collect();
            assert_eq!(
                part.fillet_edges(&names, asked).err(),
                Some(Declined::Unsupported),
                "{what}, named in the order {order:?}, asked {asked}: declined, the rounds would \
                 close on an end that is not theirs alone"
            );
        }
    }
    let value = 4.0;
    let filleted = part
        .fillet_edges(edges, value)
        .unwrap_or_else(|declined| panic!("{what}, asked {value}: declined, {declined:?}"));
    let what = format!("{what}, asked {value}");
    taken_as_asked(filleted.asked, filleted.taken, value, &what);
    let body = &filleted.body;
    assert_eq!(
        faces_made(part, body).len(),
        4,
        "{what}: four rounds, one for each edge"
    );
    sound_drawn_within(
        body,
        part.volume() - 4.0 * length * value * value * (1.0 - PI / 4.0),
        DRAWN * reach_of(part) * 4.0 * PI / 2.0 * value * length,
        &what,
    );
}

#[test]
fn a_fillet_on_the_four_long_edges_of_a_square_cavity_closed_inside_a_block_asked_half_its_side_or_more_is_declined_as_unsupported_and_less_fills_four_hollows()
 {
    let side = 30.0;
    let block = block([side, side, side]);
    let cavity = block
        .tool_raised(
            &profile(Contour::rectangle(
                DVec2::new(5.0, 10.0),
                DVec2::new(5.0 + 20.0, 10.0 + BAR),
            )),
            Frame {
                origin: DVec3::Z * 10.0,
                ..ground()
            },
            DVec3::Z * BAR,
        )
        .expect("the cavity raises");
    let part = block.difference(&cavity).expect("the cavity is cut");
    let [floor, top, near, far] = [6, 7, 8, 10];
    let edges: Vec<EdgeName> = [[floor, near], [top, far], [top, near], [floor, far]]
        .iter()
        .map(|&[first, second]| edge_between(&part, first, second))
        .collect();
    let what = format!(
        "the fillet on the long edges of a cavity {BAR} square, 20 long, closed inside a block"
    );
    for asked in [BAR / 2.0, MORE_THAN_HALF] {
        for order in every_order(&[0, 1, 2, 3]) {
            let names: Vec<EdgeName> = order.iter().map(|&at| edges[at].clone()).collect();
            assert_eq!(
                part.fillet_edges(&names, asked).err(),
                Some(Declined::Unsupported),
                "{what}, named in the order {order:?}, asked {asked}: declined, the hollows \
                 would close on themselves and fills do not"
            );
        }
    }
    let value = 4.0;
    let filleted = part
        .fillet_edges(&edges, value)
        .unwrap_or_else(|declined| panic!("{what}, asked {value}: declined, {declined:?}"));
    let what = format!("{what}, asked {value}");
    taken_as_asked(filleted.asked, filleted.taken, value, &what);
    let body = &filleted.body;
    assert_eq!(
        faces_made(&part, body).len(),
        4,
        "{what}: four hollows, one for each edge"
    );
    sound_drawn_within(
        body,
        part.volume() + 4.0 * 20.0 * value * value * (1.0 - PI / 4.0),
        DRAWN * reach_of(&part) * 4.0 * PI / 2.0 * value * 20.0,
        &what,
    );
}

#[test]
fn a_fillet_on_the_four_upright_edges_of_a_square_post_standing_on_a_plate_asked_half_its_side_or_more_is_declined_as_unsupported_and_less_lays_four_rounds()
 {
    let plate = block([LENGTH, LENGTH, 4.0]);
    let part = joined_to(&plate, [DVec2::splat(10.0), DVec2::splat(10.0 + BAR)], 24.0);
    let walls: Vec<usize> = (plate.faces_end() + 2..plate.faces_end() + 6).collect();
    let edges: Vec<EdgeName> = (0..4)
        .map(|at| edge_between(&part, walls[at], walls[(at + 1) % 4]))
        .collect();
    declined_from_half_and_laid_below(
        &part,
        &edges,
        20.0,
        &format!("the fillet on the upright edges of a post {BAR} square standing on a plate"),
    );
}

#[test]
fn a_fillet_on_the_four_long_edges_of_a_square_bar_with_a_boss_on_one_end_asked_half_its_side_or_more_is_declined_as_unsupported_and_less_lays_four_rounds()
 {
    let bar = block([LENGTH, BAR, BAR]);
    let end = Frame {
        origin: DVec3::X * LENGTH,
        u: DVec3::Y,
        v: DVec3::Z,
    };
    let boss = bar
        .tool_raised(
            &profile(Contour::rectangle(DVec2::splat(4.0), DVec2::splat(6.0))),
            end,
            DVec3::X * 5.0,
        )
        .expect("the boss raises");
    let part = bar.union(&boss).expect("the boss is joined");
    let edges: Vec<EdgeName> = long_edges_of(BAR, BAR)
        .iter()
        .map(|edge| edge.name(&part))
        .collect();
    declined_from_half_and_laid_below(
        &part,
        &edges,
        LENGTH,
        &format!("the fillet on the long edges of a bar {BAR} square with a boss on its right end"),
    );
}

#[test]
fn a_fillet_on_the_four_long_edges_of_a_square_tube_asked_half_its_side_or_more_is_declined_as_unsupported_and_less_lays_four_rounds()
 {
    let bar = block([LENGTH, BAR, BAR]);
    let behind_the_left_end = Frame {
        origin: -DVec3::X,
        u: DVec3::Y,
        v: DVec3::Z,
    };
    let bore = bar
        .tool_raised(
            &profile(Contour::circle(DVec2::splat(BAR / 2.0), 1.5)),
            behind_the_left_end,
            DVec3::X * (LENGTH + 2.0),
        )
        .expect("the bore raises");
    let part = bar.difference(&bore).expect("the bore is cut");
    let edges: Vec<EdgeName> = long_edges_of(BAR, BAR)
        .iter()
        .map(|edge| edge.name(&part))
        .collect();
    declined_from_half_and_laid_below(
        &part,
        &edges,
        LENGTH,
        &format!("the fillet on the long edges of a tube {BAR} square, bored through"),
    );
}

#[test]
fn a_fillet_and_a_chamfer_on_the_four_long_edges_of_a_square_bar_asked_less_than_half_its_side_lay_each_at_the_value_asked()
 {
    let value = 4.0;
    let bar = block([LENGTH, BAR, BAR]);
    let edges = long_edges_of(BAR, BAR);
    let names: Vec<EdgeName> = edges.iter().map(|edge| edge.name(&bar)).collect();
    let laid = bar.faces_end()..bar.faces_end() + edges.len();

    let filleted = bar
        .fillet_edges(&names, value)
        .unwrap_or_else(|declined| panic!("the fillet: declined, {declined:?}"));
    let chamfered = bar
        .chamfer_edges(&names, value)
        .unwrap_or_else(|declined| panic!("the chamfer: declined, {declined:?}"));

    let what = format!("the fillet on the four long edges of a bar {BAR} square, asked {value}");
    taken_as_asked(filleted.asked, filleted.taken, value, &what);
    let body = &filleted.body;
    let mut area = 0.0;
    for (edge, number) in edges.iter().zip(laid.clone()) {
        let round = edge.round(value);
        is_the_round(
            body,
            number,
            &round,
            &format!(
                "{what}: face {number}, the round between faces {:?}",
                edge.faces
            ),
        );
        area += round.area();
    }
    sound_drawn_within(
        body,
        LENGTH * BAR * BAR - 4.0 * LENGTH * value * value * (1.0 - PI / 4.0),
        DRAWN * reach_of(&bar) * area,
        &what,
    );
    corners_are(
        body,
        TOP,
        &strip(
            DVec3::new(0.0, value, BAR),
            DVec3::new(0.0, BAR - value, BAR),
        ),
        &format!("{what}: what is left of the top, a strip between two rounds"),
    );

    let what = format!("the chamfer on the four long edges of a bar {BAR} square, asked {value}");
    taken_as_asked(chamfered.asked, chamfered.taken, value, &what);
    let body = &chamfered.body;
    for (edge, number) in edges.iter().zip(laid) {
        let (corners, facing) = edge.flat(value);
        let which = format!(
            "{what}: face {number}, the flat between faces {:?}",
            edge.faces
        );
        face_facing(body, &[number], facing, &which);
        corners_are(body, number, &corners, &which);
    }
    let promised = LENGTH * BAR * BAR - 4.0 * LENGTH * value * value / 2.0;
    sound_drawn_within(body, promised, EXACT * promised, &what);
    corners_are(
        body,
        TOP,
        &strip(
            DVec3::new(0.0, value, BAR),
            DVec3::new(0.0, BAR - value, BAR),
        ),
        &format!("{what}: what is left of the top, a strip between two flats"),
    );
}

/// How far across the slotted block reaches, and where the slot's two walls
/// stand across it, 10 apart, and how high its floor is.
const SLOTTED_BREADTH: f64 = 30.0;
const SLOT_WALLS: [f64; 2] = [10.0, 20.0];
const SLOT_FLOOR_HEIGHT: f64 = 10.0;

/// The numbers the cut that makes the slot gives its floor and its two walls,
/// past the block's six: its floor six, the wall at y = 10 eight and the
/// wall at y = 20 ten. Its top, its right and its left stand outside the
/// block and are gone.
const SLOT_FLOOR: usize = 6;
const SLOT_NEAR_WALL: usize = 8;
const SLOT_FAR_WALL: usize = 10;

/// A block [`LENGTH`] long, [`SLOTTED_BREADTH`] across and [`HEIGHT`] high,
/// with a slot cut along its whole length between [`SLOT_WALLS`], from its top
/// down to [`SLOT_FLOOR_HEIGHT`]. The slot's two floor edges meet inwards,
/// facing each other across its floor 10 wide, and run from the block's left
/// wall to its right, where the slot opens.
fn slotted() -> Body {
    let solid = block([LENGTH, SLOTTED_BREADTH, HEIGHT]);
    let [near, far] = SLOT_WALLS;
    let tool = solid
        .tool_raised(
            &profile(Contour::rectangle(
                DVec2::new(-1.0, near),
                DVec2::new(LENGTH + 1.0, far),
            )),
            Frame {
                origin: DVec3::Z * SLOT_FLOOR_HEIGHT,
                ..ground()
            },
            DVec3::Z * HEIGHT,
        )
        .expect("the tool raises");
    solid.difference(&tool).expect("the slot is cut")
}

/// The matter the slotted block holds.
const SLOTTED_VOLUME: f64 = LENGTH
    * (SLOTTED_BREADTH * HEIGHT - (SLOT_WALLS[1] - SLOT_WALLS[0]) * (HEIGHT - SLOT_FLOOR_HEIGHT));

/// The names of the slot's two floor edges, the near wall's first.
fn slot_floor_edges(slotted: &Body) -> [EdgeName; 2] {
    [
        edge_between(slotted, SLOT_FLOOR, SLOT_NEAR_WALL),
        edge_between(slotted, SLOT_FLOOR, SLOT_FAR_WALL),
    ]
}

/// The corners, on the left wall, of what is left of each of the slot's walls
/// once its floor is filled in to `up` above it.
fn slot_walls_from(up: f64) -> [[DVec3; 4]; 2] {
    SLOT_WALLS.map(|across| {
        strip(
            DVec3::new(0.0, across, SLOT_FLOOR_HEIGHT + up),
            DVec3::new(0.0, across, HEIGHT),
        )
    })
}

#[test]
fn a_fillet_on_the_two_floor_edges_of_a_slot_10_wide_asked_5_or_more_takes_5_in_one_round_bottom() {
    let slotted = slotted();
    let names = slot_floor_edges(&slotted);
    let half = (SLOT_WALLS[1] - SLOT_WALLS[0]) / 2.0;
    alone_each_takes_more(
        |name| {
            slotted
                .fillet(name, MORE_THAN_HALF)
                .map(|single| single.taken)
        },
        &names,
        half,
        "the fillet on one floor edge of the slot",
    );

    for asked in [MORE_THAN_HALF, half] {
        let filleted = slotted
            .fillet_edges(&names, asked)
            .unwrap_or_else(|declined| panic!("asked {asked}: declined, {declined:?}"));

        let what = format!("the fillet on both floor edges of a slot 10 wide, asked {asked}");
        reduced(filleted.asked, filleted.taken, [asked, half], &what);
        let body = &filleted.body;
        let laid = [slotted.faces_end(), slotted.faces_end() + 1];
        assert!(
            !body.has_face(SLOT_FLOOR),
            "{what}: the slot's floor is gone"
        );
        let mut kept_and_laid = numbers(&slotted);
        kept_and_laid.remove(&SLOT_FLOOR);
        kept_and_laid.extend(laid);
        assert_eq!(
            numbers(body),
            kept_and_laid,
            "{what}: every face of the slotted block but the slot's floor, and the round bottom \
             answering to {laid:?}"
        );
        assert_eq!(
            body.faces_end(),
            laid[1] + 1,
            "{what}: the next step numbers its faces past both the round bottom answers to"
        );
        one_curved_face_answers_to(body, &laid, &what);
        let axis = DVec3::new(
            0.0,
            (SLOT_WALLS[0] + SLOT_WALLS[1]) / 2.0,
            SLOT_FLOOR_HEIGHT + half,
        );
        let halves = [[DVec3::Y, DVec3::Z], [DVec3::Z, -DVec3::Y]].map(|facing| Hollow {
            start: axis,
            along: DVec3::X,
            length: LENGTH,
            radius: half,
            facing,
        });
        is_hollow_made_of(
            body,
            laid[0],
            &[&halves[0], &halves[1]],
            &format!(
                "{what}: the round bottom, a hollow half-cylinder of {half} about the line {half} \
                 above the middle of the floor, tangent to both walls"
            ),
        );
        assert!(
            body.edge_names()
                .iter()
                .all(|name| !lies_between(name, laid[0], laid[1])),
            "{what}: no edge stands between {} and {}: they are one face",
            laid[0],
            laid[1]
        );
        assert_eq!(
            sides_met_by(body, &laid),
            sides(&[RIGHT, LEFT, SLOT_NEAR_WALL, SLOT_FAR_WALL]),
            "{what}: the round bottom meets the slot's two walls along its sides, and the block's \
             right and left walls at its ends, where the slot opens, by one edge each"
        );
        for (wall, corners) in [SLOT_NEAR_WALL, SLOT_FAR_WALL]
            .into_iter()
            .zip(slot_walls_from(half))
        {
            corners_are(
                body,
                wall,
                &corners,
                &format!("{what}: what is left of the slot's wall {wall}, above the round bottom"),
            );
        }
        let [near, far] = SLOT_WALLS;
        for (wall, x) in [(LEFT, 0.0), (RIGHT, LENGTH)] {
            corners_are_or_on(
                body,
                wall,
                &[
                    DVec3::new(x, 0.0, 0.0),
                    DVec3::new(x, SLOTTED_BREADTH, 0.0),
                    DVec3::new(x, SLOTTED_BREADTH, HEIGHT),
                    DVec3::new(x, far, HEIGHT),
                    DVec3::new(x, far, SLOT_FLOOR_HEIGHT + half),
                    DVec3::new(x, near, SLOT_FLOOR_HEIGHT + half),
                    DVec3::new(x, near, HEIGHT),
                    DVec3::new(x, 0.0, HEIGHT),
                ],
                |corner| halves.iter().any(|hollow| hollow.holds(corner)),
                &format!("{what}: the wall at x = {x}, the slot's bottom a half-circle on it"),
            );
        }
        sound_drawn_within(
            body,
            SLOTTED_VOLUME + LENGTH * ((far - near) * half - PI * half * half / 2.0),
            DRAWN * reach_of(&slotted) * PI * half * LENGTH,
            &what,
        );
    }
}

#[test]
fn a_chamfer_on_the_two_floor_edges_of_a_slot_10_wide_asked_more_than_5_takes_5_its_flats_meeting_in_a_valley()
 {
    let slotted = slotted();
    let names = slot_floor_edges(&slotted);
    let half = (SLOT_WALLS[1] - SLOT_WALLS[0]) / 2.0;
    alone_each_takes_more(
        |name| {
            slotted
                .chamfer(name, MORE_THAN_HALF)
                .map(|single| single.taken)
        },
        &names,
        half,
        "the chamfer on one floor edge of the slot",
    );

    let chamfered = slotted
        .chamfer_edges(&names, MORE_THAN_HALF)
        .unwrap_or_else(|declined| panic!("declined, {declined:?}"));

    let what = format!("the chamfer on both floor edges of a slot 10 wide, asked {MORE_THAN_HALF}");
    reduced(
        chamfered.asked,
        chamfered.taken,
        [MORE_THAN_HALF, half],
        &what,
    );
    let body = &chamfered.body;
    let laid = [slotted.faces_end(), slotted.faces_end() + 1];
    assert!(
        !body.has_face(SLOT_FLOOR),
        "{what}: the slot's floor is gone"
    );
    let mut kept_and_laid = numbers(&slotted);
    kept_and_laid.remove(&SLOT_FLOOR);
    kept_and_laid.extend(laid);
    assert_eq!(
        numbers(body),
        kept_and_laid,
        "{what}: every face of the slotted block but the slot's floor, and a flat for each edge"
    );
    let [near, far] = SLOT_WALLS;
    let middle = (near + far) / 2.0;
    let valley = DVec3::new(0.0, middle, SLOT_FLOOR_HEIGHT);
    for (number, across, facing) in [
        (laid[0], near, DVec3::new(0.0, 1.0, 1.0)),
        (laid[1], far, DVec3::new(0.0, -1.0, 1.0)),
    ] {
        let which = format!("{what}: face {number}, the flat from the wall at y = {across}");
        face_facing(body, &[number], facing.normalize(), &which);
        corners_are(
            body,
            number,
            &strip(valley, DVec3::new(0.0, across, SLOT_FLOOR_HEIGHT + half)),
            &format!("{which}, down to the valley at the middle of the floor"),
        );
    }
    edge_between(body, laid[0], laid[1]);
    assert_eq!(
        sides_met(body, laid[0]),
        sides(&[RIGHT, LEFT, SLOT_NEAR_WALL, laid[1]]),
        "{what}: the near flat meets its wall, the block's two walls, and the far flat along the \
         valley, by one edge each"
    );
    assert_eq!(
        sides_met(body, laid[1]),
        sides(&[RIGHT, SLOT_FAR_WALL, LEFT, laid[0]]),
        "{what}: the far flat meets its wall, the block's two walls, and the near flat along the \
         valley, by one edge each"
    );
    for (wall, corners) in [SLOT_NEAR_WALL, SLOT_FAR_WALL]
        .into_iter()
        .zip(slot_walls_from(half))
    {
        corners_are(
            body,
            wall,
            &corners,
            &format!("{what}: what is left of the slot's wall {wall}, above its flat"),
        );
    }
    for (wall, x) in [(LEFT, 0.0), (RIGHT, LENGTH)] {
        corners_are(
            body,
            wall,
            &[
                DVec3::new(x, 0.0, 0.0),
                DVec3::new(x, SLOTTED_BREADTH, 0.0),
                DVec3::new(x, SLOTTED_BREADTH, HEIGHT),
                DVec3::new(x, far, HEIGHT),
                DVec3::new(x, far, SLOT_FLOOR_HEIGHT + half),
                DVec3::new(x, middle, SLOT_FLOOR_HEIGHT),
                DVec3::new(x, near, SLOT_FLOOR_HEIGHT + half),
                DVec3::new(x, near, HEIGHT),
                DVec3::new(x, 0.0, HEIGHT),
            ],
            &format!("{what}: the wall at x = {x}, the slot a V on it"),
        );
    }
    let promised = SLOTTED_VOLUME + LENGTH * (far - near) * half / 2.0;
    sound_drawn_within(body, promised, EXACT * promised, &what);
}

/// The part with a block standing on the ground between the corners `from`
/// and `to`, raised from `from_height` by `height`, cut from it as `cao_part`
/// cuts a step: the tool's faces numbered past the part's.
fn cut_from(part: &Body, [from, to]: [DVec2; 2], from_height: f64, height: f64) -> Body {
    let tool = part
        .tool_raised(
            &profile(Contour::rectangle(from, to)),
            Frame {
                origin: DVec3::Z * from_height,
                ..ground()
            },
            DVec3::Z * height,
        )
        .expect("the tool raises");
    part.difference(&tool).expect("the cut is built")
}

/// A block of a length, a breadth and a height whose corner stands at
/// `origin`, numbered as [`block`] numbers it.
fn block_at(origin: DVec3, [length, breadth, height]: [f64; 3]) -> Body {
    let part = Body::default();
    let tool = part
        .tool_raised(
            &profile(Contour::rectangle(DVec2::ZERO, DVec2::new(length, breadth))),
            Frame { origin, ..ground() },
            DVec3::Z * height,
        )
        .expect("the tool raises");
    part.union(&tool).expect("the part is built")
}

#[test]
fn a_fillet_and_a_chamfer_on_a_list_holding_an_edge_its_name_no_longer_finds_are_declined_with_that_edge_s_reason()
 {
    let drawn = block([LENGTH, BREADTH, BREADTH]);
    let floor_back = edge_between(&drawn, FLOOR, BACK);
    let top_front = edge_between(&drawn, TOP, FRONT);
    let rabbeted = cut_from(
        &drawn,
        [DVec2::new(-1.0, -1.0), DVec2::new(LENGTH + 1.0, 2.0)],
        8.0,
        3.0,
    );
    let notched = cut_from(
        &drawn,
        [DVec2::new(10.0, -1.0), DVec2::new(20.0, 3.0)],
        7.0,
        4.0,
    );

    for (what, part, lost) in [
        (
            "a rabbet all along the top's front edge",
            rabbeted,
            Lost::Gone,
        ),
        (
            "a notch a third of the way along the top's front edge",
            notched,
            Lost::Split { now: 2 },
        ),
    ] {
        assert_eq!(part.lost(&top_front), Some(lost), "{what}");
        assert_eq!(
            part.lost(&floor_back),
            None,
            "{what}: the floor's back edge is still found"
        );
        let names = [floor_back.clone(), top_front.clone()];

        let filleted = part.fillet_edges(&names, 1.0).err();
        let chamfered = part.chamfer_edges(&names, 1.0).err();

        assert_eq!(
            filleted,
            Some(Declined::Lost(lost)),
            "the fillet after {what}"
        );
        assert_eq!(
            chamfered,
            Some(Declined::Lost(lost)),
            "the chamfer after {what}"
        );
    }
}

#[test]
fn a_fillet_and_a_chamfer_on_two_names_finding_the_same_edge_are_declined_as_unsupported() {
    let block = block([LENGTH, BREADTH, HEIGHT]);
    let top_front = edge_between(&block, TOP, FRONT);
    let half = LENGTH / 2.0;
    let second_from = |from: f64| {
        joined_to(
            &joined_to(
                &Body::default(),
                [DVec2::ZERO, DVec2::new(half, BREADTH)],
                BREADTH,
            ),
            [DVec2::new(from, 0.0), DVec2::new(from + half, BREADTH)],
            BREADTH,
        )
    };
    let apart = second_from(half + 5.0);
    let bridged = [
        edge_between(&apart, TOP, FRONT),
        edge_between(&apart, TOP + 6, FRONT + 6),
    ];
    let abutting = second_from(half);
    for name in &bridged {
        assert_eq!(
            abutting.lost(name),
            None,
            "the names of both blocks' top front edges find the edge the join bridged"
        );
    }

    for (what, part, names) in [
        (
            "the top's front edge named twice",
            &block,
            [top_front.clone(), top_front.clone()],
        ),
        (
            "both blocks' top front edges, which a join bridged into one",
            &abutting,
            bridged.clone(),
        ),
    ] {
        let filleted = part.fillet_edges(&names, 2.0).err();
        let chamfered = part.chamfer_edges(&names, 2.0).err();

        assert_eq!(
            filleted,
            Some(Declined::Unsupported),
            "the fillet on {what}"
        );
        assert_eq!(
            chamfered,
            Some(Declined::Unsupported),
            "the chamfer on {what}"
        );
    }
}

#[test]
fn a_fillet_and_a_chamfer_on_several_edges_take_the_edges_their_names_find_again_after_a_change_of_size()
 {
    let value = 3.0;
    let drawn = block([LENGTH, BREADTH, HEIGHT]);
    let names: Vec<EdgeName> = long_edges().iter().map(|edge| edge.name(&drawn)).collect();
    let (breadth, height) = (14.0, 26.0);

    for (what, origin, [breadth, height]) in [
        ("as drawn", DVec3::ZERO, [BREADTH, HEIGHT]),
        ("resized", DVec3::ZERO, [breadth, height]),
        (
            "resized and moved",
            DVec3::new(100.0, -40.0, 7.0),
            [breadth, height],
        ),
    ] {
        let part = block_at(origin, [LENGTH, breadth, height]);
        let edges = long_edges_of(breadth, height).map(|edge| edge.moved(origin));
        let laid = part.faces_end()..part.faces_end() + edges.len();

        let filleted = part
            .fillet_edges(&names, value)
            .unwrap_or_else(|declined| panic!("the fillet {what}: declined, {declined:?}"));
        let chamfered = part
            .chamfer_edges(&names, value)
            .unwrap_or_else(|declined| panic!("the chamfer {what}: declined, {declined:?}"));

        let fillet = format!("the fillet {what}");
        taken_as_asked(filleted.asked, filleted.taken, value, &fillet);
        let mut area = 0.0;
        for (edge, number) in edges.iter().zip(laid.clone()) {
            let round = edge.round(value);
            is_the_round(
                &filleted.body,
                number,
                &round,
                &format!(
                    "{fillet}: face {number}, the round between faces {:?}",
                    edge.faces
                ),
            );
            area += round.area();
        }
        sound_drawn_within(
            &filleted.body,
            LENGTH * breadth * height - 4.0 * LENGTH * value * value * (1.0 - PI / 4.0),
            DRAWN * reach_of(&part) * area,
            &fillet,
        );

        let chamfer = format!("the chamfer {what}");
        taken_as_asked(chamfered.asked, chamfered.taken, value, &chamfer);
        for (edge, number) in edges.iter().zip(laid) {
            let (corners, facing) = edge.flat(value);
            let which = format!(
                "{chamfer}: face {number}, the flat between faces {:?}",
                edge.faces
            );
            face_facing(&chamfered.body, &[number], facing, &which);
            corners_are(&chamfered.body, number, &corners, &which);
        }
        let promised = LENGTH * breadth * height - 4.0 * LENGTH * value * value / 2.0;
        sound_drawn_within(&chamfered.body, promised, EXACT * promised, &chamfer);
    }
}

#[test]
fn a_fillet_and_a_chamfer_on_a_list_of_one_edge_give_the_body_and_the_value_the_step_of_that_edge_gives()
 {
    let block = block([LENGTH, BREADTH, HEIGHT]);
    let ell = raised_with(
        &Body::default(),
        Contour::straight(ELL_OUTLINE.to_vec()),
        ELL_HEIGHT,
    );
    let rabbeted = cut_from(
        &block,
        [DVec2::new(-1.0, -1.0), DVec2::new(LENGTH + 1.0, 2.0)],
        HEIGHT - 2.0,
        3.0,
    );
    let top_front = edge_between(&block, TOP, FRONT);
    let inner = edge_between(&ell, ARM_BACK, STEM_RIGHT);

    for (what, part, edge, value) in [
        (
            "the block's top front edge, a value it fits",
            &block,
            &top_front,
            3.0,
        ),
        (
            "the block's top front edge, reduced",
            &block,
            &top_front,
            50.0,
        ),
        ("the L's inner corner, a value it fits", &ell, &inner, 3.0),
        ("the L's inner corner, reduced", &ell, &inner, 50.0),
        (
            "the block's top front edge, gone under a rabbet",
            &rabbeted,
            &top_front,
            1.0,
        ),
        ("a value that is no length", &block, &top_front, -1.0),
    ] {
        let list = [edge.clone()];

        assert_eq!(
            part.fillet_edges(&list, value),
            part.fillet(edge, value),
            "the fillet on {what}, asked {value}"
        );
        assert_eq!(
            part.chamfer_edges(&list, value),
            part.chamfer(edge, value),
            "the chamfer on {what}, asked {value}"
        );
    }
}

/// How deep the stair's tread is, from its nose to its foot, and how high
/// it stands.
const TREAD: f64 = 10.0;
const TREAD_HEIGHT: f64 = 10.0;

/// The numbers the stair's riser, its tread and the tread's nose answer to:
/// the riser is the taller block's front, two; the tread and the nose are the
/// lower block's top and front, seven and eight, past the taller block's six.
/// Their floors, their rights and their lefts merge; the lower block's back,
/// against the riser, is gone.
const RISER: usize = FRONT;
const TREAD_TOP: usize = TOP + 6;
const NOSE: usize = FRONT + 6;
const STAIR_RIGHT: [u32; 2] = [RIGHT as u32, RIGHT as u32 + 6];
const STAIR_LEFT: [u32; 2] = [LEFT as u32, LEFT as u32 + 6];

/// One step of a stair: a block [`LENGTH`] long from y = 10 to 20 and
/// [`HEIGHT`] high, and in front of it a block from y = 0 to 10, as long and
/// [`TREAD_HEIGHT`] high, joined to it. The lower block's top is the tread,
/// [`TREAD`] deep: its nose meets outwards, along the front, and its foot
/// meets inwards, against the riser. The two face each other across it.
/// The riser is as high as the tread is deep, and the taller block's top as
/// deep again: a flight of two treads, the second's nose where the riser
/// meets that top, facing the foot across the riser.
fn stair() -> Body {
    let taller = joined_to(
        &Body::default(),
        [DVec2::new(0.0, TREAD), DVec2::new(LENGTH, 2.0 * TREAD)],
        HEIGHT,
    );
    joined_to(
        &taller,
        [DVec2::ZERO, DVec2::new(LENGTH, TREAD)],
        TREAD_HEIGHT,
    )
}

/// The sides a face is expected to meet, given as the numbers each answers
/// to, in the order [`sides_met`] lists them.
fn sides_by(faces: &[Vec<u32>]) -> Vec<Vec<u32>> {
    let mut sorted = faces.to_vec();
    sorted.sort();
    sorted
}

#[test]
fn a_fillet_on_a_tread_s_nose_and_foot_asked_more_than_half_its_depth_takes_half_a_round_and_a_hollow_meeting_in_an_s()
 {
    let stair = stair();
    let names = [
        edge_between(&stair, TREAD_TOP, NOSE),
        edge_between(&stair, RISER, TREAD_TOP),
    ];
    let half = TREAD / 2.0;
    let laid = [stair.faces_end(), stair.faces_end() + 1];

    for (asked, taken) in [(4.9, 4.9), (MORE_THAN_HALF, half)] {
        let filleted = stair
            .fillet_edges(&names, asked)
            .unwrap_or_else(|declined| panic!("asked {asked}: declined, {declined:?}"));

        let what = format!("the fillet on the tread's nose and foot, asked {asked}");
        reduced(filleted.asked, filleted.taken, [asked, taken], &what);
        let body = &filleted.body;
        let round = LongEdge {
            faces: [TREAD_TOP, NOSE],
            start: DVec3::new(0.0, 0.0, TREAD_HEIGHT),
            facing: [DVec3::Z, -DVec3::Y],
        }
        .round(taken);
        let hollow = Hollow {
            start: DVec3::new(0.0, TREAD - taken, TREAD_HEIGHT + taken),
            along: DVec3::X,
            length: LENGTH,
            radius: taken,
            facing: [-DVec3::Y, DVec3::Z],
        };
        is_the_round(
            body,
            laid[0],
            &round,
            &format!("{what}: face {}, the nose rounded off at {taken}", laid[0]),
        );
        is_the_hollow(
            body,
            laid[1],
            &hollow,
            &format!("{what}: face {}, the foot filled in at {taken}", laid[1]),
        );
        let ends = [STAIR_RIGHT.to_vec(), STAIR_LEFT.to_vec()];
        if taken < half {
            corners_are(
                body,
                TREAD_TOP,
                &strip(
                    DVec3::new(0.0, taken, TREAD_HEIGHT),
                    DVec3::new(0.0, TREAD - taken, TREAD_HEIGHT),
                ),
                &format!("{what}: what is left of the tread, a strip between the two"),
            );
            assert!(
                body.edge_names()
                    .iter()
                    .all(|name| !lies_between(name, laid[0], laid[1])),
                "{what}: the round and the hollow do not meet, the tread between them"
            );
        } else {
            assert!(!body.has_face(TREAD_TOP), "{what}: the tread is gone");
            edge_between(body, laid[0], laid[1]);
            assert_eq!(
                sides_met(body, laid[0]),
                sides_by(&[&ends[..], &[vec![NOSE as u32], vec![laid[1] as u32]]].concat()),
                "{what}: the round meets the nose's front, the hollow along one edge where the \
                 two are tangent, and the stair's two ends, by one edge each"
            );
            assert_eq!(
                sides_met(body, laid[1]),
                sides_by(&[&ends[..], &[vec![RISER as u32], vec![laid[0] as u32]]].concat()),
                "{what}: the hollow meets the riser, the round along one edge where the two are \
                 tangent, and the stair's two ends, by one edge each"
            );
        }
        corners_are(
            body,
            NOSE,
            &strip(DVec3::ZERO, DVec3::new(0.0, 0.0, TREAD_HEIGHT - taken)),
            &format!("{what}: what is left of the nose's front, under the round"),
        );
        corners_are(
            body,
            RISER,
            &strip(
                DVec3::new(0.0, TREAD, TREAD_HEIGHT + taken),
                DVec3::new(0.0, TREAD, HEIGHT),
            ),
            &format!("{what}: what is left of the riser, above the hollow"),
        );
        sound_drawn_within(
            body,
            LENGTH * TREAD * (HEIGHT + TREAD_HEIGHT),
            DRAWN * reach_of(&stair) * (round.area() + hollow.area()),
            &format!("{what}: as much matter as the nose gives up, the foot takes in"),
        );
    }
}

/// Holds that the numbers `laid` answer to one and the same face, which is
/// flat: on one plane with the same corners, and drawn with the same
/// triangles, whichever is asked.
fn one_flat_face_answers_to(body: &Body, laid: &[usize], what: &str) {
    let plane = body.plane_of(laid[0]);
    let drawn: Vec<[DVec3; 3]> = body.triangles_of(laid[0]).collect();
    assert!(
        plane.is_some() && !drawn.is_empty(),
        "{what}: face {} is flat, and drawn",
        laid[0]
    );
    for &number in laid {
        assert!(
            body.is_flat(number),
            "{what}: face {number} lies on one plane and faces one way"
        );
        assert_eq!(
            body.plane_of(number),
            plane,
            "{what}: faces {laid:?} are one face, on one plane with the same corners"
        );
        assert_eq!(
            body.triangles_of(number).collect::<Vec<_>>(),
            drawn,
            "{what}: faces {laid:?} are one face, drawn with the same triangles"
        );
    }
}

/// The corners of the stair's wall at `x` once its nose is cut off and its
/// foot filled in, each set back by `distance`: its floor, in two edges
/// that part where the two blocks' floors met, its back, the top of the
/// taller block, what is left of the riser, the foot's flat, the tread left
/// between the two flats, the nose's flat, and what is left of the nose's
/// front. Where the two flats meet, at half the tread's depth, the tread
/// left is a single point, and the two flats one straight side.
fn stair_wall_at(x: f64, distance: f64) -> Vec<DVec3> {
    let mut corners = vec![
        DVec3::new(x, 0.0, 0.0),
        DVec3::new(x, TREAD, 0.0),
        DVec3::new(x, 2.0 * TREAD, 0.0),
        DVec3::new(x, 2.0 * TREAD, HEIGHT),
        DVec3::new(x, TREAD, HEIGHT),
        DVec3::new(x, TREAD, TREAD_HEIGHT + distance),
    ];
    if distance < TREAD / 2.0 {
        corners.extend([
            DVec3::new(x, TREAD - distance, TREAD_HEIGHT),
            DVec3::new(x, distance, TREAD_HEIGHT),
        ]);
    }
    corners.push(DVec3::new(x, 0.0, TREAD_HEIGHT - distance));
    corners
}

/// Holds that what is left of the nose's front and of the riser, and the
/// stair's two walls, are what a chamfer of `distance` on the tread's nose
/// and foot leaves of them.
fn stair_walls_are(body: &Body, distance: f64, what: &str) {
    corners_are(
        body,
        NOSE,
        &strip(DVec3::ZERO, DVec3::new(0.0, 0.0, TREAD_HEIGHT - distance)),
        &format!("{what}: what is left of the nose's front, under its flat"),
    );
    corners_are(
        body,
        RISER,
        &strip(
            DVec3::new(0.0, TREAD, TREAD_HEIGHT + distance),
            DVec3::new(0.0, TREAD, HEIGHT),
        ),
        &format!("{what}: what is left of the riser, above the foot's flat"),
    );
    for (wall, x) in [(LEFT, 0.0), (RIGHT, LENGTH)] {
        corners_are(
            body,
            wall,
            &stair_wall_at(x, distance),
            &format!("{what}: the stair's wall at x = {x}"),
        );
    }
}

/// The stair's nose and foot, each list in the two orders a step may name
/// them, with the order said.
fn nose_and_foot_either_way(stair: &Body) -> [(&'static str, [EdgeName; 2]); 2] {
    let [nose, foot] = [
        edge_between(stair, TREAD_TOP, NOSE),
        edge_between(stair, RISER, TREAD_TOP),
    ];
    [
        ("the nose named first", [nose.clone(), foot.clone()]),
        ("the foot named first", [foot, nose]),
    ]
}

/// Holds that a chamfer on the stair's nose and foot laid one slope where
/// the two flats meet, at half the tread's depth: the tread gone, one flat
/// face answering to both edges' numbers, `laid`, from half down the nose's
/// front to half up the riser, meeting those two and the stair's two ends by
/// one edge each, and the matter the nose gives up what the foot takes in.
fn is_one_slope(stair: &Body, body: &Body, laid: [usize; 2], what: &str) {
    let half = TREAD / 2.0;
    assert!(!body.has_face(TREAD_TOP), "{what}: the tread is gone");
    let mut kept_and_laid = numbers(stair);
    kept_and_laid.remove(&TREAD_TOP);
    kept_and_laid.extend(laid);
    assert_eq!(
        numbers(body),
        kept_and_laid,
        "{what}: every face of the stair but its tread, and the slope answering to {laid:?}"
    );
    assert_eq!(
        body.faces_end(),
        laid[1] + 1,
        "{what}: the next step numbers its faces past both the slope answers to"
    );
    one_flat_face_answers_to(body, &laid, what);
    let which =
        format!("{what}: the slope, from {half} down the nose's front to {half} up the riser");
    face_facing(
        body,
        &[laid[0]],
        DVec3::new(0.0, -1.0, 1.0).normalize(),
        &which,
    );
    corners_are(
        body,
        laid[0],
        &strip(
            DVec3::new(0.0, 0.0, TREAD_HEIGHT - half),
            DVec3::new(0.0, TREAD, TREAD_HEIGHT + half),
        ),
        &which,
    );
    assert!(
        body.edge_names()
            .iter()
            .all(|name| !lies_between(name, laid[0], laid[1])),
        "{what}: no edge stands between {} and {}: they are one face",
        laid[0],
        laid[1]
    );
    let ends = [STAIR_RIGHT.to_vec(), STAIR_LEFT.to_vec()];
    assert_eq!(
        sides_met_by(body, &laid),
        sides_by(&[&ends[..], &[vec![NOSE as u32], vec![RISER as u32]]].concat()),
        "{what}: the slope meets the nose's front, the riser, and the stair's two ends, by one \
         edge each"
    );
    stair_walls_are(body, half, what);
    let promised = LENGTH * TREAD * (HEIGHT + TREAD_HEIGHT);
    sound_drawn_within(
        body,
        promised,
        EXACT * promised,
        &format!("{what}: as much matter as the nose gives up, the foot takes in"),
    );
}

#[test]
fn a_chamfer_on_a_tread_s_nose_and_foot_asked_half_its_depth_or_more_takes_half_in_one_slope_answering_to_both_their_numbers()
 {
    let stair = stair();
    let half = TREAD / 2.0;
    let [(_, names), _] = nose_and_foot_either_way(&stair);
    alone_each_takes_more(
        |name| {
            stair
                .chamfer(name, MORE_THAN_HALF)
                .map(|single| single.taken)
        },
        &names,
        half,
        "the chamfer on the tread's nose or its foot",
    );
    let laid = [stair.faces_end(), stair.faces_end() + 1];

    for (order, names) in nose_and_foot_either_way(&stair) {
        for asked in [half, MORE_THAN_HALF] {
            let chamfered = stair
                .chamfer_edges(&names, asked)
                .unwrap_or_else(|declined| {
                    panic!("{order}, asked {asked}: declined, {declined:?}")
                });

            let what = format!("the chamfer on the tread's nose and foot, {order}, asked {asked}");
            reduced(chamfered.asked, chamfered.taken, [asked, half], &what);
            is_one_slope(&stair, &chamfered.body, laid, &what);
        }
    }
}

#[test]
fn a_chamfer_on_a_tread_s_nose_and_foot_asked_a_hair_under_half_its_depth_lays_one_slope() {
    let stair = stair();
    let half = TREAD / 2.0;
    let laid = [stair.faces_end(), stair.faces_end() + 1];

    for (order, names) in nose_and_foot_either_way(&stair) {
        for (under, past_one_hair) in [(1e-6, true), (1e-7, false)] {
            let asked = half - under;
            let strip = TREAD - 2.0 * asked;
            assert!(
                strip < MEETING_HAIR && (strip > ONE_HAIR) == past_one_hair,
                "asked {asked}: the strip of tread the two flats leave, {strip}, is shorter than \
                 the kernel keeps, {MEETING_HAIR}, and longer than one of its hairs, \
                 {ONE_HAIR}, or not, as {past_one_hair} says"
            );

            let chamfered = stair
                .chamfer_edges(&names, asked)
                .unwrap_or_else(|declined| {
                    panic!("{order}, asked {asked}: declined, {declined:?}")
                });

            let what = format!("the chamfer on the tread's nose and foot, {order}, asked {asked}");
            assert!(
                (chamfered.asked - asked).abs() < PLACE
                    && (chamfered.taken - asked).abs() <= MEETING_HAIR,
                "{what}: asked {} and took {}, where {asked} is a hair from what both give",
                chamfered.asked,
                chamfered.taken
            );
            is_one_slope(&stair, &chamfered.body, laid, &what);
        }
    }
}

#[test]
fn a_chamfer_on_a_tread_s_nose_and_foot_asked_less_than_half_its_depth_lays_two_flats_apart_a_strip_of_tread_between()
 {
    let stair = stair();
    let names = [
        edge_between(&stair, TREAD_TOP, NOSE),
        edge_between(&stair, RISER, TREAD_TOP),
    ];
    let asked = 4.9;
    let laid = [stair.faces_end(), stair.faces_end() + 1];

    let chamfered = stair
        .chamfer_edges(&names, asked)
        .unwrap_or_else(|declined| panic!("declined, {declined:?}"));

    let what = format!("the chamfer on the tread's nose and foot, asked {asked}");
    taken_as_asked(chamfered.asked, chamfered.taken, asked, &what);
    let body = &chamfered.body;
    let mut kept_and_laid = numbers(&stair);
    kept_and_laid.extend(laid);
    assert_eq!(
        numbers(body),
        kept_and_laid,
        "{what}: every face of the stair, its tread among them, and a flat for each edge"
    );
    let ends = [STAIR_RIGHT.to_vec(), STAIR_LEFT.to_vec()];
    for (number, corners, met, which) in [
        (
            laid[0],
            strip(
                DVec3::new(0.0, 0.0, TREAD_HEIGHT - asked),
                DVec3::new(0.0, asked, TREAD_HEIGHT),
            ),
            NOSE,
            "the nose cut off",
        ),
        (
            laid[1],
            strip(
                DVec3::new(0.0, TREAD - asked, TREAD_HEIGHT),
                DVec3::new(0.0, TREAD, TREAD_HEIGHT + asked),
            ),
            RISER,
            "the foot filled in",
        ),
    ] {
        let which = format!("{what}: face {number}, {which} at {asked}");
        face_facing(
            body,
            &[number],
            DVec3::new(0.0, -1.0, 1.0).normalize(),
            &which,
        );
        corners_are(body, number, &corners, &which);
        assert_eq!(
            sides_met(body, number),
            sides_by(&[&ends[..], &[vec![met as u32], vec![TREAD_TOP as u32]]].concat()),
            "{which}: a face answering to its own number alone, meeting face {met}, the strip \
             of tread, and the stair's two ends, by one edge each"
        );
    }
    assert!(
        body.edge_names()
            .iter()
            .all(|name| !lies_between(name, laid[0], laid[1])),
        "{what}: the two flats do not meet, the tread between them"
    );
    corners_are(
        body,
        TREAD_TOP,
        &strip(
            DVec3::new(0.0, asked, TREAD_HEIGHT),
            DVec3::new(0.0, TREAD - asked, TREAD_HEIGHT),
        ),
        &format!("{what}: what is left of the tread, a strip between the two flats"),
    );
    stair_walls_are(body, asked, &what);
    let promised = LENGTH * TREAD * (HEIGHT + TREAD_HEIGHT);
    sound_drawn_within(
        body,
        promised,
        EXACT * promised,
        &format!("{what}: as much matter as the nose gives up, the foot takes in"),
    );
}

#[test]
fn a_chamfer_on_a_tread_s_nose_and_foot_and_an_edge_far_from_both_numbers_the_slope_by_their_ranks_and_the_far_flat_by_its_own_wherever_the_far_edge_is_named()
 {
    let stair = stair();
    let nose = edge_between(&stair, TREAD_TOP, NOSE);
    let foot = edge_between(&stair, RISER, TREAD_TOP);
    let far = edge_between(&stair, TOP, BACK);
    let half = TREAD / 2.0;
    let start = stair.faces_end();
    let ends = [STAIR_RIGHT.to_vec(), STAIR_LEFT.to_vec()];

    for (order, names) in [
        (
            "the far edge named first",
            [far.clone(), nose.clone(), foot.clone()],
        ),
        (
            "the far edge named between",
            [nose.clone(), far.clone(), foot.clone()],
        ),
        (
            "the far edge named last",
            [nose.clone(), foot.clone(), far.clone()],
        ),
    ] {
        let number_of = |edge: &EdgeName| {
            start
                + names
                    .iter()
                    .position(|name| name == edge)
                    .expect("each edge is named")
        };
        let slope = [number_of(&nose), number_of(&foot)];
        let flat = number_of(&far);

        let chamfered = stair
            .chamfer_edges(&names, half)
            .unwrap_or_else(|declined| panic!("{order}, asked {half}: declined, {declined:?}"));

        let what = format!(
            "the chamfer on the tread's nose and foot and the top's back edge, {order}, asked \
             {half}"
        );
        taken_as_asked(chamfered.asked, chamfered.taken, half, &what);
        let body = &chamfered.body;
        assert!(!body.has_face(TREAD_TOP), "{what}: the tread is gone");
        let mut kept_and_laid = numbers(&stair);
        kept_and_laid.remove(&TREAD_TOP);
        kept_and_laid.extend(start..start + 3);
        assert_eq!(
            numbers(body),
            kept_and_laid,
            "{what}: every face of the stair but its tread, the slope answering to {slope:?} and \
             the far flat to {flat}"
        );
        assert_eq!(
            body.faces_end(),
            start + 3,
            "{what}: the next step numbers its faces past the three the step laid"
        );
        one_flat_face_answers_to(body, &slope, &what);
        corners_are(
            body,
            slope[0],
            &strip(
                DVec3::new(0.0, 0.0, TREAD_HEIGHT - half),
                DVec3::new(0.0, TREAD, TREAD_HEIGHT + half),
            ),
            &format!(
                "{what}: face {}, the slope from the nose's front to the riser",
                slope[0]
            ),
        );
        assert_eq!(
            sides_met_by(body, &slope),
            sides_by(&[&ends[..], &[vec![NOSE as u32], vec![RISER as u32]]].concat()),
            "{what}: the slope answers to {slope:?} and to no other number, meeting the nose's \
             front, the riser and the stair's two ends by one edge each"
        );
        let which = format!("{what}: face {flat}, the top's back edge cut off at {half}");
        face_facing(body, &[flat], DVec3::new(0.0, 1.0, 1.0).normalize(), &which);
        corners_are(
            body,
            flat,
            &strip(
                DVec3::new(0.0, 2.0 * TREAD, HEIGHT - half),
                DVec3::new(0.0, 2.0 * TREAD - half, HEIGHT),
            ),
            &which,
        );
        assert_eq!(
            sides_met(body, flat),
            sides_by(&[&ends[..], &[vec![TOP as u32], vec![BACK as u32]]].concat()),
            "{which}: a face answering to its own number alone, meeting the top, the back and \
             the stair's two ends by one edge each"
        );
        let promised = LENGTH * (TREAD * (HEIGHT + TREAD_HEIGHT) - half * half / 2.0);
        sound_drawn_within(
            body,
            promised,
            EXACT * promised,
            &format!(
                "{what}: as much matter as the nose gives up, the foot takes in, and what the \
                 far edge gives up taken off"
            ),
        );
    }
}

/// How far the turned stair's riser is turned about the upright, in
/// radians: within what the kernel reads as parallel to the nose, a
/// millionth, yet parting the nose's flat and the foot's by some hundred of
/// its tolerances from one end of the stair to the other.
const RISER_TURN: f64 = 1e-7;

/// The [`stair`] with its riser turned by `turn` about the upright through
/// its middle, so that the foot is parallel to the nose only within the
/// kernel's reading: the taller block's front runs from a hair in front of
/// y = 10 at the left end to a hair behind it at the right, and the lower
/// block reaches in under the taller one, so that no face of either lies a
/// hair from another's. Its faces answer to the numbers [`stair`]'s do.
fn stair_with_its_riser_turned_by(turn: f64) -> Body {
    stair_with_its_riser_turned_by_up_to(turn, HEIGHT)
}

/// The [`stair_with_its_riser_turned_by`] `turn`, its taller block `height`
/// high rather than [`HEIGHT`].
fn stair_with_its_riser_turned_by_up_to(turn: f64, height: f64) -> Body {
    let shift = LENGTH / 2.0 * turn;
    let taller = raised_with(
        &Body::default(),
        Contour::straight(vec![
            DVec2::new(0.0, TREAD - shift),
            DVec2::new(LENGTH, TREAD + shift),
            DVec2::new(LENGTH, 2.0 * TREAD),
            DVec2::new(0.0, 2.0 * TREAD),
        ]),
        height,
    );
    joined_to(
        &taller,
        [DVec2::ZERO, DVec2::new(LENGTH, 1.5 * TREAD)],
        TREAD_HEIGHT,
    )
}

/// Where the turned stair's riser stands across it at `x` along it: a hair
/// in front of [`TREAD`] at its left end, a hair behind it at its right.
fn turned_riser_at(x: f64) -> f64 {
    riser_turned_by_at(RISER_TURN, x)
}

/// Where the riser of the [`stair_with_its_riser_turned_by`] `turn` stands
/// across it at `x` along it.
fn riser_turned_by_at(turn: f64, x: f64) -> f64 {
    let shift = LENGTH / 2.0 * turn;
    TREAD - shift + 2.0 * shift * x / LENGTH
}

/// How far the slope a chamfer lays on the nose and foot of the stair whose
/// riser is turned by `turn` may stand from where the straight stair's
/// stands: a hair, the most a set-back may be off the value the step takes,
/// and how far the nose's flat and the foot's part from one end of the stair
/// to the other.
fn turned_stray(turn: f64) -> f64 {
    MEETING_HAIR + LENGTH * turn
}

/// Holds that a chamfer on a stair's nose and foot laid one slope where the
/// faces numbered `gone` were: every other face of the stair kept, one flat
/// face answering to both edges' numbers, `laid`, the next step numbering
/// past both, and the slope meeting each face whose numbers are among `met`
/// by one edge.
fn is_one_slope_meeting(
    stair: &Body,
    body: &Body,
    laid: [usize; 2],
    gone: &[usize],
    met: &[Vec<u32>],
    what: &str,
) {
    let mut kept_and_laid = numbers(stair);
    for face in gone {
        assert!(!body.has_face(*face), "{what}: face {face} is gone");
        kept_and_laid.remove(face);
    }
    kept_and_laid.extend(laid);
    assert_eq!(
        numbers(body),
        kept_and_laid,
        "{what}: every face of the stair but {gone:?}, and the slope answering to {laid:?}"
    );
    assert_eq!(
        body.faces_end(),
        laid[1] + 1,
        "{what}: the next step numbers its faces past both the slope answers to"
    );
    one_flat_face_answers_to(body, &laid, what);
    assert!(
        body.edge_names()
            .iter()
            .all(|name| !lies_between(name, laid[0], laid[1])),
        "{what}: no edge stands between {} and {}: they are one face",
        laid[0],
        laid[1]
    );
    assert_eq!(
        sides_met_by(body, &laid),
        sides_by(met),
        "{what}: the slope meets the faces {met:?}, by one edge each"
    );
}

/// The corners of the flat face numbered `face` on a stair [`LENGTH`] long,
/// each on the plane of one of the stair's two ends and on the plane of the
/// nose's front, y = 0, or of the riser, which stands across the stair at
/// `riser(x, z)`, to the place's tolerance, one at each of the four places
/// those planes meet and none anywhere else: the front's at x = 0 and at
/// [`LENGTH`], then the riser's.
fn slope_corners(
    body: &Body,
    face: usize,
    riser: impl Fn(f64, f64) -> f64,
    what: &str,
) -> [[DVec3; 2]; 2] {
    let plane = body
        .plane_of(face)
        .unwrap_or_else(|| panic!("{what}: the body offers no plane on face {face}"));
    let mut found = [[None::<DVec3>; 2]; 2];
    for corner in &plane.corners {
        let end = [0.0, LENGTH]
            .iter()
            .position(|x| (corner.x - x).abs() < PLACE);
        let side = [0.0, riser(corner.x, corner.z)]
            .iter()
            .position(|y| (corner.y - y).abs() < PLACE);
        let (Some(end), Some(side)) = (end, side) else {
            panic!(
                "{what}: a corner of the slope at {corner}, off the stair's two ends or off both \
                 the nose's front and the riser"
            );
        };
        let seen = *found[side][end].get_or_insert(*corner);
        assert!(
            seen.distance(*corner) < PLACE,
            "{what}: two corners of the slope where one end meets one face, {seen} and {corner}"
        );
    }
    found.map(|side| {
        side.map(|corner| {
            corner.unwrap_or_else(|| {
                panic!(
                    "{what}: no corner of the slope at one of the four places the stair's ends \
                     meet the nose's front and the riser: {:?}",
                    plane.corners
                )
            })
        })
    })
}

/// Holds that each of the slope's `corners`, as [`slope_corners`] gives
/// them, stands `distance` from the edge it was laid on, to `within`: down
/// the nose's front from the tread, up the riser from it.
fn set_back_by(corners: [[DVec3; 2]; 2], distance: f64, within: f64, what: &str) {
    let [front, riser] = corners;
    let set_backs = (front.iter().map(|corner| (corner, TREAD_HEIGHT - corner.z)))
        .chain(riser.iter().map(|corner| (corner, corner.z - TREAD_HEIGHT)));
    for (corner, set_back) in set_backs {
        assert!(
            (set_back - distance).abs() <= within,
            "{what}: the slope's corner at {corner} stands {set_back} from its edge, where \
             {distance} is set back, to {within}"
        );
    }
}

/// Holds that two lists of a slope's corners, as [`slope_corners`] gives
/// them, are the same corners, to the place's tolerance.
fn same_corners(one: [[DVec3; 2]; 2], other: [[DVec3; 2]; 2], what: &str) {
    for (corner, again) in one.iter().flatten().zip(other.iter().flatten()) {
        assert!(
            corner.distance(*again) < PLACE,
            "{what}: the slope's corner at {corner} in one order is at {again} in the other"
        );
    }
}

/// The matter of the turned stair once one slope through `corners`, as
/// [`slope_corners`] gives them, stands in place of its tread: over the
/// ground in front of the riser, the slope's height, and behind it the
/// taller block's, summed along the stair by Simpson's rule, exact for what
/// is a square in x.
fn turned_matter_under(corners: [[DVec3; 2]; 2], turn: f64) -> f64 {
    let [[front, far_front], [riser, _]] = corners;
    let along = (far_front.z - front.z) / LENGTH;
    let across = (riser.z - front.z) / riser.y;
    let at = |x: f64| {
        let reach = riser_turned_by_at(turn, x);
        (front.z + along * x) * reach
            + across * reach * reach / 2.0
            + (2.0 * TREAD - reach) * HEIGHT
    };
    LENGTH / 6.0 * (at(0.0) + 4.0 * at(LENGTH / 2.0) + at(LENGTH))
}

/// Holds that a chamfer on the nose and foot of the stair whose riser is
/// turned by `turn` laid one slope where the two flats meet, at half the
/// tread's depth, as on the straight stair: the tread gone, one flat face
/// answering to both edges' numbers, `laid`, meeting the nose's front, the
/// riser and the stair's two ends by one edge each; its four corners on
/// those faces' planes, set back half the tread's depth within a hair and
/// how far the two flats part along the stair, [`turned_stray`], on one
/// plane; and the matter the arithmetic gives under them, which is the
/// straight stair's slope's within what a slope that far off can change
/// over the tread. Gives back the corners, as [`slope_corners`] does.
fn is_one_turned_slope(
    turn: f64,
    stair: &Body,
    body: &Body,
    laid: [usize; 2],
    what: &str,
) -> [[DVec3; 2]; 2] {
    let half = TREAD / 2.0;
    let stray = turned_stray(turn);
    is_one_slope_meeting(
        stair,
        body,
        laid,
        &[TREAD_TOP],
        &[
            STAIR_RIGHT.to_vec(),
            STAIR_LEFT.to_vec(),
            vec![NOSE as u32],
            vec![RISER as u32],
        ],
        what,
    );
    let corners = slope_corners(body, laid[0], |x, _| riser_turned_by_at(turn, x), what);
    set_back_by(corners, half, stray, what);
    let [[front, far_front], [riser, far_riser]] = corners;
    let on_the_slope = front.z
        + (far_front.z - front.z) / LENGTH * far_riser.x
        + (riser.z - front.z) / riser.y * far_riser.y;
    assert!(
        (on_the_slope - far_riser.z).abs() < PLACE,
        "{what}: the slope's corner at {far_riser} lies on the plane of its other three, which \
         passes {on_the_slope} there"
    );
    let promised = turned_matter_under(corners, turn);
    let straight = LENGTH * TREAD * (HEIGHT + TREAD_HEIGHT);
    assert!(
        (promised - straight).abs() <= 2.0 * LENGTH * TREAD * stray,
        "{what}: under the slope the turned stair holds {promised}, where the straight stair's \
         slope holds {straight}"
    );
    sound_drawn_within(
        body,
        promised,
        EXACT * promised,
        &format!("{what}: the matter under the slope's own corners"),
    );
    corners
}

#[test]
fn a_chamfer_on_a_tread_s_nose_and_foot_whose_riser_is_turned_by_a_hair_asked_half_its_depth_or_a_hair_under_lays_one_slope_whichever_is_named_first()
 {
    let stair = stair_with_its_riser_turned_by(RISER_TURN);
    let half = TREAD / 2.0;
    let laid = [stair.faces_end(), stair.faces_end() + 1];

    for asked in [half, MORE_THAN_HALF, half - 1e-6] {
        let mut corners_by_order = Vec::new();
        for (order, names) in nose_and_foot_either_way(&stair) {
            let chamfered = stair
                .chamfer_edges(&names, asked)
                .unwrap_or_else(|declined| {
                    panic!("{order}, asked {asked}: declined, {declined:?}")
                });

            let what = format!(
                "the chamfer on the nose and foot of the tread whose riser is turned by \
                 {RISER_TURN}, {order}, asked {asked}"
            );
            assert!(
                (chamfered.asked - asked).abs() < PLACE
                    && (chamfered.taken - half).abs() <= MEETING_HAIR,
                "{what}: asked {} and took {}, where the two flats meet a hair from {half}",
                chamfered.asked,
                chamfered.taken
            );
            corners_by_order.push(is_one_turned_slope(
                RISER_TURN,
                &stair,
                &chamfered.body,
                laid,
                &what,
            ));
        }
        same_corners(
            corners_by_order[0],
            corners_by_order[1],
            &format!("the turned stair, asked {asked}, the nose named first and the foot"),
        );
    }
}

/// A turn of the riser a hundredth of [`RISER_TURN`]: the nose's flat and
/// the foot's part by about the kernel's own tolerance from one end of the
/// stair to the other, nought as a chamfer reads it.
const HAIR_TURN: f64 = 1e-9;

#[test]
fn a_chamfer_on_a_tread_s_nose_and_foot_whose_riser_is_turned_by_a_billionth_lays_one_slope_whichever_is_named_first_the_riser_gone_where_it_is_as_high_as_half_its_depth()
 {
    let half = TREAD / 2.0;
    let ends = [STAIR_RIGHT.to_vec(), STAIR_LEFT.to_vec()];

    for height in [HEIGHT, TREAD_HEIGHT + half] {
        let stair = stair_with_its_riser_turned_by_up_to(HAIR_TURN, height);
        let laid = [stair.faces_end(), stair.faces_end() + 1];
        let riser_gone = height - TREAD_HEIGHT <= half + PLACE;
        let (gone, above) = if riser_gone {
            (vec![TREAD_TOP, RISER], TOP)
        } else {
            (vec![TREAD_TOP], RISER)
        };
        let promised = LENGTH * TREAD * (TREAD_HEIGHT + height);
        let within = 2.0 * LENGTH * TREAD * MEETING_HAIR;

        for (order, names) in nose_and_foot_either_way(&stair) {
            for asked in [half, MORE_THAN_HALF] {
                let what = format!(
                    "the chamfer on the nose and foot of the tread whose riser, {} high, is \
                     turned by {HAIR_TURN}, {order}, asked {asked}",
                    height - TREAD_HEIGHT
                );
                let chamfered = stair
                    .chamfer_edges(&names, asked)
                    .unwrap_or_else(|declined| panic!("{what}: declined, {declined:?}"));

                assert!(
                    (chamfered.asked - asked).abs() < PLACE
                        && (chamfered.taken - half).abs() <= MEETING_HAIR,
                    "{what}: asked {} and took {}, where the two flats meet a hair from {half}",
                    chamfered.asked,
                    chamfered.taken
                );
                let body = &chamfered.body;
                is_one_slope_meeting(
                    &stair,
                    body,
                    laid,
                    &gone,
                    &[&ends[..], &[vec![NOSE as u32], vec![above as u32]]].concat(),
                    &what,
                );
                let plane = body
                    .plane_of(laid[0])
                    .unwrap_or_else(|| panic!("{what}: the body offers no plane on the slope"));
                let expected: Vec<DVec3> = [0.0, LENGTH]
                    .into_iter()
                    .flat_map(|x| {
                        [
                            DVec3::new(x, 0.0, TREAD_HEIGHT - half),
                            DVec3::new(x, riser_turned_by_at(HAIR_TURN, x), TREAD_HEIGHT + half),
                        ]
                    })
                    .collect();
                for corner in &plane.corners {
                    assert!(
                        expected
                            .iter()
                            .any(|point| point.distance(*corner) <= MEETING_HAIR),
                        "{what}: the slope's corner at {corner}, where its corners are \
                         {expected:?}, to a hair"
                    );
                }
                for point in &expected {
                    assert!(
                        plane
                            .corners
                            .iter()
                            .any(|corner| corner.distance(*point) <= MEETING_HAIR),
                        "{what}: no corner of the slope within a hair of {point}, among {:?}",
                        plane.corners
                    );
                }
                assert!(
                    body.is_exact(),
                    "{what}: the exact kernel computed the body"
                );
                let triangles = body.triangles();
                assert_eq!(closed(&triangles), Ok(()), "{what}");
                assert_eq!(uncrossed(&triangles), Ok(()), "{what}");
                let enclosed = enclosed(&triangles);
                assert!(
                    (body.volume() - promised).abs() <= within
                        && (enclosed - promised).abs() <= within,
                    "{what}: the body holds {} and its triangles enclose {enclosed}, where the \
                     stair, as much matter given up at the nose as taken in at the foot, holds \
                     {promised}, to {within}",
                    body.volume()
                );
            }
        }
    }
}

#[test]
fn a_chamfer_on_a_tread_s_nose_and_foot_whose_riser_is_turned_by_a_ten_millionth_and_as_high_as_half_its_depth_is_declined_as_unsupported_whichever_is_named_first()
 {
    let half = TREAD / 2.0;
    let tall = stair_with_its_riser_turned_by(RISER_TURN);
    for (order, names) in nose_and_foot_either_way(&tall) {
        assert!(
            tall.chamfer_edges(&names, half).is_ok(),
            "the riser turned by {RISER_TURN}, {} high, {order}, asked {half}: one slope, so \
             that what declines the shorter riser is the slope taking it whole",
            HEIGHT - TREAD_HEIGHT
        );
    }
    let stair = stair_with_its_riser_turned_by_up_to(RISER_TURN, TREAD_HEIGHT + half);

    for (order, names) in nose_and_foot_either_way(&stair) {
        for asked in [half, MORE_THAN_HALF] {
            let declined = stair.chamfer_edges(&names, asked).err();

            assert_eq!(
                declined,
                Some(Declined::Unsupported),
                "the riser turned by {RISER_TURN}, {half} high, {order}, asked {asked}: the \
                 slope would take the riser whole, its top line parting from the riser's top \
                 edge by up to {} along the stair, declined until #928",
                LENGTH * RISER_TURN
            );
        }
    }
}

/// The [`stair`] with its taller block `height` high rather than
/// [`HEIGHT`], its riser `height` less [`TREAD_HEIGHT`] high, its faces
/// answering to the numbers [`stair`]'s do.
fn stair_up_to(height: f64) -> Body {
    let taller = joined_to(
        &Body::default(),
        [DVec2::new(0.0, TREAD), DVec2::new(LENGTH, 2.0 * TREAD)],
        height,
    );
    joined_to(
        &taller,
        [DVec2::ZERO, DVec2::new(LENGTH, TREAD)],
        TREAD_HEIGHT,
    )
}

/// The corners of the wall at `x` of the stair [`stair_up_to`] `height`
/// once one slope set back half the tread's depth stands in place of its
/// tread: as [`stair_wall_at`] gives them, the taller block's top at
/// `height`, and the riser left between the slope and that top only where
/// the riser stands higher than half the tread's depth. Where the two
/// blocks' floors met, the floor's edge may part or not: that corner is
/// [`on_the_floor`].
fn short_stair_wall_at(x: f64, height: f64) -> Vec<DVec3> {
    let half = TREAD / 2.0;
    let mut corners = vec![
        DVec3::new(x, 0.0, 0.0),
        DVec3::new(x, 2.0 * TREAD, 0.0),
        DVec3::new(x, 2.0 * TREAD, height),
        DVec3::new(x, TREAD, height),
    ];
    if height > TREAD_HEIGHT + half + PLACE {
        corners.push(DVec3::new(x, TREAD, TREAD_HEIGHT + half));
    }
    corners.push(DVec3::new(x, 0.0, TREAD_HEIGHT - half));
    corners
}

/// Whether a corner of a stair's wall lies on its floor's edge, between its
/// front and its back.
fn on_the_floor(corner: DVec3) -> bool {
    corner.z.abs() < PLACE && (-PLACE..=2.0 * TREAD + PLACE).contains(&corner.y)
}

#[test]
fn a_chamfer_on_a_tread_s_nose_and_foot_under_a_riser_as_high_as_half_its_depth_or_a_little_higher_takes_half_in_one_slope_the_riser_gone_or_a_strip_of_it_kept_whichever_is_named_first()
 {
    let half = TREAD / 2.0;
    let ends = [STAIR_RIGHT.to_vec(), STAIR_LEFT.to_vec()];

    for riser in [half, half + 1e-3, half + 1e-2] {
        let height = TREAD_HEIGHT + riser;
        let stair = stair_up_to(height);
        let laid = [stair.faces_end(), stair.faces_end() + 1];
        let riser_gone = riser <= half + PLACE;
        let (gone, above) = if riser_gone {
            (vec![TREAD_TOP, RISER], TOP)
        } else {
            (vec![TREAD_TOP], RISER)
        };

        for (order, names) in nose_and_foot_either_way(&stair) {
            for asked in [half, MORE_THAN_HALF] {
                let chamfered = stair
                    .chamfer_edges(&names, asked)
                    .unwrap_or_else(|declined| {
                        panic!(
                            "a riser {riser} high, {order}, asked {asked}: declined, {declined:?}"
                        )
                    });

                let what = format!(
                    "the chamfer on the nose and foot of a tread under a riser {riser} high, \
                     {order}, asked {asked}"
                );
                reduced(chamfered.asked, chamfered.taken, [asked, half], &what);
                let body = &chamfered.body;
                is_one_slope_meeting(
                    &stair,
                    body,
                    laid,
                    &gone,
                    &[&ends[..], &[vec![NOSE as u32], vec![above as u32]]].concat(),
                    &what,
                );
                let which = format!(
                    "{what}: the slope, from {half} down the nose's front to {half} up the riser"
                );
                face_facing(
                    body,
                    &[laid[0]],
                    DVec3::new(0.0, -1.0, 1.0).normalize(),
                    &which,
                );
                corners_are(
                    body,
                    laid[0],
                    &strip(
                        DVec3::new(0.0, 0.0, TREAD_HEIGHT - half),
                        DVec3::new(0.0, TREAD, TREAD_HEIGHT + half),
                    ),
                    &which,
                );
                if !riser_gone {
                    corners_are(
                        body,
                        RISER,
                        &strip(
                            DVec3::new(0.0, TREAD, TREAD_HEIGHT + half),
                            DVec3::new(0.0, TREAD, height),
                        ),
                        &format!("{what}: what is left of the riser, a strip above the slope"),
                    );
                }
                for (wall, x) in [(LEFT, 0.0), (RIGHT, LENGTH)] {
                    corners_are_or_on(
                        body,
                        wall,
                        &short_stair_wall_at(x, height),
                        on_the_floor,
                        &format!("{what}: the stair's wall at x = {x}"),
                    );
                }
                let promised = LENGTH * TREAD * (TREAD_HEIGHT + height);
                sound_drawn_within(
                    body,
                    promised,
                    EXACT * promised,
                    &format!("{what}: as much matter as the nose gives up, the foot takes in"),
                );
            }
        }
    }
}

/// How far the tipped stair's riser stands back at its top, [`HEIGHT`] up,
/// from its bottom on the ground: tipped back by a ten-millionth of a
/// radian, parallel to the nose as the kernel reads two edges, its foot's
/// flat and the nose's a hair apart, further than the kernel's tolerance.
const RISER_TIP: f64 = 2e-6;

/// The numbers the tipped stair's riser and its two ends answer to: the
/// taller block raised along X, its start at x = 0 nought, its end one and
/// its riser five, the lower block's left and right merged with them.
const TIPPED_RISER: usize = 5;
const TIPPED_START: [u32; 2] = [0, LEFT as u32 + 6];
const TIPPED_END: [u32; 2] = [1, RIGHT as u32 + 6];

/// Where the tipped stair's riser stands across it at the height `z`.
fn tipped_riser_at(z: f64) -> f64 {
    TREAD + RISER_TIP * z / HEIGHT
}

/// The [`stair`] with its riser tipped back by [`RISER_TIP`] about its
/// bottom: the taller block raised along X from a section whose front runs
/// from (10, 0) to (10 + [`RISER_TIP`], 20), numbered its start nought, its
/// end one, its floor two, its back three, its top four and its riser five,
/// and in front of it the lower block, numbered as [`stair`]'s, reaching in
/// under it, as [`stair_with_its_riser_turned_by`]'s does, so that no face
/// of either lies a hair from another's.
fn stair_with_its_riser_tipped_back() -> Body {
    let part = Body::default();
    let taller = part
        .union(&raised_along_x(
            &part,
            vec![
                DVec2::new(TREAD, 0.0),
                DVec2::new(2.0 * TREAD, 0.0),
                DVec2::new(2.0 * TREAD, HEIGHT),
                DVec2::new(TREAD + RISER_TIP, HEIGHT),
            ],
            [0.0, LENGTH],
        ))
        .expect("the taller block is built");
    joined_to(
        &taller,
        [DVec2::ZERO, DVec2::new(LENGTH, 1.5 * TREAD)],
        TREAD_HEIGHT,
    )
}

/// The area a polygon of the plane encloses, its corners in order.
fn area_of(polygon: &[DVec2]) -> f64 {
    let turned = polygon.iter().zip(polygon.iter().cycle().skip(1));
    turned
        .map(|(at, next)| at.perp_dot(*next))
        .sum::<f64>()
        .abs()
        / 2.0
}

/// The matter of the tipped stair once one slope through `corners`, as
/// [`slope_corners`] gives them, stands in place of its tread: the section
/// at x = 0, the slope's corners there two of its corners, along the
/// stair's length.
fn tipped_matter_under(corners: [[DVec3; 2]; 2]) -> f64 {
    let [[front, _], [riser, _]] = corners;
    LENGTH
        * area_of(&[
            DVec2::ZERO,
            DVec2::new(2.0 * TREAD, 0.0),
            DVec2::new(2.0 * TREAD, HEIGHT),
            DVec2::new(TREAD + RISER_TIP, HEIGHT),
            DVec2::new(riser.y, riser.z),
            DVec2::new(front.y, front.z),
        ])
}

#[test]
fn a_chamfer_on_a_tread_s_nose_and_foot_whose_riser_tips_back_by_a_hair_asked_half_its_depth_or_more_lays_one_slope_whichever_is_named_first()
 {
    let stair = stair_with_its_riser_tipped_back();
    let half = TREAD / 2.0;
    let laid = [stair.faces_end(), stair.faces_end() + 1];
    let nose_and_foot = [
        edge_between(&stair, TREAD_TOP, NOSE),
        edge_between(&stair, TIPPED_RISER, TREAD_TOP),
    ];
    let whole = LENGTH
        * (2.0 * TREAD * TREAD_HEIGHT + TREAD * (HEIGHT - TREAD_HEIGHT)
            - RISER_TIP * (HEIGHT * HEIGHT - TREAD_HEIGHT * TREAD_HEIGHT) / (2.0 * HEIGHT));

    for asked in [half, MORE_THAN_HALF] {
        let mut corners_by_order = Vec::new();
        for (order, names, _) in ranked_either_way(nose_and_foot.clone()) {
            let chamfered = stair
                .chamfer_edges(&names, asked)
                .unwrap_or_else(|declined| {
                    panic!("{order}, asked {asked}: declined, {declined:?}")
                });

            let what = format!(
                "the chamfer on the nose and foot of the tread whose riser tips back by \
                 {RISER_TIP}, {order}, asked {asked}"
            );
            assert!(
                (chamfered.asked - asked).abs() < PLACE
                    && (chamfered.taken - half).abs() <= MEETING_HAIR,
                "{what}: asked {} and took {}, where the two flats meet a hair from {half}",
                chamfered.asked,
                chamfered.taken
            );
            let body = &chamfered.body;
            is_one_slope_meeting(
                &stair,
                body,
                laid,
                &[TREAD_TOP],
                &[
                    TIPPED_START.to_vec(),
                    TIPPED_END.to_vec(),
                    vec![NOSE as u32],
                    vec![TIPPED_RISER as u32],
                ],
                &what,
            );
            let corners = slope_corners(body, laid[0], |_, z| tipped_riser_at(z), &what);
            set_back_by(corners, half, MEETING_HAIR, &what);
            for [near, far] in corners {
                assert!(
                    DVec2::new(near.y, near.z).distance(DVec2::new(far.y, far.z)) < PLACE,
                    "{what}: the slope's corner at {near} and the one at {far} stand alike on \
                     either end, the stair a prism along X"
                );
            }
            let promised = tipped_matter_under(corners);
            assert!(
                (promised - whole).abs() <= 2.0 * LENGTH * TREAD * MEETING_HAIR,
                "{what}: under the slope the stair holds {promised}, where with its tread it held \
                 {whole}: what the nose gives up, the foot takes in"
            );
            sound_drawn_within(
                body,
                promised,
                EXACT * promised,
                &format!("{what}: the matter under the slope's own corners"),
            );
            corners_by_order.push(corners);
        }
        same_corners(
            corners_by_order[0],
            corners_by_order[1],
            &format!("the tipped stair, asked {asked}, the nose named first and the foot"),
        );
    }
}

/// Turns of the riser near the most the kernel reads as parallel to the
/// nose, a millionth of a radian: the nose's flat and the foot's part by up
/// to some thousand of its tolerances from one end of the stair to the
/// other, far more than any strip it reads as none.
const TURNS_NEAR_TANGENT: [f64; 2] = [5e-7, 9e-7];

/// The most two edges may turn from one another and still read as parallel
/// to the kernel, a millionth.
const PARALLEL: f64 = 1e-6;

#[test]
fn a_chamfer_on_a_tread_s_nose_and_foot_whose_riser_is_turned_by_nearly_what_the_kernel_reads_as_parallel_lays_one_slope_whichever_is_named_first()
 {
    let half = TREAD / 2.0;

    for turn in TURNS_NEAR_TANGENT {
        let stair = stair_with_its_riser_turned_by(turn);
        let along_the = |face: usize| {
            let corners = corners_of(&stair, face, "the turned stair");
            let foot: Vec<DVec3> = corners
                .into_iter()
                .filter(|corner| (corner.z - TREAD_HEIGHT).abs() < PLACE)
                .collect();
            assert_eq!(
                foot.len(),
                2,
                "the turned stair: face {face} has two corners at the tread's height: {foot:?}"
            );
            (foot[1] - foot[0]).normalize()
        };
        let parting = along_the(NOSE).cross(along_the(RISER)).length();
        assert!(
            (parting - turn).abs() <= turn / 100.0 && parting <= PARALLEL,
            "the riser turned by {turn}: its foot turns from the nose by {parting}, that turn, \
             within what the kernel reads as parallel, {PARALLEL}"
        );
        let laid = [stair.faces_end(), stair.faces_end() + 1];
        let mut corners_by_order = Vec::new();

        for (order, names) in nose_and_foot_either_way(&stair) {
            let what = format!(
                "the chamfer on the nose and foot of the tread whose riser is turned by {turn}, \
                 {order}, asked {half}"
            );
            let chamfered = stair
                .chamfer_edges(&names, half)
                .unwrap_or_else(|declined| panic!("{what}: declined, {declined:?}"));

            assert!(
                (chamfered.asked - half).abs() < PLACE
                    && (chamfered.taken - half).abs() <= turned_stray(turn),
                "{what}: asked {} and took {}, where the two flats meet at {half}, within a hair \
                 and how far they part along the stair",
                chamfered.asked,
                chamfered.taken
            );
            corners_by_order.push(is_one_turned_slope(
                turn,
                &stair,
                &chamfered.body,
                laid,
                &what,
            ));
        }
        same_corners(
            corners_by_order[0],
            corners_by_order[1],
            &format!("the riser turned by {turn}, the nose named first and the foot"),
        );
    }
}

/// Holds that a chamfer on the turned stair's nose and foot set both back by
/// `distance`, short of where they meet: the nose's flat answering to
/// `flats[0]` alone, from `distance` down the nose's front to `distance` in
/// along the tread, the foot's to `flats[1]` alone, from `distance` up the
/// turned riser to `distance` short of it on the tread, a strip of tread left
/// between them, and the stair's matter whole: what the nose gives up, the
/// foot takes in.
fn are_two_turned_flats_apart(
    stair: &Body,
    body: &Body,
    distance: f64,
    [nose_flat, foot_flat]: [usize; 2],
    what: &str,
) {
    let mut kept_and_laid = numbers(stair);
    kept_and_laid.extend([nose_flat, foot_flat]);
    assert_eq!(
        numbers(body),
        kept_and_laid,
        "{what}: every face of the stair, its tread among them, and a flat for each edge"
    );
    assert_eq!(
        body.faces_end(),
        stair.faces_end() + 2,
        "{what}: the next step numbers its faces past both flats"
    );
    let foot_at = |x: f64| DVec3::new(x, turned_riser_at(x), TREAD_HEIGHT);
    let short_of_the_riser = |x: f64| foot_at(x) - DVec3::Y * distance;
    let ends = [STAIR_RIGHT.to_vec(), STAIR_LEFT.to_vec()];
    for (number, corners, met, which) in [
        (
            nose_flat,
            strip(
                DVec3::new(0.0, 0.0, TREAD_HEIGHT - distance),
                DVec3::new(0.0, distance, TREAD_HEIGHT),
            ),
            NOSE,
            "the nose cut off",
        ),
        (
            foot_flat,
            [
                foot_at(0.0) + DVec3::Z * distance,
                foot_at(LENGTH) + DVec3::Z * distance,
                short_of_the_riser(LENGTH),
                short_of_the_riser(0.0),
            ],
            RISER,
            "the foot filled in",
        ),
    ] {
        let which = format!("{what}: face {number}, {which} at {distance}");
        corners_are(body, number, &corners, &which);
        assert_eq!(
            sides_met(body, number),
            sides_by(&[&ends[..], &[vec![met as u32], vec![TREAD_TOP as u32]]].concat()),
            "{which}: a face answering to its own number alone, meeting face {met}, the strip \
             of tread, and the stair's two ends, by one edge each"
        );
    }
    assert!(
        body.edge_names()
            .iter()
            .all(|name| !lies_between(name, nose_flat, foot_flat)),
        "{what}: the two flats do not meet, the tread between them"
    );
    corners_are(
        body,
        TREAD_TOP,
        &[
            DVec3::new(0.0, distance, TREAD_HEIGHT),
            DVec3::new(LENGTH, distance, TREAD_HEIGHT),
            short_of_the_riser(LENGTH),
            short_of_the_riser(0.0),
        ],
        &format!("{what}: what is left of the tread, a strip between the two flats"),
    );
    let promised = LENGTH * TREAD * (HEIGHT + TREAD_HEIGHT);
    sound_drawn_within(
        body,
        promised,
        EXACT * promised,
        &format!("{what}: as much matter as the nose gives up, the foot takes in"),
    );
}

/// The section of a stair whose front leans back, drawn on the plane of Y
/// and Z as [`on_section`] reads it, and raised along X for [`LENGTH`]: a
/// floor 20 deep, a back and the riser [`HEIGHT`] and [`TREAD_HEIGHT`] up,
/// the taller part's top 10 deep, and a tread [`LEANING_TREAD`] deep from
/// the riser to its nose, whose front leans down from the nose to the floor's
/// front. The face below the nose is not parallel to the riser, so a cut off
/// the nose and a fill at the foot, set back by the same value, are flats at
/// two angles. Raised along X, its numbers are its start at x = 0 nought, its
/// end one, then its floor two, its back three, the top four, the riser five,
/// the tread six and the leaning front seven.
const LEANING_STAIR: [DVec2; 6] = [
    DVec2::new(0.0, 0.0),
    DVec2::new(2.0 * TREAD, 0.0),
    DVec2::new(2.0 * TREAD, HEIGHT),
    DVec2::new(TREAD, HEIGHT),
    DVec2::new(TREAD, TREAD_HEIGHT),
    DVec2::new(TREAD - LEANING_TREAD, TREAD_HEIGHT),
];
const LEANING_TREAD: f64 = 8.0;
const LEANING_START: usize = 0;
const LEANING_END: usize = 1;
const LEANING_RISER: usize = 5;
const LEANING_TREAD_TOP: usize = 6;
const LEANING_FRONT: usize = 7;

/// The [`LEANING_STAIR`] section raised along X for [`LENGTH`].
fn leaning_stair() -> Body {
    let part = Body::default();
    part.union(&raised_along_x(
        &part,
        LEANING_STAIR.to_vec(),
        [0.0, LENGTH],
    ))
    .expect("the part is built")
}

/// A section drawn on the plane of Y and Z, as [`on_section`] reads it,
/// raised along X from `from` to `to` as a tool for `part`, its faces
/// numbered past the part's.
fn raised_along_x(part: &Body, section: Vec<DVec2>, [from, to]: [f64; 2]) -> Body {
    part.tool_raised(
        &profile(Contour::straight(section)),
        Frame {
            origin: DVec3::X * from,
            u: DVec3::Y,
            v: DVec3::Z,
        },
        DVec3::X * (to - from),
    )
    .expect("the tool raises")
}

/// The way a flat face of a part raised along X faces, where its section
/// runs from `from` to `to` with the matter on its left.
fn facing_out_of(from: DVec2, to: DVec2) -> DVec3 {
    let along = (to - from).normalize();
    DVec3::new(0.0, along.y, -along.x)
}

/// The edge between the face whose numbers are `first` and the one whose
/// numbers are `second`, either of them a face a join merged, answering to
/// the number of each block it came from.
fn edge_on_sides(body: &Body, first: &[u32], second: &[u32]) -> EdgeName {
    let named: Vec<EdgeName> = body
        .edge_names()
        .into_iter()
        .filter(|name| {
            let [one, other] = &name.between;
            (one == first && other == second) || (one == second && other == first)
        })
        .collect();
    assert_eq!(
        named.len(),
        1,
        "one edge between {first:?} and {second:?}: {named:?}"
    );
    named[0].clone()
}

/// The leaning stair's nose and foot.
fn leaning_nose_and_foot(stair: &Body) -> [EdgeName; 2] {
    [
        edge_between(stair, LEANING_TREAD_TOP, LEANING_FRONT),
        edge_between(stair, LEANING_RISER, LEANING_TREAD_TOP),
    ]
}

/// A nose and a foot, each list in the two orders a step may name them, with
/// the order said and the ranks the list gives the nose and the foot.
fn ranked_either_way(
    [nose, foot]: [EdgeName; 2],
) -> [(&'static str, [EdgeName; 2], [usize; 2]); 2] {
    [
        ("the nose named first", [nose.clone(), foot.clone()], [0, 1]),
        ("the foot named first", [foot, nose], [1, 0]),
    ]
}

/// Where a chamfer of `distance` on the leaning stair's nose and foot sets
/// back the faces they lie between, on its section: on the tread from the
/// nose, down the leaning front from it, up the riser from the foot, and on
/// the tread from the foot. At half the tread's depth the two on the tread
/// are one point, (6, 10).
fn leaning_set_backs(distance: f64) -> [DVec2; 4] {
    let nose_at = LEANING_STAIR[5];
    let foot_at = LEANING_STAIR[4];
    [
        nose_at + DVec2::X * distance,
        nose_at + (LEANING_STAIR[0] - nose_at).normalize() * distance,
        foot_at + DVec2::Y * distance,
        foot_at - DVec2::X * distance,
    ]
}

/// The matter of the leaning stair once a chamfer of `distance` cuts its nose
/// off and fills its foot in, along its length: its section, 290, less the
/// triangle the nose gives up, two sides `distance` long holding between
/// them the angle of the tread and the leaning front, whose sine is 10/√104,
/// and more the triangle the foot takes in, two sides `distance` long square
/// to each other — 30·(290 − 5d²/√104 + d²/2).
fn leaning_matter(distance: f64) -> f64 {
    let section = 2.0 * TREAD * HEIGHT
        - TREAD * (HEIGHT - TREAD_HEIGHT)
        - (TREAD - LEANING_TREAD) * TREAD_HEIGHT / 2.0;
    let nose_at = LEANING_STAIR[5];
    let [on_tread, on_front, _, _] = leaning_set_backs(distance);
    let cut = (on_tread - nose_at).perp_dot(on_front - nose_at).abs() / 2.0;
    let filled = distance * distance / 2.0;
    LENGTH * (section - cut + filled)
}

/// The matter the booleans leave of the leaning stair once a chamfer's
/// triangles of `distance` are cut away at its nose and joined on at its
/// foot: the nose's raised past both ends of the stair and run on into the
/// air beyond the tread and the leaning front, the foot's raised from one
/// end to the other and run on into the matter behind the riser and under
/// the tread.
fn leaning_matter_by_booleans(stair: &Body, distance: f64) -> f64 {
    let [nose_on_tread, on_front, on_riser, foot_on_tread] = leaning_set_backs(distance);
    let above = TREAD_HEIGHT + 2.0;
    let before = -2.0;
    let nose = raised_along_x(
        stair,
        vec![
            on_front,
            nose_on_tread,
            DVec2::new(nose_on_tread.x, above),
            DVec2::new(before, above),
            DVec2::new(before, on_front.y),
        ],
        [-1.0, LENGTH + 1.0],
    );
    let cut = stair
        .difference(&nose)
        .expect("the nose's triangle cuts the stair");
    let below = TREAD_HEIGHT - 1.0;
    let behind = TREAD + 1.0;
    let foot = raised_along_x(
        &cut,
        vec![
            foot_on_tread,
            DVec2::new(foot_on_tread.x, below),
            DVec2::new(behind, below),
            DVec2::new(behind, on_riser.y),
            on_riser,
        ],
        [0.0, LENGTH],
    );
    cut.union(&foot)
        .expect("the foot's triangle joins the stair")
        .volume()
}

/// Holds that a chamfer on the leaning stair's nose and foot set both back by
/// `distance`: the nose's flat answering to `flats[0]` alone, from the tread
/// down the leaning front, and the foot's to `flats[1]` alone, from the riser
/// down to the tread, on z − y = `distance`. Short of half the tread's depth
/// the tread is left, a strip between them; at half it is gone, and the two
/// meet by one edge, a crease along y = 6, z = 10 from one end of the stair
/// to the other. What is left of the leaning front, of the riser and of the
/// stair's two ends follows, and the matter is what [`leaning_matter`]
/// gives, which the booleans give too.
fn is_the_leaning_chamfer(
    stair: &Body,
    body: &Body,
    distance: f64,
    [nose_flat, foot_flat]: [usize; 2],
    what: &str,
) {
    let meet = distance >= LEANING_TREAD / 2.0 - PLACE;
    let [nose_on_tread, on_front, on_riser, foot_on_tread] = leaning_set_backs(distance);
    let mut kept_and_laid = numbers(stair);
    if meet {
        assert!(
            !body.has_face(LEANING_TREAD_TOP),
            "{what}: the tread is gone"
        );
        kept_and_laid.remove(&LEANING_TREAD_TOP);
    }
    kept_and_laid.extend([nose_flat, foot_flat]);
    assert_eq!(
        numbers(body),
        kept_and_laid,
        "{what}: every face of the stair, {}, and a flat for each edge",
        if meet {
            "but its tread"
        } else {
            "its tread among them"
        }
    );
    assert_eq!(
        body.faces_end(),
        stair.faces_end() + 2,
        "{what}: the next step numbers its faces past both flats"
    );
    for (number, [from, to], met, other, which) in [
        (
            nose_flat,
            [nose_on_tread, on_front],
            LEANING_FRONT,
            foot_flat,
            "the nose cut off",
        ),
        (
            foot_flat,
            [on_riser, foot_on_tread],
            LEANING_RISER,
            nose_flat,
            "the foot filled in",
        ),
    ] {
        let which = format!("{what}: face {number}, {which} at {distance}");
        face_facing(body, &[number], facing_out_of(from, to), &which);
        corners_are(
            body,
            number,
            &strip(on_section(from, 0.0), on_section(to, 0.0)),
            &which,
        );
        let (beside, how) = if meet {
            (other, "the other flat along the crease where the tread was")
        } else {
            (LEANING_TREAD_TOP, "the strip of tread")
        };
        assert_eq!(
            sides_met(body, number),
            sides(&[LEANING_START, LEANING_END, met, beside]),
            "{which}: a face answering to its own number alone, meeting face {met}, {how}, and \
             the stair's two ends, by one edge each"
        );
    }
    let foot_corners = body
        .plane_of(foot_flat)
        .map(|plane| plane.corners)
        .unwrap_or_default();
    assert!(
        foot_corners
            .iter()
            .all(|corner| (corner.z - corner.y - distance).abs() < PLACE),
        "{what}: the foot's flat lies on z − y = {distance}: {foot_corners:?}"
    );
    let mut pair = [nose_flat, foot_flat];
    pair.sort_unstable();
    assert!(
        sides_met_by(body, &pair).is_empty(),
        "{what}: no face answers to both {pair:?}: the two flats are two faces, not one slope"
    );
    if meet {
        edge_between(body, nose_flat, foot_flat);
    } else {
        assert!(
            body.edge_names()
                .iter()
                .all(|name| !lies_between(name, nose_flat, foot_flat)),
            "{what}: the two flats do not meet, the tread between them"
        );
        corners_are(
            body,
            LEANING_TREAD_TOP,
            &strip(
                on_section(nose_on_tread, 0.0),
                on_section(foot_on_tread, 0.0),
            ),
            &format!("{what}: what is left of the tread, a strip between the two flats"),
        );
    }
    corners_are(
        body,
        LEANING_FRONT,
        &strip(on_section(LEANING_STAIR[0], 0.0), on_section(on_front, 0.0)),
        &format!("{what}: what is left of the leaning front, under the nose's flat"),
    );
    corners_are(
        body,
        LEANING_RISER,
        &strip(on_section(on_riser, 0.0), on_section(LEANING_STAIR[3], 0.0)),
        &format!("{what}: what is left of the riser, above the foot's flat"),
    );
    let mut section = vec![
        LEANING_STAIR[0],
        LEANING_STAIR[1],
        LEANING_STAIR[2],
        LEANING_STAIR[3],
        on_riser,
        foot_on_tread,
    ];
    if !meet {
        section.push(nose_on_tread);
    }
    section.push(on_front);
    for (end, x) in [(LEANING_START, 0.0), (LEANING_END, LENGTH)] {
        let corners: Vec<DVec3> = section.iter().map(|&at| on_section(at, x)).collect();
        corners_are(
            body,
            end,
            &corners,
            &format!("{what}: the stair's end at x = {x}"),
        );
    }
    let promised = leaning_matter(distance);
    let by_booleans = leaning_matter_by_booleans(stair, distance);
    assert!(
        (by_booleans - promised).abs() <= EXACT * promised,
        "{what}: the booleans leave {by_booleans}, where arithmetic gives {promised}"
    );
    sound_drawn_within(
        body,
        promised,
        EXACT * promised,
        &format!("{what}: what the nose gives up taken off, what the foot takes in added"),
    );
}

#[test]
fn a_chamfer_on_the_nose_and_foot_of_a_tread_whose_front_leans_asked_half_its_depth_or_more_takes_half_in_two_flats_meeting_in_a_crease_whichever_is_named_first()
 {
    let stair = leaning_stair();
    let limit = LEANING_TREAD / 2.0;
    let past = 6.0;
    let edges = leaning_nose_and_foot(&stair);
    alone_each_takes_more(
        |name| stair.chamfer(name, past).map(|single| single.taken),
        &edges,
        limit,
        "the chamfer on the leaning stair's nose or its foot",
    );
    let crease = 30.0 * (290.0 - 80.0 / 104_f64.sqrt() + 8.0);
    assert!(
        (leaning_matter(limit) - crease).abs() <= EXACT * crease,
        "the leaning stair chamfered at {limit} holds 30·(290 − 80/√104 + 8), {crease}, where \
         the arithmetic of its triangles gives {}",
        leaning_matter(limit)
    );

    for (order, names, ranks) in ranked_either_way(edges.clone()) {
        for asked in [limit, past] {
            let chamfered = stair
                .chamfer_edges(&names, asked)
                .unwrap_or_else(|declined| {
                    panic!("{order}, asked {asked}: declined, {declined:?}")
                });

            let what =
                format!("the chamfer on the leaning stair's nose and foot, {order}, asked {asked}");
            reduced(chamfered.asked, chamfered.taken, [asked, limit], &what);
            is_the_leaning_chamfer(
                &stair,
                &chamfered.body,
                limit,
                ranks.map(|rank| stair.faces_end() + rank),
                &what,
            );
        }
    }
}

#[test]
fn a_chamfer_on_the_nose_and_foot_of_a_tread_whose_front_leans_asked_a_hair_under_half_its_depth_lays_the_same_crease_whichever_is_named_first()
 {
    let stair = leaning_stair();
    let limit = LEANING_TREAD / 2.0;

    for (order, names, ranks) in ranked_either_way(leaning_nose_and_foot(&stair)) {
        for (under, past_one_hair) in [(1e-6, true), (1e-7, false)] {
            let asked = limit - under;
            let strip = LEANING_TREAD - 2.0 * asked;
            assert!(
                strip < MEETING_HAIR && (strip > ONE_HAIR) == past_one_hair,
                "asked {asked}: the strip of tread the two flats leave, {strip}, is shorter than \
                 the kernel keeps, {MEETING_HAIR}, and longer than one of its hairs, \
                 {ONE_HAIR}, or not, as {past_one_hair} says"
            );

            let chamfered = stair
                .chamfer_edges(&names, asked)
                .unwrap_or_else(|declined| {
                    panic!("{order}, asked {asked}: declined, {declined:?}")
                });

            let what =
                format!("the chamfer on the leaning stair's nose and foot, {order}, asked {asked}");
            assert!(
                (chamfered.asked - asked).abs() < PLACE
                    && (chamfered.taken - limit).abs() <= MEETING_HAIR,
                "{what}: asked {} and took {}, where {asked} is a hair from what both give",
                chamfered.asked,
                chamfered.taken
            );
            is_the_leaning_chamfer(
                &stair,
                &chamfered.body,
                limit,
                ranks.map(|rank| stair.faces_end() + rank),
                &what,
            );
        }
    }
}

#[test]
fn a_chamfer_on_the_nose_and_foot_of_a_tread_whose_front_leans_or_whose_riser_is_turned_short_of_the_limit_keeps_the_body_main_lays_whichever_is_named_first()
 {
    let leaning = leaning_stair();
    for (asked, measured) in [(3.0, 8702.621609), (2.0, 8701.165159)] {
        assert!(
            (leaning_matter(asked) - measured).abs() < 1e-6,
            "the leaning stair chamfered at {asked} holds 30·(290 − 5d²/√104 + d²/2), {}, as a \
             probe on main measured it, {measured}",
            leaning_matter(asked)
        );
        for (order, names, ranks) in ranked_either_way(leaning_nose_and_foot(&leaning)) {
            let chamfered = leaning
                .chamfer_edges(&names, asked)
                .unwrap_or_else(|declined| {
                    panic!("the leaning stair, {order}, asked {asked}: declined, {declined:?}")
                });

            let what =
                format!("the chamfer on the leaning stair's nose and foot, {order}, asked {asked}");
            taken_as_asked(chamfered.asked, chamfered.taken, asked, &what);
            is_the_leaning_chamfer(
                &leaning,
                &chamfered.body,
                asked,
                ranks.map(|rank| leaning.faces_end() + rank),
                &what,
            );
        }
    }

    let turned = stair_with_its_riser_turned_by(RISER_TURN);
    let asked = 4.0;
    let [(_, nose_and_foot), _] = nose_and_foot_either_way(&turned);
    for (order, names, ranks) in ranked_either_way(nose_and_foot) {
        let chamfered = turned
            .chamfer_edges(&names, asked)
            .unwrap_or_else(|declined| {
                panic!("the turned stair, {order}, asked {asked}: declined, {declined:?}")
            });

        let what = format!(
            "the chamfer on the nose and foot of the tread whose riser is turned by {RISER_TURN}, \
             {order}, asked {asked}"
        );
        taken_as_asked(chamfered.asked, chamfered.taken, asked, &what);
        are_two_turned_flats_apart(
            &turned,
            &chamfered.body,
            asked,
            ranks.map(|rank| turned.faces_end() + rank),
            &what,
        );
    }
}

/// A flight of `steps` steps, [`LENGTH`] long, each tread [`TREAD`] deep and
/// each riser [`TREAD_HEIGHT`] high: the block of the `step`-th step,
/// counting from one at the front, from y = 10·(step − 1) to 10·step and
/// 10·step high, the blocks joined as [`stair`] joins its two, the tallest
/// first, so that a flight of two steps is the [`stair`]. Each block's faces
/// answer to the numbers [`flight_top`] and [`flight_front`] give.
fn flight_of(steps: usize) -> Body {
    (1..=steps).rev().fold(Body::default(), |part, step| {
        let back = TREAD * step as f64;
        joined_to(
            &part,
            [DVec2::new(0.0, back - TREAD), DVec2::new(LENGTH, back)],
            TREAD_HEIGHT * step as f64,
        )
    })
}

/// The number the top of the `step`-th step of a flight of `steps` answers
/// to, counting from one at the front: the blocks are joined the tallest
/// first, six faces each, so that the front block's come last.
fn flight_top(steps: usize, step: usize) -> usize {
    6 * (steps - step) + TOP
}

/// The number the front of the `step`-th step of a flight of `steps`
/// answers to: the nose's front under the first tread, and the riser under
/// each other.
fn flight_front(steps: usize, step: usize) -> usize {
    6 * (steps - step) + FRONT
}

/// The sides of the two ends of a flight of `steps`, its right then its
/// left, each the walls of every block merged into one.
fn flight_ends(steps: usize) -> [Vec<u32>; 2] {
    [RIGHT, LEFT].map(|wall| (0..steps).map(|block| (6 * block + wall) as u32).collect())
}

/// The noses and the feet of a flight of `steps`, alternating up from the
/// first nose: each step's nose, between its top and its front, then its
/// foot, between its top and the next step's front.
fn flight_edges(flight: &Body, steps: usize) -> Vec<EdgeName> {
    (1..=steps)
        .flat_map(|step| {
            let nose = edge_between(flight, flight_top(steps, step), flight_front(steps, step));
            let foot = (step < steps).then(|| {
                edge_between(
                    flight,
                    flight_front(steps, step + 1),
                    flight_top(steps, step),
                )
            });
            std::iter::once(nose).chain(foot)
        })
        .collect()
}

/// The matter of a flight of `steps` once one slope set back half a tread's
/// depth stands in place of its noses and feet: its section, each block a
/// tread deep and as many risers high as its step's count, less the
/// triangle each nose gives up and more the one each foot takes in, two
/// sides half a tread long square to each other, a nose more than there are
/// feet — 30·(300 − 12.5), 8625, on two steps, and 30·(600 − 12.5), 17625,
/// on three.
fn flight_matter(steps: usize) -> f64 {
    let half = TREAD / 2.0;
    let section: f64 = (1..=steps)
        .map(|step| TREAD * TREAD_HEIGHT * step as f64)
        .sum();
    let triangle = half * half / 2.0;
    let [noses, feet] = [steps, steps - 1].map(|count| count as f64 * triangle);
    LENGTH * (section - noses + feet)
}

/// The matter the booleans leave of a flight of `steps` once a box over each
/// foot is joined on, run on into the tread below it and the riser behind
/// it, and everything above z = y + 5 is cut away by a wedge run on past the
/// front, the top and both ends: what one slope leaves, no face of either
/// tool lying on one of the flight's but the boxes' ends, on its own.
fn flight_matter_by_booleans(flight: &Body, steps: usize) -> f64 {
    let half = TREAD / 2.0;
    let filled = (1..steps).fold(flight.clone(), |part, step| {
        let foot = DVec2::new(TREAD, TREAD_HEIGHT) * step as f64;
        let [low, high] = [
            foot + DVec2::new(-half - 1.0, -2.0),
            foot + DVec2::new(1.0, half + 2.0),
        ];
        let over_the_foot = raised_along_x(
            &part,
            vec![
                low,
                DVec2::new(high.x, low.y),
                high,
                DVec2::new(low.x, high.y),
            ],
            [0.0, LENGTH],
        );
        part.union(&over_the_foot)
            .expect("the box over the foot joins the flight")
    });
    let over = TREAD_HEIGHT - half;
    let above = TREAD_HEIGHT * steps as f64 + 2.0;
    let before = -2.0;
    let wedge = raised_along_x(
        &filled,
        vec![
            DVec2::new(before, before + over),
            DVec2::new(above - over, above),
            DVec2::new(before, above),
        ],
        [-1.0, LENGTH + 1.0],
    );
    filled
        .difference(&wedge)
        .expect("the wedge above the slope cuts the flight")
        .volume()
}

/// Holds that a chamfer at half a tread's depth on every nose and foot of
/// the [`flight_of`] `steps` laid one slope across it: its treads and its
/// risers between gone and no other face of it, one flat face answering to
/// every edge's number on z = y + 5, from half down the first nose's front
/// to half back on the last step's top, meeting those two and the flight's
/// two ends by one edge each, the two ends cut back to it, and the matter
/// [`flight_matter`] gives.
fn is_one_slope_across_the_flight(flight: &Body, steps: usize, body: &Body, what: &str) {
    let half = TREAD / 2.0;
    let depth = TREAD * steps as f64;
    let rise = TREAD_HEIGHT * steps as f64;
    let start = flight.faces_end();
    let laid: Vec<usize> = (start..start + 2 * steps - 1).collect();
    let between: Vec<usize> = (1..steps)
        .flat_map(|step| [flight_top(steps, step), flight_front(steps, step + 1)])
        .collect();
    let mut kept_and_laid = numbers(flight);
    for gone in &between {
        assert!(
            !body.has_face(*gone),
            "{what}: face {gone}, a tread or a riser between, is gone"
        );
        kept_and_laid.remove(gone);
    }
    kept_and_laid.extend(&laid);
    assert_eq!(
        numbers(body),
        kept_and_laid,
        "{what}: every face of the flight but its treads and its risers between, and the slope \
         answering to {laid:?}"
    );
    assert_eq!(
        body.faces_end(),
        start + laid.len(),
        "{what}: the next step numbers its faces past every number the slope answers to"
    );
    one_flat_face_answers_to(body, &laid, what);
    for (rank, &one) in laid.iter().enumerate() {
        for &other in &laid[rank + 1..] {
            assert!(
                body.edge_names()
                    .iter()
                    .all(|name| !lies_between(name, one, other)),
                "{what}: no edge stands between {one} and {other}: they are one face"
            );
        }
    }
    let front = flight_front(steps, 1);
    let top = flight_top(steps, steps);
    let [right, left] = flight_ends(steps);
    assert_eq!(
        sides_met_by(body, &laid),
        sides_by(&[right, left, vec![front as u32], vec![top as u32]]),
        "{what}: the slope meets the first nose's front, the last step's top and the flight's \
         two ends, by one edge each"
    );
    let [low, high] = [
        DVec3::new(0.0, 0.0, TREAD_HEIGHT - half),
        DVec3::new(0.0, depth - half, rise),
    ];
    let which = format!(
        "{what}: the slope on z = y + {}, from {low} to {high}",
        low.z
    );
    face_facing(
        body,
        &[laid[0]],
        DVec3::new(0.0, -1.0, 1.0).normalize(),
        &which,
    );
    corners_are(body, laid[0], &strip(low, high), &which);
    corners_are(
        body,
        front,
        &strip(DVec3::ZERO, low),
        &format!("{what}: what is left of the first nose's front, under the slope"),
    );
    corners_are(
        body,
        top,
        &strip(high, DVec3::new(0.0, depth, rise)),
        &format!("{what}: what is left of the last step's top, behind the slope"),
    );
    let on_its_floor =
        |corner: DVec3| corner.z.abs() < PLACE && (-PLACE..=depth + PLACE).contains(&corner.y);
    for (wall, x) in [(LEFT, 0.0), (RIGHT, LENGTH)] {
        let at = DVec3::X * x;
        corners_are_or_on(
            body,
            wall,
            &[
                at,
                at + DVec3::new(0.0, depth, 0.0),
                at + DVec3::new(0.0, depth, rise),
                at + high,
                at + low,
            ],
            on_its_floor,
            &format!("{what}: the flight's end at x = {x}, cut back to the slope"),
        );
    }
    let promised = flight_matter(steps);
    sound_drawn_within(
        body,
        promised,
        EXACT * promised,
        &format!("{what}: what the noses give up taken off, what the feet take in added"),
    );
}

#[test]
fn a_chamfer_on_a_flight_s_first_nose_the_foot_between_and_the_second_nose_where_its_treads_are_as_deep_as_its_risers_are_high_takes_half_in_one_slope_across_it_in_every_order()
 {
    let stair = stair();
    let edges = [
        edge_between(&stair, TREAD_TOP, NOSE),
        edge_between(&stair, RISER, TREAD_TOP),
        edge_between(&stair, TOP, RISER),
    ];
    assert!(
        flight_edges(&stair, 2) == edges
            && flight_ends(2) == [STAIR_RIGHT.to_vec(), STAIR_LEFT.to_vec()],
        "the stair is the flight of two steps, its first nose, its foot and its second nose \
         named up from the front, its two ends the stair's"
    );
    let half = TREAD / 2.0;
    alone_each_takes_more(
        |name| {
            stair
                .chamfer(name, MORE_THAN_HALF)
                .map(|single| single.taken)
        },
        &edges,
        half,
        "the chamfer on the flight's first nose, its foot or its second nose",
    );
    for (pair, which) in [
        ([0, 1], "the first nose and the foot, across the tread"),
        ([1, 2], "the foot and the second nose, across the riser"),
    ] {
        let names = pair.map(|edge| edges[edge].clone());
        assert!(
            stair.chamfer_edges(&names, half).is_ok(),
            "{which}, alone, asked {half}: laid as one slope, so that the three are the foot \
             meeting both"
        );
    }
    let promised = flight_matter(2);
    let by_booleans = flight_matter_by_booleans(&stair, 2);
    assert!(
        (promised - 8625.0).abs() < PLACE && (by_booleans - promised).abs() <= EXACT * promised,
        "the stair under one slope holds 30·(300 − 12.5), 8625: arithmetic gives {promised}, \
         the booleans {by_booleans}"
    );

    for asked in [half, MORE_THAN_HALF] {
        for order in every_order(&[0, 1, 2]) {
            let names: Vec<EdgeName> = order.iter().map(|&edge| edges[edge].clone()).collect();
            let what = format!(
                "the chamfer on the flight's first nose, foot and second nose, named in the \
                 order {order:?}, asked {asked}"
            );

            let chamfered = stair
                .chamfer_edges(&names, asked)
                .unwrap_or_else(|declined| panic!("{what}: declined, {declined:?}"));

            reduced(chamfered.asked, chamfered.taken, [asked, half], &what);
            is_one_slope_across_the_flight(&stair, 2, &chamfered.body, &what);
        }
    }
}

#[test]
fn a_chamfer_on_the_three_noses_and_two_feet_of_a_flight_of_three_steps_whose_treads_are_as_deep_as_its_risers_are_high_takes_half_in_one_slope_across_it_in_every_order()
 {
    let steps = 3;
    let flight = flight_of(steps);
    let edges = flight_edges(&flight, steps);
    let half = TREAD / 2.0;
    let promised = flight_matter(steps);
    let by_booleans = flight_matter_by_booleans(&flight, steps);
    assert!(
        edges.len() == 5
            && (promised - 17625.0).abs() < PLACE
            && (by_booleans - promised).abs() <= EXACT * promised,
        "the flight of three steps has three noses and two feet, {} edges, and under one slope \
         holds 30·(600 − 12.5), 17625: arithmetic gives {promised}, the booleans {by_booleans}",
        edges.len()
    );
    for end in flight_ends(steps) {
        edge_on_sides(&flight, &end, &[flight_front(steps, 1) as u32]);
    }
    let up: Vec<usize> = (0..edges.len()).collect();
    let down: Vec<usize> = up.iter().rev().copied().collect();

    for (asked, orders) in [
        (half, every_order(&up)),
        (MORE_THAN_HALF, vec![up.clone(), down]),
    ] {
        for order in orders {
            let names: Vec<EdgeName> = order.iter().map(|&edge| edges[edge].clone()).collect();
            let what = format!(
                "the chamfer on the three noses and two feet of a flight of three steps, named \
                 in the order {order:?}, asked {asked}"
            );

            let chamfered = flight
                .chamfer_edges(&names, asked)
                .unwrap_or_else(|declined| panic!("{what}: declined, {declined:?}"));

            reduced(chamfered.asked, chamfered.taken, [asked, half], &what);
            is_one_slope_across_the_flight(&flight, steps, &chamfered.body, &what);
        }
    }
}

/// The [`stair`] with its tread `tread` deep and its riser `riser` high
/// rather than both 10: the lower block from y = 0 to `tread`,
/// [`TREAD_HEIGHT`] high, and behind it the taller, [`TREAD`] deep and
/// `riser` higher, joined as [`stair`] joins them, its faces answering to
/// the numbers [`stair`]'s do.
fn stair_of(tread: f64, riser: f64) -> Body {
    let taller = joined_to(
        &Body::default(),
        [DVec2::new(0.0, tread), DVec2::new(LENGTH, tread + TREAD)],
        TREAD_HEIGHT + riser,
    );
    joined_to(
        &taller,
        [DVec2::ZERO, DVec2::new(LENGTH, tread)],
        TREAD_HEIGHT,
    )
}

/// A flat face a chamfer lays on a stair: the edges it answers to, by their
/// places in the list up the stair, its run on the stair's section from the
/// front up, and the two faces it meets either side, besides the stair's
/// two ends.
struct LaidFlat {
    edges: Vec<usize>,
    run: [DVec2; 2],
    between: [usize; 2],
}

#[test]
fn a_chamfer_on_a_flight_s_first_nose_the_foot_between_and_the_second_nose_where_its_tread_and_riser_differ_keeps_the_body_main_lays_one_slope_across_the_shorter_and_a_strip_of_the_other_in_every_order()
 {
    let ends = [STAIR_RIGHT.to_vec(), STAIR_LEFT.to_vec()];

    for (tread, riser) in [(TREAD, 6.0), (6.0, TREAD_HEIGHT)] {
        let stair = stair_of(tread, riser);
        let edges = [
            edge_between(&stair, TREAD_TOP, NOSE),
            edge_between(&stair, RISER, TREAD_TOP),
            edge_between(&stair, TOP, RISER),
        ];
        let taken = tread.min(riser) / 2.0;
        let back = tread + TREAD;
        let height = TREAD_HEIGHT + riser;
        let on_front = DVec2::new(0.0, TREAD_HEIGHT - taken);
        let nose_on_tread = DVec2::new(taken, TREAD_HEIGHT);
        let foot_on_tread = DVec2::new(tread - taken, TREAD_HEIGHT);
        let foot_on_riser = DVec2::new(tread, TREAD_HEIGHT + taken);
        let top_on_riser = DVec2::new(tread, height - taken);
        let on_top = DVec2::new(tread + taken, height);
        let stair_is = format!("the stair whose tread is {tread} deep under a riser {riser} high");
        let (slope, flat, (kept, kept_run), gone) = if riser < tread {
            (
                LaidFlat {
                    edges: vec![1, 2],
                    run: [foot_on_tread, on_top],
                    between: [TREAD_TOP, TOP],
                },
                LaidFlat {
                    edges: vec![0],
                    run: [on_front, nose_on_tread],
                    between: [NOSE, TREAD_TOP],
                },
                (TREAD_TOP, [nose_on_tread, foot_on_tread]),
                RISER,
            )
        } else {
            (
                LaidFlat {
                    edges: vec![0, 1],
                    run: [on_front, foot_on_riser],
                    between: [NOSE, RISER],
                },
                LaidFlat {
                    edges: vec![2],
                    run: [top_on_riser, on_top],
                    between: [RISER, TOP],
                },
                (RISER, [foot_on_riser, top_on_riser]),
                TREAD_TOP,
            )
        };
        let section = [
            DVec2::ZERO,
            DVec2::new(back, 0.0),
            DVec2::new(back, height),
            on_top,
            kept_run[1],
            kept_run[0],
            on_front,
        ];
        let promised = LENGTH * (tread * TREAD_HEIGHT + TREAD * height - taken * taken / 2.0);
        assert!(
            (promised - 7665.0).abs() < PLACE,
            "{stair_is}, chamfered at {taken}, holds 30·(260 − 4.5), 7665, where arithmetic \
             gives {promised}"
        );
        let start = stair.faces_end();

        for asked in [taken, MORE_THAN_HALF] {
            for order in every_order(&[0, 1, 2]) {
                let names: Vec<EdgeName> = order.iter().map(|&edge| edges[edge].clone()).collect();
                let what = format!(
                    "the chamfer on the first nose, foot and second nose of {stair_is}, named in \
                     the order {order:?}, asked {asked}"
                );

                let chamfered = stair
                    .chamfer_edges(&names, asked)
                    .unwrap_or_else(|declined| panic!("{what}: declined, {declined:?}"));

                reduced(chamfered.asked, chamfered.taken, [asked, taken], &what);
                let body = &chamfered.body;
                let number_of = |edge: usize| {
                    start
                        + order
                            .iter()
                            .position(|&named| named == edge)
                            .expect("each edge is named")
                };
                assert!(!body.has_face(gone), "{what}: face {gone} is gone");
                let mut kept_and_laid = numbers(&stair);
                kept_and_laid.remove(&gone);
                kept_and_laid.extend(start..start + 3);
                assert_eq!(
                    numbers(body),
                    kept_and_laid,
                    "{what}: every face of the stair but {gone}, a slope and a flat"
                );
                assert_eq!(
                    body.faces_end(),
                    start + 3,
                    "{what}: the next step numbers its faces past the three edges' numbers"
                );
                for (laid, which) in [(&slope, "the slope"), (&flat, "the flat apart")] {
                    let mut answers: Vec<usize> =
                        laid.edges.iter().map(|&edge| number_of(edge)).collect();
                    answers.sort_unstable();
                    let which = format!(
                        "{what}: {which}, answering to {answers:?}, from {} to {}",
                        laid.run[0], laid.run[1]
                    );
                    one_flat_face_answers_to(body, &answers, &which);
                    face_facing(
                        body,
                        &[answers[0]],
                        DVec3::new(0.0, -1.0, 1.0).normalize(),
                        &which,
                    );
                    corners_are(
                        body,
                        answers[0],
                        &strip(on_section(laid.run[0], 0.0), on_section(laid.run[1], 0.0)),
                        &which,
                    );
                    let [one, other] = laid.between.map(|face| vec![face as u32]);
                    assert_eq!(
                        sides_met_by(body, &answers),
                        sides_by(&[&ends[..], &[one, other]].concat()),
                        "{which}: answering to those numbers and to no other, meeting faces \
                         {:?} and the stair's two ends by one edge each",
                        laid.between
                    );
                }
                corners_are(
                    body,
                    kept,
                    &strip(on_section(kept_run[0], 0.0), on_section(kept_run[1], 0.0)),
                    &format!("{what}: what is left of face {kept}, a strip between the two"),
                );
                corners_are(
                    body,
                    NOSE,
                    &strip(DVec3::ZERO, on_section(on_front, 0.0)),
                    &format!("{what}: what is left of the nose's front"),
                );
                corners_are(
                    body,
                    TOP,
                    &strip(
                        on_section(on_top, 0.0),
                        on_section(DVec2::new(back, height), 0.0),
                    ),
                    &format!("{what}: what is left of the taller block's top"),
                );
                let on_its_floor = |corner: DVec3| {
                    corner.z.abs() < PLACE && (-PLACE..=back + PLACE).contains(&corner.y)
                };
                for (wall, x) in [(LEFT, 0.0), (RIGHT, LENGTH)] {
                    let corners: Vec<DVec3> = section.iter().map(|&at| on_section(at, x)).collect();
                    corners_are_or_on(
                        body,
                        wall,
                        &corners,
                        on_its_floor,
                        &format!("{what}: the stair's end at x = {x}"),
                    );
                }
                sound_drawn_within(
                    body,
                    promised,
                    EXACT * promised,
                    &format!(
                        "{what}: what the two noses give up taken off, what the foot takes in \
                         added"
                    ),
                );
            }
        }
    }
}

/// The [`LEANING_FLIGHT`] with its top rising by `rise` from the riser to
/// the back, raised along X for [`LENGTH`], its faces answering to the
/// numbers the [`LEANING_FLIGHT`]'s do.
fn leaning_flight_rising(rise: f64) -> Body {
    let mut section = LEANING_FLIGHT.to_vec();
    section[2] += DVec2::Y * rise;
    let part = Body::default();
    part.union(&raised_along_x(&part, section, [0.0, LENGTH]))
        .expect("the flight is built")
}

/// The way the flat a chamfer of equal set-backs lays on an edge of a
/// section raised along X runs across it, the edge at `corner` and its two
/// faces running on to `along`: from the set-back on the second face to the
/// one on the first.
fn flat_across(corner: DVec2, along: [DVec2; 2]) -> DVec2 {
    let [first, second] = along.map(|to| (to - corner).normalize());
    (first - second).normalize()
}

/// The angle the kernel reads two flats as one plane within, a millionth of
/// a radian, as `TANGENT` in `wedge/chain.rs` says it.
const KERNEL_TANGENT: f64 = 1e-6;

#[test]
fn a_chamfer_on_a_leaning_flight_s_first_nose_the_foot_between_and_the_second_nose_whose_top_rises_meeting_in_two_creases_or_in_a_crease_and_a_slope_a_hair_off_is_declined_as_unsupported_in_every_order()
 {
    let limit = LEANING_TREAD / 2.0;
    for (rise, which) in [
        (2.0, "two flats meeting in a crease across the riser"),
        (1e-5, "one slope across the riser, a hair off one plane"),
    ] {
        let flight = leaning_flight_rising(rise);
        let [_, _, back_top, second_nose_at, foot_at, first_nose_at] = LEANING_FLIGHT;
        let foot = flat_across(foot_at, [first_nose_at, second_nose_at]);
        let second_nose = flat_across(second_nose_at, [back_top + DVec2::Y * rise, foot_at]);
        let angle = foot.perp_dot(second_nose).abs().asin();
        assert!(
            if rise < 1.0 {
                (angle - 5e-7).abs() < 1e-8 && angle < KERNEL_TANGENT
            } else {
                angle > 1e3 * KERNEL_TANGENT
            },
            "the leaning flight whose top rises {rise}: the foot's flat and the second nose's lie \
             {angle} apart, {which}"
        );
        let edges = [
            edge_between(&flight, LEANING_TREAD_TOP, LEANING_FRONT),
            edge_between(&flight, LEANING_RISER, LEANING_TREAD_TOP),
            edge_between(&flight, LEANING_TOP, LEANING_RISER),
        ];
        for (pair, laid) in [
            (
                [0, 1],
                "the first nose and the foot, two flats meeting in a crease across the tread",
            ),
            ([1, 2], which),
        ] {
            for order in every_order(&pair) {
                let names: Vec<EdgeName> = order.iter().map(|&edge| edges[edge].clone()).collect();
                assert!(
                    flight.chamfer_edges(&names, limit).is_ok(),
                    "the leaning flight whose top rises {rise}: {laid}, alone, named in the order \
                     {order:?}, asked {limit}: laid, so that what declines the three is the foot \
                     meeting both"
                );
            }
        }

        for asked in [limit, 6.0] {
            for order in every_order(&[0, 1, 2]) {
                let names: Vec<EdgeName> = order.iter().map(|&edge| edges[edge].clone()).collect();

                let declined = flight.chamfer_edges(&names, asked).err();

                assert_eq!(
                    declined,
                    Some(Declined::Unsupported),
                    "the leaning flight whose top rises {rise}, its first nose, foot and second \
                     nose named in the order {order:?}, asked {asked}: the foot's flat would meet \
                     the first nose's in a crease across the tread, and the second nose's in \
                     {which}, a chain holding a crease, declined in every order until #926"
                );
            }
        }
    }
}

/// The number the [`stair_with_its_riser_tipped_back`]'s top answers to:
/// the taller block raised along X, its top four.
const TIPPED_TOP: usize = 4;

#[test]
fn a_chamfer_on_the_first_nose_the_foot_and_the_second_nose_of_a_stair_whose_riser_strays_from_the_flight_s_one_plane_by_a_hair_is_declined_as_unsupported_in_every_order()
 {
    for (stair, [top, riser], which) in [
        (
            stair_with_its_riser_turned_by(HAIR_TURN),
            [TOP, RISER],
            format!("the stair whose riser is turned by {HAIR_TURN}"),
        ),
        (
            stair_with_its_riser_turned_by(RISER_TURN),
            [TOP, RISER],
            format!("the stair whose riser is turned by {RISER_TURN}"),
        ),
        (
            stair_with_its_riser_tipped_back(),
            [TIPPED_TOP, TIPPED_RISER],
            format!("the stair whose riser is tipped back by {RISER_TIP} at its top"),
        ),
        (
            stair_of(TREAD, TREAD_HEIGHT + 1e-7),
            [TOP, RISER],
            "the stair whose riser stands 1e-7 higher than its tread is deep".to_string(),
        ),
    ] {
        let edges = [
            edge_between(&stair, TREAD_TOP, NOSE),
            edge_between(&stair, riser, TREAD_TOP),
            edge_between(&stair, top, riser),
        ];
        for asked in [TREAD / 2.0, MORE_THAN_HALF] {
            for order in every_order(&[0, 1, 2]) {
                let names: Vec<EdgeName> = order.iter().map(|&edge| edges[edge].clone()).collect();

                let declined = stair.chamfer_edges(&names, asked).err();

                assert_eq!(
                    declined,
                    Some(Declined::Unsupported),
                    "{which}, its first nose, foot and second nose named in the order {order:?}, \
                     asked {asked}: the three flats a hair off one plane, declined Unsupported as \
                     in every other order, one outcome whatever the order (#822)"
                );
            }
        }
    }
}

/// The matter the booleans leave of the [`flight_of`] three steps once its
/// second step's nose is cut off and its two feet filled in, each set back
/// half a tread's depth: the nose's triangle cut away by a wedge whose
/// floor lies on z = y + 5, run on into the air before the riser and above
/// the top, and each foot's triangle joined on by a prism whose slope lies
/// on that plane too, run on into the tread below it and the riser behind
/// it — the wedge's slope and each prism's meeting by a line, never lying
/// on one another.
fn feet_and_nose_matter_by_booleans(flight: &Body) -> f64 {
    let half = TREAD / 2.0;
    let on_plane = |y: f64| DVec2::new(y, y + TREAD_HEIGHT - half);
    let nose = DVec2::new(TREAD, 2.0 * TREAD_HEIGHT);
    let above = nose.y + 2.0;
    let wedge = raised_along_x(
        flight,
        vec![
            on_plane(nose.x - 2.0),
            on_plane(above - TREAD_HEIGHT + half),
            DVec2::new(nose.x - 2.0, above),
        ],
        [-1.0, LENGTH + 1.0],
    );
    let cut = flight
        .difference(&wedge)
        .expect("the wedge cuts the second nose's triangle away");
    [1.0, 2.0]
        .into_iter()
        .fold(cut, |part, step| {
            let foot = DVec2::new(TREAD, TREAD_HEIGHT) * step;
            let fill = raised_along_x(
                &part,
                vec![
                    foot - DVec2::X * half,
                    foot + DVec2::new(-half, -1.0),
                    foot + DVec2::new(1.0, -1.0),
                    foot + DVec2::new(1.0, half),
                    foot + DVec2::Y * half,
                ],
                [0.0, LENGTH],
            );
            part.union(&fill)
                .expect("the foot's triangle joins the flight")
        })
        .volume()
}

#[test]
fn a_chamfer_on_a_flight_of_three_steps_s_first_foot_second_nose_and_second_foot_takes_half_in_one_slope_from_the_first_tread_to_the_last_riser_in_every_order()
 {
    let steps = 3;
    let flight = flight_of(steps);
    let edges: Vec<EdgeName> = flight_edges(&flight, steps)[1..4].to_vec();
    let half = TREAD / 2.0;
    let [first_tread, last_riser] = [flight_top(steps, 1), flight_front(steps, 3)];
    let gone = [flight_top(steps, 2), flight_front(steps, 2)];
    let [right, left] = flight_ends(steps);
    let promised = LENGTH * (600.0 + half * half / 2.0);
    let by_booleans = feet_and_nose_matter_by_booleans(&flight);
    assert!(
        (promised - 18375.0).abs() < PLACE && (by_booleans - promised).abs() <= EXACT * promised,
        "the flight of three steps, its second nose cut off and its two feet filled in, holds \
         30·(600 + 12.5), 18375: arithmetic gives {promised}, the booleans {by_booleans}"
    );
    let start = flight.faces_end();
    let laid: Vec<usize> = (start..start + 3).collect();
    let [low, high] = [
        DVec3::new(0.0, half, TREAD_HEIGHT),
        DVec3::new(0.0, 2.0 * TREAD, 2.0 * TREAD_HEIGHT + half),
    ];

    for asked in [half, MORE_THAN_HALF] {
        for order in every_order(&[0, 1, 2]) {
            let names: Vec<EdgeName> = order.iter().map(|&edge| edges[edge].clone()).collect();
            let what = format!(
                "the chamfer on the first foot, the second nose and the second foot of a flight \
                 of three steps, named in the order {order:?}, asked {asked}"
            );

            let chamfered = flight
                .chamfer_edges(&names, asked)
                .unwrap_or_else(|declined| panic!("{what}: declined, {declined:?}"));

            reduced(chamfered.asked, chamfered.taken, [asked, half], &what);
            let body = &chamfered.body;
            let mut kept_and_laid = numbers(&flight);
            for face in gone {
                assert!(!body.has_face(face), "{what}: face {face} is gone");
                kept_and_laid.remove(&face);
            }
            kept_and_laid.extend(&laid);
            assert_eq!(
                numbers(body),
                kept_and_laid,
                "{what}: every face of the flight but the second tread and the riser under it, \
                 and the slope answering to {laid:?}"
            );
            assert_eq!(
                body.faces_end(),
                start + 3,
                "{what}: the next step numbers its faces past the three edges' numbers"
            );
            one_flat_face_answers_to(body, &laid, &what);
            for (rank, &one) in laid.iter().enumerate() {
                for &other in &laid[rank + 1..] {
                    assert!(
                        body.edge_names()
                            .iter()
                            .all(|name| !lies_between(name, one, other)),
                        "{what}: no edge stands between {one} and {other}: they are one face"
                    );
                }
            }
            assert_eq!(
                sides_met_by(body, &laid),
                sides_by(&[
                    right.clone(),
                    left.clone(),
                    vec![first_tread as u32],
                    vec![last_riser as u32],
                ]),
                "{what}: the slope meets the first tread, the last riser and the flight's two \
                 ends, by one edge each"
            );
            let which = format!("{what}: the slope on z = y + 5, from {low} to {high}");
            face_facing(
                body,
                &[laid[0]],
                DVec3::new(0.0, -1.0, 1.0).normalize(),
                &which,
            );
            corners_are(body, laid[0], &strip(low, high), &which);
            corners_are(
                body,
                first_tread,
                &strip(DVec3::new(0.0, 0.0, TREAD_HEIGHT), low),
                &format!("{what}: what is left of the first tread, from its nose to the slope"),
            );
            corners_are(
                body,
                last_riser,
                &strip(high, DVec3::new(0.0, 2.0 * TREAD, 3.0 * TREAD_HEIGHT)),
                &format!("{what}: what is left of the last riser, from the slope to its nose"),
            );
            let on_its_floor = |corner: DVec3| {
                corner.z.abs() < PLACE && (-PLACE..=3.0 * TREAD + PLACE).contains(&corner.y)
            };
            for (wall, x) in [(LEFT, 0.0), (RIGHT, LENGTH)] {
                let at = DVec3::X * x;
                corners_are_or_on(
                    body,
                    wall,
                    &[
                        at,
                        at + DVec3::new(0.0, 3.0 * TREAD, 0.0),
                        at + DVec3::new(0.0, 3.0 * TREAD, 3.0 * TREAD_HEIGHT),
                        at + DVec3::new(0.0, 2.0 * TREAD, 3.0 * TREAD_HEIGHT),
                        at + high,
                        at + low,
                        at + DVec3::new(0.0, 0.0, TREAD_HEIGHT),
                    ],
                    on_its_floor,
                    &format!("{what}: the flight's end at x = {x}, cut back to the slope"),
                );
            }
            sound_drawn_within(
                body,
                promised,
                EXACT * promised,
                &format!(
                    "{what}: what the second nose gives up taken off, what the two feet take in \
                     added"
                ),
            );
        }
    }
}

#[test]
fn a_chamfer_on_a_tread_s_nose_and_foot_and_the_front_s_upright_edge_at_one_end_of_the_nose_asked_half_its_depth_or_more_is_declined_as_unsupported()
 {
    let stair = stair();
    let nose = edge_between(&stair, TREAD_TOP, NOSE);
    let foot = edge_between(&stair, RISER, TREAD_TOP);
    let upright = edge_on_sides(&stair, &STAIR_RIGHT, &[NOSE as u32]);
    let half = TREAD / 2.0;
    assert!(
        stair.chamfer(&upright, half).is_ok(),
        "the front's upright edge at the right end of the nose, alone, asked {half}: laid"
    );
    assert!(
        stair
            .chamfer_edges(&[nose.clone(), upright.clone()], half)
            .is_ok(),
        "the nose and the front's upright edge, asked {half}: laid, mitred at the corner they \
         meet at, so that what declines the three is the slope meeting that corner"
    );

    for asked in [half, MORE_THAN_HALF] {
        let declined = stair
            .chamfer_edges(&[nose.clone(), foot.clone(), upright.clone()], asked)
            .err();

        assert_eq!(
            declined,
            Some(Declined::Unsupported),
            "asked {asked}: the nose's and the foot's flats are one slope, and the front's \
             upright edge meets it at the nose's right end, a corner the step blends until #814"
        );
    }
}

/// The matter a probe on `main` measured on the leaning stair chamfered on
/// its nose, its foot and the front's upright edge at one end of the nose,
/// in every order, at the value the three are reduced to.
const LEANING_UPRIGHT_ON_MAIN: f64 = 8645.084326035;

/// The corners of the flat face numbered `face`, each once.
fn corners_of(body: &Body, face: usize, what: &str) -> Vec<DVec3> {
    let plane = body
        .plane_of(face)
        .unwrap_or_else(|| panic!("{what}: the body offers no plane on face {face}"));
    let mut corners: Vec<DVec3> = Vec::new();
    for corner in plane.corners {
        if corners.iter().all(|seen| seen.distance(corner) >= PLACE) {
            corners.push(corner);
        }
    }
    corners
}

#[test]
fn a_chamfer_on_the_nose_and_foot_of_a_tread_whose_front_leans_and_the_front_s_upright_edge_at_one_end_of_the_nose_is_reduced_short_of_the_crease_and_lays_the_body_main_lays_in_every_order()
 {
    let stair = leaning_stair();
    let [nose, foot] = leaning_nose_and_foot(&stair);
    let upright = edge_on_sides(&stair, &[LEANING_START as u32], &[LEANING_FRONT as u32]);
    let edges = [nose, foot, upright];
    let limit = LEANING_TREAD / 2.0;
    let front = LEANING_STAIR[5] - LEANING_STAIR[0];
    let taken = LEANING_TREAD / (1.0 + front.length() / front.y);
    assert!(
        taken < limit - PLACE,
        "the upright's set-back across the stair's start, reaching the tread {} along it for each \
         unit set back, meets the foot's at {taken}, short of the crease's {limit}",
        front.length() / front.y
    );
    let start = stair.faces_end();
    let mut first: Option<Vec<Vec<DVec3>>> = None;

    for asked in [limit, 6.0] {
        for order in every_order(&[0, 1, 2]) {
            let names: Vec<EdgeName> = order.iter().map(|&edge| edges[edge].clone()).collect();
            let what = format!(
                "the chamfer on the leaning stair's nose, foot and the front's upright edge at \
                 the start of the nose, named in the order {order:?}, asked {asked}"
            );
            let chamfered = stair
                .chamfer_edges(&names, asked)
                .unwrap_or_else(|declined| panic!("{what}: declined, {declined:?}"));

            reduced(chamfered.asked, chamfered.taken, [asked, taken], &what);
            let body = &chamfered.body;
            assert!(
                body.has_face(LEANING_TREAD_TOP),
                "{what}: a strip of the tread is kept, the nose's and the foot's flats short of \
                 meeting"
            );
            assert_eq!(
                body.faces_end(),
                start + 3,
                "{what}: the next step numbers its faces past the three flats"
            );
            let flat_of = |edge: usize| {
                start
                    + order
                        .iter()
                        .position(|&named| named == edge)
                        .expect("each edge is named")
            };
            assert!(
                body.edge_names()
                    .iter()
                    .all(|name| !lies_between(name, flat_of(0), flat_of(1))),
                "{what}: the nose's flat and the foot's do not meet, the tread between them"
            );
            let flats: Vec<Vec<DVec3>> = (0..3)
                .map(|edge| corners_of(body, flat_of(edge), &what))
                .collect();
            sound_drawn_within(
                body,
                LEANING_UPRIGHT_ON_MAIN,
                EXACT * LEANING_UPRIGHT_ON_MAIN,
                &format!("{what}: the matter main lays"),
            );
            match &first {
                None => first = Some(flats),
                Some(seen) => {
                    for (edge, (corners, again)) in flats.iter().zip(seen).enumerate() {
                        assert!(
                            corners.len() == again.len()
                                && corners.iter().all(|corner| {
                                    again.iter().any(|other| other.distance(*corner) < PLACE)
                                }),
                            "{what}: edge {edge}'s flat has the corners {corners:?}, where the \
                             first order laid {again:?}"
                        );
                    }
                }
            }
        }
    }
}

/// The section of a flight whose front leans as the [`LEANING_STAIR`]'s
/// does, its riser as high as its tread is deep, [`LEANING_TREAD`]: the
/// taller part's top at [`TREAD_HEIGHT`] and [`LEANING_TREAD`] up. Raised
/// along X, its faces answer to the numbers the [`LEANING_STAIR`]'s do, its
/// top four.
const LEANING_FLIGHT: [DVec2; 6] = [
    DVec2::new(0.0, 0.0),
    DVec2::new(2.0 * TREAD, 0.0),
    DVec2::new(2.0 * TREAD, TREAD_HEIGHT + LEANING_TREAD),
    DVec2::new(TREAD, TREAD_HEIGHT + LEANING_TREAD),
    DVec2::new(TREAD, TREAD_HEIGHT),
    DVec2::new(TREAD - LEANING_TREAD, TREAD_HEIGHT),
];
const LEANING_TOP: usize = 4;

#[test]
fn a_chamfer_on_a_leaning_flight_s_first_nose_the_foot_between_and_the_second_nose_where_its_treads_are_as_deep_as_its_risers_are_high_is_declined_as_unsupported_in_every_order()
 {
    let part = Body::default();
    let flight = part
        .union(&raised_along_x(
            &part,
            LEANING_FLIGHT.to_vec(),
            [0.0, LENGTH],
        ))
        .expect("the flight is built");
    let edges = [
        edge_between(&flight, LEANING_TREAD_TOP, LEANING_FRONT),
        edge_between(&flight, LEANING_RISER, LEANING_TREAD_TOP),
        edge_between(&flight, LEANING_TOP, LEANING_RISER),
    ];
    let limit = LEANING_TREAD / 2.0;
    for (pair, which) in [
        (
            [0, 1],
            "the first nose and the foot, two flats meeting in a crease across the tread",
        ),
        (
            [1, 2],
            "the foot and the second nose, one slope across the riser",
        ),
    ] {
        for order in every_order(&pair) {
            let names: Vec<EdgeName> = order.iter().map(|&edge| edges[edge].clone()).collect();
            assert!(
                flight.chamfer_edges(&names, limit).is_ok(),
                "{which}, alone, named in the order {order:?}, asked {limit}: laid, so that what \
                 declines the three is the foot meeting both"
            );
        }
    }

    for asked in [limit, 6.0] {
        for order in every_order(&[0, 1, 2]) {
            let names: Vec<EdgeName> = order.iter().map(|&edge| edges[edge].clone()).collect();

            let declined = flight.chamfer_edges(&names, asked).err();

            assert_eq!(
                declined,
                Some(Declined::Unsupported),
                "the first nose, the foot and the second nose named in the order {order:?}, \
                 asked {asked}: the foot's flat would meet the first nose's in a crease across \
                 the tread and be one slope with the second nose's across the riser, a crease \
                 meeting another chamfered edge, declined in every order until #926"
            );
        }
    }
}

/// The numbers the cut that makes the widening slot gives the faces of its
/// floor's far edge: its far wall where the slot is 10 wide, twelve, and the
/// step where it widens, eleven.
const NARROW_FAR_WALL: usize = 12;
const WIDENING_STEP: usize = 11;

/// Where along the block the widening slot widens, and where its far wall
/// stands beyond.
const WIDENS_AT: f64 = 20.0;
const WIDENED_WALL: f64 = 25.0;

/// The slotted block's slot, 10 wide from the left wall to x = 20 and 15
/// wide from there to the right wall, cut by one outline: its near wall, at
/// y = 10, runs the block's whole length, and its floor's near edge ends on
/// the block's left wall and its right; its far wall, at y = 20, stops at the
/// step where the slot widens, and its floor's far edge ends on the left wall
/// and on that step.
fn widening_slot() -> Body {
    let solid = block([LENGTH, SLOTTED_BREADTH, HEIGHT]);
    let [near, far] = SLOT_WALLS;
    let tool = solid
        .tool_raised(
            &profile(Contour::straight(vec![
                DVec2::new(-1.0, near),
                DVec2::new(LENGTH + 1.0, near),
                DVec2::new(LENGTH + 1.0, WIDENED_WALL),
                DVec2::new(WIDENS_AT, WIDENED_WALL),
                DVec2::new(WIDENS_AT, far),
                DVec2::new(-1.0, far),
            ])),
            Frame {
                origin: DVec3::Z * SLOT_FLOOR_HEIGHT,
                ..ground()
            },
            DVec3::Z * HEIGHT,
        )
        .expect("the tool raises");
    solid.difference(&tool).expect("the slot is cut")
}

#[test]
fn a_fillet_on_the_two_floor_edges_of_a_slot_that_widens_past_the_end_of_one_is_declined_as_unsupported()
 {
    let slot = widening_slot();
    let names = [
        edge_between(&slot, SLOT_FLOOR, SLOT_NEAR_WALL),
        edge_between(&slot, SLOT_FLOOR, NARROW_FAR_WALL),
    ];
    let far_ends = &names[1].ends;
    assert!(
        far_ends
            .iter()
            .any(|end| end.contains(&vec![WIDENING_STEP as u32])),
        "the far floor edge ends on the step where the slot widens: {far_ends:?}"
    );
    let half = (SLOT_WALLS[1] - SLOT_WALLS[0]) / 2.0;

    for asked in [half, MORE_THAN_HALF] {
        for name in &names {
            let alone = slot
                .fillet(name, asked)
                .unwrap_or_else(|declined| panic!("one edge alone, asked {asked}: {declined:?}"));
            taken_as_asked(alone.asked, alone.taken, asked, "one floor edge alone");
        }

        let declined = slot.fillet_edges(&names, asked).err();

        assert_eq!(
            declined,
            Some(Declined::Unsupported),
            "asked {asked}: the two fills' set-backs would meet across the floor, but the edges \
             end on different faces, and no one hollow runs from end to end"
        );
    }
}

#[test]
fn a_chamfer_on_the_two_floor_edges_of_a_slot_that_widens_past_the_end_of_one_takes_5_a_valley_along_its_narrow_part()
 {
    let slot = widening_slot();
    let names = [
        edge_between(&slot, SLOT_FLOOR, SLOT_NEAR_WALL),
        edge_between(&slot, SLOT_FLOOR, NARROW_FAR_WALL),
    ];
    let [near, far] = SLOT_WALLS;
    let half = (far - near) / 2.0;
    let (middle, up) = (near + half, SLOT_FLOOR_HEIGHT + half);
    let laid = [slot.faces_end(), slot.faces_end() + 1];
    let at = |x: f64, y: f64, z: f64| DVec3::new(x, y, z);

    for asked in [half, MORE_THAN_HALF] {
        let chamfered = slot
            .chamfer_edges(&names, asked)
            .unwrap_or_else(|declined| panic!("asked {asked}: declined, {declined:?}"));

        let what = format!("the chamfer on both floor edges of the widening slot, asked {asked}");
        reduced(chamfered.asked, chamfered.taken, [asked, half], &what);
        let body = &chamfered.body;
        let mut kept_and_laid = numbers(&slot);
        kept_and_laid.extend(laid);
        assert_eq!(
            numbers(body),
            kept_and_laid,
            "{what}: every face of the slotted block, and a flat for each edge"
        );
        let floor = SLOT_FLOOR_HEIGHT;
        for (number, facing, corners, which) in [
            (
                laid[0],
                DVec3::new(0.0, 1.0, 1.0),
                vec![
                    at(0.0, near, up),
                    at(LENGTH, near, up),
                    at(LENGTH, middle, floor),
                    at(WIDENS_AT, middle, floor),
                    at(0.0, middle, floor),
                ],
                "the near flat, the block's whole length, down to the valley and then to the \
                 floor beyond the widening",
            ),
            (
                laid[1],
                DVec3::new(0.0, -1.0, 1.0),
                vec![
                    at(0.0, far, up),
                    at(WIDENS_AT, far, up),
                    at(WIDENS_AT, middle, floor),
                    at(0.0, middle, floor),
                ],
                "the far flat, from the left wall to the step where the slot widens",
            ),
        ] {
            let which = format!("{what}: face {number}, {which}");
            face_facing(body, &[number], facing.normalize(), &which);
            corners_are(body, number, &corners, &which);
        }
        edge_between(body, laid[0], laid[1]);
        assert_eq!(
            sides_met(body, laid[0]),
            sides(&[RIGHT, LEFT, SLOT_FLOOR, SLOT_NEAR_WALL, laid[1]]),
            "{what}: the near flat meets its wall, the block's two walls, the far flat along the \
             valley and the floor beyond, by one edge each"
        );
        assert_eq!(
            sides_met(body, laid[1]),
            sides(&[LEFT, WIDENING_STEP, NARROW_FAR_WALL, laid[0]]),
            "{what}: the far flat meets its wall, the left wall, the step at its end and the near \
             flat along the valley, by one edge each"
        );
        corners_are(
            body,
            SLOT_FLOOR,
            &[
                at(WIDENS_AT, middle, floor),
                at(LENGTH, middle, floor),
                at(LENGTH, WIDENED_WALL, floor),
                at(WIDENS_AT, WIDENED_WALL, floor),
            ],
            &format!("{what}: what is left of the floor, beyond the widening"),
        );
        corners_are(
            body,
            WIDENING_STEP,
            &[
                at(WIDENS_AT, middle, floor),
                at(WIDENS_AT, WIDENED_WALL, floor),
                at(WIDENS_AT, WIDENED_WALL, HEIGHT),
                at(WIDENS_AT, far, HEIGHT),
                at(WIDENS_AT, far, up),
            ],
            &format!("{what}: the step, holding the far flat's end flush"),
        );
        let [near_wall, _] = slot_walls_from(half);
        for (wall, corners, which) in [
            (
                SLOT_NEAR_WALL,
                near_wall.to_vec(),
                "the near wall, the block's whole length",
            ),
            (
                NARROW_FAR_WALL,
                vec![
                    at(0.0, far, up),
                    at(WIDENS_AT, far, up),
                    at(WIDENS_AT, far, HEIGHT),
                    at(0.0, far, HEIGHT),
                ],
                "the far wall, up to the widening",
            ),
        ] {
            corners_are(
                body,
                wall,
                &corners,
                &format!("{what}: what is left of {which}, above its flat"),
            );
        }
        for (wall, x, beyond) in [(LEFT, 0.0, far), (RIGHT, LENGTH, WIDENED_WALL)] {
            let mut corners = vec![
                at(x, 0.0, 0.0),
                at(x, SLOTTED_BREADTH, 0.0),
                at(x, SLOTTED_BREADTH, HEIGHT),
                at(x, beyond, HEIGHT),
                at(x, near, up),
                at(x, middle, floor),
                at(x, near, HEIGHT),
                at(x, 0.0, HEIGHT),
            ];
            corners.push(if x == 0.0 {
                at(x, far, up)
            } else {
                at(x, WIDENED_WALL, floor)
            });
            corners_are(
                body,
                wall,
                &corners,
                &format!("{what}: the wall at x = {x}, the slot's section on it"),
            );
        }
        let widening = (HEIGHT - SLOT_FLOOR_HEIGHT)
            * (WIDENS_AT * (far - near) + (LENGTH - WIDENS_AT) * (WIDENED_WALL - near));
        let promised =
            LENGTH * SLOTTED_BREADTH * HEIGHT - widening + half * half / 2.0 * (LENGTH + WIDENS_AT);
        sound_drawn_within(body, promised, EXACT * promised, &what);
    }
}

#[test]
fn a_fillet_and_a_chamfer_on_a_list_of_no_edge_are_declined_as_unsupported() {
    let block = block([LENGTH, BREADTH, HEIGHT]);

    let filleted = block.fillet_edges(&[], 3.0).err();
    let chamfered = block.chamfer_edges(&[], 3.0).err();

    assert_eq!(filleted, Some(Declined::Unsupported), "the fillet");
    assert_eq!(chamfered, Some(Declined::Unsupported), "the chamfer");
}

/// The side of the hexagonal prism, and its height.
const HEXAGON_SIDE: f64 = 10.0;
const HEXAGON_HEIGHT: f64 = 5.0;

/// The numbers the hexagonal prism's top and two of its walls answer to: its
/// front two, along the bottom of the hexagon, and four, two walls round,
/// which meets it at 120 degrees across the top. The wall between them,
/// three, is where both their rounds run on past their ends.
const HEXAGON_FRONT: usize = 2;
const HEXAGON_ACROSS: usize = 4;

/// A regular hexagon of side [`HEXAGON_SIDE`] about the origin, its front the
/// side along the bottom, raised [`HEXAGON_HEIGHT`]: its walls numbered from
/// the front round, two to seven.
fn hexagonal_prism() -> Body {
    let half = HEXAGON_SIDE / 2.0 * 3.0f64.sqrt();
    let side = HEXAGON_SIDE;
    raised_with(
        &Body::default(),
        Contour::straight(vec![
            DVec2::new(-side / 2.0, -half),
            DVec2::new(side / 2.0, -half),
            DVec2::new(side, 0.0),
            DVec2::new(side / 2.0, half),
            DVec2::new(-side / 2.0, half),
            DVec2::new(-side, 0.0),
        ]),
        HEXAGON_HEIGHT,
    )
}

/// The round a fillet of `radius` lays on the hexagon's top front edge,
/// running on past both its ends to the walls beside it, turned about the
/// vertical by `turn`: a third of a turn gives the round on wall four's top
/// edge. The second is the same round over the edge alone, between its ends.
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
    let half = HEXAGON_SIDE / 2.0 * 3.0f64.sqrt();
    let turned = |way: DVec3| {
        DVec3::new(
            way.x * turn.cos() - way.y * turn.sin(),
            way.x * turn.sin() + way.y * turn.cos(),
            way.z,
        )
    };
    Round {
        start: turned(DVec3::new(-past, radius - half, HEXAGON_HEIGHT - radius)),
        along: turned(DVec3::X),
        length: 2.0 * past,
        radius,
        facing: [DVec3::Z, turned(-DVec3::Y)],
    }
}

#[test]
fn a_fillet_and_a_chamfer_on_two_alternate_top_edges_of_a_hexagonal_prism_lay_each_as_alone_where_they_do_not_meet()
 {
    let prism = hexagonal_prism();
    for value in [2.0, 4.2] {
        let names = [
            edge_between(&prism, TOP, HEXAGON_FRONT),
            edge_between(&prism, TOP, HEXAGON_ACROSS),
        ];
        let laid = [prism.faces_end(), prism.faces_end() + 1];
        let rounds = [0.0, 2.0 * PI / 3.0].map(|turn| hexagon_round(value, turn));
        let alone = "one top edge alone";
        let filleted_alone: Vec<Body> = names
            .iter()
            .zip(&rounds)
            .map(|(name, [running_on, over_the_edge])| {
                let single = prism.fillet(name, value).expect("each edge's fillet alone");
                is_the_round_running_on(
                    &single.body,
                    made_one(&prism, &single.body, alone),
                    [running_on, over_the_edge],
                    &format!("{alone}: the round its fillet lays, running on past both its ends"),
                );
                single.body
            })
            .collect();
        let chamfered_alone: Vec<Body> = names
            .iter()
            .map(|name| {
                prism
                    .chamfer(name, value)
                    .expect("each edge's chamfer alone")
                    .body
            })
            .collect();

        let filleted = prism
            .fillet_edges(&names, value)
            .unwrap_or_else(|declined| panic!("the fillet: declined, {declined:?}"));
        let chamfered = prism
            .chamfer_edges(&names, value)
            .unwrap_or_else(|declined| panic!("the chamfer: declined, {declined:?}"));

        let what = format!(
            "the fillet on the hexagon's top front edge and the top edge two walls round, asked \
         {value}"
        );
        taken_as_asked(filleted.asked, filleted.taken, value, &what);
        for ([running_on, over_the_edge], number) in rounds.iter().zip(laid) {
            is_the_round_running_on(
                &filleted.body,
                number,
                [running_on, over_the_edge],
                &format!(
                    "{what}: face {number}, the round its edge alone lays, with the other laid"
                ),
            );
        }
        takes_what_each_alone_takes(&prism, &filleted.body, &filleted_alone, &what);
        let area: f64 = rounds.iter().map(|[running_on, _]| running_on.area()).sum();
        sound_drawn_within(
            &filleted.body,
            filleted.body.volume(),
            DRAWN * reach_of(&prism) * area,
            &what,
        );

        let what = format!(
            "the chamfer on the hexagon's top front edge and the top edge two walls round, asked \
         {value}"
        );
        taken_as_asked(chamfered.asked, chamfered.taken, value, &what);
        for (single, number) in chamfered_alone.iter().zip(laid) {
            let flat = single
                .plane_of(made_one(&prism, single, alone))
                .expect("a chamfer is flat");
            face_facing(
                &chamfered.body,
                &[number],
                flat.normal,
                &format!("{what}: face {number}, facing as its edge's flat alone"),
            );
            corners_are(
                &chamfered.body,
                number,
                &flat.corners,
                &format!("{what}: face {number}, the corners of its edge's flat alone"),
            );
        }
        takes_what_each_alone_takes(&prism, &chamfered.body, &chamfered_alone, &what);
        let promised = chamfered.body.volume();
        sound_drawn_within(&chamfered.body, promised, EXACT * promised, &what);
    }
}

#[test]
fn a_fillet_on_two_edges_whose_rounds_would_meet_on_the_wall_one_ends_on_is_laid_as_asked_short_of_it()
 {
    let block = block([LENGTH, BREADTH, HEIGHT]);
    let names = [
        edge_between(&block, TOP, FRONT),
        edge_between(&block, RIGHT, BACK),
    ];
    let short = 3.0;

    let filleted = block
        .fillet_edges(&names, short)
        .unwrap_or_else(|declined| panic!("asked {short}: declined, {declined:?}"));

    let what =
        format!("the fillet on the top's front edge and the right wall's back edge, asked {short}");
    taken_as_asked(filleted.asked, filleted.taken, short, &what);
    let body = &filleted.body;
    let [top_front, ..] = long_edges();
    let along_the_top = top_front.round(short);
    let upright = Round {
        start: DVec3::new(LENGTH - short, BREADTH - short, 0.0),
        along: DVec3::Z,
        length: HEIGHT,
        radius: short,
        facing: [DVec3::X, DVec3::Y],
    };
    let laid = block.faces_end();
    is_the_round(
        body,
        laid,
        &along_the_top,
        &format!("{what}: face {laid}, the top's front edge rounded off"),
    );
    is_the_round(
        body,
        laid + 1,
        &upright,
        &format!(
            "{what}: face {}, the right wall's back edge rounded off",
            laid + 1
        ),
    );
    let quarter = short * short * (1.0 - PI / 4.0);
    sound_drawn_within(
        body,
        LENGTH * BREADTH * HEIGHT - quarter * (LENGTH + HEIGHT),
        DRAWN * reach_of(&block) * (along_the_top.area() + upright.area()),
        &what,
    );
}

#[test]
fn a_chamfer_on_the_four_long_edges_of_a_square_bar_asked_half_its_side_or_more_takes_half_a_diamond()
 {
    let bar = block([LENGTH, BAR, BAR]);
    let edges = long_edges_of(BAR, BAR);
    let names: Vec<EdgeName> = edges.iter().map(|edge| edge.name(&bar)).collect();
    let half = BAR / 2.0;
    let laid: Vec<usize> = (bar.faces_end()..bar.faces_end() + edges.len()).collect();

    for asked in [half, MORE_THAN_HALF] {
        let chamfered = bar
            .chamfer_edges(&names, asked)
            .unwrap_or_else(|declined| panic!("asked {asked}: declined, {declined:?}"));

        let what =
            format!("the chamfer on the four long edges of a bar {BAR} square, asked {asked}");
        reduced(chamfered.asked, chamfered.taken, [asked, half], &what);
        let body = &chamfered.body;
        let mut ends_and_laid: BTreeSet<usize> = [RIGHT, LEFT].into();
        ends_and_laid.extend(laid.iter().copied());
        assert_eq!(
            numbers(body),
            ends_and_laid,
            "{what}: the four long faces gone, the bar's two ends and the four flats left"
        );
        for (edge, &number) in edges.iter().zip(&laid) {
            let (corners, facing) = edge.flat(half);
            let which = format!(
                "{what}: face {number}, the flat between faces {:?}",
                edge.faces
            );
            face_facing(body, &[number], facing, &which);
            corners_are(body, number, &corners, &which);
        }
        let [top_front, floor_back, top_back, floor_front] = [laid[0], laid[1], laid[2], laid[3]];
        for (one, other, where_) in [
            (top_front, top_back, "where the top was"),
            (top_front, floor_front, "where the front was"),
            (floor_back, top_back, "where the back was"),
            (floor_back, floor_front, "where the floor was"),
        ] {
            assert_eq!(
                body.edge_names()
                    .iter()
                    .filter(|name| lies_between(name, one, other))
                    .count(),
                1,
                "{what}: faces {one} and {other} meet along one ridge, {where_}"
            );
        }
        for (wall, x) in [(LEFT, 0.0), (RIGHT, LENGTH)] {
            corners_are(
                body,
                wall,
                &[
                    DVec3::new(x, half, BAR),
                    DVec3::new(x, BAR, half),
                    DVec3::new(x, half, 0.0),
                    DVec3::new(x, 0.0, half),
                ],
                &format!("{what}: the end at x = {x}, a diamond"),
            );
        }
        let promised = LENGTH * BAR * BAR / 2.0;
        sound_drawn_within(body, promised, EXACT * promised, &what);
    }
}

/// How short a strip left between two set-backs on a face may be and still
/// read as none to the kernel, on the block [`LENGTH`] long: four of its
/// hairs of twenty tolerances each, a tolerance a billionth of the reach.
const MEETING_HAIR: f64 = 4.0 * ONE_HAIR;

/// One of the kernel's hairs on the block [`LENGTH`] long: twenty
/// tolerances, a tolerance a billionth of the reach.
const ONE_HAIR: f64 = 20.0 * 1e-9 * LENGTH;

#[test]
fn a_fillet_on_the_two_edges_either_side_of_a_face_10_wide_asked_a_hair_under_5_lays_one_full_round()
 {
    let block = block([LENGTH, BREADTH, HEIGHT]);
    let [top_front, _, top_back, _] = long_edges();
    let names = [top_front.name(&block), top_back.name(&block)];
    let half = BREADTH / 2.0;
    let laid = [block.faces_end(), block.faces_end() + 1];

    for (under, past_one_hair) in [(1e-6, true), (1e-7, false)] {
        let asked = half - under;
        let strip = BREADTH - 2.0 * asked;
        assert!(
            strip < MEETING_HAIR && (strip > ONE_HAIR) == past_one_hair,
            "asked {asked}: the strip the two rounds leave, {strip}, is shorter than the kernel \
             keeps, {MEETING_HAIR}, and longer than one of its hairs, {ONE_HAIR}, or not, as \
             {past_one_hair} says"
        );

        let filleted = block
            .fillet_edges(&names, asked)
            .unwrap_or_else(|declined| panic!("asked {asked}: declined, {declined:?}"));

        let what = format!("the fillet on both edges of a top {BREADTH} wide, asked {asked}");
        assert!(
            (filleted.asked - asked).abs() < PLACE
                && (filleted.taken - asked).abs() <= MEETING_HAIR,
            "{what}: asked {} and took {}, where {asked} is a hair from what both give",
            filleted.asked,
            filleted.taken
        );
        let body = &filleted.body;
        assert!(!body.has_face(TOP), "{what}: the top is gone");
        one_curved_face_answers_to(body, &laid, &what);
        let halves = [[-DVec3::Y, DVec3::Z], [DVec3::Z, DVec3::Y]].map(|facing| Round {
            start: DVec3::new(0.0, half, HEIGHT - half),
            along: DVec3::X,
            length: LENGTH,
            radius: half,
            facing,
        });
        is_made_of(
            body,
            laid[0],
            &[&halves[0], &halves[1]],
            &format!(
                "{what}: the full round, the half-cylinder of {half} the two set-backs meet at"
            ),
        );
        assert_eq!(
            sides_met_by(body, &laid),
            sides(&[FRONT, RIGHT, BACK, LEFT]),
            "{what}: the full round meets the front and the back along its sides, and the right \
             and the left walls at its ends, by one edge each"
        );
        sound_drawn_within(
            body,
            LENGTH * BREADTH * HEIGHT - LENGTH * (BREADTH * half - PI * half * half / 2.0),
            DRAWN * reach_of(&block) * PI * half * LENGTH,
            &what,
        );
    }
}

/// A full round, or a round bottom, a step on two edges facing each other
/// across a face 10 wide lays at half that width: the part, the faces of each
/// edge in the order the step names them, the face that goes, the axis the
/// round turns about and how long it runs, the ways the round faces where it
/// meets the first edge's other face, where the face was and the second
/// edge's other face, the faces it meets, and the matter the body holds.
struct FullRound {
    what: &'static str,
    part: Body,
    edges: [[usize; 2]; 2],
    gone: usize,
    axis: DVec3,
    along: DVec3,
    length: f64,
    facing: [DVec3; 3],
    meets: Vec<usize>,
    volume: f64,
    hollow: bool,
}

/// What a round of half [`BREADTH`] takes off a quarter of a block's section
/// on either side of the face 10 wide, or fills in.
const HALF_ROUND_SECTION: f64 = BREADTH * BREADTH / 2.0 - PI * BREADTH * BREADTH / 8.0;

/// The numbers the vertical notch's walls answer to: its right side nine, its
/// back ten and its left side eleven, past the block's six.
const NOTCH_RIGHT: usize = 9;
const NOTCH_BACK: usize = 10;
const NOTCH_LEFT: usize = 11;

/// A block 30 long, 20 across and 20 high, notched from its front through its
/// whole height between x = 10 and 20, 12 deep: the notch's back, 10 wide,
/// meets its two sides inwards along two upright edges facing each other.
fn notched_through() -> Body {
    let part = block([LENGTH, 2.0 * BREADTH, HEIGHT]);
    cut_from(
        &part,
        [DVec2::new(10.0, -1.0), DVec2::new(20.0, 12.0)],
        -1.0,
        HEIGHT + 2.0,
    )
}

/// The full rounds and round bottoms whose shared face stands second among
/// an edge's two, for either edge or for both, as well as first: the list in
/// either order, along X, along Y and upright, on a top, a floor, a front and
/// a side wall, and the round bottom of a slot and of a notch.
fn full_rounds() -> Vec<FullRound> {
    let half = BREADTH / 2.0;
    let block_volume = LENGTH * BREADTH * HEIGHT;
    let along_x = LENGTH * HALF_ROUND_SECTION;
    let mut rounds = Vec::new();
    let both_ways = |round: FullRound| {
        let [first, second] = round.edges;
        let [one, middle, other] = round.facing;
        let reversed = FullRound {
            edges: [second, first],
            facing: [other, middle, one],
            part: round.part.clone(),
            meets: round.meets.clone(),
            ..round
        };
        [round, reversed]
    };
    rounds.extend(both_ways(FullRound {
        what: "the top's front and back edges",
        part: block([LENGTH, BREADTH, HEIGHT]),
        edges: [[TOP, FRONT], [TOP, BACK]],
        gone: TOP,
        axis: DVec3::new(0.0, half, HEIGHT - half),
        along: DVec3::X,
        length: LENGTH,
        facing: [-DVec3::Y, DVec3::Z, DVec3::Y],
        meets: vec![FRONT, RIGHT, BACK, LEFT],
        volume: block_volume - along_x,
        hollow: false,
    }));
    rounds.extend(both_ways(FullRound {
        what: "the floor's front and back edges",
        part: block([LENGTH, BREADTH, HEIGHT]),
        edges: [[FLOOR, FRONT], [FLOOR, BACK]],
        gone: FLOOR,
        axis: DVec3::new(0.0, half, half),
        along: DVec3::X,
        length: LENGTH,
        facing: [-DVec3::Y, -DVec3::Z, DVec3::Y],
        meets: vec![FRONT, RIGHT, BACK, LEFT],
        volume: block_volume - along_x,
        hollow: false,
    }));
    rounds.extend(both_ways(FullRound {
        what: "the front's top and floor edges, on a block 10 high",
        part: block([LENGTH, 2.0 * BREADTH, BREADTH]),
        edges: [[TOP, FRONT], [FLOOR, FRONT]],
        gone: FRONT,
        axis: DVec3::new(0.0, half, half),
        along: DVec3::X,
        length: LENGTH,
        facing: [DVec3::Z, -DVec3::Y, -DVec3::Z],
        meets: vec![FLOOR, TOP, RIGHT, LEFT],
        volume: LENGTH * 2.0 * BREADTH * BREADTH - along_x,
        hollow: false,
    }));
    rounds.extend(both_ways(FullRound {
        what: "the right wall's two upright edges",
        part: block([LENGTH, BREADTH, HEIGHT]),
        edges: [[FRONT, RIGHT], [RIGHT, BACK]],
        gone: RIGHT,
        axis: DVec3::new(LENGTH - half, half, 0.0),
        along: DVec3::Z,
        length: HEIGHT,
        facing: [-DVec3::Y, DVec3::X, DVec3::Y],
        meets: vec![FLOOR, TOP, FRONT, BACK],
        volume: block_volume - HEIGHT * HALF_ROUND_SECTION,
        hollow: false,
    }));
    rounds.extend(both_ways(FullRound {
        what: "the top's left and right edges, running along Y",
        part: block([BREADTH, LENGTH, HEIGHT]),
        edges: [[TOP, LEFT], [TOP, RIGHT]],
        gone: TOP,
        axis: DVec3::new(half, 0.0, HEIGHT - half),
        along: DVec3::Y,
        length: LENGTH,
        facing: [-DVec3::X, DVec3::Z, DVec3::X],
        meets: vec![FRONT, RIGHT, BACK, LEFT],
        volume: block_volume - along_x,
        hollow: false,
    }));
    rounds.extend(both_ways(FullRound {
        what: "the slot's two floor edges",
        part: slotted(),
        edges: [[SLOT_FLOOR, SLOT_NEAR_WALL], [SLOT_FLOOR, SLOT_FAR_WALL]],
        gone: SLOT_FLOOR,
        axis: DVec3::new(0.0, SLOT_WALLS[0] + half, SLOT_FLOOR_HEIGHT + half),
        along: DVec3::X,
        length: LENGTH,
        facing: [DVec3::Y, DVec3::Z, -DVec3::Y],
        meets: vec![RIGHT, LEFT, SLOT_NEAR_WALL, SLOT_FAR_WALL],
        volume: SLOTTED_VOLUME + along_x,
        hollow: true,
    }));
    rounds.extend(both_ways(FullRound {
        what: "the notch's two upright inner edges",
        part: notched_through(),
        edges: [[NOTCH_BACK, NOTCH_LEFT], [NOTCH_RIGHT, NOTCH_BACK]],
        gone: NOTCH_BACK,
        axis: DVec3::new(10.0 + half, 12.0 - half, 0.0),
        along: DVec3::Z,
        length: HEIGHT,
        facing: [DVec3::X, -DVec3::Y, -DVec3::X],
        meets: vec![FLOOR, TOP, NOTCH_RIGHT, NOTCH_LEFT],
        volume: LENGTH * 2.0 * BREADTH * HEIGHT - 10.0 * 12.0 * HEIGHT
            + HEIGHT * HALF_ROUND_SECTION,
        hollow: true,
    }));
    rounds
}

#[test]
fn a_fillet_lays_one_full_round_or_round_bottom_whichever_of_its_two_faces_each_edge_shares_and_in_either_order()
 {
    let half = BREADTH / 2.0;

    for round in full_rounds() {
        let part = &round.part;
        let names = round
            .edges
            .map(|[one, other]| edge_between(part, one, other));
        let laid = [part.faces_end(), part.faces_end() + 1];
        let order = round.edges;

        let filleted = part
            .fillet_edges(&names, MORE_THAN_HALF)
            .unwrap_or_else(|declined| panic!("{}, {order:?}: declined, {declined:?}", round.what));

        let what = format!(
            "the fillet on {}, named as {order:?}, asked {MORE_THAN_HALF}",
            round.what
        );
        reduced(
            filleted.asked,
            filleted.taken,
            [MORE_THAN_HALF, half],
            &what,
        );
        let body = &filleted.body;
        assert!(
            !body.has_face(round.gone),
            "{what}: face {}, which both edges share, is gone",
            round.gone
        );
        let mut kept_and_laid = numbers(part);
        kept_and_laid.remove(&round.gone);
        kept_and_laid.extend(laid);
        assert_eq!(
            numbers(body),
            kept_and_laid,
            "{what}: every face but the one gone, and the round answering to {laid:?}"
        );
        one_curved_face_answers_to(body, &laid, &what);
        assert!(
            body.edge_names()
                .iter()
                .all(|name| !lies_between(name, laid[0], laid[1])),
            "{what}: no edge stands between {} and {}: they are one face",
            laid[0],
            laid[1]
        );
        let [one, middle, other] = round.facing;
        let which = format!("{what}: one round of {half} about one axis");
        if round.hollow {
            let parts = [[one, middle], [middle, other]].map(|facing| Hollow {
                start: round.axis,
                along: round.along,
                length: round.length,
                radius: half,
                facing,
            });
            is_hollow_made_of(body, laid[0], &[&parts[0], &parts[1]], &which);
        } else {
            let parts = [[one, middle], [middle, other]].map(|facing| Round {
                start: round.axis,
                along: round.along,
                length: round.length,
                radius: half,
                facing,
            });
            is_made_of(body, laid[0], &[&parts[0], &parts[1]], &which);
        }
        assert_eq!(
            sides_met_by(body, &laid),
            sides(&round.meets),
            "{what}: the round meets the two edges' other faces along its sides, and the faces \
             at its ends, by one edge each"
        );
        sound_drawn_within(
            body,
            round.volume,
            DRAWN * reach_of(part) * PI * half * round.length,
            &what,
        );
    }
}

#[test]
fn a_fillet_on_a_meeting_pair_whose_earlier_edge_is_not_named_first_numbers_the_edge_before_it_first_and_the_full_round_after()
 {
    let block = block([LENGTH, BREADTH, HEIGHT]);
    let [top_front, floor_back, top_back, _] = long_edges();
    let names = [
        floor_back.name(&block),
        top_front.name(&block),
        top_back.name(&block),
    ];
    let half = BREADTH / 2.0;

    let filleted = block
        .fillet_edges(&names, MORE_THAN_HALF)
        .unwrap_or_else(|declined| panic!("declined, {declined:?}"));

    let what = format!(
        "the fillet on the floor's back edge, the top's front edge and the top's back edge, asked \
         {MORE_THAN_HALF}"
    );
    reduced(
        filleted.asked,
        filleted.taken,
        [MORE_THAN_HALF, half],
        &what,
    );
    let body = &filleted.body;
    let start = block.faces_end();
    let (before, pair) = (start, [start + 1, start + 2]);
    let mut kept_and_laid = numbers(&block);
    kept_and_laid.remove(&TOP);
    kept_and_laid.extend(start..start + 3);
    assert_eq!(
        numbers(body),
        kept_and_laid,
        "{what}: every face of the block but its top, the floor's back round answering to \
         {before} and the full round to {pair:?}"
    );
    is_the_round(
        body,
        before,
        &floor_back.round(half),
        &format!("{what}: face {before}, the floor's back edge rounded off at {half}"),
    );
    one_curved_face_answers_to(body, &pair, &what);
    let halves = [[-DVec3::Y, DVec3::Z], [DVec3::Z, DVec3::Y]].map(|facing| Round {
        start: DVec3::new(0.0, half, HEIGHT - half),
        along: DVec3::X,
        length: LENGTH,
        radius: half,
        facing,
    });
    is_made_of(
        body,
        pair[0],
        &[&halves[0], &halves[1]],
        &format!("{what}: the full round over where the top was"),
    );
    assert_eq!(
        sides_met_by(body, &pair),
        sides(&[FRONT, RIGHT, BACK, LEFT]),
        "{what}: the full round meets the front, the back and the two walls, by one edge each"
    );
    sound_drawn_within(
        body,
        LENGTH * BREADTH * HEIGHT
            - LENGTH * HALF_ROUND_SECTION
            - LENGTH * half * half * (1.0 - PI / 4.0),
        DRAWN * reach_of(&block) * (PI * half * LENGTH + floor_back.round(half).area()),
        &what,
    );
}
