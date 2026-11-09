//! What app · ui/tool_button.rs is held to.
//!
//! Closes #725.
//! - they take the colour of the widget's text: a disabled button greys its
//!   icon, an active tool turns it amber —
//!   `at_rest_a_tool_button_s_icon_wears_the_colour_of_its_name`,
//!   `a_greyed_out_tool_button_greys_its_icon_as_it_greys_its_name`,
//!   `the_tool_in_hand_turns_its_icon_amber_with_its_name`; with names off,
//!   the icon alone wears what its name would have been written in —
//!   `with_names_off_an_icon_at_rest_wears_the_colour_its_name_would_be_written_in`,
//!   `with_names_off_a_greyed_out_button_greys_its_icon_as_it_would_its_name`,
//!   `with_names_off_the_tool_in_hand_turns_its_icon_amber`; and in a menu —
//!   `a_greyed_out_menu_entry_greys_its_icon_as_it_greys_its_name`,
//!   `the_menu_entry_in_hand_turns_its_icon_amber_with_its_name`
//!
//! Closes #731. The ribbon around these buttons is held in
//! `crates/app/tests/ribbon_groups.rs`.
//! - Large button (C1): at least 50x64 —
//!   `a_large_button_is_at_least_50_wide_and_64_tall`
//! - icon of 22 px above the name —
//!   `a_large_button_wears_an_icon_of_22_points_above_its_name`
//! - the key in the top right corner —
//!   `a_large_button_writes_its_key_in_its_top_right_corner`
//! - drawn apart from the name —
//!   `a_large_button_writes_its_key_apart_from_its_name`
//! - in monospace, 9.5 px, the colour `text.shortcut` —
//!   `a_large_button_writes_its_key_small_in_monospace_and_the_shortcut_colour`
//! - two lines for a long name — `a_long_name_on_a_large_button_takes_two_lines`
//! - and one for a short name —
//!   `a_short_name_on_a_large_button_stays_on_one_line`
//! - its accessible label stays its name and its key, `Ligne (L)`, as the
//!   driver finds it — `a_large_button_is_called_by_its_name_and_its_key`
//! - state normal: no background, text `text.button` —
//!   `a_large_button_at_rest_has_no_background_and_writes_in_the_button_colour`
//! - state hover: background `background.hover_strong` —
//!   `a_hovered_large_button_wears_the_strong_hover_background`
//! - state active (amber): background, border and text —
//!   `the_tool_in_hand_is_amber_in_its_background_its_border_and_its_text`
//! - state disabled: text `text.disabled`, no hover —
//!   `a_greyed_out_large_button_writes_in_the_disabled_colour_and_does_not_answer_to_hover`
//! - all its text, read so: the key of a disabled button is `text.disabled`
//!   too, never brighter than its name —
//!   `a_greyed_out_large_button_writes_its_key_in_the_disabled_colour`
//! - found in review: a long key never overlaps the icon —
//!   `a_large_button_s_key_never_overlaps_its_icon`
//! - found in review: a single word longer than 70 is not cut in two —
//!   `a_word_longer_than_70_is_not_cut_in_two`
//! - Small button (C2): icon of 15 px left of the name —
//!   `a_small_button_wears_an_icon_of_15_points_left_of_its_name`
//! - key aligned right — `a_small_button_aligns_its_key_on_the_right`
//! - and the keys of a column line up on its right, the only place the
//!   alignment shows — `the_keys_of_a_small_column_line_up_on_its_right`
//! - its key in the colour `text.shortcut`, read from C1 —
//!   `a_small_button_writes_its_key_in_the_shortcut_colour`
//! - its accessible label is `Name (key)`, as on the large one —
//!   `a_small_button_is_called_by_its_name_and_its_key`
//! - same states as C1, normal —
//!   `a_small_button_at_rest_has_no_background_and_writes_in_the_button_colour`
//! - hover — `a_hovered_small_button_wears_the_strong_hover_background`
//! - active, its background, border and text —
//!   `the_small_tool_in_hand_is_amber_in_its_background_its_border_and_its_text`
//! - active, its icon — `the_small_tool_in_hand_turns_its_icon_amber_with_its_name`
//! - disabled, no hover —
//!   `a_greyed_out_small_button_writes_in_the_disabled_colour_and_does_not_answer_to_hover`
//! - disabled, its key greyed too —
//!   `a_greyed_out_small_button_writes_its_key_in_the_disabled_colour`
//! - disabled, its icon —
//!   `a_greyed_out_small_button_greys_its_icon_as_it_greys_its_name`
//! - laid in columns of 3 rows of 20 px —
//!   `small_buttons_on_three_rows_are_20_tall_and_fill_columns_of_three`
//! - or 2 rows of 26 px —
//!   `small_buttons_on_two_rows_are_26_tall_and_fill_columns_of_two`
//! - two commands on 2 rows make one column —
//!   `two_small_buttons_on_two_rows_make_one_column`
//! - filled top to bottom, then the next column, which starts past the widest
//!   button before it — `a_small_column_starts_past_the_widest_button_before_it`
//! - the whole fits inside the 64 px of a large button —
//!   `a_full_small_grid_fits_inside_the_64_points_of_a_large_button`
//! - Tooltip (C9): 230 px wide, on `#262A33` —
//!   `a_tooltip_is_230_wide_on_its_background_and_its_border`
//! - the name, then the key in a frame, then the sentence of help —
//!   `a_tooltip_lays_out_its_name_then_its_framed_key_then_its_help`
//! - the key in a frame (C14): monospace 10.5, border `#454B57`, radius 3 —
//!   `a_key_frame_writes_its_key_in_monospace_10_5_inside_its_border`
//! - a tooltip with no key draws no frame —
//!   `a_tooltip_without_a_key_draws_no_key_frame`
//! - and with no help, its name alone —
//!   `a_tooltip_without_help_writes_its_name_alone`
//!
//! The colour an icon shows is the picture it is painted from times the tint
//! it is painted with, which is how the GPU will draw it. Both are read: a
//! picture drawn black stays black under any tint, and a white one under a
//! tint that never changes wears one colour in every state.
//!
//! What is read are the shapes the frame painted, not the triangles they
//! become: `architecture.rs` keeps the word for those to `cao_solid`, whose
//! matter it names.

