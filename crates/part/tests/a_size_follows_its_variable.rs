//! A size written from the part's variables keeps what it was written from,
//! and follows the variables when they change.
//!
//! Closes #175.
//! - changing a variable changes every size written from it, and the part is
//!   rebuilt — `changing_a_variable_moves_every_size_written_from_it`
//! - undo puts a variable's former formula back, and the part with it —
//!   `undoing_a_change_to_a_variable_puts_the_part_back_with_it`
//! - a chamfer's values keep the formulas they were typed as —
//!   `a_chamfer_written_from_variables_lays_each_value_with_its_own_formula`
//! - and a diameter a cut carries onto an arc keeps its formula, halved —
//!   `a_diameter_written_from_a_variable_is_halved_onto_the_arc_a_cut_leaves`

use cao_part::history::{ExtrusionMode, Operation, PointRef};
use cao_part::{Formula, PartDocument, VariableChange, VariableId};
use cao_sketch::{DimensionTarget, SegmentId, WorkPlane};
use cao_solid::Mesh;
use glam::DVec2;

const TOLERANCE: f64 = 1e-6;

fn thickness(mesh: &Mesh) -> f64 {
    let (min, max) = mesh.bounds().expect("some matter");
    max.z - min.z
}

fn at_nine() -> chrono::DateTime<chrono::Utc> {
    "2026-01-02T09:00:00Z".parse().expect("a date")
}

fn written(document: &PartDocument, text: &str) -> Formula {
    document
        .variables()
        .read(text)
        .expect("a formula that reads")
}

fn add(document: &mut PartDocument, name: &str, value: f64) {
    document.apply(Operation::Variable(VariableChange::Added {
        name: name.to_string(),
        formula: Formula::Number(value),
    }));
}

fn set(document: &mut PartDocument, variable: usize, name: &str, value: f64) {
    document.apply(Operation::Variable(VariableChange::Edited {
        variable: VariableId(variable),
        name: name.to_string(),
        formula: Formula::Number(value),
    }));
}

/// A plate 100 mm wide, as high as `height`, raised by `thickness`.
fn plate() -> PartDocument {
    let mut document = PartDocument::new("Plate", at_nine());
    add(&mut document, "height", 40.0);
    add(&mut document, "thickness", 4.0);
    document.apply(Operation::CreateSketch {
        plane: WorkPlane::XY,
        on: None,
    });
    document.apply(Operation::AddRectangle {
        sketch: 0,
        corner: PointRef::New(DVec2::ZERO),
        opposite: PointRef::New(DVec2::new(100.0, 30.0)),
        construction: false,
    });
    document.apply(Operation::SetDimension {
        sketch: 0,
        target: DimensionTarget::Length(SegmentId(0)),
        value: 100.0.into(),
        placement: None,
    });
    let height = written(&document, "height");
    document.apply(Operation::SetDimension {
        sketch: 0,
        target: DimensionTarget::Length(SegmentId(1)),
        value: height,
        placement: None,
    });
    let areas = document.areas_at(0, &[DVec2::new(50.0, 15.0)]);
    let thickness = written(&document, "thickness");
    document.apply(Operation::Extrude {
        sketch: 0,
        areas,
        distance: thickness,
        mode: ExtrusionMode::Add,
    });
    document
}

fn height_of(document: &PartDocument) -> f64 {
    document
        .measured(0, DimensionTarget::Length(SegmentId(1)))
        .expect("the side is measured")
}

#[test]
fn a_dimension_written_from_a_variable_comes_to_its_value() {
    let document = plate();

    assert!((height_of(&document) - 40.0).abs() < TOLERANCE);
    assert!((thickness(document.body()) - 4.0).abs() < 1e-6);
}

#[test]
fn changing_a_variable_moves_every_size_written_from_it() {
    let mut document = plate();

    set(&mut document, 0, "height", 60.0);
    set(&mut document, 1, "thickness", 10.0);

    assert!(
        (height_of(&document) - 60.0).abs() < TOLERANCE,
        "the side followed height to {}",
        height_of(&document),
    );
    assert!(
        (thickness(document.body()) - 10.0).abs() < 1e-6,
        "the plate was raised again as thick as thickness: {}",
        thickness(document.body()),
    );
}

