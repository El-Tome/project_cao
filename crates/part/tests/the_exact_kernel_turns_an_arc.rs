//! Closes #576.
//! - a sphere, a zone of one, a ring torus and a rounded shaft end, turned
//!   whole and part way, hold the arithmetic's volume (Pappus), exactly —
//!   `a_ball_turned_whole_and_part_way_holds_the_arithmetics_volume` (4πr³/3,
//!   and a lune of it),
//!   `a_zone_of_a_sphere_turned_whole_and_part_way_holds_the_arithmetics_volume`
//!   (π[r²(h_b − h_a) − (h_b³ − h_a³)/3]),
//!   `a_ring_torus_turned_whole_and_part_way_holds_the_arithmetics_volume`
//!   (2π²Rr²),
//!   `a_rounded_shaft_end_turned_whole_and_part_way_holds_the_arithmetics_volume`
//!   (πr²L + 2πr³/3, the cap tangent to the shaft)
//! - a plane square to the axis or holding it, and a coaxial cylinder or
//!   cone, joined to or cut from a sphere or a torus, are built exactly —
//!   `a_ball_cut_by_a_plane_square_to_its_axis_holds_the_arithmetics_volume`
//!   (a cap of height H: πH²(3r − H)/3),
//!   `a_ball_cut_by_a_block_through_its_axis_holds_the_arithmetics_volume`
//!   (the block's side a plane holding the axis, the cut a meridian),
//!   `a_coaxial_bore_through_a_ball_holds_the_arithmetics_volume`
//!   (two circles, the rest a zone less a cylinder),
//!   `a_coaxial_shaft_joined_to_a_ball_at_its_equator_holds_the_arithmetics_volume`
//!   (the shaft's wall touching the ball along one circle, decided once),
//!   `a_coaxial_cone_cut_from_a_ring_torus_holds_the_arithmetics_volume`
//!   (the cone meeting the tube along two circles),
//!   `a_round_bottomed_groove_cut_into_a_shaft_holds_the_arithmetics_volume`
//!   (a torus taking matter away, tangent to the shaft's wall)
//! - a sphere or a torus against anything else is a broken step with its
//!   reason —
//!   `a_hole_drilled_off_axis_through_a_ball_is_a_broken_step_with_its_reason`,
//!   `a_plane_at_a_slant_through_a_ball_is_a_broken_step_with_its_reason`
//!   (`Declined::Unsupported` read through `declined_because`, the part as
//!   before the step, and still exact)
//! - every face keeps the number the flats give it today, so a drawing laid
//!   on a face keeps it —
//!   `a_ball_knob_and_a_rounded_shaft_number_their_faces_as_the_flats_do`
//!   (the faces made counted from the floor against the same step on a part
//!   already on the flats, whole and part way); every rounded leaf of the
//!   harness's draw is held to the flats' numbers in `cao_solid` by
//!   `a_rounded_leaf_names_its_faces_as_the_flats_name_them`
//! - #448's harness draws arcs turned, before the kernel code: whole and part
//!   way, alone and against coaxial and other walls, weighted towards hairs
//!   and tangencies — no test: held in `cao_solid` by
//!   `random_turned_solids.rs` (`Case::drawn_rounded`,
//!   `Case::drawn_rounded_off_the_lattice`,
//!   `the_rounded_generator_draws_every_kind_it_keeps`,
//!   `the_draws_before_rounds_still_give_each_seed_its_case`)
//! - campaigns end with the new draws under one failure in a thousand on
//!   fresh seeds, no seed of the other draws that held before failing; a
//!   family that does not move in two rounds is reported and set aside — no
//!   test: counts and families in `docs/exact-kernel-failures.md` ("#576:
//!   spheres and tori"), the campaigns behind `--features campaigns`; the
//!   draw's fast cases are in the gate,
//!   `random_rounded_cases_keep_every_rule_on_the_exact_kernel` and
//!   `rounded_cases_the_exact_kernel_keeps_are_kept_through_the_application_s_body_and_stay_exact`
//! - `REBUILT_BY` moves, and `docs/exact-kernel.md` and `docs/glossary.md`
//!   no longer send an arc turned to the flats — no test: held beside the
//!   cache by `a_cache_written_before_arcs_turned_were_exact_is_replayed`,
//!   since only there can a test tell a part opened on its cache from one
//!   replayed; the docs are prose, held by `language.rs` only
//! - a turn of a profile holding an ellipse stays on the flats, and a part
//!   already on the flats turns an arc on the flats (what must not break) —
//!   `a_part_already_on_the_flats_turns_an_arc_on_the_flats`; the ellipse
//!   turned is held in `the_exact_kernel_raises_an_ellipse.rs` by its own
//!   bullet
//!
//! A part holding an arc turned, as the application builds it: the sphere or
//! the torus is the exact kernel's, and so is every step after it. The
//! kernel's own tests are `cao_solid`'s, beside the code in `brep/` and in
//! `random_turned_solids.rs`.
//!
//! Written at the opening of #576, before the kernel code: every test here is
//! ignored until the flip lifts it.

