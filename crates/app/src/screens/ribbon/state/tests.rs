//! What app · screens/ribbon/state.rs is held to.
//!
//! Closes #319.
//! - the three-state button is shown only while a sketch is open, next to the
//!   re-align button — read, with the user, as a ribbon button beside
//!   « Recadrer » that is disabled outside a sketch rather than hidden, as
//!   « Recadrer » was until #728 had it frame the whole part outside a sketch —
//!   `the_matter_in_front_button_is_offered_only_while_a_sketch_is_open`
//! - its icon shows the current one — its name says the state,
//!   `the_matter_in_front_button_names_the_state_it_is_in`, and so does its
//!   icon, all it shows with labels off, which must still tell the three apart
//!   — `the_matter_in_front_button_wears_the_icon_of_the_state_it_is_in`
//!
//! Closes #725.
//! - every `Command` has an icon, looked up by command — and the matter in
//!   front, drawn three times in the redesign, wears the icon of the state it
//!   is in, as its name already says it —
//!   `the_matter_in_front_button_wears_the_icon_of_the_state_it_is_in`; every
//!   other button the one its command is looked up to —
//!   `every_other_button_wears_the_icon_of_its_command`
//!
//! Closes #580.
//! - the three levels are offered while the part holds matter and not before
//!   — `the_three_stl_levels_are_offered_only_while_the_part_holds_matter`
//! - they are one menu, « Exporter en STL… », under a tab « Export » of its
//!   own rather than inside « Extrusion » —
//!   `the_three_stl_levels_are_one_menu_under_an_export_tab_of_their_own`
//! - to try it: the file opens in a slicer, PrusaSlicer or Cura, at the right
//!   size — no test: a human opens the file in a slicer, which nothing in this
//!   workspace runs
//!
//! Closes #581.
//! - to try it: each part opens in FreeCAD — the command « Exporter en STEP… »
//!   is offered while the part holds matter and not before —
//!   `the_step_export_is_offered_only_while_the_part_holds_matter` — under the
//!   tab « Export », beside the STL menu —
//!   `the_step_export_sits_under_the_export_tab_and_nowhere_else`
//!
//! Closes #193.
//! - « Vue en coupe » is offered while the part holds matter —
//!   `the_section_view_is_offered_while_the_part_holds_matter`; decided in
//!   review, not while a sketch is edited, whose own cut wins —
//!   `the_section_view_is_not_offered_while_a_sketch_is_edited`
//! - found in review: the button shows pressed while the section is on, the
//!   ribbon learning it from its context —
//!   `the_section_view_button_shows_pressed_while_the_section_is_on`
//!
//! Closes #728 — the whole transcription is in `crates/app/tests/view_bar.rs`;
//! this is the bullet the rule of what is offered answers, which a shortcut is
//! filtered through as well as a button.
//! - « Recadrer » is offered in and out of a sketch —
//!   `reframe_is_offered_in_and_out_of_a_sketch`
//!
//! Closes #729. The rest of the issue is in the test files of the viewport,
//! each under its own block.
//! - found in review: « Vue d'origine » is hidden while a sketch is open, and a
//!   shortcut or a ribbon button goes no further than the button does — it is
//!   greyed exactly while a sketch is open —
//!   `the_origin_view_is_offered_outside_a_sketch_and_while_a_plane_is_chosen`
//!
//! Closes #731. The drawing of a split button is held in
//! `crates/app/tests/ribbon_groups.rs`; what it runs, here.
//! - a command immediately followed by a menu runs that command —
//!   `a_split_with_a_command_before_it_runs_that_command`
//! - accepted: a toggle never pairs with the menu after it, so « Contraintes »
//!   stands alone though it follows « Construction » —
//!   `a_toggle_never_pairs_with_the_menu_after_it`
//! - and choosing a variant from its menu does not change what its icon runs —
//!   `a_variant_chosen_from_a_menu_leaves_a_split_with_a_command_of_its_own_alone`
//! - decided: the icon of « Exporter en STL… » runs the normal level, whichever
//!   was last chosen — decided by what the group holds, not by what it is
//!   called — `the_stl_split_runs_the_normal_level_whichever_level_was_last_chosen`
//! - found in review: a group holding no normal level, even named `stl`, never
//!   runs what its menu does not hold —
//!   `a_group_without_the_normal_level_never_runs_it_whatever_it_is_called`
//! - the pairing of a command with the menu after it needs no window: a command
//!   then a menu pair — `a_command_followed_by_a_menu_is_one_split`
//! - a toggle does not — `a_toggle_followed_by_a_menu_is_not_one_split`
//! - a group with no command before it stands alone —
//!   `a_menu_with_no_command_before_it_stands_alone`
//! - a command at the end stands alone —
//!   `a_command_at_the_end_of_a_run_stands_alone`
//! - decided: the icon of « Contraintes » runs the first rule until one was
//!   used from that menu —
//!   `until_a_rule_was_used_the_constraints_split_runs_the_first`
//! - and then the last one used, for the session —
//!   `the_constraints_split_runs_the_last_rule_used_from_its_menu`
//! - the chevron is pressable while any entry is —
//!   `a_chevron_is_offered_while_any_entry_can_be_carried_out`
//!
//! Closes #736 — the whole transcription is in `crates/app/tests/the_alerts.rs`;
//! this bullet is the one the ribbon's presenter holds.
//! - found in review: a step of matter reopened from its alert keeps its
//!   shape, an extrusion staying one and a revolution one; « Extrusion
//!   droite » and « Révolution » are offered on a panel armed afresh and not
//!   on one reopened — `the_shape_of_a_reopened_step_is_not_offered_for_change`

