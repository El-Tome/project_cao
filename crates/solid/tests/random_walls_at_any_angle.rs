//! Walls at any angle to each other, drawn at random and held to the rules
//! every result must keep (#577): #448's harness, drawing them before the
//! kernel builds them.
//!
//! The cases (`Case::drawn_walls_at_any_angle`, a seed space of its own)
//! start with a shaft, a tube, a cone or an elliptic shaft about a world
//! axis, and cross its wall with holes and pins, round or elliptic, raised on
//! leaning planes, and with cones turned about leaning lines: two cylinders
//! at a skew angle, a cylinder against a cone off its axis — a radial hole
//! through a cone, a pin through a taper — two cones of other axes, an
//! elliptic wall against a cone. The angle comes on the lattice, on a
//! degree's step, at a real angle, or a hair from square or from parallel;
//! the axes pass through each other, a hair off, where the walls touch
//! inside or outside, a hair from there, or anywhere between. Two walls at a
//! skew angle are built since #577; the scope rule excuses a decline as
//! unsupported where a cone meets a wall off its axis, until the kernel
//! builds it, and where two walls lean in the band along a stretch, which it
//! declines (`Case::asks_beyond_the_conics`).
//!
//! Done when, the part of #577 this file holds (the whole list is at the
//! head of `two_walls_at_any_angle.rs`; the cones' criteria are lane 3's,
//! not answered here, though the draw already draws them): #448's harness
//! draws walls at any angle before the kernel code —
//! `the_draw_of_walls_at_any_angle_draws_every_pair_the_issue_names`,
//! `random_walls_at_any_angle_keep_every_rule_on_the_exact_kernel`;
//! campaigns end under one failure in a thousand on fresh seeds, with no
//! regression — the campaigns below, run by hand:
//!
//! ```text
//! CAO_FUZZ_SECONDS=3600 cargo test --release -p cao_solid --features campaigns \
//!     --test random_walls_at_any_angle -- --ignored --nocapture \
//!     a_campaign_of_walls_at_any_angle_on_the_exact_kernel
//! ```
//!
//! `…_through_the_application_s_body_…` runs them as the application
//! computes them. `CAO_FUZZ_SEED`, `CAO_FUZZ_CASES` and `CAO_FUZZ_PATIENCE`
//! are read as in `random_exact_solids.rs`; the seeds a campaign names are
//! shrunk by `the_seeds_a_walls_campaign_named_are_shrunk_one_by_one`, given
//! in `CAO_TRIAGE_SEEDS`, with `CAO_TRIAGE_KERNEL=application` for the
//! campaign through the application's body.
//!
//! Two cylinders nearly parallel and closer than a chord's sag stay
//! declined (#706). The first four steps of #706 lay two such walls on one
//! shared order in the tessellation, held beside the code they change from
//! `crates/solid/src/brep/tessellation/contact/facing/tests.rs`, each with
//! every campaign identical to `main` seed by seed. Building them — lifting
//! the decline — is set aside: against `main`, the campaigns found bodies
//! `main` declines built wrong, a third wall crossing the close stretch
//! among them. The cases it would build are kept under `#[ignore]`, to run
//! once it lands; the cases the decline answers run, and hold it
//! (`docs/exact-kernel-failures.md`).
//!
//! Part of #706; both criteria of its Done when wait on the lift.
//! - the B reds of #577 (the nearly coaxial posts at 1e-3°, seeds
//!   224442167739556 and 224442167740434) are built, uncrossed and noted
//!   unstable, not declined — waits on the lift: the cases are
//!   `two_posts_of_one_radius_crossing_nearly_coaxially_at_1e_3_degrees_are_built_uncrossed_and_noted_unstable`
//!   and
//!   `seed_224442167739556_two_shafts_of_radius_150_crossing_at_1e_4_degrees_are_built_uncrossed_and_noted_unstable`,
//!   ignored; 224442167740434, which the owner wants built too, is held
//!   declined meanwhile by
//!   `seed_224442167740434_two_posts_of_radius_22_5_crossing_at_1e_4_degrees_are_declined_as_unsupported`
//! - the walls-at-any-angle draw keeps its rate or improves, with no
//!   regression against `main` — waits on the lift; no test: a campaign
//!   fails on any finding, so the rate is measured by hand, the campaigns
//!   above run on `main` and on the branch over the same seeds and compared
//!   seed by seed, and the comparison is reported on the issue
//!
//! Closes #754.
//! - The case 352000000000597 above is pinned and holds on both kernels —
//!   `seed_352000000000597_a_shaft_bored_by_its_radius_two_thousandths_off_its_axis_then_twice_more_keeps_every_rule`,
//!   `seed_352000000000597_the_same_bores_keep_every_rule_through_the_application_s_body`;
//!   the case 352000000002028 holds with the decline lifted — waits on
//!   #706's lift:
//!   `seed_352000000002028_a_shaft_bored_at_15_degrees_then_by_its_radius_at_1e_4_degrees_is_built_uncrossed_and_noted_unstable`,
//!   ignored, the case held declined meanwhile by
//!   `seed_352000000002028_a_shaft_bored_at_15_degrees_then_by_its_radius_at_1e_4_degrees_is_declined_as_unsupported`
//! - No draw regresses against `main`, seed by seed — the walls campaigns
//!   above, on both kernels, run on `main` and on the branch over the same
//!   seeds and compared seed by seed, reported on the pull request; the
//!   draws they found held on `main` and crossed on the branch are pinned:
//!   `seed_354000000008476_an_elliptic_shaft_bored_round_along_its_wall_a_thousandth_of_a_degree_off_is_drawn_uncrossed`,
//!   `seed_354000000008476_the_same_bore_is_drawn_uncrossed_through_the_application_s_body`;
//!   seed 352000000007085 is answered by #759 —
//!   `seed_352000000007085_a_tube_slotted_along_its_bore_banded_and_posted_is_drawn_uncrossed`,
//!   `seed_352000000007085_the_same_tube_is_drawn_uncrossed_through_the_application_s_body`;
//!   seed 354000000005870 is carried by #761, its test ignored until then:
//!   `seed_354000000005870_a_shaft_bored_at_80_degrees_then_by_a_cone_leaning_24_degrees_is_drawn_uncrossed`
//!
//! The second criterion, that one rule tells a wall from its snapped copy,
//! is held beside the tessellation, in
//! `crates/solid/src/brep/tessellation/contact/facing/tests/snapped_copies.rs`.
//!
//! Closes #759.
//! - `seed_352000000007085_a_tube_slotted_along_its_bore_banded_and_posted_is_drawn_uncrossed`
//!   and `seed_352000000007085_the_same_tube_is_drawn_uncrossed_through_the_application_s_body`
//!   in `crates/solid/tests/random_walls_at_any_angle.rs` lose their
//!   `#[ignore]` and pass — both, here
//! - 354000000005870's test too loses its `#[ignore]` and passes (the
//!   issue's comment) — no test: its crossing is another mechanism, the
//!   shaft's fan against a bore touching it inside at a point, carried by
//!   #761
//! - No draw regresses against `main`, seed by seed, on the three sets #754
//!   was measured on — the walls campaigns above run on `main` and on the
//!   branch over those seeds and compared seed by seed, reported on the
//!   pull request; the draw they found held on `main` and crossed on the
//!   branch, 353000000013767, is answered by #762:
//!   `seed_353000000013767_a_shaft_bored_three_times_one_bore_grazing_is_drawn_uncrossed`,
//!   `seed_353000000013767_the_same_bores_are_drawn_uncrossed_through_the_application_s_body`
//!
//! Closes #762.
//! - `seed_353000000013767_a_shaft_bored_three_times_one_bore_grazing_is_drawn_uncrossed`
//!   and `seed_353000000013767_the_same_bores_are_drawn_uncrossed_through_the_application_s_body`
//!   lose their `#[ignore]` and pass — both, here; the mechanism in
//!   miniature, a flip taken though a thin triangle stands two thirds of a
//!   tolerance above what it makes, is held beside the folds in
//!   `crates/solid/src/brep/tessellation/folds/tests.rs`
//! - 352000000007085's two tests and the hair-high unit test stay green —
//!   `seed_352000000007085_a_tube_slotted_along_its_bore_banded_and_posted_is_drawn_uncrossed`,
//!   `seed_352000000007085_the_same_tube_is_drawn_uncrossed_through_the_application_s_body`,
//!   and `a_triangle_a_hair_high_and_a_wide_one_lying_on_it_across_their_edge_are_folded_from_either`
//!   beside the folds
//! - No draw regresses against `main`, seed by seed, on sets
//!   352000000000000, 353000000000000 and 354000000000000, both kernels —
//!   no test: a campaign fails on any finding, so the walls campaigns above
//!   are run on `main` and on the branch over those seeds and compared seed
//!   by seed, the counts on the pull request. Not met strictly: every seed
//!   holds except the drawn 353000000010626 through the application's body,
//!   a regression the human accepted, carried by #765 and pinned ignored:
//!   `seed_353000000010626_the_same_shaft_as_drawn_is_drawn_uncrossed_through_the_application_s_body`;
//!   the draws found on the way, held on `main` and crossed on the branch,
//!   are pinned, shrunk, and pass:
//!   `seed_354000000005119_a_cone_bored_at_89_99_degrees_by_a_hole_of_radius_2_577_is_drawn_uncrossed`,
//!   `seed_353000000010626_an_elliptic_shaft_bored_along_its_wall_a_hair_inside_then_across_at_75_degrees_is_drawn_uncrossed_through_the_application_s_body`
//!
//! Closes #761.
//! - `seed_354000000005870_a_shaft_bored_at_80_degrees_then_by_a_cone_leaning_24_degrees_is_drawn_uncrossed`
//!   loses its `#[ignore]` and passes — it passes since #766's fold rule
//!   (#762, a fold flip refused only when it leaves no fewer crossings, by
//!   the predicate the soundness check reads), no production change here
//! - No draw regresses against `main`, seed by seed, on the three sets #754
//!   was measured on — no test: nothing outside this test changes, so every
//!   draw is `main`'s and no campaign is needed
//!
//! Closes #765.
//! - The drawn case of 353000000010626, pinned in
//!   `crates/solid/tests/random_walls_at_any_angle.rs`, loses its `#[ignore]`
//!   and passes —
//!   `seed_353000000010626_the_same_shaft_as_drawn_is_drawn_uncrossed_through_the_application_s_body`
//! - No draw regresses against #762's merge, seed by seed, on the three sets,
//!   both kernels — no test: a campaign fails on any finding, so the walls
//!   campaigns above are run on #762's merge and on the branch over sets
//!   352000000000000, 353000000000000 and 354000000000000 and compared seed
//!   by seed, the counts on the pull request; the draws found held on
//!   #762's merge and crossed on the branch, pinned:
//!   `seed_352000000014330_an_elliptic_shaft_bored_at_24_degrees_a_hair_wider_than_it_then_across_and_posted_is_drawn_uncrossed_through_the_application_s_body`
//!   as drawn, its shrunk form crossing on #762's merge too,
//!   `seed_352000000017007_a_shaft_bored_at_30_degrees_by_its_radius_then_posted_a_thousandth_of_a_degree_off_square_is_drawn_uncrossed_through_the_application_s_body`,
//!   `seed_352000000017007_the_same_shaft_as_drawn_is_drawn_uncrossed_through_the_application_s_body`
//!
//! Closes #775.
//! - An edge along a `Meet` curve whose largest x lies between two samples
//!   gives that largest x, to rounding — no test: in this file; it is held
//!   beside `extremes`, in `crates/solid/src/brep/topology/tests/meet.rs`, by
//!   `an_edge_along_a_meet_curve_gives_its_largest_x_where_it_falls_between_two_samples`
//! - The three cases above hold on both kernels and are pinned —
//!   `seed_353000000011653_a_shaft_bored_at_75_degrees_then_by_a_square_a_hair_inside_its_wall_keeps_every_rule`,
//!   `seed_353000000011653_the_same_bores_keep_every_rule_through_the_application_s_body`,
//!   `seed_354000000006300_a_shaft_cut_by_a_cone_then_joined_to_a_square_a_hair_inside_its_wall_keeps_every_rule`,
//!   `seed_354000000006300_the_same_shaft_and_square_keep_every_rule_through_the_application_s_body`,
//!   `seed_354000000017912_a_shaft_bored_at_45_degrees_then_by_a_square_a_hair_from_its_wall_keeps_every_rule`,
//!   `seed_354000000017912_the_same_bores_keep_every_rule_through_the_application_s_body`
//! - No draw regresses against `main`, seed by seed, on the three sets, both
//!   kernels — no test: a campaign fails on any finding, so the walls
//!   campaigns above are run on `main` and on the branch over sets
//!   352000000000000, 353000000000000 and 354000000000000 and compared seed
//!   by seed, the counts on the pull request

