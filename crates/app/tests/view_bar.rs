//! The view bar: the view's own tools gathered at the bottom right of the 3D
//! view — « Recadrer », « Vue en coupe », « Matière devant » and the cursor's
//! coordinates (`docs/redesign/part-screen.md` P10, drawn in
//! `docs/redesign/screens/01-part-sketch-line.png`).
//!
//! Closes #728.
//! The issue carries no "Done when": what follows is its "What it does" and
//! its "What must not break", each with what was decided where the issue left
//! a choice.
//! - « Recadrer » outside a sketch frames the whole part in the free 3D view,
//!   and F fires it there: the direction the part is seen from is kept, the
//!   view aims at the middle of the body's bounds and stands back until they
//!   fit — `f_in_the_free_view_frames_the_whole_block`; the camera itself,
//!   which no driver reads, in `crates/app/src/screens/viewport/state/tests.rs`
//!   by `reframing_outside_a_sketch_frames_the_whole_part_from_where_it_is_seen`
//! - with no volume, « Recadrer » puts back the camera a new part opens with —
//!   no test: in this file, since the driver reads no camera; held in
//!   `crates/app/src/screens/viewport/state/tests.rs` by
//!   `reframing_a_part_with_no_volume_puts_back_the_camera_a_new_part_opens_with`
//! - « Recadrer » is offered in and out of a sketch — no test: in this file;
//!   held in `crates/app/src/screens/ribbon/state/tests.rs` by
//!   `reframe_is_offered_in_and_out_of_a_sketch`, the rule a shortcut is
//!   filtered through as well as a button
//! - F reframes, as today, in a sketch: it faces the plane and frames the
//!   sketch — `f_in_a_sketch_still_frames_its_drawing`; the facing, and the
//!   sketch framed rather than the part, in
//!   `crates/app/src/screens/viewport/state/tests.rs` by
//!   `reframing_in_a_sketch_faces_its_plane_and_frames_its_drawing_rather_than_the_part`
//! - found in review: while a plane is being chosen for a new sketch, no
//!   sketch is open, and « Recadrer » frames the whole part as in the free
//!   view — no test: in this file, since the driver reads no camera; held in
//!   `crates/app/src/screens/viewport/state/tests.rs` by
//!   `reframing_while_a_plane_is_being_chosen_frames_the_whole_part`
//! - `Command::RecenterOnSketch` becomes `Command::Reframe`, its label key and
//!   its hint following — no test: the compiler holds the name, and a profile
//!   saved under the old one is not migrated, as decided for the whole redesign
//! - in a sketch, the bar's « Matière devant » offers « cachée »,
//!   « transparente » and « opaque », and picking one sets the open sketch's
//!   matter in front to it directly rather than cycling —
//!   `choosing_see_through_in_the_bar_s_menu_makes_the_matter_in_front_see_through`;
//!   the three offered in order, in
//!   `crates/app/src/screens/view_bar/state/tests.rs` by
//!   `the_matter_in_front_menu_offers_hidden_see_through_and_opaque_in_that_order`;
//!   the state picked set from whichever one it was, in
//!   `crates/app/src/screens/sketch/tests.rs` by
//!   `picking_a_state_of_the_matter_in_front_sets_the_open_sketch_to_it_whatever_it_was`
//! - the bar's label reads « Matière devant : cachée ▾ » after the current
//!   state — `the_bar_s_matter_in_front_reads_the_state_the_sketch_is_in`; the
//!   label and the state marked among the three, in
//!   `crates/app/src/screens/view_bar/state/tests.rs` by
//!   `the_matter_in_front_menu_reads_and_marks_the_state_the_open_sketch_is_in`.
//!   The ▾ is read only if it is written in the button's name: drawn as an
//!   icon, it is seen in the app
//! - outside a sketch the bar's « Matière devant » is greyed, as the ribbon's
//!   is today — `outside_a_sketch_the_bar_s_matter_in_front_is_greyed`; the
//!   rule, in `crates/app/src/screens/view_bar/state/tests.rs` by
//!   `outside_a_sketch_the_matter_in_front_menu_is_greyed`
//! - found in review: a pick from the bar's « Matière devant » menu is a
//!   command like any other, clearing the message in the pill and giving an
//!   error its way out, as the ribbon's does — no test: in this file; held in
//!   `crates/app/tests/title_bar.rs` by
//!   `a_pick_from_the_view_bar_s_matter_in_front_clears_the_message_as_a_command_does`
//!   and `an_error_gives_way_to_a_pick_from_the_view_bar_s_matter_in_front`
//! - found in review: the bar's « Matière devant » wears the icon of the state
//!   the sketch is in, as the ribbon's does — no test: in this file, whose tree
//!   carries no icon; held in `crates/app/src/screens/view_bar/state/tests.rs`
//!   by `the_matter_in_front_menu_wears_the_icon_of_the_state_the_sketch_is_in`
//! - the ribbon's own « Matière devant » stays and keeps cycling — no test: in
//!   this file; held in `crates/app/src/commands/tests.rs` by
//!   `the_matter_in_front_button_moves_the_open_sketch_to_the_next_state`,
//!   left as it is
//! - in a sketch the coordinates read « X 12,50 Y -3,00 mm », the point in the
//!   sketch plane's own frame, and follow the mouse: the point a click would
//!   place, pulled onto the origin by a line tool aiming near it, else the raw
//!   point the selection tool points at —
//!   `in_a_sketch_the_bar_s_x_and_y_follow_the_mouse`; the text, in
//!   `crates/app/src/screens/view_bar/state/tests.rs` by
//!   `on_a_sketch_s_plane_the_coordinates_read_x_and_y_to_two_decimals_after_a_comma`,
//!   and which point is read, by
//!   `in_a_sketch_the_coordinates_are_where_a_click_would_place_the_point`
//! - in the free view they read « X … Y … Z … mm », the world coordinates of
//!   the first point of the drawn body the cursor's ray meets, nothing when the
//!   body is hidden — `hovering_the_block_in_the_free_view_reads_its_x_y_and_z`;
//!   the text and the choice of the part's point outside a sketch, in
//!   `crates/app/src/screens/view_bar/state/tests.rs` by
//!   `on_the_part_the_coordinates_read_x_y_and_z` and
//!   `outside_a_sketch_the_coordinates_are_the_point_of_the_part_under_the_cursor`;
//!   the point met, the hidden body and the section's cut, in
//!   `crates/app/src/screens/viewport/state/tests.rs` by
//!   `the_point_of_the_part_under_a_ray_is_the_first_the_ray_meets`,
//!   `a_hidden_body_has_no_point_under_a_ray` and
//!   `with_the_section_view_on_the_point_under_a_ray_is_on_the_cut`
//! - « — » over nothing, and when the cursor is not over the view —
//!   `off_the_part_and_off_the_view_the_coordinates_read_a_dash`, and in a
//!   sketch by `in_a_sketch_the_bar_s_x_and_y_follow_the_mouse`; in the
//!   presenter, in `crates/app/src/screens/view_bar/state/tests.rs` by
//!   `over_nothing_the_coordinates_read_a_dash` and
//!   `in_a_sketch_with_the_cursor_off_the_view_the_coordinates_point_at_nothing`
//! - two decimals, a comma for the decimal separator, the view's length unit
//!   after them, converted as the scale bar converts, and never « -0,00 » — no
//!   test: in this file; held in
//!   `crates/app/src/screens/view_bar/state/tests.rs` by
//!   `a_coordinate_a_hair_below_zero_reads_zero_with_no_sign` and
//!   `the_coordinates_are_said_in_the_view_s_unit_at_the_part_s_scale`
//! - the coordinates are written in the monospace family, JetBrains Mono — no
//!   test: a font is chosen where the bar is drawn, and neither the presenter
//!   nor the tree the driver reads carries one; it is seen in the app
//! - the bar stands at the bottom right of the view, inside it, and holds, in
//!   order, « Recadrer » with its key F, « Vue en coupe », « Matière devant : …
//!   ▾ », a separator, the coordinates —
//!   `the_bar_stands_at_the_bottom_right_of_the_view_in_the_design_s_order`;
//!   the separator is a line the tree does not carry, and is seen in the app
//! - on the floating background, `BACKGROUND_FLOATING` — no test: a colour of
//!   the frame, which neither the presenter nor the tree the driver reads
//!   carries; it is seen in the app
//! - « Recadrer » in the bar does what F does —
//!   `the_bar_s_reframe_frames_the_whole_block_as_f_does`
//! - each button follows its command's rule: « Recadrer » offered with a
//!   volume and without, « Vue en coupe » greyed in a sketch and with no
//!   volume — `the_bar_s_section_view_is_greyed_in_a_sketch_and_with_no_volume`;
//!   shown pressed while a section is on —
//!   `the_bar_s_section_view_shows_pressed_while_the_section_is_on`
//! - the section view panel opens and closes from the bar as it did from the
//!   ribbon —
//!   `the_bar_s_section_view_opens_and_closes_the_section_panel_as_the_ribbon_s_does`
//! - found on the branch: the bar does not jitter as the mouse moves — its
//!   buttons keep their place, within a point, whatever length the reading
//!   takes, from « — » to a full one and back, in a sketch —
//!   `in_a_sketch_the_bar_s_buttons_keep_their_place_whatever_the_coordinates_read`
//!   — and in the free view —
//!   `in_the_free_view_the_bar_s_buttons_keep_their_place_whatever_the_coordinates_read`
//! - found on the branch: the readings missed their second decimal by a hair,
//!   the ray under the cursor passing a tenth of a pixel off the point drawn
//!   there — no test: in this file; held in
//!   `crates/render/src/camera/tests.rs` by
//!   `the_ray_through_the_pixel_a_point_is_drawn_at_passes_through_that_point`
//! - the bar's buttons are found as the bar's, and the ribbon's « Recadrer »,
//!   « Vue en coupe » and « Matière devant », which stay, are still found as the
//!   ribbon's — `the_bar_s_buttons_are_told_apart_from_the_ribbon_s`; the
//!   driver tells them apart by where they stand, the bar's inside the view
//!   and the ribbon's outside it (`driver::the_bar_s`, `driver::the_ribbon_s`),
//!   and the tests aimed at the ribbon's go through the latter
//! - the scale bar keeps its corner and its settings: at its default corner,
//!   bottom left, it does not move; nor does the orientation cube at its
//!   default corner, top right, nor either of them at a top corner or at the
//!   bottom left — `the_scale_bar_at_its_default_corner_stands_where_it_stood`
//! - set to the bottom right, the scale bar, or the orientation cube, stays in
//!   that corner, against the right edge as before, and is lifted above the
//!   bar, so that the two do not overlap —
//!   `a_scale_bar_set_to_the_bottom_right_stands_above_the_view_bar`,
//!   `an_orientation_cube_set_to_the_bottom_right_stands_above_the_view_bar`
//!
//! The scale bar and the cube are painted, not laid out as widgets, so the
//! tree carries neither: they are read off what the last frame painted. The
//! scale bar is its three strokes, the baseline and its two ticks, in the
//! secondary ink; the cube is the rectangle its face labels are clipped to,
//! which is the cube's own; the bar is the filled frame around what it holds.
//! The redesign of the cube (P11) may paint it otherwise, and `the_cube` will
//! have to follow.
//!
//! What « fills the view » is held to: every corner of what is framed stands
//! inside the view, and it covers at least a third of the view across or up,
//! whichever it covers more of. Framing a sphere around the bounds, as the
//! sketch has always been framed, leaves a long flat drawing at about that;
//! a view that did not move after the wheel stepped back leaves the part a
//! speck.
//!
//! Closes #729. The rest of the issue is in
//! `crates/app/src/screens/viewport/render/tests.rs`, `render/overlays/tests.rs`
//! and `state/tests.rs`, each under its own block.
//! - « Vue d'origine » under the cube: the isometric view the part opens with,
//!   framed on the part — the button under the cube, following its settings —
//!   `the_origin_view_button_stands_under_the_cube_and_frames_the_part`; decided
//!   on the branch: above the cube when it sits in a bottom corner —
//!   `the_origin_view_button_goes_above_a_cube_in_a_bottom_corner`; decided by
//!   the human: hidden while a sketch is open —
//!   `the_origin_view_button_is_hidden_while_a_sketch_is_open`
//! - Clicking a face, an edge or a corner turns the camera as today; hovering
//!   lights it: the clicking half —
//!   `clicking_a_face_an_edge_or_a_corner_of_the_cube_still_turns_the_camera`;
//!   the hovering half in `crates/app/src/screens/viewport/render/tests.rs`,
//!   under its own block

