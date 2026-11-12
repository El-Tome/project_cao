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
//! - found in the window at 1512 wide: an entry never stands above the one
//!   before it in the tree, and a line reads in the tree's order —
//!   `an_entry_of_a_tab_never_stands_above_the_one_before_it_in_the_tree`
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
//! - found in review: and reaches its nested commands when its group holds only
//!   sub-groups — `a_chevron_reaches_the_commands_of_its_sub_groups`
//! - found in review: a separator in a small group leaves its later columns on
//!   the bar — `a_separator_in_a_small_group_keeps_its_later_columns_on_the_bar`
//! - found in review: no rule dangles at the end of a wrapped line —
//!   `no_separator_dangles_at_the_end_of_a_wrapped_line`
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
//! - with « Noms des boutons » off: icons only —
//!   `with_names_off_every_ribbon_button_is_its_icon_alone`
//! - and a small group's buttons stay at their row's height —
//!   `with_names_off_a_small_group_is_icons_only_at_the_height_of_its_rows`
//! - and a split button is its icon and its chevron —
//!   `with_names_off_a_split_button_is_its_icon_and_its_chevron`
//! - a group keeps its whole name, read as « Noms des boutons » speaking of
//!   buttons — `with_names_off_a_group_keeps_its_name_under_its_buttons`
//! - and down a side, not elided either —
//!   `down_a_side_with_names_off_a_group_writes_its_whole_name`
//! - down the left or right: groups stack — `down_a_side_the_groups_stack`
//! - every button small — `down_a_side_every_button_is_small`
//! - a group's name still sits under its buttons —
//!   `down_a_side_a_group_writes_its_name_under_its_buttons`
//! - nothing runs off the panel —
//!   `down_a_side_nothing_runs_off_the_panel`; reached by scroll when the
//!   column is taller than the window, as `fitting_the_window.rs` holds
//! - found in review: a command directly in a tab, down a side, is small —
//!   `down_a_side_a_command_sitting_directly_in_a_tab_is_small`
//! - floating: as at the top — `a_floating_ribbon_is_laid_out_as_along_the_top`
//! - found in review: no wider than what it holds —
//!   `a_floating_bar_is_no_wider_than_what_it_holds`
//! - sizes as the setting says —
//!   `a_floating_ribbon_draws_each_group_at_its_size`
//! - and wrapped group by group in a narrow window —
//!   `a_floating_ribbon_wraps_group_by_group_in_a_narrow_window`
//! - every command of today's ribbon can be reached, as a button, a split body
//!   or a menu entry — `every_command_of_the_default_tree_can_be_reached`
//! - every button keeps its name as its accessible label —
//!   `every_ribbon_button_keeps_its_name_as_its_accessible_label`
//! - toolbar positions (top, bottom, left, right, floating) —
//!   `the_ribbon_is_drawn_at_each_of_the_five_toolbar_positions`
//! - the dock / undock command — no test: `Command::ToggleToolbarDocked` stays
//!   what it is today, a no-op in `commands.rs`, and nothing here changes it
//!
//! Closes #732. The bullets keep the issue's wording, its French names between
//! guillemets; what the prefs and the wording hold is named as held there.
//! - Esquisse tab holds, in this order: « Esquisse » (« Nouvelle esquisse »,
//!   large) · « Dessin », large: « Sélection », « Ligne », « Ligne sym. »,
//!   « Rectangle », « Cercle ▾ » (5), « Arc ▾ » (2), « Ellipse ▾ » (2),
//!   « Point » · « Cotes et contraintes », small on 3 rows: « Cote »,
//!   « Mesurer », « Contraintes ▾ » (9) · « Modifier », small on 3 rows:
//!   « Ajuster », « Diviser », « Symétrie », « Congé », « Chanfrein ▾ » (3),
//!   « Construction », « Réseau circulaire », « Réseau rectangulaire » ·
//!   « Édition », small on 2 rows: « Annuler », « Rétablir » (greyed when there
//!   is nothing to redo) —
//!   `the_sketch_tab_writes_its_five_groups_in_order_each_under_its_buttons`,
//!   `each_group_of_the_sketch_tab_draws_its_buttons_at_its_size` and, for
//!   « Contraintes ▾ » standing apart from « Mesurer »,
//!   `measure_stays_a_plain_button_and_the_constraints_split_stands_alone`; the
//!   tree and its sizes in `crates/prefs/src/toolbar/tests.rs` by
//!   `the_default_sketch_tab_holds_its_five_groups_and_their_commands_in_order`,
//!   the group names in `crates/app/src/wording/toolbar/tests.rs` by
//!   `the_five_groups_of_the_sketch_tab_read_as_the_design_names_them`, and
//!   « Rétablir » greyed by
//!   `redo_on_the_default_ribbon_is_greyed_until_something_can_be_redone`, and
//!   at the presenter by `a_fresh_part_has_nothing_to_undo_and_nothing_to_redo`
//!   in `crates/app/src/screens/ribbon/state/tests.rs`
//! - Extrusion tab holds « Matière » (« Ajout de matière », « Enlèvement de
//!   matière ») · « Forme » (« Extrusion droite », « Révolution ») · « Édition »
//!   — `the_extrusion_tab_writes_its_four_groups_in_order_each_under_its_buttons`,
//!   which reads « Arêtes » (« Chanfrein », « Congé ») too since #920;
//!   the tree and its sizes in `crates/prefs/src/toolbar/tests.rs` by
//!   `the_default_extrusion_tab_holds_its_four_groups_and_their_commands_in_order`,
//!   the group names in `crates/app/src/wording/toolbar/tests.rs` by
//!   `the_four_groups_of_the_extrusion_tab_read_as_the_design_names_them`
//! - Export tab holds « Exporter en STL… ▾ » (« Grossière » « rapide »,
//!   « Normale » « par défaut », « Fine » « lourd ») and « Exporter en STEP… » —
//!   `the_export_tab_draws_one_stl_split_with_its_three_levels_and_the_step_export`;
//!   the tree in `crates/prefs/src/toolbar/tests.rs` by
//!   `the_default_export_tab_holds_one_stl_split_and_the_step_export`
//! - added on request on pull request #819, as screen `04` draws it: the Export
//!   tab writes « EXPORTER » under its two buttons —
//!   `the_export_tab_writes_its_group_under_its_buttons`, its wording in
//!   `crates/app/src/wording/toolbar/tests.rs` by
//!   `the_group_of_the_export_tab_reads_exporter_and_the_tab_keeps_its_name`
//! - added on request on pull request #819: the ribbon is the same height on
//!   every tab — `the_ribbon_is_the_same_height_on_every_tab`
//! - Moved out: « Recadrer », « Vue en coupe », « Matière devant » and
//!   « Terminer » are no longer in the default ribbon. They stay commands, with
//!   their keys, and can be put back from the preferences —
//!   `the_four_moved_commands_stand_in_no_tab_of_the_default_ribbon` and
//!   `the_four_moved_commands_can_be_put_back_from_the_preferences`; the tree
//!   and the keys in `crates/prefs/src/toolbar/tests.rs` by
//!   `the_four_moved_commands_are_nowhere_in_the_default_tree_and_keep_their_keys`
//! - The « Par défaut » profile takes this ribbon; a profile Tom created keeps
//!   its own until « Réinitialiser la barre d'outils » —
//!   `resetting_the_toolbar_gives_the_profile_the_default_ribbon`, the button
//!   still reading « ↺ Barre par défaut » until #747 renames it; the load in
//!   `crates/prefs/src/profiles/tests.rs` by
//!   `a_default_profile_saved_with_the_old_ribbon_takes_the_new_one_at_load` and
//!   `a_profile_the_user_created_keeps_its_own_toolbar_across_a_save_and_a_load`
//! - At 1440 px the sketch tab fits on one line —
//!   `the_sketch_tab_fits_on_one_line_at_1440`; and, as screens `01` and `03`
//!   draw them, the names of a tab's groups stand at one height whatever the
//!   size of each group — `the_group_names_of_a_tab_stand_on_one_line`; found
//!   in review: wrapped, each line's names stay level and above the next
//!   line — `a_wrapped_ribbon_keeps_each_line_s_names_level_and_above_the_next_line`
//!   — and a group beside one wrapped inside itself keeps its name under its own
//!   buttons — `a_group_beside_a_wrapped_group_keeps_its_name_under_its_own_buttons`
//! - Every command stays reachable (G2): what leaves the ribbon has its place in
//!   the view bar or the sketch pill —
//!   `what_left_the_ribbon_has_its_place_in_the_view_bar_or_the_sketch_pill`,
//!   and what stays by `every_command_of_the_default_tree_can_be_reached`
//! - F and ↵ still reframe and finish —
//!   `f_still_reframes_and_enter_still_finishes_the_sketch`
//! - added on request on pull request #819: a hovered row of a chevron menu
//!   can be seen, its fill standing clear of the menu's —
//!   `a_hovered_row_of_a_chevron_menu_stands_out_from_the_menu`; the hover of a
//!   button outside a menu does not change —
//!   `hovering_a_ribbon_button_outside_a_menu_keeps_its_fill`; a row that is on
//!   keeps its amber — `a_menu_row_that_is_on_keeps_its_amber`; a disabled row
//!   paints nothing — `a_disabled_menu_row_paints_nothing_when_hovered`; the
//!   same fill on the view bar's « Matière devant » menu —
//!   `a_hovered_choice_of_the_view_bar_s_matter_menu_stands_out_from_the_menu`,
//!   its current choice staying marked —
//!   `the_current_choice_of_the_view_bar_s_matter_menu_stays_marked`, and on a
//!   sub-group nested in a chevron menu —
//!   `a_hovered_sub_group_of_a_chevron_menu_stands_out_from_the_menu`, and an
//!   entry of the popup that sub-group opens —
//!   `a_hovered_entry_of_a_nested_sub_menu_stands_out_from_the_menu`
//!
//! Decided on the way. The tree does not change: « Cercle » and the group
//! `circles` stay two siblings, and a split button is a rule of the drawing —
//! a command immediately followed by a menu-depth group is one button, the
//! command's icon, name and key, its chevron opening that group. A group wider
//! than the whole bar may wrap inside itself as a last resort, because the
//! default content stays as it is until the next issue and
//! `fitting_the_window.rs` (#650) must stay true.

