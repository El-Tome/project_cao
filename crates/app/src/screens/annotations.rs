use cao_core::theme::Theme;
use cao_render::{Vertex, srgb};
use cao_sketch::{DimensionTarget, Sketch, WorkPlane};
use glam::DVec2;

/// Dimensions are drawn, not pasted in from pictures: extension lines,
/// arrowheads and arcs are a handful of segments, they follow the geometry as
/// it moves, and they stay crisp at any zoom. An image would have to be
/// re-made for every value and every angle.
pub struct Style {
    pub color: [f32; 4],
    pub width: f32,
    /// How far the dimension line sits from what it measures, in pixels.
    pub offset_pixels: f64,
    /// Length of an arrowhead, in pixels.
    pub arrow_pixels: f64,
    /// Radius of an angle's arc, in pixels.
    pub arc_pixels: f64,
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

/// Where the value should be written, once the annotation is drawn, and the
/// offset that would put the annotation exactly where it has just been drawn.
///
/// The second is what a drag records: the annotation says where it ended up, so
/// nothing outside has to redo its geometry to work it out.
pub struct Placement {
    pub text_at: DVec2,
    pub offset: DVec2,
}

/// Draws one dimension and says where its value belongs.
///
/// `pixel` is how many sketch units one screen pixel covers, so the annotation
/// keeps the same size on screen however far the camera is.
/// `nudge` is added to the offset the dimension carries, for an annotation
/// being dragged: nothing is recorded until the button is let go, but it has to
/// follow the cursor meanwhile or the drag looks like it did nothing.
pub fn push(
    out: &mut Vec<Vertex>,
    sketch: &Sketch,
    target: DimensionTarget,
    style: &Style,
    pixel: f64,
    nudge: DVec2,
) -> Option<Placement> {
    let plane = &sketch.plane;
    let placed = sketch.dimension_of(target).and_then(|dimension| dimension.offset);
    match target {
        DimensionTarget::Length(segment) => {
            let (start, end) = endpoints(sketch, segment)?;
            Some(linear(
                out,
                plane,
                Span::between(start, end),
                away_from(sketch),
                Moved { placed, nudge },
                style,
                pixel,
            ))
        }
        DimensionTarget::Distance { from, to } => {
            let (start, end) = (*sketch.points().get(from.0)?, *sketch.points().get(to.0)?);
            Some(linear(
                out,
                plane,
                Span::between(start, end),
                away_from(sketch),
                Moved { placed, nudge },
                style,
                pixel,
            ))
        }
        DimensionTarget::Projected { from, to, axis } => {
            let (start, end) = (*sketch.points().get(from.0)?, *sketch.points().get(to.0)?);
            Some(linear(
                out,
                plane,
                Span::along(start, end, axis.direction()),
                away_from(sketch),
                Moved { placed, nudge },
                style,
                pixel,
            ))
        }
        DimensionTarget::PointToSegment { point, segment } => {
            let at = *sketch.points().get(point.0)?;
            let foot = sketch.foot_on_segment(point, segment)?;
            // The line is measured, not the drawn part of it: when the foot
            // lands past the end, a thin line carries the segment out to it, as
            // on a drawing.
            let (start, end) = endpoints(sketch, segment)?;
            for corner in [start, end] {
                if (foot - start).dot(foot - end) > 0.0 {
                    line(out, plane, corner, foot, style);
                }
            }
            Some(linear(
                out,
                plane,
                Span::between(foot, at),
                away_from(sketch),
                Moved { placed, nudge },
                style,
                pixel,
            ))
        }
        DimensionTarget::Angle { first, second } => {
            let (pivot, a, b) = sketch.corner_points(first, second)?;
            Some(angular(
                out,
                plane,
                pivot,
                a,
                b,
                Moved { placed, nudge },
                style,
                pixel,
            ))
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
                Moved { placed, nudge },
                style,
                pixel,
            ))
        }
        DimensionTarget::Diameter(circle) => {
            let circle = *sketch.circles().get(circle.0)?;
            let center = *sketch.points().get(circle.center.0)?;
            Some(across(
                out,
                plane,
                center,
                circle.radius,
                Moved { placed, nudge },
                style,
                pixel,
            ))
        }
        DimensionTarget::Radius(circle) => {
            let circle = *sketch.circles().get(circle.0)?;
            let center = *sketch.points().get(circle.center.0)?;
            Some(radial(
                out,
                plane,
                center,
                circle.radius,
                Moved { placed, nudge },
                style,
                pixel,
            ))
        }
    }
}

