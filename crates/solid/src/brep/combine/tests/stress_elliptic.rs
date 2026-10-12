//! Stresses of joins and cuts on elliptic walls (#571), run by hand: blocks
//! whose sides cross an elliptic boss, touch it or stand a hair off it, and
//! slabs at a slant, profiles of arcs of ellipse and raises tilted off the
//! ground. Each case is held to its listing, to triangles closed and
//! uncrossed at two tolerances, and to the volumes of its join and its cuts
//! adding up, for every seed of [`SEEDS`]:
//!
//! ```text
//! cargo test --release -p cao_solid --lib a_stress_of -- --ignored
//! ```

use std::f64::consts::{PI, TAU};
use std::ops::Range;

use glam::{DVec2, DVec3};

use super::*;
use crate::soundness::{closed, uncrossed};

/// The same cases for the same seed.
struct Lcg(u64);

impl Lcg {
    fn next(&mut self) -> f64 {
        self.0 = self
            .0
            .wrapping_mul(6364136223846793005)
            .wrapping_add(1442695040888963407);
        ((self.0 >> 11) as f64) / ((1u64 << 53) as f64)
    }

    fn pick<T: Copy>(&mut self, from: &[T]) -> T {
        from[((self.next() * from.len() as f64) as usize).min(from.len() - 1)]
    }
}

/// The seeds each stress runs.
const SEEDS: Range<u64> = 1..20_001;

/// The ellipse about `center` whose major half-axis is `first` and whose
/// minor one is `minor` long, raised from `from` to `to`.
fn oval(center: DVec2, first: DVec2, minor: f64, from: f64, to: f64) -> Option<Body> {
    let outline = Contour {
        corners: vec![center + first],
        runs: vec![Run::Oval {
            center,
            first,
            second: minor,
            turn: TAU,
        }],
    };
    Body::raised(&outline, &[], ground(from), DVec3::Z * (to - from)).ok()
}

fn check(body: &Body) -> Result<(), String> {
    listed(&body.listing(), body.scale().reach()).map_err(|why| format!("listing {why:?}"))?;
    for share in [1e-3, 2e-2] {
        let triangles = body.triangles(share * body.scale().reach());
        closed(&triangles).map_err(|why| format!("open at {share}: {why:?}"))?;
        uncrossed(&triangles).map_err(|why| format!("crossed at {share}: {why:?}"))?;
    }
    Ok(())
}

/// What went wrong with the join and the two cuts of `boss` and `tool`, the
/// volumes of the three held to the two operands' within what `room` gives
/// for the join's.
fn failures_of(boss: &Body, tool: &Body, room: fn(f64) -> f64) -> Vec<String> {
    let results = [boss.joined(tool), boss.cut_by(tool), tool.cut_by(boss)];
    let [Ok(union), Ok(difference), Ok(cut)] = results else {
        let declined: Vec<Option<Declined>> = results
            .iter()
            .map(|result| result.as_ref().err().copied())
            .collect();
        return vec![format!("declined {declined:?}")];
    };
    let mut failures: Vec<String> = [
        ("union", &union),
        ("difference", &difference),
        ("cut", &cut),
    ]
    .into_iter()
    .filter_map(|(name, body)| check(body).err().map(|why| format!("{name}: {why}")))
    .collect();
    let [u, d, c, b, t] = [&union, &difference, &cut, boss, tool].map(Body::volume);
    if (u - (d + t)).abs() > room(u) || (u - (c + b)).abs() > room(u) {
        failures.push(format!("volumes {u} {d} {c} {b} {t}"));
    }
    failures
}

/// A slab above the plane through `origin` whose normal is `normal`.
fn slab(origin: DVec3, normal: DVec3) -> Body {
    let normal = normal.normalize();
    let u = normal.cross(DVec3::new(0.3, 1.0, 0.2)).normalize();
    let frame = Frame {
        origin,
        u,
        v: normal.cross(u),
    };
    Body::raised(
        &Contour::rectangle(DVec2::new(-30.0, -30.0), DVec2::new(30.0, 30.0)),
        &[],
        frame,
        normal * 30.0,
    )
    .expect("the slab raises")
}

