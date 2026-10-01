//! Steps 8 and 9 of `docs/exact-kernel.md`: the kept regions made the faces of
//! a body, keeping only the edges they use and what those edges stand on,
//! renumbered in the arena's order; and the body checked before it is handed
//! back — what the kernel cannot verify it does not answer. Faces on one
//! surface with one side are merged across an arc nothing else uses first,
//! and edges at a vertex only they reach, in `merged.rs`.

mod merged;

use super::Declined;
use super::combine::Arena;
use super::topology::{Body, Coedge, CurveId, Edge, EdgeId, Face, SurfaceId, Vertex, VertexId};

pub(super) fn assembled(arena: Arena, faces: Vec<Face>) -> Result<Body, Declined> {
    let mut body = arena.body;
    let faces = merged::merged(&mut body.edges, &body.curves, &body.vertices, faces);
    let mut edges = Renumbering::of(body.edges.len());
    for coedge in faces.iter().flat_map(|face| face.loops.iter().flatten()) {
        edges.mark(coedge.edge.0);
    }
    let mut vertices = Renumbering::of(body.vertices.len());
    let mut curves = Renumbering::of(body.curves.len());
    for edge in edges.kept().map(|rank| &body.edges[rank]) {
        curves.mark(edge.curve.0);
        for end in edge.ends.iter().flatten() {
            vertices.mark(end.0);
        }
    }
    let mut surfaces = Renumbering::of(body.surfaces.len());
    for face in &faces {
        surfaces.mark(face.surface.0);
    }
    let assembled = Body {
        surfaces: surfaces.kept().map(|rank| body.surfaces[rank]).collect(),
        curves: curves.kept().map(|rank| body.curves[rank]).collect(),
        vertices: vertices
            .kept()
            .map(|rank| {
                let vertex = &body.vertices[rank];
                Vertex {
                    point: vertex.point,
                    on: vertex
                        .on
                        .iter()
                        .filter_map(|surface| surfaces.rank(surface.0).map(SurfaceId))
                        .collect(),
                }
            })
            .collect(),
        edges: edges
            .kept()
            .map(|rank| {
                let edge = &body.edges[rank];
                Edge {
                    curve: CurveId(curves.renumbered(edge.curve.0)),
                    ends: edge
                        .ends
                        .map(|ends| ends.map(|end| VertexId(vertices.renumbered(end.0)))),
                    from: edge.from,
                    to: edge.to,
                }
            })
            .collect(),
        faces: faces
            .iter()
            .map(|face| Face {
                surface: SurfaceId(surfaces.renumbered(face.surface.0)),
                flipped: face.flipped,
                loops: face
                    .loops
                    .iter()
                    .map(|lap| {
                        lap.iter()
                            .map(|coedge| Coedge {
                                edge: EdgeId(edges.renumbered(coedge.edge.0)),
                                forward: coedge.forward,
                            })
                            .collect()
                    })
                    .collect(),
            })
            .collect(),
        scale: body.scale,
        arrivals: surfaces
            .kept()
            .map(|rank| body.arrived(SurfaceId(rank as u32)))
            .collect(),
    };
    verified(&assembled)?;
    Ok(assembled)
}

/// Which of an arena's items are kept, and the rank each kept one takes.
struct Renumbering {
    new: Vec<Option<u32>>,
}

impl Renumbering {
    fn of(count: usize) -> Renumbering {
        Renumbering {
            new: vec![None; count],
        }
    }

    fn mark(&mut self, old: u32) {
        self.new[old as usize] = Some(0);
    }

    /// The kept items' old ranks, in order, numbering them as it goes.
    fn kept(&mut self) -> impl Iterator<Item = usize> + '_ {
        for (next, slot) in self.new.iter_mut().flatten().enumerate() {
            *slot = next as u32;
        }
        self.new
            .iter()
            .enumerate()
            .filter(|(_, slot)| slot.is_some())
            .map(|(rank, _)| rank)
    }

    fn rank(&self, old: u32) -> Option<u32> {
        self.new[old as usize]
    }

    fn renumbered(&self, old: u32) -> u32 {
        self.rank(old).expect("an item a kept one names is kept")
    }
}

/// Every edge run as many times one way as the other, and at least once;
/// every face with a loop, and every loop closed: each use starting where the
/// one before it ends, or a whole closed curve alone.
fn verified(body: &Body) -> Result<(), Declined> {
    let mut runs = vec![[0_usize; 2]; body.edges.len()];
    for face in &body.faces {
        if face.loops.is_empty() {
            return Err(Declined::Unverified);
        }
        for lap in &face.loops {
            for coedge in lap {
                runs[coedge.edge.0 as usize][usize::from(!coedge.forward)] += 1;
            }
            if !closed(body, lap) {
                return Err(Declined::Unverified);
            }
        }
    }
    if runs
        .iter()
        .any(|[forward, backward]| forward != backward || *forward == 0)
    {
        return Err(Declined::Unverified);
    }
    Ok(())
}

fn closed(body: &Body, lap: &[Coedge]) -> bool {
    let ends = |coedge: &Coedge| {
        body.edge(coedge.edge).ends.map(|[from, to]| {
            if coedge.forward {
                [from, to]
            } else {
                [to, from]
            }
        })
    };
    match lap {
        [] => false,
        [alone] if ends(alone).is_none() => true,
        _ => lap.iter().zip(lap.iter().cycle().skip(1)).all(|(one, next)| {
            matches!((ends(one), ends(next)), (Some([_, end]), Some([start, _])) if end == start)
        }),
    }
}

#[cfg(test)]
mod tests;
