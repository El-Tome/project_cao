//! What a boolean reads of an operand lying on a cone (#536): which of its
//! edges the cone carries all along, and where a point stands against its
//! faces.

use std::f64::consts::PI;

use super::*;
use crate::brep::domain::Location;
use crate::turning::{Axis, Corner, Straight, Turn};

/// A point of radius 5 on the plane `y = 0`, its apex at `(0, 10, 0)`,
/// turned about Y by `angle`.
fn point_turned(angle: f64) -> Body {
    point_of(10.0, 5.0, angle)
}

/// A point of `radius` on the plane `y = 0`, its apex at `(0, height, 0)`,
/// turned about Y by `angle`.
fn point_of(height: f64, radius: f64, angle: f64) -> Body {
    let corners = [[0.0, 0.0], [height, 0.0], [0.0, radius]];
    let straight = Straight {
        side: -1.0,
        contours: vec![
            (0..3)
                .map(|run| Corner {
                    at: DVec2::from(corners[run]),
                    run: run as u32,
                })
                .collect(),
        ],
        runs: 3,
        last_off_the_axis: Some(2),
    };
    let turn = Turn {
        axis: Axis {
            origin: DVec2::ZERO,
            direction: DVec2::Y,
        },
        angle,
        resolution: 0.0,
        on_the_axis: 0.0,
    };
    Body::turned(&straight, ground(0.0), &turn).expect("the point turns")
}

fn operands<'a>(first: &'a Body, second: &'a Body) -> Operands<'a> {
    Operands::of(first, second, first.scale().joined(second.scale()))
}

#[test]
fn every_edge_of_a_point_is_carried_all_along_by_the_surfaces_beside_it() {
    let far = block([40.0, 40.0, 40.0], [50.0, 50.0, 50.0]);
    for angle in [TAU, PI / 2.0, PI] {
        let point = point_turned(angle);
        let operands = operands(&point, &far);
        let list = &operands.surfaces.list;
        let apart = Apart::of(list, |pair| operands.scale_of(pair));
        let mut registry = Registry::new(operands.scale, apart, Planes::of(list));
        let (held, _) = held::held(&operands, &mut registry);
        for found in held.iter().filter(|found| found.operand == 0) {
            assert!(found.partly.is_empty(), "{angle}: {found:?}");
        }
    }
}

/// A flat point, its rulings further from its axis than along it: a point
/// of the other nappe reads in `(θ, l)` as one in front of the apex.
#[test]
fn a_point_of_the_mirror_nappe_is_outside_every_face() {
    let point = point_of(2.0, 10.0, TAU);
    let far = block([40.0, 40.0, 40.0], [50.0, 50.0, 50.0]);
    let operands = operands(&point, &far);
    let cone = operands
        .surfaces
        .list
        .iter()
        .position(|surface| matches!(surface, Surface::Cone(_)))
        .map(|rank| SurfaceId(rank as u32))
        .expect("the point lies on a cone");
    let Surface::Cone(nappe) = operands.surfaces.list[cone.0 as usize] else {
        unreachable!()
    };
    let at = DVec3::new(2.0, 2.4, 0.0);
    assert!(nappe.parameters(at).y > nappe.apex_at());
    let mirrored = operands
        .located(0, cone, at, operands.eps())
        .expect("the point is located");
    assert!(
        mirrored
            .iter()
            .all(|(_, location)| *location == Location::Outside),
        "{mirrored:?}"
    );
}
