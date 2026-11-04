//! A fillet's or a chamfer's points — the centre of a fillet, and where a cut
//! meets each of its sides — are points something can hang on. They are not
//! points of the drawing until something does: then a point is laid there and
//! a rule holds it to the place, worked out again from the corner, its sides
//! and the value. What hangs on them follows them, and pulled, moves them.
//!
//! Closes #620.
//! - a line starting from a cut's point, a circle concentric with a fillet, a
//!   dimension from a cut's point and a point placed on an arc stay attached
//!   when the corner, its sides or the value change —
//!   `a_line_started_from_a_fillet_s_end_follows_a_new_radius`,
//!   `a_line_started_from_a_fillet_s_end_follows_its_side_turned`,
//!   `a_circle_drawn_on_a_fillet_s_centre_stays_on_it_when_the_corner_moves`,
//!   `a_dimension_from_a_cut_s_point_holds_when_the_value_changes`,
//!   `a_point_placed_on_a_fillet_follows_its_arc`
//! - pulling what hangs on a cut moves the corner as the rules allow —
//!   `a_length_typed_on_a_line_hung_on_a_cut_moves_the_corner`
//! - « Cote point à point » between a rounded side's free end and its tangent
//!   point measures the visible part —
//!   `a_dimension_from_the_free_end_to_the_tangent_point_measures_the_visible_part`;
//!   the tool laying the point it measures from, in the dimension's own step,
//!   is `cao_app`'s, held by
//!   `a_dimension_from_a_cut_s_point_is_one_step_with_the_point_it_hangs`
//! - the questions are settled with the owner and their answers hold — no
//!   test: the settling is the issue's comment of 2026-11-02; each answer is
//!   held by a bullet of its own below
//!   - a red cut keeps what hangs on it where it is drawn, nothing jumps,
//!     and a dimension of the visible part keeps its value —
//!     `a_cut_that_stops_fitting_keeps_what_hangs_on_it_on_the_red_cut`,
//!     `a_dimensioned_visible_part_keeps_its_value_however_big_the_radius`
//!   - a cut taken off leaves what hung on it where it was, an end held on
//!     its side — `taking_a_cut_off_leaves_what_hung_on_it_where_it_was`
//!   - a kind changed keeps the ends and lets the centre go —
//!     `a_fillet_become_a_chamfer_lets_its_centre_go_and_keeps_its_ends`
//!   - dragging a cut's point drags its corner — no test: it is the press's,
//!     held in the sketch crate by
//!     `a_press_on_a_point_of_a_fillet_drags_its_corner_from_afar` and
//!     `a_press_on_a_point_hung_on_a_fillet_drags_its_corner_from_afar_too`
//!
//! What #620 also asked, and the owner's notes on it:
//! - a « Coïncidence » hangs a point on a cut's point —
//!   `a_coincidence_hangs_a_point_on_a_cut_s_point`
//! - the centre of a fillet and the points where a cut meets its sides are
//!   shown as points — no test: the canvas draws them (`render/cuts.rs`);
//!   what it draws is held in the sketch crate by
//!   `a_chamfer_offers_no_centre_and_a_fillet_offers_three_points`
//! - a cut nothing hangs on costs the solver nothing more than in #617 — no
//!   test: held in the sketch crate by
//!   `a_cut_nothing_hangs_on_asks_the_solver_for_nothing`
//! - a shape held only by its right angles keeps its way up when what hangs
//!   on its cut makes it give —
//!   `a_rectangle_whose_visible_part_is_dimensioned_does_not_turn_when_its_radius_changes`
//! - a change to a cut that what hangs on it cannot follow is refused, as a
//!   value that cannot be held is, and nothing moves —
//!   `a_cut_change_what_hangs_cannot_follow_is_refused`
//! - what hangs on a cut is kept through a compaction of the history, and
//!   through a part written and read back, a corner merged into another
//!   point included —
//!   `a_compacted_part_keeps_what_hangs_on_its_cuts`,
//!   `a_corner_merged_into_another_point_keeps_what_hangs_on_its_cut`,
//!   `a_part_read_back_keeps_what_hangs_on_its_cuts`
//! - a line tangent to the arc — no test: not done here, the constraint tool
//!   has no way yet to point at a cut's curve; it is its own issue