use super::*;
use cao_sketch::WorkPlane;
use chrono::Utc;

fn a_part() -> PartDocument {
    PartDocument::new("part", Utc::now())
}

fn drawing_on_a_sketch() -> SketchEditor {
    let mut editor = SketchEditor::default();
    editor.begin_editing(0, WorkPlane::XY);
    editor
}

#[test]
fn a_drawing_tool_is_offered_only_while_a_sketch_is_open() {
    let part = a_part();
    let extrusion = ExtrusionState::default();

    assert!(!is_enabled(
        Command::ToolLine,
        &part,
        &SketchEditor::default(),
        &extrusion
    ));
    assert!(is_enabled(
        Command::ToolLine,
        &part,
        &drawing_on_a_sketch(),
        &extrusion
    ));
}

#[test]
fn a_fresh_part_has_nothing_to_undo_and_nothing_to_redo() {
    let part = a_part();
    let editor = SketchEditor::default();
    let extrusion = ExtrusionState::default();

    assert!(!is_enabled(Command::Undo, &part, &editor, &extrusion));
    assert!(!is_enabled(Command::Redo, &part, &editor, &extrusion));
    assert!(!is_enabled(
        Command::CompactHistory,
        &part,
        &editor,
        &extrusion
    ));
}

#[test]
fn an_extrusion_is_applied_only_once_it_holds_every_value_it_needs() {
    let part = a_part();
    let editor = SketchEditor::default();
    let ready = ExtrusionState::default();

    assert_eq!(
        is_enabled(Command::ExtrusionApply, &part, &editor, &ready),
        ready.is_ready(part.variables()),
    );
}

#[test]
fn the_tool_in_hand_is_the_one_shown_as_pressed() {
    let part = a_part();
    let editor = drawing_on_a_sketch();
    let extrusion = ExtrusionState::default();
    let settings = Settings::default();
    let lang = Catalogue::french();
    let state = Context {
        settings: &settings,
        document: &part,
        editor: &editor,
        extrusion: &extrusion,
        lang: &lang,
        section_on: false,
    };

    assert!(active(Command::ToolLine, &state));
    assert!(!active(Command::ToolCircle, &state));
}

