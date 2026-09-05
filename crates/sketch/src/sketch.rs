use glam::Vec2;
use serde::{Deserialize, Serialize};

use crate::constraints::{Dimension, DimensionTarget, Freedom, SketchAxis};
use crate::plane::WorkPlane;
use crate::solver::{self, SolveOutcome};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct PointId(pub usize);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct SegmentId(pub usize);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct CircleId(pub usize);

/// A circle, kept as a centre point shared with the rest of the drawing plus a
/// radius, so that moving the centre moves the circle with it.
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct Circle {
    pub center: PointId,
    pub radius: f32,
}

/// A straight line between two points. Points are shared: chaining a polyline
/// reuses the previous end, which is what makes a dimension able to drag the
/// rest of the chain along.
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct Segment {
    pub start: PointId,
    pub end: PointId,
}

/// What happened when a length was applied.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LengthOutcome {
    /// The segment now has exactly the requested length, and whatever hung off
    /// its far end moved rigidly with it.
    Exact,
    /// The far end could not move freely because the geometry loops back to the
    /// fixed end. Only the far end moved, so the shapes around it are distorted.
    BestEffort,
    /// The segment has no direction to stretch along.
    Degenerate,
}

/// A 2D sketch on a plane. Everything is stored in the plane's own coordinates
/// and in world units; millimetres only appear on dimensions, because the
/// document scale can be redefined by the first one.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Sketch {
    pub plane: WorkPlane,
    points: Vec<Vec2>,
    segments: Vec<Segment>,
    #[serde(default)]
    circles: Vec<Circle>,
    dimensions: Vec<Dimension>,
}

impl Sketch {
    /// Every sketch owns a point at its origin from the moment it is created.
    ///
    /// It is not drawn like the others and cannot be moved: it is there to be
    /// snapped onto and measured from, so that pinning a drawing never means
    /// first remembering to place a point by hand.
    pub const ORIGIN: PointId = PointId(0);

    pub fn new(plane: WorkPlane) -> Self {
        Self {
            plane,
            points: vec![Vec2::ZERO],
            segments: Vec::new(),
            circles: Vec::new(),
            dimensions: Vec::new(),
        }
    }

    pub fn is_origin(&self, point: PointId) -> bool {
        point == Self::ORIGIN
    }

