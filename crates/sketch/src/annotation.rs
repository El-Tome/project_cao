//! Where a dimension's annotation is drawn: which side it stands off on, how
//! its offset splits into standing off and sliding along, and where its value
//! belongs. The document records the offset; this is what turns that offset,
//! and the geometry it measures, into a shape. `cao_app` only turns the
//! segments this produces into vertices, with a colour.

use std::f64::consts::FRAC_1_SQRT_2;

use glam::DVec2;

use crate::annotation::angle::{Arm, angular};
use crate::annotation::between_traits::between_traits;
use crate::arc_annotation;
use crate::constraints::DimensionTarget;
use crate::segment::overshot_end;
use crate::sketch::Sketch;

mod angle;
mod between_traits;

/// The numeric norms an annotation is drawn against — how far it stands off
/// on its own, how long an arrow or an arc is — together with the pixel scale
/// of the view it is drawn in and a drag not yet recorded.
///
/// Colour and line width stay in `cao_app`: they say how a dimension looks,
/// never where it sits.
#[derive(Clone, Copy)]
pub struct AnnotationMetrics {
    pub offset_pixels: f64,
    pub arrow_pixels: f64,
    pub arc_pixels: f64,
    /// How many sketch units one screen pixel covers, so the annotation keeps
    /// the same size on screen however far the camera is.
    pub pixel: f64,
    /// Added to the offset a dimension already carries, for an annotation
    /// being dragged: nothing is recorded until the button is let go, but it
    /// has to follow the cursor meanwhile or the drag looks like it did
    /// nothing.
    pub nudge: DVec2,
}

/// Where an annotation's value belongs, the offset that would record it
/// there, and the segments — extension lines, arrows, an arc — that draw it.
pub struct Placement {
    pub text_at: DVec2,
    pub offset: DVec2,
    pub shape: Vec<(DVec2, DVec2)>,
}

impl Sketch {
    /// Works out how one dimension is drawn: its extension lines, its arrows
    /// or its arc, and where its value belongs.
    pub fn place(&self, target: DimensionTarget, metrics: AnnotationMetrics) -> Option<Placement> {
        let placed = self.dimension_of(target).and_then(|d| d.offset);
        let nudge = metrics.nudge;
        let by = Moved { placed, nudge };
        let away = away_from(self);
        let mut shape = Vec::new();
        let (text_at, offset) = match target {
            DimensionTarget::Length(segment) => {
                let (start, end) = endpoints(self, segment)?;
                linear(&mut shape, Span::between(start, end), away, by, metrics)
            }
            DimensionTarget::Distance { from, to } => {
                let (start, end) = (*self.points().get(from.0)?, *self.points().get(to.0)?);
                linear(&mut shape, Span::between(start, end), away, by, metrics)
            }
            DimensionTarget::Projected { from, to, axis } => {
                let (start, end) = (*self.points().get(from.0)?, *self.points().get(to.0)?);
                let span = Span::along(start, end, axis.direction());
                linear(&mut shape, span, away, by, metrics)
            }
            DimensionTarget::PointToSegment { point, segment } => {
                let at = *self.points().get(point.0)?;
                let foot = self.foot_on_segment(point, segment)?;
                // The line is measured, not the drawn part of it: when the
                // foot lands past the end, a thin line carries the segment
                // out to it, as on a drawing.
                let (start, end) = endpoints(self, segment)?;
                if let Some(corner) = overshot_end(start, end, foot) {
                    shape.push((corner, foot));
                }
                linear(&mut shape, Span::between(foot, at), away, by, metrics)
            }
            DimensionTarget::Angle { first, second } => {
                let (pivot, a, b) = self.corner_points(first, second)?;
                angular(&mut shape, pivot, Arm::Drawn(a), b, by, metrics)
            }
            DimensionTarget::AngleBetween {
                first,
                first_toward,
                second,
                second_toward,
            } => between_traits(
                &mut shape,
                self,
                [(first, first_toward), (second, second_toward)],
                by,
                metrics,
            )?,
            DimensionTarget::AxisAngle { segment, axis } => {
                let (start, end) = endpoints(self, segment)?;
                let along = axis.direction();
                let opens_from = start + along * start.distance(end);
                let arm = match start.perp_dot(along).abs() <= metrics.pixel {
                    true => Arm::Drawn(opens_from),
                    false => Arm::Reference(opens_from),
                };
                angular(&mut shape, start, arm, end, by, metrics)
            }
            DimensionTarget::Diameter(circle) => {
                let circle = *self.circles().get(circle.0)?;
                let center = *self.points().get(circle.center.0)?;
                across(&mut shape, center, circle.radius, by, metrics)
            }
            DimensionTarget::Radius(circle) => {
                let circle = *self.circles().get(circle.0)?;
                let center = *self.points().get(circle.center.0)?;
                arc_annotation::radial(&mut shape, center, circle.radius, by, metrics)
            }
            DimensionTarget::ArcRadius(arc) => {
                arc_annotation::radius(self, arc, by, metrics, &mut shape)?
            }
            DimensionTarget::ArcSweep(arc) => {
                arc_annotation::sweep(self, arc, by, metrics, &mut shape)?
            }
        };
        Some(Placement {
            text_at,
            offset,
            shape,
        })
    }

