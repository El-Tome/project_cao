//! The exact kernel turning a profile of straight runs about an axis lying in
//! its plane (#533), a run slanted to the axis included, which turns into a
//! cone (#536), held to the arithmetic: Pappus's volume, the rules every
//! body keeps, and what the same profile cut into annular slabs, each raised
//! from a sector of a disc and joined by the boolean, holds along a grid of
//! lines. A cone meets what shares its axis along circles, what holds its
//! axis along rulings, and a plane at a slant along the conic it cuts (#552);
//! a cylinder or a cone off its axis is declined as unsupported, unless the
//! two stand clear of each other.
//!
//! The profiles are handed to `cao_solid::brep::Body::turned` already laid
//! square to their axis, as `cao_solid::turning::Straight` literals: what
//! reads a drawn profile into one is tested beside it. The tests through
//! `cao_solid::Body` are the ones a drawn profile reaches.
//!
//! A smoke campaign of turned leaves, alone and combined with prisms, is run
//! by hand:
//!
//! ```text
//! CAO_FUZZ_SECONDS=3600 cargo test --release -p cao_solid --features campaigns \
//!     --test the_exact_kernel_turns -- --ignored --nocapture \
//!     a_campaign_of_turned_leaves_and_prisms_keeps_every_rule
//! ```
//!
//! `CAO_FUZZ_SEED` starts it from a given seed rather than from the clock,
//! and `CAO_FUZZ_CASES` stops it after that many.

use std::f64::consts::{FRAC_PI_2, PI, TAU};

use cao_solid::brep::{Body, Declined, Surface};
use cao_solid::profile::{Contour, Frame, Profile, Run};
use cao_solid::soundness::{closed, listed, uncrossed};
use cao_solid::turning::{Axis, Corner, Straight, Turn};
use cao_solid::{Body as Part, Loop};
use glam::{DVec2, DVec3};

/// A profile laid square to its axis, its corners `(h, r)`: along the axis
/// and away from it on the side `side` names. The outline comes first; run
/// `k` leaves the `k`th corner, counted on through the holes.
fn laid(side: f64, contours: &[&[[f64; 2]]]) -> Straight {
    let mut run = 0;
    let mut last_off_the_axis = None;
    let contours = contours
        .iter()
        .map(|corners| {
            let count = corners.len();
            (0..count)
                .map(|index| {
                    let [at, next] = [corners[index], corners[(index + 1) % count]];
                    if at[1] != 0.0 || next[1] != 0.0 {
                        last_off_the_axis = Some(run);
                    }
                    run += 1;
                    Corner {
                        at: DVec2::from(at),
                        run: run - 1,
                        bend: None,
                    }
                })
                .collect()
        })
        .collect();
    Straight {
        side,
        contours,
        runs: run,
        last_off_the_axis,
    }
}

/// Where a turn stands: a point of its axis, the axis, and the direction of
/// the profile's side `+1`.
#[derive(Clone, Copy, Debug)]
struct Turning {
    origin: DVec3,
    along: DVec3,
    side: DVec3,
}

impl Turning {
    /// The plane the profile is drawn in, its first axis the turn's.
    fn frame(&self) -> Frame {
        Frame {
            origin: self.origin,
            u: self.along,
            v: self.side,
        }
    }

    fn by(&self, angle: f64) -> Turn {
        Turn {
            axis: Axis {
                origin: DVec2::ZERO,
                direction: DVec2::X,
            },
            angle,
            resolution: 0.0,
            on_the_axis: 0.0,
        }
    }
}

fn about_y() -> Turning {
    Turning {
        origin: DVec3::ZERO,
        along: DVec3::Y,
        side: DVec3::X,
    }
}

/// An axis leaning in XY, away from the origin.
fn slanted() -> Turning {
    let along = DVec3::new(3.0, 4.0, 0.0).normalize();
    Turning {
        origin: DVec3::new(1.5, -2.0, 0.0),
        along,
        side: DVec3::new(-along.y, along.x, 0.0),
    }
}

fn turned(straight: &Straight, turning: Turning, angle: f64) -> Result<Body, Declined> {
    Body::turned(straight, turning.frame(), &turning.by(angle))
}

/// What the profile turned by `angle` holds, by Pappus: the angle, a whole
/// turn within a thousandth of a radian of one, times the integral of the
/// distance from the axis over the area, the holes taken out.
fn pappus(straight: &Straight, angle: f64) -> f64 {
    let integral = |corners: &[Corner]| -> f64 {
        let count = corners.len();
        let twice: f64 = (0..count)
            .map(|index| {
                let [one, next] = [corners[index].at, corners[(index + 1) % count].at];
                one.perp_dot(next) * (one.y + next.y)
            })
            .sum();
        twice.abs() / 6.0
    };
    let (outline, holes) = straight.contours.split_first().expect("an outline");
    let area = integral(outline) - holes.iter().map(|hole| integral(hole)).sum::<f64>();
    let angle = if (angle.abs() - TAU).abs() < 1e-3 {
        TAU
    } else {
        angle.abs()
    };
    angle * area
}

/// A body held to every rule and to the volume arithmetic promises it.
fn holds(body: &Body, volume: f64) {
    let reach = body.scale().reach();
    assert_eq!(listed(&body.listing(), reach), Ok(()));
    let triangles = body.triangles(1e-3 * reach);
    assert_eq!(closed(&triangles), Ok(()));
    assert_eq!(uncrossed(&triangles), Ok(()));
    assert!(
        (body.volume() - volume).abs() <= 1e-9 * volume,
        "{} against the arithmetic's {volume}",
        body.volume()
    );
}

fn square() -> Straight {
    laid(
        1.0,
        &[&[[0.0, 0.0], [10.0, 0.0], [10.0, 10.0], [0.0, 10.0]]],
    )
}

fn shaft() -> Straight {
    laid(
        1.0,
        &[&[
            [0.0, 0.0],
            [20.0, 0.0],
            [20.0, 4.0],
            [12.0, 4.0],
            [12.0, 7.0],
            [5.0, 7.0],
            [5.0, 10.0],
            [0.0, 10.0],
        ]],
    )
}

#[test]
fn a_square_turned_about_one_of_its_sides_holds_the_arithmetics_volume() {
    for turning in [about_y(), slanted()] {
        let body = turned(&square(), turning, TAU).expect("the square turns");
        holds(&body, 1000.0 * PI);
    }
}

#[test]
fn a_ring_turned_whole_holds_the_arithmetics_volume() {
    let ring = laid(
        -1.0,
        &[&[[0.0, 5.0], [10.0, 5.0], [10.0, 10.0], [0.0, 10.0]]],
    );
    for turning in [about_y(), slanted()] {
        let body = turned(&ring, turning, TAU).expect("the ring turns");
        holds(&body, 750.0 * PI);
    }
}

#[test]
fn a_stepped_shaft_turned_whole_holds_the_arithmetics_volume() {
    for turning in [about_y(), slanted()] {
        let body = turned(&shaft(), turning, -TAU).expect("the shaft turns");
        holds(&body, PI * (16.0 * 8.0 + 49.0 * 7.0 + 100.0 * 5.0));
    }
}

#[test]
fn a_partial_turn_either_way_holds_the_arithmetics_volume() {
    let block = laid(1.0, &[&[[0.0, 2.0], [4.0, 2.0], [4.0, 5.0], [0.0, 5.0]]]);
    for angle in [FRAC_PI_2, -FRAC_PI_2] {
        let body = turned(&block, about_y(), angle).expect("the block turns");
        holds(&body, 21.0 * PI);
    }
    for angle in [0.3, -2.0, 4.0, -5.5] {
        let body = turned(&shaft(), slanted(), angle).expect("the shaft turns");
        holds(&body, pappus(&shaft(), angle));
    }
}