mod driver;

use cao_app::theme::tokens;
use cao_prefs::config::ViewportCorner;
use egui_kittest::kittest::{NodeT, Queryable};
use glam::{DVec2, DVec3};

/// How much of the view a framed part covers at the least, across or up.
const FILLS_THE_VIEW: f32 = 1.0 / 3.0;

/// How much of the view a part left a speck covers at the most.
const A_SPECK: f32 = 0.1;

/// How far the middle of what is framed may stand from the middle of the view,
/// as a share of the view's height: the block's corners are the points
/// clicked, which the grid may have pulled a little.
const OFF_THE_MIDDLE: f32 = 0.05;

/// What the view does with a part: how much of the view it covers, across or
/// up, whichever is more — one from edge to edge — and how far its middle
/// stands from the view's, as a share of the view's height.
struct Framed {
    covers: f32,
    off_the_middle: f32,
}

/// How the view frames these places of the screen, every one of which has to
/// stand inside it.
fn framed(app: &driver::App, corners: &[(String, egui::Pos2)], middle: egui::Pos2) -> Framed {
    let view = driver::canvas(app);
    for (what, at) in corners {
        assert!(
            view.contains(*at),
            "{what} lands at {at:?}, outside the view {view:?}"
        );
    }
    let covered = egui::Rect::from_points(&corners.iter().map(|(_, at)| *at).collect::<Vec<_>>());
    Framed {
        covers: (covered.width() / view.width()).max(covered.height() / view.height()),
        off_the_middle: (middle - view.center()).length() / view.height(),
    }
}

