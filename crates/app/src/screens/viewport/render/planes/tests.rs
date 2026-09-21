//! What app · screens/viewport/render/planes.rs is held to.

use cao_part::PartDocument;
use chrono::Utc;
use glam::Vec3;

use super::*;
use crate::lang::Catalogue;
use crate::screens::extrusion::ExtrusionState;
use crate::screens::sketch::SketchEditor;

/// What the three planes are painted, when one of them is being pointed at.
fn offered(hovered: Option<PlaneChoice>) -> (Vec<cao_render::Vertex>, Vec<cao_render::Vertex>) {
    let state = ViewportState::default();
    let mut document = PartDocument::new("part", Utc::now());
    let mut editor = SketchEditor::default();
    let mut extrusion = ExtrusionState::default();
    let lang = Catalogue::french();
    editor.hovered_plane = hovered;

    let context = SketchContext {
        document: &mut document,
        editor: &mut editor,
        extrusion: &mut extrusion,
        lang: &lang,
    };

    let (mut surfaces, mut lines) = (Vec::new(), Vec::new());
    push_choosable_planes(&mut surfaces, &mut lines, &state, &context);
    (surfaces, lines)
}

/// The corners each plane's quad is drawn at, one group per plane, in the
/// order [`WorkPlane::ORIGIN_PLANES`] gives them.
fn per_plane(painted: &[cao_render::Vertex]) -> Vec<&[cao_render::Vertex]> {
    painted
        .chunks(painted.len() / WorkPlane::ORIGIN_PLANES.len())
        .collect()
}

#[test]
fn every_plane_a_sketch_can_start_on_is_offered() {
    let (surfaces, lines) = offered(None);

    assert!(
        !surfaces.is_empty() && !lines.is_empty(),
        "a plane is drawn neither as a surface to click nor as an outline to see",
    );
    for painted in [&surfaces, &lines] {
        assert_eq!(
            painted.len() % WorkPlane::ORIGIN_PLANES.len(),
            0,
            "the three planes are not drawn alike",
        );
    }
    for (index, plane) in WorkPlane::ORIGIN_PLANES.iter().enumerate() {
        let normal = plane.normal().as_vec3();
        for vertex in per_plane(&surfaces)[index] {
            let off = (Vec3::from(vertex.position) - plane.origin.as_vec3()).dot(normal);
            assert!(
                off.abs() < 1e-5,
                "a corner of the plane at {index} stands {off} away from the plane itself",
            );
        }
    }
}

#[test]
fn the_plane_under_the_cursor_is_drawn_apart_from_the_other_two() {
    let (plain, plain_lines) = offered(None);
    let (hovered, hovered_lines) = offered(Some(PlaneChoice::Origin(0)));

    assert_eq!(
        hovered.len(),
        plain.len(),
        "pointing at a plane changed how many are drawn",
    );
    assert_ne!(
        per_plane(&hovered)[0][0].color,
        per_plane(&plain)[0][0].color,
        "the plane under the cursor is filled exactly like the ones that are not",
    );
    assert_eq!(
        per_plane(&hovered)[1][0].color,
        per_plane(&plain)[1][0].color,
        "pointing at one plane changed the look of another",
    );
    assert!(
        per_plane(&hovered_lines)[0][0].width > per_plane(&plain_lines)[0][0].width,
        "the plane under the cursor is outlined no more strongly than the others",
    );
}

#[test]
fn a_part_with_no_face_at_all_is_lit_up_on_none() {
    let mut surfaces = Vec::new();
    let state = ViewportState::default();
    let mut document = PartDocument::new("part", Utc::now());
    let mut editor = SketchEditor::default();
    let mut extrusion = ExtrusionState::default();
    let lang = Catalogue::french();
    let context = SketchContext {
        document: &mut document,
        editor: &mut editor,
        extrusion: &mut extrusion,
        lang: &lang,
    };

    push_hovered_face(&mut surfaces, tint(state.theme.refused), &context, 0);

    assert!(
        surfaces.is_empty(),
        "a part with no face at all was lit up on one",
    );
}