#[test]
fn a_groove_turned_into_a_shaft_leaves_the_arithmetics_volume() {
    let stock = laid(
        1.0,
        &[&[[0.0, 0.0], [10.0, 0.0], [10.0, 20.0], [0.0, 20.0]]],
    );
    let groove = laid(
        1.0,
        &[&[[4.0, 18.0], [6.0, 18.0], [6.0, 22.0], [4.0, 22.0]]],
    );
    for angle in [TAU, PI, 1.0] {
        let stock = turned(&stock, about_y(), angle).expect("the stock turns");
        let groove = turned(&groove, about_y(), angle).expect("the groove turns");
        let cut = stock.cut_by(&groove).expect("the groove is cut");
        let whole = if angle == TAU { TAU } else { angle };
        holds(&cut, whole / 2.0 * (400.0 * 10.0 - (400.0 - 324.0) * 2.0));
    }
}

/// A block between two corners, raised from XY.
fn block(low: [f64; 3], high: [f64; 3]) -> Body {
    let frame = Frame {
        origin: DVec3::new(0.0, 0.0, low[2]),
        u: DVec3::X,
        v: DVec3::Y,
    };
    let outline = Contour::rectangle(DVec2::new(low[0], low[1]), DVec2::new(high[0], high[1]));
    Body::raised(&outline, &[], frame, DVec3::Z * (high[2] - low[2])).expect("a block raises")
}

/// A circle raised along Y from XZ, about the Y axis.
fn bore(radius: f64, from: f64, to: f64) -> Body {
    let frame = Frame {
        origin: DVec3::Y * from,
        u: DVec3::Z,
        v: DVec3::X,
    };
    let circle = Contour {
        corners: vec![DVec2::X * radius],
        runs: vec![Run::Round {
            center: DVec2::ZERO,
            turn: TAU,
        }],
    };
    Body::raised(&circle, &[], frame, DVec3::Y * (to - from)).expect("a bore raises")
}

#[test]
fn a_prism_raised_on_a_turned_shaft_stays_exact() {
    let shaft = turned(&shaft(), about_y(), TAU).expect("the shaft turns");
    let shaft_volume = shaft.volume();
    let joined = shaft
        .joined(&block([-2.0, 18.0, -2.0], [2.0, 26.0, 2.0]))
        .expect("a boss joins the shaft's end");
    holds(&joined, shaft_volume + 16.0 * 6.0);
    let cut = joined
        .cut_by(&block([8.0, 1.0, -20.0], [12.0, 3.0, 20.0]))
        .expect("a flat is cut on the shaft's shoulder");
    holds(&cut, cut.volume());
    assert!(cut.volume() < joined.volume());
}

#[test]
fn a_bore_raised_along_a_turned_shaft_s_axis_shares_its_cylinder() {
    let tube = laid(
        1.0,
        &[&[[0.0, 5.0], [10.0, 5.0], [10.0, 20.0], [0.0, 20.0]]],
    );
    let turned_tube = turned(&tube, about_y(), TAU).expect("the tube turns");
    let raised = bore(5.0, -5.0, 15.0);
    let cylinder = |body: &Body, radius: f64| {
        body.face_ids()
            .map(|face| *body.surface(body.face(face).surface))
            .find(|surface| matches!(surface, Surface::Cylinder(c) if c.radius == radius))
            .expect("a wall of that radius")
    };
    assert_eq!(cylinder(&turned_tube, 5.0), cylinder(&raised, 5.0));
    let rod = turned(&square(), about_y(), TAU).expect("the rod turns");
    let bored = rod.cut_by(&raised).expect("the bore is cut along the axis");
    holds(&bored, PI * (100.0 - 25.0) * 10.0);
    let filled = turned_tube
        .joined(&raised)
        .expect("the bore fills the tube");
    holds(&filled, PI * (400.0 * 10.0 + 25.0 * 10.0));
}

#[test]
fn two_sides_of_an_area_across_the_axis_turned_and_joined_hold_their_volume() {
    let left = laid(1.0, &[&[[0.0, 0.0], [10.0, 0.0], [10.0, 6.0], [0.0, 6.0]]]);
    let right = laid(-1.0, &[&[[0.0, 0.0], [10.0, 0.0], [10.0, 4.0], [0.0, 4.0]]]);
    for degrees in [90.0_f64, 180.0, 270.0, 360.0] {
        let angle = degrees.to_radians();
        let both = turned(&left, about_y(), angle)
            .and_then(|left| left.joined(&turned(&right, about_y(), angle)?))
            .expect("the two sides join");
        let near = if angle <= PI {
            2.0 * angle
        } else {
            (angle + PI).min(TAU)
        };
        holds(&both, 10.0 * (8.0 * near + 10.0 * angle));
    }
}

/// A cell of a profile cut into annular slabs: radii and heights.
#[derive(Clone, Copy, Debug)]
struct Cell {
    r0: f64,
    r1: f64,
    h0: f64,
    h1: f64,
}

/// The sector of a disc or of an annulus a cell's turn sweeps: arcs about
/// the origin, whole circles for a whole turn.
fn sector(cell: Cell, turn: f64) -> (Contour, Vec<Contour>) {
    let at = |angle: f64| DVec2::from_angle(angle);
    if (turn.abs() - TAU).abs() < 1e-3 {
        let holes = if cell.r0 > 0.0 {
            vec![Contour::circle(DVec2::ZERO, cell.r0)]
        } else {
            Vec::new()
        };
        return (Contour::circle(DVec2::ZERO, cell.r1), holes);
    }
    let round = |turn| Run::Round {
        center: DVec2::ZERO,
        turn,
    };
    if cell.r0 == 0.0 {
        let corners = vec![DVec2::ZERO, at(0.0) * cell.r1, at(turn) * cell.r1];
        let runs = vec![Run::Straight, round(turn), Run::Straight];
        return (Contour { corners, runs }, Vec::new());
    }
    let corners = vec![
        at(0.0) * cell.r0,
        at(0.0) * cell.r1,
        at(turn) * cell.r1,
        at(turn) * cell.r0,
    ];
    let runs = vec![Run::Straight, round(turn), Run::Straight, round(-turn)];
    (Contour { corners, runs }, Vec::new())
}

/// The cells raised each from its sector, square to the axis, and joined.
fn slabs(turning: Turning, cells: &[Cell], turn: f64) -> Body {
    let raised = |cell: Cell| {
        let frame = Frame {
            origin: turning.origin + turning.along * cell.h0,
            u: turning.side,
            v: turning.along.cross(turning.side),
        };
        let (outline, holes) = sector(cell, turn);
        Body::raised(&outline, &holes, frame, turning.along * (cell.h1 - cell.h0))
            .expect("a slab raises")
    };
    cells[1..].iter().fold(raised(cells[0]), |body, cell| {
        body.joined(&raised(*cell))
            .unwrap_or_else(|declined| panic!("{cell:?} joins: {declined:?}"))
    })
}

/// The stepped shaft and its slabs: radius ten on `[0, 5]`, seven on
/// `[5, 12]`, four on `[12, 20]`, each cut where a smaller step's radius
/// stands so that no slab overhangs another.
fn shaft_slabs() -> Vec<Cell> {
    vec![
        Cell {
            r0: 0.0,
            r1: 4.0,
            h0: 0.0,
            h1: 5.0,
        },
        Cell {
            r0: 4.0,
            r1: 7.0,
            h0: 0.0,
            h1: 5.0,
        },
        Cell {
            r0: 7.0,
            r1: 10.0,
            h0: 0.0,
            h1: 5.0,
        },
        Cell {
            r0: 0.0,
            r1: 4.0,
            h0: 5.0,
            h1: 12.0,
        },
        Cell {
            r0: 4.0,
            r1: 7.0,
            h0: 5.0,
            h1: 12.0,
        },
        Cell {
            r0: 0.0,
            r1: 4.0,
            h0: 12.0,
            h1: 20.0,
        },
    ]
}

