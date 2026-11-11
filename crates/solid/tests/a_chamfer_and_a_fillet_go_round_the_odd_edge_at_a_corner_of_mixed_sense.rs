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
//! the upright's round. Slice 2, the top of the L's inner corner, and slice
//! 3, every edge of a standing block, are below.
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
//! - Not in the issue's wording, a guard held until slice 3: a foot corner
//!   named with the top corner of its upright — both feet, the upright and
//!   the standing block's two top edges at its top — was declined
//!   `Unsupported`, chamfered or filleted, whatever the order, so that no
//!   shape shipped before a test held it. Slice 3 turns it into the body
//!   test below. What the guard still declines is a post joining a floor
//!   plate and a ceiling plate, its upright the odd edge at both its ends,
//!   which #847 builds —
//!   `a_fillet_and_a_chamfer_on_a_post_s_upright_and_the_feet_at_both_its_ends_are_declined_as_unsupported_until_847_builds_them`
//! - Not in the issue's wording: where the odd edges, laid first, close a
//!   corner of their own, each face is still numbered as the fourth
//!   criterion reads it. A triangular pyramid standing on a plate, each
//!   slanted edge the odd one at its foot corner and the three meeting at
//!   the apex, chamfered on its six edges: each edge's flat at its rank, then
//!   the apex's triangle and the three fills along the slanted flats' feet,
//!   by the lowest ranks of their edges, in every order; and the body the two
//!   steps build, the slanted edges then the six feet —
//!   `a_chamfer_on_a_triangular_pyramid_s_slanted_edges_and_feet_numbers_each_face_by_its_rank_the_apex_s_triangle_among_the_patches`.
//!   The same where the odd edges close two corners, two pyramids standing on
//!   one plate, set where no foot's line reaches the other's feet —
//!   `a_chamfer_on_two_triangular_pyramids_slanted_edges_and_feet_numbers_both_apexes_triangles_among_the_patches_by_their_ranks`;
//!   and where the edges laid after the odd ones close corners of their own:
//!   a foot corner named with the convex corner at the top of another
//!   upright and one of the lower block's top, chamfered or filleted, each
//!   patch at its own corner, the volume the three corners' added —
//!   `a_chamfer_and_a_fillet_on_a_foot_corner_and_two_convex_corners_number_every_patch_after_the_edges_faces_by_their_ranks`
//! - Not in the issue's wording: each edge laid after the odd ones is found
//!   again on what they left as the straight edge between its own sides on
//!   its own line, never another edge lying on that line — a second block's
//!   front foot on the line of the first's, its front right foot corner the
//!   body as alone —
//!   `a_chamfer_and_a_fillet_on_a_foot_corner_follow_each_foot_and_not_another_block_s_foot_on_the_same_line`
//! - Found on the way, not in the issue's wording: a foot corner named with
//!   edges of the lower block's outline beyond its feet — its top right
//!   edge, its top front edge, or its front right top corner, the three
//!   edges there — takes the value asked, chamfered or filleted, each laid as
//!   alone: the volume the corner's own less each outline edge's, the
//!   numbering by rank, the patches after. The step was declined `Travel`:
//!   at a corner whose three edges are all named, each foot's set-back was
//!   read on past the corner without end, reaching the far edge at no
//!   value —
//!   `a_chamfer_and_a_fillet_on_a_foot_corner_and_edges_of_the_lower_block_s_outline_beyond_its_feet_take_the_value_asked`
//! - Found on the way, not in the issue's wording: a foot corner beside a
//!   second block that none of its blends reaches takes the value asked,
//!   chamfered at 2.5 as filleted, the volume the corner's own and the
//!   block's. The chamfer was declined `Unsupported`, the fill along the foot
//!   of the upright's flat read as reaching the block's back left corner
//!   along its line past its mitre with the right foot, at `3.5/√2`; so did
//!   #801's own step on the three feet round the upright chamfered earlier,
//!   which took `3.5/√2`. Each now reads a face's reach only where the blend
//!   runs, and the chamfer is the body those two steps build, face for face,
//!   as at a corner alone —
//!   `a_chamfer_and_a_fillet_on_a_foot_corner_beside_a_block_its_flat_s_foot_reaches_only_past_its_mitre_take_the_value_asked`
//! - Found on the way, not in the issue's wording: read only where the blend
//!   runs, a face's reach still stops at what the fill sweeps into, its
//!   corners in the sweep or past its ends. A foot filled alone, and its
//!   corner chamfered, beside a small block standing 1 in front of the foot
//!   take 1; beside a low wall 1.5 in front of the whole standing block, its
//!   corners past the planes of the left and right walls that trim the fill
//!   at its ends, they take 1.5, as they did when every corner of the face
//!   counted — the wall's edge across the sweep stops the fill where its
//!   corners no longer do —
//!   `a_chamfer_and_a_fillet_on_a_foot_alone_and_on_its_corner_stop_where_their_fill_reaches_what_stands_in_its_sweep`.
//!   Where a foot enters the sweep between its corners, across the plane a
//!   fill ends on, the fill stops where it crosses that plane, at a depth
//!   neither corner has: a slanted wall in front of the front foot, its back
//!   crossing the plane of the right wall at its middle, 2 in front of the
//!   foot, its corners 2.5 in front inside the sweep and 1.5 past it, takes
//!   2 asked 2.5 —
//!   `a_chamfer_and_a_fillet_on_a_foot_alone_stop_where_a_slanted_wall_s_foot_enters_their_sweep_between_its_corners`
//! - Found on the way, not in the issue's wording: a block standing where a
//!   foot corner's patch sweeps — past both walls, out to twice the radius
//!   from the upright round's axis, or under the fill along the foot of the
//!   upright's flat — but where no foot's fill reaches is never taken away.
//!   Every face of the block is kept and its matter added to the corner's.
//!   The value is reduced to where the patch first touches the block's
//!   nearest corner, `gap` off both walls — `gap·√2/(2 − √2)` filleted,
//!   `2·gap/(√2 − 1)` chamfered — or the step is declined; short of that it
//!   is taken whole. The fillet laid the piece of torus through the block,
//!   its filling having swallowed it, and left a sound body without it —
//!   `a_chamfer_and_a_fillet_on_a_foot_corner_never_take_away_a_block_standing_where_the_corner_s_patch_sweeps`
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
//!
//! Part of #829, its second slice: the top of the L's inner corner, the
//! mirror of the first. Its two top edges meet outwards and the inner upright
//! below them inwards. The upright is filled first, its fill ending flush on
//! the top face as it does alone; the top edges' cuts then run along their
//! walls up to where the fill begins and go round it. Chamfered, the top edge
//! of the fill's flat is chamfered too, though no name in the list finds it,
//! and the three flats meet in mitres at the top face's two reflex corners —
//! the body #801 cuts in two steps. Filleted, the top edges' rounds join
//! through a piece of torus cut out about the fill's axis, its tube of the
//! value's radius, its centre circle twice the value from that axis and the
//! value below the top, tangent to both rounds, to the top face and to the
//! fill's cylinder. The issue sets a corner of the L that is not square out
//! of its scope, but such a corner builds all the same — every three-edge
//! corner of mixed sense is taken, and nothing in the filling, its axis or
//! its piece of torus is held to a square — so it is held below, as slice 1
//! held its trapezoid, rather than left to ship untested.
//!
//! - The three edges at the top of the L's inner corner — the two top edges
//!   and the inner upright — leave a closed, sound, drawn body when chamfered
//!   or filleted at one value, shaped as above, its volume exact, the same
//!   body in every order. Chamfered, the body is the one two steps leave, the
//!   upright filled alone and then the three top edges round its fill —
//!   `a_chamfer_on_the_two_top_edges_at_the_l_s_inner_corner_and_the_upright_below_them_cuts_the_top_of_the_upright_s_fill_and_mitres_the_three_flats_in_every_order`;
//!   filleted —
//!   `a_fillet_on_the_two_top_edges_at_the_l_s_inner_corner_and_the_upright_below_them_joins_their_rounds_through_a_piece_of_torus_cut_out_in_every_order`
//! - Numbered as slice 1 — held in the two tests above: each named edge's
//!   face where the steps' numbers ended plus its rank in the list, then the
//!   cut along the top of the fill's flat, or the piece of torus, and the
//!   next step's numbers past it
//! - The decline test
//!   `a_fillet_and_a_chamfer_on_edges_meeting_at_a_corner_with_a_concave_edge_are_declined_as_unsupported`
//!   loses the list of the three edges at the top of the L's inner corner —
//!   no test: a change to that test, in
//!   `a_chamfer_and_a_fillet_blend_the_corner_their_edges_meet_at.rs`, which
//!   keeps its two lists of two edges for #846; and in `corners/tests.rs`,
//!   where the corner's own answer is read, the three edges there are taken
//!   with no corner beside an edge, whatever the order
//! - What the first slice found on the way, held here where it applies, not
//!   in the issue's wording. Every order of the three names, and each top
//!   edge's name found again on what the fill leaves, both its ends moved —
//!   in the two tests above. The corner named with an edge of the L's outline
//!   that the line of a top edge's set-back would reach past the corner, the
//!   L's left top edge, taking the value asked, each laid as alone —
//!   `a_chamfer_and_a_fillet_on_the_top_of_the_l_s_inner_corner_and_its_left_top_edge_take_the_value_asked`;
//!   and a block standing on the L's top where the corner's patch cuts, but
//!   no top edge's cut reaches, never cut away, the value reduced to where
//!   the patch first touches it, or the step declined —
//!   `a_chamfer_and_a_fillet_on_the_top_of_the_l_s_inner_corner_never_cut_a_block_standing_where_the_corner_s_patch_sweeps`.
//!   Beside that block #801's chamfer path declines between about 1.05 and
//!   1.44, short of where anything reaches the block, tracked as #863; the
//!   test asks 0.5, 1 and 2.5
//! - Not in the issue's wording, which sets it out of scope: the same at an
//!   inner corner that is not square, an L whose stem's right side leans
//!   from the corner so that the notch between it and the arm's back is
//!   105°, 75° or 50° wide — below 60° too, where it was read to decline and
//!   builds, right. Chamfered, the body two steps leave, the volume the
//!   arithmetic's; filleted, the rounds, the fill's cylinder and the piece of
//!   torus turning through 180° less the notch, on their surfaces; every
//!   order, at 1 and 2 —
//!   `a_chamfer_and_a_fillet_on_the_top_of_an_inner_corner_that_is_not_square_go_round_its_upright_in_every_order`.
//!   At 2.5 the 105° corner's chamfer is declined, and #801's own step on the
//!   three edges round the fill takes 2.48: the cut along the top of the
//!   fill's flat reads the stem's back left corner, (0, 14), as far as its
//!   face runs, 2.48 along the way it sets back, though it runs only by the
//!   inner corner
//!
//! The L, an arm 30 by 6 and a stem 12 by 8 raised 10, holds 2760. The
//! corner stands at (12, 6, 10): its top edge on the arm's back runs to the
//! arm's end, 18 long, its top edge on the stem's right side to the stem's
//! back, 8, each square to the face it ends on, and the inner upright stands
//! 10 below them, from the floor.
//!
//! Chamfered at `d`, the upright's fill adds `10·d²/2` and leaves the top's
//! outline a side `d√2` long between the arm's back, `18 − d` from the arm's
//! end, and the stem's right side, `8 − d` to the stem's back, at two reflex
//! corners of 225°. At a depth `c` below the top the three cuts take what
//! lies between that outline and the outline offset `t = d − c` into the
//! matter, its two corners mitred: `(26 − 2d + d√2)·t + 2·tan(22.5°)·t²`. Over
//! the depth they take `(26 − 2d + d√2)·d²/2 + 2·tan(22.5°)·d³/3`, and the
//! body holds `2760 + 5d² − (26 − 2d + d√2)·d²/2 − 2·tan(22.5°)·d³/3`, that
//! is `2760 − 8d² + (5/3 − 7√2/6)·d³` — slice 1's
//! `7000 + 5d² + (7√2/6 − 5/3)·d³` mirrored, every term taken where it was
//! added: 2752.0168 at 1, #801's body round its upright filled by 1 in an
//! earlier step. Held another way in the test itself: the body those two
//! steps build.
//!
//! Filleted at `r`, the upright's round, its axis `r` out from both walls in
//! the air, adds `10·r²(1 − π/4)`. The top edges' rounds run from the faces
//! at their far ends to the two lines where the fill's round leaves the
//! walls, `18 − r` and `8 − r`, and take `(26 − 2r)·r²(1 − π/4)`. The piece of
//! torus between them turns their section a quarter turn about the fill's
//! axis: in a plane through that axis the section spans `ρ` from `r` to `2r`
//! out from it and `z` from 0 to `r` down from the top, less the disc of
//! radius `r` centred `(2r, r)` below the top — slice 1's section mirrored
//! across the plane halfway up it, so its `∫ρ dA` is the same
//! `r³(11/6 − π/2)`, and the piece takes `(π/2)·r³(11/6 − π/2)`. The body
//! holds `2760 + 10·r²(1 − π/4) − (26 − 2r)·r²(1 − π/4) − (π/2)·r³(11/6 − π/2)`.
//!
//! At a notch `β` that is not square, the leaning side runs from the corner
//! at `β` from the arm's back, `8/sin β` up to the stem's back at 14, which
//! leans across it, and the L holds `10·(180 + 4·(24 + 8·cot β))`. The
//! formulas of slice 1's trapezoid hold with every term's sign turned — the
//! fill's flat adds `5d²·sin β`, the round's kite less its sector
//! `10·(r²·cot(β/2) − (π − β)·r²/2)`, the piece of torus takes
//! `(π − β)·r³(11/6 − π/2)`, the mitres turn `(π − β)/2` each — but one: the
//! term a face leaning across an edge's far end brings, the section's first
//! moment times `cot β`, keeps its sign. A fill's lines along the edge stand
//! out from the wall, a cut's inside it, so the lean that shortens the one
//! lengthens the other: the stem's back, leaning across the leaning side's
//! far end, gives the part the moment times `cot β` whether that edge is
//! filled or cut. Held another way in the test itself: the body two steps
//! build, to a hair at 1 and 2.
//!
//! Part of #829, its third slice: every edge of a standing block. Its top
//! corners are convex, each closed as #699 closes one, by a triangle among
//! flats and a piece of sphere among rounds; its foot corners are gone round
//! their uprights as the first slice goes round one. Each upright is the odd
//! edge at its foot and one of three like edges at its top.
//!
//! - The twelve edges of a standing block — its four top edges, four
//!   uprights and four feet — chamfered or filleted at one value in one
//!   step: its top corners closed as #699 closes a convex corner (a
//!   triangle, a piece of sphere), its foot corners as slice 1. Closed,
//!   sound, drawn, volume exact, the same body for the list as given,
//!   reversed, and one other order —
//!   `a_chamfer_and_a_fillet_on_the_twelve_edges_of_a_standing_block_close_its_top_corners_as_a_convex_corner_and_its_foot_corners_round_their_uprights`.
//!   Read as: each edge's face at its rank, lying about its edge's middle, a
//!   flat facing halfway between its edge's two faces, a round on its
//!   cylinder from the plane a radius in from the corner at one end to the
//!   one at the other, meeting its two faces and the patches there; the
//!   eight patches
//!   after them by their corners' lowest ranks, then the next; at each top
//!   corner the triangle facing out of it, or the eighth of a sphere `r` in
//!   from its three faces, touching each, met by rays from its centre and
//!   meeting its three rounds; at each foot corner the fill along the foot
//!   of the upright's flat, or the first slice's piece of torus. Chamfered,
//!   the body two steps leave too, face for face: the block's top half
//!   first, then the feet and the feet of the uprights' flats
//! - Beside it, the smaller case it is built from, not in the issue's
//!   wording: one foot corner and the corner at the top of its upright, both
//!   feet, the upright and the two top edges at its top, which slice 1's
//!   guard declined — held the same way, every order —
//!   `a_chamfer_and_a_fillet_on_a_foot_corner_and_the_top_corner_of_its_upright_close_both_in_one_step`
//! - Not in the issue's wording, what the guard still declines beside it: an
//!   upright odd at its foot whose top meets an edge of the list alone, that
//!   edge closing a corner of one sense elsewhere but none with the upright —
//!   the standing block's two front foot corners, its three feet there and
//!   its two front uprights, with the top front edge and the top left edge,
//!   which close the front left top corner while the front right upright's
//!   top meets the top front edge alone: declined `Unsupported`, chamfered or
//!   filleted, whatever the order —
//!   `a_fillet_and_a_chamfer_on_two_foot_corners_and_the_top_corner_of_one_upright_are_declined_as_unsupported_where_the_other_upright_s_top_meets_a_top_edge_alone`
//! - Found on the way, not in the issue's wording: every edge of an L
//!   standing on a plate, its eighteen edges filleted at one value in one
//!   step, each of its twelve corners closed by its kind — a piece of sphere
//!   at each of its five convex top corners, the second slice's piece of
//!   torus cut out at the top of its inner corner, the first slice's piece of
//!   torus at each of its five outer foot corners, and a hollow piece of
//!   sphere at the foot of its inner corner, where its two inner feet and its
//!   inner upright all meet inwards — sound, exact, numbered by rank, each
//!   round and each patch on its surface, the same body in every order —
//!   `a_fillet_on_every_edge_of_an_l_shaped_block_standing_on_another_closes_each_corner_by_its_kind`.
//!   Chamfered, the L's inner upright is the odd edge at its top and one of
//!   three like edges at its foot, which ties the top edges round it to the
//!   first of the two layings the chamfer goes round its uprights in, where
//!   the second needs them: the step is declined `Unsupported`, every edge
//!   named or only the inner upright, the top edges at its top, the arm
//!   end's back upright, its two feet and the arm end's top edge, until #867
//!   builds it —
//!   `a_chamfer_on_every_edge_of_an_l_shaped_block_standing_on_another_is_declined_as_unsupported_until_867_builds_it`
//!
//! The volumes are arithmetic: every edge takes or adds its section along its
//! whole length, the top corners give back what #699's convex corner gives
//! back — `2d³/3` among flats, `r³(2 − 7π/12)` among rounds, each corner's
//! three edges counted whole — and the foot corners add what the first slice
//! adds round its uprights, the uprights counted once. On the five edges at
//! both ends of the front right upright, the front right foot corner's own
//! volume and its two top edges', 10 each:
//! `7000 − 15d² + 2d³/3 + (20 − 2d + d√2)·d²/2 + 2·tan(22.5°)·d³/3` among
//! flats, and
//! `7000 − 30·r²(1 − π/4) + r³(2 − 7π/12) + 2(10 − r)·r²(1 − π/4) + (π/2)·r³(11/6 − π/2)`
//! among rounds. On the twelve, the four feet and four uprights' octagon or
//! rounds, and the four top edges' 40 with their four corners:
//! `7000 − 40d² + 8d³/3 + (40 − 8d + 4d√2)·d²/2 + 8·tan(22.5°)·d³/3` among flats,
//! and `7000 − 80·r²(1 − π/4) + 4(10 − 2r)·r²(1 − π/4) + 2π·r³(11/6 − π/2) +
//! 4r³(2 − 7π/12)` among rounds. Held another way in the test, among flats:
//! the body two steps build, the block's top half and then its feet, to a
//! hair at 1 and 2.5.
//!
//! The L standing on the plate, an arm 18 by 7 and a stem 8 by 7 raised 10 on
//! the plate's 6000, holds 7820; every corner of it is square. Filleted at
//! `r`, each top edge takes its section along its length, but the two at the
//! top of the inner corner, which stop `r` short of it, there: `P − 2r` along
//! the outline's length `P`. Each foot adds its section from `r` short of
//! each outer foot corner and up to the inner one: `P − 10r`. The five convex
//! uprights take theirs along their 10 and the inner one adds its own: `−40`
//! sections. The five convex top corners give back `r³(2 − 7π/12)` each and
//! the hollow sphere takes it back once at the inner foot; the five outer
//! foot corners' pieces of torus add `(π/2)·r³(11/6 − π/2)` each, and the one
//! cut out at the inner top corner takes it once. The outline's length
//! cancels: `7820 − 40·r²(1 − π/4) − 8r·r²(1 − π/4) + 4r³(2 − 7π/12) +
//! 2π·r³(11/6 − π/2)`, the twelve edges' form on the plain block, as the
//! outline still turns once round.

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