#[allow(dead_code, unused_imports)]
mod random_solids;

use cao_solid::brep::{Body, Declined, Surface};
use glam::DVec3;
use random_solids::{Application, Axis, Case, Exact, Kernel, Leaf, Outline, Plane, Section, Step};

/// What a leaf's wall is, as the pairs of #577 name them.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Kind {
    Cylinder,
    Elliptic,
    Cone,
}

fn kind_of(leaf: &Leaf) -> Kind {
    match (leaf, leaf.as_turned()) {
        (_, Some(turned)) if turned.section.slopes() => Kind::Cone,
        (
            Leaf::Prism {
                outline: Outline::Ellipse { .. },
                ..
            },
            _,
        ) => Kind::Elliptic,
        _ => Kind::Cylinder,
    }
}

fn axis_of(leaf: &Leaf) -> DVec3 {
    leaf.as_turned()
        .map_or_else(|| leaf.plane().normal(), |turned| turned.swept(1.0).along)
}

/// A tool against the start: what each wall is, and the sines of the angle
/// between their axes off parallel and off square.
#[derive(Clone, Copy, Debug)]
struct Pair {
    start: Kind,
    tool: Kind,
    off_parallel: f64,
    off_square: f64,
}

impl Pair {
    fn of(case: &Case, tool: &Leaf) -> Pair {
        let (one, other) = (axis_of(&case.start), axis_of(tool));
        Pair {
            start: kind_of(&case.start),
            tool: kind_of(tool),
            off_parallel: one.cross(other).length(),
            off_square: one.dot(other).abs(),
        }
    }

    fn is(&self, one: Kind, other: Kind) -> bool {
        (self.start, self.tool) == (one, other) || (self.start, self.tool) == (other, one)
    }

    fn skew(&self) -> bool {
        self.off_parallel > 1e-3 && self.off_square > 1e-3
    }
}

#[test]
fn the_draw_of_walls_at_any_angle_draws_every_pair_the_issue_names() {
    let cases: Vec<Case> = (0..400).map(Case::drawn_walls_at_any_angle).collect();
    let pairs: Vec<Pair> = cases
        .iter()
        .flat_map(|case| case.steps.iter().map(|step| Pair::of(case, &step.tool)))
        .collect();
    let count = |wanted: &dyn Fn(&Pair) -> bool| pairs.iter().filter(|pair| wanted(pair)).count();
    let families = [
        (
            "two cylinders at a skew angle",
            count(&|pair| pair.is(Kind::Cylinder, Kind::Cylinder) && pair.skew()),
        ),
        (
            "a cylinder against a cone",
            count(&|pair| pair.is(Kind::Cylinder, Kind::Cone)),
        ),
        (
            "two cones of other axes",
            count(&|pair| pair.is(Kind::Cone, Kind::Cone)),
        ),
        (
            "an elliptic wall against a cone",
            count(&|pair| pair.is(Kind::Elliptic, Kind::Cone)),
        ),
        (
            "an elliptic wall at a skew angle to a cylinder",
            count(&|pair| pair.is(Kind::Elliptic, Kind::Cylinder) && pair.skew()),
        ),
        (
            "a pair a hair off square",
            count(&|pair| pair.off_square > 0.0 && pair.off_square < 1e-3),
        ),
        (
            "a pair a hair off parallel",
            count(&|pair| pair.off_parallel > 0.0 && pair.off_parallel < 1e-3),
        ),
    ];
    for (family, drawn) in families {
        assert!(drawn >= 10, "{family}: {drawn} pairs in 400 cases");
    }
    let coned: Vec<&Case> = cases
        .iter()
        .filter(|case| !case.steps.is_empty())
        .filter(|case| case.leaves().any(|leaf| kind_of(leaf) == Kind::Cone))
        .collect();
    let asking = coned
        .iter()
        .filter(|case| case.asks_beyond_the_conics())
        .count();
    assert!(
        asking < coned.len(),
        "{asking} of {} cases with a cone ask beyond the conics: since #577 a cone \
         off its axis is built but where the kernel still declines it",
        coned.len()
    );
    let empty = cases.iter().filter(|case| case.steps.is_empty()).count();
    assert!(empty * 50 < cases.len(), "{empty} cases with no tool");
}

#[test]
fn random_walls_at_any_angle_keep_every_rule_on_the_exact_kernel() {
    let seeds: Vec<u64> = (0..16).collect();
    let weighed = random_solids::on_every_core(&seeds, |seed| {
        let case = Case::drawn_walls_at_any_angle(*seed);
        (random_solids::held_to_arithmetic(&case, &Exact), case)
    });
    for (seed, (measured, case)) in seeds.iter().zip(weighed) {
        assert!(measured.is_ok(), "seed {seed}: {measured:?}\n{case}");
    }
}

/// The case built on the exact kernel, step by step.
fn built(case: &Case) -> Body {
    let answered = |raised: Result<Body, Declined>| {
        raised.unwrap_or_else(|declined| panic!("the kernel answers: {declined:?}"))
    };
    let mut body = answered(Exact.raised(&case.start));
    for step in &case.steps {
        let tool = answered(Exact.raised(&step.tool));
        body = answered(Exact.combined(&body, &tool, step.mode));
    }
    body
}

/// Every pair of distinct walls of a body within a thousandth of square or
/// of parallel stands exactly so — there is at least one — and the step is
/// noted as nothing unstable: what decision 11 asks of walls a hair off.
fn laid_exactly(body: &Body) {
    let mut walls: Vec<Surface> = Vec::new();
    for face in body.listing().faces {
        if !walls.contains(&face.surface) {
            walls.push(face.surface);
        }
    }
    let axes: Vec<DVec3> = walls
        .iter()
        .filter_map(|wall| match wall {
            Surface::Cylinder(wall) => Some(wall.axis),
            Surface::Elliptic(wall) => Some(wall.axis),
            Surface::Cone(wall) => Some(wall.axis),
            _ => None,
        })
        .collect();
    let hairs: Vec<f64> = axes
        .iter()
        .enumerate()
        .flat_map(|(index, one)| {
            axes[index + 1..]
                .iter()
                .map(|other| one.cross(*other).length().min(one.dot(*other).abs()))
        })
        .filter(|sine| *sine < 1e-3)
        .collect();
    assert!(
        !hairs.is_empty() && hairs.iter().all(|sine| *sine == 0.0),
        "walls left a hair off: {hairs:?}"
    );
    assert_eq!(body.unstable(), None);
}

/// A pin 2e-8 degrees off square to a shaft, and a bore exactly parallel
/// to it a hair inside its wall: built crossed (`Uncrossed`) on the
/// campaign's seed 121233920451060, shrunk. In decision 11's band: laid
/// square, and sound.
#[test]
fn seed_121233920451060_a_pin_a_hair_off_square_beside_a_bore_a_hair_inside_the_wall_is_laid_square()
 {
    let case = Case::new(
        Leaf::prism(Plane::xy(0.0), Outline::circle([1.0, 1.0], 4.5), 7.0),
        vec![
            Step::add(Leaf::prism(
                Plane::tilted([-19.0, 1.0, 3.0], [0.0, 89.99999998, 0.0]),
                Outline::circle([-0.0, -3.0], 2.0),
                40.0,
            )),
            Step::cut(Leaf::prism(
                Plane::tilted(
                    [1.0, 1.000000013980673, -17.577453065133664],
                    [0.0, 0.0, 0.0],
                ),
                Outline::circle([-0.0, -0.0], 4.4999999),
                40.0516780926679,
            )),
        ],
    );
    laid_exactly(&built(&case));
    random_solids::holds_exactly(&case);
}

/// A shaft along X, a pin of its radius 2e-8 degrees off square through its
/// axis — two ellipses crossing at two nodes — and a bore 3e-7 degrees off,
/// a hair narrower and a hair aside: declined as a tie on the campaign's
/// seed 121233920454262, shrunk. In decision 11's band: laid square, and
/// sound.
#[test]
fn seed_121233920454262_two_walls_a_hair_off_square_a_hair_from_their_nodes_are_laid_square() {
    let case = Case::new(
        Leaf::prism(Plane::yz(-3.0), Outline::circle([2.0, -1.0], 4.5), 4.0),
        vec![
            Step::add(Leaf::prism(
                Plane::tilted(
                    [-1.0000000065769814, 2.0, -19.841664064126334],
                    [0.0, 2e-8, 0.0],
                ),
                Outline::circle([-0.0, -0.0], 4.5),
                37.68332812825267,
            )),
            Step::cut(Leaf::prism(
                Plane::tilted(
                    [-1.0000000986547224, 2.0, -19.841664064126334],
                    [0.0, 3e-7, 0.0],
                ),
                Outline::circle([-0.0, -1.999999987845058e-8], 4.49999998),
                37.68332812825267,
            )),
        ],
    );
    laid_exactly(&built(&case));
    random_solids::holds_exactly(&case);
}

/// A shaft along X, a pin and an elliptic boss on it 2e-8 and 4e-8 degrees
/// off square: declined as unverified through the application's body on
/// the campaign's seed 187613593885498, shrunk. In decision 11's band: laid
/// square on the exact kernel, and sound through the application's body.
#[test]
fn seed_187613593885498_a_pin_and_an_elliptic_boss_a_hair_off_square_are_laid_square_through_the_application_s_body()
 {
    let case = Case::new(
        Leaf::prism(
            Plane::yz(-120.0),
            Outline::circle([30.0, -45.0], 150.0),
            240.0,
        ),
        vec![
            Step::add(Leaf::prism(
                Plane::tilted([-0.0, 697.0, -45.0], [90.0, 2e-8, 0.0]),
                Outline::circle([-0.0, -90.0], 60.0),
                1335.0,
            )),
            Step::add(Leaf::prism(
                Plane::tilted([-0.0, 697.0, -45.0], [90.0, 4e-8, 0.0]),
                Outline::ellipse([-0.0, -91.0], [-60.0, 0.0], 30.0),
                1335.0,
            )),
        ],
    );
    laid_exactly(&built(&case));
    random_solids::holds_through_the_application(&case);
}

/// The case of the decision of #577: a shaft Ø20 eighty long about Z, cut
/// by another of its radius whose axis crosses its own at its middle,
/// leaning `degrees` off it.
fn nearly_coaxial(degrees: f64) -> Case {
    let (sin, cos) = degrees.to_radians().sin_cos();
    Case::new(
        Leaf::prism(Plane::xy(-40.0), Outline::circle([0.0, 0.0], 10.0), 80.0),
        vec![Step::cut(Leaf::prism(
            Plane::tilted([0.0, 50.0 * sin, -50.0 * cos], [degrees, 0.0, 0.0]),
            Outline::circle([0.0, 0.0], 10.0),
            100.0,
        ))],
    )
}