use cao_part::history::{ChamferAsked, CutAsked, CutChange, Operation, PointRef};
use cao_part::{History, Outcome, PartDocument, PartState, compact};
use cao_sketch::{
    Chamfer, Constraint, Corner, CornerCutId, CutPoint, CutSide, DimensionTarget, Element, PointId,
    Rule, RulePick, SegmentId, Sketch, Support, WorkPlane, rule_intent,
};
use glam::DVec2;

/// How near two places must stand to be the same place after a solve.
const SAME: f64 = 1e-6;

const CUT: CornerCutId = CornerCutId(0);
/// The corner, and the far ends of its two sides.
const CORNER: PointId = PointId(1);
const RIGHT_END: PointId = PointId(2);
/// The first point laid after the corner was drawn.
const HUNG: PointId = PointId(4);

/// A corner standing on its own at (10, 10): one trait right to (60, 10),
/// one up to (10, 50), rounded to 5. Its first value fixes the scale: a unit
/// is a millimetre. The fillet touches its sides at (15, 10) and (10, 15),
/// about (15, 15).
fn a_rounded_corner() -> History {
    let mut history = History::default();
    history.push(Operation::CreateSketch {
        plane: WorkPlane::XY,
        on: None,
    });
    history.push(Operation::AddSegment {
        sketch: 0,
        start: PointRef::New(DVec2::new(10.0, 10.0)),
        end: PointRef::New(DVec2::new(60.0, 10.0)),
        construction: false,
    });
    history.push(Operation::AddSegment {
        sketch: 0,
        start: PointRef::Existing(CORNER),
        end: PointRef::New(DVec2::new(10.0, 50.0)),
        construction: false,
    });
    history.push(Operation::Fillet {
        sketch: 0,
        corners: vec![Corner::Between(SegmentId(0).into(), SegmentId(1).into())],
        radius: 5.0.into(),
        construction: false,
    });
    history
}

/// Where a point laid on a cut's point stands, held there.
fn on_the_cut(history: &History, at: CutPoint) -> PointRef {
    let place = the_drawing(history).cut_point(CUT, at).unwrap();
    PointRef::Held {
        at: place,
        on: vec![Support::CutPoint(CUT, at)],
    }
}

/// A point laid on a cut's point, held there.
fn hang_a_point(history: &mut History, at: CutPoint) {
    let PointRef::Held { at: place, on } = on_the_cut(history, at) else {
        unreachable!()
    };
    history.push(Operation::AddPoint {
        sketch: 0,
        position: place,
        on,
    });
}

/// A line from a cut's point down to `to`.
fn a_line_from(history: &mut History, at: CutPoint, to: DVec2) {
    let start = on_the_cut(history, at);
    history.push(Operation::AddSegment {
        sketch: 0,
        start,
        end: PointRef::New(to),
        construction: false,
    });
}

fn rounded_to(history: &mut History, radius: f64) {
    history.push(Operation::CutChange(CutChange::Reshape {
        sketch: 0,
        cut: CUT,
        value: CutAsked::Fillet(radius.into()),
    }));
}

fn the_drawing(history: &History) -> Sketch {
    PartState::rebuild(history).sketches[0].clone()
}

fn assert_on(sketch: &Sketch, point: PointId, at: CutPoint) {
    let stands = sketch.point(point);
    let place = sketch.cut_point(CUT, at).unwrap();
    assert!(
        stands.distance(place) < SAME,
        "the point stands at {stands}, the cut's {at:?} at {place}",
    );
}

