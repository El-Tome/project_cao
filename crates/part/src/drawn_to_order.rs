//! A part drawn from a recipe, for a test or a measurement that needs
//! something complete to work on.
//!
//! Every test draws the smallest part that proves its point, which is right
//! for a test and leaves nothing to measure an open, a save or a replay
//! against. A recipe says how much of each thing a part holds and draws one:
//! rings of traits, circles bored through them, arcs, dimensions, rules that
//! hold, matter raised from every ring, and sketches started on the face that
//! matter left.
//!
//! The same recipe always draws the same part. Nothing here is random: a
//! figure measured on one part has to mean the same thing the next run.

use chrono::{DateTime, Utc};
use glam::{DVec2, DVec3};

use cao_sketch::{Constraint, DimensionTarget, PointId, SegmentId, WorkPlane};

use crate::document::PartDocument;
use crate::history::{ExtrusionMode, Operation, PointRef};

/// How far the corners of a ring stand from its middle, in world units.
const REACH: f64 = 20.0;
/// Between the middles of two rings, wide enough that they never touch.
const APART: f64 = 60.0;
/// What every extrusion raises, in millimetres like any length typed.
const STOREY: f64 = 10.0;

/// What a part drawn to order is to hold.
pub struct Recipe {
    /// Rings drawn on the base plane, each raised into matter.
    pub sketches: usize,
    /// Traits in each ring, three or more.
    pub sides: usize,
    /// Circles bored through the matter of a ring.
    pub circles: usize,
    /// Arcs laid beside a ring, part of no area.
    pub arcs: usize,
    /// Traits of a ring given a length. The first one a part is given fixes
    /// what its drawing is worth in millimetres; the ones after it move the
    /// drawing to the nearest whole millimetre.
    pub dimensions: usize,
    /// Traits held to the length of the first one.
    pub rules: usize,
    /// Sketches started on the face the matter left, each raised in turn.
    pub storeys: usize,
}

impl Default for Recipe {
    fn default() -> Self {
        Self {
            sketches: 1,
            sides: 4,
            circles: 1,
            arcs: 1,
            dimensions: 2,
            rules: 1,
            storeys: 1,
        }
    }
}

impl Recipe {
    pub fn drawn(&self, name: impl Into<String>, now: DateTime<Utc>) -> PartDocument {
        let mut document = PartDocument::new(name, now);

        for ring in 0..self.sketches {
            let middle = DVec2::new(ring as f64 * APART, 0.0);
            document.apply(Operation::CreateSketch {
                plane: WorkPlane::XY,
            });
            self.draw(&mut document, ring, middle, REACH);
            document.apply(Operation::Extrude {
                sketch: ring,
                picks: vec![middle],
                distance: STOREY,
                mode: ExtrusionMode::Add,
            });
        }

        for storey in 0..self.storeys {
            let sketch = self.sketches + storey;
            // Where the matter below has got to, rather than what was asked
            // for: an extrusion travels the distance typed in millimetres, and
            // what that is worth in world units is the scale the first
            // dimension fixed.
            let height = document.body().bounds().map_or(0.0, |(_, top)| top.z);
            document.apply(Operation::CreateSketch {
                plane: WorkPlane::from_normal(DVec3::new(0.0, 0.0, height), DVec3::Z),
            });
            let middle = document.sketches()[sketch]
                .plane
                .to_local(DVec3::ZERO.with_z(height));
            self.draw(&mut document, sketch, middle, REACH / 3.0);
            document.apply(Operation::Extrude {
                sketch,
                picks: vec![middle],
                distance: STOREY,
                mode: ExtrusionMode::Add,
            });
        }

        document
    }

    /// A ring of traits, whatever the recipe hangs on it, and the values and
    /// rules that hold it.
    fn draw(&self, document: &mut PartDocument, sketch: usize, middle: DVec2, reach: f64) {
        let corner = |side: usize| {
            middle
                + reach * DVec2::from_angle(std::f64::consts::TAU * side as f64 / self.sides as f64)
        };

        let mut first = PointId(0);
        let mut start = PointRef::New(corner(0));
        for side in 1..=self.sides {
            let end = if side == self.sides {
                PointRef::Existing(first)
            } else {
                PointRef::New(corner(side))
            };
            document.apply(Operation::AddSegment {
                sketch,
                start,
                end,
                construction: false,
            });
            let drawn = document.sketches()[sketch].points().len();
            if side == 1 {
                first = PointId(drawn - 2);
            }
            start = PointRef::Existing(PointId(drawn - 1));
        }

        for bore in 0..self.circles {
            let turn = std::f64::consts::TAU * bore as f64 / self.circles.max(1) as f64;
            document.apply(Operation::AddCircle {
                sketch,
                center: PointRef::New(middle + reach / 2.0 * DVec2::from_angle(turn)),
                radius: reach / 6.0,
                rim: vec![],
                construction: false,
            });
        }

        for arc in 0..self.arcs {
            let centre = middle + DVec2::new(arc as f64 * reach / 2.0, reach * 1.7);
            document.apply(Operation::AddArc {
                sketch,
                center: PointRef::New(centre),
                start: PointRef::New(centre + DVec2::new(reach / 4.0, 0.0)),
                end: PointRef::New(centre + DVec2::new(0.0, reach / 4.0)),
                construction: false,
            });
        }

        for value in 0..self.dimensions {
            let target = DimensionTarget::Length(SegmentId(value % self.sides));
            let Some(measured) = document.measured(sketch, target) else {
                continue;
            };
            let asked = measured.round();
            document.apply(Operation::SetDimension {
                sketch,
                target,
                value: if asked > 0.0 { asked } else { measured },
                placement: None,
            });
        }

        for rule in 0..self.rules {
            document.apply(Operation::Constrain {
                sketch,
                constraint: Constraint::Equal {
                    first: SegmentId(0),
                    second: SegmentId(1 + rule % self.sides.saturating_sub(1).max(1)),
                },
            });
        }
    }
}

#[cfg(test)]
mod tests;
