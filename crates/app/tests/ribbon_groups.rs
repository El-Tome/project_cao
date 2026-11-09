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
//! - a thin separator between groups —
//!   `a_thin_separator_stands_between_two_groups`
//! - below 1440 px the ribbon wraps group by group rather than button by button
//!   (#650) — `below_1440_the_ribbon_wraps_group_by_group_rather_than_button_by_button`
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
