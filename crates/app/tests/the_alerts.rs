//! The part's problems as alerts, driven through the whole application: a
//! count on the right of the title bar, and the list it opens under it
//! (`docs/redesign/part-screen.md` P2, screen
//! `docs/redesign/screens/05-part-alerts-open.png`).
//!
//! Closes #736 — the issue has no Done when of its own; these are the points of
//! its « What it does » and « What must not break », approved as its criteria
//! when the work opened, a long point split into its parts, and the two
//! decisions Tom took then, the last two bullets of « What it does ». The
//! bullets the title bar's presenter holds are transcribed again, checked, in
//! `crates/app/src/screens/title_bar/state/tests.rs`.
//!
//! What it does:
//! - Two levels. Warning, yellow: an unstable case (`unstable_at`); a sketch
//!   whose face is lost and that stays where it was (`is_adrift`). Error, red:
//!   a volume not built (`is_declined`); a lost area — no test: held by
//!   `each_problem_of_the_part_is_an_alert_at_its_level`, in
//!   `crates/app/src/screens/title_bar/state/tests.rs`, which reads the level
//!   of each problem's alert off the presenter. The red a level is drawn in is
//!   held on the count by the bullet after next, and the yellow with the
//!   badges' colours, below
//! - a kept section whose face is lost, which wears « face perdue » in the tree
//!   today, is a warning like a sketch off its face — no test: held by
//!   `each_problem_of_the_part_is_an_alert_at_its_level`, as above. Not one of
//!   the issue's words: read from the first point of « What must not break »,
//!   and from P2, whose count is of every element wearing a badge
//! - anything that stops the computation is an error — no test: nothing stops
//!   the computation today. A step the kernel declines leaves the part as it
//!   stood before it and the rest is computed, which is the volume not built
//!   above; a new kind of problem is out of scope
//! - « ⚠ N alertes » on the right of the title bar, only when N > 0, in the
//!   colour of the worst level —
//!   `the_count_stands_on_the_right_of_the_title_bar_in_the_colour_of_the_worst_alert`;
//!   the count, its singular « 1 alerte » and the level it takes are held case
//!   by case beside the title bar's presenter, by
//!   `the_title_bar_counts_the_alerts_only_while_there_is_one` and
//!   `the_count_wears_the_level_of_the_worst_alert`
//! - A click opens the list (340 px) under the button —
//!   `a_click_on_the_count_opens_the_list_340_points_wide_under_it`
//! - Each alert: the warning or error icon, « Élément · problème »
//!   (« Extrusion 3 · instable »), one sentence of explanation —
//!   `each_alert_reads_its_icon_its_element_and_problem_and_a_sentence`
//! - then « Montrer » (selects the element and lights it in the view) —
//!   `show_selects_the_element_and_lights_it_in_the_view`
//! - and « Modifier » (reopens the step) —
//!   `modify_reopens_the_step_the_alert_is_about`
//! - The list follows the part live — `the_open_list_follows_the_part_live`
//! - ✕, Escape or a click elsewhere closes it —
//!   `the_cross_escape_or_a_click_elsewhere_closes_the_list`
//! - What describes the part leaves the message pill: an extrusion not built
//!   or unstable is an alert —
//!   `an_extrusion_not_built_or_unstable_is_an_alert_and_no_message`
//! - and stays while it is true —
//!   `an_alert_stays_while_its_problem_holds_and_goes_with_it`
//! - The pill keeps what answers a gesture —
//!   `an_extrusion_that_changes_nothing_still_says_so_in_the_pill`
//! - The same two colours for the tree's badges —
//!   `a_badge_and_an_alert_of_one_level_wear_the_same_colours`
//! - and, later, the reader's boxes — no test: out of scope here, as the issue
//!   says; the step reader (P7) does not exist yet, and takes the two colours
//!   when it is built
//! - Preferences › Apparence can set and reset them —
//!   `the_alert_colours_are_set_and_reset_in_preferences_appearance`
//! - decided by Tom when the work opened: « Modifier » on an extrusion's alert
//!   reopens the extrusion, its panel filled with its values, where no
//!   extrusion could be reopened before —
//!   `modify_on_an_extrusion_s_alert_reopens_it_with_its_values`
//! - decided by Tom when the work opened: « Montrer » on a volume's alert
//!   selects the volume and lights it in the view —
//!   `show_on_a_volume_s_alert_selects_the_volume_and_lights_it`
//!
//! What must not break:
//! - Every problem detected today is still shown: as an alert now — no test:
//!   held by `each_problem_of_the_part_is_an_alert_at_its_level`, in
//!   `crates/app/src/screens/title_bar/state/tests.rs`, one part for each
//!   problem the tree wears a badge for today; on screen, the count of
//!   `the_count_stands_on_the_right_of_the_title_bar_in_the_colour_of_the_worst_alert`
//!   takes an area lost and a face lost together
//! - The refusal at the moment of the gesture (« Cette extrusion ne change
//!   rien ») still shows in the pill —
//!   `an_extrusion_that_changes_nothing_still_says_so_in_the_pill`
//!
//! The « ⚠ » of the count is the `warning` icon, the triangle the mockup's
//! button wears (`docs/redesign/source/part.html`), and the count is a button
//! named by its words alone, « 2 alertes », as the mockup names it. The colour
//! of a level is the pair the tree's badges are drawn in: `state.error_text` on
//! `state.error_background`, `state.warning_text` on
//! `state.warning_background`.
//!
//! A part with an error and a warning is made as `the_part_panel.rs` makes a
//! sketch lose its face: a block, a sketch laid on its face, then the block's
//! own sketch emptied. The block's extrusion is left with no area, and the
//! sketch on its face with no face.