    /// Where every dimension of the sketch writes its value. What picking an
    /// annotation and boxing a selection both need, and neither works out on
    /// its own.
    pub(crate) fn anchors(&self, metrics: AnnotationMetrics) -> Vec<(DimensionTarget, DVec2)> {
        self.dimensions()
            .iter()
            .filter_map(|dimension| {
                self.place(dimension.target, metrics)
                    .map(|placement| (dimension.target, placement.text_at))
            })
            .collect()
    }
}

/// Where an annotation sits: what was recorded for it, if anything, plus what
/// a drag in progress is adding on top.
#[derive(Clone, Copy)]
pub(crate) struct Moved {
    pub(crate) placed: Option<DVec2>,
    pub(crate) nudge: DVec2,
}

impl Moved {
    /// How far along `direction` the annotation has been pushed, falling back
    /// to `default` when it has never been placed.
    fn along(&self, direction: DVec2, default: f64) -> f64 {
        self.placed.map_or(default, |offset| offset.dot(direction)) + self.nudge.dot(direction)
    }
}

fn endpoints(sketch: &Sketch, segment: crate::sketch::SegmentId) -> Option<(DVec2, DVec2)> {
    (segment.0 < sketch.segments().len()).then(|| sketch.endpoints(segment))
}

/// The middle of the drawing, used to push dimension lines outwards. Laid
/// over the shape they measure, they hide it; outside, they read like a
/// drawing.
fn away_from(sketch: &Sketch) -> DVec2 {
    sketch
        .bounds()
        .map(|(min, max)| (min + max) * 0.5)
        .unwrap_or(DVec2::ZERO)
}

/// What a linear dimension measures: two ends, and the direction its
/// dimension line runs in.
///
/// The two are separate because a dimension on a slanted trait can be read
/// three ways — its length, its width, or its height — and the width is drawn
/// along the horizontal even though its ends are not level.
#[derive(Clone, Copy)]
struct Span {
    start: DVec2,
    end: DVec2,
    direction: DVec2,
}

impl Span {
    fn between(start: DVec2, end: DVec2) -> Self {
        Self {
            start,
            end,
            direction: (end - start).normalize_or(DVec2::X),
        }
    }

    fn along(start: DVec2, end: DVec2, direction: DVec2) -> Self {
        // Pointing the line the way the trait goes keeps the arrows outward.
        let direction = if (end - start).dot(direction) < 0.0 {
            -direction
        } else {
            direction
        };
        Self {
            start,
            end,
            direction,
        }
    }
}