fn of_the_world(app: &driver::App, point: DVec3) -> egui::Pos2 {
    app.state()
        .where_the_view_shows(point)
        .unwrap_or_else(|| panic!("{point:?} of the part stands behind the eye"))
}

fn of_the_sketch(app: &driver::App, point: DVec2) -> egui::Pos2 {
    app.state()
        .where_the_view_shows_the_sketch_point(point)
        .unwrap_or_else(|| panic!("{point:?} of the sketch stands behind the eye"))
}

/// The block `driver::a_block` raises: the rectangle drawn on YZ through the
/// points it clicked, raised 10 mm along X.
fn the_block_s_corners() -> Vec<DVec3> {
    let [one, other] = driver::THE_BLOCK_S_CORNERS;
    let mut corners = Vec::new();
    for x in [0.0, 10.0] {
        for (y, z) in [
            (one.x, one.y),
            (one.x, other.y),
            (other.x, one.y),
            (other.x, other.y),
        ] {
            corners.push(DVec3::new(x, y, z));
        }
    }
    corners
}

fn the_block_s_middle() -> DVec3 {
    DVec3::new(5.0, driver::ON_THE_BLOCK.x, driver::ON_THE_BLOCK.y)
}

fn how_the_view_frames_the_block(app: &driver::App) -> Framed {
    let corners: Vec<(String, egui::Pos2)> = the_block_s_corners()
        .into_iter()
        .map(|corner| {
            (
                format!("the block's corner {corner:?}"),
                of_the_world(app, corner),
            )
        })
        .collect();
    framed(app, &corners, of_the_world(app, the_block_s_middle()))
}

/// The mouse wheel turned towards the user over this place, thirty notches at
/// once, as a hand stepping well back from the part does.
fn step_back(app: &mut driver::App, over: egui::Pos2) {
    app.hover_at(over);
    app.run();
    app.event(egui::Event::MouseWheel {
        unit: egui::MouseWheelUnit::Line,
        delta: egui::vec2(0.0, -30.0),
        phase: egui::TouchPhase::Move,
        modifiers: egui::Modifiers::NONE,
    });
    app.run();
    driver::hold_the_canvas(app);
}