/// A whole ellipse, an arc of one closed by a trait, or a slot whose ends
/// are halves of two ellipses.
fn profile(random: &mut Lcg, center: DVec2, first: DVec2, minor: f64) -> Contour {
    let at =
        |angle: f64| center + first * angle.cos() + first.perp().normalize() * minor * angle.sin();
    match random.pick(&[0, 1, 2]) {
        0 => Contour {
            corners: vec![center + first],
            runs: vec![Run::Oval {
                center,
                first,
                second: minor,
                turn: TAU,
            }],
        },
        1 => {
            let from: f64 = random.pick(&[-0.5, 0.3, 1.0]);
            let sweep: f64 = random.pick(&[2.0, 3.0, 4.5]);
            Contour {
                corners: vec![at(from), at(from + sweep)],
                runs: vec![
                    Run::Oval {
                        center,
                        first,
                        second: minor,
                        turn: sweep,
                    },
                    Run::Straight,
                ],
            }
        }
        _ => {
            let along = first.normalize();
            let across = along.perp() * minor;
            let length: f64 = random.pick(&[2.0, 5.0]);
            let [left, right] = [center - along * length, center + along * length];
            let reach: f64 = random.pick(&[1.0, 2.5]);
            Contour {
                corners: vec![left - across, right - across, right + across, left + across],
                runs: vec![
                    Run::Straight,
                    Run::Oval {
                        center: right,
                        first: -across,
                        second: reach,
                        turn: PI,
                    },
                    Run::Straight,
                    Run::Oval {
                        center: left,
                        first: across,
                        second: reach * 0.7,
                        turn: PI,
                    },
                ],
            }
        }
    }
}

#[test]
#[ignore = "a stress, run by hand"]
fn a_stress_of_elliptic_profiles_slanted_and_tilted_keeps_every_rule() {
    let mut failures = Vec::new();
    for seed in SEEDS {
        let mut random = Lcg(seed.wrapping_mul(2654435761).wrapping_add(17));
        let center = DVec2::new(
            random.pick(&[-2.0, 0.0, 1.5]),
            random.pick(&[-1.0, 0.0, 2.0]),
        );
        let major: f64 = random.pick(&[2.0, 4.0, 7.0]);
        let minor = random.pick(&[1.0f64, 1.5, 3.0]).min(major * 0.8);
        let turned = random.pick(&[0.0, PI / 2.0, 0.4, 1.1, PI / 4.0]);
        let first = DVec2::from_angle(turned) * major;
        let contour = profile(&mut random, center, first, minor);
        let mode = random.pick(&[0, 1, 2]);
        let tilt: f64 = random.pick(&[0.05, 0.3, 0.8]);
        let way: f64 = random.pick(&[0.0, 1.0, 2.5]);
        let lean = DVec3::new(way.cos(), way.sin(), 0.0);
        let normal = if mode == 2 {
            (DVec3::Z * tilt.cos() + lean * tilt.sin()).normalize()
        } else {
            DVec3::Z
        };
        let u = normal.cross(DVec3::Y).normalize();
        let frame = Frame {
            origin: DVec3::ZERO,
            u,
            v: normal.cross(u),
        };
        let height = random.pick(&[3.0, 4.0]);
        let Ok(boss) = Body::raised(&contour, &[], frame, normal * height) else {
            failures.push(format!("seed {seed}: {contour:?} does not raise"));
            continue;
        };
        let tool = match mode {
            1 => {
                let slant: f64 = random.pick(&[0.05, 0.4, 1.0, 1.3]);
                let normal = DVec3::Z * slant.cos() + lean * slant.sin();
                let at = DVec3::new(center.x, center.y, random.pick(&[1.0, 2.0, 2.5]));
                slab(at, normal)
            }
            _ => {
                let x0 = random.pick(&[center.x - 1.0, center.x, center.x + 0.5, center.x - 9.0]);
                let y0 = random.pick(&[center.y - 1.0, center.y, center.y - 9.0]);
                let z0 = random.pick(&[-1.0, 0.0, 1.0, 2.0]);
                let wide = random.pick(&[3.0, 12.0]);
                block([x0, y0, z0], [x0 + 6.0, y0 + wide, z0 + 2.0])
            }
        };
        for why in failures_of(&boss, &tool, |union| 1e-6 * union) {
            failures.push(format!(
                "seed {seed}, mode {mode}, tilt {tilt}, {contour:?}: {why}"
            ));
        }
    }
    assert!(failures.is_empty(), "{}", failures.join("\n"));
}

