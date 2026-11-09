//! The ribbon drawn as named groups of large and small buttons, split buttons
//! for the variants, and a tooltip on each — read back from the accessibility
//! tree and from what the frame painted.
//!
//! Closes #731. What a group stores — its size, and the commands of the default
//! layout — is held in `crates/prefs/src/toolbar/tests.rs`; the large and
//! small buttons (C1, C2) and the grid of small ones, beside their primitives
//! in `crates/app/src/ui/tool_button/tests.rs`.
//! - Group (C3): its buttons, and its name under them —
//!   `a_group_writes_its_name_under_its_buttons`
//! - in small capitals — `a_group_writes_its_name_in_small_capitals`
//! - with no frame around it — `a_group_has_no_frame_around_it`
//! - its name stays an accessible name, so a test can find it —
//!   `a_group_s_name_stays_an_accessible_name`
//! - a thin separator between groups, with a margin of 10 px —
//!   `a_thin_separator_stands_between_two_groups`
//! - and none after the last one — `no_separator_follows_the_last_group`
//! - the size of a group decides how its commands are drawn, and the default
//!   `edit` is small on 2 rows —
//!   `the_edit_group_stacks_its_two_buttons_on_two_rows_of_26`
//! - small on 3 rows — `a_group_on_three_rows_draws_its_buttons_20_tall_in_columns_of_three`
//! - large — `a_large_group_draws_large_buttons`
//! - and commands sitting directly in a tab are large —
//!   `a_command_sitting_directly_in_a_tab_is_large`
//! - below 1440 px the ribbon wraps group by group rather than button by button
//!   (#650) — `below_1440_the_ribbon_wraps_group_by_group_rather_than_button_by_button`
//! - a bar wide enough holds its groups on one line —
//!   `a_bar_wide_enough_lays_all_its_groups_on_one_line`
//! - decided: a group wider than the whole bar wraps inside itself as a last
//!   resort — `a_group_wider_than_the_whole_bar_wraps_inside_itself_as_a_last_resort`
//! - Split button for a group of variants (« Cercle », « Arc », « Ellipse »,
//!   « Chanfrein », « Contraintes », « Exporter en STL ») —
//!   `every_group_of_variants_is_drawn_as_one_split_button`
//! - a click on the icon runs the tool as today's « Cercle » does —
//!   `a_click_on_the_icon_of_a_split_button_runs_its_tool`
//! - a click on the chevron opens the menu (C8) —
//!   `a_click_on_the_chevron_opens_the_menu_of_the_variants`
//! - 230 px wide — `the_menu_of_a_split_button_is_230_wide`
//! - each line with a monospace hint on the right —
//!   `each_line_of_a_split_button_s_menu_writes_a_monospace_hint_on_its_right`
//! - decided: the chevron's accessible label is the group's name, « Cercles » —
//!   `the_chevron_of_a_split_button_answers_to_the_name_of_its_group`
//! - decided: a menu-depth group with no command before it is split too, with
//!   a primary command of its own —
//!   `a_menu_with_no_command_before_it_is_split_around_a_primary_command_of_its_own`
//! - decided: the icon of « Exporter en STL… » runs the normal level —
//!   `a_click_on_the_icon_of_the_stl_export_runs_the_normal_level`
//! - decided: its menu writes « Grossière » « rapide », « Normale »
//!   « par défaut », « Fine » « lourd » —
//!   `each_stl_level_writes_its_hint_on_its_line`
//! - decided: the other variant menus' hint column shows the entry's key if it
//!   has one, else nothing —
//!   `the_hint_of_a_variant_is_its_key_if_it_has_one_and_nothing_otherwise`
//! - decided: the icon of « Contraintes » re-applies the last rule used from
//!   that menu — `a_click_on_the_constraints_icon_reapplies_the_last_rule_used`
//! - decided: until one was used, the first —
//!   `until_a_rule_was_used_the_constraints_icon_runs_the_first`
//! - decided: the button keeps the name « Contraintes », and its icon is that
//!   of the rule it would run —
//!   `the_constraints_button_keeps_its_name_and_wears_the_icon_of_the_rule_it_would_run`
//! - decided: that rule is remembered for the session, not in the settings
//!   file — `the_rule_the_constraints_icon_remembers_is_not_written_to_the_settings`
//! - Tooltip (C9): the name, the key in a frame (C14), one sentence of help —
//!   `hovering_a_button_shows_its_name_its_key_in_a_frame_and_a_sentence_of_help`
//! - missing sentences are written in this issue — no test: every command
//!   already has a hint in `crates/app/src/wording/command.rs`, so no sentence
//!   is missing and none is written
//! - with « Noms des boutons » off: icons only —
//!   `with_names_off_every_ribbon_button_is_its_icon_alone`
//! - down the left or right: groups stack — `down_a_side_the_groups_stack`
//! - every button small — `down_a_side_every_button_is_small`
//! - floating: as at the top — `a_floating_ribbon_is_laid_out_as_along_the_top`
//! - every button keeps its name as its accessible label —
//!   `every_ribbon_button_keeps_its_name_as_its_accessible_label`
//! - toolbar positions (top, bottom, left, right, floating) —
//!   `the_ribbon_is_drawn_at_each_of_the_five_toolbar_positions`
//! - the dock / undock command —
//!   `the_dock_command_takes_the_bar_off_its_edge_and_back`
//!
//! Decided on the way. The tree does not change: « Cercle » and the group
//! `circles` stay two siblings, and a split button is a rule of the drawing —
//! a command immediately followed by a menu-depth group is one button, the
//! command's icon, name and key, its chevron opening that group. A group wider
//! than the whole bar may wrap inside itself as a last resort, because the
//! default content stays as it is until the next issue and
//! `fitting_the_window.rs` (#650) must stay true.

