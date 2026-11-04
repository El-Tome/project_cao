//! The rulings two parallel walls meet along, read round an elliptic one:
//! from the angles of it at which their distance turns, a touch where a
//! foot stands within the tolerance of the other wall, a crossing halved for
//! between two feet whose gaps part in sign.

use std::f64::consts::TAU;

use glam::{DVec2, DVec3};

use super::Ruled;
use crate::brep::curve::Line;
use crate::brep::scale::Scale;
use crate::brep::surface::{Cylinder, EllipticCylinder};

/// Halvings of a crossing's bracket: past the fifty-three bits of a double
/// the midpoint stands on an end, and the search stops there.
const HALVINGS: usize = 1100;

/// How many equal steps a stretch between two feet is read at, against an
/// elliptic wall: the widest gap is flat at its top, and a sixteenth of the
/// stretch misses it by a few in a thousand.
const STRETCH_READS: usize = 16;

/// The wall an elliptic one is measured against.
#[derive(Clone, Copy)]
pub(super) enum Other<'a> {
    Round(&'a Cylinder),
    Oval(&'a EllipticCylinder),
}

impl Other<'_> {
    /// How far `point` stands outside the wall, negative inside.
    fn gap(self, point: DVec3) -> f64 {
        match self {
            Other::Round(round) => round.distance(point),
            Other::Oval(wall) => wall.distance(point),
        }
    }

    /// Above nought outside the wall, below inside: the distance for a
    /// circular wall, the section's own equation for an elliptic one.
    fn side(self, point: DVec3) -> f64 {
        match self {
            Other::Round(round) => round.distance(point),
            Other::Oval(wall) => {
                let from = point - wall.origin;
                let (x, y) = (from.dot(wall.u) / wall.major, from.dot(wall.v) / wall.minor);
                (x - 1.0) * (x + 1.0) + y * y
            }
        }
    }

    /// The point of the wall nearest `point`.
    fn nearest(self, point: DVec3) -> DVec3 {
        match self {
            Other::Round(round) => {
                let from = point - round.origin;
                let foot = round.origin + round.axis * round.axis.dot(from);
                foot + (point - foot).normalize_or_zero() * round.radius
            }
            Other::Oval(wall) => wall.nearest(point),
        }
    }
}

/// The rulings two walls meet along, the feet of the normal they share at
/// each touch — on the wall the pair is read round, and on the other — and
/// the least gap between them at the feet of the normals.
pub(super) struct Met {
    pub(super) rulings: Vec<Ruled>,
    pub(super) feet: Vec<[DVec3; 2]>,
    pub(super) nearest: f64,
}

/// The rulings `walked` meets `other` along, from the angles of `walked`
/// at which their distance turns, in order round it; none where the two
/// stand within decision 8's hair of each other at every one of those
/// angles.
pub(super) fn met(
    walked: &EllipticCylinder,
    other: Other,
    angles: Vec<f64>,
    scale: Scale,
) -> Option<Met> {
    let eps = scale.eps();
    let point = |theta: f64| walked.point(DVec2::new(theta, 0.0));
    let gaps: Vec<f64> = angles
        .iter()
        .map(|&theta| other.gap(point(theta)))
        .collect();
    if widest(walked, other, &angles, &gaps) <= Scale::HAIR * eps {
        return None;
    }
    let touching: Vec<bool> = gaps.iter().map(|gap| gap.abs() <= eps).collect();
    let start = touching.iter().position(|touch| !touch)?;
    let count = angles.len();
    let turned = |index: usize| {
        let at = (start + index) % count;
        let turns = ((start + index) / count) as f64;
        (at, angles[at] + TAU * turns)
    };
    let mut found = Met {
        rulings: Vec::new(),
        feet: Vec::new(),
        nearest: gaps
            .iter()
            .map(|gap| gap.abs())
            .fold(f64::INFINITY, f64::min),
    };
    let mut index = 0;
    while index < count {
        let (at, from) = turned(index);
        if touching[at] {
            let mut nearest = at;
            while index < count && touching[turned(index).0] {
                let (run, _) = turned(index);
                if gaps[run].abs() < gaps[nearest].abs() {
                    nearest = run;
                }
                index += 1;
            }
            let here = point(angles[nearest]);
            let there = other.nearest(here);
            found.rulings.push(Ruled {
                line: Line::through((here + there) / 2.0, walked.axis),
                touching: true,
            });
            found.feet.push([here, there]);
            continue;
        }
        let (next, to) = turned(index + 1);
        if !touching[next] && (gaps[at] > 0.0) != (gaps[next] > 0.0) {
            let theta = crossing([from, to], gaps[at] > 0.0, |theta| other.side(point(theta)));
            found.rulings.push(Ruled {
                line: Line::through(point(theta), walked.axis),
                touching: false,
            });
        }
        index += 1;
    }
    Some(found)
}

/// How far apart two walls stand at most. Against a circular wall, at the
/// feet of the normals, where the distance turns. Against an elliptic one,
/// the feet were found through the map that makes it round, which keeps
/// where the two cross but not how far apart they stand: the same ellipse
/// turned a hair stands furthest from itself a quarter of the way between
/// two feet, a fifth further than at either (5585704198). There the
/// distance is read again along each stretch between two feet.
fn widest(walked: &EllipticCylinder, other: Other, angles: &[f64], gaps: &[f64]) -> f64 {
    let at_feet = gaps.iter().map(|gap| gap.abs()).fold(0.0, f64::max);
    if matches!(other, Other::Round(_)) {
        return at_feet;
    }
    let count = angles.len();
    (0..count)
        .flat_map(|index| {
            let from = angles[index];
            let to = angles.get(index + 1).copied().unwrap_or(angles[0] + TAU);
            (1..STRETCH_READS)
                .map(move |step| from + (to - from) * step as f64 / STRETCH_READS as f64)
        })
        .map(|theta| other.gap(walked.point(DVec2::new(theta, 0.0))).abs())
        .fold(at_feet, f64::max)
}

/// The angle in `bracket` where `side` changes sign, by halving: above
/// nought at its low end when `outside`, below it otherwise.
fn crossing([mut low, mut high]: [f64; 2], outside: bool, side: impl Fn(f64) -> f64) -> f64 {
    let mut middle = (low + high) / 2.0;
    for _ in 0..HALVINGS {
        middle = (low + high) / 2.0;
        if middle <= low || middle >= high {
            break;
        }
        let value = side(middle);
        if value == 0.0 {
            break;
        }
        if (value > 0.0) == outside {
            low = middle;
        } else {
            high = middle;
        }
    }
    middle
}
