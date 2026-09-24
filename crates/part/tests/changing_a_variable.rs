//! Changing the part's variables through the part: what is refused, and why,
//! and that nothing is applied when it is.
//!
//! Closes #175.
//! - a loop between variables is refused, naming the variables in it —
//!   `a_formula_that_would_lean_on_itself_is_refused_naming_the_loop`
//! - renaming a variable renames it in every formula that uses it —
//!   `renaming_a_variable_renames_it_in_what_was_written_from_it`
//! - a variable change that would break a size — a length at zero or below,
//!   a count no longer whole and positive, a dimension the drawing can no
//!   longer hold — is refused naming it —
//!   `a_change_that_would_leave_a_length_at_zero_or_below_is_refused_naming_it`,
//!   `a_change_that_would_leave_a_count_no_longer_whole_is_refused_naming_the_pattern`,
//!   `a_change_the_drawing_could_not_hold_is_refused_naming_the_value`,
//!   `a_change_that_would_leave_a_step_with_no_size_is_refused_naming_it`,
//!   `a_variable_that_would_come_to_no_number_is_refused`,
//!   `a_count_that_would_renumber_what_was_drawn_after_the_pattern_is_refused`,
//!   `a_count_is_held_while_anything_follows_the_pattern_and_the_refusal_says_what`,
//!   `a_count_in_a_sketch_matter_was_raised_from_is_refused_naming_the_step_raised`,
//!   `a_count_that_would_take_away_an_extruded_copy_is_refused_naming_the_extrusion`,
//!   `a_change_that_would_take_a_sketch_off_its_face_is_refused_naming_the_sketch`,
//!   `a_chamfer_already_short_of_a_corner_that_would_cut_fewer_is_refused`
//! - changing a variable changes every size written from it — and only those
//!   still written from it: a value taken away from the drawing stops
//!   following it —
//!   `a_count_on_the_last_thing_drawn_changes_how_many_copies_stand`,
//!   `a_value_erased_from_the_drawing_stops_following_its_variable`
//! - deleting a variable something uses is refused, with the list of what
//!   uses it — the dimensions, the features, the other variables —
//!   `a_variable_in_use_cannot_be_erased_and_says_what_uses_it`,
//!   `a_step_written_from_a_variable_is_named_as_using_it`,
//!   `a_value_laid_inside_a_gesture_is_named_as_the_value_and_not_as_the_gesture`
//! - changing a variable is one step of the history —
//!   `a_variable_added_through_the_part_is_one_step_undo_takes_back`
//! - compacting the history keeps the variables and the formulas that refer
//!   to them — `compacting_keeps_the_variables_and_what_is_written_from_them`,
//!   `compacting_leaves_an_erased_variable_out_and_the_others_still_answer`,
//!   `compacting_keeps_a_pattern_written_from_a_variable_following_it`

use cao_part::history::{
    ChamferAsked, ExtrusionMode, FaceAnchor, Operation, PointRef, RepeatsAsked,
};
use cao_part::{
    Broken, Formula, NameProblem, PartDocument, Refused, Use, VariableChange, VariableId,
};
use cao_sketch::{ChosenAxis, Corner, DimensionTarget, Element, SegmentId, SketchAxis, WorkPlane};
use glam::{DVec2, DVec3};

fn at_nine() -> chrono::DateTime<chrono::Utc> {
    "2026-01-02T09:00:00Z".parse().expect("a date")
}

fn written(document: &PartDocument, text: &str) -> Formula {
    document
        .variables()
        .read(text)
        .expect("a formula that reads")
}

fn added(name: &str, formula: Formula) -> VariableChange {
    VariableChange::Added {
        name: name.to_string(),
        formula,
    }
}

fn edited(variable: usize, name: &str, formula: Formula) -> VariableChange {
    VariableChange::Edited {
        variable: VariableId(variable),
        name: name.to_string(),
        formula,
    }
}

