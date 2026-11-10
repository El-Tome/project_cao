//! What app · screens/viewport/render.rs is held to.
//!
//! Closes #319.
//! - See-through: the matter in front of the plane is drawn translucent, and
//!   the sketch shows through it —
//!   `see_through_draws_the_matter_in_front_translucent`
//! - what is drawn see-through is only what stands in front, and only the side
//!   of it turned to the eye: the surface pass culls nothing, and back faces
//!   stacked over front ones read blotchy — found in review, not in the issue —
//!   `the_see_through_layer_carries_nothing_lying_on_the_sketch_plane`,
//!   `the_see_through_layer_draws_no_face_turned_away_from_the_eye`
//!
//! Closes #193. The issue carries no "Done when": what follows is read from
//! "What the user does", "The section is a surface", "How it is verified",
//! "Left open" and the comment of 2026-11-05, reworded into criteria.
//! - a « Vue en coupe » button in the ribbon, offered while the part holds
//!   matter — `the_section_view_is_offered_while_the_part_holds_matter`, in
//!   `crates/app/src/screens/ribbon/state/tests.rs`; it opens a section, on XY
//!   through the middle of the part, and clicked again turns it off —
//!   `the_section_view_opens_a_section_through_the_middle_of_the_part_and_closes_it`,
//!   in `crates/app/src/commands/tests.rs`; it opens a floating panel, and
//!   closing the panel, by the button again or by Échap, turns the view off —
//!   `the_section_view_button_opens_its_panel_and_closing_it_turns_the_view_off`,
//!   in `crates/app/tests/section_view.rs`; it starts off when the part is
//!   reopened — no test: the section lives in `ViewportState`, which opening a
//!   part builds afresh, and nothing of it reaches the file, as the criterion
//!   on the document holds
//! - the cutting plane is one of the three origin planes —
//!   `the_section_cuts_along_whichever_origin_plane_is_chosen` — or a flat face
//!   of the part clicked once the panel waits for one —
//!   `the_panel_waits_for_a_face_until_one_is_clicked_or_a_plane_chosen`, in
//!   `crates/app/src/screens/viewport/section/tests.rs`;
//!   `a_flat_face_clicked_becomes_the_cutting_plane`,
//!   `a_curved_face_clicked_is_no_cutting_plane`, in
//!   `crates/app/src/screens/viewport/input/planes/tests.rs`
//! - the distance is typed in the panel, from the chosen plane or face, signed
//!   along its normal — `the_distance_moves_the_cut_along_the_normal_of_the_plane`,
//!   `a_distance_from_a_face_is_signed_along_its_outward_normal`; a handle on
//!   the plane is pulled in the view, where a pointer ray lands along the
//!   normal — `a_ray_across_the_normal_pulls_the_handle_to_the_closest_point`,
//!   `a_ray_along_the_normal_pulls_the_handle_nowhere`,
//!   `a_ray_far_from_the_handle_takes_no_hold_of_it`; each follows the other —
//!   `the_handle_stands_where_the_distance_typed_puts_the_plane`,
//!   `pulling_the_handle_changes_the_distance_the_panel_shows`, all five in
//!   `crates/app/src/screens/viewport/section/tests.rs`
//! - one state per side, behind and in front of the plane: hidden, see-through
//!   or opaque; by default one side hidden, the other opaque; « Inverser »
//!   swaps them — read as: the sides belong to the plane, behind against its
//!   normal and in front along it, not to the eye —
//!   `a_section_draws_what_lies_behind_opaque_and_hides_what_stands_in_front`,
//!   `each_side_of_the_section_is_hidden_see_through_or_opaque_on_its_own`,
//!   `inverting_the_section_swaps_the_states_of_its_two_sides`, and a click on
//!   a side's row steps it round the three —
//!   `each_side_steps_round_its_three_states_on_its_own`, in
//!   `crates/app/src/screens/viewport/section/tests.rs`; the cut is computed
//!   once per plane, distance and part, never every frame, whatever the states
//!   or the eye — `the_section_is_cut_once_per_plane_distance_and_part`
//! - the section is drawn filled, the exact cut of #578 —
//!   `the_section_is_drawn_filled_on_the_cutting_plane`; it can be picked to
//!   start a sketch on, and the sketch lands on the cutting plane, a plane and
//!   not a face of the part —
//!   `a_click_on_the_section_starts_a_sketch_on_the_cutting_plane`, in
//!   `crates/app/src/screens/viewport/input/planes/tests.rs`, and
//!   `a_sketch_on_a_cutting_plane_is_not_called_a_face_of_the_part`, in
//!   `crates/app/src/wording/history/tests.rs`; starting a sketch turns the
//!   section view off, the sketch's own cut of #319 taking over — decided by
//!   the orchestrator, not the issue — the same test and
//!   `starting_a_sketch_on_a_plane_of_the_origin_turns_the_section_view_off`,
//!   beside it, and `starting_a_sketch_turns_the_section_view_off`, in
//!   `crates/app/tests/section_view.rs`
//! - the section view never touches the document: nothing enters the history,
//!   the `.caopart` is unchanged — no test: in this file; held in
//!   `crates/app/src/commands/tests.rs`, under its own block for this issue,
//!   since only a command reaches the document
//! - "slicing a cube at mid-height gives the expected cross-section, in
//!   `cao_solid`, with no window" — no test: in this file; held in
//!   `crates/solid/tests/the_section_of_an_exact_part_is_filled.rs`, under its
//!   own block for this issue
//!
//! Closes #348. The issue carries no "Done when": what follows is what was
//! left by the comment of 2026-09-17, read with the comment of 2026-11-05, the
//! issue's own sentences kept word for word between quotes.
//! - "Once a sketch has been used — extruded, revolved — its traits stop being
//!   drawn in the viewport, automatically" —
//!   `a_sketch_once_extruded_draws_neither_its_traits_nor_its_areas`,
//!   `a_sketch_once_revolved_draws_neither_its_traits_nor_its_areas`
//! - "An eye in the tree brings them back" —
//!   `the_eye_of_a_used_sketch_brings_its_traits_back_and_takes_them_away_again`;
//!   the eye itself, a widget of `part_tree/view.rs` answering
//!   `TreeAction::ToggleEye`, is reached by no window here
//! - "A sketch never used keeps its traits on screen, with the eye lit" —
//!   `a_sketch_never_used_keeps_its_traits_on_screen_with_its_eye_lit`
//! - reopening a sketch shows it, whatever its eye — decided on the branch —
//!   `a_used_sketch_reopened_is_drawn_whatever_its_eye`
//! - found on the branch: an eye switched by hand belongs to its sketch, and a
//!   sketch drawn anew where a rewind took one away does not inherit it —
//!   `a_sketch_drawn_anew_where_one_was_rewound_away_does_not_inherit_its_eye`
//! - the eye is view state, never the document's — no test: `toggle_eye` reads
//!   the document by shared reference, and `ViewportState` is built afresh on
//!   opening a part and written nowhere, as the section view's is
//! - "A click on an element selects it — in the viewport, as if it had been
//!   clicked there": in the sketch being edited it selects an element, a
//!   dimension or a rule, alone or, the modifier held, added — no test: in
//!   this file; held in `crates/app/src/screens/part_tree/state/tests.rs`,
//!   under its own block for this issue, since what a click on a row means is
//!   the tree's presenter's
//! - in a sketch not open, it lights the element or the area in the viewport,
//!   and opens nothing —
//!   `an_element_lit_from_the_tree_is_drawn_lit_in_its_closed_sketch`,
//!   `an_area_lit_from_the_tree_is_drawn_lit_in_its_closed_sketch`; that the
//!   click lights rather than opens is held beside the selecting, in the
//!   tree's tests
//! - the double-click still reopens, and a single click on a step still folds
//!   it — no test: both are the `CollapsingHeader` of `part_tree/view.rs`,
//!   untouched, and no driver opens the tree
//! - "Selecting from the viewport, which must agree with selecting from the
//!   tree — one selection, two ways in" — no test: in this file; held in
//!   `crates/app/src/screens/part_tree/state/tests.rs`, under its own block
//! - one display mode for the whole body — invisible, see-through, opaque —
//!   "opaque by default" — `the_body_is_drawn_opaque_by_default`,
//!   `the_body_is_drawn_invisible_see_through_or_opaque_as_its_mode_says`,
//!   set since #734 from « TOUT LE CORPS ». The « Corps » rubric whose click
//!   stepped round the three is gone with #734: each volume now steps round
//!   them on its own, held in `crates/app/src/screens/viewport/visibility/tests.rs`
//!   by `a_volume_s_visibility_cycles_opaque_see_through_invisible_and_round_again`
//! - the sketch's own cut of #319 and the section of #193 still apply on top —
//!   `a_see_through_body_keeps_the_matter_in_front_of_the_sketch_cut_away`,
//!   `a_see_through_body_keeps_the_side_the_section_hides_hidden`; found in
//!   review: whatever each side of the section is set to, a see-through body
//!   shows see-through what the section shows and nothing it hides —
//!   `a_see_through_body_shows_what_the_section_shows_and_nothing_it_hides`;
//!   an invisible body draws nothing, whatever the section or the sketch's cut
//!   — `an_invisible_body_draws_nothing_whatever_the_section_or_the_sketch_shows`
//! - found in review: what is offered to draw on or to cut along is what is
//!   drawn — a face of a see-through body is offered, nothing of an invisible
//!   one — no test: in this file; held in
//!   `crates/app/src/screens/viewport/input/planes/tests.rs`, under its own
//!   block for this issue
//! - found in review: the light goes out on undo, redo and compaction, and a
//!   compaction puts every eye back to its default — no test: in this file;
//!   held in `crates/app/src/commands/tests.rs`, under its own block for this
//!   issue; that it goes out when the tree is closed — no test: the tree is
//!   closed by a `toggle_value` in `ribbon/view.rs`, which no presenter sees
//! - decided in review: an area row of the open sketch lights the area —
//!   `an_area_of_the_open_sketch_lit_from_the_tree_is_drawn_lit`; a dimension
//!   or a rule row of a closed sketch is no click at all, and a click while a
//!   tool is part-way through something does nothing — held in the tree's
//!   tests
//!
//! Closes #693.
//! - With the section tool open, a square and an arrow are drawn on the
//!   cutting plane, the arrow along its normal; « Inverser » turns the arrow
//!   round —
//!   `with_the_section_tool_open_a_square_and_an_arrow_stand_on_the_cutting_plane`,
//!   `inverser_turns_the_arrow_round`; the arrow only shows: pressed and
//!   dragged, it moves nothing —
//!   `pressing_on_the_arrow_s_tip_changes_neither_the_distance_nor_the_direction`
//! - The square keeps its size on screen when the view is zoomed in and out —
//!   `the_square_keeps_its_size_on_screen_when_the_view_is_zoomed_in_and_out`
//! - Typing an angle tilts the cut, the square and the arrow together, about
//!   the square's centre; 0 and 0 gives today's cut exactly —
//!   `typing_an_angle_tilts_the_cut_the_square_and_the_arrow_together_about_the_square_s_centre`,
//!   `zero_and_zero_typed_back_give_today_s_cut_exactly`; the cut filled on
//!   the tilted plane — `the_filled_cut_lies_on_the_tilted_plane`; an angle
//!   typed as every value is, a formula or a variable —
//!   `a_formula_typed_in_an_angle_field_takes_its_value_as_the_distance_field_does`;
//!   read as: the fields
//!   « Inclinaison 1 » and « Inclinaison 2 » turn the plane about the first and
//!   the second of its own axes, and the centre is held where the section
//!   opens, through the middle of the part; decided on the branch: a section
//!   on a face turns about the point of the face under the middle of the part,
//!   where its square stood when the face was picked —
//!   `a_section_on_a_face_turns_about_the_point_of_the_face_its_square_stood_on`,
//!   and kept so through the `.caopart` —
//!   `a_kept_face_section_reopened_from_the_caopart_cuts_where_it_cut_when_kept`,
//!   and `a_kept_section_on_a_face_turns_about_the_point_of_the_face_under_the_middle_of_the_part`,
//!   in `crates/part/tests/a_section_keeps_its_tilt.rs`; moved onto another
//!   face and kept again, it is kept where it cut on screen —
//!   `a_kept_tilted_section_moved_onto_another_face_is_kept_where_it_cut_on_screen`
//! - The typed distance moves a tilted plane along its tilted normal —
//!   `the_typed_distance_moves_a_tilted_plane_along_its_tilted_normal`, and on
//!   a face from the point it turns about —
//!   `a_distance_typed_on_a_tilted_face_section_moves_it_that_far_along_its_tilted_normal_from_the_face`;
//!   the handle dragged does the same, as far as the part reaches along that
//!   normal —
//!   `dragging_the_handle_of_a_tilted_plane_moves_it_along_the_tilted_normal_as_far_as_the_part_reaches`
//! - A sketch started on a tilted cut lands on the tilted plane — no test: held
//!   by `a_sketch_started_on_a_tilted_cut_lands_on_the_tilted_plane`, in
//!   `crates/app/tests/tilting_a_section.rs`, and on a kept tilted section
//!   switched on by `a_sketch_on_a_kept_tilted_section_lands_on_the_tilted_plane`,
//!   beside it
//! - decided on the issue, 2026-11-06: the two angles are kept with a saved
//!   section, and a part saved before reads them as 0 and 0 — no test: held by
//!   `a_tilted_section_comes_back_with_its_two_angles_from_the_caopart_and_from_the_tree`,
//!   in `crates/app/tests/tilting_a_section.rs`, and by
//!   `a_tilted_section_comes_back_from_the_caopart_with_its_two_angles` and
//!   `a_section_saved_before_its_angles_existed_reads_both_as_zero_and_cuts_where_it_did`,
//!   in `crates/part/tests/a_section_keeps_its_tilt.rs`
//! - decided on the issue, revised: the point a kept section turns about is
//!   kept with it at « Enregistrer la coupe », on a face in the face's own
//!   terms, so matter raised elsewhere moves nothing and the face carries it —
//!   `a_kept_tilted_section_stays_where_it_was_kept_when_matter_is_raised_away_from_it`,
//!   `a_sketch_laid_on_a_kept_tilted_section_still_lies_on_its_cut_once_matter_is_raised_and_the_part_reopened`,
//!   `a_kept_tilted_section_on_a_face_follows_the_face_when_its_extrusion_grows_and_not_the_matter_raised_aside`
//!
//! Closes #724.
//! - the view's background: plain `vue_esquisse` — `background.sketch_view`
//!   since #749 put the tokens in English — in a sketch, a radial gradient in
//!   the free view, as the default of the existing background setting — the
//!   frame built while a sketch is edited is painted with the theme's sketch
//!   background, `a_sketch_being_edited_is_drawn_on_the_sketch_background`,
//!   and the free view with its own,
//!   `the_free_view_is_drawn_on_the_view_background`; the defaults themselves
//!   are held in `crates/prefs/src/theme/tests.rs`
//! - found in review: the plain colour goes where the grid goes, with the view
//!   on a plane, and the gradient with the free view, a sketch open or not —
//!   `a_plane_view_with_no_sketch_open_is_drawn_on_the_sketch_background`,
//!   and the free view with a sketch open in
//!   `the_free_view_is_drawn_on_the_view_background`
//!
//! Closes #729. The issue carries no "Done when": what follows is its "What it
//! does" and its "What must not break", word for word, each criterion in the
//! file that holds it, with what was decided where the issue left a choice.
//! - In a sketch, the face it looks at is lit amber (« DESSUS » for the XY
//!   plane) — `a_sketch_lights_amber_the_face_of_the_cube_it_looks_at`; the
//!   sketch's own plane says which, wherever the view is turned —
//!   `a_sketch_keeps_its_face_lit_with_the_view_turned_away_from_it`; and
//!   nothing is lit but in a sketch —
//!   `the_cube_is_lit_amber_only_while_a_sketch_is_open`
//! - A sketch on a part's face lights the cube face closest to its normal —
//!   `a_sketch_on_a_part_s_face_lights_the_cube_face_closest_to_its_normal`;
//!   decided on the branch: on an exact tie the first in the order of
//!   `CubeFace::ALL` wins, so that the result never flickers —
//!   `a_normal_exactly_between_two_faces_lights_the_first_in_their_fixed_order`;
//!   a hovered zone keeps its hover colour on top of the lit face —
//!   `a_hovered_zone_keeps_its_hover_colour_on_the_lit_face`
//! - Clicking a face, an edge or a corner turns the camera as today; hovering
//!   lights it: the hovering half — `hovering_a_zone_of_the_cube_still_lights_it`;
//!   the clicking half in `crates/app/tests/view_bar.rs`, under its own block
//!
//! Closes #733 — the whole transcription is in
//! `crates/app/src/screens/part_tree/state/tests.rs`; these are the bullets of
//! the hover that only a frame drawn shows.
//! - Hovering a row of a closed sketch lights its element in the view as its
//!   click does — `hovering_a_row_of_a_closed_sketch_lights_it_as_its_click_does`
//! - the hover light lasts only while the hand is on the row, and leaves a
//!   click's light as it found it —
//!   `a_row_lit_by_the_hand_goes_out_when_the_hand_leaves_it`
//! - found in review: with the hand on another row of the same closed sketch,
//!   the trait a click lit stays lit beside the one under the hand —
//!   `a_trait_a_click_lit_stays_lit_while_the_hand_is_on_another_row_of_its_sketch`
//!
//! Closes #734 — the whole transcription is in
//! `crates/app/src/screens/part_tree/state/tests.rs`; these bullets are the
//! ones only a frame shows.
//! - a volume's state applies to the faces of the part made by that step
//!   (`faces_made_by`); for a cut, the walls it dug. On a block with a hole
//!   cut through it, making the cut « transparente » makes the hole's walls
//!   see-through and leaves the block opaque —
//!   `a_see_through_cut_draws_the_walls_of_its_hole_see_through_and_leaves_the_block_opaque`;
//!   and an invisible volume's faces are not drawn at all, whichever of the
//!   two it is — `a_hidden_volume_draws_none_of_its_faces`
//! - a face made by no listed volume takes the last whole-body choice — no
//!   test: no part built today has a face that no extrusion or revolution
//!   made, so no frame can show one; the rule is held, face by face, in
//!   `crates/app/src/screens/viewport/visibility/tests.rs`
//! - found in review: a face of a volume the section parts in two, one piece
//!   numbered anew, is still drawn as its volume — every piece of it —
//!   `a_volume_the_section_parts_is_drawn_in_its_own_state_every_piece_of_it`
//! - a see-through wall behind an opaque face is hidden by it — no test: the
//!   depth test lives in the wgpu pipelines of `crates/render/src/renderer.rs`,
//!   which no test reaches; the frame only says which layer a triangle goes
//!   to, so this is checked on screen

