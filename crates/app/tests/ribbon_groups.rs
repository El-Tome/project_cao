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
//! - the name, key and icon are the command's —
//!   `a_split_button_keeps_the_name_the_key_and_the_icon_of_its_command`
//! - a click on the icon runs the tool as today's « Cercle » does —
//!   `a_click_on_the_icon_of_a_split_button_runs_its_tool`
//! - the circle mode left alone —
//!   `a_click_on_the_icon_leaves_the_circle_mode_alone`
//! - a click on the chevron opens the menu (C8), running nothing —
//!   `a_click_on_the_chevron_opens_the_menu_of_the_variants`
//! - a click on an entry runs it and closes the menu —
//!   `a_click_on_an_entry_runs_it_and_closes_the_menu`
//! - 230 px wide — `the_menu_of_a_split_button_is_230_wide`
//! - rows of 30 — `each_row_of_a_split_button_s_menu_is_30_tall`
//! - each line with a monospace hint on the right —
//!   `each_line_of_a_split_button_s_menu_writes_a_monospace_hint_on_its_right`
//! - decided: the chevron's accessible label is the group's name, « Cercles »,
//!   or the group's name followed by a chevron when that name is the button's —
//!   `the_chevron_of_a_split_button_answers_to_the_name_of_its_group`
//! - decided: a menu-depth group with no command before it is split too, with
//!   a primary command of its own —
//!   `a_menu_with_no_command_before_it_is_split_around_a_primary_command_of_its_own`
//! - decided: its menu writes « Grossière » « rapide », « Normale »
//!   « par défaut », « Fine » « lourd » —
//!   `each_stl_level_writes_its_hint_on_its_line`
//! - decided: the other variant menus' hint column shows the entry's key if it
//!   has one, else nothing —
//!   `the_hint_of_a_variant_is_its_key_if_it_has_one_and_nothing_otherwise`
//! - a split button in a small group is small, its key on the column's right —
//!   `a_split_button_in_a_small_group_is_small_and_keeps_its_key_on_the_column_s_right`
//! - decided: the icon of « Contraintes » re-applies the last rule used from
//!   that menu — `a_click_on_the_constraints_icon_reapplies_the_last_rule_used`
//! - decided: until one was used, the first —
//!   `until_a_rule_was_used_the_constraints_icon_runs_the_first`
//! - decided: the button keeps the name « Contraintes », and its icon is that
//!   of the rule it would run —
//!   `the_constraints_button_keeps_its_name_and_wears_the_icon_of_the_rule_it_would_run`
//! - decided: that rule is remembered for the session, not in the settings
//!   file — `the_rule_the_constraints_icon_remembers_is_not_written_to_the_settings`
//! - the icon part is greyed out while its command cannot be run —
//!   `the_icon_part_of_a_split_button_follows_its_command`
//! - the chevron stays clickable when any entry is enabled, even if the
//!   primary is greyed out —
//!   `the_chevron_stays_clickable_while_its_primary_is_greyed_out`
//! - amber when the command it runs is the active one —
//!   `a_split_button_is_on_while_the_command_it_runs_is_in_hand`
//! - Tooltip (C9): the name, the key in a frame (C14), one sentence of help —
//!   `hovering_a_button_shows_its_name_its_key_in_a_frame_and_a_sentence_of_help`
//! - a command with no key shows no frame —
//!   `a_command_with_no_key_shows_no_frame_in_its_tooltip`
//! - a small button shows the same tooltip —
//!   `a_small_button_shows_the_same_tooltip`
//! - the body of a split button shows its command's —
//!   `the_body_of_a_split_button_shows_the_tooltip_of_its_command`
//! - « Contraintes » keeps its name, with the help of the rule it would run —
//!   `the_tooltip_of_the_constraints_body_says_the_help_of_the_rule_it_would_run`
//! - with names off, the tooltip still names the button and its key —
//!   `with_names_off_the_tooltip_still_names_the_button_and_its_key`
//! - a chevron shows the group's name and no help —
//!   `hovering_a_chevron_shows_only_the_name_of_its_group`
//! - menu rows keep showing the entry's help —
//!   `hovering_a_menu_row_shows_its_help`
//! - a greyed-out button keeps today's behaviour, read as it stands —
//!   `a_greyed_out_button_shows_no_tooltip`
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

