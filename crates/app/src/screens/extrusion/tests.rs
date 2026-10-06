//! What app · screens/extrusion.rs is held to.
//!
//! Closes #175.
//! - a formula can be typed as a revolution's angle —
//!   `a_revolution_swept_by_a_formula_keeps_it_turned_the_way_asked`
//! - one that does not read says why where it is typed —
//!   `a_distance_that_does_not_read_says_why_and_is_not_ready`
//!
//! Closes #526.
//! - an extrusion the exact kernel declines says so, rather than that it made
//!   nothing — `an_extrusion_the_kernel_declines_says_so_rather_than_that_it_made_nothing`
//! - an exact cut that misses the matter still says it removed nothing, though
//!   the kernel handed back a body built anew —
//!   `a_cut_that_misses_the_exact_matter_says_it_removed_nothing`
//!
//! Since #533 an area across its axis is turned on both sides, so a revolution
//! that changes nothing no longer blames the axis —
//! `a_revolution_across_its_axis_makes_matter_and_has_nothing_to_say`,
//! `a_revolution_that_changes_nothing_says_so_without_blaming_the_axis`.

use cao_part::{Operation, PartDocument, PointRef};
use cao_sketch::WorkPlane;
use chrono::Utc;
use glam::DVec3;

use super::*;

fn a_part_with_a_square() -> PartDocument {
    let mut document = PartDocument::new("part", Utc::now());
    document.apply(Operation::CreateSketch {
        plane: WorkPlane::XY,
        on: None,
    });
    document.apply(Operation::AddRectangle {
        sketch: 0,
        corner: PointRef::New(DVec2::new(-10.0, -10.0)),
        opposite: PointRef::New(DVec2::new(10.0, 10.0)),
        construction: false,
    });
    document
}

fn armed_on(picks: Vec<DVec2>) -> ExtrusionState {
    let mut extrusion = ExtrusionState::default();
    extrusion.offer(0);
    extrusion.picks = picks;
    extrusion.arm(ExtrusionMode::Add);
    extrusion
}

#[test]
fn an_extrusion_that_made_no_matter_says_so_in_the_sentence_it_was_handed() {
    let lang = Catalogue::french();
    let mut document = a_part_with_a_square();
    let mut extrusion = armed_on(vec![DVec2::new(500.0, 500.0)]);
    let mut notice = None;

    assert!(apply_extrusion(
        &mut document,
        &mut extrusion,
        &mut notice,
        &lang
    ));
    assert_eq!(notice, Some(lang.t("extrusion.nothing_added")));
}

#[test]
fn an_extrusion_that_made_matter_leaves_nothing_to_say() {
    let lang = Catalogue::french();
    let mut document = a_part_with_a_square();
    let mut extrusion = armed_on(vec![DVec2::ZERO]);
    let mut notice = None;

    assert!(apply_extrusion(
        &mut document,
        &mut extrusion,
        &mut notice,
        &lang
    ));
    assert_eq!(notice, None);
}

#[test]
fn an_extrusion_speaks_where_the_drawing_did_rather_than_beside_it() {
    let lang = Catalogue::french();
    let mut document = a_part_with_a_square();
    let mut extrusion = armed_on(vec![DVec2::ZERO]);
    let mut notice = Some(lang.t("sketch.click_a_trait"));

    apply_extrusion(&mut document, &mut extrusion, &mut notice, &lang);

    assert_eq!(notice, None);
}

fn a_table_holding_a_turn() -> Variables {
    let mut variables = Variables::default();
    variables.change(&cao_part::VariableChange::Added {
        name: "turn".to_string(),
        formula: Formula::Number(90.0),
    });
    variables
}

#[test]
fn a_revolution_swept_by_a_formula_keeps_it_turned_the_way_asked() {
    let variables = a_table_holding_a_turn();
    let mut extrusion = armed_on(vec![DVec2::ZERO]);
    extrusion.shape = Shape::Revolution;
    extrusion.angle_input = "=turn * 2".to_string();
    extrusion.reversed = true;

    let (written, value) = extrusion.angle(&variables).expect("it reads");

    assert_eq!(variables.written(&written), "-(turn * 2)");
    assert!((value + 180.0).abs() < 1e-9, "{value}");
    assert!(extrusion.is_ready(&variables));
}

#[test]
fn a_distance_that_does_not_read_says_why_and_is_not_ready() {
    let variables = a_table_holding_a_turn();
    let mut extrusion = armed_on(vec![DVec2::ZERO]);
    extrusion.distance_input = "depth * 2".to_string();

    assert_eq!(
        extrusion.wrong(&variables),
        Some(cao_part::Unusable::Unreadable(
            cao_part::Unreadable::UnknownName("depth".to_string())
        )),
    );
    assert!(!extrusion.is_ready(&variables));
}

