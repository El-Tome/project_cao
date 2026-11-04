//! What app · screens/viewport/input/corner.rs is held to.
//!
//! Closes #315.
//! - clicking a corner point takes that corner, with the fillet and with the
//!   chamfer in equal distances — `a_click_on_a_corner_point_names_both_its_sides`
//! - clicking a corner point again drops it — `a_second_click_on_a_corner_drops_it`
//! - the same corner cannot be taken twice, however it is named —
//!   `the_same_corner_named_the_other_way_round_is_still_that_corner`
//! - a corner is let go of by one click on its point, whatever the mode —
//!   `a_corner_named_by_its_two_traits_is_let_go_of_by_its_point`,
//!   `a_point_holding_no_corner_lets_go_of_nothing`
//! - a point where more than two traits meet cannot be taken as a corner: the
//!   tool asks for the two traits instead —
//!   `a_click_on_a_point_too_crowded_to_be_a_corner_asks_for_the_two_traits`
//! - the chamfer in distance and angle, or in two distances, still takes a side
//!   and then the other, though it gathers as many corners as the rest —
//!   `an_asymmetric_mode_reads_a_click_as_a_side_even_on_a_corner_point`
//! - changing the chamfer's mode to one of those says to click one side and
//!   then the other — `each_mode_asks_for_what_it_can_actually_take`
//! - changing the mode empties the corners taken — no test: `reset_pending`
//!   already empties the tool state on every mode change, and it lives in a
//!   file `docs/code-map.md` lists as having no net; taking that file off the
//!   list on the strength of one test would overstate what covers it
//! - `Entrée` that cannot lay anything says why, instead of doing nothing —
//!   `enter_with_half_a_corner_asks_for_the_other_side_rather_than_dropping_it`,
//!   `enter_with_no_side_clicked_asks_for_a_corner`,
//!   `enter_with_a_corner_taken_cuts_it`
//!
//! Closes #175.
//! - a formula can be typed as a chamfer's and a fillet's values —
//!   `a_chamfer_typed_from_variables_is_laid_as_it_was_written`,
//!   `a_fillet_typed_as_a_plain_number_is_laid_as_that_number`
//!
//! The tool's share of #619, whose criteria live with the sketch's tests in
//! `crates/sketch/tests/several_cuts_stand_on_one_point.rs`: a point three
//! traits meet at asks for the two traits even once a pair is cut —
//! `a_click_on_a_point_three_traits_meet_at_asks_for_the_traits_once_a_pair_is_cut`
//! — two pairs of one point are two corners —
//! `two_pairs_of_one_point_are_taken_in_one_gesture` — and a click on that
//! point lets go of neither — `a_click_on_a_point_two_corners_are_taken_at_lets_go_of_neither`.
//!
//! The tool's share of #621, whose criteria live with the part's tests in
//! `crates/part/tests/a_fillet_rounds_a_corner_with_a_curve.rs`: a click on
//! the point a trait and a curve end at takes their corner —
//! `a_click_on_the_point_a_trait_and_an_arc_end_at_names_both_its_sides` — a
//! click on a curve names it as one side —
//! `a_click_on_an_arc_away_from_its_ends_names_that_side` — a chamfer there is
//! refused with a sentence of its own —
//! `a_chamfer_on_a_corner_with_a_curve_says_why_it_is_refused` — and a value
//! too big is held back —
//! `a_fillet_too_big_beside_an_arc_is_held_back_and_the_field_offers_the_largest`.
//!
//! The tool's share of #617, whose criteria live with the part's tests in
//! `crates/part/tests/a_cut_is_drawn_from_its_sharp_corner.rs`: a value too big
//! is held back with the largest offered —
//! `a_value_too_big_is_held_back_and_the_field_offers_the_largest` — and the
//! « Construction » toggle decides what the cut is laid as —
//! `a_cut_laid_while_construction_is_on_is_laid_in_construction`.

