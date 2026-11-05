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
//!   the `.caopart` is unchanged —
//!   `the_section_view_leaves_the_history_and_the_document_untouched`, in
//!   `crates/app/src/commands/tests.rs`; moving, inverting and pointing the
//!   section are `Section`'s, which is handed no document
//! - "slicing a cube at mid-height gives the expected cross-section, in
//!   `cao_solid`, with no window" —
//!   `a_block_sliced_at_mid_height_leaves_its_footprint_on_the_plane`, in
//!   `crates/solid/tests/the_section_of_an_exact_part_is_filled.rs`; #578 left
//!   only slanted cuts there, so this one is a pin, green from the start

use cao_part::{ExtrusionMode, Operation, PartDocument, PointRef};
use cao_prefs::config::ViewportConfig;
use cao_render::MatterInFront;
use cao_render::camera::{OrbitCamera, ViewTransition};
use cao_sketch::WorkPlane;
use chrono::Utc;
use glam::{DVec2, DVec3, Vec3};

use super::*;
use crate::lang::Catalogue;
use crate::screens::extrusion::ExtrusionState;
use crate::screens::sketch::SketchEditor;
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
