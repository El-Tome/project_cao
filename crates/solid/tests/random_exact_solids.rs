//! Solids drawn at random and combined at random by the exact kernel of #498,
//! held to the rules every result must keep.
//!
//! The cases are #448's, drawn among what the kernel raises: prisms of
//! rectangles and circles on the three planes of the origin
//! (`Case::drawn_square`), still weighted towards coincidences and hairs —
//! or, in a campaign of its own, among the profiles the application makes all
//! the time (`Case::drawn_profiles`): rectangles with rounded corners, slots
//! and rings beside rectangles and circles, each handed to the kernel as its
//! straight runs and arcs. Both draws stay what they were for a seed, so a
//! campaign on either is comparable with the ones before it.
//! Every leaf is raised by `cao_solid::brep::Body::raised` and every step
//! joined or cut; a leaf or a step declined is no answer. At every step the
//! body's listing is held to its geometry, its triangles are closed and
//! uncrossed, they hold along the harness's lines what arithmetic promised
//! from the leaves — within the room their tolerance takes, the lines grazing
//! a curved wall left out and counted — and nothing beyond the box the leaves
//! span; and the whole case run twice gives the same triangles, bit for bit.
//!
//! The arithmetic is held to the flats here, where it can be: on straight
//! sides it is what the flats measure, on circles it is within their
//! sagitta, and the check that holds the exact kernel keeps every case the
//! flats keep their own rules on — some two thirds of the first three
//! thousand square cases, when it was written — and breaks every one they
//! break.
//!
//! Ellipses come in three draws of their own (#571), each with a seed space
//! of its own, so that no draw before them changes for any seed: prisms of a
//! whole ellipse, one cut by one trait or two, a slot with elliptic ends or
//! an elliptic ring, its axes on the plane's or turned, among blocks,
//! circles and the other profiles, all raised along one normal
//! (`Case::drawn_ellipses`), weighted towards the same ellipse a hair off, a
//! block's side at its reach, a circle across its minor axis or at one of
//! its vertices, two ellipses touching and half-axes a hair apart; walls
//! square to each other, round through elliptic, elliptic through round and
//! through elliptic, weighted towards equal half-widths and axes meeting
//! (`Case::drawn_ellipses_crossed`); and planes leaning through elliptic
//! walls, elliptic holes square to a slanted face, leans in decision 11's
//! band and in the band the kernel warns of, bores at a skew angle and now
//! and then a cone about an elliptic wall's axis
//! (`Case::drawn_ellipses_tilted`).
//! An ellipse is handed to the exact kernel as its arcs (`Run::Oval`) and to
//! the flats as the application samples it, and held along every line to
//! the true curve, the flats within the sagitta of the 48-gon. A case with
//! an ellipse may be declined as unsupported only where an elliptic wall
//! meets a cone or a wall at a skew angle (`Case::asks_beyond_the_conics`).
//! Through the application's body an ellipse is the exact kernel's as
//! well: a decline is a broken step, never the flats. The flats answered 199, 280 and 272 of
//! the first 300 cases of the three draws by their own rules when they were
//! written, and the arithmetic held every one.
//!
//! A campaign is run by hand, the square one, the one of profiles, or one of
//! ellipses:
//!
//! ```text
//! CAO_FUZZ_SECONDS=3600 cargo test --release -p cao_solid --features campaigns \
//!     --test random_exact_solids -- --ignored --nocapture \
//!     a_campaign_of_random_square_solids
//! CAO_FUZZ_SECONDS=3600 cargo test --release -p cao_solid --features campaigns \
//!     --test random_exact_solids -- --ignored --nocapture \
//!     a_campaign_of_random_profiles_on
//! CAO_FUZZ_SECONDS=3600 cargo test --release -p cao_solid --features campaigns \
//!     --test random_exact_solids -- --ignored --nocapture \
//!     a_campaign_of_random_ellipses_crossed_on
//! ```
//!
//! `a_campaign_of_random_profiles_through_the_application_s_body` runs the
//! profiles through `cao_solid::Body`, as the application computes them: the
//! face numbers, the drawing as each body is built, the declines it adds.
//! The campaigns of ellipses run either way too:
//! `a_campaign_of_random_ellipses_…`, `…_ellipses_crossed_…` and
//! `…_ellipses_tilted_…`, each `on_the_exact_kernel` or
//! `through_the_application_s_body`.
//!
//! `CAO_FUZZ_SEED` starts it from a given seed rather than from the clock,
//! `CAO_FUZZ_CASES` stops it after that many seeds, so that two campaigns on
//! the same machine under different loads try the same cases, and
//! `CAO_FUZZ_PATIENCE` is how many seconds one case may take before it counts
//! as no answer. The seeds it names are shrunk one by one by
//! `the_seeds_a_campaign_named_are_shrunk_one_by_one`, given in
//! `CAO_TRIAGE_SEEDS` with `CAO_TRIAGE_DRAW=profiles` for the profiles,
//! `ellipses`, `ellipses_crossed` or `ellipses_tilted` for the draws of
//! ellipses, and `CAO_TRIAGE_KERNEL=application` for the seeds a campaign
//! through the application's body named.

// The drawing, the promise and the checks are shared with the flats'
// campaign; each file uses its own part of them.
#[allow(dead_code, unused_imports)]
mod random_solids;

use std::cell::Cell;

use cao_solid::Declined;
use cao_solid::brep::{Curve, Line, ListedEdge, Listing};
use cao_solid::profile::Run;
use cao_solid::soundness::{Flaw, Lines, Mislisted, Silence, Spans, Triangle, shrink};
use glam::{DVec2, DVec3};
use random_solids::{
    Application, Axis, Case, Exact, Flats, Kernel, Leaf, Mode, Outline, Oval, Plane, Section, Step,
    Stretch,
};

/// A bundle of lines laid over the box a leaf's flats span, and how far that
/// box reaches.
fn lines_over(leaf: &Leaf) -> (Lines, Vec<Spans>, f64) {
    let flats = leaf.solid().expect("a leaf the flats raise");
    let (low, high) = flats.bounds().expect("a solid with a box");
    let reach = low.abs().max(high.abs()).max_element().max(1.0);
    let lines = Lines::across(low - 1.0, high + 1.0, 24);
    let measured = lines.inside(&flats.triangles());
    (lines, measured, reach)
}

fn spans_of(stretches: &[Stretch]) -> Spans {
    Spans::gathered(
        stretches
            .iter()
            .map(|stretch| (stretch.from.at, stretch.to.at))
            .collect(),
    )
}

/// The prisms of the first cases of both draws that hold every outline: every
/// kind, and the profiles.
fn drawn_prisms(kept: impl Fn(&Outline) -> bool) -> Vec<Leaf> {
    (0..400)
        .flat_map(|seed| [Case::drawn(seed), Case::drawn_profiles(seed)])
        .flat_map(|case| case.leaves().cloned().collect::<Vec<Leaf>>())
        .filter(|leaf| {
            leaf.is_solid() && matches!(leaf, Leaf::Prism { outline, .. } if kept(outline))
        })
        .collect()
}

#[test]
fn a_prism_of_straight_sides_holds_along_every_line_exactly_what_its_flats_hold() {
    let leaves =
        drawn_prisms(|outline| matches!(outline, Outline::Rectangle { .. } | Outline::Star { .. }));
    assert!(leaves.len() > 100, "{} leaves", leaves.len());
    for leaf in &leaves {
        let (lines, measured, reach) = lines_over(leaf);
        for (index, measure) in measured.iter().enumerate() {
            let (origin, direction) = lines.line(index);
            let promise = leaf.along(origin, direction).expect("a prism");
            let promised = spans_of(&promise);
            assert_eq!(
                promised.stretches().len(),
                measure.stretches().len(),
                "line {index} across {leaf}: {promised:?} against {measure:?}"
            );
            for (one, other) in promised.stretches().iter().zip(measure.stretches()) {
                assert!(
                    (one.0 - other.0).abs() <= 1e-9 * reach
                        && (one.1 - other.1).abs() <= 1e-9 * reach,
                    "line {index} across {leaf}: {one:?} against {other:?}"
                );
            }
        }
    }
}

/// The same leaf with its circles drawn at other radii.
fn resized(leaf: &Leaf, outer: impl Fn(f64) -> f64, inner: impl Fn(f64) -> f64) -> Leaf {
    let Leaf::Prism {
        plane,
        outline,
        height,
    } = leaf
    else {
        unreachable!("only prisms are resized")
    };
    let outline = match outline {
        Outline::Circle {
            center,
            radius,
            from,
        } => Outline::Circle {
            center: *center,
            radius: outer(*radius),
            from: *from,
        },
        Outline::Ring {
            center,
            outer: rim,
            inner: bore,
        } => Outline::Ring {
            center: *center,
            outer: outer(*rim),
            inner: inner(*bore),
        },
        other => other.clone(),
    };
    Leaf::prism(*plane, outline, *height)
}

#[test]
fn a_prism_of_circles_holds_along_every_line_what_its_flats_hold_but_for_their_sagitta() {
    let inscribed = (std::f64::consts::PI / random_solids::CIRCLE_STEPS as f64).cos();
    let leaves =
        drawn_prisms(|outline| matches!(outline, Outline::Circle { .. } | Outline::Ring { .. }));
    assert!(leaves.len() > 100, "{} leaves", leaves.len());
    for leaf in &leaves {
        let least = resized(leaf, |radius| radius * inscribed, |radius| radius);
        let most = resized(leaf, |radius| radius, |radius| radius * inscribed);
        let (lines, measured, reach) = lines_over(leaf);
        for (index, measure) in measured.iter().enumerate() {
            let (origin, direction) = lines.line(index);
            let [least, most] = [&least, &most]
                .map(|bound| spans_of(&bound.along(origin, direction).expect("a prism")));
            let short = least.without(measure).length();
            let over = measure.without(&most).length();
            assert!(
                short <= 1e-9 * reach && over <= 1e-9 * reach,
                "line {index} across {leaf}: {measure:?} between {least:?} and {most:?}"
            );
        }
    }
}

/// Whether a point lies on one of the two ends of a prism, rather than on its
/// wall.
fn on_an_end(leaf: &Leaf, point: DVec3, reach: f64) -> bool {
    let Leaf::Prism { plane, height, .. } = leaf else {
        unreachable!("only prisms have ends")
    };
    let level = (point - plane.frame().0).dot(plane.normal());
    level.abs() <= 1e-9 * reach || (level - height).abs() <= 1e-9 * reach
}

fn corner_radius(leaf: &Leaf) -> f64 {
    match leaf {
        Leaf::Prism {
            outline: Outline::Rounded { radius, .. } | Outline::Slot { radius, .. },
            ..
        } => *radius,
        _ => unreachable!("only rounded rectangles and slots have a corner radius"),
    }
}

#[test]
fn a_rounded_rectangle_or_a_slot_holds_along_every_line_what_its_flats_hold_on_its_straight_runs_and_but_for_their_sagitta_on_its_arcs()
 {
    let inscribed = (std::f64::consts::PI / random_solids::CIRCLE_STEPS as f64).cos();
    let leaves =
        drawn_prisms(|outline| matches!(outline, Outline::Rounded { .. } | Outline::Slot { .. }));
    assert!(leaves.len() > 100, "{} leaves", leaves.len());
    let mut straight = 0;
    for leaf in &leaves {
        let sagitta = corner_radius(leaf) * (1.0 - inscribed);
        let (lines, measured, reach) = lines_over(leaf);
        for (index, measure) in measured.iter().enumerate() {
            let (origin, direction) = lines.line(index);
            let promise = leaf.along(origin, direction).expect("a prism");
            let most = spans_of(&promise);
            let least = spans_of(
                &leaf
                    .along_grown(origin, direction, -sagitta)
                    .expect("a prism"),
            );
            let short = least.without(measure).length();
            let over = measure.without(&most).length();
            assert!(
                short <= 1e-9 * reach && over <= 1e-9 * reach,
                "line {index} across {leaf}: {measure:?} between {least:?} and {most:?}"
            );
            let ends: Vec<f64> = measure
                .stretches()
                .iter()
                .flat_map(|&(from, to)| [from, to])
                .collect();
            for end in promise
                .iter()
                .flat_map(|stretch| [stretch.from, stretch.to])
                .filter(|end| {
                    !end.curved
                        && end.at.is_finite()
                        && !on_an_end(leaf, origin + direction * end.at, reach)
                })
            {
                straight += 1;
                assert!(
                    ends.iter().any(|at| (at - end.at).abs() <= 1e-9 * reach),
                    "line {index} across {leaf}: {measure:?} has no end at {end:?}"
                );
            }
        }
    }
    assert!(straight > 1000, "{straight} ends on straight runs");
}

#[test]
fn a_line_through_a_rounded_rectangle_or_a_slot_is_told_whether_it_crosses_a_straight_run_or_an_arc()
 {
    let along = |leaf: &Leaf, origin: [f64; 3], direction: DVec3, by: f64| {
        leaf.along_grown(DVec3::from(origin), direction, by)
            .expect("a prism")
    };
    let ends = |stretches: Vec<Stretch>| {
        stretches
            .iter()
            .map(|stretch| (stretch.from.at, stretch.to.at))
            .collect::<Vec<_>>()
    };
    let root = 3.0_f64.sqrt();

    let block = Leaf::prism(
        Plane::xy(0.0),
        Outline::rounded([0.0, 0.0], [10.0, 6.0], 2.0),
        10.0,
    );
    let [low] = along(&block, [-1.0, 1.0, 1.0], DVec3::X, 0.0)[..] else {
        panic!("one stretch across the lower corners");
    };
    assert!((low.from.at - (3.0 - root)).abs() < 1e-12, "{low:?}");
    assert!((low.to.at - (9.0 + root)).abs() < 1e-12, "{low:?}");
    for end in [low.from, low.to] {
        assert!(
            end.curved && (end.cosine - root / 2.0).abs() < 1e-12,
            "{end:?}"
        );
    }
    let [middle] = along(&block, [-1.0, 3.0, 1.0], DVec3::X, 0.0)[..] else {
        panic!("one stretch across the middle");
    };
    assert_eq!((middle.from.at, middle.to.at), (1.0, 11.0));
    for end in [middle.from, middle.to] {
        assert!(!end.curved && end.cosine == 1.0, "{end:?}");
    }
    assert_eq!(
        ends(along(&block, [1.0, 1.0, -1.0], DVec3::Z, 0.0)),
        [(1.0, 11.0)]
    );
    assert_eq!(ends(along(&block, [0.3, 0.3, -1.0], DVec3::Z, 0.0)), []);
    assert_eq!(
        ends(along(&block, [0.3, 0.3, -1.0], DVec3::Z, 0.5)),
        [(0.5, 11.5)]
    );
    assert_eq!(
        ends(along(&block, [2.6, 2.6, -1.0], DVec3::Z, -2.5)),
        [(3.5, 8.5)]
    );

    let slot = Leaf::prism(
        Plane::xy(0.0),
        Outline::slot([2.0, 5.0], [8.0, 5.0], 1.5),
        3.0,
    );
    let [cap] = along(&slot, [9.0, 0.0, 1.0], DVec3::Y, 0.0)[..] else {
        panic!("one stretch through a cap");
    };
    let half = 1.25_f64.sqrt();
    assert!((cap.from.at - (5.0 - half)).abs() < 1e-12, "{cap:?}");
    assert!((cap.to.at - (5.0 + half)).abs() < 1e-12, "{cap:?}");
    for end in [cap.from, cap.to] {
        assert!(
            end.curved && (end.cosine - half / 1.5).abs() < 1e-12,
            "{end:?}"
        );
    }
    let [side] = along(&slot, [5.0, 0.0, 1.0], DVec3::Y, 0.0)[..] else {
        panic!("one stretch across the straight runs");
    };
    assert_eq!((side.from.at, side.to.at), (3.5, 6.5));
    assert!(!side.from.curved && !side.to.curved, "{side:?}");
    assert_eq!(
        ends(along(&slot, [0.0, 5.0, 1.0], DVec3::X, 0.0)),
        [(0.5, 9.5)]
    );
    assert_eq!(ends(along(&slot, [5.0, 0.0, 1.0], DVec3::Y, -1.5)), []);
}

#[test]
fn a_line_through_the_place_a_straight_run_meets_an_arc_is_told_the_slant_of_the_wall_there() {
    let slant = |stretch: Stretch| [stretch.from, stretch.to].map(|end| (end.at, end.cosine));

    let slot = Leaf::prism(
        Plane::xy(4.0),
        Outline::slot([4.5, -1.0], [6.5, -1.0], 0.5),
        -5.0,
    );
    let square = 1.0 / 1.01_f64.sqrt();
    let [entering] = slot
        .along(DVec3::new(4.5, -1.5, 3.0), DVec3::new(0.1, 1.0, 0.0))
        .expect("a prism")[..]
    else {
        panic!("one stretch into the slot");
    };
    let [(at, cosine), _] = slant(entering);
    assert!(at == 0.0 && (cosine - square).abs() < 1e-12, "{entering:?}");
    let [leaving] = slot
        .along(DVec3::new(6.5, -0.5, 3.0), DVec3::new(0.1, 1.0, 0.0))
        .expect("a prism")[..]
    else {
        panic!("one stretch out of the slot");
    };
    let [_, (at, cosine)] = slant(leaving);
    assert!(at == 0.0 && (cosine - square).abs() < 1e-12, "{leaving:?}");

    let block = Leaf::prism(
        Plane::xz(1.0),
        Outline::rounded([0.0, 0.0], [10.0, 6.0], 2.0),
        -3.0,
    );
    let [entering] = block
        .along(DVec3::new(2.0, 2.5, 0.0), DVec3::new(0.1, -0.3, 1.0))
        .expect("a prism")[..]
    else {
        panic!("one stretch into the block");
    };
    let [(at, cosine), _] = slant(entering);
    assert!(
        at == 0.0 && (cosine - 1.0 / 1.1_f64.sqrt()).abs() < 1e-12,
        "{entering:?}"
    );
}

/// A profile's wall nearest a place: how far the place stands outside the
/// profile, or inside it when negative; the wall's outward normal, unless
/// the place is at a square corner; and whether the wall curves there, unless
/// the place is where a straight run meets an arc — where which of the two a
/// line is told it crosses is left to a rounding.
struct Wall {
    outside: f64,
    normal: Option<DVec2>,
    curved: Option<bool>,
}