const HEIGHT: DimensionTarget = DimensionTarget::Length(SegmentId(1));

/// `height` at 40 and `thickness` at 4, a rectangle whose right side is as
/// high as `height`, raised by `thickness`.
fn plate() -> PartDocument {
    let mut document = PartDocument::new("Plate", at_nine());
    document
        .change_variable(added("height", Formula::Number(40.0)))
        .expect("a first variable");
    document
        .change_variable(added("thickness", Formula::Number(4.0)))
        .expect("a second one");
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
        target: HEIGHT,
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

#[test]
fn a_variable_added_through_the_part_is_one_step_undo_takes_back() {
    let mut document = PartDocument::new("Plate", at_nine());

    document
        .change_variable(added("width", Formula::Number(120.0)))
        .expect("nothing to refuse");
    assert_eq!(document.variables().named("width"), Some(VariableId(0)));
    document.undo();

    assert_eq!(document.variables().named("width"), None);
}

#[test]
fn a_name_that_does_not_do_is_refused_and_nothing_is_applied() {
    let mut document = plate();
    let before = document.history.operations().len();

    let refused = document.change_variable(added("height", Formula::Number(1.0)));

    assert_eq!(refused, Err(Refused::Name(NameProblem::Taken)));
    assert_eq!(document.history.operations().len(), before);
}

#[test]
fn a_formula_that_would_lean_on_itself_is_refused_naming_the_loop() {
    let mut document = plate();
    let from_height = written(&document, "height / 10");
    document
        .change_variable(edited(1, "thickness", from_height))
        .expect("thickness may lean on height");
    let from_thickness = written(&document, "thickness * 10");

    let refused = document.change_variable(edited(0, "height", from_thickness));

    assert_eq!(
        refused,
        Err(Refused::Loop(vec![VariableId(0), VariableId(1)]))
    );
}

#[test]
fn a_variable_in_use_cannot_be_erased_and_says_what_uses_it() {
    let mut document = plate();
    let from_height = written(&document, "height * 2");
    document
        .change_variable(added("double", from_height))
        .expect("a variable written from another");

    let refused = document.change_variable(VariableChange::Erased {
        variable: VariableId(0),
    });

    let Err(Refused::InUse(uses)) = refused else {
        panic!("erasing a variable in use is refused, not {refused:?}");
    };
    assert!(
        uses.contains(&Use::Dimension {
            sketch: 0,
            target: HEIGHT
        }),
        "{uses:?}"
    );
    assert!(uses.contains(&Use::Variable(VariableId(2))), "{uses:?}");
    assert!(document.variables().named("height").is_some());
}

#[test]
fn a_step_written_from_a_variable_is_named_as_using_it() {
    let mut document = plate();

    let refused = document.change_variable(VariableChange::Erased {
        variable: VariableId(1),
    });

    let Err(Refused::InUse(uses)) = refused else {
        panic!("the extrusion uses thickness, so it stays: {refused:?}");
    };
    assert!(
        matches!(uses.as_slice(), [Use::Operation(_)]),
        "the extrusion, and nothing else: {uses:?}"
    );
}

#[test]
fn a_variable_nothing_uses_is_erased() {
    let mut document = plate();
    document
        .change_variable(added("margin", Formula::Number(5.0)))
        .expect("a variable nothing uses yet");

    document
        .change_variable(VariableChange::Erased {
            variable: VariableId(2),
        })
        .expect("nothing uses it");

    assert_eq!(document.variables().named("margin"), None);
}

#[test]
fn renaming_a_variable_renames_it_in_what_was_written_from_it() {
    let mut document = plate();

    document
        .change_variable(edited(0, "h", Formula::Number(40.0)))
        .expect("a new name for the same variable");

    assert_eq!(document.formula_of(0, HEIGHT), Some("h".to_string()));
}

#[test]
fn a_change_that_would_leave_a_length_at_zero_or_below_is_refused_naming_it() {
    let mut document = plate();

    let refused = document.change_variable(edited(0, "height", Formula::Number(-5.0)));

    assert_eq!(
        refused,
        Err(Refused::Breaks(vec![Broken::Dimension {
            sketch: 0,
            target: HEIGHT
        }]))
    );
    assert_eq!(
        document.measured(0, HEIGHT).map(f64::round),
        Some(40.0),
        "nothing moved"
    );
}

#[test]
fn a_change_that_would_leave_a_step_with_no_size_is_refused_naming_it() {
    let mut document = plate();

    let refused = document.change_variable(edited(1, "thickness", Formula::Number(0.0)));

    assert!(
        matches!(&refused, Err(Refused::Breaks(broken)) if matches!(broken.as_slice(), [Broken::Operation(_)])),
        "{refused:?}"
    );
}

#[test]
fn a_variable_that_would_come_to_no_number_is_refused() {
    let mut document = plate();
    let divided = written(&document, "10 / (height - 40)");

    let refused = document.change_variable(added("endless", divided));

    assert_eq!(
        refused,
        Err(Refused::Breaks(vec![Broken::Variable(VariableId(2))]))
    );
}

#[test]
fn a_value_laid_inside_a_gesture_is_named_as_the_value_and_not_as_the_gesture() {
    let mut document = plate();
    let from_height = written(&document, "height / 2");
    document.apply(Operation::Gesture(vec![
        Operation::AddSegment {
            sketch: 0,
            start: PointRef::New(DVec2::new(10.0, 50.0)),
            end: PointRef::New(DVec2::new(30.0, 50.0)),
            construction: false,
        },
        Operation::SetDimension {
            sketch: 0,
            target: DimensionTarget::Length(SegmentId(4)),
            value: from_height,
            placement: None,
        },
    ]));

    let uses = document.uses_of(VariableId(0));

    assert!(
        !uses.iter().any(|used| matches!(used, Use::Operation(_))),
        "{uses:?}"
    );
    assert!(uses.contains(&Use::Dimension {
        sketch: 0,
        target: DimensionTarget::Length(SegmentId(4))
    }));
}

fn thickness(document: &PartDocument) -> f64 {
    let (min, max) = document.body().bounds().expect("some matter");
    max.z - min.z
}

#[test]
fn compacting_keeps_the_variables_and_what_is_written_from_them() {
    let mut document = plate();

    document.compact_history();
    document
        .change_variable(edited(0, "height", Formula::Number(60.0)))
        .expect("the variable is still there to change");
    document
        .change_variable(edited(1, "thickness", Formula::Number(10.0)))
        .expect("and so is the other");

    assert_eq!(document.formula_of(0, HEIGHT), Some("height".to_string()));
    assert_eq!(document.measured(0, HEIGHT).map(f64::round), Some(60.0));
    assert!((thickness(&document) - 10.0).abs() < 1e-6);
}

#[test]
fn compacting_leaves_an_erased_variable_out_and_the_others_still_answer() {
    let mut document = PartDocument::new("Plate", at_nine());
    document
        .change_variable(added("old", Formula::Number(1.0)))
        .expect("a variable soon erased");
    document
        .change_variable(VariableChange::Erased {
            variable: VariableId(0),
        })
        .expect("nothing uses it");
    document
        .change_variable(added("height", Formula::Number(40.0)))
        .expect("the one that stays");
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
        target: HEIGHT,
        value: height,
        placement: None,
    });

    document.compact_history();

    assert_eq!(document.variables().live().count(), 1);
    assert_eq!(document.formula_of(0, HEIGHT), Some("height".to_string()));
    let height = document.variables().named("height").expect("still there");
    document
        .change_variable(VariableChange::Edited {
            variable: height,
            name: "height".to_string(),
            formula: Formula::Number(55.0),
        })
        .expect("it still answers");
    assert_eq!(document.measured(0, HEIGHT).map(f64::round), Some(55.0));
}

