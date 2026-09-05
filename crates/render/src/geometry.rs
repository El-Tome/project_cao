use bytemuck::{Pod, Zeroable};
use glam::Vec3;

/// A single vertex of the line/triangle soup the renderer consumes. Colors are
/// linear (not sRGB): see [`srgb`]. `width` is the line thickness in physical
/// pixels, and is ignored by triangle geometry.
#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct Vertex {
    pub position: [f32; 3],
    pub color: [f32; 4],
    pub width: f32,
}

impl Vertex {
    pub fn line(position: Vec3, color: [f32; 4], width: f32) -> Self {
        Self {
            position: position.to_array(),
            color,
            width,
        }
    }

    pub fn solid(position: Vec3, color: [f32; 4]) -> Self {
        Self::line(position, color, 0.0)
    }
}

/// Converts an sRGB color (the space color pickers and CSS use) to the linear
/// space the shader blends and writes in.
pub fn srgb(r: f32, g: f32, b: f32, a: f32) -> [f32; 4] {
    fn channel(c: f32) -> f32 {
        if c <= 0.04045 {
            c / 12.92
        } else {
            ((c + 0.055) / 1.055).powf(2.4)
        }
    }
    [channel(r), channel(g), channel(b), a]
}

pub struct AxisStyle {
    pub x: [f32; 4],
    pub y: [f32; 4],
    pub z: [f32; 4],
    pub width: f32,
}

impl Default for AxisStyle {
    fn default() -> Self {
        Self {
            x: srgb(0.90, 0.30, 0.35, 1.0),
            y: srgb(0.45, 0.75, 0.30, 1.0),
            z: srgb(0.30, 0.55, 0.95, 1.0),
            width: 2.0,
        }
    }
}

/// The three world axes as lines through the origin, long enough to always
/// leave the view at the given camera distance.
///
/// `facing` drops the axis pointing at the camera: on a work plane it would
/// project to a single dot sitting over the drawing, which reads as a stray
/// mark rather than an axis.
pub fn push_axes(out: &mut Vec<Vertex>, half_length: f32, style: &AxisStyle, facing: Option<Vec3>) {
    for (axis, color) in [(Vec3::X, style.x), (Vec3::Y, style.y), (Vec3::Z, style.z)] {
        if facing.is_some_and(|normal| normal.normalize_or_zero().dot(axis).abs() > 0.999) {
            continue;
        }
        out.push(Vertex::line(-axis * half_length, color, style.width));
        out.push(Vertex::line(axis * half_length, color, style.width));
    }
}

/// Picks the grid step from the 1–2–5–10 sequence, the smallest one whose
/// on-screen spacing stays above `target_pixel_spacing`. Zooming in therefore
/// subdivides the grid, zooming out merges it.
pub fn adaptive_step(world_units_per_pixel: f32, target_pixel_spacing: f32) -> f32 {
    let minimum = (world_units_per_pixel * target_pixel_spacing).max(1e-6);
    let decade = 10f32.powf(minimum.log10().floor());
    for multiple in [1.0, 2.0, 5.0] {
        if decade * multiple >= minimum {
            return decade * multiple;
        }
    }
    decade * 10.0
}

pub struct GridStyle {
    pub minor: [f32; 4],
    pub major: [f32; 4],
    pub minor_width: f32,
    pub major_width: f32,
    /// Every n-th line uses `major`.
    pub major_every: i32,
    /// Segments per line; more segments make the radial fade smoother.
    pub segments: i32,
}

impl Default for GridStyle {
    fn default() -> Self {
        Self {
            minor: srgb(0.55, 0.58, 0.62, 0.35),
            major: srgb(0.65, 0.68, 0.73, 0.60),
            minor_width: 1.0,
            major_width: 1.5,
            major_every: 10,
            segments: 24,
        }
    }
}