/// The wall of a profile grown by `by` nearest a place, worked out from what
/// the profile is rounded from — the box of a rounded rectangle's arc
/// centres, the segment between a slot's — rather than from its pieces.
/// `near` is how close to a change of wall a place is taken to be on it.
fn wall_of(outline: &Outline, place: DVec2, by: f64, near: f64) -> Wall {
    let rounded_about = |out: DVec2, beyond: bool, between: bool, radius: f64| Wall {
        outside: out.length() - radius,
        normal: Some(out.normalize_or_zero()),
        curved: if beyond {
            Some(true)
        } else if between {
            Some(false)
        } else {
            None
        },
    };
    match *outline {
        Outline::Rounded { low, high, radius } if radius + by > 0.0 => {
            let (first, last) = (low + radius, high - radius);
            let out = place - place.clamp(first, last);
            let beyond = (0..2)
                .all(|axis| place[axis] < first[axis] - near || place[axis] > last[axis] + near);
            let between = (0..2)
                .any(|axis| first[axis] + near < place[axis] && place[axis] < last[axis] - near);
            rounded_about(out, beyond, between, radius + by)
        }
        Outline::Rounded { low, high, .. } => {
            let (low, high) = (low - by, high + by);
            let mut walls = [
                (low.x - place.x, DVec2::NEG_X),
                (place.x - high.x, DVec2::X),
                (low.y - place.y, DVec2::NEG_Y),
                (place.y - high.y, DVec2::Y),
            ];
            walls.sort_by(|one, other| other.0.total_cmp(&one.0));
            let beyond = (place - place.clamp(low, high)).length();
            Wall {
                outside: if beyond > 0.0 { beyond } else { walls[0].0 },
                normal: (walls[0].0 - walls[1].0 > near).then_some(walls[0].1),
                curved: Some(false),
            }
        }
        Outline::Slot { from, to, radius } => {
            let way = to - from;
            let along = (place - from).dot(way) / way.length();
            let out = place - (from + way * (along / way.length()).clamp(0.0, 1.0));
            rounded_about(
                out,
                along < -near || along > way.length() + near,
                near < along && along < way.length() - near,
                radius + by,
            )
        }
        Outline::Ring {
            center,
            outer,
            inner,
        } => {
            let out = place - center;
            let (rim, bore) = (out.length() - outer - by, inner - by - out.length());
            let (outside, normal) = if inner - by <= 0.0 || rim >= bore {
                (rim, out.normalize_or_zero())
            } else {
                (bore, -out.normalize_or_zero())
            };
            Wall {
                outside,
                normal: Some(normal),
                curved: Some(true),
            }
        }
        _ => unreachable!("only rounded rectangles, slots and rings are measured"),
    }
}

/// Lines through the places a profile's walls change — the arcs' centres and
/// ends, the middle of a slot or a bore — along the plane, square to it and
/// slanted, some a hair off square or off the plane.
fn lines_through_a_profile(leaf: &Leaf) -> Vec<(DVec3, DVec3)> {
    let Leaf::Prism {
        plane,
        outline,
        height,
    } = leaf
    else {
        unreachable!("only prisms are measured")
    };
    let (base, u, v) = plane.frame();
    let normal = u.cross(v);
    let places: Vec<DVec2> = match *outline {
        Outline::Rounded { low, high, radius } => {
            let (near, far) = (low + radius, high - radius);
            vec![
                near,
                far,
                DVec2::new(near.x, low.y),
                DVec2::new(far.x, high.y),
                DVec2::new(low.x, near.y),
                DVec2::new(high.x, far.y),
                low,
                (low + high) / 2.0,
            ]
        }
        Outline::Slot { from, to, radius } => {
            let side = (to - from).normalize().perp() * radius;
            vec![from, to, from - side, to + side, (from + to) / 2.0]
        }
        Outline::Ring {
            center,
            outer,
            inner,
        } => vec![center, center + DVec2::X * inner, center + DVec2::Y * outer],
        _ => unreachable!("only rounded rectangles, slots and rings are measured"),
    };
    let ways = [
        u,
        v,
        u + v,
        u - v * 0.3,
        normal,
        u * 0.1 + v - normal * 0.3,
        u + v * 1e-9,
        v - u * 1e-7,
        normal + u * 1e-9,
        DVec3::new(std::f64::consts::FRAC_1_PI, 5f64.sqrt() - 2.0, 1.0),
    ];
    places
        .iter()
        .flat_map(|place| {
            [0.0, height / 2.0, *height]
                .map(|level| base + u * place.x + v * place.y + normal * level)
        })
        .flat_map(|origin| ways.map(|way| (origin, way)))
        .collect()
}

#[test]
fn a_rounded_rectangle_a_slot_or_a_ring_grown_or_shrunk_holds_along_any_line_the_places_within_it_and_ends_at_the_slant_of_its_wall()
 {
    let mut leaves: Vec<Leaf> = (0..100)
        .flat_map(|seed| {
            Case::drawn_profiles(seed)
                .leaves()
                .cloned()
                .collect::<Vec<_>>()
        })
        .filter(|leaf| {
            matches!(
                leaf,
                Leaf::Prism {
                    outline: Outline::Rounded { .. } | Outline::Slot { .. } | Outline::Ring { .. },
                    ..
                }
            )
        })
        .collect();
    for plane in [Plane::xy(1.5), Plane::xz(-2.0), Plane::yz(3.0)] {
        for height in [4.0, -2.5] {
            for outline in [
                Outline::rounded([0.0, -1.0], [4.0, 1.0], 1.0),
                Outline::rounded([0.5, 0.5], [3.5, 6.0], 0.5),
                Outline::slot([1.0, 2.0], [1.0, -1.5], 1.5),
                Outline::slot([-2.0, 0.0], [3.0, 0.0], 0.5),
                Outline::ring([1.0, 1.0], 3.0, 1.0),
            ] {
                leaves.push(Leaf::prism(plane, outline, height));
            }
        }
    }
    assert!(leaves.len() > 100, "{} leaves", leaves.len());

    let ends: usize = random_solids::on_every_core(&leaves, |leaf| {
        let mut ends = 0;
        let Leaf::Prism {
            plane,
            outline,
            height,
        } = leaf
        else {
            unreachable!("only prisms were kept")
        };
        let (base, u, v) = plane.frame();
        let normal = u.cross(v);
        let (low, high) = leaf.bounds().expect("a prism");
        let reach = low.abs().max(high.abs()).max_element().max(1.0);
        let size = (high - low).max_element();
        let near = 1e-9 * reach;
        let measured = |point: DVec3, by: f64| {
            let offset = point - base;
            let level = offset.dot(normal);
            let slab = (height.min(0.0) - by - level).max(level - height.max(0.0) - by);
            let place = DVec2::new(offset.dot(u), offset.dot(v));
            (slab, wall_of(outline, place, by, 1e3 * near))
        };
        for (origin, way) in lines_through_a_profile(leaf) {
            for by in [0.0, 0.1, -0.1, 0.6, -0.6].map(|share| share * size) {
                let stretches = leaf.along_grown(origin, way, by).expect("a prism");
                let held = |at: f64| {
                    stretches
                        .iter()
                        .any(|stretch| stretch.from.at <= at && at <= stretch.to.at)
                };
                let span = 4.0 * (reach + size) / way.length();
                for step in 0..=600 {
                    let at = span * (step as f64 / 300.0 - 1.0) + 1e-3;
                    let (slab, wall) = measured(origin + way * at, by);
                    let depth = slab.max(wall.outside);
                    if depth.abs() > near {
                        assert_eq!(
                            held(at),
                            depth < 0.0,
                            "{leaf} grown by {by}, at {at} along {way} from {origin}: {stretches:?}"
                        );
                    }
                }
                let step = DVec2::new(way.dot(u), way.dot(v));
                for end in stretches
                    .iter()
                    .flat_map(|stretch| [stretch.from, stretch.to])
                    .filter(|end| end.at.is_finite())
                {
                    let (slab, wall) = measured(origin + way * end.at, by);
                    let on_wall = wall.outside.abs() < 1e3 * near;
                    let (cosine, curved) = if slab.abs() < 1e3 * near && !on_wall {
                        (way.dot(normal).abs() / way.length(), false)
                    } else if let (Some(wall_normal), Some(curved)) = (wall.normal, wall.curved)
                        && on_wall
                        && slab < -1e3 * near
                    {
                        (wall_normal.dot(step).abs() / way.length(), curved)
                    } else {
                        continue;
                    };
                    if cosine < 1e-6 {
                        continue;
                    }
                    ends += 1;
                    assert!(
                        (end.cosine - cosine).abs() < 1e-6 && end.curved == curved,
                        "{leaf} grown by {by}, along {way} from {origin}: {end:?} crosses at {cosine}, curved {curved}"
                    );
                }
            }
        }
        ends
    })
    .into_iter()
    .sum();
    assert!(ends > 10_000, "{ends} ends");
}

#[test]
fn the_box_a_prism_spans_is_the_box_its_flats_span_on_any_plane() {
    let inscribed = (std::f64::consts::PI / random_solids::CIRCLE_STEPS as f64).cos();
    let leaves = drawn_prisms(|_| true);
    assert!(
        leaves
            .iter()
            .any(|leaf| matches!(leaf.plane(), Plane::Tilted { .. })),
        "no prism drawn on a turned plane"
    );
    for leaf in &leaves {
        let (low, high) = leaf.bounds().expect("a prism has a box");
        let flats = leaf.solid().expect("a leaf the flats raise");
        let (flat_low, flat_high) = flats.bounds().expect("a solid with a box");
        let reach = flat_low.abs().max(flat_high.abs()).max_element().max(1.0);
        let sagitta = match leaf {
            Leaf::Prism {
                outline: Outline::Circle { radius, .. },
                ..
            } => radius * (1.0 - inscribed),
            Leaf::Prism {
                outline: Outline::Ring { outer, .. },
                ..
            } => outer * (1.0 - inscribed),
            Leaf::Prism {
                outline: Outline::Rounded { radius, .. } | Outline::Slot { radius, .. },
                ..
            } => radius * (1.0 - inscribed),
            _ => 0.0,
        };
        let near = 1e-9 * reach;
        let within =
            |one: DVec3, other: DVec3, room: f64| (one - other).abs().max_element() <= room;
        assert!(
            flat_low.cmpge(low - near).all() && flat_high.cmple(high + near).all(),
            "{leaf}: the flats span {flat_low}..{flat_high} beyond {low}..{high}"
        );
        assert!(
            within(low, flat_low, sagitta + near) && within(high, flat_high, sagitta + near),
            "{leaf}: {low}..{high} against the flats' {flat_low}..{flat_high}"
        );
    }
}

#[test]
fn a_block_on_a_turned_plane_is_held_by_the_flats_to_the_arithmetic() {
    let case = Case::new(
        Leaf::prism(
            Plane::tilted([1.0, 2.0, 0.0], [0.0, 0.0, 45.0]),
            Outline::rectangle([0.0, 0.0], [4.0, 3.0]),
            2.0,
        ),
        vec![],
    );
    let measured = random_solids::held_to_arithmetic(&case, &Flats::for_case(&case));
    assert!(measured.is_ok(), "{measured:?}");
}

#[test]
fn a_leaf_grown_holds_every_line_it_held_and_a_leaf_shrunk_holds_none_it_did_not() {
    let leaves = drawn_prisms(|outline| !matches!(outline, Outline::Star { .. }));
    for leaf in &leaves {
        let (lines, _, reach) = lines_over(leaf);
        let by = 1e-3 * reach;
        for index in 0..lines.count() {
            let (origin, direction) = lines.line(index);
            let [shrunk, held, grown] = [-by, 0.0, by]
                .map(|by| spans_of(&leaf.along_grown(origin, direction, by).expect("a prism")));
            assert_eq!(
                shrunk.without(&held).length(),
                0.0,
                "line {index} of {leaf}"
            );
            assert_eq!(held.without(&grown).length(), 0.0, "line {index} of {leaf}");
            assert!(grown.length() > shrunk.length() || grown.length() == 0.0);
        }
    }
}

#[test]
fn the_same_seed_draws_the_same_square_case() {
    for seed in 0..200 {
        assert_eq!(
            Case::drawn_square(seed),
            Case::drawn_square(seed),
            "seed {seed}"
        );
    }
    let distinct: std::collections::BTreeSet<String> = (0..200)
        .map(|seed| Case::drawn_square(seed).to_string())
        .collect();
    assert!(distinct.len() > 190, "{} distinct cases", distinct.len());
}

#[test]
fn a_square_case_holds_prisms_of_rectangles_and_circles_on_the_planes_of_the_origin_alone() {
    for seed in 0..3000 {
        let case = Case::drawn_square(seed);
        for leaf in case.leaves() {
            let kept = match leaf {
                Leaf::Prism { plane, outline, .. } => {
                    !matches!(plane, Plane::Tilted { .. })
                        && matches!(outline, Outline::Rectangle { .. } | Outline::Circle { .. })
                }
                Leaf::Revolution { .. } | Leaf::Turned { .. } => false,
            };
            assert!(kept, "seed {seed}: {case}");
            assert!(leaf.is_solid(), "seed {seed}: {case}");
        }
    }
}

#[test]
fn the_square_generator_draws_every_kind_it_keeps() {
    let cases: Vec<Case> = (0..1000).map(Case::drawn_square).collect();
    let leaves = || cases.iter().flat_map(Case::leaves);
    let seen = |what: &str, found: bool| assert!(found, "no square case drew {what}");
    let outline = |leaf: &Leaf| match leaf {
        Leaf::Prism { outline, .. } => Some(outline.clone()),
        Leaf::Revolution { .. } | Leaf::Turned { .. } => None,
    };

    seen(
        "a rectangle",
        leaves().any(|leaf| matches!(outline(leaf), Some(Outline::Rectangle { .. }))),
    );
    seen(
        "a circle",
        leaves().any(|leaf| matches!(outline(leaf), Some(Outline::Circle { .. }))),
    );
    seen(
        "a circle not started at nought",
        leaves()
            .any(|leaf| matches!(outline(leaf), Some(Outline::Circle { from, .. }) if from != 0.0)),
    );
    seen(
        "a prism pushed backwards",
        leaves().any(|leaf| matches!(leaf, Leaf::Prism { height, .. } if *height < 0.0)),
    );
    for (name, kind) in [
        ("XY", Plane::xy(0.0)),
        ("XZ", Plane::xz(0.0)),
        ("YZ", Plane::yz(0.0)),
    ] {
        seen(
            name,
            leaves()
                .any(|leaf| std::mem::discriminant(leaf.plane()) == std::mem::discriminant(&kind)),
        );
    }
    let steps = || cases.iter().flat_map(|case| &case.steps);
    seen("a cut", steps().any(|step| step.mode == Mode::Cut));
    seen("an addition", steps().any(|step| step.mode == Mode::Add));
    for count in 1..=5 {
        seen(
            &format!("{count} steps"),
            cases.iter().any(|case| case.steps.len() == count),
        );
    }
    seen(
        "a tool on the plane of the solid before it",
        cases.iter().any(|case| {
            case.steps
                .iter()
                .any(|step| step.tool.plane() == case.start.plane())
        }),
    );
    seen(
        "a plane missed by a hair",
        cases.iter().any(|case| {
            case.steps
                .iter()
                .any(|step| match (case.start.plane(), step.tool.plane()) {
                    (Plane::Xy(first), Plane::Xy(second)) => {
                        first != second && (first - second).abs() < 1e-4
                    }
                    _ => false,
                })
        }),
    );
    seen(
        "a case drawn thirty times larger",
        leaves().any(|leaf| matches!(leaf, Leaf::Prism { height, .. } if height.abs() > 60.0)),
    );
    seen(
        "two circles on one centre",
        cases.iter().any(|case| {
            let centres: Vec<DVec2> = case
                .leaves()
                .filter_map(|leaf| match outline(leaf) {
                    Some(Outline::Circle { center, .. }) => Some(center),
                    _ => None,
                })
                .collect();
            (1..centres.len()).any(|index| centres[..index].contains(&centres[index]))
        }),
    );
}

fn outline_of(leaf: &Leaf) -> Option<&Outline> {
    match leaf {
        Leaf::Prism { outline, .. } => Some(outline),
        Leaf::Revolution { .. } | Leaf::Turned { .. } => None,
    }
}

#[test]
fn the_same_seed_draws_the_same_profile_case_and_not_the_square_one() {
    for seed in 0..200 {
        assert_eq!(
            Case::drawn_profiles(seed),
            Case::drawn_profiles(seed),
            "seed {seed}"
        );
    }
    let distinct: std::collections::BTreeSet<String> = (0..200)
        .map(|seed| Case::drawn_profiles(seed).to_string())
        .collect();
    assert!(distinct.len() > 190, "{} distinct cases", distinct.len());
    let same = (0..200)
        .filter(|seed| Case::drawn_profiles(*seed) == Case::drawn_square(*seed))
        .count();
    assert!(same < 100, "{same} profile cases are the square ones");
}

#[test]
fn a_profile_case_holds_solid_prisms_on_the_planes_of_the_origin_alone_and_no_star() {
    for seed in 0..3000 {
        let case = Case::drawn_profiles(seed);
        for leaf in case.leaves() {
            let kept = match leaf {
                Leaf::Prism { plane, outline, .. } => {
                    !matches!(plane, Plane::Tilted { .. })
                        && !matches!(outline, Outline::Star { .. })
                }
                Leaf::Revolution { .. } | Leaf::Turned { .. } => false,
            };
            assert!(kept, "seed {seed}: {case}");
            assert!(leaf.is_solid(), "seed {seed}: {case}");
        }
    }
}