    /// Points the user drew, as opposed to the origin the sketch was born with.
    pub fn drawn_points(&self) -> impl Iterator<Item = (PointId, Vec2)> + '_ {
        self.points
            .iter()
            .enumerate()
            .skip(1)
            .map(|(index, point)| (PointId(index), *point))
    }

    pub fn points(&self) -> &[Vec2] {
        &self.points
    }

    pub fn segments(&self) -> &[Segment] {
        &self.segments
    }

    pub fn circles(&self) -> &[Circle] {
        &self.circles
    }

    pub fn dimensions(&self) -> &[Dimension] {
        &self.dimensions
    }

    pub fn circle(&self, id: CircleId) -> Circle {
        self.circles[id.0]
    }

    pub fn add_circle(&mut self, center: PointId, radius: f32) -> CircleId {
        self.circles.push(Circle { center, radius });
        CircleId(self.circles.len() - 1)
    }

    /// The circle whose outline passes closest to `position`.
    pub fn nearest_circle(&self, position: Vec2, tolerance: f32) -> Option<CircleId> {
        self.circles
            .iter()
            .enumerate()
            .map(|(index, circle)| {
                let distance = (self.point(circle.center).distance(position) - circle.radius).abs();
                (CircleId(index), distance)
            })
            .filter(|(_, distance)| *distance <= tolerance)
            .min_by(|a, b| a.1.total_cmp(&b.1))
            .map(|(id, _)| id)
    }

    pub fn is_empty(&self) -> bool {
        self.segments.is_empty()
    }

    pub fn point(&self, id: PointId) -> Vec2 {
        self.points[id.0]
    }

    pub fn endpoints(&self, id: SegmentId) -> (Vec2, Vec2) {
        let segment = self.segments[id.0];
        (self.point(segment.start), self.point(segment.end))
    }

    pub fn add_point(&mut self, position: Vec2) -> PointId {
        self.points.push(position);
        PointId(self.points.len() - 1)
    }

    /// Moves an annotation away from where it would sit on its own.
    pub fn offset_dimension(&mut self, target: DimensionTarget, offset: Vec2) {
        if let Some(dimension) = self
            .dimensions
            .iter_mut()
            .find(|dimension| dimension.target == target)
        {
            dimension.offset = offset;
        }
    }

    /// The dimension whose annotation sits nearest `position`, within
    /// `tolerance`. Needs where each one is drawn, which only the front-end
    /// knows.
    pub fn nearest_dimension(
        &self,
        anchors: &[(DimensionTarget, Vec2)],
        position: Vec2,
        tolerance: f32,
    ) -> Option<DimensionTarget> {
        anchors
            .iter()
            .map(|(target, at)| (*target, at.distance(position)))
            .filter(|(_, distance)| *distance <= tolerance)
            .min_by(|a, b| a.1.total_cmp(&b.1))
            .map(|(target, _)| target)
    }

    /// Moves a point where the user dragged it. The origin stays put.
    pub fn move_point(&mut self, point: PointId, position: Vec2) {
        if self.is_origin(point) {
            return;
        }
        if let Some(existing) = self.points.get_mut(point.0) {
            *existing = position;
        }
    }

    /// Reuses an existing point when one is within `tolerance`, so that clicking
    /// back onto a corner joins the geometry there instead of laying a second
    /// point on top of it.
    pub fn point_at(&mut self, position: Vec2, tolerance: f32) -> PointId {
        match self.nearest_point(position, tolerance) {
            Some(id) => id,
            None => self.add_point(position),
        }
    }

    pub fn nearest_point(&self, position: Vec2, tolerance: f32) -> Option<PointId> {
        self.points
            .iter()
            .enumerate()
            .map(|(index, point)| (PointId(index), point.distance(position)))
            .filter(|(_, distance)| *distance <= tolerance)
            .min_by(|a, b| a.1.total_cmp(&b.1))
            .map(|(id, _)| id)
    }

    /// The segment whose body passes closest to `position`, within `tolerance`.
    pub fn nearest_segment(&self, position: Vec2, tolerance: f32) -> Option<SegmentId> {
        (0..self.segments.len())
            .map(|index| {
                let id = SegmentId(index);
                (id, self.distance_to_segment(id, position))
            })
            .filter(|(_, distance)| *distance <= tolerance)
            .min_by(|a, b| a.1.total_cmp(&b.1))
            .map(|(id, _)| id)
    }

    fn distance_to_segment(&self, id: SegmentId, position: Vec2) -> f32 {
        let (start, end) = self.endpoints(id);
        let span = end - start;
        let length_squared = span.length_squared();
        if length_squared < 1e-12 {
            return start.distance(position);
        }
        let t = ((position - start).dot(span) / length_squared).clamp(0.0, 1.0);
        (start + span * t).distance(position)
    }

    pub fn add_segment(&mut self, start: PointId, end: PointId) -> SegmentId {
        self.segments.push(Segment { start, end });
        SegmentId(self.segments.len() - 1)
    }

    pub fn segment_length(&self, id: SegmentId) -> f32 {
        let (start, end) = self.endpoints(id);
        start.distance(end)
    }

    pub fn dimension_of(&self, target: DimensionTarget) -> Option<&Dimension> {
        self.dimensions
            .iter()
            .find(|dimension| dimension.target == target)
    }

    /// Records a value the user typed, replacing any previous one on the same
    /// target. Moving the geometry is a separate step: the very first dimension
    /// of a document sets its scale instead of resizing anything, and a driven
    /// one never moves anything at all.
    pub fn set_dimension(&mut self, target: DimensionTarget, value: f32, driven: bool) {
        match self
            .dimensions
            .iter_mut()
            .find(|dimension| dimension.target == target)
        {
            Some(existing) => {
                existing.value = value;
                existing.driven = driven;
            }
            None => self.dimensions.push(Dimension {
                target,
                value,
                driven,
                offset: Vec2::ZERO,
            }),
        }
    }

    /// The angle at the point two segments share, in degrees, or `None` when
    /// they do not meet.
    pub fn angle_between(&self, first: SegmentId, second: SegmentId) -> Option<f32> {
        let (pivot, a, b) = self.corner(first, second)?;
        let first = (self.point(a) - self.point(pivot)).normalize_or_zero();
        let second = (self.point(b) - self.point(pivot)).normalize_or_zero();
        if first == Vec2::ZERO || second == Vec2::ZERO {
            return None;
        }
        Some(first.dot(second).clamp(-1.0, 1.0).acos().to_degrees())
    }

    /// The shared point of two segments, with their far ends.
    fn corner(&self, first: SegmentId, second: SegmentId) -> Option<(PointId, PointId, PointId)> {
        let (a, b) = (self.segments[first.0], self.segments[second.0]);
        for (pivot, far_a) in [(a.start, a.end), (a.end, a.start)] {
            if b.start == pivot {
                return Some((pivot, far_a, b.end));
            }
            if b.end == pivot {
                return Some((pivot, far_a, b.start));
            }
        }
        None
    }

    /// The corner two segments share, as positions: the pivot and the two far
    /// ends. What an annotation needs to draw the angle.
    pub fn corner_points(&self, first: SegmentId, second: SegmentId) -> Option<(Vec2, Vec2, Vec2)> {
        let (pivot, a, b) = self.corner(first, second)?;
        Some((self.point(pivot), self.point(a), self.point(b)))
    }

    /// The shared point of two segments, with their far ends.
    pub(crate) fn shared_corner(
        &self,
        first: SegmentId,
        second: SegmentId,
    ) -> Option<(PointId, PointId, PointId)> {
        self.corner(first, second)
    }

    pub(crate) fn translate_point(&mut self, point: PointId, delta: Vec2) {
        self.points[point.0] += delta;
    }

    /// The angle a segment makes with one of the sketch axes, in degrees.
    pub fn angle_with_axis(&self, segment: SegmentId, axis: SketchAxis) -> Option<f32> {
        let (start, end) = self.endpoints(segment);
        let direction = (end - start).normalize_or_zero();
        if direction == Vec2::ZERO {
            return None;
        }
        Some(
            direction
                .dot(axis.direction())
                .clamp(-1.0, 1.0)
                .acos()
                .to_degrees(),
        )
    }

    pub fn set_circle_radius(&mut self, id: CircleId, radius: f32) -> LengthOutcome {
        if radius <= 0.0 {
            return LengthOutcome::Degenerate;
        }
        self.circles[id.0].radius = radius;
        LengthOutcome::Exact
    }

    /// Smallest axis-aligned box containing every point, in sketch coordinates.
    pub fn bounds(&self) -> Option<(Vec2, Vec2)> {
        let first = *self.points.first()?;
        Some(
            self.points
                .iter()
                .fold((first, first), |(min, max), point| {
                    (min.min(*point), max.max(*point))
                }),
        )
    }
}