/// What a kernel answers on a case: its body, or the first decline.
fn answered<K: Kernel>(case: &Case, kernel: &K) -> Result<K::Body, Declined> {
    let mut body = kernel.raised(&case.start)?;
    for step in &case.steps {
        let tool = kernel.raised(&step.tool)?;
        body = kernel.combined(&body, &tool, step.mode)?;
    }
    Ok(body)
}

/// A decline the scope rule excuses, and only as unsupported.
fn declined_along_a_stretch(case: &Case) {
    assert_eq!(
        answered(case, &Exact).err(),
        Some(Declined::Unsupported),
        "{case}"
    );
    assert!(case.asks_of_walls_along_a_stretch(), "{case}");
}

/// The same decline through the application's body too.
fn declined_along_a_stretch_on_both_kernels(case: &Case) {
    declined_along_a_stretch(case);
    assert_eq!(
        answered(case, &Application).err(),
        Some(Declined::Unsupported),
        "{case}"
    );
}

/// Two walls nearly parallel, leaning `degrees` off each other at the last
/// step, built on both kernels, their triangles uncrossed and every other
/// rule kept, and the lean noted unstable.
fn built_uncrossed_and_noted_unstable(case: &Case, degrees: f64) {
    let body = answered(case, &Exact).unwrap_or_else(|declined| panic!("{declined:?}\n{case}"));
    let unstable = body.unstable().expect("a lean in the band is noted");
    assert!(
        matches!(
            unstable.lean,
            cao_solid::Lean::Abreast | cao_solid::Lean::Square
        ),
        "the walls lean abreast, and the tool's wall off square to the part's \
         caps by as much: either is the step's least lean, {:?}\n{case}",
        unstable.lean
    );
    assert!(
        ((unstable.degrees - degrees) / degrees).abs() <= 1e-6,
        "noted {} degrees off, drawn {degrees}",
        unstable.degrees
    );
    random_solids::holds_exactly(case);
    let applied = answered(case, &Application).unwrap_or_else(|declined| {
        panic!("declined through the application: {declined:?}\n{case}")
    });
    assert!(
        Application.unstable(&applied),
        "the application's body notes the lean too\n{case}"
    );
    random_solids::holds_through_the_application(case);
}

#[test]
fn two_shafts_of_one_radius_crossing_nearly_coaxially_at_1e_5_degrees_are_declined_as_unsupported()
{
    declined_along_a_stretch(&nearly_coaxial(1e-5));
}

/// At 1e-4° the same shafts stand closer than a chord's sag of their grids
/// over two steps: declined too, by the second decision of lane 2's rounds.
#[test]
fn the_same_shafts_crossing_at_1e_4_degrees_are_declined_closer_than_a_sag() {
    declined_along_a_stretch(&nearly_coaxial(1e-4));
}

/// An elliptic shaft along X and a round bar of radius 2 leaning 1e-5° off
/// parallel to it, touching its wall from inside: held by the exact kernel
/// on the walls draw's seed 214398260937555 before the decision of #577,
/// which declines it.
#[test]
fn seed_214398260937555_a_bar_touching_an_elliptic_shaft_inside_leaning_1e_5_degrees_is_declined() {
    declined_along_a_stretch(&Case::new(
        Leaf::prism(
            Plane::yz(-3.0),
            Outline::ellipse([0.0, -1.5], [1.0000000000000002, 1.7320508075688772], 2.5),
            6.0,
        ),
        vec![Step::add(Leaf::prism(
            Plane::tilted(
                [12.159812390934299, 0.0, -1.500002122287628],
                [0.0, -89.99999, 0.0],
            ),
            Outline::circle([0.0, 0.384848003542364], 2.0),
            24.319624781868974,
        ))],
    ));
}

/// Two posts of radius 4 whose axes cross leaning 1e-5°: declined as
/// unverified on the walls draw's seed 175696479588841.
#[test]
fn seed_175696479588841_two_posts_of_one_radius_crossing_at_1e_5_degrees_are_declined_as_unsupported()
 {
    declined_along_a_stretch(&Case::new(
        Leaf::prism(Plane::xz(-1.0), Outline::circle([-1.0, 1.0], 4.0), -5.5),
        vec![Step::cut(Leaf::prism(
            Plane::tilted(
                [-0.9999969317555717, 17.790913166907888, 0.9999999999999979],
                [90.0, -1e-5, 0.0],
            ),
            Outline::circle([-0.0, -0.0], 4.0),
            35.15949125081825,
        ))],
    ));
}

/// Seed 175696479588841's two posts of radius 4, the second's axis crossing
/// the first's leaning `degrees` off it.
fn posts_crossing_at(degrees: f64) -> Case {
    posts_from(Outline::circle([-1.0, 1.0], 4.0), Step::cut, degrees)
}

/// The same posts, the first of `outline`, the second taken by `step`: it
/// runs past the first's caps at both ends.
fn posts_from(outline: Outline, step: fn(Leaf) -> Step, degrees: f64) -> Case {
    let along = 17.790913166907888;
    Case::new(
        Leaf::prism(Plane::xz(-1.0), outline, -5.5),
        vec![step(Leaf::prism(
            Plane::tilted(
                [-1.0 + along * degrees.to_radians().tan(), along, 1.0],
                [90.0, -degrees, 0.0],
            ),
            Outline::circle([0.0, 0.0], 4.0),
            35.15949125081825,
        ))],
    )
}

#[test]
fn two_posts_of_one_radius_crossing_nearly_coaxially_at_1e_3_degrees_are_declined_as_unsupported() {
    declined_along_a_stretch(&posts_crossing_at(1e-3));
}

/// Two shafts of radius 150 whose axes cross leaning 1e-4°, closer than a
/// chord's sag along a stretch: built with crossed triangles on the walls
/// draw's seed 224442167739556, shrunk.
#[test]
fn seed_224442167739556_two_shafts_of_radius_150_crossing_at_1e_4_degrees_are_declined_as_unsupported()
 {
    declined_along_a_stretch(&Case::new(
        Leaf::prism(
            Plane::xz(-60.0),
            Outline::circle([0.0, 30.0], 150.0),
            -300.0,
        ),
        vec![Step::cut(Leaf::prism(
            Plane::tilted(
                [0.0, 753.7553595345719, 29.99877894105255],
                [89.9999, 0.0, 0.0],
            ),
            Outline::circle([-0.0, -0.0], 150.0),
            1399.0,
        ))],
    ));
}

/// A post of radius 22.5 bored at 30°, then cut by another of its radius
/// whose axis crosses its own leaning 1e-4°: built with crossed triangles on
/// the walls draw's seed 224442167740434, shrunk.
#[test]
fn seed_224442167740434_two_posts_of_radius_22_5_crossing_at_1e_4_degrees_are_declined_as_unsupported()
 {
    declined_along_a_stretch(&Case::new(
        Leaf::prism(Plane::xy(-15.0), Outline::circle([-5.0, 5.0], 22.5), 40.0),
        vec![
            Step::cut(Leaf::prism(
                Plane::tilted(
                    [-5.0, -46.33324094593226, -83.91178143552975],
                    [-30.0, 0.0, 0.0],
                ),
                Outline::circle([-0.0, -0.0], 10.0),
                205.0,
            )),
            Step::cut(Leaf::prism(
                Plane::tilted(
                    [-4.999820813185955, 5.0, -97.66648189170816],
                    [0.0, -0.0001, 0.0],
                ),
                Outline::circle([0.0, 0.0], 22.5),
                205.0,
            )),
        ],
    ));
}

/// A shaft of radius 4 turned about Y, cut by one of radius 6 turned about
/// a line five from it leaning `degrees`: their walls cross, the first's
/// inside the second's at one side and outside at the other (#577's
/// decision on walls a sliver past decision 11, from the slanted draw's seed
/// 134820673287757).
fn crossing_a_sliver_past_decision_11(degrees: f64) -> Case {
    Case::new(
        Leaf::turned(
            Plane::xy(0.0),
            Axis::second(0.0).backwards(),
            Section::bands(-8.0, &[[7.0, 0.0, 4.0]]),
            360.0,
        ),
        vec![Step::cut(Leaf::turned(
            Plane::xy(0.0),
            Axis::second(5.0).leaning(-degrees),
            Section::bands(1.0, &[[4.0, 0.0, 6.0]]),
            360.0,
        ))],
    )
}

#[test]
fn two_shafts_crossing_at_1e_5_degrees_a_sliver_past_decision_11_are_declined_as_unsupported() {
    declined_along_a_stretch(&crossing_a_sliver_past_decision_11(1e-5));
}

#[test]
fn the_same_shafts_crossing_at_3e_5_degrees_are_built_and_keep_every_rule() {
    let case = crossing_a_sliver_past_decision_11(3e-5);
    let body = answered(&case, &Exact).unwrap_or_else(|declined| panic!("{declined:?}\n{case}"));
    let unstable = body.unstable().expect("a lean in the band is noted");
    assert!(
        matches!(
            unstable.lean,
            cao_solid::Lean::Abreast | cao_solid::Lean::Square
        ),
        "the walls lean abreast, and the tool's ends off square to the shaft's \
         axis by as much: either is the step's least lean, {:?}\n{case}",
        unstable.lean
    );
    assert!(
        ((unstable.degrees - 3e-5) / 3e-5).abs() <= 1e-6,
        "noted {} degrees off, drawn 3e-5",
        unstable.degrees
    );
    random_solids::holds_exactly(&case);
}

/// A post of radius 5 and a bore of radius 1.5 crossing its wall, leaning
/// 1e-5°: declined as unverified at the bore's caps on the tilted draw's
/// seed 12099367009951, shrunk.
#[test]
fn seed_12099367009951_a_bore_crossing_a_post_s_wall_at_1e_5_degrees_is_declined_as_unsupported() {
    declined_along_a_stretch(&Case::new(
        Leaf::prism(Plane::xy(0.0), Outline::circle([3.0, 10.0], 5.0), 9.0),
        vec![Step::cut(Leaf::prism(
            Plane::tilted(
                [2.9999998254653297, 10.0, 3.4999900000000155],
                [0.0, 1e-5, 0.0],
            ),
            Outline::circle([4.5, 0.0], 1.5),
            6.0,
        ))],
    ));
}

/// A post of radius 10 and a bore of radius 7.5 inside it leaning 1e-3°,
/// its axis drifting across the line their walls would touch along, the
/// gap growing as the square of the height: declined by the kernel on the
/// tilted draw's seed 12099367018660, reduced, and excused by the scope rule
/// only once it read that bound.
#[test]
fn seed_12099367018660_a_bore_drifting_across_its_line_of_touch_is_declined_and_excused() {
    declined_along_a_stretch(&Case::new(
        Leaf::prism(Plane::xz(30.0), Outline::circle([25.0, 0.0], 10.0), 38.0),
        vec![Step::cut(Leaf::prism(
            Plane::tilted(
                [25.0, 39.76081505401357, -5.000497607595171],
                [89.999, 0.0, 0.0],
            ),
            Outline::circle([-2.5, 5.0], 7.5),
            57.0,
        ))],
    ));
}

