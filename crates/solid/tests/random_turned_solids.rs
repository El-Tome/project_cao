//! Profiles of straight runs turned about lines of their planes, drawn at
//! random among prisms and combined at random, held to the rules every
//! result must keep (#533).
//!
//! The cases are #448's, drawn among turns (`Case::drawn_turned`): a section
//! of bands laid along a line of one of the planes of the origin — a
//! rectangle, a stepped shaft, an L, a tube, a band with a hole, on its axis,
//! off it, a hair either side of it, or across it — turned whole, by a
//! quarter turn's multiple or by a step of fifteen degrees, either way; half
//! of them drawn from a leaf before them, about a circle's axis, on a
//! block's edge, along the axis of a turn before them, and the prisms drawn
//! from turns, coaxial bosses, blocks tangent to a shaft, holes across it.
//!
//! The arithmetic holds a turned leaf to a slab of an annulus cut by a wedge
//! along every line, and to Pappus's volume, which the corners the kernels
//! are handed are held to here in turn. A section across its axis is handed
//! over as its two sides, each turned and the two joined, so that the
//! kernel's own joins of the halves are tried at every angle. The flats hold
//! the arithmetic where they can be trusted with a turn: lone leaves turned
//! forwards, off their axis, within the sagitta of their steps. Whole cases
//! of turns are too slow on the flats and too often broken for the gate
//! (#418, #486, #492), so they hold the arithmetic once, by hand, behind
//! `campaigns`: the flats answered 161 of the first 300 turned cases by
//! their own rules when it was written, and the arithmetic held every one.
//!
//! Runs slant too (#536), in a draw of their own (`Case::drawn_slanted`),
//! which keeps every seed of `drawn_turned` its case: three fresh sections
//! in four take a chamfer where a shaft steps — its slant landing on the
//! wall past the step, a hair short of it, or short on the lattice — a
//! point, a chamfered end or bore, a countersink, a taper, a ridge, two
//! slants on one line, or a slope a hair from parallel or from square; and
//! tools are drawn from them: a countersunk hole into a block, a chamfer cut
//! round a raised circle, a cone standing on it, and after a slant the same
//! cone again, a cylinder at its rim, a cone crossing it there, a slant a
//! hair off it, a circle on its rim. A slanted piece of a section turns into
//! the room between two cones, a line against each of them one quadratic,
//! and encloses a frustum's volume; a line through a cone's tip, where the
//! cone has no normal, is left out and counted as a grazing one is. The
//! reading lays a slanted run as drawn, and the exact kernel turns it into a
//! cone. The flats answered 231 of the first 300 slanted cases by their own
//! rules when the draw was written, and the arithmetic held every one.
//!
//! The exact kernel lays a section against its axis before turning it; a
//! hole a hair from its band's edge is laid onto the outline, and the wall
//! between them is not there. Where that leaves the matter in two pieces, or
//! touching itself at a corner, it declines the section, which the
//! application turns on the flats then, and that decline is counted apart;
//! so is a section a hair long from end to end, which laying makes nothing.
//! A campaign is run by hand, on the exact kernel or through the
//! application's body:
//!
//! ```text
//! CAO_FUZZ_SECONDS=3600 cargo test --release -p cao_solid --features campaigns \
//!     --test random_turned_solids -- --ignored --nocapture \
//!     a_campaign_of_turned_solids_on_the_exact_kernel
//! ```
//!
//! It draws past the gate's lattice (`Case::drawn_turned_off_the_lattice`):
//! angles a hair from half a turn, a quarter and a whole one, and axes
//! slanted or leaning a hair, where a case asking for a conic may be
//! declined as unsupported and is counted so. The campaigns of slanted
//! solids (`a_campaign_of_slanted_solids_…`) draw past it the same way
//! (`Case::drawn_slanted_off_the_lattice`), with slopes at thirty and sixty
//! degrees besides, and coaxial tools a hair off their axis.
//! `CAO_FUZZ_SEED`, `CAO_FUZZ_CASES` and `CAO_FUZZ_PATIENCE` are read as in
//! `random_exact_solids.rs`; the seeds a campaign names are shrunk by
//! `the_seeds_a_turning_campaign_named_are_shrunk_one_by_one`, given in
//! `CAO_TRIAGE_SEEDS`, with `CAO_TRIAGE_KERNEL=application` for the
//! campaign through the application's body and `CAO_TRIAGE_DRAW=slanted`
//! for a campaign of slanted solids.

// The drawing, the promise and the checks are shared with the other
// campaigns; each file uses its own part of them.
#[allow(dead_code, unused_imports)]
mod random_solids;

use std::f64::consts::PI;

use cao_solid::Declined;
use cao_solid::brep::Scale;
use cao_solid::soundness::{Lines, Spans, shrink};
use cao_solid::turning::Lie;
use cao_solid::turning::Straight;
use glam::{DQuat, DVec2, DVec3};
use random_solids::{
    Application, Axis, Case, Drawn, Exact, Flats, Kernel, Leaf, Mode, ON_THE_AXIS, Outline, Piece,
    Plane, Section, Step, Stretch, TESSELLATION, Trapezoid, Turned,
};

fn spans_of(stretches: &[Stretch]) -> Spans {
    Spans::gathered(
        stretches
            .iter()
            .map(|stretch| (stretch.from.at, stretch.to.at))
            .collect(),
    )
}

fn turned(leaf: &Leaf) -> Option<Turned> {
    match leaf {
        Leaf::Turned { .. } => leaf.as_turned(),
        _ => None,
    }
}

/// The turned leaves of the first cases of the draw.
fn drawn_turns(seeds: u64) -> Vec<Leaf> {
    (0..seeds)
        .flat_map(|seed| {
            Case::drawn_turned(seed)
                .leaves()
                .cloned()
                .collect::<Vec<_>>()
        })
        .filter(|leaf| matches!(leaf, Leaf::Turned { .. }))
        .collect()
}

/// A bundle of lines laid over the box a leaf spans, and how far it reaches.
fn lines_over(leaf: &Leaf) -> (Lines, f64) {
    let (low, high) = leaf.bounds().expect("a leaf with a box");
    let reach = low.abs().max(high.abs()).max_element().max(1.0);
    (Lines::across(low - 1.0, high + 1.0, 24), reach)
}

fn fingerprint(draw: fn(u64) -> Case) -> u64 {
    (0..300)
        .flat_map(|seed| draw(seed).to_string().into_bytes())
        .fold(0xcbf2_9ce4_8422_2325, |hash, byte| {
            (hash ^ u64::from(byte)).wrapping_mul(0x0100_0000_01b3)
        })
}

#[test]
fn the_draws_before_turns_still_give_each_seed_its_case() {
    assert_eq!(
        Case::drawn(29).to_string(),
        "Case::new(
    Leaf::revolution(Plane::xz(7.0), [1.0, -3.0], [6.0, 0.0], -270.0),
    vec![
        Step::cut(Leaf::revolution(Plane::xy(4.0), [2.0, 4.0], [7.0, 8.0], 360.0)),
    ],
)"
    );
    assert_eq!(
        Case::drawn_profiles(175).to_string(),
        "Case::new(
    Leaf::prism(Plane::xy(1.0), Outline::slot([8.0, 2.0], [12.0, 2.0], 1.0), 9.5),
    vec![
        Step::cut(Leaf::prism(Plane::xy(1.0), Outline::circle([8.0, 2.0], 2.0), -7.0)),
    ],
)"
    );
    assert_eq!(
        Case::drawn_square(128).to_string(),
        "Case::new(
    Leaf::prism(Plane::xy(6.0), Outline::circle([6.0, 4.0], 3.5), 5.5),
    vec![
        Step::cut(Leaf::prism(Plane::xy(6.0), Outline::circle([6.0, 4.0], 2.0), 5.5)),
    ],
)"
    );
    assert_eq!(fingerprint(Case::drawn), 0x9adb_9ced_5c90_2374);
    assert_eq!(fingerprint(Case::drawn_square), 0x4d0a_3585_66ad_9363);
    assert_eq!(fingerprint(Case::drawn_profiles), 0xfce7_f8b3_1736_214a);
}

#[test]
fn the_same_seed_draws_the_same_turned_case_and_not_one_of_the_other_draws() {
    for seed in 0..200 {
        assert_eq!(
            Case::drawn_turned(seed),
            Case::drawn_turned(seed),
            "seed {seed}"
        );
    }
    let distinct: std::collections::BTreeSet<String> = (0..200)
        .map(|seed| Case::drawn_turned(seed).to_string())
        .collect();
    assert!(distinct.len() > 190, "{} distinct cases", distinct.len());
    let same = (0..200)
        .filter(|seed| {
            let turned = Case::drawn_turned(*seed);
            turned == Case::drawn_profiles(*seed) || turned == Case::drawn_square(*seed)
        })
        .count();
    assert!(same < 20, "{same} turned cases are another draw's");
}

#[test]
fn every_seed_draws_a_turned_case_though_a_turn_before_has_no_radius_to_draw_from() {
    for seed in 0..20_000 {
        let drawn = std::panic::catch_unwind(|| {
            Case::drawn_turned(seed);
            Case::drawn_turned_off_the_lattice(seed);
            Case::drawn_slanted(seed);
            Case::drawn_slanted_off_the_lattice(seed);
        });
        assert!(drawn.is_ok(), "seed {seed} draws no case");
    }
}

#[test]
fn a_turned_case_holds_straight_profiles_turned_about_lines_of_the_planes_of_the_origin_and_prisms()
{
    for seed in 0..3000 {
        let case = Case::drawn_turned(seed);
        for leaf in case.leaves() {
            assert!(leaf.is_solid(), "seed {seed}: {case}");
            assert!(
                !matches!(leaf.plane(), Plane::Tilted { .. }),
                "seed {seed}: {case}"
            );
            match leaf {
                Leaf::Prism { outline, .. } => {
                    assert!(
                        !matches!(outline, Outline::Star { .. }),
                        "seed {seed}: {case}"
                    )
                }
                Leaf::Turned { axis, .. } => {
                    assert_eq!(axis.lean, 0.0, "seed {seed}: {case}");
                    let line = axis.line();
                    for side in turned(leaf).expect("a turn").drawn() {
                        for corners in std::iter::once(&side.outline).chain(&side.holes) {
                            for (index, corner) in corners.iter().enumerate() {
                                let run = corners[(index + 1) % corners.len()] - *corner;
                                assert!(
                                    run.dot(line.direction) == 0.0
                                        || run.perp_dot(line.direction) == 0.0,
                                    "seed {seed}, a run leaning off its axis: {case}"
                                );
                            }
                        }
                    }
                }
                Leaf::Revolution { .. } => panic!("seed {seed} drew a revolution: {case}"),
            }
        }
    }
}

/// The world line a leaf turns about, or a circle-like prism is raised
/// along: a point of it and the world axis it runs along.
fn world_line(leaf: &Leaf) -> Option<(DVec3, usize)> {
    match leaf {
        Leaf::Turned { plane, axis, .. } => {
            let line = axis.line();
            let point = plane.to_world(line.origin);
            let direction = plane.to_world(line.origin + line.direction) - point;
            Some((point, direction.abs().max_position()))
        }
        Leaf::Prism {
            plane,
            outline: Outline::Circle { center, .. } | Outline::Ring { center, .. },
            ..
        } => Some((plane.to_world(*center), plane.normal().abs().max_position())),
        _ => None,
    }
}

fn same_line(one: (DVec3, usize), other: (DVec3, usize)) -> bool {
    one.1 == other.1 && (0..3).all(|axis| axis == one.1 || one.0[axis] == other.0[axis])
}

/// Whether two lines, along different world axes, cross.
fn crossing_lines(one: (DVec3, usize), other: (DVec3, usize)) -> bool {
    one.1 != other.1 && {
        let third = 3 - one.1 - other.1;
        one.0[third] == other.0[third]
    }
}

fn radius_of(leaf: &Leaf) -> Option<f64> {
    match leaf {
        Leaf::Prism {
            outline: Outline::Circle { radius, .. },
            ..
        } => Some(*radius),
        _ => None,
    }
}