use cao_app::theme::tokens::{BORDER_GROUP, BORDER_KEY, TEXT_QUIET};
use cao_prefs::{Chord, Command, GroupSize, Item, Key, Settings};
use egui::accesskit::{Role, Toggled};
use egui::epaint::TextShape;
use egui_kittest::kittest::{NodeT, Queryable};

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

const CIRCLE_ROWS: [&str; 5] = [
    "Centre et diamètre",
    "Deux points du bord",
    "Deux points puis le centre",
    "Tangent à deux droites",
    "Tangent à trois droites",
];

#[test]
fn every_group_of_variants_is_drawn_as_one_split_button() {
    let mut app = sketching("every_group_of_variants_is_one_split");

    for (primary, chevron) in [
        ("Cercle (C)", "Cercles"),
        ("Arc", "Arcs"),
        ("Ellipse", "Ellipses"),
        ("Chanfrein", "Chanfreins"),
        ("Contraintes", "Contraintes ⌄"),
    ] {
        assert_one_split_button(&app, primary, chevron);
    }

    driver::click_the_button(&mut app, "Export");
    assert_one_split_button(&app, "Exporter en STL…", "Exporter en STL… ⌄");
}

#[test]
fn a_split_button_keeps_the_name_the_key_and_the_icon_of_its_command() {
    let app = sketching("a_split_keeps_the_command_s_face");

    assert_eq!(
        app.query_all_by_label("Cercle (C)").count(),
        1,
        "one button is named « Cercle (C) »",
    );
    assert_eq!(
        icons_within(&app, the_button(&app, "Cercle (C)").rect()),
        ["circle-centre"],
    );
}

#[test]
fn a_click_on_the_icon_of_a_split_button_runs_its_tool() {
    let mut app = sketching("a_click_on_the_icon_runs_its_tool");

    driver::click_exactly(&mut app, "Cercle (C)");

    assert!(is_down(&app, "Cercle (C)"), "« Cercle » is not in hand");
}

#[test]
fn a_click_on_the_icon_leaves_the_circle_mode_alone() {
    let mut app = sketching("a_click_on_the_icon_leaves_the_mode");
    driver::click_exactly(&mut app, "Cercles");
    driver::click_exactly(&mut app, "Deux points du bord");
    driver::click_exactly(&mut app, "Sélection (S)");

    driver::click_exactly(&mut app, "Cercle (C)");
    driver::click_exactly(&mut app, "Cercles");

    assert!(
        is_down(&app, "Deux points du bord"),
        "a click on the icon of « Cercle » changed the circle's mode",
    );
}

#[test]
fn a_click_on_the_chevron_opens_the_menu_of_the_variants() {
    let mut app = sketching("a_click_on_the_chevron_opens");

    driver::click_exactly(&mut app, "Cercles");

    assert!(
        !is_down(&app, "Cercle (C)"),
        "a click on the chevron ran the tool",
    );
    let rows = CIRCLE_ROWS.map(|row| the_button(&app, row).rect().top());
    assert!(
        rows.is_sorted(),
        "the entries are not in the group's order: {rows:?}",
    );
}

#[test]
fn a_click_on_an_entry_runs_it_and_closes_the_menu() {
    let mut app = sketching("a_click_on_an_entry_closes");
    driver::click_exactly(&mut app, "Cercles");

    driver::click_exactly(&mut app, "Deux points du bord");

    assert!(is_down(&app, "Cercle (C)"), "the entry did not run");
    assert_eq!(
        app.query_all_by_label("Deux points du bord").count(),
        0,
        "the menu did not close",
    );
}

