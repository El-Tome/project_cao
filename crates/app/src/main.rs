// On Windows a graphical application must not open a console. It is kept in
// debug, to go on seeing the panics and the logs.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use cao_app::{MSAA_SAMPLES, adapters, app, crash};

fn main() -> eframe::Result<()> {
    // A part named on the command line is the part this window opens: that is
    // how the library panel puts a second part beside the first one.
    let opening = std::env::args().nth(1).map(std::path::PathBuf::from);
    let locations = adapters::locations::discover();
    if let Some(at) = &locations {
        crash::record_panics(at);
    }
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
        Box::new(|cc| Ok(Box::new(app::CaoApp::new(cc, locations, opening)))),
    )
}