use std::sync::Arc;

use super::*;
use crate::ui::icon::{self, Icon};
use crate::ui::theme::tokens::{
    ACCENT_AMBER, ACCENT_AMBER_BACKGROUND, ACCENT_AMBER_BORDER, ACCENT_AMBER_LIGHT,
    BACKGROUND_HOVER_STRONG, BORDER_GROUP, BORDER_KEY, BORDER_TOOLTIP, TEXT_BUTTON, TEXT_DISABLED,
    TEXT_SHORTCUT,
};

/// How far two colours may drift per channel and still read as one: what
/// multiplying through a premultiplied alpha costs in rounding.
const SAME_COLOUR: u8 = 2;

/// How far two sizes may differ and still be the one the redesign asks for:
/// a texture and a galley are placed on rounded points.
const SIZED_WITHIN: f32 = 0.5;

/// C9 paints its background in the colour `border.group` already names.
const TOOLTIP_BACKGROUND: egui::Color32 = BORDER_GROUP;
const KEY_FRAME_BORDER: egui::Color32 = BORDER_KEY;

/// How far from an edge of the button a thing still sits in that corner: the
/// padding of a frame, and a little.
const CORNER_REACH: f32 = 10.0;

#[test]
fn at_rest_a_tool_button_s_icon_wears_the_colour_of_its_name() {
    let worn = worn(icon::LINE, true, false);

    assert_alike(worn.icon, worn.name(), "at rest");
}

#[test]
fn a_greyed_out_tool_button_greys_its_icon_as_it_greys_its_name() {
    let at_rest = worn(icon::LINE, true, false);
    let greyed = worn(icon::LINE, false, false);

    assert_alike(greyed.icon, greyed.name(), "greyed out");
    assert!(
        !alike(greyed.icon, at_rest.icon),
        "a greyed-out button's icon shows {:?}, as it does at rest: nothing says it cannot \
         be pressed",
        greyed.icon,
    );
}

#[test]
fn the_tool_in_hand_turns_its_icon_amber_with_its_name() {
    let in_hand = worn(icon::LINE, true, true);

    assert_alike(in_hand.icon, in_hand.name(), "in hand");
    assert_alike(
        in_hand.icon,
        ACCENT_AMBER_LIGHT,
        "in hand, against the theme's light amber",
    );
}

#[test]
fn with_names_off_an_icon_at_rest_wears_the_colour_its_name_would_be_written_in() {
    let alone = worn_alone(true, false);

    assert_alike(
        alone.icon,
        worn(icon::LINE, true, false).name(),
        "alone, at rest",
    );
}

#[test]
fn with_names_off_a_greyed_out_button_greys_its_icon_as_it_would_its_name() {
    let at_rest = worn_alone(true, false);
    let greyed = worn_alone(false, false);

    assert_alike(
        greyed.icon,
        worn(icon::LINE, false, false).name(),
        "alone, greyed out",
    );
    assert!(
        !alike(greyed.icon, at_rest.icon),
        "a greyed-out icon alone shows {:?}, as it does at rest: nothing says it cannot \
         be pressed",
        greyed.icon,
    );
}

#[test]
fn with_names_off_the_tool_in_hand_turns_its_icon_amber() {
    let in_hand = worn_alone(true, true);

    assert_alike(
        in_hand.icon,
        worn(icon::LINE, true, true).name(),
        "alone, in hand",
    );
    assert_alike(
        in_hand.icon,
        ACCENT_AMBER_LIGHT,
        "alone, in hand, against the theme's light amber",
    );
}

#[test]
fn a_greyed_out_menu_entry_greys_its_icon_as_it_greys_its_name() {
    let at_rest = worn_in_a_menu(true, false);
    let greyed = worn_in_a_menu(false, false);

    assert_alike(greyed.icon, greyed.name(), "a menu entry greyed out");
    assert!(
        !alike(greyed.icon, at_rest.icon),
        "a greyed-out menu entry's icon shows {:?}, as it does at rest",
        greyed.icon,
    );
}

/// Greying out only fades, so a tint fixed at the colour of rest would fade
/// with the name and pass for it: the entry in hand is where it shows.
#[test]
fn the_menu_entry_in_hand_turns_its_icon_amber_with_its_name() {
    let in_hand = worn_in_a_menu(true, true);

    assert_alike(in_hand.icon, in_hand.name(), "a menu entry in hand");
    assert_alike(
        in_hand.icon,
        ACCENT_AMBER,
        "a menu entry in hand, against the theme's amber",
    );
}

#[test]
fn a_large_button_is_at_least_50_wide_and_64_tall() {
    let button = at_rest("I", None).button;

    assert!(
        button.width() >= 50.0 && button.height() >= 64.0,
        "a large button with a short name measures {:?}, where C1 asks for 50 by 64 at least",
        button.size(),
    );
}