/// A rounded rectangle's corner of radius 1 and a bore of radius 0.5 inside
/// it leaning 1e-5°: declined by the kernel on the tilted draw's seed
/// 12099367023529, reduced, and excused by the scope rule only once it read
/// the arcs of a rounded outline.
#[test]
fn seed_12099367023529_a_bore_in_a_rounded_corner_leaning_1e_5_degrees_is_declined_and_excused() {
    declined_along_a_stretch(&Case::new(
        Leaf::prism(
            Plane::xy(-1.0),
            Outline::rounded([7.0, 8.0], [9.0, 12.0], 1.0),
            9.0,
        ),
        vec![Step::cut(Leaf::prism(
            Plane::tilted([8.0, 10.0, -2.0], [0.0, -1e-5, 0.0]),
            Outline::circle([0.0, 0.5], 0.5),
            11.0,
        ))],
    ));
}

/// A block with a round pocket in its top, bored at 75° through the
/// pocket's wall and floor: two walls at a skew angle, far from parallel,
/// declined as unsupported on the tilted ellipses draw's seed
/// 106731298824406, shrunk. Nothing here leans in the band, and the scope
/// rule rightly excuses nothing.
#[test]
#[ignore = "an oblique family lane 2 leaves, #707"]
fn seed_106731298824406_a_bore_at_75_degrees_through_a_pocket_s_wall_keeps_every_rule() {
    let case = Case::new(
        Leaf::prism(
            Plane::xy(1.0),
            Outline::rectangle([2.0, 5.0], [8.0, 12.0]),
            7.0,
        ),
        vec![
            Step::cut(Leaf::prism(
                Plane::xy(8.5),
                Outline::circle([4.75, 8.25], 2.75),
                -1.5,
            )),
            Step::cut(Leaf::prism(
                Plane::tilted(
                    [0.0914323562073589, 8.25, 6.501740561960004],
                    [0.0, 75.0, 0.0],
                ),
                Outline::circle([0.0, -1.0], 1.0),
                10.0,
            )),
        ],
    );
    assert!(!case.asks_beyond_the_conics(), "{case}");
    assert!(answered(&case, &Exact).is_ok(), "{case}");
    random_solids::holds_exactly(&case);
}

#[test]
fn the_same_posts_crossing_at_3e_4_or_1e_4_degrees_are_declined_closer_than_a_sag() {
    declined_along_a_stretch(&posts_crossing_at(3e-4));
    declined_along_a_stretch(&posts_crossing_at(1e-4));
}

/// The shaft of `nearly_coaxial`, cut by a bore of `radius` drawn with
/// `shaft` as its outline's first half-axis, its axis shifted `off` along X
/// and leaning `degrees` about it.
fn a_shaft_bored_nearly_coaxially(shaft: Outline, radius: f64, off: f64, degrees: f64) -> Case {
    let (sin, cos) = degrees.to_radians().sin_cos();
    Case::new(
        Leaf::prism(Plane::xy(-40.0), shaft, 80.0),
        vec![Step::cut(Leaf::prism(
            Plane::tilted([off, 50.0 * sin, -50.0 * cos], [degrees, 0.0, 0.0]),
            Outline::circle([0.0, 0.0], radius),
            100.0,
        ))],
    )
}

/// Seed 318277455103234's block, from -11 to -1 along Y, bored by a pin of
/// radius 0.5 leaning 0.01°, then refilled by a bar of its radius square to
/// the block, from `from` along Y over `length` towards -Y.
fn block_bored_and_refilled(from: f64, length: f64) -> Case {
    Case::new(
        Leaf::prism(
            Plane::xz(-1.0),
            Outline::rectangle([-3.0, -4.0], [7.0, 2.0]),
            10.0,
        ),
        vec![
            Step::cut(Leaf::prism(
                Plane::tilted(
                    [1.9966206426438418, 21.165917727536772, -1.0000000000000027],
                    [90.0, 0.01, 0.0],
                ),
                Outline::circle([-0.0, -0.0], 0.5),
                39.0,
            )),
            Step::add(Leaf::prism(
                Plane::tilted([2.0, from, -1.0], [90.0, 0.0, 0.0]),
                Outline::circle([-0.0, -0.0], 0.5),
                length,
            )),
        ],
    )
}

/// At 1e-4° the shafts of `nearly_coaxial` stand closer than a chord's sag
/// of their grids over two steps.
#[test]
#[ignore = "#706: built uncrossed only once the two walls share one order beside the crossing line"]
fn the_same_shafts_crossing_at_1e_4_degrees_are_built_uncrossed_and_noted_unstable() {
    built_uncrossed_and_noted_unstable(&nearly_coaxial(1e-4), 1e-4);
}

#[test]
#[ignore = "#706: built uncrossed only once the two walls share one order beside the crossing line"]
fn the_same_posts_crossing_at_3e_4_or_1e_4_degrees_are_built_uncrossed_and_noted_unstable() {
    built_uncrossed_and_noted_unstable(&posts_crossing_at(3e-4), 3e-4);
    built_uncrossed_and_noted_unstable(&posts_crossing_at(1e-4), 1e-4);
}

#[test]
#[ignore = "#706: built uncrossed only once the two walls share one order beside the crossing line"]
fn two_posts_of_one_radius_crossing_nearly_coaxially_at_1e_3_degrees_are_built_uncrossed_and_noted_unstable()
 {
    built_uncrossed_and_noted_unstable(&posts_crossing_at(1e-3), 1e-3);
}

/// The shafts of
/// `seed_224442167739556_two_shafts_of_radius_150_crossing_at_1e_4_degrees_are_declined_as_unsupported`,
/// built.
#[test]
#[ignore = "#706: built uncrossed only once the two walls share one order beside the crossing line"]
fn seed_224442167739556_two_shafts_of_radius_150_crossing_at_1e_4_degrees_are_built_uncrossed_and_noted_unstable()
 {
    built_uncrossed_and_noted_unstable(
        &Case::new(
            Leaf::prism(
                Plane::xz(-60.0),
                Outline::circle([0.0, 30.0], 150.0),
                -300.0,
            ),
            vec![Step::cut(Leaf::prism(
                Plane::tilted(
                    [0.0, 753.7553595345719, 29.99877894105255],
                    [89.9999, 0.0, 0.0],
                ),
                Outline::circle([-0.0, -0.0], 150.0),
                1399.0,
            ))],
        ),
        1e-4,
    );
}

/// A shaft of radius 15 about Z cut by a bore of its radius leaning 0.001°
/// off it, no other wall: drawn with crossed triangles through the
/// application's body by #706's attempt, on the walls draw's seed
/// 318277455102885, shrunk.
#[test]
#[ignore = "#706: built uncrossed only once the two walls share one order beside the crossing line"]
fn seed_318277455102885_a_shaft_cut_by_a_bore_of_its_radius_at_0_001_degrees_is_built_uncrossed_and_noted_unstable()
 {
    built_uncrossed_and_noted_unstable(
        &Case::new(
            Leaf::prism(Plane::xy(-15.0), Outline::circle([-5.0, 0.0], 15.0), 20.0),
            vec![Step::cut(Leaf::prism(
                Plane::tilted(
                    [-4.998832302166697, 0.0, -72.43055956539959],
                    [0.0, -0.001, 0.0],
                ),
                Outline::circle([0.0, 0.0], 15.0),
                134.0,
            ))],
        ),
        1e-3,
    );
}

/// A shaft of radius 17.5 about Z cut by a bore of its radius leaning 0.01°,
/// their axes crossing six and a half below the top cap: drawn with crossed
/// triangles on both kernels by #706's attempt, on the walls draw's seed
/// 331906274213776, shrunk.
#[test]
#[ignore = "#706: built uncrossed only once the two walls share one order beside the crossing line"]
fn seed_331906274213776_a_shaft_cut_by_a_bore_of_its_radius_at_0_01_degrees_is_built_uncrossed_and_noted_unstable()
 {
    built_uncrossed_and_noted_unstable(
        &Case::new(
            Leaf::prism(Plane::xy(0.0), Outline::circle([5.0, -10.0], 17.5), 20.0),
            vec![Step::cut(Leaf::prism(
                Plane::tilted(
                    [5.013244499249297, -10.0, -62.42770816231506],
                    [0.0, -0.01, 0.0],
                ),
                Outline::circle([-0.0, -0.0], 17.5),
                152.0,
            ))],
        ),
        1e-2,
    );
}

/// A shaft of radius 5 about Z cut by a bore of its radius leaning 1e-4°,
/// their axes crossing a quarter below the top cap: drawn with crossed
/// triangles through the application's body by #706's attempt, on the walls
/// draw's seed 331906274218763, shrunk.
#[test]
#[ignore = "#706: built uncrossed only once the two walls share one order beside the crossing line"]
fn seed_331906274218763_a_shaft_cut_by_a_bore_of_its_radius_at_1e_4_degrees_is_built_uncrossed_and_noted_unstable()
 {
    built_uncrossed_and_noted_unstable(
        &Case::new(
            Leaf::prism(Plane::xy(0.0), Outline::circle([-1.0, 1.0], 5.0), 4.5),
            vec![Step::cut(Leaf::prism(
                Plane::tilted(
                    [-1.0, 1.000036374096411, -16.588113279919327],
                    [0.0001, 0.0, 0.0],
                ),
                Outline::circle([-0.0, -0.0], 5.0),
                42.0,
            ))],
        ),
        1e-4,
    );
}

#[test]
#[ignore = "#706: built uncrossed only once the two walls share one order beside the crossing line"]
fn a_bore_a_hundredth_inside_a_shaft_s_wall_leaning_1e_4_degrees_never_meeting_it_is_built_uncrossed_and_noted_unstable()
 {
    built_uncrossed_and_noted_unstable(
        &a_shaft_bored_nearly_coaxially(Outline::circle([0.0, 0.0], 10.0), 9.99, 0.0, 1e-4),
        1e-4,
    );
}

/// A bore a hundredth narrower than the shaft, shifted two hundredths off
/// its axis: their walls cross along two lines sixty degrees either side of
/// the shift, within a chord's sag of each other all round.
#[test]
#[ignore = "#706: built uncrossed only once the two walls share one order beside the crossing line"]
fn a_narrower_bore_off_the_axis_crossing_a_shaft_s_wall_at_1e_4_degrees_is_built_uncrossed_and_noted_unstable()
 {
    built_uncrossed_and_noted_unstable(
        &a_shaft_bored_nearly_coaxially(Outline::circle([0.0, 0.0], 10.0), 9.99, 0.02, 1e-4),
        1e-4,
    );
}

/// A tube of radius 4 bored 2, cut by a post of its outer radius leaning
/// 1e-3°: the tube's own bore is no third wall meeting the pair.
#[test]
#[ignore = "#706: built uncrossed only once the two walls share one order beside the crossing line"]
fn a_tube_cut_by_a_post_of_its_outer_radius_at_1e_3_degrees_is_built_uncrossed_and_noted_unstable()
{
    built_uncrossed_and_noted_unstable(
        &posts_from(Outline::ring([-1.0, 1.0], 4.0, 2.0), Step::cut, 1e-3),
        1e-3,
    );
}

/// The posts at 1e-3°, the second added rather than cut, running past the
/// first's caps: those caps are the pair's own, no third leaf's.
#[test]
#[ignore = "#706: built uncrossed only once the two walls share one order beside the crossing line"]
fn a_post_added_to_one_of_its_radius_at_1e_3_degrees_past_its_caps_is_built_uncrossed_and_noted_unstable()
 {
    built_uncrossed_and_noted_unstable(
        &posts_from(Outline::circle([-1.0, 1.0], 4.0), Step::add, 1e-3),
        1e-3,
    );
}

/// The same block and bore, the bar ending flush on both of the block's
/// faces: both walls end on each face, and nothing passes through it.
#[test]
#[ignore = "#706: built uncrossed only once the two walls share one order beside the crossing line"]
fn the_same_bore_refilled_by_a_bar_ending_flush_on_both_faces_is_built_uncrossed_and_noted_unstable()
 {
    built_uncrossed_and_noted_unstable(&block_bored_and_refilled(-1.0, 10.0), 1e-2);
}

