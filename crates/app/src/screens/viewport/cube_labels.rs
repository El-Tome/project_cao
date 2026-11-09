//! The names on the orientation cube's faces: the word laid out flat, then
//! carried onto its face.
//!
//! The cube is orthographic, so a face projects by an affine map: the corners
//! of each glyph go through it and the result is still that glyph, turned and
//! foreshortened as the face is. The word is sized in the face's own square,
//! which is how the view stays out of it.

use cao_render::CubeFace;
use cao_render::cube;
use glam::Vec3;

/// The smallest a label is still drawn at; below this it is dropped instead.
pub(super) const MINIMUM_READABLE_FONT: f32 = 8.0;

const REFERENCE_FONT_SIZE: f32 = 64.0;

/// The share of its area a face has to show for a name on it to be read.
const LEAST_SHARE_SEEN: f32 = 0.25;

/// Which way a name on `face` reads and which way is up on it, in cube space.
/// The first crossed with the second gives the face's outward normal, so that
/// no name is ever mirrored from outside; a side face reads with +Z up.
fn reading_axes(face: CubeFace) -> (Vec3, Vec3) {
    match face {
        CubeFace::PlusX => (Vec3::Y, Vec3::Z),
        CubeFace::MinusX => (Vec3::NEG_Y, Vec3::Z),
        CubeFace::PlusY => (Vec3::NEG_X, Vec3::Z),
        CubeFace::MinusY => (Vec3::X, Vec3::Z),
        CubeFace::PlusZ => (Vec3::X, Vec3::Y),
        CubeFace::MinusZ => (Vec3::X, Vec3::NEG_Y),
    }
}

/// Where a point of cube space lands on the cube's square.
pub(super) fn on_the_square(
    view_projection: glam::Mat4,
    cube_rect: egui::Rect,
    point: Vec3,
) -> egui::Pos2 {
    let clip = view_projection * point.extend(1.0);
    egui::pos2(
        cube_rect.center().x + clip.x / clip.w * cube_rect.width() * 0.5,
        cube_rect.center().y - clip.y / clip.w * cube_rect.height() * 0.5,
    )
}

/// How many points of the square a unit of cube space covers, square to the
/// view.
pub(super) fn points_per_unit(cube_rect: egui::Rect) -> f32 {
    cube_rect.width() * 0.5 / cube::ORTHO_HALF_SIZE
}

/// `text` as a shape lying on `face`, in `ink` — `None` when the face is seen
/// too edge-on to read or nothing readable fits, in which case it is better
/// left bare.
pub(super) fn face_label(
    painter: &egui::Painter,
    view_projection: glam::Mat4,
    cube_rect: egui::Rect,
    face: CubeFace,
    text: &str,
    preferred: f32,
    ink: egui::Color32,
) -> Option<egui::Shape> {
    let project = |point: Vec3| on_the_square(view_projection, cube_rect, point);
    let points_per_unit = points_per_unit(cube_rect);
    let middle = cube::face_center(face);
    let (along, up) = reading_axes(face);
    let centre = project(middle);
    let along = (project(middle + along) - centre) / points_per_unit;
    let up = (project(middle + up) - centre) / points_per_unit;
    if (along.x * up.y - along.y * up.x).abs() < LEAST_SHARE_SEEN {
        return None;
    }

    let half_side = middle.length() * points_per_unit;
    let half_extent = painter
        .layout_no_wrap(
            text.to_string(),
            egui::FontId::proportional(REFERENCE_FONT_SIZE),
            ink,
        )
        .size()
        * 0.5;
    let font_size = fitting_font_size(
        half_side,
        half_extent,
        REFERENCE_FONT_SIZE,
        preferred,
        MINIMUM_READABLE_FONT,
    )?;

    let galley =
        painter.layout_no_wrap(text.to_string(), egui::FontId::proportional(font_size), ink);
    let word_middle = galley.mesh_bounds.center();
    let atlas = painter.fonts(|fonts| fonts.font_image_size());
    painter
        .ctx()
        .data_mut(|data| data.insert_temp(atlas_the_names_used(), atlas));
    let texels = egui::vec2(atlas[0] as f32, atlas[1] as f32);

    let placed = galley.rows.first()?;
    let mut laid = placed.row.visuals.mesh.clone();
    for vertex in &mut laid.vertices {
        let from_middle = vertex.pos + placed.pos.to_vec2() - word_middle;
        vertex.pos = centre + along * from_middle.x - up * from_middle.y;
        vertex.uv = (vertex.uv.to_vec2() / texels).to_pos2();
    }
    Some(egui::Shape::mesh(laid))
}

fn atlas_the_names_used() -> egui::Id {
    egui::Id::new("cube_labels_atlas")
}

/// A name's texture coordinates are divided by the font atlas's size as it is
/// painted. A glyph first laid out later in the same frame can grow the atlas,
/// which leaves the names sampling the wrong texels, and egui asks for no
/// repaint on its own: called once everything is laid out, this asks for one.
pub(crate) fn repaint_if_the_atlas_grew(ctx: &egui::Context) {
    let used = ctx.data_mut(|data| data.remove_temp::<[usize; 2]>(atlas_the_names_used()));
    if used.is_some_and(|used| used != ctx.fonts(|fonts| fonts.font_image_size())) {
        ctx.request_repaint();
    }
}

/// The largest font size, at or below `preferred`, whose text — measured at
/// `reference_size` giving `half_extent_at_reference` — stays inside a face's
/// own square of half-side `half_side`. `None` when even `minimum_readable`
/// would not fit, which is when the label is better dropped than left spilling
/// onto a neighbouring face.
fn fitting_font_size(
    half_side: f32,
    half_extent_at_reference: egui::Vec2,
    reference_size: f32,
    preferred: f32,
    minimum_readable: f32,
) -> Option<f32> {
    let scale = inscribed_scale(half_side, half_extent_at_reference);
    let size = (reference_size * scale * FIT_MARGIN).min(preferred);
    (size >= minimum_readable).then_some(size)
}

/// Room left between the fitted text and the face's own edge.
const FIT_MARGIN: f32 = 0.85;

/// The largest `t` for which a rectangle of half-extents `t * corner` stays
/// inside a square of half-side `half_side`, around the same centre.
fn inscribed_scale(half_side: f32, corner: egui::Vec2) -> f32 {
    half_side / corner.max_elem()
}

#[cfg(test)]
mod tests;