#[test]
fn the_matter_in_front_button_is_offered_only_while_a_sketch_is_open() {
    let part = a_part();
    let extrusion = ExtrusionState::default();
    let mut closed = drawing_on_a_sketch();
    closed.close();

    assert!(!is_enabled(
        Command::NextMatterInFront,
        &part,
        &SketchEditor::default(),
        &extrusion
    ));
    assert!(is_enabled(
        Command::NextMatterInFront,
        &part,
        &drawing_on_a_sketch(),
        &extrusion
    ));
    assert!(
        !is_enabled(Command::NextMatterInFront, &part, &closed, &extrusion),
        "a closed sketch has no matter in front of it to show",
    );
}

#[test]
fn reframe_is_offered_in_and_out_of_a_sketch() {
    let part = a_part();
    let extrusion = ExtrusionState::default();
    let mut finished = drawing_on_a_sketch();
    finished.close();

    for (editor, when) in [
        (SketchEditor::default(), "before a sketch is opened"),
        (drawing_on_a_sketch(), "while a sketch is open"),
        (finished, "once the sketch is finished"),
    ] {
        assert!(
            is_enabled(Command::Reframe, &part, &editor, &extrusion),
            "« Recadrer » is greyed {when}",
        );
    }
}

#[test]
fn the_origin_view_is_offered_outside_a_sketch_and_while_a_plane_is_chosen() {
    let part = a_part();
    let extrusion = ExtrusionState::default();
    let lang = Catalogue::french();
    let mut finished = drawing_on_a_sketch();
    finished.close();
    let mut choosing_a_plane = drawing_on_a_sketch();
    choosing_a_plane.start_choosing_plane(&lang);

    for (editor, offered, when) in [
        (SketchEditor::default(), true, "before a sketch is opened"),
        (drawing_on_a_sketch(), false, "while a sketch is open"),
        (choosing_a_plane, true, "while a plane is being chosen"),
        (finished, true, "once the sketch is finished"),
    ] {
        assert_eq!(
            is_enabled(Command::OriginView, &part, &editor, &extrusion),
            offered,
            "« Vue d'origine » {when}: a shortcut and a ribbon button go through this, and \
             the button under the cube is shown exactly when it answers true",
        );
    }
}

#[test]
fn the_matter_in_front_button_names_the_state_it_is_in() {
    let part = a_part();
    let extrusion = ExtrusionState::default();
    let settings = Settings::default();
    let lang = Catalogue::french();
    let mut editor = drawing_on_a_sketch();

    let mut faces = Vec::new();
    for _ in 0..4 {
        let state = Context {
            settings: &settings,
            document: &part,
            editor: &editor,
            extrusion: &extrusion,
            lang: &lang,
            section_on: false,
        };
        faces.push(label(Command::NextMatterInFront, &state));
        editor.next_matter_in_front();
    }

    assert!(
        faces.iter().all(|face| face.contains("Matière devant")),
        "the button is not the one a hand looks for: {faces:?}",
    );
    assert!(
        faces[0] != faces[1] && faces[1] != faces[2] && faces[0] != faces[2],
        "two of the three states read alike: {faces:?}",
    );
    assert_eq!(
        faces[3], faces[0],
        "a third click is back on Hidden, and the button says something else",
    );
}