#[test]
fn f_in_the_free_view_frames_the_whole_block() {
    let mut app = driver::a_block("f_in_the_free_view_frames_the_whole_block");
    let over_the_block = of_the_world(&app, the_block_s_middle());
    step_back(&mut app, over_the_block);
    let stepped_back = how_the_view_frames_the_block(&app);
    assert!(
        stepped_back.covers < A_SPECK,
        "the wheel was meant to leave the block a speck, and it covers {} of the view",
        stepped_back.covers,
    );

    driver::press(&mut app, egui::Key::F);

    let reframed = how_the_view_frames_the_block(&app);
    assert!(
        reframed.covers >= FILLS_THE_VIEW,
        "F pressed in the free view leaves the block covering {} of the view, \
         where the whole part should fill it",
        reframed.covers,
    );
    assert!(
        reframed.off_the_middle <= OFF_THE_MIDDLE,
        "once F is pressed, the block's middle stands {} of the view's height \
         from the middle of the view",
        reframed.off_the_middle,
    );
}

#[test]
fn f_in_a_sketch_still_frames_its_drawing() {
    let mut app = driver::open("f_in_a_sketch_still_frames_its_drawing");
    driver::create_a_part(&mut app, "Platine");
    driver::start_a_sketch(&mut app);
    driver::draw_a_segment(&mut app);
    let the_segment_and_the_origin = [
        DVec2::ZERO,
        driver::A_POINT_ABOVE_THE_ORIGIN,
        driver::A_POINT_BELOW_AND_RIGHT,
    ];
    let (low, high) = the_segment_and_the_origin
        .iter()
        .fold((DVec2::ZERO, DVec2::ZERO), |(low, high), point| {
            (low.min(*point), high.max(*point))
        });
    let how_the_view_frames_the_drawing = |app: &driver::App| {
        let corners: Vec<(String, egui::Pos2)> = [
            low,
            DVec2::new(low.x, high.y),
            high,
            DVec2::new(high.x, low.y),
        ]
        .into_iter()
        .map(|corner| {
            (
                format!("the drawing's corner {corner:?}"),
                of_the_sketch(app, corner),
            )
        })
        .collect();
        framed(app, &corners, of_the_sketch(app, (low + high) * 0.5))
    };
    let over_the_segment = of_the_sketch(&app, driver::ON_THE_SEGMENT);
    step_back(&mut app, over_the_segment);
    let stepped_back = how_the_view_frames_the_drawing(&app);
    assert!(
        stepped_back.covers < A_SPECK,
        "the wheel was meant to leave the drawing a speck, and it covers {} of the view",
        stepped_back.covers,
    );

    driver::press(&mut app, egui::Key::F);

    let reframed = how_the_view_frames_the_drawing(&app);
    assert!(
        reframed.covers >= FILLS_THE_VIEW,
        "F pressed in a sketch leaves its drawing covering {} of the view",
        reframed.covers,
    );
    assert!(
        reframed.off_the_middle <= OFF_THE_MIDDLE,
        "once F is pressed in a sketch, the middle of its drawing stands {} of the \
         view's height from the middle of the view",
        reframed.off_the_middle,
    );
}

/// How far, in points, the bar's texts may stand from the view's right edge
/// and from its bottom: the bar's margin and its padding, with room to spare.
const AGAINST_THE_CORNER: f32 = 48.0;

/// A hair off the origin of a sketch, within reach of the pull a drawing tool
/// feels from the point there.
const NEAR_THE_ORIGIN: DVec2 = DVec2::new(1.2, -0.8);

/// On the face of the block `driver::a_block` turns towards the eye, the
/// one 10 mm out along X, in its middle.
const ON_THE_BLOCK_S_FACE: DVec3 = DVec3::new(10.0, 57.5, -4.05);

/// Beyond the block's upper corner and above it, where the ray from the eye
/// passes over the block and meets nothing of it.
const BESIDE_THE_BLOCK: DVec3 = DVec3::new(5.0, 140.0, 60.0);

/// A part named « Platine », a sketch opened on its plane YZ.
fn a_sketch_open(test: &str) -> driver::App {
    let mut app = driver::open(test);
    driver::create_a_part(&mut app, "Platine");
    driver::start_a_sketch(&mut app);
    app
}

fn the_bar_reads(app: &driver::App) -> Vec<String> {
    driver::the_view_bar(app)
        .into_iter()
        .map(|(text, _)| text)
        .collect()
}

/// Whether every one of these words is read, each after the one before it.
fn in_this_order(read: &str, words: &[&str]) -> bool {
    let mut from = 0;
    for word in words {
        match read[from..].find(word) {
            Some(at) => from += at + word.len(),
            None => return false,
        }
    }
    true
}

/// How far the bar's texts stand from the view's right edge and from its
/// bottom.
fn gaps_to_the_view_s_corner(app: &driver::App) -> (f32, f32) {
    let view = driver::canvas(app);
    let held = driver::the_view_bar(app)
        .into_iter()
        .fold(egui::Rect::NOTHING, |held, (_, rect)| held.union(rect));
    (view.max.x - held.max.x, view.max.y - held.max.y)
}

fn greyed(node: &egui_kittest::Node<'_>) -> bool {
    node.accesskit_node().is_disabled()
}

fn pressed(node: &egui_kittest::Node<'_>) -> bool {
    node.accesskit_node().toggled() == Some(egui::accesskit::Toggled::True)
}

fn the_section_panel_is_open(app: &driver::App) -> bool {
    driver::a_button_reads(app, "Inverser")
}

fn the_bar_s_matter_in_front(app: &driver::App) -> String {
    driver::the_bar_s(app, "Matière devant")
        .accesskit_node()
        .label()
        .unwrap_or_default()
}

fn hover_over_the_world(app: &mut driver::App, point: DVec3) {
    let at = driver::world_point_on_screen(app, point);
    app.hover_at(at);
    app.run();
}

/// The pointer left over the ribbon, off the view.
fn hover_off_the_view(app: &mut driver::App) {
    let off_the_view = driver::the_ribbon_s(app, "Nouvelle esquisse")
        .expect("the ribbon offers « Nouvelle esquisse »")
        .rect()
        .center();
    app.hover_at(off_the_view);
    app.run();
}