#[test]
fn a_stepped_profile_turned_holds_what_its_slabs_raised_and_joined_hold() {
    for turning in [about_y(), slanted()] {
        for turn in [TAU, FRAC_PI_2, -3.0 * PI / 4.0, 4.0, 1.234] {
            let direct = turned(&shaft(), turning, turn).expect("the shaft turns");
            let oracle = slabs(turning, &shaft_slabs(), turn);
            let volume = pappus(&shaft(), turn);
            holds(&direct, volume);
            assert!((oracle.volume() - volume).abs() <= 1e-9 * volume);
            let eps = direct.scale().eps().max(oracle.scale().eps());
            let mut compared = 0;
            for row in 0..12 {
                for column in 0..12 {
                    let origin = turning.origin
                        + DVec3::new(
                            -24.0 + 4.0 * f64::from(row),
                            -24.0 + 4.0 * f64::from(column),
                            -30.0,
                        );
                    let direction = DVec3::new(0.0123, 0.0371, 1.0);
                    let (Ok(one), Ok(other)) = (
                        direct.crossings_along(origin, direction, eps),
                        oracle.crossings_along(origin, direction, eps),
                    ) else {
                        continue;
                    };
                    compared += 1;
                    assert_eq!(
                        one.len(),
                        other.len(),
                        "{origin} by {turn}: {one:?} {other:?}"
                    );
                    for ((at, step), (other_at, other_step)) in one.iter().zip(&other) {
                        assert_eq!(step, other_step);
                        assert!((at - other_at).abs() <= 1e3 * eps, "{one:?} {other:?}");
                    }
                }
            }
            assert!(compared > 100, "only {compared} lines read on both bodies");
        }
    }
}

/// The volume two endless cylinders of radii `big` and `small` share,
/// their axes meeting at `degrees`: each line of the smaller one `a` across
/// from the larger's axis, along the line square to both, runs
/// `2√(R² − a²)/sin θ` inside the larger, and over the smaller's disc
/// `a = r sin φ`, the whole turn counting the disc twice. The integrand is
/// periodic and smooth, so the trapezoid rule over the turn converges
/// geometrically.
fn shared(big: f64, small: f64, degrees: f64) -> f64 {
    let samples = 256;
    let sum: f64 = (0..samples)
        .map(|k| {
            let (sin, cos) = (TAU * k as f64 / samples as f64).sin_cos();
            cos * cos * (big * big - small * small * sin * sin).sqrt()
        })
        .sum();
    2.0 * small * small / degrees.to_radians().sin() * sum * TAU / samples as f64
}

#[test]
fn a_turn_about_a_slanted_axis_crossing_a_raised_cylinder_at_a_skew_angle_holds_its_volume() {
    let leaning = Turning {
        origin: DVec3::ZERO,
        along: DVec3::new(3.0_f64.sqrt() / 2.0, 0.5, 0.0),
        side: DVec3::new(-0.5, 3.0_f64.sqrt() / 2.0, 0.0),
    };
    let ring = laid(
        1.0,
        &[&[[-10.0, 2.0], [10.0, 2.0], [10.0, 4.0], [-10.0, 4.0]]],
    );
    let ring = turned(&ring, leaning, TAU).expect("the ring turns");
    let joined = bore(3.0, -20.0, 20.0)
        .joined(&ring)
        .expect("the kernel answers");
    let bore = PI * 9.0 * 40.0;
    let tube = PI * (16.0 - 4.0) * 20.0;
    let common = shared(4.0, 3.0, 60.0) - shared(3.0, 2.0, 60.0);
    holds(&joined, bore + tube - common);
}

/// A profile drawn in XY as its corners and the triangles its area is cut
/// into, every run straight, ready for the body to turn about Y.
fn drawn<'a>(corners: &'a [DVec2], triangles: &'a [[DVec2; 3]]) -> Profile<'a> {
    Profile {
        exact: Some((Contour::straight(corners.to_vec()), Vec::new())),
        sampled: Loop::straight(corners),
        sampled_holes: Vec::new(),
        triangles,
    }
}

/// A square drawn on the sketch's Y axis, its last side on it.
fn square_on_y() -> ([DVec2; 4], [[DVec2; 3]; 2]) {
    let corners = [
        DVec2::new(0.0, 0.0),
        DVec2::new(10.0, 0.0),
        DVec2::new(10.0, 10.0),
        DVec2::new(0.0, 10.0),
    ];
    let triangles = [
        [corners[0], corners[1], corners[2]],
        [corners[0], corners[2], corners[3]],
    ];
    (corners, triangles)
}

fn ground() -> Frame {
    Frame {
        origin: DVec3::ZERO,
        u: DVec3::X,
        v: DVec3::Y,
    }
}

fn about_the_sketch_s_y(angle: f64, profile: &Profile) -> Turn {
    Turn::of(
        Axis {
            origin: DVec2::ZERO,
            direction: DVec2::Y,
        },
        angle,
        1e-5 * 10.0,
        profile,
    )
}

/// An exact part: a block raised from XY.
fn part() -> Part {
    let corners = [
        DVec2::new(20.0, 20.0),
        DVec2::new(30.0, 20.0),
        DVec2::new(30.0, 30.0),
        DVec2::new(20.0, 30.0),
    ];
    let profile = drawn(&corners, &[]);
    let block = Part::default()
        .tool_raised(&profile, ground(), DVec3::Z * 5.0)
        .expect("the block raises");
    Part::default()
        .union(&block)
        .expect("the part holds the block")
}

#[test]
fn a_straight_profile_is_turned_by_the_exact_kernel_through_the_body() {
    let (corners, triangles) = square_on_y();
    let profile = drawn(&corners, &triangles);
    let part = part();
    let tool = part
        .tool_turned(&profile, ground(), &about_the_sketch_s_y(TAU, &profile))
        .expect("the square turns");
    let turned = part.union(&tool).expect("the turn joins the part");
    assert!(turned.is_exact());
    assert!((turned.volume() - (500.0 + 1000.0 * PI)).abs() <= 1e-9 * turned.volume());
}

/// A shaft Ø20 for 30 drawn on the sketch's Y axis, its top rim chamfered
/// 2, its last side on the axis, cut into triangles.
fn chamfered_shaft_on_y() -> ([DVec2; 5], [[DVec2; 3]; 3]) {
    let corners = [
        DVec2::new(0.0, 0.0),
        DVec2::new(10.0, 0.0),
        DVec2::new(10.0, 28.0),
        DVec2::new(8.0, 30.0),
        DVec2::new(0.0, 30.0),
    ];
    let triangles = [
        [corners[0], corners[1], corners[2]],
        [corners[0], corners[2], corners[3]],
        [corners[0], corners[3], corners[4]],
    ];
    (corners, triangles)
}

#[test]
fn a_slanted_profile_is_turned_by_the_exact_kernel_through_the_body() {
    let (corners, triangles) = chamfered_shaft_on_y();
    let profile = drawn(&corners, &triangles);
    let part = part();
    let tool = part
        .tool_turned(&profile, ground(), &about_the_sketch_s_y(TAU, &profile))
        .expect("the chamfered shaft turns");
    let turned = part.union(&tool).expect("the turn joins the part");
    assert!(turned.is_exact());
    let volume = 500.0 + 8888.0 * PI / 3.0;
    assert!((turned.volume() - volume).abs() <= 1e-9 * volume);
}

