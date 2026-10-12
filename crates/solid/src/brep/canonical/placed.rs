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
//! turn lays it along; a straight edge of either touching a cylinder of the
//! other across its axis. A flat that passes near several lines parting
//! from each other lies on none of them. Only where the two stand within
//! the tolerance of each other, taken for one place already: laid from as far as `Scale::HAIR` tolerances, a cut's face was
//! moved three tolerances onto what it only passed near, and refused
//! (5545209574). The pairs nearest to lying come first; a pair the shift
//! cannot lay without taking an earlier one off, or without moving the
//! operand further than the hair, is left as it stands.

use glam::{DQuat, DVec3};

use crate::brep::curve::Curve;
use crate::brep::surface::{Cylinder, Plane, Surface};
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

/// The share of the tolerance under which lines a flat passes near are one
/// line to it.
const ONE_LINE: f64 = 1e-3;

/// A pair the shift lays: `along · shift = gap`.
struct Lying {
    along: DVec3,
    gap: f64,
}

/// A line, as a point of it and its direction.
type Straight = (DVec3, DVec3);

/// What an operand may hold against the other: its flats, its straight
/// edges, the axes of its walls, and its cylinders as an axis and a radius,
/// the second's turned and set about where the two meet.
struct Held {
    flats: Vec<Plane>,
    edges: Vec<Straight>,
    axes: Vec<Straight>,
    walls: Vec<(Straight, f64)>,
}

impl Held {
    fn of(
        body: &Body,
        near: (DVec3, DVec3),
        eps: f64,
        place: impl Fn(DVec3) -> DVec3,
        turn: DQuat,
    ) -> Held {
        let line = |(point, direction): Straight| (place(point), turn * direction);
        Held {
            flats: flats(body, near, eps)
                .iter()
                .map(|flat| Plane::through(place(flat.origin), turn * flat.normal).0)
                .collect(),
            edges: edges(body, near, eps).into_iter().map(line).collect(),
            axes: axes(body).map(line).collect(),
            walls: walls(body)
                .map(|wall| (line((wall.origin, wall.axis)), wall.radius))
                .collect(),
        }
    }
}

/// The shift that lays the second operand, turned, exactly on what it held
/// against the first.
pub(super) fn placed(first: &Body, second: &Body, laying: &Laying) -> DVec3 {
    let (Some(first_box), Some(second_box)) = (first.bounds(), second.bounds()) else {
        return DVec3::ZERO;
    };
    let Laying { turn, met, eps, .. } = *laying;
    let known = Held::of(first, second_box, eps, |point| point, DQuat::IDENTITY);
    let tool = Held::of(
        second,
        first_box,
        eps,
        |point| met + turn * (point - met),
        turn,
    );
    let mut lying: Vec<Lying> = Vec::new();
    for one in &known.flats {
        for other in &tool.flats {
            lying.extend(on(one, other, laying));
        }
    }
    lying.extend(held(&known, &tool, laying, 1.0));
    lying.extend(held(&tool, &known, laying, -1.0));
    solved(lying, turn.xyz().normalize_or_zero(), laying.left)
}

/// The pairs where `moving` stands against `still`: a line of it in a flat,
/// a straight edge of it on a wall. Each pair is read as it parts when
/// `moving` moves; `toward` is -1 where it is the first operand, which the
/// shift moves the other way.
fn held(still: &Held, moving: &Held, laying: &Laying, toward: f64) -> Vec<Lying> {
    let mut lying = Vec::new();
    let lay = |along: DVec3, apart: f64| Lying {
        along: along * toward,
        gap: -apart,
    };
    for flat in &still.flats {
        let aways: Vec<f64> = moving
            .edges
            .iter()
            .chain(&moving.axes)
            .filter_map(|&(point, direction)| lies_in(flat, point, direction, laying))
            .collect();
        if one_place(&aways, laying.eps) {
            lying.extend(aways.into_iter().map(|away| lay(flat.normal, away)));
        }
    }
    for &((origin, axis), radius) in &still.walls {
        for &(point, direction) in &moving.edges {
            if axis.dot(direction).abs() > laying.within {
                continue;
            }
            let across = axis.cross(direction).normalize_or_zero();
            let from_the_axis = (point - origin).dot(across);
            lying.extend(
                touching(from_the_axis, radius, laying)
                    .map(|apart| lay(across * from_the_axis.signum(), apart)),
            );
        }
    }
    lying
}

/// Whether the lines a flat passes within the tolerance of stand as one
/// line to it, within `ONE_LINE` of the tolerance of each other. A cut
/// drawn through a point a hair from a turn's axis, the slit's line and an
/// earlier cut's corner, each a fraction of the tolerance from the others,
/// was laid on the axis, the nearest, and refused; as turned, between them,
/// it held (5546116488). A flat moves along its normal alone: it cannot lie
/// on two lines that part.
fn one_place(aways: &[f64], eps: f64) -> bool {
    let (least, most) = aways
        .iter()
        .fold((f64::INFINITY, f64::NEG_INFINITY), |(least, most), away| {
            (least.min(*away), most.max(*away))
        });
    aways.len() < 2 || most - least <= ONE_LINE * eps
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

/// How far a straight edge, `from_the_axis` of a wall and square to its
/// axis, stands off touching the wall, where it does within the tolerance.
/// A slot's side drawn on a disc's wall, and a rounded block's seam on a
/// ring's, were left a fraction of the tolerance off by the turn and
/// refused (5546322326, 5546607658); their edges lay them. A flat is not
/// laid on a wall it touches: a face drawn on a turn's wall, a hair inside
/// a second wall of the turn a hair larger, was laid on that one and taken
/// further off the first, and refused (5546111832).
fn touching(from_the_axis: f64, radius: f64, laying: &Laying) -> Option<f64> {
    let apart = from_the_axis.abs() - radius;
    (apart.abs() <= laying.eps).then_some(apart)
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

/// The axis of each wall of a body.
fn axes(body: &Body) -> impl Iterator<Item = Straight> {
    body.surfaces.iter().filter_map(|surface| match surface {
        Surface::Cylinder(cylinder) => Some((cylinder.origin, cylinder.axis)),
        Surface::Cone(cone) => Some((cone.origin, cone.axis)),
        Surface::Plane(_) => None,
    })
}

/// The cylinders of a body.
fn walls(body: &Body) -> impl Iterator<Item = &Cylinder> {
    body.surfaces.iter().filter_map(|surface| match surface {
        Surface::Cylinder(cylinder) => Some(cylinder),
        Surface::Plane(_) | Surface::Cone(_) => None,
    })
}

/// Each line a body has an edge on within `grown` of the other operand's
/// box.
fn edges(body: &Body, near: (DVec3, DVec3), grown: f64) -> Vec<Straight> {
    body.edges
        .iter()
        .filter(|edge| reaches(body, edge, near, grown))
        .filter_map(|edge| match body.curve(edge.curve) {
            Curve::Line(line) => Some((line.origin, line.direction)),
            _ => None,
        })
        .collect()
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
