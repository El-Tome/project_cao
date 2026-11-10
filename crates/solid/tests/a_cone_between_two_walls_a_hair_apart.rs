//! A cone face between two walls a hair apart, or between a face and a bore
//! tangent to it, is drawn closed (#810, part of #755).
//!
//! The face's two edges, where the cone cuts two walls within a chord's sag
//! of each other, were each sampled on their own angles; a sample one edge
//! had and the other lacked stood above the other's chord, the two chords
//! crossed in the cone's chart, and the face was left uncut: `Open` on the
//! exact kernel, `Undrawn` through the application's body. Two shapes, one
//! mechanism: a shaft and a bore exactly parallel a hair apart, and a block
//! face tangent to a bore.
//!
//! The pass is held beside its code, in
//! `crates/solid/src/brep/tessellation/sampling/rulings/tests.rs`. The
//! mutants of `rulings.rs` those tests leave alive are equivalent:
//! - `67:28` `>` → `>=`: a cone face with one curved edge has no other edge
//!   to read, and takes nothing either way.
//! - `80:55` `||` → `&&`: an edge whose box lies wholly outside has every
//!   sample outside, and the filter on each sample refuses them all.
//! - `85:52` `||` → `&&`: a sample more than three tolerances past an edge's
//!   box is more than twice the tolerance from its curve, which bulges past
//!   its samples by no more than the tolerance; the reach refuses it.
//! - `176:48` `+` → `-` and `194:42` `+` → `-`: the two sums differ by a turn
//!   or a period, which `rem_euclid` takes off; an ulp of rounding moves a
//!   sign only within rounding of a sample's own angle, where the place found
//!   is that sample and is held.
//! - `199:37` `||` → `&&`: once the bisection's two ends are neighbouring
//!   doubles, its remaining rounds move the parameter by an ulp at most.
//! - `213:54` `*` → `/`: `curve_side` is refused at zero first, so quotient
//!   and product share their sign, and neither underflows at a chart's
//!   scale.
//!
//! Closes #810, which delivers the cone pass alone — decided with the human,
//! 2026-11-09, in a comment on #810; the criteria below read as it reworded
//! them.
//! - The six shrunk cases are pinned and hold on both kernels — reworded: the
//!   two tangent-bore seeds hold on both kernels, and of the four parallel
//!   seeds what holds now is pinned, the rest kept ignored under #811 —
//!   `seed_353000000008193_a_block_bored_along_its_length_tangent_to_two_faces_then_cut_by_a_cone_leaning_18_degrees_keeps_every_rule`,
//!   `seed_353000000008193_the_same_bore_and_cone_keep_every_rule_through_the_application_s_body`,
//!   `seed_354000000012842_a_block_cut_by_a_cone_leaning_45_degrees_then_bored_along_its_length_tangent_to_two_faces_keeps_every_rule`,
//!   `seed_354000000012842_the_same_cone_and_bore_keep_every_rule_through_the_application_s_body`,
//!   `seed_353000000005193_the_same_bore_and_cone_keep_every_rule_through_the_application_s_body`,
//!   `seed_353000000005461_the_same_cone_and_bore_keep_every_rule_through_the_application_s_body`,
//!   `seed_354000000002444_the_same_cone_and_bore_keep_every_rule_through_the_application_s_body`
//! - A tessellation test shows a cone face between two parallel walls 1e-5
//!   apart, and between a plane and a cylinder tangent to it, cut into
//!   closed, uncrossed triangles — reworded: the tangent one whole, and the
//!   parallel one cut and closed, its walls left uncrossed by #811 —
//!   `a_cone_face_between_a_block_s_face_and_a_bore_tangent_to_it_is_cut_closed_and_uncrossed`,
//!   `a_cone_face_between_a_shaft_and_a_bore_of_its_radius_1e_5_apart_is_cut_and_closed`
//! - The measurement against `main` is recorded in
//!   `docs/exact-kernel-failures.md`, and the six seeds' attribution there
//!   moves from #756 to this issue — no test: a document; the walls
//!   campaigns are run on `main` and on the branch over sets
//!   352000000000000, 353000000000000 and 354000000000000, both kernels,
//!   and compared seed by seed
//!
//! Once the cone face is cut, the four parallel seeds meet a flaw it hid, on
//! `main` too: the shaft's and the bore's walls cross each other where the
//! cone meets them. Their meets with the cone, read along its rulings, found
//! the other wall's angle at the next node of their table as well as where
//! it stands, and each wall at a different node.
//!
//! Closes #811.
//! - A unit test: for every node of a ruled lap, `at_angle(false, θ)` at that
//!   node's exact angle returns only parameters where the meet stands at θ —
//!   no test: in this file; held by
//!   `at_the_exact_angle_of_a_node_of_its_table_a_ruled_meet_is_found_only_where_it_stands_there`,
//!   in `crates/solid/src/brep/meet/tests/ruled.rs`
//! - The six tests #810 ignored hold, the ignore lifted, and the four seeds of
//!   the comment are pinned and hold —
//!   `a_cone_face_between_a_shaft_and_a_bore_of_its_radius_1e_5_apart_is_cut_closed_and_uncrossed`,
//!   `seed_353000000005193_a_shaft_and_a_bore_of_its_radius_3e_4_apart_joined_to_a_cone_across_them_keeps_every_rule`,
//!   `seed_353000000005461_a_shaft_cut_by_a_cone_across_then_bored_by_its_radius_6e_3_off_its_axis_keeps_every_rule`,
//!   `seed_353000000014733_a_shaft_and_a_bore_of_its_radius_9e_5_apart_cut_by_a_cone_leaning_30_degrees_keeps_every_rule`,
//!   `seed_353000000014733_the_same_bore_and_cone_keep_every_rule_through_the_application_s_body`,
//!   `seed_354000000002444_a_shaft_cut_by_a_cone_across_then_bored_a_hair_inside_its_wall_2e_7_off_its_axis_keeps_every_rule`,
//!   `seed_352000000000867_a_shaft_cut_across_by_a_cone_then_bored_a_hair_inside_its_wall_6e_8_off_its_axis_keeps_every_rule`,
//!   `seed_352000000001195_a_shaft_cut_across_by_a_cone_then_bored_on_its_axis_2e_8_inside_its_wall_keeps_every_rule`,
//!   `seed_352000000001195_the_same_cone_and_bore_keep_every_rule_through_the_application_s_body`,
//!   `seed_353000000016959_a_shaft_bored_a_hair_inside_its_wall_and_joined_to_a_cone_keeps_every_rule_through_the_application_s_body`
//! - No draw regresses against `main`, seed by seed — the seven above
//!   included — no test: a campaign measurement; the walls sets
//!   352000000000000, 353000000000000 and 354000000000000 on both kernels and
//!   the turned set, run on `main` and on the branch and compared seed by seed
//! - The measurement is recorded in `docs/exact-kernel-failures.md` — no test:
//!   a document
//!
//! Seed 353000000016959 is pinned as drawn, not shrunk: once the meet finds
//! the other wall where it stands the draw holds, and its shrunk case still
//! crosses, elsewhere.