#[test]
fn the_bar_stands_at_the_bottom_right_of_the_view_in_the_design_s_order() {
    let app = a_sketch_open("the_bar_stands_at_the_bottom_right_of_the_view");

    let read = the_bar_reads(&app);
    assert!(
        in_this_order(
            &read.join(" | "),
            &["Recadrer", "F", "Vue en coupe", "Matière devant : cachée"],
        ),
        "the bar reads {read:?}, not « Recadrer » with its key F, « Vue en coupe » and \
         « Matière devant : cachée », in that order",
    );
    let last = read.last().cloned().unwrap_or_default();
    assert!(
        last.starts_with("X ") && last.ends_with(" mm"),
        "the bar ends on {last:?}, not on the coordinates of the cursor left in the sketch",
    );
    let (right, below) = gaps_to_the_view_s_corner(&app);
    assert!(
        right <= AGAINST_THE_CORNER && below <= AGAINST_THE_CORNER,
        "the bar stands {right} points from the view's right edge and {below} from its \
         bottom, not against its bottom-right corner",
    );
}

#[test]
fn the_bar_s_reframe_frames_the_whole_block_as_f_does() {
    let mut app = driver::a_block("the_bar_s_reframe_frames_the_whole_block");
    let over_the_block = of_the_world(&app, the_block_s_middle());
    step_back(&mut app, over_the_block);
    let stepped_back = how_the_view_frames_the_block(&app);
    assert!(
        stepped_back.covers < A_SPECK,
        "the wheel was meant to leave the block a speck, and it covers {} of the view",
        stepped_back.covers,
    );

    driver::click_in_the_bar(&mut app, "Recadrer");

    let reframed = how_the_view_frames_the_block(&app);
    assert!(
        reframed.covers >= FILLS_THE_VIEW,
        "« Recadrer » clicked in the bar leaves the block covering {} of the view",
        reframed.covers,
    );
    assert!(
        reframed.off_the_middle <= OFF_THE_MIDDLE,
        "once « Recadrer » is clicked in the bar, the block's middle stands {} of the \
         view's height from the middle of the view",
        reframed.off_the_middle,
    );
}

#[test]
fn the_bar_s_section_view_is_greyed_in_a_sketch_and_with_no_volume() {
    let mut app = driver::open("the_bar_s_section_view_is_greyed_with_no_volume");
    driver::create_a_part(&mut app, "Platine");
    assert!(
        greyed(&driver::the_bar_s(&app, "Vue en coupe")),
        "the bar offers « Vue en coupe » on a part with no volume",
    );
    assert!(
        !greyed(&driver::the_bar_s(&app, "Recadrer")),
        "the bar greys « Recadrer » on a part with no volume",
    );

    let mut app = driver::a_block("the_bar_s_section_view_is_greyed_in_a_sketch");
    assert!(
        !greyed(&driver::the_bar_s(&app, "Vue en coupe")),
        "the bar greys « Vue en coupe » on a part that holds a volume",
    );
    driver::start_a_sketch(&mut app);
    assert!(
        driver::is_enabled(&app, driver::FINISH_IN_THE_RIBBON),
        "no sketch was opened on the block, showing {:?}",
        driver::on_screen(&app),
    );
    assert!(
        greyed(&driver::the_bar_s(&app, "Vue en coupe")),
        "the bar offers « Vue en coupe » while a sketch is edited",
    );
    assert!(
        !greyed(&driver::the_bar_s(&app, "Recadrer")),
        "the bar greys « Recadrer » while a sketch is edited",
    );
}

#[test]
fn the_bar_s_section_view_shows_pressed_while_the_section_is_on() {
    let mut app = driver::a_block("the_bar_s_section_view_shows_pressed");
    assert!(
        !pressed(&driver::the_bar_s(&app, "Vue en coupe")),
        "the bar's « Vue en coupe » shows pressed before any section is on",
    );

    driver::click_in_the_ribbon(&mut app, "Vue en coupe");
    assert!(
        pressed(&driver::the_bar_s(&app, "Vue en coupe")),
        "the section is on, and the bar's « Vue en coupe » does not show pressed",
    );

    driver::click_in_the_ribbon(&mut app, "Vue en coupe");
    assert!(
        !pressed(&driver::the_bar_s(&app, "Vue en coupe")),
        "the section is off, and the bar's « Vue en coupe » still shows pressed",
    );
}

#[test]
fn the_bar_s_section_view_opens_and_closes_the_section_panel_as_the_ribbon_s_does() {
    let mut app = driver::a_block("the_bar_s_section_view_opens_and_closes_the_panel");
    assert!(!the_section_panel_is_open(&app), "a panel is open already");

    driver::click_in_the_bar(&mut app, "Vue en coupe");
    assert!(
        the_section_panel_is_open(&app),
        "« Vue en coupe » in the bar opens no panel, showing {:?}",
        driver::on_screen(&app),
    );
    assert!(
        driver::the_ribbon_s(&app, "Vue en coupe").is_some_and(|node| pressed(&node)),
        "the bar turned the section on, and the ribbon's « Vue en coupe » does not show it",
    );

    driver::click_in_the_bar(&mut app, "Vue en coupe");
    assert!(
        !the_section_panel_is_open(&app),
        "« Vue en coupe » clicked again in the bar leaves its panel open",
    );

    driver::click_in_the_ribbon(&mut app, "Vue en coupe");
    assert!(
        the_section_panel_is_open(&app),
        "the ribbon's « Vue en coupe » opens no panel any more",
    );
    driver::click_in_the_bar(&mut app, "Vue en coupe");
    assert!(
        !the_section_panel_is_open(&app),
        "the bar's « Vue en coupe » leaves open the panel the ribbon's opened",
    );
}