#[test]
fn a_change_that_would_leave_a_count_no_longer_whole_is_refused_naming_the_pattern() {
    let mut document = plate();
    document
        .change_variable(added("holes", Formula::Number(8.0)))
        .expect("a count to write a pattern from");
    let count = written(&document, "holes / 2");
    document.apply(Operation::CircularPattern {
        sketch: 0,
        elements: vec![cao_sketch::Element::Segment(SegmentId(0))],
        centre: cao_sketch::Sketch::ORIGIN,
        degrees: 90.0.into(),
        count,
    });

    let refused = document.change_variable(edited(2, "holes", Formula::Number(5.0)));

    assert!(
        matches!(&refused, Err(Refused::Breaks(broken)) if matches!(broken.as_slice(), [Broken::Operation(_)])),
        "four copies would become two and a half: {refused:?}"
    );
}

#[test]
fn a_change_the_drawing_could_not_hold_is_refused_naming_the_value() {
    let mut document = PartDocument::new("Arc", at_nine());
    document
        .change_variable(added("radius", Formula::Number(50.0)))
        .expect("a radius");
    document.apply(Operation::CreateSketch {
        plane: WorkPlane::XY,
        on: None,
    });
    document.apply(Operation::AddArc {
        sketch: 0,
        center: PointRef::New(DVec2::new(40.0, 30.0)),
        start: PointRef::Existing(cao_sketch::Sketch::ORIGIN),
        end: PointRef::New(DVec2::new(80.0, 0.0)),
        construction: false,
    });
    let chord = DimensionTarget::Distance {
        from: cao_sketch::Sketch::ORIGIN,
        to: cao_sketch::PointId(2),
    };
    document.apply(Operation::SetDimension {
        sketch: 0,
        target: chord,
        value: 80.0.into(),
        placement: None,
    });
    let radius = DimensionTarget::ArcRadius(cao_sketch::ArcId(0));
    let written_radius = written(&document, "radius");
    document.apply(Operation::SetDimension {
        sketch: 0,
        target: radius,
        value: written_radius,
        placement: None,
    });

    // No arc through both ends of an 80 mm chord is smaller than a 40 mm
    // half circle.
    let refused = document.change_variable(edited(0, "radius", Formula::Number(5.0)));

    assert_eq!(
        refused,
        Err(Refused::Breaks(vec![Broken::Dimension {
            sketch: 0,
            target: radius
        }]))
    );
}

