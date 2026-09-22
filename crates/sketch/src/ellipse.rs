//! An ellipse, as a centre and the two axes it is laid with.

use glam::DVec2;
use serde::{Deserialize, Serialize};

use crate::ellipsing::EllipseDraft;

/// How far round the turn two readings of one place may stand apart and still
/// be that place: what the drawing already calls one place, read as a turn.
const ON_THE_CURVE: f64 = 1e-9;
use crate::erased::Erased;
use crate::sketch::{Element, PointId, SegmentId, Sketch};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct EllipseId(pub usize);

/// An ellipse, held by its centre and its two axes.
///
/// The axes are ordinary traits of the drawing, laid as construction, each
/// running across the whole curve through the centre — as on a drawing. They
/// are what the ellipse is dimensioned by: a length on an axis is its width
/// that way, as a diameter is a circle's. Their four ends are the handles it is
/// turned and stretched by.
///
/// Nothing else is kept. The size, the direction and the way round are all
/// read off the axes, so none of them can disagree with what is drawn.
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct Ellipse {
    pub center: PointId,
    pub first: SegmentId,
    pub second: SegmentId,
    #[serde(default)]
    pub construction: bool,
    /// What is left of the curve after a cut: the stretch running the way the
    /// curve runs, from the first point round to the second. `None` while the
    /// whole of it is drawn.
    ///
    /// An arc of ellipse is the ellipse it was cut from with a stretch taken
    /// away, and is kept as one: it holds its axes, its rules and its values,
    /// and every tool that reads an ellipse reads it without being told.
    #[serde(default)]
    pub drawn: Option<(PointId, PointId)>,
}

impl Sketch {
    /// Lays an ellipse from its centre and the four ends of its axes, the two
    /// axes drawn as construction traits.
    pub fn add_ellipse(
        &mut self,
        center: PointId,
        first: [PointId; 2],
        second: [PointId; 2],
    ) -> EllipseId {
        self.push_ellipse(center, first, second, false)
    }

    /// Helps place the rest of the drawing without becoming part of it.
    pub fn add_construction_ellipse(
        &mut self,
        center: PointId,
        first: [PointId; 2],
        second: [PointId; 2],
    ) -> EllipseId {
        self.push_ellipse(center, first, second, true)
    }

    fn push_ellipse(
        &mut self,
        center: PointId,
        [first_start, first_end]: [PointId; 2],
        [second_start, second_end]: [PointId; 2],
        construction: bool,
    ) -> EllipseId {
        let first = self.add_construction_segment(first_start, first_end);
        let second = self.add_construction_segment(second_start, second_end);
        self.ellipses.push(Ellipse {
            center,
            first,
            second,
            construction,
            drawn: None,
        });
        EllipseId(self.ellipses.len() - 1)
    }

    pub fn ellipses(&self) -> &[Ellipse] {
        &self.ellipses
    }

    pub fn is_erased_ellipse(&self, ellipse: EllipseId) -> bool {
        Erased::holds(&self.erased.ellipses, ellipse.0)
    }

