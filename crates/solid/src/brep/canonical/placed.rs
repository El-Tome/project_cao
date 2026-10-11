//! Where decision 11 sets the second operand down once it is turned. The
//! turn lays its directions exactly; the point it is made about decides
//! where the operand ends up, a hair either way, and a hair is all it takes
//! to part what the second held against the first: a face drawn on an end
//! of the first, a line of the first lying in a face of the second, a
//! wall's axis lying in a face of the other. Turned about the middle of its
//! box, a wedge's side stood a hair off a disc's end it was drawn on, and
//! its corners at the wedge's axis four times further (5544600162); a
//! post's end stood a hair off a line of a turn it held (5544510380); a
//! turn's axis stood a hair off a disc's end it lay in (5544600864); a face
//! laid square to a cone stood 0.98 of the tolerance short of the circle
//! where the cone meets a shaft (5544232904).
//!
//! The operand is turned about the middle of where the two meet, then
//! shifted square to the turn's axis — the shift a turn about another point
//! makes — so that each such pair lies exactly: two flats the turn lays
//! parallel, a plane or the plane of a circle of each, one on the other; a
//! line of either, straight edge or wall's axis, in a flat of the other the
//! turn lays it along. Only where the two stand within the tolerance of
//! each other, taken for one place already: laid from as far as
//! `Scale::HAIR` tolerances, a cut's face was moved three tolerances onto
//! what it only passed near, and refused (5545209574). The pairs nearest to
//! lying come first; a pair the shift cannot lay without taking an earlier
//! one off, or without moving the operand further than the hair, is left as
//! it stands.

use glam::{DQuat, DVec3};

use crate::brep::curve::Curve;
use crate::brep::surface::{Plane, Surface};
use crate::brep::topology::{Body, Edge};

/// What the shift is read from: the turn and the point it is made about,
/// the sine under which a turn lays two directions square, the tolerance
/// under which two places are one, and how far the shift may still move the
/// operand.
pub(super) struct Laying {
    pub turn: DQuat,
    pub met: DVec3,
    pub within: f64,
    pub eps: f64,
    pub left: f64,
}

/// Under this share of its length, the part of a pair's normal a shift
/// square to the turn's axis can still move along is none: laying the pair
/// would throw the shift far for a gap within the tolerance.
const FREE: f64 = 0.1;

/// A pair the shift lays: `along · shift = gap`.
struct Lying {
    along: DVec3,
    gap: f64,
}

/// A line, as a point of it and its direction.
type Straight = (DVec3, DVec3);

/// The shift that lays the second operand, turned, exactly on what it held
/// against the first.
pub(super) fn placed(first: &Body, second: &Body, laying: &Laying) -> DVec3 {
    let (Some(first_box), Some(second_box)) = (first.bounds(), second.bounds()) else {
        return DVec3::ZERO;
    };
    let Laying { turn, met, eps, .. } = *laying;
    let place = |point: DVec3| met + turn * (point - met);
    let known_flats = flats(first, second_box, eps);
    let tool_flats: Vec<Plane> = flats(second, first_box, eps)
        .iter()
        .map(|flat| Plane::through(place(flat.origin), turn * flat.normal).0)
        .collect();
    let known_lines = straights(first, second_box, eps);
    let tool_lines: Vec<Straight> = straights(second, first_box, eps)
        .into_iter()
        .map(|(point, direction)| (place(point), turn * direction))
        .collect();
    let mut lying: Vec<Lying> = Vec::new();
    for known in &known_flats {
        for tool in &tool_flats {
            lying.extend(on(known, tool, laying));
        }
        for &(point, direction) in &tool_lines {
            lying.extend(lies_in(known, point, direction, laying).map(|away| Lying {
                along: known.normal,
                gap: -away,
            }));
        }
    }
    for tool in &tool_flats {
        for &(point, direction) in &known_lines {
            lying.extend(lies_in(tool, point, direction, laying).map(|away| Lying {
                along: tool.normal,
                gap: away,
            }));
        }
    }
    solved(lying, turn.xyz().normalize_or_zero(), laying.left)
}