fn faces_of_the_three_states<Face>(face: fn(Command, &Context<'_>) -> Face) -> Vec<Face> {
    let part = a_part();
    let extrusion = ExtrusionState::default();
    let mut settings = Settings::default();
    settings.toolbar.show_labels = false;
    let lang = Catalogue::french();
    let mut editor = drawing_on_a_sketch();

    let mut faces = Vec::new();
    for _ in 0..3 {
        let state = Context {
            settings: &settings,
            document: &part,
            editor: &editor,
            extrusion: &extrusion,
            lang: &lang,
            section_on: false,
        };
        faces.push(face(Command::NextMatterInFront, &state));
        editor.next_matter_in_front();
    }
    faces
}

#[test]
fn the_matter_in_front_button_wears_the_icon_of_the_state_it_is_in() {
    let worn = faces_of_the_three_states(|command, state| icon(command, state).id().to_owned());

    assert_eq!(
        worn,
        [
            "matter-in-front-hidden",
            "matter-in-front-see-through",
            "matter-in-front-opaque",
        ],
        "hidden, see-through then opaque, the button does not wear the icon of the state it is in",
    );
}

#[test]
fn every_other_button_wears_the_icon_of_its_command() {
    let part = a_part();
    let extrusion = ExtrusionState::default();
    let mut settings = Settings::default();
    settings.toolbar.show_labels = false;
    let lang = Catalogue::french();
    let editor = drawing_on_a_sketch();
    let state = Context {
        settings: &settings,
        document: &part,
        editor: &editor,
        extrusion: &extrusion,
        lang: &lang,
        section_on: false,
    };

    for command in Command::ALL
        .into_iter()
        .filter(|command| *command != Command::NextMatterInFront)
    {
        assert_eq!(
            icon(command, &state),
            super::super::icons::of(command),
            "{command:?} wears another icon on its button than the one it is looked up to",
        );
    }
}

const STL_LEVELS: [Command; 3] = [
    Command::ExportStlCoarse,
    Command::ExportStlNormal,
    Command::ExportStlFine,
];

fn reached(items: &[cao_prefs::Item]) -> Vec<Command> {
    items
        .iter()
        .flat_map(|item| match item {
            cao_prefs::Item::Command(command) => vec![*command],
            cao_prefs::Item::Group { items, .. } => reached(items),
            cao_prefs::Item::Separator => Vec::new(),
        })
        .collect()
}

fn holds_the_levels_together(items: &[cao_prefs::Item]) -> bool {
    items.iter().any(|item| match item {
        cao_prefs::Item::Group { items, .. } => {
            STL_LEVELS
                .iter()
                .all(|level| items.contains(&cao_prefs::Item::Command(*level)))
                || holds_the_levels_together(items)
        }
        _ => false,
    })
}

fn a_raised_block() -> PartDocument {
    use cao_part::{ExtrusionMode, Operation, PointRef};
    use glam::DVec2;

    let mut document = a_part();
    document.apply(Operation::CreateSketch {
        plane: WorkPlane::XY,
        on: None,
    });
    document.apply(Operation::AddRectangle {
        sketch: 0,
        corner: PointRef::New(DVec2::ZERO),
        opposite: PointRef::New(DVec2::new(10.0, 20.0)),
        construction: false,
    });
    document.apply(Operation::Extrude {
        sketch: 0,
        areas: document.areas_at(0, &[DVec2::new(5.0, 10.0)]),
        distance: 4.0.into(),
        mode: ExtrusionMode::Add,
    });
    assert!(!document.body().is_empty(), "the block is raised");
    document
}

#[test]
fn the_three_stl_levels_are_offered_only_while_the_part_holds_matter() {
    let editor = SketchEditor::default();
    let extrusion = ExtrusionState::default();
    let (empty, raised) = (a_part(), a_raised_block());

    for level in STL_LEVELS {
        assert!(
            !is_enabled(level, &empty, &editor, &extrusion),
            "{level:?} is offered on a part with no matter"
        );
        assert!(
            is_enabled(level, &raised, &editor, &extrusion),
            "{level:?} is not offered on a part holding a block"
        );
        assert!(
            is_enabled(level, &raised, &drawing_on_a_sketch(), &extrusion),
            "{level:?} is not offered while a sketch is open on a part holding a block"
        );
    }
}

fn tab<'a>(items: &'a [cao_prefs::Item], name: &str) -> Option<&'a [cao_prefs::Item]> {
    items.iter().find_map(|item| match item {
        cao_prefs::Item::Group {
            name: named, items, ..
        } if named == name => Some(items.as_slice()),
        _ => None,
    })
}

