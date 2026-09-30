use glam::{DVec2, DVec3};

use super::*;
use crate::brep::curve::{Curve, Line};
use crate::profile::{Contour, Frame};

fn block() -> Body {
    let outline = Contour::rectangle(DVec2::new(-20.0, -20.0), DVec2::new(20.0, 20.0));
    let frame = Frame {
        origin: DVec3::ZERO,
        u: DVec3::X,
        v: DVec3::Y,
    };
    Body::raised(&outline, &[], frame, DVec3::Z * 10.0).expect("a block raises")
}

#[test]
fn a_raised_block_is_verified() {
    assert_eq!(verified(&block()), Ok(()));
}

#[test]
fn a_face_laid_twice_leaves_its_edges_run_more_one_way_than_the_other() {
    let mut body = block();
    let again = body.faces[0].clone();
    body.faces.push(again);
    assert_eq!(verified(&body), Err(Declined::Unverified));
}

#[test]
fn a_face_missing_leaves_its_edges_run_one_way_only() {
    let mut body = block();
    body.faces.pop();
    assert_eq!(verified(&body), Err(Declined::Unverified));
}

#[test]
fn a_loop_whose_uses_do_not_follow_each_other_is_not_closed() {
    let mut body = block();
    body.faces[0].loops[0].swap(0, 1);
    assert_eq!(verified(&body), Err(Declined::Unverified));
}

#[test]
fn an_edge_no_face_uses_is_left_out_with_what_only_it_stands_on() {
    let body = block();
    let mut spare = body.clone();
    let extra = Vertex {
        point: DVec3::new(100.0, 0.0, 0.0),
        on: vec![SurfaceId(0)],
    };
    spare.vertices.insert(0, extra);
    for edge in &mut spare.edges {
        if let Some(ends) = edge.ends.as_mut() {
            for end in ends {
                end.0 += 1;
            }
        }
    }
    spare
        .curves
        .insert(0, Curve::Line(Line::through(DVec3::X * 100.0, DVec3::Y)));
    for edge in &mut spare.edges {
        edge.curve.0 += 1;
    }
    spare.edges.push(Edge {
        curve: CurveId(0),
        ends: Some([VertexId(0), VertexId(1)]),
        from: 0.0,
        to: 1.0,
    });
    let faces = spare.faces.clone();
    spare.faces.clear();
    let arena = Arena {
        body: spare,
        supports: Vec::new(),
    };
    let assembled = assembled(arena, faces).expect("a block assembles");
    assert_eq!(assembled, body);
}