#[test]
#[ignore = "a stress, run by hand"]
fn a_stress_of_blocks_against_elliptic_bosses_keeps_every_rule() {
    let mut failures = Vec::new();
    for seed in SEEDS {
        let mut random = Lcg(seed.wrapping_mul(2654435761));
        let center = DVec2::new(
            random.pick(&[-2.0, 0.0, 1.5, 3.0]),
            random.pick(&[-1.0, 0.0, 2.0]),
        );
        let major: f64 = random.pick(&[2.0, 4.0, 7.0]);
        let minor = random.pick(&[1.0f64, 1.5, 3.0]).min(major * 0.8);
        let turned = random.pick(&[0.0, PI / 2.0, 0.4, 1.1, PI / 4.0]);
        let first = DVec2::from_angle(turned) * major;
        let (from, to) = (
            random.pick(&[-1.0, 0.0, 1.0]),
            random.pick(&[3.0, 4.0, 5.0]),
        );
        let Some(boss) = oval(center, first, minor, from, to) else {
            failures.push(format!("seed {seed}: the oval does not raise"));
            continue;
        };
        let (sx, sy) = match boss.surfaces.iter().find_map(|surface| match surface {
            Surface::Elliptic(wall) => Some(*wall),
            _ => None,
        }) {
            Some(wall) => (wall.support(DVec3::X), wall.support(DVec3::Y)),
            None => (major, major),
        };
        let eps = 2e-8;
        let hair = random.pick(&[0.0, 0.0, 0.0, eps / 3.0, -eps / 3.0, 3.0 * eps, 1e-6]);
        let x0 = random.pick(&[
            center.x - sx - 3.0,
            center.x - sx + hair,
            center.x - 1.0,
            center.x,
            center.x + 0.5,
        ]);
        let x1 = random.pick(&[
            center.x + sx + 3.0,
            center.x + sx + hair,
            center.x + 1.0,
            center.x + 2.0,
            x0 + 8.0,
        ]);
        let y0 = random.pick(&[center.y - sy - 2.0, center.y - sy + hair, center.y - 1.0]);
        let y1 = random.pick(&[center.y + sy + 2.0, center.y + sy + hair, center.y + 0.5]);
        let z0 = random.pick(&[from, from - 1.0, 1.0, 2.0]);
        let z1 = random.pick(&[to, to + 1.0, 2.5, 3.0]);
        if !(x1 > x0 + 0.1 && y1 > y0 + 0.1 && z1 > z0 + 0.1) {
            continue;
        }
        let tool = block([x0, y0, z0], [x1, y1, z1]);
        for why in failures_of(&boss, &tool, |_| 1e-6) {
            failures.push(format!(
                "seed {seed}: center {center}, major {major}, minor {minor}, turned {turned}, \
                 z {from}..{to}, block [{x0}, {y0}, {z0}]..[{x1}, {y1}, {z1}]: {why}"
            ));
        }
    }
    assert!(failures.is_empty(), "{}", failures.join("\n"));
}