#[test]
fn the_three_stl_levels_are_one_menu_under_an_export_tab_of_their_own() {
    let toolbar = Settings::default().toolbar;

    let export = tab(&toolbar.items, "export")
        .unwrap_or_else(|| panic!("the ribbon has no tab « export »: {:?}", toolbar.items));
    assert!(
        holds_the_levels_together(export),
        "the tab « export » holds the three levels as one menu: {export:?}",
    );
    let extrusion = tab(&toolbar.items, "extrusion").expect("the ribbon has its tab « extrusion »");
    let strays: Vec<Command> = reached(extrusion)
        .into_iter()
        .filter(|command| STL_LEVELS.contains(command))
        .collect();
    assert!(
        strays.is_empty(),
        "the tab « extrusion » still holds {strays:?}"
    );
}

#[test]
fn the_step_export_is_offered_only_while_the_part_holds_matter() {
    let editor = SketchEditor::default();
    let extrusion = ExtrusionState::default();

    assert!(
        !is_enabled(Command::ExportStep, &a_part(), &editor, &extrusion),
        "the STEP export is offered on a part with no matter"
    );
    assert!(
        is_enabled(Command::ExportStep, &a_raised_block(), &editor, &extrusion),
        "the STEP export is not offered on a part holding a block"
    );
    assert!(
        is_enabled(
            Command::ExportStep,
            &a_raised_block(),
            &drawing_on_a_sketch(),
            &extrusion
        ),
        "the STEP export is not offered while a sketch is open on a part holding a block"
    );
}

#[test]
fn the_step_export_sits_under_the_export_tab_and_nowhere_else() {
    let toolbar = Settings::default().toolbar;

    let export = tab(&toolbar.items, "export")
        .unwrap_or_else(|| panic!("the ribbon has no tab « export »: {:?}", toolbar.items));
    assert!(
        reached(export).contains(&Command::ExportStep),
        "the tab « export » offers no STEP export: {export:?}",
    );
    assert!(
        holds_the_levels_together(export),
        "the STL menu is still whole beside it: {export:?}",
    );
    let elsewhere = toolbar
        .items
        .iter()
        .filter(|item| !matches!(item, cao_prefs::Item::Group { name, .. } if name == "export"))
        .flat_map(|item| reached(std::slice::from_ref(item)))
        .filter(|command| *command == Command::ExportStep)
        .count();
    assert_eq!(elsewhere, 0, "the STEP export is offered outside its tab");
}

#[test]
fn the_section_view_is_offered_while_the_part_holds_matter() {
    let editor = SketchEditor::default();
    let extrusion = ExtrusionState::default();

    assert!(
        !is_enabled(Command::SectionView, &a_part(), &editor, &extrusion),
        "the section view is offered on a part with no matter to cut",
    );
    assert!(
        is_enabled(Command::SectionView, &a_raised_block(), &editor, &extrusion),
        "the section view is not offered on a part holding a block",
    );
    assert_eq!(
        wording::label(&Catalogue::french(), Command::SectionView),
        "Vue en coupe",
    );
}

#[test]
fn the_section_view_is_not_offered_while_a_sketch_is_edited() {
    assert!(
        !is_enabled(
            Command::SectionView,
            &a_raised_block(),
            &drawing_on_a_sketch(),
            &ExtrusionState::default(),
        ),
        "« Vue en coupe » is offered over a sketch, whose own cut already wins",
    );
}

