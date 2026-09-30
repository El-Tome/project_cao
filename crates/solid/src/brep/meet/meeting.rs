//! What two perpendicular cylinders make together, decided once within the
//! kernel's tolerance: the components of the curve they meet along, its nodes,
//! or the point where they touch.

use std::cmp::Ordering;
use std::f64::consts::{PI, TAU};

use glam::DVec3;

use super::pair::{Configuration, Pair};
use crate::brep::curve::Meet;
use crate::brep::scale::Scale;
use crate::brep::surface::Cylinder;

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
    /// The larger cylinder is taken first whatever the order asked, so the
    /// same pair gives the same curve both ways. The first is kept bit for
    /// bit; the second is moved, by less than the tolerance, onto the touch
    /// decided here, and every component carries it moved.
    pub fn of(one: &Cylinder, other: &Cylinder, scale: Scale) -> Meeting {
        let (first, second) = match goes_first(one, other) {
            Ordering::Less | Ordering::Equal => (*one, *other),
            Ordering::Greater => (*other, *one),
        };
        let second = snapped(&first, &second, scale.eps());
        let pair = Pair::of(&first, &second);
        let configuration = pair.configuration();
        let components = (0..pair.components())
            .map(|component| Meet {
                first,
                second,
                component,
            })
            .collect();
        let at_end = |y: f64| pair.world(DVec3::new(0.0, y, pair.e));
        let singular = [
            pair.x_vanishes()[0] && pair.z_vanishes()[0],
            pair.x_vanishes()[1] && pair.z_vanishes()[1],
        ];
        let nodes = match configuration {
            Configuration::TwoEllipses => vec![
                Node {
                    point: at_end(pair.low),
                    on: vec![(0, 0.0), (1, 0.0)],
                },
                Node {
                    point: at_end(pair.high),
                    on: vec![(0, PI), (1, PI)],
                },
            ],
            Configuration::FigureOfEight if singular[1] => vec![Node {
                point: at_end(pair.high),
                on: vec![(0, PI), (0, 3.0 * PI)],
            }],
            Configuration::FigureOfEight => vec![Node {
                point: at_end(pair.low),
                on: vec![(0, 0.0), (0, TAU)],
            }],
            _ => Vec::new(),
        };
        let contact = (configuration == Configuration::Contact)
            .then(|| at_end(if pair.d > 0.0 { pair.high } else { pair.low }));
        Meeting {
            configuration,
            components,
            nodes,
            contact,
        }
    }
}

/// The second cylinder moved across the first's axis, and for two equal ones
/// given the first's radius, so that an end of the span within `eps` of a
/// touch is one exactly: the ends of the span meet for a contact, both roots
/// vanish together at a node.
fn snapped(first: &Cylinder, second: &Cylinder, eps: f64) -> Cylinder {
    let pair = Pair::of(first, second);
    let (a, b, d) = (pair.a, pair.b, pair.d);
    let span = pair.high - pair.low;
    let at = |end: usize| pair.x_gaps[end].max(pair.z_gaps[end]) <= eps;
    let (offset, radius) = if span < -eps {
        return *second;
    } else if span <= eps {
        (d.signum() * (a + b), b)
    } else {
        match [at(0), at(1)] {
            [true, true] => (0.0, a),
            [true, false] => (b - a, b),
            [false, true] => (a - b, b),
            [false, false] => return *second,
        }
    };
    if offset == d && radius == b {
        return *second;
    }
    Cylinder::about(
        second.origin + pair.axes[1] * (offset - d),
        second.axis,
        radius,
    )
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
