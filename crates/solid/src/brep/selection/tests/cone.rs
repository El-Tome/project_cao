use std::f64::consts::{PI, TAU};

use glam::DVec2;

use super::*;
use crate::brep::curve::Curve;
use crate::brep::topology::Edge;
use crate::turning::{Axis, Corner, Straight, Turn};

/// A point of radius 5, its apex 10 along Y, turned by `degrees` about Y.
fn point(degrees: f64) -> Body {
    let contour = [[0.0, 0.0], [10.0, 0.0], [0.0, 5.0]]
        .iter()
        .enumerate()
        .map(|(run, at)| Corner {
            at: DVec2::from(*at),
            run: run as u32,
        })
        .collect();
    let straight = Straight {
        side: -1.0,
        contours: vec![contour],
        runs: 3,
        last_off_the_axis: Some(2),
    };
    let turn = Turn {
        axis: Axis {
            origin: DVec2::ZERO,
            direction: DVec2::Y,
        },
        angle: degrees.to_radians(),
        resolution: 0.0,
        on_the_axis: 0.0,
    };
    Body::turned(&straight, ground(0.0), &turn).expect("the point turns")
}

/// A body alone as an arena: each edge on the surfaces of the faces using
/// it.
fn alone(body: Body) -> Arena {
    let mut supports = vec![Vec::new(); body.edges.len()];
    for face in &body.faces {
        for coedge in face.loops.iter().flatten() {
            supports[coedge.edge.0 as usize].push(face.surface);
        }
    }
    for surfaces in &mut supports {
        surfaces.sort();
        surfaces.dedup();
    }
    Arena { body, supports }
}

fn the_cone(arena: &Arena) -> SurfaceId {
    let rank = arena
        .body
        .surfaces
        .iter()
        .position(|surface| matches!(surface, Surface::Cone(_)))
        .expect("a cone is there");
    SurfaceId(rank as u32)
}

/// A half point with the other half of its rim drawn on its cone: a whole
/// cone parted by two rulings through its apex.
fn halved() -> Arena {
    let mut arena = alone(point(180.0));
    let cone = the_cone(&arena);
    let body = &arena.body;
    let rim = body
        .edge_ids()
        .filter(|edge| arena.supports[edge.0 as usize].contains(&cone))
        .map(|edge| body.edge(edge))
        .find(|edge| matches!(body.curve(edge.curve), Curve::Circle(_)))
        .expect("the rim is there")
        .clone();
    let [one, other] = rim.ends.expect("the rim ends on the rulings");
    assert!(rim.from < rim.to);
    arena.body.edges.push(Edge {
        curve: rim.curve,
        ends: Some([other, one]),
        from: rim.to,
        to: rim.from + TAU,
    });
    arena.supports.push(vec![cone]);
    arena
}

fn bounded(overlay: &Overlay) -> Vec<&Region> {
    overlay
        .regions
        .iter()
        .filter(|region| !region.unbounded)
        .collect()
}

/// Every arc of the surface is run once each way over all the regions, and
/// nothing else is.
fn assert_each_arc_run_once_each_way(edges: &[EdgeId], overlay: &Overlay) {
    let mut runs = vec![[0, 0]; edges.len()];
    for (arc, forward) in overlay
        .regions
        .iter()
        .flat_map(|region| region.cycles.iter().flatten())
    {
        assert!(*arc < edges.len(), "a region is bounded by a floor arc");
        runs[*arc][usize::from(*forward)] += 1;
    }
    assert!(runs.iter().all(|run| *run == [1, 1]), "{runs:?}");
}

#[test]
fn a_whole_point_s_surface_has_a_bounded_cap_with_its_rim_alone() {
    let arena = alone(point(360.0));
    let cone = the_cone(&arena);
    let Surface::Cone(geometry) = *arena.body.surface(cone) else {
        unreachable!();
    };
    let (edges, overlay) = parted(&arena, cone, true).expect("the cone is parted");
    assert_eq!(edges.len(), 1);
    let [cap] = bounded(&overlay)[..] else {
        panic!("one bounded region, not {overlay:?}");
    };
    assert_eq!(cap.cycles.len(), 1);
    assert_eq!(cap.cycles[0].len(), 1);
    let rim = arena.body.trace(edges[0], cone).expect("traced").start().y;
    assert!(
        geometry.apex_at() < cap.inside.y && cap.inside.y < rim,
        "{cap:?}"
    );
    let (_, unheld) = parted(&arena, cone, false).expect("the cone is parted");
    assert!(bounded(&unheld).is_empty(), "no floor, no cap");
}