// The drawing, the promise and the checks are the campaigns'; this file
// only builds cases and holds them.
#[allow(dead_code, unused_imports)]
mod random_solids;

use cao_solid::brep::Surface;
use cao_solid::soundness::{closed, uncrossed};
use glam::DVec3;
use random_solids::{Axis, Case, Exact, Kernel, Leaf, Outline, Plane, Section, Step, TESSELLATION};

/// Builds a case on the exact kernel and cuts its last body into triangles
/// at the campaigns' tolerance, holding every face cut, at least one of them
/// on a cone, and the triangles closed.
fn every_cone_face_is_cut_and_closed(case: &Case) -> Vec<[DVec3; 3]> {
    let leaves: Vec<&Leaf> = case.leaves().collect();
    let mut body = Exact.raised(leaves[0]).expect("the first leaf is raised");
    for (step, leaf) in case.steps.iter().zip(&leaves[1..]) {
        let tool = Exact.raised(leaf).expect("the tool is raised");
        body = Exact
            .combined(&body, &tool, step.mode)
            .expect("the step is built");
    }
    let is_a_cone = |surface: &Surface| matches!(surface, Surface::Cone(_));
    assert!(
        body.face_ids()
            .any(|id| is_a_cone(body.surface(body.face(id).surface))),
        "the body holds no cone face\n{case}"
    );
    let cut = body.triangles_by_face(TESSELLATION * body.scale().reach());
    let uncut: Vec<_> = cut
        .uncut
        .iter()
        .map(|id| (id, body.surface(body.face(*id).surface)))
        .collect();
    assert!(uncut.is_empty(), "faces left uncut: {uncut:?}\n{case}");
    closed(&cut.triangles).expect("the triangles close");
    cut.triangles
}

