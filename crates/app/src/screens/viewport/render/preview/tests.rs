//! What app · screens/viewport/render/preview.rs is held to.
//!
//! Closes #484.
//! - the preview before the click shows the angle the click lays: the quarter
//!   under the cursor, about the middle of the trait —
//!   `an_angle_to_an_axis_is_previewed_in_the_quarter_under_the_cursor`
//!
//! Closes #485.
//! - the preview before the click shows the angle the click lays: at a corner,
//!   the quarter under the cursor —
//!   `an_angle_at_a_corner_is_previewed_in_the_quarter_under_the_cursor`
//!
//! Closes #567, with `input/measure/tests.rs`.
//! - the preview before the click shows which of the two the click does: in
//!   another quarter, the angle of that quarter, not yet on the drawing; in
//!   the driving angle's own, that angle —
//!   `the_quarter_shown_before_the_click_is_the_read_only_one_it_would_lay`;
//!   and what it hands the painter says whether the click lays a readout or
//!   opens the driving angle —
//!   `the_preview_says_whether_the_click_lays_a_readout_or_opens_the_driving_angle`.
//!   The colour that flag is painted in gets no test: it is chosen inside
//!   `paint_dimension_labels`, which needs an `egui::Ui`
//! - decided in review: the preview and the click read "read-only" from one
//!   question to the document, so what the preview says is what the click
//!   records — in the quarter beside the driving angle, the one opposite, the
//!   driving one's own, on a pair with no angle yet, and on a length nothing
//!   is left to set —
//!   `what_the_preview_calls_read_only_is_what_the_click_records`

use cao_part::{Operation, PartDocument, PointRef};
use cao_prefs::config::ViewportConfig;
use cao_render::camera::OrbitCamera;
use cao_sketch::{DimensionTarget, SegmentId, SketchAxis, ToolState, WorkPlane};
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

#[test]
fn an_angle_to_an_axis_is_previewed_in_the_quarter_under_the_cursor() {
    let (acute, obtuse) = (78.690_067_525_979_8, 101.309_932_474_020_2);
    for (cursor, expected) in [
        (DVec2::new(40.0, 80.0), obtuse),
        (DVec2::new(100.0, 80.0), acute),
        (DVec2::new(40.0, 0.0), acute),
        (DVec2::new(100.0, 0.0), obtuse),
    ] {
        let mut document = PartDocument::new("part", Utc::now());
        document.apply(Operation::CreateSketch {
            plane: WorkPlane::XY,
            on: None,
        });
        document.apply(Operation::AddSegment {
            sketch: 0,
            start: PointRef::New(DVec2::new(20.0, 30.0)),
            end: PointRef::New(DVec2::new(120.0, 50.0)),
            construction: false,
        });
        let chosen = document.sketches()[0].angle_to_axis(SegmentId(0), SketchAxis::V);
        let mut editor = SketchEditor::default();
        let mut extrusion = ExtrusionState::default();
        let lang = Catalogue::french();
        editor.tool = Tool::Dimension;
        editor.tool_state = ToolState::Dimension {
            placing: Some(chosen),
            picks: Default::default(),
        };
        let context = SketchContext {
            document: &mut document,
            editor: &mut editor,
            extrusion: &mut extrusion,
            lang: &lang,
        };

        let shown = pending_annotation(&context, 0, cursor, 1.0, 0.05)
            .expect("an angle is shown")
            .target;

        let reads = context.document.measured(0, shown).expect("it reads");
        assert!(
            (reads - expected).abs() < 1e-6,
            "under {cursor:?} the preview shows {reads}°, not {expected}°",
        );
    }
}

#[test]
fn an_angle_at_a_corner_is_previewed_in_the_quarter_under_the_cursor() {
    let acute = 50.0_f64.atan2(40.0).to_degrees();
    let corner = DVec2::new(60.0, 20.0);
    for (offset, expected) in [
        (DVec2::new(-20.0, 30.0), 180.0 - acute),
        (DVec2::new(40.0, 20.0), acute),
        (DVec2::new(20.0, -30.0), 180.0 - acute),
        (DVec2::new(-30.0, -10.0), acute),
    ] {
        let mut document = PartDocument::new("part", Utc::now());
        document.apply(Operation::CreateSketch {
            plane: WorkPlane::XY,
            on: None,
        });
        document.apply(Operation::AddSegment {
            sketch: 0,
            start: PointRef::New(DVec2::new(0.0, 20.0)),
            end: PointRef::New(corner),
            construction: false,
        });
        let pivot = document.sketches()[0].segments()[0].end;
        document.apply(Operation::AddSegment {
            sketch: 0,
            start: PointRef::Existing(pivot),
            end: PointRef::New(DVec2::new(100.0, 70.0)),
            construction: false,
        });
        let chosen = document.sketches()[0].corner_angle(SegmentId(0), SegmentId(1));
        let mut editor = SketchEditor::default();
        let mut extrusion = ExtrusionState::default();
        let lang = Catalogue::french();
        editor.tool = Tool::Dimension;
        editor.tool_state = ToolState::Dimension {
            placing: Some(chosen),
            picks: Default::default(),
        };
        let context = SketchContext {
            document: &mut document,
            editor: &mut editor,
            extrusion: &mut extrusion,
            lang: &lang,
        };
        let cursor = corner + offset;

        let shown = pending_annotation(&context, 0, cursor, 1.0, 0.05)
            .expect("an angle is shown")
            .target;

        let reads = context.document.measured(0, shown).expect("it reads");
        assert!(
            (reads - expected).abs() < 1e-6,
            "under {cursor:?} the preview shows {reads}°, not {expected}°",
        );
    }
}

