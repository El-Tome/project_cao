//! What app · screens/viewport/input/corner.rs is held to.
//!
//! Closes #315.
//! - clicking a corner point takes that corner, with the fillet and with the
//!   chamfer in equal distances — `a_click_on_a_corner_point_names_both_its_sides`
//! - clicking a corner point again drops it — `a_second_click_on_a_corner_drops_it`
//! - a point where more than two traits meet cannot be taken as a corner: the
//!   tool asks for the two traits instead —
//!   `a_click_on_a_point_too_crowded_to_be_a_corner_asks_for_the_two_traits`
//! - the chamfer in distance and angle, or in two distances, still takes a side
//!   and then the other — `an_asymmetric_mode_reads_a_click_as_a_side_even_on_a_corner_point`
//! - changing the chamfer's mode to one of those says to click one side and
//!   then the other — `each_mode_asks_for_what_it_can_actually_take`
//! - changing the mode empties the corners taken — no test: `reset_pending`
//!   already empties the tool state on every mode change, and it lives in a
//!   file `docs/code-map.md` lists as having no net; taking that file off the
//!   list on the strength of one test would overstate what covers it
//! - `Entrée` that cannot lay anything says why, instead of doing nothing —
//!   `enter_with_half_a_corner_asks_for_the_other_side_rather_than_dropping_it`,
//!   `enter_with_no_side_clicked_asks_for_a_corner`,
//!   `enter_with_both_sides_named_cuts_them`

use cao_sketch::{Corner, PointId, Sketch, WorkPlane};
use glam::DVec2;

use super::*;

const CORNER: DVec2 = DVec2::new(2.0, 1.0);
const SNAP: f64 = 0.5;

/// A right angle, one side running east and the other north.
fn a_right_angle() -> (Sketch, SegmentId, SegmentId, PointId) {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let pivot = sketch.add_point(CORNER);
    let east = sketch.add_point(CORNER + DVec2::new(10.0, 0.0));
    let north = sketch.add_point(CORNER + DVec2::new(0.0, 10.0));
    let along = sketch.add_segment(pivot, east);
    let up = sketch.add_segment(pivot, north);
    (sketch, along, up, pivot)
}

fn locked(first: Option<f64>, second: Option<f64>) -> LockedInput {
    LockedInput { first, second }
}

#[test]
fn the_equal_mode_takes_the_one_value_typed_for_both_sides() {
    assert_eq!(
        typed(locked(Some(4.0), None), ChamferMode::Equal, false),
        Some(Chamfer::Equal(4.0)),
        "a second value would say nothing the first does not"
    );
}

#[test]
fn a_mode_asking_two_values_waits_until_both_are_typed() {
    assert_eq!(
        typed(locked(Some(4.0), None), ChamferMode::Sided, false),
        None
    );
    assert_eq!(
        typed(locked(None, Some(30.0)), ChamferMode::Angled, false),
        None
    );
    assert_eq!(
        typed(locked(Some(4.0), Some(30.0)), ChamferMode::Angled, false),
        Some(Chamfer::Angled {
            along: 4.0,
            degrees: 30.0
        })
    );
}

#[test]
fn a_fillet_asks_for_one_value_whatever_the_chamfer_beside_it_is_set_to() {
    assert_eq!(
        typed(locked(Some(5.0), None), ChamferMode::Sided, true),
        Some(Chamfer::Equal(5.0)),
        "a radius is a radius, and the chamfer's second field says nothing about it"
    );
}

#[test]
fn enter_with_half_a_corner_asks_for_the_other_side_rather_than_dropping_it() {
    assert_eq!(
        on_enter(&ToolState::Corner {
            taken: Vec::new(),
            half: Some(SegmentId(1)),
        }),
        CornerEnter::Waiting("sketch.click_the_other_side"),
        "Enter used to fall through to the rectangle, which put its own state \
         where the corner's was — so the side already clicked was lost and had \
         to be clicked again"
    );
}

#[test]
fn enter_with_no_side_clicked_asks_for_a_corner() {
    assert_eq!(
        on_enter(&ToolState::None),
        CornerEnter::Waiting("sketch.click_a_corner")
    );
}

