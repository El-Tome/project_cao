//! What app · screens/viewport/render/preview.rs is held to.

use cao_part::PartDocument;
use cao_prefs::config::ViewportConfig;
use cao_render::camera::OrbitCamera;
use cao_sketch::{ToolState, WorkPlane};
use chrono::Utc;
use glam::Vec3;

use super::*;
use crate::lang::Catalogue;
use crate::screens::extrusion::ExtrusionState;
use crate::screens::sketch::SketchEditor;

/// A vertex holds its position in `f32`, so a place read back off one has lost
/// the tail of its `f64`. Well under the width of the marks drawn here.
const TOLERANCE: f64 = 1e-5;

/// One straight step of what was painted, in the coordinates of the drawing.
#[derive(Clone, Copy, Debug)]
struct Step {
    from: DVec2,
    to: DVec2,
}

fn steps(painted: &[cao_render::Vertex], sketch: &Sketch) -> Vec<Step> {
    painted
        .as_chunks::<2>()
        .0
        .iter()
        .map(|[from, to]| Step {
            from: place(*from, sketch),
            to: place(*to, sketch),
        })
        .collect()
}

fn place(vertex: cao_render::Vertex, sketch: &Sketch) -> DVec2 {
    sketch
        .plane
        .to_local(Vec3::from(vertex.position).as_dvec3())
}

/// Whether anything painted runs the whole way from one place to another. A
/// dashed line comes back as several steps, a plain one as a single step, so
/// what is asked is that both ends were reached along the way.
fn runs_between(steps: &[Step], from: DVec2, to: DVec2) -> bool {
    let span = to - from;
    let length = span.length();
    if length < TOLERANCE {
        return false;
    }
    let direction = span / length;
    let on_the_way = |place: DVec2| {
        let offset = place - from;
        let reach = offset.dot(direction);
        (offset - direction * reach).length() < TOLERANCE
            && (-TOLERANCE..=length + TOLERANCE).contains(&reach)
    };
    let reached = |at: DVec2| {
        steps
            .iter()
            .any(|step| step.from.distance(at) < TOLERANCE || step.to.distance(at) < TOLERANCE)
    };
    steps
        .iter()
        .all(|step| on_the_way(step.from) && on_the_way(step.to))
        && reached(from)
        && reached(to)
}

/// One frame of what a tool is offering, read back as the straight steps it is
/// made of.
fn a_promise(tool: Tool, state: ToolState, cursor: Option<DVec2>) -> (Vec<Step>, usize) {
    let mut document = PartDocument::new("part", Utc::now());
    let mut editor = SketchEditor::default();
    let mut extrusion = ExtrusionState::default();
    let lang = Catalogue::french();
    editor.tool = tool;
    editor.tool_state = state;
    editor.cursor = cursor;

    let millimetres = document.scale();
    let context = SketchContext {
        document: &mut document,
        editor: &mut editor,
        extrusion: &mut extrusion,
        lang: &lang,
    };
    let scale = ViewScale::of(
        &OrbitCamera::default(),
        egui::Rect::from_min_size(egui::Pos2::ZERO, egui::vec2(1280.0, 800.0)),
        1.0,
        &ViewportConfig::default(),
        millimetres,
    );

    let sketch = Sketch::new(WorkPlane::XY);
    let mut out = Vec::new();
    push_preview(&mut out, &sketch, &Theme::default(), scale, &context);
    (steps(&out, &sketch), out.len())
}

#[test]
fn a_line_in_progress_shows_the_stretch_the_next_click_would_lay() {
    let (from, cursor) = (DVec2::new(2.0, 1.0), DVec2::new(9.0, 5.0));

    let (painted, _) = a_promise(
        Tool::Line,
        ToolState::Line {
            anchor: ChainAnchor::Pending(from),
            previous: None,
        },
        Some(cursor),
    );

    assert!(
        runs_between(&painted, from, cursor),
        "nothing runs from where the line carries on to where the cursor is: {painted:?}",
    );
}

#[test]
fn nothing_is_promised_while_the_cursor_is_off_the_canvas() {
    let (_, vertices) = a_promise(
        Tool::Line,
        ToolState::Line {
            anchor: ChainAnchor::Pending(DVec2::ZERO),
            previous: None,
        },
        None,
    );

    assert_eq!(
        vertices, 0,
        "a promise was painted for a cursor that is nowhere",
    );
}

#[test]
fn a_rectangle_shows_all_four_of_its_sides_before_the_second_click() {
    let (start, cursor) = (DVec2::new(1.0, 2.0), DVec2::new(7.0, 6.0));
    let corners = [
        start,
        DVec2::new(cursor.x, start.y),
        cursor,
        DVec2::new(start.x, cursor.y),
    ];

    let (painted, _) = a_promise(
        Tool::Rectangle,
        ToolState::Rectangle { start },
        Some(cursor),
    );

    for index in 0..4 {
        let (from, to) = (corners[index], corners[(index + 1) % 4]);
        let side: Vec<Step> = painted
            .iter()
            .copied()
            .filter(|step| {
                let span = to - from;
                let direction = span / span.length();
                [step.from, step.to].into_iter().all(|place| {
                    let offset = place - from;
                    (offset - direction * offset.dot(direction)).length() < TOLERANCE
                })
            })
            .collect();
        assert!(
            runs_between(&side, from, to),
            "the side from {from:?} to {to:?} is missing from the promise",
        );
    }
}

#[test]
fn the_point_tool_shows_where_the_point_would_land() {
    let cursor = DVec2::new(4.0, -3.0);

    let (painted, _) = a_promise(Tool::Point, ToolState::None, Some(cursor));

    assert!(
        !painted.is_empty(),
        "placing a point shows nothing at all until the click",
    );
    let corners: Vec<DVec2> = painted.iter().map(|step| step.from).collect();
    let middle = corners.iter().sum::<DVec2>() / corners.len() as f64;
    assert!(
        middle.distance(cursor) < TOLERANCE,
        "the mark is drawn around {middle:?} rather than around the cursor at {cursor:?}",
    );
}
