//! What app · screens/extrusion.rs is held to.

use cao_part::{Operation, PartDocument, PointRef};
use cao_sketch::WorkPlane;
use chrono::Utc;

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
