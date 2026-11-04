//! What two perpendicular walls make together, decided once within the
//! kernel's tolerance: the components of the curve they meet along, its nodes,
//! or the point where they touch.

use std::cmp::Ordering;
use std::f64::consts::{PI, TAU};

use glam::DVec3;

use super::pair::{Configuration, Pair};
use crate::brep::curve::{Meet, Wall};
use crate::brep::scale::Scale;
use crate::brep::surface::{Cylinder, EllipticCylinder};

#[derive(Clone, Debug, PartialEq)]
pub struct Meeting {
    pub configuration: Configuration,
    pub components: Vec<Meet>,
    pub nodes: Vec<Node>,
    pub contact: Option<DVec3>,
}

/// A point where the curve crosses itself, with its parameter on each
/// component each time it passes: a vertex of whatever uses the curve.
#[derive(Clone, Debug, PartialEq)]
pub struct Node {
    pub point: DVec3,
    pub on: Vec<(u8, f64)>,
}

impl Meeting {
    /// The larger wall is taken first whatever the order asked, so the
    /// same pair gives the same curve both ways. The first is kept bit for
    /// bit; the second is moved, by less than the tolerance, onto the touch
    /// decided here, and every component carries it moved.
    pub fn of(one: &Wall, other: &Wall, scale: Scale) -> Meeting {
        Meeting::snapped_within(one, other, scale.eps())
    }

    /// The same, the two walls taken as they stand: nothing moved beyond
    /// the rounding `pair.rs` reads as a touch. Decision 2 has made exact
    /// every touch it could; where it could not — the wall it would move
    /// held at another touch — the curve stays on the walls the corners
    /// around it are found on, rather than on a copy a hair off one of
    /// them that a crossing at a slant would part from by more than the
    /// tolerance.
    pub fn as_they_stand(one: &Wall, other: &Wall) -> Meeting {
        Meeting::snapped_within(one, other, 0.0)
    }

    fn snapped_within(one: &Wall, other: &Wall, eps: f64) -> Meeting {
        let (first, second) = match ordered(one, other) {
            Ordering::Less | Ordering::Equal => (*one, *other),
            Ordering::Greater => (*other, *one),
        };
        let second = snapped(&first, &second, eps);
        let pair = Pair::of(&first, &second);
        let configuration = pair.configuration();
        let components = (0..pair.components())
            .map(|component| Meet {
                first,
                second,
                component,
            })
            .collect();
        let nodes = nodes(&pair);
        let contact = (configuration == Configuration::Contact)
            .then(|| pair.at_end(if pair.d > 0.0 { pair.high } else { pair.low }));
        Meeting {
            configuration,
            components,
            nodes,
            contact,
        }
    }
}

/// The wall of two perpendicular ones that `Meeting::of` moves onto the
/// touch it decides, by its rank in the pair as asked, and where it moves it
/// to; none where nothing moves.
pub(in crate::brep) fn moved(one: &Wall, other: &Wall, scale: Scale) -> Option<(usize, Wall)> {
    let (first, second, rank) = match ordered(one, other) {
        Ordering::Greater => (other, one, 0),
        Ordering::Less | Ordering::Equal => (one, other, 1),
    };
    let snapped = snapped(first, second, scale.eps());
    (snapped != *second).then_some((rank, snapped))
}

/// The other wall of the pair moved onto the touch `moved` decides, by
/// its rank in the pair as asked: across the axis of the one `moved` moves,
/// by the same offset the other way, and given that one's half-width where
/// the two were taken for one half-width. None where nothing moves.
pub(in crate::brep) fn moved_instead(
    one: &Wall,
    other: &Wall,
    scale: Scale,
) -> Option<(usize, Wall)> {
    let (first, second, rank) = match ordered(one, other) {
        Ordering::Greater => (other, one, 1),
        Ordering::Less | Ordering::Equal => (one, other, 0),
    };
    let pair = Pair::of(first, second);
    let (offset, radius) = touch(&pair, scale.eps())?;
    let kept = if radius == pair.b { pair.a } else { pair.b };
    let by = -pair.axes[1] * (offset - pair.d);
    Some((
        rank,
        match first {
            Wall::Cylinder(cylinder) => {
                Wall::Cylinder(Cylinder::about(cylinder.origin + by, cylinder.axis, kept))
            }
            Wall::Elliptic(oval) => {
                let moved = oval.moved(by);
                Wall::Elliptic(if radius == pair.b {
                    moved
                } else {
                    widened(&moved, pair.axes[1], kept)
                })
            }
        },
    ))
}