#[test]
fn a_large_button_wears_an_icon_of_22_points_above_its_name() {
    let drawn = at_rest("Ligne", None);
    let icon = drawn.icon();
    let (name, _) = drawn.written("Ligne");

    assert!(
        (icon.width() - 22.0).abs() < SIZED_WITHIN && (icon.height() - 22.0).abs() < SIZED_WITHIN,
        "the icon is painted at {:?} points, where C1 draws it at 22",
        icon.size(),
    );
    assert!(
        icon.bottom() <= name.top() && name.x_range().contains(icon.center().x),
        "the icon sits at {icon:?} and the name at {name:?}: the icon is not above the name",
    );
}

#[test]
fn a_large_button_writes_its_key_in_its_top_right_corner() {
    let drawn = at_rest("Ligne", Some("L"));
    let button = drawn.button;
    let (key, _) = drawn.written("L");

    assert!(
        button.right() - key.right() <= CORNER_REACH
            && key.top() - button.top() <= CORNER_REACH
            && button.expand(1.0).contains_rect(key),
        "the key sits at {key:?} in a button at {button:?}: not in its top right corner",
    );
}

#[test]
fn a_large_button_writes_its_key_apart_from_its_name() {
    let drawn = at_rest("Ligne", Some("L"));
    let (name, _) = drawn.written("Ligne");
    let (key, _) = drawn.written("L");

    assert!(
        !name.intersects(key),
        "the name at {name:?} and the key at {key:?} are written over each other",
    );
}

#[test]
fn a_large_button_writes_its_key_small_in_monospace_and_the_shortcut_colour() {
    let drawn = at_rest("Ligne", Some("L"));
    let (_, key) = drawn.written("L");
    let font = &key.galley.job.sections[0].format.font_id;

    assert_eq!(font.family, egui::FontFamily::Monospace, "the key's font");
    assert!(
        (font.size - 9.5).abs() < SIZED_WITHIN,
        "the key is written at {} points, where C1 writes it at 9.5",
        font.size,
    );
    assert_alike(written_in(key), TEXT_SHORTCUT, "the key");
}

#[test]
fn a_long_name_on_a_large_button_takes_two_lines() {
    for long in ["Nouvelle esquisse", "Ajout de matière"] {
        let drawn = at_rest(long, None);
        let (name, text) = drawn.written(long);

        assert_eq!(text.galley.rows.len(), 2, "the lines of « {long} »");
        assert!(
            name.width() <= 70.0 + SIZED_WITHIN,
            "« {long} » is {} wide, where C1 wraps it within 70",
            name.width(),
        );
        assert!(
            drawn.button.expand(1.0).contains_rect(name),
            "« {long} » at {name:?} runs out of its button at {:?}",
            drawn.button,
        );
    }
}

#[test]
fn a_short_name_on_a_large_button_stays_on_one_line() {
    let drawn = at_rest("Ligne", None);
    let (_, text) = drawn.written("Ligne");

    assert_eq!(text.galley.rows.len(), 1, "the lines of « Ligne »");
}

#[test]
fn a_large_button_s_key_never_overlaps_its_icon() {
    for (name, key) in [("Nouvelle esquisse", "Cmd+N"), ("Terminer", "Entrée")] {
        let drawn = at_rest(name, Some(key));
        let (written_key, _) = drawn.written(key);
        let icon = drawn.icon();

        assert!(
            !written_key.intersects(icon),
            "the key « {key} » of « {name} » at {written_key:?} overlaps the icon at {icon:?}",
        );
    }
}

#[test]
fn a_word_longer_than_70_is_not_cut_in_two() {
    for word in ["Perpendiculaire", "Concentrique", "Rectangulaire"] {
        let drawn = at_rest(word, None);
        let (_, text) = drawn.written(word);

        assert_eq!(text.galley.rows.len(), 1, "the lines of « {word} »");
    }
}

#[test]
fn a_large_button_is_called_by_its_name_and_its_key() {
    assert_eq!(called("Ligne", Some("L")), "Ligne (L)");
    assert_eq!(called("Ligne", None), "Ligne");
}

#[test]
fn a_large_button_at_rest_has_no_background_and_writes_in_the_button_colour() {
    let drawn = at_rest("Ligne", None);
    let (_, name) = drawn.written("Ligne");

    assert_eq!(
        drawn.backgrounds(),
        [],
        "the backgrounds of a button at rest"
    );
    assert_alike(written_in(name), TEXT_BUTTON, "at rest, its name");
}

#[test]
fn a_hovered_large_button_wears_the_strong_hover_background() {
    let backgrounds = drawn("Ligne", None, true, false, true).backgrounds();

    assert!(
        backgrounds
            .iter()
            .any(|fill| alike(*fill, BACKGROUND_HOVER_STRONG)),
        "a hovered button is filled with {backgrounds:?}, where C1 asks for {BACKGROUND_HOVER_STRONG:?}",
    );
}