impl Sketch {
    /// Coordinates that no constraint holds, worked out from the rank of the
    /// system: how many independent things the dimensions actually say.
    ///
    /// Points pinned to the origin are taken out of the count outright, since
    /// neither of their coordinates can move.
    pub fn freedom(&self, millimeters_per_unit: f32) -> Freedom {
        // The origin never moves, so its two coordinates are not in play.
        let free_coordinates = self.points.len().saturating_sub(1) * 2;
        let held = solver::rank(&self.analysed_system(millimeters_per_unit)).min(free_coordinates);

        // A circle brings its own radius, which only its own dimension can
        // settle; that pair never touches the point coordinates.
        let radii_without_a_value = self.circles.len().saturating_sub(
            self.dimensions
                .iter()
                .filter(|dimension| {
                    !dimension.driven && matches!(dimension.target, DimensionTarget::Radius(_))
                })
                .count(),
        );

        Freedom {
            degrees_of_freedom: (free_coordinates - held) + radii_without_a_value,
        }
    }

    pub fn is_fully_constrained(&self, millimeters_per_unit: f32) -> bool {
        self.points.len() > 1 && self.freedom(millimeters_per_unit).fully_constrained()
    }

    /// Whether a value on this target would say anything new.
    ///
    /// A constraint is redundant when its equation is a combination of those
    /// already there — exactly the case of a triangle's third side once its
    /// other sides and angles are fixed. Counting constraints could never see
    /// that; comparing their directions can.
    pub fn would_be_redundant(&self, target: DimensionTarget, millimeters_per_unit: f32) -> bool {
        if self.dimension_of(target).is_some() {
            return false;
        }
        if let DimensionTarget::Radius(circle) = target {
            // A radius stands alone: redundant only if that circle already has
            // one driving it.
            return self.dimensions.iter().any(|dimension| {
                !dimension.driven && dimension.target == DimensionTarget::Radius(circle)
            });
        }

        let existing = self.equations(millimeters_per_unit);
        let Some(candidate) = self.candidate_equation(target, millimeters_per_unit) else {
            return false;
        };
        solver::is_dependent(&existing, &candidate)
    }