#[test]
#[ignore = "#706: built uncrossed only once the two walls share one order beside the crossing line"]
fn two_walls_closer_than_a_sag_are_drawn_uncrossed_and_closed_three_and_ten_times_coarser_than_the_campaigns()
 {
    for case in [
        nearly_coaxial(1e-4),
        posts_crossing_at(1e-3),
        posts_crossing_at(1e-4),
        posts_from(Outline::circle([-1.0, 1.0], 4.0), Step::add, 1e-3),
        Case::new(
            Leaf::prism(Plane::xy(-15.0), Outline::circle([-5.0, 0.0], 15.0), 20.0),
            vec![Step::cut(Leaf::prism(
                Plane::tilted(
                    [-4.998832302166697, 0.0, -72.43055956539959],
                    [0.0, -0.001, 0.0],
                ),
                Outline::circle([0.0, 0.0], 15.0),
                134.0,
            ))],
        ),
    ] {
        let body = built(&case);
        for share in [3e-3, 1e-2] {
            let tolerance = share * body.scale().reach();
            let triangles = body.triangles(tolerance);
            assert_eq!(
                cao_solid::soundness::uncrossed(&triangles),
                Ok(()),
                "drawn within {tolerance}\n{case}"
            );
            assert_eq!(
                cao_solid::soundness::closed(&triangles),
                Ok(()),
                "drawn within {tolerance}\n{case}"
            );
        }
    }
}

/// An elliptic shaft of half-axes 10 and 9.995 bored round by 9.99 leaning
/// 1e-4°: their walls never meet, within a chord's sag of each other all
/// round, and #706's attempt kept the decline for an elliptic wall.
#[test]
fn an_elliptic_shaft_bored_round_closer_than_a_sag_at_1e_4_degrees_is_declined_as_unsupported() {
    declined_along_a_stretch_on_both_kernels(&a_shaft_bored_nearly_coaxially(
        Outline::ellipse([0.0, 0.0], [10.0, 0.0], 9.995),
        9.99,
        0.0,
        1e-4,
    ));
}

/// A shaft of radius 60 cut by another of its radius leaning 1e-4°, then
/// bored at 30° by one of radius 45 whose wall meets both where they stand
/// closer than a chord's sag: drawn with crossed triangles by #706's attempt
/// on the walls draw's seed 318277455101196, shrunk, which then declined it.
#[test]
fn seed_318277455101196_two_shafts_of_one_radius_at_1e_4_degrees_bored_at_30_degrees_are_declined_as_unsupported()
 {
    declined_along_a_stretch_on_both_kernels(&Case::new(
        Leaf::prism(
            Plane::xz(-30.0),
            Outline::circle([-60.0, 30.0], 60.0),
            -135.0,
        ),
        vec![
            Step::cut(Leaf::prism(
                Plane::tilted([-60.0, 381.0, 30.0], [90.0, -0.0001, 0.0]),
                Outline::circle([0.0, 0.0], 60.0),
                614.0,
            )),
            Step::cut(Leaf::prism(
                Plane::tilted([-60.0, 161.0, -241.0], [30.0, 0.0, 0.0]),
                Outline::circle([15.0, 0.0], 45.0),
                614.0,
            )),
        ],
    ));
}

/// A shaft of radius 25 cut at 44.9° by one of its radius, then by another
/// of its radius leaning 0.05° off it, whose wall the first cut meets where
/// the two stand closer than a chord's sag: drawn with crossed triangles by
/// #706's attempt on the walls draw's seed 318277455100734, shrunk, which
/// then declined it.
#[test]
fn seed_318277455100734_two_shafts_of_one_radius_at_0_05_degrees_beside_a_cut_at_45_degrees_are_declined_as_unsupported()
 {
    declined_along_a_stretch_on_both_kernels(&Case::new(
        Leaf::prism(
            Plane::xz(-10.0),
            Outline::circle([10.0, -10.0], 25.0),
            -25.0,
        ),
        vec![
            Step::cut(Leaf::prism(
                Plane::tilted(
                    [84.10773177279917, 76.88443447046465, -10.000000000000016],
                    [90.0, -44.89323424388429, 0.0],
                ),
                Outline::circle([0.0, 0.0], 25.0),
                210.0,
            )),
            Step::cut(Leaf::prism(
                Plane::tilted(
                    [10.09162977409969, 112.16576409920087, -10.000000000000023],
                    [90.0, -0.05, 0.0],
                ),
                Outline::circle([-0.0, -0.0], 25.0),
                210.0,
            )),
        ],
    ));
}

/// A block bored by a pin of radius 0.5 leaning 0.01°, the bore then filled
/// by a bar of its radius square to the block, running out through both of
/// its faces: one wall passes through a face where the other ends on it.
/// Drawn with crossed triangles on that face through the application's body
/// by #706's attempt on the walls draw's seed 318277455103234, shrunk, which
/// then declined it.
#[test]
fn seed_318277455103234_a_bore_of_radius_0_5_at_0_01_degrees_refilled_by_a_bar_running_through_the_block_is_declined_as_unsupported()
 {
    declined_along_a_stretch_on_both_kernels(&block_bored_and_refilled(23.0, 39.0));
}

/// A turned part about Z leaning 1e-5°, the hole of its section turning a
/// groove whose wall the end of a slot crosses: declined by the kernel on
/// the slanted draw's seed 252118752294648, and excused by the scope rule
/// only once it read a turned section's holes.
#[test]
fn seed_252118752294648_a_slot_s_end_crossing_a_turned_groove_at_1e_5_degrees_is_declined_and_excused()
 {
    declined_along_a_stretch(&Case::new(
        Leaf::prism(
            Plane::xy(8.0),
            Outline::slot([1.5, 1.0], [1.5, -4.5], 1.0),
            -8.5,
        ),
        vec![
            Step::cut(Leaf::prism(
                Plane::xy(7.9999999),
                Outline::circle([1.0, 4.0], 4.0),
                1.0,
            )),
            Step::cut(Leaf::turned(
                Plane::xz(0.0),
                Axis::second(0.0).leaning(-1e-5),
                Section::bands(-3.0, &[[6.0, 3.0, 7.5]]).with_holes(&[([-1.0, 5.5], [0.0, 7.0])]),
                360.0,
            )),
        ],
    ));
}

/// A ring of radius 5.5, a thin rounded rectangle leaning 1e-5° added on it,
/// one of its corners crossing the ring's wall: declined by the kernel on
/// the tilted draw's seed 3408641411362, and excused by the scope rule only
/// once it measured the tolerance by the reach of the step it is read at.
#[test]
fn seed_3408641411362_a_rounded_corner_crossing_a_ring_s_wall_at_1e_5_degrees_is_declined_and_excused()
 {
    declined_along_a_stretch(&Case::new(
        Leaf::prism(Plane::xy(7.0), Outline::ring([6.0, 6.0], 5.5, 2.0), 8.5),
        vec![
            Step::cut(Leaf::prism(
                Plane::tilted([6.0, 0.4999999999999651, 15.5000002], [1e-5, 0.0, 0.0]),
                Outline::rectangle([-6.0, -1.0], [6.0, 11.5]),
                0.5,
            )),
            Step::add(Leaf::prism(
                Plane::tilted(
                    [6.0, 0.5000001745328903, 14.500000200000017],
                    [1e-5, 0.0, 0.0],
                ),
                Outline::rounded([3.0, 1.0], [4.0, 5.5], 0.5),
                0.4999999,
            )),
            Step::cut(Leaf::prism(
                Plane::tilted([6.0, 6.0, 16.0], [30.0, 45.0, 0.0]),
                Outline::rectangle([-7.5, -6.5], [7.5, 6.0]),
                -7.5,
            )),
        ],
    ));
}

/// A block, a bore at 84°, and a bore 1.8e-8 off square to it: declined on
/// the tilted draw's seed 3408641393912, where decision 11 should lay the
/// two square and the kernel build them.
#[test]
fn seed_3408641393912_two_bores_a_hair_off_square_to_each_other_keep_every_rule() {
    let case = Case::new(
        Leaf::prism(
            Plane::xy(1.0),
            Outline::rectangle([1.0, -1.0], [9.0, 2.0]),
            3.0,
        ),
        vec![
            Step::cut(Leaf::prism(
                Plane::tilted(
                    [-0.4712399506834357, 0.5, 1.9512255678448192],
                    [0.0, 84.0, 0.0],
                ),
                Outline::circle([-1.0, -1.0], 0.5),
                11.0,
            )),
            Step::cut(Leaf::prism(
                Plane::tilted(
                    [4.75, 5.749999999999919, 2.499999083702142],
                    [89.99999, 0.0, 0.0],
                ),
                Outline::circle([0.0, 0.0], 1.0),
                11.0,
            )),
        ],
    );
    assert!(answered(&case, &Exact).is_ok(), "{case}");
    random_solids::holds_exactly(&case);
}

/// Two posts of radius 4.5, the second a ten-millionth wider and off, at
/// 45.5° to the first: two ellipses crossing at two nodes, a hair off;
/// built with a volume off the promise (seed 86751972229989, shrunk).
#[test]
fn seed_86751972229989_two_posts_of_one_radius_crossing_at_45_degrees_keep_every_rule() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(Plane::yz(-1.0), Outline::circle([-1.0, -1.0], 4.5), 8.0),
        vec![Step::add(Leaf::prism(
            Plane::tilted([-10.0, -1.0, -16.0], [0.0, 44.489716596826945, 0.0]),
            Outline::circle([0.0, 1.000000002804313e-7], 4.5000001),
            41.06659275674582,
        ))],
    ));
}

fn a_shaft_with_a_post_a_thousandth_of_a_degree_off_touching_it_inside(cut: Option<Step>) -> Case {
    Case::new(
        Leaf::prism(Plane::yz(-5.0), Outline::circle([0.0, 7.5], 15.0), 23.0),
        cut.into_iter()
            .chain([Step::add(Leaf::prism(
                Plane::tilted(
                    [
                        -62.899457621699895,
                        0.0011872328530238138,
                        7.4999999999999964,
                    ],
                    [90.0, 89.999, 0.0],
                ),
                Outline::circle([-0.0, -7.4999899999999995], 7.5),
                136.0,
            ))])
            .collect(),
    )
}

/// A shaft of radius 15 about X joined to a post of half its radius leaning
/// 0.001° off it, touching it inside along its top: held on `main`, left
/// open beside the line they touch along by #706's second step (seed
/// 352000000014902, shrunk).
#[test]
fn seed_352000000014902_a_post_touching_a_shaft_inside_a_thousandth_of_a_degree_off_keeps_every_rule()
 {
    random_solids::holds_exactly(
        &a_shaft_with_a_post_a_thousandth_of_a_degree_off_touching_it_inside(None),
    );
}

/// The same, a slot cut through the shaft first: answered on `main`,
/// refused through the application's body by #706's second step.
#[test]
fn seed_352000000014902_the_same_post_beside_a_slot_keeps_every_rule_through_the_application_s_body()
 {
    random_solids::holds_through_the_application(
        &a_shaft_with_a_post_a_thousandth_of_a_degree_off_touching_it_inside(Some(Step::cut(
            Leaf::prism(
                Plane::xy(-51.0),
                Outline::rectangle([-5.0, 2.0], [5.0, 17.0]),
                136.0,
            ),
        ))),
    );
}