#[test]
fn a_profile_with_an_arc_is_turned_exactly_through_the_body() {
    let center = DVec2::new(6.0, 2.0);
    let points: Vec<DVec2> =
        std::iter::once(DVec2::new(2.0, 0.0))
            .chain((0..=16).map(|step| {
                center + DVec2::from_angle(-FRAC_PI_2 + PI * f64::from(step) / 16.0) * 2.0
            }))
            .chain(std::iter::once(DVec2::new(2.0, 4.0)))
            .collect();
    let profile = Profile {
        exact: Some((
            Contour {
                corners: vec![
                    DVec2::new(2.0, 0.0),
                    DVec2::new(6.0, 0.0),
                    DVec2::new(6.0, 4.0),
                    DVec2::new(2.0, 4.0),
                ],
                runs: vec![
                    Run::Straight,
                    Run::Round { center, turn: PI },
                    Run::Straight,
                    Run::Straight,
                ],
            },
            Vec::new(),
        )),
        sampled: Loop::straight(&points),
        sampled_holes: Vec::new(),
        triangles: &[],
    };
    let part = part();
    let tool = part
        .tool_turned(&profile, ground(), &about_the_sketch_s_y(TAU, &profile))
        .expect("the exact kernel turns the arc");
    let turned = part.union(&tool).expect("the turn joins the part");
    assert!(turned.is_exact());
    let volume = 500.0 + 128.0 * PI + 24.0 * PI * PI + 32.0 * PI / 3.0;
    assert!((turned.volume() - volume).abs() <= 1e-9 * volume);
}

#[test]
fn a_declined_turn_counts_past_the_numbers_it_would_have_named() {
    let (square, square_triangles) = square_on_y();
    let (chamfered, chamfered_triangles) = chamfered_shaft_on_y();
    for (profile, whole, partial) in [
        (drawn(&square, &square_triangles), 3, 6),
        (drawn(&chamfered, &chamfered_triangles), 4, 7),
    ] {
        for (angle, named) in [(TAU, whole), (FRAC_PI_2, partial)] {
            let mut part = part();
            let before = part.faces_end();
            part.count_past_turned(&profile, ground(), &about_the_sketch_s_y(angle, &profile));
            assert_eq!(
                part.faces_end(),
                before + named,
                "{:?} turned {angle} rad",
                profile.sampled.points
            );
        }
    }
}

/// A rectangle from `(left, 0)` to `(50, 20)`, its last side the one at
/// `left`, cut into its two triangles.
fn rectangle_from(left: f64) -> ([DVec2; 4], [[DVec2; 3]; 2]) {
    let corners = [
        DVec2::new(left, 0.0),
        DVec2::new(50.0, 0.0),
        DVec2::new(50.0, 20.0),
        DVec2::new(left, 20.0),
    ];
    let triangles = [
        [corners[0], corners[1], corners[2]],
        [corners[0], corners[2], corners[3]],
    ];
    (corners, triangles)
}

#[test]
fn a_whole_turn_whose_last_side_lies_near_the_axis_names_the_numbers_main_named() {
    for (left, named) in [(0.0, 3), (1e-5, 3), (5e-3, 4), (0.03, 4), (0.06, 4)] {
        let (corners, triangles) = rectangle_from(left);
        let exact = drawn(&corners, &triangles);
        let flats = Profile {
            exact: None,
            ..drawn(&corners, &triangles)
        };
        for profile in [&exact, &flats] {
            let turn = about_the_sketch_s_y(TAU, profile);
            let tool = part()
                .tool_turned(profile, ground(), &turn)
                .expect("the rectangle turns");
            assert_eq!(
                (tool.is_exact(), tool.faces_end()),
                (profile.exact.is_some(), named),
                "a side at {left} from the axis"
            );
            let mut declined = part();
            let before = declined.faces_end();
            declined.count_past_turned(profile, ground(), &turn);
            assert_eq!(declined.faces_end(), before + named, "a side at {left}");
        }
    }
}

#[test]
fn a_turn_a_hair_short_of_a_whole_turn_is_whole() {
    let angle = 359.95_f64.to_radians();
    let body = turned(&square(), about_y(), angle).expect("the square turns");
    assert_eq!(body.vertex_ids().count(), 0);
    holds(&body, 1000.0 * PI);
}

#[test]
fn a_turn_a_tenth_of_a_degree_short_of_a_whole_turn_keeps_its_slit() {
    let angle = 359.9_f64.to_radians();
    let body = turned(&square(), about_y(), angle).expect("the square turns");
    assert_eq!(body.vertex_ids().count(), 6);
    holds(&body, 500.0 * angle);
}

#[test]
fn a_turn_of_half_a_turn_and_a_hair_keeps_two_ends() {
    let angle = 180.0001_f64.to_radians();
    let body = turned(&square(), about_y(), angle).expect("the square turns");
    let ends: Vec<_> = body
        .face_ids()
        .filter(|&face| body.numbers(face).iter().any(|&number| number >= 4))
        .collect();
    assert_eq!(ends.len(), 2);
    holds(&body, 500.0 * angle);
}

/// A shaft of radius ten and length thirty, its far end chamfered by two.
fn chamfered_shaft() -> Straight {
    laid(
        1.0,
        &[&[
            [0.0, 0.0],
            [30.0, 0.0],
            [30.0, 8.0],
            [28.0, 10.0],
            [0.0, 10.0],
        ]],
    )
}

/// A point of radius five, its apex ten along the axis.
fn point() -> Straight {
    laid(1.0, &[&[[0.0, 0.0], [10.0, 0.0], [0.0, 5.0]]])
}

/// A plate of forty by forty by ten, on XY.
fn plate() -> Body {
    block([-20.0, -20.0, 0.0], [20.0, 20.0, 10.0])
}

/// A bore of radius two through the plate, its mouth countersunk to radius
/// six, about the line through `(x, y)` along Z: cut whole, it takes 280π/3.
fn countersink(x: f64, y: f64) -> Body {
    let section = laid(
        1.0,
        &[&[
            [-1.0, 0.0],
            [11.0, 0.0],
            [11.0, 7.0],
            [6.0, 2.0],
            [-1.0, 2.0],
        ]],
    );
    let along_z = Turning {
        origin: DVec3::new(x, y, 0.0),
        along: DVec3::Z,
        side: DVec3::X,
    };
    turned(&section, along_z, TAU).expect("the countersink turns")
}

/// A circle about `(x, y)` raised along Z from `from` to `to`.
fn post(x: f64, y: f64, radius: f64, from: f64, to: f64) -> Body {
    let frame = Frame {
        origin: DVec3::Z * from,
        u: DVec3::X,
        v: DVec3::Y,
    };
    let circle = Contour::circle(DVec2::new(x, y), radius);
    Body::raised(&circle, &[], frame, DVec3::Z * (to - from)).expect("a post raises")
}

/// How many vertices stand within the body's tolerance of `point`.
fn corners_at(body: &Body, point: DVec3) -> usize {
    let eps = body.scale().eps();
    body.vertex_ids()
        .filter(|&vertex| body.vertex(vertex).point.distance(point) <= eps)
        .count()
}

#[test]
fn a_chamfered_shaft_holds_pappus_s_volume() {
    for turning in [about_y(), slanted()] {
        let body = turned(&chamfered_shaft(), turning, TAU).expect("the shaft turns");
        holds(&body, 8888.0 * PI / 3.0);
        for angle in [FRAC_PI_2, -2.0, 4.0] {
            let body = turned(&chamfered_shaft(), turning, angle).expect("the shaft turns");
            holds(&body, pappus(&chamfered_shaft(), angle));
        }
    }
}