#[test]
fn the_section_view_button_shows_pressed_while_the_section_is_on() {
    let settings = Settings::default();
    let (part, editor, extrusion, lang) = (
        a_raised_block(),
        SketchEditor::default(),
        ExtrusionState::default(),
        Catalogue::french(),
    );
    let pressed = |section_on| {
        active(
            Command::SectionView,
            &Context {
                settings: &settings,
                document: &part,
                editor: &editor,
                extrusion: &extrusion,
                lang: &lang,
                section_on,
            },
        )
    };

    assert!(
        pressed(true),
        "« Vue en coupe » does not show pressed while the section is on"
    );
    assert!(
        !pressed(false),
        "« Vue en coupe » shows pressed with no section on"
    );
}

#[test]
fn select_all_is_offered_only_in_a_sketch_with_the_selection_tool() {
    let part = a_part();
    let extrusion = ExtrusionState::default();
    let mut selecting = drawing_on_a_sketch();
    selecting.tool = Tool::Select;
    let mut lining = drawing_on_a_sketch();
    lining.tool = Tool::Line;

    assert!(
        is_enabled(Command::SelectAll, &part, &selecting, &extrusion),
        "in a sketch with the selection tool",
    );
    assert!(
        !is_enabled(Command::SelectAll, &part, &lining, &extrusion),
        "with the line tool in hand",
    );
    assert!(
        !is_enabled(
            Command::SelectAll,
            &part,
            &SketchEditor::default(),
            &extrusion
        ),
        "with no sketch open",
    );
}

const THE_RULES: [Command; 3] = [
    Command::RulePerpendicular,
    Command::RuleParallel,
    Command::RuleEqual,
];

#[test]
fn a_split_with_a_command_before_it_runs_that_command() {
    let ribbon = Ribbon::new();

    assert_eq!(
        ribbon.primary_of(
            "circles",
            Some(Command::ToolCircle),
            &[Command::CircleCenter, Command::CircleTwoPoints],
        ),
        Some(Command::ToolCircle),
    );
}

#[test]
fn a_variant_chosen_from_a_menu_leaves_a_split_with_a_command_of_its_own_alone() {
    let mut ribbon = Ribbon::new();
    ribbon.used_from_menu("circles", Command::CircleTwoPoints);

    assert_eq!(
        ribbon.primary_of(
            "circles",
            Some(Command::ToolCircle),
            &[Command::CircleCenter, Command::CircleTwoPoints],
        ),
        Some(Command::ToolCircle),
    );
}

#[test]
fn the_stl_split_runs_the_normal_level_whichever_level_was_last_chosen() {
    for called in ["stl", "levels"] {
        let mut ribbon = Ribbon::new();
        assert_eq!(
            ribbon.primary_of(called, None, &STL_LEVELS),
            Some(Command::ExportStlNormal),
        );

        ribbon.used_from_menu(called, Command::ExportStlFine);

        assert_eq!(
            ribbon.primary_of(called, None, &STL_LEVELS),
            Some(Command::ExportStlNormal),
        );
    }
}

#[test]
fn a_group_without_the_normal_level_never_runs_it_whatever_it_is_called() {
    let levels = [Command::ExportStlCoarse, Command::ExportStlFine];

    for called in ["stl", "levels"] {
        assert_eq!(
            Ribbon::new().primary_of(called, None, &levels),
            Some(Command::ExportStlCoarse),
            "the group « {called} » runs what its menu holds, its first entry",
        );
    }
}

const WITHIN_A_GROUP: usize = 2;

fn what_is_paired(items: &[cao_prefs::Item]) -> Vec<String> {
    runs(items, WITHIN_A_GROUP)
        .iter()
        .map(|run| match run {
            Run::Plain(_) => "plain".to_owned(),
            Run::Split(split) => format!("split {} after {:?}", split.group, split.before),
        })
        .collect()
}

fn a_menu_of_circles() -> cao_prefs::Item {
    cao_prefs::Item::group(
        "circles",
        vec![cao_prefs::Item::Command(Command::CircleCenter)],
    )
}