/// Where an annotation sits: what was recorded for it, if anything, plus what a
/// drag in progress is adding on top.
///
/// The two are kept apart because they answer different questions. A recorded
/// offset is in sketch units and holds the annotation in place whatever the
/// zoom; with nothing recorded, the annotation stands off by a distance in
/// pixels, which is what keeps a fresh drawing readable at any scale.
#[derive(Clone, Copy)]
struct Moved {
    placed: Option<DVec2>,
    nudge: DVec2,
}

impl Moved {
    /// How far along `direction` the annotation has been pushed, falling back
    /// to `default` when it has never been placed.
    fn along(&self, direction: DVec2, default: f64) -> f64 {
        self.placed.map_or(default, |offset| offset.dot(direction)) + self.nudge.dot(direction)
    }
}

fn endpoints(sketch: &Sketch, segment: cao_sketch::SegmentId) -> Option<(DVec2, DVec2)> {
    (segment.0 < sketch.segments().len()).then(|| sketch.endpoints(segment))
}

/// The middle of the drawing, used to push dimension lines outwards. Laid over
/// the shape they measure, they hide it; outside, they read like a drawing.
fn away_from(sketch: &Sketch) -> DVec2 {
    sketch
        .bounds()
        .map(|(min, max)| (min + max) * 0.5)
        .unwrap_or(DVec2::ZERO)
}

/// What a linear dimension measures: two ends, and the direction its dimension
/// line runs in.
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
    out: &mut Vec<Vertex>,
    plane: &WorkPlane,
    span: Span,
    center: DVec2,
    moved_by: Moved,
    style: &Style,
    pixel: f64,
) -> Placement {
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

    // A dimension line has to stay parallel to the direction it measures, with
    // its extension lines perpendicular to it — otherwise it is a skewed pair
    // of arrows that no longer reads as a measurement. So the movement is
    // split: across the line it steps the line away, along it, it only slides
    // the value.
    //
    // The two ends are not always level (a width taken on a slanted trait is
    // exactly that case), so the line clears the outer of the two and the
    // extension lines come out different lengths, as on a drawing.
    let outer = start.dot(normal).max(end.dot(normal));
    let stepped = moved_by.along(normal, style.offset_pixels * pixel);
    let level = outer + stepped;
    let onto = |point: DVec2| point + normal * (level - point.dot(normal));
    let (from, to) = (onto(start), onto(end));

    // Extension lines overshoot the dimension line a little, as on a drawing.
    let overshoot = normal * 4.0 * pixel * stepped.signum();
    line(out, plane, start, from + overshoot, style);
    line(out, plane, end, to + overshoot, style);
    line(out, plane, from, to, style);

    arrow(out, plane, from, direction, style, pixel);
    arrow(out, plane, to, -direction, style, pixel);

    // The value may still slide along the line, which is what lets two
    // dimensions sharing a direction stop covering each other.
    let slid = moved_by.along(direction, 0.0);
    let foot = (from + to) * 0.5 + direction * slid;
    let clearance = normal * text_clearance(normal) * pixel;

    // Pushed past the ends, the value has nothing next to it saying which
    // dimension it belongs to. A leader carries the line out to it.
    if slid.abs() > (to - from).length() * 0.5 {
        let nearest = if slid > 0.0 { to } else { from };
        line(out, plane, nearest, foot, style);
        line(out, plane, foot, foot + clearance * 0.6, style);
    }

    Placement {
        text_at: foot + clearance,
        offset: normal * stepped + direction * slid,
    }
}

/// How far to push a label off its line so the line does not run through it.
///
/// Text is much wider than it is tall, so clearing it sideways — which is what
/// a vertical dimension needs — takes far more room than clearing it upwards.
fn text_clearance(normal: DVec2) -> f64 {
    12.0 + 24.0 * normal.x.abs()
}

