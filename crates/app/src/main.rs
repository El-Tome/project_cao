mod app;
mod screens;

/// Multisampling for the whole surface: the viewport is drawn with thin lines,
/// which alias badly without it. The renderer's pipelines must be built with
/// this same count.
pub const MSAA_SAMPLES: u16 = 4;

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([1280.0, 800.0]),
        multisampling: MSAA_SAMPLES,
        ..Default::default()
    };

    eframe::run_native(
        "CAO",
        options,
        Box::new(|cc| Ok(Box::new(app::CaoApp::new(cc)))),
    )
}