#[test]
fn a_line_started_from_a_fillet_s_end_follows_a_new_radius() {
    let mut history = a_rounded_corner();
    a_line_from(&mut history, CutPoint::First, DVec2::new(15.0, -30.0));

    rounded_to(&mut history, 8.0);

    let sketch = the_drawing(&history);
    assert_eq!(sketch.segments()[2].start, HUNG);
    assert_on(&sketch, HUNG, CutPoint::First);
    assert!(sketch.point(HUNG).distance(DVec2::new(18.0, 10.0)) < SAME);
}

#[test]
fn a_circle_drawn_on_a_fillet_s_centre_stays_on_it_when_the_corner_moves() {
    let mut history = a_rounded_corner();
    let center = on_the_cut(&history, CutPoint::Centre);
    history.push(Operation::AddCircle {
        sketch: 0,
        center,
        radius: 2.0,
        rim: Vec::new(),
        construction: false,
    });

    history.push(Operation::MovePoint {
        sketch: 0,
        point: CORNER,
        position: DVec2::new(0.0, 4.0),
        merged_into: None,
        on: Vec::new(),
        let_go: false,
    });

    let sketch = the_drawing(&history);
    assert_eq!(sketch.circles()[0].center, HUNG);
    assert_on(&sketch, HUNG, CutPoint::Centre);
    assert!(sketch.point(CORNER).distance(DVec2::new(0.0, 4.0)) < SAME);
}

#[test]
fn a_dimension_from_a_cut_s_point_holds_when_the_value_changes() {
    let mut history = a_rounded_corner();
    let touch = on_the_cut(&history, CutPoint::First);
    history.push(Operation::AddPoint {
        sketch: 0,
        position: DVec2::new(15.0, 10.0),
        on: match touch {
            PointRef::Held { on, .. } => on,
            _ => unreachable!(),
        },
    });
    let visible = DimensionTarget::Distance {
        from: RIGHT_END,
        to: HUNG,
    };
    history.push(Operation::SetDimension {
        sketch: 0,
        target: visible,
        value: 45.0.into(),
        placement: None,
    });

    rounded_to(&mut history, 8.0);

    let sketch = the_drawing(&history);
    assert_on(&sketch, HUNG, CutPoint::First);
    let span = sketch.point(RIGHT_END).distance(sketch.point(HUNG));
    assert!(
        (span - 45.0).abs() < SAME,
        "the dimension holds 45, the span is {span}"
    );
}

#[test]
fn a_point_placed_on_a_fillet_follows_its_arc() {
    let mut history = a_rounded_corner();
    let on_the_arc = DVec2::new(15.0, 15.0) - DVec2::splat(5.0 / 2f64.sqrt());
    history.push(Operation::AddPoint {
        sketch: 0,
        position: on_the_arc,
        on: vec![Support::Cut(CUT)],
    });

    rounded_to(&mut history, 9.0);

    let sketch = the_drawing(&history);
    let centre = sketch.cut_point(CUT, CutPoint::Centre).unwrap();
    let reach = sketch.point(HUNG).distance(centre);
    assert!(
        (reach - 9.0).abs() < SAME,
        "the point stands {reach} from the centre"
    );
}

#[test]
fn a_length_typed_on_a_line_hung_on_a_cut_moves_the_corner() {
    let mut history = a_rounded_corner();
    a_line_from(&mut history, CutPoint::First, DVec2::new(15.0, -30.0));
    let below = PointId(5);
    history.push(Operation::Constrain {
        sketch: 0,
        constraint: Constraint::Fixed {
            element: Element::Point(below),
        },
    });

    history.push(Operation::SetDimension {
        sketch: 0,
        target: DimensionTarget::Length(SegmentId(2)),
        value: 60.0.into(),
        placement: None,
    });

    let sketch = the_drawing(&history);
    assert!((sketch.segment_length(SegmentId(2)) - 60.0).abs() < SAME);
    assert_on(&sketch, HUNG, CutPoint::First);
    assert_eq!(
        sketch.point(below),
        DVec2::new(15.0, -30.0),
        "held where it was"
    );
    assert!(
        sketch.point(CORNER).distance(DVec2::new(10.0, 10.0)) > 1.0,
        "the corner gave, at {}",
        sketch.point(CORNER),
    );
}