#[test]
fn the_menu_of_a_split_button_is_230_wide() {
    let mut app = sketching("the_menu_is_230_wide");
    driver::click_exactly(&mut app, "Cercles");

    let rows = CIRCLE_ROWS.iter().fold(egui::Rect::NOTHING, |all, row| {
        all.union(the_button(&app, row).rect())
    });
    let frame = shapes(&app)
        .iter()
        .filter_map(|shape| match shape {
            egui::Shape::Rect(rect) if rect.fill.a() > 0 && rect.rect.contains_rect(rows) => {
                Some(rect.rect)
            }
            _ => None,
        })
        .min_by(|one, other| one.area().total_cmp(&other.area()))
        .expect("a menu frame painted around its rows");

    assert!(
        (frame.width() - 230.0).abs() <= 1.0,
        "the menu is {} wide, where C8 asks for 230",
        frame.width(),
    );
}

#[test]
fn each_row_of_a_split_button_s_menu_is_30_tall() {
    let mut app = sketching("each_row_is_30_tall");
    driver::click_exactly(&mut app, "Cercles");

    for row in CIRCLE_ROWS {
        assert_tall(the_button(&app, row).rect(), 30.0, "a row of the menu");
    }
}

#[test]
fn each_line_of_a_split_button_s_menu_writes_a_monospace_hint_on_its_right() {
    let app = the_stl_menu_open("each_line_writes_a_hint");

    let row = the_button(&app, "Grossière").rect();
    let name = written_where(&app, "Grossière", |read| read.starts_with("Grossière"));
    let hint = written(&app, "rapide");

    assert_eq!(
        hint.text.galley.job.sections[0].format.font_id.family,
        egui::FontFamily::Monospace,
        "the font of the hint",
    );
    assert!(
        hint.rect.left() > name.rect.right() && row.right() - hint.rect.right() <= 16.0,
        "the hint at {:?} is not on the right of the row at {row:?}, after its name at {:?}",
        hint.rect,
        name.rect,
    );
}

#[test]
fn each_stl_level_writes_its_hint_on_its_line() {
    let app = the_stl_menu_open("each_stl_level_writes_its_hint");

    for (level, hint) in [
        ("Grossière", "rapide"),
        ("Normale", "par défaut"),
        ("Fine", "lourd"),
    ] {
        let row = the_button(&app, level).rect();
        assert!(
            row.expand(1.0).contains_rect(written(&app, hint).rect),
            "« {hint} » is not on the line of « {level} » at {row:?}",
        );
    }
}

#[test]
fn the_hint_of_a_variant_is_its_key_if_it_has_one_and_nothing_otherwise() {
    let mut settings = bar_of(vec![Item::group(
        "drawing",
        vec![
            Item::Command(Command::NewSketch),
            Item::Command(Command::ToolCircle),
            Item::group(
                "circles",
                commands(&[Command::CircleCenter, Command::CircleTwoPoints]),
            ),
        ],
    )]);
    settings
        .shortcuts
        .bind(Command::CircleCenter, Chord::new(Key::K));
    let mut app = opened("the_hint_of_a_variant", WIDE, settings);
    driver::start_a_sketch(&mut app);
    driver::click_exactly(&mut app, "Cercles");

    let keyed = row_starting(&app, "Centre et diamètre").rect();
    let plain = row_starting(&app, "Deux points du bord").rect();

    assert_eq!(
        texts_within(&app, keyed),
        ["Centre et diamètre".to_owned(), "K".to_owned()],
    );
    assert_eq!(
        texts_within(&app, plain),
        ["Deux points du bord".to_owned()],
    );
}

#[test]
fn a_split_button_in_a_small_group_is_small_and_keeps_its_key_on_the_column_s_right() {
    let settings = bar_of(vec![Item::Group {
        name: "edit".to_string(),
        size: GroupSize::SmallThreeRows,
        items: vec![
            Item::Command(Command::ToolLine),
            Item::Command(Command::ToolCircle),
            Item::group(
                "circles",
                commands(&[Command::CircleCenter, Command::CircleTwoPoints]),
            ),
        ],
    }]);
    let app = opened("a_split_in_a_small_group", WIDE, settings);

    let primary = the_button(&app, "Cercle (C)").rect();
    let line_key = written(&app, "L").rect;
    let circle_key = written(&app, "C").rect;

    assert_tall(primary, 20.0, "a small split button");
    assert!(
        (line_key.right() - circle_key.right()).abs() <= ALIGNED_WITHIN,
        "the keys of the column end at {} and {}",
        line_key.right(),
        circle_key.right(),
    );
}

