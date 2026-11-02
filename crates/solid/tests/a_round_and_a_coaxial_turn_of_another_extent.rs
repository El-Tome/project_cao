//! Closes #623.
//! - the four seeds #576's lane C shrunk hold on the exact kernel —
//!   `a_quarter_ring_joined_to_a_whole_coaxial_band_it_touches_keeps_every_rule`,
//!   `a_half_ring_cut_by_a_coaxial_three_quarter_turn_keeps_every_rule`,
//!   `a_whole_ring_cut_by_a_coaxial_half_turn_keeps_every_rule`,
//!   `a_half_ring_joined_to_a_whole_coaxial_disc_on_its_crest_keeps_every_rule`
//! - the rounded campaign's `Unverified` falls without one case built wrong
//!   that declined before — no test: measured, base against tip on the same
//!   seeds of `a_campaign_of_rounded_solids_on_the_exact_kernel_keeps_every_rule`,
//!   in `docs/exact-kernel-failures.md`
//! - the bar of #576 measured again — no test: a fresh campaign, in
//!   `docs/exact-kernel-failures.md`
//!
//! A sphere or a torus turned through one extent combined with a body turned
//! about the same axis through another: the kernel built the boolean, its own
//! check found a loop on the ring torus that did not close across the chart's
//! cut, or an edge used once, and declined `Unverified`.

// The drawing, the promise and the checks are the campaigns'; this file
// only builds cases and holds them.
#[allow(dead_code, unused_imports)]
mod random_solids;

use std::f64::consts::PI;

use random_solids::{Axis, Case, Exact, Kernel, Leaf, Outline, Plane, Section, Step};

/// Lane C round 1 of #576, the rounded draw on the exact kernel: Answers.
#[test]
fn a_quarter_ring_joined_to_a_whole_coaxial_band_it_touches_keeps_every_rule() {
    random_solids::holds_exactly(&Case::new(
        Leaf::turned(
            Plane::xy(-2.0),
            Axis::first(-1.0),
            Section::bands(1.5, &[[3.0, 4.5, 4.5]]).bowed(&[[[3.0, 4.5, -1.5], [3.0, 4.5, 1.5]]]),
            90.0,
        ),
        vec![Step::add(Leaf::turned(
            Plane::xy(-2.0),
            Axis::first(-1.0),
            Section::bands(2.0, &[[3.0, 6.0, 7.0]]),
            360.0,
        ))],
    ));
}

/// Lane C round 1 of #576, the rounded draw on the exact kernel: Answers.
#[test]
fn a_half_ring_cut_by_a_coaxial_three_quarter_turn_keeps_every_rule() {
    random_solids::holds_exactly(&Case::new(
        Leaf::turned(
            Plane::xy(5.0),
            Axis::first(0.0),
            Section::bands(-1.5, &[[1.0, 1.5, 1.5]])
                .bowed(&[[[-1.0, 1.5, -0.5], [-1.0, 1.5, 0.5]]]),
            180.0,
        ),
        vec![Step::cut(Leaf::turned(
            Plane::xz(0.0),
            Axis::first(5.0),
            Section::bands(-1.0, &[[1.0, 0.0, 3.0]]),
            270.0,
        ))],
    ));
}

/// Lane C round 1 of #576, the rounded draw on the exact kernel: Answers.
#[test]
fn a_whole_ring_cut_by_a_coaxial_half_turn_keeps_every_rule() {
    random_solids::holds_exactly(&Case::new(
        Leaf::turned(
            Plane::xy(0.0),
            Axis::second(0.0),
            Section::bands(2.5, &[[15.0, -10.0, -10.0]])
                .bowed(&[[[10.0, -10.0, -7.5], [10.0, -10.0, 7.5]]]),
            360.0,
        ),
        vec![Step::cut(Leaf::turned(
            Plane::xy(0.0),
            Axis::second(0.0),
            Section::bands(-2.5, &[[22.5, 0.0, 17.5]]),
            180.0,
        ))],
    ));
}