#[test]
fn the_profile_generator_draws_every_kind_it_keeps_and_rectangles_and_circles_for_about_half() {
    let cases: Vec<Case> = (0..1000).map(Case::drawn_profiles).collect();
    let leaves = || cases.iter().flat_map(Case::leaves);
    let outlines = || leaves().filter_map(outline_of);
    let seen = |what: &str, found: bool| assert!(found, "no profile case drew {what}");

    seen(
        "a rounded rectangle with a straight run on every side",
        outlines().any(|outline| {
            matches!(outline, Outline::Rounded { low, high, radius }
                if 2.0 * radius < (*high - *low).min_element())
        }),
    );
    seen(
        "a rounded rectangle whose radius is half its shorter side",
        outlines().any(|outline| {
            matches!(outline, Outline::Rounded { low, high, radius }
                if 2.0 * radius == (*high - *low).min_element())
        }),
    );
    seen(
        "a slot along the plane's first axis",
        outlines()
            .any(|outline| matches!(outline, Outline::Slot { from, to, .. } if from.y == to.y)),
    );
    seen(
        "a slot along the plane's second axis",
        outlines()
            .any(|outline| matches!(outline, Outline::Slot { from, to, .. } if from.x == to.x)),
    );
    seen(
        "a ring",
        outlines().any(|outline| matches!(outline, Outline::Ring { .. })),
    );
    seen(
        "a prism pushed backwards",
        leaves().any(|leaf| matches!(leaf, Leaf::Prism { height, .. } if *height < 0.0)),
    );
    for (name, kind) in [
        ("XY", Plane::xy(0.0)),
        ("XZ", Plane::xz(0.0)),
        ("YZ", Plane::yz(0.0)),
    ] {
        seen(
            name,
            leaves()
                .any(|leaf| std::mem::discriminant(leaf.plane()) == std::mem::discriminant(&kind)),
        );
    }
    seen(
        "a case drawn thirty times larger",
        leaves().any(|leaf| matches!(leaf, Leaf::Prism { height, .. } if height.abs() > 60.0)),
    );

    let earlier_and_later = |related: &dyn Fn(&Outline, &Outline) -> bool| {
        cases.iter().any(|case| {
            let outlines: Vec<&Outline> = case.leaves().filter_map(outline_of).collect();
            (1..outlines.len()).any(|later| {
                outlines[..later]
                    .iter()
                    .any(|earlier| related(earlier, outlines[later]))
            })
        })
    };
    seen(
        "a corner radius equal to a circle's drawn before, on its centre",
        earlier_and_later(&|earlier, later| match (earlier, later) {
            (
                Outline::Circle { center, radius, .. },
                Outline::Rounded {
                    low,
                    radius: corner,
                    ..
                },
            ) => corner == radius && *low + *corner == *center,
            _ => false,
        }),
    );
    seen(
        "a slot's end on a circle's centre",
        earlier_and_later(&|earlier, later| match (earlier, later) {
            (Outline::Circle { center, .. }, Outline::Slot { from, .. }) => from == center,
            _ => false,
        }),
    );
    seen(
        "a slot's end missed by a hair",
        earlier_and_later(&|earlier, later| match (earlier, later) {
            (Outline::Circle { center, .. }, Outline::Slot { from, .. }) => {
                from != center && from.distance(*center) < 1e-4
            }
            _ => false,
        }),
    );
    seen(
        "a slot's straight run on a rectangle's side",
        earlier_and_later(&|earlier, later| match (earlier, later) {
            (Outline::Rectangle { high, .. }, Outline::Slot { from, to, radius }) => {
                from.y == to.y && (from.y - radius == high.y || from.y + radius == high.y)
            }
            _ => false,
        }),
    );
    seen(
        "a ring's bore equal to a circle drawn before",
        earlier_and_later(&|earlier, later| match (earlier, later) {
            (
                Outline::Circle { center, radius, .. },
                Outline::Ring {
                    center: hub, inner, ..
                },
            ) => center == hub && radius == inner,
            _ => false,
        }),
    );

    let count = leaves().count();
    let plain = outlines()
        .filter(|outline| matches!(outline, Outline::Rectangle { .. } | Outline::Circle { .. }))
        .count();
    let share = plain as f64 / count as f64;
    assert!(
        (0.4..0.6).contains(&share),
        "rectangles and circles are {plain} leaves of {count}"
    );
}

#[test]
fn a_profile_case_prints_as_the_rust_that_builds_it_again() {
    let pasted = Case::new(
        Leaf::prism(
            Plane::xy(0.0),
            Outline::rounded([0.0, 0.0], [10.0, 6.0], 1.5),
            4.0,
        ),
        vec![
            Step::cut(Leaf::prism(
                Plane::xz(-1.0),
                Outline::slot([2.0, 1.0], [2.0, 3.5], 0.75),
                -8.0,
            )),
            Step::add(Leaf::prism(
                Plane::xy(4.0),
                Outline::ring([5.0, 3.0], 2.5, 1.0),
                2.0,
            )),
        ],
    );
    assert_eq!(
        pasted.to_string(),
        "Case::new(
    Leaf::prism(Plane::xy(0.0), Outline::rounded([0.0, 0.0], [10.0, 6.0], 1.5), 4.0),
    vec![
        Step::cut(Leaf::prism(Plane::xz(-1.0), Outline::slot([2.0, 1.0], [2.0, 3.5], 0.75), -8.0)),
        Step::add(Leaf::prism(Plane::xy(4.0), Outline::ring([5.0, 3.0], 2.5, 1.0), 2.0)),
    ],
)"
    );
    for seed in 0..200 {
        let printed = Case::drawn_profiles(seed).to_string();
        assert!(
            !printed.contains("NaN") && !printed.contains("inf"),
            "seed {seed} prints a number Rust cannot read back: {printed}"
        );
    }
}

/// The outlines a lone prism of `outline` could shrink into.
fn shrunk_outlines(outline: Outline) -> Vec<Outline> {
    Case::new(Leaf::prism(Plane::xy(0.0), outline, 1.0), vec![])
        .smaller()
        .into_iter()
        .filter_map(|case| outline_of(&case.start).cloned())
        .collect()
}

#[test]
fn a_rounded_rectangle_shrinks_into_a_rectangle_and_a_slot_or_a_ring_into_a_circle_and_all_into_round_numbers()
 {
    let rounded = shrunk_outlines(Outline::rounded([0.3, 0.7], [10.2, 6.4], 1.7));
    for expected in [
        Outline::rectangle([0.3, 0.7], [10.2, 6.4]),
        Outline::rounded([0.0, 1.0], [10.0, 6.0], 2.0),
        Outline::rounded([0.5, 0.5], [10.0, 6.5], 1.5),
    ] {
        assert!(rounded.contains(&expected), "{expected} among {rounded:?}");
    }
    let tight = shrunk_outlines(Outline::rounded([0.0, 0.0], [10.0, 2.6], 1.3));
    assert!(
        tight.contains(&Outline::rounded([0.0, 0.0], [10.0, 3.0], 1.0)),
        "{tight:?}"
    );

    let slot = shrunk_outlines(Outline::slot([2.2, 5.0], [8.1, 5.0], 1.3));
    for expected in [
        Outline::circle([2.2, 5.0], 1.3),
        Outline::slot([2.0, 5.0], [8.0, 5.0], 1.0),
        Outline::slot([2.0, 5.0], [8.0, 5.0], 1.5),
    ] {
        assert!(slot.contains(&expected), "{expected} among {slot:?}");
    }
    assert!(
        slot.iter()
            .any(|outline| matches!(outline, Outline::Rectangle { low, high }
            if low.distance(DVec2::new(0.9, 3.7)) < 1e-12
                && high.distance(DVec2::new(9.4, 6.3)) < 1e-12)),
        "no box among {slot:?}"
    );
    let short = shrunk_outlines(Outline::slot([2.2, 5.0], [2.2, 5.4], 1.0));
    assert!(
        short
            .iter()
            .all(|outline| !matches!(outline, Outline::Slot { from, to, .. } if from == to)),
        "{short:?}"
    );

    let ring = shrunk_outlines(Outline::ring([5.2, 3.0], 2.6, 1.1));
    for expected in [
        Outline::circle([5.2, 3.0], 2.6),
        Outline::ring([5.0, 3.0], 3.0, 1.0),
        Outline::ring([5.0, 3.0], 2.5, 1.0),
    ] {
        assert!(ring.contains(&expected), "{expected} among {ring:?}");
    }
    let thin = shrunk_outlines(Outline::ring([5.0, 3.0], 2.6, 2.4));
    assert!(
        thin.iter().all(
            |outline| !matches!(outline, Outline::Ring { outer, inner, .. } if inner >= outer)
        ),
        "{thin:?}"
    );
}

#[test]
fn shrinking_any_drawn_profile_case_comes_to_an_end() {
    for seed in 0..200 {
        let mut rounds = 0;
        let shrunk = shrink(
            Case::drawn_profiles(seed),
            Case::smaller,
            |_| true,
            || {
                rounds += 1;
                rounds < 100_000
            },
        );
        assert!(rounds < 100_000, "seed {seed} was still shrinking");
        assert!(
            shrunk.smaller().is_empty(),
            "seed {seed} stopped short: {shrunk}"
        );
    }
}

fn bored_and_bossed() -> Case {
    Case::new(
        Leaf::prism(
            Plane::xy(0.0),
            Outline::rectangle([0.0, 0.0], [20.0, 10.0]),
            10.0,
        ),
        vec![
            Step::cut(Leaf::prism(
                Plane::xy(-1.0),
                Outline::circle([6.0, 5.0], 2.5),
                12.0,
            )),
            Step::add(Leaf::prism(
                Plane::xy(10.0),
                Outline::rectangle([12.0, 2.0], [18.0, 8.0]),
                4.0,
            )),
        ],
    )
}

/// A block bored through along Y from a circle not started at nought, given a
/// boss on its side along X, and pocketed from its top by a circle.
fn bored_across() -> Case {
    Case::new(
        Leaf::prism(
            Plane::xy(0.0),
            Outline::rectangle([0.0, 0.0], [10.0, 8.5]),
            10.0,
        ),
        vec![
            Step::cut(Leaf::prism(
                Plane::xz(-1.0),
                Outline::circle_from([3.0, 4.0], 2.5, 30.0),
                -12.0,
            )),
            Step::add(Leaf::prism(
                Plane::yz(10.0),
                Outline::rectangle([2.0, 2.0], [6.0, 6.0]),
                3.0,
            )),
            Step::cut(Leaf::prism(
                Plane::xy(10.0),
                Outline::circle([7.0, 5.0], 1.5),
                -3.0,
            )),
        ],
    )
}

#[test]
fn a_block_bored_and_given_a_boss_by_the_flats_keeps_every_rule_held_to_the_arithmetic() {
    let case = bored_and_bossed();
    let measured = random_solids::held_to_arithmetic(&case, &Flats::for_case(&case));
    assert!(measured.is_ok(), "{measured:?}");
}

#[test]
fn a_block_bored_across_and_given_a_boss_on_its_side_by_the_flats_keeps_every_rule_held_to_the_arithmetic()
 {
    let case = bored_across();
    let measured = random_solids::held_to_arithmetic(&case, &Flats::for_case(&case));
    assert!(measured.is_ok(), "{measured:?}");
}

#[test]
fn what_every_core_makes_of_the_seeds_comes_back_in_the_order_of_the_seeds() {
    let seeds: Vec<u64> = (0..1000).collect();
    let squared = random_solids::on_every_core(&seeds, |seed| seed * seed);
    assert_eq!(
        squared,
        seeds.iter().map(|seed| seed * seed).collect::<Vec<_>>()
    );
}

/// Every case of `draw` among the seeds that the exact kernel answers
/// keeping every rule, held again through the application's body: what it
/// adds — the face numbers, the drawing as it builds, the declines it adds —
/// must not lose a case the kernel keeps.
fn kept_through_the_application(draw: fn(u64) -> Case) {
    let seeds: Vec<u64> = (0..60).collect();
    let weighed = random_solids::on_every_core(&seeds, |seed| {
        let case = draw(*seed);
        random_solids::held_to_arithmetic(&case, &Exact).ok()?;
        Some((random_solids::held_to_arithmetic(&case, &Application), case))
    });
    let mut held = 0;
    for (seed, (measured, case)) in seeds
        .iter()
        .zip(weighed)
        .filter_map(|(seed, one)| Some((seed, one?)))
    {
        held += 1;
        assert!(measured.is_ok(), "seed {seed}: {measured:?}\n{case}");
    }
    assert!(held > 30, "{held} cases held");
}

#[test]
fn square_cases_the_exact_kernel_keeps_are_kept_through_the_application_s_body() {
    kept_through_the_application(Case::drawn_square);
}

#[test]
fn profile_cases_the_exact_kernel_keeps_are_kept_through_the_application_s_body() {
    kept_through_the_application(Case::drawn_profiles);
}

#[test]
fn square_cases_the_flats_answer_by_their_own_rules_keep_every_rule_held_to_the_arithmetic() {
    let seeds: Vec<u64> = (0..120).collect();
    let weighed = random_solids::on_every_core(&seeds, |seed| {
        let case = Case::drawn_square(*seed);
        random_solids::check(&case).ok()?;
        let measured = random_solids::held_to_arithmetic(&case, &Flats::for_case(&case));
        Some((measured, case))
    });
    let mut held = 0;
    for (seed, (measured, case)) in seeds
        .iter()
        .zip(weighed)
        .filter_map(|(seed, one)| Some((seed, one?)))
    {
        held += 1;
        assert!(measured.is_ok(), "seed {seed}: {measured:?}\n{case}");
    }
    assert!(held > 60, "{held} cases held");
}

#[test]
fn profile_cases_the_flats_answer_by_their_own_rules_keep_every_rule_held_to_the_arithmetic() {
    let seeds: Vec<u64> = (0..120).collect();
    let weighed = random_solids::on_every_core(&seeds, |seed| {
        let case = Case::drawn_profiles(*seed);
        random_solids::check(&case).ok()?;
        let measured = random_solids::held_to_arithmetic(&case, &Flats::for_case(&case));
        Some((measured, case))
    });
    let mut held = 0;
    let mut profiles = 0;
    for (seed, (measured, case)) in seeds
        .iter()
        .zip(weighed)
        .filter_map(|(seed, one)| Some((seed, one?)))
    {
        held += 1;
        profiles += case
            .leaves()
            .filter_map(outline_of)
            .filter(|outline| {
                matches!(
                    outline,
                    Outline::Rounded { .. } | Outline::Slot { .. } | Outline::Ring { .. }
                )
            })
            .count();
        assert!(measured.is_ok(), "seed {seed}: {measured:?}\n{case}");
    }
    assert!(held > 60, "{held} cases held");
    assert!(profiles > 60, "{profiles} profiles held");
}

#[test]
fn lines_grazing_a_curved_wall_are_left_out_and_counted() {
    let case = Case::new(
        Leaf::prism(Plane::xz(2.0), Outline::circle([1.0, 3.0], 4.0), 5.0),
        vec![],
    );
    let measured =
        random_solids::held_to_arithmetic(&case, &Flats::for_case(&case)).expect("a sound disc");
    assert!(measured.grazing > 0, "{measured:?}");
    assert!(measured.held > 20 * measured.grazing, "{measured:?}");
}

/// A kernel broken on purpose, the flats underneath.
struct Broken {
    flats: Flats,
    breaking: Breaking,
    calls: Cell<usize>,
}

#[derive(Clone, Copy, PartialEq)]
enum Breaking {
    ForgetsItsCuts,
    DeclinesLeaves,
    DeclinesSteps,
    ListsWrong,
    AnswersTwiceDifferently,
    LeavesMatterAside,
    HoldsNothingExactly,
}

impl Broken {
    fn on(case: &Case, breaking: Breaking) -> Broken {
        Broken {
            flats: Flats::for_case(case),
            breaking,
            calls: Cell::new(0),
        }
    }
}

impl Kernel for Broken {
    type Body = cao_solid::Body;

    fn raised(&self, leaf: &Leaf) -> Result<cao_solid::Body, Declined> {
        if self.breaking == Breaking::DeclinesLeaves {
            return Err(Declined::Unfinished);
        }
        self.flats.raised(leaf)
    }

    fn combined(
        &self,
        body: &cao_solid::Body,
        tool: &cao_solid::Body,
        mode: Mode,
    ) -> Result<cao_solid::Body, Declined> {
        match (self.breaking, mode) {
            (Breaking::ForgetsItsCuts, Mode::Cut) => Ok(body.clone()),
            (Breaking::DeclinesSteps, _) => Err(Declined::Unfinished),
            _ => self.flats.combined(body, tool, mode),
        }
    }

    fn triangles(&self, body: &cao_solid::Body) -> (Vec<Triangle>, f64) {
        let (mut triangles, tolerance) = self.flats.triangles(body);
        self.calls.set(self.calls.get() + 1);
        match self.breaking {
            Breaking::AnswersTwiceDifferently => {
                let corner = &mut triangles[0][0].x;
                *corner = f64::from_bits(corner.to_bits() + self.calls.get() as u64);
            }
            Breaking::LeavesMatterAside => {
                let far = Leaf::prism(
                    Plane::xy(0.0),
                    Outline::rectangle([100.0, 100.0], [101.0, 101.0]),
                    1.0,
                );
                triangles.extend(far.solid().expect("a cube").triangles());
            }
            _ => {}
        }
        (triangles, tolerance)
    }

    fn crossings(
        &self,
        _body: &cao_solid::Body,
        _origin: DVec3,
        _direction: DVec3,
    ) -> Option<Vec<(f64, i32)>> {
        (self.breaking == Breaking::HoldsNothingExactly).then(Vec::new)
    }

    fn listing(&self, _body: &cao_solid::Body) -> Option<(Listing, f64)> {
        (self.breaking == Breaking::ListsWrong).then(|| {
            let line = Line::through(DVec3::ZERO, DVec3::X);
            let edge = ListedEdge {
                curve: Curve::Line(line),
                from: 0.0,
                to: 1.0,
                ends: Some([0, 1]),
                sides: Vec::new(),
            };
            let listing = Listing {
                faces: Vec::new(),
                edges: vec![edge],
                vertices: vec![DVec3::ZERO, DVec3::X],
            };
            (listing, 1.0)
        })
    }
}

fn broken(breaking: Breaking) -> Result<random_solids::Measured, Flaw> {
    let case = bored_and_bossed();
    random_solids::held_to_arithmetic(&case, &Broken::on(&case, breaking))
}