mod driver;

use cao_app::theme::tokens::{BORDER_GROUP, TEXT_QUIET};
use cao_prefs::{Command, GroupSize, Item, Settings};
use egui::epaint::TextShape;

/// How far two things laid out by egui may be apart and still be aligned:
/// frames and galleys are placed on rounded points.
const ALIGNED_WITHIN: f32 = 1.0;

const WIDE: egui::Vec2 = egui::vec2(1920.0, 1000.0);

#[test]
fn a_group_writes_its_name_under_its_buttons() {
    let app = opened("a_group_writes_its_name_under", WIDE, Settings::default());

    for (name, buttons) in [
        ("ÉDITION", ["Annuler", "Rétablir"].as_slice()),
        ("DESSIN", ["Ligne"].as_slice()),
    ] {
        let written = written_whatever_its_case(&app, name);
        let lowest = buttons
            .iter()
            .map(|button| the_ribbon_button(&app, button).bottom())
            .fold(f32::NEG_INFINITY, f32::max);

        assert!(
            written.rect.top() >= lowest,
            "« {name} » is written at {:?}, above the bottom of its buttons at {lowest}",
            written.rect,
        );
    }
}

#[test]
fn a_group_writes_its_name_in_small_capitals() {
    let app = opened(
        "a_group_writes_its_name_in_small",
        WIDE,
        Settings::default(),
    );

    let written = written(&app, "ÉDITION");
    let format = &written.text.galley.job.sections[0].format;

    assert!(
        (format.font_id.size - 10.0).abs() < 1e-3,
        "the name of a group is written at {} points, where C3 writes it at 10",
        format.font_id.size,
    );
    assert_eq!(ink_of(&written.text), TEXT_QUIET, "the colour of the name");
    assert!(
        (format.extra_letter_spacing - 0.8).abs() < 1e-3,
        "the name of a group is spaced by {}, where C3 asks for 0.08 em of 10 points",
        format.extra_letter_spacing,
    );
}

#[test]
fn a_group_has_no_frame_around_it() {
    let app = opened("a_group_has_no_frame", WIDE, Settings::default());

    let group = the_ribbon_button(&app, "Annuler").union(the_ribbon_button(&app, "Rétablir"));
    let framed: Vec<egui::Rect> = shapes(&app)
        .iter()
        .filter_map(|shape| match shape {
            egui::Shape::Rect(rect)
                if rect.rect.contains_rect(group)
                    && rect.rect.width() < WIDE.x / 2.0
                    && (rect.stroke.width > 0.0 || rect.fill.a() > 0) =>
            {
                Some(rect.rect)
            }
            _ => None,
        })
        .collect();

    assert!(
        framed.is_empty(),
        "a frame is drawn around the group of « Annuler » and « Rétablir » ({group:?}): {framed:?}",
    );
}

#[test]
fn a_group_s_name_stays_an_accessible_name() {
    let app = opened("a_group_s_name_stays_accessible", WIDE, Settings::default());

    assert!(
        driver::said(&app)
            .iter()
            .any(|said| said.to_uppercase() == "ÉDITION"),
        "nothing on the screen says « Édition »: {:?}",
        driver::said(&app),
    );
}

