//! A body as truck holds it, behind the vocabulary `cao_solid` speaks: raise
//! this profile, join, cut, and hand back the triangles to draw.

use glam::{DVec2, DVec3};

pub use crate::profile::{Contour, Frame, Run};
use truck_meshalgo::prelude::*;
use truck_modeling::{Face, Point3, Solid, Vector3, Wire, builder};

pub type Triangle = [DVec3; 3];

#[derive(Clone, Debug)]
pub struct Body(Solid);

fn point(of: DVec3) -> Point3 {
    Point3::new(of.x, of.y, of.z)
}

fn vector(of: DVec3) -> Vector3 {
    Vector3::new(of.x, of.y, of.z)
}

fn wire(contour: &Contour, frame: &Frame) -> Wire {
    let vertices: Vec<_> = contour
        .corners
        .iter()
        .map(|corner| builder::vertex(point(frame.at(*corner))))
        .collect();
    let count = vertices.len();
    (0..count)
        .map(|index| {
            let (from, to) = (&vertices[index], &vertices[(index + 1) % count]);
            match contour.runs[index] {
                Run::Straight => builder::line(from, to),
                Run::Round { center, turn } => {
                    let start = contour.corners[index] - center;
                    let middle = center + DVec2::from_angle(turn / 2.0).rotate(start);
                    builder::circle_arc(from, to, point(frame.at(middle)))
                }
            }
        })
        .collect()
}

/// Truck's own triangles for a body, `None` when the mesher gave up on a face
/// or panicked.
fn meshed(solid: &Solid, tolerance: f64) -> Option<Vec<Triangle>> {
    let run = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let shape = solid.triangulation(tolerance);
        let complete = shape
            .boundaries()
            .iter()
            .flat_map(|shell| shell.face_iter())
            .all(|face| face.surface().is_some());
        let mut mesh = shape.to_polygon();
        mesh.put_together_same_attrs(TOLERANCE * 2.0)
            .remove_degenerate_faces()
            .remove_unused_attrs();
        let positions = mesh.positions().to_vec();
        let triangles: Vec<Triangle> = mesh
            .tri_faces()
            .iter()
            .map(|corners| {
                corners.map(|vertex| {
                    let p = positions[vertex.pos];
                    DVec3::new(p.x, p.y, p.z)
                })
            })
            .collect();
        complete.then_some(triangles)
    }));
    run.ok().flatten()
}

impl Body {
    /// A profile pushed along `travel`, which need not be square to its
    /// plane's normal but must not lie in it.
    pub fn raised(
        outline: &Contour,
        holes: &[Contour],
        frame: Frame,
        travel: DVec3,
    ) -> Option<Body> {
        let mut wires = vec![wire(outline, &frame)];
        for hole in holes {
            wires.push(wire(hole, &frame));
        }
        let mut face: Face = builder::try_attach_plane(&wires).ok()?;
        let normal = match face.oriented_surface() {
            truck_modeling::Surface::Plane(plane) => plane.normal(),
            _ => return None,
        };
        if normal.dot(vector(travel)) < 0.0 {
            face.invert();
        }
        Some(Body(builder::tsweep(&face, vector(travel))))
    }

    pub fn joined(&self, other: &Body) -> Option<Body> {
        truck_shapeops::or(&self.0, &other.0, BOOLEAN).map(Body)
    }

    pub fn cut_by(&self, tool: &Body) -> Option<Body> {
        let mut inside_out = tool.0.clone();
        inside_out.not();
        truck_shapeops::and(&self.0, &inside_out, BOOLEAN).map(Body)
    }

    pub fn faces(&self) -> usize {
        self.0.boundaries().iter().map(|shell| shell.len()).sum()
    }

    /// The triangles to draw, laid within `MESH` of the true surface.
    pub fn triangles(&self) -> Option<Vec<Triangle>> {
        meshed(&self.0, MESH)
    }
}

/// How far apart two surfaces may be and still be found meeting, in the
/// units of the part: truck's own floor is a millionth.
pub const BOOLEAN: f64 = 0.05;

/// How far a triangle may stand from the surface it stands for.
pub const MESH: f64 = 0.01;