#[test]
fn a_point_turned_whole_holds_a_third_of_its_cylinder() {
    for turning in [about_y(), slanted()] {
        let body = turned(&point(), turning, TAU).expect("the point turns");
        holds(&body, 250.0 * PI / 3.0);
        let tip = turning.origin + turning.along * 10.0;
        assert_eq!(body.vertex_ids().count(), 1);
        assert_eq!(corners_at(&body, tip), 1, "{turning:?}");
    }
}

#[test]
fn a_shaft_ending_in_a_point_holds_pappus_s_volume() {
    let shaft = laid(1.0, &[&[[0.0, 0.0], [15.0, 0.0], [10.0, 5.0], [0.0, 5.0]]]);
    for turning in [about_y(), slanted()] {
        let body = turned(&shaft, turning, TAU).expect("the shaft turns");
        holds(&body, 875.0 * PI / 3.0);
        assert_eq!(corners_at(&body, turning.origin + turning.along * 15.0), 1);
        for angle in [PI, -FRAC_PI_2] {
            let body = turned(&shaft, turning, angle).expect("the shaft turns");
            holds(&body, pappus(&shaft, angle));
        }
    }
}

#[test]
fn a_countersunk_hole_cut_into_a_plate_leaves_the_arithmetic_s_volume() {
    for (x, y) in [(0.0, 0.0), (5.0, -3.0)] {
        let cut = plate()
            .cut_by(&countersink(x, y))
            .expect("the countersink cuts");
        holds(&cut, 16000.0 - 280.0 * PI / 3.0);
    }
}

#[test]
fn a_partial_turn_of_a_slanted_run_either_way_holds_pappus_s_volume() {
    let trapezoid = laid(1.0, &[&[[0.0, 2.0], [0.0, 5.0], [4.0, 4.0], [4.0, 2.0]]]);
    for angle in [FRAC_PI_2, -FRAC_PI_2] {
        let body = turned(&trapezoid, about_y(), angle).expect("the trapezoid turns");
        holds(&body, 49.0 * PI / 3.0);
    }
    for angle in [0.3, -2.0, 4.0, -5.5, PI] {
        let body = turned(&trapezoid, slanted(), angle).expect("the trapezoid turns");
        holds(&body, pappus(&trapezoid, angle));
    }
}

#[test]
fn a_point_turned_part_way_holds_its_volume_and_its_apex_as_one_corner() {
    for turning in [about_y(), slanted()] {
        for angle in [FRAC_PI_2, -2.0, PI, 4.0] {
            let body = turned(&point(), turning, angle).expect("the point turns");
            holds(&body, pappus(&point(), angle));
            let tip = turning.origin + turning.along * 10.0;
            assert_eq!(corners_at(&body, tip), 1, "by {angle}");
        }
    }
}

#[test]
fn a_point_whose_tip_a_partial_turn_cuts_away_keeps_its_tip_in_the_sector_left() {
    let block = laid(1.0, &[&[[5.0, 0.0], [12.0, 0.0], [12.0, 8.0], [5.0, 8.0]]]);
    for turning in [about_y(), slanted()] {
        let point = turned(&point(), turning, TAU).expect("the point turns");
        for angle in [1.5 * PI, -1.5 * PI, FRAC_PI_2] {
            let tool = turned(&block, turning, angle).expect("the block turns");
            let cut = point.cut_by(&tool).expect("the turn cuts the tip");
            let tip = 31.25 * PI / 3.0;
            holds(&cut, 250.0 * PI / 3.0 - tip * angle.abs() / TAU);
        }
    }
}

#[test]
fn a_cone_joined_to_the_cylinder_it_continues_holds_its_volume() {
    let rod = laid(1.0, &[&[[0.0, 0.0], [10.0, 0.0], [10.0, 5.0], [0.0, 5.0]]]);
    let tip = laid(1.0, &[&[[10.0, 0.0], [15.0, 0.0], [10.0, 5.0]]]);
    for turning in [about_y(), slanted()] {
        for angle in [TAU, FRAC_PI_2, -4.0] {
            let rod_turned = turned(&rod, turning, angle).expect("the rod turns");
            let tip_turned = turned(&tip, turning, angle).expect("the tip turns");
            let joined = rod_turned
                .joined(&tip_turned)
                .expect("the tip joins the rod");
            holds(&joined, pappus(&rod, angle) + pappus(&tip, angle));
            if angle == TAU {
                assert_eq!(joined.face_ids().count(), 3, "the shared disc is gone");
            }
        }
    }
}

#[test]
fn two_coaxial_cones_cut_one_from_the_other_hold_their_volume() {
    let wide = laid(1.0, &[&[[0.0, 0.0], [12.0, 0.0], [0.0, 6.0]]]);
    let slender = laid(1.0, &[&[[-1.0, 0.0], [15.0, 0.0], [-1.0, 4.0]]]);
    let left = laid(1.0, &[&[[0.0, 3.75], [0.0, 6.0], [9.0, 1.5]]]);
    for turning in [about_y(), slanted()] {
        for angle in [TAU, FRAC_PI_2] {
            let wide = turned(&wide, turning, angle).expect("the wide cone turns");
            let slender = turned(&slender, turning, angle).expect("the slender cone turns");
            let cut = wide.cut_by(&slender).expect("the slender cone cuts");
            holds(&cut, pappus(&left, angle));
            let joined = wide.joined(&slender).expect("the cones join");
            holds(&joined, pappus(&left, angle) + slender.volume());
        }
    }
}

#[test]
fn two_points_apex_to_apex_hold_their_volume() {
    let lower = laid(1.0, &[&[[0.0, 0.0], [10.0, 0.0], [0.0, 5.0]]]);
    let upper = laid(1.0, &[&[[10.0, 0.0], [16.0, 0.0], [16.0, 3.0]]]);
    for turning in [about_y(), slanted()] {
        let lower = turned(&lower, turning, TAU).expect("the lower point turns");
        let upper = turned(&upper, turning, TAU).expect("the upper point turns");
        let joined = lower.joined(&upper).expect("the points join at their tips");
        holds(&joined, 250.0 * PI / 3.0 + 18.0 * PI);
        let cut = lower.cut_by(&upper).expect("a tip cuts nothing from a tip");
        holds(&cut, 250.0 * PI / 3.0);
    }
}

#[test]
fn a_point_halved_by_a_block_through_its_axis_holds_its_volume() {
    let point = turned(&point(), about_y(), TAU).expect("the point turns");
    let half = block([0.0, -1.0, -10.0], [10.0, 11.0, 10.0]);
    let cut = point.cut_by(&half).expect("the block halves the point");
    holds(&cut, 125.0 * PI / 3.0);
    let joined = point.joined(&half).expect("the block joins the point");
    holds(&joined, 2400.0 + 125.0 * PI / 3.0);
}

#[test]
fn a_partial_point_joined_to_a_block_holds_its_volume() {
    let quarter = turned(&point(), about_y(), FRAC_PI_2).expect("the point turns");
    let cap = block([-10.0, 4.0, -10.0], [10.0, 12.0, 10.0]);
    let joined = quarter.joined(&cap).expect("the block joins the point");
    holds(&joined, 3200.0 + 49.0 * PI / 3.0);
    let cut = quarter.cut_by(&cap).expect("the block cuts the tip off");
    holds(&cut, 49.0 * PI / 3.0);
}

#[test]
fn a_countersink_beside_a_bolt_hole_is_passed_over() {
    let bolt = post(-10.0, 8.0, 2.0, -1.0, 11.0);
    let both = 16000.0 - 280.0 * PI / 3.0 - 40.0 * PI;
    let sunk_first = plate()
        .cut_by(&countersink(5.0, -3.0))
        .and_then(|plate| plate.cut_by(&bolt))
        .expect("the bolt hole is cut beside the countersink");
    holds(&sunk_first, both);
    let bolted_first = plate()
        .cut_by(&bolt)
        .and_then(|plate| plate.cut_by(&countersink(5.0, -3.0)))
        .expect("the countersink is cut beside the bolt hole");
    holds(&bolted_first, both);
}

