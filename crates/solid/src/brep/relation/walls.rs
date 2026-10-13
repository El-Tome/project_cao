//! Two parallel walls, an elliptic one among them (#571): a circular wall
//! against an elliptic one, and two elliptic walls.
//!
//! Seen square to their axes they are a circle `(q, r)` and an ellipse
//! `E(θ)`, read in the ellipse's own frame. The distance `|E(θ) − q| − r`
//! only grows or only falls between two consecutive feet of the normals the
//! ellipse sends through `q` (`normals.rs`), so each stretch between them
//! crosses the circle once at most, where the distance changes sign along
//! it, found by halving. A foot where the two walls stand within the
//! tolerance of each other is a touch, decided once there and built as one
//! ruling halfway between them, as two circular walls a hair from touching
//! are; a stretch ending at a touch crosses nothing, the touch standing for
//! any crossing within its band, and touches at consecutive feet are one,
//! at the foot where the walls stand nearest. Four crossings at most, or
//! touches among them.
//!
//! Two elliptic walls are read through the affine frame taking the section
//! of one of them, the rounder, to the unit circle: the other's becomes an
//! ellipse, read in its own principal frame, and the pair a circle against
//! an ellipse. Crossings survive the map, and the feet of the mapped normals
//! still bracket them; each touch is decided on the world's gap, the foot
//! mapped back and measured against the rounder wall.
//!
//! Two crossings are `Lines` and one touch `Tangent`, as for two circular
//! walls; any other set of rulings — four crossings, two touches, a touch
//! beside crossings — is `Parallel`, each ruling with whether the walls only
//! touch along it. Walls standing within the tolerance of each other at
//! every foot, which no ruling tells apart, are unsupported.

mod normals;

use glam::{DVec2, DVec3};

use super::{Relation, parallel, sorted};
use crate::brep::curve::Line;
use crate::brep::scale::Scale;
use crate::brep::surface::{Cylinder, EllipticCylinder, Surface};

/// A ruling two parallel walls meet along, and whether they only touch
/// along it rather than cross.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Ruled {
    pub line: Line,
    pub touching: bool,
}

/// Halvings of a crossing's bracket: past the fifty-three bits of a double
/// the midpoint stands on an end, and the search stops there.
const HALVINGS: usize = 1100;

/// A circular wall and an elliptic one about parallel axes.
pub(super) fn round_and_elliptic(
    round: &Cylinder,
    wall: &EllipticCylinder,
    scale: Scale,
) -> Relation {
    if !parallel(round.axis, wall.axis, scale) {
        return Relation::Unsupported;
    }
    met_round(round, wall, scale).map_or(Relation::Unsupported, |met| ruled(met.rulings))
}

/// Two elliptic walls about parallel axes, neither the other within the
/// tolerance.
pub(super) fn elliptic_pair(
    one: &EllipticCylinder,
    other: &EllipticCylinder,
    scale: Scale,
) -> Relation {
    met_ovals(one, other, scale).map_or(Relation::Unsupported, |(met, _)| ruled(met.rulings))
}

/// Whether two surfaces are walls about parallel axes, one of them
/// elliptic at least: the pairs `walls.rs` reads.
pub(in crate::brep) fn abreast(first: &Surface, second: &Surface) -> bool {
    matches!(
        (first, second),
        (
            Surface::Elliptic(_),
            Surface::Elliptic(_) | Surface::Cylinder(_)
        ) | (Surface::Cylinder(_), Surface::Elliptic(_))
    )
}

/// Where two parallel walls, an elliptic one among them, decided to touch
/// along rulings stand nearest each other at each touch: the feet of the
/// normal they share there, on `one` and on `other`. None for any other
/// pair.
pub(in crate::brep) fn feet_of_touches(
    one: &Surface,
    other: &Surface,
    scale: Scale,
) -> Vec<[DVec3; 2]> {
    read(one, other, scale).map_or_else(Vec::new, |(met, swapped)| {
        met.feet
            .into_iter()
            .map(|[walked, other]| {
                if swapped {
                    [other, walked]
                } else {
                    [walked, other]
                }
            })
            .collect()
    })
}

/// How near two parallel walls, an elliptic one among them, come where the
/// distance between them turns: the least of their gaps at the feet of the
/// normals one sends through the other, nought where they touch, a hair
/// where they cross a hair from touching. None for any other pair.
pub(in crate::brep) fn closest(one: &Surface, other: &Surface, scale: Scale) -> Option<f64> {
    read(one, other, scale).map(|(met, _)| met.nearest)
}

/// The pair read round its elliptic wall, or the less round of two, and
/// whether that wall is `other`.
fn read(one: &Surface, other: &Surface, scale: Scale) -> Option<(Met, bool)> {
    match (one, other) {
        (Surface::Cylinder(round), Surface::Elliptic(wall))
            if parallel(round.axis, wall.axis, scale) =>
        {
            Some((met_round(round, wall, scale)?, true))
        }
        (Surface::Elliptic(wall), Surface::Cylinder(round))
            if parallel(round.axis, wall.axis, scale) =>
        {
            Some((met_round(round, wall, scale)?, false))
        }
        (Surface::Elliptic(first), Surface::Elliptic(second))
            if parallel(first.axis, second.axis, scale) =>
        {
            let (met, walked_first) = met_ovals(first, second, scale)?;
            Some((met, !walked_first))
        }
        _ => None,
    }
}

