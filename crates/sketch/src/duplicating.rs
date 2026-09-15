//! Laying a second copy of part of the drawing down, wherever a transform
//! puts it. What the mirror and the two patterns each ask for, once.

use glam::DVec2;

use crate::arc::ArcId;
use crate::element::Element;
use crate::sketch::{CircleId, PointId, SegmentId, Sketch};

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

/// Two places nearer than this are the one place, and a copy laid at the second
/// is a copy laid on the first.
const SAME_PLACE: f64 = 1e-9;

impl Sketch {
    /// Lays down a second copy of the elements given, every place they stand on
    /// carried through `by`.
    ///
    /// The copy keeps the original's own joins: two sides that shared a corner
    /// share their copy's corner too, rather than each getting one of its own.
    ///
    /// What would land on its original is left out: the drawing gains nothing
    /// from a second thing nobody can tell from the first.
    pub fn duplicate(&mut self, of: &[Element], by: impl Fn(DVec2) -> DVec2) -> Duplicated {
        let laid: Vec<Element> = of
            .iter()
            .copied()
            .filter(|held| !self.lands_on_its_original(*held, &by))
            .collect();

        let mut made = Duplicated::default();
        let mut copied: Vec<(PointId, PointId)> = Vec::new();
        for held in &laid {
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

        for held in &laid {
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

    /// Whether carrying an element through `by` leaves it standing on the very
    /// places it already stands on.
    ///
    /// A trait lying along a mirror axis, a circle centred on one, a point
    /// turned about itself: the copy covers the original exactly, and two
    /// things nobody can tell apart is what the drawing is left with.
    fn lands_on_its_original(&self, held: Element, by: &impl Fn(DVec2) -> DVec2) -> bool {
        let places: Vec<DVec2> = self
            .points_it_leans_on(held)
            .into_iter()
            .map(|point| self.point(point))
            .collect();
        !places.is_empty()
            && places.iter().all(|place| {
                places
                    .iter()
                    .any(|other| by(*place).distance(*other) <= SAME_PLACE)
            })
    }
}

/// Whether a transform turns the plane over, read off what it does to the two
/// directions the plane is measured in.
fn turns_over(by: &impl Fn(DVec2) -> DVec2) -> bool {
    let origin = by(DVec2::ZERO);
    (by(DVec2::X) - origin).perp_dot(by(DVec2::Y) - origin) < 0.0
}