#[test]
fn a_thin_separator_stands_between_two_groups() {
    let app = three_groups("a_thin_separator_stands", WIDE);
    let drawing = extent(&app, &["Rectangle", "Mesurer"]);
    let editing = extent(&app, &["Cote", "Chanfrein"]);
    let export = extent(&app, &["Sélection", "Nouvelle esquisse"]);

    for (left, right) in [(drawing, editing), (editing, export)] {
        let between = rules_between(&app, left, right);
        assert_eq!(
            between.len(),
            1,
            "between the groups at {left:?} and {right:?} stand the rules {between:?}",
        );
        let rule = &between[0];
        assert_eq!(rule.colour, BORDER_GROUP, "the colour of the separator");
        assert!(
            (rule.width - 1.0).abs() < 1e-3,
            "the separator is {} wide, where C3 draws it at 1",
            rule.width,
        );
        for margin in [rule.at - left.right(), right.left() - rule.at] {
            assert!(
                (margin - 10.0).abs() <= 2.0,
                "the separator at {} leaves a margin of {margin} beside the groups at \
                 {left:?} and {right:?}, where C3 asks for 10",
                rule.at,
            );
        }
    }
}

#[test]
fn no_separator_follows_the_last_group() {
    let app = three_groups("no_separator_follows_the_last", WIDE);
    let export = extent(&app, &["Sélection", "Nouvelle esquisse"]);

    let after: Vec<Rule> = vertical_rules(&app)
        .into_iter()
        .filter(|rule| rule.at >= export.right() && rule.runs_beside(export))
        .collect();

    assert!(
        after.is_empty(),
        "beyond the last group at {export:?} stand the rules {after:?}",
    );
}

#[test]
fn the_edit_group_stacks_its_two_buttons_on_two_rows_of_26() {
    let app = opened("the_edit_group_stacks", WIDE, Settings::default());
    let undo = the_ribbon_button(&app, "Annuler");
    let redo = the_ribbon_button(&app, "Rétablir");

    assert_tall(undo, 26.0, "« Annuler »");
    assert_tall(redo, 26.0, "« Rétablir »");
    assert!(
        (undo.left() - redo.left()).abs() <= ALIGNED_WITHIN && redo.top() >= undo.bottom(),
        "« Annuler » at {undo:?} and « Rétablir » at {redo:?} do not stand one above the other",
    );
}

#[test]
fn a_group_on_three_rows_draws_its_buttons_20_tall_in_columns_of_three() {
    let settings = bar_of(vec![Item::Group {
        name: "edit".to_string(),
        size: GroupSize::SmallThreeRows,
        items: commands(&[
            Command::Undo,
            Command::Redo,
            Command::ToolSelect,
            Command::ToolLine,
        ]),
    }]);
    let app = opened("a_group_on_three_rows", WIDE, settings);

    let column = ["Annuler", "Rétablir", "Sélection"].map(|name| the_ribbon_button(&app, name));
    let next = the_ribbon_button(&app, "Ligne");

    for button in column.iter().chain([&next]) {
        assert_tall(*button, 20.0, "a small button");
    }
    assert!(
        (column[0].left() - column[1].left()).abs() <= ALIGNED_WITHIN
            && (column[1].left() - column[2].left()).abs() <= ALIGNED_WITHIN
            && column[1].top() >= column[0].bottom()
            && column[2].top() >= column[1].bottom(),
        "the first three do not stand in one column: {column:?}",
    );
    assert!(
        next.left() >= column[0].left() + ALIGNED_WITHIN
            && (next.top() - column[0].top()).abs() <= ALIGNED_WITHIN,
        "the fourth button at {next:?} does not start the next column after {column:?}",
    );
}

#[test]
fn a_large_group_draws_large_buttons() {
    let app = opened("a_large_group", WIDE, Settings::default());

    let line = the_ribbon_button(&app, "Ligne");
    assert!(
        line.height() >= 64.0,
        "« Ligne », in the large group « Dessin », is {} high",
        line.height(),
    );
}

#[test]
fn a_command_sitting_directly_in_a_tab_is_large() {
    let app = opened("a_command_sitting_directly", WIDE, Settings::default());

    let new_sketch = the_ribbon_button(&app, "Nouvelle esquisse");
    assert!(
        new_sketch.height() >= 64.0,
        "« Nouvelle esquisse », directly in its tab, is {} high",
        new_sketch.height(),
    );
}

