//! A question asked in a modal, answered by one of two buttons.

/// What the question says, and the two answers it offers.
pub struct Question {
    pub title: String,
    pub body: String,
    pub yes: String,
    pub no: String,
}

/// `Some(true)` once answered yes, `Some(false)` once answered no or backed out
/// of — the backdrop or Escape — and `None` while nothing is decided.
pub fn ask(ui: &egui::Ui, id: &str, question: &Question) -> Option<bool> {
    let mut decision = None;
    let modal = egui::Modal::new(egui::Id::new(id)).show(ui.ctx(), |ui| {
        ui.heading(&question.title);
        ui.label(&question.body);
        ui.add_space(8.0);
        ui.horizontal(|ui| {
            if ui.button(&question.yes).clicked() {
                decision = Some(true);
            }
            if ui.button(&question.no).clicked() {
                decision = Some(false);
            }
        });
    });
    if decision.is_none() && modal.should_close() {
        decision = Some(false);
    }
    decision
}