#[test]
fn the_turned_generator_draws_every_kind_it_keeps() {
    let cases: Vec<Case> = (0..1000).map(Case::drawn_turned).collect();
    let leaves = || cases.iter().flat_map(Case::leaves);
    let turns = || leaves().filter_map(turned);
    let seen = |what: &str, found: bool| assert!(found, "no turned case drew {what}");

    seen(
        "a whole turn",
        turns().any(|turn| turn.degrees.abs() == 360.0),
    );
    seen(
        "a quarter turn's multiple short of a whole one",
        turns().any(|turn| turn.degrees.abs() < 360.0 && turn.degrees % 90.0 == 0.0),
    );
    seen(
        "a turn off the quarter turns",
        turns().any(|turn| turn.degrees % 90.0 != 0.0),
    );
    seen("a turn backwards", turns().any(|turn| turn.degrees < 0.0));
    seen(
        "three bands or more",
        turns().any(|turn| turn.section.bands.len() >= 3),
    );
    seen(
        "an L",
        turns().any(|turn| {
            let bands = &turn.section.bands;
            bands.len() == 2 && bands[0][1] == bands[1][1] && bands[0][1] > 0.0
        }),
    );
    seen(
        "a tube",
        turns().any(|turn| turn.section.bands.iter().all(|band| band[1] > 0.0)),
    );
    seen("a hole", turns().any(|turn| !turn.section.holes.is_empty()));
    seen(
        "a section on its axis",
        turns().any(|turn| turn.section.bands.iter().any(|band| band[1] == 0.0)),
    );
    for (name, sign) in [("on its side", 1.0), ("across", -1.0)] {
        seen(
            &format!("a section a hair {name} its axis"),
            turns().any(|turn| {
                let hair = 1e-5 * turn.section.reach();
                turn.section
                    .bands
                    .iter()
                    .any(|band| band[1] != 0.0 && band[1].abs() < hair && band[1].signum() == sign)
            }),
        );
    }
    seen(
        "a section on the other side of its axis",
        turns().any(|turn| turn.section.bands.iter().all(|band| band[2] <= 0.0)),
    );
    seen(
        "a section across its axis",
        turns().any(|turn| turn.section.side().is_none()),
    );
    seen(
        "a turn about the sketch's axis",
        turns().any(|turn| turn.axis.across == 0.0),
    );
    seen(
        "a turn about a line drawn parallel to it",
        turns().any(|turn| turn.axis.across != 0.0),
    );
    seen(
        "an axis run backwards",
        turns().any(|turn| turn.axis.backwards),
    );
    for (name, kind) in [
        ("XY", Plane::xy(0.0)),
        ("XZ", Plane::xz(0.0)),
        ("YZ", Plane::yz(0.0)),
    ] {
        seen(
            name,
            turns()
                .any(|turn| std::mem::discriminant(&turn.plane) == std::mem::discriminant(&kind)),
        );
    }

    let earlier_and_later = |related: &dyn Fn(&Leaf, &Leaf) -> bool| {
        cases.iter().any(|case| {
            let leaves: Vec<&Leaf> = case.leaves().collect();
            (1..leaves.len()).any(|later| {
                leaves[..later]
                    .iter()
                    .any(|earlier| related(earlier, leaves[later]))
            })
        })
    };
    let coaxial = |one: &Leaf, other: &Leaf| match (world_line(one), world_line(other)) {
        (Some(one), Some(other)) => same_line(one, other),
        _ => false,
    };
    seen(
        "a turn coaxial with a circle raised before it",
        earlier_and_later(&|earlier, later| {
            radius_of(earlier).is_some() && turned(later).is_some() && coaxial(earlier, later)
        }),
    );
    seen(
        "a turn whose wall is flush with a circle raised before it",
        earlier_and_later(&|earlier, later| {
            radius_of(earlier).is_some_and(|radius| {
                coaxial(earlier, later)
                    && turned(later).is_some_and(|turn| {
                        turn.section
                            .bands
                            .iter()
                            .any(|band| band[2].abs() == radius)
                    })
            })
        }),
    );
    seen(
        "a turn with a shoulder flush with the end of a circle raised before it",
        earlier_and_later(&|earlier, later| {
            let (Some(turn), Leaf::Prism { plane, height, .. }) = (turned(later), earlier) else {
                return false;
            };
            let Some((_, axis)) = world_line(earlier) else {
                return false;
            };
            let floor = plane.to_world(DVec2::ZERO)[axis];
            let caps = [floor, floor + plane.normal()[axis] * height];
            coaxial(earlier, later)
                && turn.section.ends().iter().any(|end| {
                    let level = if turn.axis.backwards { -end } else { *end };
                    caps.contains(&level)
                })
        }),
    );
    seen(
        "a turn touching a circle raised before it from across it",
        earlier_and_later(&|earlier, later| {
            let (Some(radius), Some(turn)) = (radius_of(earlier), turned(later)) else {
                return false;
            };
            let (Some(one), Some(other)) = (world_line(earlier), world_line(later)) else {
                return false;
            };
            one.1 != other.1
                && turn.section.bands.len() == 1
                && (one.0[3 - one.1 - other.1] - other.0[3 - one.1 - other.1]).abs()
                    == radius + turn.section.bands[0][2].abs()
        }),
    );
    seen(
        "a turn about the edge of a block raised before it",
        earlier_and_later(&|earlier, later| {
            let (
                Leaf::Prism {
                    plane,
                    outline: Outline::Rectangle { low, high },
                    height,
                },
                Some(line),
            ) = (earlier, world_line(later))
            else {
                return false;
            };
            let floor = plane.to_world(DVec2::ZERO);
            let caps = [floor, floor + plane.normal() * *height];
            [low.x, high.x, low.y, high.y].iter().any(|edge| {
                caps.iter().any(|cap| {
                    let (_, u, v) = plane.frame();
                    [u, v].iter().any(|across| {
                        let corner = *cap + *across * *edge;
                        (0..3).all(|axis| axis == line.1 || corner[axis] == line.0[axis])
                    })
                })
            })
        }),
    );
    seen(
        "a turn tangent to the top or the floor of a block raised before it",
        earlier_and_later(&|earlier, later| {
            let (
                Leaf::Prism {
                    plane,
                    outline: Outline::Rectangle { .. },
                    height,
                },
                Some(turn),
                Some((point, along)),
            ) = (earlier, turned(later), world_line(later))
            else {
                return false;
            };
            let normal = plane.normal();
            let across = normal.abs().max_position();
            let floor = plane.to_world(DVec2::ZERO)[across];
            let radius = turn.section.bands[0][2].abs();
            along != across
                && turn.section.bands.len() == 1
                && [floor, floor + normal[across] * height]
                    .iter()
                    .any(|cap| (point[across] - cap).abs() == radius)
        }),
    );
    seen(
        "two turns about one line",
        earlier_and_later(&|earlier, later| {
            turned(earlier).is_some() && turned(later).is_some() && coaxial(earlier, later)
        }),
    );
    seen(
        "a groove across the outer wall of a turn before it",
        earlier_and_later(&|earlier, later| {
            let (Some(first), Some(second)) = (turned(earlier), turned(later)) else {
                return false;
            };
            let outer = first
                .section
                .bands
                .iter()
                .map(|band| band[1].abs().max(band[2].abs()))
                .fold(0.0, f64::max);
            coaxial(earlier, later)
                && second.section.bands.iter().any(|band| {
                    let (near, far) = (
                        band[1].abs().min(band[2].abs()),
                        band[1].abs().max(band[2].abs()),
                    );
                    near < outer && outer < far
                })
        }),
    );
    seen(
        "a hole across a turn before it",
        earlier_and_later(
            &|earlier, later| match (world_line(earlier), world_line(later)) {
                (Some(one), Some(other)) => {
                    turned(earlier).is_some()
                        && radius_of(later).is_some()
                        && crossing_lines(one, other)
                }
                _ => false,
            },
        ),
    );

    let count = leaves().count();
    let share = turns().count() as f64 / count as f64;
    assert!(
        (0.35..0.65).contains(&share),
        "turns are {} leaves of {count}",
        turns().count()
    );
}

#[test]
fn a_section_across_its_axis_is_handed_over_as_its_two_sides() {
    let leaf = Leaf::turned(
        Plane::xz(1.0),
        Axis::second(2.0),
        Section::bands(0.0, &[[2.0, -4.0, 5.0], [1.0, -1.0, 3.0]])
            .with_holes(&[([0.5, 1.0], [1.5, 2.0])]),
        90.0,
    );
    let turn = turned(&leaf).expect("a turn");
    let sides = turn.section.sides();
    assert_eq!(
        sides,
        vec![
            Section::bands(0.0, &[[2.0, 0.0, 5.0], [1.0, 0.0, 3.0]])
                .with_holes(&[([0.5, 1.0], [1.5, 2.0])]),
            Section::bands(0.0, &[[2.0, -4.0, 0.0], [1.0, -1.0, 0.0]]),
        ]
    );
    let measure = |pieces: Vec<Piece>, side: f64| {
        pieces
            .iter()
            .filter(|piece| piece.side == side)
            .map(|piece| {
                let length = piece.along[1] - piece.along[0];
                DVec2::new(
                    length * (piece.away[1] - piece.away[0]),
                    length * (piece.away[1].powi(2) - piece.away[0].powi(2)),
                )
            })
            .sum::<DVec2>()
    };
    for side in [1.0, -1.0] {
        let halves = sides
            .iter()
            .map(|half| measure(half.pieces(), side))
            .sum::<DVec2>();
        assert_eq!(halves, measure(turn.section.pieces(), side), "side {side}");
    }

    let across: Vec<Turned> = drawn_turns(1000)
        .iter()
        .filter_map(turned)
        .filter(|turn| turn.section.side().is_none())
        .chain(std::iter::once(turn))
        .collect();
    assert!(across.len() > 20, "{} sections across", across.len());
    for turn in across {
        let drawn = turn.drawn();
        assert_eq!(drawn.len(), 2, "{:?}", turn.section);
        let line = turn.turn(0.0);
        for (side, expected) in drawn.iter().zip([1.0, -1.0]) {
            assert_eq!(
                line.lie(&side.profile(true)),
                Lie::Side(expected),
                "{:?}",
                turn.section
            );
        }
        for side in turn.section.sides() {
            assert!(side.is_solid(), "{side:?} of {:?}", turn.section);
        }
    }
}

/// What the corners a kernel is handed sweep per radian, by Pappus: the
/// first moment of the area about the axis, read off the corners alone,
/// those within a hair of the axis laid on it as the arithmetic lays them —
/// which moves a slant from such a corner by more than the hair's square.
fn moment(turn: &Turned) -> Vec<f64> {
    let line = turn.axis.line();
    let snap = ON_THE_AXIS * turn.section.reach();
    let snapped = |s: f64| if s.abs() <= snap { 0.0 } else { s };
    turn.drawn()
        .iter()
        .map(|side| {
            std::iter::once(&side.outline)
                .chain(&side.holes)
                .map(|corners| {
                    let read: Vec<DVec2> = corners
                        .iter()
                        .map(|corner| {
                            DVec2::new(
                                (*corner - line.origin).dot(line.direction),
                                snapped(line.side(*corner)),
                            )
                        })
                        .collect();
                    (0..read.len())
                        .map(|index| {
                            let (one, other) = (read[index], read[(index + 1) % read.len()]);
                            one.perp_dot(other) * (one.y + other.y) / 6.0
                        })
                        .sum::<f64>()
                })
                .sum::<f64>()
                .abs()
        })
        .collect()
}

#[test]
fn the_pieces_of_a_section_turn_into_pappus_s_volume_of_its_corners() {
    let block = Section::bands(0.0, &[[4.0, 2.0, 5.0]]);
    for degrees in [90.0, -90.0] {
        let leaf = Leaf::turned(Plane::xy(0.0), Axis::first(0.0), block.clone(), degrees);
        let volume = turned(&leaf).expect("a turn").volume();
        assert!((volume - 21.0 * PI).abs() < 1e-12, "{volume}");
    }
    let across = Section::bands(0.0, &[[2.0, -4.0, 5.0]]);
    for (degrees, expected) in [(360.0, 50.0 * PI), (90.0, 20.5 * PI)] {
        let leaf = Leaf::turned(Plane::xy(0.0), Axis::second(0.0), across.clone(), degrees);
        let volume = turned(&leaf).expect("a turn").volume();
        assert!((volume - expected).abs() < 1e-12, "{degrees}: {volume}");
    }

    let turns: Vec<Turned> = drawn_turns(1000).iter().filter_map(turned).collect();
    assert!(turns.len() > 300, "{} turns", turns.len());
    for turn in turns {
        let sides = moment(&turn);
        let pappus: f64 = sides.iter().map(|side| side * turn.angle()).sum();
        let volume = turn.volume();
        if sides.len() == 1 || turn.degrees.abs() <= 180.0 {
            assert!(
                (volume - pappus).abs() <= 1e-9 * pappus,
                "{:?}: {volume} against {pappus}",
                turn.section
            );
        } else {
            let largest = sides.iter().copied().fold(0.0, f64::max) * turn.angle();
            assert!(
                largest <= volume * (1.0 + 1e-9) && volume <= pappus * (1.0 + 1e-9),
                "{:?}: {volume} between {largest} and {pappus}",
                turn.section
            );
        }
    }
}

/// The lone turns among `leaves` that the flats answer by their own rules,
/// turned forwards and off their axis: where the flats can be trusted with
/// a turn.
fn turns_the_flats_hold(leaves: Vec<Leaf>) -> Vec<Leaf> {
    let leaves: Vec<Leaf> = leaves
        .into_iter()
        .filter(|leaf| {
            turned(leaf).is_some_and(|turn| {
                turn.degrees > 0.0
                    && turn.section.side().is_some()
                    && turn
                        .section
                        .pieces()
                        .iter()
                        .all(|piece| piece.away[0] > 0.0 && piece.ending[0] > 0.0)
            })
        })
        .collect();
    let kept = random_solids::on_every_core(&leaves, |leaf| {
        random_solids::check(&Case::new(leaf.clone(), vec![])).is_ok()
    });
    leaves
        .into_iter()
        .zip(kept)
        .filter_map(|(leaf, kept)| kept.then_some(leaf))
        .collect()
}

