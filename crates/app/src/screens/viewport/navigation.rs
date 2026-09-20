//! Driving the camera: which gesture a drag is, what a scroll wheel and a
//! trackpad each mean, and the transition that carries the view from one
//! angle to the next.
//!
//! The canvas itself is [`super`]; this is only what moves the eye.

use cao_prefs::config::{Binding, TrackpadGesture};
use cao_render::cube;

use super::{ViewportState, to_ndc};

#[derive(Clone, Copy, PartialEq, Eq)]
pub(super) enum Drag {
    Orbit,
    Pan,
}
pub(super) fn advance_transition(ui: &egui::Ui, state: &mut ViewportState) {
    let Some(transition) = state.transition.as_mut() else {
        return;
    };
    let dt = ui.input(|input| input.stable_dt);
    if transition.advance(&mut state.camera, dt) {
        ui.ctx().request_repaint();
    } else {
        state.transition = None;
    }
}
/// Camera navigation. Returns true when the click belonged to the orientation
/// cube, so the sketch tools do not also act on it.
pub(super) fn handle_navigation(
    ui: &egui::Ui,
    state: &mut ViewportState,
    response: &egui::Response,
    cube_rect: egui::Rect,
) -> bool {
    let pointer = response.hover_pos();
    let over_cube = pointer.is_some_and(|position| cube_rect.contains(position));

    state.hovered_zone = match (state.drag, pointer) {
        (None, Some(position)) if over_cube => {
            cube::pick_zone(state.camera.rotation(), to_ndc(position, cube_rect))
        }
        _ => None,
    };

    if response.clicked()
        && let Some(zone) = state.hovered_zone
    {
        state.snap_to_zone(zone);
        return true;
    }

    let (delta, scroll, drag) = ui.input(|input| {
        let matches = |bindings: &[Binding]| {
            bindings.iter().any(|binding| {
                input.pointer.button_down(to_egui_button(binding.button))
                    && input.modifiers.shift == binding.shift
                    && input.modifiers.command == binding.ctrl
                    && input.modifiers.alt == binding.alt
            })
        };
        let navigation = state.config.navigation;
        let drag = if matches(navigation.orbit()) {
            Some(Drag::Orbit)
        } else if matches(navigation.pan()) {
            Some(Drag::Pan)
        } else {
            None
        };
        (
            glam::Vec2::new(input.pointer.delta().x, input.pointer.delta().y),
            ScrollInput::read(input),
            drag,
        )
    });

    // Only a drag that started over the canvas keeps control, but once it has
    // it survives the cursor leaving the canvas.
    state.drag = match (state.drag, drag) {
        (Some(_), None) => None,
        (Some(active), Some(_)) => Some(active),
        (None, Some(new)) if response.hovered() && !over_cube => Some(new),
        (None, _) => None,
    };

    let height_px = rect_height_px(ui, response.rect);

    match state.drag {
        Some(Drag::Orbit) => state.orbit(delta, state.config.orbit_sensitivity),
        Some(Drag::Pan) => state.camera.pan(delta, height_px),
        None => {}
    }

    if !response.hovered() {
        return over_cube;
    }

    // A mouse wheel zooms, as in every CAD package. A trackpad's two-finger
    // scroll is a different gesture arriving in the same event stream, so it
    // gets its own mapping and its own sensitivity.
    if scroll.wheel_notches != 0.0 {
        state
            .camera
            .zoom(scroll.wheel_notches, state.config.wheel_zoom_sensitivity);
    }

    if scroll.zoom_points != 0.0 {
        state
            .camera
            .zoom(scroll.zoom_points, state.config.zoom_sensitivity);
    }

    if scroll.trackpad != glam::Vec2::ZERO {
        let trackpad = state.config.trackpad;
        let shift = ui.input(|input| input.modifiers.shift);
        let gesture = if shift {
            trackpad.shift_scroll
        } else {
            trackpad.scroll
        };
        let delta = scroll.trackpad * trackpad.scroll_sensitivity;

        match gesture {
            TrackpadGesture::Pan => state.camera.pan(delta, height_px),
            TrackpadGesture::Orbit => state.orbit(-delta, state.config.orbit_sensitivity),
            TrackpadGesture::Zoom => state.camera.zoom(delta.y, state.config.zoom_sensitivity),
            TrackpadGesture::Ignore => {}
        }
    }

    if state.config.trackpad.pinch_zooms && scroll.pinch != 1.0 {
        state.camera.zoom_by_factor(scroll.pinch);
    }

    over_cube
}
fn to_egui_button(button: cao_prefs::config::PointerButton) -> egui::PointerButton {
    match button {
        cao_prefs::config::PointerButton::Primary => egui::PointerButton::Primary,
        cao_prefs::config::PointerButton::Middle => egui::PointerButton::Middle,
        cao_prefs::config::PointerButton::Secondary => egui::PointerButton::Secondary,
    }
}
fn rect_height_px(ui: &egui::Ui, rect: egui::Rect) -> f32 {
    rect.height() * ui.ctx().pixels_per_point()
}
/// The three ways a scroll gesture arrives, kept apart because they mean
/// different things.
///
/// They cannot be read from `smooth_scroll_delta`/`zoom_delta`, which merge a
/// wheel, a two-finger scroll and a pinch into common values: a wheel notch is
/// one *line* while a trackpad reports *pixels*, so sharing a sensitivity makes
/// one of them crawl and the other bolt.
#[derive(Default)]
struct ScrollInput {
    /// Mouse wheel, in notches.
    wheel_notches: f32,
    /// Two-finger scroll, in points.
    trackpad: glam::Vec2,
    /// Two-finger scroll held with the zoom modifier, in points.
    zoom_points: f32,
    /// Pinch, as a scale factor (1.0 = no change).
    pinch: f32,
}

impl ScrollInput {
    fn read(input: &egui::InputState) -> Self {
        let mut scroll = Self {
            pinch: 1.0,
            ..Self::default()
        };

        for event in &input.events {
            match event {
                egui::Event::MouseWheel {
                    unit,
                    delta,
                    modifiers,
                    ..
                } => match unit {
                    egui::MouseWheelUnit::Point if modifiers.command => {
                        scroll.zoom_points += delta.y;
                    }
                    egui::MouseWheelUnit::Point => {
                        scroll.trackpad += glam::Vec2::new(delta.x, delta.y);
                    }
                    egui::MouseWheelUnit::Line | egui::MouseWheelUnit::Page => {
                        scroll.wheel_notches += delta.y;
                    }
                },
                egui::Event::Zoom(factor) => scroll.pinch *= factor,
                _ => {}
            }
        }

        if let Some(touch) = input.multi_touch() {
            scroll.pinch *= touch.zoom_delta;
        }

        scroll
    }
}