    pub fn live_ellipses(&self) -> impl Iterator<Item = (EllipseId, Ellipse)> + '_ {
        self.ellipses
            .iter()
            .enumerate()
            .map(|(rank, ellipse)| (EllipseId(rank), *ellipse))
            .filter(|(id, _)| !self.is_erased_ellipse(*id))
    }

    /// The curve as its axes draw it now.
    pub fn ellipse_draft(&self, id: EllipseId) -> EllipseDraft {
        let ellipse = self.ellipses[id.0];
        let (start, end) = self.endpoints(ellipse.first);
        let (across_start, across_end) = self.endpoints(ellipse.second);
        EllipseDraft {
            centre: self.point(ellipse.center),
            first: (end - start) * 0.5,
            second: across_start.distance(across_end) * 0.5,
        }
    }

    /// Leaves an ellipse drawn over the stretch running from one point round
    /// to the other — what a history replaying an arc of ellipse lays.
    pub fn draw_the_stretch(&mut self, id: EllipseId, from: PointId, to: PointId) {
        if let Some(ellipse) = self.ellipses.get_mut(id.0) {
            ellipse.drawn = Some((from, to));
        }
    }

    /// The two ends of the stretch left of an ellipse, when a cut left one.
    pub fn ellipse_ends(&self, id: EllipseId) -> Option<(PointId, PointId)> {
        self.ellipses.get(id.0)?.drawn
    }

    /// Where the drawn stretch of an ellipse starts and how far round it runs,
    /// in turns — the whole of it for an ellipse nothing has cut.
    pub fn ellipse_run(&self, id: EllipseId) -> (f64, f64) {
        let drawn = self.ellipse_draft(id);
        let Some((from, to)) = self.ellipse_ends(id) else {
            return (0.0, std::f64::consts::TAU);
        };
        let starts = drawn.turn_on(self.point(from));
        let sweep = (drawn.turn_on(self.point(to)) - starts).rem_euclid(std::f64::consts::TAU);
        (starts, sweep)
    }

    /// The stretch of the curve that is drawn, as a run of places.
    pub fn ellipse_polyline(&self, id: EllipseId) -> Vec<DVec2> {
        let drawn = self.ellipse_draft(id);
        match self.ellipse_ends(id) {
            None => drawn.places(),
            Some(_) => {
                let (from, sweep) = self.ellipse_run(id);
                let steps = drawn.steps_over(sweep);
                drawn.places_along(from, sweep, steps)
            }
        }
    }

    /// Whether the stretch running from one point round to the other lies on
    /// what an ellipse has drawn — which is what says the cut of that stretch
    /// is this piece's to give up.
    pub fn ellipse_holds_the_stretch(&self, id: EllipseId, from: PointId, to: PointId) -> bool {
        // A curve nothing has cut yet has no ends to run past: either way round
        // between two points on it is a stretch it can give up.
        if self.ellipse_ends(id).is_none() {
            return true;
        }
        let drawn = self.ellipse_draft(id);
        let (opens, sweep) = self.ellipse_run(id);
        let along = |point: PointId| {
            self.points()
                .get(point.0)
                .map(|place| (drawn.turn_on(*place) - opens).rem_euclid(std::f64::consts::TAU))
        };
        let (Some(near), Some(far)) = (along(from), along(to)) else {
            return false;
        };
        near <= far && far <= sweep + ON_THE_CURVE
    }

    /// Whether a turn, as a fraction of a whole one, falls on the stretch of
    /// an ellipse that is drawn.
    pub(crate) fn ellipse_holds_the_turn(&self, id: EllipseId, fraction: f64) -> bool {
        if self.ellipse_ends(id).is_none() {
            return true;
        }
        let turn = std::f64::consts::TAU;
        let (from, sweep) = self.ellipse_run(id);
        let along = (fraction - from / turn + ON_THE_CURVE).rem_euclid(1.0) - ON_THE_CURVE;
        (-ON_THE_CURVE..=sweep / turn + ON_THE_CURVE).contains(&along)
    }

    /// The smallest box the stretch that is drawn fits in — the whole curve's
    /// own box while nothing has cut it.
    pub fn ellipse_bounds(&self, id: EllipseId) -> (DVec2, DVec2) {
        let drawn = self.ellipse_draft(id);
        if self.ellipse_ends(id).is_none() {
            return drawn.bounds();
        }
        let places = self.ellipse_polyline(id);
        places
            .iter()
            .fold((places[0], places[0]), |(low, high), place| {
                (low.min(*place), high.max(*place))
            })
    }

    /// Where the drawn stretch of an ellipse takes a place near it.
    ///
    /// Past either end it is that end, not the far side of the curve: the rest
    /// of the ellipse is not drawn, and a click out there must find nothing.
    pub fn place_on_ellipse(&self, id: EllipseId, place: DVec2) -> DVec2 {
        let drawn = self.ellipse_draft(id);
        let (from, sweep) = self.ellipse_run(id);
        let turn = drawn.turn_nearest(place);
        let along = (turn - from).rem_euclid(std::f64::consts::TAU);
        if along <= sweep {
            return drawn.at(turn);
        }
        let (start, end) = (drawn.at(from), drawn.at(from + sweep));
        match place.distance(start) <= place.distance(end) {
            true => start,
            false => end,
        }
    }

    /// How far a place stands from the stretch that is drawn.
    pub fn distance_to_ellipse(&self, id: EllipseId, place: DVec2) -> f64 {
        self.place_on_ellipse(id, place).distance(place)
    }

    /// The ellipse whose drawn curve passes closest to `position`.
    pub fn nearest_ellipse(&self, position: DVec2, tolerance: f64) -> Option<EllipseId> {
        self.live_ellipses()
            .map(|(id, _)| (id, self.distance_to_ellipse(id, position)))
            .filter(|(_, distance)| *distance <= tolerance)
            .min_by(|a, b| a.1.total_cmp(&b.1))
            .map(|(id, _)| id)
    }

    /// The ellipse a trait is an axis of, if it is one.
    pub fn ellipse_of_axis(&self, segment: SegmentId) -> Option<EllipseId> {
        self.live_ellipses()
            .find(|(_, ellipse)| ellipse.first == segment || ellipse.second == segment)
            .map(|(id, _)| id)
    }

    /// Every point an ellipse stands on: the five its axes give it, and the
    /// two ends of the stretch drawn when a cut left one.
    pub fn ellipse_stands_on(&self, id: EllipseId) -> Vec<PointId> {
        let mut points = self.ellipse_points(id).to_vec();
        if let Some((from, to)) = self.ellipse_ends(id) {
            points.extend([from, to]);
        }
        points
    }

    /// The five points the axes of an ellipse give it: its centre, then the
    /// ends of its first axis and of its second.
    pub fn ellipse_points(&self, id: EllipseId) -> [PointId; 5] {
        let ellipse = self.ellipses[id.0];
        let (first, second) = (
            self.segments()[ellipse.first.0],
            self.segments()[ellipse.second.0],
        );
        [
            ellipse.center,
            first.start,
            first.end,
            second.start,
            second.end,
        ]
    }

    /// The ends of the axes of every ellipse centred on `point`, which move
    /// with it: dragging the centre alone would leave them behind and pull the
    /// curve out of shape.
    pub fn ellipse_ends_around(&self, point: PointId) -> Vec<PointId> {
        self.live_ellipses()
            .filter(|(_, ellipse)| ellipse.center == point)
            .flat_map(|(id, _)| self.ellipse_points(id)[1..].to_vec())
            .collect()
    }

    /// The centre an end of an ellipse's axis turns about, when it is one.
    pub(crate) fn centre_turned_about(&self, point: PointId) -> Option<PointId> {
        self.live_ellipses()
            .find(|(id, _)| self.ellipse_points(*id)[1..].contains(&point))
            .map(|(_, ellipse)| ellipse.center)
    }

    /// Every ellipse standing on `point`, to be taken with it.
    pub(crate) fn ellipses_leaning_on(&self, point: PointId) -> Vec<Element> {
        self.live_ellipses()
            .filter(|(id, _)| self.ellipse_stands_on(*id).contains(&point))
            .map(|(id, _)| Element::Ellipse(id))
            .collect()
    }

    /// Takes an ellipse away with its two axes, and the points it stood on that
    /// nothing else stands on: an axis cannot outlive the curve it measures,
    /// and the curve cannot stand without its axes.
    pub(crate) fn erase_ellipse(&mut self, id: EllipseId) {
        if self.is_erased_ellipse(id) {
            return;
        }
        Erased::mark(&mut self.erased.ellipses, id.0);
        let ellipse = self.ellipses[id.0];
        let points = self.ellipse_stands_on(id);
        // A cut in the middle of a curve leaves two pieces on one pair of
        // axes: the axes go with the last of them, not with the first.
        if !self.shares_its_axes(id) {
            self.erase(Element::Segment(ellipse.first));
            self.erase(Element::Segment(ellipse.second));
        }
        for point in points {
            if !self.is_erased_point(point) && !self.anything_stands_on(point) {
                self.erase(Element::Point(point));
            }
        }
    }

    /// Takes away every ellipse a trait was an axis of — a cut in the middle
    /// of a curve leaves two pieces on one pair of axes, and neither can stand
    /// without them.
    pub(crate) fn erase_ellipse_of(&mut self, segment: SegmentId) {
        while let Some(ellipse) = self.ellipse_of_axis(segment) {
            self.erase_ellipse(ellipse);
        }
    }

    /// Whether a point is one of an ellipse's own handles and nothing else
    /// leans on it — nothing, that is, but the two axes the ellipse is laid
    /// with.
    pub(crate) fn is_a_bare_handle_of(&self, ellipse: EllipseId, point: PointId) -> bool {
        if !self.ellipse_points(ellipse).contains(&point) {
            return false;
        }
        let axes = self.ellipses()[ellipse.0];
        !self.live_segments().any(|(id, segment)| {
            id != axes.first
                && id != axes.second
                && (segment.start == point || segment.end == point)
        }) && !self
            .live_circles()
            .any(|(_, circle)| circle.center == point)
            && self.arcs_leaning_on(point).is_empty()
            && self
                .ellipses_leaning_on(point)
                .iter()
                .all(|held| *held == Element::Ellipse(ellipse))
    }

    /// Whether any curve still drawn stands on a point.
    fn anything_stands_on(&self, point: PointId) -> bool {
        self.live_segments()
            .any(|(_, segment)| segment.start == point || segment.end == point)
            || self
                .live_circles()
                .any(|(_, circle)| circle.center == point)
            || !self.arcs_leaning_on(point).is_empty()
            || !self.ellipses_leaning_on(point).is_empty()
    }
}

#[cfg(test)]
mod tests;