/// How far in a flat laid across a step of a turn stands from the circle:
/// the flats take at most a sixty-fourth of a turn a step.
fn step_cosine() -> f64 {
    (PI / 64.0).cos()
}

/// Whether the flats a leaf is turned into hold along every line what the
/// leaf holds with its outer walls drawn in by their sagitta, and no more
/// than it holds with its inner walls drawn in by theirs.
fn holds_what_its_flats_hold(leaf: &Leaf, flats: &cao_solid::Body) {
    let turn = turned(leaf).expect("a turn");
    let (lines, reach) = lines_over(leaf);
    let measured = lines.inside(&flats.triangles());
    let pieces = turn.section.pieces();
    let least: Vec<Piece> = pieces
        .iter()
        .map(|piece| Piece {
            away: [piece.away[0], piece.away[1] * step_cosine()],
            ending: [piece.ending[0], piece.ending[1] * step_cosine()],
            ..*piece
        })
        .collect();
    let most: Vec<Piece> = pieces
        .iter()
        .map(|piece| Piece {
            away: [piece.away[0] * step_cosine(), piece.away[1]],
            ending: [piece.ending[0] * step_cosine(), piece.ending[1]],
            ..*piece
        })
        .collect();
    let room = 1e-9 * reach;
    for (index, measure) in measured.iter().enumerate() {
        let (origin, direction) = lines.line(index);
        let least = spans_of(&turn.along_pieces(&least, origin, direction, 0.0));
        let most = spans_of(&turn.along_pieces(&most, origin, direction, 0.0));
        assert!(
            least.without(measure).length() <= room && measure.without(&most).length() <= room,
            "line {index} across {leaf}: {measure:?} not between {least:?} and {most:?}"
        );
    }
}

/// Whether the box a leaf spans holds the box its flats span, and stands
/// past it by no more than their sagitta.
fn spans_the_box_its_flats_span(leaf: &Leaf, flats: &cao_solid::Body) {
    let turn = turned(leaf).expect("a turn");
    let (low, high) = leaf.bounds().expect("a box");
    let (flat_low, flat_high) = flats.bounds().expect("a box of flats");
    let reach = low.abs().max(high.abs()).max_element();
    let outer = turn
        .section
        .pieces()
        .iter()
        .map(|piece| piece.away[1].max(piece.ending[1]))
        .fold(0.0, f64::max);
    let sagitta = outer * (1.0 - step_cosine()) + 1e-9 * reach;
    assert!(
        (low - flat_low).max_element() <= 1e-9 * reach
            && (flat_high - high).max_element() <= 1e-9 * reach,
        "{leaf}: {low} {high} against the flats' {flat_low} {flat_high}"
    );
    assert!(
        (flat_low - low).max_element() <= sagitta && (high - flat_high).max_element() <= sagitta,
        "{leaf}: {low} {high} past the flats' {flat_low} {flat_high}"
    );
}

#[test]
fn a_turned_leaf_off_its_axis_holds_along_every_line_what_its_flats_hold_but_for_their_sagitta() {
    let leaves = turns_the_flats_hold(drawn_turns(400));
    assert!(leaves.len() > 60, "{} leaves", leaves.len());
    for leaf in &leaves {
        holds_what_its_flats_hold(leaf, &leaf.solid().expect("a turn the flats make"));
    }
}

#[test]
fn the_box_a_turned_leaf_spans_holds_the_box_its_flats_span_within_their_sagitta() {
    for leaf in &turns_the_flats_hold(drawn_turns(400)) {
        spans_the_box_its_flats_span(leaf, &leaf.solid().expect("a turn the flats make"));
    }
}

/// A line turned about the axis of a turn by `turn` radians.
fn turned_line(turn: &Turned, radians: f64, (origin, direction): (DVec3, DVec3)) -> (DVec3, DVec3) {
    let swept = turn.swept(1.0);
    let rotation = DQuat::from_axis_angle(swept.along, radians);
    (
        swept.origin + rotation * (origin - swept.origin),
        rotation * direction,
    )
}

#[test]
fn a_turn_backwards_holds_along_every_line_what_the_turn_forwards_holds_along_the_line_turned_back()
{
    let leaves: Vec<Leaf> = drawn_turns(300)
        .into_iter()
        .filter(|leaf| turned(leaf).is_some_and(|turn| turn.degrees > 0.0))
        .collect();
    assert!(leaves.len() > 100, "{} leaves", leaves.len());
    for leaf in &leaves {
        let forwards = turned(leaf).expect("a turn");
        let backwards = Turned {
            degrees: -forwards.degrees,
            ..forwards.clone()
        };
        let (lines, reach) = lines_over(leaf);
        for index in 0..lines.count() {
            let line = lines.line(index);
            let (origin, direction) = turned_line(&forwards, forwards.degrees.to_radians(), line);
            let back = spans_of(&backwards.along_grown(line.0, line.1, 0.0));
            let forth = spans_of(&forwards.along_grown(origin, direction, 0.0));
            assert!(
                back.without(&forth).length() + forth.without(&back).length() <= 1e-9 * reach,
                "line {index} across {leaf}: {back:?} against {forth:?}"
            );
        }
    }
}

#[test]
fn a_line_through_a_turned_leaf_is_told_how_squarely_it_crosses_each_wall_and_whether_it_curves() {
    let tube = Leaf::turned(
        Plane::xy(0.0),
        Axis::first(0.0),
        Section::bands(0.0, &[[4.0, 1.0, 3.0]]),
        360.0,
    );
    let along = tube
        .along(DVec3::new(-1.0, 2.0, 0.0), DVec3::X)
        .expect("a turn");
    let [stretch] = along.as_slice() else {
        panic!("{along:?}");
    };
    assert_eq!((stretch.from.at, stretch.to.at), (1.0, 5.0));
    for end in [stretch.from, stretch.to] {
        assert_eq!((end.cosine, end.curved), (1.0, false));
    }

    let across = tube
        .along(DVec3::new(2.0, 0.0, -5.0), DVec3::Z)
        .expect("a turn");
    let ends: Vec<(f64, bool)> = across
        .iter()
        .flat_map(|stretch| [stretch.from, stretch.to])
        .map(|end| (end.at, end.curved))
        .collect();
    assert_eq!(
        ends,
        vec![(2.0, true), (4.0, true), (6.0, true), (8.0, true)]
    );

    let quarter = Leaf::turned(
        Plane::xy(0.0),
        Axis::first(0.0),
        Section::bands(0.0, &[[4.0, 1.0, 3.0]]),
        90.0,
    );
    let through = quarter
        .along(DVec3::new(2.0, 2.0, -5.0), DVec3::Z)
        .expect("a turn");
    let [stretch] = through.as_slice() else {
        panic!("{through:?}");
    };
    assert_eq!(
        (stretch.from.at, stretch.from.cosine, stretch.from.curved),
        (5.0, 1.0, false)
    );
    assert!(
        (stretch.to.at - 5.0 - 5f64.sqrt()).abs() < 1e-12,
        "{stretch:?}"
    );
    assert!(stretch.to.curved);
    assert!(
        (stretch.to.cosine - 5f64.sqrt() / 3.0).abs() < 1e-12,
        "{stretch:?}"
    );

    let backwards = Leaf::turned(
        Plane::xy(0.0),
        Axis::first(0.0),
        Section::bands(0.0, &[[4.0, 1.0, 3.0]]),
        -90.0,
    );
    let below = backwards
        .along(DVec3::new(2.0, 2.0, -5.0), DVec3::Z)
        .expect("a turn");
    let [stretch] = below.as_slice() else {
        panic!("{below:?}");
    };
    assert!(
        (stretch.from.at - 5.0 + 5f64.sqrt()).abs() < 1e-12,
        "{stretch:?}"
    );
    assert_eq!((stretch.to.at, stretch.to.curved), (5.0, false));
}