/// Where the curve of a pair crosses itself, each node with the parameter
/// of every pass of a component through it.
pub(super) fn nodes(pair: &Pair) -> Vec<Node> {
    let singular = [
        pair.x_vanishes()[0] && pair.z_vanishes()[0],
        pair.x_vanishes()[1] && pair.z_vanishes()[1],
    ];
    match pair.configuration() {
        Configuration::TwoEllipses => vec![
            Node {
                point: pair.at_end(pair.low),
                on: vec![(0, 0.0), (1, 0.0)],
            },
            Node {
                point: pair.at_end(pair.high),
                on: vec![(0, PI), (1, PI)],
            },
        ],
        Configuration::FigureOfEight if singular[1] => vec![Node {
            point: pair.at_end(pair.high),
            on: vec![(0, PI), (0, 3.0 * PI)],
        }],
        Configuration::FigureOfEight => vec![Node {
            point: pair.at_end(pair.low),
            on: vec![(0, 0.0), (0, TAU)],
        }],
        _ => Vec::new(),
    }
}

/// The second wall moved across the first's axis, and for two of one
/// half-width given the first's, so that an end of the span within `eps`
/// of a touch is one exactly: the ends of the span meet for a contact,
/// both roots vanish together at a node.
fn snapped(first: &Wall, second: &Wall, eps: f64) -> Wall {
    let pair = Pair::of(first, second);
    match (touch(&pair, eps), second) {
        (Some((offset, radius)), Wall::Cylinder(cylinder)) => Wall::Cylinder(Cylinder::about(
            cylinder.origin + pair.axes[1] * (offset - pair.d),
            cylinder.axis,
            radius,
        )),
        (Some((offset, half)), Wall::Elliptic(oval)) => {
            let moved = oval.moved(pair.axes[1] * (offset - pair.d));
            Wall::Elliptic(if half == pair.b {
                moved
            } else {
                widened(&moved, pair.axes[1], half)
            })
        }
        (None, _) => *second,
    }
}

/// The offset of the second wall's axis across the first's, and its
/// half-width, that make an end of the span within `eps` of a touch one
/// exactly; none where nothing is within `eps` of a touch, or where it is
/// one already.
fn touch(pair: &Pair, eps: f64) -> Option<(f64, f64)> {
    let (a, b, d) = (pair.a, pair.b, pair.d);
    let span = pair.high - pair.low;
    let at = |end: usize| pair.x_gaps[end].max(pair.z_gaps[end]) <= eps;
    let (offset, radius) = if span < -eps {
        return None;
    } else if span <= eps {
        (d.signum() * (a + b), b)
    } else {
        match [at(0), at(1)] {
            [true, true] => (0.0, a),
            [true, false] => (b - a, b),
            [false, true] => (a - b, b),
            [false, false] => return None,
        }
    };
    (offset != d || radius != b).then_some((offset, radius))
}

/// An elliptic wall made to reach `half` along `along`, square to its axis,
/// by the half-axis nearer that direction alone: where the two walls of a
/// node are drawn of one half-width, an axis of the ellipse runs along the
/// other wall's, and that half-axis alone is the hair off.
fn widened(oval: &EllipticCylinder, along: DVec3, half: f64) -> EllipticCylinder {
    let [on_u, on_v] = [along.dot(oval.u), along.dot(oval.v)];
    let (first, second) = if on_u.abs() >= on_v.abs() {
        let rest = oval.minor * on_v;
        let length = ((half - rest) * (half + rest)).max(0.0).sqrt() / on_u.abs();
        (oval.u * length, oval.minor)
    } else {
        let rest = oval.major * on_u;
        let length = ((half - rest) * (half + rest)).max(0.0).sqrt() / on_v.abs();
        (oval.v * length, oval.major)
    };
    EllipticCylinder::about(oval.origin, oval.axis, first, second)
}

/// Which of two walls goes first: two cylinders as they always have; with
/// an elliptic wall among them, the larger by its radius or major
/// half-axis, then by where it stands and which way it runs.
fn ordered(one: &Wall, other: &Wall) -> Ordering {
    match (one, other) {
        (Wall::Cylinder(one), Wall::Cylinder(other)) => goes_first(one, other),
        _ => size(other)
            .total_cmp(&size(one))
            .then_with(|| lexicographic(one.origin(), other.origin()))
            .then_with(|| lexicographic(one.axis(), other.axis())),
    }
}

fn size(wall: &Wall) -> f64 {
    match wall {
        Wall::Cylinder(cylinder) => cylinder.radius,
        Wall::Elliptic(oval) => oval.major,
    }
}

pub(in crate::brep) fn goes_first(one: &Cylinder, other: &Cylinder) -> Ordering {
    other
        .radius
        .total_cmp(&one.radius)
        .then_with(|| lexicographic(one.origin, other.origin))
        .then_with(|| lexicographic(one.axis, other.axis))
}

fn lexicographic(one: DVec3, other: DVec3) -> Ordering {
    (0..3).fold(Ordering::Equal, |order, index| {
        order.then_with(|| one[index].total_cmp(&other[index]))
    })
}
