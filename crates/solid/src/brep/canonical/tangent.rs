//! Decision 12 (#626): a plane square to a sphere's or a torus's axis, a
//! cylinder or a cone about it, one of each operand, standing within
//! `Scale::HAIR` tolerances of touching the round, is moved onto the touch.
//! Left a hair off, the two cross along two circles `√(2rδ)` apart for a
//! hair `δ` — a few ten-thousandths: real geometry, but thinner than any
//! triangle laid between them, and the faces' triangles crossed. Moved, the
//! pair is decided once as a touch and built as the one circle, as a ball
//! end on a shaft of its radius already is.
//!
//! So is such a surface standing within the hair of a rim the other operand
//! drew on its round, where the round meets another of its surfaces: a
//! rounded zone cut by its own section moved 5e-8 along the axis crossed
//! the zone's sphere a hair under its rim and left a ring 1.7e-8 wide on
//! the zone's end, which no ray tells the side of (5576809934).
//!
//! The second operand moves, with what it built on the surface, as
//! `carried.rs` moves it: along the axis for a plane or a cone, whichever of
//! the two operands carries the round, and away from the axis for a
//! cylinder, its radius grown or shrunk by the hair — or the ring of a torus
//! the second carries, where the first carries the cylinder. A sphere the
//! second carries against a cylinder of the first is no such move, and is
//! left where it is; so is a move further than the hair, a cone's along the
//! axis being the hair over the share of its slope it moves the line by.
//!
//! Only where faces of the two come within the hair of the circle: a bar a
//! hair narrower than the rim of a cap far along the axis from it was grown
//! onto the rim for nothing (5576873713, shrunk).
//!
//! A move is not made where it would take off a surface of either operand a
//! circle of a round that stood on it exactly — a rim, or where the round
//! touches a surface of the other operand: a ball a hair under a disc's top,
//! its equator touching the disc's wall exactly on the disc's floor, raised
//! onto the top, took its equator off the floor (5576859311).
//!
//! Nor where it would part a surface of the second operand from one of the
//! first it was one with — a tool's wall one with a disc's, shrunk onto a
//! ball a hair inside both (5577024047) — or change how a round of the
//! second stands with a round of the first: a cap cut by its copy 1e-7
//! along the axis, laid onto it, lost the lens between the two, which a
//! line through it at a slant still saw (5577028782); a ring whose tube is
//! a hair thinner than another's about the same circle, raised onto a
//! crest, came to touch it inside along a circle it stands within the
//! tolerance of all round (5576883559, and 5577020735 shrunk).
//!
//! Snapped as decision 8 snaps two walls of one radius, and as #552 lays a
//! lean below the tolerance exactly, rather than built both ways and
//! warned of: the triangles would still have to keep two rims a
//! ten-thousandth apart.

use std::mem::{Discriminant, discriminant};

use glam::DVec3;

use super::carried::{Across, Move, carried_by};
use crate::brep::curve::{Circle, Curve};
use crate::brep::relation::{Gap, Relation, off_the_rim, relation, untouched};
use crate::brep::scale::Scale;
use crate::brep::surface::Surface;
use crate::brep::topology::Body;

/// Under this share of the tolerance, a surface touches a round, or passes
/// through a rim, already.
const ROUNDING: f64 = 1e-6;

