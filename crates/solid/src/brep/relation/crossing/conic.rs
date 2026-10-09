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
//! A point found is kept where it stands within the tolerance of the conic,
//! on its arm's side of the plane of symmetry when it is an arm. Where `Q`
//! and `S` meet in a way the kernel does not build, so does the conic.

use super::super::planar::planes;
use super::super::{Relation, relation};
use super::{Crossing, Crossings, Touches, crossings_given};
use crate::brep::conic::{Conic, Round};
use crate::brep::curve::Curve;
use crate::brep::scale::Scale;
use crate::brep::surface::Surface;

pub(super) fn conic_and_surface(
    conic: &Conic,
    surface: &Surface,
    scale: Scale,
    touching: &Touches,
) -> Crossings {
    let wall = match conic.round {
        Round::Cylinder(cylinder) => Surface::Cylinder(cylinder),
        Round::Cone(cone) => Surface::Cone(cone),
    };
    let own_plane = Surface::Plane(conic.plane);
    let (curves, across, along_a_touch) = match surface {
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
        Surface::Cylinder(_) | Surface::Cone(_) => match relation(&wall, surface, scale) {
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
        },
    };
    let mut kept = Vec::new();
    for curve in &curves {
        let Crossings::At(found) = crossings_given(curve, &across, scale, touching) else {
            return Crossings::Unsupported;
        };
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
