//! Preferences › Apparence, driven whole in the application: the viewport's
//! colours edited on that page, and read back from the profile it writes.
//!
//! Closes #724.
//! - Preferences › Apparence still overrides every viewport colour — and the
//!   sketch's own background, new with this issue, has a background editor of
//!   its own there, apart from the free view's —
//!   `the_sketch_background_is_edited_apart_from_the_free_view_s`
//! - and its reset still resets, the sketch's background included —
//!   `the_reset_puts_every_viewport_colour_back_the_sketch_background_included`
//!
//! Which of the two background editors is the sketch's is read off what they
//! show rather than off where they stand: by default the free view is a
//! radial gradient and a sketch a plain colour, so the editor whose « Uni » is
//! chosen is the sketch's, wherever the page puts it.

mod driver;

use cao_prefs::{Background, Rgba, Theme};
use egui_kittest::kittest::{NodeT, Queryable};

#[test]
fn the_sketch_background_is_edited_apart_from_the_free_view_s() {
    let test = "the_sketch_background_is_edited_apart";
    let mut app = driver::open(test);
    the_appearance_page(&mut app);

    let editors = app.get_all_by_label("Dégradé linéaire").count();
    assert_eq!(
        editors, 2,
        "Préférences › Apparence edits {editors} background(s), where the free view and \
         a sketch have one each",
    );
    let the_sketch_s_row = app
        .get_all_by_label("Uni")
        .find(|kind| kind.accesskit_node().toggled() == Some(egui::accesskit::Toggled::True))
        .expect("the sketch's background, a plain colour by default, shows « Uni » chosen")
        .rect()
        .center()
        .y;
    let linear = app
        .get_all_by_label("Dégradé linéaire")
        .position(|kind| (kind.rect().center().y - the_sketch_s_row).abs() < 1.0)
        .expect("the sketch's editor offers a linear gradient on the same row");
    press(&mut app, "Dégradé linéaire", linear);

    let theme = driver::saved_settings(test).theme;
    assert!(
        matches!(theme.sketch_background, Background::Linear { .. }),
        "the sketch's background was turned into a linear gradient, and reads {:?}",
        theme.sketch_background,
    );
    assert_eq!(
        theme.background,
        Theme::default().background,
        "editing the sketch's background moved the free view's",
    );
}

#[test]
fn the_reset_puts_every_viewport_colour_back_the_sketch_background_included() {
    let test = "the_reset_puts_every_viewport_colour_back";
    let mut settings = cao_prefs::Settings::default();
    settings.theme.background = Background::Solid(Rgba::opaque(0.30, 0.10, 0.20));
    settings.theme.sketch_background = Background::Solid(Rgba::opaque(0.10, 0.30, 0.20));
    settings.theme.axis_x = Rgba::opaque(0.20, 0.20, 0.20);
    settings.theme.grid_minor = Rgba::opaque(0.40, 0.40, 0.40);
    settings.theme.solid = Rgba::opaque(0.60, 0.60, 0.60);
    let mut app = driver::open_with(test, driver::SCREEN, settings);
    the_appearance_page(&mut app);

    press(&mut app, "↺ Couleurs par défaut", 0);

    assert_eq!(
        driver::saved_settings(test).theme,
        Theme::default(),
        "the reset left a viewport colour of the profile behind",
    );
}

/// Reached from the ribbon of a part: the start menu's « ⚙ Préférences » is
/// laid out below the bottom of the screen, where no click reaches it.
fn the_appearance_page(app: &mut driver::App) {
    driver::create_a_part(app, "Teintes");
    driver::click_exactly(app, "⚙");
    driver::click(app, "Apparence");
}

/// The `rank`-th widget of the page reading `label`, brought into view first:
/// the page scrolls, and much of it stands below the window's edge.
fn press(app: &mut driver::App, label: &str, rank: usize) {
    assert!(
        app.get_all_by_label(label).nth(rank).is_some(),
        "no « {label} » number {rank} on the page",
    );
    if let Some(widget) = app.get_all_by_label(label).nth(rank) {
        widget.scroll_to_me();
    }
    app.run();
    if let Some(widget) = app.get_all_by_label(label).nth(rank) {
        widget.click_accesskit();
    }
    app.run();
}