/// Two traits crossing at (50, 0) at 60°, the 60° between them driving, and
/// a place in that quarter and one in the 120° quarter beside it.
fn an_x_carrying_a_driving_sixty() -> (PartDocument, DimensionTarget, DVec2, DVec2) {
    let crossing = DVec2::new(50.0, 0.0);
    let degrees = |angle: f64| DVec2::from_angle(angle.to_radians());
    let mut document = PartDocument::new("part", Utc::now());
    document.apply(Operation::CreateSketch {
        plane: WorkPlane::XY,
        on: None,
    });
    for (start, end) in [
        (DVec2::ZERO, DVec2::new(100.0, 0.0)),
        (
            crossing - degrees(60.0) * 40.0,
            crossing + degrees(60.0) * 40.0,
        ),
    ] {
        document.apply(Operation::AddSegment {
            sketch: 0,
            start: PointRef::New(start),
            end: PointRef::New(end),
            construction: false,
        });
    }
    let (in_the_sixty, in_the_hundred_and_twenty) = (
        crossing + degrees(30.0) * 20.0,
        crossing + degrees(120.0) * 20.0,
    );
    let sketch = &document.sketches()[0];
    let driving = sketch.oriented(
        sketch.angle_between_traits(SegmentId(0), SegmentId(1)),
        in_the_sixty,
    );
    document.apply(Operation::SetDimension {
        sketch: 0,
        target: driving,
        value: 60.0.into(),
        placement: None,
    });
    (document, driving, in_the_sixty, in_the_hundred_and_twenty)
}

fn placing(target: DimensionTarget) -> SketchEditor {
    SketchEditor {
        tool: Tool::Dimension,
        tool_state: ToolState::Dimension {
            placing: Some(target),
            picks: Default::default(),
        },
        ..Default::default()
    }
}

#[test]
fn the_quarter_shown_before_the_click_is_the_read_only_one_it_would_lay() {
    let (mut document, driving, in_the_sixty, in_the_hundred_and_twenty) =
        an_x_carrying_a_driving_sixty();
    let mut editor = placing(driving);
    let mut extrusion = ExtrusionState::default();
    let lang = Catalogue::french();
    let context = SketchContext {
        document: &mut document,
        editor: &mut editor,
        extrusion: &mut extrusion,
        lang: &lang,
    };

    let shown = pending_annotation(&context, 0, in_the_hundred_and_twenty, 1.0, 0.05)
        .expect("an angle is shown")
        .target;
    let reads = context.document.measured(0, shown);
    assert!(
        shown != driving
            && context.document.sketches()[0].dimension_of(shown).is_none()
            && reads.is_some_and(|reads| (reads - 120.0).abs() < 1e-6),
        "in the 120° quarter, the preview shows that quarter's angle, not yet on the \
         drawing, got {shown:?} reading {reads:?}",
    );

    let shown = pending_annotation(&context, 0, in_the_sixty, 1.0, 0.05)
        .expect("an angle is shown")
        .target;
    assert_eq!(
        shown, driving,
        "in the 60° quarter, the preview shows the driving angle the click opens",
    );
}

#[test]
fn the_preview_says_whether_the_click_lays_a_readout_or_opens_the_driving_angle() {
    let (mut document, driving, in_the_sixty, in_the_hundred_and_twenty) =
        an_x_carrying_a_driving_sixty();
    let mut editor = placing(driving);
    let mut extrusion = ExtrusionState::default();
    let lang = Catalogue::french();
    let context = SketchContext {
        document: &mut document,
        editor: &mut editor,
        extrusion: &mut extrusion,
        lang: &lang,
    };

    let in_another_quarter = pending_annotation(&context, 0, in_the_hundred_and_twenty, 1.0, 0.05)
        .expect("an angle is shown");
    assert!(
        in_another_quarter.read_only,
        "in the 120° quarter the click lays a readout, and the preview says so",
    );

    let in_its_own_quarter =
        pending_annotation(&context, 0, in_the_sixty, 1.0, 0.05).expect("an angle is shown");
    assert!(
        !in_its_own_quarter.read_only,
        "in the 60° quarter the click opens the driving angle, and the preview says so",
    );
}