#[test]
fn a_kernel_that_forgets_its_cuts_is_caught_along_a_line_that_should_have_lost_matter() {
    let found = broken(Breaking::ForgetsItsCuts);
    assert!(
        matches!(&found, Err(Flaw::Volume { worst: Some(worst), .. }) if worst.enclosed > worst.promised),
        "{found:?}"
    );
}

#[test]
fn a_kernel_that_declines_a_leaf_or_a_step_gives_no_answer() {
    for breaking in [Breaking::DeclinesLeaves, Breaking::DeclinesSteps] {
        assert!(matches!(
            broken(breaking),
            Err(Flaw::NoAnswer(Silence::Refused))
        ));
    }
}

#[test]
fn a_kernel_that_lists_its_body_wrong_is_caught_by_the_listing() {
    assert!(matches!(
        broken(Breaking::ListsWrong),
        Err(Flaw::Mislisted(Mislisted::Unused { edge: 0 }))
    ));
}

#[test]
fn a_kernel_that_answers_the_same_case_twice_differently_is_caught() {
    assert!(matches!(
        broken(Breaking::AnswersTwiceDifferently),
        Err(Flaw::Unrepeatable { .. })
    ));
}

#[test]
fn a_kernel_whose_exact_body_holds_nothing_where_matter_was_promised_is_caught_before_its_triangles()
 {
    assert!(
        matches!(broken(Breaking::HoldsNothingExactly), Err(Flaw::Spans(worst)) if worst.enclosed == 0.0 && worst.promised > 0.0)
    );
}

#[test]
fn matter_a_kernel_leaves_beyond_what_its_leaves_span_is_caught() {
    assert!(matches!(
        broken(Breaking::LeavesMatterAside),
        Err(Flaw::Volume { worst: None, .. })
    ));
}

#[test]
fn a_line_through_a_prism_is_told_how_squarely_it_crosses_each_end_and_whether_it_curves() {
    let leaf = Leaf::prism(Plane::xy(1.0), Outline::circle([0.0, 0.0], 2.0), 3.0);
    let slanted = DVec3::new(0.6, 0.0, 0.8);
    let stretches = leaf
        .along(DVec3::new(-0.5, 0.0, 0.0), slanted)
        .expect("a prism");
    let [stretch] = stretches.as_slice() else {
        panic!("one stretch: {stretches:?}");
    };
    assert!((stretch.from.at - 1.25).abs() < 1e-12, "{stretch:?}");
    assert!((stretch.from.cosine - 0.8).abs() < 1e-12 && !stretch.from.curved);
    assert!((stretch.to.at - 2.5 / 0.6).abs() < 1e-12, "{stretch:?}");
    assert!((stretch.to.cosine - 0.6).abs() < 1e-12 && stretch.to.curved);
}

#[test]
fn a_line_square_to_a_star_holds_the_whole_prism_inside_the_star_and_nothing_between_its_points() {
    let center = DVec2::new(1.0, 1.0);
    let corners: Vec<DVec2> = (0..10)
        .map(|index| {
            let reach = if index % 2 == 0 { 2.0 } else { 0.8 };
            center + DVec2::from_angle(0.3 + std::f64::consts::TAU * index as f64 / 10.0) * reach
        })
        .collect();
    let leaf = Leaf::prism(
        Plane::xy(1.0),
        Outline::Star {
            center,
            corners: corners.clone(),
        },
        3.0,
    );
    let square = |at: DVec2| {
        leaf.along(at.extend(-1.0), DVec3::Z)
            .expect("a prism")
            .iter()
            .map(|stretch| (stretch.from.at, stretch.to.at))
            .collect::<Vec<_>>()
    };
    assert_eq!(square(center), [(2.0, 5.0)]);
    let between = center + (corners[1] - center) * 1.5;
    assert_eq!(square(between), []);
    assert_eq!(square(center + DVec2::X * 5.0), []);
}

/// The area a profile encloses, by the formula for its shape.
fn area(outline: &Outline) -> f64 {
    use std::f64::consts::PI;
    match outline {
        Outline::Rectangle { low, high } => (*high - *low).x * (*high - *low).y,
        Outline::Circle { radius, .. } => PI * radius * radius,
        Outline::Ring { outer, inner, .. } => PI * (outer * outer - inner * inner),
        Outline::Rounded { low, high, radius } => {
            (*high - *low).x * (*high - *low).y - (4.0 - PI) * radius * radius
        }
        Outline::Slot { from, to, radius } => {
            2.0 * radius * from.distance(*to) + PI * radius * radius
        }
        Outline::Ellipse { first, second, .. } => PI * first.length() * second,
        Outline::Chorded {
            first,
            second,
            arcs,
            ..
        } => {
            let gaps: f64 = (0..arcs.len())
                .map(|index| {
                    let next = arcs[(index + 1) % arcs.len()][0]
                        + if index + 1 == arcs.len() { 360.0 } else { 0.0 };
                    let gap = (next - arcs[index][1]).to_radians();
                    (gap - gap.sin()) / 2.0
                })
                .sum();
            first.length() * second * (PI - gaps)
        }
        Outline::OvalSlot {
            from,
            to,
            across,
            along,
        } => 2.0 * across * from.distance(*to) + PI * across * along,
        Outline::OvalRing {
            first,
            second,
            hole,
            ..
        } => PI * (first.length() * second - hole.x * hole.y),
        Outline::Star { .. } => unreachable!("a star is not raised by the exact kernel"),
    }
}

#[test]
fn every_profile_raised_by_the_exact_kernel_alone_keeps_every_rule_and_encloses_its_area_times_its_height()
 {
    let leaves: Vec<Leaf> = (0..200)
        .flat_map(|seed| {
            Case::drawn_profiles(seed)
                .leaves()
                .cloned()
                .collect::<Vec<_>>()
        })
        .filter(|leaf| {
            matches!(
                leaf,
                Leaf::Prism {
                    outline: Outline::Rounded { .. } | Outline::Slot { .. } | Outline::Ring { .. },
                    ..
                }
            )
        })
        .collect();
    assert!(leaves.len() > 60, "{} leaves", leaves.len());
    let weighed = random_solids::on_every_core(&leaves, |leaf| {
        let alone = Case::new(leaf.clone(), vec![]);
        let held = random_solids::held_to_arithmetic(&alone, &Exact);
        (held, Exact.raised(leaf).map(|body| body.volume()))
    });
    for (leaf, (held, volume)) in leaves.iter().zip(weighed) {
        assert!(held.is_ok(), "{leaf}: {held:?}");
        let Leaf::Prism {
            outline, height, ..
        } = leaf
        else {
            unreachable!("only prisms were kept")
        };
        let promised = area(outline) * height.abs();
        let volume = volume.expect("a raised profile");
        assert!(
            (volume - promised).abs() <= 1e-9 * promised,
            "{leaf}: {volume} against {promised}"
        );
    }
}

#[test]
fn a_rounded_rectangle_is_handed_over_with_a_straight_run_only_on_a_side_longer_than_its_two_corners()
 {
    let straight = |outline: Outline| {
        let contour = outline.contour().expect("an outline of runs and arcs");
        assert_eq!(contour.corners.len(), contour.runs.len());
        contour
            .runs
            .iter()
            .filter(|run| matches!(run, Run::Straight))
            .count()
    };
    assert_eq!(straight(Outline::rounded([0.0, 0.0], [10.0, 6.0], 2.0)), 4);
    assert_eq!(straight(Outline::rounded([0.0, 0.0], [10.0, 6.0], 3.0)), 2);
    assert_eq!(straight(Outline::rounded([0.0, 0.0], [6.0, 6.0], 3.0)), 0);
    assert_eq!(straight(Outline::slot([0.0, 0.0], [0.0, 4.0], 1.0)), 2);
}

#[test]
fn a_rounded_rectangle_is_its_rectangle_less_four_corners_a_circle_never_filled() {
    let block = Leaf::prism(
        Plane::xz(1.0),
        Outline::rounded([0.0, 0.0], [10.0, 6.0], 2.0),
        -3.0,
    );
    let volume = Exact.raised(&block).expect("a rounded block").volume();
    let promised = 3.0 * (60.0 - (4.0 - std::f64::consts::PI) * 4.0);
    assert!((volume - promised).abs() < 1e-9, "{volume}");
}

#[test]
fn a_circle_is_handed_to_the_exact_kernel_as_one_whole_turn_from_where_its_flats_start() {
    let circle = random_solids::whole_circle(DVec2::new(3.0, -1.0), 2.0, 90.0);
    let [corner] = circle.corners.as_slice() else {
        panic!("one corner: {circle:?}");
    };
    assert!(
        (*corner - DVec2::new(3.0, 1.0)).length() < 1e-12,
        "{corner}"
    );
    assert!(matches!(
        circle.runs.as_slice(),
        [Run::Round { center, turn }] if *center == DVec2::new(3.0, -1.0) && *turn == std::f64::consts::TAU
    ));
}

#[test]
fn a_block_bored_and_given_a_boss_by_the_exact_kernel_keeps_every_rule() {
    random_solids::holds_exactly(&bored_and_bossed());
}

#[test]
fn a_block_bored_across_and_given_a_boss_on_its_side_by_the_exact_kernel_keeps_every_rule() {
    random_solids::holds_exactly(&bored_across());
}

#[test]
fn a_line_through_an_ellipse_is_told_where_it_crosses_its_wall_and_how_squarely() {
    let crossed = |leaf: &Leaf, origin: [f64; 3], direction: DVec3, by: f64| {
        let stretches = leaf
            .along_grown(DVec3::from(origin), direction, by)
            .expect("a prism");
        let [stretch] = stretches[..] else {
            panic!("one stretch across {leaf}: {stretches:?}");
        };
        stretch
    };
    let close = |found: f64, expected: f64| (found - expected).abs() < 1e-12;

    let oval = Leaf::prism(
        Plane::xy(0.0),
        Outline::ellipse([1.0, 2.0], [3.0, 0.0], 2.0),
        4.0,
    );
    let along = crossed(&oval, [-5.0, 2.0, 1.0], DVec3::X, 0.0);
    assert!(
        close(along.from.at, 3.0) && close(along.to.at, 9.0),
        "{along:?}"
    );
    for end in [along.from, along.to] {
        assert!(end.curved && close(end.cosine, 1.0), "{end:?}");
    }
    let across = crossed(&oval, [2.5, 0.0, 1.0], DVec3::Y, 0.0);
    let half = 3.0_f64.sqrt();
    assert!(
        close(across.from.at, 2.0 - half) && close(across.to.at, 2.0 + half),
        "{across:?}"
    );
    for end in [across.from, across.to] {
        assert!(close(end.cosine, (27.0_f64 / 31.0).sqrt()), "{end:?}");
    }

    let turned = Leaf::prism(
        Plane::xz(1.0),
        Outline::ellipse_from([0.0, 0.0], [3.0, 4.0], 2.0, 40.0),
        -2.0,
    );
    let way = DVec3::new(0.6, 0.0, 0.8);
    let diagonal = crossed(&turned, [-6.0, 1.5, -8.0], way, 0.0);
    assert!(
        close(diagonal.from.at, 5.0) && close(diagonal.to.at, 15.0),
        "{diagonal:?}"
    );

    let flat = Leaf::prism(
        Plane::xy(0.0),
        Outline::ellipse([0.0, 0.0], [4.0, 0.0], 1.0),
        4.0,
    );
    for (by, major, minor) in [(0.5, 24.75_f64.sqrt(), 1.5), (-0.5, 7.75_f64.sqrt(), 0.5)] {
        let long = crossed(&flat, [-10.0, 0.0, 2.0], DVec3::X, by);
        assert!(
            close(long.from.at, 10.0 - major) && close(long.to.at, 10.0 + major),
            "grown by {by}: {long:?}"
        );
        let short = crossed(&flat, [0.0, -10.0, 2.0], DVec3::Y, by);
        assert!(
            close(short.from.at, 10.0 - minor) && close(short.to.at, 10.0 + minor),
            "grown by {by}: {short:?}"
        );
    }
    let gone = flat
        .along_grown(DVec3::new(0.0, -10.0, 2.0), DVec3::Y, -1.0)
        .expect("a prism");
    assert!(gone.is_empty(), "{gone:?}");
}

/// The distance from a place to the nearest place of an ellipse's wall: the
/// nearest of many places round it, then the turn about it narrowed down.
fn distance_to_the_wall(oval: Oval, place: DVec2) -> f64 {
    let distance = |angle: f64| oval.at(angle).distance(place);
    let count = 1024;
    let step = std::f64::consts::TAU / count as f64;
    let nearest = (0..count)
        .map(|index| index as f64 * step)
        .min_by(|one, other| distance(*one).total_cmp(&distance(*other)))
        .expect("places round the ellipse");
    let (mut low, mut high) = (nearest - step, nearest + step);
    for _ in 0..100 {
        let (one, other) = (low + (high - low) / 3.0, high - (high - low) / 3.0);
        if distance(one) < distance(other) {
            high = other;
        } else {
            low = one;
        }
    }
    distance((low + high) / 2.0)
}

#[test]
fn an_ellipse_grown_holds_every_place_within_reach_of_it_and_shrunk_none_nearer_its_wall() {
    let mut random = cao_solid::soundness::Random::seeded(571);
    let mut shrunk_ones = 0;
    for _ in 0..200 {
        let first = DVec2::from_angle(random.between(0.0, 6.3)) * random.between(0.2, 8.0);
        let oval = Oval::new(
            DVec2::new(random.between(-3.0, 3.0), random.between(-3.0, 3.0)),
            first,
            random.between(0.2, 8.0),
        );
        let by = random.between(0.0, 1.5) * oval.minor();
        let grown = oval.grown(by).expect("an ellipse grows");
        for step in 0..720 {
            let angle = step as f64 / 720.0 * std::f64::consts::TAU;
            let reached = oval.at(angle) + oval.normal_at(angle) * by;
            assert!(
                grown.level(reached) <= 1e-12,
                "{oval:?} grown by {by} leaves out {reached}, {} past it",
                grown.level(reached)
            );
        }
        let Some(shrunk) = oval.grown(-by) else {
            assert!(by >= oval.minor(), "{oval:?} shrunk by {by} is gone");
            continue;
        };
        shrunk_ones += 1;
        for step in 0..90 {
            let place = shrunk.at(step as f64 / 90.0 * std::f64::consts::TAU);
            assert!(oval.level(place) < 0.0, "{oval:?} shrunk by {by}: {place}");
            let distance = distance_to_the_wall(oval, place);
            assert!(
                distance >= by - 1e-9 * oval.major(),
                "{oval:?} shrunk by {by} holds {place}, {distance} from the wall"
            );
        }
    }
    assert!(shrunk_ones > 60, "{shrunk_ones} ellipses shrunk");
}

#[test]
fn an_outline_an_ellipse_bounds_is_handed_to_the_exact_kernel_as_its_arcs_among_its_straight_runs()
{
    use std::f64::consts::{PI, TAU};
    let near = |one: DVec2, other: [f64; 2]| one.distance(DVec2::from(other)) < 1e-12;
    let arc = |run: &Run| match *run {
        Run::Oval {
            center,
            first,
            second,
            turn,
        } => Some((center, first, second, turn)),
        _ => None,
    };

    let (whole, holes) = Outline::ellipse_from([1.0, 2.0], [3.0, 0.0], 2.0, 90.0)
        .oval_contours()
        .expect("an ellipse");
    assert!(holes.is_empty() && whole.corners.len() == 1, "{whole:?}");
    assert!(near(whole.corners[0], [1.0, 4.0]), "{whole:?}");
    assert_eq!(
        arc(&whole.runs[0]),
        Some((DVec2::new(1.0, 2.0), DVec2::new(3.0, 0.0), 2.0, TAU))
    );

    let (cut, _) = Outline::chorded(
        [0.0, 0.0],
        [2.0, 0.0],
        1.0,
        &[[-60.0, 60.0], [120.0, 240.0]],
    )
    .oval_contours()
    .expect("an ellipse cut twice");
    let half = 3.0_f64.sqrt() / 2.0;
    for (corner, expected) in
        cut.corners
            .iter()
            .zip([[1.0, -half], [1.0, half], [-1.0, half], [-1.0, -half]])
    {
        assert!(near(*corner, expected), "{cut:?}");
    }
    for (index, run) in cut.runs.iter().enumerate() {
        match arc(run) {
            Some((_, _, _, turn)) => {
                assert!(
                    index % 2 == 0 && (turn - 2.0 * PI / 3.0).abs() < 1e-12,
                    "{cut:?}"
                )
            }
            None => assert!(index % 2 == 1 && *run == Run::Straight, "{cut:?}"),
        }
    }
    let (_, curves) = random_solids::sampled(&cut);
    let mut arcs: Vec<usize> = curves.iter().flatten().copied().collect();
    arcs.dedup();
    assert_eq!(
        arcs.len(),
        2,
        "each arc of the ellipse its own curve, as the application numbers \
         every run: {curves:?}"
    );

    let (slot, _) = Outline::oval_slot([0.0, 0.0], [4.0, 0.0], 1.0, 2.0)
        .oval_contours()
        .expect("a slot with elliptic ends");
    for (corner, expected) in
        slot.corners
            .iter()
            .zip([[0.0, -1.0], [4.0, -1.0], [4.0, 1.0], [0.0, 1.0]])
    {
        assert!(near(*corner, expected), "{slot:?}");
    }
    assert_eq!(
        slot.runs.iter().map(arc).collect::<Vec<_>>(),
        [
            None,
            Some((DVec2::new(4.0, 0.0), DVec2::new(2.0, 0.0), 1.0, PI)),
            None,
            Some((DVec2::ZERO, DVec2::new(2.0, 0.0), 1.0, PI)),
        ]
    );

    let (rim, bores) = Outline::oval_ring([1.0, 1.0], [0.0, 3.0], 2.0, [1.0, 1.0])
        .oval_contours()
        .expect("an elliptic ring");
    assert!(near(rim.corners[0], [1.0, 4.0]), "{rim:?}");
    let [bore] = bores.as_slice() else {
        panic!("one hole: {bores:?}");
    };
    assert!(near(bore.corners[0], [1.0, 2.0]), "{bore:?}");
    assert!(
        matches!(bore.runs.as_slice(), [Run::Round { center, turn }] if *center == DVec2::ONE && *turn == TAU),
        "a round hole is a whole circle: {bore:?}"
    );
    let (_, bores) = Outline::oval_ring([1.0, 1.0], [0.0, 3.0], 2.0, [1.5, 0.5])
        .oval_contours()
        .expect("an elliptic ring");
    assert!(near(bores[0].corners[0], [1.0, 2.5]), "{bores:?}");
    assert_eq!(
        arc(&bores[0].runs[0]),
        Some((DVec2::ONE, DVec2::new(0.0, 1.5), 0.5, TAU))
    );
    assert_eq!(Outline::circle([0.0, 0.0], 1.0).oval_contours(), None);
}