/// `copies` at 3, a trait turned three times about the origin, and — when
/// `then_a_trait` — a trait drawn after the pattern and dimensioned.
fn a_pattern_counted_by_a_variable(then_a_trait: bool) -> PartDocument {
    let mut document = PartDocument::new("Rosace", at_nine());
    document
        .change_variable(added("copies", Formula::Number(3.0)))
        .expect("a count");
    document.apply(Operation::CreateSketch {
        plane: WorkPlane::XY,
        on: None,
    });
    document.apply(Operation::AddSegment {
        sketch: 0,
        start: PointRef::New(DVec2::new(10.0, 0.0)),
        end: PointRef::New(DVec2::new(20.0, 0.0)),
        construction: false,
    });
    let count = written(&document, "copies");
    document.apply(Operation::CircularPattern {
        sketch: 0,
        elements: vec![cao_sketch::Element::Segment(SegmentId(0))],
        centre: cao_sketch::Sketch::ORIGIN,
        degrees: 30.0.into(),
        count,
    });
    if then_a_trait {
        document.apply(Operation::AddSegment {
            sketch: 0,
            start: PointRef::New(DVec2::new(50.0, -40.0)),
            end: PointRef::New(DVec2::new(80.0, -40.0)),
            construction: false,
        });
        document.apply(Operation::SetDimension {
            sketch: 0,
            target: DimensionTarget::Length(SegmentId(3)),
            value: 30.0.into(),
            placement: None,
        });
    }
    document
}