#[test]
fn a_slanted_plate_far_from_a_countersink_is_passed_over() {
    let (sin, cos) = 30.0_f64.to_radians().sin_cos();
    let tilted = Frame {
        origin: DVec3::new(-18.0, 10.0, 8.0),
        u: DVec3::X,
        v: DVec3::new(0.0, cos, sin),
    };
    let slab = Body::raised(
        &Contour::rectangle(DVec2::ZERO, DVec2::new(6.0, 6.0)),
        &[],
        tilted,
        tilted.u.cross(tilted.v) * 4.0,
    )
    .expect("the slanted plate raises");
    let plain = plate().joined(&slab).expect("the slanted plate joins");
    let sunk = plate()
        .cut_by(&countersink(5.0, -3.0))
        .and_then(|plate| plate.joined(&slab))
        .expect("the slanted plate joins beside the countersink");
    holds(&sunk, plain.volume() - 280.0 * PI / 3.0);
}

/// A frustum of radius ten at its foot and five at its top, twenty high,
/// about Y: 3500π/3.
fn frustum() -> Body {
    let section = laid(1.0, &[&[[0.0, 0.0], [20.0, 0.0], [20.0, 5.0], [0.0, 10.0]]]);
    turned(&section, about_y(), TAU).expect("the frustum turns")
}

/// A flat six from the frustum's axis, milled from two to eighteen along it:
/// it reaches the slope where the radius comes down to six, at sixteen,
/// along a hyperbola. The segment of the disc of radius `r` beyond the flat,
/// integrated over the radius, is `F`; the slope gives four along the axis
/// for one of radius.
#[test]
fn a_flat_milled_on_a_cone_holds_the_arithmetics_volume() {
    let flat = block([-15.0, 2.0, 6.0], [15.0, 18.0, 20.0]);
    let integral = |r: f64| {
        let half = (r * r - 36.0).sqrt();
        r.powi(3) / 3.0 * (6.0 / r).acos() - 4.0 * r * half + 72.0 * (r + half).ln()
    };
    let milled = frustum().cut_by(&flat).expect("the flat is milled");
    holds(
        &milled,
        3500.0 * PI / 3.0 - 4.0 * (integral(9.5) - integral(6.0)),
    );
}

/// The nodes and weights of Gauss–Legendre's rule of `count` points on
/// `[0, 1]`, each node found by Newton from Tricomi's guess.
fn gauss(count: usize) -> Vec<(f64, f64)> {
    (1..=count)
        .map(|index| {
            let legendre = |x: f64| {
                let (mut before, mut now) = (1.0, x);
                for degree in 2..=count {
                    let degree = degree as f64;
                    (before, now) = (
                        now,
                        ((2.0 * degree - 1.0) * x * now - (degree - 1.0) * before) / degree,
                    );
                }
                (now, count as f64 * (x * now - before) / (x * x - 1.0))
            };
            let mut x = (PI * (index as f64 - 0.25) / (count as f64 + 0.5)).cos();
            for _ in 0..100 {
                let (value, slope) = legendre(x);
                let step = value / slope;
                x -= step;
                if step.abs() < 1e-16 {
                    break;
                }
            }
            let (_, slope) = legendre(x);
            ((x + 1.0) / 2.0, 1.0 / ((1.0 - x * x) * slope * slope))
        })
        .collect()
}

/// Where the line through `from` along `way`, of unit length, enters and
/// leaves the frustum's cone `x² + z² = (10 − y/4)²`.
fn through_the_frustum(from: DVec3, way: DVec3) -> [f64; 2] {
    let radius = 10.0 - 0.25 * from.y;
    let a = way.x * way.x + way.z * way.z - 0.0625 * way.y * way.y;
    let b = 2.0 * (from.x * way.x + from.z * way.z) + 0.5 * radius * way.y;
    let c = from.x * from.x + from.z * from.z - radius * radius;
    let root = (b * b - 4.0 * a * c).max(0.0).sqrt();
    [(-b - root) / (2.0 * a), (-b + root) / (2.0 * a)]
}

/// What a cylinder of `radius` along `way` through `center` shares with the
/// frustum's cone: each of its lines' chord inside it, over its section, by
/// Gauss over the radius and trapezoids round the turn — the integrand is
/// smooth where every line crosses the cone's wall, and sixteen nodes agree
/// with thirty-two to the last digit or two.
fn shared_with_the_frustum(center: DVec3, way: DVec3, radius: f64) -> f64 {
    let u = way.any_orthonormal_vector();
    let v = way.cross(u);
    let turns = 64;
    gauss(16)
        .into_iter()
        .map(|(share, weight)| {
            let round: f64 = (0..turns)
                .map(|step| {
                    let (sin, cos) = (TAU * step as f64 / turns as f64).sin_cos();
                    let from = center + (u * cos + v * sin) * (radius * share);
                    let [enter, leave] = through_the_frustum(from, way);
                    leave - enter
                })
                .sum();
            weight * radius * radius * share * round * TAU / turns as f64
        })
        .sum()
}

const FRUSTUM: f64 = 3500.0 * PI / 3.0;

#[test]
fn a_radial_hole_through_a_cone_holds_its_volume() {
    let hole = post(0.0, 10.0, 2.0, -20.0, 20.0);
    let cut = frustum().cut_by(&hole).expect("the kernel answers");
    holds(
        &cut,
        FRUSTUM - shared_with_the_frustum(DVec3::Y * 10.0, DVec3::Z, 2.0),
    );
}

/// A slab two thick leaning thirty degrees off square to the frustum's
/// axis, through it at ten: it takes what lies between two ellipses, the
/// difference of the two points their planes cut off the cone, whose apex
/// stands at forty, thirty and `30 + 4/√3` from them along the axis.
#[test]
fn a_plane_at_a_slant_across_a_cone_holds_the_arithmetics_volume() {
    let (sin, cos) = 30.0_f64.to_radians().sin_cos();
    let tilted = Frame {
        origin: DVec3::Y * 10.0,
        u: DVec3::X,
        v: DVec3::new(0.0, sin, cos),
    };
    let slab = Body::raised(
        &Contour::rectangle(DVec2::splat(-20.0), DVec2::splat(20.0)),
        &[],
        tilted,
        tilted.u.cross(tilted.v) * 2.0,
    )
    .expect("the slanted slab raises");
    let far = 30.0 + 4.0 / 3f64.sqrt();
    let between = PI / 48.0 * (far.powi(3) - 30f64.powi(3)) / (47.0f64 / 48.0).powf(1.5);
    let cut = frustum().cut_by(&slab).expect("the slab cuts across");
    holds(&cut, 3500.0 * PI / 3.0 - between);
}

#[test]
fn a_cylinder_at_a_skew_angle_across_a_cone_s_slope_alone_holds_its_volume() {
    let along = DVec3::new(2.0, 1.0, 0.0).normalize();
    let skew = Frame {
        origin: DVec3::Y * 10.0 - along * 20.0,
        u: DVec3::Z,
        v: along.cross(DVec3::Z),
    };
    let hole = Body::raised(&Contour::circle(DVec2::ZERO, 1.0), &[], skew, along * 40.0)
        .expect("the skew rod raises");
    let cut = frustum().cut_by(&hole).expect("the kernel answers");
    holds(
        &cut,
        FRUSTUM - shared_with_the_frustum(DVec3::Y * 10.0, along, 1.0),
    );
}