/// The second operand with each of its surfaces standing within
/// `Scale::HAIR` tolerances of touching a round of the first or of passing
/// through a rim on it, or each of its rounds as near touching a surface of
/// the first or as near a rim on it the surface passes through, moved onto
/// the nearest; none when nothing moves.
pub(in crate::brep) fn tangent(first: &Body, second: &Body, scale: Scale) -> Option<Body> {
    let theirs: Vec<(usize, Circle)> = (0..first.surfaces.len())
        .flat_map(|rank| {
            rims(first, rank)
                .into_iter()
                .map(move |(_, rim)| (rank, rim))
        })
        .collect();
    let by = |current: &Body, rank: usize| {
        let own = &current.surfaces[rank];
        let mine = rims(current, rank);
        let near_own = |circle: &Circle| near(current, rank, circle, scale);
        let mut gaps = Vec::new();
        for (index, known) in first.surfaces.iter().enumerate() {
            let near_known = |circle: &Circle| near(first, index, circle, scale);
            let touched = match untouched(known, own, scale) {
                Some(gap) => Some((gap, own, 1.0)),
                None => untouched(own, known, scale).map(|gap| (gap, known, -1.0)),
            };
            gaps.extend(touched.filter(|(gap, _, _)| near_own(&gap.at) && near_known(&gap.at)));
            for (_, rim) in &mine {
                gaps.extend(
                    off_the_rim(own, rim, known, scale)
                        .filter(|_| near_known(rim))
                        .map(|gap| (gap, known, -1.0)),
                );
            }
        }
        for (round, rim) in theirs.iter().filter(|(_, rim)| near_own(rim)) {
            let round = &first.surfaces[*round];
            gaps.extend(off_the_rim(round, rim, own, scale).map(|gap| (gap, own, 1.0)));
        }
        gaps.into_iter()
            .filter_map(|(gap, line, sense)| closing(gap, line, own, sense, scale))
            .min_by(|(one, _), (other, _)| one.total_cmp(other))
            .map(|(_, by)| by)
    };
    carried_by(first, second, scale, by, |before, after| {
        unbroken(first, before, after, scale) && standing(first, before, after, scale)
    })
}

/// The circles about its axis a sphere or a torus of `body`, of rank
/// `rank`, carries as edges of its faces, each with its rank among the
/// body's curves: where its operand drew another of its surfaces meeting
/// it. None for any other surface.
fn rims(body: &Body, rank: usize) -> Vec<(usize, Circle)> {
    if !matches!(body.surfaces[rank], Surface::Sphere(_) | Surface::Torus(_)) {
        return Vec::new();
    }
    let mut curves: Vec<usize> = body
        .face_ids()
        .filter(|face| body.face(*face).surface.0 as usize == rank)
        .flat_map(|face| body.face(face).loops.iter().flatten())
        .map(|coedge| body.edge(coedge.edge).curve.0 as usize)
        .collect();
    curves.sort_unstable();
    curves.dedup();
    curves
        .into_iter()
        .filter_map(|curve| match body.curves[curve] {
            Curve::Circle(circle) => Some((curve, circle)),
            Curve::Line(_) | Curve::Conic(_) | Curve::Meet(_) => None,
        })
        .collect()
}

/// The move of the second operand that closes `gap`, the gap `line` stands
/// at from a round's touch or rim — `line` the second's where `sense` is
/// one, the first's where it is minus one — and how far it goes, `own`
/// being the second's surface that moves; none where the gap is closed
/// already, or wider than the hair, or the move goes further, or would grow
/// a sphere.
fn closing(
    gap: Gap,
    line: &Surface,
    own: &Surface,
    sense: f64,
    scale: Scale,
) -> Option<(f64, Move)> {
    let eps = scale.eps();
    let Gap {
        origin,
        axis,
        way,
        gap,
        ..
    } = gap;
    if gap.abs() <= ROUNDING * eps || gap.abs() > Scale::HAIR * eps {
        return None;
    }
    let (length, by) = match line {
        Surface::Cylinder(_) if matches!(own, Surface::Sphere(_)) => return None,
        Surface::Cylinder(_) => {
            let by = sense * gap / way.y;
            (by.abs(), Move::Across(Across { origin, axis, by }))
        }
        Surface::Plane(_) | Surface::Cone(_) => {
            let along = sense * gap / way.x;
            (along.abs(), Move::Along(axis * along))
        }
        Surface::Elliptic(_) | Surface::Sphere(_) | Surface::Torus(_) => return None,
    };
    (length <= Scale::HAIR * eps).then_some((length, by))
}

/// Whether each surface of the second operand, moved from `before` to
/// `after`, stands with the first's as it stood: one with every surface it
/// was one with, and a round with a round as it stood — apart, one, or
/// along rims crossing or touching as they did.
fn standing(first: &Body, before: &Body, after: &Body, scale: Scale) -> bool {
    let round = |surface: &Surface| matches!(surface, Surface::Sphere(_) | Surface::Torus(_));
    before
        .surfaces
        .iter()
        .zip(&after.surfaces)
        .all(|(was, is)| {
            first.surfaces.iter().all(|known| {
                let (then, now) = (relation(known, was, scale), relation(known, is, scale));
                if round(known) && round(was) {
                    kind(&then) == kind(&now)
                } else {
                    !matches!(then, Relation::Same { .. }) || matches!(now, Relation::Same { .. })
                }
            })
        })
}