fn every_cone_face_is_cut_closed_and_uncrossed(case: &Case) {
    let triangles = every_cone_face_is_cut_and_closed(case);
    uncrossed(&triangles).expect("no triangle crosses another");
}

/// A shaft of radius 4 about Z bored along it by a hole of its radius 1e-5
/// off its axis, then joined to a cone turned about Y across both.
fn a_shaft_and_a_bore_of_its_radius_1e_5_apart_joined_to_a_cone_across_them() -> Case {
    Case::new(
        Leaf::prism(Plane::xy(-3.0), Outline::circle([0.0, 2.0], 4.0), 4.0),
        vec![
            Step::cut(Leaf::prism(
                Plane::tilted([1e-5, 2.0, -18.9], [0.0, 0.0, 0.0]),
                Outline::circle([0.0, 0.0], 4.0),
                34.0,
            )),
            Step::add(Leaf::turned(
                Plane::xy(-2.0),
                Axis::second(0.0),
                Section::bands(-18.0, &[[34.0, 0.0, 0.0]]).sloping_to(&[[0.0, 0.5]]),
                360.0,
            )),
        ],
    )
}

#[test]
fn a_cone_face_between_a_shaft_and_a_bore_of_its_radius_1e_5_apart_is_cut_and_closed() {
    every_cone_face_is_cut_and_closed(
        &a_shaft_and_a_bore_of_its_radius_1e_5_apart_joined_to_a_cone_across_them(),
    );
}

#[test]
fn a_cone_face_between_a_shaft_and_a_bore_of_its_radius_1e_5_apart_is_cut_closed_and_uncrossed() {
    every_cone_face_is_cut_closed_and_uncrossed(
        &a_shaft_and_a_bore_of_its_radius_1e_5_apart_joined_to_a_cone_across_them(),
    );
}

#[test]
fn a_cone_face_between_a_block_s_face_and_a_bore_tangent_to_it_is_cut_closed_and_uncrossed() {
    every_cone_face_is_cut_closed_and_uncrossed(&Case::new(
        Leaf::prism(
            Plane::yz(-4.0),
            Outline::rectangle([-2.0, -4.0], [6.0, 6.0]),
            9.0,
        ),
        vec![
            Step::cut(Leaf::turned(
                Plane::xy(1.0),
                Axis::first(0.0).leaning(45.0),
                Section::bands(-16.0, &[[33.0, 0.0, 1.0]]).sloping_to(&[[0.0, 0.0]]),
                360.0,
            )),
            Step::cut(Leaf::prism(
                Plane::tilted([-16.0, 1.0, 1.0], [0.0, 90.0, 0.0]),
                Outline::circle([0.0, 0.0], 3.0),
                33.0,
            )),
        ],
    ));
}

/// A shaft of radius 4 about Z bored along it by a hole of its radius 3e-4
/// off its axis, then joined to a cone turned about Y across both: its cone
/// face was left uncut on the exact kernel, and held through the body only
/// because its tolerance lands the samples elsewhere (seed 353000000005193,
/// shrunk; #755).
fn a_shaft_and_a_bore_of_its_radius_3e_4_apart_joined_to_a_cone_across_them() -> Case {
    Case::new(
        Leaf::prism(Plane::xy(-3.0), Outline::circle([0.0, 2.0], 4.0), 4.0),
        vec![
            Step::cut(Leaf::prism(
                Plane::tilted(
                    [-0.00029670597282397245, 2.0, -18.90623802179891],
                    [0.0, 0.0, 0.0],
                ),
                Outline::circle([0.0, 0.0], 4.0),
                34.0,
            )),
            Step::add(Leaf::turned(
                Plane::xy(-2.0),
                Axis::second(0.0),
                Section::bands(-18.0, &[[34.0, 0.0, 0.0]]).sloping_to(&[[0.0, 0.5]]),
                360.0,
            )),
        ],
    )
}