#[test]
fn a_dimension_from_the_free_end_to_the_tangent_point_measures_the_visible_part() {
    let mut history = a_rounded_corner();
    let touch = on_the_cut(&history, CutPoint::First);
    let PointRef::Held { at, on } = touch else {
        unreachable!()
    };
    history.push(Operation::AddPoint {
        sketch: 0,
        position: at,
        on,
    });
    let visible = DimensionTarget::Distance {
        from: RIGHT_END,
        to: HUNG,
    };
    let drawn = the_drawing(&history);
    let (from, to) = drawn.visible_part(SegmentId(0)).unwrap();
    let measured = PartState::rebuild(&history).measured(0, visible).unwrap();
    assert!(
        (measured - from.distance(to)).abs() < SAME,
        "it reads {measured}"
    );

    history.push(Operation::SetDimension {
        sketch: 0,
        target: visible,
        value: 30.0.into(),
        placement: None,
    });

    let sketch = the_drawing(&history);
    let (from, to) = sketch.visible_part(SegmentId(0)).unwrap();
    assert!(
        (from.distance(to) - 30.0).abs() < SAME,
        "the visible part runs {}",
        from.distance(to),
    );
}

#[test]
fn a_cut_that_stops_fitting_keeps_what_hangs_on_it_on_the_red_cut() {
    let mut history = a_rounded_corner();
    a_line_from(&mut history, CutPoint::First, DVec2::new(15.0, -30.0));

    rounded_to(&mut history, 70.0);

    let sketch = the_drawing(&history);
    assert!(!sketch.drawn_cut(CUT).unwrap().fits, "the cut is red");
    assert_on(&sketch, HUNG, CutPoint::First);
    assert!(sketch.point(HUNG).distance(DVec2::new(80.0, 10.0)) < SAME);

    rounded_to(&mut history, 5.0);
    let sketch = the_drawing(&history);
    assert!(sketch.drawn_cut(CUT).unwrap().fits);
    assert!(sketch.point(HUNG).distance(DVec2::new(15.0, 10.0)) < SAME);
    assert!(sketch.point(CORNER).distance(DVec2::new(10.0, 10.0)) < SAME);
}

#[test]
fn a_dimensioned_visible_part_keeps_its_value_however_big_the_radius() {
    let mut history = a_rounded_corner();
    hang_a_point(&mut history, CutPoint::First);
    let visible = DimensionTarget::Distance {
        from: RIGHT_END,
        to: HUNG,
    };
    history.push(Operation::SetDimension {
        sketch: 0,
        target: visible,
        value: 45.0.into(),
        placement: None,
    });

    rounded_to(&mut history, 70.0);

    let sketch = the_drawing(&history);
    let (from, to) = sketch.visible_part(SegmentId(0)).unwrap();
    assert!(
        (from.distance(to) - 45.0).abs() < SAME,
        "{}",
        from.distance(to)
    );
    assert!(
        sketch.drawn_cut(CUT).unwrap().fits,
        "the side grew to give it room"
    );
}

#[test]
fn a_line_started_from_a_fillet_s_end_follows_its_side_turned() {
    let mut history = a_rounded_corner();
    a_line_from(&mut history, CutPoint::Second, DVec2::new(-30.0, 15.0));

    history.push(Operation::MovePoint {
        sketch: 0,
        point: PointId(3),
        position: DVec2::new(40.0, 50.0),
        merged_into: None,
        on: Vec::new(),
        let_go: false,
    });

    let sketch = the_drawing(&history);
    assert_on(&sketch, HUNG, CutPoint::Second);
    assert!(
        sketch.point(HUNG).distance(DVec2::new(10.0, 15.0)) > 1.0,
        "it moved"
    );
}

