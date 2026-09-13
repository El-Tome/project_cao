use serde::{Deserialize, Serialize};

use crate::arc::ArcId;
use crate::sketch::{CircleId, PointId, SegmentId, Sketch};

/// One thing a sketch is made of, for deleting it.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Element {
    Point(PointId),
    Segment(SegmentId),
    Circle(CircleId),
    Arc(ArcId),
}

impl Sketch {
    /// The points one piece of the drawing stands on.
    ///
    /// A trait stands on both its ends, a circle on its centre alone — its size
    /// is not a point and stays free — and an arc on all three of its own, which
    /// is what leaves an arc held still with nothing left to give.
    pub(crate) fn points_it_leans_on(&self, element: Element) -> Vec<PointId> {
        match element {
            Element::Point(id) => vec![id],
            Element::Segment(id) => match self.segments().get(id.0) {
                Some(segment) => vec![segment.start, segment.end],
                None => Vec::new(),
            },
            Element::Circle(id) => match self.circles().get(id.0) {
                Some(circle) => vec![circle.center],
                None => Vec::new(),
            },
            Element::Arc(id) => match self.arcs().get(id.0) {
                Some(arc) => vec![arc.center, arc.start, arc.end],
                None => Vec::new(),
            },
        }
    }
}
