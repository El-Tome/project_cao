//! What app · picture/scene.rs is held to.

use super::*;

use cao_part::{Operation, PointRef};
use cao_sketch::WorkPlane;

fn at(text: &str) -> chrono::DateTime<chrono::Utc> {
    text.parse().expect("a date")
}

fn part_with(operations: Vec<Operation>) -> PartDocument {
    let mut document = PartDocument::new("Test", at("2026-01-02T09:00:00Z"));
    for operation in operations {
        document.apply(operation);
    }
    document
}

fn a_square() -> Vec<Operation> {
    let corners = [
        (DVec2::ZERO, DVec2::new(10.0, 0.0)),
        (DVec2::new(10.0, 0.0), DVec2::new(10.0, 10.0)),
        (DVec2::new(10.0, 10.0), DVec2::new(0.0, 10.0)),
        (DVec2::new(0.0, 10.0), DVec2::ZERO),
    ];
    let mut drawn = vec![Operation::CreateSketch {
        plane: WorkPlane::XY,
        on: None,
    }];
    drawn.extend(corners.map(|(start, end)| Operation::AddSegment {
        sketch: 0,
        start: PointRef::New(start),
        end: PointRef::New(end),
        construction: false,
    }));
    drawn
}

#[test]
fn a_drawn_trait_reaches_the_picture_as_a_pair_of_ends() {
    let frame = of(&part_with(a_square()), 128);

    assert_eq!(
        frame.scene_lines.len(),
        8,
        "four traits, two ends each, and nothing else drawn",
    );
}

#[test]
fn a_picture_carries_no_grid_no_axes_and_no_cube() {
    let frame = of(&part_with(a_square()), 128);

    assert!(frame.scene_world_lines.is_empty(), "the grid and the axes");
    assert!(frame.cube_triangles.is_empty());
    assert!(frame.cube_edges.is_empty());
    assert!(frame.scene_background.is_empty());
}

#[test]
fn a_circle_reaches_the_picture_as_straight_steps_that_close_on_themselves() {
    let drawn = vec![
        Operation::CreateSketch {
            plane: WorkPlane::XY,
            on: None,
        },
        Operation::AddCircle {
            sketch: 0,
            center: PointRef::New(DVec2::ZERO),
            radius: 5.0,
            rim: Vec::new(),
            construction: false,
        },
    ];

    let frame = of(&part_with(drawn), 128);

    const TOLERANCE: f32 = 1e-5;
    assert_eq!(frame.scene_lines.len(), STEPS_OF_A_TURN * 2);
    let first = Vec3::from(frame.scene_lines[0].position);
    let last = Vec3::from(frame.scene_lines[frame.scene_lines.len() - 1].position);
    assert!(
        first.distance(last) < TOLERANCE,
        "the last step lands at {last} where the first one started at {first}",
    );
}

#[test]
fn an_empty_part_is_looked_at_from_somewhere_rather_than_from_nowhere() {
    let frame = of(&PartDocument::new("Test", at("2026-01-02T09:00:00Z")), 128);

    assert!(frame.scene_lines.is_empty());
    assert!(
        frame.scene_view_projection.is_finite(),
        "a camera framed on nothing divides by nothing and the picture is blank",
    );
}

fn on_screen(frame: &SceneFrame, point: Vec3) -> glam::Vec2 {
    let clipped = frame.scene_view_projection * point.extend(1.0);
    (clipped.truncate() / clipped.w).truncate()
}

#[test]
fn a_drawing_is_framed_so_that_every_corner_of_it_lands_inside_the_picture() {
    let frame = of(&part_with(a_square()), 128);

    for corner in [
        Vec3::ZERO,
        Vec3::new(10.0, 0.0, 0.0),
        Vec3::new(10.0, 10.0, 0.0),
        Vec3::new(0.0, 10.0, 0.0),
    ] {
        let ndc = on_screen(&frame, corner);
        assert!(
            ndc.x.abs() <= 1.0 && ndc.y.abs() <= 1.0,
            "the corner at {corner} falls outside the picture, at {ndc}",
        );
    }
}

#[test]
fn a_drawing_fills_the_picture_rather_than_sitting_as_a_speck_in_the_middle() {
    let frame = of(&part_with(a_square()), 128);

    let reach = [
        Vec3::ZERO,
        Vec3::new(10.0, 0.0, 0.0),
        Vec3::new(10.0, 10.0, 0.0),
        Vec3::new(0.0, 10.0, 0.0),
    ]
    .into_iter()
    .map(|corner| on_screen(&frame, corner).abs().max_element())
    .fold(0.0f32, f32::max);

    assert!(
        reach > 0.5,
        "the drawing reaches only {reach} of the way to the edge: at 128 pixels \
         that is a few strokes lost in a field of ground",
    );
}

#[test]
fn a_point_nothing_draws_does_not_drag_the_frame_out_to_it() {
    let mut drawn = a_square();
    // A sketch holds points no stroke ever reaches — the centre an arc
    // turns about, the corner a rectangle was pulled from, a point dropped
    // and left. The picture draws none of them.
    drawn.push(Operation::AddPoint {
        sketch: 0,
        position: DVec2::new(400.0, 400.0),
        on: Vec::new(),
    });

    let frame = of(&part_with(drawn), 128);
    let ndc = on_screen(&frame, Vec3::new(5.0, 5.0, 0.0));

    assert!(
        ndc.length() < 0.5,
        "the middle of the square sits at {ndc}, pushed aside by a point \
         nothing draws",
    );
}

#[test]
fn a_part_that_has_been_extruded_shows_its_matter() {
    let drawn = vec![
        Operation::CreateSketch {
            plane: WorkPlane::XY,
            on: None,
        },
        Operation::AddRectangle {
            sketch: 0,
            corner: PointRef::New(DVec2::ZERO),
            opposite: PointRef::New(DVec2::new(10.0, 10.0)),
            construction: false,
        },
    ];
    let mut document = part_with(drawn);
    document.apply(Operation::Extrude {
        sketch: 0,
        areas: document.areas_at(0, &[DVec2::new(5.0, 5.0)]),
        distance: 4.0,
        mode: cao_part::ExtrusionMode::Add,
    });

    let frame = of(&document, 128);

    assert!(
        !frame.scene_solids.is_empty(),
        "the matter the extrusion made is what the picture is mostly for",
    );
}