/// A shaft of radius 2.5 about Y bored by one of its radius two thousandths
/// off its axis, then across along Z, then by one of its radius at ten
/// degrees: on `main`, the third bore, snapped a few ulps onto its touch at
/// one meet, was taken for a surface the meet grazes at the vertex, and the
/// mesh opened on the exact kernel and crossed through the application's
/// body (seed 352000000000597, shrunk on #706's lift).
fn a_shaft_bored_by_its_radius_two_thousandths_off_its_axis_then_twice_more() -> Case {
    Case::new(
        Leaf::prism(Plane::xz(-0.5), Outline::circle([0.0, 1.0], 2.5), -6.0),
        vec![
            Step::cut(Leaf::prism(
                Plane::tilted(
                    [
                        -0.002174372436540451,
                        14.708236243834666,
                        0.9999999999999983,
                    ],
                    [90.0, 0.0, 0.0],
                ),
                Outline::circle([0.0, 0.0], 2.5),
                25.0,
            )),
            Step::cut(Leaf::prism(
                Plane::xy(-8.0),
                Outline::circle([1.0, 0.0], 1.5),
                25.0,
            )),
            Step::cut(Leaf::prism(
                Plane::tilted([0.0, 16.0, -1.0], [79.89117639215662, 0.0, 0.0]),
                Outline::circle([0.0, 0.0], 2.5),
                25.0,
            )),
        ],
    )
}

#[test]
fn seed_352000000000597_a_shaft_bored_by_its_radius_two_thousandths_off_its_axis_then_twice_more_keeps_every_rule()
 {
    random_solids::holds_exactly(
        &a_shaft_bored_by_its_radius_two_thousandths_off_its_axis_then_twice_more(),
    );
}

#[test]
fn seed_352000000000597_the_same_bores_keep_every_rule_through_the_application_s_body() {
    random_solids::holds_through_the_application(
        &a_shaft_bored_by_its_radius_two_thousandths_off_its_axis_then_twice_more(),
    );
}

/// A shaft of radius 4 about Z bored at 15° by one of radius 2, then by one
/// of its own radius leaning 1e-4° off it: the skew family #706's lift
/// reaches, built open on the lift where a third wall's snapped copy was
/// taken for another surface (seed 352000000002028, shrunk on the lift).
fn a_shaft_bored_at_15_degrees_then_by_its_radius_at_1e_4_degrees() -> Case {
    Case::new(
        Leaf::prism(Plane::xy(-3.0), Outline::circle([-1.5, 0.0], 4.0), 6.0),
        vec![
            Step::cut(Leaf::prism(
                Plane::tilted(
                    [-1.5, 4.608596227122955, -17.199515271193107],
                    [15.0, 0.0, 0.0],
                ),
                Outline::circle([-2.0, -0.0], 2.0),
                36.0,
            )),
            Step::cut(Leaf::prism(
                Plane::tilted(
                    [-1.5, -3.107776633144674e-5, -19.273794477160152],
                    [-0.0001, 0.0, 0.0],
                ),
                Outline::circle([-0.0, -0.0], 4.0),
                36.0,
            )),
        ],
    )
}

#[test]
fn seed_352000000002028_a_shaft_bored_at_15_degrees_then_by_its_radius_at_1e_4_degrees_is_declined_as_unsupported()
 {
    declined_along_a_stretch_on_both_kernels(
        &a_shaft_bored_at_15_degrees_then_by_its_radius_at_1e_4_degrees(),
    );
}

#[test]
#[ignore = "#706: built uncrossed only once the two walls share one order beside the crossing line"]
fn seed_352000000002028_a_shaft_bored_at_15_degrees_then_by_its_radius_at_1e_4_degrees_is_built_uncrossed_and_noted_unstable()
 {
    built_uncrossed_and_noted_unstable(
        &a_shaft_bored_at_15_degrees_then_by_its_radius_at_1e_4_degrees(),
        1e-4,
    );
}

/// A shaft of radius 20 bored by one of radius 7.5 at 80°, then by a cone
/// leaning 24°: held on `main`, drawn crossed on the exact kernel once a
/// wall's snapped copy is told for its own (seed 354000000005870).
#[test]
fn seed_354000000005870_a_shaft_bored_at_80_degrees_then_by_a_cone_leaning_24_degrees_is_drawn_uncrossed()
 {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(Plane::xz(-5.0), Outline::circle([10.0, 0.0], 20.0), -42.5),
        vec![
            Step::cut(Leaf::prism(
                Plane::tilted(
                    [10.0, 110.5501270299569, -16.627656679617015],
                    [80.0, 0.0, 0.0],
                ),
                Outline::circle([12.499999999999998, 0.0], 7.5),
                191.50971698084905,
            )),
            Step::cut(Leaf::turned(
                Plane::xy(-1.0000000000000001e-7),
                Axis::first(11.797713146914639).leaning(24.0),
                Section::bands(-84.80849570536405, &[[191.50971698084905, 0.0, 5.0]])
                    .sloping_to(&[[0.0, 2.5]]),
                360.0,
            )),
        ],
    ));
}

/// An elliptic shaft of half-axes 90 and 75 bored by a round bore of radius
/// 75 a thousandth of a degree off parallel, touching its wall along a line:
/// held on `main`, drawn crossed once a wall's snapped copy is told for its
/// own (seed 354000000008476, shrunk).
fn an_elliptic_shaft_bored_round_along_its_wall_a_thousandth_of_a_degree_off() -> Case {
    Case::new(
        Leaf::prism(
            Plane::yz(-120.0),
            Outline::ellipse([-60.0, 0.0], [90.0, 0.0], 75.0),
            135.0,
        ),
        vec![Step::cut(Leaf::prism(
            Plane::tilted(
                [
                    322.91634560261997,
                    -59.99344774870274,
                    -2.0840884704471192e-14,
                ],
                [90.0, -89.999, 0.0],
            ),
            Outline::circle([-0.0, -0.0], 75.0),
            751.0,
        ))],
    )
}

#[test]
fn seed_354000000008476_an_elliptic_shaft_bored_round_along_its_wall_a_thousandth_of_a_degree_off_is_drawn_uncrossed()
 {
    random_solids::holds_exactly(
        &an_elliptic_shaft_bored_round_along_its_wall_a_thousandth_of_a_degree_off(),
    );
}

#[test]
fn seed_354000000008476_the_same_bore_is_drawn_uncrossed_through_the_application_s_body() {
    random_solids::holds_through_the_application(
        &an_elliptic_shaft_bored_round_along_its_wall_a_thousandth_of_a_degree_off(),
    );
}

/// A tube of radii 20 and 10 cut by an elliptic slot whose half-axis of 10
/// touches the tube's bore along a line, then by a turned band, then joined
/// to a post of radius 10 leaning about 38° off the tube's axis: held on
/// `main`, drawn crossed once a wall's snapped copy is told for its own
/// (seed 352000000007085).
fn a_tube_slotted_along_its_bore_banded_and_posted() -> Case {
    Case::new(
        Leaf::prism(
            Plane::xz(-15.0),
            Outline::ring([0.0, 0.0], 20.0, 10.0),
            -42.5,
        ),
        vec![
            Step::cut(Leaf::prism(
                Plane::tilted(
                    [0.0, -89.50485849042452, -3.342475995830307e-8],
                    [-89.99999998, 0.0, 0.0],
                ),
                Outline::ellipse([-1e-5, -0.0], [0.0, 5.0], 10.0),
                191.50971698084905,
            )),
            Step::cut(Leaf::turned(
                Plane::xy(1e-5),
                Axis::first(21.040500500744173).leaning(1e-7),
                Section::bands(-95.75485849042452, &[[191.50971698084905, 0.0, 2.5]]),
                360.0,
            )),
            Step::add(Leaf::prism(
                Plane::tilted(
                    [0.0, -68.71791358247505, -59.57184618267071],
                    [-51.528248972576364, 0.0, 0.0],
                ),
                Outline::circle([5.0000000000000004e-8, 0.0], 10.0),
                191.50971698084905,
            )),
        ],
    )
}

#[test]
fn seed_352000000007085_a_tube_slotted_along_its_bore_banded_and_posted_is_drawn_uncrossed() {
    random_solids::holds_exactly(&a_tube_slotted_along_its_bore_banded_and_posted());
}

#[test]
fn seed_352000000007085_the_same_tube_is_drawn_uncrossed_through_the_application_s_body() {
    random_solids::holds_through_the_application(&a_tube_slotted_along_its_bore_banded_and_posted());
}

/// A shaft of radius 17.5 about X bored by one of radius 17 at 75°, by one
/// of radius 5 at 30° set off so that it grazes, and by one of radius 10
/// square to it: held on `main`, drawn crossed near (5, -5, 7.5) on #759's
/// branch (seed 353000000013767, as drawn).
fn a_shaft_bored_three_times_one_bore_grazing() -> Case {
    Case::new(
        Leaf::prism(Plane::yz(-20.0), Outline::circle([-5.0, -10.0], 17.5), 50.0),
        vec![
            Step::cut(Leaf::prism(
                Plane::tilted([-85.0, -5.0, -34.0], [0.0, 75.0, 0.0]),
                Outline::circle([0.0, 7.0], 17.0),
                186.0,
            )),
            Step::cut(Leaf::prism(
                Plane::tilted([51.0, 75.0, -10.0], [90.0, -30.0, 0.0]),
                Outline::circle([0.0, 12.4999999], 5.0),
                186.0,
            )),
            Step::cut(Leaf::prism(
                Plane::tilted([98.0, -5.0, -10.0], [90.0, -90.0, 0.0]),
                Outline::circle([0.0, 7.5], 10.0),
                186.0,
            )),
        ],
    )
}

#[test]
fn seed_353000000013767_a_shaft_bored_three_times_one_bore_grazing_is_drawn_uncrossed() {
    random_solids::holds_exactly(&a_shaft_bored_three_times_one_bore_grazing());
}

#[test]
fn seed_353000000013767_the_same_bores_are_drawn_uncrossed_through_the_application_s_body() {
    random_solids::holds_through_the_application(&a_shaft_bored_three_times_one_bore_grazing());
}

/// A cone of radius 4.5 tapering to its tip over 9 about Y, bored by a hole
/// of radius 2.577 standing a hundredth of a degree off square to the
/// plane it is raised on: held on `main`, drawn crossed on #762's branch
/// (seed 354000000005119, shrunk).
#[test]
fn seed_354000000005119_a_cone_bored_at_89_99_degrees_by_a_hole_of_radius_2_577_is_drawn_uncrossed()
{
    random_solids::holds_exactly(&Case::new(
        Leaf::turned(
            Plane::yz(-1.0),
            Axis::second(-1.0),
            Section::bands(-4.0, &[[9.0, 0.0, 4.5]]).sloping_to(&[[0.0, 0.0]]),
            360.0,
        ),
        vec![Step::cut(Leaf::prism(
            Plane::tilted(
                [-1.0, 18.16551555484623, -0.157462170794368],
                [89.99, 0.0, 0.0],
            ),
            Outline::circle([9.99999987845058e-9, 0.0], 2.5770585586338313),
            38.0,
        ))],
    ));
}

/// An elliptic shaft of half-axes 60 and 90 bored along its wall by a hole
/// of radius 15 leaning a ten-thousandth of a degree, six ten-millionths
/// inside the wall, then by a bore of radius 90 at 75°: held on `main`,
/// drawn crossed through the application's body on #762's branch (seed
/// 353000000010626, shrunk).
#[test]
fn seed_353000000010626_an_elliptic_shaft_bored_along_its_wall_a_hair_inside_then_across_at_75_degrees_is_drawn_uncrossed_through_the_application_s_body()
 {
    random_solids::holds_through_the_application(&Case::new(
        Leaf::prism(
            Plane::xy(-45.0),
            Outline::ellipse([0.0, -30.0], [-60.0, 0.0], 90.0),
            300.0,
        ),
        vec![
            Step::cut(Leaf::prism(
                Plane::tilted([0.0, -30.0, -393.0], [0.0, -0.0001, 0.0]),
                Outline::circle([0.0, 74.9999994], 15.0),
                980.0,
            )),
            Step::cut(Leaf::prism(
                Plane::tilted([-473.0, -30.0, -57.0], [0.0, 75.0, 0.0]),
                Outline::circle([0.0, 0.0], 90.0),
                980.0,
            )),
        ],
    ));
}

