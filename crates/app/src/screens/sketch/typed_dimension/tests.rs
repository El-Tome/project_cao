//! What app · screens/sketch/typed_dimension.rs is held to.

use cao_part::{Operation, PointRef};
use cao_sketch::{ArcId, PointId, Sketch, WorkPlane};
use chrono::Utc;
use glam::DVec2;

use super::*;
use crate::lang::Catalogue;
use crate::screens::sketch::DimensionEdit;

#[test]
fn a_radius_a_fixed_chord_already_rules_out_warns_instead_of_bending_the_arc_silently() {
    let mut document = PartDocument::new("part", Utc::now());
    document.apply(Operation::CreateSketch {
        plane: WorkPlane::XY,
        on: None,
    });
    document.apply(Operation::AddArc {
        sketch: 0,
        center: PointRef::New(DVec2::new(40.0, 30.0)),
        start: PointRef::Existing(Sketch::ORIGIN),
        end: PointRef::New(DVec2::new(80.0, 0.0)),
        construction: false,
    });
    document.apply(Operation::SetDimension {
        sketch: 0,
        target: DimensionTarget::Distance {
            from: Sketch::ORIGIN,
            to: PointId(2),
        },
        value: 80.0,
        placement: None,
    });

    let mut editor = SketchEditor::default();
    let target = DimensionTarget::ArcRadius(ArcId(0));
    editor.editing = Some(DimensionEdit {
        target,
        input: "5".to_string(),
        focus: false,
    });
    let lang = Catalogue::french();

    let applied = apply_dimension_value(&mut document, &mut editor, 0, target, &lang);

    assert!(!applied, "refused: the field stays open for another try");
    assert_eq!(
        editor.message.as_deref(),
        Some(crate::wording::dimension::conflict_warning(&lang).as_str()),
    );
    assert!(
        document.sketches()[0].dimension_of(target).is_none(),
        "the refused value is not kept"
    );
}

#[test]
fn an_angle_between_two_traits_typed_at_a_half_turn_is_refused() {
    let mut document = PartDocument::new("part", Utc::now());
    document.apply(Operation::CreateSketch {
        plane: WorkPlane::XY,
        on: None,
    });
    for (from, to) in [
        (DVec2::new(10.0, 3.0), DVec2::new(30.0, 10.0)),
        (DVec2::new(10.0, 20.0), DVec2::new(30.0, 30.0)),
    ] {
        document.apply(Operation::AddSegment {
            sketch: 0,
            start: PointRef::New(from),
            end: PointRef::New(to),
            construction: false,
        });
    }
    let target = DimensionTarget::AngleBetween {
        first: cao_sketch::SegmentId(0),
        first_toward: cao_sketch::Toward::End,
        second: cao_sketch::SegmentId(1),
        second_toward: cao_sketch::Toward::End,
    };
    document.apply(Operation::SetDimension {
        sketch: 0,
        target,
        value: 10.0,
        placement: None,
    });

    let mut editor = SketchEditor {
        editing: Some(DimensionEdit {
            target,
            input: "180".to_string(),
            focus: false,
        }),
        ..SketchEditor::default()
    };
    let lang = Catalogue::french();

    let applied = apply_dimension_value(&mut document, &mut editor, 0, target, &lang);

    assert!(!applied, "refused: the field stays open for another try");
    assert_eq!(
        editor.message.as_deref(),
        Some(lang.t("sketch.angle_would_lay_parallel").as_str()),
        "and the reason is given",
    );
    assert!(
        (document.sketches()[0]
            .dimension_of(target)
            .expect("still there")
            .value
            - 10.0)
            .abs()
            < 1e-9,
        "the angle already laid keeps the value it had",
    );
}
