//! What an ellipse encloses, raised into matter.
//!
//! Closes #413.
//! - a closed ellipse extrudes with one curved face for its wall —
//!   `an_ellipse_extrudes_with_one_face_for_its_wall`
//! - the matter raised is the ellipse's own —
//!   `an_ellipse_raised_holds_the_volume_the_curve_encloses`
//! - the areas themselves, and the crossings, are the drawing's — no test: not
//!   in this file, since they are held beside the geometry, in the sketch's
//!   own what_an_ellipse_closes_and_crosses

use cao_part::history::{ExtrusionMode, Operation, PointRef};
use cao_part::{History, PartState};
use cao_sketch::{Area, WorkPlane};
use cao_solid::Mesh;
use glam::DVec2;

fn volume(mesh: &Mesh) -> f64 {
    mesh.triangles()
        .iter()
        .map(|[a, b, c]| a.dot(b.cross(*c)) / 6.0)
        .sum()
}

/// A sketch holding one ellipse about (50, 20), sixty wide and forty high,
/// with the areas the middle of it falls in.
fn a_raised_ellipse(distance: f64) -> PartState {
    let mut history = History::default();
    history.push(Operation::CreateSketch {
        plane: WorkPlane::XY,
        on: None,
    });
    history.push(Operation::AddEllipse {
        sketch: 0,
        center: PointRef::New(DVec2::new(50.0, 20.0)),
        first: [
            PointRef::New(DVec2::new(20.0, 20.0)),
            PointRef::New(DVec2::new(80.0, 20.0)),
        ],
        second: [
            PointRef::New(DVec2::new(50.0, 0.0)),
            PointRef::New(DVec2::new(50.0, 40.0)),
        ],
        construction: false,
    });
    let areas: Vec<Area> = PartState::rebuild(&history).areas_at(0, &[DVec2::new(50.0, 20.0)]);
    assert_eq!(areas.len(), 1, "the ellipse encloses one area: {areas:?}");
    history.push(Operation::Extrude {
        sketch: 0,
        areas,
        distance,
        mode: ExtrusionMode::Add,
    });
    PartState::rebuild(&history)
}

#[test]
fn an_ellipse_raised_holds_the_volume_the_curve_encloses() {
    let state = a_raised_ellipse(5.0);

    let raised = volume(&state.body);
    let wanted = std::f64::consts::PI * 30.0 * 20.0 * 5.0;
    assert!(
        (raised - wanted).abs() < wanted * 0.01,
        "the matter raised is {raised}, where the ellipse encloses {wanted}",
    );
}

#[test]
fn an_ellipse_extrudes_with_one_face_for_its_wall() {
    let state = a_raised_ellipse(5.0);

    let mut faces: Vec<usize> = state.body.polygons.iter().map(|it| it.face).collect();
    faces.sort_unstable();
    faces.dedup();
    assert_eq!(
        faces.len(),
        3,
        "a floor, a ceiling and one wall, not one wall per straight step: {faces:?}",
    );
}
