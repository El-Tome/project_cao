//! What app · screens/view_bar/state.rs is held to.
//!
//! Closes #728 — the whole transcription is in `crates/app/tests/view_bar.rs`;
//! these bullets are the ones the bar's presenter answers, with no window.
//! - in a sketch, the bar's « Matière devant » offers « cachée »,
//!   « transparente » and « opaque », in that order, each the state it names —
//!   `the_matter_in_front_menu_offers_hidden_see_through_and_opaque_in_that_order`
//! - it reads « Matière devant : cachée » after the open sketch's state, and
//!   marks that state among the three —
//!   `the_matter_in_front_menu_reads_and_marks_the_state_the_open_sketch_is_in`;
//!   the ▾ that says it opens is the drawing's, and the driver reads it once
//!   the bar is drawn
//! - outside a sketch it is greyed —
//!   `outside_a_sketch_the_matter_in_front_menu_is_greyed`
//! - in a sketch the coordinates read « X 12,50 Y -3,00 mm », the point in the
//!   sketch plane's own frame —
//!   `on_a_sketch_s_plane_the_coordinates_read_x_and_y_to_two_decimals_after_a_comma`
//! - it is the point a click would place, the end a line is aimed at once
//!   what was typed has had its say, else the cursor as the tool in hand
//!   leaves it — snapped when the tool aims, the raw point on the plane when
//!   it only points —
//!   `in_a_sketch_the_coordinates_are_where_a_click_would_place_the_point`;
//!   the snapping itself is the input code's, on the no-net list, and the
//!   driver holds it once the bar is drawn
//! - in the free view they read « X … Y … Z … mm », the point of the part
//!   under the cursor — `on_the_part_the_coordinates_read_x_y_and_z`,
//!   `outside_a_sketch_the_coordinates_are_the_point_of_the_part_under_the_cursor`
//! - « — » over nothing, and when the cursor is not over the view —
//!   `over_nothing_the_coordinates_read_a_dash`,
//!   `in_a_sketch_with_the_cursor_off_the_view_the_coordinates_point_at_nothing`
//! - two decimals after a comma, never « -0,00 » —
//!   `a_coordinate_a_hair_below_zero_reads_zero_with_no_sign`
//! - the view's length unit after them, the values converted as the scale bar
//!   converts its own, from the part's units to millimetres and on to the
//!   unit — `the_coordinates_are_said_in_the_view_s_unit_at_the_part_s_scale`
//!
//! The unit is handed in as a `LengthUnit`, already chosen: with
//! `UnitDisplay::Auto` the bar is meant to read in the unit the scale bar
//! shows, which is the caller's to pass.

use cao_prefs::config::LengthUnit;
use cao_render::MatterInFront;
use cao_sketch::{Aim, WorkPlane};
use glam::{DVec2, DVec3};

use super::*;
use crate::lang::Catalogue;
use crate::screens::sketch::SketchEditor;

fn drawing_on_a_sketch(matter_in_front: MatterInFront) -> SketchEditor {
    let mut editor = SketchEditor::default();
    editor.begin_editing(0, WorkPlane::XY);
    editor.matter_in_front = matter_in_front;
    editor
}

#[test]
fn the_matter_in_front_menu_offers_hidden_see_through_and_opaque_in_that_order() {
    let menu = MatterInFrontMenu::of(
        &drawing_on_a_sketch(MatterInFront::Opaque),
        &Catalogue::french(),
    );

    let offered: Vec<(&str, MatterInFront)> = menu
        .choices
        .iter()
        .map(|choice| (choice.label.as_str(), choice.state))
        .collect();

    assert_eq!(
        offered,
        [
            ("cachée", MatterInFront::Hidden),
            ("transparente", MatterInFront::SeeThrough),
            ("opaque", MatterInFront::Opaque),
        ],
        "the menu does not offer the three states, each by its name, in the design's order",
    );
}

#[test]
fn the_matter_in_front_menu_reads_and_marks_the_state_the_open_sketch_is_in() {
    let lang = Catalogue::french();

    for (state, reads) in [
        (MatterInFront::Hidden, "Matière devant : cachée"),
        (MatterInFront::SeeThrough, "Matière devant : transparente"),
        (MatterInFront::Opaque, "Matière devant : opaque"),
    ] {
        let menu = MatterInFrontMenu::of(&drawing_on_a_sketch(state), &lang);

        assert_eq!(
            menu.label, reads,
            "the sketch's matter in front is {state:?}"
        );
        let marked: Vec<MatterInFront> = menu
            .choices
            .iter()
            .filter(|choice| choice.current)
            .map(|choice| choice.state)
            .collect();
        assert_eq!(
            marked,
            [state],
            "the sketch's matter in front is {state:?}, and the menu marks {marked:?}",
        );
    }
}