#[test]
fn below_1440_the_ribbon_wraps_group_by_group_rather_than_button_by_button() {
    let app = three_groups("below_1440_wraps", egui::vec2(900.0, 800.0));
    let lines = line_of_each_group(&app);

    for (name, tops) in &lines {
        assert!(
            tops.iter()
                .all(|top| (top - tops[0]).abs() <= ALIGNED_WITHIN),
            "the group {name} is cut in two, its buttons at the heights {tops:?}",
        );
    }
    assert!(
        lines.last().expect("three groups").1[0] > lines[0].1[0] + ALIGNED_WITHIN,
        "at 900 wide no group went to the next line: {lines:?}",
    );
}

#[test]
fn a_bar_wide_enough_lays_all_its_groups_on_one_line() {
    let app = three_groups("a_bar_wide_enough", egui::vec2(1500.0, 800.0));
    let lines = line_of_each_group(&app);

    let first = lines[0].1[0];
    assert!(
        lines
            .iter()
            .flat_map(|(_, tops)| tops)
            .all(|top| (top - first).abs() <= ALIGNED_WITHIN),
        "at 1500 wide the groups are not on one line: {lines:?}",
    );
}

#[test]
fn a_group_wider_than_the_whole_bar_wraps_inside_itself_as_a_last_resort() {
    let size = egui::vec2(1024.0, 768.0);
    let app = opened("a_group_wider_than_the_bar", size, Settings::default());

    let buttons = [
        "Sélection",
        "Rectangle",
        "Point",
        "Cote",
        "Mesurer",
        "Ajuster",
        "Diviser",
        "Symétrie",
        "Congé",
    ]
    .map(|name| the_ribbon_button(&app, name));
    let inside = buttons.iter().all(|button| button.right() <= size.x + 0.5);
    let tops: Vec<f32> = buttons.iter().map(|button| button.top()).collect();

    assert!(
        inside,
        "a button of « Dessin » runs off a bar 1024 wide: {buttons:?}",
    );
    assert!(
        tops.iter()
            .any(|top| (top - tops[0]).abs() > ALIGNED_WITHIN),
        "« Dessin », wider than the bar, stays on one line: {tops:?}",
    );
}

fn opened(test: &str, size: egui::Vec2, settings: Settings) -> driver::App {
    let mut app = driver::open_with(test, size, settings);
    driver::create_a_part(&mut app, "Platine");
    app
}

fn commands(commands: &[Command]) -> Vec<Item> {
    commands.iter().copied().map(Item::Command).collect()
}

fn bar_of(groups: Vec<Item>) -> Settings {
    let mut settings = Settings::default();
    settings.toolbar.items = vec![Item::group("sketch", groups)];
    settings
}

/// A bar holding three large groups, each too wide to sit beside the others on
/// a window under 1440 and narrower than the whole bar.
fn three_groups(test: &str, size: egui::Vec2) -> driver::App {
    use Command as C;
    let settings = bar_of(vec![
        Item::group(
            "drawing",
            commands(&[
                C::ToolRectangle,
                C::ToolCircle,
                C::ToolArc,
                C::ToolEllipse,
                C::ToolPoint,
                C::ToolMeasure,
            ]),
        ),
        Item::group(
            "edit",
            commands(&[
                C::ToolDimension,
                C::ToolTrim,
                C::ToolSplit,
                C::ToolMirror,
                C::ToolFillet,
                C::ToolChamfer,
            ]),
        ),
        Item::group(
            "export",
            commands(&[C::ToolSelect, C::ToolLine, C::Undo, C::Redo, C::NewSketch]),
        ),
    ]);
    opened(test, size, settings)
}

fn line_of_each_group(app: &driver::App) -> Vec<(&'static str, Vec<f32>)> {
    [
        (
            "« Dessin »",
            ["Rectangle", "Cercle", "Arc", "Ellipse", "Point", "Mesurer"].as_slice(),
        ),
        (
            "« Édition »",
            [
                "Cote",
                "Ajuster",
                "Diviser",
                "Symétrie",
                "Congé",
                "Chanfrein",
            ]
            .as_slice(),
        ),
        (
            "« Export »",
            ["Sélection", "Ligne", "Annuler", "Rétablir"].as_slice(),
        ),
    ]
    .iter()
    .map(|(name, buttons)| {
        (
            *name,
            buttons
                .iter()
                .map(|button| the_ribbon_button(app, button).top())
                .collect(),
        )
    })
    .collect()
}