/// What a relation is, and for rims whether each only touches.
fn kind(relation: &Relation) -> (Discriminant<Relation>, Vec<bool>) {
    let touching = match relation {
        Relation::Rims(rims) => rims.iter().map(|rim| rim.touching).collect(),
        _ => Vec::new(),
    };
    (discriminant(relation), touching)
}

/// A circle of a round of one operand: a rim its operand drew on it, or
/// where it touches a surface of the other operand, within the hair.
#[derive(Clone, Copy)]
enum Feature {
    Rim {
        operand: usize,
        round: usize,
        curve: usize,
    },
    Touch {
        operand: usize,
        round: usize,
        line: usize,
    },
}

/// Whether a face of `body` on its surface of rank `rank` comes within the
/// hair of the box round `circle`.
fn near(body: &Body, rank: usize, circle: &Circle, scale: Scale) -> bool {
    let hair = Scale::HAIR * scale.eps();
    let across = circle
        .axis
        .to_array()
        .map(|along| circle.radius * (1.0 - along * along).max(0.0).sqrt());
    let (low, high) = (
        circle.center - DVec3::from(across),
        circle.center + DVec3::from(across),
    );
    body.face_ids()
        .filter(|face| body.face(*face).surface.0 as usize == rank)
        .any(|face| {
            let [lowest, highest] = body.boxed(face, hair);
            lowest.cmple(high).all() && low.cmple(highest).all()
        })
}

/// Whether the second operand, moved from `before` to `after`, leaves on
/// every surface of either operand a face of which comes near it each
/// circle of a round that stood on it exactly.
fn unbroken(first: &Body, before: &Body, after: &Body, scale: Scale) -> bool {
    let room = ROUNDING * scale.eps();
    features([first, before], scale).into_iter().all(|feature| {
        (0..2).all(|operand| {
            (0..first.surfaces.len().max(before.surfaces.len())).all(|rank| {
                let off = |bodies: [&Body; 2]| {
                    let (round, circle) = circle_of(feature, bodies, scale)?;
                    let surface = bodies[operand].surfaces.get(rank)?;
                    off_the_rim(round, &circle, surface, scale)
                        .filter(|_| near(bodies[operand], rank, &circle, scale))
                        .map(|gap| gap.gap)
                };
                match (off([first, before]), off([first, after])) {
                    (Some(was), Some(is)) => was > room || is <= room,
                    (Some(was), None) => was > room,
                    (None, _) => true,
                }
            })
        })
    })
}

/// Every rim on a round of either operand, and every touch of such a round
/// with a surface of the other within the hair, where faces of both come
/// near it.
fn features(bodies: [&Body; 2], scale: Scale) -> Vec<Feature> {
    let hair = Scale::HAIR * scale.eps();
    let mut features = Vec::new();
    for operand in 0..2 {
        let body = bodies[operand];
        for round in 0..body.surfaces.len() {
            for (curve, _) in rims(body, round) {
                features.push(Feature::Rim {
                    operand,
                    round,
                    curve,
                });
            }
            let other = bodies[1 - operand];
            for line in 0..other.surfaces.len() {
                let touching = untouched(&body.surfaces[round], &other.surfaces[line], scale)
                    .is_some_and(|gap| {
                        gap.gap.abs() <= hair
                            && near(body, round, &gap.at, scale)
                            && near(other, line, &gap.at, scale)
                    });
                if touching {
                    features.push(Feature::Touch {
                        operand,
                        round,
                        line,
                    });
                }
            }
        }
    }
    features
}

/// The round a feature stands on in `bodies`, and its circle there.
fn circle_of(feature: Feature, bodies: [&Body; 2], scale: Scale) -> Option<(&Surface, Circle)> {
    match feature {
        Feature::Rim {
            operand,
            round,
            curve,
        } => match bodies[operand].curves[curve] {
            Curve::Circle(circle) => Some((&bodies[operand].surfaces[round], circle)),
            Curve::Line(_) | Curve::Conic(_) | Curve::Meet(_) => None,
        },
        Feature::Touch {
            operand,
            round,
            line,
        } => {
            let round = &bodies[operand].surfaces[round];
            let gap = untouched(round, &bodies[1 - operand].surfaces[line], scale)?;
            Some((round, gap.at))
        }
    }
}