#[test]
fn the_tool_in_hand_is_amber_in_its_background_its_border_and_its_text() {
    let drawn = drawn("Ligne", None, true, true, false);
    let (_, name) = drawn.written("Ligne");
    let frames = drawn.frames();

    assert!(
        frames
            .iter()
            .any(|frame| alike(frame.fill, ACCENT_AMBER_BACKGROUND)),
        "the tool in hand is filled with {:?}, where C1 asks for {ACCENT_AMBER_BACKGROUND:?}",
        drawn.backgrounds(),
    );
    assert!(
        frames
            .iter()
            .any(|frame| frame.stroke.width > 0.0 && alike(frame.stroke.color, ACCENT_AMBER_BORDER)),
        "the tool in hand has the borders {:?}, where C1 asks for {ACCENT_AMBER_BORDER:?}",
        frames.iter().map(|frame| frame.stroke).collect::<Vec<_>>(),
    );
    assert_alike(written_in(name), ACCENT_AMBER_LIGHT, "in hand, its name");
}

#[test]
fn a_greyed_out_large_button_writes_in_the_disabled_colour_and_does_not_answer_to_hover() {
    let drawn = drawn("Ligne", None, false, false, true);
    let (_, name) = drawn.written("Ligne");

    assert_alike(written_in(name), TEXT_DISABLED, "greyed out, its name");
    assert_eq!(
        drawn.backgrounds(),
        [],
        "the backgrounds of a greyed-out button under the pointer",
    );
}

#[test]
fn a_greyed_out_large_button_writes_its_key_in_the_disabled_colour() {
    let drawn = drawn("Ligne", Some("L"), false, false, false);
    let (_, key) = drawn.written("L");

    assert_alike(written_in(key), TEXT_DISABLED, "greyed out, its key");
}

#[test]
fn a_small_button_wears_an_icon_of_15_points_left_of_its_name() {
    let drawn = small("Ligne", None, true, false, false);
    let icon = drawn.icon();
    let (name, _) = drawn.written("Ligne");

    assert!(
        (icon.width() - 15.0).abs() < SIZED_WITHIN && (icon.height() - 15.0).abs() < SIZED_WITHIN,
        "the icon is painted at {:?} points, where C2 draws it at 15",
        icon.size(),
    );
    assert!(
        icon.right() <= name.left() && name.y_range().contains(icon.center().y),
        "the icon sits at {icon:?} and the name at {name:?}: the icon is not left of the name",
    );
}

#[test]
fn a_small_button_aligns_its_key_on_the_right() {
    let drawn = small("Ligne", Some("L"), true, false, false);
    let (name, _) = drawn.written("Ligne");
    let (key, _) = drawn.written("L");

    assert!(
        drawn.button.right() - key.right() <= CORNER_REACH
            && drawn.button.expand(1.0).contains_rect(key),
        "the key sits at {key:?} in a button at {:?}: not on its right",
        drawn.button,
    );
    assert!(
        name.right() <= key.left(),
        "the name at {name:?} runs into the key at {key:?}",
    );
}

#[test]
fn a_small_button_writes_its_key_in_the_shortcut_colour() {
    let drawn = small("Ligne", Some("L"), true, false, false);
    let (_, key) = drawn.written("L");

    assert_alike(written_in(key), TEXT_SHORTCUT, "the key");
}

#[test]
fn a_small_button_is_called_by_its_name_and_its_key() {
    let called_with = |key: Option<&str>| {
        called_by(|ui| small_button(ui, true, false, icon::LINE, "Ligne", key, Rows::Three))
    };

    assert_eq!(called_with(Some("L")), "Ligne (L)");
    assert_eq!(called_with(None), "Ligne");
}

#[test]
fn a_small_button_at_rest_has_no_background_and_writes_in_the_button_colour() {
    let drawn = small("Ligne", None, true, false, false);
    let (_, name) = drawn.written("Ligne");

    assert_eq!(
        drawn.backgrounds(),
        [],
        "the backgrounds of a button at rest"
    );
    assert_alike(written_in(name), TEXT_BUTTON, "at rest, its name");
}

#[test]
fn a_hovered_small_button_wears_the_strong_hover_background() {
    let backgrounds = small("Ligne", None, true, false, true).backgrounds();

    assert!(
        backgrounds
            .iter()
            .any(|fill| alike(*fill, BACKGROUND_HOVER_STRONG)),
        "a hovered small button is filled with {backgrounds:?}, where C1 asks for \
         {BACKGROUND_HOVER_STRONG:?}",
    );
}

#[test]
fn the_small_tool_in_hand_is_amber_in_its_background_its_border_and_its_text() {
    let drawn = small("Ligne", None, true, true, false);
    let (_, name) = drawn.written("Ligne");
    let frames = drawn.frames();

    assert!(
        frames
            .iter()
            .any(|frame| alike(frame.fill, ACCENT_AMBER_BACKGROUND)),
        "the small tool in hand is filled with {:?}, where C1 asks for \
         {ACCENT_AMBER_BACKGROUND:?}",
        drawn.backgrounds(),
    );
    assert!(
        frames.iter().any(
            |frame| frame.stroke.width > 0.0 && alike(frame.stroke.color, ACCENT_AMBER_BORDER)
        ),
        "the small tool in hand has the borders {:?}, where C1 asks for {ACCENT_AMBER_BORDER:?}",
        frames.iter().map(|frame| frame.stroke).collect::<Vec<_>>(),
    );
    assert_alike(written_in(name), ACCENT_AMBER_LIGHT, "in hand, its name");
}

#[test]
fn the_small_tool_in_hand_turns_its_icon_amber_with_its_name() {
    let in_hand = worn_by(|ui| small_button(ui, true, true, icon::LINE, "Line", None, Rows::Three));

    assert_alike(in_hand.icon, in_hand.name(), "small, in hand");
    assert_alike(in_hand.icon, ACCENT_AMBER_LIGHT, "small, in hand");
}

