//! Vertices for everything in the scene that is not the part itself.

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

/// How a background gradient runs across the viewport.
#[derive(Clone, Copy, Debug)]
pub enum BackgroundShape {
    /// A flat colour: one call of the sampler decides everything.
    Flat,
    /// Straight across, at any angle. 0° runs bottom to top.
    Linear { angle_degrees: f32 },
    /// Out from a point given in fractions of the viewport, with the radius as
    /// a fraction of half its diagonal.
    Radial { center: [f32; 2], radius: f32 },
}

/// How finely a gradient is cut up. Colours are carried on the corners, so this
/// is what decides whether a gradient reads as smooth or as bands.
const GRADIENT_STEPS: usize = 48;

/// Fills the viewport with a gradient, behind everything else.
///
/// The colours come from a sampler rather than a list of stops: how a gradient
/// is described is the caller's business, and keeping that out of here leaves
/// this crate free of any settings type.
///
/// It is emitted straight in normalized device coordinates, so no camera is
/// involved — the background does not move when the view turns.
pub fn push_background(
    out: &mut Vec<Vertex>,
    shape: BackgroundShape,
    sample: impl Fn(f32) -> [f32; 4],
) {
    let corner = |x: f32, y: f32, color: [f32; 4]| Vertex::solid(Vec3::new(x, y, 0.0), color);

    match shape {
        BackgroundShape::Flat => {
            let color = sample(0.0);
            for [x, y] in [
                [-1.0, -1.0],
                [1.0, -1.0],
                [1.0, 1.0],
                [-1.0, -1.0],
                [1.0, 1.0],
                [-1.0, 1.0],
            ] {
                out.push(corner(x, y, color));
            }
        }
        BackgroundShape::Linear { angle_degrees } => {
            let angle = angle_degrees.to_radians();
            let along = glam::Vec2::new(angle.sin(), angle.cos());
            let across = glam::Vec2::new(-along.y, along.x);
            // Far enough to cover the screen whatever the angle: the corners of
            // a 2×2 square are √2 from its centre.
            let reach = std::f32::consts::SQRT_2;

            for step in 0..GRADIENT_STEPS {
                let (low, high) = (
                    step as f32 / GRADIENT_STEPS as f32,
                    (step + 1) as f32 / GRADIENT_STEPS as f32,
                );
                let place = |t: f32, side: f32| {
                    let position = along * (t * 2.0 - 1.0) * reach + across * side * reach;
                    (position.x, position.y)
                };
                let (a, b) = (sample(low), sample(high));
                let (near_left, near_right) = (place(low, -1.0), place(low, 1.0));
                let (far_left, far_right) = (place(high, -1.0), place(high, 1.0));

                for (position, color) in [
                    (near_left, a),
                    (near_right, a),
                    (far_right, b),
                    (near_left, a),
                    (far_right, b),
                    (far_left, b),
                ] {
                    out.push(corner(position.0, position.1, color));
                }
            }
        }
        BackgroundShape::Radial { center, radius } => {
            let middle = glam::Vec2::new(center[0] * 2.0 - 1.0, center[1] * 2.0 - 1.0);
            // Beyond the far corner the last colour simply carries on, so the
            // reach only has to be enough to leave no gap.
            let reach = radius.max(1e-3) * 2.0 * std::f32::consts::SQRT_2;
            const SECTORS: usize = 48;

            for ring in 0..GRADIENT_STEPS {
                let (low, high) = (
                    ring as f32 / GRADIENT_STEPS as f32,
                    (ring + 1) as f32 / GRADIENT_STEPS as f32,
                );
                let (inner, outer) = (sample(low), sample(high));

                for sector in 0..SECTORS {
                    let angle =
                        |index: usize| std::f32::consts::TAU * index as f32 / SECTORS as f32;
                    let at = |t: f32, index: usize| {
                        let direction = glam::Vec2::from_angle(angle(index));
                        let point = middle + direction * t * reach;
                        (point.x, point.y)
                    };

                    for (position, color) in [
                        (at(low, sector), inner),
                        (at(high, sector), outer),
                        (at(high, sector + 1), outer),
                        (at(low, sector), inner),
                        (at(high, sector + 1), outer),
                        (at(low, sector + 1), inner),
                    ] {
                        out.push(corner(position.0, position.1, color));
                    }
                }
            }
        }
    }
}

/// Emits a solid's triangles, each shaded from the way it faces.
///
/// The shade is worked out here, once per face, and baked into the colour: a
/// part made of flat faces reads better flat-shaded than smoothed, and it saves
/// carrying a normal all the way through the vertex format for geometry that
/// has no curves to smooth.
///
/// The light follows the camera rather than sitting in the world: a face turned
/// towards the viewer is always the bright one, so turning the part never
/// leaves it staring at an unlit side.
pub fn push_solid(out: &mut Vec<Vertex>, triangles: &[[Vec3; 3]], color: [f32; 4], towards: Vec3) {
    let light = (towards.normalize_or(Vec3::NEG_Z) * -1.0 + Vec3::new(0.35, 0.2, 0.55))
        .normalize_or(Vec3::Z);

    for [a, b, c] in triangles {
        let normal = (*b - *a).cross(*c - *a).normalize_or_zero();
        // Ambient light keeps a face turned away readable instead of black.
        let shade = 0.42 + 0.58 * normal.dot(light).max(0.0);
        let shaded = [
            color[0] * shade,
            color[1] * shade,
            color[2] * shade,
            color[3],
        ];
        for corner in [a, b, c] {
            out.push(Vertex::solid(*corner, shaded));
        }
    }
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

/// The two axes a drawing is measured from, through the origin its own plane
/// was given.
///
/// [`push_axes`] draws the world's three through the world origin. On a face
/// of the part a sketch is counted from a corner instead, where those no
/// longer cross — and a drawing with nothing at its origin has nothing to read
/// its sizes against.
pub fn push_plane_axes(
    out: &mut Vec<Vertex>,
    origin: Vec3,
    u: Vec3,
    v: Vec3,
    half_length: f32,
    style: &AxisStyle,
) {
    for (axis, color) in [(u, style.x), (v, style.y)] {
        out.push(Vertex::line(
            origin - axis * half_length,
            color,
            style.width,
        ));
        out.push(Vertex::line(
            origin + axis * half_length,
            color,
            style.width,
        ));
    }
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
mod tests;