#[test]
fn the_chevron_of_a_split_button_answers_to_the_name_of_its_group() {
    let mut app = sketching("the_chevron_answers_to_its_name");

    the_button(&app, "Cercles");
    the_button(&app, "Contraintes ⌄");

    driver::click_the_button(&mut app, "Export");
    the_button(&app, "Exporter en STL… ⌄");
}

#[test]
fn a_menu_with_no_command_before_it_is_split_around_a_primary_command_of_its_own() {
    let mut app = sketching("a_menu_with_no_command_before");

    driver::click_exactly(&mut app, "Contraintes");

    assert_eq!(
        app.query_all_by_label("Perpendiculaire").count(),
        0,
        "the click on the icon of « Contraintes » opened its menu",
    );
    assert!(
        is_down(&app, "Contraintes"),
        "the click on the icon of « Contraintes » ran nothing",
    );
}

#[test]
fn a_click_on_the_constraints_icon_reapplies_the_last_rule_used() {
    let mut app = sketching("a_click_reapplies_the_last_rule");
    driver::click_exactly(&mut app, "Contraintes ⌄");
    driver::click_exactly(&mut app, "Parallèle");
    driver::click_exactly(&mut app, "Sélection (S)");

    driver::click_exactly(&mut app, "Contraintes");
    driver::click_exactly(&mut app, "Contraintes ⌄");

    assert!(is_down(&app, "Parallèle"), "« Parallèle » is not in hand");
}

#[test]
fn until_a_rule_was_used_the_constraints_icon_runs_the_first() {
    let mut app = sketching("until_a_rule_was_used");

    driver::click_exactly(&mut app, "Contraintes");
    driver::click_exactly(&mut app, "Contraintes ⌄");

    assert!(
        is_down(&app, "Perpendiculaire"),
        "« Perpendiculaire » is not in hand",
    );
}

#[test]
fn the_constraints_button_keeps_its_name_and_wears_the_icon_of_the_rule_it_would_run() {
    let mut app = sketching("the_constraints_button_wears");
    assert_eq!(
        icons_within(&app, the_button(&app, "Contraintes").rect()),
        ["perpendicular"],
    );

    driver::click_exactly(&mut app, "Contraintes ⌄");
    driver::click_exactly(&mut app, "Parallèle");

    assert_eq!(
        icons_within(&app, the_button(&app, "Contraintes").rect()),
        ["parallel"],
    );
}

#[test]
fn the_rule_the_constraints_icon_remembers_is_not_written_to_the_settings() {
    let test = "the_rule_is_not_written_to_the_settings";
    let mut app = sketching(test);

    driver::click_exactly(&mut app, "Contraintes ⌄");
    driver::click_exactly(&mut app, "Parallèle");

    assert_eq!(driver::saved_settings(test), Settings::default());
}

#[test]
fn the_icon_part_of_a_split_button_follows_its_command() {
    let app = opened("the_icon_part_follows", WIDE, Settings::default());

    for primary in ["Cercle (C)", "Contraintes"] {
        assert!(
            the_button(&app, primary).accesskit_node().is_disabled(),
            "« {primary} » can be pressed with no sketch open",
        );
    }
}

