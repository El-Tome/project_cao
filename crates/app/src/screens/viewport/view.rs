//! The frame loop: what the canvas allocates, what it hands to each painter,
//! and the callback that puts the scene on the GPU.
//!
//! What it decides lives in [`super::state`].

use cao_prefs::config::LengthUnit;
use cao_render::SceneFrame;
use glam::DVec3;

use crate::screens::SketchContext;
use crate::screens::sketch::Tool;

use super::finish::advance_on_enter;
use super::input::{handle_sketch_input, pick_areas, point_the_section};
use super::matter::seen;
use super::navigation::{advance_transition, handle_navigation};
use super::render::{
    frame_of, paint_band, paint_copy_panel, paint_dimension_field, paint_dimension_labels,
    paint_face_labels, paint_measure, paint_pattern_labels, paint_rule_marks, paint_ruler,
    paint_section_handle, paint_section_panel, push_blinking_matter, what_is_measured,
    what_would_go,
};
use super::state::{GestureGoesTo, ViewScale, ViewportState, cube_rect, gesture_goes_to};
use super::values::refused_for_what_is_typed;

/// What a frame of the viewport did that the shell above it has to carry out.
#[derive(Default)]
pub struct Shown {
    /// The part was modified and should be saved.
    pub changed: bool,
    /// Where the copy tools' panel was let go this frame, to be kept.
    pub copy_panel_let_go_at: Option<[f32; 2]>,
    /// The point of the part under the pointer, while no sketch is open.
    pub on_the_part: Option<DVec3>,
    /// The unit the scale bar says its step in, once the canvas is laid.
    pub unit: Option<LengthUnit>,
}

pub fn show(ui: &mut egui::Ui, state: &mut ViewportState, sketch: &mut SketchContext<'_>) -> Shown {
    let (rect, response) =
        ui.allocate_exact_size(ui.available_size(), egui::Sense::click_and_drag());
    if rect.width() < 1.0 || rect.height() < 1.0 {
        return Shown::default();
    }

    let cube_rect = cube_rect(rect, &state.config);
    state.canvas = Some(rect);

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
    let sectioning =
        !handled_cube && point_the_section(ui, state, &response, (rect, scale), sketch);
    let mut changed = match gesture_goes_to(handled_cube, sketch) {
        _ if sectioning => false,
        GestureGoesTo::TheCube => false,
        GestureGoesTo::PickingAnArea => {
            pick_areas(state, &response, rect, scale, sketch);
            false
        }
        // A click is refused while a field at the cursor holds what cannot be
        // used, rather than laying the shape where the cursor is and dropping
        // what was typed without a word. A copy tool's click lays nothing: its
        // panel refuses Validate instead.
        GestureGoesTo::TheToolInHand
            if response.clicked()
                && sketch.editor.copy_panel().is_none()
                && refused_for_what_is_typed(sketch) =>
        {
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
    let (laid, copy_panel_let_go_at) = paint_copy_panel(ui, state.copy_panel_at, rect, sketch);
    changed |= laid;
    changed |= paint_section_panel(ui, state, rect, sketch);

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

    // After every gesture of the frame, a sketch laid on the canvas included,
    // and before the part is drawn; the panel, drawn earlier, catches up on
    // the next frame.
    if state.keep_reading_up_with(sketch.document) {
        ui.ctx().request_repaint();
    }
    let drawn = seen(state, sketch.editor, sketch.document);
    let shown = (going.as_ref(), measuring.as_ref(), drawn.as_ref());
    let mut frame = frame_of(state, (rect, cube_rect), scale, sketch, shown);
    let on_the_part = response
        .hover_pos()
        .filter(|_| sketch.editor.active_sketch().is_none())
        .zip(drawn)
        .and_then(|(at, drawn)| state.point_of_the_part_at(&drawn, at, rect));
    let now = ui.input(|input| input.time);
    push_blinking_matter(&mut frame.scene_surfaces, state, sketch, now);
    if state.faces_blinking(now).is_some() {
        ui.ctx().request_repaint();
    }
    ui.painter().add(egui_wgpu::Callback::new_paint_callback(
        rect,
        ViewportCallback { frame },
    ));

    paint_section_handle(ui, state, rect, sketch);
    paint_face_labels(ui, state, cube_rect, sketch.lang);
    paint_band(ui, state, rect, sketch);
    paint_rule_marks(ui, state, rect, sketch, going.as_ref());
    paint_dimension_labels(ui, state, rect, sketch, going.as_ref());
    paint_measure(ui, state, rect, sketch, measuring.as_ref());
    paint_pattern_labels(ui, state, rect, sketch);
    if state.config.ruler_visible {
        paint_ruler(ui, state, rect, scale);
    }
    Shown {
        changed: changed | paint_dimension_field(ui, state, rect, sketch),
        copy_panel_let_go_at,
        on_the_part,
        unit: Some(state.config.unit.unit_for(scale.step_millimeters)),
    }
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
        || crate::ui::formula_field::keeps_escape(ui.ctx())
    {
        return;
    }
    let editor = &mut context.editor;
    if editor.copy_panel().is_some() {
        editor.cancel_the_copy();
        return;
    }
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