#[test]
fn the_bar_s_buttons_are_told_apart_from_the_ribbon_s() {
    let mut app = a_sketch_open("the_bar_s_buttons_are_told_apart_from_the_ribbon_s");

    for label in ["Recadrer", "Vue en coupe", "Matière devant"] {
        driver::bring_into_the_ribbon(&mut app, label);
        let ribbon = driver::the_ribbon_s(&app, label)
            .unwrap_or_else(|| panic!("the ribbon no longer offers its own « {label} »"))
            .rect();
        let bar = driver::the_bar_s(&app, label).rect();
        assert_ne!(
            ribbon, bar,
            "the ribbon's « {label} » and the bar's are found as one button",
        );
    }
}

#[test]
fn choosing_see_through_in_the_bar_s_menu_makes_the_matter_in_front_see_through() {
    let mut app = a_sketch_open("choosing_see_through_in_the_bar_s_menu");

    driver::click_in_the_bar(&mut app, "Matière devant");
    let offered: Vec<&str> = ["cachée", "transparente", "opaque"]
        .into_iter()
        .filter(|choice| app.query_all_by_label(choice).next().is_some())
        .collect();
    assert_eq!(
        offered,
        ["cachée", "transparente", "opaque"],
        "the bar's « Matière devant » opens no menu of the three, showing {:?}",
        driver::on_screen(&app),
    );
    driver::click_exactly(&mut app, "transparente");

    let reads = the_bar_s_matter_in_front(&app);
    assert!(
        reads.starts_with("Matière devant : transparente"),
        "« transparente » was picked, and the bar reads {reads:?}",
    );
    assert!(
        driver::the_ribbon_s(&app, "Matière devant : transparente").is_some(),
        "the bar reads « transparente », and the ribbon's button, which reads the \
         sketch's own state, does not: {:?}",
        driver::on_screen(&app),
    );
}

#[test]
fn the_bar_s_matter_in_front_reads_the_state_the_sketch_is_in() {
    let mut app = a_sketch_open("the_bar_s_matter_in_front_reads_the_state");
    let opened = the_bar_s_matter_in_front(&app);
    assert!(
        opened.starts_with("Matière devant : cachée"),
        "a sketch just opened hides the matter in front of it, and the bar reads {opened:?}",
    );

    driver::click_in_the_ribbon(&mut app, "Matière devant");
    driver::click_in_the_ribbon(&mut app, "Matière devant");

    let reads = the_bar_s_matter_in_front(&app);
    assert!(
        reads.starts_with("Matière devant : opaque"),
        "the ribbon's button took the sketch on to « opaque », and the bar reads {reads:?}",
    );
}

#[test]
fn outside_a_sketch_the_bar_s_matter_in_front_is_greyed() {
    let mut app = driver::open("outside_a_sketch_the_bar_s_matter_in_front_is_greyed");
    driver::create_a_part(&mut app, "Platine");
    assert!(
        greyed(&driver::the_bar_s(&app, "Matière devant")),
        "the bar offers « Matière devant » before a sketch is opened",
    );

    driver::start_a_sketch(&mut app);
    assert!(
        !greyed(&driver::the_bar_s(&app, "Matière devant")),
        "the bar greys « Matière devant » while a sketch is open",
    );

    driver::click_in_the_ribbon(&mut app, driver::FINISH_IN_THE_RIBBON);
    assert!(
        greyed(&driver::the_bar_s(&app, "Matière devant")),
        "the bar offers « Matière devant » once the sketch is finished",
    );
}

#[test]
fn in_a_sketch_the_bar_s_x_and_y_follow_the_mouse() {
    let mut app = a_sketch_open("in_a_sketch_the_bar_s_x_and_y_follow_the_mouse");
    driver::click(&mut app, "Ligne (L)");

    driver::hover_at_sketch_point(&mut app, driver::A_POINT_BELOW_AND_RIGHT);
    assert_eq!(
        driver::the_bar_s_coordinates(&app),
        "X 137,50 Y -37,50 mm",
        "the line tool over a node of the grid",
    );

    driver::hover_at_sketch_point(&mut app, NEAR_THE_ORIGIN);
    assert_eq!(
        driver::the_bar_s_coordinates(&app),
        "X 0,00 Y 0,00 mm",
        "the line tool near the origin aims at it, and the bar reads where the cursor is",
    );

    driver::click(&mut app, "Sélection (S)");
    driver::hover_at_sketch_point(&mut app, NEAR_THE_ORIGIN);
    assert_eq!(
        driver::the_bar_s_coordinates(&app),
        "X 1,20 Y -0,80 mm",
        "the selection tool only points, and the bar reads a place it was not pulled to",
    );

    hover_off_the_view(&mut app);
    assert_eq!(
        driver::the_bar_s_coordinates(&app),
        "—",
        "the pointer left the view, and the bar still reads a place in the sketch",
    );
}

#[test]
fn hovering_the_block_in_the_free_view_reads_its_x_y_and_z() {
    let mut app = driver::a_block("hovering_the_block_in_the_free_view_reads_its_x_y_and_z");

    hover_over_the_world(&mut app, ON_THE_BLOCK_S_FACE);

    assert_eq!(
        driver::the_bar_s_coordinates(&app),
        "X 10,00 Y 57,50 Z -4,05 mm"
    );
}