/// A corner of mixed sense: what it is called, the part and the matter it
/// holds, where the corner stands on the face its two like edges share, those
/// two edges, what the odd edge between their walls is called, and whether
/// that edge meets inwards and is filled first, its two neighbours cut. At a
/// foot corner of a block standing on another the two are its feet, filled,
/// and the odd edge the upright above them, cut; at the top of an L's inner
/// corner the two are its top edges, cut, and the odd edge the inner upright
/// below them, filled. Either way the odd edge stands [`STANDING_HEIGHT`]
/// tall, up from the face the two others share or down from it to the face
/// `odd_ends_on`. Its three edges are named, in every list here, by their place in
/// [`FootCorner::edges`]: the first of the two, the second, and the odd edge.
struct FootCorner {
    called: &'static str,
    part: fn() -> Body,
    volume: f64,
    at: DVec3,
    feet: [Foot; 2],
    upright: &'static str,
    odd_filled: bool,
    odd_ends_on: usize,
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

    /// Where the axis of a round of `radius` on the odd edge stands on the
    /// face the two others share: `radius` in from both walls where the odd
    /// edge is cut, `radius` out from them where it is filled.
    fn axis(&self, radius: f64) -> DVec3 {
        let foot = &self.feet[0];
        self.at + foot.along * self.tangent(radius) - foot.facing * radius * self.sense()
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

    /// What a blend whose section is `section`, its first moment about either
    /// face `moment`, gives along the two like edges, from `from` along each
    /// from the corner to the face at its far end — a fill adds it, a cut
    /// takes it, as [`FootCorner::sense`] signs it: the section times that
    /// length, and where that face leans across the edge, the moment times how
    /// far the lines along the edge shrink a unit out from its wall, as #801's
    /// arithmetic reads it. A fill's lines stand out from the wall and a cut's
    /// inside it, so the lean shortens the one where it lengthens the other:
    /// that term is signed again, to keep its sign once the whole is.
    fn along_the_feet(&self, from: f64, section: f64, moment: f64) -> f64 {
        self.feet
            .iter()
            .map(|foot| {
                let length = (foot.far.at - self.at).dot(foot.along) - from;
                let lean = foot.facing.dot(foot.far.facing) / foot.along.dot(foot.far.facing);
                section * length - moment * lean * self.sense()
            })
            .sum()
    }

    /// One where the odd edge is cut and its two neighbours filled, minus one
    /// where it is filled and they are cut: the sign of what each blend gives
    /// the part.
    fn sense(&self) -> f64 {
        if self.odd_filled { -1.0 } else { 1.0 }
    }

    /// What the part holds once the corner's three edges are chamfered at
    /// `value`, as the module's documentation derives it: the odd edge's flat
    /// taken or added, and at each height the outline it leaves offset out
    /// from the walls over the face the two others share, its two corners
    /// mitred, added or taken.
    fn chamfered(&self, value: f64) -> f64 {
        let d = value;
        let half = self.angle() / 2.0;
        self.volume
            + self.sense()
                * (self.along_the_feet(d, d * d / 2.0, d.powi(3) / 6.0)
                    + 2.0 * d * half.sin() * d * d / 2.0
                    + 2.0 * (self.turn() / 4.0).tan() * d.powi(3) / 3.0
                    - STANDING_HEIGHT * d * d * self.angle().sin() / 2.0)
    }

    /// What the part holds once the corner's three edges are filleted at
    /// `radius`, as the module's documentation derives it: the odd edge's
    /// round taken or added, the two others' rounds added or taken from where
    /// it leaves their walls, and the piece of torus between them by Pappus.
    fn filleted(&self, radius: f64) -> f64 {
        let r = radius;
        let tangent = self.tangent(r);
        self.volume
            + self.sense()
                * (self.along_the_feet(
                    tangent,
                    r * r * (1.0 - PI / 4.0),
                    r.powi(3) * (5.0 / 6.0 - PI / 4.0),
                ) + self.turn() * r.powi(3) * (11.0 / 6.0 - PI / 2.0)
                    - STANDING_HEIGHT * (r * tangent - self.turn() * r * r / 2.0))
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
        odd_filled: false,
        odd_ends_on: STANDING_TOP,
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
        odd_filled: false,
        odd_ends_on: STANDING_TOP,
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
        odd_filled: false,
        odd_ends_on: STANDING_TOP,
    }
}

/// The corner's part chamfered at `value` in two steps, as #801 builds it:
/// the odd edge alone first, the flat laid on it answering to where the
/// part's numbers ended and stopping flush on the face the two others share;
/// then, in one step, those two and the edge of that flat on that face,
/// numbered in that order past it, as [`IN_TWO_STEPS`] says.
fn chamfered_in_two_steps(corner: &FootCorner, value: f64) -> Body {
    let part = (corner.part)();
    let [one, other] = corner.feet.each_ref().map(|foot| foot.wall);
    let upright = part
        .chamfer(&edge_between(&part, one, other), value)
        .unwrap_or_else(|declined| panic!("the odd edge alone is chamfered: {declined:?}"));
    taken_as_asked(
        upright.asked,
        upright.taken,
        value,
        "the odd edge chamfered alone",
    );
    let chamfered = upright.body;
    let feet = [
        edge_between(&chamfered, TOP, one),
        edge_between(&chamfered, TOP, other),
        edge_between(&chamfered, TOP, part.faces_end()),
    ];
    let filled = chamfered
        .chamfer_edges(&feet, value)
        .unwrap_or_else(|declined| {
            panic!("the three edges round the odd edge's flat are chamfered: {declined:?}")
        });
    taken_as_asked(
        filled.asked,
        filled.taken,
        value,
        "the three edges round the odd edge's flat",
    );
    filled.body
}

/// The numbers the flats on a corner's three edges answer to in the body
/// [`chamfered_in_two_steps`] leaves, past where the part's numbers ended, in
/// the order of [`FootCorner::edges`], and that of the fill along the foot of
/// the upright's flat.
const IN_TWO_STEPS: ([usize; 3], usize) = ([1, 2, 0], 3);

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
///   plus the edge's rank in the list, then the flat along the end of the
///   odd edge's flat on the face the two others share — the fill along its
///   foot, or the cut along its top — which no name finds, and the next
///   step's numbers past it; no face of the part gone;
/// - each of those flats facing the way it lies;
/// - and `two_steps`, the body the odd edge chamfered and then the three
///   edges round its flat leave, the same body, face for face.
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
        "{what}: one flat for each edge, numbered in the order the step names them, and the flat \
         along the end of the odd edge's flat after them: {:?}",
        numbers(body)
    );
    assert_eq!(
        body.faces_end(),
        patch + 1,
        "{what}: the next step numbers its own faces past the flat along the end of the odd \
         edge's flat"
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
        &format!("{what}: the flat along the end of the odd edge's flat"),
    );
    let (edges_in_two_steps, foot_in_two_steps) = IN_TWO_STEPS;
    let to_two_steps: Vec<(usize, usize)> = (0..3)
        .map(|edge| (case.laid_on(part, edge), start + edges_in_two_steps[edge]))
        .chain([(patch, start + foot_in_two_steps)])
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

impl FootCorner {
    /// The rounds a fillet of `radius` on the corner's three edges leaves, in
    /// the order of [`FootCorner::edges`]: each of the two like edges', from
    /// the plane through the odd edge round's axis square to its wall to the
    /// plane of the face at its far end; and the odd edge's, from the circle
    /// where the piece of torus meets it, `radius` from the face the two
    /// share, to the face it ends on, [`STANDING_HEIGHT`] away.
    fn rounds(&self, radius: f64) -> [Cylinder; 3] {
        let r = radius;
        let sense = self.sense();
        let tangent = self.tangent(r);
        let [one, other] = &self.feet;
        let foot = |foot: &Foot| Cylinder {
            called: foot.called,
            through: self.at + (foot.facing + DVec3::Z) * r * sense,
            along: foot.along,
            radius: r,
            toward: [-DVec3::Z * sense, -foot.facing * sense],
            ends: [
                (self.at + foot.along * tangent, -foot.along),
                (foot.far.at, foot.far.facing),
            ],
            meets: vec![TOP, foot.wall, foot.far.on],
        };
        let axis = self.axis(r);
        let up = DVec3::Z * sense;
        [
            foot(one),
            foot(other),
            Cylinder {
                called: self.upright,
                through: axis,
                along: DVec3::Z,
                radius: r,
                toward: [one.facing * sense, other.facing * sense],
                ends: [(axis + up * r, -up), (axis + up * STANDING_HEIGHT, up)],
                meets: vec![self.odd_ends_on, one.wall, other.wall],
            },
        ]
    }