#[test]
fn a_point_turned_part_way_is_one_sector_bounded_by_its_rulings_and_its_rim() {
    for degrees in [90.0, 270.0, -200.0] {
        let arena = alone(point(degrees));
        let cone = the_cone(&arena);
        let (edges, overlay) = parted(&arena, cone, false).expect("the cone is parted");
        let [sector] = bounded(&overlay)[..] else {
            panic!("one sector at {degrees}°, not {overlay:?}");
        };
        assert_eq!(sector.cycles.len(), 1);
        assert_eq!(sector.cycles[0].len(), 3);
        assert_each_arc_run_once_each_way(&edges, &overlay);
    }
}

#[test]
fn a_cone_parted_by_two_rulings_through_its_apex_keeps_both_sectors() {
    let arena = halved();
    let cone = the_cone(&arena);
    let (edges, overlay) = parted(&arena, cone, true).expect("the cone is parted");
    let sectors = bounded(&overlay);
    assert_eq!(sectors.len(), 2, "{overlay:?}");
    for sector in sectors {
        assert_eq!(sector.cycles.len(), 1);
        assert_eq!(sector.cycles[0].len(), 3);
    }
    assert_each_arc_run_once_each_way(&edges, &overlay);
}

#[test]
fn no_region_of_a_cone_is_bounded_by_a_floor_arc() {
    for (arena, holds) in [
        (alone(point(360.0)), true),
        (alone(point(90.0)), false),
        (alone(point(270.0)), true),
        (halved(), false),
    ] {
        let cone = the_cone(&arena);
        let (edges, overlay) = parted(&arena, cone, holds).expect("the cone is parted");
        assert_each_arc_run_once_each_way(&edges, &overlay);
    }
}

fn surface_of(operands: &Operands, picked: impl Fn(&Surface) -> bool) -> SurfaceId {
    let rank = operands
        .surfaces
        .list
        .iter()
        .position(picked)
        .expect("the surface is there");
    SurfaceId(rank as u32)
}

#[test]
fn a_cone_and_a_plane_standing_apart_are_each_read_on_the_side_the_other_stands() {
    let point = point(360.0);
    let floor = block([-20.0, -20.0, -20.0], [20.0, -1.0, 20.0]);
    let operands = Operands::of(&point, &floor, point.scale().joined(floor.scale()));
    let cone = surface_of(&operands, |surface| matches!(surface, Surface::Cone(_)));
    let top = surface_of(&operands, |surface| {
        matches!(surface, Surface::Plane(plane)
            if plane.normal.y.abs() > 0.5 && (plane.offset().abs() - 1.0).abs() < 1e-9)
    });
    let (Surface::Cone(geometry), Surface::Plane(plane)) = (
        operands.surfaces.list[cone.0 as usize],
        operands.surfaces.list[top.0 as usize],
    ) else {
        unreachable!();
    };
    let above = |pair: [SurfaceId; 2], place: DVec3| {
        wrapped::lies_above(&operands, operands.scale_of(pair), pair, place)
    };
    let on_the_cone = geometry.point(DVec2::new(0.4, geometry.apex_at() / 2.0));
    let opening_against_its_axis = geometry.ruling.x < 0.0;
    assert_eq!(
        above([cone, top], on_the_cone),
        Some(opening_against_its_axis)
    );
    let on_the_plane = DVec3::new(1.0, -1.0, 0.0);
    assert_eq!(above([top, cone], on_the_plane), Some(plane.normal.y > 0.0));
}

#[test]
fn a_cone_halved_by_a_plane_through_its_axis_has_it_on_its_axis_side_up_to_the_rulings() {
    let point = point(360.0);
    let half = block([0.0, -20.0, -20.0], [20.0, 20.0, 20.0]);
    let operands = Operands::of(&point, &half, point.scale().joined(half.scale()));
    let cone = surface_of(&operands, |surface| matches!(surface, Surface::Cone(_)));
    let side = surface_of(&operands, |surface| {
        matches!(surface, Surface::Plane(plane)
            if plane.normal.x.abs() > 0.5 && plane.offset().abs() < 1e-9)
    });
    let Surface::Cone(geometry) = operands.surfaces.list[cone.0 as usize] else {
        unreachable!();
    };
    let above = |pair: [SurfaceId; 2], place: DVec3| {
        wrapped::lies_above(&operands, operands.scale_of(pair), pair, place)
    };
    let ruling = DVec3::Z.dot(geometry.v).atan2(DVec3::Z.dot(geometry.u));
    let l = geometry.apex_at() / 2.0;
    for angle in [ruling, ruling + 1.0, ruling + PI, ruling - 1.0] {
        let place = geometry.point(DVec2::new(angle, l));
        assert_eq!(
            above([cone, side], place),
            Some(geometry.ruling.x < 0.0),
            "at {angle}"
        );
    }
    let on_the_plane = geometry.point(DVec2::new(ruling, l));
    assert_eq!(above([side, cone], on_the_plane), None);
}
