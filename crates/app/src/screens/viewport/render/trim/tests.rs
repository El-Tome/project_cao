//! Closes #286.
//! - the stretch that will go is drawn in the alert colour and thicker, over
//!   that stretch and no further —
//!   `only_the_stretch_that_goes_is_drawn_in_the_alert_colour_and_thicker`
//! - the rest of the trait keeps its own colour — no test: the trait
//!   itself is drawn by the sketch loop, which reads nothing of `Going`, and the
//!   stretch is laid over it. What did break it once was the hover lighting the
//!   whole trait, and that is held since #401 by
//!   `only_the_tool_that_takes_the_whole_thing_lights_the_whole_of_it`, beside
//!   `Tool`. The name this test carried until #402 claimed it was asserted
//!   here, by a test that paints the stretch alone and so cannot see the rest.
//! - the same on an arc — `an_arc_loses_a_curve_rather_than_a_straight_line`

use cao_prefs::config::ViewportConfig;
use cao_render::camera::OrbitCamera;
use cao_sketch::{Sketch, WorkPlane};
use glam::DVec2;

use super::*;

const SCREEN: egui::Rect = egui::Rect {
    min: egui::Pos2::ZERO,
    max: egui::pos2(1280.0, 800.0),
};

fn a_view() -> ViewScale {
    ViewScale::of(
        &OrbitCamera::default(),
        SCREEN,
        1.0,
        &ViewportConfig::default(),
        1.0,
    )
}

/// A trait ten units long with two points sitting on it, cut between them.
fn a_trait_cut_in_the_middle() -> (Sketch, Going) {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let start = sketch.add_point(DVec2::new(0.0, 1.0));
    let end = sketch.add_point(DVec2::new(10.0, 1.0));
    let segment = sketch.add_segment(start, end);
    let near = sketch.add_point(DVec2::new(3.0, 1.0));
    let far = sketch.add_point(DVec2::new(7.0, 1.0));
    let going = sketch
        .trim_takes(segment, near, far)
        .expect("a cut that can be made");
    (sketch, going)
}

#[test]
fn only_the_stretch_that_goes_is_drawn_in_the_alert_colour_and_thicker() {
    let (sketch, going) = a_trait_cut_in_the_middle();
    let theme = Theme::default();
    let mut out = Vec::new();

    push_going(&mut out, &sketch, &going, &theme, a_view());

    assert!(!out.is_empty(), "the stretch that goes is not drawn at all");
    assert!(
        out.iter().all(|vertex| vertex.color == tint(theme.going)),
        "the stretch that goes is drawn in a colour that is not the theme's alert one",
    );
    assert!(
        out.iter().all(|vertex| vertex.width > theme.sketch_width),
        "the stretch that goes is no thicker than the trait it is a piece of",
    );
    let along: Vec<f32> = out.iter().map(|vertex| vertex.position[0]).collect();
    let (low, high) = (
        along.iter().copied().fold(f32::MAX, f32::min),
        along.iter().copied().fold(f32::MIN, f32::max),
    );
    assert!(
        (low - 3.0).abs() < 1e-4 && (high - 7.0).abs() < 1e-4,
        "the stretch drawn runs from {low} to {high}, not between the two points \
         the click fell between",
    );
}

#[test]
fn an_arc_loses_a_curve_rather_than_a_straight_line() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let on_the_rim = |sketch: &mut Sketch, degrees: f64| {
        sketch.add_point(DVec2::from_angle(f64::to_radians(degrees)) * 10.0)
    };
    let east = on_the_rim(&mut sketch, 0.0);
    let west = on_the_rim(&mut sketch, 180.0);
    let arc = sketch.add_arc(Sketch::ORIGIN, east, west);
    let first = on_the_rim(&mut sketch, 60.0);
    let second = on_the_rim(&mut sketch, 120.0);
    let going = sketch
        .arc_trim_takes(arc, first, second)
        .expect("a cut that can be made");
    let mut out = Vec::new();

    push_going(&mut out, &sketch, &going, &Theme::default(), a_view());

    let reaches: Vec<f32> = out
        .iter()
        .map(|vertex| (vertex.position[0].powi(2) + vertex.position[1].powi(2)).sqrt())
        .collect();
    assert!(out.len() > 2, "a curve drawn as one straight line");
    assert!(
        reaches.iter().all(|reach| (reach - 10.0).abs() < 1e-3),
        "the stretch drawn leaves the curve it is a piece of: {reaches:?}",
    );
}