/// A taper whose axis leans ten degrees off the frustum's, its slope of
/// eight and a half and the frustum's of fourteen: each one's rulings fall
/// within the other's slope, and neither's meet the other in a true
/// quadratic all round — still unsupported.
#[test]
fn a_cone_within_another_s_slope_either_way_declines_as_unsupported() {
    let (sin, cos) = 10f64.to_radians().sin_cos();
    let leaning = Turning {
        origin: DVec3::new(2.0, 0.0, 0.0),
        along: DVec3::new(-sin, cos, 0.0),
        side: DVec3::new(cos, sin, 0.0),
    };
    let taper = laid(1.0, &[&[[0.0, 0.0], [20.0, 0.0], [20.0, 3.0], [0.0, 6.0]]]);
    let tool = turned(&taper, leaning, TAU).expect("the taper turns");
    assert_eq!(frustum().cut_by(&tool), Err(Declined::Unsupported));
}
/// A truncated cone about X through the frustum's axis at ten, from radius
/// one at four to three at nine, its apex at one and a half inside the
/// frustum: it crosses the frustum's slope, and every ray from its apex
/// runs inside the frustum from the plane at four, `2.5/cos ψ` out, to the
/// frustum's wall, before eight and a quarter, short of the plane at nine.
/// What they share is each ray's `(ρ_out³ − ρ_in³)/3` over the tool's solid
/// angle.
#[test]
fn a_cone_of_another_axis_across_a_cone_s_slope_holds_its_volume() {
    let about_x = Turning {
        origin: DVec3::Y * 10.0,
        along: DVec3::X,
        side: DVec3::Y,
    };
    let sides = laid(1.0, &[&[[4.0, 0.0], [9.0, 0.0], [9.0, 3.0], [4.0, 1.0]]]);
    let tool = turned(&sides, about_x, TAU).expect("the truncated cone turns");
    let apex = DVec3::new(1.5, 10.0, 0.0);
    let half = 0.4f64.atan();
    let turns = 64;
    let shared: f64 = gauss(16)
        .into_iter()
        .map(|(share, weight)| {
            let tilt = share * half;
            let round: f64 = (0..turns)
                .map(|step| {
                    let (sin, cos) = (TAU * step as f64 / turns as f64).sin_cos();
                    let ray =
                        DVec3::X * tilt.cos() + (DVec3::Y * cos + DVec3::Z * sin) * tilt.sin();
                    let [_, leave] = through_the_frustum(apex, ray);
                    let enter = 2.5 / tilt.cos();
                    (leave.powi(3) - enter.powi(3)) / 3.0
                })
                .sum();
            weight * half * tilt.sin() * round * TAU / turns as f64
        })
        .sum();
    let truncated = PI * 5.0 / 3.0 * (1.0 + 3.0 + 9.0);
    holds(
        &frustum().cut_by(&tool).expect("the kernel answers"),
        FRUSTUM - shared,
    );
    holds(
        &tool.cut_by(&frustum()).expect("the kernel answers"),
        truncated - shared,
    );
}

/// Two points about X through the frustum's axis at ten, tip to tip at
/// their feet at nine: the one whose apex stands inside the frustum, at
/// four, crosses its slope all round its own axis, which parts that apex
/// from the rest — built; every ray of it leaves the frustum's cone before
/// eight and a quarter, short of its foot, and the other point stands
/// outside the frustum. What they share is each ray's `ρ³/3` out to the
/// frustum's wall, over that point's solid angle.
#[test]
fn two_points_of_another_axis_across_a_cone_s_slope_hold_their_volume() {
    let about_x = Turning {
        origin: DVec3::Y * 10.0,
        along: DVec3::X,
        side: DVec3::Y,
    };
    let tips = laid(1.0, &[&[[4.0, 0.0], [14.0, 0.0], [9.0, 3.0]]]);
    let tool = turned(&tips, about_x, TAU).expect("the two points turn");
    let apex = DVec3::new(4.0, 10.0, 0.0);
    let half = 0.6f64.atan();
    let turns = 64;
    let shared: f64 = gauss(16)
        .into_iter()
        .map(|(share, weight)| {
            let tilt = share * half;
            let round: f64 = (0..turns)
                .map(|step| {
                    let (sin, cos) = (TAU * step as f64 / turns as f64).sin_cos();
                    let ray =
                        DVec3::X * tilt.cos() + (DVec3::Y * cos + DVec3::Z * sin) * tilt.sin();
                    let [_, leave] = through_the_frustum(apex, ray);
                    leave.powi(3) / 3.0
                })
                .sum();
            weight * half * tilt.sin() * round * TAU / turns as f64
        })
        .sum();
    let points = 2.0 * PI * 9.0 * 5.0 / 3.0;
    holds(
        &frustum().cut_by(&tool).expect("the kernel answers"),
        FRUSTUM - shared,
    );
    holds(
        &tool.cut_by(&frustum()).expect("the kernel answers"),
        points - shared,
    );
}

/// A hole along Z through the side of a point about Y, its apex outside the
/// hole: the curve does not go round the point's axis, and lies on a face
/// holding its apex — still declined until the cone's development is built.
#[test]
fn a_radial_hole_through_a_point_s_side_still_declines_as_unsupported() {
    let point = turned(&point(), about_y(), TAU).expect("the point turns");
    assert_eq!(
        point.cut_by(&post(0.0, 3.0, 1.0, -20.0, 20.0)),
        Err(Declined::Unsupported)
    );
}
#[test]
fn a_point_resting_on_a_plate_by_its_tip_a_hair_into_it_keeps_every_rule() {
    let tip_down = laid(1.0, &[&[[0.0, 0.0], [10.0, 5.0], [10.0, 0.0]]]);
    let slab = block([-20.0, -20.0, -10.0], [20.0, 20.0, 0.0]);
    let hair = slab.scale().eps();
    for depth in [0.0, 0.5 * hair, -0.5 * hair] {
        let standing = Turning {
            origin: DVec3::Z * -depth,
            along: DVec3::Z,
            side: DVec3::X,
        };
        let point = turned(&tip_down, standing, TAU).expect("the point turns");
        let joined = slab
            .joined(&point)
            .unwrap_or_else(|declined| panic!("{declined:?} at a depth of {depth}"));
        holds(&joined, 16000.0 + 250.0 * PI / 3.0);
    }
}

/// Every body above with a cone in it, as each test builds it.
fn bodies_with_cones() -> Vec<Body> {
    let mut bodies = Vec::new();
    for angle in [TAU, FRAC_PI_2] {
        for straight in [chamfered_shaft(), point()] {
            bodies.push(turned(&straight, slanted(), angle).expect("the profile turns"));
        }
    }
    bodies.push(
        plate()
            .cut_by(&countersink(5.0, -3.0))
            .expect("the countersink cuts"),
    );
    let whole = turned(&point(), about_y(), TAU).expect("the point turns");
    bodies.push(
        whole
            .cut_by(&block([0.0, -1.0, -10.0], [10.0, 11.0, 10.0]))
            .expect("the block halves the point"),
    );
    bodies.push(frustum());
    let rod = laid(1.0, &[&[[0.0, 0.0], [10.0, 0.0], [10.0, 5.0], [0.0, 5.0]]]);
    let tip = laid(1.0, &[&[[10.0, 0.0], [15.0, 0.0], [10.0, 5.0]]]);
    let upper = laid(1.0, &[&[[10.0, 0.0], [16.0, 0.0], [16.0, 3.0]]]);
    let wide = laid(1.0, &[&[[0.0, 0.0], [12.0, 0.0], [0.0, 6.0]]]);
    let slender = laid(1.0, &[&[[-1.0, 0.0], [15.0, 0.0], [-1.0, 4.0]]]);
    for (one, other) in [(&rod, &tip), (&point(), &upper)] {
        let [one, other] = [one, other]
            .map(|straight| turned(straight, slanted(), TAU).expect("the profile turns"));
        bodies.push(one.joined(&other).expect("the two turns join"));
    }
    let [wide, slender] =
        [&wide, &slender].map(|straight| turned(straight, slanted(), TAU).expect("it turns"));
    bodies.push(wide.cut_by(&slender).expect("the slender cone cuts"));
    let quarter = turned(&point(), about_y(), FRAC_PI_2).expect("the point turns");
    bodies.push(
        quarter
            .joined(&block([-10.0, 4.0, -10.0], [10.0, 12.0, 10.0]))
            .expect("the block joins the point"),
    );
    bodies
}

