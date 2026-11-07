//! A command has a picture as well as a name: the redesign's icons, worn by
//! the ribbon's buttons, the panel toggles, the menus and the eye of a sketch.
//!
//! Closes #725.
//! - the SVGs are drawn by `egui_extras` with its `svg` feature (`resvg`), at
//!   22, 15 and 14 px — `each_icon_is_drawn_at_the_size_its_widget_gives_it`
//! - crisp on a Retina screen — no test: the driver lays the frame out and
//!   paints no pixel; checked by launching the app on the Mac's Retina screen
//!   and looking at a capture of the ribbon
//! - they take the colour of the widget's text: a disabled button greys its
//!   icon, an active tool turns it amber —
//!   `an_icon_wears_the_colour_its_button_writes_in`
//! - they appear in today's ribbon buttons, the icon above the name —
//!   `a_ribbon_button_shows_its_icon_above_its_name`
//! - in the panel toggles — `a_panel_toggle_shows_its_icon_beside_its_name`
//! - and in the menus — `a_menu_entry_shows_its_icon_beside_its_name`
//! - with « Noms des boutons » off, a button is its icon alone, and its name
//!   is in the tooltip —
//!   `with_names_off_a_button_is_its_icon_alone_and_its_name_comes_on_hover`
//! - the eye of a sketch is the `eye` / `eye-off` icon instead of the emoji —
//!   `a_sketch_s_eye_is_drawn_open_while_shown_and_shut_while_hidden`
//! - every button keeps its name as its accessible label, so the headless
//!   driver still finds an icon-only button —
//!   `with_names_off_a_button_is_still_found_and_pressed_by_its_name`
//! - `language.rs`: the icon files keep their English names — no test: held
//!   by `no_file_and_no_folder_carries_a_french_name` in `language.rs`, which
//!   walks every file of the workspace, wherever the icons are embedded from
//! - the shortcuts and what each button does — no test: the driver tests that
//!   pick a tool by its name or its key (`picking_a_tool.rs`,
//!   `selecting_everything.rs`, …) and the shortcut tests of `cao_prefs` hold
//!   them already, and stay green on this branch
