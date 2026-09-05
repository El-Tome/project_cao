use cao_core::theme::Theme;
use cao_render::{Vertex, srgb};
use cao_sketch::{DimensionTarget, Sketch, WorkPlane};
use glam::Vec2;

/// Dimensions are drawn, not pasted in from pictures: extension lines,
/// arrowheads and arcs are a handful of segments, they follow the geometry as
/// it moves, and they stay crisp at any zoom. An image would have to be
/// re-made for every value and every angle.
pub struct Style {
    pub color: [f32; 4],
    pub width: f32,
    /// How far the dimension line sits from what it measures, in pixels.
    pub offset_pixels: f32,
    /// Length of an arrowhead, in pixels.
    pub arrow_pixels: f32,
    /// Radius of an angle's arc, in pixels.
    pub arc_pixels: f32,
}

impl Style {
    pub fn driving(theme: &Theme) -> Self {
        let color = theme.dimension;
        Self {
            color: srgb(color.r, color.g, color.b, color.a),
            width: 1.2,
            offset_pixels: 22.0,
            arrow_pixels: 8.0,
            arc_pixels: 34.0,
        }
    }

    /// A readout is drawn more quietly: it reports rather than decides.
    pub fn driven(theme: &Theme) -> Self {
        let color = theme.dimension_driven;
        Self {
            color: srgb(color.r, color.g, color.b, color.a),
            ..Self::driving(theme)
        }
    }
}

/// Where the value should be written, once the annotation is drawn.
pub struct Placement {
    pub text_at: Vec2,
}

/// Draws one dimension and says where its value belongs.
///
/// `pixel` is how many sketch units one screen pixel covers, so the annotation
/// keeps the same size on screen however far the camera is.
pub fn push(
    out: &mut Vec<Vertex>,
    sketch: &Sketch,
    target: DimensionTarget,
    style: &Style,
    pixel: f32,
) -> Option<Placement> {
    let plane = &sketch.plane;
    let offset = sketch
        .dimension_of(target)
        .map(|dimension| dimension.offset)
        .unwrap_or(Vec2::ZERO);
    match target {
        DimensionTarget::Length(segment) => {
            let (start, end) = endpoints(sketch, segment)?;
            Some(linear(
                out,
                plane,
                start,
                end,
                away_from(sketch),
                offset,
                style,
                pixel,
            ))
        }
        DimensionTarget::Distance { from, to } => {
            let (start, end) = (*sketch.points().get(from.0)?, *sketch.points().get(to.0)?);
            Some(linear(
                out,
                plane,
                start,
                end,
                away_from(sketch),
                offset,
                style,
                pixel,
            ))
        }
        DimensionTarget::Angle { first, second } => {
            let (pivot, a, b) = sketch.corner_points(first, second)?;
            Some(angular(out, plane, pivot, a, b, offset, style, pixel))
        }
        DimensionTarget::AxisAngle { segment, axis } => {
            let (start, end) = endpoints(sketch, segment)?;
            // Measured from the axis direction taken at the segment's start.
            Some(angular(
                out,
                plane,
                start,
                start + axis.direction() * start.distance(end),
                end,
                offset,
                style,
                pixel,
            ))
        }
        DimensionTarget::Radius(circle) => {
            let circle = *sketch.circles().get(circle.0)?;
            let center = *sketch.points().get(circle.center.0)?;
            Some(radial(out, plane, center, circle.radius, offset, style, pixel))
        }
    }
}

fn endpoints(sketch: &Sketch, segment: cao_sketch::SegmentId) -> Option<(Vec2, Vec2)> {
    (segment.0 < sketch.segments().len()).then(|| sketch.endpoints(segment))
}

/// The middle of the drawing, used to push dimension lines outwards. Laid over
/// the shape they measure, they hide it; outside, they read like a drawing.
fn away_from(sketch: &Sketch) -> Vec2 {
    sketch
        .bounds()
        .map(|(min, max)| (min + max) * 0.5)
        .unwrap_or(Vec2::ZERO)
}

/// A length or a distance: the classic pair of extension lines with a
/// dimension line between them.
#[allow(clippy::too_many_arguments)]
fn linear(
    out: &mut Vec<Vertex>,
    plane: &WorkPlane,
    start: Vec2,
    end: Vec2,
    center: Vec2,
    moved_by: Vec2,
    style: &Style,
    pixel: f32,
) -> Placement {
    let span = end - start;
    let direction = span.normalize_or(Vec2::X);
    let mut normal = Vec2::new(-direction.y, direction.x);

    // Always step away from the drawing: on a closed contour the inward side
    // lays the dimension line straight over the shape it measures.
    let middle = (start + end) * 0.5;
    if normal.dot(middle - center) < 0.0 {
        normal = -normal;
    }

    // Dragging moves the whole annotation, not just its value: the line, its
    // arrows and its text travel together, with the extension lines stretching
    // to follow. A number floating away from its own line reads as a stray
    // label rather than a dimension.
    let offset = normal * style.offset_pixels * pixel + moved_by;
    let (from, to) = (start + offset, end + offset);

    // Extension lines overshoot the dimension line a little, as on a drawing.
    let overshoot = normal * 4.0 * pixel;
    line(out, plane, start, from + overshoot, style);
    line(out, plane, end, to + overshoot, style);
    line(out, plane, from, to, style);

    arrow(out, plane, from, direction, style, pixel);
    arrow(out, plane, to, -direction, style, pixel);

    Placement {
        text_at: (from + to) * 0.5 + normal * text_clearance(normal) * pixel,
    }
}

