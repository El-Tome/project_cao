//! What app · screens/viewport/input/copying/opening.rs is held to.

use cao_part::{Operation, PointRef};
use cao_prefs::Command;
use cao_sketch::{CircleId, DimensionTarget, Element};
use glam::DVec2;

use super::super::tests::bench::Bench;

/// A drawing holding one circle four units across, and nothing else.
fn a_circle_four_units_across() -> Vec<Operation> {
    vec![Operation::AddCircle {
        sketch: 0,
        center: PointRef::New(DVec2::new(10.0, 10.0)),
        radius: 2.0,
        rim: Vec::new(),
        construction: false,
    }]
}

/// The four fields as the panel opens them, the tool taken with that circle
/// selected.
fn fields_opened_by(command: Command, drawing: Vec<Operation>) -> Vec<Option<f64>> {
    let mut bench = Bench::drawn(drawing);
    bench.select(&[Element::Circle(CircleId(0))]);

    bench.take(command);

    (0..4).map(|rank| bench.value(rank)).collect()
}

#[test]
fn a_grid_opens_on_a_step_the_hold_fits_in_and_one_copy_each_way() {
    assert_eq!(
        fields_opened_by(
            Command::ToolRectangularPattern,
            a_circle_four_units_across()
        ),
        vec![Some(4.0), Some(2.0), Some(4.0), Some(2.0)]
    );
}

#[test]
fn a_circular_pattern_opens_on_a_quarter_turn_four_times_round() {
    assert_eq!(
        fields_opened_by(Command::ToolCircularPattern, a_circle_four_units_across()),
        vec![Some(90.0), Some(4.0), None, None]
    );
}

#[test]
fn the_mirror_opens_no_fields_at_all() {
    assert_eq!(
        fields_opened_by(Command::ToolMirror, a_circle_four_units_across()),
        vec![None; 4]
    );
}

#[test]
fn a_step_opens_in_the_millimetres_its_field_is_labelled_with() {
    let mut drawing = a_circle_four_units_across();
    drawing.push(Operation::SetDimension {
        sketch: 0,
        target: DimensionTarget::Diameter(CircleId(0)),
        value: 20.0.into(),
        placement: None,
    });

    let opened = fields_opened_by(Command::ToolRectangularPattern, drawing);

    assert_eq!(
        opened[0],
        Some(20.0),
        "five millimetres to the unit, and the circle four units across"
    );
}