/// The area of the polygon a unit circle's arcs are sampled into, each arc
/// from one angle to a larger one in degrees, the next one's start joined to
/// each one's end: as many steps as its share of a whole turn's, and never
/// fewer than two.
fn sampled_unit_area(arcs: &[[f64; 2]]) -> f64 {
    let steps = random_solids::CIRCLE_STEPS as f64;
    (0..arcs.len())
        .map(|index| {
            let [from, to] = arcs[index];
            let sweep = (to - from).to_radians();
            let count = (sweep / std::f64::consts::TAU * steps).ceil().max(2.0);
            let next = arcs[(index + 1) % arcs.len()][0]
                + if index + 1 == arcs.len() { 360.0 } else { 0.0 };
            count / 2.0 * (sweep / count).sin() + (next - to).to_radians().sin() / 2.0
        })
        .sum()
}

#[test]
fn the_flats_of_an_outline_an_ellipse_bounds_enclose_the_polygon_its_places_are_sampled_on() {
    let whole = sampled_unit_area(&[[0.0, 360.0]]);
    let leaves = [
        (Outline::ellipse([1.0, 2.0], [3.0, 0.0], 2.0), 6.0 * whole),
        (
            Outline::ellipse_from([0.0, -1.0], [1.2, -1.6], 4.5, 33.0),
            9.0 * whole,
        ),
        (
            Outline::chorded([2.0, 2.0], [0.0, 3.0], 1.5, &[[-60.0, 60.0]]),
            4.5 * sampled_unit_area(&[[-60.0, 60.0]]),
        ),
        (
            Outline::chorded([2.0, 2.0], [3.0, 0.0], 1.5, &[[-20.0, 20.0]]),
            4.5 * sampled_unit_area(&[[-20.0, 20.0]]),
        ),
        (
            Outline::chorded(
                [0.0, 0.0],
                [2.0, 2.0],
                1.0,
                &[[-60.0, 60.0], [120.0, 240.0]],
            ),
            8.0_f64.sqrt() * sampled_unit_area(&[[-60.0, 60.0], [120.0, 240.0]]),
        ),
        (
            Outline::oval_slot([0.0, 0.0], [3.0, 4.0], 1.0, 2.0),
            2.0 * 5.0 + 2.0 * whole,
        ),
        (
            Outline::oval_ring([1.0, 1.0], [0.0, 3.0], 2.0, [1.0, 1.0]),
            5.0 * whole,
        ),
        (
            Outline::oval_ring([1.0, 1.0], [2.0, 2.0], 2.0, [2.0, 1.0]),
            (8.0_f64.sqrt() * 2.0 - 2.0) * whole,
        ),
    ];
    for (outline, area) in leaves {
        for plane in [
            Plane::xy(1.0),
            Plane::xz(-2.0),
            Plane::tilted([1.0, 0.0, 2.0], [30.0, 15.0, 0.0]),
        ] {
            let leaf = Leaf::prism(plane, outline.clone(), -2.5);
            assert!(leaf.is_solid(), "{leaf}");
            let (promised, _) = leaf.promise();
            assert!(
                (promised - 2.5 * area).abs() < 1e-12 * promised,
                "{leaf}: {promised} against {}",
                2.5 * area
            );
            let flats = leaf.solid().expect("a leaf the flats raise");
            random_solids::kept_its_promise(&leaf, &flats.triangles())
                .unwrap_or_else(|flaw| panic!("{leaf}: {flaw:?}"));
            random_solids::check(&Case::new(leaf.clone(), vec![]))
                .unwrap_or_else(|flaw| panic!("{leaf}: {flaw:?}"));
        }
    }
}

/// Places along the outline and the holes an outline an ellipse bounds is
/// handed over as, close enough together that, from a place a tenth of a
/// unit or more from the wall, the nearest of them stands under a
/// ten-thousandth further than the wall itself.
fn walls_of(outline: &Outline) -> Vec<DVec2> {
    let (contour, holes) = outline
        .oval_contours()
        .expect("an outline an ellipse bounds");
    std::iter::once(&contour)
        .chain(&holes)
        .flat_map(|contour| {
            let count = contour.corners.len();
            (0..count).flat_map(move |index| {
                let corner = contour.corners[index];
                let next = contour.corners[(index + 1) % count];
                let steps = 1000;
                (0..steps).map(move |step| {
                    let share = step as f64 / steps as f64;
                    match contour.runs[index] {
                        Run::Straight => corner + (next - corner) * share,
                        Run::Round { center, turn } => {
                            center + DVec2::from_angle(turn * share).rotate(corner - center)
                        }
                        Run::Oval {
                            center,
                            first,
                            second,
                            turn,
                        } => {
                            let oval = Oval::new(center, first, second);
                            let out = corner - center;
                            let along = first.perp().normalize() * second;
                            let start = DVec2::new(
                                out.dot(first) / first.length_squared(),
                                out.dot(along) / (second * second),
                            )
                            .to_angle();
                            oval.at(start + turn * share)
                        }
                    }
                })
            })
        })
        .collect()
}

#[test]
fn a_slot_with_elliptic_ends_or_an_elliptic_ring_shrunk_holds_no_place_nearer_its_wall_than_it_is_shrunk_by()
 {
    let outlines = [
        Outline::oval_slot([0.0, 0.0], [4.0, 0.0], 2.0, 0.5),
        Outline::oval_slot([0.0, 0.0], [3.0, 4.0], 1.5, 0.25),
        Outline::oval_slot([1.0, 1.0], [1.0, 3.0], 0.5, 2.0),
        Outline::oval_ring([0.0, 0.0], [4.0, 0.0], 1.0, [1.0, 0.9]),
        Outline::oval_ring([1.0, -1.0], [2.0, 2.0], 1.2, [2.0, 1.0]),
        Outline::oval_ring([0.0, 0.0], [0.0, 3.0], 2.0, [1.9, 1.9]),
    ];
    let held: usize = random_solids::on_every_core(&outlines, |outline| {
        let walls = walls_of(outline);
        let leaf = Leaf::prism(Plane::xy(0.0), outline.clone(), 10.0);
        let mut held = 0;
        for by in [0.1, 0.4] {
            for way in [DVec3::X, DVec3::new(0.6, 0.8, 0.0)] {
                for across in -30..=30 {
                    let origin =
                        way.cross(DVec3::Z) * (across as f64 * 0.2) + DVec3::Z * 5.0 - way * 20.0;
                    let stretches = leaf.along_grown(origin, way, -by).expect("a prism");
                    for stretch in stretches {
                        for step in 0..=20 {
                            let at = stretch.from.at
                                + (stretch.to.at - stretch.from.at) * step as f64 / 20.0;
                            let place = origin + way * at;
                            let place = DVec2::new(place.x, place.y);
                            let nearest = walls
                                .iter()
                                .map(|wall| wall.distance(place))
                                .fold(f64::INFINITY, f64::min);
                            held += 1;
                            assert!(
                                nearest >= by - 1e-3,
                                "{outline} shrunk by {by} holds {place}, {nearest} from its wall"
                            );
                        }
                    }
                }
            }
        }
        held
    })
    .into_iter()
    .sum();
    assert!(held > 10_000, "{held} places held");
}

/// Prisms of every outline an ellipse bounds, on every kind of plane, drawn
/// from a seed of their own: axes on the plane's and turned, the first
/// semi-axis the major or the minor, one trait or two, a slot either way, a
/// round hole or an elliptic one.
fn elliptic_prisms(count: usize) -> Vec<Leaf> {
    use std::f64::consts::TAU;
    let mut random = cao_solid::soundness::Random::seeded(5_571);
    (0..count)
        .map(|_| {
            let plane = match random.below(4) {
                0 => Plane::xy(random.between(-2.0, 2.0)),
                1 => Plane::xz(random.between(-2.0, 2.0)),
                2 => Plane::yz(random.between(-2.0, 2.0)),
                _ => Plane::tilted(
                    [random.between(-2.0, 2.0), 0.0, random.between(-2.0, 2.0)],
                    [
                        random.between(-80.0, 80.0),
                        random.between(-80.0, 80.0),
                        0.0,
                    ],
                ),
            };
            let center = DVec2::new(random.between(-4.0, 4.0), random.between(-4.0, 4.0));
            let direction = if random.chance(0.5) {
                *random.pick(&[DVec2::X, DVec2::Y, DVec2::NEG_X, DVec2::NEG_Y])
            } else {
                DVec2::from_angle(random.between(0.0, TAU))
            };
            let (length, second) = (random.between(0.5, 5.0), random.between(0.5, 5.0));
            let first = direction * length;
            let outline = match random.below(4) {
                0 => Outline::Ellipse {
                    center,
                    first,
                    second,
                    from: if random.chance(0.5) {
                        0.0
                    } else {
                        random.between(0.0, 360.0)
                    },
                },
                1 => {
                    let start = random.between(-180.0, 180.0);
                    let arcs = if random.chance(0.5) {
                        vec![[start, start + random.between(20.0, 340.0)]]
                    } else {
                        let ends =
                            [10.0, 10.0, 10.0].map(|least| least + random.between(0.0, 70.0));
                        vec![
                            [start, start + ends[0]],
                            [
                                start + ends[0] + ends[1],
                                start + ends[0] + ends[1] + ends[2],
                            ],
                        ]
                    };
                    Outline::Chorded {
                        center,
                        first,
                        second,
                        arcs,
                    }
                }
                2 => Outline::OvalSlot {
                    from: center,
                    to: center + direction * random.between(0.5, 6.0),
                    across: second,
                    along: length,
                },
                _ => {
                    let least = length.min(second);
                    let hole = if random.chance(0.5) {
                        DVec2::splat(least * random.between(0.1, 0.9))
                    } else {
                        DVec2::new(length, second) * random.between(0.1, 0.9)
                    };
                    Outline::OvalRing {
                        center,
                        first,
                        second,
                        hole,
                    }
                }
            };
            let height = random.between(1.0, 6.0) * if random.chance(0.3) { -1.0 } else { 1.0 };
            Leaf::prism(plane, outline, height)
        })
        .collect()
}

/// How far a place stands outside an ellipse, or inside it when negative,
/// to the first order: its level over the level's slope, which is the
/// distance itself near the wall; and the wall's outward normal there.
fn off_an_ellipse(oval: Oval, place: DVec2) -> (f64, DVec2) {
    let unit = oval.first.normalize();
    let away = place - oval.center;
    let (a, b) = (oval.first.length(), oval.second);
    let (x, y) = (away.dot(unit), away.dot(unit.perp()));
    let level = (x / a).powi(2) + (y / b).powi(2) - 1.0;
    let slope = unit * (2.0 * x / (a * a)) + unit.perp() * (2.0 * y / (b * b));
    if slope.length() == 0.0 {
        return (-a.min(b), DVec2::ZERO);
    }
    (level / slope.length(), slope.normalize())
}

/// The wall of an outline an ellipse bounds, grown by `by`, nearest a place,
/// read off the true curves and the traits through the corners the outline
/// is handed over by, rather than off the line arithmetic: inside every
/// piece of an intersection, inside some piece of a union. `near` is how
/// close two walls are taken to meet at a corner.
fn oval_wall_of(outline: &Outline, place: DVec2, by: f64, near: f64) -> Wall {
    let nothing = Wall {
        outside: f64::INFINITY,
        normal: None,
        curved: None,
    };
    let curve = |oval: Option<Oval>, inward: bool| match oval {
        Some(oval) => {
            let (outside, normal) = off_an_ellipse(oval, place);
            if inward {
                (-outside, -normal, true)
            } else {
                (outside, normal, true)
            }
        }
        None if inward => (f64::NEG_INFINITY, DVec2::ZERO, true),
        None => (f64::INFINITY, DVec2::ZERO, true),
    };
    let decided = |mut walls: Vec<(f64, DVec2, bool)>, deepest: bool| {
        walls.sort_by(|one, other| one.0.total_cmp(&other.0));
        if !deepest {
            walls.reverse();
        }
        let Some(&(outside, normal, curved)) = walls.last() else {
            return nothing;
        };
        let alone = walls.len() < 2 || (walls[walls.len() - 2].0 - outside).abs() > near;
        Wall {
            outside,
            normal: alone.then_some(normal),
            curved: alone.then_some(curved),
        }
    };
    let oval = outline.oval().expect("an outline an ellipse bounds");
    match outline {
        Outline::Ellipse { .. } => decided(vec![curve(oval.grown(by), false)], true),
        Outline::Chorded { arcs, .. } => {
            let mut walls = vec![curve(oval.grown(by), false)];
            for index in 0..arcs.len() {
                let start = oval.at(arcs[(index + 1) % arcs.len()][0].to_radians());
                let end = oval.at(arcs[index][1].to_radians());
                let normal = (start - end).perp().normalize() * -1.0;
                walls.push(((place - end).dot(normal) - by, normal, false));
            }
            decided(walls, true)
        }
        Outline::OvalSlot {
            from, to, across, ..
        } => {
            let way = (*to - *from).normalize();
            let (along, aside) = ((place - *from).dot(way), (place - *from).dot(way.perp()));
            let trimmed = (-by).max(0.0);
            let (start, end) = (trimmed, from.distance(*to) - trimmed);
            let side = aside.abs() - (across + by);
            let band = if side >= start - along && side >= along - end {
                (side, way.perp() * aside.signum(), false)
            } else {
                ((start - along).max(along - end), way, false)
            };
            let band = if *across + by <= 0.0 || end <= start {
                (f64::INFINITY, DVec2::ZERO, false)
            } else {
                band
            };
            let mut walls = vec![band];
            walls.extend(
                outline
                    .ovals()
                    .iter()
                    .map(|end| curve(end.grown(by), false)),
            );
            decided(walls, false)
        }
        Outline::OvalRing { .. } => decided(
            vec![
                curve(oval.grown(by), false),
                curve(outline.oval_hole().and_then(|hole| hole.grown(-by)), true),
            ],
            true,
        ),
        _ => unreachable!("only outlines an ellipse bounds"),
    }
}

/// Lines through the places an outline an ellipse bounds changes wall or
/// turns most — its centres, the ends of its axes, its corners — along the
/// plane, square to it and slanted, some a hair off square or off the plane.
fn lines_through_an_oval(leaf: &Leaf) -> Vec<(DVec3, DVec3)> {
    let Leaf::Prism {
        plane,
        outline,
        height,
    } = leaf
    else {
        unreachable!("only prisms are measured")
    };
    let (base, u, v) = plane.frame();
    let normal = u.cross(v);
    let mut places: Vec<DVec2> = outline
        .ovals()
        .iter()
        .flat_map(|oval| {
            [
                oval.center,
                oval.center + oval.first,
                oval.center - oval.across(),
                oval.at(1.0),
            ]
        })
        .collect();
    if let Some((contour, holes)) = outline.oval_contours() {
        places.extend(contour.corners);
        places.extend(holes.iter().flat_map(|hole| hole.corners.clone()));
    }
    let ways = [
        u,
        v,
        u + v,
        u - v * 0.3,
        normal,
        u * 0.1 + v - normal * 0.3,
        u + v * 1e-9,
        v - u * 1e-7,
        normal + u * 1e-9,
        DVec3::new(std::f64::consts::FRAC_1_PI, 5f64.sqrt() - 2.0, 1.0),
    ];
    places
        .iter()
        .flat_map(|place| {
            [0.0, height / 2.0, *height]
                .map(|level| base + u * place.x + v * place.y + normal * level)
        })
        .flat_map(|origin| ways.map(|way| (origin, way)))
        .collect()
}

#[test]
fn an_outline_an_ellipse_bounds_grown_or_shrunk_holds_along_any_line_the_places_within_it_and_ends_at_the_slant_of_its_wall()
 {
    let leaves = elliptic_prisms(60);
    let ends: usize = random_solids::on_every_core(&leaves, |leaf| {
        let mut ends = 0;
        let Leaf::Prism {
            plane,
            outline,
            height,
        } = leaf
        else {
            unreachable!("only prisms were drawn")
        };
        let (base, u, v) = plane.frame();
        let normal = u.cross(v);
        let (low, high) = leaf.bounds().expect("a prism");
        let reach = low.abs().max(high.abs()).max_element().max(1.0);
        let size = (high - low).max_element();
        let near = 1e-9 * reach;
        let measured = |point: DVec3, by: f64| {
            let offset = point - base;
            let level = offset.dot(normal);
            let slab = (height.min(0.0) - by - level).max(level - height.max(0.0) - by);
            let place = DVec2::new(offset.dot(u), offset.dot(v));
            (slab, oval_wall_of(outline, place, by, 1e3 * near))
        };
        for (origin, way) in lines_through_an_oval(leaf) {
            for by in [0.0, 0.03, -0.03, 0.2, -0.2].map(|share| share * size) {
                let stretches = leaf.along_grown(origin, way, by).expect("a prism");
                let held = |at: f64| {
                    stretches
                        .iter()
                        .any(|stretch| stretch.from.at <= at && at <= stretch.to.at)
                };
                let span = 4.0 * (reach + size) / way.length();
                for step in 0..=600 {
                    let at = span * (step as f64 / 300.0 - 1.0) + 1e-3;
                    let (slab, wall) = measured(origin + way * at, by);
                    let depth = slab.max(wall.outside);
                    if depth.abs() > 1e3 * near {
                        assert_eq!(
                            held(at),
                            depth < 0.0,
                            "{leaf} grown by {by}, at {at} along {way} from {origin}: {stretches:?}"
                        );
                    }
                }
                let step = DVec2::new(way.dot(u), way.dot(v));
                for end in stretches
                    .iter()
                    .flat_map(|stretch| [stretch.from, stretch.to])
                    .filter(|end| end.at.is_finite())
                {
                    let (slab, wall) = measured(origin + way * end.at, by);
                    let on_wall = wall.outside.abs() < 1e3 * near;
                    let (cosine, curved) = if slab.abs() < 1e3 * near && !on_wall {
                        (way.dot(normal).abs() / way.length(), false)
                    } else if let (Some(wall_normal), Some(curved)) = (wall.normal, wall.curved)
                        && on_wall
                        && slab < -1e3 * near
                    {
                        (wall_normal.dot(step).abs() / way.length(), curved)
                    } else {
                        continue;
                    };
                    if cosine < 1e-6 {
                        continue;
                    }
                    ends += 1;
                    assert!(
                        (end.cosine - cosine).abs() < 1e-6 && end.curved == curved,
                        "{leaf} grown by {by}, along {way} from {origin}: {end:?} crosses at {cosine}, curved {curved}"
                    );
                }
            }
        }
        ends
    })
    .into_iter()
    .sum();
    assert!(ends > 5_000, "{ends} ends");
}