#[test]
fn seed_353000000010626_the_same_shaft_as_drawn_is_drawn_uncrossed_through_the_application_s_body()
{
    walls_seed_through_the_application(353000000010626);
}

/// An elliptic shaft of half-axes 25 and 20 bored at 24° by a hole a
/// hundredth wider than its minor half-axis, then across by a hole of radius
/// 7.5 and posted by a pin of that radius: held through the application's
/// body on #762's merge (seed 352000000014330, as drawn; its shrunk form
/// crosses there too).
#[test]
fn seed_352000000014330_an_elliptic_shaft_bored_at_24_degrees_a_hair_wider_than_it_then_across_and_posted_is_drawn_uncrossed_through_the_application_s_body()
 {
    walls_seed_through_the_application(352000000014330);
}

/// A shaft of radius 5 bored at 30° by a hole of its radius less a
/// ten-millionth, then posted by a pin of radius 5 a thousandth of a degree
/// off square: held through the application's body on #762's merge (seed
/// 352000000017007, shrunk).
#[test]
fn seed_352000000017007_a_shaft_bored_at_30_degrees_by_its_radius_then_posted_a_thousandth_of_a_degree_off_square_is_drawn_uncrossed_through_the_application_s_body()
 {
    random_solids::holds_through_the_application(&Case::new(
        Leaf::prism(Plane::xy(-4.0), Outline::circle([1.0, -1.0], 5.0), 9.0),
        vec![
            Step::cut(Leaf::prism(
                Plane::tilted(
                    [1.0, -12.381527307120104, -19.21338356366461],
                    [-30.0, 0.0, 0.0],
                ),
                Outline::circle([-0.0, -0.0], 4.9999999),
                46.0,
            )),
            Step::add(Leaf::prism(
                Plane::tilted(
                    [1.0, -23.7630546107732, 0.4996027097491902],
                    [-89.999, 0.0, 0.0],
                ),
                Outline::circle([6e-8, 0.0], 5.0),
                46.0,
            )),
        ],
    ));
}

#[test]
fn seed_352000000017007_the_same_shaft_as_drawn_is_drawn_uncrossed_through_the_application_s_body()
{
    walls_seed_through_the_application(352000000017007);
}

/// A shaft of radius 2.5 about X bored by a hole of radius 3 at 75°, then
/// along its axis by a square of half-side 2.49999998, its sides a hair
/// inside the shaft's wall: a face went missing where a face along a meet
/// curve was boxed short of the wall (seed 353000000011653, shrunk; #775).
fn a_shaft_bored_at_75_degrees_then_by_a_square_a_hair_inside_its_wall() -> Case {
    Case::new(
        Leaf::prism(Plane::yz(-4.0), Outline::circle([2.0, -2.0], 2.5), 6.0),
        vec![
            Step::cut(Leaf::prism(
                Plane::tilted([12.0, 6.0, -2.0], [0.0, -75.0, 0.0]),
                Outline::circle([-0.0, -0.0], 3.0),
                26.0,
            )),
            Step::cut(Leaf::prism(
                Plane::tilted([12.0, 2.0, -2.0], [0.0, -90.0, 0.0]),
                Outline::rectangle([-2.49999998, -2.49999998], [2.49999998, 2.49999998]),
                26.0,
            )),
        ],
    )
}

#[test]
fn seed_353000000011653_a_shaft_bored_at_75_degrees_then_by_a_square_a_hair_inside_its_wall_keeps_every_rule()
 {
    random_solids::holds_exactly(
        &a_shaft_bored_at_75_degrees_then_by_a_square_a_hair_inside_its_wall(),
    );
}

#[test]
fn seed_353000000011653_the_same_bores_keep_every_rule_through_the_application_s_body() {
    random_solids::holds_through_the_application(
        &a_shaft_bored_at_75_degrees_then_by_a_square_a_hair_inside_its_wall(),
    );
}

/// A shaft of radius 4 about Z cut across by a cone turned about X, then
/// joined along its axis to a square of half-side 3.99999994, its sides a
/// hair inside the shaft's wall: a face went missing where a face along a
/// meet curve was boxed short of the wall (seed 354000000006300, shrunk;
/// #775).
fn a_shaft_cut_by_a_cone_then_joined_to_a_square_a_hair_inside_its_wall() -> Case {
    Case::new(
        Leaf::prism(Plane::xy(-3.0), Outline::circle([-2.0, 1.0], 4.0), 9.0),
        vec![
            Step::cut(Leaf::turned(
                Plane::xy(1.0),
                Axis::first(0.0),
                Section::bands(-21.0, &[[39.0, 0.0, 0.0]]).sloping_to(&[[0.0, 3.0]]),
                360.0,
            )),
            Step::add(Leaf::prism(
                Plane::tilted([-2.0, 1.0, -16.0], [0.0, 0.0, 0.0]),
                Outline::rectangle([-3.99999994, -3.9999998199999998], [3.99999994, 4.00000006]),
                39.0,
            )),
        ],
    )
}

#[test]
fn seed_354000000006300_a_shaft_cut_by_a_cone_then_joined_to_a_square_a_hair_inside_its_wall_keeps_every_rule()
 {
    random_solids::holds_exactly(
        &a_shaft_cut_by_a_cone_then_joined_to_a_square_a_hair_inside_its_wall(),
    );
}

#[test]
fn seed_354000000006300_the_same_shaft_and_square_keep_every_rule_through_the_application_s_body() {
    random_solids::holds_through_the_application(
        &a_shaft_cut_by_a_cone_then_joined_to_a_square_a_hair_inside_its_wall(),
    );
}

/// A shaft of radius 4.5 about Z bored by a hole of radius 5 at 45°, then
/// along its axis by a square whose sides stand a hair from its wall: the
/// bore's face was boxed to x = 4.49966 where it reaches 4.5, beside a side
/// standing at x = 4.49999994, and went missing (seed 354000000017912,
/// shrunk; #775).
fn a_shaft_bored_at_45_degrees_then_by_a_square_a_hair_from_its_wall() -> Case {
    Case::new(
        Leaf::prism(Plane::xy(-4.0), Outline::circle([0.0, -2.0], 4.5), 9.0),
        vec![
            Step::cut(Leaf::prism(
                Plane::tilted([0.0, 16.0, -14.0], [45.0, 0.0, 0.0]),
                Outline::circle([-0.0, -0.0], 5.0),
                42.0,
            )),
            Step::cut(Leaf::prism(
                Plane::xy(-22.0),
                Outline::rectangle([-4.50000006, -4.5], [4.49999994, 4.5]),
                42.0,
            )),
        ],
    )
}

#[test]
fn seed_354000000017912_a_shaft_bored_at_45_degrees_then_by_a_square_a_hair_from_its_wall_keeps_every_rule()
 {
    random_solids::holds_exactly(
        &a_shaft_bored_at_45_degrees_then_by_a_square_a_hair_from_its_wall(),
    );
}

#[test]
fn seed_354000000017912_the_same_bores_keep_every_rule_through_the_application_s_body() {
    random_solids::holds_through_the_application(
        &a_shaft_bored_at_45_degrees_then_by_a_square_a_hair_from_its_wall(),
    );
}

/// An elliptic shaft cut by a bore at a skew angle and joined to a post at
/// another: built with crossed triangles (seed 86751972219231, shrunk).
#[test]
#[ignore = "an oblique family lane 2 leaves, #707"]
fn seed_86751972219231_an_elliptic_shaft_with_a_skew_bore_and_a_skew_post_keeps_every_rule() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(
            Plane::xz(-3.0),
            Outline::ellipse(
                [-1.0, -2.0],
                [-1.4546261850263216, -2.0332394501965996],
                4.0,
            ),
            -6.0,
        ),
        vec![
            Step::cut(Leaf::prism(
                Plane::tilted([-14.0, 8.0, -2.0], [90.0, 60.0, 0.0]),
                Outline::circle([-0.0, -0.0], 3.0904469332988396),
                30.541573641595576,
            )),
            Step::add(Leaf::prism(
                Plane::tilted([-1.0, -6.0, -16.0], [-23.0, 0.0, 0.0]),
                Outline::circle([-0.0, -0.0], 4.0),
                31.0,
            )),
        ],
    ));
}

/// A post of radius 4, a bar of its radius square to it a hair inside its
/// touch, and a third of its radius at 15°: declined (seed 86751972220241,
/// shrunk).
#[test]
#[ignore = "an oblique family lane 2 leaves, #707"]
fn seed_86751972220241_a_post_a_square_bar_a_hair_from_its_touch_and_a_bar_at_15_degrees_keep_every_rule()
 {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(Plane::xy(-1.0), Outline::circle([2.0, -2.0], 4.0), 10.0),
        vec![
            Step::add(Leaf::prism(
                Plane::tilted([2.0, -22.0, 4.0], [-90.0, 0.0, 0.0]),
                Outline::circle([-7.99999994, -0.0], 4.0),
                40.0,
            )),
            Step::add(Leaf::prism(
                Plane::tilted(
                    [2.0, 3.2021771039504117, -15.414789261914457],
                    [15.0, 0.0, 0.0],
                ),
                Outline::circle([-0.0, -0.0], 4.0),
                40.0,
            )),
        ],
    ));
}

/// A post of radius 2.5 bored square through by a bore a hair wider, then
/// at 12° by a small one: declined (seed 86751972222643, shrunk).
#[test]
#[ignore = "an oblique family lane 2 leaves, #707"]
fn seed_86751972222643_a_post_bored_square_by_a_hair_wider_bore_then_at_12_degrees_keeps_every_rule()
 {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(Plane::xy(-1.0), Outline::circle([-2.0, 1.0], 2.5), 4.0),
        vec![
            Step::cut(Leaf::prism(
                Plane::tilted(
                    [-2.0, -10.62403840463596, 1.4999999959424442],
                    [-90.0, 0.0, 0.0],
                ),
                Outline::circle([-0.0, -0.0], 2.50000006),
                23.0,
            )),
            Step::cut(Leaf::prism(
                Plane::tilted(
                    [-2.0, 3.4167734788384316, -9.870025276332276],
                    [12.0, 0.0, 0.0],
                ),
                Outline::circle([2.0, 0.0], 0.5),
                23.0,
            )),
        ],
    ));
}

/// A post of radius 60 cut by an elliptic bore at 30° and a turned bore
/// leaning 32°: built with crossed triangles (seed 86751972224624, shrunk).
#[test]
#[ignore = "an oblique family lane 2 leaves, #707"]
fn seed_86751972224624_a_post_cut_by_an_elliptic_bore_and_a_turned_one_at_skew_angles_keeps_every_rule()
 {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(
            Plane::xy(-60.0),
            Outline::circle([60.0, -60.0], 60.0),
            120.0,
        ),
        vec![
            Step::cut(Leaf::prism(
                Plane::tilted([-89.0, -60.0, -258.0], [0.0, 30.0, 0.0]),
                Outline::ellipse([0.0, 26.5], [10.606601717798211, -10.606601717798217], 45.0),
                596.0,
            )),
            Step::cut(Leaf::turned(
                Plane::yz(30.0),
                Axis::second(-38.21177906001094).leaning(32.0),
                Section::bands(-257.0, &[[596.0, 0.0, 30.0]]),
                360.0,
            )),
        ],
    ));
}

