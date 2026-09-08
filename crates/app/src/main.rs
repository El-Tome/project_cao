// On Windows a graphical application must not open a console. It is kept in
// debug, to go on seeing the panics and the logs.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod adapters;
mod app;
mod autosave;
mod screens;
mod shortcuts;

/// Multisampling for the whole surface: the viewport is drawn with thin lines,
/// which alias badly without it. The renderer's pipelines must be built with
/// this same count.
pub const MSAA_SAMPLES: u16 = 4;

fn main() -> eframe::Result<()> {
    cao_prefs::record_panics();
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([1280.0, 800.0]),
        multisampling: MSAA_SAMPLES,
        // Asked for by the renderer, which is where the format is decided: a
        // solid part needs its near faces to hide its far ones.
        depth_buffer: cao_render::SceneRenderer::DEPTH_BITS,
        ..Default::default()
    };

    eframe::run_native(
        "CAO",
        options,
        Box::new(|cc| Ok(Box::new(app::CaoApp::new(cc)))),
    )
}
