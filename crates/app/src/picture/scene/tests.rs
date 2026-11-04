//! What app · picture/scene.rs is held to.
//!
//! The picture's share of #621, whose criteria live with the part's tests in
//! `crates/part/tests/a_fillet_rounds_a_corner_with_a_curve.rs`: a curve
//! rounded at its end is drawn from its far end to the fillet only, as a trait
//! side is — `a_curve_rounded_at_its_end_reaches_the_picture_from_its_far_end_to_the_fillet`.

use super::*;

use cao_part::{Operation, PointRef};
use cao_sketch::{Corner, WorkPlane};

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
        distance: 4.0.into(),
        mode: cao_part::ExtrusionMode::Add,
    });

    let frame = of(&document, 128);

    assert!(
        !frame.scene_solids.is_empty(),
        "the matter the extrusion made is what the picture is mostly for",
    );
}

/// A curve side of a corner, leaving the corner the way its drawing runs
/// when `counter_clockwise`: an arc round `centre`, or an arc of ellipse round
/// it whose first axis reaches `first` and whose second reaches `second`.
#[derive(Clone, Copy, Debug)]
struct CurveSide {
    centre: DVec2,
    first: DVec2,
    second: f64,
    counter_clockwise: bool,
}

impl CurveSide {
    fn arc(centre: DVec2, radius: f64) -> Self {
        Self {
            centre,
            first: DVec2::new(radius, 0.0),
            second: radius,
            counter_clockwise: true,
        }
    }

    fn squashed(self, place: DVec2) -> DVec2 {
        let out = place - self.centre;
        DVec2::new(
            out.dot(self.first.normalize()) / self.first.length(),
            out.dot(self.first.perp().normalize()) / self.second,
        )
    }

    /// How far round from the corner a place lies, the way the side leaves
    /// it, within half a turn either way — when the place lies on the curve.
    fn along(self, corner: DVec2, place: DVec2) -> Option<f64> {
        if (self.squashed(place).length() - 1.0).abs() > 1e-5 {
            return None;
        }
        let turned = self.squashed(place).to_angle() - self.squashed(corner).to_angle();
        let turned = match self.counter_clockwise {
            true => turned,
            false => -turned,
        };
        let tau = std::f64::consts::TAU;
        Some(turned - tau * (turned / tau).round())
    }
}

/// A trait from (10, 10) to (50, 10), an arc round (2, 10) of radius 8 from
/// that corner to (2, 18), and the corner rounded by five; then an arc of
/// ellipse round (30, 60), its first axis (8, 6) and its second reaching 5,
/// drawn from (32.4, 66.8) round to (22, 54), and an arc round (42.4, 66.8) of
/// radius 10 from that corner to (42.4, 56.8), that corner rounded by two.
fn curves_rounded_at_their_ends() -> PartDocument {
    let mut document = part_with(vec![
        Operation::CreateSketch {
            plane: WorkPlane::XY,
            on: None,
        },
        Operation::AddPoint {
            sketch: 0,
            position: DVec2::new(10.0, 10.0),
            on: Vec::new(),
        },
        Operation::AddSegment {
            sketch: 0,
            start: PointRef::Existing(cao_sketch::PointId(1)),
            end: PointRef::New(DVec2::new(50.0, 10.0)),
            construction: false,
        },
        Operation::AddArc {
            sketch: 0,
            center: PointRef::New(DVec2::new(2.0, 10.0)),
            start: PointRef::Existing(cao_sketch::PointId(1)),
            end: PointRef::New(DVec2::new(2.0, 18.0)),
            construction: false,
        },
        Operation::AddPoint {
            sketch: 0,
            position: DVec2::new(32.4, 66.8),
            on: Vec::new(),
        },
    ]);
    let other_corner = cao_sketch::PointId(document.sketches()[0].points().len() - 1);
    let back = PointRef::New(DVec2::new(22.0, 54.0));
    for operation in [
        Operation::AddEllipse {
            sketch: 0,
            center: PointRef::New(DVec2::new(30.0, 60.0)),
            first: [back.clone(), PointRef::New(DVec2::new(38.0, 66.0))],
            second: [
                PointRef::New(DVec2::new(33.0, 56.0)),
                PointRef::New(DVec2::new(27.0, 64.0)),
            ],
            construction: false,
            drawn: Some([PointRef::Existing(other_corner), back]),
        },
        Operation::AddArc {
            sketch: 0,
            center: PointRef::New(DVec2::new(42.4, 66.8)),
            start: PointRef::Existing(other_corner),
            end: PointRef::New(DVec2::new(42.4, 56.8)),
            construction: false,
        },
        Operation::Fillet {
            sketch: 0,
            corners: vec![Corner::At(cao_sketch::PointId(1))],
            radius: 5.0.into(),
            construction: false,
        },
        Operation::Fillet {
            sketch: 0,
            corners: vec![Corner::Between(
                cao_sketch::EllipseId(0).into(),
                cao_sketch::ArcId(1).into(),
            )],
            radius: 2.0.into(),
            construction: false,
        },
    ] {
        document.apply(operation);
    }
    document
}