    /// The equation a not-yet-placed dimension would contribute, taken at the
    /// value the geometry already has so only its direction matters.
    fn candidate_equation(
        &self,
        target: DimensionTarget,
        millimeters_per_unit: f32,
    ) -> Option<solver::Equation> {
        let value = match target {
            DimensionTarget::Length(segment) => {
                self.segment_length(segment) * millimeters_per_unit.max(1e-9)
            }
            DimensionTarget::Distance { from, to } => {
                self.point(from).distance(self.point(to)) * millimeters_per_unit.max(1e-9)
            }
            DimensionTarget::Angle { first, second } => self.angle_between(first, second)?,
            DimensionTarget::AxisAngle { segment, axis } => self.angle_with_axis(segment, axis)?,
            DimensionTarget::Radius(_) => return None,
        };

        let mut probe = self.clone();
        probe.dimensions.clear();
        probe.set_dimension(target, value, false);
        probe.equations(millimeters_per_unit).into_iter().next()
    }

    /// Whether the drawing has any freedom left, as a whole.
    pub fn is_settled(&self, millimeters_per_unit: f32) -> bool {
        self.is_fully_constrained(millimeters_per_unit)
    }

    /// Which points can no longer move at all.
    ///
    /// A drawing is rarely all-or-nothing: one contour can be nailed down while
    /// another is still floating beside it. Showing that per point, rather than
    /// one verdict for the whole sketch, says what is left to do.
    pub fn settled_points(&self, millimeters_per_unit: f32) -> Vec<bool> {
        let variables = self.points.len() * 2;
        let pinned: Vec<bool> = (0..self.points.len())
            .map(|index| self.is_origin(PointId(index)))
            .collect();
        let free =
            solver::null_space(&self.analysed_system(millimeters_per_unit), &pinned, variables);

        (0..self.points.len())
            .map(|index| {
                if pinned[index] {
                    return true;
                }
                // Settled means no way to move survives at this point.
                free.iter().all(|direction| {
                    direction[index * 2].abs() < 1e-3 && direction[index * 2 + 1].abs() < 1e-3
                })
            })
            .collect()
    }