#[test]
fn outside_a_sketch_the_matter_in_front_menu_is_greyed() {
    let lang = Catalogue::french();
    let mut finished = drawing_on_a_sketch(MatterInFront::Hidden);
    finished.close();

    assert!(
        MatterInFrontMenu::of(&drawing_on_a_sketch(MatterInFront::Hidden), &lang).enabled,
        "« Matière devant » is greyed while a sketch is open",
    );
    for (editor, when) in [
        (SketchEditor::default(), "before a sketch is opened"),
        (finished, "once the sketch is finished"),
    ] {
        assert!(
            !MatterInFrontMenu::of(&editor, &lang).enabled,
            "« Matière devant » can be picked {when}, with no sketch to show the matter \
             in front of",
        );
    }
}

fn read(pointed: Pointed) -> String {
    coordinates(pointed, LengthUnit::Millimeter, 1.0, &Catalogue::french())
}

#[test]
fn on_a_sketch_s_plane_the_coordinates_read_x_and_y_to_two_decimals_after_a_comma() {
    assert_eq!(
        read(Pointed::OnThePlane(DVec2::new(12.5, -3.0))),
        "X 12,50 Y -3,00 mm",
    );
}

#[test]
fn on_the_part_the_coordinates_read_x_y_and_z() {
    assert_eq!(
        read(Pointed::OnThePart(DVec3::new(1.0, -2.25, 300.1))),
        "X 1,00 Y -2,25 Z 300,10 mm",
    );
}

#[test]
fn over_nothing_the_coordinates_read_a_dash() {
    assert_eq!(read(Pointed::Nothing), "—");
}

#[test]
fn a_coordinate_a_hair_below_zero_reads_zero_with_no_sign() {
    assert_eq!(
        read(Pointed::OnThePlane(DVec2::new(-0.001, -0.0))),
        "X 0,00 Y 0,00 mm",
        "a point on the axes, a hair off them, reads as though a side had been taken",
    );
    assert_eq!(
        read(Pointed::OnThePart(DVec3::new(-0.004, 0.004, -0.0))),
        "X 0,00 Y 0,00 Z 0,00 mm",
    );
}

#[test]
fn the_coordinates_are_said_in_the_view_s_unit_at_the_part_s_scale() {
    let lang = Catalogue::french();
    let a_point = Pointed::OnThePlane(DVec2::new(5.0, -2.0));

    assert_eq!(
        coordinates(a_point, LengthUnit::Millimeter, 2.5, &lang),
        "X 12,50 Y -5,00 mm",
        "a unit of the part is worth 2.5 mm",
    );
    assert_eq!(
        coordinates(a_point, LengthUnit::Centimeter, 2.5, &lang),
        "X 1,25 Y -0,50 cm",
        "a unit of the part is worth 2.5 mm, said in centimetres",
    );
}

#[test]
fn in_a_sketch_the_coordinates_are_where_a_click_would_place_the_point() {
    let mut editor = drawing_on_a_sketch(MatterInFront::Hidden);
    editor.cursor = Some(DVec2::new(12.5, -3.0));

    assert_eq!(
        Pointed::of(&editor, Some(DVec3::new(7.0, 8.0, 9.0))),
        Pointed::OnThePlane(DVec2::new(12.5, -3.0)),
        "in a sketch, the bar reads something else than the cursor the tool in hand leaves",
    );

    editor.aimed = Some(Aim::at(DVec2::new(20.0, 0.0)));
    assert_eq!(
        Pointed::of(&editor, None),
        Pointed::OnThePlane(DVec2::new(20.0, 0.0)),
        "a line aimed at the length typed ends there, and the bar reads the cursor instead",
    );
}

#[test]
fn in_a_sketch_with_the_cursor_off_the_view_the_coordinates_point_at_nothing() {
    let editor = drawing_on_a_sketch(MatterInFront::Hidden);
    assert_eq!(
        editor.cursor, None,
        "the fixture has the cursor off the view"
    );

    assert_eq!(
        Pointed::of(&editor, Some(DVec3::new(7.0, 8.0, 9.0))),
        Pointed::Nothing
    );
}

#[test]
fn outside_a_sketch_the_coordinates_are_the_point_of_the_part_under_the_cursor() {
    let mut finished = drawing_on_a_sketch(MatterInFront::Hidden);
    finished.close();
    finished.cursor = Some(DVec2::new(12.5, -3.0));

    assert_eq!(
        Pointed::of(&finished, Some(DVec3::new(7.0, 8.0, 9.0))),
        Pointed::OnThePart(DVec3::new(7.0, 8.0, 9.0)),
        "outside a sketch, the bar reads a cursor left on a plane rather than the part",
    );
    assert_eq!(
        Pointed::of(&finished, None),
        Pointed::Nothing,
        "outside a sketch, off the part, the bar reads a point all the same",
    );
}