#[test]
fn a_cut_change_what_hangs_cannot_follow_is_refused() {
    let mut document = PartDocument::new("Plaque", "2026-11-02T09:00:00Z".parse().expect("a date"));
    for operation in a_rounded_corner().operations() {
        document.apply(operation.clone());
    }
    let touch = on_the_cut(&document.history, CutPoint::First);
    let PointRef::Held { at, on } = touch else {
        unreachable!()
    };
    document.apply(Operation::AddPoint {
        sketch: 0,
        position: at,
        on,
    });
    for point in [CORNER, RIGHT_END, PointId(3)] {
        document.apply(Operation::Constrain {
            sketch: 0,
            constraint: Constraint::Fixed {
                element: Element::Point(point),
            },
        });
    }
    document.apply(Operation::SetDimension {
        sketch: 0,
        target: DimensionTarget::Distance {
            from: RIGHT_END,
            to: HUNG,
        },
        value: 45.0.into(),
        placement: None,
    });
    let before = document.sketches()[0].clone();

    let outcome = document.apply(Operation::CutChange(CutChange::Reshape {
        sketch: 0,
        cut: CUT,
        value: CutAsked::Fillet(8.0.into()),
    }));

    assert!(
        matches!(
            outcome,
            Some(Outcome::Dimension(cao_part::DimensionOutcome::Geometry(
                cao_sketch::LengthOutcome::BestEffort
            )))
        ),
        "{outcome:?}",
    );
    let sketch = &document.sketches()[0];
    assert_eq!(sketch.points(), before.points(), "nothing moved");
    assert_eq!(
        sketch.corner_cuts()[CUT.0].shape,
        before.corner_cuts()[CUT.0].shape
    );
}

#[test]
fn a_corner_merged_into_another_point_keeps_what_hangs_on_its_cut() {
    let mut history = a_rounded_corner();
    a_line_from(&mut history, CutPoint::First, DVec2::new(15.0, -30.0));
    history.push(Operation::AddPoint {
        sketch: 0,
        position: DVec2::new(4.0, 4.0),
        on: Vec::new(),
    });
    history.push(Operation::MergePoints {
        sketch: 0,
        kept: PointId(6),
        dropped: CORNER,
    });
    let merged = the_drawing(&history);
    assert_eq!(merged.cuts_standing_at(PointId(6)), vec![CUT]);

    let compacted = compact(&history);

    let sketch = the_drawing(&compacted);
    assert_eq!(what_hangs(&sketch).len(), 1);
    assert_eq!(what_hangs(&sketch), what_hangs(&merged));
}

#[test]
fn taking_a_cut_off_leaves_what_hung_on_it_where_it_was() {
    let mut history = a_rounded_corner();
    a_line_from(&mut history, CutPoint::First, DVec2::new(15.0, -30.0));
    let center = on_the_cut(&history, CutPoint::Centre);
    history.push(Operation::AddCircle {
        sketch: 0,
        center,
        radius: 2.0,
        rim: Vec::new(),
        construction: false,
    });
    let centre = PointId(6);

    history.push(Operation::CutChange(CutChange::TakeOff {
        sketch: 0,
        cuts: vec![CUT],
    }));

    let sketch = the_drawing(&history);
    assert_eq!(sketch.live_corner_cuts().count(), 0);
    assert!(sketch.point(HUNG).distance(DVec2::new(15.0, 10.0)) < SAME);
    assert!(sketch.point(centre).distance(DVec2::new(15.0, 15.0)) < SAME);
    assert_eq!(sketch.holds_on(HUNG), vec![Support::Segment(SegmentId(0))]);
    assert_eq!(sketch.holds_on(centre), Vec::new());
    assert_eq!(sketch.live_circles().count(), 1, "the circle stays");
}

