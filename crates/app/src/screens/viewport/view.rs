//! The frame loop: what the canvas allocates, what it hands to each painter,
//! and the callback that puts the scene on the GPU.
//!
//! What it decides lives in [`super::state`].

use cao_render::SceneFrame;

use crate::screens::SketchContext;
use crate::screens::sketch::Tool;

use super::finish::advance_on_enter;
use super::input::{handle_sketch_input, pick_areas};
use super::navigation::{advance_transition, handle_navigation};
use super::render::{
    build_frame, paint_band, paint_dimension_field, paint_dimension_labels, paint_face_labels,
    paint_measure, paint_rule_marks, paint_ruler, what_is_measured, what_would_go,
};
use super::state::{GestureGoesTo, ViewScale, ViewportState, cube_rect, gesture_goes_to};

/// Returns true when the part was modified and should be saved.
pub fn show(ui: &mut egui::Ui, state: &mut ViewportState, sketch: &mut SketchContext<'_>) -> bool {
    let (rect, response) =
        ui.allocate_exact_size(ui.available_size(), egui::Sense::click_and_drag());
    if rect.width() < 1.0 || rect.height() < 1.0 {
        return false;
    }

    let cube_rect = cube_rect(rect, &state.config);
    state.aspect = rect.width() / rect.height();

    advance_transition(ui, state);
    handle_escape(ui, sketch);
    let handled_cube = handle_navigation(ui, state, &response, cube_rect);
    let scale = ViewScale::of(
        &state.camera,
        rect,
        ui.ctx().pixels_per_point(),
        &state.config,
        sketch.document.scale(),
    );
    let mut changed = match gesture_goes_to(handled_cube, sketch) {
        GestureGoesTo::TheCube => false,
        GestureGoesTo::PickingAnArea => {
            pick_areas(state, &response, rect, scale, sketch);
            false
        }
        GestureGoesTo::TheToolInHand => {
            handle_sketch_input(ui, state, &response, rect, scale, sketch)
        }
    };

    // Read before the scene below is built from it: a value typed this very frame has to be what
    // the preview reflects, not what it was a frame ago. The live field's own popup asks for the
    // egui::Order::Foreground layer regardless of when it is painted, so moving this earlier does
    // not move it behind anything.
    changed |= advance_on_enter(ui, sketch, scale);

    // A measure read a moment ago is about a drawing that has just moved, so
    // it goes with the change rather than staying on screen being wrong.
    if changed {
        sketch.editor.forget_the_measure();
    }

    // The scene goes down first. Everything egui paints — the values of the dimensions, the scale
    // bar, the labels — is added to the same layer, in order, and the scene now fills the viewport
    // with its background: put it last and it wipes all of them out.
    // Worked out here rather than in each painter: the stretch drawn over the
    // drawing, the values and the marks all have to be the same answer.
    let going = what_would_go(sketch, scale);
    // Read once, for the same reason: the shape lit under a measure and the
    // numbers written on it have to be the one answer, and walking the
    // drawing's areas twice for it would be the same answer at twice the
    // price.
    let measuring = what_is_measured(sketch);

    let frame = build_frame(
        state,
        rect,
        cube_rect,
        scale,
        sketch,
        going.as_ref(),
        measuring.as_ref(),
    );
    ui.painter().add(egui_wgpu::Callback::new_paint_callback(
        rect,
        ViewportCallback { frame },
    ));

    paint_face_labels(ui, state, cube_rect, sketch.lang);
    paint_band(ui, state, rect, sketch);
    paint_rule_marks(ui, state, rect, sketch, going.as_ref());
    paint_dimension_labels(ui, state, rect, sketch, going.as_ref());
    paint_measure(ui, state, rect, sketch, measuring.as_ref());
    if state.config.ruler_visible {
        paint_ruler(ui, state, rect, scale);
    }
    changed | paint_dimension_field(ui, state, rect, sketch)
}

/// Escape steps back out of whatever is going on: the shape in progress, the
/// dimension being placed, and then the tool itself.
///
/// A tool that stays in hand after its work is done is a tool that draws a
/// stray line on the next click; falling back to the selection tool is the
/// habit every CAD package has taught.
fn handle_escape(ui: &egui::Ui, context: &mut SketchContext<'_>) {
    if context.editor.active_sketch().is_none()
        || !ui.input(|input| input.key_pressed(egui::Key::Escape))
    {
        return;
    }
    let editor = &mut context.editor;
    let busy = editor.editing.is_some() || editor.tool_state.is_busy();
    // The tool changes first: reset_pending() reads it to decide the right idle tool_state, and
    // giving Select back after clearing would leave it with none, silently disabling the selection
    // tool until it is chosen again by hand.
    if !busy {
        editor.tool = Tool::Select;
    }
    editor.reset_pending();
}

struct ViewportCallback {
    frame: SceneFrame,
}

impl egui_wgpu::CallbackTrait for ViewportCallback {
    fn prepare(
        &self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        _screen_descriptor: &egui_wgpu::ScreenDescriptor,
        _encoder: &mut wgpu::CommandEncoder,
        resources: &mut egui_wgpu::CallbackResources,
    ) -> Vec<wgpu::CommandBuffer> {
        if let Some(renderer) = resources.get_mut::<cao_render::SceneRenderer>() {
            renderer.prepare(device, queue, &self.frame);
        }
        Vec::new()
    }

    fn paint(
        &self,
        _info: egui::PaintCallbackInfo,
        render_pass: &mut wgpu::RenderPass<'static>,
        resources: &egui_wgpu::CallbackResources,
    ) {
        if let Some(renderer) = resources.get::<cao_render::SceneRenderer>() {
            renderer.paint(render_pass);
        }
    }
}