mod driver;

use cao_app::theme::tokens::{ACCENT_AMBER_BACKGROUND, BORDER_GROUP, BORDER_KEY, TEXT_QUIET};
use cao_prefs::{Chord, Command, Edge, GroupSize, Item, Key, Settings};
use egui::accesskit::{Role, Toggled};
use egui::epaint::TextShape;
use egui_kittest::kittest::{NodeT, Queryable};
use glam::DVec3;

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
    let (drawing, editing, export) = (
        whole_group(&app, 0),
        whole_group(&app, 1),
        whole_group(&app, 2),
    );

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
    let export = whole_group(&app, 2);

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
    let settings = bar_of(vec![Item::Command(Command::NewSketch)]);
    let app = opened("a_command_sitting_directly", WIDE, settings);

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
fn an_entry_of_a_tab_never_stands_above_the_one_before_it_in_the_tree() {
    const IN_TREE_ORDER: [&str; 6] = [
        "Nouvelle esquisse",
        "Sélection",
        "Point",
        "Cote",
        "Ajuster",
        "Annuler",
    ];
    let mut wrong = Vec::new();

    for width in [1400.0, 1440.0, 1512.0, 1600.0, 1728.0] {
        let app = opened(
            &format!("an_entry_never_stands_above_{width}"),
            egui::vec2(width, 900.0),
            Settings::default(),
        );
        let drawn = IN_TREE_ORDER.map(|name| (name, the_ribbon_button(&app, name)));

        for pair in drawn.windows(2) {
            let ((before, earlier), (after, later)) = (pair[0], pair[1]);
            let above = later.top() < earlier.top() - ALIGNED_WITHIN;
            let on_one_line = (later.top() - earlier.top()).abs() <= ALIGNED_WITHIN;
            if above || (on_one_line && later.left() < earlier.left()) {
                wrong.push(format!(
                    "at {width}: « {after} » at {later:?} stands before « {before} » at {earlier:?}"
                ));
            }
        }
    }

    assert!(
        wrong.is_empty(),
        "entries of the tab are drawn out of the tree's order:\n  {}",
        wrong.join("\n  "),
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
    use Command as C;
    let size = egui::vec2(1024.0, 768.0);
    let settings = bar_of(vec![Item::group(
        "drawing",
        commands(&[
            C::ToolSelect,
            C::ToolLine,
            C::ToolLineSymmetric,
            C::ToolRectangle,
            C::ToolCircle,
            C::ToolArc,
            C::ToolEllipse,
            C::ToolPoint,
            C::ToolDimension,
            C::ToolMeasure,
            C::ToolTrim,
            C::ToolSplit,
            C::ToolMirror,
            C::ToolCircularPattern,
            C::ToolRectangularPattern,
            C::ToolFillet,
            C::ToolChamfer,
            C::ToggleConstruction,
        ]),
    )]);
    let app = opened("a_group_wider_than_the_bar", size, settings);

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

#[test]
fn with_names_off_every_ribbon_button_is_its_icon_alone() {
    let app = opened_with_names_off("with_names_off_every_button_is_its_icon");

    let line = the_button(&app, "Ligne").rect();

    assert_eq!(texts_within(&app, line), Vec::<String>::new());
    assert_eq!(icons_within(&app, line), ["line"]);
}

#[test]
fn with_names_off_a_small_group_is_icons_only_at_the_height_of_its_rows() {
    let mut settings = bar_of(vec![Item::Group {
        name: "edit".to_string(),
        size: GroupSize::SmallThreeRows,
        items: commands(&[Command::Undo, Command::Redo, Command::ToolLine]),
    }]);
    settings.toolbar.show_labels = false;
    let app = opened("with_names_off_a_small_group", WIDE, settings);

    let undo = the_button(&app, "Annuler").rect();

    assert_eq!(texts_within(&app, undo), Vec::<String>::new());
    assert_eq!(icons_within(&app, undo).len(), 1, "the icon of « Annuler »");
    assert_tall(undo, 20.0, "a small button with its name off");
}

#[test]
fn with_names_off_a_split_button_is_its_icon_and_its_chevron() {
    let app = opened_with_names_off("with_names_off_a_split_button");

    let body = the_button(&app, "Cercle").rect();
    let chevron = the_button(&app, "Cercles").rect();

    assert_eq!(icons_within(&app, body), ["circle-centre"]);
    assert_eq!(texts_within(&app, body), Vec::<String>::new());
    assert_eq!(texts_within(&app, chevron), Vec::<String>::new());
}

#[test]
fn with_names_off_a_group_keeps_its_name_under_its_buttons() {
    let app = opened_with_names_off("with_names_off_a_group_keeps_its_name");

    let lowest = the_button(&app, "Annuler")
        .rect()
        .union(the_button(&app, "Rétablir").rect())
        .bottom();
    let name = written(&app, "ÉDITION");

    assert!(
        name.rect.top() >= lowest,
        "« ÉDITION » at {:?} is not under its buttons, which end at {lowest}",
        name.rect,
    );
    assert_not_cut_short(&name, "ÉDITION");
}

#[test]
fn down_a_side_with_names_off_a_group_writes_its_whole_name() {
    let mut settings = Settings::default();
    settings.toolbar.show_labels = false;
    settings.toolbar.edge = Edge::Left;
    let app = opened("down_a_side_with_names_off_whole_name", WIDE, settings);

    let name = written(&app, "ÉDITION");

    assert_not_cut_short(&name, "ÉDITION");
}

#[test]
fn down_a_side_a_command_sitting_directly_in_a_tab_is_small() {
    let mut settings = bar_of(vec![Item::Command(Command::NewSketch)]);
    settings.toolbar.edge = Edge::Left;
    let app = opened("down_a_side_a_command_directly_in_a_tab", WIDE, settings);

    let new_sketch = row_starting(&app, "Nouvelle esquisse").rect();

    assert!(
        new_sketch.height() <= 26.0 + ALIGNED_WITHIN,
        "« Nouvelle esquisse », directly in its tab down a side, is {} high",
        new_sketch.height(),
    );
}

#[test]
fn a_floating_bar_is_no_wider_than_what_it_holds() {
    let mut settings = Settings::default();
    settings.toolbar.edge = Edge::Floating;
    let mut app = opened("a_floating_bar_is_no_wider", WIDE, settings);
    driver::click_the_button(&mut app, "Extrusion");

    let holds = row_starting(&app, "Ajout de matière")
        .rect()
        .union(row_starting(&app, "Révolution").rect());
    let window = shapes(&app)
        .iter()
        .filter_map(|shape| match shape {
            egui::Shape::Rect(rect) if rect.fill.a() > 0 && rect.rect.contains_rect(holds) => {
                Some(rect.rect)
            }
            _ => None,
        })
        .min_by(|one, other| one.area().total_cmp(&other.area()))
        .expect("the floating window's frame around its tab");

    assert!(
        window.width() <= WIDE.x / 2.0,
        "a floating bar holding the extrusion tab is {} wide on a window of {}",
        window.width(),
        WIDE.x,
    );
}

#[test]
fn a_separator_in_a_small_group_keeps_its_later_columns_on_the_bar() {
    use Command as C;
    let settings = bar_of(vec![Item::Group {
        name: "edit".to_string(),
        size: GroupSize::SmallThreeRows,
        items: vec![
            Item::Command(C::Undo),
            Item::Separator,
            Item::Command(C::Redo),
            Item::Command(C::ToolLine),
            Item::Command(C::ToolSelect),
            Item::Command(C::ToolRectangle),
            Item::Command(C::ToolPoint),
            Item::Command(C::ToolMeasure),
        ],
    }]);
    let app = opened("a_separator_in_a_small_group", WIDE, settings);

    for button in [
        "Annuler",
        "Rétablir",
        "Ligne",
        "Sélection",
        "Rectangle",
        "Point",
        "Mesurer",
    ] {
        let at = row_starting(&app, button).rect();
        assert!(
            at.right() <= WIDE.x,
            "« {button} » at {at:?} is pushed off the bar by a separator before it",
        );
    }
}

#[test]
fn no_separator_dangles_at_the_end_of_a_wrapped_line() {
    let app = three_groups("no_separator_dangles", egui::vec2(900.0, 800.0));
    let groups = [0, 1, 2].map(|rank| whole_group(&app, rank));

    let rules: Vec<Rule> = vertical_rules(&app)
        .into_iter()
        .filter(|rule| groups.iter().any(|group| rule.runs_beside(*group)))
        .collect();

    for rule in &rules {
        let before = groups
            .iter()
            .any(|group| group.right() <= rule.at && rule.runs_beside(*group));
        let after = groups
            .iter()
            .any(|group| group.left() >= rule.at && rule.runs_beside(*group));
        assert!(
            before && after,
            "the rule {rule:?} stands at the end of a wrapped line, beside the groups {groups:?}",
        );
    }
}

#[test]
fn a_chevron_reaches_the_commands_of_its_sub_groups() {
    use Command as C;
    let settings = bar_of(vec![Item::group(
        "drawing",
        vec![
            Item::Command(C::NewSketch),
            Item::Command(C::ToolCircle),
            Item::group(
                "circles",
                vec![Item::group(
                    "arcs",
                    commands(&[C::ArcByCenter, C::ArcByEnds]),
                )],
            ),
        ],
    )]);
    let mut app = opened("a_chevron_reaches_its_sub_groups", WIDE, settings);
    driver::start_a_sketch(&mut app);

    assert!(
        !the_button(&app, "Cercles").accesskit_node().is_disabled(),
        "the chevron of a group holding only sub-groups is greyed out",
    );
    driver::click_exactly(&mut app, "Cercles");
    driver::click_exactly(&mut app, "Arcs");

    the_button(&app, "Centre, départ et arrivée");
}

/// The five groups of the default sketch tab, as written under their buttons,
/// and the buttons each one holds on the bar.
const THE_FIVE_GROUPS: [(&str, &[&str]); 5] = [
    ("ESQUISSE", &["Nouvelle esquisse"]),
    (
        "DESSIN",
        &[
            "Sélection",
            "Ligne",
            "Ligne symétrique",
            "Rectangle",
            "Cercle",
            "Arc",
            "Ellipse",
            "Point",
        ],
    ),
    ("COTES ET CONTRAINTES", &["Cote", "Mesurer", "Contraintes"]),
    (
        "MODIFIER",
        &[
            "Ajuster",
            "Diviser",
            "Symétrie",
            "Congé",
            "Chanfrein",
            "Construction",
            "Réseau circulaire",
            "Réseau rectangulaire",
        ],
    ),
    ("ÉDITION", &["Annuler", "Rétablir"]),
];

#[test]
fn the_sketch_tab_writes_its_five_groups_in_order_each_under_its_buttons() {
    let app = opened(
        "the_sketch_tab_writes_its_five_groups",
        WIDE,
        Settings::default(),
    );

    assert_groups_read_in_order_each_under_its_buttons(&app, &THE_FIVE_GROUPS);
}

/// Each group's name, as written, stands under its buttons, and the names read
/// left to right in the order given.
fn assert_groups_read_in_order_each_under_its_buttons(
    app: &driver::App,
    groups: &[(&str, &[&str])],
) {
    let mut lefts = Vec::new();
    for (name, buttons) in groups {
        let written_name = written(app, name);
        let lowest = buttons
            .iter()
            .map(|button| the_ribbon_button(app, button).bottom())
            .fold(f32::NEG_INFINITY, f32::max);
        assert!(
            written_name.rect.top() >= lowest,
            "« {name} » is written at {:?}, above the bottom of its buttons at {lowest}",
            written_name.rect,
        );
        lefts.push(written_name.rect.left());
    }

    assert!(
        lefts.is_sorted(),
        "the groups {groups:?} do not read left to right in order: {lefts:?}",
    );
}

#[test]
fn the_extrusion_tab_writes_its_four_groups_in_order_each_under_its_buttons() {
    let mut app = opened(
        "the_extrusion_tab_writes_its_four_groups",
        WIDE,
        Settings::default(),
    );
    driver::click_the_button(&mut app, "Extrusion");

    assert_groups_read_in_order_each_under_its_buttons(
        &app,
        &[
            ("MATIÈRE", &["Ajout de matière", "Enlèvement de matière"]),
            ("FORME", &["Extrusion droite", "Révolution"]),
            ("ARÊTES", &["Chanfrein", "Congé"]),
            ("ÉDITION", &["Annuler", "Rétablir"]),
        ],
    );
    for name in ["Ajout de matière", "Extrusion droite", "Chanfrein"] {
        let height = the_ribbon_button(&app, name).height();
        assert!(
            height >= 64.0,
            "« {name} », in a large group, is {height} high",
        );
    }
    for name in ["Annuler", "Rétablir"] {
        assert_tall(
            the_ribbon_button(&app, name),
            26.0,
            &format!("« {name} », in a group small on 2 rows,"),
        );
    }
}

/// The fill the frame last painted exactly over this rectangle, if any.
fn fill_painted_over(app: &driver::App, rect: egui::Rect) -> Option<egui::Color32> {
    shapes(app).iter().find_map(|shape| match shape {
        egui::Shape::Rect(painted)
            if painted.fill.a() > 0
                && painted.rect.expand(ALIGNED_WITHIN).contains_rect(rect)
                && rect.expand(ALIGNED_WITHIN).contains_rect(painted.rect) =>
        {
            Some(painted.fill)
        }
        _ => None,
    })
}

/// The fill of the rectangle holding these rows: the frame of their menu.
fn fill_of_the_menu_around(app: &driver::App, rows: egui::Rect) -> egui::Color32 {
    shapes(app)
        .iter()
        .filter_map(|shape| match shape {
            egui::Shape::Rect(rect) if rect.fill.a() > 0 && rect.rect.contains_rect(rows) => {
                Some((rect.rect, rect.fill))
            }
            _ => None,
        })
        .min_by(|one, other| one.0.area().total_cmp(&other.0.area()))
        .expect("a menu frame painted around its rows")
        .1
}

/// Hovers a row and lets the frame settle, the way a hand resting on it would.
fn hover_the_row(app: &mut driver::App, row: egui::Rect) {
    app.hover_at(row.center());
    app.run_steps(4);
}

/// The WCAG relative luminance of an opaque colour.
fn luminance(colour: egui::Color32) -> f32 {
    let linear = |channel: u8| {
        let c = f32::from(channel) / 255.0;
        if c <= 0.04045 {
            c / 12.92
        } else {
            ((c + 0.055) / 1.055).powf(2.4)
        }
    };
    0.2126 * linear(colour.r()) + 0.7152 * linear(colour.g()) + 0.0722 * linear(colour.b())
}

fn contrast(one: egui::Color32, other: egui::Color32) -> f32 {
    let (a, b) = (luminance(one), luminance(other));
    (a.max(b) + 0.05) / (a.min(b) + 0.05)
}

#[test]
fn a_hovered_row_of_a_chevron_menu_stands_out_from_the_menu() {
    let mut app = the_stl_menu_open("a_hovered_row_stands_out");
    let rows = ["Grossière", "Normale", "Fine"]
        .iter()
        .fold(egui::Rect::NOTHING, |all, row| {
            all.union(the_button(&app, row).rect())
        });
    let row = the_button(&app, "Normale").rect();
    let menu = fill_of_the_menu_around(&app, rows);

    hover_the_row(&mut app, row);

    assert_the_hovered_row_stands_out(&app, row, menu);
}

/// The hovered row wears the fill chosen on sight, and stands clear of the
/// fill of the menu it sits in.
fn assert_the_hovered_row_stands_out(app: &driver::App, row: egui::Rect, menu: egui::Color32) {
    let fill = fill_painted_over(app, row).expect("the row hovered paints a fill");
    assert_eq!(
        fill,
        egui::Color32::from_rgb(0x35, 0x3A, 0x45),
        "the hovered row is filled {fill:?}, where #353A45 was chosen on sight",
    );
    let against_the_menu = contrast(fill, menu);
    assert!(
        against_the_menu >= 1.25,
        "the hovered row's fill stands {against_the_menu}:1 from the menu's fill {menu:?}: \
         it can barely be seen",
    );
}

const THE_THREE_MATTER_CHOICES: [&str; 3] = ["cachée", "transparente", "opaque"];

fn the_view_bar_s_matter_menu_open(test: &str) -> driver::App {
    let mut app = sketching(test);
    driver::click_in_the_bar(&mut app, "Matière devant");
    app
}

#[test]
fn a_hovered_choice_of_the_view_bar_s_matter_menu_stands_out_from_the_menu() {
    let mut app = the_view_bar_s_matter_menu_open("a_hovered_matter_choice_stands_out");
    let rows = THE_THREE_MATTER_CHOICES
        .iter()
        .fold(egui::Rect::NOTHING, |all, row| {
            all.union(the_button(&app, row).rect())
        });
    let menu = fill_of_the_menu_around(&app, rows);
    let row = the_button(&app, "opaque").rect();

    hover_the_row(&mut app, row);

    assert_the_hovered_row_stands_out(&app, row, menu);
}

#[test]
fn the_current_choice_of_the_view_bar_s_matter_menu_stays_marked() {
    let mut app = the_view_bar_s_matter_menu_open("the_current_matter_choice_stays_marked");
    let (current, other) = (
        the_button(&app, "cachée").rect(),
        the_button(&app, "opaque").rect(),
    );

    assert_eq!(
        fill_painted_over(&app, current),
        Some(ACCENT_AMBER_BACKGROUND)
    );
    hover_the_row(&mut app, other);
    assert_eq!(
        fill_painted_over(&app, current),
        Some(ACCENT_AMBER_BACKGROUND),
        "the current choice lost its amber while another row is hovered",
    );
    hover_the_row(&mut app, current);
    assert_eq!(
        fill_painted_over(&app, current),
        Some(ACCENT_AMBER_BACKGROUND),
        "the current choice, hovered, lost its amber",
    );
}

/// A sketch under a chevron menu « Cercles » holding the sub-group « Arcs », the
/// menu open.
fn a_chevron_menu_holding_a_sub_group(test: &str) -> driver::App {
    use Command as C;
    let settings = bar_of(vec![Item::group(
        "drawing",
        vec![
            Item::Command(C::NewSketch),
            Item::Command(C::ToolCircle),
            Item::group(
                "circles",
                vec![
                    Item::group("arcs", commands(&[C::ArcByCenter, C::ArcByEnds])),
                    Item::Command(C::CircleCenter),
                ],
            ),
        ],
    )]);
    let mut app = opened(test, WIDE, settings);
    driver::start_a_sketch(&mut app);
    driver::click_exactly(&mut app, "Cercles");
    app
}

#[test]
fn a_hovered_entry_of_a_nested_sub_menu_stands_out_from_the_menu() {
    let mut app = a_chevron_menu_holding_a_sub_group("a_hovered_nested_entry_stands_out");
    driver::click_exactly(&mut app, "Arcs");
    let entries = [
        "Centre, départ et arrivée",
        "Deux bouts et un point traversé",
    ]
    .map(|entry| the_button(&app, entry).rect());
    let menu = fill_of_the_menu_around(&app, entries[0].union(entries[1]));

    hover_the_row(&mut app, entries[1]);

    assert_the_hovered_row_stands_out(&app, entries[1], menu);
}

#[test]
fn a_hovered_sub_group_of_a_chevron_menu_stands_out_from_the_menu() {
    let mut app = a_chevron_menu_holding_a_sub_group("a_hovered_sub_group_stands_out");
    let row = the_button(&app, "Arcs").rect();
    let menu = fill_of_the_menu_around(
        &app,
        row.union(row_starting(&app, "Centre et diamètre").rect()),
    );

    hover_the_row(&mut app, row);

    assert_the_hovered_row_stands_out(&app, row, menu);
}

#[test]
fn hovering_a_ribbon_button_outside_a_menu_keeps_its_fill() {
    let mut app = sketching("hovering_a_ribbon_button_keeps_its_fill");
    let rectangle = the_button(&app, "Rectangle (R)").rect();

    hover_the_row(&mut app, rectangle);

    assert_eq!(
        fill_painted_over(&app, rectangle),
        Some(egui::Color32::from_rgb(0x23, 0x27, 0x31)),
        "the hover of « Rectangle » on the ribbon changed",
    );
}

#[test]
fn a_menu_row_that_is_on_keeps_its_amber() {
    let mut app = sketching("a_menu_row_that_is_on_keeps_its_amber");
    driver::click_exactly(&mut app, "Cercles");
    driver::click_exactly(&mut app, "Deux points du bord");
    driver::click_exactly(&mut app, "Cercles");
    let row = the_button(&app, "Deux points du bord").rect();

    hover_the_row(&mut app, row);

    assert_eq!(
        fill_painted_over(&app, row),
        Some(ACCENT_AMBER_BACKGROUND),
        "the row in hand, hovered, lost its amber",
    );
}

#[test]
fn a_disabled_menu_row_paints_nothing_when_hovered() {
    let settings = bar_of(vec![Item::group(
        "drawing",
        vec![
            Item::Command(Command::NewSketch),
            Item::Command(Command::ToolCircle),
            Item::group("circles", commands(&[Command::Redo, Command::CircleCenter])),
        ],
    )]);
    let mut app = opened("a_disabled_menu_row_paints_nothing", WIDE, settings);
    driver::start_a_sketch(&mut app);
    driver::click_exactly(&mut app, "Cercles");
    let row = row_starting(&app, "Rétablir").rect();
    assert!(
        row_starting(&app, "Rétablir")
            .accesskit_node()
            .is_disabled(),
        "there is something to redo: the premise of this test",
    );

    hover_the_row(&mut app, row);

    assert_eq!(
        fill_painted_over(&app, row),
        None,
        "« Rétablir », greyed out and hovered, paints a fill",
    );
}

#[test]
fn the_export_tab_writes_its_group_under_its_buttons() {
    let mut app = opened("the_export_tab_writes_its_group", WIDE, Settings::default());
    driver::click_the_button(&mut app, "Export");

    assert_groups_read_in_order_each_under_its_buttons(
        &app,
        &[("EXPORTER", &["Exporter en STL…", "Exporter en STEP…"])],
    );
}

#[test]
fn the_ribbon_is_the_same_height_on_every_tab() {
    let mut app = opened(
        "the_ribbon_is_the_same_height_on_every_tab",
        egui::vec2(1440.0, 900.0),
        Settings::default(),
    );

    let view_starts = ["Esquisse", "Extrusion", "Export"].map(|tab| {
        driver::click_the_button(&mut app, tab);
        (tab, driver::canvas(&app).top())
    });

    assert!(
        view_starts
            .iter()
            .all(|(_, top)| (top - view_starts[0].1).abs() <= ALIGNED_WITHIN),
        "the view does not start at one height from tab to tab: {view_starts:?}",
    );
}

#[test]
fn the_export_tab_draws_one_stl_split_with_its_three_levels_and_the_step_export() {
    let app = the_stl_menu_open("the_export_tab_draws_one_stl_split");

    assert_one_split_button(&app, "Exporter en STL…", "Exporter en STL… ⌄");
    let (stl, step) = (
        the_button(&app, "Exporter en STL…").rect(),
        the_ribbon_button(&app, "Exporter en STEP…"),
    );
    assert!(
        step.left() >= stl.right(),
        "« Exporter en STEP… » at {step:?} does not stand after « Exporter en STL… » at {stl:?}",
    );

    let levels = [
        ("Grossière", "rapide"),
        ("Normale", "par défaut"),
        ("Fine", "lourd"),
    ];
    let tops = levels.map(|(level, _)| the_button(&app, level).rect().top());
    assert!(
        tops.is_sorted(),
        "the three levels do not read in order, their rows at {tops:?}",
    );
    for (level, hint) in levels {
        let row = the_button(&app, level).rect();
        assert!(
            row.expand(1.0).contains_rect(written(&app, hint).rect),
            "« {hint} » is not on the line of « {level} » at {row:?}",
        );
    }
}

#[test]
fn each_group_of_the_sketch_tab_draws_its_buttons_at_its_size() {
    let app = opened(
        "each_group_of_the_sketch_tab_at_its_size",
        WIDE,
        Settings::default(),
    );

    for name in ["Nouvelle esquisse", "Ligne", "Point"] {
        let height = the_ribbon_button(&app, name).height();
        assert!(
            height >= 64.0,
            "« {name} », in a large group, is {height} high",
        );
    }
    for name in ["Cote", "Mesurer", "Ajuster", "Diviser", "Réseau circulaire"] {
        assert_tall(
            the_ribbon_button(&app, name),
            20.0,
            &format!("« {name} », in a group small on 3 rows,"),
        );
    }
    for name in ["Annuler", "Rétablir"] {
        assert_tall(
            the_ribbon_button(&app, name),
            26.0,
            &format!("« {name} », in a group small on 2 rows,"),
        );
    }
}

#[test]
fn redo_on_the_default_ribbon_is_greyed_until_something_can_be_redone() {
    let mut app = sketching("redo_on_the_default_ribbon_is_greyed");
    let greyed = |app: &driver::App| {
        driver::the_ribbon_s(app, "Rétablir")
            .expect("the default ribbon's group « Édition » holds « Rétablir »")
            .accesskit_node()
            .is_disabled()
    };

    assert!(greyed(&app), "« Rétablir » is offered on a fresh sketch");

    driver::draw_a_segment(&mut app);
    assert!(
        greyed(&app),
        "« Rétablir » is offered with nothing undone yet"
    );

    driver::press_with_command(&mut app, egui::Key::Z);
    assert!(
        !greyed(&app),
        "« Rétablir » stays greyed though a step was just undone",
    );
}

const THE_NINE_RULES: [&str; 9] = [
    "Perpendiculaire",
    "Parallèle",
    "Égalité",
    "Coïncidence",
    "Colinéaire",
    "Tangence",
    "Milieu",
    "Fixe",
    "Concentrique",
];

#[test]
fn measure_stays_a_plain_button_and_the_constraints_split_stands_alone() {
    let mut app = sketching("measure_stays_plain_constraints_stand_alone");

    let (dimension, measure, constraints) = (
        row_starting(&app, "Cote").rect(),
        row_starting(&app, "Mesurer").rect(),
        the_button(&app, "Contraintes").rect(),
    );
    assert!(
        (dimension.left() - measure.left()).abs() <= ALIGNED_WITHIN
            && (measure.left() - constraints.left()).abs() <= ALIGNED_WITHIN
            && measure.top() >= dimension.bottom()
            && constraints.top() >= measure.bottom(),
        "« Cote » at {dimension:?}, « Mesurer » at {measure:?} and « Contraintes » at \
         {constraints:?} do not stand one above the other, in that order",
    );

    assert_one_split_button(&app, "Contraintes", "Contraintes ⌄");
    let beside_measure: Vec<String> = app
        .query_all_by_role(Role::Button)
        .filter(|node| {
            let at = node.rect();
            at != measure
                && (at.center().y - measure.center().y).abs() <= ALIGNED_WITHIN
                && at.expand(2.0).intersects(measure)
        })
        .filter_map(|node| node.accesskit_node().label())
        .collect();
    assert!(
        beside_measure.is_empty(),
        "« Mesurer » at {measure:?} is a split button, with beside it {beside_measure:?}",
    );

    driver::click_exactly(&mut app, "Contraintes ⌄");
    for rule in THE_NINE_RULES {
        the_button(&app, rule);
    }
}

#[test]
fn the_four_moved_commands_stand_in_no_tab_of_the_default_ribbon() {
    let mut app = sketching("the_four_moved_commands_stand_in_no_tab");

    let mut still_there = Vec::new();
    for tab in ["Esquisse", "Extrusion", "Export"] {
        driver::click_the_button(&mut app, tab);
        for name in ["Recadrer", "Vue en coupe", "Matière devant", "Terminer"] {
            if driver::the_ribbon_s(&app, name).is_some() {
                still_there.push(format!("« {name} » under the tab « {tab} »"));
            }
        }
    }

    assert!(
        still_there.is_empty(),
        "the default ribbon still offers {still_there:?}",
    );
}

#[test]
fn the_four_moved_commands_can_be_put_back_from_the_preferences() {
    let mut settings = Settings::default();
    settings
        .shortcuts
        .bind(Command::SectionView, Chord::new(Key::K));
    let mut app = opened("the_four_moved_commands_in_the_palette", WIDE, settings);
    for keyed in ["Recadrer", "Vue en coupe"] {
        let read = driver::the_bar_s(&app, keyed)
            .accesskit_node()
            .label()
            .unwrap_or_default();
        assert_ne!(
            read, keyed,
            "the view bar's button reads exactly the palette's, so a search could find either: \
             the premise of this test",
        );
    }
    driver::press_with_command(&mut app, egui::Key::Comma);
    driver::click_exactly(&mut app, "Barre d'outils");
    for name in ["Recadrer", "Vue en coupe", "Matière devant", "Terminer"] {
        let offered = app
            .query_all_by_label(name)
            .any(|node| node.accesskit_node().role() == Role::Button);
        assert!(
            offered,
            "the palette does not offer « {name} » to put on the toolbar: {:?}",
            driver::on_screen(&app),
        );
    }

    let mut app = opened(
        "the_four_moved_commands_put_back_are_drawn",
        WIDE,
        driver::the_moved_commands_put_back(),
    );
    driver::start_a_sketch(&mut app);
    for name in [
        "Recadrer",
        "Vue en coupe",
        "Matière devant",
        "Terminer (Entrée)",
    ] {
        assert!(
            driver::the_ribbon_s(&app, name).is_some(),
            "« {name} », put back on the toolbar, is not drawn in the ribbon: {:?}",
            driver::on_screen(&app),
        );
    }
}

#[test]
fn resetting_the_toolbar_gives_the_profile_the_default_ribbon() {
    let test = "resetting_the_toolbar_gives_the_default_ribbon";
    let own = bar_of(vec![Item::group("drawing", commands(&[Command::ToolLine]))]);
    let mut app = opened(test, WIDE, own);
    assert!(
        driver::the_ribbon_s(&app, "Nouvelle esquisse").is_none(),
        "this profile's own toolbar holds no « Nouvelle esquisse »: the premise of this test",
    );

    driver::press_with_command(&mut app, egui::Key::Comma);
    driver::click_exactly(&mut app, "Barre d'outils");
    driver::click_the_button(&mut app, "↺ Barre par défaut");

    assert_eq!(
        driver::saved_settings(test).toolbar,
        cao_prefs::ToolbarLayout::default(),
        "the toolbar was reset, and the next start would read another one",
    );
    assert!(
        driver::the_ribbon_s(&app, "Nouvelle esquisse").is_some(),
        "the toolbar was reset, and the ribbon does not show the default one: {:?}",
        driver::on_screen(&app),
    );
}

#[test]
fn the_sketch_tab_fits_on_one_line_at_1440() {
    let size = egui::vec2(1440.0, 900.0);
    let mut app = opened("the_sketch_tab_fits_at_1440", size, Settings::default());
    driver::start_a_sketch(&mut app);

    let firsts = THE_FIVE_GROUPS.map(|(_, buttons)| the_ribbon_button(&app, buttons[0]));
    assert!(
        firsts
            .iter()
            .all(|first| (first.top() - firsts[0].top()).abs() <= ALIGNED_WITHIN),
        "at 1440 wide the groups of the sketch tab start on several lines: {firsts:?}",
    );

    let line_ends_at = firsts
        .iter()
        .map(|first| first.bottom())
        .fold(f32::NEG_INFINITY, f32::max);
    for (_, buttons) in THE_FIVE_GROUPS {
        for button in buttons {
            let at = the_ribbon_button(&app, button);
            assert!(
                at.top() < line_ends_at,
                "« {button} » at {at:?} went to a second line, the first ending at {line_ends_at}",
            );
            assert!(
                at.right() <= size.x,
                "« {button} » at {at:?} runs off a window 1440 wide",
            );
        }
    }
}

#[test]
fn the_group_names_of_a_tab_stand_on_one_line() {
    let size = egui::vec2(1440.0, 900.0);
    let mut app = opened(
        "the_group_names_stand_on_one_line",
        size,
        Settings::default(),
    );

    let mut wrong = Vec::new();
    for (tab, names) in [
        (
            "Esquisse",
            [
                "ESQUISSE",
                "DESSIN",
                "COTES ET CONTRAINTES",
                "MODIFIER",
                "ÉDITION",
            ]
            .as_slice(),
        ),
        ("Extrusion", ["MATIÈRE", "FORME", "ÉDITION"].as_slice()),
    ] {
        driver::click_the_button(&mut app, tab);
        let tops: Vec<(&str, f32)> = names
            .iter()
            .map(|name| (*name, line_of(&written(&app, name))))
            .collect();
        if tops
            .iter()
            .any(|(_, top)| (top - tops[0].1).abs() > ALIGNED_WITHIN)
        {
            wrong.push(format!("under the tab « {tab} »: {tops:?}"));
        }
    }

    assert!(
        wrong.is_empty(),
        "the names of the groups do not stand at one height:\n  {}",
        wrong.join("\n  "),
    );
}

/// The height a written name stands at: the galley's origin, because the ink
/// of an accented capital starts higher than that of a plain one.
fn line_of(name: &Written) -> f32 {
    name.text.pos.y
}

#[test]
fn a_wrapped_ribbon_keeps_each_line_s_names_level_and_above_the_next_line() {
    let app = opened(
        "a_wrapped_ribbon_keeps_each_line_level",
        egui::vec2(900.0, 800.0),
        Settings::default(),
    );

    let mut lines: Vec<(f32, Vec<(&str, Written)>)> = Vec::new();
    for (name, buttons) in THE_FIVE_GROUPS {
        let top = the_ribbon_button(&app, buttons[0]).top();
        let held = (name, written(&app, name));
        match lines
            .iter_mut()
            .find(|(line, _)| (line - top).abs() <= ALIGNED_WITHIN)
        {
            Some((_, names)) => names.push(held),
            None => lines.push((top, vec![held])),
        }
    }
    lines.sort_by(|one, other| one.0.total_cmp(&other.0));
    assert!(
        lines.len() >= 2,
        "at 900 wide the sketch tab did not wrap: the premise of this test",
    );

    for (rank, (top, names)) in lines.iter().enumerate() {
        let level = line_of(&names[0].1);
        let at: Vec<(&str, f32)> = names.iter().map(|(n, w)| (*n, line_of(w))).collect();
        assert!(
            at.iter().all(|(_, y)| (y - level).abs() <= ALIGNED_WITHIN),
            "the names of the line at {top} do not stand at one height: {at:?}",
        );
        if let Some((next, _)) = lines.get(rank + 1) {
            for (name, read) in names {
                assert!(
                    read.rect.bottom() <= *next,
                    "« {name} » at {:?} runs into the next line's buttons, which start at {next}",
                    read.rect,
                );
            }
        }
    }
}

#[test]
fn a_group_beside_a_wrapped_group_keeps_its_name_under_its_own_buttons() {
    use Command as C;
    let settings = bar_of(vec![
        Item::group("a", commands(&[C::ToolPoint])),
        Item::group(
            "drawing",
            commands(&[
                C::ToolRectangle,
                C::ToolArc,
                C::ToolEllipse,
                C::ToolDimension,
                C::ToolMeasure,
                C::ToolTrim,
                C::ToolSplit,
                C::ToolMirror,
                C::ToolFillet,
                C::ToolSelect,
                C::ToolCircle,
                C::ToolLineSymmetric,
                C::ExtrusionCut,
                C::ToolLine,
            ]),
        ),
    ]);
    let app = opened(
        "a_group_beside_a_wrapped_group",
        egui::vec2(890.0, 700.0),
        settings,
    );

    let beside = the_ribbon_button(&app, "Point");
    let first = the_ribbon_button(&app, "Rectangle");
    let last = the_ribbon_button(&app, "Ligne (L)");
    assert!(
        (beside.top() - first.top()).abs() <= ALIGNED_WITHIN && last.top() > first.bottom(),
        "« Point » at {beside:?} does not stand beside a group wrapped inside itself, from \
         {first:?} to {last:?}: the premise of this test",
    );

    let (name, wrapped) = (written(&app, "A"), written(&app, "DESSIN"));
    assert!(
        line_of(&name) < line_of(&wrapped) - ALIGNED_WITHIN && name.rect.top() >= beside.bottom(),
        "the name « A » at {} took the height of the group wrapped beside it, whose name \
         stands at {}, instead of staying under its own button, which ends at {}",
        line_of(&name),
        line_of(&wrapped),
        beside.bottom(),
    );
}

/// How many points of the screen a length of the part spans: what stepping
/// away from it shrinks and framing it brings back.
fn points_across_the_part(app: &driver::App) -> f32 {
    let at = |point: DVec3| {
        app.state()
            .where_the_view_shows(point)
            .unwrap_or_else(|| panic!("{point:?} of the part stands behind the eye"))
    };
    (at(DVec3::new(0.0, 100.0, 0.0)) - at(DVec3::ZERO)).length()
}

/// Steps well back from the block, then says whether this gesture framed it
/// again.
fn assert_frames_the_block(
    app: &mut driver::App,
    what: &str,
    gesture: impl FnOnce(&mut driver::App),
) {
    let the_block = DVec3::new(5.0, driver::ON_THE_BLOCK.x, driver::ON_THE_BLOCK.y);
    let over_the_block = app
        .state()
        .where_the_view_shows(the_block)
        .expect("the block stands before the eye");
    driver::step_back(app, over_the_block);
    let stepped_back = points_across_the_part(app);

    gesture(app);

    let framed = points_across_the_part(app);
    assert!(
        framed > 2.0 * stepped_back,
        "{what} spans the part over {framed} points, where it was stepped back to {stepped_back}",
    );
}

#[test]
fn what_left_the_ribbon_has_its_place_in_the_view_bar_or_the_sketch_pill() {
    let mut app = driver::a_block("what_left_the_ribbon_has_its_place");

    driver::start_a_sketch(&mut app);
    assert!(
        driver::a_sketch_is_open(&app),
        "no sketch was opened on the block, showing {:?}",
        driver::on_screen(&app),
    );
    driver::click_in_the_bar(&mut app, "Matière devant");
    driver::click_the_button(&mut app, "transparente");
    let reads = driver::the_bar_s(&app, "Matière devant")
        .accesskit_node()
        .label()
        .unwrap_or_default();
    assert!(
        reads.starts_with("Matière devant : transparente"),
        "« transparente » was picked in the view bar, and it reads {reads:?}",
    );
    driver::finish_the_sketch(&mut app);
    assert!(
        !driver::a_sketch_is_open(&app),
        "the pill's « Terminer » left the sketch open, showing {:?}",
        driver::on_screen(&app),
    );

    driver::click_in_the_bar(&mut app, "Vue en coupe");
    assert!(
        driver::a_button_reads(&app, "Inverser"),
        "« Vue en coupe » in the view bar opens no panel, showing {:?}",
        driver::on_screen(&app),
    );
    driver::click_in_the_bar(&mut app, "Vue en coupe");
    assert!(
        !driver::a_button_reads(&app, "Inverser"),
        "« Vue en coupe » clicked again in the view bar leaves its panel open",
    );

    assert_frames_the_block(&mut app, "« Recadrer » in the view bar", |app| {
        driver::click_in_the_bar(app, "Recadrer");
    });
}

#[test]
fn f_still_reframes_and_enter_still_finishes_the_sketch() {
    let mut app = driver::a_block("f_still_reframes_and_enter_still_finishes");

    driver::start_a_sketch(&mut app);
    assert!(
        driver::a_sketch_is_open(&app),
        "no sketch was opened on the block, showing {:?}",
        driver::on_screen(&app),
    );
    driver::press(&mut app, egui::Key::Enter);
    assert!(
        !driver::a_sketch_is_open(&app),
        "↵ left the sketch open, showing {:?}",
        driver::on_screen(&app),
    );

    assert_frames_the_block(&mut app, "F", |app| {
        driver::press(app, egui::Key::F);
    });
}

#[test]
fn every_command_of_the_default_tree_can_be_reached() {
    let mut app = driver::a_block("every_command_can_be_reached");
    driver::start_a_sketch(&mut app);
    let tree = cao_prefs::ToolbarLayout::default();

    for tab in &tree.items {
        let Item::Group { name, items, .. } = tab else {
            continue;
        };
        driver::click_the_button(&mut app, tab_named(name));
        reach(&mut app, items, 1);
    }
}
#[test]
fn down_a_side_the_groups_stack() {
    for edge in [Edge::Left, Edge::Right] {
        let test = format!("down_a_side_the_groups_stack_{edge:?}");
        let app = three_groups_on(&test, WIDE, edge);

        let (first, second, third) = (
            whole_group(&app, 0),
            whole_group(&app, 1),
            whole_group(&app, 2),
        );

        assert!(
            second.top() >= first.bottom() && third.top() >= second.bottom(),
            "on {edge:?}, the groups at {first:?}, {second:?} and {third:?} do not stand \
             one under the other",
        );
    }
}

#[test]
fn down_a_side_every_button_is_small() {
    for edge in [Edge::Left, Edge::Right] {
        let mut settings = bar_of(vec![Item::group(
            "drawing",
            vec![
                Item::Command(Command::ToolRectangle),
                Item::Command(Command::ToolCircle),
                Item::group(
                    "circles",
                    commands(&[Command::CircleCenter, Command::CircleTwoPoints]),
                ),
                Item::Command(Command::ToolLine),
            ],
        )]);
        settings.toolbar.edge = edge;
        let app = opened(
            &format!("down_a_side_every_button_is_small_{edge:?}"),
            WIDE,
            settings,
        );

        for button in ["Rectangle (R)", "Cercle (C)", "Ligne (L)"] {
            let height = the_button(&app, button).rect().height();
            assert!(
                height <= 26.0 + ALIGNED_WITHIN,
                "on {edge:?}, « {button} » in a large group is {height} high, where a small \
                 button is 20 or 26",
            );
        }
    }
}

#[test]
fn down_a_side_a_group_writes_its_name_under_its_buttons() {
    for edge in [Edge::Left, Edge::Right] {
        let test = format!("down_a_side_a_group_writes_its_name_{edge:?}");
        let app = three_groups_on(&test, WIDE, edge);

        for (rank, (name, buttons)) in THE_THREE_GROUPS.iter().enumerate() {
            let lowest = buttons
                .iter()
                .map(|button| row_starting(&app, button).rect().bottom())
                .fold(f32::NEG_INFINITY, f32::max);
            let written_name = written_whatever_its_case(&app, name);
            assert!(
                written_name.rect.top() >= lowest,
                "on {edge:?}, « {name} » (group {rank}) at {:?} is not under its buttons, \
                 which end at {lowest}",
                written_name.rect,
            );
        }
    }
}

#[test]
fn down_a_side_nothing_runs_off_the_panel() {
    for edge in [Edge::Left, Edge::Right] {
        let mut settings = Settings::default();
        settings.toolbar.edge = edge;
        let app = opened(
            &format!("down_a_side_nothing_runs_off_{edge:?}"),
            WIDE,
            settings,
        );
        for button in [
            "Nouvelle esquisse",
            "Sélection",
            "Point",
            "Annuler",
            "Ajuster",
            "Cercle",
            "Cercles",
        ] {
            let at = row_starting(&app, button).rect();
            assert!(
                driver::stands_beside_the_view(&app, at),
                "on {edge:?}, « {button} » at {at:?} runs off the panel over the view",
            );
        }
    }
}

#[test]
fn a_floating_ribbon_is_laid_out_as_along_the_top() {
    let app = three_groups_on("a_floating_ribbon_as_along_the_top", WIDE, Edge::Floating);
    let lines = line_of_each_group(&app);

    let first = lines[0].1[0];
    assert!(
        lines
            .iter()
            .flat_map(|(_, tops)| tops)
            .all(|top| (top - first).abs() <= ALIGNED_WITHIN),
        "a floating bar does not lay its groups side by side: {lines:?}",
    );
    let lefts: Vec<f32> = (0..3).map(|rank| whole_group(&app, rank).left()).collect();
    assert!(
        lefts.is_sorted(),
        "the groups of a floating bar stand in another order: {lefts:?}",
    );
}

#[test]
fn a_floating_ribbon_draws_each_group_at_its_size() {
    let mut settings = Settings::default();
    settings.toolbar.edge = Edge::Floating;
    let app = opened("a_floating_ribbon_draws_each_size", WIDE, settings);

    assert_tall(
        row_starting(&app, "Annuler").rect(),
        26.0,
        "« Annuler » in a floating bar",
    );
    let line = the_button(&app, "Ligne (L)").rect();
    assert!(
        line.height() >= 64.0,
        "« Ligne » in a floating bar is {} high",
        line.height(),
    );
}

#[test]
fn a_floating_ribbon_wraps_group_by_group_in_a_narrow_window() {
    let app = three_groups_on(
        "a_floating_ribbon_wraps_group_by_group",
        egui::vec2(900.0, 800.0),
        Edge::Floating,
    );
    let lines = line_of_each_group(&app);

    for (name, tops) in &lines {
        assert!(
            tops.iter()
                .all(|top| (top - tops[0]).abs() <= ALIGNED_WITHIN),
            "the group {name} is cut in two in a floating bar, its buttons at {tops:?}",
        );
    }
    assert!(
        lines.last().expect("three groups").1[0] > lines[0].1[0] + ALIGNED_WITHIN,
        "at 900 wide no group of a floating bar went to the next line: {lines:?}",
    );
}

#[test]
fn every_ribbon_button_keeps_its_name_as_its_accessible_label() {
    let app = opened("every_button_keeps_its_name", WIDE, Settings::default());
    for name in [
        "Nouvelle esquisse (Cmd+N)",
        "Ligne (L)",
        "Cercle (C)",
        "Annuler (Cmd+Z)",
    ] {
        the_button(&app, name);
    }

    let app = opened_with_names_off("every_button_keeps_its_name_without_names");
    for name in ["Ligne", "Cercle", "Annuler"] {
        the_button(&app, name);
    }
}

#[test]
fn the_ribbon_is_drawn_at_each_of_the_five_toolbar_positions() {
    for edge in Edge::ALL {
        let mut settings = Settings::default();
        settings.toolbar.edge = edge;
        let app = opened(&format!("the_ribbon_at_{edge:?}"), WIDE, settings);

        let at = row_starting(&app, "Nouvelle esquisse").rect();
        let screen = egui::Rect::from_min_size(egui::Pos2::ZERO, WIDE);
        let where_it_belongs = match edge {
            Edge::Top => at.bottom() < WIDE.y / 2.0,
            Edge::Bottom => at.top() > WIDE.y / 2.0,
            Edge::Left => at.right() < WIDE.x / 2.0,
            Edge::Right => at.left() > WIDE.x / 2.0,
            Edge::Floating => true,
        };

        assert!(
            screen.contains_rect(at) && where_it_belongs,
            "on {edge:?}, « Nouvelle esquisse » is at {at:?}",
        );
    }
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
    three_groups_on(test, size, Edge::Top)
}

fn three_groups_on(test: &str, size: egui::Vec2, edge: Edge) -> driver::App {
    use Command as C;
    let mut settings = bar_of(vec![
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
    settings.toolbar.edge = edge;
    opened(test, size, settings)
}

/// A group's name as it is written, and the buttons that open each of them.
const THE_THREE_GROUPS: [(&str, &[&str]); 3] = [
    (
        "DESSIN",
        &["Rectangle", "Cercle", "Arc", "Ellipse", "Point", "Mesurer"],
    ),
    (
        "ÉDITION",
        &[
            "Cote",
            "Ajuster",
            "Diviser",
            "Symétrie",
            "Congé",
            "Chanfrein",
        ],
    ),
    (
        "EXPORT",
        &[
            "Sélection",
            "Ligne",
            "Annuler",
            "Rétablir",
            "Nouvelle esquisse",
        ],
    ),
];

fn opened_with_names_off(test: &str) -> driver::App {
    let mut settings = Settings::default();
    settings.toolbar.show_labels = false;
    opened(test, WIDE, settings)
}

/// A group of the three, from its first button to its name under the last.
fn whole_group(app: &driver::App, rank: usize) -> egui::Rect {
    let (name, buttons) = THE_THREE_GROUPS[rank];
    buttons.iter().fold(
        written_whatever_its_case(app, name).rect,
        |group, button| group.union(row_starting(app, button).rect()),
    )
}

fn line_of_each_group(app: &driver::App) -> Vec<(&'static str, Vec<f32>)> {
    THE_THREE_GROUPS
        .iter()
        .map(|(name, buttons)| {
            (
                *name,
                buttons
                    .iter()
                    .map(|button| row_starting(app, button).rect().top())
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

fn assert_not_cut_short(name: &Written, whole: &str) {
    assert!(
        !name.text.galley.elided,
        "« {whole} » is written cut short, at {:?}",
        name.rect,
    );
}

fn tab_named(name: &str) -> &'static str {
    match name {
        "sketch" => "Esquisse",
        "extrusion" => "Extrusion",
        "export" => "Export",
        other => panic!("no tab of the default tree is called {other}"),
    }
}

/// Finds every entry of a run of the tree in the drawn bar: a command as a
/// button of the bar, a group below the first level, or asking for it, by
/// opening its menu and finding each of its entries there.
fn reach(app: &mut driver::App, items: &[Item], depth: usize) {
    for item in items {
        match item {
            Item::Command(command) => {
                let name = name_of(*command);
                assert!(
                    can_be_found(app, name),
                    "« {name} » cannot be found; the screen names {:?}",
                    driver::on_screen(app),
                );
            }
            Item::Group { name, size, items } => {
                if depth > 1 || *size == GroupSize::Menu {
                    let chevron = chevron_of(name);
                    driver::click_exactly(app, chevron);
                    for entry in items {
                        let Item::Command(command) = entry else {
                            panic!("the menu {name} holds something else than commands");
                        };
                        let rows = name_of(*command);
                        assert!(
                            can_be_found(app, rows),
                            "« {rows} » cannot be found in the open menu {name}; the screen names \
                             {:?}",
                            driver::on_screen(app),
                        );
                    }
                    driver::click_exactly(app, chevron);
                } else {
                    reach(app, items, depth + 1);
                }
            }
            Item::Separator => {}
        }
    }
}

fn chevron_of(group: &str) -> &'static str {
    match group {
        "circles" => "Cercles",
        "arcs" => "Arcs",
        "ellipses" => "Ellipses",
        "chamfers" => "Chanfreins",
        "constraints" => "Contraintes ⌄",
        "stl" => "Exporter en STL… ⌄",
        other => panic!("the group {other} has no chevron in the default tree"),
    }
}

fn can_be_found(app: &driver::App, name: &str) -> bool {
    app.query_all_by_role(Role::Button).any(|node| {
        node.accesskit_node().label().is_some_and(|label| {
            label == name
                || label.starts_with(&format!("{name} ("))
                || (name == "Matière devant" && label.starts_with(name))
        })
    })
}

/// What each command of the default tree reads on its button.
fn name_of(command: Command) -> &'static str {
    use Command as C;
    [
        (C::NewSketch, "Nouvelle esquisse"),
        (C::SectionView, "Vue en coupe"),
        (C::ToolSelect, "Sélection"),
        (C::ToolLine, "Ligne"),
        (C::ToolLineSymmetric, "Ligne symétrique"),
        (C::ToolRectangle, "Rectangle"),
        (C::ToolCircle, "Cercle"),
        (C::CircleCenter, "Centre et diamètre"),
        (C::CircleTwoPoints, "Deux points du bord"),
        (C::CircleThreePoints, "Deux points puis le centre"),
        (C::CircleTwoTangents, "Tangent à deux droites"),
        (C::CircleThreeTangents, "Tangent à trois droites"),
        (C::ToolArc, "Arc"),
        (C::ArcByCenter, "Centre, départ et arrivée"),
        (C::ArcByEnds, "Deux bouts et un point traversé"),
        (C::ToolEllipse, "Ellipse"),
        (C::EllipseByCentre, "Centre et deux axes"),
        (C::EllipseByEnds, "Deux bouts et une flèche"),
        (C::ToolPoint, "Point"),
        (C::ToolDimension, "Cote"),
        (C::ToolMeasure, "Mesurer"),
        (C::ToolTrim, "Ajuster"),
        (C::ToolSplit, "Diviser"),
        (C::ToolMirror, "Symétrie"),
        (C::ToolCircularPattern, "Réseau circulaire"),
        (C::ToolRectangularPattern, "Réseau rectangulaire"),
        (C::ToolFillet, "Congé"),
        (C::ToolChamfer, "Chanfrein"),
        (C::ChamferEqual, "Chanfrein à distance égale"),
        (C::ChamferAngled, "Chanfrein distance et angle"),
        (C::ChamferSided, "Chanfrein à deux distances"),
        (C::ToggleConstruction, "Construction"),
        (C::RulePerpendicular, "Perpendiculaire"),
        (C::RuleParallel, "Parallèle"),
        (C::RuleEqual, "Égalité"),
        (C::RuleCoincident, "Coïncidence"),
        (C::RuleCollinear, "Colinéaire"),
        (C::RuleTangent, "Tangence"),
        (C::RuleMidpoint, "Milieu"),
        (C::RuleFixed, "Fixe"),
        (C::RuleConcentric, "Concentrique"),
        (C::Reframe, "Recadrer"),
        (C::NextMatterInFront, "Matière devant"),
        (C::FinishSketch, "Terminer"),
        (C::Undo, "Annuler"),
        (C::Redo, "Rétablir"),
        (C::ExtrusionAdd, "Ajout de matière"),
        (C::ExtrusionCut, "Enlèvement de matière"),
        (C::ExtrusionStraight, "Extrusion droite"),
        (C::ExtrusionRevolution, "Révolution"),
        (C::ExportStlCoarse, "Grossière"),
        (C::ExportStlNormal, "Normale"),
        (C::ExportStlFine, "Fine"),
        (C::ExportStep, "Exporter en STEP…"),
    ]
    .into_iter()
    .find(|(named, _)| *named == command)
    .map(|(_, name)| name)
    .unwrap_or_else(|| panic!("{command:?} is not in the default tree's table"))
}