#[test]
fn a_greyed_out_small_button_writes_in_the_disabled_colour_and_does_not_answer_to_hover() {
    let drawn = small("Ligne", None, false, false, true);
    let (_, name) = drawn.written("Ligne");

    assert_alike(written_in(name), TEXT_DISABLED, "greyed out, its name");
    assert_eq!(
        drawn.backgrounds(),
        [],
        "the backgrounds of a greyed-out small button under the pointer",
    );
}

#[test]
fn a_greyed_out_small_button_writes_its_key_in_the_disabled_colour() {
    let drawn = small("Ligne", Some("L"), false, false, false);
    let (_, key) = drawn.written("L");

    assert_alike(written_in(key), TEXT_DISABLED, "greyed out, its key");
}

#[test]
fn a_greyed_out_small_button_greys_its_icon_as_it_greys_its_name() {
    let greyed =
        worn_by(|ui| small_button(ui, false, false, icon::LINE, "Line", None, Rows::Three));

    assert_alike(greyed.icon, greyed.name(), "small, greyed out");
    assert_alike(greyed.icon, TEXT_DISABLED, "small, greyed out");
}

const UNNAMED: [(&str, Option<&str>); 9] = [
    ("Un", None),
    ("Deux", None),
    ("Trois", None),
    ("Quatre", None),
    ("Cinq", None),
    ("Six", None),
    ("Sept", None),
    ("Huit", None),
    ("Neuf", None),
];

#[test]
fn small_buttons_on_three_rows_are_20_tall_and_fill_columns_of_three() {
    let grid = grid_of(Rows::Three, &UNNAMED[..7]);

    grid.assert_all_tall(20.0);
    assert_eq!(grid.columns(), [3, 3, 1], "seven commands on three rows");
    grid.assert_filled_down_each_column();
}

#[test]
fn small_buttons_on_two_rows_are_26_tall_and_fill_columns_of_two() {
    let grid = grid_of(Rows::Two, &UNNAMED[..5]);

    grid.assert_all_tall(26.0);
    assert_eq!(grid.columns(), [2, 2, 1], "five commands on two rows");
    grid.assert_filled_down_each_column();
}

#[test]
fn two_small_buttons_on_two_rows_make_one_column() {
    let grid = grid_of(Rows::Two, &UNNAMED[..2]);

    assert_eq!(grid.columns(), [2], "two commands on two rows");
}

#[test]
fn a_small_column_starts_past_the_widest_button_before_it() {
    let grid = grid_of(
        Rows::Three,
        &[
            ("Un", None),
            ("Réseau rectangulaire", None),
            ("Trois", None),
            ("Quatre", None),
        ],
    );

    let widest = grid.buttons[..3]
        .iter()
        .map(|button| button.right())
        .fold(f32::NEG_INFINITY, f32::max);
    assert!(
        grid.buttons[3].left() >= widest,
        "the next column starts at {}, inside a column reaching {widest}",
        grid.buttons[3].left(),
    );
}

#[test]
fn the_keys_of_a_small_column_line_up_on_its_right() {
    let grid = grid_of(
        Rows::Three,
        &[
            ("Un", Some("U")),
            ("Réseau rectangulaire", Some("R")),
            ("Trois", Some("T")),
        ],
    );
    let drawn = Drawn {
        button: egui::Rect::NOTHING,
        shapes: grid.shapes,
    };
    let rights: Vec<f32> = ["U", "R", "T"]
        .iter()
        .map(|key| drawn.written(key).0.right())
        .collect();

    assert!(
        rights
            .iter()
            .all(|right| (right - rights[0]).abs() <= SIZED_WITHIN),
        "the keys of one column end at {rights:?}: they do not line up on its right",
    );
}

#[test]
fn a_full_small_grid_fits_inside_the_64_points_of_a_large_button() {
    for (rows, full) in [(Rows::Three, 9), (Rows::Two, 6)] {
        let height = grid_of(rows, &UNNAMED[..full]).whole.height();

        assert!(
            height <= 64.0 + SIZED_WITHIN,
            "a full grid is {height} high, where C2 fits it inside the 64 of a large button",
        );
    }
}

#[test]
fn a_key_frame_writes_its_key_in_monospace_10_5_inside_its_border() {
    let drawn = drawn_by(false, |ui| key_frame(ui, "L"));
    let (at, key) = drawn.written("L");
    let font = &key.galley.job.sections[0].format.font_id;
    let frame = *drawn
        .frames()
        .iter()
        .find(|frame| frame.stroke.color == KEY_FRAME_BORDER)
        .unwrap_or_else(|| {
            panic!(
                "no frame is drawn in {KEY_FRAME_BORDER:?}, only {:?}",
                drawn.frames(),
            )
        });

    assert_eq!(font.family, egui::FontFamily::Monospace, "the key's font");
    assert!(
        (font.size - 10.5).abs() < SIZED_WITHIN,
        "the key is written at {} points, where C14 writes it at 10.5",
        font.size,
    );
    assert_eq!(
        frame.corner_radius,
        egui::CornerRadius::same(3),
        "its radius"
    );
    assert!(
        frame.rect.contains_rect(at),
        "the key stands inside its frame"
    );
}