#[test]
fn off_the_part_and_off_the_view_the_coordinates_read_a_dash() {
    let mut app = driver::a_block("off_the_part_and_off_the_view_the_coordinates_read_a_dash");

    hover_over_the_world(&mut app, BESIDE_THE_BLOCK);
    assert_eq!(
        driver::the_bar_s_coordinates(&app),
        "—",
        "the pointer stands beside the block",
    );

    hover_over_the_world(&mut app, ON_THE_BLOCK_S_FACE);
    let on_the_block = driver::the_bar_s_coordinates(&app);
    assert!(
        on_the_block.starts_with("X "),
        "over the block, the bar reads {on_the_block:?}",
    );

    hover_off_the_view(&mut app);
    assert_eq!(
        driver::the_bar_s_coordinates(&app),
        "—",
        "the pointer left the view, and the bar still reads the block",
    );
}

/// How far, in points, a button of the bar may stray while the reading beside
/// it changes length.
const KEEPS_ITS_PLACE: f32 = 1.0;

/// Where the bar's three buttons stand.
fn the_bar_s_buttons(app: &driver::App) -> [egui::Rect; 3] {
    ["Recadrer", "Vue en coupe", "Matière devant"].map(|label| driver::the_bar_s(app, label).rect())
}

/// Checks the bar's buttons stand where they stood while the coordinates read
/// what they read now.
fn assert_in_their_place(app: &driver::App, stood: [egui::Rect; 3]) {
    let reading = driver::the_bar_s_coordinates(app);
    for (now, then) in the_bar_s_buttons(app).into_iter().zip(stood) {
        assert!(
            (now.min - then.min).length() <= KEEPS_ITS_PLACE,
            "with the coordinates reading {reading:?}, a button of the bar moved from \
             {then:?} to {now:?}",
        );
    }
}

#[test]
fn in_a_sketch_the_bar_s_buttons_keep_their_place_whatever_the_coordinates_read() {
    let mut app = a_sketch_open("in_a_sketch_the_bar_s_buttons_keep_their_place");
    hover_off_the_view(&mut app);
    assert_eq!(driver::the_bar_s_coordinates(&app), "—");
    let stood = the_bar_s_buttons(&app);

    driver::hover_at_sketch_point(&mut app, driver::A_POINT_BELOW_AND_RIGHT);
    assert_eq!(driver::the_bar_s_coordinates(&app), "X 137,50 Y -37,50 mm");
    assert_in_their_place(&app, stood);

    driver::hover_at_sketch_point(&mut app, NEAR_THE_ORIGIN);
    assert_in_their_place(&app, stood);

    hover_off_the_view(&mut app);
    assert_in_their_place(&app, stood);
}

#[test]
fn in_the_free_view_the_bar_s_buttons_keep_their_place_whatever_the_coordinates_read() {
    let mut app = driver::a_block("in_the_free_view_the_bar_s_buttons_keep_their_place");
    hover_off_the_view(&mut app);
    assert_eq!(driver::the_bar_s_coordinates(&app), "—");
    let stood = the_bar_s_buttons(&app);

    hover_over_the_world(&mut app, ON_THE_BLOCK_S_FACE);
    assert_eq!(
        driver::the_bar_s_coordinates(&app),
        "X 10,00 Y 57,50 Z -4,05 mm"
    );
    assert_in_their_place(&app, stood);

    hover_over_the_world(&mut app, BESIDE_THE_BLOCK);
    assert_in_their_place(&app, stood);
}

/// How far within a point the scale bar and the cube stand from where they
/// are meant to: the corners are laid by the same arithmetic the frame paints
/// with.
const IN_ITS_CORNER: f32 = 1.0;

/// How high the scale bar stands, in points, from its baseline to the top of
/// its label: a 5-point tick under a 16-point line of text.
const SCALE_BAR_HEIGHT: f32 = 21.0;

/// The names the cube's faces carry, one of which at least is painted on it.
const FACE_NAMES: [&str; 6] = ["FACE", "ARRIÈRE", "DROITE", "GAUCHE", "DESSUS", "DESSOUS"];

/// Every shape the last frame painted, groups opened, with the rectangle it
/// was clipped to.
fn painted(app: &driver::App) -> Vec<(egui::Shape, egui::Rect)> {
    let mut found = Vec::new();
    for clipped in &app.output().shapes {
        let mut pending = vec![clipped.shape.clone()];
        while let Some(shape) = pending.pop() {
            match shape {
                egui::Shape::Vec(group) => pending.extend(group),
                shape => found.push((shape, clipped.clip_rect)),
            }
        }
    }
    found
}

/// The scale bar's strokes, its baseline and its two ticks: the bottom of what
/// it returns is the baseline.
fn the_scale_bar(app: &driver::App) -> egui::Rect {
    let stroke = egui::Stroke::new(1.5, tokens::TEXT_SECONDARY);
    painted(app)
        .into_iter()
        .filter_map(|(shape, _)| match shape {
            egui::Shape::LineSegment {
                points,
                stroke: drawn,
            } if drawn == stroke => Some(egui::Rect::from_two_pos(points[0], points[1])),
            _ => None,
        })
        .reduce(|one, other| one.union(other))
        .unwrap_or_else(|| panic!("no scale bar is painted"))
}

/// The orientation cube: the rectangle its face labels are clipped to.
fn the_cube(app: &driver::App) -> egui::Rect {
    painted(app)
        .into_iter()
        .find_map(|(shape, clip)| match shape {
            egui::Shape::Text(text) if FACE_NAMES.contains(&text.galley.text()) => Some(clip),
            _ => None,
        })
        .unwrap_or_else(|| panic!("no face of the cube is named on screen"))
}

