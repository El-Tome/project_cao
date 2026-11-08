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
//! - `Command::RecenterOnSketch` becomes `Command::Reframe`, its label key and
//!   its hint following — no test: the compiler holds the name, and a profile
//!   saved under the old one is not migrated, as decided for the whole redesign
//! - in a sketch, the bar's « Matière devant » offers « cachée »,
//!   « transparente » and « opaque », and picking one sets the open sketch's
//!   matter in front to it directly rather than cycling —
//!   `choosing_see_through_in_the_bar_s_menu_makes_the_matter_in_front_see_through`,
//!   not yet written: it waits for the bar to be drawn. The three offered in
//!   order, in `crates/app/src/screens/view_bar/state/tests.rs` by
//!   `the_matter_in_front_menu_offers_hidden_see_through_and_opaque_in_that_order`;
//!   the state picked set from whichever one it was, in
//!   `crates/app/src/screens/sketch/tests.rs` by
//!   `picking_a_state_of_the_matter_in_front_sets_the_open_sketch_to_it_whatever_it_was`
//! - the bar's label reads « Matière devant : cachée ▾ » after the current
//!   state — `the_bar_s_matter_in_front_reads_the_state_the_sketch_is_in`, not
//!   yet written, for the ▾ and the window; the label and the state marked
//!   among the three, in `crates/app/src/screens/view_bar/state/tests.rs` by
//!   `the_matter_in_front_menu_reads_and_marks_the_state_the_open_sketch_is_in`
//! - outside a sketch the bar's « Matière devant » is greyed, as the ribbon's
//!   is today — `outside_a_sketch_the_bar_s_matter_in_front_is_greyed`, not yet
//!   written; the rule, in `crates/app/src/screens/view_bar/state/tests.rs` by
//!   `outside_a_sketch_the_matter_in_front_menu_is_greyed`
//! - the ribbon's own « Matière devant » stays and keeps cycling — no test: in
//!   this file; held in `crates/app/src/commands/tests.rs` by
//!   `the_matter_in_front_button_moves_the_open_sketch_to_the_next_state`,
//!   left as it is
//! - in a sketch the coordinates read « X 12,50 Y -3,00 mm », the point in the
//!   sketch plane's own frame, and follow the mouse —
//!   `in_a_sketch_the_bar_s_x_and_y_follow_the_mouse`, not yet written: it
//!   waits for the bar to be drawn, and with it the snapped point, a line tool
//!   near the origin reading « X 0,00 Y 0,00 mm », which only the input code on
//!   the no-net list decides; the text, in
//!   `crates/app/src/screens/view_bar/state/tests.rs` by
//!   `on_a_sketch_s_plane_the_coordinates_read_x_and_y_to_two_decimals_after_a_comma`,
//!   and the point a click would place, else the raw point the tool leaves, by
//!   `in_a_sketch_the_coordinates_are_where_a_click_would_place_the_point`
//! - in the free view they read « X … Y … Z … mm », the world coordinates of
//!   the first point of the drawn body the cursor's ray meets, nothing when the
//!   body is hidden — `hovering_the_block_in_the_free_view_reads_its_x_y_and_z`,
//!   not yet written; the text and the choice of the part's point outside a
//!   sketch, in `crates/app/src/screens/view_bar/state/tests.rs` by
//!   `on_the_part_the_coordinates_read_x_y_and_z` and
//!   `outside_a_sketch_the_coordinates_are_the_point_of_the_part_under_the_cursor`;
//!   the point met, the hidden body and the section's cut, in
//!   `crates/app/src/screens/viewport/state/tests.rs` by
//!   `the_point_of_the_part_under_a_ray_is_the_first_the_ray_meets`,
//!   `a_hidden_body_has_no_point_under_a_ray` and
//!   `with_the_section_view_on_the_point_under_a_ray_is_on_the_cut`
//! - « — » over nothing, and when the cursor is not over the view —
//!   `off_the_part_and_off_the_view_the_coordinates_read_a_dash`, not yet
//!   written; the dash and the cursor off the view, in
//!   `crates/app/src/screens/view_bar/state/tests.rs` by
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
//! - the bar stands at the bottom right of the view and holds, in order,
//!   « Recadrer » with its key F, « Vue en coupe », « Matière devant : … ▾ », a
//!   separator, the coordinates —
//!   `the_bar_stands_at_the_bottom_right_of_the_view_in_the_design_s_order`
//! - on the floating background, `BACKGROUND_FLOATING` — no test: in this file,
//!   whose tree carries no colour; held beside the drawing of the bar, under
//!   `crates/app/src/screens/view_bar/`, once it exists
//! - each button follows its command's rule: « Vue en coupe » greyed in a
//!   sketch and with no volume —
//!   `the_bar_s_section_view_is_greyed_in_a_sketch_and_with_no_volume`; shown
//!   pressed while a section is on —
//!   `the_bar_s_section_view_shows_pressed_while_the_section_is_on`
//! - the section view panel opens and closes from the bar as it did from the
//!   ribbon —
//!   `the_bar_s_section_view_opens_and_closes_the_section_panel_as_the_ribbon_s_does`
//! - the bar's buttons are found as the bar's, and the ribbon's « Recadrer »,
//!   « Vue en coupe » and « Matière devant », which stay, are still found as the
//!   ribbon's — `the_bar_s_buttons_are_told_apart_from_the_ribbon_s`; and the
//!   driver tests already aimed at the ribbon's, `section_view.rs` and
//!   `keeping_a_section.rs`, keep passing
//! - the scale bar keeps its corner and its settings: at its default corner,
//!   bottom left, it does not move — no test: in this file; held in
//!   `crates/app/src/screens/viewport/state/tests.rs` by
//!   `the_scale_bar_at_its_default_corner_stands_where_it_stood`
//! - set to the bottom right, the scale bar, or the orientation cube, stays in
//!   that corner and is lifted above the bar, so that the two do not overlap —
//!   no test: in this file; held in
//!   `crates/app/src/screens/viewport/state/tests.rs` by
//!   `a_scale_bar_set_to_the_bottom_right_stands_above_the_view_bar` and
//!   `an_orientation_cube_set_to_the_bottom_right_stands_above_the_view_bar`
//!
//! What « fills the view » is held to: every corner of what is framed stands
//! inside the view, and it covers at least a third of the view across or up,
//! whichever it covers more of. Framing a sphere around the bounds, as the
//! sketch has always been framed, leaves a long flat drawing at about that;
//! a view that did not move after the wheel stepped back leaves the part a
//! speck.

mod driver;

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