fn the_ribbon_button(app: &driver::App, name: &str) -> egui::Rect {
    driver::the_ribbon_s(app, name)
        .unwrap_or_else(|| {
            panic!(
                "the ribbon offers no button « {name} », only {:?}",
                driver::on_screen(app)
            )
        })
        .rect()
}

fn assert_tall(button: egui::Rect, height: f32, what: &str) {
    assert!(
        (button.height() - height).abs() < ALIGNED_WITHIN,
        "{what} is {} high, where C2 asks for {height}",
        button.height(),
    );
}

/// Where a group stands: the width of the buttons named.
fn extent(app: &driver::App, buttons: &[&str]) -> egui::Rect {
    buttons.iter().fold(egui::Rect::NOTHING, |group, button| {
        group.union(the_ribbon_button(app, button))
    })
}

fn shapes(app: &driver::App) -> Vec<egui::Shape> {
    let mut pending: Vec<egui::Shape> = app
        .output()
        .shapes
        .iter()
        .map(|clipped| clipped.shape.clone())
        .collect();
    let mut shapes = Vec::new();
    while let Some(shape) = pending.pop() {
        match shape {
            egui::Shape::Vec(group) => pending.extend(group),
            shape => shapes.push(shape),
        }
    }
    shapes
}

struct Written {
    rect: egui::Rect,
    text: TextShape,
}

fn written(app: &driver::App, exactly: &str) -> Written {
    written_where(app, exactly, |read| read == exactly)
}

fn written_whatever_its_case(app: &driver::App, words: &str) -> Written {
    written_where(app, words, |read| read.to_uppercase() == words)
}

fn written_where(app: &driver::App, words: &str, reads: impl Fn(&str) -> bool) -> Written {
    shapes(app)
        .into_iter()
        .find_map(|shape| match &shape {
            egui::Shape::Text(text) if reads(text.galley.text()) => Some(Written {
                rect: shape.visual_bounding_rect(),
                text: text.clone(),
            }),
            _ => None,
        })
        .unwrap_or_else(|| {
            let read: Vec<String> = shapes(app)
                .iter()
                .filter_map(|shape| match shape {
                    egui::Shape::Text(text) => Some(text.galley.text().to_owned()),
                    _ => None,
                })
                .collect();
            panic!("nothing on the screen is written « {words} », only {read:?}")
        })
}

/// The colour a text's glyphs reach the screen in.
fn ink_of(text: &TextShape) -> egui::Color32 {
    text.override_text_color.unwrap_or_else(|| {
        let colour = text.galley.job.sections[0].format.color;
        if colour == egui::Color32::PLACEHOLDER {
            text.fallback_color
        } else {
            colour
        }
    })
}

/// A vertical stroke of the frame: what egui draws for a separator or a thin
/// filled rectangle standing in for one.
#[derive(Debug)]
struct Rule {
    at: f32,
    top: f32,
    bottom: f32,
    colour: egui::Color32,
    width: f32,
}

impl Rule {
    fn runs_beside(&self, group: egui::Rect) -> bool {
        self.top < group.bottom() && self.bottom > group.top()
    }
}

fn vertical_rules(app: &driver::App) -> Vec<Rule> {
    shapes(app)
        .iter()
        .filter_map(|shape| match shape {
            egui::Shape::LineSegment {
                points: [a, b],
                stroke,
            } if (a.x - b.x).abs() < 0.01 && (a.y - b.y).abs() > 1.0 => Some(Rule {
                at: a.x,
                top: a.y.min(b.y),
                bottom: a.y.max(b.y),
                colour: stroke.color,
                width: stroke.width,
            }),
            egui::Shape::Rect(rect)
                if rect.rect.width() <= 1.5 && rect.rect.height() > 1.0 && rect.fill.a() > 0 =>
            {
                Some(Rule {
                    at: rect.rect.center().x,
                    top: rect.rect.top(),
                    bottom: rect.rect.bottom(),
                    colour: rect.fill,
                    width: rect.rect.width(),
                })
            }
            _ => None,
        })
        .collect()
}

fn rules_between(app: &driver::App, left: egui::Rect, right: egui::Rect) -> Vec<Rule> {
    vertical_rules(app)
        .into_iter()
        .filter(|rule| rule.at > left.right() && rule.at < right.left() && rule.runs_beside(left))
        .collect()
}