    /// Re-satisfies every dimension at once, reporting whether it managed.
    pub fn resolve(&mut self, millimeters_per_unit: f32) -> LengthOutcome {
        match self.solve(millimeters_per_unit) {
            SolveOutcome::Solved | SolveOutcome::Nothing => LengthOutcome::Exact,
            SolveOutcome::Residual => LengthOutcome::BestEffort,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::constraints::SketchAxis;

    /// A right-angled triangle hung off the sketch origin.
    fn triangle() -> (Sketch, [SegmentId; 3]) {
        let mut sketch = Sketch::new(WorkPlane::XY);
        let corner = Sketch::ORIGIN;
        let right = sketch.add_point(Vec2::new(40.0, 0.0));
        let top = sketch.add_point(Vec2::new(0.0, 30.0));
        let base = sketch.add_segment(corner, right);
        let side = sketch.add_segment(corner, top);
        let hypotenuse = sketch.add_segment(right, top);
        (sketch, [base, side, hypotenuse])
    }

    /// The bug this solver exists for: setting a second value must not undo the
    /// first. Applying each dimension once and forgetting it could never do
    /// this.
    #[test]
    fn an_angle_still_holds_after_a_length_is_changed() {
        let (mut sketch, [base, side, _]) = triangle();

        sketch.set_dimension(
            DimensionTarget::Angle {
                first: base,
                second: side,
            },
            60.0,
            false,
        );
        sketch.resolve(1.0);
        assert!((sketch.angle_between(base, side).unwrap() - 60.0).abs() < 0.1);

        sketch.set_dimension(DimensionTarget::Length(base), 100.0, false);
        sketch.resolve(1.0);

        let angle = sketch.angle_between(base, side).unwrap();
        let length = sketch.segment_length(base);
        assert!((angle - 60.0).abs() < 0.1, "the angle drifted to {angle}°");
        assert!((length - 100.0).abs() < 0.1, "the length is {length}");
    }

    #[test]
    fn every_value_holds_at_once() {
        let (mut sketch, [base, side, _]) = triangle();
        sketch.set_dimension(DimensionTarget::Length(base), 50.0, false);
        sketch.set_dimension(DimensionTarget::Length(side), 20.0, false);
        sketch.set_dimension(
            DimensionTarget::Angle {
                first: base,
                second: side,
            },
            45.0,
            false,
        );

        assert_eq!(sketch.resolve(1.0), LengthOutcome::Exact);

        assert!((sketch.segment_length(base) - 50.0).abs() < 0.1);
        assert!((sketch.segment_length(side) - 20.0).abs() < 0.1);
        assert!((sketch.angle_between(base, side).unwrap() - 45.0).abs() < 0.1);
    }

    /// The case reported from the drawing: with two angles and two sides given,
    /// the third side follows and cannot be set independently.
    #[test]
    fn the_third_side_of_a_settled_triangle_is_redundant() {
        let (mut sketch, [base, side, hypotenuse]) = triangle();
        sketch.set_dimension(DimensionTarget::Length(base), 40.0, false);
        sketch.set_dimension(DimensionTarget::Length(side), 30.0, false);
        sketch.set_dimension(
            DimensionTarget::Angle {
                first: base,
                second: side,
            },
            90.0,
            false,
        );
        sketch.resolve(1.0);

        assert!(
            sketch.would_be_redundant(DimensionTarget::Length(hypotenuse), 1.0),
            "the hypotenuse follows from the two sides and their angle"
        );
    }

    /// Nothing is redundant while the shape can still change.
    #[test]
    fn a_side_of_an_open_shape_is_not_redundant() {
        let (sketch, [_, _, hypotenuse]) = triangle();
        assert!(!sketch.would_be_redundant(DimensionTarget::Length(hypotenuse), 1.0));
    }

    /// Pinning a point takes away the two ways a drawing can slide, never the
    /// way it can turn: that last freedom needs an angle to a fixed direction.
    #[test]
    fn a_drawing_needs_no_angle_to_the_axes_to_be_complete() {
        let (mut sketch, [base, side, _]) = triangle();
        sketch.set_dimension(DimensionTarget::Length(base), 40.0, false);
        sketch.set_dimension(DimensionTarget::Length(side), 30.0, false);
        sketch.set_dimension(
            DimensionTarget::Angle {
                first: base,
                second: side,
            },
            90.0,
            false,
        );
        sketch.resolve(1.0);

        // Which way up the drawing sits is implicit, like its origin point.
        assert_eq!(sketch.freedom(1.0).degrees_of_freedom, 0);
        assert!(sketch.is_fully_constrained(1.0));
    }

    /// Stating the orientation by hand must not take the same freedom twice,
    /// or a drawing free to slide would be reported as pinned.
    #[test]
    fn an_angle_to_an_axis_replaces_the_implicit_one() {
        let (mut sketch, [base, side, _]) = triangle();
        sketch.set_dimension(DimensionTarget::Length(base), 40.0, false);
        sketch.set_dimension(DimensionTarget::Length(side), 30.0, false);
        sketch.set_dimension(
            DimensionTarget::Angle {
                first: base,
                second: side,
            },
            90.0,
            false,
        );
        sketch.set_dimension(
            DimensionTarget::AxisAngle {
                segment: base,
                axis: SketchAxis::U,
            },
            0.0,
            false,
        );
        sketch.resolve(1.0);

        assert_eq!(sketch.freedom(1.0).degrees_of_freedom, 0);
    }

    /// One contour nailed down beside another still floating: the drawing has
    /// to say so per element, not give one verdict for the whole.
    #[test]
    fn part_of_a_drawing_can_be_settled_while_the_rest_floats() {
        let mut sketch = Sketch::new(WorkPlane::XY);

        let held = sketch.add_point(Vec2::new(20.0, 0.0));
        let fixed = sketch.add_segment(Sketch::ORIGIN, held);
        sketch.set_dimension(DimensionTarget::Length(fixed), 20.0, false);
        sketch.set_dimension(
            DimensionTarget::AxisAngle {
                segment: fixed,
                axis: SketchAxis::U,
            },
            0.0,
            false,
        );

        let loose_a = sketch.add_point(Vec2::new(50.0, 50.0));
        let loose_b = sketch.add_point(Vec2::new(70.0, 50.0));
        sketch.add_segment(loose_a, loose_b);

        let settled = sketch.settled_points(1.0);

        assert!(settled[Sketch::ORIGIN.0], "the origin never moves");
        assert!(settled[held.0], "held by a length and a direction");
        assert!(!settled[loose_a.0], "nothing holds this one");
        assert!(!settled[loose_b.0]);
        assert!(!sketch.is_fully_constrained(1.0));
    }

    /// A length from the origin is enough on its own: the drawing keeps the
    /// direction it was drawn in, so the far end has nowhere left to go.
    #[test]
    fn a_length_from_the_origin_settles_its_end() {
        let mut sketch = Sketch::new(WorkPlane::XY);
        let end = sketch.add_point(Vec2::new(20.0, 0.0));
        let segment = sketch.add_segment(Sketch::ORIGIN, end);
        sketch.set_dimension(DimensionTarget::Length(segment), 20.0, false);

        assert!(sketch.settled_points(1.0)[end.0]);
    }

    /// A shape drawn beside another must not stop it from being settled: each
    /// group of joined geometry keeps the direction it was drawn in on its own.
    #[test]
    fn a_loose_shape_beside_a_measured_one_leaves_it_settled() {
        let mut sketch = Sketch::new(WorkPlane::XY);
        let corner = sketch.add_point(Vec2::new(80.0, 0.0));
        let side = sketch.add_segment(Sketch::ORIGIN, corner);
        sketch.set_dimension(DimensionTarget::Length(side), 80.0, false);

        let loose_a = sketch.add_point(Vec2::new(200.0, 200.0));
        let loose_b = sketch.add_point(Vec2::new(260.0, 200.0));
        sketch.add_segment(loose_a, loose_b);

        let settled = sketch.settled_points(1.0);
        assert!(settled[corner.0], "held by a length and the way it was drawn");
        assert!(!settled[loose_a.0]);
    }

    /// A length that hangs off nothing fixed still swings freely.
    #[test]
    fn a_length_away_from_the_origin_leaves_its_end_free() {
        let mut sketch = Sketch::new(WorkPlane::XY);
        let anchor = sketch.add_point(Vec2::new(30.0, 30.0));
        let end = sketch.add_point(Vec2::new(50.0, 30.0));
        let segment = sketch.add_segment(anchor, end);
        sketch.set_dimension(DimensionTarget::Length(segment), 20.0, false);

        assert!(!sketch.settled_points(1.0)[end.0]);
    }

    /// A drawing that touches nothing fixed can still slide about, however many
    /// values it carries.
    #[test]
    fn a_drawing_that_is_not_pinned_is_never_complete() {
        let mut sketch = Sketch::new(WorkPlane::XY);
        let a = sketch.add_point(Vec2::new(5.0, 5.0));
        let b = sketch.add_point(Vec2::new(15.0, 5.0));
        let segment = sketch.add_segment(a, b);
        sketch.set_dimension(DimensionTarget::Length(segment), 10.0, false);
        sketch.set_dimension(
            DimensionTarget::AxisAngle {
                segment,
                axis: SketchAxis::U,
            },
            0.0,
            false,
        );

        assert!(!sketch.is_fully_constrained(1.0));
        assert_eq!(sketch.freedom(1.0).degrees_of_freedom, 2, "free to slide");
    }

    /// A readout says nothing about the shape, so it must not remove freedom.
    #[test]
    fn a_driven_dimension_constrains_nothing() {
        let (mut sketch, [base, _, _]) = triangle();
        let before = sketch.freedom(1.0).degrees_of_freedom;
        sketch.set_dimension(DimensionTarget::Length(base), 100.0, true);
        assert_eq!(sketch.freedom(1.0).degrees_of_freedom, before);
    }

    #[test]
    fn an_angle_needs_two_segments_that_meet() {
        let mut sketch = Sketch::new(WorkPlane::XY);
        let a = sketch.add_point(Vec2::ZERO);
        let b = sketch.add_point(Vec2::new(10.0, 0.0));
        let c = sketch.add_point(Vec2::new(0.0, 5.0));
        let d = sketch.add_point(Vec2::new(5.0, 5.0));
        let first = sketch.add_segment(a, b);
        let apart = sketch.add_segment(c, d);

        assert!(sketch.angle_between(first, apart).is_none());
    }

    #[test]
    fn an_axis_angle_measures_from_the_sketch_direction() {
        let mut sketch = Sketch::new(WorkPlane::XY);
        let a = sketch.add_point(Vec2::ZERO);
        let b = sketch.add_point(Vec2::new(10.0, 10.0));
        let segment = sketch.add_segment(a, b);

        let angle = sketch.angle_with_axis(segment, SketchAxis::U).unwrap();
        assert!((angle - 45.0).abs() < 1e-3, "got {angle}°");
    }

    /// Contradictory values cannot both be met, and the solver has to say so
    /// rather than quietly settling on one of them.
    #[test]
    fn impossible_values_are_reported() {
        let mut sketch = Sketch::new(WorkPlane::XY);
        let a = sketch.add_point(Vec2::ZERO);
        let b = sketch.add_point(Vec2::new(10.0, 0.0));
        let first = sketch.add_segment(a, b);
        let second = sketch.add_segment(a, b);

        sketch.set_dimension(DimensionTarget::Length(first), 50.0, false);
        sketch.set_dimension(DimensionTarget::Length(second), 90.0, false);

        assert_eq!(sketch.resolve(1.0), LengthOutcome::BestEffort);
    }

    #[test]
    fn clicking_back_onto_a_corner_reuses_it() {
        let mut sketch = Sketch::new(WorkPlane::XY);
        let first = sketch.point_at(Vec2::new(5.0, 5.0), 0.5);
        let again = sketch.point_at(Vec2::new(5.2, 5.1), 0.5);
        let elsewhere = sketch.point_at(Vec2::new(40.0, 5.0), 0.5);

        assert_eq!(first, again);
        assert_ne!(first, elsewhere);
        assert_eq!(sketch.points().len(), 3, "the origin plus the two placed");
    }

    /// Clicking where the origin sits must join it rather than lay a second
    /// point on top: that is how a drawing gets pinned without thinking about
    /// it.
    #[test]
    fn clicking_the_origin_joins_it() {
        let mut sketch = Sketch::new(WorkPlane::XY);
        assert_eq!(sketch.point_at(Vec2::new(0.05, -0.05), 0.5), Sketch::ORIGIN);
        assert_eq!(sketch.points().len(), 1);
    }

    #[test]
    fn the_origin_never_moves() {
        let mut sketch = Sketch::new(WorkPlane::XY);
        sketch.move_point(Sketch::ORIGIN, Vec2::new(10.0, 10.0));
        assert_eq!(sketch.point(Sketch::ORIGIN), Vec2::ZERO);
    }

    /// A distance can be measured between any two points, joined or not, which
    /// is what lets a shape be positioned from the origin.
    #[test]
    fn a_distance_pins_a_point_against_the_origin() {
        let mut sketch = Sketch::new(WorkPlane::XY);
        let free = sketch.add_point(Vec2::new(3.0, 4.0));

        sketch.set_dimension(
            DimensionTarget::Distance {
                from: Sketch::ORIGIN,
                to: free,
            },
            10.0,
            false,
        );
        assert_eq!(sketch.resolve(1.0), LengthOutcome::Exact);

        let distance = sketch.point(free).length();
        assert!((distance - 10.0).abs() < 0.01, "got {distance}");
        assert_eq!(sketch.point(Sketch::ORIGIN), Vec2::ZERO);
    }

    #[test]
    fn the_nearest_segment_is_found_along_its_body() {
        let mut sketch = Sketch::new(WorkPlane::XY);
        let a = sketch.add_point(Vec2::ZERO);
        let b = sketch.add_point(Vec2::new(100.0, 0.0));
        let segment = sketch.add_segment(a, b);

        assert_eq!(
            sketch.nearest_segment(Vec2::new(50.0, 2.0), 5.0),
            Some(segment)
        );
        assert_eq!(sketch.nearest_segment(Vec2::new(50.0, 40.0), 5.0), None);
        assert_eq!(sketch.nearest_segment(Vec2::new(150.0, 0.0), 5.0), None);
    }

    #[test]
    fn a_circle_is_found_by_its_outline() {
        let mut sketch = Sketch::new(WorkPlane::XY);
        let center = sketch.add_point(Vec2::ZERO);
        let circle = sketch.add_circle(center, 10.0);

        assert_eq!(
            sketch.nearest_circle(Vec2::new(10.2, 0.0), 1.0),
            Some(circle)
        );
        assert_eq!(
            sketch.nearest_circle(Vec2::ZERO, 1.0),
            None,
            "not the middle"
        );
        assert_eq!(sketch.nearest_circle(Vec2::new(30.0, 0.0), 1.0), None);
    }

    #[test]
    fn bounds_cover_every_point() {
        let mut sketch = Sketch::new(WorkPlane::XY);
        sketch.add_point(Vec2::new(-3.0, 7.0));
        sketch.add_point(Vec2::new(12.0, -1.0));
        let (min, max) = sketch.bounds().expect("some points");
        // The origin is a point like any other as far as framing goes.
        assert_eq!(min, Vec2::new(-3.0, -1.0));
        assert_eq!(max, Vec2::new(12.0, 7.0));
    }
}
