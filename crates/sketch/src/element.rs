//! What a drawing is made of, and the one place that says how many kinds there
//! are.

use serde::{Deserialize, Serialize};

use crate::arc::ArcId;
use crate::ellipse::EllipseId;
use crate::sketch::{CircleId, PointId, SegmentId, Sketch};

/// One thing a sketch is made of, for deleting it.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Element {
    Point(PointId),
    Segment(SegmentId),
    Circle(CircleId),
    Arc(ArcId),
    Ellipse(EllipseId),
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
            Element::Ellipse(id) => match self.ellipses().get(id.0) {
                Some(_) => self.ellipse_stands_on(id),
                None => Vec::new(),
            },
        }
    }
}

#[cfg(test)]
pub(crate) mod kinds {
    use glam::DVec2;

    use super::Element;
    use crate::sketch::Sketch;

    /// One of every kind of element, drawn well inside a square reaching from
    /// the origin to (60, 60), each far enough from the others to be picked on
    /// its own.
    ///
    /// The match below has no wildcard arm, so a fifth kind of element stops
    /// the build here rather than slipping past every sweep that walks this
    /// list. That is the whole of the idea: the enumeration is the compiler's,
    /// not a list somebody remembered to keep up.
    pub(crate) fn one_of_every_kind(sketch: &mut Sketch) -> Vec<Element> {
        let free = sketch.add_point(DVec2::new(10.0, 10.0));

        let start = sketch.add_point(DVec2::new(20.0, 10.0));
        let end = sketch.add_point(DVec2::new(30.0, 10.0));
        let side = sketch.add_segment(start, end);

        let middle = sketch.add_point(DVec2::new(20.0, 25.0));
        let round = sketch.add_circle(middle, 4.0);

        let centre = sketch.add_point(DVec2::new(40.0, 25.0));
        let east = sketch.add_point(DVec2::new(45.0, 25.0));
        let north = sketch.add_point(DVec2::new(40.0, 30.0));
        let bend = sketch.add_arc(centre, east, north);

        let middle_of_oval = sketch.add_point(DVec2::new(30.0, 45.0));
        let west_end = sketch.add_point(DVec2::new(22.0, 45.0));
        let east_end = sketch.add_point(DVec2::new(38.0, 45.0));
        let south_end = sketch.add_point(DVec2::new(30.0, 41.0));
        let north_end = sketch.add_point(DVec2::new(30.0, 49.0));
        let oval = sketch.add_ellipse(middle_of_oval, [west_end, east_end], [south_end, north_end]);

        let drawn = vec![
            Element::Point(free),
            Element::Segment(side),
            Element::Circle(round),
            Element::Arc(bend),
            Element::Ellipse(oval),
        ];

        let mut kinds: Vec<&str> = drawn.iter().map(name_of).collect();
        kinds.sort_unstable();
        kinds.dedup();
        assert_eq!(
            kinds.len(),
            drawn.len(),
            "one of every kind means one of each: {kinds:?} for {} drawn",
            drawn.len(),
        );

        drawn
    }

    pub(crate) fn name_of(element: &Element) -> &'static str {
        match element {
            Element::Point(_) => "point",
            Element::Segment(_) => "segment",
            Element::Circle(_) => "circle",
            Element::Arc(_) => "arc",
            Element::Ellipse(_) => "ellipse",
        }
    }

    /// A place on the element itself, away from the points it leans on, so
    /// that a click there finds the element rather than one of its ends.
    ///
    /// The bisector is what puts the arc's place on its curve: it is exact for
    /// the quarter turn drawn above and honest for anything under half a turn.
    pub(crate) fn somewhere_on(sketch: &Sketch, element: Element) -> DVec2 {
        match element {
            Element::Point(id) => sketch.point(id),
            Element::Segment(id) => {
                let side = sketch.segments()[id.0];
                (sketch.point(side.start) + sketch.point(side.end)) / 2.0
            }
            Element::Circle(id) => {
                let round = sketch.circles()[id.0];
                sketch.point(round.center) + DVec2::new(round.radius, 0.0)
            }
            Element::Arc(id) => {
                let bend = sketch.arcs()[id.0];
                let centre = sketch.point(bend.center);
                let out = ((sketch.point(bend.start) - centre).normalize()
                    + (sketch.point(bend.end) - centre).normalize())
                .normalize();
                centre + out * sketch.arc_radius(id)
            }
            Element::Ellipse(id) => {
                let drawn = sketch.ellipse_draft(id);
                drawn.at(std::f64::consts::FRAC_PI_4)
            }
        }
    }

    /// Whether the drawing still holds the element, kind by kind. A sweep that
    /// takes something away is read back through this rather than through
    /// another sweep, so that two of them cannot agree on being wrong.
    pub(crate) fn still_drawn(sketch: &Sketch, element: Element) -> bool {
        match element {
            Element::Point(id) => sketch.live_points().any(|(live, _)| live == id),
            Element::Segment(id) => sketch.live_segments().any(|(live, _)| live == id),
            Element::Circle(id) => sketch.live_circles().any(|(live, _)| live == id),
            Element::Arc(id) => sketch.live_arcs().any(|(live, _)| live == id),
            Element::Ellipse(id) => sketch.live_ellipses().any(|(live, _)| live == id),
        }
    }
}

#[cfg(test)]
mod tests;