#[test]
fn undoing_a_change_to_a_variable_puts_the_part_back_with_it() {
    let mut document = plate();
    set(&mut document, 0, "height", 60.0);

    document.undo();

    assert!((height_of(&document) - 40.0).abs() < TOLERANCE);
}

#[test]
fn a_dimension_remembers_the_formula_it_was_written_from() {
    let document = plate();

    let side = document.sketches()[0]
        .dimension_of(DimensionTarget::Length(SegmentId(1)))
        .expect("the side carries its value");

    assert_eq!(side.written.as_deref(), Some("#0"));
    assert_eq!(
        document.formula_of(0, DimensionTarget::Length(SegmentId(1))),
        Some("height".to_string()),
    );
    assert_eq!(
        document.formula_of(0, DimensionTarget::Length(SegmentId(0))),
        None,
        "a plain number was written from nothing",
    );
}

/// A right angle, its corner at (2, 1), with `setback` at 3 and `slope` at 30.
fn a_right_angle_and_two_variables() -> PartDocument {
    let mut document = PartDocument::new("Square", at_nine());
    add(&mut document, "setback", 3.0);
    add(&mut document, "slope", 30.0);
    document.apply(Operation::CreateSketch {
        plane: WorkPlane::XY,
        on: None,
    });
    let corner = DVec2::new(2.0, 1.0);
    document.apply(Operation::AddSegment {
        sketch: 0,
        start: PointRef::New(corner),
        end: PointRef::New(corner + DVec2::new(10.0, 0.0)),
        construction: false,
    });
    document.apply(Operation::AddSegment {
        sketch: 0,
        start: PointRef::Existing(cao_sketch::PointId(1)),
        end: PointRef::New(corner + DVec2::new(0.0, 10.0)),
        construction: false,
    });
    document
}

#[test]
fn a_chamfer_written_from_variables_lays_each_value_with_its_own_formula() {
    let mut document = a_right_angle_and_two_variables();
    let along = written(&document, "setback");
    let degrees = written(&document, "slope");

    document.apply(Operation::Chamfer {
        sketch: 0,
        corners: vec![cao_sketch::Corner::Between(SegmentId(0), SegmentId(1))],
        mode: cao_part::history::ChamferAsked::Angled { along, degrees },
    });

    let mut notes: Vec<(bool, String)> = document.sketches()[0]
        .dimensions()
        .iter()
        .filter_map(|value| Some((value.is_angle(), value.written.clone()?)))
        .collect();
    notes.sort();
    assert_eq!(
        notes,
        vec![(false, "#0".to_string()), (true, "#1".to_string())],
        "the distance remembers setback and the angle slope",
    );
}

#[test]
fn a_diameter_written_from_a_variable_is_halved_onto_the_arc_a_cut_leaves() {
    let mut document = PartDocument::new("Came", at_nine());
    add(&mut document, "bore", 20.0);
    document.apply(Operation::CreateSketch {
        plane: WorkPlane::XY,
        on: None,
    });
    document.apply(Operation::AddCircle {
        sketch: 0,
        center: PointRef::Existing(cao_sketch::Sketch::ORIGIN),
        radius: 10.0,
        rim: vec![
            PointRef::New(DVec2::new(10.0, 0.0)),
            PointRef::New(DVec2::new(0.0, 10.0)),
        ],
        construction: false,
    });
    let bore = written(&document, "bore");
    document.apply(Operation::SetDimension {
        sketch: 0,
        target: DimensionTarget::Diameter(cao_sketch::CircleId(0)),
        value: bore,
        placement: None,
    });

    document.apply(Operation::TrimCircle {
        sketch: 0,
        circle: cao_sketch::CircleId(0),
        between: Some((cao_sketch::PointId(1), cao_sketch::PointId(2))),
    });

    assert_eq!(
        document.formula_of(0, DimensionTarget::ArcRadius(cao_sketch::ArcId(0))),
        Some("bore / 2".to_string()),
        "the arc's radius goes on following the bore, as half of it",
    );
}