#[test]
fn a_tooltip_is_230_wide_on_its_background_and_its_border() {
    let tip = tooltip_of("Ligne", Some("L"), Some("Clics successifs."));

    assert!(
        (tip.button.width() - 230.0).abs() <= 1.0,
        "the tooltip is {} wide, where C9 asks for 230",
        tip.button.width(),
    );
    assert_eq!(tip.frame().fill, TOOLTIP_BACKGROUND, "its background");
    assert_eq!(tip.frame().stroke.color, BORDER_TOOLTIP, "its border");
}

#[test]
fn a_tooltip_lays_out_its_name_then_its_framed_key_then_its_help() {
    let tip = tooltip_of("Ligne", Some("L"), Some("Clics successifs."));
    let (name, _) = tip.written("Ligne");
    let (key, _) = tip.written("L");
    let (help, _) = tip.written("Clics successifs.");

    assert!(
        tip.frames()
            .iter()
            .any(|frame| frame.stroke.color == KEY_FRAME_BORDER && frame.rect.contains_rect(key)),
        "the key at {key:?} stands in no frame of its own: {:?}",
        tip.frames(),
    );
    assert!(
        key.left() >= name.right() || key.top() >= name.bottom(),
        "the key at {key:?} does not come after the name at {name:?}",
    );
    assert!(
        help.top() >= name.bottom() && help.top() >= key.bottom(),
        "the help at {help:?} is not below the name at {name:?} and the key at {key:?}",
    );
}

#[test]
fn a_tooltip_without_a_key_draws_no_key_frame() {
    let tip = tooltip_of("Ligne", None, Some("Clics successifs."));

    assert!(
        !tip.frames()
            .iter()
            .any(|frame| frame.stroke.color == KEY_FRAME_BORDER),
        "a tooltip with no key draws a frame: {:?}",
        tip.frames(),
    );
}

#[test]
fn a_tooltip_without_help_writes_its_name_alone() {
    let tip = tooltip_of("Cercles", None, None);

    assert_eq!(tip.written_texts(), ["Cercles"]);
}

/// The colours a button shows in one frame: what a pixel its icon fully
/// covers shows, and what its name's glyphs are painted in, if it writes one.
struct Worn {
    icon: egui::Color32,
    name: Option<egui::Color32>,
}

impl Worn {
    fn name(&self) -> egui::Color32 {
        self.name.expect("the button wrote no name")
    }
}

fn worn(icon: Icon, enabled: bool, on: bool) -> Worn {
    worn_by(|ui| tool_button(ui, enabled, on, icon, "Line", None))
}

fn worn_alone(enabled: bool, on: bool) -> Worn {
    let alone = worn_by(|ui| icon_button(ui, enabled, on, icon::LINE, "Line"));
    assert!(
        alone.name.is_none(),
        "with names off, the button still writes its name",
    );
    alone
}

fn worn_in_a_menu(enabled: bool, on: bool) -> Worn {
    worn_by(|ui| menu_entry(ui, enabled, on, icon::LINE, "Line".to_owned()))
}

fn worn_by(mut draw: impl FnMut(&mut egui::Ui) -> egui::Response) -> Worn {
    let ctx = egui::Context::default();
    crate::ui::theme::install(&ctx);
    icon::install(&ctx);

    let input = egui::RawInput {
        screen_rect: Some(egui::Rect::from_min_size(
            egui::Pos2::ZERO,
            egui::vec2(400.0, 200.0),
        )),
        ..Default::default()
    };
    let mut button = egui::Rect::NOTHING;
    let mut output = ctx.run_ui(input, |ui| {
        button = draw(ui).rect;
    });
    let mut pictures = std::mem::take(&mut output.textures_delta.set);
    output.textures_delta.clear();

    let shapes = every_shape(output.shapes);
    let within = button.expand(1.0);

    let drawn = shapes
        .iter()
        .find_map(|shape| match shape {
            egui::Shape::Rect(rect)
                if rect.fill_texture_id() != egui::TextureId::default()
                    && within.contains_rect(rect.rect) =>
            {
                Some(rect)
            }
            _ => None,
        })
        .unwrap_or_else(|| panic!("the button painted no picture inside {button:?}"));
    let picture: Arc<egui::ColorImage> = pictures
        .remove(&drawn.fill_texture_id())
        .into_iter()
        .flatten()
        .find(|delta| delta.pos.is_none())
        .map(|delta| {
            let egui::ImageData::Color(image) = delta.image;
            image
        })
        .expect("the button painted a picture the frame never handed over");

    let name = shapes.iter().find_map(|shape| match shape {
        egui::Shape::Text(text) if within.contains_rect(shape.visual_bounding_rect()) => {
            Some(written_in(text))
        }
        _ => None,
    });

    Worn {
        icon: seen_through(drawn_in(&picture), drawn.fill),
        name,
    }
}

/// What a large button painted, in the last of a few frames.
struct Drawn {
    button: egui::Rect,
    shapes: Vec<egui::Shape>,
}

impl Drawn {
    fn icon(&self) -> egui::Rect {
        self.shapes
            .iter()
            .find_map(|shape| match shape {
                egui::Shape::Rect(rect) if rect.fill_texture_id() != egui::TextureId::default() => {
                    Some(rect.rect)
                }
                _ => None,
            })
            .expect("the button painted no icon")
    }

    fn written(&self, exactly: &str) -> (egui::Rect, &egui::epaint::TextShape) {
        self.shapes
            .iter()
            .find_map(|shape| match shape {
                egui::Shape::Text(text) if text.galley.text() == exactly => {
                    Some((shape.visual_bounding_rect(), text))
                }
                _ => None,
            })
            .unwrap_or_else(|| {
                panic!(
                    "nothing written reads exactly {exactly:?}; the button wrote {:?}",
                    self.written_texts(),
                )
            })
    }