use cao_part::{ExtrusionMode, Operation, PartDocument, PointRef};
use cao_prefs::config::ViewportConfig;
use cao_render::camera::{OrbitCamera, ViewTransition};
use cao_render::{CubeFace, CubeZone, MatterInFront};
use cao_sketch::{Element, WorkPlane};
use chrono::Utc;
use glam::{DVec2, DVec3, Vec3};
use std::collections::BTreeMap;

use super::*;
use crate::lang::Catalogue;
use crate::screens::extrusion::ExtrusionState;
use crate::screens::part_tree::state::{Points, hover};
use crate::screens::sketch::SketchEditor;
use crate::screens::viewport::matter::shown;
use crate::screens::viewport::section::Section;

const SCREEN: egui::Rect = egui::Rect {
    min: egui::Pos2::ZERO,
    max: egui::pos2(1280.0, 800.0),
};

/// One frame of an empty part, seen the way `state` says it is seen.
fn a_frame(prepare: impl FnOnce(&mut ViewportState)) -> SceneFrame {
    let mut state = ViewportState::default();
    prepare(&mut state);

    let mut document = PartDocument::new("part", Utc::now());
    let mut editor = SketchEditor::default();
    let mut extrusion = ExtrusionState::default();
    let lang = Catalogue::french();
    let millimetres = document.scale();
    let context = SketchContext {
        document: &mut document,
        editor: &mut editor,
        extrusion: &mut extrusion,
        lang: &lang,
    };
    let scale = ViewScale::of(
        &OrbitCamera::default(),
        SCREEN,
        1.0,
        &ViewportConfig::default(),
        millimetres,
    );

    build_frame(&state, SCREEN, SCREEN, scale, &context, None, None)
}

/// How many of a frame's world lines lie in `plane`. The grid is the only thing
/// drawn wholly inside a work plane, so this counts it and nothing else.
fn lines_in(frame: &SceneFrame, plane: WorkPlane) -> usize {
    let (origin, normal) = (plane.origin.as_vec3(), plane.normal().as_vec3());
    frame
        .scene_world_lines
        .iter()
        .filter(|vertex| (Vec3::from(vertex.position) - origin).dot(normal).abs() < 1e-4)
        .count()
}

#[test]
fn no_grid_is_drawn_until_the_view_has_landed_on_the_plane() {
    let landed = a_frame(|state| state.mode = ViewMode::Plane(WorkPlane::XY));
    let swinging = a_frame(|state| {
        state.mode = ViewMode::Plane(WorkPlane::XY);
        state.transition = Some(ViewTransition::to_angles(&OrbitCamera::default(), 0.5, 0.5));
    });

    assert!(
        lines_in(&landed, WorkPlane::XY) > 100,
        "a view sitting on a plane is drawn no grid on it",
    );
    assert!(
        lines_in(&swinging, WorkPlane::XY) < lines_in(&landed, WorkPlane::XY),
        "the grid is drawn while the view is still swinging onto the plane, where \
         it reads as a disc floating in the middle of the screen",
    );
}

#[test]
fn the_world_axes_stand_in_until_the_drawing_has_its_own() {
    let free = a_frame(|state| state.mode = ViewMode::Free);
    let landed = a_frame(|state| state.mode = ViewMode::Plane(WorkPlane::XY));

    assert!(
        !free.scene_world_lines.is_empty(),
        "a free view is drawn neither a grid nor an axis, so it shows nothing at all",
    );
    assert_eq!(
        lines_in(&free, WorkPlane::XY),
        4,
        "the world axes lying in XY are drawn as something other than two lines",
    );
    assert!(
        lines_in(&landed, WorkPlane::XY) > lines_in(&free, WorkPlane::XY),
        "landing on a plane adds nothing to what is drawn on it",
    );
}

#[test]
fn a_frame_carries_the_orientation_cube_wherever_the_view_is() {
    for frame in [
        a_frame(|state| state.mode = ViewMode::Free),
        a_frame(|state| state.mode = ViewMode::Plane(WorkPlane::XY)),
    ] {
        assert!(
            !frame.cube_triangles.is_empty() && !frame.cube_edges.is_empty(),
            "the cube is the one way back to a known view, and it is not drawn",
        );
        assert!(
            !frame.scene_background.is_empty(),
            "the frame carries no background at all",
        );
    }
}

/// Two backgrounds no default wears, so that a frame painted with either can
/// only have taken it from the theme it was handed.
fn a_theme_with_two_backgrounds() -> cao_prefs::Theme {
    cao_prefs::Theme {
        background: cao_prefs::theme::Background::Solid(FREE_VIEW_BACKGROUND),
        sketch_background: cao_prefs::theme::Background::Solid(SKETCH_BACKGROUND),
        ..cao_prefs::Theme::default()
    }
}

const FREE_VIEW_BACKGROUND: cao_prefs::theme::Rgba =
    cao_prefs::theme::Rgba::opaque(0.30, 0.10, 0.20);
const SKETCH_BACKGROUND: cao_prefs::theme::Rgba = cao_prefs::theme::Rgba::opaque(0.10, 0.30, 0.20);

fn background_colours(frame: &SceneFrame) -> Vec<[f32; 4]> {
    frame
        .scene_background
        .iter()
        .map(|vertex| vertex.color)
        .collect()
}

#[test]
fn a_sketch_being_edited_is_drawn_on_the_sketch_background() {
    let mut document = a_block();
    let mut state = ViewportState::default();
    state.mode = ViewMode::Plane(WorkPlane::XY);
    state.theme = a_theme_with_two_backgrounds();
    let mut editor = SketchEditor::default();
    editor.begin_editing(0, WorkPlane::XY);

    let colours = background_colours(&a_frame_of(&state, &mut document, &mut editor));

    assert!(
        !colours.is_empty()
            && colours
                .iter()
                .all(|colour| *colour == tint(SKETCH_BACKGROUND)),
        "a sketch being edited is painted {:?}, the sketch's background is {:?} and the free \
         view's {:?}",
        colours.first(),
        tint(SKETCH_BACKGROUND),
        tint(FREE_VIEW_BACKGROUND),
    );
}

/// A sketch open and the view turned away from its plane: the grid is gone,
/// and so is the plain colour it is drawn on.
#[test]
fn the_free_view_is_drawn_on_the_view_background() {
    let mut document = a_block();
    let mut state = ViewportState::default();
    state.mode = ViewMode::Free;
    state.theme = a_theme_with_two_backgrounds();
    let mut editor = SketchEditor::default();
    editor.begin_editing(0, WorkPlane::XY);

    let colours = background_colours(&a_frame_of(&state, &mut document, &mut editor));

    assert!(
        !colours.is_empty()
            && colours
                .iter()
                .all(|colour| *colour == tint(FREE_VIEW_BACKGROUND)),
        "the free view is painted {:?}, its background is {:?}",
        colours.first(),
        tint(FREE_VIEW_BACKGROUND),
    );
}

/// The view left on a plane once « Terminer » is pressed, or once a face of
/// the cube is clicked: the grid is still drawn, and it is drawn on the plain
/// colour it was made to be seen on.
#[test]
fn a_plane_view_with_no_sketch_open_is_drawn_on_the_sketch_background() {
    let colours = background_colours(&a_frame(|state| {
        state.mode = ViewMode::Plane(WorkPlane::XY);
        state.theme = a_theme_with_two_backgrounds();
    }));

    assert!(
        !colours.is_empty()
            && colours
                .iter()
                .all(|colour| *colour == tint(SKETCH_BACKGROUND)),
        "a view on a plane with no sketch open is painted {:?}, the sketch's \
         background is {:?}",
        colours.first(),
        tint(SKETCH_BACKGROUND),
    );
}

const HALFWAY: f32 = 2.0;
const ON_THE_TOP: f32 = 4.0;

/// A block 10 by 10 from 0 to 4 over XY, raised from a rectangle on XY.
fn a_block() -> PartDocument {
    a_block_up_to(4.0)
}

/// A block 10 by 10 from 0 to `height` over XY.
fn a_block_up_to(height: f64) -> PartDocument {
    let mut document = PartDocument::new("part", Utc::now());
    document.apply(Operation::CreateSketch {
        plane: WorkPlane::XY,
        on: None,
    });
    document.apply(Operation::AddRectangle {
        sketch: 0,
        corner: PointRef::New(DVec2::ZERO),
        opposite: PointRef::New(DVec2::new(10.0, 10.0)),
        construction: false,
    });
    document.apply(Operation::Extrude {
        sketch: 0,
        areas: document.areas_at(0, &[DVec2::new(5.0, 5.0)]),
        distance: height.into(),
        mode: ExtrusionMode::Add,
    });
    document
}

/// One frame of `document`, seen the way `state` says and edited the way
/// `editor` says.
fn a_frame_of(
    state: &ViewportState,
    document: &mut PartDocument,
    editor: &mut SketchEditor,
) -> SceneFrame {
    let mut extrusion = ExtrusionState::default();
    let lang = Catalogue::french();
    let millimetres = document.scale();
    let context = SketchContext {
        document,
        editor,
        extrusion: &mut extrusion,
        lang: &lang,
    };
    let scale = ViewScale::of(
        &OrbitCamera::default(),
        SCREEN,
        1.0,
        &ViewportConfig::default(),
        millimetres,
    );

    build_frame(state, SCREEN, SCREEN, scale, &context, None, None)
}

/// One frame of the block, seen onto a sketch opened at `sketched_at` across
/// it, after `clicks` on the button that says how the matter in front of the
/// sketch is drawn.
fn a_frame_through_a_block(sketched_at: f32, clicks: usize) -> SceneFrame {
    a_frame_through_a_block_seen(sketched_at, clicks, |_| {})
}

/// The same, the view then left as `prepare` says.
fn a_frame_through_a_block_seen(
    sketched_at: f32,
    clicks: usize,
    prepare: impl FnOnce(&mut ViewportState),
) -> SceneFrame {
    let mut document = a_block();
    let across = WorkPlane {
        origin: DVec3::Z * f64::from(sketched_at),
        ..WorkPlane::XY
    };
    document.apply(Operation::CreateSketch {
        plane: across,
        on: None,
    });

    let mut editor = SketchEditor::default();
    editor.begin_editing(1, across);
    for _ in 0..clicks {
        editor.next_matter_in_front();
    }
    let mut state = ViewportState::default();
    state.mode = ViewMode::Plane(across);
    assert!(
        state.camera.eye().z > ON_THE_TOP,
        "the fixture looks at the block from under its top",
    );
    prepare(&mut state);

    a_frame_of(&state, &mut document, &mut editor)
}

fn triangles(vertices: &[cao_render::Vertex]) -> Vec<[&cao_render::Vertex; 3]> {
    vertices
        .as_chunks::<3>()
        .0
        .iter()
        .map(|[a, b, c]| [a, b, c])
        .collect()
}

fn lies_at(triangle: &[&cao_render::Vertex; 3], height: f32) -> bool {
    triangle
        .iter()
        .all(|corner| (corner.position[2] - height).abs() <= 1e-3)
}

/// Whether a triangle stands on the eye's side of the plane at `height`,
/// touching it at most.
fn stands_over(triangle: &[&cao_render::Vertex; 3], height: f32) -> bool {
    triangle
        .iter()
        .all(|corner| corner.position[2] >= height - 1e-3)
        && triangle
            .iter()
            .any(|corner| corner.position[2] > height + 1e-3)
}