/// Lane C round 1 of #576, the rounded draw on the exact kernel: Answers.
#[test]
fn a_half_ring_joined_to_a_whole_coaxial_disc_on_its_crest_keeps_every_rule() {
    random_solids::holds_exactly(&Case::new(
        Leaf::turned(
            Plane::xy(5.0),
            Axis::second(0.0),
            Section::bands(-2.0, &[[4.0, -5.0, -5.0]])
                .bowed(&[[[0.0, -5.0, -2.0], [0.0, -5.0, 2.0]]]),
            180.0,
        ),
        vec![Step::add(Leaf::turned(
            Plane::xy(5.0),
            Axis::second(0.0),
            Section::bands(2.0, &[[1.0, 0.0, 9.0]]),
            360.0,
        ))],
    ));
}

// What the first measure of #623 built wrong where the kernel had declined
// before, each shrunk: older families the gluing laid bare.

/// Seed 5576851701: a whole ring joined to a coaxial disc turned 315°, its
/// tube kept whole in the gap and holed through the cut elsewhere — a face
/// reaching round the axis, walled.
#[test]
fn a_whole_ring_joined_to_a_coaxial_disc_turned_short_of_a_turn_keeps_every_rule() {
    random_solids::holds_exactly(&Case::new(
        Leaf::turned(
            Plane::xy(1.0),
            Axis::first(3.0),
            Section::bands(-4.0, &[[6.0, 6.0, 6.0]])
                .bowed(&[[[-1.0, 6.0, -3.0], [-1.0, 6.0, 3.0]]]),
            360.0,
        ),
        vec![Step::add(Leaf::turned(
            Plane::xz(3.0),
            Axis::first(1.0),
            Section::bands(-2.0, &[[3.0, -11.0, 0.0]]),
            315.0,
        ))],
    ));
}

/// Seed 5576856298: a whole ring with a quarter of the top of its tube cut
/// away — a face of holes alone, holding the whole cut.
#[test]
fn a_whole_ring_slotted_by_a_coaxial_quarter_disc_keeps_every_rule_and_its_volume() {
    let case = Case::new(
        Leaf::turned(
            Plane::xy(6.0),
            Axis::first(0.0),
            Section::bands(3.0, &[[2.0, 4.0, 4.0]]).bowed(&[[[4.0, 4.0, -1.0], [4.0, 4.0, 1.0]]]),
            360.0,
        ),
        vec![Step::cut(Leaf::turned(
            Plane::xy(6.0),
            Axis::first(0.0),
            Section::bands(4.0, &[[1.0, 0.0, 7.0]]),
            90.0,
        ))],
    );
    random_solids::holds_exactly(&case);
    let body = exactly(&case);
    let ring = 2.0 * PI * PI * 4.0;
    assert!(
        (body.volume() - ring * 7.0 / 8.0).abs() < 1e-9 * ring,
        "the ring less an eighth of it: {}",
        body.volume()
    );
}

/// Seed 5576852458: half of a ring's tube turned a half turn, joined to a
/// coaxial half cylinder that notches it — the meridian at the far end
/// sampled at the notch's rims, where the face's strips end.
#[test]
fn a_half_tube_notched_by_a_coaxial_half_cylinder_keeps_every_rule() {
    random_solids::holds_exactly(&Case::new(
        Leaf::turned(
            Plane::xz(7.0),
            Axis::second(0.0),
            Section::bands(-1.5, &[[3.0, 3.5, 3.5]]).bowed(&[[[0.0, 3.5, -1.5], [0.0, 0.0, 0.0]]]),
            180.0,
        ),
        vec![Step::add(Leaf::turned(
            Plane::yz(0.0),
            Axis::second(7.0),
            Section::bands(-2.0, &[[3.0, -3.0, 0.0]]),
            180.0,
        ))],
    ));
}

/// Seed 5576851706: a ring turned three quarters touching a coaxial shaft
/// along its inner equator — its face runs round the tube though its rims
/// all stand at the shaft, and its grid is cut for the crest.
#[test]
fn a_ring_turned_part_way_touching_a_shaft_along_its_inner_equator_keeps_every_rule() {
    for turn in [180.0, 270.0] {
        random_solids::holds_exactly(&Case::new(
            Leaf::turned(
                Plane::xy(1.0),
                Axis::second(-2.0).backwards(),
                Section::bands(1.0, &[[8.0, 6.0, 6.0]])
                    .bowed(&[[[5.0, 6.0, -4.0], [5.0, 6.0, 4.0]]]),
                turn,
            ),
            vec![Step::add(Leaf::turned(
                Plane::yz(-2.0),
                Axis::first(1.0),
                Section::bands(-11.0, &[[12.0, 0.0, 2.0]]),
                360.0,
            ))],
        ));
    }
}