/// A grid on the plane spanned by `u` and `v`, centred on `center` snapped to
/// the step so lines stay put while panning, fading out radially instead of
/// ending on a hard edge.
///
/// Taking the basis rather than one of three named planes means any work plane
/// gets a grid, including one lying at an angle on a face.
pub fn push_grid(
    out: &mut Vec<Vertex>,
    u: Vec3,
    v: Vec3,
    center: Vec3,
    step: f32,
    half_extent: f32,
    style: &GridStyle,
) {
    let lines = (half_extent / step).ceil() as i32;
    let origin = snap_to_step(center, u, v, step);

    for (along, across) in [(u, v), (v, u)] {
        for index in -lines..=lines {
            let offset = index as f32 * step;
            let base = origin + across * offset;

            if base.dot(across).abs() < step * 0.001 {
                continue;
            }

            // "Major" must follow the world coordinate, not the index: the
            // index is counted from the panned centre, so using it would make
            // the heavy lines drift away from the origin as you pan.
            let steps_from_origin = base.dot(across) / (step * style.major_every as f32);
            let major = (steps_from_origin - steps_from_origin.round()).abs() < 1e-3;
            let color = if major { style.major } else { style.minor };
            let width = if major {
                style.major_width
            } else {
                style.minor_width
            };

            push_faded_line(
                out,
                &GridLine {
                    base,
                    direction: along,
                    color,
                    width,
                },
                half_extent,
                origin,
                style.segments,
            );
        }
    }
}

fn snap_to_step(center: Vec3, u: Vec3, v: Vec3, step: f32) -> Vec3 {
    u * (center.dot(u) / step).round() * step + v * (center.dot(v) / step).round() * step
}

/// One grid line, ready to be emitted as segments.
struct GridLine {
    base: Vec3,
    direction: Vec3,
    color: [f32; 4],
    width: f32,
}

/// Emits one grid line as a strip of segments whose alpha falls off with the
/// distance to the grid centre.
fn push_faded_line(
    out: &mut Vec<Vertex>,
    line: &GridLine,
    half_extent: f32,
    center: Vec3,
    segments: i32,
) {
    let GridLine {
        base,
        direction,
        color,
        width,
    } = *line;
    let segments = segments.max(1);
    let mut previous = None;

    for index in 0..=segments {
        let t = index as f32 / segments as f32 * 2.0 - 1.0;
        let point = base + direction * (t * half_extent);
        let fade = radial_fade(point.distance(center), half_extent);
        let vertex = Vertex::line(
            point,
            [color[0], color[1], color[2], color[3] * fade],
            width,
        );

        if let Some(previous) = previous {
            out.push(previous);
            out.push(vertex);
        }
        previous = Some(vertex);
    }
}

/// Fully opaque out to `FADE_START` of the extent, then falling off to
/// nothing at the edge. Fading from the very centre would draw the grid as a
/// small bright disc floating in the middle of the view.
const FADE_START: f32 = 0.55;

fn radial_fade(distance: f32, half_extent: f32) -> f32 {
    let normalized = (distance / half_extent).clamp(0.0, 1.0);
    if normalized <= FADE_START {
        return 1.0;
    }
    let outer = (normalized - FADE_START) / (1.0 - FADE_START);
    (1.0 - outer * outer).clamp(0.0, 1.0)
}

/// A square patch of a plane, centred on `center`, as two triangles. Used to
/// show the work planes a sketch can start on.
pub fn push_plane_quad(
    out: &mut Vec<Vertex>,
    center: Vec3,
    u: Vec3,
    v: Vec3,
    half_size: f32,
    color: [f32; 4],
) {
    let corners = [
        center - u * half_size - v * half_size,
        center + u * half_size - v * half_size,
        center + u * half_size + v * half_size,
        center - u * half_size + v * half_size,
    ];
    for point in [
        corners[0], corners[1], corners[2], corners[0], corners[2], corners[3],
    ] {
        out.push(Vertex::solid(point, color));
    }
}

