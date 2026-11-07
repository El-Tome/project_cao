//! What app · ui/icon.rs is held to.
//!
//! Closes #725.
//! - the SVGs are drawn by `egui_extras` with its `svg` feature (`resvg`), at
//!   22, 15 and 14 px — `each_size_draws_an_icon_at_the_points_the_redesign_gives_it`
//! - crisp on a Retina screen —
//!   `on_a_retina_screen_an_icon_is_drawn_from_as_many_pixels_as_it_covers`
//!
//! Crisp is read off the picture the frame hands the GPU: an icon drawn from
//! as many pixels as it covers on the screen is sharp, and one drawn from half
//! of them is stretched to twice its size, which is the blur a Retina screen
//! shows. No pixel is painted here, but the picture one would paint from is in
//! the frame's output all the same.

use std::sync::Arc;

use super::*;

/// The three sizes the redesign draws an icon at, in points:
/// `docs/redesign/icons.md`.
const SIZES: [(IconSize, f32); 3] = [
    (IconSize::Large, 22.0),
    (IconSize::Small, 15.0),
    (IconSize::Row, 14.0),
];

/// How far a painted edge may sit from where it was asked for, in points.
const TOLERANCE: f32 = 1e-3;

#[test]
fn each_size_draws_an_icon_at_the_points_the_redesign_gives_it() {
    for (size, points) in SIZES {
        let painted = painted_alone(LINE, size, 1.0);

        assert!(
            (painted.rect.width() - points).abs() < TOLERANCE
                && (painted.rect.height() - points).abs() < TOLERANCE,
            "an icon of size {size:?} covers {:?} points, the redesign draws it at {points} × {points}",
            painted.rect.size(),
        );
    }
}

#[test]
fn on_a_retina_screen_an_icon_is_drawn_from_as_many_pixels_as_it_covers() {
    for pixels_per_point in [1.0, 2.0] {
        for (size, points) in SIZES {
            let painted = painted_alone(LINE, size, pixels_per_point);
            let covered = (points * pixels_per_point).round() as usize;

            assert!(
                (painted.rect.width() - points).abs() < TOLERANCE,
                "at {pixels_per_point} pixels a point, an icon of size {size:?} covers {} points \
                 instead of {points}",
                painted.rect.width(),
            );
            assert_eq!(
                painted.picture.size,
                [covered, covered],
                "at {pixels_per_point} pixels a point, an icon of size {size:?} covers \
                 {covered} × {covered} pixels and is drawn from a picture of {:?}: \
                 scaled to fit them, it blurs",
                painted.picture.size,
            );
        }
    }
}

/// What one frame painted of an icon drawn on its own: the rectangle it
/// covers, in points, and the picture it was painted from.
struct Painted {
    rect: egui::Rect,
    picture: Arc<egui::ColorImage>,
}

fn painted_alone(icon: Icon, size: IconSize, pixels_per_point: f32) -> Painted {
    let ctx = egui::Context::default();
    install(&ctx);

    let mut input = egui::RawInput {
        screen_rect: Some(egui::Rect::from_min_size(
            egui::Pos2::ZERO,
            egui::vec2(200.0, 200.0),
        )),
        ..Default::default()
    };
    input
        .viewports
        .entry(egui::ViewportId::ROOT)
        .or_default()
        .native_pixels_per_point = Some(pixels_per_point);

    let mut output = ctx.run_ui(input, |ui| {
        ui.add(icon.image(size));
    });
    let mut pictures = std::mem::take(&mut output.textures_delta.set);
    output.textures_delta.clear();

    let shapes = every_shape(output.shapes);
    let mut pictured = shapes
        .iter()
        .filter(|shape| shape.texture_id() != egui::TextureId::default());
    let shape = pictured
        .next()
        .unwrap_or_else(|| panic!("an icon of size {size:?} painted no picture at all"));
    assert!(
        pictured.next().is_none(),
        "an icon of size {size:?} painted more than one picture",
    );
    let egui::Shape::Rect(drawn) = shape else {
        panic!(
            "an icon of size {size:?} was painted other than as the textured rectangle \
             `egui::Image` paints, which is the shape this test reads"
        );
    };

    let picture = pictures
        .remove(&drawn.fill_texture_id())
        .into_iter()
        .flatten()
        .find(|delta| delta.pos.is_none())
        .map(|delta| {
            let egui::ImageData::Color(image) = delta.image;
            image
        })
        .unwrap_or_else(|| {
            panic!("an icon of size {size:?} painted a picture the frame never handed over")
        });

    Painted {
        rect: drawn.rect,
        picture,
    }
}

/// The shapes a frame painted, with every group opened.
fn every_shape(clipped: Vec<egui::epaint::ClippedShape>) -> Vec<egui::Shape> {
    let mut pending: Vec<egui::Shape> = clipped.into_iter().map(|clipped| clipped.shape).collect();
    let mut shapes = Vec::new();
    while let Some(shape) = pending.pop() {
        match shape {
            egui::Shape::Vec(group) => pending.extend(group),
            shape => shapes.push(shape),
        }
    }
    shapes
}