#[test]
fn the_chevron_stays_clickable_while_its_primary_is_greyed_out() {
    let settings = bar_of(vec![Item::group(
        "drawing",
        vec![
            Item::Command(Command::NewSketch),
            Item::Command(Command::Redo),
            Item::group(
                "circles",
                commands(&[Command::CircleCenter, Command::CircleTwoPoints]),
            ),
        ],
    )]);
    let mut app = opened("the_chevron_stays_clickable", WIDE, settings);
    driver::start_a_sketch(&mut app);
    assert!(
        the_button(&app, "Rétablir (Cmd+Maj+Z)")
            .accesskit_node()
            .is_disabled(),
        "there is something to redo: the premise of this test",
    );

    driver::click_exactly(&mut app, "Cercles");

    the_button(&app, "Centre et diamètre");
}

#[test]
fn a_split_button_is_on_while_the_command_it_runs_is_in_hand() {
    let mut app = sketching("a_split_is_on_while_in_hand");

    driver::click_exactly(&mut app, "Cercle (C)");
    assert!(is_down(&app, "Cercle (C)"), "« Cercle » does not show on");

    driver::click_exactly(&mut app, "Contraintes");
    assert!(
        is_down(&app, "Contraintes"),
        "« Contraintes » does not show on"
    );
    assert!(
        !is_down(&app, "Cercle (C)"),
        "« Cercle » still shows on with a rule in hand",
    );
}

const LINE_HELP: &str = "Clics successifs, Échap pour terminer la chaîne";

#[test]
fn hovering_a_button_shows_its_name_its_key_in_a_frame_and_a_sentence_of_help() {
    let mut app = sketching("hovering_a_button_shows_its_tooltip");

    let tip = hover_for_a_tooltip(&mut app, "Ligne (L)");

    assert!(
        (tip.frame.width() - 230.0).abs() <= 1.0,
        "the tooltip is {} wide, where C9 asks for 230",
        tip.frame.width(),
    );
    assert_eq!(tip.texts, ["Ligne", "L", LINE_HELP]);
    assert_eq!(tip.key_frames.len(), 1, "the frames around the key");
}

#[test]
fn a_command_with_no_key_shows_no_frame_in_its_tooltip() {
    let mut app = sketching("a_command_with_no_key_shows_no_frame");

    let tip = hover_for_a_tooltip(&mut app, "Ligne symétrique");

    assert_eq!(
        tip.texts,
        [
            "Ligne symétrique",
            "Un centre, puis un bout : l'autre suit en miroir"
        ],
    );
    assert_eq!(tip.key_frames.len(), 0, "the frames around a key with none");
}

#[test]
fn a_small_button_shows_the_same_tooltip() {
    let settings = bar_of(vec![Item::Group {
        name: "edit".to_string(),
        size: GroupSize::SmallThreeRows,
        items: commands(&[Command::NewSketch, Command::ToolLine]),
    }]);
    let mut app = opened("a_small_button_shows_the_tooltip", WIDE, settings);
    driver::start_a_sketch(&mut app);

    let tip = hover_for_a_tooltip(&mut app, "Ligne (L)");

    assert_eq!(tip.texts, ["Ligne", "L", LINE_HELP]);
    assert_eq!(tip.key_frames.len(), 1, "the frames around the key");
}

#[test]
fn the_body_of_a_split_button_shows_the_tooltip_of_its_command() {
    let mut app = sketching("the_body_of_a_split_shows_its_command");

    let tip = hover_for_a_tooltip(&mut app, "Cercle (C)");

    assert_eq!(tip.texts, ["Cercle", "C", "Deux clics : centre puis rayon"]);
    assert_eq!(tip.key_frames.len(), 1, "the frames around the key");
}

#[test]
fn the_tooltip_of_the_constraints_body_says_the_help_of_the_rule_it_would_run() {
    let mut app = sketching("the_tooltip_of_the_constraints_body");

    let tip = hover_for_a_tooltip(&mut app, "Contraintes");
    assert_eq!(tip.texts, ["Contraintes", "Deux traits, mis d'équerre"]);

    driver::click_exactly(&mut app, "Contraintes ⌄");
    driver::click_exactly(&mut app, "Parallèle");
    let tip = hover_for_a_tooltip(&mut app, "Contraintes");

    assert_eq!(
        tip.texts,
        ["Contraintes", "Deux traits, gardant la même direction"],
    );
}

