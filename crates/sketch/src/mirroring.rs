use glam::DVec2;
use serde::{Deserialize, Serialize};

use crate::arc::ArcId;
use crate::constraints::SketchAxis;
use crate::element::Element;
use crate::sketch::{CircleId, PointId, SegmentId, Sketch};

/// Below this a trait runs nowhere, and names no direction to mirror across.
const NO_DIRECTION: f64 = 1e-9;

/// What a selection is mirrored across.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum MirrorAxis {
    /// A trait of the drawing, which follows it when it moves.
    Trait(SegmentId),
    /// One of the sketch's own two axes.
    Sketch(SketchAxis),
}

/// What a copy of part of the drawing left behind.
///
/// Geometry and nothing else: the rules and the values that spoke of the
/// original stay with it. A pattern lays the same copy down many times, and
/// carrying a rule over each time would hand the solver the same figure to hold
/// once per copy.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Duplicated {
    pub points: Vec<PointId>,
    pub segments: Vec<SegmentId>,
    pub circles: Vec<CircleId>,
    pub arcs: Vec<ArcId>,
}

impl Sketch {
    /// Copies the elements given to the other side of an axis.
    ///
    /// Nothing when the axis names no direction.
    pub fn mirror(&mut self, of: &[Element], axis: MirrorAxis) -> Option<Duplicated> {
        let (through, along) = self.axis_line(axis)?;
        let kept: Vec<Element> = match axis {
            MirrorAxis::Trait(id) => of
                .iter()
                .copied()
                .filter(|held| *held != Element::Segment(id))
                .collect(),
            MirrorAxis::Sketch(_) => of.to_vec(),
        };
        Some(self.duplicate(&kept, |at| {
            let offset = at - through;
            through + along * 2.0 * offset.dot(along) - offset
        }))
    }

    /// A point the axis runs through, and the direction it runs in.
    fn axis_line(&self, axis: MirrorAxis) -> Option<(DVec2, DVec2)> {
        match axis {
            MirrorAxis::Sketch(SketchAxis::U) => Some((DVec2::ZERO, DVec2::X)),
            MirrorAxis::Sketch(SketchAxis::V) => Some((DVec2::ZERO, DVec2::Y)),
            MirrorAxis::Trait(id) => {
                if self.is_erased_segment(id) || id.0 >= self.segments().len() {
                    return None;
                }
                let (start, end) = self.endpoints(id);
                let along = end - start;
                (along.length() > NO_DIRECTION).then(|| (start, along.normalize()))
            }
        }
    }

    /// Lays down a second copy of the elements given, every place they stand on
    /// carried through `by`.
    ///
    /// The copy keeps the original's own joins: two sides that shared a corner
    /// share their copy's corner too, rather than each getting one of its own.
    pub fn duplicate(&mut self, of: &[Element], by: impl Fn(DVec2) -> DVec2) -> Duplicated {
        let mut made = Duplicated::default();
        let mut copied: Vec<(PointId, PointId)> = Vec::new();
        for held in of {
            for point in self.points_it_leans_on(*held) {
                if copied.iter().any(|(was, _)| *was == point) {
                    continue;
                }
                let now = self.add_point(by(self.point(point)));
                copied.push((point, now));
                made.points.push(now);
            }
        }
        let copy_of = |point: PointId| {
            copied
                .iter()
                .find(|(was, _)| *was == point)
                .map(|(_, now)| *now)
        };

        for held in of {
            match *held {
                Element::Point(_) => {}
                Element::Segment(id) => {
                    let Some(side) = self.segments().get(id.0).copied() else {
                        continue;
                    };
                    let (Some(start), Some(end)) = (copy_of(side.start), copy_of(side.end)) else {
                        continue;
                    };
                    made.segments.push(match side.construction {
                        true => self.add_construction_segment(start, end),
                        false => self.add_segment(start, end),
                    });
                }
                Element::Circle(id) => {
                    let Some(round) = self.circles().get(id.0).copied() else {
                        continue;
                    };
                    let Some(centre) = copy_of(round.center) else {
                        continue;
                    };
                    made.circles.push(match round.construction {
                        true => self.add_construction_circle(centre, round.radius),
                        false => self.add_circle(centre, round.radius),
                    });
                }
                Element::Arc(id) => {
                    let Some(curve) = self.arcs().get(id.0).copied() else {
                        continue;
                    };
                    let (Some(centre), Some(start), Some(end)) = (
                        copy_of(curve.center),
                        copy_of(curve.start),
                        copy_of(curve.end),
                    ) else {
                        continue;
                    };
                    // An arc runs counter-clockwise from one end to the other.
                    // A transform that turns the plane over turns that around
                    // with it, so the two ends change places or the copy sweeps
                    // the long way round.
                    let (start, end) = match turns_over(&by) {
                        true => (end, start),
                        false => (start, end),
                    };
                    made.arcs.push(match curve.construction {
                        true => self.add_construction_arc(centre, start, end),
                        false => self.add_arc(centre, start, end),
                    });
                }
            }
        }
        made
    }
}

/// Whether a transform turns the plane over, read off what it does to the two
/// directions the plane is measured in.
fn turns_over(by: &impl Fn(DVec2) -> DVec2) -> bool {
    let origin = by(DVec2::ZERO);
    (by(DVec2::X) - origin).perp_dot(by(DVec2::Y) - origin) < 0.0
}

#[cfg(test)]
mod tests;