#[test]
fn every_cone_body_above_is_drawn_closed_and_uncrossed() {
    for body in bodies_with_cones() {
        let reach = body.scale().reach();
        for fraction in [1e-1, 1e-2, 1e-4] {
            let triangles = body.triangles(fraction * reach);
            assert_eq!(closed(&triangles), Ok(()), "at {fraction} of the reach");
            assert_eq!(uncrossed(&triangles), Ok(()), "at {fraction} of the reach");
        }
    }
}

#[cfg(feature = "campaigns")]
mod campaign {
    use super::*;
    use cao_solid::soundness::Random;
    use std::time::{Duration, Instant, SystemTime};

    /// A turned leaf drawn from a seed: a stepped profile on or off its
    /// axis, now and then holed, turned about a line of a plane of the origin
    /// by a lattice angle or a whole turn, either way.
    fn leaf(random: &mut Random) -> (Straight, Turning, f64) {
        let steps = 1 + random.below(4);
        let low = if random.chance(0.4) {
            0.0
        } else {
            random.on_lattice(1.0, 6.0, 0.5)
        };
        let mut heights = vec![random.on_lattice(-6.0, 6.0, 0.5)];
        let mut radii = Vec::new();
        for _ in 0..steps {
            heights.push(heights[heights.len() - 1] + random.on_lattice(1.0, 8.0, 0.5));
            radii.push(low + random.on_lattice(1.0, 8.0, 0.5));
        }
        let mut corners = vec![[heights[0], low], [heights[steps], low]];
        for step in (0..steps).rev() {
            corners.push([heights[step + 1], radii[step]]);
            corners.push([heights[step], radii[step]]);
        }
        corners.dedup();
        let hole = (random.chance(0.15) && steps == 1).then(|| {
            let (from, to, radius) = (heights[0], heights[1], radii[0]);
            let width = (to - from) / 4.0;
            let depth = (radius - low) / 4.0;
            let (h0, r0) = (from + width, low + depth);
            [
                [h0, r0],
                [h0, r0 + depth],
                [h0 + width, r0 + depth],
                [h0 + width, r0],
            ]
        });
        let side = if random.chance(0.5) { 1.0 } else { -1.0 };
        let mut contours: Vec<&[[f64; 2]]> = vec![&corners];
        if let Some(hole) = &hole {
            contours.push(hole);
        }
        let straight = laid(side, &contours);
        let planes = [
            (DVec3::X, DVec3::Y),
            (DVec3::Y, DVec3::X),
            (DVec3::X, DVec3::Z),
            (DVec3::Z, DVec3::X),
            (DVec3::Y, DVec3::Z),
            (DVec3::Z, DVec3::Y),
        ];
        let (along, across) = *random.pick(&planes);
        let offset = if random.chance(0.5) {
            0.0
        } else {
            random.on_lattice(-3.0, 3.0, 1.0)
        };
        let turning = Turning {
            origin: across * offset,
            along,
            side: across,
        };
        let degrees = if random.chance(0.4) {
            360.0
        } else if random.chance(0.6) {
            *random.pick(&[90.0, 180.0, 270.0])
        } else {
            random.on_lattice(15.0, 345.0, 15.0)
        };
        let sign = if random.chance(0.3) { -1.0 } else { 1.0 };
        (straight, turning, sign * f64::to_radians(degrees))
    }

    /// A block or a bore drawn near the leaf: on the planes of the origin, on
    /// the lattice.
    fn prism(random: &mut Random) -> Body {
        if random.chance(0.5) {
            let low = [
                random.on_lattice(-10.0, 4.0, 1.0),
                random.on_lattice(-10.0, 4.0, 1.0),
                random.on_lattice(-10.0, 4.0, 1.0),
            ];
            let size = [
                random.on_lattice(1.0, 10.0, 1.0),
                random.on_lattice(1.0, 10.0, 1.0),
                random.on_lattice(1.0, 10.0, 1.0),
            ];
            block(low, [low[0] + size[0], low[1] + size[1], low[2] + size[2]])
        } else {
            let radius = random.on_lattice(0.5, 6.0, 0.5);
            let from = random.on_lattice(-12.0, 0.0, 1.0);
            bore(radius, from, from + random.on_lattice(2.0, 20.0, 1.0))
        }
    }

    /// What went wrong with a case, if anything; a decline is counted apart.
    fn check(seed: u64) -> Result<bool, String> {
        let mut random = Random::seeded(seed);
        let (straight, turning, angle) = leaf(&mut random);
        let body = match turned(&straight, turning, angle) {
            Ok(body) => body,
            Err(_) => return Ok(false),
        };
        let rules = |body: &Body| -> Result<(), String> {
            let reach = body.scale().reach();
            listed(&body.listing(), reach).map_err(|flaw| format!("{flaw:?}"))?;
            let triangles = body.triangles(1e-3 * reach);
            closed(&triangles).map_err(|flaw| format!("{flaw:?}"))?;
            uncrossed(&triangles).map_err(|flaw| format!("{flaw:?}"))
        };
        rules(&body)?;
        let volume = pappus(&straight, angle);
        if (body.volume() - volume).abs() > 1e-9 * volume {
            return Err(format!("volume {} against {volume}", body.volume()));
        }
        let tool = prism(&mut random);
        let (Ok(joined), Ok(cut)) = (body.joined(&tool), body.cut_by(&tool)) else {
            return Ok(false);
        };
        rules(&joined)?;
        rules(&cut)?;
        let promised = cut.volume() + tool.volume();
        if (joined.volume() - promised).abs() > 1e-7 * promised {
            return Err(format!(
                "joined {} against cut and tool {promised}",
                joined.volume()
            ));
        }
        Ok(true)
    }

    fn from_the_environment(name: &str) -> Option<u64> {
        std::env::var(name).ok()?.parse().ok()
    }

    #[test]
    #[ignore = "a campaign, run by hand: see the head of this file"]
    fn a_campaign_of_turned_leaves_and_prisms_keeps_every_rule() {
        let seconds = from_the_environment("CAO_FUZZ_SECONDS").unwrap_or(60);
        let first = from_the_environment("CAO_FUZZ_SEED").unwrap_or_else(|| {
            SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .map_or(0, |since| since.as_secs())
        });
        let cases = from_the_environment("CAO_FUZZ_CASES").unwrap_or(u64::MAX);
        let started = Instant::now();
        let (mut kept, mut declined, mut broken) = (0, 0, Vec::new());
        let mut seed = first;
        while seed - first < cases && started.elapsed() < Duration::from_secs(seconds) {
            match std::panic::catch_unwind(|| check(seed)) {
                Ok(Ok(true)) => kept += 1,
                Ok(Ok(false)) => declined += 1,
                Ok(Err(flaw)) => broken.push((seed, flaw)),
                Err(_) => broken.push((seed, "panicked".to_string())),
            }
            seed += 1;
        }
        println!(
            "seeds {first}..{seed}: {kept} kept, {declined} declined, {} broken",
            broken.len()
        );
        for (seed, flaw) in &broken {
            println!("  seed {seed}: {flaw}");
        }
        assert!(broken.is_empty());
    }
}