const NOT_YET: &str = "#576: the exact kernel does not turn an arc yet";

#[test]
#[ignore = "#576: the exact kernel does not turn an arc yet"]
fn a_ball_turned_whole_and_part_way_holds_the_arithmetics_volume() {
    unimplemented!("{NOT_YET}");
}

#[test]
#[ignore = "#576: the exact kernel does not turn an arc yet"]
fn a_zone_of_a_sphere_turned_whole_and_part_way_holds_the_arithmetics_volume() {
    unimplemented!("{NOT_YET}");
}

#[test]
#[ignore = "#576: the exact kernel does not turn an arc yet"]
fn a_ring_torus_turned_whole_and_part_way_holds_the_arithmetics_volume() {
    unimplemented!("{NOT_YET}");
}

#[test]
#[ignore = "#576: the exact kernel does not turn an arc yet"]
fn a_rounded_shaft_end_turned_whole_and_part_way_holds_the_arithmetics_volume() {
    unimplemented!("{NOT_YET}");
}

#[test]
#[ignore = "#576: the exact kernel does not turn an arc yet"]
fn a_ball_cut_by_a_plane_square_to_its_axis_holds_the_arithmetics_volume() {
    unimplemented!("{NOT_YET}");
}

#[test]
#[ignore = "#576: the exact kernel does not turn an arc yet"]
fn a_ball_cut_by_a_block_through_its_axis_holds_the_arithmetics_volume() {
    unimplemented!("{NOT_YET}");
}

#[test]
#[ignore = "#576: the exact kernel does not turn an arc yet"]
fn a_coaxial_bore_through_a_ball_holds_the_arithmetics_volume() {
    unimplemented!("{NOT_YET}");
}

#[test]
#[ignore = "#576: the exact kernel does not turn an arc yet"]
fn a_coaxial_shaft_joined_to_a_ball_at_its_equator_holds_the_arithmetics_volume() {
    unimplemented!("{NOT_YET}");
}

#[test]
#[ignore = "#576: the exact kernel does not turn an arc yet"]
fn a_coaxial_cone_cut_from_a_ring_torus_holds_the_arithmetics_volume() {
    unimplemented!("{NOT_YET}");
}

#[test]
#[ignore = "#576: the exact kernel does not turn an arc yet"]
fn a_round_bottomed_groove_cut_into_a_shaft_holds_the_arithmetics_volume() {
    unimplemented!("{NOT_YET}");
}

#[test]
#[ignore = "#576: the exact kernel does not turn an arc yet"]
fn a_hole_drilled_off_axis_through_a_ball_is_a_broken_step_with_its_reason() {
    unimplemented!("{NOT_YET}");
}

#[test]
#[ignore = "#576: the exact kernel does not turn an arc yet"]
fn a_plane_at_a_slant_through_a_ball_is_a_broken_step_with_its_reason() {
    unimplemented!("{NOT_YET}");
}

#[test]
#[ignore = "#576: the exact kernel does not turn an arc yet"]
fn a_ball_knob_and_a_rounded_shaft_number_their_faces_as_the_flats_do() {
    unimplemented!("{NOT_YET}");
}

#[test]
#[ignore = "#576: the exact kernel does not turn an arc yet"]
fn a_part_already_on_the_flats_turns_an_arc_on_the_flats() {
    unimplemented!("{NOT_YET}");
}