#[test]
fn a_prism_an_ellipse_bounds_holds_along_every_line_what_its_flats_hold_but_for_their_sagitta() {
    let inscribed = (std::f64::consts::PI / random_solids::CIRCLE_STEPS as f64).cos();
    let leaves = elliptic_prisms(120);
    random_solids::on_every_core(&leaves, |leaf| {
        let Leaf::Prism {
            plane,
            outline,
            height,
        } = leaf
        else {
            unreachable!("only prisms were drawn")
        };
        let sagitta = |oval: Oval| oval.major() * (1.0 - inscribed);
        let deepest = outline.ovals().into_iter().map(sagitta).fold(0.0, f64::max);
        let most = match outline {
            Outline::OvalRing {
                center,
                first,
                second,
                ..
            } => {
                let hole = outline.oval_hole().expect("a ring's hole");
                let shrunk = hole.grown(-sagitta(hole)).expect("a hole a sagitta wide");
                Leaf::prism(
                    *plane,
                    Outline::OvalRing {
                        center: *center,
                        first: *first,
                        second: *second,
                        hole: DVec2::new(shrunk.first.length(), shrunk.second),
                    },
                    *height,
                )
            }
            _ => leaf.clone(),
        };
        let (lines, measured, reach) = lines_over(leaf);
        for (index, measure) in measured.iter().enumerate() {
            let (origin, direction) = lines.line(index);
            let least = spans_of(
                &leaf
                    .along_grown(origin, direction, -deepest)
                    .expect("a prism"),
            );
            let most = spans_of(&most.along(origin, direction).expect("a prism"));
            let short = least.without(measure).length();
            let over = measure.without(&most).length();
            assert!(
                short <= 1e-9 * reach && over <= 1e-9 * reach,
                "line {index} across {leaf}: {measure:?} between {least:?} and {most:?}"
            );
        }
    });
}

#[test]
fn the_box_a_prism_an_ellipse_bounds_spans_is_the_box_its_flats_span() {
    let inscribed = (std::f64::consts::PI / random_solids::CIRCLE_STEPS as f64).cos();
    for leaf in elliptic_prisms(200) {
        let Leaf::Prism { outline, .. } = &leaf else {
            unreachable!("only prisms were drawn")
        };
        let (low, high) = leaf.bounds().expect("a prism has a box");
        let flats = leaf.solid().expect("a leaf the flats raise");
        let (flat_low, flat_high) = flats.bounds().expect("a solid with a box");
        let reach = flat_low.abs().max(flat_high.abs()).max_element().max(1.0);
        let sagitta = outline
            .ovals()
            .iter()
            .map(|oval| oval.major() * (1.0 - inscribed))
            .fold(0.0, f64::max);
        let near = 1e-9 * reach;
        let within =
            |one: DVec3, other: DVec3, room: f64| (one - other).abs().max_element() <= room;
        assert!(
            flat_low.cmpge(low - near).all() && flat_high.cmple(high + near).all(),
            "{leaf}: the flats span {flat_low}..{flat_high} beyond {low}..{high}"
        );
        assert!(
            within(low, flat_low, sagitta + near) && within(high, flat_high, sagitta + near),
            "{leaf}: {low}..{high} against the flats' {flat_low}..{flat_high}"
        );
    }
}

#[test]
fn a_case_of_ellipses_prints_as_the_rust_that_builds_it_again() {
    let pasted = Case::new(
        Leaf::prism(
            Plane::xy(0.0),
            Outline::ellipse([0.0, 0.0], [5.0, 0.0], 3.0),
            4.0,
        ),
        vec![
            Step::cut(Leaf::prism(
                Plane::xy(-1.0),
                Outline::ellipse_from([1.0, 0.5], [0.6, 0.8], 1.5, 30.0),
                6.0,
            )),
            Step::add(Leaf::prism(
                Plane::xz(-1.0),
                Outline::chorded(
                    [2.0, 1.0],
                    [0.0, 2.0],
                    1.0,
                    &[[-60.0, 60.0], [120.0, 240.0]],
                ),
                -8.0,
            )),
            Step::cut(Leaf::prism(
                Plane::yz(2.0),
                Outline::oval_slot([0.0, 0.0], [3.0, 4.0], 1.0, 2.5),
                2.5,
            )),
            Step::add(Leaf::prism(
                Plane::xy(4.0),
                Outline::oval_ring([5.0, 3.0], [2.5, 0.0], 2.0, [1.0, 1.0]),
                2.0,
            )),
        ],
    );
    assert_eq!(
        pasted.to_string(),
        "Case::new(
    Leaf::prism(Plane::xy(0.0), Outline::ellipse([0.0, 0.0], [5.0, 0.0], 3.0), 4.0),
    vec![
        Step::cut(Leaf::prism(Plane::xy(-1.0), Outline::ellipse_from([1.0, 0.5], [0.6, 0.8], 1.5, 30.0), 6.0)),
        Step::add(Leaf::prism(Plane::xz(-1.0), Outline::chorded([2.0, 1.0], [0.0, 2.0], 1.0, &[[-60.0, 60.0], [120.0, 240.0]]), -8.0)),
        Step::cut(Leaf::prism(Plane::yz(2.0), Outline::oval_slot([0.0, 0.0], [3.0, 4.0], 1.0, 2.5), 2.5)),
        Step::add(Leaf::prism(Plane::xy(4.0), Outline::oval_ring([5.0, 3.0], [2.5, 0.0], 2.0, [1.0, 1.0]), 2.0)),
    ],
)"
    );
}

#[test]
fn an_ellipse_shrinks_into_a_circle_onto_the_plane_s_axes_and_into_round_numbers() {
    let first = DVec2::new(2.6, 1.5);
    let length = first.length();
    let ellipse = shrunk_outlines(Outline::ellipse_from([0.3, 0.7], [2.6, 1.5], 1.7, 25.0));
    for expected in [
        Outline::circle([0.3, 0.7], 1.7),
        Outline::ellipse([0.3, 0.7], [2.6, 1.5], 1.7),
        Outline::Ellipse {
            center: DVec2::new(0.3, 0.7),
            first: DVec2::X * length,
            second: 1.7,
            from: 25.0,
        },
        Outline::Ellipse {
            center: DVec2::new(0.3, 0.7),
            first,
            second: length,
            from: 25.0,
        },
        Outline::Ellipse {
            center: DVec2::new(0.0, 1.0),
            first: first.normalize() * 3.0,
            second: 2.0,
            from: 25.0,
        },
    ] {
        assert!(ellipse.contains(&expected), "{expected} among {ellipse:?}");
    }
    let turned = ellipse
        .iter()
        .filter_map(|outline| match outline {
            Outline::Ellipse { first, .. } => Some(first.to_angle().to_degrees()),
            _ => None,
        })
        .any(|degrees| (degrees - 30.0).abs() < 1e-9);
    assert!(
        turned,
        "no ellipse turned onto 30 degrees among {ellipse:?}"
    );

    let chorded = shrunk_outlines(Outline::chorded(
        [0.0, 0.0],
        [3.0, 0.0],
        2.0,
        &[[-61.0, 59.0], [118.0, 241.0]],
    ));
    for expected in [
        Outline::ellipse([0.0, 0.0], [3.0, 0.0], 2.0),
        Outline::chorded([0.0, 0.0], [3.0, 0.0], 2.0, &[[-61.0, 59.0]]),
        Outline::chorded(
            [0.0, 0.0],
            [3.0, 0.0],
            2.0,
            &[[-60.0, 60.0], [120.0, 240.0]],
        ),
    ] {
        assert!(chorded.contains(&expected), "{expected} among {chorded:?}");
    }

    let slot = shrunk_outlines(Outline::oval_slot([0.2, 0.0], [3.2, 4.0], 1.0, 2.5));
    let (way, span) = {
        let between = DVec2::new(3.2, 4.0) - DVec2::new(0.2, 0.0);
        (between.normalize(), between.length())
    };
    for expected in [
        Outline::slot([0.2, 0.0], [3.2, 4.0], 1.0),
        Outline::Ellipse {
            center: DVec2::new(0.2, 0.0),
            first: way * 2.5,
            second: 1.0,
            from: 0.0,
        },
        Outline::oval_slot([0.2, 0.0], [3.2, 4.0], 1.0, 1.0),
        Outline::OvalSlot {
            from: DVec2::new(0.2, 0.0),
            to: DVec2::new(0.2, 0.0) + DVec2::Y * span,
            across: 1.0,
            along: 2.5,
        },
        Outline::oval_slot([0.0, 0.0], [3.0, 4.0], 1.0, 3.0),
    ] {
        assert!(slot.contains(&expected), "{expected} among {slot:?}");
    }

    let ring = shrunk_outlines(Outline::oval_ring([1.0, 1.0], [0.0, 3.2], 2.0, [1.6, 1.2]));
    for expected in [
        Outline::ellipse([1.0, 1.0], [0.0, 3.2], 2.0),
        Outline::ring([1.0, 1.0], 2.0, 1.2),
        Outline::oval_ring([1.0, 1.0], [0.0, 3.2], 2.0, [1.2, 1.2]),
        Outline::oval_ring([1.0, 1.0], [0.0, 3.2], 2.0, [2.0, 1.0]),
    ] {
        assert!(ring.contains(&expected), "{expected} among {ring:?}");
    }
}

#[test]
fn an_elliptic_wall_asks_beyond_the_conics_only_against_a_cone_or_a_wall_at_a_skew_angle_near_it() {
    let oval = Leaf::prism(
        Plane::xy(0.0),
        Outline::ellipse([0.0, 0.0], [4.0, 0.0], 2.0),
        5.0,
    );
    let cone = |axis: Axis| {
        Leaf::turned(
            Plane::xz(0.0),
            axis,
            Section::bands(1.0, &[[3.0, 0.0, 2.0]]).sloping_to(&[[0.0, 1.0]]),
            360.0,
        )
    };
    let skew = |at: [f64; 3]| {
        Leaf::prism(
            Plane::tilted(at, [30.0, 0.0, 0.0]),
            Outline::circle([0.0, 0.0], 1.0),
            4.0,
        )
    };
    let told = [
        (
            "a plate under it",
            Leaf::prism(
                Plane::xy(-1.0),
                Outline::rectangle([-5.0, -5.0], [5.0, 5.0]),
                1.0,
            ),
            false,
        ),
        (
            "a block's side along its axis",
            Leaf::prism(
                Plane::xz(3.0),
                Outline::rectangle([-5.0, 1.0], [5.0, 4.0]),
                6.0,
            ),
            false,
        ),
        (
            "a slanted cut",
            Leaf::prism(
                Plane::tilted([0.0, 0.0, 3.0], [20.0, 0.0, 0.0]),
                Outline::rectangle([-6.0, -6.0], [6.0, 6.0]),
                6.0,
            ),
            false,
        ),
        (
            "a round bore parallel to it",
            Leaf::prism(Plane::xy(-1.0), Outline::circle([1.0, 0.0], 1.5), 7.0),
            false,
        ),
        (
            "an elliptic boss parallel to it",
            Leaf::prism(
                Plane::xy(5.0),
                Outline::ellipse([2.0, 1.0], [0.0, 3.0], 1.0),
                2.0,
            ),
            false,
        ),
        (
            "a round hole from the side",
            Leaf::prism(Plane::xz(-5.0), Outline::circle([0.0, 2.5], 1.0), -10.0),
            false,
        ),
        (
            "an elliptic hole from the side",
            Leaf::prism(
                Plane::yz(-5.0),
                Outline::ellipse([0.0, 2.5], [1.0, 1.0], 0.5),
                10.0,
            ),
            false,
        ),
        ("a round bore at a skew angle", skew([0.0, 0.0, 0.0]), true),
        (
            "an elliptic bore at a skew angle",
            Leaf::prism(
                Plane::tilted([0.0, 0.0, 0.0], [0.0, 45.0, 0.0]),
                Outline::ellipse([0.0, 0.0], [1.0, 0.0], 2.0),
                4.0,
            ),
            true,
        ),
        (
            "a round bore at a skew angle far off",
            skew([40.0, 40.0, 40.0]),
            false,
        ),
        ("a cone about its axis", cone(Axis::second(0.0)), true),
        ("a cone beside its axis", cone(Axis::second(3.0)), true),
        ("a cone far off", cone(Axis::second(30.0)), false),
        (
            "a cylinder turned about its axis",
            Leaf::turned(
                Plane::xz(0.0),
                Axis::second(0.0),
                Section::bands(1.0, &[[3.0, 0.0, 2.0]]),
                360.0,
            ),
            false,
        ),
    ];
    for (what, tool, asks) in told {
        for case in [
            Case::new(oval.clone(), vec![Step::cut(tool.clone())]),
            Case::new(tool.clone(), vec![Step::add(oval.clone())]),
        ] {
            assert_eq!(case.asks_beyond_the_conics(), asks, "{what}:\n{case}");
        }
    }
    let ring = Leaf::prism(
        Plane::xy(0.0),
        Outline::oval_ring([0.0, 0.0], [4.0, 0.0], 3.0, [2.0, 2.0]),
        5.0,
    );
    let cylinder = Leaf::turned(
        Plane::xz(0.0),
        Axis::second(0.0),
        Section::bands(1.0, &[[3.0, 0.0, 1.0]]),
        360.0,
    );
    assert!(
        !Case::new(ring, vec![Step::add(cylinder)]).asks_beyond_the_conics(),
        "a ring's round hole is a cylinder to the rule"
    );
}

#[test]
fn the_slices_of_an_elliptic_leaf_cut_by_a_plane_add_up_to_the_closed_forms() {
    use std::f64::consts::PI;
    let tan30 = 30f64.to_radians().tan();
    let slanted = DVec3::new(0.0, -tan30, 1.0);
    let shaft = |outline: Outline| Leaf::prism(Plane::xy(0.0), outline, 30.0);
    let kept = [
        (
            "an elliptic shaft cut at a slant across its walls",
            shaft(Outline::ellipse([0.0, 0.0], [5.0, 0.0], 3.0))
                .kept_by_a_half_space(slanted, 15.0),
            225.0 * PI,
        ),
        (
            "an elliptic ring cut at a slant across its walls",
            shaft(Outline::oval_ring([0.0, 0.0], [5.0, 0.0], 3.0, [2.0, 1.0]))
                .kept_by_a_half_space(slanted, 15.0),
            195.0 * PI,
        ),
        (
            "an elliptic shaft turned, cut by a plane holding neither axis",
            shaft(Outline::ellipse_from([1.0, -2.0], [2.4, 1.8], 2.0, 70.0))
                .kept_by_a_half_space(DVec3::new(0.3, -0.5, 1.0), 15.0),
            6.0 * PI * (15.0 - 0.3 * 1.0 - 0.5 * 2.0),
        ),
        (
            "an elliptic ungula through its major axis",
            shaft(Outline::ellipse([0.0, 0.0], [5.0, 0.0], 3.0))
                .kept_by_a_half_space(DVec3::new(0.0, -2.0, 1.0), 0.0),
            60.0,
        ),
        (
            "an elliptic ungula through its minor axis",
            shaft(Outline::ellipse([0.0, 0.0], [0.0, 5.0], 3.0))
                .kept_by_a_half_space(DVec3::new(0.0, -2.0, 1.0), 0.0),
            100.0,
        ),
        (
            "an elliptic shaft cut square to its axis",
            shaft(Outline::ellipse([0.0, 0.0], [5.0, 0.0], 3.0))
                .kept_by_a_half_space(DVec3::Z, 12.0),
            180.0 * PI,
        ),
        (
            "an elliptic ring cut along its axis through its centre",
            shaft(Outline::oval_ring([0.0, 0.0], [5.0, 0.0], 3.0, [2.0, 1.0]))
                .kept_by_a_half_space(DVec3::X, 0.0),
            195.0 * PI,
        ),
    ];
    for (what, found, expected) in kept {
        let found = found.expect("an elliptic leaf");
        assert!(
            (found - expected).abs() <= 1e-12 * expected,
            "{what}: {found} against {expected}"
        );
    }
    for normal in [
        slanted,
        DVec3::new(0.3, -0.5, 1.0),
        DVec3::new(1.0, 0.2, 0.05),
    ] {
        let round = shaft(Outline::circle([1.0, 0.0], 4.0)).kept_by_a_half_space(normal, 10.0);
        let oval = shaft(Outline::ellipse_from([1.0, 0.0], [0.0, 4.0], 4.0, 15.0))
            .kept_by_a_half_space(normal, 10.0);
        let (round, oval) = (round.expect("a circle"), oval.expect("an ellipse"));
        assert!(
            (round - oval).abs() <= 1e-12 * round,
            "{normal}: a circle keeps {round}, an ellipse of equal half-axes {oval}"
        );
    }
}

/// The three draws of ellipses (#571), each by the name a campaign gives it.
/// A draw of cases, one to a seed.
type Draw = fn(u64) -> Case;

const ELLIPSE_DRAWS: [(&str, Draw); 3] = [
    ("ellipses", Case::drawn_ellipses),
    ("ellipses crossed", Case::drawn_ellipses_crossed),
    ("ellipses tilted", Case::drawn_ellipses_tilted),
];