#[test]
fn see_through_draws_the_matter_in_front_translucent() {
    let frame = a_frame_through_a_block(HALFWAY, 1);

    assert!(
        !triangles(&frame.scene_solids)
            .iter()
            .any(|triangle| stands_over(triangle, HALFWAY)),
        "the matter in front of the sketch is drawn opaque, and hides it",
    );
    assert!(
        frame
            .scene_solids
            .iter()
            .any(|vertex| vertex.position[2].abs() <= 1e-3),
        "the matter behind the sketch is not drawn opaque",
    );
    let see_through: Vec<_> = triangles(&frame.scene_surfaces)
        .into_iter()
        .filter(|triangle| stands_over(triangle, HALFWAY))
        .collect();
    assert!(
        !see_through.is_empty(),
        "the matter in front of the sketch is drawn neither opaque nor see-through",
    );
    let opacities: Vec<f32> = see_through
        .iter()
        .flatten()
        .map(|vertex| vertex.color[3])
        .collect();
    assert!(
        opacities.iter().all(|alpha| *alpha > 0.0 && *alpha < 1.0),
        "the matter in front is not drawn translucent, at opacities {opacities:?}",
    );
}

#[test]
fn the_see_through_layer_carries_nothing_lying_on_the_sketch_plane() {
    for sketched_at in [HALFWAY, ON_THE_TOP] {
        let frame = a_frame_through_a_block(sketched_at, 1);

        assert!(
            !triangles(&frame.scene_surfaces)
                .iter()
                .any(|triangle| lies_at(triangle, sketched_at)),
            "a sketch at {sketched_at} has a face of the part laid see-through over it",
        );
    }
    let on_the_top = a_frame_through_a_block(ON_THE_TOP, 1);
    assert!(
        !triangles(&on_the_top.scene_surfaces)
            .iter()
            .any(|triangle| stands_over(triangle, ON_THE_TOP - 1e-2)),
        "a sketch on the top face has nothing in front of it, and something is drawn see-through",
    );
}

#[test]
fn the_see_through_layer_draws_no_face_turned_away_from_the_eye() {
    let frame = a_frame_through_a_block(HALFWAY, 1);
    let eye = ViewportState::default().camera.eye();

    let turned_away = triangles(&frame.scene_surfaces)
        .into_iter()
        .filter(|triangle| stands_over(triangle, HALFWAY))
        .filter(|[a, b, c]| {
            let [a, b, c] = [a, b, c].map(|corner| Vec3::from(corner.position));
            (b - a).cross(c - a).dot(eye - a) <= 0.0
        })
        .count();

    assert_eq!(
        turned_away, 0,
        "the far side of the matter in front is drawn see-through behind its near side",
    );
}

/// How one side of a cut reaches the painter.
#[derive(Clone, Copy, Debug, PartialEq)]
enum Seen {
    Hidden,
    SeeThrough,
    Opaque,
}

fn seen(state: MatterInFront) -> Seen {
    match state {
        MatterInFront::Hidden => Seen::Hidden,
        MatterInFront::SeeThrough => Seen::SeeThrough,
        MatterInFront::Opaque => Seen::Opaque,
    }
}

/// How the triangles `on_this_side` picks out of a frame are drawn: opaque
/// with the solids, see-through with the surfaces, or not at all.
fn how_drawn(frame: &SceneFrame, on_this_side: impl Fn(&[&cao_render::Vertex; 3]) -> bool) -> Seen {
    let opaque = triangles(&frame.scene_solids).iter().any(&on_this_side);
    let veiled: Vec<_> = triangles(&frame.scene_surfaces)
        .into_iter()
        .filter(|triangle| on_this_side(triangle))
        .collect();
    match (opaque, veiled.is_empty()) {
        (true, true) => Seen::Opaque,
        (false, true) => Seen::Hidden,
        (false, false) => {
            let opacities: Vec<f32> = veiled
                .iter()
                .flatten()
                .map(|vertex| vertex.color[3])
                .collect();
            assert!(
                opacities.iter().all(|alpha| *alpha > 0.0 && *alpha < 1.0),
                "matter handed to the surfaces is not translucent, at opacities {opacities:?}",
            );
            Seen::SeeThrough
        }
        (true, false) => panic!("the same side of the cut is drawn both opaque and see-through"),
    }
}

/// How the matter under `height` is drawn: a triangle reaching below it, not
/// lying flat on the ground where the block's own sketch is drawn.
fn under(frame: &SceneFrame, height: f32) -> Seen {
    how_drawn(frame, |triangle| {
        !lies_at(triangle, 0.0)
            && triangle
                .iter()
                .any(|corner| corner.position[2] < height - 1e-3)
    })
}

/// How the matter over `height` is drawn.
fn over(frame: &SceneFrame, height: f32) -> Seen {
    how_drawn(frame, |triangle| {
        triangle
            .iter()
            .any(|corner| corner.position[2] > height + 1e-3)
    })
}

const MID_HEIGHT: f32 = 2.0;

const EVERY_STATE: [MatterInFront; 3] = [
    MatterInFront::Hidden,
    MatterInFront::SeeThrough,
    MatterInFront::Opaque,
];

/// A view with the section on, through XY at mid-height of the block, its
/// sides as `prepare` leaves them.
fn sectioned(prepare: impl FnOnce(&mut Section)) -> ViewportState {
    let mut section = Section::new(WorkPlane::XY);
    section.offset = f64::from(MID_HEIGHT);
    prepare(&mut section);
    let mut state = ViewportState::default();
    state.section = Some(section);
    state
}

fn a_view_of_the_block(state: &ViewportState) -> SceneFrame {
    a_frame_of(state, &mut a_block(), &mut SketchEditor::default())
}

/// How the two sides of the block cut at mid-height are drawn: under the
/// plane, then over it.
fn sides(frame: &SceneFrame) -> (Seen, Seen) {
    (under(frame, MID_HEIGHT), over(frame, MID_HEIGHT))
}

#[test]
fn a_section_draws_what_lies_behind_opaque_and_hides_what_stands_in_front() {
    let mut state = sectioned(|_| {});

    let from_above = a_view_of_the_block(&state);
    state.camera.set_view_angles(0.3, -0.5);
    let from_under = a_view_of_the_block(&state);

    assert_eq!(
        sides(&from_above),
        (Seen::Opaque, Seen::Hidden),
        "a section just turned on does not show the matter under XY opaque and hide what stands over it",
    );
    assert_eq!(
        sides(&from_under),
        (Seen::Opaque, Seen::Hidden),
        "the sides of the section swapped when the eye went under its plane",
    );
}

#[test]
fn each_side_of_the_section_is_hidden_see_through_or_opaque_on_its_own() {
    for behind in EVERY_STATE {
        for in_front in EVERY_STATE {
            let frame = a_view_of_the_block(&sectioned(|section| {
                section.behind = behind;
                section.in_front = in_front;
            }));

            assert_eq!(
                sides(&frame),
                (seen(behind), seen(in_front)),
                "the side behind set {behind:?} and the side in front set {in_front:?} \
                 are drawn otherwise",
            );
        }
    }
}

#[test]
fn inverting_the_section_swaps_the_states_of_its_two_sides() {
    let inverted = a_view_of_the_block(&sectioned(Section::invert));
    let inverted_twice = a_view_of_the_block(&sectioned(|section| {
        section.invert();
        section.invert();
    }));
    let mixed = a_view_of_the_block(&sectioned(|section| {
        section.behind = MatterInFront::SeeThrough;
        section.in_front = MatterInFront::Opaque;
        section.invert();
    }));

    assert_eq!(
        sides(&inverted),
        (Seen::Hidden, Seen::Opaque),
        "« Inverser » does not show the matter over the plane and hide what lies under it",
    );
    assert_eq!(
        sides(&inverted_twice),
        (Seen::Opaque, Seen::Hidden),
        "« Inverser » twice does not come back to where it started",
    );
    assert_eq!(
        sides(&mixed),
        (Seen::Opaque, Seen::SeeThrough),
        "« Inverser » does not swap a see-through side with an opaque one",
    );
}

#[test]
fn the_section_is_cut_once_per_plane_distance_and_part() {
    let mut document = a_block();
    let mut editor = SketchEditor::default();
    let mut state = sectioned(|_| {});
    let cuts = |state: &ViewportState| state.sections.behind_cuts() + state.sections.front_cuts();

    a_frame_of(&state, &mut document, &mut editor);
    a_frame_of(&state, &mut document, &mut editor);
    assert_eq!(
        cuts(&state),
        1,
        "a second frame with nothing changed cut the part again"
    );

    state.camera.set_view_angles(0.3, -0.5);
    a_frame_of(&state, &mut document, &mut editor);
    assert_eq!(
        cuts(&state),
        1,
        "the eye went under the plane and the part was cut again"
    );

    for behind in EVERY_STATE {
        for in_front in EVERY_STATE {
            let section = state.section.as_mut().expect("the section is on");
            section.behind = behind;
            section.in_front = in_front;
            a_frame_of(&state, &mut document, &mut editor);
        }
    }
    assert_eq!(
        cuts(&state),
        2,
        "going round the states of both sides cut a half more than once, or not at all",
    );

    let section = state.section.as_mut().expect("the section is on");
    section.behind = MatterInFront::Opaque;
    section.in_front = MatterInFront::Hidden;
    section.offset = 1.0;
    let moved = a_frame_of(&state, &mut document, &mut editor);
    assert_eq!(
        cuts(&state),
        3,
        "the distance changed and the part was not cut again"
    );
    assert_eq!(
        (under(&moved, 1.0), over(&moved, 1.0)),
        (Seen::Opaque, Seen::Hidden),
        "the cut drawn is not where the distance now puts it",
    );

    state.section.as_mut().expect("the section is on").plane = WorkPlane::YZ;
    a_frame_of(&state, &mut document, &mut editor);
    assert_eq!(
        cuts(&state),
        4,
        "the plane changed and the part was not cut again"
    );

    let mut lower = a_block_up_to(3.0);
    a_frame_of(&state, &mut lower, &mut editor);
    assert_eq!(cuts(&state), 5, "the part changed and was not cut again");
}

/// How far along `normal` the matter drawn opaque reaches, nearest and
/// furthest.
fn opaque_reach(frame: &SceneFrame, normal: DVec3) -> (f64, f64) {
    assert!(!frame.scene_solids.is_empty(), "nothing is drawn opaque");
    frame
        .scene_solids
        .iter()
        .map(|vertex| Vec3::from(vertex.position).as_dvec3().dot(normal))
        .fold((f64::INFINITY, f64::NEG_INFINITY), |(low, high), at| {
            (low.min(at), high.max(at))
        })
}

fn sectioned_along(plane: WorkPlane, offset: f64) -> SceneFrame {
    a_view_of_the_block(&sectioned(|section| {
        section.plane = plane;
        section.offset = offset;
    }))
}

#[test]
fn the_section_cuts_along_whichever_origin_plane_is_chosen() {
    let middle = DVec3::new(5.0, 5.0, 2.0);

    for plane in WorkPlane::ORIGIN_PLANES {
        let normal = plane.normal();
        let offset = normal.dot(middle) + 0.5;

        let (nearest, furthest) = opaque_reach(&sectioned_along(plane, offset), normal);

        assert!(
            (furthest - offset).abs() <= 1e-3,
            "along {plane:?}, the matter drawn reaches {furthest} along its normal, \
             not the cut at {offset}",
        );
        assert!(
            nearest < offset - 1.0,
            "along {plane:?}, the matter behind the cut is not drawn",
        );
    }
}

#[test]
fn the_distance_moves_the_cut_along_the_normal_of_the_plane() {
    for offset in [1.0, 3.0] {
        let (_, top) = opaque_reach(&sectioned_along(WorkPlane::XY, offset), DVec3::Z);

        assert!(
            (top - offset).abs() <= 1e-3,
            "a distance of {offset} from XY draws the block up to {top}",
        );
    }
}

#[test]
fn a_distance_from_a_face_is_signed_along_its_outward_normal() {
    let top = WorkPlane::from_normal(DVec3::Z * 4.0, DVec3::Z);
    let bottom = WorkPlane::from_normal(DVec3::ZERO, DVec3::NEG_Z);

    let (_, under_the_top) = opaque_reach(&sectioned_along(top, -1.0), DVec3::Z);
    let (over_the_bottom, _) = opaque_reach(&sectioned_along(bottom, -1.0), DVec3::Z);

    assert!(
        (under_the_top - 3.0).abs() <= 1e-3,
        "a unit into the part from its top face cuts at {under_the_top}, not 3",
    );
    assert!(
        (over_the_bottom - 1.0).abs() <= 1e-3,
        "a unit into the part from its bottom face cuts at {over_the_bottom}, not 1",
    );
}

/// How much of the frame's opaque matter lies flat within a hundredth of
/// `height`: what stands in front is closed a drawing tolerance ahead of the
/// plane, by #578's design, so that none of it lies on a sketch drawn there.
fn area_lying_at(frame: &SceneFrame, height: f32) -> f32 {
    triangles(&frame.scene_solids)
        .into_iter()
        .filter(|triangle| {
            triangle
                .iter()
                .all(|corner| (corner.position[2] - height).abs() <= 1e-2)
        })
        .map(|[a, b, c]| {
            let [a, b, c] = [a, b, c].map(|corner| Vec3::from(corner.position));
            (b - a).cross(c - a).length() / 2.0
        })
        .sum()
}

#[test]
fn the_section_is_drawn_filled_on_the_cutting_plane() {
    for (state, which) in [
        (sectioned(|_| {}), "the matter behind"),
        (sectioned(Section::invert), "the matter in front"),
    ] {
        let filled = area_lying_at(&a_view_of_the_block(&state), MID_HEIGHT);

        assert!(
            (filled - 100.0).abs() <= 1e-2,
            "{which} kept, the cut across the block 10 by 10 is filled over {filled}, not 100",
        );
    }
}

/// How many vertices of a frame's sketch lines, and of its tinted areas, lie
/// in `plane`. The part's own matter goes to the solids, and the grid and the
/// axes to the world lines, so on a free view these count the drawing alone.
fn drawn_in(frame: &SceneFrame, plane: WorkPlane) -> (usize, usize) {
    let (origin, normal) = (plane.origin.as_vec3(), plane.normal().as_vec3());
    let lying = |vertices: &[cao_render::Vertex]| {
        vertices
            .iter()
            .filter(|vertex| (Vec3::from(vertex.position) - origin).dot(normal).abs() < 1e-4)
            .count()
    };
    (lying(&frame.scene_lines), lying(&frame.scene_surfaces))
}

fn twenty_over_the_block() -> WorkPlane {
    WorkPlane {
        origin: DVec3::Z * 20.0,
        ..WorkPlane::XY
    }
}

/// The block, and a rectangle drawn twenty over it that nothing was raised
/// from.
fn a_block_and_a_sketch_never_used() -> PartDocument {
    let mut document = a_block();
    document.apply(Operation::CreateSketch {
        plane: twenty_over_the_block(),
        on: None,
    });
    document.apply(Operation::AddRectangle {
        sketch: 1,
        corner: PointRef::New(DVec2::ZERO),
        opposite: PointRef::New(DVec2::new(10.0, 10.0)),
        construction: false,
    });
    document
}