#[test]
fn with_names_off_the_tooltip_still_names_the_button_and_its_key() {
    let mut settings = Settings::default();
    settings.toolbar.show_labels = false;
    let mut app = opened("with_names_off_the_tooltip_names", WIDE, settings);
    driver::start_a_sketch(&mut app);

    let tip = hover_for_a_tooltip(&mut app, "Ligne");

    assert_eq!(tip.texts, ["Ligne", "L", LINE_HELP]);
    assert_eq!(tip.key_frames.len(), 1, "the frames around the key");
}

#[test]
fn hovering_a_chevron_shows_only_the_name_of_its_group() {
    let mut app = sketching("hovering_a_chevron_shows_only_its_name");

    let tip = hover_for_a_tooltip(&mut app, "Cercles");

    assert_eq!(tip.texts, ["Cercles"]);
    assert_eq!(tip.key_frames.len(), 0, "the frames around a key with none");
}

#[test]
fn hovering_a_menu_row_shows_its_help() {
    let mut app = sketching("hovering_a_menu_row_shows_its_help");
    driver::click_exactly(&mut app, "Cercles");

    let row = the_button(&app, "Centre et diamètre").rect();
    app.hover_at(row.center());
    app.run_steps(4);

    let help = "Le centre, puis un point du bord";
    assert!(
        read_on_screen(&app).iter().any(|text| text == help),
        "hovering the row does not say « {help} », only {:?}",
        read_on_screen(&app),
    );
}

#[test]
fn a_greyed_out_button_shows_no_tooltip() {
    let mut app = opened(
        "a_greyed_out_button_shows_no_tooltip",
        WIDE,
        Settings::default(),
    );

    let line = the_button(&app, "Ligne (L)").rect();
    app.hover_at(line.center());
    app.run_steps(4);

    assert!(
        !read_on_screen(&app).iter().any(|text| text == LINE_HELP),
        "« Ligne », greyed out, says « {LINE_HELP} »",
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

fn sketching(test: &str) -> driver::App {
    let mut app = opened(test, WIDE, Settings::default());
    driver::start_a_sketch(&mut app);
    app
}

fn the_stl_menu_open(test: &str) -> driver::App {
    let mut app = driver::a_block(test);
    driver::click_the_button(&mut app, "Export");
    driver::click_exactly(&mut app, "Exporter en STL… ⌄");
    app
}

fn the_button<'a>(app: &'a driver::App, name: &'a str) -> egui_kittest::Node<'a> {
    app.query_all_by_label(name)
        .find(|node| node.accesskit_node().role() == Role::Button)
        .unwrap_or_else(|| {
            panic!(
                "no button is named « {name} »; the screen names {:?}",
                driver::on_screen(app)
            )
        })
}

/// The row of a menu, for an entry whose name carries a key after it.
fn row_starting<'a>(app: &'a driver::App, start: &str) -> egui_kittest::Node<'a> {
    app.query_all_by_role(Role::Button)
        .find(|node| {
            node.accesskit_node()
                .label()
                .is_some_and(|label| label.starts_with(start))
        })
        .unwrap_or_else(|| {
            panic!(
                "no row starts « {start} »; the screen names {:?}",
                driver::on_screen(app)
            )
        })
}

fn is_down(app: &driver::App, name: &str) -> bool {
    the_button(app, name).accesskit_node().toggled() == Some(Toggled::True)
}

fn assert_one_split_button(app: &driver::App, primary: &str, chevron: &str) {
    let (body, arrow) = (
        the_button(app, primary).rect(),
        the_button(app, chevron).rect(),
    );

    assert!(
        body.expand(2.0).intersects(arrow),
        "« {primary} » at {body:?} and « {chevron} » at {arrow:?} are two buttons apart",
    );
    assert!(
        !read_on_screen(app).iter().any(|text| text == chevron),
        "« {chevron} » is written on the button, as a menu of its own",
    );
}