/// An angle: an arc between the two arms, with an arrowhead at each end.
#[allow(clippy::too_many_arguments)]
fn angular(
    out: &mut Vec<Vertex>,
    plane: &WorkPlane,
    pivot: DVec2,
    first: DVec2,
    second: DVec2,
    moved_by: Moved,
    style: &Style,
    pixel: f64,
) -> Placement {
    let start = (first - pivot).to_angle();
    let mut sweep = (second - pivot).to_angle() - start;
    // Always draw the smaller way round: that is the angle being talked about.
    while sweep > std::f64::consts::PI {
        sweep -= std::f64::consts::TAU;
    }
    while sweep < -std::f64::consts::PI {
        sweep += std::f64::consts::TAU;
    }

    // An arc stays hinged on the corner it measures: what is recorded is where
    // the value sits relative to that corner, and the arc is drawn just inside
    // it. Splitting the movement into radius and slide instead let a value
    // dragged sideways shrink its own arc to nothing.
    let bisector = DVec2::from_angle(start + sweep * 0.5);
    let clearance = 14.0 * pixel;
    let reach = moved_by
        .placed
        .unwrap_or(bisector * (style.arc_pixels * pixel + clearance))
        + moved_by.nudge;
    let radius = (reach.length() - clearance).max(6.0 * pixel);

    const STEPS: usize = 24;
    let mut previous = None;
    for step in 0..=STEPS {
        let angle = start + sweep * step as f64 / STEPS as f64;
        let point = pivot + DVec2::from_angle(angle) * radius;
        if let Some(previous) = previous {
            line(out, plane, previous, point, style);
        }
        previous = Some(point);
    }

    // Arrowheads point along the arc, so they lie tangent to it.
    let tangent =
        |angle: f64, sign: f64| DVec2::from_angle(angle + std::f64::consts::FRAC_PI_2) * sign;
    let at_start = pivot + DVec2::from_angle(start) * radius;
    let at_end = pivot + DVec2::from_angle(start + sweep) * radius;
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

    // Dragged outside the two arms, the value has nothing joining it to the arc
    // it belongs to; a leader says where it comes from.
    let text_at = pivot + reach;
    let towards = reach.to_angle();
    let mut turn = towards - start;
    while turn > std::f64::consts::PI {
        turn -= std::f64::consts::TAU;
    }
    while turn < -std::f64::consts::PI {
        turn += std::f64::consts::TAU;
    }
    let fraction = turn / sweep;
    if !fraction.is_finite() || !(0.0..=1.0).contains(&fraction) {
        let nearer = if turn.abs() < (turn - sweep).abs() {
            at_start
        } else {
            at_end
        };
        line(out, plane, nearer, text_at - reach.normalize_or_zero() * clearance, style);
    }

    Placement {
        text_at,
        offset: reach,
    }
}

/// A diameter: the line right across the circle, an arrow at each end.
///
/// Drawn through the middle rather than from it, which is what tells a diameter
/// from a radius at a glance.
fn across(
    out: &mut Vec<Vertex>,
    plane: &WorkPlane,
    center: DVec2,
    radius: f64,
    moved_by: Moved,
    style: &Style,
    pixel: f64,
) -> Placement {
    let default = DVec2::splat(std::f64::consts::FRAC_1_SQRT_2);
    let placed = moved_by.placed.unwrap_or(default * radius) + moved_by.nudge;
    let direction = placed.normalize_or(default);
    let (from, to) = (center - direction * radius, center + direction * radius);

    line(out, plane, from, to, style);
    arrow(out, plane, from, direction, style, pixel);
    arrow(out, plane, to, -direction, style, pixel);

    let aside = DVec2::new(-direction.y, direction.x);
    Placement {
        text_at: center + direction * radius * 0.5 + aside * text_clearance(aside) * pixel,
        offset: direction * radius,
    }
}

/// A radius: a line from the centre out to the circle, arrow on the rim.
fn radial(
    out: &mut Vec<Vertex>,
    plane: &WorkPlane,
    center: DVec2,
    radius: f64,
    moved_by: Moved,
    style: &Style,
    pixel: f64,
) -> Placement {
    // A radius is always drawn from the centre outwards, so dragging it turns
    // the leader about the circle rather than detaching it.
    let default = DVec2::splat(std::f64::consts::FRAC_1_SQRT_2);
    let placed = moved_by.placed.unwrap_or(default * radius) + moved_by.nudge;
    let direction = placed.normalize_or(default);
    let rim = center + direction * radius;
    line(out, plane, center, rim, style);
    arrow(out, plane, rim, -direction, style, pixel);

    let aside = DVec2::new(-direction.y, direction.x);
    Placement {
        text_at: center + direction * radius * 0.55 + aside * text_clearance(aside) * pixel,
        offset: direction * radius,
    }
}

/// An arrowhead at `tip`, opening along `direction` (which points away from
/// the tip, back down the line).
fn arrow(
    out: &mut Vec<Vertex>,
    plane: &WorkPlane,
    tip: DVec2,
    direction: DVec2,
    style: &Style,
    pixel: f64,
) {
    let length = style.arrow_pixels * pixel;
    let back = direction.normalize_or(DVec2::X) * length;
    let side = DVec2::new(-back.y, back.x) * 0.35;

    line(out, plane, tip, tip + back + side, style);
    line(out, plane, tip, tip + back - side, style);
}

fn line(out: &mut Vec<Vertex>, plane: &WorkPlane, from: DVec2, to: DVec2, style: &Style) {
    out.push(Vertex::line(plane.to_world(from).as_vec3(), style.color, style.width));
    out.push(Vertex::line(plane.to_world(to).as_vec3(), style.color, style.width));
}