/// The X with nothing on it yet: the first angle laid on it drives.
fn an_x_with_no_angle() -> (PartDocument, DimensionTarget, DVec2) {
    let (mut document, driving, in_the_sixty, _) = an_x_carrying_a_driving_sixty();
    document.apply(Operation::EraseMany {
        sketch: 0,
        elements: vec![],
        dimensions: vec![driving],
        constraints: vec![],
    });
    (document, driving, in_the_sixty)
}

/// A right triangle from (10, 10): a base 40 along, a side 30 up, both with
/// their lengths and the right angle between them, so that nothing is left
/// for its hypotenuse's length to set.
fn a_settled_right_triangle() -> (PartDocument, DimensionTarget) {
    let mut document = PartDocument::new("part", Utc::now());
    document.apply(Operation::CreateSketch {
        plane: WorkPlane::XY,
        on: None,
    });
    document.apply(Operation::AddSegment {
        sketch: 0,
        start: PointRef::New(DVec2::new(10.0, 10.0)),
        end: PointRef::New(DVec2::new(50.0, 10.0)),
        construction: false,
    });
    let base = document.sketches()[0].segments()[0];
    document.apply(Operation::AddSegment {
        sketch: 0,
        start: PointRef::Existing(base.end),
        end: PointRef::New(DVec2::new(50.0, 40.0)),
        construction: false,
    });
    let top = document.sketches()[0].segments()[1].end;
    document.apply(Operation::AddSegment {
        sketch: 0,
        start: PointRef::Existing(top),
        end: PointRef::Existing(base.start),
        construction: false,
    });
    for (target, value) in [
        (DimensionTarget::Length(SegmentId(0)), 40.0),
        (DimensionTarget::Length(SegmentId(1)), 30.0),
        (DimensionTarget::corner(SegmentId(0), SegmentId(1)), 90.0),
    ] {
        document.apply(Operation::SetDimension {
            sketch: 0,
            target,
            value: value.into(),
            placement: None,
        });
    }
    (document, DimensionTarget::Length(SegmentId(2)))
}

#[test]
fn what_the_preview_calls_read_only_is_what_the_click_records() {
    let crossing = DVec2::new(50.0, 0.0);
    let opposite_the_sixty = crossing + DVec2::from_angle(210.0_f64.to_radians()) * 20.0;
    let (x, driving, in_the_sixty, in_the_hundred_and_twenty) = an_x_carrying_a_driving_sixty();
    let (bare_x, first_angle, in_its_quarter) = an_x_with_no_angle();
    let (triangle, hypotenuse) = a_settled_right_triangle();
    for (mut document, placed, cursor, read_only, said) in [
        (
            x.clone(),
            driving,
            in_the_hundred_and_twenty,
            true,
            "the quarter beside the driving 60°",
        ),
        (
            x.clone(),
            driving,
            opposite_the_sixty,
            true,
            "the quarter opposite the driving 60°",
        ),
        (
            x,
            driving,
            in_the_sixty,
            false,
            "the driving 60°'s own quarter",
        ),
        (
            bare_x,
            first_angle,
            in_its_quarter,
            false,
            "a pair with no angle yet",
        ),
        (
            triangle,
            hypotenuse,
            DVec2::new(32.0, 23.0),
            true,
            "the hypotenuse of a settled triangle",
        ),
    ] {
        let mut editor = placing(placed);
        let mut extrusion = ExtrusionState::default();
        let lang = Catalogue::french();
        let context = SketchContext {
            document: &mut document,
            editor: &mut editor,
            extrusion: &mut extrusion,
            lang: &lang,
        };

        let shown =
            pending_annotation(&context, 0, cursor, 1.0, 0.05).expect("a dimension is shown");
        let asked = context.document.lays_as_readout(0, shown.target);
        if context.document.sketches()[0]
            .dimension_of(shown.target)
            .is_none()
        {
            let value = context
                .document
                .measured(0, shown.target)
                .expect("it reads");
            context.document.apply(Operation::SetDimension {
                sketch: 0,
                target: shown.target,
                value: value.into(),
                placement: None,
            });
        }
        let recorded = context.document.sketches()[0]
            .dimension_of(shown.target)
            .map(|laid| laid.driven);

        assert_eq!(
            shown.read_only, read_only,
            "{said}: the preview says read-only {}, not {read_only}",
            shown.read_only,
        );
        assert_eq!(
            asked, shown.read_only,
            "{said}: the document is asked the question the preview answers",
        );
        assert_eq!(
            recorded,
            Some(shown.read_only),
            "{said}: the click records what the preview said, on {:?}",
            shown.target,
        );
    }
}