/// Nothing was undone, so the numbers run from one in the order things were
/// done: the count, the sketch, the trait, the pattern, and what follows.
const THE_PATTERN: u32 = 4;
const WHAT_FOLLOWS_IT: u32 = 5;

#[test]
fn a_count_that_would_renumber_what_was_drawn_after_the_pattern_is_refused() {
    let mut document = a_pattern_counted_by_a_variable(true);

    let refused = document.change_variable(edited(0, "copies", Formula::Number(4.0)));

    let renumbered = Broken::Renumbered {
        changed: THE_PATTERN,
        followed_by: WHAT_FOLLOWS_IT,
    };
    assert!(
        matches!(&refused, Err(Refused::Breaks(broken)) if broken.contains(&renumbered)),
        "the trait drawn after would be named by a copy's number: {refused:?}"
    );
    assert_eq!(document.sketches()[0].live_segments().count(), 4);
}

#[test]
fn a_count_is_held_while_anything_follows_the_pattern_and_the_refusal_says_what() {
    let mut document = a_pattern_counted_by_a_variable(false);
    document.apply(Operation::MoveDimension {
        sketch: 0,
        target: DimensionTarget::Length(SegmentId(0)),
        offset: DVec2::new(0.0, 5.0),
    });

    let refused = document.change_variable(edited(0, "copies", Formula::Number(4.0)));

    assert_eq!(
        refused,
        Err(Refused::Breaks(vec![Broken::Renumbered {
            changed: THE_PATTERN,
            followed_by: WHAT_FOLLOWS_IT,
        }])),
        "a label moved after the pattern names nothing the pattern lays, and \
         is still what the count is held by"
    );
}

fn raised_at(document: &PartDocument, place: DVec2) -> bool {
    document
        .body()
        .ray_hit(
            DVec3::new(place.x, place.y, 100.0),
            DVec3::new(0.0, 0.0, -1.0),
        )
        .is_some()
}

/// `n` at `along`, a square of 10 repeated `n` times along U and `across`
/// times across, 20 apart, then the pads at these places raised by 5. The
/// pattern is the fifth thing done and the extrusion the sixth.
fn a_grid_raised_at(along: f64, across: f64, pads: &[DVec2]) -> PartDocument {
    let mut document = PartDocument::new("Grille", at_nine());
    document
        .change_variable(added("n", Formula::Number(along)))
        .expect("a count");
    document.apply(Operation::CreateSketch {
        plane: WorkPlane::XY,
        on: None,
    });
    document.apply(Operation::AddRectangle {
        sketch: 0,
        corner: PointRef::New(DVec2::ZERO),
        opposite: PointRef::New(DVec2::new(10.0, 10.0)),
        construction: false,
    });
    document.apply(Operation::SetDimension {
        sketch: 0,
        target: DimensionTarget::Length(SegmentId(0)),
        value: 10.0.into(),
        placement: None,
    });
    let n = written(&document, "n");
    document.apply(Operation::RectangularPattern {
        sketch: 0,
        elements: (0..4)
            .map(|side| Element::Segment(SegmentId(side)))
            .collect(),
        direction: ChosenAxis::Sketch(SketchAxis::U),
        along: RepeatsAsked {
            step: 20.0.into(),
            count: n,
        },
        across: RepeatsAsked {
            step: 20.0.into(),
            count: across.into(),
        },
    });
    let areas = document.areas_at(0, pads);
    document.apply(Operation::Extrude {
        sketch: 0,
        areas,
        distance: 5.0.into(),
        mode: ExtrusionMode::Add,
    });
    document
}

const THE_GRID: u32 = 5;
const RAISED_FROM_IT: u32 = 6;

