//! What a cone and another wall make together, decided once within the
//! kernel's tolerance (#577): the components of the curve, its nodes, the
//! point where they touch — the other wall moved first, across its own
//! axis, onto a touch it stands within the tolerance of.

use glam::{DVec2, DVec3};

use super::{NEAR, Quadric, Ruled, angle_between};
use crate::brep::curve::{Meet, Wall};
use crate::brep::meet::{Meeting, Node, ruled};
use crate::brep::surface::Cylinder;

/// Under this, a sine says the cone's axis and the wall's stand parallel, or
/// the direction square to both stands along the wall: the gap is closed
/// across the wall then, along its normal at the touch.
const SQUARE: f64 = 1e-6;

/// The meeting of a cone and a cylinder, an elliptic wall or another cone,
/// a wall moved onto a touch within `eps`; none for any other pair.
pub(in crate::brep::meet) fn meeting(cone: &Wall, other: &Wall, eps: f64) -> Option<Meeting> {
    let other = snapped(cone, other, eps)?;
    let read = ruled(cone, &other)?;
    let components = (0..read.laps.len() as u8)
        .map(|component| Meet {
            first: *cone,
            second: other,
            component,
        })
        .collect();
    Some(Meeting {
        configuration: read.configuration,
        components,
        nodes: nodes(&read),
        contact: read.contact.map(|theta| read.touching(theta)),
    })
}

/// Where the curve crosses itself, each node with the parameter of every
/// pass of a component through it: two laps passing one node pass it at
/// the angle of the same double zero of `D`, within `NEAR`.
pub(in crate::brep::meet) fn nodes(read: &Ruled) -> Vec<Node> {
    let mut nodes: Vec<(f64, Node)> = Vec::new();
    for (rank, lap) in read.laps.iter().enumerate() {
        for &node in &lap.nodes {
            let on = read.through(lap, node).into_iter().map(|t| (rank as u8, t));
            match nodes
                .iter_mut()
                .find(|(known, _)| angle_between(*known, node) <= NEAR)
            {
                Some((_, known)) => known.on.extend(on),
                None => nodes.push((
                    node,
                    Node {
                        point: read.touching(node),
                        on: on.collect(),
                    },
                )),
            }
        }
    }
    nodes.into_iter().map(|(_, node)| node).collect()
}

/// The other wall moved across its axis so that a touch within `eps` of
/// the cone is one exactly: where a ruling comes nearest it, at an angle
/// where `D` turns, the gap is read and closed along the direction square to
/// both axes, as two walls' is. The wall as it stands where no touch is
/// that near, or where it is a cone; none where the pair is not read along
/// a cone's rulings.
pub(in crate::brep::meet) fn snapped(cone: &Wall, other: &Wall, eps: f64) -> Option<Wall> {
    let read = ruled(cone, other)?;
    if eps == 0.0 || matches!(other, Wall::Cone(_)) {
        return Some(*other);
    }
    let gap = |theta: f64| {
        let point = read.touching(theta);
        let (along, gap) = across(&read.other, point);
        (along, gap)
    };
    let nearest = read
        .turning
        .iter()
        .copied()
        .map(gap)
        .filter(|(_, gap)| gap.abs() <= eps && *gap != 0.0)
        .min_by(|one, other| one.1.abs().total_cmp(&other.1.abs()));
    let Some((along, gap)) = nearest else {
        return Some(*other);
    };
    let square = read.cone.axis.cross(read.other.axis);
    let way = if square.length() > SQUARE {
        square.normalize() * square.dot(along).signum()
    } else {
        along
    };
    let lean = way.dot(along);
    let by = if lean > SQUARE {
        way * (gap / lean)
    } else {
        along * gap
    };
    Some(match other {
        Wall::Cylinder(cylinder) => Wall::Cylinder(Cylinder::about(
            cylinder.origin + by,
            cylinder.axis,
            cylinder.radius,
        )),
        Wall::Elliptic(oval) => Wall::Elliptic(oval.moved(by)),
        Wall::Cone(_) => *other,
    })
}

/// The direction square to the quadric's axis from it towards `point`, and
/// how far `point` stands outside the quadric along it, to first order.
fn across(quadric: &Quadric, point: DVec3) -> (DVec3, f64) {
    let from = point - quadric.origin;
    let flat = from - quadric.axis * from.dot(quadric.axis);
    let [(u, a), (v, b)] = quadric.halves;
    let scaled = DVec2::new(flat.dot(u) / a, flat.dot(v) / b);
    let level = scaled.length_squared() - 1.0;
    let slope = u * (2.0 * flat.dot(u) / (a * a)) + v * (2.0 * flat.dot(v) / (b * b));
    let length = slope.length();
    if length == 0.0 {
        return (DVec3::ZERO, 0.0);
    }
    (slope / length, level / length)
}
