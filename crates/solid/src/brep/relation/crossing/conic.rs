//! Where the conic a plane cuts a wall along at a slant crosses a surface
//! (#552, decision 4).
//!
//! The conic is its plane `P` cut by its wall `Q`, and the triple it makes
//! with a third surface `S` is solved from its most degenerate pair, by the
//! very call the other curve of that corner makes: against a plane, the line
//! `P` and `S` share crossing `Q`; against a wall, the lines, the circle or
//! the curve `Q` and `S` meet along crossing `P`. Each corner is therefore
//! computed once, with the same double-root decision and the same bits
//! whichever curve reaches it. Two conics of one plane meet through their
//! walls, two of one wall through their planes: no quartic is solved.
//!
//! Where the line of `P` and `S` grazes `Q` and one of the two planes meets
//! `Q` more grazingly still, the corner is decided on that plane's hairpin
//! instead, as the line's own crossing is (`grazed.rs`).
//!
//! A curve `Q` and `S` meet along that lies on `P` is the conic itself, `P`
//! cutting `Q` along one curve: the conic lies on `S` — a plane at 45°
//! through where the axes of two walls of one radius meet square holds one
//! of the two ellipses they meet along (9501873).
//!
//! A point found is kept where it stands within the tolerance of the conic,
//! on its arm's side of the plane of symmetry when it is an arm. Where `Q`
//! and `S` meet in a way the kernel does not build, so does the conic.

use super::super::elliptic::plane_and_elliptic;
use super::super::planar::{plane_and_cylinder, planes};
use super::super::{Relation, relation, square};
use super::grazed::grazed;
use super::lying::{STEEP, leans};
use super::section;
use super::{Crossing, Crossings, Touches, crossings_given};
use crate::brep::conic::{Conic, Round};
use crate::brep::curve::Curve;
use crate::brep::scale::Scale;
use crate::brep::surface::{Plane, Surface};

pub(super) fn conic_and_surface(
    conic: &Conic,
    surface: &Surface,
    scale: Scale,
    touching: &Touches,
) -> Crossings {
    let wall = match conic.round {
        Round::Cylinder(cylinder) => Surface::Cylinder(cylinder),
        Round::Cone(cone) => Surface::Cone(cone),
        Round::Elliptic(wall) => Surface::Elliptic(wall),
    };
    let own_plane = Surface::Plane(conic.plane);
    let (curves, across, along_a_touch) = match surface {
        Surface::Plane(plane) if let Some(lines) = along_the_axis(conic, plane, scale) => lines,
        Surface::Plane(plane) => match planes(&conic.plane, plane, scale) {
            Relation::Same { .. } => return Crossings::Along,
            Relation::Line(line) => (vec![Curve::Line(line)], wall, false),
            Relation::Apart => (Vec::new(), wall, false),
            Relation::Apex(_)
            | Relation::Lines(_)
            | Relation::Tangent(_)
            | Relation::Circle(_)
            | Relation::Meet(_)
            | Relation::Rulings { .. }
            | Relation::Conic(_)
            | Relation::Ruling { .. }
            | Relation::Unsupported => return Crossings::Unsupported,
        },
        Surface::Cylinder(_) | Surface::Cone(_) | Surface::Elliptic(_) => {
            match relation(&wall, surface, scale) {
                Relation::Same { .. } => return Crossings::Along,
                Relation::Lines(lines) => (lines.map(Curve::Line).to_vec(), own_plane, false),
                Relation::Tangent(line) => (vec![Curve::Line(line)], own_plane, true),
                Relation::Circle(circle) => (vec![Curve::Circle(circle)], own_plane, false),
                Relation::Meet(meeting) => (
                    meeting
                        .components
                        .iter()
                        .copied()
                        .map(Curve::Meet)
                        .collect(),
                    own_plane,
                    false,
                ),
                Relation::Apart | Relation::Apex(_) => (Vec::new(), own_plane, false),
                Relation::Line(_)
                | Relation::Rulings { .. }
                | Relation::Conic(_)
                | Relation::Ruling { .. }
                | Relation::Unsupported => return Crossings::Unsupported,
            }
        }
    };
    let mut kept = Vec::new();
    for curve in &curves {
        let touched = match (curve, surface, conic.round) {
            (Curve::Line(line), Surface::Plane(plane), Round::Cylinder(cylinder))
                if across == wall =>
            {
                section::touched(line, [&conic.plane, plane], &cylinder, scale)
            }
            (Curve::Line(line), Surface::Plane(plane), Round::Elliptic(elliptic))
                if across == wall =>
            {
                section::touched_on_an_ellipse(line, [&conic.plane, plane], &elliptic, scale)
            }
            _ => None,
        };
        let found = match touched {
            Some(at) => vec![Crossing {
                parameter: at,
                point: curve.point(at),
                tangent: true,
            }],
            None => match crossings_given(curve, &across, scale, touching) {
                Crossings::At(found) => found,
                Crossings::Along if across == own_plane => return Crossings::Along,
                Crossings::Along | Crossings::Unsupported => return Crossings::Unsupported,
            },
        };
        let decided = match surface {
            Surface::Plane(plane) if across == wall => {
                grazed([&conic.plane, plane], &wall, &found, scale)
            }
            _ => None,
        };
        let found = decided.map_or(found, |decided| {
            decided
                .into_iter()
                .map(|(point, tangent)| Crossing {
                    parameter: 0.0,
                    point,
                    tangent,
                })
                .collect()
        });
        for crossing in found {
            let point = crossing.point;
            let parameter = conic.parameter(point);
            if conic.holds(point) && conic.point(parameter).distance(point) <= scale.eps() {
                kept.push(Crossing {
                    parameter,
                    point,
                    tangent: crossing.tangent || along_a_touch,
                });
            }
        }
    }
    kept.sort_by(|one, other| one.parameter.total_cmp(&other.parameter));
    Crossings::At(kept)
}

/// The lines a plane read along the axis of a conic's cylinder or elliptic
/// wall cuts or touches it along, to be crossed with the conic's own plane: the triple's
/// most degenerate pair, so that the corner is the one those lines give
/// (`lying.rs`). None for a plane across the axis, on a cone, or where
/// the conic's plane leans too near the axis for those lines to fix where
/// they cross it.
fn along_the_axis(
    conic: &Conic,
    plane: &Plane,
    scale: Scale,
) -> Option<(Vec<Curve>, Surface, bool)> {
    let (wall, axis) = match conic.round {
        Round::Cylinder(cylinder) => (Surface::Cylinder(cylinder), cylinder.axis),
        Round::Elliptic(elliptic) => (Surface::Elliptic(elliptic), elliptic.axis),
        Round::Cone(_) => return None,
    };
    if !square(plane.normal, axis, scale)
        || !leans(&Surface::Plane(*plane), &wall, scale)
        || conic.plane.normal.dot(axis).abs() < STEEP
    {
        return None;
    }
    let own_plane = Surface::Plane(conic.plane);
    let found = match conic.round {
        Round::Elliptic(elliptic) => plane_and_elliptic(plane, &elliptic, scale),
        Round::Cylinder(cylinder) => plane_and_cylinder(plane, &cylinder, scale),
        Round::Cone(_) => return None,
    };
    Some(match found {
        Relation::Lines(lines) => (lines.map(Curve::Line).to_vec(), own_plane, false),
        Relation::Tangent(line) => (vec![Curve::Line(line)], own_plane, true),
        _ => (Vec::new(), own_plane, false),
    })
}