/// Seed 5576850144: a sloped section rounded inside, cut by a coaxial turn of
/// the same torus — what is kept of the tube is a band the chart's cut runs
/// through, one face across it.
#[test]
fn a_band_of_a_torus_the_chart_s_cut_runs_through_is_one_face() {
    random_solids::holds_exactly(&Case::new(
        Leaf::turned(
            Plane::yz(6.5),
            Axis::first(0.0).backwards(),
            Section::bands(0.5, &[[3.5, 3.0, 8.0]])
                .sloping_to(&[[3.5, 8.0]])
                .bowed(&[[[2.0, 5.0, -2.5], [0.0, 0.0, 0.0]]]),
            360.0,
        ),
        vec![Step::cut(Leaf::turned(
            Plane::xy(0.0),
            Axis::second(6.5),
            Section::bands(-4.5, &[[5.0, -5.0, 0.0]])
                .bowed(&[[[-2.0, -5.0, -2.5], [0.0, 0.0, 0.0]]]),
            360.0,
        ))],
    ));
}

/// Seed 5576876097: a whole ring a coaxial slab only rests on, along its
/// crest, where the chart was cut — moved off the crest, the cut must not
/// leave the window's middle, where a whole ring's face is sampled, on it.
#[test]
fn a_whole_ring_a_coaxial_slab_rests_on_keeps_every_rule_and_its_volume() {
    let case = Case::new(
        Leaf::turned(
            Plane::xy(-1.0),
            Axis::second(0.0).backwards(),
            Section::bands(-0.5, &[[5.0, 4.5, 4.5]]).bowed(&[[[2.0, 4.5, -2.5], [2.0, 4.5, 2.5]]]),
            360.0,
        ),
        vec![Step::cut(Leaf::turned(
            Plane::xy(-1.0),
            Axis::second(0.0),
            Section::bands(0.5, &[[1.0, 0.0, 7.5]]),
            90.0,
        ))],
    );
    random_solids::holds_exactly(&case);
    random_solids::holds_through_the_application(&case);
    let ring = 2.0 * PI * PI * 4.5 * 2.5 * 2.5;
    let body = exactly(&case);
    assert!(
        (body.volume() - ring).abs() < 1e-9 * ring,
        "the whole ring: {}",
        body.volume()
    );
}

/// Seeds 5576873290 and 5577106512: a whole ring joined to a coaxial tube
/// whose bore it touches along its inner equator, the chart cut there.
#[test]
fn a_whole_ring_joined_to_a_coaxial_tube_whose_bore_it_touches_keeps_every_rule() {
    random_solids::holds_exactly(&Case::new(
        Leaf::turned(
            Plane::xy(45.0),
            Axis::first(180.0).backwards(),
            Section::bands(75.0, &[[150.0, 120.0, 120.0]])
                .bowed(&[[[150.0, 120.0, -75.0], [150.0, 120.0, 75.0]]]),
            360.0,
        ),
        vec![Step::add(Leaf::prism(
            Plane::yz(-75.0),
            Outline::ring([180.0, 45.0], 165.0, 120.0),
            195.0,
        ))],
    ));
    random_solids::holds_through_the_application(&Case::new(
        Leaf::turned(
            Plane::xz(8.0),
            Axis::first(-2.5).backwards(),
            Section::bands(-3.0, &[[4.0, 2.5, 2.5]])
                .bowed(&[[[-1.0, 2.5, -2.0], [-1.0, 2.5, 2.0]]]),
            360.0,
        ),
        vec![Step::add(Leaf::prism(
            Plane::yz(3.0),
            Outline::ring([8.0, -2.5], 4.0, 2.5),
            10.0,
        ))],
    ));
}

/// The case raised and its steps taken on the exact kernel.
fn exactly(case: &Case) -> cao_solid::brep::Body {
    let mut body = Exact.raised(&case.start).expect("the leaf is raised");
    for step in &case.steps {
        let tool = Exact.raised(&step.tool).expect("the tool is raised");
        body = Exact
            .combined(&body, &tool, step.mode)
            .expect("the step is built");
    }
    body
}