#[test]
fn seed_353000000005193_a_shaft_and_a_bore_of_its_radius_3e_4_apart_joined_to_a_cone_across_them_keeps_every_rule()
 {
    random_solids::holds_exactly(
        &a_shaft_and_a_bore_of_its_radius_3e_4_apart_joined_to_a_cone_across_them(),
    );
}

#[test]
fn seed_353000000005193_the_same_bore_and_cone_keep_every_rule_through_the_application_s_body() {
    random_solids::holds_through_the_application(
        &a_shaft_and_a_bore_of_its_radius_3e_4_apart_joined_to_a_cone_across_them(),
    );
}

/// A shaft of radius 60 about Z cut across by a cone turned about Y, then
/// bored along it by a hole of its radius 6e-3 off its axis: its cone face
/// was left uncut (seed 353000000005461, shrunk; #755).
fn a_shaft_cut_by_a_cone_across_then_bored_by_its_radius_6e_3_off_its_axis() -> Case {
    Case::new(
        Leaf::prism(Plane::xy(-30.0), Outline::circle([60.0, 0.0], 60.0), 195.0),
        vec![
            Step::cut(Leaf::turned(
                Plane::xy(60.0),
                Axis::second(0.0),
                Section::bands(-180.0, &[[697.0, 0.0, 30.0]]).sloping_to(&[[0.0, 60.0]]),
                360.0,
            )),
            Step::cut(Leaf::prism(
                Plane::tilted([60.0060825652786, 0.0, -281.0053190411309], [0.0, 0.0, 0.0]),
                Outline::circle([-0.0, -0.0], 60.0),
                697.0,
            )),
        ],
    )
}

#[test]
fn seed_353000000005461_a_shaft_cut_by_a_cone_across_then_bored_by_its_radius_6e_3_off_its_axis_keeps_every_rule()
 {
    random_solids::holds_exactly(
        &a_shaft_cut_by_a_cone_across_then_bored_by_its_radius_6e_3_off_its_axis(),
    );
}

#[test]
fn seed_353000000005461_the_same_cone_and_bore_keep_every_rule_through_the_application_s_body() {
    random_solids::holds_through_the_application(
        &a_shaft_cut_by_a_cone_across_then_bored_by_its_radius_6e_3_off_its_axis(),
    );
}

/// A shaft of radius 10 about Z bored along it by a hole of its radius 9e-5
/// off its axis, then cut by a cone turned about a line leaning 30°: its
/// cone face was left uncut (seed 353000000014733, shrunk; #755).
fn a_shaft_and_a_bore_of_its_radius_9e_5_apart_cut_by_a_cone_leaning_30_degrees() -> Case {
    Case::new(
        Leaf::prism(Plane::xy(-20.0), Outline::circle([5.0, 0.0], 10.0), 25.0),
        vec![
            Step::cut(Leaf::prism(
                Plane::tilted(
                    [5.000092064673217, 0.0, -51.50882259118668],
                    [0.0, 0.0, 0.0],
                ),
                Outline::circle([-0.0, -0.0], 10.0),
                105.0,
            )),
            Step::cut(Leaf::turned(
                Plane::yz(5.0000001),
                Axis::second(-6.696115854552918).leaning(30.0),
                Section::bands(-66.14140388545958, &[[105.49834435270749, 0.0, 7.5]])
                    .sloping_to(&[[0.0, 5.0]]),
                360.0,
            )),
        ],
    )
}

#[test]
fn seed_353000000014733_a_shaft_and_a_bore_of_its_radius_9e_5_apart_cut_by_a_cone_leaning_30_degrees_keeps_every_rule()
 {
    random_solids::holds_exactly(
        &a_shaft_and_a_bore_of_its_radius_9e_5_apart_cut_by_a_cone_leaning_30_degrees(),
    );
}

#[test]
fn seed_353000000014733_the_same_bore_and_cone_keep_every_rule_through_the_application_s_body() {
    random_solids::holds_through_the_application(
        &a_shaft_and_a_bore_of_its_radius_9e_5_apart_cut_by_a_cone_leaning_30_degrees(),
    );
}