/// A shaft of radius 135 cut by a bore 1e-3° off square to it, then by one
/// at 30°: built with crossed triangles (seed 86751972230062, shrunk).
#[test]
#[ignore = "an oblique family lane 2 leaves, #707"]
fn seed_86751972230062_a_shaft_cut_by_a_bore_1e_3_degrees_off_square_and_one_at_30_degrees_keeps_every_rule()
 {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(
            Plane::yz(-30.0),
            Outline::circle([0.0, -30.0], 135.0),
            225.0,
        ),
        vec![
            Step::cut(Leaf::prism(
                Plane::tilted([83.0, 608.0, -30.0], [90.0, -0.001, 0.0]),
                Outline::circle([0.0, 90.0], 45.0),
                1216.0,
            )),
            Step::cut(Leaf::prism(
                Plane::tilted([386.0, 0.0, -557.0], [0.0, -30.0, 0.0]),
                Outline::circle([0.0, 0.0], 60.0),
                1216.0,
            )),
        ],
    ));
}

/// A post of radius 2 bored square by a small bore, then by one of its
/// radius at 45° a hair off its axis: built open (seed 86751972230147,
/// shrunk).
#[test]
fn seed_86751972230147_a_post_bored_square_then_by_its_radius_at_45_degrees_keeps_every_rule() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(Plane::xz(-2.5), Outline::circle([-2.0, 1.0], 2.0), -7.0),
        vec![
            Step::cut(Leaf::prism(
                Plane::xy(1.0),
                Outline::circle([0.0, 0.0], 0.5),
                30.0,
            )),
            Step::cut(Leaf::prism(
                Plane::tilted([-2.0, -9.0, -9.0], [-45.0, 0.0, 0.0]),
                Outline::circle([2e-7, 0.0], 2.0),
                27.0,
            )),
        ],
    ));
}

/// Seeds of the walls-at-any-angle campaigns lane 3 leaves wrong, each
/// replayed from its draw as the campaign drew it: the exact kernel's, or
/// the application's body's.
fn walls_seed_on_the_exact_kernel(seed: u64) {
    random_solids::holds_exactly(&Case::drawn_walls_at_any_angle(seed));
}

fn walls_seed_through_the_application(seed: u64) {
    random_solids::holds_through_the_application(&Case::drawn_walls_at_any_angle(seed));
}

#[test]
#[ignore = "triangles crossing on a cone face, #710"]
fn seed_93075255850034_a_cone_face_is_drawn_uncrossed_on_the_exact_kernel() {
    walls_seed_on_the_exact_kernel(93075255850034);
}

#[test]
#[ignore = "triangles crossing on a cone face, #710"]
fn seed_93075255850115_a_cone_face_is_drawn_uncrossed_on_the_exact_kernel() {
    walls_seed_on_the_exact_kernel(93075255850115);
}

#[test]
#[ignore = "triangles crossing on a cone face, #710"]
fn seed_93075255850150_a_cone_face_is_drawn_uncrossed_on_the_exact_kernel() {
    walls_seed_on_the_exact_kernel(93075255850150);
}

#[test]
#[ignore = "triangles crossing on a cone face, #710"]
fn seed_93075255850326_a_cone_face_is_drawn_uncrossed_on_the_exact_kernel() {
    walls_seed_on_the_exact_kernel(93075255850326);
}

#[test]
#[ignore = "triangles crossing on a cone face, #710"]
fn seed_93075255850400_a_cone_face_is_drawn_uncrossed_on_the_exact_kernel() {
    walls_seed_on_the_exact_kernel(93075255850400);
}

#[test]
#[ignore = "triangles crossing on a cone face, #710"]
fn seed_93075255850402_a_cone_face_is_drawn_uncrossed_on_the_exact_kernel() {
    walls_seed_on_the_exact_kernel(93075255850402);
}

#[test]
#[ignore = "triangles crossing on a cone face, #710"]
fn seed_93075255850476_a_cone_face_is_drawn_uncrossed_on_the_exact_kernel() {
    walls_seed_on_the_exact_kernel(93075255850476);
}

#[test]
#[ignore = "triangles crossing on a cone face, #710"]
fn seed_93075255850694_a_cone_face_is_drawn_uncrossed_on_the_exact_kernel() {
    walls_seed_on_the_exact_kernel(93075255850694);
}

#[test]
#[ignore = "triangles crossing on a cone face, #710"]
fn seed_93075255850884_a_cone_face_is_drawn_uncrossed_on_the_exact_kernel() {
    walls_seed_on_the_exact_kernel(93075255850884);
}

#[test]
#[ignore = "triangles crossing on a cone face, #710"]
fn seed_25875769009446_a_cone_face_is_drawn_uncrossed_through_the_application_s_body() {
    walls_seed_through_the_application(25875769009446);
}

#[test]
#[ignore = "triangles crossing on a cone face, #710"]
fn seed_25875769009839_a_cone_face_is_drawn_uncrossed_through_the_application_s_body() {
    walls_seed_through_the_application(25875769009839);
}

#[test]
#[ignore = "triangles crossing on a cone face, #710"]
fn seed_25875769009854_a_cone_face_is_drawn_uncrossed_through_the_application_s_body() {
    walls_seed_through_the_application(25875769009854);
}

#[test]
#[ignore = "triangles crossing on a cone face, #710"]
fn seed_25875769009915_a_cone_face_is_drawn_uncrossed_through_the_application_s_body() {
    walls_seed_through_the_application(25875769009915);
}

#[test]
#[ignore = "triangles crossing on a cone face, #710"]
fn seed_25875769010058_a_cone_face_is_drawn_uncrossed_through_the_application_s_body() {
    walls_seed_through_the_application(25875769010058);
}

#[test]
#[ignore = "triangles crossing on a cone face, #710"]
fn seed_25875769010076_a_cone_face_is_drawn_uncrossed_through_the_application_s_body() {
    walls_seed_through_the_application(25875769010076);
}

#[test]
#[ignore = "triangles crossing on a cone face, #710"]
fn seed_25875769010112_a_cone_face_is_drawn_uncrossed_through_the_application_s_body() {
    walls_seed_through_the_application(25875769010112);
}

#[test]
#[ignore = "triangles crossing on a cone face, #710"]
fn seed_25875769010597_a_cone_face_is_drawn_uncrossed_through_the_application_s_body() {
    walls_seed_through_the_application(25875769010597);
}

#[test]
#[ignore = "triangles crossing on a cone face, #710"]
fn seed_25875769010603_a_cone_face_is_drawn_uncrossed_through_the_application_s_body() {
    walls_seed_through_the_application(25875769010603);
}

#[test]
#[ignore = "triangles crossing on a cone face, #710"]
fn seed_25875769010631_a_cone_face_is_drawn_uncrossed_through_the_application_s_body() {
    walls_seed_through_the_application(25875769010631);
}

#[test]
#[ignore = "triangles crossing on a cone face, #710"]
fn seed_25875769010654_a_cone_face_is_drawn_uncrossed_through_the_application_s_body() {
    walls_seed_through_the_application(25875769010654);
}

#[test]
#[ignore = "triangles crossing on a cone face, #710"]
fn seed_25875769010704_a_cone_face_is_drawn_uncrossed_through_the_application_s_body() {
    walls_seed_through_the_application(25875769010704);
}

#[test]
#[ignore = "triangles crossing on a cone face, #710"]
fn seed_25875769010757_a_cone_face_is_drawn_uncrossed_through_the_application_s_body() {
    walls_seed_through_the_application(25875769010757);
}

#[test]
#[ignore = "an open cone face, a family lane 3 leaves, #707"]
fn seed_93075255850420_a_cone_face_is_drawn_closed_on_the_exact_kernel() {
    walls_seed_on_the_exact_kernel(93075255850420);
}

#[test]
#[ignore = "a decline nothing excuses, a family lane 3 leaves, #707"]
fn seed_25875769010357_is_answered_through_the_application_s_body() {
    walls_seed_through_the_application(25875769010357);
}

#[test]
#[ignore = "a box floor against a cylinder cap leaning a hair, failing on main too, #707"]
fn seed_93075255850138_a_floor_against_a_cap_leaning_a_hair_keeps_every_rule() {
    walls_seed_on_the_exact_kernel(93075255850138);
}

/// The campaigns, compiled only when asked for — `--features campaigns` —
/// so that work on any other issue never pays for them.
#[cfg(feature = "campaigns")]
mod campaign {
    use std::sync::Arc;
    use std::sync::atomic::Ordering;

    use cao_solid::soundness::{Check, Flaw};
    use random_solids::Application;
    use random_solids::campaigning::{
        Answering, BOWED, DECLINED, GRAZING, HELD, THIN, TIPS, UNSTABLE, campaign_over,
        shrunk_one_by_one, tally_conics,
    };

    use super::*;

    fn tallied(measured: random_solids::Measured) {
        HELD.fetch_add(measured.held, Ordering::Relaxed);
        GRAZING.fetch_add(measured.grazing, Ordering::Relaxed);
        TIPS.fetch_add(measured.through_a_tip, Ordering::Relaxed);
        DECLINED.fetch_add(measured.declined, Ordering::Relaxed);
        UNSTABLE.fetch_add(measured.unstable, Ordering::Relaxed);
        THIN.fetch_add(measured.thin, Ordering::Relaxed);
        BOWED.fetch_add(measured.bowed, Ordering::Relaxed);
    }

    fn exactly(case: &Case) -> Result<(), Flaw> {
        tallied(random_solids::held_to_arithmetic(case, &Exact)?);
        tally_conics(case);
        Ok(())
    }

    fn through_the_application(case: &Case) -> Result<(), Flaw> {
        tallied(random_solids::held_to_arithmetic(case, &Application)?);
        tally_conics(case);
        Ok(())
    }

    /// The case a seed stands for, with the seed written where a campaign
    /// that ends the program still leaves it to be read.
    fn drawn(seed: u64) -> Case {
        eprint!("\rseed {seed} ");
        Case::drawn_walls_at_any_angle(seed)
    }

    #[test]
    #[ignore = "a campaign, run by hand: see the head of this file"]
    fn a_campaign_of_walls_at_any_angle_on_the_exact_kernel_keeps_every_rule() {
        campaign_over("walls at any angle", Answering::Exactly, drawn, exactly);
    }

    #[test]
    #[ignore = "a campaign, run by hand: see the head of this file"]
    fn a_campaign_of_walls_at_any_angle_through_the_application_s_body_keeps_every_rule() {
        campaign_over(
            "walls at any angle",
            Answering::ThroughTheApplication,
            drawn,
            through_the_application,
        );
    }

    /// Each seed named in `CAO_TRIAGE_SEEDS`, commas between them, run again
    /// on the exact kernel — through the application's body when
    /// `CAO_TRIAGE_KERNEL` is `application` — shrunk while it still breaks
    /// the same rule, and printed as a test.
    #[test]
    #[ignore = "run by hand on the seeds a campaign named"]
    fn the_seeds_a_walls_campaign_named_are_shrunk_one_by_one() {
        let seeds: Vec<u64> = std::env::var("CAO_TRIAGE_SEEDS")
            .unwrap_or_default()
            .split(',')
            .filter_map(|seed| seed.trim().parse().ok())
            .collect();
        let check: Check<Case> = match std::env::var("CAO_TRIAGE_KERNEL").as_deref() {
            Ok("application") => Arc::new(through_the_application),
            _ => Arc::new(exactly),
        };
        shrunk_one_by_one(seeds, Case::drawn_walls_at_any_angle, check);
    }
}