/// Every draw a seed names a case of.
const EVERY_DRAW: [Draw; 12] = [
    Case::drawn,
    Case::drawn_square,
    Case::drawn_profiles,
    Case::drawn_turned,
    Case::drawn_turned_off_the_lattice,
    Case::drawn_slanted,
    Case::drawn_slanted_off_the_lattice,
    Case::drawn_tilted,
    Case::drawn_tilted_off_the_lattice,
    Case::drawn_ellipses,
    Case::drawn_ellipses_crossed,
    Case::drawn_ellipses_tilted,
];

#[test]
fn the_same_seed_draws_the_same_ellipse_case_and_none_another_draw_gives_it() {
    for (index, (name, draw)) in ELLIPSE_DRAWS.iter().enumerate() {
        for seed in 0..200 {
            assert_eq!(draw(seed), draw(seed), "{name}, seed {seed}");
        }
        let distinct: std::collections::BTreeSet<String> =
            (0..200).map(|seed| draw(seed).to_string()).collect();
        assert!(
            distinct.len() > 190,
            "{name}: {} distinct cases",
            distinct.len()
        );
        for seed in 0..200 {
            let case = draw(seed);
            for (other, drawn) in EVERY_DRAW.iter().enumerate() {
                if other != 9 + index {
                    assert_ne!(drawn(seed), case, "{name}, seed {seed}, draw {other}");
                }
            }
        }
    }
}

/// Whether a leaf is a prism of an outline an ellipse bounds.
fn elliptic(leaf: &Leaf) -> bool {
    outline_of(leaf).is_some_and(|outline| outline.oval().is_some())
}

#[test]
fn every_seed_draws_a_case_of_solid_prisms_holding_an_ellipse_and_now_and_then_a_cone_beside_it() {
    for (name, draw) in ELLIPSE_DRAWS {
        for seed in 0..3000 {
            let case = draw(seed);
            assert!(case.leaves().any(elliptic), "{name}, seed {seed}:\n{case}");
            for leaf in case.leaves() {
                let turned = name == "ellipses tilted"
                    && leaf
                        .as_turned()
                        .is_some_and(|turned| turned.section.slopes());
                assert!(
                    (matches!(leaf, Leaf::Prism { .. }) || turned) && leaf.is_solid(),
                    "{name}, seed {seed}: {leaf}\n{case}"
                );
            }
            if name != "ellipses tilted" {
                assert!(
                    case.leaves()
                        .all(|leaf| !matches!(leaf.plane(), Plane::Tilted { .. })),
                    "{name}, seed {seed}:\n{case}"
                );
            }
            if name == "ellipses" {
                let normal = case.start.plane().normal();
                assert!(
                    case.leaves()
                        .all(|leaf| leaf.plane().normal().cross(normal).length() == 0.0),
                    "{name}, seed {seed}: not all raised along one normal\n{case}"
                );
            }
        }
    }
}

#[test]
fn every_seed_draws_an_ellipse_case_though_no_leaf_before_it_has_room_for_one() {
    for (name, draw) in ELLIPSE_DRAWS {
        for seed in 0..20_000 {
            let drawn = std::panic::catch_unwind(|| draw(seed));
            assert!(drawn.is_ok(), "{name}: seed {seed} draws no case");
        }
    }
}

/// How far a case reaches, to read its hairs against.
fn reach_of(case: &Case) -> f64 {
    case.leaves()
        .filter_map(Leaf::bounds)
        .map(|(low, high)| low.abs().max(high.abs()).max_element())
        .fold(1.0, f64::max)
}

/// The ellipse of a prism of the planes of the origin and the world axis it
/// runs along, or the circle of a round one as an ellipse.
fn round_wall_of(leaf: &Leaf) -> Option<(Oval, usize, Plane)> {
    let Leaf::Prism { plane, outline, .. } = leaf else {
        return None;
    };
    if matches!(plane, Plane::Tilted { .. }) {
        return None;
    }
    let oval = match outline {
        Outline::Circle { center, radius, .. }
        | Outline::Ring {
            center,
            outer: radius,
            ..
        } => Oval::new(*center, DVec2::X * *radius, *radius),
        _ => outline.oval()?,
    };
    Some((oval, plane.normal().abs().max_position(), *plane))
}

/// The ends of the axes of an ellipse.
fn vertices(oval: Oval) -> [DVec2; 4] {
    let (major, minor) = (oval.major_direction(), oval.major_direction().perp());
    [
        oval.center + major * oval.major(),
        oval.center - major * oval.major(),
        oval.center + minor * oval.minor(),
        oval.center - minor * oval.minor(),
    ]
}

/// The relations a tool of the draw of ellipses on one normal stands in to
/// an ellipse drawn before it, read off the two leaves, within `hair`.
fn related_to_an_ellipse(earlier: Oval, later: &Outline, hair: f64) -> Vec<&'static str> {
    let mut found = Vec::new();
    let close = |one: f64, other: f64| (one - other).abs() <= hair;
    let same = |one: DVec2, other: DVec2| one.distance(other) <= hair;
    if let Some(oval) = later.oval() {
        if oval == earlier {
            found.push("the very same ellipse");
        } else if same(oval.center, earlier.center)
            && same(oval.first, earlier.first)
            && close(oval.second, earlier.second)
        {
            found.push("the same ellipse a hair off");
        }
        if !same(oval.center, earlier.center)
            && vertices(earlier)
                .iter()
                .any(|vertex| oval.level(*vertex).abs() <= hair / oval.minor())
        {
            found.push("two ellipses touching at a vertex");
        }
        if same(oval.center, earlier.center)
            && oval != earlier
            && (oval.first.length() - oval.second).abs() <= 1e-3 * oval.first.length()
        {
            found.push("an ellipse a hair from a circle about the same centre");
        }
    }
    match later {
        Outline::Rectangle { low, high } => {
            let reach = DVec2::new(earlier.support(DVec2::X), earlier.support(DVec2::Y));
            let touches = (0..2).any(|axis| {
                [low[axis], high[axis]].iter().any(|side| {
                    close(*side, earlier.center[axis] + reach[axis])
                        || close(*side, earlier.center[axis] - reach[axis])
                })
            });
            if touches {
                found.push("a block's side at the ellipse's reach");
            }
        }
        Outline::Circle { center, radius, .. } => {
            if same(*center, earlier.center) && close(*radius, earlier.minor()) {
                found.push("a circle across the minor axis");
            }
            let at_a_vertex = vertices(earlier).iter().any(|vertex| {
                let out = (*vertex - earlier.center).normalize();
                close(center.distance(*vertex), *radius)
                    && (*center - *vertex).perp_dot(out).abs() <= hair
                    && !same(*center, earlier.center)
            });
            if at_a_vertex {
                let inside = earlier.level(*center) < 0.0;
                found.push(if inside {
                    "a circle inside at a vertex"
                } else {
                    "a circle outside at a vertex"
                });
            }
        }
        Outline::Slot { radius, .. } if close(*radius, earlier.minor()) => {
            found.push("a slot hugging the ellipse");
        }
        Outline::OvalRing { .. }
            if later.oval_hole().is_some_and(|hole| {
                same(hole.center, earlier.center)
                    && same(hole.first, earlier.first)
                    && close(hole.second, earlier.second)
            }) =>
        {
            found.push("a ring about the very ellipse");
        }
        _ => {}
    }
    found
}

/// What the outline of a leaf an ellipse bounds is, and how its axes lie.
fn kinds_of_ellipse(outline: &Outline) -> Vec<&'static str> {
    let mut found = vec![match outline {
        Outline::Ellipse { .. } => "a whole ellipse",
        Outline::Chorded { arcs, .. } if arcs.len() == 1 => "an ellipse cut by a trait",
        Outline::Chorded { .. } => "an ellipse cut by two traits",
        Outline::OvalSlot { .. } => "a slot with elliptic ends",
        Outline::OvalRing { hole, .. } if hole.x == hole.y => "an elliptic ring bored round",
        Outline::OvalRing { .. } => "an elliptic ring bored elliptic",
        _ => return Vec::new(),
    }];
    for oval in outline.ovals() {
        let direction = oval.first.normalize();
        found.push(if direction.x == 0.0 || direction.y == 0.0 {
            "axes on the plane's"
        } else {
            "axes turned"
        });
        if oval.first.length() < oval.second {
            found.push("the first semi-axis the minor");
        }
        if oval.first.length() == oval.second {
            found.push("half-axes equal");
        } else if (oval.first.length() - oval.second).abs() <= 1e-3 * oval.first.length() {
            found.push("half-axes a hair apart");
        }
    }
    if let Outline::Ellipse { from, .. } = outline
        && *from != 0.0
    {
        found.push("a corner off the first semi-axis");
    }
    found
}

/// How two walls square to each other stand across the direction both run
/// square to: their half-widths and their centres along it, read within
/// `hair`.
fn crossing(one: &Leaf, other: &Leaf, hair: f64) -> Option<&'static str> {
    let (first, first_axis, first_plane) = round_wall_of(one)?;
    let (second, second_axis, second_plane) = round_wall_of(other)?;
    if first_axis == second_axis {
        return None;
    }
    let shared = 3 - first_axis - second_axis;
    let across = |oval: Oval, plane: Plane| {
        let (_, u, v) = plane.frame();
        let direction = DVec2::new(DVec3::AXES[shared].dot(u), DVec3::AXES[shared].dot(v));
        (oval.support(direction), plane.to_world(oval.center)[shared])
    };
    let ((width, at), (other_width, other_at)) =
        (across(first, first_plane), across(second, second_plane));
    let apart = (at - other_at).abs();
    Some(if (width - other_width).abs() <= hair && apart <= hair {
        "a node: equal half-widths, the axes meeting"
    } else if (width - other_width).abs() > hair
        && (apart - (width - other_width).abs()).abs() <= hair
    {
        "a touch inside"
    } else if (apart - (width + other_width)).abs() <= hair {
        "a touch outside"
    } else if (width - other_width).abs() <= hair {
        "equal half-widths, the axes apart"
    } else {
        "a crossing"
    })
}

/// The sine of the least lean of a leaning plane off square to, or along,
/// the axis of an elliptic wall: nothing when it stands square or along
/// but for the rounding of the angles it is turned by.
fn lean_off(plane: Plane, axis: usize) -> Option<f64> {
    let normal = plane.normal().normalize();
    let along = normal[axis].abs();
    let across = normal.cross(DVec3::AXES[axis]).length();
    let least = along.min(across);
    (least > 1e-12).then_some(least)
}

/// What a case of the draws of ellipses holds, kind by kind, each kind
/// counted once a case.
fn kinds_in(case: &Case) -> std::collections::BTreeSet<&'static str> {
    let mut found = std::collections::BTreeSet::new();
    let hair = 2e-4 * reach_of(case);
    let leaves: Vec<&Leaf> = case.leaves().collect();
    for (index, leaf) in leaves.iter().enumerate() {
        if let Some(outline) = outline_of(leaf) {
            found.extend(kinds_of_ellipse(outline));
        }
        for earlier in &leaves[..index] {
            if let (Some((oval, axis, _)), Some(outline)) =
                (round_wall_of(earlier), outline_of(leaf))
                && elliptic(earlier)
                && round_wall_of(leaf).is_none_or(|(_, other, _)| other == axis)
            {
                found.extend(related_to_an_ellipse(oval, outline, hair));
            }
            if let Some(kind) = crossing(earlier, leaf, hair)
                && (elliptic(earlier) || elliptic(leaf))
            {
                found.insert(kind);
                found.insert(match (elliptic(earlier), elliptic(leaf)) {
                    (true, true) => "an elliptic wall through an elliptic one",
                    (true, false) => "a round wall through an elliptic one",
                    _ => "an elliptic wall through a round one",
                });
            }
            if let (Some((_, axis, _)), Plane::Tilted { .. }) =
                (round_wall_of(earlier), leaf.plane())
                && elliptic(earlier)
                && let Some(lean) = lean_off(*leaf.plane(), axis)
            {
                found.insert(if lean <= 1e-8 {
                    "a lean in decision 11's band"
                } else if lean <= 1e-3 {
                    "a lean in the band the kernel warns of"
                } else {
                    "a plane at a slant through an elliptic wall"
                });
                if elliptic(leaf) || matches!(outline_of(leaf), Some(Outline::Circle { .. })) {
                    found.insert("a bore at a slant through an elliptic wall");
                }
            }
            if let (Some((oval, axis, plane)), Leaf::Prism { plane: cut, .. }) =
                (round_wall_of(earlier), leaf)
                && elliptic(earlier)
                && !matches!(cut, Plane::Tilted { .. })
                && cut.normal()[axis] == 0.0
                && matches!(outline_of(leaf), Some(Outline::Rectangle { .. }))
            {
                let across = cut.normal().abs().max_position();
                let (_, u, v) = plane.frame();
                let direction = DVec2::new(DVec3::AXES[across].dot(u), DVec3::AXES[across].dot(v));
                let center = plane.to_world(oval.center)[across];
                let level = cut.frame().0[across];
                if ((level - center).abs() - oval.support(direction)).abs() <= hair {
                    found.insert("a plane along an elliptic wall at its reach");
                }
            }
        }
        if let Leaf::Prism {
            plane: Plane::Tilted { turn, .. },
            outline,
            ..
        } = leaf
            && outline.oval().is_some()
            && leaves[..index].iter().any(|earlier| {
                matches!(earlier, Leaf::Prism { plane: Plane::Tilted { turn: face, .. }, outline: Outline::Rectangle { .. }, .. } if face == turn)
            })
        {
            found.insert("an elliptic hole or boss square to a slanted face");
        }
    }
    if case.asks_beyond_the_conics() {
        found.insert("asking beyond the conics");
    }
    if case.leaves().any(|leaf| leaf.as_turned().is_some()) {
        found.insert("a cone beside an elliptic wall");
    }
    found
}