mod driver;

use cao_app::theme::tokens;
use egui::accesskit::Role;
use egui_kittest::kittest::{NodeT, Queryable};
use glam::{DVec2, DVec3};

#[test]
fn the_count_stands_on_the_right_of_the_title_bar_in_the_colour_of_the_worst_alert() {
    let mut app = driver::a_block("the_count_stands_on_the_right_of_the_title_bar");
    lay_a_sketch_on_the_block_s_face(&mut app);
    let before = the_title_bar(&app);

    empty_the_block_s_sketch(&mut app);
    assert!(
        driver::says(&app, "aire perdue") && driver::says(&app, "face perdue"),
        "the block's sketch emptied, the tree does not wear « aire perdue » and « face \
         perdue », which leaves the count nothing to count: {:?}",
        driver::said(&app),
    );

    let mut wrong = Vec::new();
    let counted_before: Vec<&String> = before
        .iter()
        .filter(|read| read.contains("alerte"))
        .collect();
    if !counted_before.is_empty() {
        wrong.push(format!(
            "with nothing wrong with the part, the title bar already reads {counted_before:?}"
        ));
    }
    match the_button_reading(&app, "2 alertes") {
        None => wrong.push(format!(
            "no button reads « 2 alertes »; the title bar reads {:?}",
            the_title_bar(&app),
        )),
        Some(count) => wrong.extend(not_on_the_right_in_red(&app, count)),
    }

    assert!(
        wrong.is_empty(),
        "an area lost and a face lost are not counted as the redesign draws it:\n  {}",
        wrong.join("\n  "),
    );
}

/// What keeps the count at `count` from standing at the right end of the title
/// bar, wearing the warning triangle, written in `state.error_text` on
/// `state.error_background`.
fn not_on_the_right_in_red(app: &driver::App, count: egui::Rect) -> Vec<String> {
    let mut wrong = Vec::new();
    let home = the_topmost_reading(app, "Accueil");
    if !home.y_range().contains(count.center().y) {
        wrong.push(format!(
            "the count at {count:?} is not on the title bar's row, « Accueil » at {home:?}"
        ));
    }
    let window = app.ctx.content_rect();
    if count.left() < window.center().x || window.right() - count.right() > AT_THE_RIGHT_EDGE {
        wrong.push(format!(
            "the count stands at {count:?}, not at the right end of a window {window:?}"
        ));
    }
    let worn = icons_painted_within(app, count);
    if !worn.iter().any(|icon| icon == "warning") {
        wrong.push(format!(
            "the count at {count:?} wears {worn:?}, not the `warning` triangle"
        ));
    }
    let ink = colours_written_in(app, "2 alertes");
    if ink.is_empty() || ink.iter().any(|colour| *colour != tokens::STATE_ERROR_TEXT) {
        wrong.push(format!(
            "« 2 alertes » is written in {ink:?}, where `state.error_text` is {:?}",
            tokens::STATE_ERROR_TEXT,
        ));
    }
    let filled = fills_around(app, count);
    if !filled.contains(&tokens::STATE_ERROR_BACKGROUND) {
        wrong.push(format!(
            "the count at {count:?} stands on {filled:?}, where `state.error_background` \
             is {:?}",
            tokens::STATE_ERROR_BACKGROUND,
        ));
    }
    wrong
}

/// How far from the window's right edge the count may end, in points: the
/// title bar's margin, and some.
const AT_THE_RIGHT_EDGE: f32 = 32.0;

/// How far a frame may sit inside the widget it is drawn around, in points:
/// the rect the tree gives a widget and the frame painted around it are
/// rounded apart.
const HUGGED_WITHIN: f32 = 0.5;

/// On the face of the block at x = 10, the one `keeping_a_section.rs` clicks
/// to lay a section there.
const ON_THE_BLOCK_S_FACE: DVec3 = DVec3::new(10.0, 41.51, -19.82);

/// A point of the block's sketch away from what is drawn on it, where the hand
/// rests while the selection is deleted.
const AWAY_FROM_THE_BLOCK_S_TRAITS: DVec2 = DVec2::new(173.9, -56.77);

/// « Esquisse 2 », laid on the block's face, and finished.
fn lay_a_sketch_on_the_block_s_face(app: &mut driver::App) {
    driver::click(app, "Nouvelle esquisse");
    driver::click_at_world_point(app, ON_THE_BLOCK_S_FACE);
    driver::finish_the_sketch(app);
}