/// The square raised 10 into a block, then a circle on a plane tilted 30°
/// about X standing over the block's top: raised down its normal, the
/// cylinder meets that top along an ellipse the exact kernel does not draw.
fn a_block_under_a_slanted_circle() -> PartDocument {
    let mut document = a_part_with_a_square();
    document.apply(Operation::Extrude {
        sketch: 0,
        areas: document.areas_at(0, &[DVec2::ZERO]),
        distance: 10.0.into(),
        mode: ExtrusionMode::Add,
    });
    let (sin, cos) = (std::f64::consts::PI / 6.0).sin_cos();
    document.apply(Operation::CreateSketch {
        plane: WorkPlane {
            origin: DVec3::new(0.0, 0.0, 12.0),
            u: DVec3::X,
            v: DVec3::new(0.0, cos, sin),
        },
        on: None,
    });
    document.apply(Operation::AddCircle {
        sketch: 1,
        center: PointRef::New(DVec2::ZERO),
        radius: 2.0,
        rim: Vec::new(),
        construction: false,
    });
    document
}

#[test]
fn an_extrusion_the_kernel_declines_says_so_rather_than_that_it_made_nothing() {
    let lang = Catalogue::french();
    let mut document = a_block_under_a_slanted_circle();
    let mut extrusion = ExtrusionState::default();
    extrusion.offer(1);
    extrusion.picks = vec![DVec2::ZERO];
    extrusion.reversed = true;
    extrusion.arm(ExtrusionMode::Add);
    let mut notice = None;

    assert!(apply_extrusion(
        &mut document,
        &mut extrusion,
        &mut notice,
        &lang
    ));
    assert_eq!(notice, Some(lang.t("extrusion.declined")));
}

#[test]
fn a_cut_that_misses_the_exact_matter_says_it_removed_nothing() {
    let lang = Catalogue::french();
    let mut document = a_part_with_a_square();
    document.apply(Operation::Extrude {
        sketch: 0,
        areas: document.areas_at(0, &[DVec2::ZERO]),
        distance: 10.0.into(),
        mode: ExtrusionMode::Add,
    });
    document.apply(Operation::CreateSketch {
        plane: WorkPlane::XY,
        on: None,
    });
    document.apply(Operation::AddRectangle {
        sketch: 1,
        corner: PointRef::New(DVec2::new(10.0, -5.0)),
        opposite: PointRef::New(DVec2::new(20.0, 5.0)),
        construction: false,
    });
    let mut extrusion = ExtrusionState::default();
    extrusion.offer(1);
    extrusion.picks = vec![DVec2::new(15.0, 0.0)];
    extrusion.arm(ExtrusionMode::Cut);
    let mut notice = None;

    apply_extrusion(&mut document, &mut extrusion, &mut notice, &lang);

    assert_eq!(notice, Some(lang.t("extrusion.nothing_removed")));
}

fn armed_to_turn(picks: Vec<DVec2>, mode: ExtrusionMode) -> ExtrusionState {
    let mut extrusion = ExtrusionState::default();
    extrusion.offer(0);
    extrusion.shape = Shape::Revolution;
    extrusion.picks = picks;
    extrusion.arm(mode);
    extrusion
}

#[test]
fn a_revolution_across_its_axis_makes_matter_and_has_nothing_to_say() {
    let lang = Catalogue::french();
    let mut document = a_part_with_a_square();
    let mut extrusion = armed_to_turn(vec![DVec2::ZERO], ExtrusionMode::Add);
    let mut notice = None;

    apply_extrusion(&mut document, &mut extrusion, &mut notice, &lang);

    assert_eq!(notice, None);
    assert!(document.body().volume() > 0.0, "the square turned about V");
}

#[test]
fn a_revolution_that_changes_nothing_says_so_without_blaming_the_axis() {
    let lang = Catalogue::french();
    let mut document = a_part_with_a_square();
    let mut extrusion = armed_to_turn(vec![DVec2::ZERO], ExtrusionMode::Cut);
    let mut notice = None;

    apply_extrusion(&mut document, &mut extrusion, &mut notice, &lang);

    let said = notice.expect("a cut out of nothing says so");
    assert_eq!(said, lang.t("extrusion.nothing_from_revolution"));
    assert!(
        !said.contains("axe"),
        "an area across its axis is turned, so the axis is no reason: {said}",
    );
}