    /// The piece of torus a fillet of `radius` on the corner's three edges
    /// joins the two like edges' rounds through.
    fn torus(&self, radius: f64) -> Torus {
        let sense = self.sense();
        Torus {
            axis: self.axis(radius),
            radius,
            from: self.feet[0].facing * sense,
            to: self.feet[1].facing * sense,
            sense,
            shared: TOP,
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
/// met first by every ray from its tube's centre circle out to it, facing
/// out of the matter, and meeting the face the two like edges share and the
/// rounds `rounds`, one edge each.
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
    let met: Vec<usize> = [torus.shared].iter().chain(rounds).copied().collect();
    assert_eq!(
        sides_met(body, piece),
        sides(&met),
        "{which} meets the face the two like edges share and the rounds round it, one edge each"
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

/// Holds what [`round_the_upright_in_every_order_at`] holds at every one of
/// [`VALUES`].
fn round_the_upright_in_every_order(corners: &[FootCorner], cuts: &[Cut], called: &str) {
    round_the_upright_in_every_order_at(corners, cuts, &VALUES, called);
}

/// Holds that the three edges of each of `corners`, named in every one of
/// [`ORDERS_OF_THREE`] and blended by each of `cuts` at every one of
/// `values` in one step, go round the upright: a chamfer as
/// [`went_round_the_upright`] holds it, against the body two steps leave,
/// whose matter the arithmetic gives too; a fillet as
/// [`joined_through_a_piece_of_torus`] holds it; and the same body whatever
/// the order, the face laid on each edge answering to the number its rank
/// gives it. Every list is laid before any body is read, so that a decline
/// names every case that declines.
fn round_the_upright_in_every_order_at(
    corners: &[FootCorner],
    cuts: &[Cut],
    values: &[f64],
    called: &str,
) {
    let parts: Vec<Body> = corners.iter().map(|corner| (corner.part)()).collect();
    let mut in_two_steps = Vec::new();
    for corner in corners.iter().filter(|_| cuts.contains(&Cut::Flat)) {
        for &value in values {
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
            for &value in values {
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
        odd_filled: false,
        odd_ends_on: STANDING_TOP,
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

/// The five edges at both ends of [`stacked`]'s front right upright, by the
/// faces each lies between: the front and right feet at its foot, the
/// upright itself, and the standing block's top front and top right edges at
/// its top.
const BOTH_ENDS_OF_AN_UPRIGHT: [[usize; 2]; 5] = [
    [TOP, STANDING_FRONT],
    [TOP, STANDING_RIGHT],
    [STANDING_FRONT, STANDING_RIGHT],
    [STANDING_TOP, STANDING_FRONT],
    [STANDING_TOP, STANDING_RIGHT],
];

/// A corner a list of [`stacked`]'s standing block's edges closes, by its
/// three edges' places in that list: a foot corner, gone round its upright as
/// the first slice lays it, its two feet first, then the upright; or the
/// convex corner at the top of that upright, closed as #699 closes one; and
/// which corner of [`stacked_s_foot_corners`] it stands at, or above.
struct OfTheBlock {
    edges: [usize; 3],
    foot: bool,
    under: usize,
}

/// The way a face of [`stacked`] an edge of its standing block lies between
/// faces, out of the matter.
fn facing_of(face: usize) -> DVec3 {
    match face {
        TOP | STANDING_TOP => DVec3::Z,
        STANDING_FRONT => -DVec3::Y,
        STANDING_RIGHT => DVec3::X,
        STANDING_BACK => DVec3::Y,
        STANDING_LEFT => -DVec3::X,
        _ => panic!("no way known for face {face}"),
    }
}

/// Where the middle of an edge of [`stacked`]'s standing block stands, by the
/// faces it lies between: a foot at the bottom of its wall, a top edge at the
/// top, an upright halfway up between its two walls.
fn middle_on_the_block(edge: [usize; 2]) -> DVec3 {
    let across = |wall: usize| match wall {
        STANDING_FRONT => DVec2::new(15.0, 5.0),
        STANDING_RIGHT => DVec2::new(20.0, 10.0),
        STANDING_BACK => DVec2::new(15.0, 15.0),
        STANDING_LEFT => DVec2::new(10.0, 10.0),
        _ => panic!("no wall {wall}"),
    };
    match edge {
        [TOP, wall] => across(wall).extend(LOWER_TOP),
        [STANDING_TOP, wall] => across(wall).extend(LOWER_TOP + STANDING_HEIGHT),
        [one, other] => {
            let middle = across(one) + across(other) - DVec2::new(15.0, 10.0);
            middle.extend(LOWER_TOP + STANDING_HEIGHT / 2.0)
        }
    }
}

/// Holds that the face numbered `face` is the round a fillet of `radius` lays
/// on the straight edge from `from` to `to` between the faces `faces`, which
/// face `ways` out of the matter — a cut where the edge meets `outwards`, a
/// fill where it meets inwards: on its cylinder, its axis `radius` from both
/// faces, inside the matter or in the air; between the planes square to the
/// edge `radius` in from an end where a corner is closed, the patch `closed`
/// names there, and flush on the plane through an end where none is; meeting
/// its two faces, and at each end the patch closing it or the face `ending`
/// it runs into there, one edge each.
fn edge_round_held(
    body: &Body,
    face: usize,
    (faces, ways): ([usize; 2], [DVec3; 2]),
    ends: [(DVec3, usize); 2],
    (closed, outwards): ([Option<usize>; 2], bool),
    radius: f64,
    what: &str,
) {
    let [(from, from_face), (to, to_face)] = ends;
    let along = (to - from).normalize();
    let out = if outwards { -1.0 } else { 1.0 };
    let stop = |end: usize| if closed[end].is_some() { radius } else { 0.0 };
    let meets: Vec<usize> = faces
        .into_iter()
        .chain([from_face, to_face].into_iter().zip(closed).filter_map(
            |(ending, patch)| match patch {
                Some(_) => None,
                None => Some(ending),
            },
        ))
        .collect();
    let round = Cylinder {
        called: "an edge",
        through: from + (ways[0] + ways[1]) * radius * out,
        along,
        radius,
        toward: ways.map(|way| way * -out),
        ends: [
            (from + along * stop(0), -along),
            (to - along * stop(1), along),
        ],
        meets,
    };
    let pieces: Vec<usize> = closed.into_iter().flatten().collect();
    round_held(body, face, &round, &pieces, what);
}

/// The two ends of an edge of [`stacked`]'s standing block, by the faces it
/// lies between, each with the face it runs into there: a top edge or a foot
/// from the wall before its own, in turn round the block from the front, to
/// the wall after it; an upright from the lower block's top to the standing
/// block's.
fn ends_on_the_block(edge: [usize; 2]) -> [(DVec3, usize); 2] {
    let corners = [
        DVec2::new(10.0, 5.0),
        DVec2::new(20.0, 5.0),
        DVec2::new(20.0, 15.0),
        DVec2::new(10.0, 15.0),
    ];
    let wall = |turn: usize| STANDING_FRONT + turn % 4;
    match edge {
        [level @ (TOP | STANDING_TOP), own] => {
            let turn = own - STANDING_FRONT;
            let height = if level == TOP {
                LOWER_TOP
            } else {
                LOWER_TOP + STANDING_HEIGHT
            };
            [
                (corners[turn].extend(height), wall(turn + 3)),
                (corners[(turn + 1) % 4].extend(height), wall(turn + 1)),
            ]
        }
        [before, _] => {
            let corner = corners[(before - STANDING_FRONT + 1) % 4];
            [
                (corner.extend(LOWER_TOP), TOP),
                (corner.extend(LOWER_TOP + STANDING_HEIGHT), STANDING_TOP),
            ]
        }
    }
}

/// Holds that the face numbered `piece` is the piece of sphere closing a
/// corner of three like edges: drawn on the sphere of `radius` about
/// `centre`, within the octant the three ways `outward` from the centre to
/// the corner's faces bound, touching each of those faces' planes, met first
/// by every ray from the centre out into that octant, and meeting the three
/// rounds `rounds`, one edge each. Where `sense` is one the corner is convex,
/// the sphere cut out of it and facing along the rays; where it is minus one
/// the corner is concave, the sphere hollow in its fill and facing back
/// along them, toward its centre in the air.
fn sphere_held(
    body: &Body,
    piece: usize,
    (centre, radius, sense): (DVec3, f64, f64),
    outward: [DVec3; 3],
    rounds: &[usize],
    what: &str,
) {
    let which = format!("{what}: the piece of sphere, face {piece}");
    let touching = outward.map(|way| centre + way * radius);
    curved_and_drawn_on(
        body,
        piece,
        |point| {
            let off = point - centre;
            (off.length() - radius).abs() < PLACE
                && outward.iter().all(|&way| off.dot(way) >= -PLACE)
        },
        &touching,
        &which,
    );
    for weights in [
        [1.0, 1.0, 1.0],
        [3.0, 1.0, 1.0],
        [1.0, 3.0, 1.0],
        [1.0, 1.0, 3.0],
    ] {
        let way = (0..3)
            .map(|face| outward[face] * weights[face])
            .sum::<DVec3>()
            .normalize();
        let hit = body
            .ray_hit(centre, way)
            .unwrap_or_else(|| panic!("{which}: a ray from its centre {centre} meets nothing"));
        assert_eq!(
            hit.face, piece,
            "{which}: a ray from its centre {centre} meets face {} before it",
            hit.face
        );
        assert!(
            hit.normal.distance(way * sense) < PLACE,
            "{which}: where a ray from its centre meets it, it faces {}, not {}, out of the \
             matter",
            hit.normal,
            way * sense
        );
    }
    assert_eq!(
        sides_met(body, piece),
        sides(rounds),
        "{which} meets the three rounds, one edge each, and nothing else"
    );
}

/// Holds what a step of its blend and value on a list of [`stacked`]'s
/// standing block's edges `edges`, named in its order, the corners
/// `corners` closed among them, leaves of the part:
///
/// - the value taken whole; a sound body holding `promised`;
/// - one face for each edge, answering to where the part's numbers ended
///   plus the edge's rank, lying about the edge's middle, a flat facing
///   halfway between its edge's two faces; then one patch at each corner,
///   by the lowest ranks of its edges, then the next; the next step's
///   numbers past them;
/// - at a foot corner, the fill along the foot of the upright's flat, or the
///   piece of torus as the first slice lays it; at a top corner, the
///   triangle facing out of the corner, or the piece of sphere.
fn closed_round_the_block(
    part: &Body,
    edges: &[[usize; 2]],
    corners: &[OfTheBlock],
    case: &Case,
    promised: f64,
) {
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
    let foot_corners = stacked_s_foot_corners();
    let drawn = match cut {
        Cut::Flat => EXACT * promised,
        Cut::Round => {
            let curved = PI / 2.0 * value * 10.0 * edges.len() as f64
                + corners.len() as f64 * PI * value * value;
            DRAWN * reach_of(part) * curved
        }
    };
    sound_drawn_within(body, promised, drawn, what);
    let start = part.faces_end();
    assert_eq!(
        faces_made(part, body),
        (start..start + edges.len() + corners.len()).collect::<Vec<_>>(),
        "{what}: one face for each edge, and a patch at each corner after them: {:?}",
        numbers(body)
    );
    assert_eq!(
        body.faces_end(),
        start + edges.len() + corners.len(),
        "{what}: the next step numbers its own faces past the patches"
    );
    let laid = |edge: usize| case.laid_on(part, edge);
    let by_edges: Vec<[usize; 3]> = corners.iter().map(|corner| corner.edges).collect();
    let patches = case.patches(part, &by_edges);
    let corner_at = |point: DVec3| {
        (corners.iter().zip(&patches))
            .find(|(corner, _)| {
                let under = &foot_corners[corner.under];
                let at = if corner.foot {
                    under.at
                } else {
                    under.at + DVec3::Z * STANDING_HEIGHT
                };
                at.distance(point) < PLACE
            })
            .map(|(_, &patch)| patch)
    };
    for (edge, &[one, other]) in edges.iter().enumerate() {
        let which = format!(
            "{what}: the face on {:?}, numbered by its rank",
            [one, other]
        );
        lies_about(
            body,
            laid(edge),
            middle_on_the_block([one, other]),
            2.0 * value,
            &which,
        );
        match cut {
            Cut::Flat => {
                let way = (facing_of(one) + facing_of(other)).normalize();
                faces_toward(body, laid(edge), way, &which);
            }
            Cut::Round => {
                let ends = ends_on_the_block([one, other]);
                edge_round_held(
                    body,
                    laid(edge),
                    ([one, other], [facing_of(one), facing_of(other)]),
                    ends,
                    (ends.map(|(at, _)| corner_at(at)), one != TOP),
                    value,
                    &which,
                );
            }
        }
    }
    for (corner, patch) in corners.iter().zip(patches) {
        let under = &foot_corners[corner.under];
        let [one, other] = under.feet.each_ref().map(|foot| foot.facing);
        let rounds: Vec<usize> = corner.edges.iter().map(|&edge| laid(edge)).collect();
        let top = under.at + DVec3::Z * STANDING_HEIGHT;
        let which = format!(
            "{what}: the patch {} {}, face {patch}",
            if corner.foot { "at" } else { "above" },
            under.called
        );
        match (corner.foot, cut) {
            (true, Cut::Flat) => {
                faces_toward(body, patch, under.flats_facing().1, &which);
                lies_about(body, patch, under.at, 2.0 * value, &which);
            }
            (true, Cut::Round) => piece_held(body, patch, &under.torus(value), &rounds, &which),
            (false, Cut::Flat) => {
                faces_toward(body, patch, (DVec3::Z + one + other).normalize(), &which);
                lies_about(body, patch, top, 2.0 * value, &which);
            }
            (false, Cut::Round) => sphere_held(
                body,
                patch,
                (top - (DVec3::Z + one + other) * value, value, 1.0),
                [DVec3::Z, one, other],
                &rounds,
                &which,
            ),
        }
    }
}

/// [`stacked`] chamfered at `value` in two steps on the edges `edges` of its
/// standing block, the corners `corners` closed among them: first every edge
/// but the feet, in the order `edges` names them, its top corners closed by
/// #699's triangles and each upright's flat stopping flush on the lower
/// block's top; then the feet, in that order, and the feet of the flats laid
/// on the foot corners' uprights, in the order of `corners`. With it, the
/// number each edge's flat and each corner's patch answers to there.
fn chamfered_round_the_block_in_two_steps(
    edges: &[[usize; 2]],
    corners: &[OfTheBlock],
    value: f64,
) -> (Body, Vec<usize>, Vec<usize>) {
    let part = stacked();
    let start = part.faces_end();
    let is_foot = |edge: usize| edges[edge][0] == TOP;
    let upper: Vec<usize> = (0..edges.len()).filter(|&edge| !is_foot(edge)).collect();
    let feet: Vec<usize> = (0..edges.len()).filter(|&edge| is_foot(edge)).collect();
    let names = |body: &Body, ranks: &[usize]| -> Vec<EdgeName> {
        (ranks.iter())
            .map(|&edge| edge_between(body, edges[edge][0], edges[edge][1]))
            .collect()
    };
    let first = part
        .chamfer_edges(&names(&part, &upper), value)
        .unwrap_or_else(|declined| panic!("the block's top half is chamfered: {declined:?}"));
    taken_as_asked(first.asked, first.taken, value, "the block's top half");
    let halfway = first.body;
    let at_first = |edge: usize| upper.iter().position(|&upper| upper == edge);
    let tops: Vec<[usize; 3]> = (corners.iter())
        .filter(|corner| !corner.foot)
        .map(|corner| {
            corner
                .edges
                .map(|edge| at_first(edge).expect("a top corner's edges"))
        })
        .collect();
    let first_case = Case {
        order: (0..upper.len()).collect(),
        cut: Cut::Flat,
        value,
        step: Laid {
            body: halfway.clone(),
            asked: value,
            taken: value,
        },
        what: String::new(),
    };
    let triangles = first_case.patches(&part, &tops);
    let foot_corners: Vec<&OfTheBlock> = corners.iter().filter(|corner| corner.foot).collect();
    let flats: Vec<usize> = (foot_corners.iter())
        .map(|corner| start + at_first(corner.edges[2]).expect("an upright is laid first"))
        .collect();
    let second: Vec<EdgeName> = (names(&halfway, &feet).into_iter())
        .chain(flats.iter().map(|&flat| edge_between(&halfway, TOP, flat)))
        .collect();
    let then = halfway.faces_end();
    let filled = halfway
        .chamfer_edges(&second, value)
        .unwrap_or_else(|declined| panic!("the feet round the chamfered uprights: {declined:?}"));
    taken_as_asked(
        filled.asked,
        filled.taken,
        value,
        "the feet round the uprights",
    );
    let laid: Vec<usize> = (0..edges.len())
        .map(|edge| match at_first(edge) {
            Some(rank) => start + rank,
            None => then + feet.iter().position(|&foot| foot == edge).expect("a foot"),
        })
        .collect();
    let mut tops_laid = triangles.into_iter();
    let mut feet_laid = (0..foot_corners.len()).map(|place| then + feet.len() + place);
    let patches: Vec<usize> = (corners.iter())
        .map(|corner| {
            if corner.foot {
                feet_laid.next().expect("a fill for each foot corner")
            } else {
                tops_laid.next().expect("a triangle for each top corner")
            }
        })
        .collect();
    (filled.body, laid, patches)
}

/// Holds that the edges `edges` of [`stacked`]'s standing block, named in each
/// of `orders` and blended by each cut at each of [`VALUES`] in one step,
/// leave what [`closed_round_the_block`] holds, the corners `corners` closed
/// and the part keeping what `kept` says for the cut and the value; chamfered,
/// the body [`chamfered_round_the_block_in_two_steps`] builds, face for face,
/// whose matter `kept` gives too; and the same body whatever the order. Every
/// list is laid before any body is read.
fn closed_round_the_block_in_every_order(
    edges: &[[usize; 2]],
    corners: &[OfTheBlock],
    orders: &[Vec<usize>],
    kept: impl Fn(Cut, f64) -> f64,
    called: &str,
) {
    let part = stacked();
    let names: Vec<EdgeName> = edges
        .iter()
        .map(|&[first, second]| edge_between(&part, first, second))
        .collect();
    let in_two_steps = VALUES.map(|value| {
        let (body, laid, patches) = chamfered_round_the_block_in_two_steps(edges, corners, value);
        let promised = kept(Cut::Flat, value);
        assert!(
            (body.volume() - promised).abs() <= EXACT * promised,
            "{called}, the chamfer of {value} in two steps, the block's top half then its feet: \
             the body holds {}, where arithmetic promised {promised}",
            body.volume()
        );
        (body, laid, patches)
    });
    let mut laid = Vec::new();
    let mut declined = Vec::new();
    for cut in [Cut::Flat, Cut::Round] {
        for value in VALUES {
            for order in orders {
                let what = format!(
                    "the {} of {value} on {called}, named {order:?}",
                    cut.called()
                );
                let list: Vec<EdgeName> = order.iter().map(|&edge| names[edge].clone()).collect();
                match cut.laid_on(&part, &list, value) {
                    Ok(step) => laid.push(Case {
                        order: order.clone(),
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
        "{called}: every corner closed in one step, the top corners as #699 closes them, the \
         foot corners as the first slice goes round them; declined:\n{}",
        declined.join("\n")
    );
    let by_edges: Vec<[usize; 3]> = corners.iter().map(|corner| corner.edges).collect();
    for case in &laid {
        closed_round_the_block(&part, edges, corners, case, kept(case.cut, case.value));
        if case.cut == Cut::Flat {
            let at = VALUES
                .iter()
                .position(|&value| value == case.value)
                .expect("every case is laid at one of the values");
            let (two_steps, in_two, patches_in_two) = &in_two_steps[at];
            let to_two_steps: Vec<(usize, usize)> = (0..edges.len())
                .map(|edge| (case.laid_on(&part, edge), in_two[edge]))
                .chain(
                    (case.patches(&part, &by_edges).into_iter())
                        .zip(patches_in_two.iter().copied()),
                )
                .collect();
            the_same_bodies(
                two_steps,
                &case.step.body,
                &to_two_steps,
                &format!(
                    "{}, and the body two steps leave, the block's top half then its feet",
                    case.what
                ),
            );
        }
    }
    for case in &laid {
        let first = laid
            .iter()
            .find(|other| other.cut == case.cut && other.value == case.value)
            .expect("the case itself is among them");
        if std::ptr::eq(first, case) {
            continue;
        }
        let [mine, theirs] = [case, first].map(|one| one.patches(&part, &by_edges));
        let to_first: Vec<(usize, usize)> = (0..edges.len())
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

/// What a blend of `value` gives back at a convex corner #699 closes, its
/// three edges' sections each counted along the whole of its edge: `2d³/3`
/// among flats, `r³(2 − 7π/12)` among rounds.
fn given_back_at_a_convex_corner(cut: Cut, value: f64) -> f64 {
    match cut {
        Cut::Flat => 2.0 * value.powi(3) / 3.0,
        Cut::Round => value.powi(3) * (2.0 - 7.0 * PI / 12.0),
    }
}

/// The section a blend of `value` takes along a square edge.
fn section_of(cut: Cut, value: f64) -> f64 {
    match cut {
        Cut::Flat => value * value / 2.0,
        Cut::Round => value * value * (1.0 - PI / 4.0),
    }
}

#[test]
fn a_chamfer_and_a_fillet_on_a_foot_corner_and_the_top_corner_of_its_upright_close_both_in_one_step()
 {
    let corner = stacked_s_front_right();
    closed_round_the_block_in_every_order(
        &BOTH_ENDS_OF_AN_UPRIGHT,
        &[
            OfTheBlock {
                edges: [0, 1, 2],
                foot: true,
                under: 0,
            },
            OfTheBlock {
                edges: [3, 4, 2],
                foot: false,
                under: 0,
            },
        ],
        &[
            vec![0, 1, 2, 3, 4],
            vec![4, 3, 2, 1, 0],
            vec![2, 3, 0, 4, 1],
        ],
        |cut, value| {
            let at_the_foot = match cut {
                Cut::Flat => corner.chamfered(value),
                Cut::Round => corner.filleted(value),
            };
            at_the_foot - 20.0 * section_of(cut, value) + given_back_at_a_convex_corner(cut, value)
        },
        "both feet, the front right upright and the two top edges at its top",
    );
}

/// The twelve edges of [`stacked`]'s standing block, by the faces each lies
/// between, in the order the list as given names them: its four top edges,
/// front, right, back and left; its four uprights, front right, back right,
/// back left and front left; and its four feet, front, right, back and left.
const TWELVE_EDGES: [[usize; 2]; 12] = [
    [STANDING_TOP, STANDING_FRONT],
    [STANDING_TOP, STANDING_RIGHT],
    [STANDING_TOP, STANDING_BACK],
    [STANDING_TOP, STANDING_LEFT],
    [STANDING_FRONT, STANDING_RIGHT],
    [STANDING_RIGHT, STANDING_BACK],
    [STANDING_BACK, STANDING_LEFT],
    [STANDING_LEFT, STANDING_FRONT],
    [TOP, STANDING_FRONT],
    [TOP, STANDING_RIGHT],
    [TOP, STANDING_BACK],
    [TOP, STANDING_LEFT],
];

#[test]
fn a_chamfer_and_a_fillet_on_the_twelve_edges_of_a_standing_block_close_its_top_corners_as_a_convex_corner_and_its_foot_corners_round_their_uprights()
 {
    let tops = [[0, 1, 4], [1, 2, 5], [2, 3, 6], [3, 0, 7]];
    let feet = [[8, 9, 4], [9, 10, 5], [10, 11, 6], [11, 8, 7]];
    let corners: Vec<OfTheBlock> = (0..4)
        .map(|under| OfTheBlock {
            edges: tops[under],
            foot: false,
            under,
        })
        .chain((0..4).map(|under| OfTheBlock {
            edges: feet[under],
            foot: true,
            under,
        }))
        .collect();
    closed_round_the_block_in_every_order(
        &TWELVE_EDGES,
        &corners,
        &[
            (0..12).collect(),
            (0..12).rev().collect(),
            vec![8, 0, 4, 9, 1, 5, 10, 2, 6, 11, 3, 7],
        ],
        |cut, value| {
            let round_the_feet = match cut {
                Cut::Flat => chamfered_all_round(value),
                Cut::Round => filleted_all_round(value),
            };
            round_the_feet - 40.0 * section_of(cut, value)
                + 4.0 * given_back_at_a_convex_corner(cut, value)
        },
        "the twelve edges of the standing block",
    );
}

/// [`stacked`] with a plate 30 by 20 by 10 laid on its standing block, from
/// 20 up: a post between two plates, the plate below's top numbered one, the
/// post's walls, front, right, back and left, eight to eleven, and the plate
/// above's floor twelve. The post's uprights meet outwards, and both their
/// ends are foot corners: on the plate below, its feet there meeting inwards,
/// and under the plate above, its feet there too.
fn post_between_two_plates() -> Body {
    joined(
        &stacked(),
        Contour::rectangle(DVec2::ZERO, DVec2::new(30.0, 20.0)),
        LOWER_TOP + STANDING_HEIGHT,
        10.0,
    )
}

#[test]
fn a_fillet_and_a_chamfer_on_a_post_s_upright_and_the_feet_at_both_its_ends_are_declined_as_unsupported_until_847_builds_them()
 {
    let part = post_between_two_plates();
    let ceiling = 12;
    let five = [
        [TOP, STANDING_FRONT],
        [TOP, STANDING_RIGHT],
        [STANDING_FRONT, STANDING_RIGHT],
        [ceiling, STANDING_FRONT],
        [ceiling, STANDING_RIGHT],
    ]
    .map(|[first, second]| edge_between(&part, first, second));
    for order in [[0, 1, 2, 3, 4], [4, 3, 2, 1, 0], [2, 3, 0, 4, 1]] {
        let list: Vec<EdgeName> = order.iter().map(|&edge| five[edge].clone()).collect();
        for cut in [Cut::Flat, Cut::Round] {
            assert_eq!(
                cut.laid_on(&part, &list, 1.0).err(),
                Some(Declined::Unsupported),
                "the {} of 1 on the post's front right upright and the feet at both its ends, \
                 named {order:?}: a broken step and its reason, and no body, until #847 holds \
                 what an upright odd at both its ends makes",
                cut.called()
            );
        }
    }
}

#[test]
fn a_fillet_and_a_chamfer_on_two_foot_corners_and_the_top_corner_of_one_upright_are_declined_as_unsupported_where_the_other_upright_s_top_meets_a_top_edge_alone()
 {
    let part = stacked();
    let seven = [
        [TOP, STANDING_FRONT],
        [TOP, STANDING_RIGHT],
        [TOP, STANDING_LEFT],
        [STANDING_FRONT, STANDING_RIGHT],
        [STANDING_LEFT, STANDING_FRONT],
        [STANDING_TOP, STANDING_FRONT],
        [STANDING_TOP, STANDING_LEFT],
    ]
    .map(|[first, second]| edge_between(&part, first, second));
    for order in [
        [0, 1, 2, 3, 4, 5, 6],
        [6, 5, 4, 3, 2, 1, 0],
        [5, 3, 0, 6, 2, 4, 1],
    ] {
        let list: Vec<EdgeName> = order.iter().map(|&edge| seven[edge].clone()).collect();
        for cut in [Cut::Flat, Cut::Round] {
            assert_eq!(
                cut.laid_on(&part, &list, 1.0).err(),
                Some(Declined::Unsupported),
                "the {} of 1 on the standing block's two front foot corners and the top corner \
                 of its front left upright, named {order:?}: the front right upright's top meets \
                 the top front edge, which closes the front left top corner but no corner with \
                 that upright, and the step is declined",
                cut.called()
            );
        }
    }
}

/// A plate 30 by 30 by 10, its top numbered one, with a triangular pyramid
/// standing on it: its base the equilateral triangle of side 10 from (0, 0)
/// along X, on the plate's top, its apex 8 above the base's centre. Its three
/// slanted faces, front, right and left, are numbered eleven, seventeen and
/// twenty-three, as the planes that cut them from a prism number them. Its
/// feet, where they meet the plate's top, meet inwards, its slanted edges
/// outwards: at each foot corner the slanted edge above two feet is the odd
/// one, and the three slanted edges meet at the apex, a convex corner.
fn pyramid_on_a_plate() -> Body {
    let plate = joined(
        &Body::default(),
        Contour::rectangle(DVec2::splat(-10.0), DVec2::splat(20.0)),
        0.0,
        10.0,
    );
    standing_on(&plate, PYRAMID_BASE.map(|[x, y]| DVec2::new(x, y)))
}

/// `plate` with a triangular pyramid standing on its top: its base the
/// triangle `base`, its corners counter-clockwise, its apex
/// [`PYRAMID_HEIGHT`] above the base's centre. A prism raised over the base
/// is cut by the planes through each side of the base and the apex, their
/// faces numbered six apart past the prism's, then joined to the plate.
fn standing_on(plate: &Body, base: [DVec2; 3]) -> Body {
    let centre = (base[0] + base[1] + base[2]) / 3.0;
    let apex = DVec3::new(centre.x, centre.y, LOWER_TOP + PYRAMID_HEIGHT);
    let prism = tool_raised(plate, Contour::straight(base.to_vec()), 5.0, 20.0);
    let pyramid = (0..3).fold(prism, |part, side| {
        let [from, to] = [base[side], base[(side + 1) % 3]];
        let along = (to - from).normalize().extend(0.0);
        let at = from.extend(LOWER_TOP);
        let out = DVec3::new(along.y, -along.x, 0.0);
        let leaning = (apex - at).cross(along).normalize();
        let away = if leaning.dot(out) > 0.0 {
            leaning
        } else {
            -leaning
        };
        part.difference(&beyond(&part, at, away, 60.0))
            .expect("a plane through the apex cuts the prism")
    });
    plate
        .union(&pyramid)
        .expect("the pyramid stands on the plate")
}

/// The corners of the base of [`pyramid_on_a_plate`], on the plate's top.
const PYRAMID_BASE: [[f64; 2]; 3] = [[0.0, 0.0], [10.0, 0.0], [5.0, 8.660254037844386]];

/// How high the apex of [`pyramid_on_a_plate`] stands over the plate's top.
const PYRAMID_HEIGHT: f64 = 8.0;

/// The numbers of [`pyramid_on_a_plate`]'s slanted faces, front, right and
/// left.
const PYRAMID_FACES: [usize; 3] = [11, 17, 23];

/// The six edges of [`pyramid_on_a_plate`] a list names, by the faces each
/// lies between: its three slanted edges, front right, right left and left
/// front, then its three feet, front, right and left.
const PYRAMID_EDGES: [[usize; 2]; 6] = [
    [11, 17],
    [17, 23],
    [23, 11],
    [TOP, 11],
    [TOP, 17],
    [TOP, 23],
];

/// The four corners of [`pyramid_on_a_plate`] three of [`PYRAMID_EDGES`] meet
/// at, each by its three edges' places: the apex, then the foot of each
/// slanted edge in turn, with the two feet beside it.
const PYRAMID_CORNERS: [[usize; 3]; 4] = [[0, 1, 2], [0, 3, 4], [1, 4, 5], [2, 5, 3]];

/// The orders the six edges are named in: the slanted edges first, the feet
/// first, each slanted edge followed by a foot, and reversed.
const PYRAMID_ORDERS: [[usize; 6]; 4] = [
    [0, 1, 2, 3, 4, 5],
    [3, 4, 5, 0, 1, 2],
    [0, 3, 1, 4, 2, 5],
    [5, 4, 3, 2, 1, 0],
];

/// [`pyramid_on_a_plate`] chamfered at `value` in two steps: its three
/// slanted edges first, their flats numbered twenty-nine to thirty-one and
/// the apex closed by a triangle numbered thirty-two; then its three feet and
/// the feet of those three flats, numbered thirty-three to thirty-eight in
/// that order.
fn pyramid_chamfered_in_two_steps(value: f64) -> Body {
    let part = pyramid_on_a_plate();
    let slanted: Vec<EdgeName> = PYRAMID_EDGES[..3]
        .iter()
        .map(|&[first, second]| edge_between(&part, first, second))
        .collect();
    let first = part
        .chamfer_edges(&slanted, value)
        .unwrap_or_else(|declined| panic!("the three slanted edges are chamfered: {declined:?}"));
    taken_as_asked(first.asked, first.taken, value, "the three slanted edges");
    let flats = part.faces_end();
    let chamfered = first.body;
    let feet: Vec<EdgeName> = PYRAMID_EDGES[3..]
        .iter()
        .map(|&[first, second]| edge_between(&chamfered, first, second))
        .chain((0..3).map(|flat| edge_between(&chamfered, TOP, flats + flat)))
        .collect();
    let filled = chamfered
        .chamfer_edges(&feet, value)
        .unwrap_or_else(|declined| panic!("the six feet are filled: {declined:?}"));
    taken_as_asked(filled.asked, filled.taken, value, "the six feet");
    filled.body
}

#[test]
fn a_chamfer_on_a_triangular_pyramid_s_slanted_edges_and_feet_numbers_each_face_by_its_rank_the_apex_s_triangle_among_the_patches()
 {
    let part = pyramid_on_a_plate();
    let start = part.faces_end();
    let tetrahedron = 100.0 * 3f64.sqrt() / 4.0 * PYRAMID_HEIGHT / 3.0;
    assert!(
        (part.volume() - (9000.0 + tetrahedron)).abs() <= EXACT * part.volume(),
        "the pyramid on its plate holds {}",
        part.volume()
    );
    let facing = PYRAMID_FACES.map(|face| {
        part.plane_of(face)
            .unwrap_or_else(|| panic!("the pyramid's slanted face {face} is flat"))
            .normal
    });
    let slanted =
        [[0, 1], [1, 2], [2, 0]].map(|[one, other]| (facing[one] + facing[other]).normalize());
    let edges_facing: Vec<DVec3> = (slanted.iter().copied())
        .chain(facing.map(|way| (DVec3::Z + way).normalize()))
        .collect();
    let patches_facing: Vec<DVec3> = std::iter::once(DVec3::Z)
        .chain(slanted.map(|flat| (DVec3::Z + flat).normalize()))
        .collect();
    let names = PYRAMID_EDGES.map(|[first, second]| edge_between(&part, first, second));
    for value in [0.5, 1.0] {
        let two_steps = pyramid_chamfered_in_two_steps(value);
        for order in PYRAMID_ORDERS {
            let what =
                format!("the chamfer of {value} on the pyramid's six edges, named {order:?}");
            let list: Vec<EdgeName> = order.iter().map(|&edge| names[edge].clone()).collect();
            let step = part
                .chamfer_edges(&list, value)
                .unwrap_or_else(|declined| panic!("{what}: declined, {declined:?}"));
            let case = Case {
                order: order.to_vec(),
                cut: Cut::Flat,
                value,
                step: Laid {
                    body: step.body,
                    asked: step.asked,
                    taken: step.taken,
                },
                what,
            };
            let Case { step, what, .. } = &case;
            let body = &step.body;
            taken_as_asked(step.asked, step.taken, value, what);
            sound_drawn_within(body, two_steps.volume(), EXACT * body.volume(), what);
            assert_eq!(
                faces_made(&part, body),
                (start..start + 10).collect::<Vec<_>>(),
                "{what}: one flat for each edge, and a patch at each of the four corners: {:?}",
                numbers(body)
            );
            assert_eq!(
                body.faces_end(),
                start + 10,
                "{what}: the next step numbers its own faces past the four patches"
            );
            let patches = case.patches(&part, &PYRAMID_CORNERS);
            for (edge, way) in edges_facing.iter().enumerate() {
                faces_toward(
                    body,
                    case.laid_on(&part, edge),
                    *way,
                    &format!(
                        "{what}: the flat on {:?}, numbered by its rank",
                        PYRAMID_EDGES[edge]
                    ),
                );
            }
            for (corner, way) in patches_facing.iter().enumerate() {
                faces_toward(
                    body,
                    patches[corner],
                    *way,
                    &format!(
                        "{what}: the patch at the corner of {:?}, numbered after every edge's \
                         flat by the lowest ranks of its edges",
                        PYRAMID_CORNERS[corner].map(|edge| PYRAMID_EDGES[edge])
                    ),
                );
            }
            let to_two_steps: Vec<(usize, usize)> = (0..6)
                .map(|edge| {
                    let in_two_steps = match edge {
                        slanted @ 0..3 => start + slanted,
                        foot => start + 4 + foot - 3,
                    };
                    (case.laid_on(&part, edge), in_two_steps)
                })
                .chain((0..4).map(|corner| {
                    (
                        patches[corner],
                        if corner == 0 {
                            start + 3
                        } else {
                            start + 6 + corner
                        },
                    )
                }))
                .collect();
            the_same_bodies(
                &two_steps,
                body,
                &to_two_steps,
                &format!(
                    "{what}, and the body two steps leave, the slanted edges chamfered then the six \
                     feet"
                ),
            );
        }
    }
}

/// [`stacked`] with a second block, 2.5 by 4 by 5, standing on the lower
/// block's top to the right of the first, from (27, 5) to (29.5, 9): its
/// front wall on the plane of the first's, so that its front foot lies on the
/// line the first block's front foot runs along, and far enough that no fill
/// of the first's front right foot corner reaches it at 2.5. Its faces are
/// numbered twelve to seventeen; it holds 7050.
fn stacked_beside_a_block_on_its_front_line() -> Body {
    joined(
        &stacked(),
        Contour::rectangle(DVec2::new(27.0, 5.0), DVec2::new(29.5, 9.0)),
        LOWER_TOP,
        5.0,
    )
}

/// [`stacked_beside_a_block_on_its_front_line`]'s front right foot corner,
/// as [`stacked_s_front_right`] reads it.
fn front_right_beside_a_block_on_its_front_line() -> FootCorner {
    FootCorner {
        part: stacked_beside_a_block_on_its_front_line,
        volume: STACKED_VOLUME + 2.5 * 4.0 * 5.0,
        ..stacked_s_front_right()
    }
}

#[test]
fn a_chamfer_and_a_fillet_on_a_foot_corner_follow_each_foot_and_not_another_block_s_foot_on_the_same_line()
 {
    round_the_upright_in_every_order(
        &[front_right_beside_a_block_on_its_front_line()],
        &[Cut::Flat, Cut::Round],
        "the standing block's front right foot corner, a second block's front foot on the line \
         of its own",
    );
}

/// Nine edges of [`stacked`] named together, by the faces each lies between:
/// the standing block's back left foot corner — its back foot, its left foot
/// and the upright above them — then the convex corner at the top of its
/// front right upright — its top front and top right edges and that upright
/// — and the convex front right corner of the lower block's top — its top
/// front and top right edges and its upright there.
const A_FOOT_CORNER_AND_TWO_CONVEX_ONES: [[usize; 2]; 9] = [
    [TOP, STANDING_BACK],
    [TOP, STANDING_LEFT],
    [STANDING_BACK, STANDING_LEFT],
    [STANDING_TOP, STANDING_FRONT],
    [STANDING_TOP, STANDING_RIGHT],
    [STANDING_FRONT, STANDING_RIGHT],
    [TOP, 2],
    [TOP, 3],
    [2, 3],
];

/// The three corners of [`A_FOOT_CORNER_AND_TWO_CONVEX_ONES`], by their edges'
/// places, each with where it stands: the foot corner, the standing block's
/// top corner, the lower block's top corner.
const THE_THREE_CORNERS: [([usize; 3], [f64; 3]); 3] = [
    ([0, 1, 2], [10.0, 15.0, 10.0]),
    ([3, 4, 5], [20.0, 5.0, 20.0]),
    ([6, 7, 8], [30.0, 0.0, 10.0]),
];

/// Where the middle of each of [`A_FOOT_CORNER_AND_TWO_CONVEX_ONES`] stands
/// on [`stacked`].
const THE_NINE_MIDDLES: [[f64; 3]; 9] = [
    [15.0, 15.0, 10.0],
    [10.0, 10.0, 10.0],
    [10.0, 15.0, 15.0],
    [15.0, 5.0, 20.0],
    [20.0, 10.0, 20.0],
    [20.0, 5.0, 15.0],
    [15.0, 0.0, 10.0],
    [30.0, 10.0, 10.0],
    [30.0, 0.0, 5.0],
];

/// What a blend of `value` on a convex corner of three square edges, `along`
/// long in all and each ending flush on a square face at its far end, takes
/// from the part, as #699's arithmetic gives it: each edge's section along
/// its length, less the matter the three hold twice at the corner, which the
/// triangle or the piece of sphere gives back.
fn taken_at_a_convex_corner(cut: Cut, along: f64, value: f64) -> f64 {
    match cut {
        Cut::Flat => along * value * value / 2.0 - 2.0 * value.powi(3) / 3.0,
        Cut::Round => {
            along * value * value * (1.0 - PI / 4.0) - value.powi(3) * (2.0 - 7.0 * PI / 12.0)
        }
    }
}

/// Holds that the face numbered `face` lies about `point`: the middle of the
/// box its triangles fill within `within` of it.
fn lies_about(body: &Body, face: usize, point: DVec3, within: f64, what: &str) {
    let drawn: Vec<DVec3> = body.triangles_of(face).flatten().collect();
    assert!(!drawn.is_empty(), "{what}: face {face} is drawn");
    let low = drawn.iter().copied().reduce(DVec3::min).expect("drawn");
    let high = drawn.iter().copied().reduce(DVec3::max).expect("drawn");
    let middle = (low + high) / 2.0;
    assert!(
        middle.distance(point) <= within,
        "{what}: face {face} lies about {middle}, not within {within} of {point}"
    );
}

#[test]
fn a_chamfer_and_a_fillet_on_a_foot_corner_and_two_convex_corners_number_every_patch_after_the_edges_faces_by_their_ranks()
 {
    let part = stacked();
    let start = part.faces_end();
    let foot_corner = &stacked_s_foot_corners()[2];
    let names =
        A_FOOT_CORNER_AND_TWO_CONVEX_ONES.map(|[first, second]| edge_between(&part, first, second));
    let corners = THE_THREE_CORNERS.map(|(edges, _)| edges);
    let mut laid = Vec::new();
    for cut in [Cut::Flat, Cut::Round] {
        for value in VALUES {
            for order in [
                [0, 1, 2, 3, 4, 5, 6, 7, 8],
                [8, 7, 6, 5, 4, 3, 2, 1, 0],
                [6, 0, 3, 7, 1, 4, 8, 2, 5],
            ] {
                let what = format!(
                    "the {} of {value} on the back left foot corner and two convex corners, \
                     named {order:?}",
                    cut.called()
                );
                let list: Vec<EdgeName> = order.iter().map(|&edge| names[edge].clone()).collect();
                let step = cut
                    .laid_on(&part, &list, value)
                    .unwrap_or_else(|declined| panic!("{what}: declined, {declined:?}"));
                laid.push(Case {
                    order: order.to_vec(),
                    cut,
                    value,
                    step,
                    what,
                });
            }
        }
    }
    for case in &laid {
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
        let at_the_foot = match cut {
            Cut::Flat => foot_corner.chamfered(value),
            Cut::Round => foot_corner.filleted(value),
        };
        let promised = at_the_foot
            - taken_at_a_convex_corner(cut, 30.0, value)
            - taken_at_a_convex_corner(cut, 60.0, value);
        let drawn = match cut {
            Cut::Flat => EXACT * promised,
            Cut::Round => {
                foot_corner.drawn_within(&part, value) + DRAWN * reach_of(&part) * PI * value * 90.0
            }
        };
        sound_drawn_within(body, promised, drawn, what);
        assert_eq!(
            faces_made(&part, body),
            (start..start + 12).collect::<Vec<_>>(),
            "{what}: one face for each edge, and a patch at each of the three corners: {:?}",
            numbers(body)
        );
        assert_eq!(
            body.faces_end(),
            start + 12,
            "{what}: the next step numbers its own faces past the three patches"
        );
        let patches = case.patches(&part, &corners);
        for (edge, middle) in THE_NINE_MIDDLES.iter().enumerate() {
            lies_about(
                body,
                case.laid_on(&part, edge),
                DVec3::from(*middle),
                2.0 * value,
                &format!(
                    "{what}: the face on {:?}, numbered by its rank",
                    A_FOOT_CORNER_AND_TWO_CONVEX_ONES[edge]
                ),
            );
        }
        for (corner, (_, at)) in THE_THREE_CORNERS.iter().enumerate() {
            lies_about(
                body,
                patches[corner],
                DVec3::from(*at),
                2.0 * value,
                &format!(
                    "{what}: the patch at {at:?}, numbered after every edge's face by the lowest \
                     ranks of its edges"
                ),
            );
        }
        match cut {
            Cut::Flat => {
                let (facing, foot_facing) = foot_corner.flats_facing();
                for (edge, way) in facing.into_iter().enumerate() {
                    faces_toward(
                        body,
                        case.laid_on(&part, edge),
                        way,
                        &format!("{what}: the flat at the foot corner"),
                    );
                }
                faces_toward(
                    body,
                    patches[0],
                    foot_facing,
                    &format!("{what}: the fill along the foot of the upright's flat"),
                );
                for (corner, way) in [
                    (1, DVec3::new(1.0, -1.0, 1.0)),
                    (2, DVec3::new(1.0, -1.0, 1.0)),
                ] {
                    faces_toward(
                        body,
                        patches[corner],
                        way.normalize(),
                        &format!("{what}: the triangle closing a convex corner"),
                    );
                }
            }
            Cut::Round => {
                for (edge, round) in foot_corner.rounds(value).iter().enumerate() {
                    round_held(body, case.laid_on(&part, edge), round, &[patches[0]], what);
                }
                let laid: Vec<usize> = (0..3).map(|edge| case.laid_on(&part, edge)).collect();
                piece_held(body, patches[0], &foot_corner.torus(value), &laid, what);
            }
        }
    }
    for case in &laid {
        let first = laid
            .iter()
            .find(|other| other.cut == case.cut && other.value == case.value)
            .expect("the case itself is among them");
        if std::ptr::eq(first, case) {
            continue;
        }
        let [mine, theirs] = [case, first].map(|one| one.patches(&part, &corners));
        let to_first: Vec<(usize, usize)> = (0..9)
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

/// [`pyramid_on_a_plate`]'s plate grown to 50 by 35, from (−10, −15), with a
/// second pyramid standing on it beside the first, its base pointing away
/// from it, from (20, −2) through (25, −10.66) to (30, −2): set in the angle
/// past the first's front right foot corner between the lines of its two
/// feet there, so that no foot's line reaches the other's feet. Its slanted
/// faces are numbered thirty-four, forty and forty-six.
fn two_pyramids_on_a_plate() -> Body {
    let plate = joined(
        &Body::default(),
        Contour::rectangle(DVec2::new(-10.0, -15.0), DVec2::new(40.0, 20.0)),
        0.0,
        10.0,
    );
    let one = standing_on(&plate, PYRAMID_BASE.map(|[x, y]| DVec2::new(x, y)));
    standing_on(&one, SECOND_PYRAMID_BASE.map(|[x, y]| DVec2::new(x, y)))
}

/// The corners of the base of the second pyramid of
/// [`two_pyramids_on_a_plate`], counter-clockwise.
const SECOND_PYRAMID_BASE: [[f64; 2]; 3] =
    [[20.0, -2.0], [25.0, -10.660254037844386], [30.0, -2.0]];

#[test]
fn a_chamfer_on_two_triangular_pyramids_slanted_edges_and_feet_numbers_both_apexes_triangles_among_the_patches_by_their_ranks()
 {
    let part = two_pyramids_on_a_plate();
    let start = part.faces_end();
    let lone = pyramid_on_a_plate();
    let pyramids = [(PYRAMID_BASE, 11), (SECOND_PYRAMID_BASE, 34)];
    let mut edges: Vec<[usize; 2]> = Vec::new();
    let mut middles: Vec<DVec3> = Vec::new();
    let mut corners: Vec<([usize; 3], DVec3)> = Vec::new();
    for (base, first) in pyramids {
        let base = base.map(|[x, y]| DVec3::new(x, y, LOWER_TOP));
        let apex = (base[0] + base[1] + base[2]) / 3.0 + DVec3::Z * PYRAMID_HEIGHT;
        let face = |side: usize| first + 6 * (side % 3);
        let from = edges.len();
        for side in 0..3 {
            edges.push([face(side), face(side + 1)]);
            middles.push((base[(side + 1) % 3] + apex) / 2.0);
        }
        for side in 0..3 {
            edges.push([TOP, face(side)]);
            middles.push((base[side] + base[(side + 1) % 3]) / 2.0);
        }
        corners.push(([from, from + 1, from + 2], apex));
        for side in 0..3 {
            corners.push((
                [from + side, from + 3 + side, from + 3 + (side + 1) % 3],
                base[(side + 1) % 3],
            ));
        }
    }
    let names: Vec<EdgeName> = edges
        .iter()
        .map(|&[first, second]| edge_between(&part, first, second))
        .collect();
    let lone_edges: Vec<EdgeName> = PYRAMID_EDGES
        .iter()
        .map(|&[first, second]| edge_between(&lone, first, second))
        .collect();
    let by_edges: Vec<[usize; 3]> = corners.iter().map(|&(edges, _)| edges).collect();
    for value in [0.5, 1.0] {
        let one_alone = lone
            .chamfer_edges(&lone_edges, value)
            .unwrap_or_else(|declined| panic!("one pyramid alone: declined, {declined:?}"));
        let promised = part.volume() + 2.0 * (one_alone.body.volume() - lone.volume());
        let forward: Vec<usize> = (0..12).collect();
        let backward: Vec<usize> = (0..12).rev().collect();
        let apexes_last: Vec<usize> = [3, 4, 5, 9, 10, 11, 0, 6, 1, 7, 2, 8].to_vec();
        for order in [forward, backward, apexes_last] {
            let what =
                format!("the chamfer of {value} on both pyramids' twelve edges, named {order:?}");
            let list: Vec<EdgeName> = order.iter().map(|&edge| names[edge].clone()).collect();
            let step = part
                .chamfer_edges(&list, value)
                .unwrap_or_else(|declined| panic!("{what}: declined, {declined:?}"));
            let case = Case {
                order,
                cut: Cut::Flat,
                value,
                step: Laid {
                    body: step.body,
                    asked: step.asked,
                    taken: step.taken,
                },
                what,
            };
            let Case { step, what, .. } = &case;
            let body = &step.body;
            taken_as_asked(step.asked, step.taken, value, what);
            sound_drawn_within(body, promised, EXACT * promised, what);
            assert_eq!(
                faces_made(&part, body),
                (start..start + 20).collect::<Vec<_>>(),
                "{what}: one flat for each edge, and a patch at each of the eight corners: {:?}",
                numbers(body)
            );
            let patches = case.patches(&part, &by_edges);
            for (edge, middle) in middles.iter().enumerate() {
                lies_about(
                    body,
                    case.laid_on(&part, edge),
                    *middle,
                    2.0 * value,
                    &format!(
                        "{what}: the flat on {:?}, numbered by its rank",
                        edges[edge]
                    ),
                );
            }
            for (corner, &(_, at)) in corners.iter().enumerate() {
                lies_about(
                    body,
                    patches[corner],
                    at,
                    2.0 * value,
                    &format!(
                        "{what}: the patch at {at}, numbered after every edge's flat by the lowest \
                         ranks of its edges"
                    ),
                );
            }
        }
    }
}

/// The lower block of [`stacked`]'s outline edges a foot corner is named
/// with, each list by the faces its edges lie between, how long they are in
/// all, and whether three of them close a corner: its top right edge, beyond
/// the right foot; its top front edge, beyond the front foot; and the convex
/// corner of its top at the front right, its two top edges there and its
/// upright, all three beyond the foot corner's feet.
const OUTLINE_BEYOND_THE_FEET: [(&[[usize; 2]], f64, bool, &str); 3] = [
    (&[[TOP, 3]], 20.0, false, "the lower block's top right edge"),
    (&[[TOP, 2]], 30.0, false, "the lower block's top front edge"),
    (
        &[[TOP, 2], [TOP, 3], [2, 3]],
        60.0,
        true,
        "the lower block's front right top corner",
    ),
];

/// Where the middle of each of [`stacked`]'s edges a list in
/// [`OUTLINE_BEYOND_THE_FEET`] names stands, by the faces it lies between.
fn middle_of(edge: [usize; 2]) -> DVec3 {
    match edge {
        [TOP, 3] => DVec3::new(30.0, 10.0, 10.0),
        [TOP, 2] => DVec3::new(15.0, 0.0, 10.0),
        [2, 3] => DVec3::new(30.0, 0.0, 5.0),
        [TOP, STANDING_FRONT] => DVec3::new(15.0, 5.0, 10.0),
        [TOP, STANDING_RIGHT] => DVec3::new(20.0, 10.0, 10.0),
        [STANDING_FRONT, STANDING_RIGHT] => DVec3::new(20.0, 5.0, 15.0),
        _ => panic!("no middle known for the edge between {edge:?}"),
    }
}

#[test]
fn a_chamfer_and_a_fillet_on_a_foot_corner_and_edges_of_the_lower_block_s_outline_beyond_its_feet_take_the_value_asked()
 {
    let part = stacked();
    let start = part.faces_end();
    let corner = stacked_s_front_right();
    let corner_edges = corner.edges().map(|(_, between)| between);
    let mut declined = Vec::new();
    let mut laid = Vec::new();
    for (outline, along, closes, called) in OUTLINE_BEYOND_THE_FEET {
        let edges: Vec<[usize; 2]> = corner_edges.iter().chain(outline).copied().collect();
        let count = edges.len();
        let orders: Vec<Vec<usize>> = vec![
            (0..count).collect(),
            (0..count).rev().collect(),
            (3..count).chain(0..3).collect(),
        ];
        let names: Vec<EdgeName> = edges
            .iter()
            .map(|&[first, second]| edge_between(&part, first, second))
            .collect();
        for cut in [Cut::Flat, Cut::Round] {
            for value in [1.0, 2.0] {
                for order in &orders {
                    let what = format!(
                        "the {} of {value} on the front right foot corner and {called}, named \
                         {order:?}",
                        cut.called()
                    );
                    let list: Vec<EdgeName> =
                        order.iter().map(|&edge| names[edge].clone()).collect();
                    match cut.laid_on(&part, &list, value) {
                        Ok(step) => laid.push((
                            edges.clone(),
                            along,
                            closes,
                            Case {
                                order: order.clone(),
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
        "a foot corner named with edges of the lower block's outline beyond its feet, each laid \
         as alone; declined:\n{}",
        declined.join("\n")
    );
    for (edges, along, closes, case) in &laid {
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
        let at_the_corner = match cut {
            Cut::Flat => corner.chamfered(value),
            Cut::Round => corner.filleted(value),
        };
        let outline = if *closes {
            taken_at_a_convex_corner(cut, *along, value)
        } else {
            along
                * match cut {
                    Cut::Flat => value * value / 2.0,
                    Cut::Round => value * value * (1.0 - PI / 4.0),
                }
        };
        let promised = at_the_corner - outline;
        let drawn = match cut {
            Cut::Flat => EXACT * promised,
            Cut::Round => {
                corner.drawn_within(&part, value) + DRAWN * reach_of(&part) * PI * value * along
            }
        };
        sound_drawn_within(body, promised, drawn, what);
        let patches: usize = if *closes { 2 } else { 1 };
        assert_eq!(
            faces_made(&part, body),
            (start..start + edges.len() + patches).collect::<Vec<_>>(),
            "{what}: one face for each edge, and the patches after them: {:?}",
            numbers(body)
        );
        let corners: Vec<[usize; 3]> = if *closes {
            vec![[0, 1, 2], [3, 4, 5]]
        } else {
            vec![[0, 1, 2]]
        };
        let numbered = case.patches(&part, &corners);
        for (edge, &between) in edges.iter().enumerate() {
            lies_about(
                body,
                case.laid_on(&part, edge),
                middle_of(between),
                2.0 * value,
                &format!("{what}: the face on {between:?}, numbered by its rank"),
            );
        }
        lies_about(
            body,
            numbered[0],
            corner.at,
            2.0 * value,
            &format!("{what}: the foot corner's patch"),
        );
        if *closes {
            lies_about(
                body,
                numbered[1],
                DVec3::new(30.0, 0.0, 10.0),
                2.0 * value,
                &format!("{what}: the patch closing the lower block's top corner"),
            );
        }
        match cut {
            Cut::Flat => {
                let (facing, foot_facing) = corner.flats_facing();
                for (edge, way) in facing.into_iter().enumerate() {
                    faces_toward(
                        body,
                        case.laid_on(&part, edge),
                        way,
                        &format!("{what}: the flat at the foot corner"),
                    );
                }
                faces_toward(
                    body,
                    numbered[0],
                    foot_facing,
                    &format!("{what}: the fill along the foot of the upright's flat"),
                );
            }
            Cut::Round => {
                for (edge, round) in corner.rounds(value).iter().enumerate() {
                    round_held(body, case.laid_on(&part, edge), round, &[numbered[0]], what);
                }
                let rounds: Vec<usize> = (0..3).map(|edge| case.laid_on(&part, edge)).collect();
                piece_held(body, numbered[0], &corner.torus(value), &rounds, what);
            }
        }
    }
}

/// [`stacked`] with a second block, 4 by 4 by 5, standing on the lower
/// block's top 5 to the right of the first, from (25, 5) to (29, 9): no
/// blend of the first's front right foot corner reaches it at 2.5, though the
/// line of the fill along the foot of that upright's flat, read on past its
/// mitre with the right foot, reaches the block's back left corner, (25, 9),
/// at `3.5/√2`. Its faces are numbered twelve to seventeen; it holds 7080.
fn stacked_beside_a_block_past_the_flat_s_mitre() -> Body {
    joined(
        &stacked(),
        Contour::rectangle(DVec2::new(25.0, 5.0), DVec2::new(29.0, 9.0)),
        LOWER_TOP,
        5.0,
    )
}

#[test]
fn a_chamfer_and_a_fillet_on_a_foot_corner_beside_a_block_its_flat_s_foot_reaches_only_past_its_mitre_take_the_value_asked()
 {
    round_the_upright_in_every_order(
        &[FootCorner {
            called: "the standing block's front right foot corner beside a second block",
            part: stacked_beside_a_block_past_the_flat_s_mitre,
            volume: STACKED_VOLUME + 4.0 * 4.0 * 5.0,
            ..stacked_s_front_right()
        }],
        &[Cut::Flat, Cut::Round],
        "the front right foot corner beside a block no blend of it reaches",
    );
}

/// What stands on [`stacked`]'s lower block in front of the standing block,
/// in the sweep of its front foot's fill, raised 2 from (`from`) to (`to`),
/// how much it holds, the value at which that fill first reaches it, and
/// what to call it.
struct InTheSweep {
    from: [f64; 2],
    to: [f64; 2],
    holds: f64,
    reached: f64,
    called: &'static str,
}

/// The two things [`InTheSweep`] reads: a small block whose back corners
/// stand inside the sweep, 1 in front of the foot; and a low wall in front of
/// the whole standing block, its back 1.5 in front of the foot, its four
/// corners past the planes of the left and right walls, which trim the
/// fill at its two ends.
const IN_THE_SWEEP: [InTheSweep; 2] = [
    InTheSweep {
        from: [13.0, 3.0],
        to: [15.0, 4.0],
        holds: 2.0 * 1.0 * 2.0,
        reached: 1.0,
        called: "a small block standing 1 in front of the front foot",
    },
    InTheSweep {
        from: [5.0, 3.0],
        to: [25.0, 3.5],
        holds: 20.0 * 0.5 * 2.0,
        reached: 1.5,
        called: "a low wall standing 1.5 in front of the front foot, its corners past both its ends",
    },
];

#[test]
fn a_chamfer_and_a_fillet_on_a_foot_alone_and_on_its_corner_stop_where_their_fill_reaches_what_stands_in_its_sweep()
 {
    let asked = 2.5;
    let hair = 20.0 * 1e-9 * 30.0;
    let corner = stacked_s_front_right();
    let mut wrong = Vec::new();
    for InTheSweep {
        from,
        to,
        holds,
        reached,
        called,
    } in IN_THE_SWEEP
    {
        let part = joined(
            &stacked(),
            Contour::rectangle(DVec2::from(from), DVec2::from(to)),
            LOWER_TOP,
            2.0,
        );
        let volume = STACKED_VOLUME + holds;
        let foot = [[TOP, STANDING_FRONT]];
        let corner_edges = corner.edges().map(|(_, between)| between);
        for (edges, cut, alone) in [
            (&foot[..], Cut::Flat, true),
            (&foot[..], Cut::Round, true),
            (&corner_edges[..], Cut::Flat, false),
        ] {
            let what = format!(
                "the {} of {asked} on the front foot{}, beside {called}",
                cut.called(),
                if alone { " alone" } else { "'s corner" }
            );
            let names: Vec<EdgeName> = edges
                .iter()
                .map(|&[first, second]| edge_between(&part, first, second))
                .collect();
            let step = match cut.laid_on(&part, &names, asked) {
                Ok(step) => step,
                Err(reason) => {
                    wrong.push(format!("{what}: declined, {reason:?}"));
                    continue;
                }
            };
            if (step.asked - asked).abs() > PLACE
                || step.taken > reached + PLACE
                || step.taken < reached - 10.0 * hair
            {
                wrong.push(format!(
                    "{what}: asked {} and took {}, where the fill first reaches it at {reached}",
                    step.asked, step.taken
                ));
                continue;
            }
            let taken = step.taken;
            let promised = volume
                + match (cut, alone) {
                    (Cut::Flat, true) => 10.0 * taken * taken / 2.0,
                    (Cut::Round, true) => 10.0 * taken * taken * (1.0 - PI / 4.0),
                    _ => corner.chamfered(taken) - STACKED_VOLUME,
                };
            let drawn = match cut {
                Cut::Flat => EXACT * promised,
                Cut::Round => DRAWN * reach_of(&part) * PI / 2.0 * taken * 12.0,
            };
            sound_drawn_within(&step.body, promised, drawn, &what);
        }
    }
    assert!(
        wrong.is_empty(),
        "a fill stops where it first reaches what stands in its sweep, whether that thing's \
         corners stand in the sweep or past its ends:\n{}",
        wrong.join("\n")
    );
}

/// A block 1 high standing on [`stacked`]'s lower block diagonally off the
/// standing block's front right foot corner, from (`from`) to (`to`): where
/// the corner's patch sweeps — the piece of torus, out to twice the radius
/// from the upright round's axis, and the fill along the foot of the
/// upright's flat — but no foot's fill reaches. Its corner nearest the
/// standing block's stands `gap` off both walls, and its faces are numbered
/// twelve to seventeen, its floor gone into the lower block's top.
struct OffTheCorner {
    from: [f64; 2],
    to: [f64; 2],
    gap: f64,
    below: f64,
    called: &'static str,
}

impl OffTheCorner {
    /// The value at which the corner's patch first touches the block, at
    /// its corner nearest the standing block's, on the lower block's top:
    /// filleted, where the piece of torus reaches twice the radius from the
    /// axis `r` in from both walls, `2r = √2·(r + gap)`; chamfered, where the
    /// fill along the flat's foot meets the lower block's top on
    /// `x − y = 15 + (√2 − 1)·d`, `15 + 2·gap` at that corner.
    fn touched(&self, cut: Cut) -> f64 {
        let root = 2f64.sqrt();
        match cut {
            Cut::Round => self.gap * root / (2.0 - root),
            Cut::Flat => 2.0 * self.gap / (root - 1.0),
        }
    }

    fn holds(&self) -> f64 {
        (self.to[0] - self.from[0]) * (self.to[1] - self.from[1])
    }
}

/// Two blocks off the corner: one the patch reaches but cannot hold whole,
/// and one small enough to stand inside its sweep; and for each a value short
/// of where either kind of patch touches it.
const OFF_THE_CORNER: [OffTheCorner; 2] = [
    OffTheCorner {
        from: [20.2, 4.2],
        to: [20.9, 4.8],
        gap: 0.2,
        below: 0.4,
        called: "a block 0.2 off both walls",
    },
    OffTheCorner {
        from: [20.1, 4.6],
        to: [20.4, 4.9],
        gap: 0.1,
        below: 0.2,
        called: "a small block 0.1 off both walls, inside the patch's sweep",
    },
];

#[test]
fn a_chamfer_and_a_fillet_on_a_foot_corner_never_take_away_a_block_standing_where_the_corner_s_patch_sweeps()
 {
    let corner = stacked_s_front_right();
    let hair = 20.0 * 1e-9 * 30.0;
    let mut wrong = Vec::new();
    for block in OFF_THE_CORNER {
        let part = joined(
            &stacked(),
            Contour::rectangle(DVec2::from(block.from), DVec2::from(block.to)),
            LOWER_TOP,
            1.0,
        );
        let start = part.faces_end();
        let walls: Vec<usize> = (start - 5..start).collect();
        let names = corner
            .edges()
            .map(|(_, [first, second])| edge_between(&part, first, second));
        for cut in [Cut::Flat, Cut::Round] {
            let touched = block.touched(cut);
            for asked in [block.below, 1.5, 2.5] {
                for order in [[0, 1, 2], [2, 1, 0], [2, 0, 1]] {
                    let what = format!(
                        "the {} of {asked} on the front right foot corner beside {}, named \
                         {order:?}, its patch touching the block at {touched}",
                        cut.called(),
                        block.called
                    );
                    let list: Vec<EdgeName> =
                        order.iter().map(|&edge| names[edge].clone()).collect();
                    let step = match cut.laid_on(&part, &list, asked) {
                        Ok(step) => step,
                        Err(_) if asked > touched => continue,
                        Err(reason) => {
                            wrong.push(format!("{what}: declined, {reason:?}"));
                            continue;
                        }
                    };
                    let body = &step.body;
                    let gone: Vec<usize> = (walls.iter().copied())
                        .filter(|&wall| !body.has_face(wall))
                        .collect();
                    if !gone.is_empty() {
                        wrong.push(format!(
                            "{what}: took {} and laid a body without the block, its faces \
                             {gone:?} gone, holding {}",
                            step.taken,
                            body.volume()
                        ));
                        continue;
                    }
                    let expected = if asked > touched { touched } else { asked };
                    if (step.asked - asked).abs() > PLACE
                        || step.taken > expected + PLACE
                        || step.taken < expected - 10.0 * hair
                    {
                        wrong.push(format!(
                            "{what}: asked {} and took {}, where the patch first touches the \
                             block at {touched}",
                            step.asked, step.taken
                        ));
                        continue;
                    }
                    let taken = step.taken;
                    let promised = match cut {
                        Cut::Flat => corner.chamfered(taken),
                        Cut::Round => corner.filleted(taken),
                    } + block.holds();
                    let drawn = match cut {
                        Cut::Flat => EXACT * promised,
                        Cut::Round => corner.drawn_within(&part, taken),
                    };
                    sound_drawn_within(body, promised, drawn, &what);
                    assert_eq!(
                        faces_made(&part, body),
                        (start..=start + 3).collect::<Vec<_>>(),
                        "{what}: one face for each edge, then the corner's patch"
                    );
                }
            }
        }
    }
    assert!(
        wrong.is_empty(),
        "a foot corner beside a block where its patch sweeps keeps the block whole, its value \
         reduced to where the patch first touches it, or declined:\n{}",
        wrong.join("\n")
    );
}

/// [`stacked`] with a low slanted wall, 1 high, standing on the lower
/// block's top in front of the standing block's front right foot: a
/// parallelogram whose back, the foot facing the standing block, runs from
/// (18, 2.5) to (22, 3.5), and whose front lies 0.5 in front of it. The front
/// foot's fill, trimmed at its ends on the planes of the left and right
/// walls, `x = 10` and `x = 20`, sweeps the lower block's top in front of it;
/// the wall's back enters that sweep through the plane `x = 20`, crossing it
/// at its own middle, 2 in front of the foot — between its two corners,
/// 2.5 in front at (18, 2.5), inside the sweep, and 1.5 at (22, 3.5), past
/// it. It holds 7002.
fn stacked_behind_a_slanted_wall() -> Body {
    joined(
        &stacked(),
        Contour::straight(vec![
            DVec2::new(18.0, 2.0),
            DVec2::new(22.0, 3.0),
            DVec2::new(22.0, 3.5),
            DVec2::new(18.0, 2.5),
        ]),
        LOWER_TOP,
        1.0,
    )
}

#[test]
fn a_chamfer_and_a_fillet_on_a_foot_alone_stop_where_a_slanted_wall_s_foot_enters_their_sweep_between_its_corners()
 {
    let part = stacked_behind_a_slanted_wall();
    let volume = STACKED_VOLUME + 4.0 * 0.5 * 1.0;
    let entered = 2.0;
    let hair = 20.0 * 1e-9 * 30.0;
    let foot = [edge_between(&part, TOP, STANDING_FRONT)];
    for cut in [Cut::Flat, Cut::Round] {
        for asked in [1.5, 2.5] {
            let what = format!(
                "the {} of {asked} on the front foot alone, beside a slanted wall entering its \
                 sweep {entered} in front of it",
                cut.called()
            );
            let step = cut
                .laid_on(&part, &foot, asked)
                .unwrap_or_else(|declined| panic!("{what}: declined, {declined:?}"));
            let expected = asked.min(entered);
            assert!(
                (step.asked - asked).abs() < PLACE
                    && step.taken <= expected + PLACE
                    && step.taken >= expected - 10.0 * hair,
                "{what}: asked {} and took {}, where the fill first reaches the wall at {entered}, \
                 where its foot crosses the plane the fill ends on, and not at either of its \
                 corners, 2.5 in front of the foot and 1.5",
                step.asked,
                step.taken
            );
            let taken = step.taken;
            let section = match cut {
                Cut::Flat => taken * taken / 2.0,
                Cut::Round => taken * taken * (1.0 - PI / 4.0),
            };
            let promised = volume + 10.0 * section;
            let drawn = match cut {
                Cut::Flat => EXACT * promised,
                Cut::Round => DRAWN * reach_of(&part) * PI / 2.0 * taken * 12.0,
            };
            sound_drawn_within(&step.body, promised, drawn, &what);
        }
    }
}

/// The L of #699's and #801's tests: an arm 30 long and 6 deep along the
/// front, and at its left end a stem 12 wide reaching back to 14, raised 10.
/// Its top is numbered one and its walls from the front round: the front two,
/// the arm's right end three, the arm's back four, the stem's right side
/// five, the stem's back six and the left seven. The arm's back and the
/// stem's right side meet inwards along the inner upright at (12, 6), 10
/// high; the top edges on them meet outwards above it. It holds 2760.
fn ell() -> Body {
    joined(
        &Body::default(),
        Contour::straight(vec![
            DVec2::new(0.0, 0.0),
            DVec2::new(30.0, 0.0),
            DVec2::new(30.0, 6.0),
            DVec2::new(12.0, 6.0),
            DVec2::new(12.0, 14.0),
            DVec2::new(0.0, 14.0),
        ]),
        0.0,
        STANDING_HEIGHT,
    )
}
const ELL_VOLUME: f64 = (30.0 * 6.0 + 12.0 * 8.0) * 10.0;
const FLOOR: usize = 0;
const ARM_END: usize = 3;
const ARM_BACK: usize = 4;
const STEM_RIGHT: usize = 5;
const STEM_BACK: usize = 6;

/// The top of [`ell`]'s inner corner, (12, 6, 10), square: its top edge on
/// the arm's back, running to the arm's right end, its top edge on the
/// stem's right side, running to the stem's back, and the inner upright
/// below them, the odd edge, filled first.
fn ell_s_inner_top_corner() -> FootCorner {
    FootCorner {
        called: "the top of the L's inner corner",
        part: ell,
        volume: ELL_VOLUME,
        at: DVec3::new(12.0, 6.0, STANDING_HEIGHT),
        feet: [
            Foot {
                called: "the L's top edge on the arm's back",
                wall: ARM_BACK,
                facing: DVec3::Y,
                along: DVec3::X,
                far: FarEnd {
                    on: ARM_END,
                    at: DVec3::new(30.0, 6.0, STANDING_HEIGHT),
                    facing: DVec3::X,
                },
            },
            Foot {
                called: "the L's top edge on the stem's right side",
                wall: STEM_RIGHT,
                facing: DVec3::X,
                along: DVec3::Y,
                far: FarEnd {
                    on: STEM_BACK,
                    at: DVec3::new(12.0, 14.0, STANDING_HEIGHT),
                    facing: DVec3::Y,
                },
            },
        ],
        upright: "the L's inner upright",
        odd_filled: true,
        odd_ends_on: FLOOR,
    }
}

#[test]
fn a_chamfer_on_the_two_top_edges_at_the_l_s_inner_corner_and_the_upright_below_them_cuts_the_top_of_the_upright_s_fill_and_mitres_the_three_flats_in_every_order()
 {
    round_the_upright_in_every_order(
        &[ell_s_inner_top_corner()],
        &[Cut::Flat],
        "the top of the L's inner corner chamfered",
    );
}

#[test]
fn a_fillet_on_the_two_top_edges_at_the_l_s_inner_corner_and_the_upright_below_them_joins_their_rounds_through_a_piece_of_torus_cut_out_in_every_order()
 {
    round_the_upright_in_every_order(
        &[ell_s_inner_top_corner()],
        &[Cut::Round],
        "the top of the L's inner corner filleted",
    );
}

/// [`ell`] with a block 1 high standing on its top diagonally behind its
/// inner corner, from (11.2, 5.2) to (11.7, 5.7): its corner nearest the
/// inner corner 0.3 behind the planes of both walls there, where the corner's
/// patch cuts the top — the piece of torus out to twice the radius from the
/// fill's axis, or the cut along the top of the fill's flat — but no top
/// edge's cut reaches. Its top and walls are numbered nine to thirteen, its
/// floor gone into the L's top.
fn ell_with_a_block_behind_its_inner_corner() -> Body {
    joined(
        &ell(),
        Contour::rectangle(DVec2::new(11.2, 5.2), DVec2::new(11.7, 5.7)),
        STANDING_HEIGHT,
        1.0,
    )
}

#[test]
fn a_chamfer_and_a_fillet_on_the_top_of_the_l_s_inner_corner_never_cut_a_block_standing_where_the_corner_s_patch_sweeps()
 {
    let corner = ell_s_inner_top_corner();
    let block = OffTheCorner {
        from: [11.2, 5.2],
        to: [11.7, 5.7],
        gap: 0.3,
        below: 0.5,
        called: "a block standing 0.3 behind both walls",
    };
    let hair = 20.0 * 1e-9 * 30.0;
    let part = ell_with_a_block_behind_its_inner_corner();
    let start = part.faces_end();
    let kept: Vec<usize> = (start - 5..start).collect();
    let names = corner
        .edges()
        .map(|(_, [first, second])| edge_between(&part, first, second));
    let mut wrong = Vec::new();
    for cut in [Cut::Flat, Cut::Round] {
        let touched = block.touched(cut);
        for asked in [block.below, 1.0, 2.5] {
            for order in [[0, 1, 2], [2, 1, 0], [2, 0, 1]] {
                let what = format!(
                    "the {} of {asked} on the top of the L's inner corner beside {}, named \
                     {order:?}, its patch reaching the block at {touched}",
                    cut.called(),
                    block.called
                );
                let list: Vec<EdgeName> = order.iter().map(|&edge| names[edge].clone()).collect();
                let step = match cut.laid_on(&part, &list, asked) {
                    Ok(step) => step,
                    Err(_) if asked > touched => continue,
                    Err(reason) => {
                        wrong.push(format!("{what}: declined, {reason:?}"));
                        continue;
                    }
                };
                let body = &step.body;
                let gone: Vec<usize> = (kept.iter().copied())
                    .filter(|&face| !body.has_face(face))
                    .collect();
                if !gone.is_empty() {
                    wrong.push(format!(
                        "{what}: took {} and laid a body that cut the block, its faces {gone:?} \
                         gone, holding {}",
                        step.taken,
                        body.volume()
                    ));
                    continue;
                }
                let expected = asked.min(touched);
                if (step.asked - asked).abs() > PLACE
                    || step.taken > expected + PLACE
                    || step.taken < expected - 10.0 * hair
                {
                    wrong.push(format!(
                        "{what}: asked {} and took {}, where the patch first reaches the block \
                         at {touched}",
                        step.asked, step.taken
                    ));
                    continue;
                }
                let taken = step.taken;
                let promised = match cut {
                    Cut::Flat => corner.chamfered(taken),
                    Cut::Round => corner.filleted(taken),
                } + block.holds();
                let drawn = match cut {
                    Cut::Flat => EXACT * promised,
                    Cut::Round => corner.drawn_within(&part, taken),
                };
                if (body.volume() - promised).abs() > EXACT * promised {
                    wrong.push(format!(
                        "{what}: took {taken} and holds {}, where the corner's blend and the \
                         block whole hold {promised}",
                        body.volume()
                    ));
                    continue;
                }
                sound_drawn_within(body, promised, drawn, &what);
                assert_eq!(
                    faces_made(&part, body),
                    (start..=start + 3).collect::<Vec<_>>(),
                    "{what}: one face for each edge, then the corner's patch"
                );
            }
        }
    }
    assert!(
        wrong.is_empty(),
        "the top of the L's inner corner beside a block where its patch cuts keeps the block \
         whole, its value reduced to where the patch first reaches it, or declined:\n{}",
        wrong.join("\n")
    );
}

/// The L's left top edge, by the faces it lies between: 14 long, from the
/// front to the stem's back, each square to it. The line of the arm's top
/// edge's set-back, carried on past the inner corner, would reach it.
const ELL_LEFT_TOP_EDGE: [usize; 2] = [TOP, 7];

#[test]
fn a_chamfer_and_a_fillet_on_the_top_of_the_l_s_inner_corner_and_its_left_top_edge_take_the_value_asked()
 {
    let part = ell();
    let start = part.faces_end();
    let corner = ell_s_inner_top_corner();
    let edges: Vec<[usize; 2]> = (corner.edges().map(|(_, between)| between).into_iter())
        .chain([ELL_LEFT_TOP_EDGE])
        .collect();
    let middles = [
        DVec3::new(21.0, 6.0, 10.0),
        DVec3::new(12.0, 10.0, 10.0),
        DVec3::new(12.0, 6.0, 5.0),
        DVec3::new(0.0, 7.0, 10.0),
    ];
    let names: Vec<EdgeName> = edges
        .iter()
        .map(|&[first, second]| edge_between(&part, first, second))
        .collect();
    let mut declined = Vec::new();
    let mut laid = Vec::new();
    for cut in [Cut::Flat, Cut::Round] {
        for value in VALUES {
            for order in [vec![0, 1, 2, 3], vec![3, 2, 1, 0], vec![3, 0, 1, 2]] {
                let what = format!(
                    "the {} of {value} on the top of the L's inner corner and its left top edge, \
                     named {order:?}",
                    cut.called()
                );
                let list: Vec<EdgeName> = order.iter().map(|&edge| names[edge].clone()).collect();
                match cut.laid_on(&part, &list, value) {
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
        "the top of the L's inner corner named with its left top edge, each laid as alone; \
         declined:\n{}",
        declined.join("\n")
    );
    for case in &laid {
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
        let (at_the_corner, section) = match cut {
            Cut::Flat => (corner.chamfered(value), value * value / 2.0),
            Cut::Round => (corner.filleted(value), value * value * (1.0 - PI / 4.0)),
        };
        let promised = at_the_corner - 14.0 * section;
        let drawn = match cut {
            Cut::Flat => EXACT * promised,
            Cut::Round => {
                corner.drawn_within(&part, value) + DRAWN * reach_of(&part) * PI * value * 14.0
            }
        };
        sound_drawn_within(body, promised, drawn, what);
        assert_eq!(
            faces_made(&part, body),
            (start..=start + 4).collect::<Vec<_>>(),
            "{what}: one face for each edge, then the corner's patch: {:?}",
            numbers(body)
        );
        let patch = case.patches(&part, &[[0, 1, 2]])[0];
        for (edge, middle) in middles.iter().enumerate() {
            lies_about(
                body,
                case.laid_on(&part, edge),
                *middle,
                2.0 * value,
                &format!(
                    "{what}: the face on {:?}, numbered by its rank",
                    edges[edge]
                ),
            );
        }
        lies_about(
            body,
            patch,
            corner.at,
            2.0 * value,
            &format!("{what}: the corner's patch"),
        );
        if cut == Cut::Round {
            for (edge, round) in corner.rounds(value).iter().enumerate() {
                round_held(body, case.laid_on(&part, edge), round, &[patch], what);
            }
            let rounds: Vec<usize> = (0..3).map(|edge| case.laid_on(&part, edge)).collect();
            piece_held(body, patch, &corner.torus(value), &rounds, what);
        }
    }
}

/// An L like [`ell`] whose stem's right side leans from the inner corner at
/// (12, 6) at `notch` from the arm's back, the air between the two `notch`
/// wide, up to the stem's back at 14; numbered as [`ell`] is.
fn ell_whose_notch_is(notch: f64) -> Body {
    let reach = 12.0 + 8.0 / notch.tan();
    joined(
        &Body::default(),
        Contour::straight(vec![
            DVec2::new(0.0, 0.0),
            DVec2::new(30.0, 0.0),
            DVec2::new(30.0, 6.0),
            DVec2::new(12.0, 6.0),
            DVec2::new(reach, 14.0),
            DVec2::new(0.0, 14.0),
        ]),
        0.0,
        STANDING_HEIGHT,
    )
}

/// The notch of [`ell_opening_wider`], obtuse: 105°.
const WIDER_NOTCH: f64 = 105.0 * PI / 180.0;

/// The notch of [`ell_closing_in`], acute: 75°.
const NARROWER_NOTCH: f64 = 75.0 * PI / 180.0;

/// The notch of [`ell_closing_in_further`], acute and below 60°: 50°.
const NARROWEST_NOTCH: f64 = 50.0 * PI / 180.0;

/// [`ell_whose_notch_is`] at [`WIDER_NOTCH`].
fn ell_opening_wider() -> Body {
    ell_whose_notch_is(WIDER_NOTCH)
}

/// [`ell_whose_notch_is`] at [`NARROWER_NOTCH`].
fn ell_closing_in() -> Body {
    ell_whose_notch_is(NARROWER_NOTCH)
}

/// [`ell_whose_notch_is`] at [`NARROWEST_NOTCH`].
fn ell_closing_in_further() -> Body {
    ell_whose_notch_is(NARROWEST_NOTCH)
}

/// What [`ell_whose_notch_is`] holds: the arm 30 by 6, and the stem between
/// the left at 0 and the leaning side, 12 wide at the arm's back and
/// `12 + 8·cot notch` at its own back, 8 deep, raised 10.
fn ell_volume_whose_notch_is(notch: f64) -> f64 {
    let reach = 12.0 + 8.0 / notch.tan();
    (30.0 * 6.0 + (12.0 + reach) / 2.0 * 8.0) * STANDING_HEIGHT
}

/// The top of the inner corner of [`ell_whose_notch_is`], (12, 6, 10): its
/// top edge on the arm's back, to the arm's right end, square to it; its top
/// edge on the leaning side, to the stem's back, which leans across it; and
/// the inner upright below them, the odd edge, filled first.
fn inner_top_corner_whose_notch_is(
    notch: f64,
    called: &'static str,
    part: fn() -> Body,
) -> FootCorner {
    let along = DVec3::new(notch.cos(), notch.sin(), 0.0);
    let facing = DVec3::new(notch.sin(), -notch.cos(), 0.0);
    FootCorner {
        called,
        part,
        volume: ell_volume_whose_notch_is(notch),
        at: DVec3::new(12.0, 6.0, STANDING_HEIGHT),
        feet: [
            Foot {
                called: "the L's top edge on the arm's back",
                wall: ARM_BACK,
                facing: DVec3::Y,
                along: DVec3::X,
                far: FarEnd {
                    on: ARM_END,
                    at: DVec3::new(30.0, 6.0, STANDING_HEIGHT),
                    facing: DVec3::X,
                },
            },
            Foot {
                called: "the L's top edge on the leaning side",
                wall: STEM_RIGHT,
                facing,
                along,
                far: FarEnd {
                    on: STEM_BACK,
                    at: DVec3::new(12.0 + 8.0 / notch.tan(), 14.0, STANDING_HEIGHT),
                    facing: DVec3::Y,
                },
            },
        ],
        upright: "the L's inner upright",
        odd_filled: true,
        odd_ends_on: FLOOR,
    }
}

#[test]
fn a_chamfer_and_a_fillet_on_the_top_of_an_inner_corner_that_is_not_square_go_round_its_upright_in_every_order()
 {
    round_the_upright_in_every_order_at(
        &[
            inner_top_corner_whose_notch_is(
                WIDER_NOTCH,
                "the top of an L's inner corner opening to 105°",
                ell_opening_wider,
            ),
            inner_top_corner_whose_notch_is(
                NARROWER_NOTCH,
                "the top of an L's inner corner closing in to 75°",
                ell_closing_in,
            ),
            inner_top_corner_whose_notch_is(
                NARROWEST_NOTCH,
                "the top of an L's inner corner closing in to 50°",
                ell_closing_in_further,
            ),
        ],
        &[Cut::Flat, Cut::Round],
        &[1.0, 2.0],
        "the top of an L's inner corner that is not square chamfered and filleted",
    );
}

/// The outline of the L of [`ell_standing_on_a_plate`], counter-clockwise from
/// its front left corner: an arm from (6, 3) to (24, 10) along the front and a
/// stem from (6, 10) to (14, 17) at its left end, its inner corner, (14, 10),
/// the fourth.
const ELL_BOSS: [[f64; 2]; 6] = [
    [6.0, 3.0],
    [24.0, 3.0],
    [24.0, 10.0],
    [14.0, 10.0],
    [14.0, 17.0],
    [6.0, 17.0],
];

/// The place of the L's inner corner in [`ELL_BOSS`], the one reflex corner.
const ELL_BOSS_INNER: usize = 3;

/// The number of the L boss's top; its walls follow it, from the front round:
/// the front, the arm's end, the arm's back, the stem's right side, the
/// stem's back and the left.
const ELL_BOSS_TOP: usize = 7;

/// [`stacked`]'s lower block, 30 by 20 by 10, with an L 10 high standing on its
/// top in place of the block, its outline [`ELL_BOSS`]: its top numbered seven
/// and its walls eight to thirteen, its floor gone into the lower block's
/// top. Its feet meet inwards and its top edges outwards; its uprights
/// outwards but at its inner corner, where the upright meets inwards. It holds
/// 6000 and the L's 182 by 10, 7820.
fn ell_standing_on_a_plate() -> Body {
    joined(
        &joined(
            &Body::default(),
            Contour::rectangle(DVec2::ZERO, DVec2::new(30.0, 20.0)),
            0.0,
            LOWER_TOP,
        ),
        Contour::straight(ELL_BOSS.map(|[x, y]| DVec2::new(x, y)).to_vec()),
        LOWER_TOP,
        STANDING_HEIGHT,
    )
}
const ELL_BOSS_VOLUME: f64 = 30.0 * 20.0 * 10.0 + (18.0 * 7.0 + 8.0 * 7.0) * 10.0;

/// An edge of the L boss: the two faces it lies between, its two ends, each
/// with the face it runs into there, and whether it meets outwards.
struct BossEdge {
    faces: [usize; 2],
    ends: [(DVec3, usize); 2],
    outwards: bool,
}

/// The L boss's corner `at` of [`ELL_BOSS`], at `height`, and the way its wall
/// from that corner to the next faces, out of the matter.
fn ell_boss_corner(at: usize, height: f64) -> DVec3 {
    let [x, y] = ELL_BOSS[at % 6];
    DVec3::new(x, y, height)
}
fn ell_boss_wall_facing(wall: usize) -> DVec3 {
    let run = ell_boss_corner(wall + 1, 0.0) - ell_boss_corner(wall, 0.0);
    DVec3::new(run.y, -run.x, 0.0).normalize()
}

/// The L boss's eighteen edges, in the order the list as given names them:
/// its six top edges, wall by wall from the front; its six uprights, corner by
/// corner from the front left; its six feet, wall by wall from the front.
fn ell_boss_edges() -> Vec<BossEdge> {
    let wall = |at: usize| ELL_BOSS_TOP + 1 + at % 6;
    let top = LOWER_TOP + STANDING_HEIGHT;
    let along = |height: f64, own: usize| {
        [
            (ell_boss_corner(own, height), wall(own + 5)),
            (ell_boss_corner(own + 1, height), wall(own + 1)),
        ]
    };
    (0..6)
        .map(|own| BossEdge {
            faces: [ELL_BOSS_TOP, wall(own)],
            ends: along(top, own),
            outwards: true,
        })
        .chain((0..6).map(|at| BossEdge {
            faces: [wall(at + 5), wall(at)],
            ends: [
                (ell_boss_corner(at, LOWER_TOP), TOP),
                (ell_boss_corner(at, top), ELL_BOSS_TOP),
            ],
            outwards: at != ELL_BOSS_INNER,
        }))
        .chain((0..6).map(|own| BossEdge {
            faces: [TOP, wall(own)],
            ends: along(LOWER_TOP, own),
            outwards: false,
        }))
        .collect()
}

/// The L boss's twelve corners, each by its three edges' places in
/// [`ell_boss_edges`] and where it stands: the six at the top, corner by
/// corner from the front left, then the six at the foot.
fn ell_boss_corners() -> Vec<([usize; 3], DVec3)> {
    let top = LOWER_TOP + STANDING_HEIGHT;
    (0..6)
        .map(|at| ([(at + 5) % 6, at, 6 + at], ell_boss_corner(at, top)))
        .chain((0..6).map(|at| {
            (
                [12 + (at + 5) % 6, 12 + at, 6 + at],
                ell_boss_corner(at, LOWER_TOP),
            )
        }))
        .collect()
}

/// The corner of the L boss at `at` of [`ELL_BOSS`], on the face its two like
/// edges share, as the first two slices read it — its two like edges running
/// from it to the corners before and after — read here only for the piece of
/// torus a fillet lays there.
fn ell_boss_mixed_corner(at: usize, height: f64, odd_filled: bool) -> FootCorner {
    let corner = ell_boss_corner(at, height);
    let like = |towards: usize, wall: usize| Foot {
        called: "an edge of the L boss",
        wall: ELL_BOSS_TOP + 1 + wall,
        facing: ell_boss_wall_facing(wall),
        along: (ell_boss_corner(towards, height) - corner).normalize(),
        far: FarEnd {
            on: TOP,
            at: corner,
            facing: DVec3::Z,
        },
    };
    FootCorner {
        called: "a corner of the L boss",
        part: ell_standing_on_a_plate,
        volume: ELL_BOSS_VOLUME,
        at: corner,
        feet: [like(at + 5, (at + 5) % 6), like(at + 1, at)],
        upright: "an upright of the L boss",
        odd_filled,
        odd_ends_on: TOP,
    }
}

/// What [`ell_standing_on_a_plate`] holds once every edge of its L is filleted
/// at `radius`, as the module's documentation derives it.
fn ell_boss_filleted(radius: f64) -> f64 {
    let r = radius;
    let section = r * r * (1.0 - PI / 4.0);
    ELL_BOSS_VOLUME - 40.0 * section - 8.0 * r * section
        + 4.0 * r.powi(3) * (2.0 - 7.0 * PI / 12.0)
        + 2.0 * PI * r.powi(3) * (11.0 / 6.0 - PI / 2.0)
}

#[test]
fn a_fillet_on_every_edge_of_an_l_shaped_block_standing_on_another_closes_each_corner_by_its_kind()
{
    let part = ell_standing_on_a_plate();
    assert!(
        (part.volume() - ELL_BOSS_VOLUME).abs() <= EXACT * ELL_BOSS_VOLUME,
        "the L standing on its plate holds {}",
        part.volume()
    );
    let start = part.faces_end();
    let edges = ell_boss_edges();
    let corners = ell_boss_corners();
    let by_edges: Vec<[usize; 3]> = corners.iter().map(|&(edges, _)| edges).collect();
    let names: Vec<EdgeName> = (edges.iter())
        .map(|edge| edge_between(&part, edge.faces[0], edge.faces[1]))
        .collect();
    let orders: [Vec<usize>; 3] = [
        (0..18).collect(),
        (0..18).rev().collect(),
        (0..6).flat_map(|at| [at, 6 + at, 12 + at]).collect(),
    ];
    let mut laid = Vec::new();
    let mut declined = Vec::new();
    for value in VALUES {
        for order in &orders {
            let what =
                format!("the fillet of {value} on every edge of the L boss, named {order:?}");
            let list: Vec<EdgeName> = order.iter().map(|&edge| names[edge].clone()).collect();
            match Cut::Round.laid_on(&part, &list, value) {
                Ok(step) => laid.push(Case {
                    order: order.clone(),
                    cut: Cut::Round,
                    value,
                    step,
                    what,
                }),
                Err(reason) => declined.push(format!("{what}: {reason:?}")),
            }
        }
    }
    assert!(
        declined.is_empty(),
        "every edge of an L standing on a plate filleted in one step, each corner closed by its \
         kind; declined:\n{}",
        declined.join("\n")
    );
    for case in &laid {
        let Case {
            value, step, what, ..
        } = case;
        let r = *value;
        taken_as_asked(step.asked, step.taken, r, what);
        let body = &step.body;
        let promised = ell_boss_filleted(r);
        let curved = PI / 2.0 * r * 10.0 * 18.0 + 12.0 * PI * r * r;
        sound_drawn_within(body, promised, DRAWN * reach_of(&part) * curved, what);
        assert_eq!(
            faces_made(&part, body),
            (start..start + 30).collect::<Vec<_>>(),
            "{what}: one round for each edge, and a patch at each of the twelve corners: {:?}",
            numbers(body)
        );
        assert_eq!(
            body.faces_end(),
            start + 30,
            "{what}: the next step numbers its own faces past the patches"
        );
        let patches = case.patches(&part, &by_edges);
        let patch_at = |point: DVec3| {
            (corners.iter().zip(&patches))
                .find(|((_, at), _)| at.distance(point) < PLACE)
                .map(|(_, &patch)| patch)
        };
        for (place, edge) in edges.iter().enumerate() {
            edge_round_held(
                body,
                case.laid_on(&part, place),
                (
                    edge.faces,
                    edge.faces.map(|face| match face {
                        TOP | ELL_BOSS_TOP => DVec3::Z,
                        wall => ell_boss_wall_facing(wall - ELL_BOSS_TOP - 1),
                    }),
                ),
                edge.ends,
                (edge.ends.map(|(at, _)| patch_at(at)), edge.outwards),
                r,
                &format!("{what}: the round on {:?}", edge.faces),
            );
        }
        for (place, &(three, at)) in corners.iter().enumerate() {
            let corner = place % 6;
            let on_top = place < 6;
            let patch = patches[place];
            let rounds: Vec<usize> = three
                .iter()
                .map(|&edge| case.laid_on(&part, edge))
                .collect();
            let [before, after] = [
                ell_boss_wall_facing((corner + 5) % 6),
                ell_boss_wall_facing(corner),
            ];
            let which = format!(
                "{what}: the patch at the {} of the L's corner {at}, face {patch}",
                if on_top { "top" } else { "foot" }
            );
            match (on_top, corner == ELL_BOSS_INNER) {
                (true, false) => sphere_held(
                    body,
                    patch,
                    (at - (DVec3::Z + before + after) * r, r, 1.0),
                    [DVec3::Z, before, after],
                    &rounds,
                    &which,
                ),
                (true, true) => piece_held(
                    body,
                    patch,
                    &Torus {
                        shared: ELL_BOSS_TOP,
                        ..ell_boss_mixed_corner(corner, at.z, true).torus(r)
                    },
                    &rounds,
                    &which,
                ),
                (false, false) => piece_held(
                    body,
                    patch,
                    &ell_boss_mixed_corner(corner, at.z, false).torus(r),
                    &rounds,
                    &which,
                ),
                (false, true) => sphere_held(
                    body,
                    patch,
                    (at + (DVec3::Z + before + after) * r, r, -1.0),
                    [-DVec3::Z, -before, -after],
                    &rounds,
                    &which,
                ),
            }
        }
    }
    for case in &laid {
        let first = laid
            .iter()
            .find(|other| other.value == case.value)
            .expect("the case itself is among them");
        if std::ptr::eq(first, case) {
            continue;
        }
        let [mine, theirs] = [case, first].map(|one| one.patches(&part, &by_edges));
        let to_first: Vec<(usize, usize)> = (0..edges.len())
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

#[test]
fn a_chamfer_on_every_edge_of_an_l_shaped_block_standing_on_another_is_declined_as_unsupported_until_867_builds_it()
 {
    let part = ell_standing_on_a_plate();
    let edges = ell_boss_edges();
    let names: Vec<EdgeName> = (edges.iter())
        .map(|edge| edge_between(&part, edge.faces[0], edge.faces[1]))
        .collect();
    let wall = |at: usize| ELL_BOSS_TOP + 1 + at;
    let fewest: Vec<EdgeName> = [
        [wall(2), wall(3)],
        [ELL_BOSS_TOP, wall(2)],
        [ELL_BOSS_TOP, wall(3)],
        [wall(1), wall(2)],
        [TOP, wall(1)],
        [TOP, wall(2)],
        [ELL_BOSS_TOP, wall(1)],
    ]
    .map(|[first, second]| edge_between(&part, first, second))
    .to_vec();
    let every: Vec<EdgeName> = names.clone();
    for (called, list) in [
        ("every edge of the L boss", every),
        (
            "the L boss's inner upright, the top edges at its top, the arm end's back upright, \
             its two feet and the arm end's top edge",
            fewest,
        ),
    ] {
        let reversed: Vec<EdgeName> = list.iter().rev().cloned().collect();
        for (order, named) in [("as given", list), ("reversed", reversed)] {
            assert_eq!(
                Cut::Flat.laid_on(&part, &named, 1.0).err(),
                Some(Declined::Unsupported),
                "the chamfer of 1 on {called}, named {order}: two layings cannot lay a top edge \
                 both before the inner upright and after it, and the step is declined"
            );
        }
    }
}