#[test]
fn a_fillet_become_a_chamfer_lets_its_centre_go_and_keeps_its_ends() {
    let mut history = a_rounded_corner();
    a_line_from(&mut history, CutPoint::First, DVec2::new(15.0, -30.0));
    let center = on_the_cut(&history, CutPoint::Centre);
    history.push(Operation::AddCircle {
        sketch: 0,
        center,
        radius: 2.0,
        rim: Vec::new(),
        construction: false,
    });
    let centre = PointId(6);

    history.push(Operation::CutChange(CutChange::Reshape {
        sketch: 0,
        cut: CUT,
        value: CutAsked::Chamfer(ChamferAsked::from(Chamfer::Equal(7.0))),
    }));

    let sketch = the_drawing(&history);
    assert_on(&sketch, HUNG, CutPoint::First);
    assert!(sketch.point(HUNG).distance(DVec2::new(17.0, 10.0)) < SAME);
    assert_eq!(sketch.holds_on(centre), Vec::new());
    assert!(sketch.point(centre).distance(DVec2::new(15.0, 15.0)) < SAME);
}

#[test]
fn a_coincidence_hangs_a_point_on_a_cut_s_point() {
    let mut history = a_rounded_corner();
    history.push(Operation::AddSegment {
        sketch: 0,
        start: PointRef::New(DVec2::new(30.0, 30.0)),
        end: PointRef::New(DVec2::new(40.0, 40.0)),
        construction: false,
    });
    let loose = PointId(4);
    let picks = [
        RulePick::Element(Element::Point(loose)),
        RulePick::CutPoint(CUT, CutPoint::Centre),
    ];
    let Some(cao_sketch::RuleIntent::Constrain(rule)) =
        rule_intent(Rule::Coincident, &picks, &the_drawing(&history))
    else {
        panic!("a point and a cut's point make a coincidence");
    };
    history.push(Operation::Constrain {
        sketch: 0,
        constraint: rule,
    });

    rounded_to(&mut history, 8.0);

    let sketch = the_drawing(&history);
    assert_on(&sketch, loose, CutPoint::Centre);
    assert!(sketch.point(loose).distance(DVec2::new(18.0, 18.0)) < SAME);
}

/// A rounded corner carrying a line from its first end, a circle on its
/// centre and a point on its arc.
fn a_corner_carrying_all_three() -> History {
    let mut history = a_rounded_corner();
    a_line_from(&mut history, CutPoint::First, DVec2::new(15.0, -30.0));
    let center = on_the_cut(&history, CutPoint::Centre);
    history.push(Operation::AddCircle {
        sketch: 0,
        center,
        radius: 2.0,
        rim: Vec::new(),
        construction: false,
    });
    history.push(Operation::AddPoint {
        sketch: 0,
        position: DVec2::new(15.0, 15.0) - DVec2::splat(5.0 / 2f64.sqrt()),
        on: vec![Support::Cut(CUT)],
    });
    history
}

/// Every rule hanging something on a cut, said by where its point stands.
fn what_hangs(sketch: &Sketch) -> Vec<(DVec2, Support)> {
    let mut hung: Vec<(DVec2, Support)> = sketch
        .constraints()
        .iter()
        .filter_map(|rule| Support::held_by(*rule))
        .filter(|(_, on)| on.names_a_cut())
        .map(|(point, on)| (sketch.point(point), on))
        .collect();
    hung.sort_by(|one, other| {
        one.0
            .x
            .total_cmp(&other.0.x)
            .then(one.0.y.total_cmp(&other.0.y))
    });
    hung
}