/// A shaft of radius 5 about Z cut across by a cone turned about X, then
/// bored along it by a hole of radius 4.9999998 2e-7 off its axis, a hair
/// inside its wall: its cone face was left uncut (seed 354000000002444,
/// shrunk; #755).
fn a_shaft_cut_by_a_cone_across_then_bored_a_hair_inside_its_wall_2e_7_off_its_axis() -> Case {
    Case::new(
        Leaf::prism(Plane::xy(-3.0), Outline::circle([2.0, 2.0], 5.0), 8.0),
        vec![
            Step::cut(Leaf::turned(
                Plane::xy(2.0),
                Axis::first(0.0),
                Section::bands(-20.0, &[[44.0, 0.0, 0.0]]).sloping_to(&[[0.0, 1.0]]),
                360.0,
            )),
            Step::cut(Leaf::prism(
                Plane::tilted([2.0, 2.0, -25.0], [0.0, 0.0, 0.0]),
                Outline::circle([0.0, 1.9999999967268423e-7], 4.9999998),
                44.0,
            )),
        ],
    )
}

#[test]
fn seed_354000000002444_a_shaft_cut_by_a_cone_across_then_bored_a_hair_inside_its_wall_2e_7_off_its_axis_keeps_every_rule()
 {
    random_solids::holds_exactly(
        &a_shaft_cut_by_a_cone_across_then_bored_a_hair_inside_its_wall_2e_7_off_its_axis(),
    );
}

#[test]
fn seed_354000000002444_the_same_cone_and_bore_keep_every_rule_through_the_application_s_body() {
    random_solids::holds_through_the_application(
        &a_shaft_cut_by_a_cone_across_then_bored_a_hair_inside_its_wall_2e_7_off_its_axis(),
    );
}

/// A block bored along its length by a cylinder of radius 150 tangent to two
/// of its faces, then cut by a cone turned about a line leaning 18°: its
/// cone face was left uncut where it meets the face tangent to the bore
/// (seed 353000000008193, shrunk; #755).
fn a_block_bored_along_its_length_tangent_to_two_faces_then_cut_by_a_cone_leaning_18_degrees()
-> Case {
    Case::new(
        Leaf::prism(
            Plane::yz(0.0),
            Outline::rectangle([-150.0, -180.0], [150.0, 120.0]),
            270.0,
        ),
        vec![
            Step::cut(Leaf::prism(
                Plane::tilted([-548.0, 0.0, -31.0], [0.0, 90.0, 0.0]),
                Outline::circle([0.0, 0.0], 150.0),
                1366.0,
            )),
            Step::cut(Leaf::turned(
                Plane::xy(-65.0),
                Axis::second(0.0).leaning(18.0),
                Section::bands(-683.0, &[[1366.0, 0.0, 60.0]]).sloping_to(&[[0.0, 15.0]]),
                360.0,
            )),
        ],
    )
}

#[test]
fn seed_353000000008193_a_block_bored_along_its_length_tangent_to_two_faces_then_cut_by_a_cone_leaning_18_degrees_keeps_every_rule()
 {
    random_solids::holds_exactly(
        &a_block_bored_along_its_length_tangent_to_two_faces_then_cut_by_a_cone_leaning_18_degrees(
        ),
    );
}

#[test]
fn seed_353000000008193_the_same_bore_and_cone_keep_every_rule_through_the_application_s_body() {
    random_solids::holds_through_the_application(
        &a_block_bored_along_its_length_tangent_to_two_faces_then_cut_by_a_cone_leaning_18_degrees(
        ),
    );
}

/// A block cut by a cone turned about a line leaning 45°, then bored along
/// its length by a cylinder of radius 3 tangent to two of its faces: both
/// halves of its cone face were left uncut (seed 354000000012842, shrunk;
/// #755).
fn a_block_cut_by_a_cone_leaning_45_degrees_then_bored_along_its_length_tangent_to_two_faces()
-> Case {
    Case::new(
        Leaf::prism(
            Plane::yz(-4.0),
            Outline::rectangle([-2.0, -3.0], [4.0, 4.0]),
            9.0,
        ),
        vec![
            Step::cut(Leaf::turned(
                Plane::xy(1.0),
                Axis::first(0.0).leaning(45.0),
                Section::bands(-16.0, &[[33.0, 0.0, 1.0]]).sloping_to(&[[0.0, 0.0]]),
                360.0,
            )),
            Step::cut(Leaf::prism(
                Plane::tilted([-16.0, 1.0, 1.0], [0.0, 90.0, 0.0]),
                Outline::circle([0.0, 0.0], 3.0),
                33.0,
            )),
        ],
    )
}