    /// The tooltip's own rectangle, for a `Drawn` whose button is that frame.
    fn frame(&self) -> &egui::epaint::RectShape {
        self.frames()
            .into_iter()
            .find(|frame| frame.rect == self.button)
            .expect("the tooltip's frame")
    }

    fn written_texts(&self) -> Vec<String> {
        self.shapes
            .iter()
            .filter_map(|shape| match shape {
                egui::Shape::Text(text) => Some(text.galley.text().to_owned()),
                _ => None,
            })
            .collect()
    }

    /// The rectangles painted over the button that are no picture.
    fn frames(&self) -> Vec<&egui::epaint::RectShape> {
        let within = self.button.expand(1.0);
        self.shapes
            .iter()
            .filter_map(|shape| match shape {
                egui::Shape::Rect(rect)
                    if rect.fill_texture_id() == egui::TextureId::default()
                        && within.contains_rect(rect.rect) =>
                {
                    Some(rect)
                }
                _ => None,
            })
            .collect()
    }

    fn backgrounds(&self) -> Vec<egui::Color32> {
        self.frames()
            .iter()
            .map(|frame| frame.fill)
            .filter(|fill| fill.a() > 0)
            .collect()
    }
}

fn at_rest(name: &str, key: Option<&str>) -> Drawn {
    drawn(name, key, true, false, false)
}

fn themed() -> egui::Context {
    let ctx = egui::Context::default();
    crate::ui::theme::install(&ctx);
    icon::install(&ctx);
    ctx
}

fn screen(events: Vec<egui::Event>) -> egui::RawInput {
    egui::RawInput {
        screen_rect: Some(egui::Rect::from_min_size(
            egui::Pos2::ZERO,
            egui::vec2(900.0, 300.0),
        )),
        events,
        ..Default::default()
    }
}

/// The first frame finds where the button lands; the pointer comes onto it
/// after that, the frame after which a widget learns it is hovered.
fn drawn_by(hovered: bool, mut draw: impl FnMut(&mut egui::Ui) -> egui::Response) -> Drawn {
    let ctx = themed();
    let mut button = egui::Rect::NOTHING;
    let mut shapes = Vec::new();

    for frame in 0..4 {
        let events = if hovered && frame > 0 {
            vec![egui::Event::PointerMoved(button.center())]
        } else {
            Vec::new()
        };
        let mut output = ctx.run_ui(screen(events), |ui| {
            button = draw(ui).rect;
        });
        output.textures_delta.clear();
        shapes = every_shape(output.shapes);
    }

    Drawn { button, shapes }
}

/// A button hovered long enough for its tooltip to open, read as a `Drawn`
/// whose button is the tooltip's frame and whose shapes are those inside it.
/// The pointer moves once: egui opens a tooltip only once it has come to rest.
fn tooltip_of(name: &str, key: Option<&str>, help: Option<&str>) -> Drawn {
    let ctx = themed();
    let mut button = egui::Rect::NOTHING;
    let mut shapes = Vec::new();

    for frame in 0..8 {
        let events = if frame == 1 {
            vec![egui::Event::PointerMoved(button.center())]
        } else {
            Vec::new()
        };
        let input = egui::RawInput {
            time: Some(f64::from(frame)),
            ..screen(events)
        };
        let mut output = ctx.run_ui(input, |ui| {
            let response = ui.button("X");
            button = response.rect;
            tooltip(response, name, key, help);
        });
        output.textures_delta.clear();
        shapes = every_shape(output.shapes);
    }

    let frame = shapes
        .iter()
        .find_map(|shape| match shape {
            egui::Shape::Rect(rect)
                if rect.fill == TOOLTIP_BACKGROUND && rect.rect.width() > 100.0 =>
            {
                Some(rect.rect)
            }
            _ => None,
        })
        .unwrap_or_else(|| panic!("no tooltip is painted on {TOOLTIP_BACKGROUND:?}"));
    let within = frame.expand(1.0);
    Drawn {
        button: frame,
        shapes: shapes
            .into_iter()
            .filter(|shape| within.contains_rect(shape.visual_bounding_rect()))
            .collect(),
    }
}

fn drawn(name: &str, key: Option<&str>, enabled: bool, on: bool, hovered: bool) -> Drawn {
    drawn_by(hovered, |ui| {
        tool_button(ui, enabled, on, icon::LINE, name, key)
    })
}

fn small(name: &str, key: Option<&str>, enabled: bool, on: bool, hovered: bool) -> Drawn {
    drawn_by(hovered, |ui| {
        small_button(ui, enabled, on, icon::LINE, name, key, Rows::Three)
    })
}

/// A grid of small buttons as it settles, each cell's button in the order the
/// cells were asked for.
struct Grid {
    buttons: Vec<egui::Rect>,
    whole: egui::Rect,
    shapes: Vec<egui::Shape>,
}

impl Grid {
    fn assert_all_tall(&self, height: f32) {
        for button in &self.buttons {
            assert!(
                (button.height() - height).abs() < SIZED_WITHIN,
                "a small button is {} high, where C2 asks for {height}",
                button.height(),
            );
        }
    }