/// Two flats, the second's turned: the pair, where the turn lays them
/// parallel and they stand within the tolerance of each other.
fn on(known: &Plane, tool: &Plane, laying: &Laying) -> Option<Lying> {
    if known.normal.cross(tool.normal).length() > laying.within {
        return None;
    }
    let side = known.normal.dot(tool.normal).signum();
    let gap = known.offset() - side * tool.offset();
    (gap.abs() <= laying.eps).then_some(Lying {
        along: known.normal,
        gap,
    })
}

/// How far a line along a flat stands from it, where it does within the
/// tolerance.
fn lies_in(flat: &Plane, point: DVec3, direction: DVec3, laying: &Laying) -> Option<f64> {
    let away = flat.distance(point);
    (flat.normal.dot(direction).abs() <= laying.within && away.abs() <= laying.eps).then_some(away)
}

/// The least shift square to `axis` laying each pair in turn, the one
/// nearest to lying first; a pair it cannot lay without taking an earlier
/// one off, or past `left`, is left.
fn solved(mut lying: Vec<Lying>, axis: DVec3, left: f64) -> DVec3 {
    lying.sort_by(|one, other| one.gap.abs().total_cmp(&other.gap.abs()));
    let mut shift = DVec3::ZERO;
    let mut laid: Vec<DVec3> = Vec::new();
    for pair in lying {
        let along = pair.along - axis * axis.dot(pair.along);
        let free = laid
            .iter()
            .fold(along, |free, done| free - *done * done.dot(free));
        if free.length() <= FREE * along.length() || along.length() <= FREE {
            continue;
        }
        let step = free * ((pair.gap - along.dot(shift)) / along.dot(free));
        if (shift + step).length() > left {
            continue;
        }
        shift += step;
        laid.push(free.normalize());
    }
    shift
}

/// The flats of a body: its planes, and the plane of each circle it has an
/// edge on within `grown` of the other operand's box.
fn flats(body: &Body, near: (DVec3, DVec3), grown: f64) -> Vec<Plane> {
    let planes = body.surfaces.iter().filter_map(|surface| match surface {
        Surface::Plane(plane) => Some(*plane),
        Surface::Cylinder(_) | Surface::Cone(_) => None,
    });
    let circles = body
        .edges
        .iter()
        .filter(|edge| reaches(body, edge, near, grown))
        .filter_map(|edge| match body.curve(edge.curve) {
            Curve::Circle(circle) => Some(Plane::through(circle.center, circle.axis).0),
            _ => None,
        });
    planes.chain(circles).collect()
}

/// The lines of a body: the axis of each of its walls, and each line it
/// has an edge on within `grown` of the other operand's box.
fn straights(body: &Body, near: (DVec3, DVec3), grown: f64) -> Vec<Straight> {
    let axes = body.surfaces.iter().filter_map(|surface| match surface {
        Surface::Cylinder(cylinder) => Some((cylinder.origin, cylinder.axis)),
        Surface::Cone(cone) => Some((cone.origin, cone.axis)),
        Surface::Plane(_) => None,
    });
    let lines = body
        .edges
        .iter()
        .filter(|edge| reaches(body, edge, near, grown))
        .filter_map(|edge| match body.curve(edge.curve) {
            Curve::Line(line) => Some((line.origin, line.direction)),
            _ => None,
        });
    axes.chain(lines).collect()
}

/// Whether an edge of the body reaches within `grown` of the box.
fn reaches(body: &Body, edge: &Edge, (low, high): (DVec3, DVec3), grown: f64) -> bool {
    let points = body.extremes(edge);
    let one = points
        .iter()
        .fold(DVec3::INFINITY, |least, point| least.min(*point));
    let other = points
        .iter()
        .fold(DVec3::NEG_INFINITY, |most, point| most.max(*point));
    one.cmple(high + grown).all() && (low - grown).cmple(other).all()
}
