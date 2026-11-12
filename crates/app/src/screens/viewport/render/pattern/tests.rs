//! What app · screens/viewport/render/pattern.rs is held to.
//!
//! Closes #738 — the whole transcription is in
//! `crates/app/src/screens/viewport/input/copying/tests/steps.rs`, where this
//! criterion stays `no test:` for what reaches the screen; these hold the
//! geometry the guides are drawn from.
//! - In the view: the copies dashed in amber, the circle, the radius,
//!   « Centre », the arrowed arc « 60° », the source lit amber — the arrowed
//!   arc, starting on the side of what is copied and turning the way the
//!   pattern will, `a_positive_step_s_arc_starts_on_the_side_of_what_is_copied_and_turns_counter_clockwise`,
//!   `a_negative_step_s_arc_turns_clockwise`,
//!   `every_place_of_the_step_s_arc_stands_at_its_reach_from_the_centre`; its
//!   arrow, `the_arrow_s_wings_open_back_the_way_the_run_came_on_either_side_of_it`,
//!   `an_arrow_needs_a_run_of_two_places_at_least`; and where what is copied
//!   stands, the radius's end,
//!   `the_middle_of_some_places_is_their_average_and_there_is_none_of_none`

use super::*;

/// The guides are worked out directly, with no solver: a few sines.
const TOLERANCE: f64 = 1e-9;

const CENTRE: DVec2 = DVec2::new(2.0, -1.0);
/// What is copied, ten units to the right of the centre.
const SOURCE: DVec2 = DVec2::new(12.0, -1.0);
const REACH: f64 = 5.0;

fn stands_at(place: DVec2, expected: DVec2, what: &str) {
    assert!(
        place.distance(expected) < TOLERANCE,
        "{what} stands at {place}, expected {expected}"
    );
}

#[test]
fn a_positive_step_s_arc_starts_on_the_side_of_what_is_copied_and_turns_counter_clockwise() {
    let arc = step_arc(CENTRE, SOURCE, 90.0, REACH);

    stands_at(
        arc[0],
        CENTRE + DVec2::new(REACH, 0.0),
        "the arc's start, toward what is copied,",
    );
    stands_at(
        arc[arc.len() - 1],
        CENTRE + DVec2::new(0.0, REACH),
        "the end of a quarter turn counter-clockwise",
    );
}

#[test]
fn a_negative_step_s_arc_turns_clockwise() {
    let arc = step_arc(CENTRE, SOURCE, -90.0, REACH);

    stands_at(
        arc[0],
        CENTRE + DVec2::new(REACH, 0.0),
        "the arc's start, toward what is copied,",
    );
    stands_at(
        arc[arc.len() - 1],
        CENTRE - DVec2::new(0.0, REACH),
        "the end of a quarter turn clockwise",
    );
}

#[test]
fn every_place_of_the_step_s_arc_stands_at_its_reach_from_the_centre() {
    let arc = step_arc(CENTRE, DVec2::new(-3.0, 7.0), 60.0, REACH);

    assert!(arc.len() > 2, "an arc is more than its two ends: {arc:?}");
    for place in &arc {
        assert!(
            (place.distance(CENTRE) - REACH).abs() < TOLERANCE,
            "{place} stands {} from the centre, not {REACH}",
            place.distance(CENTRE)
        );
    }
}

#[test]
fn the_arrow_s_wings_open_back_the_way_the_run_came_on_either_side_of_it() {
    let run = [
        DVec2::new(0.0, 0.0),
        DVec2::new(4.0, 0.0),
        DVec2::new(10.0, 0.0),
    ];
    let tip = run[2];

    let wings = arrow_wings(&run, 2.0).expect("a run of three places carries an arrow");

    for wing in wings {
        assert!(
            (wing.distance(tip) - 2.0).abs() < TOLERANCE,
            "a wing ends {} from the tip, not 2",
            wing.distance(tip)
        );
        assert!(
            wing.x < tip.x,
            "the wing at {wing} stands behind the tip, on the side the run came from"
        );
    }
    assert!(
        wings[0].y * wings[1].y < 0.0,
        "one wing each side of the run: {wings:?}"
    );
}

#[test]
fn an_arrow_needs_a_run_of_two_places_at_least() {
    assert_eq!(arrow_wings(&[], 2.0), None);
    assert_eq!(arrow_wings(&[DVec2::new(1.0, 1.0)], 2.0), None);
    assert!(arrow_wings(&[DVec2::ZERO, DVec2::new(1.0, 1.0)], 2.0).is_some());
}

#[test]
fn the_middle_of_some_places_is_their_average_and_there_is_none_of_none() {
    let places = [
        DVec2::new(0.0, 0.0),
        DVec2::new(4.0, 0.0),
        DVec2::new(2.0, 6.0),
    ];

    stands_at(
        middle(&places).expect("three places have a middle"),
        DVec2::new(2.0, 2.0),
        "the middle of three places",
    );
    assert_eq!(middle(&[]), None);
}