    /// How many buttons stand in each column, read off their left edges: a
    /// button starting where the previous one starts is in the same column.
    fn columns(&self) -> Vec<usize> {
        let mut columns = Vec::new();
        let mut left = f32::NAN;
        for button in &self.buttons {
            if (button.left() - left).abs() <= SIZED_WITHIN {
                *columns.last_mut().expect("a column to stand in") += 1;
            } else {
                columns.push(1);
                left = button.left();
            }
        }
        columns
    }

    fn assert_filled_down_each_column(&self) {
        let mut next = 0;
        for in_the_column in self.columns() {
            let column = &self.buttons[next..next + in_the_column];
            assert!(
                (column[0].top() - self.buttons[0].top()).abs() <= SIZED_WITHIN,
                "a column starts at {}, not at the top of the grid {}",
                column[0].top(),
                self.buttons[0].top(),
            );
            for pair in column.windows(2) {
                assert!(
                    pair[1].top() >= pair[0].bottom(),
                    "a button at {:?} does not come after the one at {:?} in its column",
                    pair[1],
                    pair[0],
                );
            }
            next += in_the_column;
        }
    }
}

fn grid_of(rows: Rows, names: &[(&str, Option<&str>)]) -> Grid {
    let ctx = themed();
    let mut buttons = Vec::new();
    let mut whole = egui::Rect::NOTHING;
    let mut shapes = Vec::new();

    for _ in 0..4 {
        buttons.clear();
        let mut output = ctx.run_ui(screen(Vec::new()), |ui| {
            whole = ui
                .scope(|ui| {
                    small_grid(ui, rows, names.len(), |ui, rank| {
                        let (name, key) = names[rank];
                        let button = small_button(ui, true, false, icon::LINE, name, key, rows);
                        buttons.push(button.rect);
                    });
                })
                .response
                .rect;
        });
        output.textures_delta.clear();
        shapes = every_shape(output.shapes);
    }

    Grid {
        buttons,
        whole,
        shapes,
    }
}

/// What a button is called for whoever cannot see it, and for the driver.
fn called(name: &str, key: Option<&str>) -> String {
    called_by(|ui| tool_button(ui, true, false, icon::LINE, name, key))
}

fn called_by(mut draw: impl FnMut(&mut egui::Ui) -> egui::Response) -> String {
    let ctx = themed();
    ctx.enable_accesskit();
    let mut output = ctx.run_ui(screen(Vec::new()), |ui| {
        draw(ui);
    });
    output.textures_delta.clear();

    let update = output
        .platform_output
        .accesskit_update
        .expect("the frame describes itself for accessibility");
    update
        .nodes
        .iter()
        .find(|(_, node)| node.role() == egui::accesskit::Role::Button)
        .and_then(|(_, node)| node.label())
        .expect("a button with a label")
        .to_owned()
}

/// The shapes a frame painted, with every group opened.
fn every_shape(clipped: Vec<egui::epaint::ClippedShape>) -> Vec<egui::Shape> {
    let mut pending: Vec<egui::Shape> = clipped.into_iter().map(|clipped| clipped.shape).collect();
    let mut shapes = Vec::new();
    while let Some(shape) = pending.pop() {
        match shape {
            egui::Shape::Vec(group) => pending.extend(group),
            shape => shapes.push(shape),
        }
    }
    shapes
}

/// The colour a text's glyphs reach the screen in, settled as epaint's
/// tessellator settles it: the override if there is one, the fallback for a
/// glyph left to it, then the fading of a greyed-out widget.
fn written_in(text: &egui::epaint::TextShape) -> egui::Color32 {
    let visuals = text
        .galley
        .rows
        .iter()
        .map(|placed| &placed.row.visuals)
        .find(|visuals| !visuals.glyph_vertex_range.is_empty())
        .expect("a name has glyphs");
    let glyph = visuals.mesh.vertices[visuals.glyph_vertex_range.start].color;

    let colour = match text.override_text_color {
        Some(overridden) => overridden,
        None if glyph == egui::Color32::PLACEHOLDER => text.fallback_color,
        None => glyph,
    };
    if text.opacity_factor < 1.0 {
        colour.gamma_multiply(text.opacity_factor)
    } else {
        colour
    }
}

/// The colour the picture's strokes are drawn in, read where they cover a
/// pixel most and freed of that coverage.
fn drawn_in(picture: &egui::ColorImage) -> [u8; 3] {
    let densest = picture
        .pixels
        .iter()
        .max_by_key(|pixel| pixel.a())
        .expect("a picture has pixels");
    assert!(densest.a() > 0, "the icon's picture is blank");

    let freed = |channel: u8| (u32::from(channel) * 255 / u32::from(densest.a())).min(255) as u8;
    [freed(densest.r()), freed(densest.g()), freed(densest.b())]
}

/// What a pixel drawn in `drawn` and painted with `tint` shows.
fn seen_through(drawn: [u8; 3], tint: egui::Color32) -> egui::Color32 {
    let times = |channel: u8, by: u8| (u32::from(channel) * u32::from(by) / 255) as u8;
    egui::Color32::from_rgba_premultiplied(
        times(drawn[0], tint.r()),
        times(drawn[1], tint.g()),
        times(drawn[2], tint.b()),
        tint.a(),
    )
}

fn alike(one: egui::Color32, other: egui::Color32) -> bool {
    one.to_array()
        .iter()
        .zip(other.to_array())
        .all(|(a, b)| a.abs_diff(b) <= SAME_COLOUR)
}

fn assert_alike(icon: egui::Color32, expected: egui::Color32, state: &str) {
    assert!(
        alike(icon, expected),
        "{state}, the button shows {icon:?} where {expected:?} is expected",
    );
}