/// A length or a distance: the classic pair of extension lines with a
/// dimension line between them.
fn linear(
    out: &mut Vec<(DVec2, DVec2)>,
    span: Span,
    center: DVec2,
    by: Moved,
    metrics: AnnotationMetrics,
) -> (DVec2, DVec2) {
    let Span {
        start,
        end,
        direction,
    } = span;
    let mut normal = DVec2::new(-direction.y, direction.x);

    // Always step away from the drawing: on a closed contour the inward side
    // lays the dimension line straight over the shape it measures.
    let middle = (start + end) * 0.5;
    if normal.dot(middle - center) < 0.0 {
        normal = -normal;
    }

    // A dimension line has to stay parallel to the direction it measures,
    // with its extension lines perpendicular to it. So the movement is
    // split: across the line it steps the line away, along it, it only
    // slides the value. The two ends are not always level (a width taken on
    // a slanted trait is exactly that case), so the line clears the outer of
    // the two and the extension lines come out different lengths, as on a
    // drawing.
    let outer = start.dot(normal).max(end.dot(normal));
    let stepped = by.along(normal, metrics.offset_pixels * metrics.pixel);
    let level = outer + stepped;
    let onto = |point: DVec2| point + normal * (level - point.dot(normal));
    let (from, to) = (onto(start), onto(end));

    // Extension lines overshoot the dimension line a little, as on a
    // drawing.
    let overshoot = normal * 4.0 * metrics.pixel * stepped.signum();
    out.push((start, from + overshoot));
    out.push((end, to + overshoot));
    out.push((from, to));

    arrow(out, from, direction, metrics);
    arrow(out, to, -direction, metrics);

    // The value may still slide along the line, which is what lets two
    // dimensions sharing a direction stop covering each other.
    let slid = by.along(direction, 0.0);
    let foot = (from + to) * 0.5 + direction * slid;
    let clearance = normal * text_clearance(normal) * metrics.pixel;

    // Pushed past the ends, the value has nothing next to it saying which
    // dimension it belongs to. A leader carries the line out to it.
    if slid.abs() > (to - from).length() * 0.5 {
        let nearest = if slid > 0.0 { to } else { from };
        out.push((nearest, foot));
        out.push((foot, foot + clearance * 0.6));
    }

    (foot + clearance, normal * stepped + direction * slid)
}

/// How far to push a label off its line so the line does not run through it.
///
/// Text is much wider than it is tall, so clearing it sideways — which is
/// what a vertical dimension needs — takes far more room than clearing it
/// upwards.
pub(crate) fn text_clearance(normal: DVec2) -> f64 {
    12.0 + 24.0 * normal.x.abs()
}

/// A diameter: the line right across the circle, an arrow at each end.
///
/// Drawn through the middle rather than from it, which is what tells a
/// diameter from a radius at a glance.
fn across(
    out: &mut Vec<(DVec2, DVec2)>,
    center: DVec2,
    radius: f64,
    by: Moved,
    metrics: AnnotationMetrics,
) -> (DVec2, DVec2) {
    let default = DVec2::splat(FRAC_1_SQRT_2);
    let placed = by.placed.unwrap_or(default * radius) + by.nudge;
    let direction = placed.normalize_or(default);
    let (from, to) = (center - direction * radius, center + direction * radius);

    out.push((from, to));
    arrow(out, from, direction, metrics);
    arrow(out, to, -direction, metrics);

    let aside = DVec2::new(-direction.y, direction.x);
    (
        center + direction * radius * 0.5 + aside * text_clearance(aside) * metrics.pixel,
        direction * radius,
    )
}

/// An arrowhead at `tip`, opening along `direction` (which points away from
/// the tip, back down the line).
pub(crate) fn arrow(
    out: &mut Vec<(DVec2, DVec2)>,
    tip: DVec2,
    direction: DVec2,
    metrics: AnnotationMetrics,
) {
    let length = metrics.arrow_pixels * metrics.pixel;
    let back = direction.normalize_or(DVec2::X) * length;
    let side = DVec2::new(-back.y, back.x) * 0.35;
    out.push((tip, tip + back + side));
    out.push((tip, tip + back - side));
}
