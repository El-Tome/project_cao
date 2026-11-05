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

#[allow(dead_code, unused_imports)]
mod random_solids;

use cao_solid::brep::{Body, Declined, Surface};
use glam::DVec3;
use random_solids::{Axis, Case, Exact, Kernel, Leaf, Outline, Plane, Section, Step};

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
    let coned = |case: &&Case| case.leaves().any(|leaf| kind_of(leaf) == Kind::Cone);
    let asking = cases
        .iter()
        .filter(|case| !case.steps.is_empty())
        .filter(coned)
        .all(Case::asks_beyond_the_conics);
    assert!(
        asking,
        "a case with a cone that asks nothing beyond the conics"
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

/// What the exact kernel answers on a case: its body, or the first decline.
fn answered(case: &Case) -> Result<Body, Declined> {
    let mut body = Exact.raised(&case.start)?;
    for step in &case.steps {
        let tool = Exact.raised(&step.tool)?;
        body = Exact.combined(&body, &tool, step.mode)?;
    }
    Ok(body)
}

/// A decline the scope rule excuses, and only as unsupported.
fn declined_along_a_stretch(case: &Case) {
    assert_eq!(answered(case).err(), Some(Declined::Unsupported), "{case}");
    assert!(case.asks_of_walls_along_a_stretch(), "{case}");
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
    let along = 17.790913166907888;
    Case::new(
        Leaf::prism(Plane::xz(-1.0), Outline::circle([-1.0, 1.0], 4.0), -5.5),
        vec![Step::cut(Leaf::prism(
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
    let body = answered(&case).unwrap_or_else(|declined| panic!("{declined:?}\n{case}"));
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
    assert!(answered(&case).is_ok(), "{case}");
    random_solids::holds_exactly(&case);
}

#[test]
fn the_same_posts_crossing_at_3e_4_or_1e_4_degrees_are_declined_closer_than_a_sag() {
    declined_along_a_stretch(&posts_crossing_at(3e-4));
    declined_along_a_stretch(&posts_crossing_at(1e-4));
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
    assert!(answered(&case).is_ok(), "{case}");
    random_solids::holds_exactly(&case);
}

/// Two posts of radius 4.5, the second a ten-millionth wider and off, at
/// 45.5° to the first: two ellipses crossing at two nodes, a hair off;
/// built with a volume off the promise (seed 86751972229989, shrunk).
#[test]
#[ignore = "an oblique family lane 2 leaves, #707"]
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
#[ignore = "an oblique family lane 2 leaves, #707"]
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
