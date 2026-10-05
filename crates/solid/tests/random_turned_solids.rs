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
//! The tests that need the exact kernel to turn are written already and wait
//! for it, ignored. A campaign is run by hand, on the exact kernel or through
//! the application's body:
//!
//! ```text
//! CAO_FUZZ_SECONDS=3600 cargo test --release -p cao_solid --features campaigns \
//!     --test random_turned_solids -- --ignored --nocapture \
//!     a_campaign_of_turned_solids_on_the_exact_kernel
//! ```
//!
//! It draws past the gate's lattice (`Case::drawn_turned_off_the_lattice`):
//! angles a hair from half a turn, a quarter and a whole one, and axes
//! slanted or leaning a hair, where a case asking for an ellipse may be
//! declined as unsupported and is counted so. `CAO_FUZZ_SEED`,
//! `CAO_FUZZ_CASES` and `CAO_FUZZ_PATIENCE` are read as in
//! `random_exact_solids.rs`; the seeds a campaign names are shrunk by
//! `the_seeds_a_turning_campaign_named_are_shrunk_one_by_one`, given in
//! `CAO_TRIAGE_SEEDS`, with `CAO_TRIAGE_KERNEL=application` for the
//! campaign through the application's body.

// The drawing, the promise and the checks are shared with the other
// campaigns; each file uses its own part of them.
#[allow(dead_code, unused_imports)]
mod random_solids;

use std::f64::consts::PI;