fn read_on_screen(app: &driver::App) -> Vec<String> {
    shapes(app)
        .iter()
        .filter_map(|shape| match shape {
            egui::Shape::Text(text) => Some(text.galley.text().to_owned()),
            _ => None,
        })
        .collect()
}

/// What is written within a rectangle, from left to right.
fn texts_within(app: &driver::App, within: egui::Rect) -> Vec<String> {
    let within = within.expand(1.0);
    let mut texts: Vec<(f32, String)> = shapes(app)
        .iter()
        .filter_map(|shape| match shape {
            egui::Shape::Text(text) if within.contains_rect(shape.visual_bounding_rect()) => {
                Some((
                    shape.visual_bounding_rect().left(),
                    text.galley.text().to_owned(),
                ))
            }
            _ => None,
        })
        .collect();
    texts.sort_by(|one, other| one.0.total_cmp(&other.0));
    texts.into_iter().map(|(_, text)| text).collect()
}

fn icons_within(app: &driver::App, within: egui::Rect) -> Vec<String> {
    let within = within.expand(1.0);
    let textures = app.ctx.tex_manager();
    let textures = textures.read();
    shapes(app)
        .iter()
        .filter_map(|shape| match shape {
            egui::Shape::Rect(rect)
                if rect.fill_texture_id() != egui::TextureId::default()
                    && within.contains_rect(rect.rect) =>
            {
                textures.meta(rect.fill_texture_id()).map(|meta| {
                    let file = meta.name.rsplit('/').next().unwrap_or(&meta.name);
                    file.strip_suffix(".svg").unwrap_or(file).to_owned()
                })
            }
            _ => None,
        })
        .collect()
}

/// A tooltip as the last frame painted it: its frame, what is written in it
/// from first to last, and the frames drawn around a key.
#[derive(Debug)]
struct Tip {
    frame: egui::Rect,
    texts: Vec<String>,
    key_frames: Vec<egui::Rect>,
}

/// Hovers the button and reads what the tooltip painted: the shapes after the
/// tooltip's frame, the tooltip standing above the ribbon it may overlap.
fn hover_for_a_tooltip(app: &mut driver::App, button: &str) -> Tip {
    let at = the_button(app, button).rect();
    app.hover_at(at.center());
    app.run_steps(4);

    let painted = shapes_in_painted_order(app);
    let frame_at = painted
        .iter()
        .rposition(|shape| matches!(shape, egui::Shape::Rect(rect) if is_a_tooltip_s_frame(rect)))
        .unwrap_or_else(|| {
            panic!(
                "hovering « {button} » paints no tooltip on #262A33; the screen writes {:?}",
                read_on_screen(app)
            )
        });
    let egui::Shape::Rect(frame) = &painted[frame_at] else {
        unreachable!("the position of a rectangle");
    };

    let inside = &painted[frame_at + 1..];
    let texts = inside
        .iter()
        .filter_map(|shape| match shape {
            egui::Shape::Text(text) => Some(text.galley.text().to_owned()),
            _ => None,
        })
        .collect();
    let key_frames = inside
        .iter()
        .filter_map(|shape| match shape {
            egui::Shape::Rect(rect) if rect.stroke.color == BORDER_KEY => Some(rect.rect),
            _ => None,
        })
        .collect();
    Tip {
        frame: frame.rect,
        texts,
        key_frames,
    }
}

fn is_a_tooltip_s_frame(rect: &egui::epaint::RectShape) -> bool {
    rect.fill == BORDER_GROUP && rect.rect.width() > 100.0
}

/// The shapes of the frame, groups opened, in the order they were painted.
fn shapes_in_painted_order(app: &driver::App) -> Vec<egui::Shape> {
    fn walk(shape: &egui::Shape, out: &mut Vec<egui::Shape>) {
        match shape {
            egui::Shape::Vec(group) => group.iter().for_each(|shape| walk(shape, out)),
            shape => out.push(shape.clone()),
        }
    }
    let mut out = Vec::new();
    for clipped in &app.output().shapes {
        walk(&clipped.shape, &mut out);
    }
    out
}