/// How far to push a label off its line so the line does not run through it.
///
/// Text is much wider than it is tall, so clearing it sideways — which is what
/// a vertical dimension needs — takes far more room than clearing it upwards.
fn text_clearance(normal: Vec2) -> f32 {
    12.0 + 24.0 * normal.x.abs()
}

/// An angle: an arc between the two arms, with an arrowhead at each end.
#[allow(clippy::too_many_arguments)]
fn angular(
    out: &mut Vec<Vertex>,
    plane: &WorkPlane,
    pivot: Vec2,
    first: Vec2,
    second: Vec2,
    moved_by: Vec2,
    style: &Style,
    pixel: f32,
) -> Placement {
    let start = (first - pivot).to_angle();
    let mut sweep = (second - pivot).to_angle() - start;
    // Always draw the smaller way round: that is the angle being talked about.
    while sweep > std::f32::consts::PI {
        sweep -= std::f32::consts::TAU;
    }
    while sweep < -std::f32::consts::PI {
        sweep += std::f32::consts::TAU;
    }

    // An arc has to stay hinged on the corner it measures, so a drag opens it
    // out instead of tearing it away: the part of the movement along the
    // bisector becomes radius, the part across it slides the value round.
    let bisector = Vec2::from_angle(start + sweep * 0.5);
    let widened = moved_by.dot(bisector);
    let radius = (style.arc_pixels * pixel + widened).max(6.0 * pixel);
    let alongside = moved_by - bisector * widened;

    const STEPS: usize = 24;
    let mut previous = None;
    for step in 0..=STEPS {
        let angle = start + sweep * step as f32 / STEPS as f32;
        let point = pivot + Vec2::from_angle(angle) * radius;
        if let Some(previous) = previous {
            line(out, plane, previous, point, style);
        }
        previous = Some(point);
    }

    // Arrowheads point along the arc, so they lie tangent to it.
    let tangent =
        |angle: f32, sign: f32| Vec2::from_angle(angle + std::f32::consts::FRAC_PI_2) * sign;
    let at_start = pivot + Vec2::from_angle(start) * radius;
    let at_end = pivot + Vec2::from_angle(start + sweep) * radius;
    arrow(
        out,
        plane,
        at_start,
        tangent(start, sweep.signum()),
        style,
        pixel,
    );
    arrow(
        out,
        plane,
        at_end,
        tangent(start + sweep, -sweep.signum()),
        style,
        pixel,
    );

    Placement {
        text_at: pivot + bisector * (radius + 14.0 * pixel) + alongside,
    }
}

/// A radius: a line from the centre out to the circle, arrow on the rim.
fn radial(
    out: &mut Vec<Vertex>,
    plane: &WorkPlane,
    center: Vec2,
    radius: f32,
    moved_by: Vec2,
    style: &Style,
    pixel: f32,
) -> Placement {
    // A radius is always drawn from the centre outwards, so dragging it turns
    // the leader about the circle rather than detaching it.
    let default = Vec2::splat(std::f32::consts::FRAC_1_SQRT_2);
    let direction = (default * radius + moved_by).normalize_or(default);
    let rim = center + direction * radius;
    line(out, plane, center, rim, style);
    arrow(out, plane, rim, -direction, style, pixel);

    let aside = Vec2::new(-direction.y, direction.x);
    Placement {
        text_at: center + direction * radius * 0.55 + aside * text_clearance(aside) * pixel,
    }
}

/// An arrowhead at `tip`, opening along `direction` (which points away from
/// the tip, back down the line).
fn arrow(
    out: &mut Vec<Vertex>,
    plane: &WorkPlane,
    tip: Vec2,
    direction: Vec2,
    style: &Style,
    pixel: f32,
) {
    let length = style.arrow_pixels * pixel;
    let back = direction.normalize_or(Vec2::X) * length;
    let side = Vec2::new(-back.y, back.x) * 0.35;

    line(out, plane, tip, tip + back + side, style);
    line(out, plane, tip, tip + back - side, style);
}

fn line(out: &mut Vec<Vertex>, plane: &WorkPlane, from: Vec2, to: Vec2, style: &Style) {
    out.push(Vertex::line(plane.to_world(from), style.color, style.width));
    out.push(Vertex::line(plane.to_world(to), style.color, style.width));
}
