//! A body's topology as AP214 bounds a solid: vertices, edges along their
//! curves, loops of oriented edges, faces on their surfaces, one closed
//! shell.

use super::geometry;
use super::instances::{Instances, flag};
use crate::brep::{ListedEdge, ListedFace, Listing};

pub fn written(instances: &mut Instances, listing: &Listing) -> usize {
    let vertices: Vec<usize> = listing
        .vertices
        .iter()
        .map(|point| {
            let point = instances.point(*point);
            instances.add(format!("VERTEX_POINT('',#{point})"))
        })
        .collect();
    let edges: Vec<usize> = listing
        .edges
        .iter()
        .map(|edge| edge_curve(instances, edge, &vertices))
        .collect();
    let faces: Vec<String> = listing
        .faces
        .iter()
        .map(|face| format!("#{}", advanced_face(instances, face, &edges)))
        .collect();
    let shell = instances.add(format!("CLOSED_SHELL('',({}))", faces.join(",")));
    instances.add(format!("MANIFOLD_SOLID_BREP('',#{shell})"))
}

/// A whole closed curve has no vertex of its own: it starts and ends on one
/// laid where its parameter starts.
fn edge_curve(instances: &mut Instances, edge: &ListedEdge, vertices: &[usize]) -> usize {
    let [start, end] = match edge.ends {
        Some(ends) => ends.map(|end| vertices[end]),
        None => {
            let point = instances.point(edge.curve.point(edge.from));
            let vertex = instances.add(format!("VERTEX_POINT('',#{point})"));
            [vertex, vertex]
        }
    };
    let curve = geometry::curve(instances, &edge.curve);
    instances.add(format!(
        "EDGE_CURVE('',#{start},#{end},#{curve},{})",
        flag(edge.to > edge.from)
    ))
}

fn advanced_face(instances: &mut Instances, face: &ListedFace, edges: &[usize]) -> usize {
    let mut bounds = Vec::new();
    for lap in &face.loops {
        let mut oriented = Vec::new();
        for (edge, forward) in lap {
            let used = instances.add(format!(
                "ORIENTED_EDGE('',*,*,#{},{})",
                edges[*edge],
                flag(*forward)
            ));
            oriented.push(format!("#{used}"));
        }
        let edge_loop = instances.add(format!("EDGE_LOOP('',({}))", oriented.join(",")));
        let bound = instances.add(format!("FACE_BOUND('',#{edge_loop},.T.)"));
        bounds.push(format!("#{bound}"));
    }
    let surface = geometry::surface(instances, &face.surface);
    instances.add(format!(
        "ADVANCED_FACE('',({}),#{surface},{})",
        bounds.join(","),
        flag(face.outward)
    ))
}
