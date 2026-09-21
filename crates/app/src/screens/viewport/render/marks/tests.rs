//! What app · screens/viewport/render/marks.rs is held to.

use cao_part::PartDocument;
use cao_prefs::config::ViewportConfig;
use cao_render::camera::OrbitCamera;
use cao_sketch::WorkPlane;
use chrono::Utc;
use glam::{DVec3, Vec3};

use super::*;
use crate::lang::Catalogue;
use crate::screens::extrusion::ExtrusionState;
use crate::screens::sketch::SketchEditor;

/// A vertex holds its position in `f32`, so a place read back off one has lost
/// the tail of its `f64`. This is several orders of magnitude above that loss
/// and several below the size of the smallest mark drawn.
const TOLERANCE: f64 = 1e-5;

/// Neither through the world origin nor square to any axis, so a mark that
/// forgot the plane it is drawn on has nowhere to hide.
fn a_face_of_the_part() -> WorkPlane {
    WorkPlane {
        origin: DVec3::new(3.0, 4.0, 5.0),
        u: DVec3::new(1.0, 1.0, 0.0).normalize(),
        v: DVec3::new(-1.0, 1.0, 2.0).normalize(),
    }
}

fn a_view() -> ViewScale {
    ViewScale::of(
        &OrbitCamera::default(),
        egui::Rect::from_min_size(egui::Pos2::ZERO, egui::vec2(1280.0, 800.0)),
        1.0,
        &ViewportConfig::default(),
        1.0,
    )
}

/// Where each vertex lands, read back in the coordinates of the drawing.
fn places(painted: &[cao_render::Vertex], sketch: &Sketch) -> Vec<DVec2> {
    painted
        .iter()
        .map(|vertex| {
            sketch
                .plane
                .to_local(Vec3::from(vertex.position).as_dvec3())
        })
        .collect()
}

/// The distinct places a run of vertices visits, in the order first seen.
fn corners(painted: &[cao_render::Vertex], sketch: &Sketch) -> Vec<DVec2> {
    let mut seen: Vec<DVec2> = Vec::new();
    for place in places(painted, sketch) {
        if !seen.iter().any(|kept| kept.distance(place) < TOLERANCE) {
            seen.push(place);
        }
    }
    seen
}

fn lies_on_the_plane(painted: &[cao_render::Vertex], sketch: &Sketch) {
    let normal = sketch.plane.normal();
    for vertex in painted {
        let off = (Vec3::from(vertex.position).as_dvec3() - sketch.plane.origin).dot(normal);
        assert!(
            off.abs() < TOLERANCE,
            "a mark at {:?} stands {off} away from the plane it is drawn on",
            vertex.position,
        );
    }
}

#[test]
fn a_point_is_marked_by_a_square_of_the_size_it_was_given() {
    let sketch = Sketch::new(a_face_of_the_part());
    let at = DVec2::new(12.0, -7.0);
    let size = 0.5;

    let mut painted = Vec::new();
    push_point_marker(&mut painted, &sketch, at, size, [1.0; 4], 1.5);

    lies_on_the_plane(&painted, &sketch);
    let corners = corners(&painted, &sketch);
    assert_eq!(corners.len(), 4, "a square has four corners: {corners:?}");
    for corner in corners {
        let offset = corner - at;
        assert!(
            (offset.x.abs() - size).abs() < TOLERANCE && (offset.y.abs() - size).abs() < TOLERANCE,
            "{corner:?} is not a corner of the square of half-size {size} around {at:?}",
        );
    }
}

#[test]
fn the_middle_of_a_line_is_marked_by_a_triangle_a_point_could_not_be_mistaken_for() {
    let sketch = Sketch::new(a_face_of_the_part());
    let at = DVec2::new(4.0, 4.0);

    let mut painted = Vec::new();
    push_midpoint_mark(&mut painted, &sketch, at, a_view(), [1.0; 4]);

    lies_on_the_plane(&painted, &sketch);
    assert_eq!(
        corners(&painted, &sketch).len(),
        3,
        "the mark for a midpoint has to differ from a point by its shape, not by its size",
    );
}

#[test]
fn a_right_angle_is_marked_inside_the_two_arms_that_make_it() {
    let sketch = Sketch::new(a_face_of_the_part());
    let corner = DVec2::new(2.0, 3.0);

    let mut painted = Vec::new();
    push_square_mark(
        &mut painted,
        &sketch,
        corner,
        DVec2::X,
        DVec2::Y,
        a_view(),
        [1.0; 4],
    );

    lies_on_the_plane(&painted, &sketch);
    for place in places(&painted, &sketch) {
        let offset = place - corner;
        assert!(
            offset.x > -TOLERANCE && offset.y > -TOLERANCE,
            "{place:?} falls outside the corner, on the wrong side of an arm",
        );
    }
}

#[test]
fn a_corner_whose_arms_go_nowhere_is_marked_by_nothing() {
    let sketch = Sketch::new(a_face_of_the_part());

    let mut painted = Vec::new();
    push_square_mark(
        &mut painted,
        &sketch,
        DVec2::ZERO,
        DVec2::ZERO,
        DVec2::Y,
        a_view(),
        [1.0; 4],
    );

    assert!(
        painted.is_empty(),
        "an arm of no length has no inside, and a mark was drawn on it anyway",
    );
}

#[test]
fn the_drawings_own_origin_is_marked_by_a_diamond_and_the_others_by_squares() {
    let sketch_plane = a_face_of_the_part();
    let mut sketch = Sketch::new(sketch_plane);
    let ordinary = sketch.add_point(DVec2::new(10.0, 0.0));

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

    let mut painted = Vec::new();
    push_point_markers(
        &mut painted,
        &sketch,
        &Theme::default(),
        a_view(),
        &[],
        &[],
        &context,
    );

    lies_on_the_plane(&painted, &sketch);
    let corners = corners(&painted, &sketch);
    let (origin, ordinary) = (
        corners_around(&corners, sketch.point(Sketch::ORIGIN)),
        corners_around(&corners, sketch.point(ordinary)),
    );

    assert_eq!(origin.len(), 4, "the origin is drawn a four-cornered mark");
    assert_eq!(ordinary.len(), 4, "a point is drawn a four-cornered mark");
    assert!(
        origin
            .iter()
            .all(|offset| offset.x.abs() < TOLERANCE || offset.y.abs() < TOLERANCE),
        "the origin's corners sit square rather than turned a quarter: {origin:?}",
    );
    assert!(
        ordinary
            .iter()
            .all(|offset| offset.x.abs() > TOLERANCE && offset.y.abs() > TOLERANCE),
        "an ordinary point is turned like the origin: {ordinary:?}",
    );
    let reach = |offsets: &[DVec2]| {
        offsets
            .iter()
            .map(|offset| offset.length())
            .fold(0.0, f64::max)
    };
    assert!(
        reach(&origin) > reach(&ordinary),
        "the origin is drawn no larger than an ordinary point",
    );
}

/// The corners nearer to `place` than to anything else drawn, as offsets from
/// it.
fn corners_around(corners: &[DVec2], place: DVec2) -> Vec<DVec2> {
    corners
        .iter()
        .map(|corner| *corner - place)
        .filter(|offset| offset.length() < 5.0)
        .collect()
}