/// Everything drawn on « Esquisse 1 », the block's sketch, deleted, and the
/// sketch finished: the block's extrusion loses its area, and the sketch on
/// its face loses the face.
fn empty_the_block_s_sketch(app: &mut driver::App) {
    driver::double_click(app, "Esquisse 1");
    driver::click(app, "Sélection (S)");
    driver::press_with_command(app, egui::Key::A);
    app.hover_at(driver::sketch_point_on_screen(
        app,
        AWAY_FROM_THE_BLOCK_S_TRAITS,
    ));
    app.run();
    driver::press(app, egui::Key::Delete);
    driver::finish_the_sketch(app);
}

/// Where the button named exactly this stands.
fn the_button_reading(app: &driver::App, name: &str) -> Option<egui::Rect> {
    app.query_all_by_label(name)
        .find(|node| node.accesskit_node().role() == Role::Button)
        .map(|node| node.rect())
}

/// What the title bar reads, left to right: the row of « Accueil ».
fn the_title_bar(app: &driver::App) -> Vec<String> {
    the_row_of(app, the_topmost_reading(app, "Accueil"))
}

/// Every name and every text whose middle stands within the height of `on`,
/// left to right.
fn the_row_of(app: &driver::App, on: egui::Rect) -> Vec<String> {
    let mut read: Vec<(f32, String)> = app
        .get_all_by(|_| true)
        .filter_map(|node| {
            let read = node.accesskit_node();
            read.bounding_box()?;
            if read.role() == Role::TextRun {
                return None;
            }
            let said = read
                .label()
                .or_else(|| read.value())
                .filter(|said| !said.is_empty())?;
            let at = node.rect();
            on.y_range()
                .contains(at.center().y)
                .then_some((at.left(), said))
        })
        .collect();
    read.sort_by(|one, other| one.0.total_cmp(&other.0));
    read.into_iter().map(|(_, said)| said).collect()
}

/// The highest widget on screen named or reading exactly this.
fn the_topmost_reading(app: &driver::App, text: &str) -> egui::Rect {
    app.query_all_by(|node| {
        node.label().as_deref() == Some(text) || node.value().as_deref() == Some(text)
    })
    .map(|node| node.rect())
    .min_by(|one, other| one.top().total_cmp(&other.top()))
    .unwrap_or_else(|| {
        panic!(
            "nothing on screen reads « {text} »: {:?}",
            driver::on_screen(app)
        )
    })
}

/// Every shape the last frame painted, groups opened.
fn shapes_painted(app: &driver::App) -> Vec<egui::Shape> {
    let mut found = Vec::new();
    let mut pending: Vec<&egui::Shape> = app
        .output()
        .shapes
        .iter()
        .map(|clipped| &clipped.shape)
        .collect();
    while let Some(shape) = pending.pop() {
        match shape {
            egui::Shape::Vec(group) => pending.extend(group),
            shape => found.push(shape.clone()),
        }
    }
    found
}

/// Every colour a text reading exactly this is painted in.
fn colours_written_in(app: &driver::App, text: &str) -> Vec<egui::Color32> {
    let mut found = Vec::new();
    for shape in shapes_painted(app) {
        let egui::Shape::Text(written) = shape else {
            continue;
        };
        if written.galley.text() != text {
            continue;
        }
        for section in &written.galley.job.sections {
            found.push(match written.override_text_color {
                Some(colour) => colour,
                None if section.format.color == egui::Color32::PLACEHOLDER => {
                    written.fallback_color
                }
                None => section.format.color,
            });
        }
    }
    found
}

/// The fill of every rectangle the last frame painted around this one.
fn fills_around(app: &driver::App, within: egui::Rect) -> Vec<egui::Color32> {
    shapes_painted(app)
        .into_iter()
        .filter_map(|shape| match shape {
            egui::Shape::Rect(rect)
                if rect.fill.a() > 0 && rect.rect.expand(HUGGED_WITHIN).contains_rect(within) =>
            {
                Some(rect.fill)
            }
            _ => None,
        })
        .collect()
}

/// The ids of the icons the last frame painted inside a rectangle of the
/// screen, by the file each was loaded from — `bytes://icon/warning.svg` is
/// `warning`.
fn icons_painted_within(app: &driver::App, within: egui::Rect) -> Vec<String> {
    let within = within.expand(1.0);
    let textures = app.ctx.tex_manager();
    let textures = textures.read();
    shapes_painted(app)
        .into_iter()
        .filter_map(|shape| match shape {
            egui::Shape::Rect(rect)
                if rect.fill_texture_id() != egui::TextureId::default()
                    && within.contains_rect(rect.rect) =>
            {
                let loaded_as = textures
                    .meta(rect.fill_texture_id())
                    .map(|meta| meta.name.clone())
                    .unwrap_or_default();
                let file = loaded_as.rsplit('/').next().unwrap_or(&loaded_as);
                Some(file.strip_suffix(".svg").unwrap_or(file).to_owned())
            }
            _ => None,
        })
        .collect()
}
