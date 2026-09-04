mod app;
mod screens;

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([960.0, 640.0]),
        ..Default::default()
    };

    eframe::run_native(
        "CAO",
        options,
        Box::new(|cc| Ok(Box::new(app::CaoApp::new(cc)))),
    )
}