#[test]
fn a_command_followed_by_a_menu_is_one_split() {
    assert_eq!(
        what_is_paired(&[
            cao_prefs::Item::Command(Command::ToolCircle),
            a_menu_of_circles()
        ]),
        ["split circles after Some(ToolCircle)"],
    );
}

#[test]
fn a_toggle_followed_by_a_menu_is_not_one_split() {
    assert_eq!(
        what_is_paired(&[
            cao_prefs::Item::Command(Command::ToggleConstruction),
            a_menu_of_circles(),
        ]),
        ["plain", "split circles after None"],
    );
}

#[test]
fn a_menu_with_no_command_before_it_stands_alone() {
    assert_eq!(
        what_is_paired(&[a_menu_of_circles()]),
        ["split circles after None"],
    );
}

#[test]
fn a_command_at_the_end_of_a_run_stands_alone() {
    assert_eq!(
        what_is_paired(&[
            a_menu_of_circles(),
            cao_prefs::Item::Command(Command::ToolCircle),
        ]),
        ["split circles after None", "plain"],
    );
}

#[test]
fn until_a_rule_was_used_the_constraints_split_runs_the_first() {
    assert_eq!(
        Ribbon::new().primary_of("constraints", None, &THE_RULES),
        Some(Command::RulePerpendicular),
    );
}

#[test]
fn the_constraints_split_runs_the_last_rule_used_from_its_menu() {
    let mut ribbon = Ribbon::new();

    ribbon.used_from_menu("constraints", Command::RuleParallel);
    assert_eq!(
        ribbon.primary_of("constraints", None, &THE_RULES),
        Some(Command::RuleParallel),
    );

    ribbon.used_from_menu("constraints", Command::RuleEqual);
    assert_eq!(
        ribbon.primary_of("constraints", None, &THE_RULES),
        Some(Command::RuleEqual),
    );
}

#[test]
fn a_chevron_is_offered_while_any_entry_can_be_carried_out() {
    let (settings, part, drawing, extrusion, lang) = (
        Settings::default(),
        a_part(),
        drawing_on_a_sketch(),
        ExtrusionState::default(),
        Catalogue::french(),
    );
    let state = Context {
        settings: &settings,
        document: &part,
        editor: &drawing,
        extrusion: &extrusion,
        lang: &lang,
        section_on: false,
    };

    assert!(
        any_entry_enabled(&[Command::Undo, Command::CircleCenter], &state),
        "« Annuler » is greyed out but a circle can be drawn, so the menu opens",
    );
    assert!(
        !any_entry_enabled(&[Command::Undo, Command::Redo], &state),
        "a fresh part can neither undo nor redo, so nothing in that menu can be run",
    );
}

#[test]
fn a_toggle_never_pairs_with_the_menu_after_it() {
    assert!(
        !takes_a_menu(Command::ToggleConstruction),
        "« Construction » stands before « Contraintes » without being one of its rules",
    );
    assert!(
        takes_a_menu(Command::ToolCircle),
        "« Cercle » is a tool with variants, and pairs with its menu",
    );
}

#[test]
fn the_shape_of_a_reopened_step_is_not_offered_for_change() {
    let part = a_raised_block();
    let editor = SketchEditor::default();
    let mut afresh = ExtrusionState::default();
    afresh.offer(0);
    afresh.arm(cao_part::ExtrusionMode::Add);
    let reopened = ExtrusionState::reopened(&part, 2).expect("the block's extrusion reopens");
    let offered = |extrusion: &ExtrusionState| {
        [Command::ExtrusionStraight, Command::ExtrusionRevolution]
            .map(|shape| is_enabled(shape, &part, &editor, extrusion))
    };

    assert_eq!(
        [offered(&afresh), offered(&reopened)],
        [[true, true], [false, false]],
        "« Extrusion droite » and « Révolution » offered, on a panel armed afresh then on one \
         reopened on the block's extrusion",
    );
}
