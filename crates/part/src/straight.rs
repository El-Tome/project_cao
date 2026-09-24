//! What a part's sketches are drawn with in straight lines — a point, a trait,
//! a trait grown both ways from its middle, a rectangle — laid down again as
//! the history replays them.

use cao_sketch::Support;
use glam::DVec2;

use crate::history::PointRef;
use crate::outcome::Outcome;
use crate::state::{PartState, hold, resolve};

impl PartState {
    pub(crate) fn add_point(
        &mut self,
        sketch: usize,
        position: DVec2,
        on: &[Support],
    ) -> Option<Outcome> {
        let drawing = self.sketches.get_mut(sketch)?;
        let point = drawing.add_point(position);
        hold(drawing, point, on);
        None
    }

    pub(crate) fn add_segment(
        &mut self,
        sketch: usize,
        start: &PointRef,
        end: &PointRef,
        construction: bool,
    ) -> Option<Outcome> {
        let sketch = self.sketches.get_mut(sketch)?;
        let start = resolve(sketch, start);
        let end = resolve(sketch, end);
        if start != end {
            if construction {
                sketch.add_construction_segment(start, end);
            } else {
                sketch.add_segment(start, end);
            }
        }
        None
    }

    pub(crate) fn add_symmetric_segment(
        &mut self,
        sketch: usize,
        middle: &PointRef,
        end: &PointRef,
        construction: bool,
    ) -> Option<Outcome> {
        let sketch = self.sketches.get_mut(sketch)?;
        let middle = resolve(sketch, middle);
        let end = resolve(sketch, end);
        let mirrored = sketch.point(middle) * 2.0 - sketch.point(end);
        if mirrored.distance(sketch.point(end)) > 1e-9 {
            let start = sketch.add_point(mirrored);
            let segment = if construction {
                sketch.add_construction_segment(start, end)
            } else {
                sketch.add_segment(start, end)
            };
            sketch.add_constraint(cao_sketch::Constraint::Midpoint {
                point: middle,
                segment,
            });
        }
        None
    }

    pub(crate) fn add_rectangle(
        &mut self,
        sketch: usize,
        corner: &PointRef,
        opposite: &PointRef,
        construction: bool,
    ) -> Option<Outcome> {
        let sketch = self.sketches.get_mut(sketch)?;
        // The two given corners may reuse points already drawn; the
        // other two are always new.
        let first = resolve(sketch, corner);
        let third = resolve(sketch, opposite);
        let (a, c) = (sketch.point(first), sketch.point(third));
        let second = sketch.add_point(DVec2::new(c.x, a.y));
        let fourth = sketch.add_point(DVec2::new(a.x, c.y));

        let corners = [first, second, third, fourth];
        for index in 0..4 {
            let (from, to) = (corners[index], corners[(index + 1) % 4]);
            if construction {
                sketch.add_construction_segment(from, to);
            } else {
                sketch.add_segment(from, to);
            }
        }
        None
    }
}