/// A ring a quarter turned, swept off a rectangle drawn on XZ beside the X
/// axis.
fn a_quarter_ring() -> PartDocument {
    let mut document = PartDocument::new("part", Utc::now());
    document.apply(Operation::CreateSketch {
        plane: WorkPlane::XZ,
        on: None,
    });
    document.apply(Operation::AddRectangle {
        sketch: 0,
        corner: PointRef::New(DVec2::new(0.0, 5.0)),
        opposite: PointRef::New(DVec2::new(10.0, 10.0)),
        construction: false,
    });
    document.apply(Operation::Revolve {
        sketch: 0,
        areas: document.areas_at(0, &[DVec2::new(5.0, 7.5)]),
        axis: cao_part::RevolutionAxis::Sketch(cao_sketch::SketchAxis::U),
        angle: 90.0.into(),
        mode: ExtrusionMode::Add,
    });
    document
}

#[test]
fn a_sketch_once_extruded_draws_neither_its_traits_nor_its_areas() {
    let mut document = a_block();
    let frame = a_frame_of(
        &ViewportState::default(),
        &mut document,
        &mut SketchEditor::default(),
    );

    assert_eq!(
        drawn_in(&frame, WorkPlane::XY),
        (0, 0),
        "the rectangle the block was raised from is still drawn, as (line, area) vertices",
    );
}

#[test]
fn a_sketch_once_revolved_draws_neither_its_traits_nor_its_areas() {
    let mut document = a_quarter_ring();
    assert!(
        !document.body().triangles().is_empty(),
        "the fixture sweeps no matter at all",
    );
    let frame = a_frame_of(
        &ViewportState::default(),
        &mut document,
        &mut SketchEditor::default(),
    );

    assert_eq!(
        drawn_in(&frame, WorkPlane::XZ),
        (0, 0),
        "the rectangle the ring was swept from is still drawn, as (line, area) vertices",
    );
}

#[test]
fn a_sketch_never_used_keeps_its_traits_on_screen_with_its_eye_lit() {
    let mut document = a_block_and_a_sketch_never_used();
    let state = ViewportState::default();

    assert!(
        state.eye_lit(&document, 1),
        "a sketch nothing was raised from has its eye dark",
    );
    let frame = a_frame_of(&state, &mut document, &mut SketchEditor::default());
    let (lines, areas) = drawn_in(&frame, twenty_over_the_block());
    assert!(
        lines > 0 && areas > 0,
        "a sketch nothing was raised from is drawn {lines} line and {areas} area vertices",
    );
}

#[test]
fn the_eye_of_a_used_sketch_brings_its_traits_back_and_takes_them_away_again() {
    let mut document = a_block();
    let mut state = ViewportState::default();
    assert!(
        !state.eye_lit(&document, 0),
        "the sketch the block was raised from has its eye lit before anyone touched it",
    );

    state.toggle_eye(&document, 0);
    assert!(state.eye_lit(&document, 0), "a click leaves the eye dark");
    let lit = a_frame_of(&state, &mut document, &mut SketchEditor::default());

    state.toggle_eye(&document, 0);
    assert!(
        !state.eye_lit(&document, 0),
        "a second click leaves the eye lit"
    );
    let dark = a_frame_of(&state, &mut document, &mut SketchEditor::default());

    let (lines, areas) = drawn_in(&lit, WorkPlane::XY);
    assert!(
        lines > 0 && areas > 0,
        "its eye lit, the used sketch is drawn {lines} line and {areas} area vertices",
    );
    assert_eq!(
        drawn_in(&dark, WorkPlane::XY),
        (0, 0),
        "its eye put out again, the used sketch is still drawn, as (line, area) vertices",
    );
}

#[test]
fn a_used_sketch_reopened_is_drawn_whatever_its_eye() {
    let mut document = a_block();
    let state = ViewportState::default();
    assert!(
        !state.eye_lit(&document, 0),
        "the fixture wants the eye of the used sketch dark",
    );
    let mut editor = SketchEditor::default();
    editor.begin_editing(0, WorkPlane::XY);

    let frame = a_frame_of(&state, &mut document, &mut editor);

    let (lines, areas) = drawn_in(&frame, WorkPlane::XY);
    assert!(
        lines > 0 && areas > 0,
        "the sketch being drawn on is drawn {lines} line and {areas} area vertices",
    );
}

#[test]
fn a_sketch_drawn_anew_where_one_was_rewound_away_does_not_inherit_its_eye() {
    let mut document = a_block_and_a_sketch_never_used();
    let mut state = ViewportState::default();
    state.toggle_eye(&document, 1);
    assert!(
        !state.eye_lit(&document, 1),
        "the fixture wants the eye of the second sketch put out",
    );

    document.rewind_to(3);
    assert_eq!(
        document.sketches().len(),
        1,
        "the rewind kept the second sketch"
    );
    document.apply(Operation::CreateSketch {
        plane: twenty_over_the_block(),
        on: None,
    });

    assert!(
        state.eye_lit(&document, 1),
        "a sketch just drawn, never used, has its eye put out by the sketch rewound away",
    );
}

/// The first trait of a sketch, and where its ends stand in the world.
fn the_first_trait(document: &PartDocument, sketch: usize) -> (Element, Vec3, Vec3) {
    let drawing = &document.sketches()[sketch];
    let (id, segment) = drawing
        .live_segments()
        .next()
        .expect("the sketch holds a trait");
    let [from, to] = [segment.start, segment.end]
        .map(|point| drawing.plane.to_world(drawing.point(point)).as_vec3());
    (Element::Segment(id), from, to)
}

fn on_the_trait(at: Vec3, from: Vec3, to: Vec3) -> bool {
    let along = (at - from).dot(to - from) / (to - from).length_squared();
    (0.0..=1.0).contains(&along) && (from + (to - from) * along - at).length() < 1e-3
}

/// The colours of the frame's sketch strokes lying in `plane`, on the trait
/// from `from` to `to` and off it. A stroke is a pair of vertices, and it lies
/// on the trait when both its ends do: a neighbouring side touches the trait at
/// a corner, and is not on it for that.
fn line_colours(
    frame: &SceneFrame,
    plane: WorkPlane,
    from: Vec3,
    to: Vec3,
) -> (Vec<[f32; 4]>, Vec<[f32; 4]>) {
    let (origin, normal) = (plane.origin.as_vec3(), plane.normal().as_vec3());
    let (mut on, mut off) = (Vec::new(), Vec::new());
    for stroke in frame.scene_lines.as_chunks::<2>().0 {
        let ends = stroke.each_ref().map(|vertex| Vec3::from(vertex.position));
        if !ends
            .iter()
            .all(|end| (*end - origin).dot(normal).abs() < 1e-4)
        {
            continue;
        }
        let colours = stroke.iter().map(|vertex| vertex.color);
        match ends.iter().all(|end| on_the_trait(*end, from, to)) {
            true => on.extend(colours),
            false => off.extend(colours),
        }
    }
    (on, off)
}

#[test]
fn an_element_lit_from_the_tree_is_drawn_lit_in_its_closed_sketch() {
    let mut used = a_block();
    let (element, from, to) = the_first_trait(&used, 0);
    let mut state = ViewportState::default();
    state.light(Points::Element { sketch: 0, element });
    let frame = a_frame_of(&state, &mut used, &mut SketchEditor::default());

    let (on, off) = line_colours(&frame, WorkPlane::XY, from, to);
    assert!(
        !on.is_empty(),
        "a trait lit from the tree in a sketch whose eye is out is not drawn at all",
    );
    assert!(
        off.is_empty(),
        "lighting one trait of a sketch whose eye is out draws {} vertices of the others",
        off.len(),
    );

    let mut shown = a_block_and_a_sketch_never_used();
    let (element, from, to) = the_first_trait(&shown, 1);
    let plain = a_frame_of(
        &ViewportState::default(),
        &mut shown,
        &mut SketchEditor::default(),
    );
    let mut state = ViewportState::default();
    state.light(Points::Element { sketch: 1, element });
    let lit = a_frame_of(&state, &mut shown, &mut SketchEditor::default());

    let (plain_on, plain_off) = line_colours(&plain, twenty_over_the_block(), from, to);
    let (lit_on, lit_off) = line_colours(&lit, twenty_over_the_block(), from, to);
    assert!(
        !lit_on.is_empty() && lit_on.iter().all(|colour| !plain_on.contains(colour)),
        "a trait lit from the tree in a sketch on screen is drawn as it was, {lit_on:?}",
    );
    assert_eq!(
        lit_off, plain_off,
        "lighting one trait changed how the others are drawn"
    );
}

#[test]
fn an_area_lit_from_the_tree_is_drawn_lit_in_its_closed_sketch() {
    let mut used = a_block();
    let mut state = ViewportState::default();
    state.light(Points::Area { sketch: 0, rank: 0 });
    let frame = a_frame_of(&state, &mut used, &mut SketchEditor::default());

    let (_, areas) = drawn_in(&frame, WorkPlane::XY);
    assert!(
        areas > 0,
        "an area lit from the tree in a sketch whose eye is out is not drawn at all",
    );

    let mut shown = a_block_and_a_sketch_never_used();
    let tinted = |frame: &SceneFrame| -> Vec<[f32; 4]> {
        let plane = twenty_over_the_block();
        frame
            .scene_surfaces
            .iter()
            .filter(|vertex| (vertex.position[2] - plane.origin.z as f32).abs() < 1e-4)
            .map(|vertex| vertex.color)
            .collect()
    };
    let plain = tinted(&a_frame_of(
        &ViewportState::default(),
        &mut shown,
        &mut SketchEditor::default(),
    ));
    let mut state = ViewportState::default();
    state.light(Points::Area { sketch: 1, rank: 0 });
    let lit = tinted(&a_frame_of(
        &state,
        &mut shown,
        &mut SketchEditor::default(),
    ));

    assert!(
        !lit.is_empty() && lit.iter().all(|colour| !plain.contains(colour)),
        "an area lit from the tree in a sketch on screen is tinted as it was, {lit:?}",
    );
}

/// The view with the body drawn `mode`, chosen in « TOUT LE CORPS ».
fn body_drawn(state: &mut ViewportState, mode: MatterInFront) {
    state.set_whole_body(mode);
}

/// How the block's matter is drawn, leaving out the ground its sketch lies on.
fn the_block_seen(frame: &SceneFrame) -> Seen {
    how_drawn(frame, |triangle| !lies_at(triangle, 0.0))
}

#[test]
fn the_body_is_drawn_opaque_by_default() {
    let state = ViewportState::default();

    assert_eq!(state.whole_body_choice(), MatterInFront::Opaque);
    assert_eq!(
        the_block_seen(&a_view_of_the_block(&state)),
        Seen::Opaque,
        "a part just opened does not show its matter opaque",
    );
}

#[test]
fn the_body_is_drawn_invisible_see_through_or_opaque_as_its_mode_says() {
    for mode in EVERY_STATE {
        let mut state = ViewportState::default();
        body_drawn(&mut state, mode);

        assert_eq!(
            the_block_seen(&a_view_of_the_block(&state)),
            seen(mode),
            "the body set {mode:?} is drawn otherwise",
        );
    }
}

#[test]
fn a_see_through_body_keeps_the_matter_in_front_of_the_sketch_cut_away() {
    let frame = a_frame_through_a_block_seen(HALFWAY, 0, |state| {
        body_drawn(state, MatterInFront::SeeThrough);
    });

    assert_eq!(
        (under(&frame, HALFWAY), over(&frame, HALFWAY)),
        (Seen::SeeThrough, Seen::Hidden),
        "a see-through body under a sketch that hides what stands in front, as (behind, in front)",
    );
}

#[test]
fn a_see_through_body_keeps_the_side_the_section_hides_hidden() {
    let mut state = sectioned(|_| {});
    body_drawn(&mut state, MatterInFront::SeeThrough);

    assert_eq!(
        sides(&a_view_of_the_block(&state)),
        (Seen::SeeThrough, Seen::Hidden),
        "a see-through body cut by a section that hides its front side, as (behind, in front)",
    );
}

fn shown_or_hidden(state: MatterInFront) -> Seen {
    match state {
        MatterInFront::Hidden => Seen::Hidden,
        MatterInFront::SeeThrough | MatterInFront::Opaque => Seen::SeeThrough,
    }
}

#[test]
fn a_see_through_body_shows_what_the_section_shows_and_nothing_it_hides() {
    for behind in EVERY_STATE {
        for in_front in EVERY_STATE {
            let mut state = sectioned(|section| {
                section.behind = behind;
                section.in_front = in_front;
            });
            body_drawn(&mut state, MatterInFront::SeeThrough);

            assert_eq!(
                sides(&a_view_of_the_block(&state)),
                (shown_or_hidden(behind), shown_or_hidden(in_front)),
                "a see-through body cut by a section {behind:?} behind and {in_front:?} in \
                 front, as (behind, in front)",
            );
        }
    }
}

#[test]
fn an_invisible_body_draws_nothing_whatever_the_section_or_the_sketch_shows() {
    for behind in EVERY_STATE {
        for in_front in EVERY_STATE {
            let mut state = sectioned(|section| {
                section.behind = behind;
                section.in_front = in_front;
            });
            body_drawn(&mut state, MatterInFront::Hidden);

            assert_eq!(
                sides(&a_view_of_the_block(&state)),
                (Seen::Hidden, Seen::Hidden),
                "an invisible body cut by a section {behind:?} behind and {in_front:?} in \
                 front, as (behind, in front)",
            );
        }
    }
    for clicks in 0..3 {
        let frame = a_frame_through_a_block_seen(HALFWAY, clicks, |state| {
            body_drawn(state, MatterInFront::Hidden);
        });

        assert_eq!(
            (under(&frame, HALFWAY), over(&frame, HALFWAY)),
            (Seen::Hidden, Seen::Hidden),
            "an invisible body under a sketch after {clicks} clicks on its own cut, as \
             (behind, in front)",
        );
    }
}

#[test]
fn an_area_of_the_open_sketch_lit_from_the_tree_is_drawn_lit() {
    let mut document = a_block();
    let tinted = |frame: &SceneFrame| -> Vec<[f32; 4]> {
        frame
            .scene_surfaces
            .iter()
            .filter(|vertex| vertex.position[2].abs() < 1e-4)
            .map(|vertex| vertex.color)
            .collect()
    };
    let open = || {
        let mut editor = SketchEditor::default();
        editor.begin_editing(0, WorkPlane::XY);
        editor
    };
    let plain = tinted(&a_frame_of(
        &ViewportState::default(),
        &mut document,
        &mut open(),
    ));
    let mut state = ViewportState::default();
    state.light(Points::Area { sketch: 0, rank: 0 });
    let lit = tinted(&a_frame_of(&state, &mut document, &mut open()));

    assert!(
        !plain.is_empty(),
        "the open sketch's area is not tinted at all"
    );
    assert!(
        !lit.is_empty() && lit.iter().all(|colour| !plain.contains(colour)),
        "an area of the open sketch lit from the tree is tinted as it was, {lit:?}",
    );
}

/// Where a point of a frame stands, in the world.
fn at(vertex: &cao_render::Vertex) -> DVec3 {
    Vec3::from(vertex.position).as_dvec3()
}

/// How far from `plane` a point drawn in single precision may stand and still
/// be read as lying in it.
const ON_THE_PLANE: f64 = 1e-4;

fn lies_in(point: DVec3, plane: WorkPlane) -> bool {
    plane.normal().dot(point - plane.origin).abs() < ON_THE_PLANE
}