/// The view bar's frame: the smallest filled rectangle painted around all it
/// holds, its shadow left aside.
fn the_bar_s_frame(app: &driver::App) -> egui::Rect {
    let holds = driver::the_view_bar(app)
        .into_iter()
        .fold(egui::Rect::NOTHING, |held, (_, rect)| held.union(rect));
    painted(app)
        .into_iter()
        .filter_map(|(shape, _)| match shape {
            egui::Shape::Rect(rect)
                if rect.blur_width == 0.0
                    && rect.fill != egui::Color32::TRANSPARENT
                    && rect.rect.contains_rect(holds) =>
            {
                Some(rect.rect)
            }
            _ => None,
        })
        .min_by(|one, other| one.area().total_cmp(&other.area()))
        .unwrap_or_else(|| panic!("no frame is painted around the view bar"))
}

/// A part just made, the scale bar and the cube set to these corners.
fn a_part_with(test: &str, scale_bar: ViewportCorner, cube: ViewportCorner) -> driver::App {
    let mut settings = cao_prefs::Settings::default();
    settings.viewport.ruler_corner = scale_bar;
    settings.viewport.cube_corner = cube;
    let mut app = driver::open_held_with(test, settings);
    driver::create_a_part(&mut app, "Platine");
    app
}

/// How far the scale bar and the cube stand off the view's edges, as the
/// settings have it.
fn margin() -> f32 {
    cao_prefs::ViewportConfig::default().cube_margin
}

/// Where a thing set to a right-hand corner ends, as it always has: the
/// margin in from the view's right edge.
fn against_the_right_edge(app: &driver::App) -> f32 {
    driver::canvas(app).max.x - margin()
}

/// How far the edges of `rect` that face `corner` stand from where that corner
/// lays a thing `margin` in from the view's edges, one gap across, one up.
fn off_its_corner(app: &driver::App, rect: egui::Rect, corner: ViewportCorner) -> egui::Vec2 {
    let view = driver::canvas(app);
    let inset = view.shrink(margin());
    match corner {
        ViewportCorner::TopLeft => rect.left_top() - inset.left_top(),
        ViewportCorner::TopRight => rect.right_top() - inset.right_top(),
        ViewportCorner::BottomLeft => rect.left_bottom() - inset.left_bottom(),
        ViewportCorner::BottomRight => rect.right_bottom() - inset.right_bottom(),
    }
}

#[test]
fn the_scale_bar_at_its_default_corner_stands_where_it_stood() {
    use ViewportCorner::{BottomLeft, TopLeft, TopRight};

    for (scale_bar, cube) in [
        (BottomLeft, TopRight),
        (TopLeft, BottomLeft),
        (TopRight, TopLeft),
    ] {
        let app = a_part_with(
            &format!("the_scale_bar_stands_where_it_stood_{scale_bar:?}"),
            scale_bar,
            cube,
        );

        let mut strokes = the_scale_bar(&app);
        if matches!(scale_bar, TopLeft | TopRight) {
            strokes.min.y = strokes.max.y - SCALE_BAR_HEIGHT;
        }
        let off = off_its_corner(&app, strokes, scale_bar);
        assert!(
            off.length() <= IN_ITS_CORNER,
            "the scale bar set to {scale_bar:?} stands {off:?} off that corner, at {strokes:?}",
        );
        let at = the_cube(&app);
        let off = off_its_corner(&app, at, cube);
        assert!(
            off.length() <= IN_ITS_CORNER,
            "the cube set to {cube:?} stands {off:?} off that corner, at {at:?}",
        );
    }
}

#[test]
fn a_scale_bar_set_to_the_bottom_right_stands_above_the_view_bar() {
    let app = a_part_with(
        "a_scale_bar_set_to_the_bottom_right_stands_above_the_view_bar",
        ViewportCorner::BottomRight,
        ViewportCorner::TopRight,
    );

    let strokes = the_scale_bar(&app);
    let bar = the_bar_s_frame(&app);
    assert!(
        (against_the_right_edge(&app) - strokes.max.x).abs() <= IN_ITS_CORNER,
        "the scale bar set to the bottom right ends at {}, not against the right edge \
         of the view as before, at {}",
        strokes.max.x,
        against_the_right_edge(&app),
    );
    assert!(
        strokes.max.y <= bar.min.y,
        "the scale bar's baseline at {} stands below the top of the view bar at {}: \
         the two overlap",
        strokes.max.y,
        bar.min.y,
    );
    assert!(
        bar.min.y - strokes.max.y <= AGAINST_THE_CORNER,
        "the scale bar stands {} points above the view bar, out of its corner",
        bar.min.y - strokes.max.y,
    );
}

#[test]
fn an_orientation_cube_set_to_the_bottom_right_stands_above_the_view_bar() {
    let app = a_part_with(
        "an_orientation_cube_set_to_the_bottom_right_stands_above_the_view_bar",
        ViewportCorner::BottomLeft,
        ViewportCorner::BottomRight,
    );

    let cube = the_cube(&app);
    let bar = the_bar_s_frame(&app);
    assert!(
        (against_the_right_edge(&app) - cube.max.x).abs() <= IN_ITS_CORNER,
        "the cube set to the bottom right ends at {}, not against the right edge of \
         the view as before, at {}",
        cube.max.x,
        against_the_right_edge(&app),
    );
    assert!(
        cube.max.y <= bar.min.y,
        "the cube's foot at {} stands below the top of the view bar at {}: the two \
         overlap",
        cube.max.y,
        bar.min.y,
    );
    assert!(
        bar.min.y - cube.max.y <= AGAINST_THE_CORNER,
        "the cube stands {} points above the view bar, out of its corner",
        bar.min.y - cube.max.y,
    );
}