#[test]
fn enter_with_a_corner_taken_cuts_it() {
    assert_eq!(
        on_enter(&ToolState::Corner {
            taken: vec![Corner::Between(SegmentId(1), SegmentId(2))],
            half: None,
        }),
        CornerEnter::Cut
    );
}

#[test]
fn enter_still_cuts_when_a_side_is_half_named_beside_the_corners_taken() {
    assert_eq!(
        on_enter(&ToolState::Corner {
            taken: vec![Corner::At(PointId(1))],
            half: Some(SegmentId(3)),
        }),
        CornerEnter::Cut,
        "a side clicked by mistake does not hold back the corners already taken"
    );
}

#[test]
fn a_click_on_a_corner_point_names_both_its_sides() {
    let (sketch, _along, _up, pivot) = a_right_angle();

    assert_eq!(
        clicked(&sketch, sketch.point(pivot), SNAP, true),
        CornerClick::Corner(Corner::At(pivot)),
        "one click on the point says as much as two on the sides, and says it \
         without having to be over the right one"
    );
}

#[test]
fn a_click_on_a_point_too_crowded_to_be_a_corner_asks_for_the_two_traits() {
    let (mut sketch, _along, _up, pivot) = a_right_angle();
    let away = sketch.add_point(CORNER + DVec2::new(-10.0, -10.0));
    sketch.add_segment(pivot, away);

    assert_eq!(
        clicked(&sketch, sketch.point(pivot), SNAP, true),
        CornerClick::Crowded,
        "three traits leave no saying which two the cut is meant for"
    );
}

#[test]
fn an_asymmetric_mode_reads_a_click_as_a_side_even_on_a_corner_point() {
    let (sketch, along, up, pivot) = a_right_angle();

    let read = clicked(&sketch, sketch.point(pivot), SNAP, false);

    assert!(
        read == CornerClick::Side(along) || read == CornerClick::Side(up),
        "a corner taken by its point has no first side, and these modes need \
         one — got {read:?}"
    );
}

#[test]
fn a_click_on_a_trait_away_from_any_point_names_that_side() {
    let (sketch, along, _up, _pivot) = a_right_angle();

    assert_eq!(
        clicked(&sketch, CORNER + DVec2::new(5.0, 0.0), SNAP, true),
        CornerClick::Side(along)
    );
}

#[test]
fn a_click_on_nothing_at_all_names_nothing() {
    let (sketch, _along, _up, _pivot) = a_right_angle();

    assert_eq!(
        clicked(&sketch, CORNER + DVec2::new(-50.0, -50.0), SNAP, true),
        CornerClick::Nothing
    );
}

#[test]
fn each_mode_asks_for_what_it_can_actually_take() {
    assert_eq!(
        picks_with(Tool::Fillet, ChamferMode::Sided),
        "sketch.click_a_corner_to_round",
        "a fillet takes a corner whatever the chamfer beside it is set to"
    );
    assert_eq!(
        picks_with(Tool::Chamfer, ChamferMode::Equal),
        "sketch.click_a_corner_to_cut"
    );
    for mode in [ChamferMode::Angled, ChamferMode::Sided] {
        assert_eq!(
            picks_with(Tool::Chamfer, mode),
            "sketch.click_one_side_then_the_other",
            "{mode:?} measures from the side named first, so it cannot be \
             offered a corner taken by its point"
        );
    }
}

#[test]
fn a_second_click_on_a_corner_drops_it() {
    let taken = vec![Corner::At(PointId(1)), Corner::At(PointId(4))];

    assert_eq!(
        after_clicking(taken.clone(), Corner::At(PointId(1))),
        vec![Corner::At(PointId(4))],
        "a corner clicked twice is a corner the user changed their mind about"
    );
    assert_eq!(
        after_clicking(taken.clone(), Corner::At(PointId(7))),
        vec![
            Corner::At(PointId(1)),
            Corner::At(PointId(4)),
            Corner::At(PointId(7))
        ],
        "and one clicked once joins the rest"
    );
}

/// What the list of corners becomes when that one is clicked.
fn after_clicking(mut taken: Vec<Corner>, corner: Corner) -> Vec<Corner> {
    toggle(&mut taken, corner);
    taken
}