/// The frame's lines, as the pairs of points each is drawn between.
fn segments(frame: &SceneFrame) -> Vec<(DVec3, DVec3)> {
    frame
        .scene_lines
        .as_chunks::<2>()
        .0
        .iter()
        .map(|[from, to]| (at(from), at(to)))
        .collect()
}

/// The see-through square drawn on a plane: where its middle stands, how far
/// its corners reach from it, and how many lines lie in the plane with it.
#[derive(Debug)]
struct Square {
    centre: DVec3,
    half_diagonal: f64,
    outlined_by: usize,
}

/// The square drawn see-through on `plane`, read off its two furthest corners,
/// which a square holds across its diagonal; `None` when nothing translucent
/// lies there.
fn the_square(frame: &SceneFrame, plane: WorkPlane) -> Option<Square> {
    let veiled: Vec<&cao_render::Vertex> = frame
        .scene_surfaces
        .iter()
        .filter(|vertex| lies_in(at(vertex), plane))
        .collect();
    if veiled.is_empty() {
        return None;
    }
    assert!(
        veiled
            .iter()
            .all(|vertex| vertex.color[3] > 0.0 && vertex.color[3] < 1.0),
        "what is drawn on the cutting plane is not see-through",
    );
    let corners: Vec<DVec3> = veiled.iter().map(|vertex| at(vertex)).collect();
    let (from, to) = corners
        .iter()
        .flat_map(|a| corners.iter().map(move |b| (*a, *b)))
        .max_by(|(a, b), (c, d)| a.distance(*b).total_cmp(&c.distance(*d)))
        .expect("there is a corner");
    let outlined_by = segments(frame)
        .into_iter()
        .filter(|(a, b)| lies_in(*a, plane) && lies_in(*b, plane) && a.distance(*b) > 1e-6)
        .count();
    Some(Square {
        centre: (from + to) / 2.0,
        half_diagonal: from.distance(to) / 2.0,
        outlined_by,
    })
}

/// The way the arrow points: a line leaving `centre` along `normal`, one way or
/// the other, read from the centre outwards.
fn the_arrow(frame: &SceneFrame, centre: DVec3, normal: DVec3) -> Option<DVec3> {
    segments(frame).into_iter().find_map(|(a, b)| {
        let (from, to) = match (a.distance(centre) < 1e-3, b.distance(centre) < 1e-3) {
            (true, false) => (a, b),
            (false, true) => (b, a),
            _ => return None,
        };
        let pointing = (to - from).normalize_or_zero();
        (pointing.dot(normal).abs() > 1.0 - 1e-4).then_some(pointing)
    })
}

/// The cutting plane, the middle of the block it cuts through, and a frame of
/// the block, with the section as `prepare` leaves it.
fn the_section_drawn(prepare: impl FnOnce(&mut Section)) -> (WorkPlane, DVec3, SceneFrame) {
    let state = sectioned(prepare);
    let document = a_block();
    let section = state.section.as_ref().expect("the section is on");
    let cut = section.cutting_plane();
    let handle = section.handle(document.body());
    (cut, handle, a_view_of_the_block(&state))
}

#[test]
fn with_the_section_tool_open_a_square_and_an_arrow_stand_on_the_cutting_plane() {
    let (cut, handle, frame) = the_section_drawn(|_| {});

    let square = the_square(&frame, cut).unwrap_or_else(|| {
        panic!(
            "with the section tool open, nothing see-through is drawn on the cutting plane {cut:?}"
        )
    });

    assert!(
        square.centre.distance(handle) < 1e-3,
        "the square stands around {:?}, not where the handle stands, {handle:?}",
        square.centre,
    );
    assert!(
        square.half_diagonal > 1e-3,
        "the square on the cutting plane has no size: {square:?}",
    );
    assert!(
        square.outlined_by >= 4,
        "the square on the cutting plane is drawn without its outline: {square:?}",
    );
    assert!(
        the_arrow(&frame, square.centre, cut.normal()).is_some(),
        "no arrow stands on the square's centre along the normal {:?}",
        cut.normal(),
    );
}

#[test]
fn inverser_turns_the_arrow_round() {
    let pointing = |prepare: fn(&mut Section)| {
        let (cut, _, frame) = the_section_drawn(prepare);
        let square = the_square(&frame, cut).expect("a square stands on the cutting plane");
        the_arrow(&frame, square.centre, cut.normal())
            .expect("an arrow stands on the square along the normal")
    };

    let before = pointing(|_| {});
    let inverted = pointing(Section::invert);
    let inverted_twice = pointing(|section| {
        section.invert();
        section.invert();
    });

    assert!(
        before.dot(inverted) < -0.99,
        "« Inverser » left the arrow pointing {inverted:?}, where it pointed {before:?}",
    );
    assert!(
        before.dot(inverted_twice) > 0.99,
        "« Inverser » twice left the arrow pointing {inverted_twice:?}, not back to {before:?}",
    );
}

/// One frame of the block with the section on, and the scale it was drawn at,
/// both read from the view's own camera.
fn a_frame_at_its_own_zoom(state: &ViewportState) -> (SceneFrame, ViewScale) {
    let mut document = a_block();
    let mut editor = SketchEditor::default();
    let mut extrusion = ExtrusionState::default();
    let lang = Catalogue::french();
    let millimetres = document.scale();
    let scale = ViewScale::of(
        &state.camera,
        SCREEN,
        1.0,
        &ViewportConfig::default(),
        millimetres,
    );
    let context = SketchContext {
        document: &mut document,
        editor: &mut editor,
        extrusion: &mut extrusion,
        lang: &lang,
    };
    (
        build_frame(state, SCREEN, SCREEN, scale, &context, None, None),
        scale,
    )
}

#[test]
fn the_square_keeps_its_size_on_screen_when_the_view_is_zoomed_in_and_out() {
    let mut measured = Vec::new();
    for zoom in [1.0, 2.5, 0.4] {
        let mut state = sectioned(|_| {});
        state.camera.zoom_by_factor(zoom);
        let cut = state.section().expect("the section is on").cutting_plane();
        let (frame, scale) = a_frame_at_its_own_zoom(&state);
        let square = the_square(&frame, cut)
            .unwrap_or_else(|| panic!("zoomed by {zoom}, no square is drawn on the cutting plane"));
        measured.push((
            zoom,
            square.half_diagonal,
            square.half_diagonal / scale.world_size_of(1.0),
        ));
    }

    let (_, unzoomed, pixels) = measured[0];
    for (zoom, world, on_screen) in &measured[1..] {
        assert!(
            (world - unzoomed).abs() > 1e-6,
            "zoomed by {zoom}, the view's scale did not change: the test measures nothing",
        );
        assert!(
            (on_screen - pixels).abs() <= 1e-3 * pixels,
            "zoomed by {zoom}, the square reaches {on_screen} pixels from its centre, \
             where it reached {pixels}",
        );
    }
}

/// The section panel, over the block, and what it acts on.
struct OnTheBench {
    state: ViewportState,
    document: PartDocument,
    editor: SketchEditor,
    extrusion: ExtrusionState,
    lang: Catalogue,
}

/// The section panel over the block, the section opened as the ribbon opens
/// it: along XY, through the middle of the part.
fn the_section_panel() -> egui_kittest::Harness<'static, OnTheBench> {
    the_section_panel_over(a_block())
}

/// The section panel over a part, the section opened as the ribbon opens it.
fn the_section_panel_over(document: PartDocument) -> egui_kittest::Harness<'static, OnTheBench> {
    let mut state = ViewportState::default();
    state.toggle_section(document.body());
    let bench = OnTheBench {
        state,
        document,
        editor: SketchEditor::default(),
        extrusion: ExtrusionState::default(),
        lang: Catalogue::french(),
    };
    let mut panel = egui_kittest::Harness::builder()
        .with_size(SCREEN.size())
        .build_ui_state(
            |ui, bench: &mut OnTheBench| {
                let mut context = SketchContext {
                    document: &mut bench.document,
                    editor: &mut bench.editor,
                    extrusion: &mut bench.extrusion,
                    lang: &bench.lang,
                };
                paint_section_panel(ui, &mut bench.state, SCREEN, &mut context);
            },
            bench,
        );
    panel.run();
    panel
}

/// The field a label names: the one it is attached to, or the first drawn to
/// its right on the same row.
fn the_field_beside<'tree>(
    panel: &'tree egui_kittest::Harness<'_, OnTheBench>,
    label: &str,
) -> egui_kittest::Node<'tree> {
    use egui::accesskit::Role;
    use egui_kittest::kittest::{NodeT, Queryable};

    let reads = |node: &egui_kittest::Node<'_>| {
        let node = node.accesskit_node();
        node.label().as_deref() == Some(label) || node.value().as_deref() == Some(label)
    };
    if let Some(field) = panel
        .query_all_by_role(Role::TextInput)
        .find(|field| field.accesskit_node().label().as_deref() == Some(label))
    {
        return field;
    }
    let named = panel
        .get_all_by(|_| true)
        .find(|node| node.accesskit_node().role() != Role::TextInput && reads(node))
        .unwrap_or_else(|| panic!("the section panel shows no « {label} »"));
    let row = named.rect();
    panel
        .query_all_by_role(Role::TextInput)
        .filter(|field| {
            let at = field.rect();
            at.min.x >= row.max.x - 1.0 && at.min.y <= row.center().y && row.center().y <= at.max.y
        })
        .min_by(|a, b| a.rect().min.x.total_cmp(&b.rect().min.x))
        .unwrap_or_else(|| panic!("no field stands beside « {label} » in the section panel"))
}

/// Types over the field beside a label, the way selecting all of it first
/// would.
fn type_beside(panel: &mut egui_kittest::Harness<'_, OnTheBench>, label: &str, text: &str) {
    the_field_beside(panel, label).focus();
    panel.run();
    panel.event(egui::Event::Key {
        key: egui::Key::A,
        physical_key: None,
        pressed: true,
        repeat: false,
        modifiers: egui::Modifiers::COMMAND,
    });
    panel.event(egui::Event::Text(text.to_owned()));
    panel.run();
}

fn the_section_on_the_bench<'bench>(
    panel: &'bench egui_kittest::Harness<'_, OnTheBench>,
) -> &'bench Section {
    panel
        .state()
        .state
        .section()
        .expect("the section panel closed its section")
}

fn the_bench_drawn(panel: &mut egui_kittest::Harness<'_, OnTheBench>) -> SceneFrame {
    let bench = panel.state_mut();
    a_frame_of(&bench.state, &mut bench.document, &mut bench.editor)
}

#[test]
fn typing_an_angle_tilts_the_cut_the_square_and_the_arrow_together_about_the_square_s_centre() {
    let thirty = 30_f64.to_radians();
    for (field, turned_about) in [("Inclinaison 1", DVec3::X), ("Inclinaison 2", DVec3::Y)] {
        let mut panel = the_section_panel();
        let centre = {
            let bench = panel.state();
            the_section_on_the_bench(&panel).handle(bench.document.body())
        };

        type_beside(&mut panel, field, "30");

        let cut = the_section_on_the_bench(&panel).cutting_plane();
        let normal = cut.normal();
        let frame = the_bench_drawn(&mut panel);
        assert!(
            (normal.dot(DVec3::Z) - thirty.cos()).abs() < 1e-9,
            "« {field} » at 30 left the cut facing {normal:?}, {}° off XY's normal",
            normal.dot(DVec3::Z).clamp(-1.0, 1.0).acos().to_degrees(),
        );
        assert!(
            normal.dot(turned_about).abs() < 1e-9,
            "« {field} » turned the cut about another axis than {turned_about:?}, \
             which lies in XY: it faces {normal:?}",
        );
        assert!(
            normal.dot(centre - cut.origin).abs() < 1e-9,
            "« {field} » at 30 turned the cut off the square's centre {centre:?}",
        );
        let (nearest, furthest) = opaque_reach(&frame, normal);
        assert!(
            (furthest - normal.dot(centre)).abs() <= 1e-3 && nearest < furthest - 1.0,
            "« {field} » at 30: the matter kept is drawn from {nearest} to {furthest} along \
             the tilted normal, not up to the tilted cut at {}",
            normal.dot(centre),
        );
        let square = the_square(&frame, cut).unwrap_or_else(|| {
            panic!("« {field} » at 30: no square is drawn on the tilted cutting plane")
        });
        assert!(
            square.centre.distance(centre) < 1e-3,
            "« {field} » at 30 moved the square's centre from {centre:?} to {:?}",
            square.centre,
        );
        assert!(
            the_arrow(&frame, square.centre, normal).is_some(),
            "« {field} » at 30: no arrow stands on the square along the tilted normal {normal:?}",
        );
    }
}

#[test]
fn zero_and_zero_typed_back_give_today_s_cut_exactly() {
    let mut panel = the_section_panel();
    let section = the_section_on_the_bench(&panel);
    let today = WorkPlane {
        origin: section.plane.origin + section.plane.normal() * section.offset,
        ..section.plane
    };
    let drawn_today: Vec<[f32; 3]> = the_bench_drawn(&mut panel)
        .scene_solids
        .iter()
        .map(|vertex| vertex.position)
        .collect();

    type_beside(&mut panel, "Inclinaison 1", "30");
    type_beside(&mut panel, "Inclinaison 2", "-20");
    let tilted = the_section_on_the_bench(&panel).cutting_plane();
    type_beside(&mut panel, "Inclinaison 1", "0");
    type_beside(&mut panel, "Inclinaison 2", "0");
    let back = the_section_on_the_bench(&panel).cutting_plane();
    let drawn_back: Vec<[f32; 3]> = the_bench_drawn(&mut panel)
        .scene_solids
        .iter()
        .map(|vertex| vertex.position)
        .collect();

    assert_ne!(tilted, today, "30 and -20 typed tilted nothing");
    assert_eq!(
        back, today,
        "0 and 0 typed back, the section cuts along {back:?}, not where it cut before any angle",
    );
    assert!(
        drawn_back == drawn_today,
        "0 and 0 typed back, the matter is drawn otherwise than before any angle",
    );
}

#[test]
fn the_typed_distance_moves_a_tilted_plane_along_its_tilted_normal() {
    let mut panel = the_section_panel();
    type_beside(&mut panel, "Inclinaison 2", "20");
    let read = |panel: &mut egui_kittest::Harness<'_, OnTheBench>| {
        let cut = the_section_on_the_bench(panel).cutting_plane();
        let frame = the_bench_drawn(panel);
        let square = the_square(&frame, cut)
            .unwrap_or_else(|| panic!("no square is drawn on the cutting plane {cut:?}"));
        (cut, opaque_reach(&frame, cut.normal()).1, square.centre)
    };

    type_beside(&mut panel, "Distance", "1");
    let (at_one, drawn_at_one, centre_at_one) = read(&mut panel);
    type_beside(&mut panel, "Distance", "3");
    let (at_three, drawn_at_three, centre_at_three) = read(&mut panel);

    let normal = at_one.normal();
    assert!(
        (normal.dot(DVec3::Z) - 20_f64.to_radians().cos()).abs() < 1e-9,
        "« Inclinaison 2 » at 20 left the cut facing {normal:?}",
    );
    assert!(
        at_three.normal().abs_diff_eq(normal, 1e-12),
        "the distance typed turned the tilted cut from {normal:?} to {:?}",
        at_three.normal(),
    );
    assert!(
        (normal.dot(at_three.origin - at_one.origin) - 2.0).abs() < 1e-9,
        "from 1 to 3, the tilted cut moved {} along its normal, not 2",
        normal.dot(at_three.origin - at_one.origin),
    );
    assert!(
        (drawn_at_three - drawn_at_one - 2.0).abs() <= 1e-3,
        "from 1 to 3, the matter kept is drawn {} further along the tilted normal, not 2",
        drawn_at_three - drawn_at_one,
    );
    assert!(
        (centre_at_three - centre_at_one - normal * 2.0).length() < 1e-3,
        "from 1 to 3, the square went from {centre_at_one:?} to {centre_at_three:?}, \
         not 2 along the tilted normal",
    );
}

