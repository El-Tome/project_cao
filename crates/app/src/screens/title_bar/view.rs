use crate::lang::Catalogue;
use crate::ui::alert_line::alert_line;
use crate::ui::alerts_button::alerts_button;
use crate::ui::icon;
use crate::ui::panel_toggle::panel_button;
use crate::ui::pill::{MESSAGE_HEIGHT, Tone, message_pill, pill};
use crate::ui::sheet::sheet_under;

use super::state::{Alert, Said, TitleBar};

/// How wide P2 of `docs/redesign/part-screen.md` draws the list of alerts, in
/// points, which the tokens do not carry.
const LIST_WIDTH: f32 = 340.0;

/// Draws the title bar along the top of the screen, and says whether the way
/// home was clicked. The message stands in the middle of the window rather
/// than of what the breadcrumb leaves, so it does not move as the part's name
/// grows; the count of alerts at its right end, which the message never covers,
/// and their list under it while `list_open` says so.
pub fn show(ui: &mut egui::Ui, bar: &TitleBar<'_>, list_open: &mut bool, lang: &Catalogue) -> bool {
    let middle = ui.ctx().content_rect().center().x;
    let (home, count) = egui::Panel::top("part_title_bar")
        .show(ui, |ui| {
            let row = egui::vec2(ui.available_width(), MESSAGE_HEIGHT);
            let centred = egui::Layout::left_to_right(egui::Align::Center);
            ui.allocate_ui_with_layout(row, centred, |ui| {
                let home = panel_button(ui, false, icon::HOME, lang.t("app.home")).clicked();
                ui.weak("/");
                ui.strong(bar.name)
                    .on_hover_text(bar.path.display().to_string());
                ui.weak("/");
                pill(ui, &bar.view);
                let from_the_right = egui::Layout::right_to_left(egui::Align::Center);
                ui.with_layout(from_the_right, |ui| {
                    let count = bar
                        .alerts_button
                        .as_ref()
                        .map(|count| alerts_button(ui, &count.text, count.level));
                    ui.with_layout(centred, |ui| {
                        if let Some(said) = bar.said {
                            let (text, tone) = match said {
                                Said::Message(message) => (message, Tone::Said),
                                Said::Error(error) => (error, Tone::Failed),
                            };
                            message_pill(ui, middle, text, tone);
                        }
                    });
                    (home, count)
                })
                .inner
            })
            .inner
        })
        .inner;
    match count {
        Some(count) => list(ui.ctx(), &bar.alerts, (&count, list_open), lang),
        None => *list_open = false,
    }
    home
}

/// The list of the part's alerts, hung under their count.
fn list(
    ctx: &egui::Context,
    alerts: &[Alert],
    count: (&egui::Response, &mut bool),
    lang: &Catalogue,
) {
    let (heading, close) = (lang.t("alerts.heading"), lang.t("alerts.close"));
    let actions = (lang.t("alerts.show"), lang.t("alerts.modify"));
    let id = egui::Id::new("part_alerts");
    sheet_under(ctx, id, count, (LIST_WIDTH, &heading, &close), |ui| {
        for alert in alerts {
            // Keyed by what it is about, so that a row keeps its own buttons as
            // the list follows the part and the rows above it come and go.
            ui.push_id(alert.about, |ui| {
                let said = (alert.title.as_str(), alert.sentence.as_str());
                alert_line(ui, alert.level, said, (&actions.0, &actions.1));
            });
        }
    });
}