/// What a draw of ellipses is asked for: its name, each kind it draws
/// with the least share of its cases that hold it, and the largest share
/// that may ask beyond the conics.
type Wanted = (&'static str, &'static [(&'static str, f64)], f64);

#[test]
fn the_ellipse_generators_draw_every_kind_they_keep() {
    let wanted: [Wanted; 3] = [
        (
            "ellipses",
            &[
                ("a whole ellipse", 0.6),
                ("an ellipse cut by a trait", 0.08),
                ("an ellipse cut by two traits", 0.03),
                ("a slot with elliptic ends", 0.1),
                ("an elliptic ring bored round", 0.04),
                ("an elliptic ring bored elliptic", 0.05),
                ("axes on the plane's", 0.4),
                ("axes turned", 0.3),
                ("the first semi-axis the minor", 0.3),
                ("half-axes equal", 0.1),
                ("half-axes a hair apart", 0.08),
                ("a corner off the first semi-axis", 0.1),
                ("the very same ellipse", 0.03),
                ("the same ellipse a hair off", 0.06),
                ("a block's side at the ellipse's reach", 0.07),
                ("a circle across the minor axis", 0.05),
                ("a circle inside at a vertex", 0.025),
                ("a circle outside at a vertex", 0.04),
                ("two ellipses touching at a vertex", 0.06),
                (
                    "an ellipse a hair from a circle about the same centre",
                    0.05,
                ),
                ("a slot hugging the ellipse", 0.015),
                ("a ring about the very ellipse", 0.01),
            ],
            0.0,
        ),
        (
            "ellipses crossed",
            &[
                ("a node: equal half-widths, the axes meeting", 0.3),
                ("a touch inside", 0.12),
                ("a touch outside", 0.08),
                ("equal half-widths, the axes apart", 0.15),
                ("a crossing", 0.2),
                ("a round wall through an elliptic one", 0.25),
                ("an elliptic wall through a round one", 0.25),
                ("an elliptic wall through an elliptic one", 0.25),
                ("half-axes a hair apart", 0.04),
            ],
            0.0,
        ),
        (
            "ellipses tilted",
            &[
                ("a plane at a slant through an elliptic wall", 0.3),
                ("a lean in decision 11's band", 0.08),
                ("a lean in the band the kernel warns of", 0.08),
                ("an elliptic hole or boss square to a slanted face", 0.025),
                ("a plane along an elliptic wall at its reach", 0.06),
                ("a bore at a slant through an elliptic wall", 0.07),
                ("a block's side at the ellipse's reach", 0.06),
                ("a cone beside an elliptic wall", 0.01),
            ],
            0.2,
        ),
    ];
    for ((name, draw), (named, kinds, asking)) in ELLIPSE_DRAWS.iter().zip(wanted) {
        assert_eq!(*name, named);
        let cases: Vec<Case> = (0..1000).map(draw).collect();
        let found: Vec<_> = cases.iter().map(kinds_in).collect();
        let share = |kind: &str| {
            found.iter().filter(|kinds| kinds.contains(kind)).count() as f64 / cases.len() as f64
        };
        for (kind, least) in kinds {
            assert!(
                share(kind) >= *least,
                "{name}: {kind} in {} of the cases, under {least}",
                share(kind)
            );
        }
        assert!(
            share("asking beyond the conics") <= asking,
            "{name}: {} of the cases ask beyond the conics",
            share("asking beyond the conics")
        );
    }
}

#[test]
fn a_case_of_any_draw_of_ellipses_prints_as_numbers_rust_reads_back() {
    for (name, draw) in ELLIPSE_DRAWS {
        for seed in 0..300 {
            let printed = draw(seed).to_string();
            assert!(
                !printed.contains("NaN") && !printed.contains("inf"),
                "{name}, seed {seed} prints a number Rust cannot read back: {printed}"
            );
        }
    }
}

#[test]
fn shrinking_any_drawn_ellipse_case_comes_to_an_end() {
    for (name, draw) in ELLIPSE_DRAWS {
        for seed in 0..100 {
            let mut rounds = 0;
            let shrunk = shrink(
                draw(seed),
                Case::smaller,
                |_| true,
                || {
                    rounds += 1;
                    rounds < 100_000
                },
            );
            assert!(rounds < 100_000, "{name}, seed {seed} was still shrinking");
            assert!(
                shrunk.smaller().is_empty(),
                "{name}, seed {seed} stopped short: {shrunk}"
            );
        }
    }
}

#[test]
fn shrinking_a_case_while_it_still_holds_an_ellipse_or_a_turned_one_comes_to_an_end() {
    let holding: &dyn Fn(&Case) -> bool = &|case: &Case| case.leaves().any(elliptic);
    let turned: &dyn Fn(&Case) -> bool = &|case: &Case| {
        case.leaves().any(|leaf| {
            outline_of(leaf)
                .and_then(Outline::oval)
                .is_some_and(|oval| oval.first.x != 0.0 && oval.first.y != 0.0)
        })
    };
    let lone = |outline: Outline| Case::new(Leaf::prism(Plane::xy(0.0), outline, 4.0), vec![]);
    let cases: Vec<Case> = ELLIPSE_DRAWS
        .iter()
        .flat_map(|(_, draw)| (0..40).map(draw))
        .chain([
            lone(Outline::ellipse(
                [30.0, 35.0],
                [10.396639796596714, 14.076933470747331],
                25.000005,
            )),
            lone(Outline::ellipse([1.0, 2.0], [2.6, 1.5], 1.7)),
        ])
        .collect();
    for (index, case) in cases.iter().enumerate() {
        for (what, keeps) in [("an ellipse", holding), ("a turned ellipse", turned)] {
            if !keeps(case) {
                continue;
            }
            let mut rounds = 0;
            let shrunk = shrink(case.clone(), Case::smaller, keeps, || {
                rounds += 1;
                rounds < 100_000
            });
            assert!(
                rounds < 100_000,
                "case {index}, kept holding {what}, was still shrinking: {shrunk}"
            );
            assert!(
                shrunk.smaller().iter().all(|smaller| !keeps(smaller)),
                "case {index}, kept holding {what}, stopped short: {shrunk}"
            );
        }
    }
}

#[test]
fn ellipse_cases_the_flats_answer_by_their_own_rules_keep_every_rule_held_to_the_arithmetic() {
    for (name, draw) in ELLIPSE_DRAWS {
        let seeds: Vec<u64> = (0..20).collect();
        let weighed = random_solids::on_every_core(&seeds, |seed| {
            let case = draw(*seed);
            random_solids::check(&case).ok()?;
            let measured = random_solids::held_to_arithmetic(&case, &Flats::for_case(&case));
            Some((measured, case))
        });
        let mut held = 0;
        for (seed, (measured, case)) in seeds
            .iter()
            .zip(weighed)
            .filter_map(|(seed, one)| Some((seed, one?)))
        {
            held += 1;
            assert!(
                measured.is_ok(),
                "{name}, seed {seed}: {measured:?}\n{case}"
            );
        }
        assert!(held > 8, "{name}: {held} cases held");
    }
}

/// Every case of `draw` among `seeds` keeps every rule on the exact kernel,
/// declined only where it asks beyond the conics.
fn ellipse_cases_keep_every_rule_on_the_exact_kernel(
    name: &str,
    draw: fn(u64) -> Case,
    seeds: std::ops::Range<u64>,
) {
    let seeds: Vec<u64> = seeds.collect();
    let weighed = random_solids::on_every_core(&seeds, |seed| {
        let case = draw(*seed);
        (random_solids::held_to_arithmetic(&case, &Exact), case)
    });
    for (seed, (measured, case)) in seeds.iter().zip(weighed) {
        assert!(
            measured.is_ok(),
            "{name}, seed {seed}: {measured:?}\n{case}"
        );
    }
}

#[test]
fn random_ellipse_cases_keep_every_rule_on_the_exact_kernel() {
    for (name, draw) in ELLIPSE_DRAWS {
        ellipse_cases_keep_every_rule_on_the_exact_kernel(name, draw, 0..16);
    }
}

#[test]
fn ellipse_cases_the_exact_kernel_keeps_are_kept_through_the_application_s_body_and_stay_exact() {
    for (name, draw) in ELLIPSE_DRAWS {
        let seeds: Vec<u64> = (0..12).collect();
        let weighed = random_solids::on_every_core(&seeds, |seed| {
            let case = draw(*seed);
            let kept = random_solids::held_to_arithmetic(&case, &Exact).ok()?;
            if kept.declined > 0 {
                return None;
            }
            let leaves: Vec<cao_solid::Body> = case
                .leaves()
                .map(|leaf| Application.raised(leaf))
                .collect::<Result<_, _>>()
                .ok()?;
            let mut exact = leaves.iter().all(cao_solid::Body::is_exact);
            let mut body = leaves[0].clone();
            for (step, tool) in case.steps.iter().zip(&leaves[1..]) {
                match Application.combined(&body, tool, step.mode) {
                    Ok(next) => body = next,
                    Err(_) => break,
                }
                exact &= body.is_exact();
            }
            Some((
                random_solids::held_to_arithmetic(&case, &Application),
                exact,
                case,
            ))
        });
        let mut held = 0;
        for (seed, (measured, exact, case)) in seeds
            .iter()
            .zip(weighed)
            .filter_map(|(seed, one)| Some((seed, one?)))
        {
            held += 1;
            assert!(
                measured.is_ok(),
                "{name}, seed {seed}: {measured:?}\n{case}"
            );
            assert!(exact, "{name}, seed {seed} left the exact kernel:\n{case}");
        }
        assert!(held > 4, "{name}: {held} cases held");
    }
}

#[test]
fn every_elliptic_profile_raised_by_the_exact_kernel_alone_keeps_every_rule_and_encloses_its_area_times_its_height()
 {
    let leaves: Vec<Leaf> = (0..120)
        .flat_map(|seed| {
            ELLIPSE_DRAWS
                .iter()
                .flat_map(|(_, draw)| draw(seed).leaves().cloned().collect::<Vec<_>>())
                .collect::<Vec<_>>()
        })
        .filter(elliptic)
        .chain(elliptic_prisms(60))
        .collect();
    assert!(leaves.len() > 200, "{} leaves", leaves.len());
    let weighed = random_solids::on_every_core(&leaves, |leaf| {
        let alone = Case::new(leaf.clone(), vec![]);
        let held = random_solids::held_to_arithmetic(&alone, &Exact);
        (held, Exact.raised(leaf).map(|body| body.volume()))
    });
    for (leaf, (held, volume)) in leaves.iter().zip(weighed) {
        assert!(held.is_ok(), "{leaf}: {held:?}");
        let Leaf::Prism {
            outline, height, ..
        } = leaf
        else {
            unreachable!("only prisms were kept")
        };
        let promised = area(outline) * height.abs();
        let volume = volume.unwrap_or_else(|declined| panic!("{leaf}: {declined:?}"));
        assert!(
            (volume - promised).abs() <= 1e-9 * promised,
            "{leaf}: {volume} against {promised}"
        );
    }
}

#[test]
fn an_elliptic_leaf_names_its_faces_as_the_flats_name_them() {
    let leaves: Vec<Leaf> = (0..60)
        .flat_map(|seed| {
            ELLIPSE_DRAWS
                .iter()
                .flat_map(|(_, draw)| draw(seed).leaves().cloned().collect::<Vec<_>>())
                .collect::<Vec<_>>()
        })
        .filter(elliptic)
        .chain(elliptic_prisms(40))
        .collect();
    for leaf in &leaves {
        let exact = Application
            .raised(leaf)
            .unwrap_or_else(|declined| panic!("{leaf}: {declined:?}"));
        assert!(exact.is_exact(), "{leaf} raised on the flats");
        let flats = leaf.solid().expect("a leaf the flats raise");
        assert_eq!(exact.faces_end(), flats.faces_end(), "{leaf}");
        for face in 0..flats.faces_end() {
            assert_eq!(
                exact.has_face(face),
                flats.has_face(face),
                "{leaf}: face {face}"
            );
        }
    }
}

#[test]
fn a_lone_elliptic_leaf_cut_by_a_slanted_half_space_encloses_the_volume_of_its_slices() {
    let leaves: Vec<Leaf> = elliptic_prisms(400)
        .into_iter()
        .filter(|leaf| leaf.kept_by_a_half_space(DVec3::Z, 0.0).is_some())
        .filter(|leaf| !matches!(leaf.plane(), Plane::Tilted { .. }))
        .take(36)
        .collect();
    assert_eq!(leaves.len(), 36);
    let cut: Vec<(Case, DVec3, f64)> = leaves
        .iter()
        .enumerate()
        .map(|(index, leaf)| {
            let (low, high) = leaf.bounds().expect("a leaf with a box");
            let middle = (low + high) / 2.0;
            let holds = index % 3;
            let degrees = [20.0, 35.0, 50.0, 65.0][index / 3 % 4];
            let plane = [degrees, -degrees]
                .into_iter()
                .map(|degrees| Plane::leaning(holds, degrees, middle))
                .find(|plane| random_solids::clear_of_the_lines(*plane))
                .expect("a leaning clear of the lines");
            let normal = plane.normal();
            let reach = (high - low).length() + 1.0;
            let tool = Leaf::prism(
                plane,
                Outline::rectangle([-reach, -reach], [reach, reach]),
                reach,
            );
            (
                Case::new(leaf.clone(), vec![Step::cut(tool)]),
                normal,
                normal.dot(middle),
            )
        })
        .collect();
    let weighed = random_solids::on_every_core(&cut, |(case, _, _)| {
        let held = random_solids::held_to_arithmetic(case, &Exact);
        let volume = Exact
            .raised(&case.start)
            .and_then(|leaf| {
                let tool = Exact.raised(&case.steps[0].tool)?;
                Exact.combined(&leaf, &tool, Mode::Cut)
            })
            .map(|body| body.volume());
        (held, volume)
    });
    for ((case, normal, offset), (held, volume)) in cut.iter().zip(weighed) {
        let measured = held.unwrap_or_else(|flaw| panic!("{flaw:?}\n{case}"));
        assert_eq!(measured.declined, 0, "declined:\n{case}");
        let volume = volume.unwrap_or_else(|declined| panic!("{declined:?}\n{case}"));
        let sliced = case
            .start
            .kept_by_a_half_space(*normal, *offset)
            .expect("an elliptic leaf");
        assert!(
            (volume - sliced).abs() <= 1e-9 * sliced,
            "{volume} against {sliced} by slices\n{case}"
        );
    }
}

/// The campaign itself, compiled only when asked for — `--features
/// campaigns` — so that work on any other issue never pays for it.
#[cfg(feature = "campaigns")]
mod campaign {
    use std::sync::Arc;
    use std::sync::atomic::Ordering;

    use cao_solid::soundness::Check;
    use random_solids::campaigning::{
        Answering, DECLINED, GRAZING, HELD, UNSTABLE, campaign_over, shrunk_one_by_one,
        tally_conics,
    };

    use super::*;

    fn tallied(measured: random_solids::Measured) {
        HELD.fetch_add(measured.held, Ordering::Relaxed);
        GRAZING.fetch_add(measured.grazing, Ordering::Relaxed);
        DECLINED.fetch_add(measured.declined, Ordering::Relaxed);
        UNSTABLE.fetch_add(measured.unstable, Ordering::Relaxed);
    }

    fn exactly(case: &Case) -> Result<(), Flaw> {
        tallied(random_solids::held_to_arithmetic(case, &Exact)?);
        Ok(())
    }

    /// The case a seed stands for, with the seed written where a campaign that
    /// ends the program — a stack blown by a kernel — still leaves it to be read.
    fn drawn(seed: u64) -> Case {
        eprint!("\rseed {seed} ");
        Case::drawn_square(seed)
    }

    /// The same, among the profiles.
    fn drawn_profile(seed: u64) -> Case {
        eprint!("\rseed {seed} ");
        Case::drawn_profiles(seed)
    }

    #[test]
    #[ignore = "a campaign, run by hand: see the head of this file"]
    fn a_campaign_of_random_square_solids_on_the_exact_kernel_keeps_every_rule() {
        campaign_over("square solids", Answering::Exactly, drawn, exactly);
    }

    #[test]
    #[ignore = "a campaign, run by hand: see the head of this file"]
    fn a_campaign_of_random_profiles_on_the_exact_kernel_keeps_every_rule() {
        campaign_over("profiles", Answering::Exactly, drawn_profile, exactly);
    }

    #[test]
    #[ignore = "a campaign, run by hand: see the head of this file"]
    fn a_campaign_of_random_profiles_through_the_application_s_body_keeps_every_rule() {
        campaign_over(
            "profiles",
            Answering::ThroughTheApplication,
            drawn_profile,
            through_the_application,
        );
    }

    fn through_the_application(case: &Case) -> Result<(), Flaw> {
        tallied(random_solids::held_to_arithmetic(case, &Application)?);
        Ok(())
    }

    /// A case of ellipses held on the exact kernel, and the conics its
    /// planes meet its walls along tallied (#571).
    fn ellipses_exactly(case: &Case) -> Result<(), Flaw> {
        exactly(case)?;
        tally_conics(case);
        Ok(())
    }

    /// The same, through the application's body.
    fn ellipses_through_the_application(case: &Case) -> Result<(), Flaw> {
        through_the_application(case)?;
        tally_conics(case);
        Ok(())
    }

    /// The same, among the draws of ellipses (#571).
    fn drawn_ellipse(seed: u64) -> Case {
        eprint!("\rseed {seed} ");
        Case::drawn_ellipses(seed)
    }

    fn drawn_ellipse_crossed(seed: u64) -> Case {
        eprint!("\rseed {seed} ");
        Case::drawn_ellipses_crossed(seed)
    }

    fn drawn_ellipse_tilted(seed: u64) -> Case {
        eprint!("\rseed {seed} ");
        Case::drawn_ellipses_tilted(seed)
    }

    #[test]
    #[ignore = "a campaign, run by hand: see the head of this file"]
    fn a_campaign_of_random_ellipses_on_the_exact_kernel_keeps_every_rule() {
        campaign_over(
            "ellipses",
            Answering::Exactly,
            drawn_ellipse,
            ellipses_exactly,
        );
    }

    #[test]
    #[ignore = "a campaign, run by hand: see the head of this file"]
    fn a_campaign_of_random_ellipses_through_the_application_s_body_keeps_every_rule() {
        campaign_over(
            "ellipses",
            Answering::ThroughTheApplication,
            drawn_ellipse,
            ellipses_through_the_application,
        );
    }

    #[test]
    #[ignore = "a campaign, run by hand: see the head of this file"]
    fn a_campaign_of_random_ellipses_crossed_on_the_exact_kernel_keeps_every_rule() {
        campaign_over(
            "ellipses crossed",
            Answering::Exactly,
            drawn_ellipse_crossed,
            ellipses_exactly,
        );
    }

    #[test]
    #[ignore = "a campaign, run by hand: see the head of this file"]
    fn a_campaign_of_random_ellipses_crossed_through_the_application_s_body_keeps_every_rule() {
        campaign_over(
            "ellipses crossed",
            Answering::ThroughTheApplication,
            drawn_ellipse_crossed,
            ellipses_through_the_application,
        );
    }

    #[test]
    #[ignore = "a campaign, run by hand: see the head of this file"]
    fn a_campaign_of_random_ellipses_tilted_on_the_exact_kernel_keeps_every_rule() {
        campaign_over(
            "ellipses tilted",
            Answering::Exactly,
            drawn_ellipse_tilted,
            ellipses_exactly,
        );
    }

    #[test]
    #[ignore = "a campaign, run by hand: see the head of this file"]
    fn a_campaign_of_random_ellipses_tilted_through_the_application_s_body_keeps_every_rule() {
        campaign_over(
            "ellipses tilted",
            Answering::ThroughTheApplication,
            drawn_ellipse_tilted,
            ellipses_through_the_application,
        );
    }

    #[test]
    #[ignore = "a campaign, run by hand once: see the head of this file"]
    fn ellipse_cases_the_flats_answer_by_their_own_rules_on_300_seeds_keep_every_rule_held_to_the_arithmetic()
     {
        for (name, draw) in ELLIPSE_DRAWS {
            let seeds: Vec<u64> = (0..300).collect();
            let weighed = random_solids::on_every_core(&seeds, |seed| {
                let case = draw(*seed);
                random_solids::check(&case).ok()?;
                let measured = random_solids::held_to_arithmetic(&case, &Flats::for_case(&case));
                Some((measured, case))
            });
            let mut held = 0;
            for (seed, (measured, case)) in seeds
                .iter()
                .zip(weighed)
                .filter_map(|(seed, one)| Some((seed, one?)))
            {
                held += 1;
                assert!(
                    measured.is_ok(),
                    "{name}, seed {seed}: {measured:?}\n{case}"
                );
            }
            println!("{name}: {held} cases of {} held", seeds.len());
        }
    }

    /// Each seed named in `CAO_TRIAGE_SEEDS`, commas between them, run again,
    /// shrunk while it still breaks the same rule, and printed as a test: what a
    /// campaign's failures are sorted into distinct ones from. The seeds name
    /// square cases, or profiles when `CAO_TRIAGE_DRAW` is `profiles`, or
    /// cases of the draws of ellipses when it is `ellipses`,
    /// `ellipses_crossed` or `ellipses_tilted`; they are run on the exact
    /// kernel, or through the application's body when `CAO_TRIAGE_KERNEL` is
    /// `application`.
    #[test]
    #[ignore = "run by hand on the seeds a campaign named"]
    fn the_seeds_a_campaign_named_are_shrunk_one_by_one() {
        let seeds: Vec<u64> = std::env::var("CAO_TRIAGE_SEEDS")
            .unwrap_or_default()
            .split(',')
            .filter_map(|seed| seed.trim().parse().ok())
            .collect();
        let draw: fn(u64) -> Case = match std::env::var("CAO_TRIAGE_DRAW").as_deref() {
            Ok("profiles") => Case::drawn_profiles,
            Ok("ellipses") => Case::drawn_ellipses,
            Ok("ellipses_crossed") => Case::drawn_ellipses_crossed,
            Ok("ellipses_tilted") => Case::drawn_ellipses_tilted,
            _ => Case::drawn_square,
        };
        let check: Check<Case> = match std::env::var("CAO_TRIAGE_KERNEL").as_deref() {
            Ok("application") => Arc::new(through_the_application),
            _ => Arc::new(exactly),
        };
        shrunk_one_by_one(seeds, draw, check);
    }
}