/// The top of the block, as the replay numbers its faces.
fn the_top_face(document: &PartDocument) -> usize {
    (0..200)
        .find(|face| {
            document.body().plane_of(*face).is_some_and(|plane| {
                plane.normal.abs_diff_eq(DVec3::Z, 1e-9)
                    && plane
                        .corners
                        .iter()
                        .all(|corner| (corner.z - f64::from(ON_THE_TOP)).abs() < 1e-9)
            })
        })
        .expect("the block has a top")
}

/// The section panel over the block, the section laid on its top as a click
/// on that face lays it, and the point of the face its square stands on.
fn the_section_panel_on_the_top() -> (egui_kittest::Harness<'static, OnTheBench>, DVec3) {
    the_section_panel_on_the_top_of(a_block())
}

/// The same over another part 10 by 10 from 0 to 4 over XY.
fn the_section_panel_on_the_top_of(
    document: PartDocument,
) -> (egui_kittest::Harness<'static, OnTheBench>, DVec3) {
    let mut panel = the_section_panel_over(document);
    let bench = panel.state_mut();
    let face = the_top_face(&bench.document);
    let top = bench
        .document
        .body()
        .plane_of(face)
        .expect("the top is flat");
    let laid = WorkPlane::from_face(&top.corners, top.normal, DVec3::Y);
    let section = bench.state.section.as_mut().expect("the section is on");
    section.choose_face(laid);
    section.on = Some(cao_part::history::FaceAnchor { face, up: DVec3::Y });
    panel.run();
    let stood = {
        let bench = panel.state();
        the_section_on_the_bench(&panel).handle(bench.document.body())
    };
    assert!(
        stood.abs_diff_eq(DVec3::new(5.0, 5.0, 4.0), 1e-9),
        "laid on the top of the block, the square stands at {stood:?}, not over its middle",
    );
    (panel, stood)
}

#[test]
fn a_section_on_a_face_turns_about_the_point_of_the_face_its_square_stood_on() {
    let ten = 10_f64.to_radians();
    for field in ["Inclinaison 1", "Inclinaison 2"] {
        let (mut panel, stood) = the_section_panel_on_the_top();

        type_beside(&mut panel, field, "10");

        let cut = the_section_on_the_bench(&panel).cutting_plane();
        let normal = cut.normal();
        let frame = the_bench_drawn(&mut panel);
        assert!(
            (normal.dot(DVec3::Z) - ten.cos()).abs() < 1e-9,
            "« {field} » at 10 left the section on the top facing {normal:?}",
        );
        assert!(
            normal.dot(stood - cut.origin).abs() < 1e-9,
            "« {field} » at 10 turned the section on the top off the point {stood:?} its \
             square stood on: the cut passes {} from it",
            normal.dot(stood - cut.origin),
        );
        let square = the_square(&frame, cut).unwrap_or_else(|| {
            panic!("« {field} » at 10: no square is drawn on the tilted cut of the top")
        });
        assert!(
            square.centre.distance(stood) < 1e-3,
            "« {field} » at 10 moved the square on the top from {stood:?} to {:?}",
            square.centre,
        );
    }
}

#[test]
fn a_distance_typed_on_a_tilted_face_section_moves_it_that_far_along_its_tilted_normal_from_the_face()
 {
    let (mut panel, stood) = the_section_panel_on_the_top();
    type_beside(&mut panel, "Inclinaison 1", "10");

    type_beside(&mut panel, "Distance", "-1");

    let cut = the_section_on_the_bench(&panel).cutting_plane();
    let normal = cut.normal();
    let frame = the_bench_drawn(&mut panel);
    assert!(
        (normal.dot(DVec3::Z) - 10_f64.to_radians().cos()).abs() < 1e-9,
        "the distance typed turned the tilted section on the top to face {normal:?}",
    );
    assert!(
        (normal.dot(cut.origin - stood) + 1.0).abs() < 1e-9,
        "-1 typed, the tilted cut stands {} from the point {stood:?} of the top along its \
         normal, not -1",
        normal.dot(cut.origin - stood),
    );
    let square = the_square(&frame, cut)
        .unwrap_or_else(|| panic!("no square is drawn on the tilted cut {cut:?}"));
    assert!(
        (square.centre - (stood - normal)).length() < 1e-3,
        "-1 typed, the square stands at {:?}, not 1 under the top along the tilted normal, {:?}",
        square.centre,
        stood - normal,
    );
}

#[test]
fn a_kept_face_section_reopened_from_the_caopart_cuts_where_it_cut_when_kept() {
    let (mut panel, stood) = the_section_panel_on_the_top();
    type_beside(&mut panel, "Inclinaison 2", "10");
    type_beside(&mut panel, "Distance", "-1");
    let live = the_section_on_the_bench(&panel).cutting_plane();
    {
        use egui_kittest::kittest::Queryable;
        panel.get_by_label("Enregistrer la coupe").click();
    }
    panel.run();
    let files = cao_part::InMemoryFiles::default();
    let path = std::path::Path::new("/parts/tilted-on-its-top.caopart");
    panel
        .state()
        .document
        .save(&files, path, Utc::now())
        .expect("the part is saved");

    let reopened = PartDocument::load(&files, path).expect("the part opens again");
    let sections = reopened.sections();
    assert_eq!(sections.len(), 1, "the part came back with {sections:?}");
    let kept = sections[0]
        .cutting_plane()
        .expect("a section whose face stands cuts somewhere");
    let shown = Section::reopened(&sections[0]).cutting_plane();

    let normal = live.normal();
    for (what, plane) in [("the part", kept), ("the section view", shown)] {
        assert!(
            plane.normal().abs_diff_eq(normal, 1e-9)
                && normal.dot(plane.origin - live.origin).abs() < 1e-9,
            "reopened from the .caopart, {what} cuts along {plane:?}, where the section \
             cut along {live:?} when it was kept",
        );
        assert!(
            (normal.dot(plane.origin - stood) + 1.0).abs() < 1e-9,
            "reopened from the .caopart, {what} cuts {} from the point {stood:?} of the top \
             along the tilted normal, not -1",
            normal.dot(plane.origin - stood),
        );
    }
}

/// Whether two planes are one, whichever way each faces.
fn the_same_plane(a: WorkPlane, b: WorkPlane) -> bool {
    a.normal().dot(b.normal()).abs() > 1.0 - 1e-12
        && a.normal().dot(b.origin - a.origin).abs() < 1e-9
}

/// Keeps the section on the bench in the part, as « Enregistrer la coupe » does.
fn keep(panel: &mut egui_kittest::Harness<'_, OnTheBench>) {
    {
        use egui_kittest::kittest::Queryable;
        panel.get_by_label("Enregistrer la coupe").click();
    }
    panel.run();
}

/// The plane the part's first kept section cuts along.
fn the_kept_cut(document: &PartDocument) -> WorkPlane {
    let sections = document.sections();
    assert!(
        sections.len() == 1 && !sections[0].broken,
        "the part keeps {sections:?}, not the one section standing",
    );
    sections[0]
        .cutting_plane()
        .expect("a section that stands cuts somewhere")
}

/// A block 40 by 20 from 0 to 10 over XY.
fn a_slab() -> PartDocument {
    let mut document = PartDocument::new("part", Utc::now());
    document.apply(Operation::CreateSketch {
        plane: WorkPlane::XY,
        on: None,
    });
    document.apply(Operation::AddRectangle {
        sketch: 0,
        corner: PointRef::New(DVec2::ZERO),
        opposite: PointRef::New(DVec2::new(40.0, 20.0)),
        construction: false,
    });
    document.apply(Operation::Extrude {
        sketch: 0,
        areas: document.areas_at(0, &[DVec2::new(20.0, 10.0)]),
        distance: 10.0.into(),
        mode: ExtrusionMode::Add,
    });
    document
}

/// Raises a boss from XY, apart from everything else: its rectangle drawn in
/// a sketch of its own, the next one.
fn raise_a_boss(document: &mut PartDocument, (from, to): (DVec2, DVec2), height: f64) {
    let sketch = document.sketches().len();
    document.apply(Operation::CreateSketch {
        plane: WorkPlane::XY,
        on: None,
    });
    document.apply(Operation::AddRectangle {
        sketch,
        corner: PointRef::New(from),
        opposite: PointRef::New(to),
        construction: false,
    });
    document.apply(Operation::Extrude {
        sketch,
        areas: document.areas_at(sketch, &[(from + to) / 2.0]),
        distance: height.into(),
        mode: ExtrusionMode::Add,
    });
}

/// The slab, a section across it along YZ through its middle tilted 30° and
/// kept, and the cut it showed when it was kept.
fn a_slab_with_a_tilted_section_kept() -> (PartDocument, WorkPlane) {
    let mut panel = the_section_panel_over(a_slab());
    {
        use egui_kittest::kittest::Queryable;
        panel.get_by_label("Plan YZ").click();
    }
    panel.run();
    type_beside(&mut panel, "Inclinaison 1", "30");
    let live = the_section_on_the_bench(&panel).cutting_plane();
    keep(&mut panel);
    let document = panel.state().document.clone();
    assert!(
        the_same_plane(the_kept_cut(&document), live),
        "the part keeps the section cutting along {:?}, where it cut along {live:?} on screen",
        the_kept_cut(&document),
    );
    (document, live)
}

#[test]
fn a_kept_tilted_section_stays_where_it_was_kept_when_matter_is_raised_away_from_it() {
    let (mut document, live) = a_slab_with_a_tilted_section_kept();

    raise_a_boss(
        &mut document,
        (DVec2::new(60.0, 0.0), DVec2::new(70.0, 10.0)),
        40.0,
    );

    let after = the_kept_cut(&document);
    assert!(
        the_same_plane(after, live),
        "a boss raised away from the slab moved its kept section {} along the normal, \
         to {after:?}",
        live.normal().dot(after.origin - live.origin),
    );
}

#[test]
fn a_sketch_laid_on_a_kept_tilted_section_still_lies_on_its_cut_once_matter_is_raised_and_the_part_reopened()
 {
    let (mut document, live) = a_slab_with_a_tilted_section_kept();
    let section = document.sections()[0].id;
    document.apply(Operation::CreateSketchOnSection {
        section,
        plane: live,
    });
    raise_a_boss(
        &mut document,
        (DVec2::new(60.0, 0.0), DVec2::new(70.0, 10.0)),
        40.0,
    );
    let files = cao_part::InMemoryFiles::default();
    let path = std::path::Path::new("/parts/a-sketch-on-a-tilted-section.caopart");
    document
        .save(&files, path, Utc::now())
        .expect("the part is saved");

    let reopened = PartDocument::load(&files, path).expect("the part opens again");

    let cut = the_kept_cut(&reopened);
    let laid = reopened.sketches()[1].plane;
    assert!(
        the_same_plane(cut, laid),
        "reopened, the section cuts along {cut:?} and the sketch laid on it lies on {laid:?}",
    );
}

/// A block 10 by 10 over XY raised as thick as the variable `thickness`, 4.
fn a_block_as_thick_as_a_variable() -> PartDocument {
    let mut document = PartDocument::new("part", Utc::now());
    document.apply(Operation::Variable(cao_part::VariableChange::Added {
        name: "thickness".to_string(),
        formula: cao_part::Formula::Number(f64::from(ON_THE_TOP)),
    }));
    document.apply(Operation::CreateSketch {
        plane: WorkPlane::XY,
        on: None,
    });
    document.apply(Operation::AddRectangle {
        sketch: 0,
        corner: PointRef::New(DVec2::ZERO),
        opposite: PointRef::New(DVec2::new(10.0, 10.0)),
        construction: false,
    });
    let distance = document
        .variables()
        .read("thickness")
        .expect("a formula that reads");
    document.apply(Operation::Extrude {
        sketch: 0,
        areas: document.areas_at(0, &[DVec2::new(5.0, 5.0)]),
        distance,
        mode: ExtrusionMode::Add,
    });
    document
}

#[test]
fn a_kept_tilted_section_on_a_face_follows_the_face_when_its_extrusion_grows_and_not_the_matter_raised_aside()
 {
    let (mut panel, _) = the_section_panel_on_the_top_of(a_block_as_thick_as_a_variable());
    type_beside(&mut panel, "Inclinaison 1", "10");
    type_beside(&mut panel, "Distance", "-1");
    keep(&mut panel);
    let mut document = panel.state().document.clone();
    let kept = the_kept_cut(&document);

    raise_a_boss(
        &mut document,
        (DVec2::new(0.0, 20.0), DVec2::new(5.0, 25.0)),
        2.0,
    );
    document.apply(Operation::Variable(cao_part::VariableChange::Edited {
        variable: cao_part::VariableId(0),
        name: "thickness".to_string(),
        formula: cao_part::Formula::Number(6.0),
    }));

    let grown = the_kept_cut(&document);
    let followed = WorkPlane {
        origin: kept.origin + DVec3::Z * 2.0,
        ..kept
    };
    assert!(
        grown.normal().abs_diff_eq(kept.normal(), 1e-9),
        "the top grown 2 higher, its section turned from {:?} to {:?}",
        kept.normal(),
        grown.normal(),
    );
    assert!(
        the_same_plane(grown, followed),
        "the top grown 2 higher, its tilted section stands {} from where the face took it",
        kept.normal().dot(grown.origin - followed.origin),
    );
}

/// The face of the part lying flat with this normal, `along` it from the origin.
fn the_face_facing(document: &PartDocument, normal: DVec3, along: f64) -> usize {
    (0..200)
        .find(|face| {
            document.body().plane_of(*face).is_some_and(|plane| {
                plane.normal.abs_diff_eq(normal, 1e-9)
                    && plane
                        .corners
                        .iter()
                        .all(|corner| (corner.dot(normal) - along).abs() < 1e-9)
            })
        })
        .unwrap_or_else(|| panic!("the part has no face facing {normal:?} at {along}"))
}