#[test]
fn seed_354000000012842_a_block_cut_by_a_cone_leaning_45_degrees_then_bored_along_its_length_tangent_to_two_faces_keeps_every_rule()
 {
    random_solids::holds_exactly(
        &a_block_cut_by_a_cone_leaning_45_degrees_then_bored_along_its_length_tangent_to_two_faces(
        ),
    );
}

#[test]
fn seed_354000000012842_the_same_cone_and_bore_keep_every_rule_through_the_application_s_body() {
    random_solids::holds_through_the_application(
        &a_block_cut_by_a_cone_leaning_45_degrees_then_bored_along_its_length_tangent_to_two_faces(
        ),
    );
}

/// A shaft of radius 5 along X cut across by a cone turned about Y, then
/// bored along it by a hole of radius 4.99999994 6e-8 off its axis, a hair
/// inside its wall (seed 352000000000867, shrunk; #811).
fn a_shaft_cut_across_by_a_cone_then_bored_a_hair_inside_its_wall_6e_8_off_its_axis() -> Case {
    Case::new(
        Leaf::prism(Plane::yz(-1.0), Outline::circle([0.0, -1.0], 5.0), 5.0),
        vec![
            Step::cut(Leaf::turned(
                Plane::xy(-0.0),
                Axis::second(0.0),
                Section::bands(-22.0, &[[42.0, 0.0, 0.0]]).sloping_to(&[[0.0, 1.0]]),
                360.0,
            )),
            Step::cut(Leaf::prism(
                Plane::tilted(
                    [-19.50000000000001, 0.0, -1.0000000366519146],
                    [0.0, 90.0, 0.0],
                ),
                Outline::circle([-0.0, -5.999999963535175e-8], 4.99999994),
                42.0,
            )),
        ],
    )
}

#[test]
fn seed_352000000000867_a_shaft_cut_across_by_a_cone_then_bored_a_hair_inside_its_wall_6e_8_off_its_axis_keeps_every_rule()
 {
    random_solids::holds_exactly(
        &a_shaft_cut_across_by_a_cone_then_bored_a_hair_inside_its_wall_6e_8_off_its_axis(),
    );
}

/// A shaft of radius 3 about Z cut across by a cone turned about X, then
/// bored on its own axis by a hole of radius 2.99999998, a hair inside its
/// wall (seed 352000000001195, shrunk; #811).
fn a_shaft_cut_across_by_a_cone_then_bored_on_its_axis_2e_8_inside_its_wall() -> Case {
    Case::new(
        Leaf::prism(Plane::xy(-2.0), Outline::circle([-1.0, 1.0], 3.0), 6.0),
        vec![
            Step::cut(Leaf::turned(
                Plane::xy(1.0),
                Axis::first(0.0),
                Section::bands(-15.0, &[[28.0, 0.0, 1.0]]).sloping_to(&[[0.0, 2.0]]),
                360.0,
            )),
            Step::cut(Leaf::prism(
                Plane::tilted([-1.0, 1.0, -16.0], [0.0, 0.0, 0.0]),
                Outline::circle([0.0, 0.0], 2.99999998),
                28.0,
            )),
        ],
    )
}

#[test]
fn seed_352000000001195_a_shaft_cut_across_by_a_cone_then_bored_on_its_axis_2e_8_inside_its_wall_keeps_every_rule()
 {
    random_solids::holds_exactly(
        &a_shaft_cut_across_by_a_cone_then_bored_on_its_axis_2e_8_inside_its_wall(),
    );
}

#[test]
fn seed_352000000001195_the_same_cone_and_bore_keep_every_rule_through_the_application_s_body() {
    random_solids::holds_through_the_application(
        &a_shaft_cut_across_by_a_cone_then_bored_on_its_axis_2e_8_inside_its_wall(),
    );
}

#[test]
fn seed_353000000016959_a_shaft_bored_a_hair_inside_its_wall_and_joined_to_a_cone_keeps_every_rule_through_the_application_s_body()
 {
    random_solids::holds_through_the_application(&Case::drawn_walls_at_any_angle(353000000016959));
}
