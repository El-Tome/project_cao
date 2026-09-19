//! Turning a sketch's areas into matter, or taking matter away: a prism along
//! the plane's normal, or a sweep around an axis lying in the plane.

use cao_sketch::{Area, Region, Sketch};
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
        areas: &[Area],
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
        for area in areas {
            let Some(region) = self.standing_on(index, area, &regions) else {
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
        areas: &[Area],
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
        for area in areas {
            let Some(region) = self.standing_on(index, area, &regions) else {
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

    /// The areas of a drawing these places fall in, each named by the curves
    /// that bound it.
    ///
    /// This is what a click means, and it is worked out at the moment of the
    /// click and never again: snapping and what lies under the cursor depend
    /// on the view at the time, the same reason `PointRef` records the point
    /// a click landed on.
    pub fn areas_at(&self, sketch: usize, places: &[DVec2]) -> Vec<Area> {
        let Some(drawing) = self.sketches.get(sketch) else {
            return Vec::new();
        };
        let regions = drawing.regions();
        places
            .iter()
            .filter_map(|place| {
                let rank = cao_sketch::area_under(&regions, *place)?;
                Some(Area::of(&regions[rank], *place))
            })
            .collect()
    }

    /// The area a step of matter stands on, as the drawing holds it now.
    ///
    /// The name it was given at the click is followed through every cut made
    /// since, and then asked of the drawing. Nothing when the drawing no
    /// longer encloses it — which is what the tree says, rather than raising
    /// matter somewhere the user never pointed at.
    fn standing_on<'a>(
        &self,
        sketch: usize,
        area: &Area,
        regions: &'a [Region],
    ) -> Option<&'a Region> {
        regions.get(self.area_rank(sketch, area, regions)?)
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
