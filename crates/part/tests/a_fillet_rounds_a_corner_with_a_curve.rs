//! A fillet rounds a corner one of whose sides is a curve: a trait and an arc,
//! two arcs, an arc of ellipse beside any of those. As every cut since #617, it
//! is a property of the corner, tangent to both sides and worked out from them
//! each time the drawing is read, and a gesture on the corner is a gesture on
//! a sharp corner.
//!
//! Closes #621.
//! - A fillet rounds the corner between a trait and an arc, and between two
//!   arcs, tangent to both —
//!   `a_fillet_rounds_the_corner_between_a_trait_and_an_arc_tangent_to_both`,
//!   `a_fillet_rounds_the_corner_between_two_arcs_tangent_to_both`; the tool
//!   naming such a corner by its point or by its sides is held in `cao_app` by
//!   `a_click_on_the_point_a_trait_and_an_arc_end_at_names_both_its_sides` and
//!   `a_click_on_an_arc_away_from_its_ends_names_that_side`
//! - A fillet rounds the corner between an arc of ellipse and a trait, an arc,
//!   or another arc of ellipse, tangent to both —
//!   `a_fillet_rounds_the_corner_between_an_arc_of_ellipse_and_a_trait_tangent_to_both`,
//!   `a_fillet_rounds_the_corner_between_an_arc_of_ellipse_and_an_arc_tangent_to_both`,
//!   `a_fillet_rounds_the_corner_between_two_arcs_of_ellipse_tangent_to_both`
//! - The fillet stands in the angle under 180° between the sides' tangents at
//!   the corner, and changes side only when a drag takes that angle through
//!   180° —
//!   `a_fillet_beside_an_arc_stands_in_the_angle_under_half_a_turn_between_the_tangents`,
//!   `a_fillet_beside_an_arc_changes_side_only_when_a_drag_takes_the_angle_through_half_a_turn`
//! - A chamfer on a corner with an arc or an arc of ellipse is refused, and a
//!   message says why. A whole circle and a whole ellipse stay refused —
//!   `a_chamfer_on_a_corner_with_an_arc_or_an_arc_of_ellipse_is_refused`,
//!   `a_whole_circle_or_a_whole_ellipse_has_no_corner_to_round`; the message
//!   is the tool's, held in `cao_app` by
//!   `a_chamfer_on_a_corner_with_a_curve_says_why_it_is_refused`
//! - Dragging the corner, or grabbing the fillet, moves the corner as on a
//!   sharp corner: an arc keeps its centre and radius and the corner slides
//!   round it; between two arcs the corner follows the hand. The fillet
//!   follows, tangent and of the same radius —
//!   `dragging_a_corner_between_a_trait_and_an_arc_slides_it_round_the_arc_as_on_a_sharp_corner`,
//!   `dragging_a_corner_between_two_arcs_moves_it_as_a_sharp_corner`; the
//!   fillet grabbed is held in the sketch crate by
//!   `a_press_on_a_fillet_beside_an_arc_drags_its_corner`
//! - The visible outline runs along each side up to its tangent point, a curve
//!   trimmed there, then the fillet; the tint, the areas and the extrusion
//!   follow it —
//!   `a_plate_rounded_beside_an_arc_closes_its_area_along_the_trimmed_arc_and_the_fillet`,
//!   `a_plate_rounded_beside_an_arc_is_raised_with_its_rounded_corner`; the
//!   tint is painted from those same areas, and nothing under the canvas
//!   asserts what is painted
//! - A value too big is held back with the largest that fits, and a fillet
//!   that no longer fits during a drag turns red with the sharp corner
//!   standing in, as in #617 —
//!   `a_fillet_beside_an_arc_that_no_longer_fits_raises_the_sharp_corner`;
//!   holding the value back is the tool's, held in `cao_app` by
//!   `a_fillet_too_big_beside_an_arc_is_held_back_and_the_field_offers_the_largest`,
//!   the share it offers by
//!   `the_tightest_side_beside_an_arc_says_how_much_of_the_value_fits` in the
//!   sketch crate; the red is the canvas's
//!
//! What the design adds so that the tools around a cut do not break on a
//! curve side — not in the issue's words:
//! - erasing the arc a fillet stands on takes the fillet —
//!   `erasing_the_arc_side_of_a_rounded_corner_takes_its_cut`
//! - a division of an arc side keeps the cut on the piece touching the corner —
//!   `dividing_the_arc_side_of_a_rounded_corner_keeps_its_cut`
//! - a mirror copies a fillet standing beside an arc —
//!   `a_corner_rounded_beside_an_arc_mirrored_is_rounded_on_its_copy`
//! - compacting the history keeps a fillet beside an arc, its sides followed
//!   through what they became — `a_compacted_part_keeps_a_fillet_beside_an_arc`
