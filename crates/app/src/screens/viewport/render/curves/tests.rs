//! What app · screens/viewport/render/curves.rs is held to.
//!
//! A curve drawn dashed — a construction circle, arc or ellipse, and the
//! stretch of a side a cut hides — is broken into dashes about a dash long on
//! screen, with gaps about as long, however long the steps of the polyline it
//! is drawn as: as a line is. The owner of #621 found dashes a whole step
//! long once a curve was zoomed in on, a step being far longer on screen than
//! a dash.

use cao_prefs::config::ViewportConfig;
use cao_render::camera::OrbitCamera;
use cao_sketch::{WorkPlane, steps_along, sweep_of};
use glam::Vec3;

use super::*;

/// How far two places may be apart and still be one end of a dash and the
/// start of the next stroke of the same dash, in drawing units: the vertices
/// travel as `f32`.
const JOINED: f64 = 1e-4;

/// The canvas as the default camera shows it, on a window of the usual size.
fn a_view() -> ViewScale {
    ViewScale::of(
        &OrbitCamera::default(),
        egui::Rect::from_min_size(egui::Pos2::ZERO, egui::vec2(1280.0, 800.0)),
        1.0,
        &ViewportConfig::default(),
        1.0,
    )
}

/// The dashes a run of strokes draws, each as how long it runs along the
/// curve and where it starts and ends: strokes that follow on from one
/// another are one dash.
fn dashes(painted: &[cao_render::Vertex], sketch: &Sketch) -> Vec<(f64, DVec2, DVec2)> {
    let place = |vertex: &cao_render::Vertex| {
        sketch
            .plane
            .to_local(Vec3::from(vertex.position).as_dvec3())
    };
    let mut dashes: Vec<(f64, DVec2, DVec2)> = Vec::new();
    for stroke in painted.chunks(2) {
        let (from, to) = (place(&stroke[0]), place(&stroke[1]));
        match dashes.last_mut() {
            Some((run, _, end)) if end.distance(from) < JOINED => {
                *run += from.distance(to);
                *end = to;
            }
            _ => dashes.push((from.distance(to), from, to)),
        }
    }
    dashes
}

/// Holds the dashes painted for a curve whose longest step is `step` long, in
/// `steps` steps: none longer than a dash on screen, more than one inside a
/// step, and no gap between two much longer than a dash.
fn assert_dashed_on_screen(painted: &[cao_render::Vertex], step: f64, steps: usize, what: &str) {
    let sketch = Sketch::new(WorkPlane::XY);
    let dash = DASH_PIXELS * a_view().units_per_pixel;
    assert!(
        step > 4.0 * dash,
        "{what}: the fixture has a step of {step}, several dashes of {dash} long on screen"
    );
    let dashes = dashes(painted, &sketch);
    let longest = dashes.iter().map(|(run, _, _)| *run).fold(0.0, f64::max);
    assert!(
        longest <= dash * 1.05,
        "{what}: every dash is about a dash long on screen, {dash}; the longest runs {longest}, \
         its step being {step}"
    );
    assert!(
        dashes.len() > 2 * steps,
        "{what}: a step several dashes long holds several dashes: {} dashes over {steps} steps",
        dashes.len()
    );
    let widest_gap = dashes
        .windows(2)
        .map(|pair| pair[0].2.distance(pair[1].1))
        .fold(0.0, f64::max);
    assert!(
        widest_gap <= dash * 2.0,
        "{what}: the gaps are about a dash long on screen, {dash}; the widest is {widest_gap}"
    );
}

#[test]
fn a_dashed_arc_is_broken_into_dashes_a_dash_long_on_screen_however_long_its_steps() {
    let sketch = Sketch::new(WorkPlane::XY);
    let drawn = ArcDraft {
        centre: DVec2::ZERO,
        start: DVec2::new(100.0, 0.0),
        end: DVec2::new(0.0, 100.0),
    };
    let steps = steps_along(drawn);
    let step = sweep_of(drawn) * 100.0 / steps as f64;
    let mut painted = Vec::new();

    push_arc_at(&mut painted, &sketch, drawn, [1.0; 4], 1.0, true, a_view());

    assert_dashed_on_screen(&painted, step, steps, "a quarter of a circle of radius 100");
}

#[test]
fn a_dashed_ellipse_is_broken_into_dashes_a_dash_long_on_screen_however_long_its_steps() {
    let sketch = Sketch::new(WorkPlane::XY);
    let drawn = EllipseDraft {
        centre: DVec2::ZERO,
        first: DVec2::new(100.0, 0.0),
        second: 60.0,
    };
    let places = places_of(drawn, 0.0, std::f64::consts::FRAC_PI_2);
    let step = places
        .windows(2)
        .map(|pair| pair[0].distance(pair[1]))
        .fold(0.0, f64::max);
    let steps = places.len() - 1;
    let mut painted = Vec::new();

    push_ellipse_at(&mut painted, &sketch, places, [1.0; 4], 1.0, true, a_view());

    assert_dashed_on_screen(&painted, step, steps, "a quarter of an ellipse 100 by 60");
}

#[test]
fn a_dashed_circle_is_broken_into_dashes_a_dash_long_on_screen_however_long_its_steps() {
    let sketch = Sketch::new(WorkPlane::XY);
    let radius = 100.0;
    let mut painted = Vec::new();

    push_circle_at(
        &mut painted,
        &sketch,
        DVec2::ZERO,
        radius,
        [1.0; 4],
        1.0,
        true,
        a_view(),
    );

    let sides = 96;
    assert_dashed_on_screen(
        &painted,
        std::f64::consts::TAU * radius / sides as f64,
        sides,
        "a circle of radius 100",
    );
}
