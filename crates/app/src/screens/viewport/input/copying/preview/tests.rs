//! What app · screens/viewport/input/copying/preview.rs is held to.

use cao_part::Operation;
use cao_prefs::Command;
use glam::DVec2;

use super::super::tests::bench::Bench;
use crate::screens::sketch::CopySlot;

const ON_THE_TRAIT: DVec2 = DVec2::new(6.0, 5.0);
const OVER_THE_U_AXIS: DVec2 = DVec2::new(20.0, 0.0);
const THE_ORIGIN: DVec2 = DVec2::ZERO;
const FAR_FROM_EVERYTHING: DVec2 = DVec2::new(-30.0, -30.0);

/// A trait standing well clear of both sketch axes, a point at the origin,
/// and the tool taken with the trait held and its target slot active.
fn a_trait_off_to_one_side_held_by(command: Command) -> Bench {
    let mut bench = Bench::drawn(vec![
        Bench::a_trait(DVec2::new(4.0, 5.0), DVec2::new(8.0, 5.0), false),
        Operation::AddPoint {
            sketch: 0,
            position: THE_ORIGIN,
            on: Vec::new(),
        },
    ]);
    bench.take(command);
    bench.click(ON_THE_TRAIT);
    bench.enter();
    assert_eq!(bench.active(), CopySlot::Target);
    bench
}

#[test]
fn a_mirror_shows_the_copies_across_the_axis_it_holds() {
    let mut bench = a_trait_off_to_one_side_held_by(Command::ToolMirror);
    bench.click(OVER_THE_U_AXIS);

    assert_eq!(
        bench.previewed_traits(FAR_FROM_EVERYTHING, 1.0),
        Some(1),
        "the one trait held is copied once, wherever the cursor has gone"
    );
    assert_eq!(
        bench.document.sketches()[0].segments().len(),
        1,
        "the drawing itself gains nothing until the copy is laid"
    );
}

#[test]
fn a_selection_still_being_gathered_shows_nothing() {
    let mut bench = Bench::drawn(vec![Bench::a_trait(
        DVec2::new(4.0, 5.0),
        DVec2::new(8.0, 5.0),
        false,
    )]);
    bench.take(Command::ToolMirror);
    bench.click(ON_THE_TRAIT);

    assert_eq!(
        bench.previewed_traits(OVER_THE_U_AXIS, 1.0),
        None,
        "a click still adding to the selection is not a click that lays anything"
    );
}

#[test]
fn a_circular_pattern_shows_every_copy_its_count_asks_for_bar_the_original() {
    let mut bench = a_trait_off_to_one_side_held_by(Command::ToolCircularPattern);
    bench.click(THE_ORIGIN);

    assert_eq!(
        bench.previewed_traits(FAR_FROM_EVERYTHING, 1.0),
        Some(3),
        "a quarter turn four times round: four standing in the end is three laid"
    );
}

#[test]
fn a_pattern_with_its_values_still_untyped_shows_nothing() {
    let mut bench = a_trait_off_to_one_side_held_by(Command::ToolCircularPattern);
    bench.click(THE_ORIGIN);
    assert_eq!(
        bench.previewed_traits(FAR_FROM_EVERYTHING, 1.0),
        Some(3),
        "shown while the values the panel opened on stand"
    );

    bench.type_value(0, "");
    bench.type_value(1, "");

    assert_eq!(
        bench.previewed_traits(FAR_FROM_EVERYTHING, 1.0),
        None,
        "a pattern with no angle and no count has no copies to show"
    );
}

#[test]
fn a_step_typed_in_millimetres_lands_the_copies_where_the_drawing_measures() {
    let mut bench = a_trait_off_to_one_side_held_by(Command::ToolRectangularPattern);
    bench.click(OVER_THE_U_AXIS);
    for (rank, value) in [(0, "20"), (1, "2"), (2, "0"), (3, "1")] {
        bench.type_value(rank, value);
    }

    let sketch = &bench.document.sketches()[0];
    let shown = super::previewed(
        sketch,
        &bench.editor,
        FAR_FROM_EVERYTHING,
        super::super::tests::bench::SNAP,
        2.0,
    )
    .expect("a direction held to run along");

    let [cao_sketch::Element::Point(first), ..] = shown.laid.as_slice() else {
        panic!(
            "the copy stands on corners of its own, got {:?}",
            shown.laid
        );
    };
    let moved = shown.sketch.point(*first) - DVec2::new(4.0, 5.0);
    assert!(
        (moved.x - 10.0).abs() <= 1e-9 && moved.y.abs() <= 1e-9,
        "twenty millimetres at two per unit is ten units along the axis, got {moved}"
    );
}