#[test]
fn a_turned_leaf_grown_holds_every_line_it_held_and_shrunk_holds_none_it_did_not() {
    for leaf in drawn_turns(200) {
        let (lines, reach) = lines_over(&leaf);
        let by = 1e-3 * reach;
        for index in 0..lines.count() {
            let (origin, direction) = lines.line(index);
            let [shrunk, held, grown] = [-by, 0.0, by]
                .map(|by| spans_of(&leaf.along_grown(origin, direction, by).expect("a turn")));
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
fn a_case_asking_for_a_conic_is_told_from_one_that_does_not() {
    let shaft = |degrees: f64, axis: Axis| {
        Leaf::turned(
            Plane::xy(0.0),
            axis,
            Section::bands(-2.0, &[[6.0, 0.0, 2.0]]),
            degrees,
        )
    };
    let post = Leaf::prism(Plane::xy(-3.0), Outline::circle([1.0, 1.0], 1.0), 6.0);
    let bar = Leaf::prism(Plane::yz(-3.0), Outline::circle([0.0, 0.0], 1.0), 6.0);
    let block = Leaf::prism(
        Plane::xy(-1.0),
        Outline::rectangle([0.0, 0.0], [3.0, 3.0]),
        2.0,
    );
    let asks = |start: &Leaf, tool: &Leaf| {
        Case::new(start.clone(), vec![Step::cut(tool.clone())]).asks_for_a_conic()
    };

    for degrees in [360.0, 90.0, -180.0, 270.0] {
        assert!(!asks(&shaft(degrees, Axis::first(0.0)), &post), "{degrees}");
    }
    for degrees in [30.0, -105.0, 359.9] {
        assert!(asks(&shaft(degrees, Axis::first(0.0)), &post), "{degrees}");
        assert!(!asks(&shaft(degrees, Axis::first(0.0)), &bar), "{degrees}");
        assert!(
            !asks(&shaft(degrees, Axis::first(0.0)), &block),
            "{degrees}"
        );
    }
    let slanted = Axis::first(0.0).leaning(30.0);
    assert!(asks(&shaft(360.0, slanted), &block));
    assert!(asks(&shaft(360.0, slanted), &bar));
    assert!(!Case::new(shaft(45.0, slanted), vec![]).asks_for_a_conic());
    assert!(!asks(&post, &block));

    let asking = (0..1000)
        .filter(|seed| Case::drawn_turned_off_the_lattice(*seed).asks_for_a_conic())
        .count();
    assert!(
        (20..500).contains(&asking),
        "{asking} cases ask for an ellipse"
    );

    let cone = |degrees: f64| {
        Leaf::turned(
            Plane::xy(0.0),
            Axis::first(0.0),
            Section::bands(0.0, &[[4.0, 0.0, 3.0]]).sloping_to(&[[0.0, 1.0]]),
            degrees,
        )
    };
    let about = |axis: Axis| {
        Leaf::turned(
            Plane::xz(0.0),
            axis,
            Section::bands(2.0, &[[4.0, 0.0, 1.0]]).sloping_to(&[[0.0, 3.0]]),
            360.0,
        )
    };
    let told = [
        (
            "a cap square to its axis",
            Leaf::prism(Plane::yz(-1.0), Outline::circle([0.0, 0.0], 2.0), 3.0),
            false,
        ),
        (
            "a block whose face holds its axis",
            Leaf::prism(
                Plane::xy(0.0),
                Outline::rectangle([-1.0, -5.0], [5.0, 5.0]),
                10.0,
            ),
            false,
        ),
        (
            "a flat parallel to its axis and off it",
            Leaf::prism(
                Plane::xy(1.0),
                Outline::rectangle([-1.0, -5.0], [5.0, 5.0]),
                10.0,
            ),
            true,
        ),
        (
            "a flat tangent to its widest rim",
            Leaf::prism(
                Plane::xy(3.0),
                Outline::rectangle([-1.0, -5.0], [5.0, 5.0]),
                10.0,
            ),
            true,
        ),
        (
            "a plane at a slant",
            Leaf::prism(
                Plane::tilted([0.0, 0.0, 0.0], [0.0, 30.0, 0.0]),
                Outline::rectangle([-5.0, -5.0], [5.0, 5.0]),
                1.0,
            ),
            true,
        ),
        (
            "a cylinder about its axis",
            Leaf::prism(Plane::yz(1.0), Outline::circle([0.0, 0.0], 2.0), 2.0),
            false,
        ),
        ("a cone about its axis", about(Axis::first(0.0)), false),
        (
            "a cylinder parallel to its axis and off it",
            Leaf::prism(Plane::yz(1.0), Outline::circle([1.0, 0.0], 1.0), 2.0),
            true,
        ),
        (
            "a hole across it",
            Leaf::prism(Plane::xz(5.0), Outline::circle([2.0, 0.0], 1.0), 10.0),
            true,
        ),
        ("a cone a hair off its axis", about(Axis::first(1e-7)), true),
        (
            "a block whose box stands apart from its own",
            Leaf::prism(
                Plane::xy(1.0),
                Outline::rectangle([20.0, 20.0], [25.0, 25.0]),
                2.0,
            ),
            false,
        ),
    ];
    for (what, tool, asking) in told {
        assert_eq!(asks(&cone(360.0), &tool), asking, "{what}");
    }
    assert!(!Case::new(cone(90.0), vec![]).asks_for_a_conic());
    assert!(!Case::new(cone(-215.0), vec![]).asks_for_a_conic());

    let asking = (0..1000)
        .filter(|seed| Case::drawn_slanted(*seed).asks_for_a_conic())
        .count();
    assert!(
        (200..650).contains(&asking),
        "{asking} slanted cases ask for a conic"
    );
}

#[test]
fn a_turned_case_prints_as_the_rust_that_builds_it_again() {
    let pasted = Case::new(
        Leaf::turned(
            Plane::xz(2.0),
            Axis::second(0.0).backwards(),
            Section::bands(0.0, &[[3.0, 0.0, 2.0], [1.5, 0.0, 1.0]])
                .with_holes(&[([0.5, 0.5], [1.0, 1.5])]),
            -90.0,
        ),
        vec![
            Step::cut(Leaf::turned(
                Plane::xy(-1.0),
                Axis::first(2.5).leaning(1e-7),
                Section::bands(-1.0, &[[2.0, -1.0, 3.0]]),
                360.0,
            )),
            Step::add(Leaf::prism(
                Plane::xy(4.0),
                Outline::circle([5.0, 3.0], 2.5),
                2.0,
            )),
        ],
    );
    assert_eq!(
        pasted.to_string(),
        "Case::new(
    Leaf::turned(Plane::xz(2.0), Axis::second(0.0).backwards(), Section::bands(0.0, &[[3.0, 0.0, 2.0], [1.5, 0.0, 1.0]]).with_holes(&[([0.5, 0.5], [1.0, 1.5])]), -90.0),
    vec![
        Step::cut(Leaf::turned(Plane::xy(-1.0), Axis::first(2.5).leaning(1e-7), Section::bands(-1.0, &[[2.0, -1.0, 3.0]]), 360.0)),
        Step::add(Leaf::prism(Plane::xy(4.0), Outline::circle([5.0, 3.0], 2.5), 2.0)),
    ],
)"
    );
    for seed in 0..200 {
        for case in [
            Case::drawn_turned(seed),
            Case::drawn_turned_off_the_lattice(seed),
        ] {
            let printed = case.to_string();
            assert!(
                !printed.contains("NaN") && !printed.contains("inf"),
                "seed {seed} prints a number Rust cannot read back: {printed}"
            );
        }
    }
}

#[test]
fn a_turned_leaf_shrinks_into_one_band_on_the_sketch_s_axis_turned_whole_and_into_round_numbers() {
    let leaf = Leaf::turned(
        Plane::yz(2.3),
        Axis::second(1.2).backwards().leaning(1e-7),
        Section::bands(0.3, &[[2.2, 0.7, 3.4], [1.0, 0.7, 2.0]])
            .with_holes(&[([0.6, 1.0], [1.8, 2.0])]),
        -105.0,
    );
    let shrunk: Vec<Leaf> = Case::new(leaf.clone(), vec![])
        .smaller()
        .into_iter()
        .map(|case| case.start)
        .collect();
    let with = |change: &dyn Fn(&mut Leaf)| {
        let mut changed = leaf.clone();
        change(&mut changed);
        changed
    };
    let expected = [
        with(&|leaf| {
            if let Leaf::Turned { plane, .. } = leaf {
                *plane = Plane::xy(2.3);
            }
        }),
        with(&|leaf| {
            if let Leaf::Turned { axis, .. } = leaf {
                axis.across = 0.0;
            }
        }),
        with(&|leaf| {
            if let Leaf::Turned { axis, .. } = leaf {
                axis.backwards = false;
            }
        }),
        with(&|leaf| {
            if let Leaf::Turned { axis, .. } = leaf {
                axis.lean = 0.0;
            }
        }),
        with(&|leaf| {
            if let Leaf::Turned { degrees, .. } = leaf {
                *degrees = 360.0;
            }
        }),
        with(&|leaf| {
            if let Leaf::Turned { degrees, .. } = leaf {
                *degrees = -90.0;
            }
        }),
        with(&|leaf| {
            if let Leaf::Turned { degrees, .. } = leaf {
                *degrees = 105.0;
            }
        }),
        with(&|leaf| {
            if let Leaf::Turned { section, .. } = leaf {
                section.bands.remove(1);
            }
        }),
        with(&|leaf| {
            if let Leaf::Turned { section, .. } = leaf {
                section.holes.clear();
            }
        }),
        with(&|leaf| {
            if let Leaf::Turned { section, .. } = leaf {
                for band in &mut section.bands {
                    band[1] = 0.0;
                }
            }
        }),
        with(&|leaf| {
            if let Leaf::Turned { section, .. } = leaf {
                *section = Section::bands(0.3, &[[3.2, 0.7, 3.4]]);
            }
        }),
        with(&|leaf| {
            if let Leaf::Turned { section, .. } = leaf {
                *section = Section::bands(0.0, &[[2.0, 1.0, 3.0], [1.0, 1.0, 2.0]])
                    .with_holes(&[([1.0, 1.0], [2.0, 2.0])]);
            }
        }),
    ];
    for expected in expected {
        assert!(shrunk.contains(&expected), "{expected} among {shrunk:#?}");
    }
}

#[test]
fn shrinking_any_drawn_turned_case_comes_to_an_end() {
    for seed in 0..200 {
        let mut rounds = 0;
        let shrunk = shrink(
            Case::drawn_turned_off_the_lattice(seed),
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

#[test]
fn every_turned_leaf_raised_by_the_exact_kernel_alone_keeps_every_rule_and_encloses_pappus_s_volume()
 {
    let leaves = drawn_turns(200);
    assert!(leaves.len() > 60, "{} leaves", leaves.len());
    raised_alone_keep_every_rule_and_enclose_pappus_s_volume(&leaves);
}

/// Whether every one of `leaves`, raised alone by the exact kernel, keeps
/// every rule held to the arithmetic and encloses the volume the arithmetic
/// promised, and the volume of its section as the kernel laid it; a leaf
/// with a wall a hair thin may be declined.
fn raised_alone_keep_every_rule_and_enclose_pappus_s_volume(leaves: &[Leaf]) {
    let weighed = random_solids::on_every_core(leaves, |leaf| {
        let alone = Case::new(leaf.clone(), vec![]);
        let held = random_solids::held_to_arithmetic(&alone, &Exact);
        (held, Exact.raised(leaf).map(|body| body.volume()))
    });
    for (leaf, (held, volume)) in leaves.iter().zip(weighed) {
        assert!(held.is_ok(), "{leaf}: {held:?}");
        let turn = turned(leaf).expect("a turn");
        let volume = match volume {
            Err(Declined::Profile) if turn.has_a_wall_a_hair_thin() => continue,
            volume => volume.expect("a turned body"),
        };
        let promised = turn.volume();
        assert!(
            (volume - promised).abs() <= 1e-9 * promised + laying_room(&turn),
            "{leaf}: {volume} against {promised}"
        );
        if let [side] = turn.drawn().as_slice() {
            let (outline, holes) = side.contours();
            let straight = Straight::of(&outline, &holes, turn.frame(), &turn.turn(0.0), 0.0)
                .expect("a section of straight runs");
            let laid = turn.angle() * moment_laid(&straight);
            assert!(
                (volume - laid).abs() <= 1e-9 * laid,
                "{leaf}: {volume} against {laid} as laid"
            );
        }
    }
}

/// What a section laid square to its axis sweeps per radian: the moment of
/// its area about the axis, its holes taken out.
fn moment_laid(straight: &Straight) -> f64 {
    straight
        .contours
        .iter()
        .map(|contour| {
            let corners: Vec<DVec2> = contour.iter().map(|corner| corner.at).collect();
            corners
                .iter()
                .zip(corners.iter().cycle().skip(1))
                .map(|(from, to)| from.perp_dot(*to) * (from.y + to.y))
                .sum::<f64>()
                / 6.0
        })
        .sum::<f64>()
        .abs()
}

/// How far laying a section square to its axis may move the volume of its
/// turn: no corner moves by more than twice the tolerance the exact kernel
/// reads it at, which sweeps at most a strip that wide along every edge, at
/// the furthest radius.
fn laying_room(turn: &Turned) -> f64 {
    let frame = turn.frame();
    let furthest = turn
        .section
        .bands
        .iter()
        .flat_map(|band| [band[1].abs(), band[2].abs()])
        .chain(
            turn.section
                .sloping_to
                .iter()
                .flatten()
                .map(|edge| edge.abs()),
        )
        .fold(0.0, f64::max);
    let placed = turn
        .drawn()
        .iter()
        .flat_map(|side| side.outline.clone())
        .map(|corner| frame.at(corner).abs().max_element())
        .fold(0.0, f64::max);
    let tolerance = Scale::HAIR * Scale::of(placed + furthest).eps();
    let edges: f64 = turn
        .section
        .trapezoids()
        .iter()
        .map(|trapezoid| {
            let length = trapezoid.along[1] - trapezoid.along[0];
            let [low, high] = [trapezoid.low, trapezoid.high];
            (high[0] - low[0])
                + (high[1] - low[1])
                + length.hypot(low[1] - low[0])
                + length.hypot(high[1] - high[0])
        })
        .sum();
    turn.angle() * furthest * 2.0 * tolerance * edges
}

/// A run of a lone turn as the kernel lays it: its number, its two laid
/// corners, along the axis and away from it, and the way out of the matter
/// across it.
struct Laid {
    run: u32,
    from: DVec2,
    to: DVec2,
    out: DVec2,
}

/// The numbers of the runs of a one-sided turn the kernel lays to some
/// length, and those of them off the axis as it lays them: read off the
/// reading it is handed, at the tolerances the application's body reads an
/// empty part's tools at; nothing when the reading lays no section. The way
/// out across a run is told by the way the outline turns, its holes turning
/// the other way, since a point probed beside a run may land past a wall a
/// hair thin.
fn laid_runs(turn: &Turned) -> Option<(Vec<u32>, Vec<Laid>)> {
    let [side] = <[Drawn; 1]>::try_from(turn.drawn()).ok().expect("one side");
    let (outline, holes) = side.contours();
    let straight = Straight::of(&outline, &holes, turn.frame(), &turn.turn(0.0), 1.0)?;
    let outline: Vec<DVec2> = straight.contours[0]
        .iter()
        .map(|corner| corner.at)
        .collect();
    let turning = outline
        .iter()
        .zip(outline.iter().cycle().skip(1))
        .map(|(from, to)| from.perp_dot(*to))
        .sum::<f64>()
        .signum();
    let mut runs = Vec::new();
    let mut numbers = Vec::new();
    for contour in &straight.contours {
        for (index, corner) in contour.iter().enumerate() {
            numbers.push(corner.run);
            let (from, to) = (corner.at, contour[(index + 1) % contour.len()].at);
            if from.y == 0.0 && to.y == 0.0 {
                continue;
            }
            let along = (to - from).normalize();
            let out = DVec2::new(along.y, -along.x) * turning;
            runs.push(Laid {
                run: corner.run,
                from,
                to,
                out,
            });
        }
    }
    Some((numbers, runs))
}

#[test]
fn a_turned_leaf_names_its_faces_as_the_flats_name_them() {
    let leaves: Vec<Leaf> = drawn_turns(300)
        .into_iter()
        .filter(|leaf| turned(leaf).is_some_and(|turn| turn.section.side().is_some()))
        .collect();
    assert!(leaves.len() > 100, "{} leaves", leaves.len());
    let exact = name_their_faces_as_the_flats_name_them(&leaves);
    assert_eq!(
        exact,
        leaves.len(),
        "every leaf is turned by the exact kernel"
    );
}

/// Whether every one of `leaves`, turned on one side of its axis, is
/// numbered by the exact kernel as the flats number it, and each run it
/// lays turns into the face of its own number: or of a run on its line, or
/// of one the kernel tells apart past the flats' numbers; and how many of
/// them the exact kernel turned, the others held to the flats by the flats.
fn name_their_faces_as_the_flats_name_them(leaves: &[Leaf]) -> usize {
    let mut turned_exactly = 0;
    for leaf in leaves {
        let turn = turned(leaf).expect("a turn");
        let exact = Application.raised(leaf).expect("a turned body");
        turned_exactly += usize::from(exact.is_exact());
        let flats = Flats::for_case(&Case::new(leaf.clone(), vec![]))
            .raised(leaf)
            .expect("a turn the flats make");
        if turn.has_a_wall_a_hair_thin() && exact.is_exact() {
            assert!(exact.faces_end() >= flats.faces_end(), "{leaf}");
        } else {
            assert_eq!(exact.faces_end(), flats.faces_end(), "{leaf}");
        }
        let Some((numbers, runs)) = laid_runs(&turn) else {
            assert!(
                turn.has_a_wall_a_hair_thin(),
                "{leaf}: a section of straight runs not laid"
            );
            continue;
        };
        let laid = |face: usize| numbers.contains(&(face as u32)) || face as u32 >= runs_of(&turn);
        for face in (0..flats.faces_end()).filter(|face| laid(*face)) {
            assert_eq!(
                exact.has_face(face),
                flats.has_face(face),
                "{leaf}: face {face}"
            );
        }
        let (_, reach) = lines_over(leaf);
        let swept = turn.swept(turn.section.side().expect("one side"));
        let (sine, cosine) = (turn.angle() / 2.0).sin_cos();
        let radial = swept.out * cosine + swept.onward * sine;
        let at = |place: DVec2| swept.origin + swept.along * place.x + radial * place.y;
        let drawn_to = TESSELLATION * reach;
        for run in runs
            .iter()
            .filter(|run| run.from.distance(run.to) > drawn_to)
        {
            let middle = (run.from + run.to) / 2.0;
            let way = (at(middle + run.out) - at(middle)).normalize();
            let hit = exact
                .ray_hit(at(middle) + way * drawn_to, -way)
                .unwrap_or_else(|| panic!("{leaf}: no face where run {} turns", run.run));
            let along = (run.to - run.from).normalize();
            let on_its_line = runs.iter().any(|other| {
                other.run as usize == hit.face
                    && [other.from, other.to]
                        .iter()
                        .all(|end| along.perp_dot(*end - run.from).abs() <= 1e-12 * reach)
            });
            let told_apart = hit.face >= flats.faces_end()
                && runs.iter().filter(|other| other.run == run.run).count() > 1;
            assert!(
                hit.face == run.run as usize || on_its_line || told_apart,
                "{leaf}: run {} turned into face {}",
                run.run,
                hit.face
            );
        }
    }
    turned_exactly
}

/// How many runs a section's outline and holes have.
fn runs_of(turn: &Turned) -> u32 {
    let (outline, holes) = turn.section.corners();
    (outline.len() + holes.iter().map(Vec::len).sum::<usize>()) as u32
}

#[test]
fn turned_cases_the_exact_kernel_keeps_are_kept_through_the_application_s_body_and_stay_exact() {
    let held = kept_through_the_application_s_body_and_exact(Case::drawn_turned, 0..11);
    assert!(held > 5, "{held} cases held");
}

/// How many of the cases `draw` gives `seeds` the exact kernel keeps with
/// no wall a hair thin, each of them asserted kept through the
/// application's body too, and on the exact kernel at every step there.
fn kept_through_the_application_s_body_and_exact(
    draw: fn(u64) -> Case,
    seeds: std::ops::Range<u64>,
) -> usize {
    let seeds: Vec<u64> = seeds.collect();
    let weighed = random_solids::on_every_core(&seeds, |seed| {
        let case = draw(*seed);
        random_solids::held_to_arithmetic(&case, &Exact)
            .ok()
            .filter(|kept| kept.thin == 0)?;
        let mut exact = true;
        if let Ok(leaves) = case
            .leaves()
            .map(|leaf| Application.raised(leaf))
            .collect::<Result<Vec<_>, _>>()
        {
            exact = leaves.iter().all(cao_solid::Body::is_exact);
            let mut body = leaves[0].clone();
            for (step, tool) in case.steps.iter().zip(&leaves[1..]) {
                match Application.combined(&body, tool, step.mode) {
                    Ok(next) => body = next,
                    Err(_) => break,
                }
                exact &= body.is_exact();
            }
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
        assert!(measured.is_ok(), "seed {seed}: {measured:?}\n{case}");
        assert!(exact, "seed {seed} left the exact kernel:\n{case}");
    }
    held
}

#[test]
fn a_case_asking_for_a_conic_is_declined_as_unsupported_or_held() {
    let asking_of = |draw: fn(u64) -> Case, count: usize| -> Vec<Case> {
        (0..2000)
            .map(draw)
            .filter(Case::asks_for_a_conic)
            .take(count)
            .collect()
    };
    let turned = asking_of(Case::drawn_turned_off_the_lattice, 24);
    let slanted = asking_of(Case::drawn_slanted_off_the_lattice, 16);
    assert!(turned.len() >= 20, "{} turned cases", turned.len());
    assert_eq!(slanted.len(), 16, "{} slanted cases", slanted.len());
    let asking: Vec<Case> = turned.into_iter().chain(slanted).collect();
    let weighed = random_solids::on_every_core(&asking, |case| {
        random_solids::held_to_arithmetic(case, &Exact)
    });
    let mut declined = 0;
    for (case, measured) in asking.iter().zip(weighed) {
        let measured = measured.unwrap_or_else(|flaw| panic!("{flaw:?}\n{case}"));
        declined += measured.declined;
    }
    println!("{declined} of {} declined as unsupported", asking.len());
}

#[test]
fn a_hole_a_hair_from_its_band_s_edge_opens_onto_it_on_the_exact_kernel() {
    let holed = |top: f64| {
        Leaf::turned(
            Plane::yz(1.0),
            Axis::second(0.0),
            Section::bands(5.0, &[[6.0, -10.0, -4.0]]).with_holes(&[([6.0, top], [10.5, -7.0])]),
            360.0,
        )
    };
    let (thin, thick) = (holed(-9.99999998), holed(-9.5));
    assert!(turned(&thin).expect("a turn").has_a_wall_a_hair_thin());
    assert!(!turned(&thick).expect("a turn").has_a_wall_a_hair_thin());
    let alone = Case::new(thin.clone(), vec![]);
    let measured = random_solids::held_to_arithmetic(&alone, &Exact).expect("held");
    assert_eq!(measured.thin, 0);
    let body = Application.raised(&thin).expect("a turned body");
    assert!(body.is_exact());
    random_solids::holds_through_the_application(&alone);
    let measured = random_solids::held_to_arithmetic(&Case::new(thick, vec![]), &Exact);
    assert_eq!(measured.map(|measured| measured.thin), Ok(0));
}

#[test]
fn random_turned_cases_keep_every_rule_on_the_exact_kernel() {
    keep_every_rule_on_the_exact_kernel(Case::drawn_turned, 0..40);
}

/// Whether every case `draw` gives `seeds` keeps every rule on the exact
/// kernel, held to the arithmetic.
fn keep_every_rule_on_the_exact_kernel(draw: fn(u64) -> Case, seeds: std::ops::Range<u64>) {
    let seeds: Vec<u64> = seeds.collect();
    let weighed = random_solids::on_every_core(&seeds, |seed| {
        let case = draw(*seed);
        (random_solids::held_to_arithmetic(&case, &Exact), case)
    });
    for (seed, (measured, case)) in seeds.iter().zip(weighed) {
        assert!(measured.is_ok(), "seed {seed}: {measured:?}\n{case}");
    }
}

/// The turned leaves whose section slopes, of the first cases of the
/// slanted draw.
fn drawn_slants(seeds: u64) -> Vec<Leaf> {
    (0..seeds)
        .flat_map(|seed| {
            Case::drawn_slanted(seed)
                .leaves()
                .cloned()
                .collect::<Vec<_>>()
        })
        .filter(|leaf| turned(leaf).is_some_and(|turn| turn.section.slopes()))
        .collect()
}

#[test]
fn every_sloped_leaf_raised_by_the_exact_kernel_alone_keeps_every_rule_and_encloses_pappus_s_volume()
 {
    let leaves = drawn_slants(80);
    assert!(leaves.len() > 30, "{} leaves", leaves.len());
    raised_alone_keep_every_rule_and_enclose_pappus_s_volume(&leaves);
}

#[test]
fn a_sloped_leaf_names_its_faces_as_the_flats_name_them() {
    let leaves: Vec<Leaf> = drawn_slants(120)
        .into_iter()
        .filter(|leaf| turned(leaf).is_some_and(|turn| turn.section.side().is_some()))
        .collect();
    assert!(leaves.len() > 40, "{} leaves", leaves.len());
    let exact = name_their_faces_as_the_flats_name_them(&leaves);
    assert_eq!(
        exact,
        leaves.len(),
        "every leaf is turned by the exact kernel"
    );
}

#[test]
fn sloped_cases_the_exact_kernel_keeps_are_kept_through_the_application_s_body_and_stay_exact() {
    let held = kept_through_the_application_s_body_and_exact(Case::drawn_slanted, 0..8);
    assert!(held > 3, "{held} cases held");
}

#[test]
fn random_slanted_cases_keep_every_rule_on_the_exact_kernel() {
    keep_every_rule_on_the_exact_kernel(Case::drawn_slanted, 0..16);
}

#[test]
fn the_draws_before_cones_still_give_each_seed_its_case() {
    assert_eq!(fingerprint(Case::drawn_turned), 0x588a_d4d5_4a36_cb30);
    assert_eq!(
        fingerprint(Case::drawn_turned_off_the_lattice),
        0x1557_617a_6b85_bd8e
    );
}

#[test]
fn the_same_seed_draws_the_same_slanted_case_and_not_one_of_the_other_draws() {
    for seed in 0..200 {
        assert_eq!(
            Case::drawn_slanted(seed),
            Case::drawn_slanted(seed),
            "seed {seed}"
        );
    }
    let distinct: std::collections::BTreeSet<String> = (0..200)
        .map(|seed| Case::drawn_slanted(seed).to_string())
        .collect();
    assert!(distinct.len() > 190, "{} distinct cases", distinct.len());
    let same = (0..200)
        .filter(|seed| {
            let slanted = Case::drawn_slanted(*seed);
            slanted == Case::drawn_turned(*seed)
                || slanted == Case::drawn_profiles(*seed)
                || slanted == Case::drawn_square(*seed)
        })
        .count();
    assert!(same < 20, "{same} slanted cases are another draw's");
}

#[test]
fn a_slanted_case_holds_profiles_of_straight_runs_turned_about_lines_of_the_planes_of_the_origin_and_prisms()
 {
    let mut sloped = 0;
    for seed in 0..3000 {
        let case = Case::drawn_slanted(seed);
        for leaf in case.leaves() {
            assert!(leaf.is_solid(), "seed {seed}: {case}");
            assert!(
                !matches!(leaf.plane(), Plane::Tilted { .. }),
                "seed {seed}: {case}"
            );
            match leaf {
                Leaf::Prism { outline, .. } => {
                    assert!(
                        !matches!(outline, Outline::Star { .. }),
                        "seed {seed}: {case}"
                    )
                }
                Leaf::Turned { axis, section, .. } => {
                    assert_eq!(axis.lean, 0.0, "seed {seed}: {case}");
                    sloped += usize::from(section.slopes());
                    for side in turned(leaf).expect("a turn").drawn() {
                        for corners in std::iter::once(&side.outline).chain(&side.holes) {
                            assert!(corners.len() >= 3, "seed {seed}: {case}");
                            for (index, corner) in corners.iter().enumerate() {
                                let run = corners[(index + 1) % corners.len()] - *corner;
                                assert!(
                                    run != DVec2::ZERO,
                                    "seed {seed}, a run of no length: {case}"
                                );
                            }
                        }
                    }
                }
                Leaf::Revolution { .. } => panic!("seed {seed} drew a revolution: {case}"),
            }
        }
    }
    assert!(sloped > 1000, "{sloped} sloped leaves");
}

/// A section read on the left of its axis: itself, or turned over when it
/// lies on the right.
fn on_the_left(section: &Section) -> Section {
    if section.side() != Some(-1.0) {
        return section.clone();
    }
    Section {
        from: section.from,
        bands: section
            .bands
            .iter()
            .map(|&[length, low, high]| [length, -high, -low])
            .collect(),
        sloping_to: section
            .sloping_to
            .iter()
            .map(|&[low, high]| [-high, -low])
            .collect(),
        holes: section
            .holes
            .iter()
            .map(|[low, high]| [DVec2::new(low.x, -high.y), DVec2::new(high.x, -low.y)])
            .collect(),
    }
}

/// The bands of a section with both of their ends: length, low and high
/// where it starts, low and high where it ends.
fn ends_of(section: &Section) -> Vec<[f64; 5]> {
    (0..section.bands.len())
        .map(|index| {
            let ([low, high], [low_end, high_end]) = section.edges(index);
            [section.bands[index][0], low, high, low_end, high_end]
        })
        .collect()
}

/// An edge of a section read about the world line its leaf turns about:
/// where it starts and where it ends, each a level along the line's world
/// axis and a radius.
type Edge = [DVec2; 2];

/// The edges of a turned leaf read about the world line it turns about: the
/// line, as a point of it and its world axis, its slanted edges and its
/// level ones.
struct Meridian {
    line: (DVec3, usize),
    sloped: Vec<Edge>,
    level: Vec<Edge>,
}

fn meridian(leaf: &Leaf) -> Option<Meridian> {
    let Leaf::Turned {
        plane,
        axis,
        section,
        ..
    } = leaf
    else {
        return None;
    };
    let line = world_line(leaf)?;
    let level = |along: f64| plane.to_world(axis.at(DVec2::new(along, 0.0)))[line.1];
    let ends = section.ends();
    let mut read = Meridian {
        line,
        sloped: Vec::new(),
        level: Vec::new(),
    };
    for index in 0..section.bands.len() {
        let (start, end) = section.edges(index);
        for edge in 0..2 {
            let run = [
                DVec2::new(level(ends[index]), start[edge].abs()),
                DVec2::new(level(ends[index + 1]), end[edge].abs()),
            ];
            if start[edge] == end[edge] {
                read.level.push(run);
            } else {
                read.sloped.push(run);
            }
        }
    }
    Some(read)
}

/// How far a place stands from the line through a run, as a share of how
/// far the run reaches.
fn off_the_line(run: [DVec2; 2], place: DVec2) -> f64 {
    let along = run[1] - run[0];
    along.perp_dot(place - run[0]).abs() / along.length() / run[1].abs().max_element().max(1.0)
}

#[test]
fn the_slanted_generator_draws_every_kind_it_keeps() {
    let cases: Vec<Case> = (0..1000).map(Case::drawn_slanted).collect();
    let leaves = || cases.iter().flat_map(Case::leaves);
    let turns = || leaves().filter_map(turned);
    let sloped = || turns().filter(|turn| turn.section.slopes());
    let left = || sloped().map(|turn| on_the_left(&turn.section));
    let seen = |what: &str, found: bool| assert!(found, "no slanted case drew {what}");
    let hair = |section: &Section| 1e-4 * section.reach();

    seen(
        "a chamfer whose slant lands on the wall past the step",
        left().any(|section| {
            let bands = ends_of(&section);
            (1..bands.len().saturating_sub(1)).any(|index| {
                let ([.., before], [_, low, high, low_end, high_end], [_, next_low, next, ..]) =
                    (bands[index - 1], bands[index], bands[index + 1]);
                low == low_end
                    && high != high_end
                    && before == high
                    && next == high_end
                    && next_low == low
                    && before != next
            })
        }),
    );
    seen(
        "a chamfer whose slant stops short of the wall past the step",
        left().any(|section| {
            let bands = ends_of(&section);
            (1..bands.len().saturating_sub(1)).any(|index| {
                let ([.., before], [_, low, high, low_end, high_end], [_, _, next, ..]) =
                    (bands[index - 1], bands[index], bands[index + 1]);
                low == low_end && high != high_end && (before == high) != (next == high_end)
            })
        }),
    );
    seen(
        "a point on the axis",
        left().any(|section| {
            let bands = ends_of(&section);
            let (first, last) = (bands[0], bands[bands.len() - 1]);
            (first[1] == 0.0 && first[2] == 0.0) || (last[3] == 0.0 && last[4] == 0.0)
        }),
    );
    seen(
        "a cone cut a hair short of its point",
        left().any(|section| {
            let bands = ends_of(&section);
            let (first, last) = (bands[0], bands[bands.len() - 1]);
            (first[1] == 0.0 && 0.0 < first[2] && first[2] < hair(&section))
                || (last[3] == 0.0 && 0.0 < last[4] && last[4] < hair(&section))
        }),
    );
    seen(
        "a knife's edge",
        left().any(|section| {
            let bands = ends_of(&section);
            let (first, last) = (bands[0], bands[bands.len() - 1]);
            (first[1] == first[2] && first[1] != 0.0) || (last[3] == last[4] && last[3] != 0.0)
        }),
    );
    seen(
        "a countersink",
        left().any(|section| {
            let bands = ends_of(&section);
            let last = bands[bands.len() - 1];
            last[1] == 0.0 && last[3] == 0.0 && last[4] > last[2]
        }),
    );
    seen(
        "a chamfered bore",
        left().any(|section| {
            let bands = ends_of(&section);
            let (first, last) = (bands[0], bands[bands.len() - 1]);
            (first[1] > first[3] && first[3] > 0.0) || (last[3] > last[1] && last[1] > 0.0)
        }),
    );
    seen(
        "two slants on one line",
        left().any(|section| {
            let bands = ends_of(&section);
            bands.windows(2).any(|pair| {
                let ([length, _, high, _, high_end], [next_length, _, next, _, next_end]) =
                    (pair[0], pair[1]);
                high != high_end
                    && high_end == next
                    && ((high_end - high) / length - (next_end - next) / next_length).abs()
                        <= 1e-9 * (high_end - high).abs() / length
            })
        }),
    );
    seen(
        "a slope a hair from parallel",
        left().any(|section| {
            ends_of(&section)
                .iter()
                .any(|band| band[2] != band[4] && (band[2] - band[4]).abs() < hair(&section))
        }),
    );
    seen(
        "a slope a hair from square",
        left().any(|section| {
            ends_of(&section)
                .iter()
                .any(|band| band[0] < hair(&section) && (band[2] - band[4]).abs() > 1e3 * band[0])
        }),
    );
    seen(
        "a ridge",
        left().any(|section| {
            ends_of(&section).windows(2).any(|pair| {
                let ([_, _, high, _, rim], [_, _, next, _, next_end]) = (pair[0], pair[1]);
                rim == next && high != rim && next_end != rim && (rim > high) == (rim > next_end)
            })
        }),
    );
    seen(
        "a sloped section across its axis",
        sloped().any(|turn| turn.section.side().is_none()),
    );
    seen(
        "a sloped section on the other side of its axis",
        sloped().any(|turn| turn.section.side() == Some(-1.0)),
    );
    seen(
        "a hole in a sloped section",
        sloped().any(|turn| !turn.section.holes.is_empty()),
    );
    seen(
        "a sloped section turned whole",
        sloped().any(|turn| turn.degrees.abs() == 360.0),
    );
    seen(
        "a sloped section turned part of the way",
        sloped().any(|turn| turn.degrees.abs() < 360.0),
    );
    seen(
        "a sloped section turned backwards",
        sloped().any(|turn| turn.degrees < 0.0),
    );
    let sloped_tool = |mode: Mode| {
        cases.iter().any(|case| {
            case.steps.iter().any(|step| {
                step.mode == mode && turned(&step.tool).is_some_and(|turn| turn.section.slopes())
            })
        })
    };
    seen("a sloped tool added", sloped_tool(Mode::Add));
    seen("a sloped tool cut", sloped_tool(Mode::Cut));

    let earlier_and_later = |related: &dyn Fn(&Leaf, &Leaf) -> bool| {
        cases.iter().any(|case| {
            let leaves: Vec<&Leaf> = case.leaves().collect();
            (1..leaves.len()).any(|later| {
                leaves[..later]
                    .iter()
                    .any(|earlier| related(earlier, leaves[later]))
            })
        })
    };
    seen(
        "a countersunk hole into a block",
        earlier_and_later(&|earlier, later| {
            let (
                Leaf::Prism {
                    plane,
                    outline: Outline::Rectangle { .. },
                    ..
                },
                Some(turn),
                Some((_, along)),
            ) = (earlier, turned(later), world_line(later))
            else {
                return false;
            };
            let bands = ends_of(&on_the_left(&turn.section));
            along == plane.normal().abs().max_position()
                && bands.iter().any(|band| band[1] == 0.0 && band[4] > band[2])
                && bands
                    .iter()
                    .any(|band| band[1] == 0.0 && band[2] == band[4])
        }),
    );
    let on_a_circle = |earlier: &Leaf, later: &Leaf, kind: &dyn Fn(f64, [f64; 5]) -> bool| {
        let (Some(radius), Some(turn), Some(one), Some(other)) = (
            radius_of(earlier),
            turned(later),
            world_line(earlier),
            world_line(later),
        ) else {
            return false;
        };
        same_line(one, other)
            && ends_of(&on_the_left(&turn.section))
                .iter()
                .any(|band| kind(radius, *band))
    };
    seen(
        "a chamfer cut round a raised circle",
        earlier_and_later(&|earlier, later| {
            on_a_circle(earlier, later, &|radius, band| {
                band[1] == radius && band[3] < radius
            })
        }),
    );
    seen(
        "a cone standing on a raised circle",
        earlier_and_later(&|earlier, later| {
            on_a_circle(earlier, later, &|radius, band| {
                band[1] == 0.0 && band[2] == radius && band[4] < radius
            })
        }),
    );
    let after_a_slant = |related: &dyn Fn(&[Edge], &Meridian) -> bool| {
        earlier_and_later(&|earlier, later| {
            let (Some(earlier), Some(later)) = (meridian(earlier), meridian(later)) else {
                return false;
            };
            same_line(earlier.line, later.line) && related(&earlier.sloped, &later)
        })
    };
    seen(
        "the same cone again",
        after_a_slant(&|slants, later| {
            slants.iter().any(|slant| {
                later
                    .sloped
                    .iter()
                    .any(|other| other.iter().all(|end| off_the_line(*slant, *end) <= 1e-12))
            })
        }),
    );
    seen(
        "a cylinder at a cone's rim",
        after_a_slant(&|slants, later| {
            slants.iter().flatten().any(|rim| {
                rim.y > 0.0
                    && later
                        .level
                        .iter()
                        .any(|edge| edge[0].y == rim.y && edge.iter().any(|end| end.x == rim.x))
            })
        }),
    );
    seen(
        "a cone crossing a cone at its rim",
        after_a_slant(&|slants, later| {
            slants.iter().any(|slant| {
                later.sloped.iter().any(|other| {
                    slant.iter().any(|rim| off_the_line(*other, *rim) <= 1e-12)
                        && other.iter().any(|end| off_the_line(*slant, *end) > 1e-3)
                })
            })
        }),
    );
    seen(
        "a slant a hair off a slant",
        after_a_slant(&|slants, later| {
            slants.iter().any(|slant| {
                later.sloped.iter().any(|other| {
                    other.iter().all(|end| {
                        let off = off_the_line(*slant, *end);
                        0.0 < off && off < 1e-4
                    })
                })
            })
        }),
    );
    seen(
        "a circle on a cone's rim",
        earlier_and_later(&|earlier, later| {
            let (Some(earlier), Some(other), Leaf::Prism { plane, outline, .. }) =
                (meridian(earlier), world_line(later), later)
            else {
                return false;
            };
            let radii = match outline {
                Outline::Circle { radius, .. } => vec![*radius],
                Outline::Ring { outer, inner, .. } => vec![*outer, *inner],
                _ => return false,
            };
            let level = plane.to_world(DVec2::ZERO)[earlier.line.1];
            same_line(earlier.line, other)
                && earlier
                    .sloped
                    .iter()
                    .flatten()
                    .any(|rim| rim.x == level && radii.contains(&rim.y))
        }),
    );

    let share = sloped().count() as f64 / leaves().count() as f64;
    assert!(
        (0.2..0.5).contains(&share),
        "slanted leaves are {} of {}",
        sloped().count(),
        leaves().count()
    );
}

#[test]
fn a_sloped_section_runs_its_corners_along_its_slopes_and_closes_at_a_point() {
    let point = Section::bands(0.0, &[[3.0, 0.0, 2.0], [2.0, 0.0, 2.0]])
        .sloping_to(&[[0.0, 2.0], [0.0, 0.0]]);
    assert!(point.is_solid());
    let (outline, holes) = point.corners();
    assert_eq!(
        outline,
        [[0.0, 0.0], [3.0, 0.0], [5.0, 0.0], [3.0, 2.0], [0.0, 2.0]].map(DVec2::from)
    );
    assert!(holes.is_empty());

    let knife = Section::bands(0.0, &[[1.0, 2.0, 2.0], [3.0, 1.0, 2.0]])
        .sloping_to(&[[1.0, 2.0], [1.0, 2.0]]);
    assert!(knife.is_solid());
    assert_eq!(
        knife.corners().0,
        [[0.0, 2.0], [1.0, 1.0], [4.0, 1.0], [4.0, 2.0], [1.0, 2.0]].map(DVec2::from)
    );

    let level = Section::bands(1.0, &[[2.0, 1.0, 3.0]]);
    assert_eq!(level.clone().sloping_to(&[[1.0, 3.0]]), level);
    assert!(!level.slopes());
    assert!(point.slopes());

    let holed = Section::bands(0.0, &[[6.0, 1.0, 5.0]])
        .sloping_to(&[[1.0, 8.0]])
        .with_holes(&[([1.0, 2.0], [3.0, 5.4])]);
    assert!(holed.is_solid());
    assert_eq!(
        holed.trapezoids(),
        vec![
            Trapezoid {
                along: [0.0, 1.0],
                low: [1.0, 1.0],
                high: [5.0, 5.5],
            },
            Trapezoid {
                along: [3.0, 6.0],
                low: [1.0, 1.0],
                high: [6.5, 8.0],
            },
            Trapezoid {
                along: [1.0, 3.0],
                low: [1.0, 1.0],
                high: [2.0, 2.0],
            },
            Trapezoid {
                along: [1.0, 3.0],
                low: [5.4, 5.4],
                high: [5.5, 6.5],
            },
        ]
    );
    let through = Section::bands(0.0, &[[6.0, 1.0, 5.0]])
        .sloping_to(&[[1.0, 8.0]])
        .with_holes(&[([1.0, 2.0], [3.0, 6.0])]);
    assert!(!through.is_solid(), "a hole past the slope where it starts");

    let pieces = point.pieces();
    assert_eq!(
        pieces[1],
        Piece {
            along: [3.0, 5.0],
            away: [0.0, 2.0],
            ending: [0.0, 0.0],
            side: 1.0,
        }
    );
}

#[test]
fn a_sloped_section_across_its_axis_is_handed_over_as_its_two_sides() {
    let section = Section::bands(0.0, &[[2.0, -4.0, 5.0], [1.0, -1.0, 3.0]])
        .sloping_to(&[[-2.0, 5.0], [-1.0, 1.0]])
        .with_holes(&[([0.5, 1.0], [1.5, 2.0])]);
    assert_eq!(section.side(), None);
    let sides = section.sides();
    assert_eq!(
        sides,
        vec![
            Section::bands(0.0, &[[2.0, 0.0, 5.0], [1.0, 0.0, 3.0]])
                .sloping_to(&[[0.0, 5.0], [0.0, 1.0]])
                .with_holes(&[([0.5, 1.0], [1.5, 2.0])]),
            Section::bands(0.0, &[[2.0, -4.0, 0.0], [1.0, -1.0, 0.0]])
                .sloping_to(&[[-2.0, 0.0], [-1.0, 0.0]]),
        ]
    );
    let measure = |pieces: Vec<Piece>, side: f64| {
        pieces
            .iter()
            .filter(|piece| piece.side == side)
            .map(|piece| {
                let length = piece.along[1] - piece.along[0];
                let width = (piece.away[1] - piece.away[0]) + (piece.ending[1] - piece.ending[0]);
                DVec2::new(length * width / 2.0, piece.swept())
            })
            .sum::<DVec2>()
    };
    for side in [1.0, -1.0] {
        let halves = sides
            .iter()
            .map(|half| measure(half.pieces(), side))
            .sum::<DVec2>();
        let whole = measure(section.pieces(), side);
        assert!(
            (halves - whole).abs().max_element() <= 1e-12 * whole.max_element(),
            "side {side}: {halves} against {whole}"
        );
    }

    let across: Vec<Turned> = drawn_slants(1000)
        .iter()
        .filter_map(turned)
        .filter(|turn| turn.section.side().is_none())
        .collect();
    assert!(across.len() > 10, "{} sloped sections across", across.len());
    for turn in across {
        let drawn = turn.drawn();
        assert_eq!(drawn.len(), 2, "{:?}", turn.section);
        let line = turn.turn(0.0);
        for (side, expected) in drawn.iter().zip([1.0, -1.0]) {
            assert_eq!(
                line.lie(&side.profile(true)),
                Lie::Side(expected),
                "{:?}",
                turn.section
            );
        }
        for side in turn.section.sides() {
            assert!(side.is_solid(), "{side:?} of {:?}", turn.section);
        }
    }
}

#[test]
fn a_sloped_section_pinched_on_its_axis_is_not_solid() {
    let leaving = Section::bands(0.0, &[[3.0, 0.0, 4.0]]).sloping_to(&[[2.0, 4.0]]);
    assert!(!leaving.is_solid(), "a corner on the axis two slants leave");
    let valley = Section::bands(0.0, &[[2.0, 2.0, 4.0], [2.0, 0.0, 4.0]])
        .sloping_to(&[[0.0, 4.0], [2.0, 4.0]]);
    assert!(
        !valley.is_solid(),
        "a bore narrowing to the axis and out again"
    );
    let point = Section::bands(0.0, &[[3.0, 0.0, 4.0]]).sloping_to(&[[0.0, 0.0]]);
    assert!(point.is_solid(), "a point whose low run lies on the axis");
    let knife = Section::bands(0.0, &[[2.0, 0.0, 4.0], [2.0, 0.0, 4.0]])
        .sloping_to(&[[0.0, 0.0], [0.0, 4.0]]);
    assert!(!knife.is_solid(), "two bands meeting at a point");
    let crossing = Section::bands(0.0, &[[2.0, -2.0, 3.0]]).sloping_to(&[[1.0, 3.0]]);
    assert!(!crossing.is_solid(), "a slant crossing the axis");
    let across = Section::bands(0.0, &[[2.0, -2.0, 3.0]]).sloping_to(&[[-1.0, 3.0]]);
    assert!(across.is_solid());

    for leaf in drawn_slants(1000) {
        let turn = turned(&leaf).expect("a turn");
        let snap = 1e-5 * turn.section.reach();
        let (outline, _) = turn.section.corners();
        for (index, corner) in outline.iter().enumerate() {
            let before = outline[(index + outline.len() - 1) % outline.len()];
            let after = outline[(index + 1) % outline.len()];
            assert!(
                corner.y.abs() > snap || before.y.abs() <= snap || after.y.abs() <= snap,
                "{leaf}: pinched at {corner}"
            );
        }
    }
}

#[test]
fn the_trapezoids_of_a_sloped_section_turn_into_pappus_s_volume_of_its_corners() {
    let volume = |section: Section, degrees: f64| {
        turned(&Leaf::turned(
            Plane::xz(0.0),
            Axis::second(0.0),
            section,
            degrees,
        ))
        .expect("a turn")
        .volume()
    };
    let point = Section::bands(0.0, &[[10.0, 0.0, 5.0]]).sloping_to(&[[0.0, 0.0]]);
    let chamfered = Section::bands(0.0, &[[28.0, 0.0, 10.0], [2.0, 0.0, 10.0]])
        .sloping_to(&[[0.0, 10.0], [0.0, 8.0]]);
    let partial = Section::bands(0.0, &[[4.0, 2.0, 5.0]]).sloping_to(&[[2.0, 4.0]]);
    let across = Section::bands(0.0, &[[2.0, -4.0, 5.0]]).sloping_to(&[[-2.0, 3.0]]);
    let crossing = Section::bands(0.0, &[[2.0, -4.0, 3.0]]).sloping_to(&[[-2.0, 5.0]]);
    for (what, found, expected) in [
        ("a point", volume(point, 360.0), 250.0 * PI / 3.0),
        (
            "a chamfered shaft",
            volume(chamfered, 360.0),
            8888.0 * PI / 3.0,
        ),
        (
            "a quarter turn",
            volume(partial.clone(), 90.0),
            49.0 * PI / 3.0,
        ),
        (
            "a quarter turn back",
            volume(partial, -90.0),
            49.0 * PI / 3.0,
        ),
        ("across, whole", volume(across, 360.0), 98.0 * PI / 3.0),
        ("radii crossing", volume(crossing, 360.0), 103.25 * PI / 3.0),
    ] {
        assert!(
            (found - expected).abs() <= 1e-12 * expected,
            "{what}: {found} against {expected}"
        );
    }

    let turns: Vec<Turned> = drawn_slants(1000).iter().filter_map(turned).collect();
    assert!(turns.len() > 300, "{} turns", turns.len());
    for turn in turns {
        let sides = moment(&turn);
        let pappus: f64 = sides.iter().map(|side| side * turn.angle()).sum();
        let volume = turn.volume();
        let rounding = 1e-12 * turn.angle() * turn.section.reach().powi(3);
        if sides.len() == 1 || turn.degrees.abs() <= 180.0 {
            assert!(
                (volume - pappus).abs() <= 1e-9 * pappus + rounding,
                "{:?}: {volume} against {pappus}",
                turn.section
            );
        } else {
            let largest = sides.iter().copied().fold(0.0, f64::max) * turn.angle();
            assert!(
                largest <= volume * (1.0 + 1e-9) && volume <= pappus * (1.0 + 1e-9),
                "{:?}: {volume} between {largest} and {pappus}",
                turn.section
            );
        }
    }
}

/// The ends of the stretches a line holds, where it crosses there, how
/// squarely and whether what it crosses curves.
fn crossings_of(stretches: &[Stretch]) -> Vec<(f64, f64, bool)> {
    stretches
        .iter()
        .flat_map(|stretch| [stretch.from, stretch.to])
        .map(|end| (end.at, end.cosine, end.curved))
        .collect()
}

fn close(found: &[(f64, f64, bool)], expected: &[(f64, f64, bool)]) -> bool {
    found.len() == expected.len()
        && found.iter().zip(expected).all(|(one, other)| {
            (one.0 - other.0).abs() < 1e-12 && (one.1 - other.1).abs() < 1e-12 && one.2 == other.2
        })
}

#[test]
fn a_line_through_a_cone_is_told_where_it_crosses_how_squarely_and_that_it_curves() {
    let point = Leaf::turned(
        Plane::xy(0.0),
        Axis::first(0.0),
        Section::bands(0.0, &[[4.0, 0.0, 4.0]]).sloping_to(&[[0.0, 0.0]]),
        360.0,
    );
    let along = |origin: [f64; 3], direction: [f64; 3]| {
        crossings_of(
            &point
                .along(DVec3::from(origin), DVec3::from(direction))
                .expect("a turn"),
        )
    };
    let half = 0.5f64.sqrt();

    let axis = along([-1.0, 0.0, 0.0], [1.0, 0.0, 0.0]);
    assert!(
        close(&axis, &[(1.0, 1.0, false), (5.0, 0.0, true)]),
        "{axis:?}"
    );
    let square = along([1.0, 0.0, -5.0], [0.0, 0.0, 1.0]);
    assert!(
        close(&square, &[(2.0, half, true), (8.0, half, true)]),
        "{square:?}"
    );
    let ruling = along([0.0, 2.0, 0.0], [1.0, -1.0, 0.0]);
    assert!(
        close(&ruling, &[(0.0, half, false), (3.0, 1.0, true)]),
        "{ruling:?}"
    );
    let tip = along([6.0, -1.0, 0.0], [-2.0, 1.0, 0.0]);
    assert!(
        close(&tip, &[(1.0, 0.0, true), (3.0, 2.0 / 5f64.sqrt(), false)]),
        "{tip:?}"
    );
    let past = along([6.0, 1.0, 0.0], [-1.0, 0.0, 0.0]);
    assert!(
        close(&past, &[(3.0, half, true), (6.0, 1.0, false)]),
        "{past:?}"
    );
    assert!(along([5.0, 0.0, -5.0], [0.0, 0.0, 1.0]).is_empty());

    let bored = Leaf::turned(
        Plane::xy(0.0),
        Axis::first(0.0),
        Section::bands(0.0, &[[2.0, 1.0, 4.0]]).sloping_to(&[[3.0, 4.0]]),
        360.0,
    );
    let through = crossings_of(
        &bored
            .along(DVec3::new(1.0, 0.0, -5.0), DVec3::Z)
            .expect("a turn"),
    );
    assert!(
        close(
            &through,
            &[
                (1.0, 1.0, true),
                (3.0, half, true),
                (7.0, half, true),
                (9.0, 1.0, true)
            ]
        ),
        "{through:?}"
    );
}

/// The pieces of a section, each sloped one cut into `steps` level ones
/// along it: inside its walls when `inside`, around them otherwise.
fn stairs(pieces: &[Piece], steps: usize, inside: bool) -> Vec<Piece> {
    pieces
        .iter()
        .flat_map(|piece| {
            if piece.is_level() {
                return vec![*piece];
            }
            let length = piece.along[1] - piece.along[0];
            (0..steps)
                .map(|step| {
                    let along = [step, step + 1]
                        .map(|end| piece.along[0] + length * end as f64 / steps as f64);
                    let [start, end] = along.map(|at| piece.at(at));
                    let away = if inside {
                        [start[0].max(end[0]), start[1].min(end[1])]
                    } else {
                        [start[0].min(end[0]), start[1].max(end[1])]
                    };
                    Piece {
                        along,
                        away,
                        ending: away,
                        side: piece.side,
                    }
                })
                .filter(|piece| piece.away[0] < piece.away[1])
                .collect()
        })
        .collect()
}

#[test]
fn a_cone_holds_along_every_line_what_rings_stacked_inside_it_hold_and_no_more_than_rings_stacked_around_it()
 {
    let leaves: Vec<Leaf> = drawn_slants(60).into_iter().take(40).collect();
    assert!(leaves.len() >= 30, "{} leaves", leaves.len());
    for leaf in &leaves {
        let turn = turned(leaf).expect("a turn");
        let pieces = turn.section.pieces();
        let (inside, around) = (stairs(&pieces, 16, true), stairs(&pieces, 16, false));
        let (lines, reach) = lines_over(leaf);
        let room = 1e-9 * reach;
        for index in 0..lines.count() {
            let (origin, direction) = lines.line(index);
            let [held, least, most] = [&pieces, &inside, &around]
                .map(|pieces| spans_of(&turn.along_pieces(pieces, origin, direction, 0.0)));
            assert!(
                least.without(&held).length() <= room && held.without(&most).length() <= room,
                "line {index} across {leaf}: {held:?} not between {least:?} and {most:?}"
            );
        }
    }
}

#[test]
fn a_cone_a_hair_from_a_cylinder_holds_along_every_line_what_the_cylinder_holds_but_for_the_hair() {
    for hair in [1e-7, 1e-9, 3e-6] {
        for degrees in [360.0, 90.0, -270.0] {
            let leaf = |section: Section| {
                Leaf::turned(Plane::xz(1.0), Axis::second(2.0), section, degrees)
            };
            let cylinder = leaf(Section::bands(-1.0, &[[4.0, 1.0, 3.0]]));
            let cone =
                leaf(Section::bands(-1.0, &[[4.0, 1.0, 3.0]]).sloping_to(&[[1.0, 3.0 + hair]]));
            let (lines, reach) = lines_over(&cone);
            for index in 0..lines.count() {
                let (origin, direction) = lines.line(index);
                let spans = |leaf: &Leaf, by: f64| {
                    spans_of(&leaf.along_grown(origin, direction, by).expect("a turn"))
                };
                let (held, inner, outer) = (
                    spans(&cone, 0.0),
                    spans(&cylinder, 0.0),
                    spans(&cylinder, hair),
                );
                assert!(
                    inner.without(&held).length() <= 1e-12 * reach
                        && held.without(&outer).length() <= 1e-12 * reach,
                    "line {index} of {degrees} at a hair of {hair}"
                );
            }
        }
    }
}

#[test]
fn a_sloped_leaf_grown_holds_every_line_it_held_and_shrunk_holds_none_it_did_not() {
    for leaf in drawn_slants(120) {
        let (lines, reach) = lines_over(&leaf);
        let by = 1e-3 * reach;
        for index in 0..lines.count() {
            let (origin, direction) = lines.line(index);
            let [shrunk, held, grown] = [-by, 0.0, by]
                .map(|by| spans_of(&leaf.along_grown(origin, direction, by).expect("a turn")));
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
fn a_sloped_turn_backwards_holds_along_every_line_what_the_turn_forwards_holds_along_the_line_turned_back()
 {
    let leaves: Vec<Leaf> = drawn_slants(200)
        .into_iter()
        .filter(|leaf| turned(leaf).is_some_and(|turn| turn.degrees > 0.0))
        .collect();
    assert!(leaves.len() > 60, "{} leaves", leaves.len());
    for leaf in &leaves {
        let forwards = turned(leaf).expect("a turn");
        let backwards = Turned {
            degrees: -forwards.degrees,
            ..forwards.clone()
        };
        let (lines, reach) = lines_over(leaf);
        for index in 0..lines.count() {
            let line = lines.line(index);
            let (origin, direction) = turned_line(&forwards, forwards.degrees.to_radians(), line);
            let back = spans_of(&backwards.along_grown(line.0, line.1, 0.0));
            let forth = spans_of(&forwards.along_grown(origin, direction, 0.0));
            assert!(
                back.without(&forth).length() + forth.without(&back).length() <= 1e-9 * reach,
                "line {index} across {leaf}: {back:?} against {forth:?}"
            );
        }
    }
}

#[test]
fn a_sloped_leaf_off_its_axis_holds_along_every_line_and_spans_the_box_its_flats_hold_but_for_their_sagitta()
 {
    let leaves = turns_the_flats_hold(drawn_slants(300).into_iter().take(160).collect());
    assert!(leaves.len() > 20, "{} leaves", leaves.len());
    for leaf in &leaves {
        let flats = leaf.solid().expect("a turn the flats make");
        holds_what_its_flats_hold(leaf, &flats);
        spans_the_box_its_flats_span(leaf, &flats);
    }
}

#[test]
fn a_slanted_case_prints_as_the_rust_that_builds_it_again() {
    let pasted = Case::new(
        Leaf::turned(
            Plane::xz(2.0),
            Axis::second(0.0).backwards(),
            Section::bands(0.0, &[[3.0, 0.0, 2.0], [1.5, 0.0, 1.0]])
                .sloping_to(&[[0.0, 2.0], [0.0, 0.0]])
                .with_holes(&[([0.5, 0.5], [1.0, 1.5])]),
            -90.0,
        ),
        vec![Step::cut(Leaf::turned(
            Plane::xy(-1.0),
            Axis::first(2.5),
            Section::bands(-1.0, &[[2.0, -1.0, 3.0]]).sloping_to(&[[-1.5, 2.5]]),
            360.0,
        ))],
    );
    assert_eq!(
        pasted.to_string(),
        "Case::new(
    Leaf::turned(Plane::xz(2.0), Axis::second(0.0).backwards(), Section::bands(0.0, &[[3.0, 0.0, 2.0], [1.5, 0.0, 1.0]]).sloping_to(&[[0.0, 2.0], [0.0, 0.0]]).with_holes(&[([0.5, 0.5], [1.0, 1.5])]), -90.0),
    vec![
        Step::cut(Leaf::turned(Plane::xy(-1.0), Axis::first(2.5), Section::bands(-1.0, &[[2.0, -1.0, 3.0]]).sloping_to(&[[-1.5, 2.5]]), 360.0)),
    ],
)"
    );
    for seed in 0..200 {
        for case in [
            Case::drawn_slanted(seed),
            Case::drawn_slanted_off_the_lattice(seed),
        ] {
            let printed = case.to_string();
            assert!(
                !printed.contains("NaN") && !printed.contains("inf"),
                "seed {seed} prints a number Rust cannot read back: {printed}"
            );
        }
    }
}

#[test]
fn a_sloped_leaf_shrinks_into_level_bands_and_round_numbers() {
    let leaf = |section: Section| Leaf::turned(Plane::xy(0.0), Axis::first(0.0), section, 360.0);
    let sloped = Section::bands(0.3, &[[2.2, 0.0, 3.4], [1.0, 0.0, 2.0]])
        .sloping_to(&[[0.0, 2.7], [0.0, 0.2]]);
    let shrunk: Vec<Leaf> = Case::new(leaf(sloped.clone()), vec![])
        .smaller()
        .into_iter()
        .map(|case| case.start)
        .collect();
    let expected = [
        Section::bands(0.3, &[[2.2, 0.0, 3.4], [1.0, 0.0, 2.0]])
            .sloping_to(&[[0.0, 3.4], [0.0, 0.2]]),
        Section::bands(0.3, &[[2.2, 0.0, 3.4], [1.0, 0.0, 2.0]])
            .sloping_to(&[[0.0, 2.7], [0.0, 2.0]]),
        Section::bands(0.3, &[[2.2, 0.0, 3.4], [1.0, 0.0, 2.0]]),
        Section::bands(0.3, &[[2.2, 0.0, 3.4]]).sloping_to(&[[0.0, 2.7]]),
        Section::bands(0.3, &[[1.0, 0.0, 2.0]]).sloping_to(&[[0.0, 0.2]]),
        Section::bands(0.3, &[[3.2, 0.0, 3.4]]),
        Section::bands(0.0, &[[2.0, 0.0, 3.0], [1.0, 0.0, 2.0]])
            .sloping_to(&[[0.0, 3.0], [0.0, 0.0]]),
        Section::bands(0.5, &[[2.0, 0.0, 3.5], [1.0, 0.0, 2.0]])
            .sloping_to(&[[0.0, 2.5], [0.0, 0.0]]),
    ];
    for expected in expected {
        let expected = leaf(expected);
        assert!(shrunk.contains(&expected), "{expected} among {shrunk:#?}");
    }
    let level = Section::bands(0.0, &[[2.0, 0.0, 3.0]]);
    assert!(
        Case::new(leaf(level), vec![])
            .smaller()
            .iter()
            .all(|case| turned(&case.start).is_none_or(|turn| !turn.section.slopes())),
        "a level section shrinks into a sloped one"
    );
}

#[test]
fn shrinking_any_drawn_slanted_case_comes_to_an_end() {
    for seed in 0..200 {
        let mut rounds = 0;
        let shrunk = shrink(
            Case::drawn_slanted_off_the_lattice(seed),
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

/// The campaigns, compiled only when asked for — `--features campaigns` —
/// so that work on any other issue never pays for them.
#[cfg(feature = "campaigns")]
mod campaign {
    use std::sync::Arc;
    use std::sync::atomic::Ordering;

    use cao_solid::soundness::{Check, Flaw};
    use random_solids::campaigning::{
        Answering, DECLINED, GRAZING, HELD, THIN, TIPS, campaign_over, shrunk_one_by_one,
    };

    use super::*;

    fn tallied(measured: random_solids::Measured) {
        HELD.fetch_add(measured.held, Ordering::Relaxed);
        GRAZING.fetch_add(measured.grazing, Ordering::Relaxed);
        TIPS.fetch_add(measured.through_a_tip, Ordering::Relaxed);
        DECLINED.fetch_add(measured.declined, Ordering::Relaxed);
        THIN.fetch_add(measured.thin, Ordering::Relaxed);
    }

    fn exactly(case: &Case) -> Result<(), Flaw> {
        tallied(random_solids::held_to_arithmetic(case, &Exact)?);
        Ok(())
    }

    fn through_the_application(case: &Case) -> Result<(), Flaw> {
        tallied(random_solids::held_to_arithmetic(case, &Application)?);
        Ok(())
    }

    /// The case a seed stands for, with the seed written where a campaign
    /// that ends the program still leaves it to be read.
    fn drawn(seed: u64) -> Case {
        eprint!("\rseed {seed} ");
        Case::drawn_turned_off_the_lattice(seed)
    }

    /// The same, among slanted turns (#536).
    fn drawn_slanted(seed: u64) -> Case {
        eprint!("\rseed {seed} ");
        Case::drawn_slanted_off_the_lattice(seed)
    }

    #[test]
    #[ignore = "a campaign, run by hand: see the head of this file"]
    fn a_campaign_of_turned_solids_on_the_exact_kernel_keeps_every_rule() {
        campaign_over("turned solids", Answering::Exactly, drawn, exactly);
    }

    #[test]
    #[ignore = "a campaign, run by hand: see the head of this file"]
    fn a_campaign_of_turned_solids_through_the_application_s_body_keeps_every_rule() {
        campaign_over(
            "turned solids",
            Answering::ThroughTheApplication,
            drawn,
            through_the_application,
        );
    }

    #[test]
    #[ignore = "a campaign, run by hand: see the head of this file"]
    fn a_campaign_of_slanted_solids_on_the_exact_kernel_keeps_every_rule() {
        campaign_over("slanted solids", Answering::Exactly, drawn_slanted, exactly);
    }

    #[test]
    #[ignore = "a campaign, run by hand: see the head of this file"]
    fn a_campaign_of_slanted_solids_through_the_application_s_body_keeps_every_rule() {
        campaign_over(
            "slanted solids",
            Answering::ThroughTheApplication,
            drawn_slanted,
            through_the_application,
        );
    }

    /// Each seed named in `CAO_TRIAGE_SEEDS`, commas between them, drawn as
    /// a turning campaign draws it — as a slanted one when `CAO_TRIAGE_DRAW`
    /// is `slanted` — run again on the exact kernel — through the
    /// application's body when `CAO_TRIAGE_KERNEL` is `application` —
    /// shrunk while it still breaks the same rule, and printed as a test.
    #[test]
    #[ignore = "run by hand on the seeds a campaign named"]
    fn the_seeds_a_turning_campaign_named_are_shrunk_one_by_one() {
        let seeds: Vec<u64> = std::env::var("CAO_TRIAGE_SEEDS")
            .unwrap_or_default()
            .split(',')
            .filter_map(|seed| seed.trim().parse().ok())
            .collect();
        let check: Check<Case> = match std::env::var("CAO_TRIAGE_KERNEL").as_deref() {
            Ok("application") => Arc::new(through_the_application),
            _ => Arc::new(exactly),
        };
        let draw: fn(u64) -> Case = match std::env::var("CAO_TRIAGE_DRAW").as_deref() {
            Ok("slanted") => Case::drawn_slanted_off_the_lattice,
            _ => Case::drawn_turned_off_the_lattice,
        };
        shrunk_one_by_one(seeds, draw, check);
    }

    /// How many of the cases `draw` gives the first 300 seeds the flats
    /// answer by their own rules, each of them asserted answered by the
    /// arithmetic too.
    fn held_by_the_flats_and_the_arithmetic(draw: fn(u64) -> Case) -> usize {
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
            assert!(measured.is_ok(), "seed {seed}: {measured:?}\n{case}");
        }
        println!("{held} cases of {} held", seeds.len());
        held
    }

    /// The harness held to the flats, where whole cases of turns are too slow
    /// and too often broken for the gate (#418, #486, #492): every case the
    /// flats answer by their own rules is answered by the arithmetic too.
    #[test]
    #[ignore = "a campaign, run by hand once: see the head of this file"]
    fn turned_cases_the_flats_answer_by_their_own_rules_keep_every_rule_held_to_the_arithmetic() {
        let held = held_by_the_flats_and_the_arithmetic(Case::drawn_turned);
        assert!(held > 100, "{held} cases held");
    }

    /// The cone arithmetic held to the flats before the exact kernel turns a
    /// cone (#536): the flats were written long before it, and owe it
    /// nothing.
    #[test]
    #[ignore = "a campaign, run by hand once: see the head of this file"]
    fn slanted_cases_the_flats_answer_by_their_own_rules_keep_every_rule_held_to_the_arithmetic() {
        let held = held_by_the_flats_and_the_arithmetic(Case::drawn_slanted);
        assert!(held > 100, "{held} cases held");
    }
}