/// How far round one straight step of a curve turns at most: a stroke whose
/// two ends lie on the curve further round than this is a chord across it —
/// an axis of the ellipse — not a step along it.
const A_STEP_ROUND: f64 = std::f64::consts::PI / 8.0;

#[test]
fn a_curve_rounded_at_its_end_reaches_the_picture_from_its_far_end_to_the_fillet() {
    let document = curves_rounded_at_their_ends();
    let sketch = &document.sketches()[0];
    let drawn = sketch.drawn_cuts();
    assert_eq!(drawn.len(), 2, "both corners are rounded: {drawn:?}");
    assert!(drawn.iter().all(|cut| cut.fits), "{drawn:?}");
    let (low, high) = (DVec2::new(10.0, 10.0), DVec2::new(32.4, 66.8));
    let sides = [
        (
            low,
            CurveSide::arc(DVec2::new(2.0, 10.0), 8.0),
            DVec2::new(2.0, 18.0),
            "the arc beside the trait",
        ),
        (
            high,
            CurveSide {
                centre: DVec2::new(30.0, 60.0),
                first: DVec2::new(8.0, 6.0),
                second: 5.0,
                counter_clockwise: true,
            },
            DVec2::new(22.0, 54.0),
            "the arc of ellipse beside the arc",
        ),
        (
            high,
            CurveSide::arc(DVec2::new(42.4, 66.8), 10.0),
            DVec2::new(42.4, 56.8),
            "the arc beside the arc of ellipse",
        ),
    ];

    let frame = of(&document, 128);

    for (corner, side, far, what) in sides {
        let touch = drawn
            .iter()
            .filter(|cut| cut.corner.distance(corner) < 1e-9)
            .flat_map(|cut| cut.touches)
            .find_map(|touch| side.along(corner, touch))
            .expect("the fillet touches the curve");
        let far = side
            .along(corner, far)
            .expect("the far end is on the curve");
        let along: Vec<f64> = frame
            .scene_lines
            .chunks(2)
            .filter_map(|stroke| {
                let [from, to] = [stroke[0], stroke[1]].map(|vertex| {
                    sketch
                        .plane
                        .to_local(Vec3::from(vertex.position).as_dvec3())
                });
                let ends = [side.along(corner, from)?, side.along(corner, to)?];
                Some(ends).filter(|[one, other]| (one - other).abs() < A_STEP_ROUND)
            })
            .flatten()
            .collect();
        let lowest = along.iter().copied().fold(f64::INFINITY, f64::min);
        let highest = along.iter().copied().fold(f64::NEG_INFINITY, f64::max);

        assert!(
            (lowest - touch).abs() < 1e-4,
            "{what}: drawn from where the fillet touches it, {touch} round from the \
             corner, not from {lowest}",
        );
        assert!(
            (highest - far).abs() < 1e-4,
            "{what}: drawn up to its far end, {far} round, not {highest}",
        );
    }
}
