use cao_prefs::theme::Theme;
use cao_render::{Vertex, srgb};
use cao_sketch::{AnnotationMetrics, DimensionTarget, Sketch};
use glam::DVec2;

/// The three drawing norms shared by every style: how far a dimension stands
/// off on its own, how long an arrow is, and the radius of an angle's arc.
/// Candidates for `cao_prefs`, once a profile can say how a drawing looks.
const OFFSET_PIXELS: f64 = 22.0;
const ARROW_PIXELS: f64 = 8.0;
const ARC_PIXELS: f64 = 34.0;

/// Dimensions are drawn, not pasted in from pictures: extension lines,
/// arrowheads and arcs are a handful of segments, they follow the geometry as
/// it moves, and they stay crisp at any zoom. An image would have to be
/// re-made for every value and every angle.
pub struct Style {
    pub color: [f32; 4],
    pub width: f32,
}

impl Style {
    pub fn driving(theme: &Theme) -> Self {
        let color = theme.dimension;
        Self {
            color: srgb(color.r, color.g, color.b, color.a),
            width: 1.2,
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

/// What `sketch.place` needs to find where an annotation sits: the drawing
/// norms and the pixel scale of the view, with `nudge` added on top of
/// whatever offset the dimension already carries, for one being dragged.
///
/// No colour, so asking where an annotation is does not need a theme.
pub fn metrics(pixel: f64, nudge: DVec2) -> AnnotationMetrics {
    AnnotationMetrics {
        offset_pixels: OFFSET_PIXELS,
        arrow_pixels: ARROW_PIXELS,
        arc_pixels: ARC_PIXELS,
        pixel,
        nudge,
    }
}

/// Draws one dimension and says where its value belongs.
///
/// The geometry — which side it stands off on, its extension lines, its
/// arrows or its arc — is `sketch.place`'s decision; this only turns the
/// segments it returns into vertices, with a colour.
pub fn push(
    out: &mut Vec<Vertex>,
    sketch: &Sketch,
    target: DimensionTarget,
    style: &Style,
    pixel: f64,
    nudge: DVec2,
) -> Option<DVec2> {
    let placed = sketch.place(target, metrics(pixel, nudge))?;
    for (from, to) in placed.shape {
        out.push(Vertex::line(
            sketch.plane.to_world(from).as_vec3(),
            style.color,
            style.width,
        ));
        out.push(Vertex::line(
            sketch.plane.to_world(to).as_vec3(),
            style.color,
            style.width,
        ));
    }
    Some(placed.text_at)
}
