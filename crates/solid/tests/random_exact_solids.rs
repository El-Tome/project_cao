//! Solids drawn at random and combined at random by the exact kernel of #498,
//! held to the rules every result must keep.

// The drawing, the promise and the checks are shared with the flats'
// campaign; each file uses its own part of them.
#[allow(dead_code, unused_imports)]
mod random_solids;

use cao_solid::soundness::{Lines, Spans};
use glam::{DVec2, DVec3};
use random_solids::{Case, Leaf, Mode, Outline, Plane, Stretch};

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

fn drawn_prisms(kept: impl Fn(&Outline) -> bool) -> Vec<Leaf> {
    (0..400)
        .flat_map(|seed| {
            let case = Case::drawn(seed);
            case.leaves().cloned().collect::<Vec<Leaf>>()
        })
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
                Leaf::Revolution { .. } => false,
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
        Leaf::Revolution { .. } => None,
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