#[test]
fn a_kept_tilted_section_moved_onto_another_face_is_kept_where_it_cut_on_screen() {
    let (mut panel, _) = the_section_panel_on_the_top();
    type_beside(&mut panel, "Inclinaison 1", "10");
    keep(&mut panel);
    {
        let bench = panel.state_mut();
        let reopened = Section::reopened(&bench.document.sections()[0]);
        bench.state.section = Some(reopened);
        let face = the_face_facing(&bench.document, DVec3::X, 10.0);
        let side = bench
            .document
            .body()
            .plane_of(face)
            .expect("the side is flat");
        let section = bench.state.section.as_mut().expect("the section is on");
        section.choose_face(WorkPlane::from_face(&side.corners, side.normal, DVec3::Z));
        section.on = Some(cao_part::history::FaceAnchor { face, up: DVec3::Z });
    }
    panel.run();
    let live = the_section_on_the_bench(&panel).cutting_plane();

    keep(&mut panel);

    let kept = the_kept_cut(&panel.state().document);
    assert!(
        the_same_plane(kept, live),
        "moved onto the side and kept again, the section cuts along {live:?} on screen and \
         along {kept:?} in the part",
    );
}

#[test]
fn a_formula_typed_in_an_angle_field_takes_its_value_as_the_distance_field_does() {
    let mut document = a_block();
    document.apply(Operation::Variable(cao_part::VariableChange::Added {
        name: "slope".to_string(),
        formula: cao_part::Formula::Number(30.0),
    }));
    let mut panel = the_section_panel_over(document);

    for typed in ["15*2", "slope"] {
        type_beside(&mut panel, "Inclinaison 1", "0");
        type_beside(&mut panel, "Inclinaison 1", typed);

        let normal = the_section_on_the_bench(&panel).cutting_plane().normal();
        assert!(
            (normal.dot(DVec3::Z) - 30_f64.to_radians().cos()).abs() < 1e-9,
            "« {typed} » typed in « Inclinaison 1 » left the cut facing {normal:?}, not 30° off XY",
        );
    }
}

#[test]
fn dragging_the_handle_of_a_tilted_plane_moves_it_along_the_tilted_normal_as_far_as_the_part_reaches()
 {
    let document = a_block();
    let body = document.body();
    let mut section = Section::through_the_middle(WorkPlane::XY, body);
    section.tilt_about_u_degrees = 30.0;
    let taken = section.cutting_plane();
    let normal = taken.normal();
    let from = section.handle(body) - taken.u * 50.0;

    assert!(
        section.grab(body, from, taken.u, 0.5),
        "a ray straight through the handle of a tilted plane takes no hold of it",
    );
    section.pull(from + normal * 1.5, taken.u);
    let pulled = section.cutting_plane();
    section.pull(from + normal * 100.0, taken.u);
    let furthest = section.cutting_plane();
    section.let_go();

    assert!(
        pulled.normal().abs_diff_eq(normal, 1e-9)
            && (normal.dot(pulled.origin - taken.origin) - 1.5).abs() < 1e-9,
        "pulled 1.5 along the tilted normal, the cut went to {pulled:?} from {taken:?}",
    );
    let reach = (0..8)
        .map(|corner| {
            let at = DVec3::new(
                if corner & 1 == 0 { 0.0 } else { 10.0 },
                if corner & 2 == 0 { 0.0 } else { 10.0 },
                if corner & 4 == 0 {
                    0.0
                } else {
                    f64::from(ON_THE_TOP)
                },
            );
            normal.dot(at)
        })
        .fold(f64::NEG_INFINITY, f64::max);
    assert!(
        (normal.dot(furthest.origin) - reach).abs() < 1e-9,
        "pulled far along the tilted normal, the cut stops at {}, not where the block ends \
         along it, {reach}",
        normal.dot(furthest.origin),
    );
}

#[test]
fn the_filled_cut_lies_on_the_tilted_plane() {
    let document = a_block();
    let mut section = Section::through_the_middle(WorkPlane::XY, document.body());
    section.offset = f64::from(MID_HEIGHT);
    section.tilt_about_u_degrees = 30.0;
    let cut = section.cutting_plane();
    let mut state = ViewportState::default();
    state.section = Some(section);
    let frame = a_view_of_the_block(&state);

    let filled: f64 = triangles(&frame.scene_solids)
        .into_iter()
        .map(|triangle| triangle.map(at))
        .filter(|corners| {
            corners
                .iter()
                .all(|corner| cut.normal().dot(*corner - cut.origin).abs() <= 1e-2)
        })
        .map(|[a, b, c]| (b - a).cross(c - a).length() / 2.0)
        .sum();

    assert!(
        (filled - 80.0).abs() <= 1e-2,
        "tilted 30° through the middle of the block 10 by 10 by 4, the cut is filled over \
         {filled} on its plane, not 80",
    );
}

#[test]
fn pressing_on_the_arrow_s_tip_changes_neither_the_distance_nor_the_direction() {
    let mut state = sectioned(|section| section.tilt_about_u_degrees = 30.0);
    let document = a_block();
    let body = document.body();
    let (frame, scale) = a_frame_at_its_own_zoom(&state);
    let section = state.section.as_mut().expect("the section is on");
    let (cut, facing, offset) = (section.cutting_plane(), section.facing(), section.offset);
    let centre = the_square(&frame, cut)
        .expect("a square stands on the cutting plane")
        .centre;
    let tip = segments(&frame)
        .into_iter()
        .filter_map(
            |(a, b)| match (a.distance(centre) < 1e-3, b.distance(centre) < 1e-3) {
                (true, false) => Some(b),
                (false, true) => Some(a),
                _ => None,
            },
        )
        .find(|far| (*far - centre).normalize_or_zero().dot(facing) > 1.0 - 1e-4)
        .expect("an arrow stands on the square the way the plane faces");
    let eye = state.camera.eye().as_dvec3();

    section.grab(
        body,
        eye,
        tip - eye,
        scale.world_size_of(crate::screens::viewport::PICK_PIXELS),
    );
    section.pull(eye, tip + facing * 3.0 - eye);
    section.let_go();

    assert_eq!(
        section.offset, offset,
        "pressed on the arrow's tip and dragged, the section moved"
    );
    assert!(
        section.facing().abs_diff_eq(facing, 1e-12),
        "pressed on the arrow's tip, the section turned to face {:?}",
        section.facing(),
    );
}

/// One frame of a part holding a single sketch on `plane`, that sketch open and
/// the view on it, then left as `prepare` says.
fn a_frame_sketching_on(plane: WorkPlane, prepare: impl FnOnce(&mut ViewportState)) -> SceneFrame {
    let mut document = PartDocument::new("part", Utc::now());
    document.apply(Operation::CreateSketch { plane, on: None });
    let mut editor = SketchEditor::default();
    editor.begin_editing(0, plane);
    let mut state = ViewportState::default();
    state.mode = ViewMode::Plane(plane);
    prepare(&mut state);

    a_frame_of(&state, &mut document, &mut editor)
}

/// Wide enough for any honest way of turning the token into the shader's
/// space, and far narrower than the step between two colours of the cube.
const CUBE_COLOUR_TOLERANCE: f32 = 1e-3;

fn amber() -> [f32; 4] {
    let token = crate::ui::theme::tokens::ACCENT_AMBER;
    let channel = |value: u8| f32::from(value) / 255.0;
    srgb(
        channel(token.r()),
        channel(token.g()),
        channel(token.b()),
        1.0,
    )
}

fn is_amber(colour: &[f32; 4]) -> bool {
    colour
        .iter()
        .zip(amber())
        .all(|(channel, wanted)| (channel - wanted).abs() < CUBE_COLOUR_TOLERANCE)
}

/// The colours `face` is drawn in. A triangle of the face lies wholly on its
/// plane, where one of a neighbouring face touches it along an edge at most.
fn colours_on(frame: &SceneFrame, face: CubeFace) -> Vec<[f32; 4]> {
    let on_the_face = |vertex: &&cao_render::Vertex| {
        (Vec3::from(vertex.position) - cube::face_center(face))
            .dot(face.normal())
            .abs()
            < 1e-4
    };
    triangles(&frame.cube_triangles)
        .into_iter()
        .filter(|triangle| triangle.iter().all(&on_the_face))
        .flat_map(|triangle| triangle.map(|vertex| vertex.color))
        .collect()
}

fn faces_with_amber(frame: &SceneFrame) -> Vec<CubeFace> {
    CubeFace::ALL
        .into_iter()
        .filter(|face| colours_on(frame, *face).iter().any(is_amber))
        .collect()
}

fn assert_only_lit(frame: &SceneFrame, face: CubeFace, seen: &str) {
    let colours = colours_on(frame, face);
    assert!(
        !colours.is_empty() && colours.iter().all(is_amber),
        "{seen}: the cube's face {face:?} is drawn in {:?}, it should be amber all over, {:?}",
        colours.first(),
        amber(),
    );
    assert_eq!(
        faces_with_amber(frame),
        vec![face],
        "{seen}: amber belongs to the one face the sketch looks at",
    );
}

#[test]
fn a_sketch_lights_amber_the_face_of_the_cube_it_looks_at() {
    for (plane, face) in [
        (WorkPlane::XY, CubeFace::PlusZ),
        (WorkPlane::XZ, CubeFace::MinusY),
        (WorkPlane::YZ, CubeFace::PlusX),
    ] {
        let frame = a_frame_sketching_on(plane, |_| {});

        assert_only_lit(&frame, face, &format!("a sketch on {plane:?}"));
    }
}

#[test]
fn a_sketch_keeps_its_face_lit_with_the_view_turned_away_from_it() {
    let frame = a_frame_sketching_on(WorkPlane::XY, |state| state.mode = ViewMode::Free);

    assert_only_lit(
        &frame,
        CubeFace::PlusZ,
        "a sketch on XY, the view orbited off it",
    );
}

#[test]
fn a_sketch_on_a_part_s_face_lights_the_cube_face_closest_to_its_normal() {
    let away_from_the_world_origin = DVec3::new(30.0, -12.0, 7.0);

    for (normal, face) in [
        (DVec3::new(0.9, 0.3, 0.1), CubeFace::PlusX),
        (DVec3::new(-0.7, 0.6, 0.3), CubeFace::MinusX),
        (DVec3::new(0.2, 0.8, -0.5), CubeFace::PlusY),
        (DVec3::new(-0.2, -0.9, 0.1), CubeFace::MinusY),
        (DVec3::new(0.3, -0.4, 0.8), CubeFace::PlusZ),
        (DVec3::new(0.1, 0.2, -0.95), CubeFace::MinusZ),
    ] {
        let plane = WorkPlane::from_normal(away_from_the_world_origin, normal);

        let frame = a_frame_sketching_on(plane, |_| {});

        assert_only_lit(
            &frame,
            face,
            &format!("a sketch on a face turned to {normal:?}"),
        );
    }
}

#[test]
fn a_normal_exactly_between_two_faces_lights_the_first_in_their_fixed_order() {
    let k = std::f64::consts::FRAC_1_SQRT_2;

    for (u, v, first, second) in [
        (
            DVec3::Z,
            DVec3::new(k, -k, 0.0),
            CubeFace::PlusX,
            CubeFace::PlusY,
        ),
        (
            DVec3::Z,
            DVec3::new(-k, k, 0.0),
            CubeFace::MinusX,
            CubeFace::MinusY,
        ),
        (
            DVec3::X,
            DVec3::new(0.0, k, -k),
            CubeFace::PlusY,
            CubeFace::PlusZ,
        ),
        (
            DVec3::X,
            DVec3::new(0.0, k, k),
            CubeFace::MinusY,
            CubeFace::PlusZ,
        ),
    ] {
        let plane = WorkPlane {
            origin: DVec3::ZERO,
            u,
            v,
        };
        let towards = |face: CubeFace| plane.normal().dot(face.normal().as_dvec3()).to_bits();
        assert_eq!(
            towards(first),
            towards(second),
            "the fixture wants a normal exactly between {first:?} and {second:?}, got {:?}",
            plane.normal(),
        );

        let frame = a_frame_sketching_on(plane, |_| {});

        assert_only_lit(
            &frame,
            first,
            &format!("a sketch exactly between {first:?} and {second:?}"),
        );
    }
}

#[test]
fn the_cube_is_lit_amber_only_while_a_sketch_is_open() {
    let mut document = a_block();
    let mut state = ViewportState::default();
    state.mode = ViewMode::Plane(WorkPlane::XY);
    let mut editor = SketchEditor::default();
    let lang = Catalogue::french();

    editor.begin_editing(0, WorkPlane::XY);
    let open = a_frame_of(&state, &mut document, &mut editor);
    editor.start_choosing_plane(&lang);
    let choosing_the_next = a_frame_of(&state, &mut document, &mut editor);
    editor.close();
    let closed = a_frame_of(&state, &mut document, &mut editor);
    let free = a_frame(|state| state.mode = ViewMode::Free);
    let on_a_plane_with_no_sketch = a_frame(|state| state.mode = ViewMode::Plane(WorkPlane::XY));

    assert_only_lit(&open, CubeFace::PlusZ, "the sketch open");
    for (seen, frame) in [
        ("choosing the plane of the next sketch", &choosing_the_next),
        ("the sketch closed", &closed),
        ("the free view of a part", &free),
        (
            "a view on a plane with no sketch open",
            &on_a_plane_with_no_sketch,
        ),
    ] {
        assert_eq!(
            faces_with_amber(frame),
            Vec::<CubeFace>::new(),
            "{seen}: no sketch is open, and the cube says one looks at a face",
        );
    }
}

/// Each triangle of the cube by where it stands, so that two frames of the same
/// cube can be read tile against tile.
fn the_cube_by_place(frame: &SceneFrame) -> BTreeMap<[i32; 3], [f32; 4]> {
    triangles(&frame.cube_triangles)
        .into_iter()
        .map(|triangle| {
            let middle = triangle
                .iter()
                .map(|vertex| Vec3::from(vertex.position))
                .sum::<Vec3>()
                / 3.0;
            let place = (middle * 10_000.0).round().as_ivec3().to_array();
            (place, triangle[0].color)
        })
        .collect()
}

#[test]
fn a_hovered_zone_keeps_its_hover_colour_on_the_lit_face() {
    let plain = the_cube_by_place(&a_frame(|_| {}));

    for zone in [
        CubeZone::Face(CubeFace::PlusZ),
        CubeZone::edge(CubeFace::PlusZ, CubeFace::PlusX),
        CubeZone::corner(CubeFace::PlusX, CubeFace::MinusY, CubeFace::PlusZ),
    ] {
        let hovered_alone = the_cube_by_place(&a_frame(|state| state.hovered_zone = Some(zone)));
        let lit = a_frame_sketching_on(WorkPlane::XY, |_| {});
        let lit_and_hovered = a_frame_sketching_on(WorkPlane::XY, |state| {
            state.hovered_zone = Some(zone);
        });

        assert!(
            colours_on(&lit_and_hovered, CubeFace::PlusZ)
                .iter()
                .any(is_amber),
            "with {zone:?} hovered, none of the face the sketch looks at is amber",
        );

        let lit = the_cube_by_place(&lit);
        let drawn = the_cube_by_place(&lit_and_hovered);
        let hovered = |place: &[i32; 3]| hovered_alone[place] != plain[place];
        assert!(
            drawn.keys().any(hovered),
            "the fixture hovers {zone:?} and no tile of the cube changes for it",
        );
        for (place, colour) in &drawn {
            let wanted = if hovered(place) {
                hovered_alone[place]
            } else {
                lit[place]
            };
            assert_eq!(
                *colour, wanted,
                "with {zone:?} hovered on a lit cube, the tile at {place:?} is drawn wrong",
            );
        }
    }
}