#[test]
fn a_compacted_part_keeps_what_hangs_on_its_cuts() {
    let history = a_corner_carrying_all_three();
    let before = what_hangs(&the_drawing(&history));
    assert_eq!(before.len(), 3);

    let mut compacted = compact(&history);
    let after = what_hangs(&the_drawing(&compacted));
    assert_eq!(after.len(), 3, "{after:?}");
    for ((was, _), (is, _)) in before.iter().zip(&after) {
        assert!(was.distance(*is) < SAME, "{was} came out at {is}");
    }

    rounded_to(&mut compacted, 8.0);
    let sketch = the_drawing(&compacted);
    for (place, on) in what_hangs(&sketch) {
        if let Support::CutPoint(cut, at) = on {
            assert!(place.distance(sketch.cut_point(cut, at).unwrap()) < SAME);
        }
    }
}

#[test]
fn a_part_read_back_keeps_what_hangs_on_its_cuts() {
    let history = a_corner_carrying_all_three();

    let written = serde_json::to_string(history.operations()).expect("it writes");
    let read: Vec<Operation> = serde_json::from_str(&written).expect("it reads");
    let mut back = History::default();
    for operation in read {
        back.push(operation);
    }

    assert_eq!(
        what_hangs(&the_drawing(&back)),
        what_hangs(&the_drawing(&history))
    );
}

#[test]
fn a_rectangle_whose_visible_part_is_dimensioned_does_not_turn_when_its_radius_changes() {
    let mut history = History::default();
    history.push(Operation::CreateSketch {
        plane: WorkPlane::XY,
        on: None,
    });
    history.push(Operation::AddRectangle {
        sketch: 0,
        corner: PointRef::New(DVec2::ZERO),
        opposite: PointRef::New(DVec2::new(100.0, 60.0)),
        construction: false,
    });
    for side in 0..4 {
        history.push(Operation::Constrain {
            sketch: 0,
            constraint: Constraint::Perpendicular {
                first: SegmentId(side),
                second: SegmentId((side + 1) % 4),
            },
        });
    }
    history.push(Operation::Fillet {
        sketch: 0,
        corners: vec![Corner::Between(SegmentId(1).into(), SegmentId(2).into())],
        radius: 8.0.into(),
        construction: false,
    });
    let drawing = the_drawing(&history);
    let cut = drawing.live_corner_cuts().next().unwrap().0;
    let place = drawing.cut_point(cut, CutPoint::First).unwrap();
    history.push(Operation::AddPoint {
        sketch: 0,
        position: place,
        on: vec![Support::CutPoint(cut, CutPoint::First)],
    });
    let hung = PointId(drawing.points().len());
    let CutSide::Trait(side) = drawing.corner_cuts()[cut.0].sides[0] else {
        panic!("the corner is cut between two traits");
    };
    let far = drawing.segments()[side.0];
    let far = match far.start == drawing.corner_cuts()[cut.0].corner {
        true => far.end,
        false => far.start,
    };
    history.push(Operation::SetDimension {
        sketch: 0,
        target: DimensionTarget::Distance {
            from: far,
            to: hung,
        },
        value: 52.0.into(),
        placement: None,
    });
    let before = the_drawing(&history);
    assert!((before.point(far).distance(before.point(hung)) - 52.0).abs() < SAME);

    history.push(Operation::CutChange(CutChange::Reshape {
        sketch: 0,
        cut,
        value: CutAsked::Fillet(20.0.into()),
    }));

    let sketch = the_drawing(&history);
    assert_on_cut(&sketch, hung, cut, CutPoint::First);
    assert!((sketch.point(far).distance(sketch.point(hung)) - 52.0).abs() < SAME);
    for side in 0..4 {
        let (start, end) = sketch.endpoints(SegmentId(side));
        let (was_start, was_end) = before.endpoints(SegmentId(side));
        let turned = (end - start).angle_to(was_end - was_start);
        assert!(turned.abs() < 1e-9, "side {side} turned by {turned} rad");
    }
}

fn assert_on_cut(sketch: &Sketch, point: PointId, cut: CornerCutId, at: CutPoint) {
    let place = sketch.cut_point(cut, at).unwrap();
    assert!(sketch.point(point).distance(place) < SAME);
}