use cao_solid::soundness::{Lines, Spans, shrink};
use cao_solid::turning::Lie;
use cao_solid::turning::Straight;
use glam::{DQuat, DVec2, DVec3};
use random_solids::{
    Application, Axis, Case, Drawn, Exact, Flats, Kernel, Leaf, Outline, Piece, Plane, Section,
    Step, Stretch, Turned,
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
/// first moment of the area about the axis, read off the corners alone.
fn moment(turn: &Turned) -> Vec<f64> {
    let line = turn.axis.line();
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
                                line.side(*corner),
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

/// The lone turns of the draw that the flats answer by their own rules,
/// turned forwards and off their axis: where the flats can be trusted with
/// a turn.
fn turns_the_flats_hold() -> Vec<Leaf> {
    let leaves: Vec<Leaf> = drawn_turns(400)
        .into_iter()
        .filter(|leaf| {
            turned(leaf).is_some_and(|turn| {
                turn.degrees > 0.0
                    && turn.section.side().is_some()
                    && turn
                        .section
                        .pieces()
                        .iter()
                        .all(|piece| piece.away[0] > 0.0)
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

#[test]
fn a_turned_leaf_off_its_axis_holds_along_every_line_what_its_flats_hold_but_for_their_sagitta() {
    let leaves = turns_the_flats_hold();
    assert!(leaves.len() > 60, "{} leaves", leaves.len());
    for leaf in &leaves {
        let turn = turned(leaf).expect("a turn");
        let flats = leaf.solid().expect("a turn the flats make");
        let (lines, reach) = lines_over(leaf);
        let measured = lines.inside(&flats.triangles());
        let pieces = turn.section.pieces();
        let least: Vec<Piece> = pieces
            .iter()
            .map(|piece| Piece {
                away: [piece.away[0], piece.away[1] * step_cosine()],
                ..*piece
            })
            .collect();
        let most: Vec<Piece> = pieces
            .iter()
            .map(|piece| Piece {
                away: [piece.away[0] * step_cosine(), piece.away[1]],
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
}

#[test]
fn the_box_a_turned_leaf_spans_holds_the_box_its_flats_span_within_their_sagitta() {
    let leaves = turns_the_flats_hold();
    for leaf in &leaves {
        let turn = turned(leaf).expect("a turn");
        let (low, high) = leaf.bounds().expect("a box");
        let (flat_low, flat_high) = leaf
            .solid()
            .and_then(|flats| flats.bounds())
            .expect("a box of flats");
        let reach = low.abs().max(high.abs()).max_element();
        let outer = turn
            .section
            .pieces()
            .iter()
            .map(|piece| piece.away[1])
            .fold(0.0, f64::max);
        let sagitta = outer * (1.0 - step_cosine()) + 1e-9 * reach;
        assert!(
            (low - flat_low).max_element() <= 1e-9 * reach
                && (flat_high - high).max_element() <= 1e-9 * reach,
            "{leaf}: {low} {high} against the flats' {flat_low} {flat_high}"
        );
        assert!(
            (flat_low - low).max_element() <= sagitta
                && (high - flat_high).max_element() <= sagitta,
            "{leaf}: {low} {high} past the flats' {flat_low} {flat_high}"
        );
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
fn a_case_asking_for_an_ellipse_is_told_from_one_that_does_not() {
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
        Case::new(start.clone(), vec![Step::cut(tool.clone())]).asks_for_an_ellipse()
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
    assert!(!Case::new(shaft(45.0, slanted), vec![]).asks_for_an_ellipse());
    assert!(!asks(&post, &block));

    let asking = (0..1000)
        .filter(|seed| Case::drawn_turned_off_the_lattice(*seed).asks_for_an_ellipse())
        .count();
    assert!(
        (20..500).contains(&asking),
        "{asking} cases ask for an ellipse"
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
#[ignore = "#533: the exact kernel does not turn yet"]
fn every_turned_leaf_raised_by_the_exact_kernel_alone_keeps_every_rule_and_encloses_pappus_s_volume()
 {
    let leaves = drawn_turns(200);
    assert!(leaves.len() > 60, "{} leaves", leaves.len());
    let weighed = random_solids::on_every_core(&leaves, |leaf| {
        let alone = Case::new(leaf.clone(), vec![]);
        let held = random_solids::held_to_arithmetic(&alone, &Exact);
        (held, Exact.raised(leaf).map(|body| body.volume()))
    });
    for (leaf, (held, volume)) in leaves.iter().zip(weighed) {
        assert!(held.is_ok(), "{leaf}: {held:?}");
        let promised = turned(leaf).expect("a turn").volume();
        let volume = volume.expect("a turned body");
        assert!(
            (volume - promised).abs() <= 1e-9 * promised,
            "{leaf}: {volume} against {promised}"
        );
    }
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
/// empty part's tools at.
fn laid_runs(turn: &Turned) -> (Vec<u32>, Vec<Laid>) {
    let [side] = <[Drawn; 1]>::try_from(turn.drawn()).ok().expect("one side");
    let (outline, holes) = side.contours();
    let straight = Straight::of(&outline, &holes, turn.frame(), &turn.turn(0.0), 1.0)
        .expect("a section of straight runs");
    let pieces = turn.section.pieces();
    let inside = |place: DVec2| {
        pieces.iter().any(|piece| {
            (piece.along[0]..=piece.along[1]).contains(&place.x)
                && (piece.away[0]..=piece.away[1]).contains(&place.y)
        })
    };
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
            let right = DVec2::new(along.y, -along.x);
            let probe = (from + to) / 2.0 + right * 1e-6 * (to - from).length();
            let out = if inside(probe) { -right } else { right };
            runs.push(Laid {
                run: corner.run,
                from,
                to,
                out,
            });
        }
    }
    (numbers, runs)
}

#[test]
#[ignore = "#533: the exact kernel does not turn yet"]
fn a_turned_leaf_names_its_faces_as_the_flats_name_them() {
    let leaves: Vec<Leaf> = drawn_turns(300)
        .into_iter()
        .filter(|leaf| turned(leaf).is_some_and(|turn| turn.section.side().is_some()))
        .collect();
    assert!(leaves.len() > 100, "{} leaves", leaves.len());
    for leaf in &leaves {
        let turn = turned(leaf).expect("a turn");
        let exact = Application.raised(leaf).expect("a turned body");
        let flats = Flats::for_case(&Case::new(leaf.clone(), vec![]))
            .raised(leaf)
            .expect("a turn the flats make");
        assert_eq!(exact.faces_end(), flats.faces_end(), "{leaf}");
        let (numbers, runs) = laid_runs(&turn);
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
        let (cosine, sine) = (turn.angle() / 2.0).sin_cos();
        let radial = swept.out * sine + swept.onward * cosine;
        let at = |place: DVec2| swept.origin + swept.along * place.x + radial * place.y;
        for run in &runs {
            let middle = (run.from + run.to) / 2.0;
            let way = (at(middle + run.out) - at(middle)).normalize();
            let hit = exact
                .ray_hit(at(middle) + way * 1e-3 * reach, -way)
                .unwrap_or_else(|| panic!("{leaf}: no face where run {} turns", run.run));
            let on_its_line = runs.iter().any(|other| {
                other.run as usize == hit.face
                    && ((other.from.x == other.to.x
                        && run.from.x == run.to.x
                        && other.from.x == run.from.x)
                        || (other.from.y == other.to.y
                            && run.from.y == run.to.y
                            && other.from.y == run.from.y))
            });
            assert!(
                hit.face == run.run as usize || on_its_line,
                "{leaf}: run {} turned into face {}",
                run.run,
                hit.face
            );
        }
    }
}

/// How many runs a section's outline and holes have.
fn runs_of(turn: &Turned) -> u32 {
    let (outline, holes) = turn.section.corners();
    (outline.len() + holes.iter().map(Vec::len).sum::<usize>()) as u32
}

#[test]
#[ignore = "#533: the exact kernel does not turn yet"]
fn turned_cases_the_exact_kernel_keeps_are_kept_through_the_application_s_body_and_stay_exact() {
    let seeds: Vec<u64> = (0..60).collect();
    let weighed = random_solids::on_every_core(&seeds, |seed| {
        let case = Case::drawn_turned(*seed);
        random_solids::held_to_arithmetic(&case, &Exact).ok()?;
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
    assert!(held > 30, "{held} cases held");
}

#[test]
#[ignore = "#533: the exact kernel does not turn yet"]
fn a_case_asking_for_an_ellipse_is_declined_as_unsupported_or_held() {
    let asking: Vec<Case> = (0..2000)
        .map(Case::drawn_turned_off_the_lattice)
        .filter(Case::asks_for_an_ellipse)
        .take(40)
        .collect();
    assert!(asking.len() >= 20, "{} cases", asking.len());
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
#[ignore = "#533: the exact kernel does not turn yet"]
fn random_turned_cases_keep_every_rule_on_the_exact_kernel() {
    let seeds: Vec<u64> = (0..40).collect();
    let weighed = random_solids::on_every_core(&seeds, |seed| {
        let case = Case::drawn_turned(*seed);
        (random_solids::held_to_arithmetic(&case, &Exact), case)
    });
    for (seed, (measured, case)) in seeds.iter().zip(weighed) {
        assert!(measured.is_ok(), "seed {seed}: {measured:?}\n{case}");
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
        Answering, DECLINED, GRAZING, HELD, campaign_over, shrunk_one_by_one,
    };

    use super::*;

    fn tallied(measured: random_solids::Measured) {
        HELD.fetch_add(measured.held, Ordering::Relaxed);
        GRAZING.fetch_add(measured.grazing, Ordering::Relaxed);
        DECLINED.fetch_add(measured.declined, Ordering::Relaxed);
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

    /// Each seed named in `CAO_TRIAGE_SEEDS`, commas between them, drawn as
    /// a turning campaign draws it, run again on the exact kernel — through
    /// the application's body when `CAO_TRIAGE_KERNEL` is `application` —
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
        shrunk_one_by_one(seeds, Case::drawn_turned_off_the_lattice, check);
    }

    /// The harness held to the flats, where whole cases of turns are too slow
    /// and too often broken for the gate (#418, #486, #492): every case the
    /// flats answer by their own rules is answered by the arithmetic too.
    #[test]
    #[ignore = "a campaign, run by hand once: see the head of this file"]
    fn turned_cases_the_flats_answer_by_their_own_rules_keep_every_rule_held_to_the_arithmetic() {
        let seeds: Vec<u64> = (0..300).collect();
        let weighed = random_solids::on_every_core(&seeds, |seed| {
            let case = Case::drawn_turned(*seed);
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
        assert!(held > 100, "{held} cases held");
    }
}