/// The outline of that same square patch.
pub fn push_plane_outline(
    out: &mut Vec<Vertex>,
    center: Vec3,
    u: Vec3,
    v: Vec3,
    half_size: f32,
    color: [f32; 4],
    width: f32,
) {
    let corners = [
        center - u * half_size - v * half_size,
        center + u * half_size - v * half_size,
        center + u * half_size + v * half_size,
        center - u * half_size + v * half_size,
    ];
    for index in 0..4 {
        out.push(Vertex::line(corners[index], color, width));
        out.push(Vertex::line(corners[(index + 1) % 4], color, width));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The step must always keep lines at least `target` pixels apart, and
    /// only ever be a 1, 2 or 5 times a power of ten.
    #[test]
    fn adaptive_step_follows_the_1_2_5_sequence() {
        let target = 48.0;
        let mut units_per_pixel = 1e-4;

        while units_per_pixel < 1e4 {
            let step = adaptive_step(units_per_pixel, target);
            assert!(
                step >= units_per_pixel * target,
                "step {step} too small for {units_per_pixel}"
            );

            let mantissa = step / 10f32.powf(step.log10().floor());
            assert!(
                [1.0, 2.0, 5.0]
                    .iter()
                    .any(|value| (mantissa - value).abs() < 1e-3),
                "step {step} is not a 1/2/5 multiple"
            );

            units_per_pixel *= 1.3;
        }
    }

    /// Zooming in never coarsens the grid, zooming out never refines it.
    #[test]
    fn adaptive_step_grows_with_distance() {
        let mut previous = 0.0;
        for exponent in -4..4 {
            let step = adaptive_step(10f32.powi(exponent), 48.0);
            assert!(step >= previous);
            previous = step;
        }
    }

    /// Heavy lines must sit on world multiples of the major step whatever the
    /// grid is centred on, otherwise they drift away from the axes on a pan.
    #[test]
    fn major_lines_stay_anchored_to_the_origin_when_panning() {
        let style = GridStyle::default();
        let step = 10.0;
        let major_step = step * style.major_every as f32;

        for center in [
            Vec3::ZERO,
            Vec3::new(37.0, -114.0, 0.0),
            Vec3::new(-950.0, 620.0, 0.0),
        ] {
            let mut vertices = Vec::new();
            push_grid(&mut vertices, Vec3::X, Vec3::Y, center, step, 300.0, &style);

            let major_lines: Vec<_> = vertices
                .iter()
                .filter(|vertex| vertex.width == style.major_width)
                .collect();
            assert!(
                !major_lines.is_empty(),
                "no major line for centre {center:?}"
            );

            for vertex in major_lines {
                // A heavy line runs along one axis, so exactly one of its two
                // in-plane coordinates is the constant that must land on the
                // major step.
                let [x, y, _] = vertex.position;
                let on_x = (x / major_step - (x / major_step).round()).abs() < 1e-3;
                let on_y = (y / major_step - (y / major_step).round()).abs() < 1e-3;
                assert!(
                    on_x || on_y,
                    "major line vertex at ({x}, {y}) is off the {major_step} grid"
                );
            }
        }
    }

    #[test]
    fn grid_is_opaque_around_its_centre() {
        let mut vertices = Vec::new();
        push_grid(
            &mut vertices,
            Vec3::X,
            Vec3::Y,
            Vec3::ZERO,
            10.0,
            300.0,
            &GridStyle::default(),
        );

        let near_center = vertices
            .iter()
            .filter(|vertex| Vec3::from_array(vertex.position).length() < 100.0)
            .count();
        assert!(near_center > 0);
        assert!(
            vertices
                .iter()
                .filter(|vertex| Vec3::from_array(vertex.position).length() < 100.0)
                .all(|vertex| vertex.color[3] > 0.2),
            "the grid must not fade out right next to its centre"
        );
    }

    #[test]
    fn grid_skips_the_lines_the_axes_already_draw() {
        let mut vertices = Vec::new();
        push_grid(
            &mut vertices,
            Vec3::X,
            Vec3::Y,
            Vec3::ZERO,
            10.0,
            50.0,
            &GridStyle::default(),
        );
        assert!(!vertices.is_empty());

        // A segment lying flat on an axis would double up the coloured axis
        // line; individual vertices may still touch an axis when a line
        // crosses it.
        for segment in vertices.as_chunks::<2>().0 {
            let [start, end] = [segment[0].position, segment[1].position];
            assert!(
                !(start[0] == 0.0 && end[0] == 0.0) && !(start[1] == 0.0 && end[1] == 0.0),
                "grid segment {start:?}..{end:?} lies on an axis"
            );
        }
    }
}