#[test]
fn a_count_in_a_sketch_matter_was_raised_from_is_refused_naming_the_step_raised() {
    let pads = [
        DVec2::new(5.0, 5.0),
        DVec2::new(25.0, 5.0),
        DVec2::new(5.0, 25.0),
        DVec2::new(25.0, 25.0),
    ];
    let mut document = a_grid_raised_at(2.0, 2.0, &pads);
    let raised: Vec<bool> = pads.iter().map(|pad| raised_at(&document, *pad)).collect();

    let refused = document.change_variable(edited(0, "n", Formula::Number(3.0)));

    let renumbered = Broken::Renumbered {
        changed: THE_GRID,
        followed_by: RAISED_FROM_IT,
    };
    assert!(
        matches!(&refused, Err(Refused::Breaks(broken)) if broken.contains(&renumbered)),
        "the extrusion names its pads by the rank of their sides: {refused:?}"
    );
    let still: Vec<bool> = pads.iter().map(|pad| raised_at(&document, *pad)).collect();
    assert_eq!(raised, still, "nothing was applied");
}

#[test]
fn a_count_that_would_take_away_an_extruded_copy_is_refused_naming_the_extrusion() {
    let mut document = a_grid_raised_at(3.0, 1.0, &[DVec2::new(45.0, 5.0)]);

    let refused = document.change_variable(edited(0, "n", Formula::Number(2.0)));

    assert!(
        matches!(&refused, Err(Refused::Breaks(broken)) if broken.contains(&Broken::Operation(RAISED_FROM_IT))),
        "the third copy along is gone, and the pad raised from it with it: {refused:?}"
    );
}

#[test]
fn a_change_that_would_take_a_sketch_off_its_face_is_refused_naming_the_sketch() {
    let mut document = PartDocument::new("Bloc", at_nine());
    document
        .change_variable(added("height", Formula::Number(10.0)))
        .expect("a height");
    document.apply(Operation::CreateSketch {
        plane: WorkPlane::XY,
        on: None,
    });
    document.apply(Operation::AddRectangle {
        sketch: 0,
        corner: PointRef::New(DVec2::ZERO),
        opposite: PointRef::New(DVec2::new(40.0, 20.0)),
        construction: false,
    });
    let areas = document.areas_at(0, &[DVec2::new(20.0, 10.0)]);
    let height = written(&document, "height");
    document.apply(Operation::Extrude {
        sketch: 0,
        areas,
        distance: height,
        mode: ExtrusionMode::Add,
    });
    document.apply(Operation::CreateSketch {
        plane: WorkPlane {
            origin: DVec3::new(0.0, 0.0, 10.0),
            u: DVec3::X,
            v: DVec3::Y,
        },
        on: Some(FaceAnchor {
            face: TOP_OF_THE_BLOCK,
            up: DVec3::Y,
        }),
    });

    let refused = document.change_variable(edited(0, "height", Formula::Number(0.0)));

    assert!(
        matches!(&refused, Err(Refused::Breaks(broken)) if broken.contains(&Broken::Adrift(1))),
        "a block of no height has no top for the second sketch: {refused:?}"
    );
}

/// The face the top of a block raised from XY is, as the replay numbers it.
const TOP_OF_THE_BLOCK: usize = 1;

