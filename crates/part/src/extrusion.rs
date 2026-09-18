//! Turning a sketch's areas into matter, or taking matter away: a prism along
//! the plane's normal, or a sweep around an axis lying in the plane.

use cao_sketch::Sketch;
use cao_solid::Mesh;
use glam::DVec2;

use crate::history::{ExtrusionMode, RevolutionAxis};
use crate::state::PartState;

impl PartState {
    /// Sweeps the chosen areas around an axis of the sketch and joins the
    /// result to the part, or takes it out.
    pub(crate) fn revolve(
        &mut self,
        index: usize,
        picks: &[DVec2],
        axis: RevolutionAxis,
        degrees: f64,
        mode: ExtrusionMode,
    ) {
        let Some(sketch) = self.sketches.get(index) else {
            return;
        };
        let Some((axis_origin, axis_direction)) = axis_in_sketch(sketch, axis) else {
            return;
        };

        let plane = sketch.plane;
        let turn = degrees.to_radians();
        let regions = sketch.regions();

        let mut tool = Mesh::default();
        for pick in picks {
            let Some(region) = regions
                .iter()
                .filter(|region| region.contains(*pick))
                .max_by_key(|region| region.depth)
            else {
                continue;
            };
            let (outline, holes) = loops(region);
            let Some(piece) = cao_solid::revolution(
                outline,
                &holes,
                &region.face_triangles(),
                |point| plane.to_world(point),
                axis_origin,
                axis_direction,
                turn,
            ) else {
                continue;
            };
            tool = tool.union(&piece);
        }

        self.combine(tool, mode);
    }

    /// Joins a tool to the part, or takes it out.
    pub(crate) fn combine(&mut self, tool: Mesh, mode: ExtrusionMode) {
        if tool.is_empty() {
            return;
        }
        self.body = match mode {
            ExtrusionMode::Add => self.body.union(&tool),
            ExtrusionMode::Cut => self.body.difference(&tool),
        };
    }

    /// Turns the chosen areas of a sketch into a prism and joins it to the
    /// part, or takes it out.
    ///
    /// Every area is turned into matter first and the lot is applied in one go:
    /// two areas extruded together must behave as one shape, not as two that
    /// happen to be cut one after the other.
    pub(crate) fn extrude(
        &mut self,
        index: usize,
        picks: &[DVec2],
        distance: f64,
        mode: ExtrusionMode,
    ) {
        let scale = self.scale();
        let Some(sketch) = self.sketches.get(index) else {
            return;
        };
        if distance.abs() < 1e-6 {
            return;
        }

        let plane = sketch.plane;
        let travel = plane.normal() * (distance / scale);
        let regions = sketch.regions();

        let mut tool = Mesh::default();
        for pick in picks {
            // The area is found again by the point that was clicked, so the
            // extrusion still means the same thing after the drawing changes.
            let Some(region) = regions
                .iter()
                .filter(|region| region.contains(*pick))
                .max_by_key(|region| region.depth)
            else {
                continue;
            };
            let (outline, holes) = loops(region);
            let piece = cao_solid::prism(
                outline,
                &holes,
                &region.face_triangles(),
                |point| plane.to_world(point),
                travel,
            );
            tool = tool.union(&piece);
        }

        self.combine(tool, mode);
    }
}

/// Where a revolution's axis lies, in the sketch's own coordinates.
fn axis_in_sketch(sketch: &Sketch, axis: RevolutionAxis) -> Option<(DVec2, DVec2)> {
    match axis {
        RevolutionAxis::Sketch(axis) => Some((DVec2::ZERO, axis.direction())),
        RevolutionAxis::Segment(segment) => {
            if segment.0 >= sketch.segments().len() {
                return None;
            }
            let (start, end) = sketch.endpoints(segment);
            ((end - start).length() > 1e-6).then_some((start, end - start))
        }
    }
}

/// The loops a region hands the solid: its outline and what it leaves hollow,
/// each carrying the curve every segment was sampled from so that a wall
/// raised from one curve comes out as one face.
fn loops(region: &cao_sketch::Region) -> (cao_solid::Loop<'_>, Vec<cao_solid::Loop<'_>>) {
    fn borrow(outline: &cao_sketch::Outline) -> cao_solid::Loop<'_> {
        cao_solid::Loop {
            points: &outline.points,
            curves: &outline.curves,
        }
    }
    (
        borrow(&region.outline),
        region.holes.iter().map(borrow).collect(),
    )
}
