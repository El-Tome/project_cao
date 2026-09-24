//! An arc closing a contour, all the way to a solid: the issue's own smallest
//! honest test — one segment for the flat side, one half-arc for the round
//! one — carried through to an extrusion.

use cao_part::{ExtrusionMode, History, Operation, PartState, PointRef};
use cao_sketch::{Area, WorkPlane};
use glam::DVec2;

/// The areas these places fall in, as the drawing stands — what the
/// interface works out at the moment of the click, for a test that has no
/// interface to click in.
fn clicked(history: &History, sketch: usize, place: DVec2) -> Vec<Area> {
    PartState::rebuild(history).areas_at(sketch, &[place])
}

fn volume(mesh: &cao_solid::Mesh) -> f64 {
    mesh.triangles()
        .iter()
        .map(|[a, b, c]| a.dot(b.cross(*c)) / 6.0)
        .sum()
}

#[test]
fn an_extrusion_of_a_d_shape_gives_a_round_edge_to_the_solid() {
    let mut history = History::default();
    history.push(Operation::CreateSketch {
        plane: WorkPlane::XY,
        on: None,
    });
    let radius = 5.0;
    let (top, bottom) = (DVec2::new(0.0, radius), DVec2::new(0.0, -radius));
    history.push(Operation::AddSegment {
        sketch: 0,
        start: PointRef::New(top),
        end: PointRef::New(bottom),
        construction: false,
    });

    // The arc's ends have to be the segment's own points, not two more
    // sitting on top of them, or nothing joins the two into one area.
    let sketch = PartState::rebuild(&history).sketches[0].clone();
    let start = sketch
        .nearest_point(bottom, 1e-6)
        .expect("the segment's own end");
    let end = sketch
        .nearest_point(top, 1e-6)
        .expect("the segment's own start");
    history.push(Operation::AddArc {
        sketch: 0,
        center: PointRef::New(DVec2::ZERO),
        start: PointRef::Existing(start),
        end: PointRef::Existing(end),
        construction: false,
    });
    history.push(Operation::Extrude {
        sketch: 0,
        areas: clicked(&history, 0, DVec2::new(2.0, 0.0)),
        distance: 3.0.into(),
        mode: ExtrusionMode::Add,
    });

    let state = PartState::rebuild(&history);
    let expected = std::f64::consts::PI * radius * radius * 0.5 * 3.0;
    let found = volume(&state.body);
    assert!(
        (found - expected).abs() / expected < 0.02,
        "volume {found}, expected ~{expected}"
    );
    let (_, max) = state.body.bounds().expect("a volume");
    assert!(
        max.x > radius - 0.5,
        "the round side bulges out to the radius: max.x = {}",
        max.x
    );
}