#[test]
fn a_chamfer_already_short_of_a_corner_that_would_cut_fewer_is_refused() {
    let mut document = PartDocument::new("Plaques", at_nine());
    document
        .change_variable(added("reach", Formula::Number(15.0)))
        .expect("a reach");
    document.apply(Operation::CreateSketch {
        plane: WorkPlane::XY,
        on: None,
    });
    document.apply(Operation::AddRectangle {
        sketch: 0,
        corner: PointRef::New(DVec2::ZERO),
        opposite: PointRef::New(DVec2::new(10.0, 10.0)),
        construction: false,
    });
    document.apply(Operation::SetDimension {
        sketch: 0,
        target: DimensionTarget::Length(SegmentId(0)),
        value: 10.0.into(),
        placement: None,
    });
    document.apply(Operation::AddRectangle {
        sketch: 0,
        corner: PointRef::New(DVec2::new(100.0, 0.0)),
        opposite: PointRef::New(DVec2::new(140.0, 40.0)),
        construction: false,
    });
    let reach = written(&document, "reach");
    document.apply(Operation::Chamfer {
        sketch: 0,
        corners: vec![
            Corner::Between(SegmentId(0), SegmentId(1)),
            Corner::Between(SegmentId(4), SegmentId(5)),
        ],
        mode: ChamferAsked::Equal(reach),
    });
    document.apply(Operation::AddSegment {
        sketch: 0,
        start: PointRef::New(DVec2::new(200.0, 0.0)),
        end: PointRef::New(DVec2::new(250.0, 0.0)),
        construction: false,
    });

    let refused = document.change_variable(edited(0, "reach", Formula::Number(45.0)));

    let renumbered = Broken::Renumbered {
        changed: THE_CHAMFER,
        followed_by: THE_CHAMFER + 1,
    };
    assert!(
        matches!(&refused, Err(Refused::Breaks(broken)) if broken.contains(&renumbered)),
        "a chamfer that already missed the small square would miss the big one \
         too, and the trait after it would be named by another rank: {refused:?}"
    );
}

/// The count, the sketch, two squares and a value come before it.
const THE_CHAMFER: u32 = 6;

#[test]
fn a_count_on_the_last_thing_drawn_changes_how_many_copies_stand() {
    let mut document = a_pattern_counted_by_a_variable(false);

    document
        .change_variable(edited(0, "copies", Formula::Number(5.0)))
        .expect("nothing was drawn after the pattern");

    assert_eq!(document.sketches()[0].live_segments().count(), 5);
}

#[test]
fn compacting_keeps_a_pattern_written_from_a_variable_following_it() {
    let mut document = a_pattern_counted_by_a_variable(false);

    document.compact_history();

    let copies = document.variables().named("copies").expect("still there");
    assert!(
        !document.uses_of(copies).is_empty(),
        "the pattern still counts on it"
    );
    document
        .change_variable(VariableChange::Edited {
            variable: copies,
            name: "copies".to_string(),
            formula: Formula::Number(5.0),
        })
        .expect("the count still follows it");
    assert_eq!(document.sketches()[0].live_segments().count(), 5);
}

#[test]
fn a_value_erased_from_the_drawing_stops_following_its_variable() {
    let mut document = PartDocument::new("Plate", at_nine());
    document
        .change_variable(added("a", Formula::Number(50.0)))
        .expect("a length");
    document.apply(Operation::CreateSketch {
        plane: WorkPlane::XY,
        on: None,
    });
    document.apply(Operation::AddSegment {
        sketch: 0,
        start: PointRef::New(DVec2::new(0.0, 0.0)),
        end: PointRef::New(DVec2::new(100.0, 0.0)),
        construction: false,
    });
    document.apply(Operation::SetDimension {
        sketch: 0,
        target: DimensionTarget::Length(SegmentId(0)),
        value: 100.0.into(),
        placement: None,
    });
    document.apply(Operation::AddSegment {
        sketch: 0,
        start: PointRef::New(DVec2::new(0.0, 10.0)),
        end: PointRef::New(DVec2::new(40.0, 10.0)),
        construction: false,
    });
    let from_a = written(&document, "a");
    let side = DimensionTarget::Length(SegmentId(1));
    document.apply(Operation::SetDimension {
        sketch: 0,
        target: side,
        value: from_a,
        placement: None,
    });
    document.apply(Operation::EraseMany {
        sketch: 0,
        elements: Vec::new(),
        dimensions: vec![side],
        constraints: Vec::new(),
    });

    document
        .change_variable(edited(0, "a", Formula::Number(80.0)))
        .expect("nothing on the drawing uses a");

    let length = document.sketches()[0].segment_length(SegmentId(1)) * document.scale();
    assert!(
        (length - 50.0).abs() < 1e-6,
        "the trait kept the length it had when its value was taken away: {length}"
    );
}