#[test]
fn hovering_a_zone_of_the_cube_still_lights_it() {
    let plain = a_frame(|_| {});
    let untouched = the_cube_by_place(&plain);

    for (zone, its_faces) in [
        (CubeZone::Face(CubeFace::PlusZ), vec![CubeFace::PlusZ]),
        (
            CubeZone::edge(CubeFace::PlusZ, CubeFace::PlusX),
            vec![CubeFace::PlusZ, CubeFace::PlusX],
        ),
        (
            CubeZone::corner(CubeFace::PlusX, CubeFace::MinusY, CubeFace::PlusZ),
            vec![CubeFace::PlusX, CubeFace::MinusY, CubeFace::PlusZ],
        ),
    ] {
        let hovered = a_frame(|state| state.hovered_zone = Some(zone));

        for face in CubeFace::ALL {
            let changes = colours_on(&hovered, face) != colours_on(&plain, face);
            assert_eq!(
                changes,
                its_faces.contains(&face),
                "with {zone:?} under the cursor, {face:?} changes colour: {changes}",
            );
        }
        let drawn = the_cube_by_place(&hovered);
        let hover_colours: Vec<[f32; 4]> = drawn
            .iter()
            .filter(|(place, colour)| untouched[*place] != **colour)
            .map(|(_, colour)| *colour)
            .collect();
        assert!(
            !hover_colours.is_empty() && hover_colours.windows(2).all(|two| two[0] == two[1]),
            "with {zone:?} under the cursor, the tiles it lights are drawn in {hover_colours:?}, \
             not in one hover colour",
        );
    }
}

#[test]
fn hovering_a_row_of_a_closed_sketch_lights_it_as_its_click_does() {
    for (mut document, sketch, plane) in [
        (a_block(), 0, WorkPlane::XY),
        (
            a_block_and_a_sketch_never_used(),
            1,
            twenty_over_the_block(),
        ),
    ] {
        let (element, from, to) = the_first_trait(&document, sketch);
        let row = Points::Element { sketch, element };
        let mut nothing_open = SketchEditor::default();
        let mut clicked = ViewportState::default();
        clicked.light(row);
        let by_the_click = a_frame_of(&clicked, &mut document, &mut nothing_open);
        let mut hovered = ViewportState::default();
        hover(&document, &mut nothing_open, &mut hovered, Some(row));
        let by_the_hand = a_frame_of(&hovered, &mut document, &mut nothing_open);

        assert_eq!(
            line_colours(&by_the_hand, plane, from, to),
            line_colours(&by_the_click, plane, from, to),
            "a trait of closed sketch {sketch} hovered in the tree is drawn otherwise \
             than when its row is clicked",
        );
        assert!(
            nothing_open.selection().is_empty(),
            "hovering a row of a closed sketch selected something",
        );
    }
}

#[test]
fn a_row_lit_by_the_hand_goes_out_when_the_hand_leaves_it() {
    let mut document = a_block_and_a_sketch_never_used();
    let plane = twenty_over_the_block();
    let (first, from, to) = the_first_trait(&document, 1);
    let first = Points::Element {
        sketch: 1,
        element: first,
    };
    let (second, _) = document.sketches()[1]
        .live_segments()
        .nth(1)
        .expect("the rectangle has a second trait");
    let second = Points::Element {
        sketch: 1,
        element: Element::Segment(second),
    };
    let mut editor = SketchEditor::default();
    let plain = a_frame_of(&ViewportState::default(), &mut document, &mut editor);
    let mut click_lit = ViewportState::default();
    click_lit.light(first);
    let lit_by_the_click = a_frame_of(&click_lit, &mut document, &mut editor);

    let mut state = ViewportState::default();
    hover(&document, &mut editor, &mut state, Some(first));
    let while_hovered = a_frame_of(&state, &mut document, &mut editor);
    hover(&document, &mut editor, &mut state, None);
    let once_left = a_frame_of(&state, &mut document, &mut editor);
    assert_ne!(
        line_colours(&while_hovered, plane, from, to),
        line_colours(&plain, plane, from, to),
        "a trait hovered in the tree is not lit in the view while the hand is on its row",
    );
    assert_eq!(
        line_colours(&once_left, plane, from, to),
        line_colours(&plain, plane, from, to),
        "a trait hovered in the tree stays lit once the hand has left its row",
    );

    for hovered in [second, first] {
        let mut state = ViewportState::default();
        state.light(first);
        hover(&document, &mut editor, &mut state, Some(hovered));
        hover(&document, &mut editor, &mut state, None);
        let after = a_frame_of(&state, &mut document, &mut editor);
        assert_eq!(
            line_colours(&after, plane, from, to),
            line_colours(&lit_by_the_click, plane, from, to),
            "hovering {hovered:?} then leaving it changed what the click on {first:?} had lit",
        );
    }
}

#[test]
fn a_trait_a_click_lit_stays_lit_while_the_hand_is_on_another_row_of_its_sketch() {
    let mut document = a_block_and_a_sketch_never_used();
    let plane = twenty_over_the_block();
    let drawing = &document.sketches()[1];
    let traits: Vec<(Points, Vec3, Vec3)> = drawing
        .live_segments()
        .take(2)
        .map(|(id, segment)| {
            let [from, to] = [segment.start, segment.end]
                .map(|point| drawing.plane.to_world(drawing.point(point)).as_vec3());
            (
                Points::Element {
                    sketch: 1,
                    element: Element::Segment(id),
                },
                from,
                to,
            )
        })
        .collect();
    let [
        (clicked_row, clicked_from, clicked_to),
        (hovered_row, hovered_from, hovered_to),
    ] = traits[..]
    else {
        panic!("the rectangle has two traits");
    };
    let mut editor = SketchEditor::default();
    let lit_alone = |row: Points, document: &mut PartDocument, editor: &mut SketchEditor| {
        let mut state = ViewportState::default();
        state.light(row);
        a_frame_of(&state, document, editor)
    };
    let clicked_alone = lit_alone(clicked_row, &mut document, &mut editor);
    let hovered_alone = lit_alone(hovered_row, &mut document, &mut editor);

    let mut state = ViewportState::default();
    state.light(clicked_row);
    hover(&document, &mut editor, &mut state, Some(hovered_row));
    let both = a_frame_of(&state, &mut document, &mut editor);

    let on = |frame: &SceneFrame, from, to| line_colours(frame, plane, from, to).0;
    assert_eq!(
        on(&both, clicked_from, clicked_to),
        on(&clicked_alone, clicked_from, clicked_to),
        "the trait a click lit goes out while the hand is on another row of its sketch",
    );
    assert_eq!(
        on(&both, hovered_from, hovered_to),
        on(&hovered_alone, hovered_from, hovered_to),
        "the trait under the hand is not lit while a click lit another of its sketch",
    );
}

/// Where the block stands in the history of `a_block_with_a_hole`.
const THE_BLOCK: usize = 3;
/// Where the cut that bores the hole stands.
const THE_HOLE: usize = 4;

/// A block 10 by 20 raised 4 from XY, and a round cut of radius 2 through
/// it at (5, 10): two volumes, an add and a cut.
fn a_block_with_a_hole() -> PartDocument {
    let mut document = PartDocument::new("part", Utc::now());
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
    document.apply(Operation::AddCircle {
        sketch: 0,
        center: PointRef::New(DVec2::new(5.0, 10.0)),
        radius: 2.0,
        rim: Vec::new(),
        construction: false,
    });
    document.apply(Operation::Extrude {
        sketch: 0,
        areas: document.areas_at(0, &[DVec2::new(1.0, 1.0), DVec2::new(5.0, 10.0)]),
        distance: 4.0.into(),
        mode: ExtrusionMode::Add,
    });
    document.apply(Operation::Extrude {
        sketch: 0,
        areas: document.areas_at(0, &[DVec2::new(5.0, 10.0)]),
        distance: 4.0.into(),
        mode: ExtrusionMode::Cut,
    });
    let steps = document.history.applied_operations();
    assert!(
        matches!(
            steps.get(THE_BLOCK),
            Some(Operation::Extrude {
                mode: ExtrusionMode::Add,
                ..
            })
        ) && matches!(
            steps.get(THE_HOLE),
            Some(Operation::Extrude {
                mode: ExtrusionMode::Cut,
                ..
            })
        ),
        "the fixture wants the block then the hole: {steps:?}",
    );
    document
}

/// How far from the hole's circle a corner of its wall may stand once handed
/// to the GPU in `f32`.
const ON_THE_WALL: f32 = 1e-3;

/// Whether a triangle is a piece of the hole's wall: every corner on the
/// circle of radius 2 round (5, 10), and not all of them at one height, which
/// would be a piece of the top or the bottom round the hole's rim.
fn on_the_hole_s_wall(triangle: &[&cao_render::Vertex; 3]) -> bool {
    let on_the_circle = triangle.iter().all(|corner| {
        let [x, y, _] = corner.position;
        ((x - 5.0).hypot(y - 10.0) - 2.0).abs() <= ON_THE_WALL
    });
    let height = triangle[0].position[2];
    let upright = triangle
        .iter()
        .any(|corner| (corner.position[2] - height).abs() > ON_THE_WALL);
    on_the_circle && upright
}

/// How the hole's wall and the rest of the block are drawn, leaving out the
/// ground its sketch lies on.
fn the_wall_and_the_block_seen(state: &ViewportState) -> (Seen, Seen) {
    let frame = a_frame_of(
        state,
        &mut a_block_with_a_hole(),
        &mut SketchEditor::default(),
    );
    (
        how_drawn(&frame, on_the_hole_s_wall),
        how_drawn(&frame, |triangle| {
            !lies_at(triangle, 0.0) && !on_the_hole_s_wall(triangle)
        }),
    )
}

#[test]
fn a_see_through_cut_draws_the_walls_of_its_hole_see_through_and_leaves_the_block_opaque() {
    let mut state = ViewportState::default();
    assert_eq!(
        the_wall_and_the_block_seen(&state),
        (Seen::Opaque, Seen::Opaque),
        "the fixture wants the wall and the block found, opaque, as (wall, block)",
    );

    state.next_volume_mode(&a_block_with_a_hole(), THE_HOLE);

    assert_eq!(
        the_wall_and_the_block_seen(&state),
        (Seen::SeeThrough, Seen::Opaque),
        "the cut « transparente », as (wall, block)",
    );
}

#[test]
fn a_hidden_volume_draws_none_of_its_faces() {
    let document = a_block_with_a_hole();
    let hidden = |step: usize| {
        let mut state = ViewportState::default();
        state.next_volume_mode(&document, step);
        state.next_volume_mode(&document, step);
        assert_eq!(state.volume_mode(&document, step), MatterInFront::Hidden);
        state
    };

    assert_eq!(
        the_wall_and_the_block_seen(&hidden(THE_HOLE)),
        (Seen::Hidden, Seen::Opaque),
        "the cut invisible, as (wall, block)",
    );
    assert_eq!(
        the_wall_and_the_block_seen(&hidden(THE_BLOCK)),
        (Seen::Opaque, Seen::Hidden),
        "the block invisible, as (wall, block)",
    );
}

/// Where the U stands in the history of `a_block_and_a_u_beside_it`.
const THE_U: usize = 5;

/// How far along Y the section of `a_u_sectioned` cuts: through the U's two
/// arms, above the bar that joins them.
const ACROSS_THE_ARMS: f64 = 5.0;

/// A block 10 by 10 and 4 high at the origin, then a U 6 high over x from 20
/// to 30, its two arms 2 wide standing up along Y from a bar 2 deep: two
/// volumes. The U is the outer rectangle less the notch drawn over it.
fn a_block_and_a_u_beside_it() -> PartDocument {
    let mut document = PartDocument::new("part", Utc::now());
    document.apply(Operation::CreateSketch {
        plane: WorkPlane::XY,
        on: None,
    });
    let rectangle = |document: &mut PartDocument, corner: DVec2, opposite: DVec2| {
        document.apply(Operation::AddRectangle {
            sketch: 0,
            corner: PointRef::New(corner),
            opposite: PointRef::New(opposite),
            construction: false,
        });
    };
    rectangle(&mut document, DVec2::ZERO, DVec2::splat(10.0));
    document.apply(Operation::Extrude {
        sketch: 0,
        areas: document.areas_at(0, &[DVec2::splat(5.0)]),
        distance: 4.0.into(),
        mode: ExtrusionMode::Add,
    });
    rectangle(&mut document, DVec2::new(20.0, 0.0), DVec2::new(30.0, 10.0));
    rectangle(&mut document, DVec2::new(22.0, 2.0), DVec2::new(28.0, 12.0));
    document.apply(Operation::Extrude {
        sketch: 0,
        areas: document.areas_at(0, &[DVec2::new(21.0, 1.0)]),
        distance: 6.0.into(),
        mode: ExtrusionMode::Add,
    });
    assert!(
        matches!(
            document.history.applied_operations().get(THE_U),
            Some(Operation::Extrude { .. })
        ),
        "the fixture wants the U raised at step {THE_U}",
    );
    document
}

/// A view with the section on along XZ, through the U's arms, the side the
/// arms stand on drawn and the side of the bar hidden.
fn a_u_sectioned() -> ViewportState {
    let mut section = Section::new(WorkPlane::XZ);
    section.offset = -ACROSS_THE_ARMS;
    let mut state = ViewportState::default();
    state.section = Some(section);
    state
}

/// Whether every corner of a triangle stands over the U, off the cutting plane
/// where the section's own fill is drawn.
fn of_the_u(triangle: &[&cao_render::Vertex; 3]) -> bool {
    let over = triangle
        .iter()
        .all(|corner| (19.999..=30.001).contains(&corner.position[0]));
    let on_the_cut = triangle
        .iter()
        .all(|corner| (f64::from(corner.position[1]) - ACROSS_THE_ARMS).abs() < 1e-3);
    over && !on_the_cut
}

#[test]
fn a_volume_the_section_parts_is_drawn_in_its_own_state_every_piece_of_it() {
    let mut document = a_block_and_a_u_beside_it();
    let mut state = a_u_sectioned();
    let drawn = shown(&state, &SketchEditor::default(), document.body());
    let renumbered: Vec<usize> = drawn
        .opaque
        .triangles_by_face()
        .into_iter()
        .filter_map(|(triangle, face)| {
            let over_the_u = triangle.iter().all(|corner| corner.x > 19.999);
            face.filter(|face| over_the_u && *face >= document.body().faces_end())
        })
        .collect();
    assert!(
        !renumbered.is_empty(),
        "the fixture wants the section to part a face of the U and number a piece anew",
    );
    state.next_volume_mode(&document, THE_U);
    assert_eq!(
        state.volume_mode(&document, THE_U),
        MatterInFront::SeeThrough
    );

    let frame = a_frame_of(&state, &mut document, &mut SketchEditor::default());

    let opaque = triangles(&frame.scene_solids)
        .into_iter()
        .filter(of_the_u)
        .count();
    let veiled = triangles(&frame.scene_surfaces)
        .into_iter()
        .filter(of_the_u)
        .count();
    assert!(
        veiled > 0,
        "nothing of the see-through U is drawn see-through"
    );
    assert_eq!(
        opaque, 0,
        "{opaque} triangles of the see-through U are drawn opaque, as the whole body is, \
         beside {veiled} drawn see-through",
    );
}