/// The rulings a circular wall and an elliptic one meet along, read round
/// the elliptic one.
fn met_round(round: &Cylinder, wall: &EllipticCylinder, scale: Scale) -> Option<Met> {
    let from = round.origin - wall.origin;
    let q = DVec2::new(from.dot(wall.u), from.dot(wall.v));
    met(
        wall,
        Other::Round(round),
        normals::feet(wall.major, wall.minor, q),
        scale,
    )
}

/// The rulings two elliptic walls meet along, read round the less round of
/// the two, and whether that is the first.
fn met_ovals(
    one: &EllipticCylinder,
    other: &EllipticCylinder,
    scale: Scale,
) -> Option<(Met, bool)> {
    let walked_first = !rounder_first(one, other);
    let (walked, rounder) = if walked_first {
        (one, other)
    } else {
        (other, one)
    };
    let flat = |way: DVec3| {
        DVec2::new(
            way.dot(rounder.u) / rounder.major,
            way.dot(rounder.v) / rounder.minor,
        )
    };
    let centre = flat(walked.origin - rounder.origin);
    let along = flat(walked.u) * walked.major;
    let across = flat(walked.v) * walked.minor;
    let tilt =
        (2.0 * along.dot(across)).atan2(along.length_squared() - across.length_squared()) / 2.0;
    let (sin, cos) = tilt.sin_cos();
    let major = along * cos + across * sin;
    let minor = across * cos - along * sin;
    let (long, short) = (major.length(), minor.length());
    let q = DVec2::new(-centre.dot(major) / long, -centre.dot(minor) / short);
    let angles = normals::feet(long, short, q)
        .into_iter()
        .map(|phi| tilt + phi)
        .collect();
    met(walked, Other::Oval(rounder), angles, scale).map(|met| (met, walked_first))
}

/// Whether the first of two elliptic walls is the rounder, its half-axes
/// nearer one: an order read off the walls alone, so that the pair is read
/// the same whichever came first.
fn rounder_first(one: &EllipticCylinder, other: &EllipticCylinder) -> bool {
    let ratio = |wall: &EllipticCylinder| wall.major / wall.minor;
    let bits = |wall: &EllipticCylinder| {
        [wall.origin, wall.axis, wall.u]
            .into_iter()
            .flat_map(|vector| vector.to_array())
            .chain([wall.major, wall.minor])
            .collect::<Vec<f64>>()
    };
    ratio(one)
        .total_cmp(&ratio(other))
        .then_with(|| {
            bits(one)
                .iter()
                .zip(bits(other).iter())
                .map(|(mine, theirs)| mine.total_cmp(theirs))
                .find(|order| order.is_ne())
                .unwrap_or(std::cmp::Ordering::Equal)
        })
        .is_le()
}

/// The wall an elliptic one is measured against.
#[derive(Clone, Copy)]
enum Other<'a> {
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
struct Met {
    rulings: Vec<Ruled>,
    feet: Vec<[DVec3; 2]>,
    nearest: f64,
}

/// The rulings `walked` meets `other` along, from the angles of `walked`
/// at which their distance turns, in order round it; none where the two
/// stand within the tolerance of each other at every one of those angles.
fn met(walked: &EllipticCylinder, other: Other, angles: Vec<f64>, scale: Scale) -> Option<Met> {
    let eps = scale.eps();
    let point = |theta: f64| walked.point(DVec2::new(theta, 0.0));
    let gaps: Vec<f64> = angles
        .iter()
        .map(|&theta| other.gap(point(theta)))
        .collect();
    let touching: Vec<bool> = gaps.iter().map(|gap| gap.abs() <= eps).collect();
    let start = touching.iter().position(|touch| !touch)?;
    let count = angles.len();
    let turned = |index: usize| {
        let at = (start + index) % count;
        let turns = ((start + index) / count) as f64;
        (at, angles[at] + std::f64::consts::TAU * turns)
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

/// The relation the rulings found make: apart for none, a touch alone as
/// `Tangent`, two crossings as `Lines`, and anything else as `Parallel`,
/// in an order read off the lines alone.
fn ruled(mut rulings: Vec<Ruled>) -> Relation {
    match rulings.as_slice() {
        [] => Relation::Apart,
        [only] if only.touching => Relation::Tangent(only.line),
        [one, other] if !one.touching && !other.touching => {
            Relation::Lines(sorted([one.line, other.line]))
        }
        _ => {
            rulings.sort_by(|one, other| {
                one.line
                    .origin
                    .to_array()
                    .partial_cmp(&other.line.origin.to_array())
                    .unwrap_or(std::cmp::Ordering::Equal)
            });
            Relation::Parallel(rulings)
        }
    }
}

#[cfg(test)]
mod tests;
