use std::f64::consts::TAU;

use glam::{DVec2, DVec3};

use super::*;
use crate::brep::curve::Curve;
use crate::brep::surface::Surface;
use crate::brep::topology::{Edge, Vertex};
use crate::profile::{Contour, Frame, Run};

fn ground(height: f64) -> Frame {
    Frame {
        origin: DVec3::Z * height,
        u: DVec3::X,
        v: DVec3::Y,
    }
}

fn block(low: [f64; 3], high: [f64; 3]) -> Body {
    let outline = Contour::rectangle(DVec2::new(low[0], low[1]), DVec2::new(high[0], high[1]));
    Body::raised(&outline, &[], ground(low[2]), DVec3::Z * (high[2] - low[2]))
        .expect("a block raises")
}

fn standing(center: [f64; 2], radius: f64, from: f64, to: f64) -> Body {
    let center = DVec2::from(center);
    let outline = Contour {
        corners: vec![center + DVec2::X * radius],
        runs: vec![Run::Round { center, turn: TAU }],
    };
    Body::raised(&outline, &[], ground(from), DVec3::Z * (to - from)).expect("a cylinder raises")
}

fn arena(first: &Body, second: &Body) -> Arena {
    let operands = Operands::of(first, second, first.scale().joined(second.scale()));
    laid(&operands).expect("the arena is laid")
}

/// The ranks of the planes of `body` at `offset` along a world axis.
fn plane_at(body: &Body, axis: DVec3, offset: f64) -> SurfaceId {
    let rank = body
        .surfaces
        .iter()
        .position(|surface| {
            matches!(surface, Surface::Plane(plane)
                if plane.normal.abs_diff_eq(axis, 1e-12) && (plane.offset() - offset).abs() < 1e-12)
        })
        .expect("the plane is there");
    SurfaceId(rank as u32)
}

#[test]
fn two_blocks_sharing_a_wall_share_the_four_corners_of_that_wall() {
    let one = block([-20.0, -20.0, 0.0], [20.0, 20.0, 10.0]);
    let other = block([20.0, -20.0, 0.0], [60.0, 20.0, 10.0]);
    let laid = arena(&one, &other);
    assert_eq!(laid.body.vertices.len(), 12);
    let wall = plane_at(&laid.body, DVec3::X, 20.0);
    let on_the_wall: Vec<&Vertex> = laid
        .body
        .vertices
        .iter()
        .filter(|vertex| vertex.on.contains(&wall))
        .collect();
    assert_eq!(on_the_wall.len(), 4);
    for vertex in on_the_wall {
        assert_eq!(vertex.on.len(), 3, "{vertex:?}");
    }
}

#[test]
fn a_hole_s_circle_on_the_cap_it_is_flush_with_is_one_edge_on_the_cap_and_on_the_hole() {
    let stock = standing([0.0, 0.0], 20.0, 0.0, 10.0);
    let hole = standing([8.0, 0.0], 5.0, 0.0, 10.0);
    let laid = arena(&stock, &hole);
    let top = plane_at(&laid.body, DVec3::Z, 10.0);
    let on_the_top_of_the_hole: Vec<&Edge> = laid
        .body
        .edges
        .iter()
        .filter(|edge| {
            matches!(laid.body.curve(edge.curve), Curve::Circle(circle)
                if circle.radius == 5.0 && circle.center.z == 10.0)
        })
        .collect();
    let [edge] = on_the_top_of_the_hole.as_slice() else {
        panic!("one edge: {on_the_top_of_the_hole:?}");
    };
    assert_eq!(edge.ends, None);
    let support = &laid.supports[edge.curve.0 as usize];
    assert_eq!(support.len(), 2);
    assert!(support.contains(&top), "{support:?}");
    assert_eq!(laid.body.edges.len(), 4);
}

#[test]
fn a_circle_where_a_top_meets_a_cylinder_is_kept_across_the_top_and_nowhere_else() {
    let one = block([0.0, 0.0, 0.0], [10.0, 10.0, 10.0]);
    let tall = standing([12.0, 5.0], 5.0, -5.0, 15.0);
    let laid = arena(&one, &tall);
    let kept: Vec<&Edge> = laid
        .body
        .edges
        .iter()
        .filter(|edge| {
            matches!(laid.body.curve(edge.curve), Curve::Circle(circle) if circle.center.z == 10.0)
        })
        .collect();
    let [edge] = kept.as_slice() else {
        panic!("one arc of the circle: {kept:?}");
    };
    let middle = laid
        .body
        .curve(edge.curve)
        .point((edge.from + edge.to) / 2.0);
    assert!(middle.x < 10.0, "{middle}");
    assert!(edge.ends.is_some());
}

#[test]
fn an_edge_of_one_block_crossing_the_other_s_plane_off_its_face_makes_no_corner() {
    let one = block([-20.0, -20.0, 0.0], [20.0, 20.0, 10.0]);
    let pocket = block([10.0, -5.0, 5.0], [20.0, 5.0, 10.0]);
    let laid = arena(&one, &pocket);
    let far = DVec3::new(10.0, -20.0, 10.0);
    assert!(
        laid.body
            .vertices
            .iter()
            .all(|vertex| vertex.point.distance(far) > 1.0)
    );
    assert_eq!(laid.body.vertices.len(), 16);
}

#[test]
fn a_ruling_where_a_hole_touches_a_side_is_one_edge_between_the_two_corners_it_touches_at() {
    let one = block([-20.0, -20.0, 0.0], [20.0, 20.0, 10.0]);
    let hole = standing([15.0, 0.0], 5.0, 0.0, 10.0);
    let laid = arena(&one, &hole);
    let rulings: Vec<&Edge> = laid
        .body
        .edges
        .iter()
        .filter(|edge| {
            matches!(laid.body.curve(edge.curve), Curve::Line(line)
                if line.direction.abs_diff_eq(DVec3::Z, 1e-12)
                    && line.origin.abs_diff_eq(DVec3::new(20.0, 0.0, 0.0), 1e-9))
        })
        .collect();
    let [ruling] = rulings.as_slice() else {
        panic!("one ruling: {rulings:?}");
    };
    let ends = ruling.ends.expect("a ruling has two corners");
    let heights = ends.map(|end| laid.body.vertex(end).point.z);
    assert_eq!(heights, [0.0, 10.0]);
    assert_eq!(laid.supports[ruling.curve.0 as usize].len(), 2);
}