use cao_part::history::ChamferAsked;
use cao_part::{Formula, Operation, Variables};
use cao_sketch::{Chamfer, Corner, CutSide, LockedInput, PointId, SegmentId, Sketch, WorkPlane};
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
            half: Some(CutSide::Trait(SegmentId(1))),
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
            taken: vec![Corner::Between(SegmentId(1).into(), SegmentId(2).into())],
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
            half: Some(CutSide::Trait(SegmentId(3))),
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
        read == CornerClick::Side(CutSide::Trait(along))
            || read == CornerClick::Side(CutSide::Trait(up)),
        "a corner taken by its point has no first side, and these modes need \
         one — got {read:?}"
    );
}

#[test]
fn a_click_on_a_trait_away_from_any_point_names_that_side() {
    let (sketch, along, _up, _pivot) = a_right_angle();

    assert_eq!(
        clicked(&sketch, CORNER + DVec2::new(5.0, 0.0), SNAP, true),
        CornerClick::Side(CutSide::Trait(along))
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
    let (sketch, _along, _up, pivot) = a_right_angle();
    let taken = vec![Corner::At(pivot)];

    assert!(
        after_clicking(&sketch, taken.clone(), Corner::At(pivot)).is_empty(),
        "a corner clicked twice is a corner the user changed their mind about"
    );
}

#[test]
fn the_same_corner_named_the_other_way_round_is_still_that_corner() {
    let (sketch, along, up, pivot) = a_right_angle();
    let taken = vec![Corner::Between(along.into(), up.into())];

    for again in [
        Corner::Between(up.into(), along.into()),
        Corner::At(pivot),
        Corner::Between(along.into(), up.into()),
    ] {
        assert!(
            after_clicking(&sketch, taken.clone(), again).is_empty(),
            "naming it {again:?} is naming the same corner, and taking it \
             twice would cut it twice"
        );
    }
}

#[test]
fn two_pairs_of_one_point_are_taken_in_one_gesture() {
    let (mut sketch, along, up, pivot) = a_right_angle();
    let away = sketch.add_point(CORNER + DVec2::new(-10.0, -10.0));
    let back = sketch.add_segment(pivot, away);
    let taken = vec![Corner::Between(along.into(), up.into())];

    let both = after_clicking(&sketch, taken, Corner::Between(back.into(), up.into()));

    assert_eq!(
        both,
        vec![
            Corner::Between(along.into(), up.into()),
            Corner::Between(back.into(), up.into())
        ],
        "two other traits of the same point are another corner"
    );
    assert_eq!(
        after_clicking(&sketch, both, Corner::Between(up.into(), along.into())),
        vec![Corner::Between(back.into(), up.into())],
        "and each is let go of on its own"
    );
}

#[test]
fn a_click_on_a_point_two_corners_are_taken_at_lets_go_of_neither() {
    let (mut sketch, along, up, pivot) = a_right_angle();
    let away = sketch.add_point(CORNER + DVec2::new(-10.0, -10.0));
    let back = sketch.add_segment(pivot, away);
    let both = vec![
        Corner::Between(along.into(), up.into()),
        Corner::Between(back.into(), up.into()),
    ];
    let mut taken = both.clone();

    assert!(!drops_at(&sketch, &mut taken, pivot));
    assert_eq!(taken, both, "the point cannot say which pair is meant");
    assert_eq!(
        clicked(&sketch, sketch.point(pivot), SNAP, true),
        CornerClick::Crowded,
        "the click goes on to ask for the traits"
    );
}

#[test]
fn a_click_on_a_point_three_traits_meet_at_asks_for_the_traits_once_a_pair_is_cut() {
    let (mut sketch, along, up, pivot) = a_right_angle();
    let away = sketch.add_point(CORNER + DVec2::new(-10.0, -10.0));
    sketch.add_segment(pivot, away);
    sketch
        .cut_corner(
            along,
            up,
            cao_sketch::CutShape::Fillet { radius: 1.0 },
            false,
            0,
        )
        .expect("a corner of the point");

    assert_eq!(
        clicked(&sketch, sketch.point(pivot), SNAP, true),
        CornerClick::Crowded,
        "a pair already cut still leaves the point two others to choose from"
    );
}

/// What the list of corners becomes when that one is clicked.
fn after_clicking(sketch: &Sketch, mut taken: Vec<Corner>, corner: Corner) -> Vec<Corner> {
    toggle(sketch, &mut taken, corner);
    taken
}

#[test]
fn a_corner_named_by_its_two_traits_is_let_go_of_by_its_point() {
    let (sketch, along, up, pivot) = a_right_angle();
    let mut taken = vec![Corner::Between(along.into(), up.into())];

    assert!(
        drops_at(&sketch, &mut taken, pivot),
        "the modes that measure from a first side cannot be shown a point to \
         take a corner, but letting one go asks nothing of the mode"
    );
    assert!(taken.is_empty());
}

#[test]
fn a_point_holding_no_corner_lets_go_of_nothing() {
    let (sketch, along, up, pivot) = a_right_angle();
    let far = sketch.segments()[along.0].end;
    let mut taken = vec![Corner::Between(along.into(), up.into())];

    assert!(!drops_at(&sketch, &mut taken, far));
    assert_eq!(
        taken,
        vec![Corner::Between(along.into(), up.into())],
        "a click elsewhere is a click elsewhere, and goes on to name a side"
    );
    assert!(drops_at(&sketch, &mut taken, pivot));
}

#[test]
fn a_chamfer_typed_from_variables_is_laid_as_it_was_written() {
    let mut variables = cao_part::Variables::default();
    variables.change(&cao_part::VariableChange::Added {
        name: "setback".to_string(),
        formula: Formula::Number(3.0),
    });
    let setback = variables.read("setback").expect("it reads");
    let slope = variables.read("setback * 10").expect("it reads");

    let laid = laying(
        0,
        vec![Corner::Between(SegmentId(0).into(), SegmentId(1).into())],
        Chamfer::Angled {
            along: 3.0,
            degrees: 30.0,
        },
        false,
        [Some(setback.clone()), Some(slope.clone())],
        false,
    );

    assert!(
        matches!(
            laid,
            Operation::Chamfer {
                mode: ChamferAsked::Angled { ref along, ref degrees },
                ..
            } if *along == setback && *degrees == slope
        ),
        "{laid:?}"
    );
}

#[test]
fn a_fillet_typed_as_a_plain_number_is_laid_as_that_number() {
    let laid = laying(
        0,
        vec![Corner::Between(SegmentId(0).into(), SegmentId(1).into())],
        Chamfer::Equal(2.0),
        true,
        [Some(Formula::Number(2.0)), None],
        false,
    );

    assert!(
        matches!(laid, Operation::Fillet { ref radius, .. } if *radius == Formula::Number(2.0)),
        "{laid:?}"
    );
}

#[test]
fn a_cut_laid_while_construction_is_on_is_laid_in_construction() {
    let laid = laying(
        0,
        vec![Corner::Between(SegmentId(0).into(), SegmentId(1).into())],
        Chamfer::Equal(2.0),
        true,
        [None, None],
        true,
    );

    assert!(
        matches!(
            laid,
            Operation::Fillet {
                construction: true,
                ..
            }
        ),
        "{laid:?}"
    );
}

/// A part holding one rectangle forty by twenty, with the fillet tool holding
/// its four corners and `typed` in the field.
fn rounding_a_plate(typed: &str) -> (cao_part::PartDocument, crate::screens::sketch::SketchEditor) {
    let mut document = cao_part::PartDocument::new("part", chrono::Utc::now());
    document.apply(Operation::CreateSketch {
        plane: WorkPlane::XY,
        on: None,
    });
    document.apply(Operation::AddRectangle {
        sketch: 0,
        corner: cao_part::history::PointRef::New(DVec2::ZERO),
        opposite: cao_part::history::PointRef::New(DVec2::new(40.0, 20.0)),
        construction: false,
    });
    let mut editor = crate::screens::sketch::SketchEditor {
        tool: Tool::Fillet,
        tool_state: ToolState::Corner {
            taken: (0..4)
                .map(|side| {
                    Corner::Between(SegmentId(side).into(), SegmentId((side + 1) % 4).into())
                })
                .collect(),
            half: None,
        },
        ..Default::default()
    };
    let field = editor.live.field(0);
    field.text = typed.to_string();
    field.take(&Variables::default());
    (document, editor)
}

#[test]
fn a_value_too_big_is_held_back_and_the_field_offers_the_largest() {
    let (mut document, mut editor) = rounding_a_plate("15");
    let mut extrusion = crate::screens::extrusion::ExtrusionState::default();
    let lang = crate::lang::Catalogue::french();
    let mut context = SketchContext {
        document: &mut document,
        editor: &mut editor,
        extrusion: &mut extrusion,
        lang: &lang,
    };

    let laid = cut(&mut context, 0);

    assert!(!laid, "nothing is laid");
    assert_eq!(context.document.sketches()[0].live_corner_cuts().count(), 0);
    assert_eq!(
        context.editor.live.typed(0),
        Some(10.0),
        "two fillets share a side of twenty"
    );
    assert_eq!(context.editor.message, Some(lang.t("sketch.cut_too_big")));

    let laid = cut(&mut context, 0);

    assert!(laid, "a second Enter lays what the field offers");
    let drawn = context.document.sketches()[0].drawn_cuts();
    assert_eq!(drawn.len(), 4);
    assert!(drawn.iter().all(|cut| cut.fits), "{drawn:?}");
}

/// A part holding a trait from (10, 10) to (50, 10) and an arc of radius five
/// round (14, 13), running from (9, 13) to the corner at (10, 10) and hollow
/// towards it, with the fillet tool holding that corner by its point and
/// `typed` in the field. A fillet of three touches the arc at its far end.
fn rounding_beside_an_arc(
    typed: &str,
) -> (cao_part::PartDocument, crate::screens::sketch::SketchEditor) {
    use cao_part::history::PointRef;
    let mut document = cao_part::PartDocument::new("part", chrono::Utc::now());
    document.apply(Operation::CreateSketch {
        plane: WorkPlane::XY,
        on: None,
    });
    document.apply(Operation::AddPoint {
        sketch: 0,
        position: DVec2::new(10.0, 10.0),
        on: Vec::new(),
    });
    document.apply(Operation::AddSegment {
        sketch: 0,
        start: PointRef::Existing(PointId(1)),
        end: PointRef::New(DVec2::new(50.0, 10.0)),
        construction: false,
    });
    document.apply(Operation::AddArc {
        sketch: 0,
        center: PointRef::New(DVec2::new(14.0, 13.0)),
        start: PointRef::New(DVec2::new(9.0, 13.0)),
        end: PointRef::Existing(PointId(1)),
        construction: false,
    });
    let mut editor = crate::screens::sketch::SketchEditor {
        tool: Tool::Fillet,
        tool_state: ToolState::Corner {
            taken: vec![Corner::At(PointId(1))],
            half: None,
        },
        ..Default::default()
    };
    let field = editor.live.field(0);
    field.text = typed.to_string();
    field.take(&Variables::default());
    (document, editor)
}

#[test]
fn a_fillet_too_big_beside_an_arc_is_held_back_and_the_field_offers_the_largest() {
    let (mut document, mut editor) = rounding_beside_an_arc("6");
    let mut extrusion = crate::screens::extrusion::ExtrusionState::default();
    let lang = crate::lang::Catalogue::french();
    let mut context = SketchContext {
        document: &mut document,
        editor: &mut editor,
        extrusion: &mut extrusion,
        lang: &lang,
    };

    let laid = cut(&mut context, 0);

    assert!(
        !laid,
        "a fillet of six cannot stand inside an arc of radius five: nothing is laid"
    );
    assert_eq!(context.document.sketches()[0].live_corner_cuts().count(), 0);
    let offered = context
        .editor
        .live
        .typed(0)
        .expect("the field offers a value");
    assert!(
        (3.0 - 2e-4..=3.0 + 1e-9).contains(&offered),
        "the field offers the largest fillet that fits, three, down to its fourth decimal: \
         {offered}"
    );
    assert_eq!(context.editor.message, Some(lang.t("sketch.cut_too_big")));

    let laid = cut(&mut context, 0);

    assert!(laid, "a second Enter lays what the field offers");
    let drawn = context.document.sketches()[0].drawn_cuts();
    assert_eq!(drawn.len(), 1, "{drawn:?}");
    assert!(drawn[0].fits, "the fillet offered fits: {drawn:?}");
}

/// A drawing whose corner stands between a trait and a curve, with where to
/// click on each away from its ends, and the radius its fillet takes.
struct CurveCorner {
    document: cao_part::PartDocument,
    corner: DVec2,
    on_the_curve: DVec2,
    on_the_trait: DVec2,
    radius: f64,
    what: &'static str,
}

/// A trait from (10, 10) to (50, 10) and an arc round (2, 10) of radius 8 from
/// that corner up to (2, 18); and a trait rising from (46, 34) to (46, 54) with
/// the arc of ellipse round (40, 30), ten wide and five high, drawn from that
/// corner over its top to (30, 30).
fn corners_beside_a_curve() -> [CurveCorner; 2] {
    use cao_part::history::PointRef;
    let drawing = |corner: DVec2, far: DVec2, curve: Operation| {
        let mut document = cao_part::PartDocument::new("part", chrono::Utc::now());
        document.apply(Operation::CreateSketch {
            plane: WorkPlane::XY,
            on: None,
        });
        document.apply(Operation::AddPoint {
            sketch: 0,
            position: corner,
            on: Vec::new(),
        });
        document.apply(Operation::AddSegment {
            sketch: 0,
            start: PointRef::Existing(PointId(1)),
            end: PointRef::New(far),
            construction: false,
        });
        document.apply(curve);
        document
    };
    let back = PointRef::New(DVec2::new(30.0, 30.0));
    let up_and_over = 120_f64.to_radians();
    [
        CurveCorner {
            document: drawing(
                DVec2::new(10.0, 10.0),
                DVec2::new(50.0, 10.0),
                Operation::AddArc {
                    sketch: 0,
                    center: PointRef::New(DVec2::new(2.0, 10.0)),
                    start: PointRef::Existing(PointId(1)),
                    end: PointRef::New(DVec2::new(2.0, 18.0)),
                    construction: false,
                },
            ),
            corner: DVec2::new(10.0, 10.0),
            on_the_curve: DVec2::new(2.0, 10.0) + DVec2::from_angle(60_f64.to_radians()) * 8.0,
            on_the_trait: DVec2::new(30.0, 10.0),
            radius: 5.0,
            what: "a trait and an arc",
        },
        CurveCorner {
            document: drawing(
                DVec2::new(46.0, 34.0),
                DVec2::new(46.0, 54.0),
                Operation::AddEllipse {
                    sketch: 0,
                    center: PointRef::New(DVec2::new(40.0, 30.0)),
                    first: [back.clone(), PointRef::New(DVec2::new(50.0, 30.0))],
                    second: [
                        PointRef::New(DVec2::new(40.0, 25.0)),
                        PointRef::New(DVec2::new(40.0, 35.0)),
                    ],
                    construction: false,
                    drawn: Some([PointRef::Existing(PointId(1)), back]),
                },
            ),
            corner: DVec2::new(46.0, 34.0),
            on_the_curve: DVec2::new(
                40.0 + 10.0 * up_and_over.cos(),
                30.0 + 5.0 * up_and_over.sin(),
            ),
            on_the_trait: DVec2::new(46.0, 44.0),
            radius: 3.0,
            what: "a trait and an arc of ellipse",
        },
    ]
}

/// The corners the tool holds.
fn corners_held(context: &SketchContext<'_>) -> Vec<Corner> {
    match &context.editor.tool_state {
        ToolState::Corner { taken, .. } => taken.clone(),
        _ => Vec::new(),
    }
}

/// Whether the one corner the tool holds stands between the corner's trait
/// and its curve.
fn holds_the_curve_corner(context: &SketchContext<'_>) -> bool {
    let sketch = &context.document.sketches()[0];
    let taken = corners_held(context);
    taken.len() == 1
        && sketch.sides_of(taken[0]).is_some_and(|(one, other)| {
            let traits = [one, other]
                .iter()
                .filter(|side| side.as_trait().is_some())
                .count();
            traits == 1
        })
}

fn type_a_value(context: &mut SketchContext<'_>, value: f64) {
    let field = context.editor.live.field(0);
    field.text = format!("{value}");
    field.take(&Variables::default());
}

/// Types the radius, presses Enter, and holds the drawing to one fillet drawn
/// that fits.
fn enter_rounds_it(context: &mut SketchContext<'_>, radius: f64, what: &str) {
    type_a_value(context, radius);
    assert!(cut(context, 0), "{what}: Enter lays the fillet");
    let drawn = context.document.sketches()[0].drawn_cuts();
    assert_eq!(drawn.len(), 1, "{what}: the corner is rounded: {drawn:?}");
    assert!(drawn[0].fits, "{what}: the fillet fits: {drawn:?}");
}

fn editing_with(tool: Tool) -> crate::screens::sketch::SketchEditor {
    crate::screens::sketch::SketchEditor {
        phase: crate::screens::sketch::SketchPhase::Editing(0),
        tool,
        ..Default::default()
    }
}

#[test]
fn a_click_on_the_point_a_trait_and_an_arc_end_at_names_both_its_sides() {
    for mut case in corners_beside_a_curve() {
        let what = case.what;
        let mut editor = editing_with(Tool::Fillet);
        let mut extrusion = crate::screens::extrusion::ExtrusionState::default();
        let lang = crate::lang::Catalogue::french();
        let mut context = SketchContext {
            document: &mut case.document,
            editor: &mut editor,
            extrusion: &mut extrusion,
            lang: &lang,
        };

        corner(&mut context, 0, case.corner, SNAP);

        assert!(
            holds_the_curve_corner(&context),
            "{what}: one click on the point they end at takes the corner between them, \
             holding {:?}",
            corners_held(&context)
        );
        enter_rounds_it(&mut context, case.radius, what);
    }
}

#[test]
fn a_click_on_an_arc_away_from_its_ends_names_that_side() {
    for mut case in corners_beside_a_curve() {
        let what = case.what;
        let mut editor = editing_with(Tool::Fillet);
        let mut extrusion = crate::screens::extrusion::ExtrusionState::default();
        let lang = crate::lang::Catalogue::french();
        let mut context = SketchContext {
            document: &mut case.document,
            editor: &mut editor,
            extrusion: &mut extrusion,
            lang: &lang,
        };

        corner(&mut context, 0, case.on_the_curve, SNAP);

        assert_eq!(
            context.editor.message,
            Some(lang.t("sketch.click_the_other_side")),
            "{what}: a click on the curve names one side, and asks for the other"
        );
        assert!(corners_held(&context).is_empty(), "{what}: no corner yet");

        corner(&mut context, 0, case.on_the_trait, SNAP);

        assert!(
            holds_the_curve_corner(&context),
            "{what}: the click on the trait completes the corner, holding {:?}",
            corners_held(&context)
        );
        enter_rounds_it(&mut context, case.radius, what);
    }
}

#[test]
fn a_chamfer_on_a_corner_with_a_curve_says_why_it_is_refused() {
    for mut case in corners_beside_a_curve() {
        let what = case.what;
        let mut editor = editing_with(Tool::Chamfer);
        let mut extrusion = crate::screens::extrusion::ExtrusionState::default();
        let lang = crate::lang::Catalogue::french();
        let mut context = SketchContext {
            document: &mut case.document,
            editor: &mut editor,
            extrusion: &mut extrusion,
            lang: &lang,
        };

        for click in [case.corner, case.on_the_curve] {
            context.editor.tool_state = ToolState::None;
            context.editor.message = None;

            corner(&mut context, 0, click, SNAP);

            assert_eq!(
                context.editor.message,
                Some(lang.t("sketch.chamfer_needs_two_traits")),
                "{what}, clicked at {click}: the click itself says the chamfer cuts only a \
                 corner between two traits"
            );
            assert!(
                matches!(
                    &context.editor.tool_state,
                    ToolState::None | ToolState::Corner { half: None, .. }
                ) && corners_held(&context).is_empty(),
                "{what}, clicked at {click}: neither a corner nor a side is taken"
            );
        }
        assert_eq!(
            context.document.sketches()[0].live_corner_cuts().count(),
            0,
            "{what}: nothing is cut"
        );
    }
}
