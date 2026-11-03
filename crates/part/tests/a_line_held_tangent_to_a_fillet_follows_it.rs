//! « Tangence » links a line and a fillet's arc as it links a line and any arc:
//! the line is held on the whole circle the fillet is a piece of, its distance
//! to the fillet's centre being the radius. The centre is worked out again from
//! the corner, its sides and the value each time the drawing settles, so the
//! line follows them, and pulled, moves them.
//!
//! Closes #651.
//! - « Tangence » with a line and a fillet's arc, in either order, makes the
//!   line tangent to the fillet —
//!   `a_line_and_a_fillet_clicked_in_either_order_are_made_tangent`, the one
//!   clicked first staying where it is —
//!   `a_tangent_with_a_fillet_leaves_the_first_clicked_where_it_was`; the tool
//!   pointing at the fillet's arc is `cao_app`'s, held by
//!   `the_tangent_tool_points_at_a_fillet_s_arc_rather_than_the_side_under_it`
//! - The line stays tangent when the corner, its sides or the radius change,
//!   even once the touch is outside the drawn arc —
//!   `a_line_tangent_to_a_fillet_follows_a_new_radius`,
//!   `a_line_tangent_to_a_fillet_follows_its_corner_moved`,
//!   `a_line_tangent_to_a_fillet_follows_its_side_turned`,
//!   `a_line_tangent_to_a_fillet_stays_on_its_circle_once_the_touch_is_past_the_arc`
//! - Pulling the tangent line moves the corner or the radius as the rules
//!   allow — `pulling_a_line_tangent_to_a_fillet_moves_its_corner`
//! - « Tangence » between a line and a chamfer's cut lays nothing —
//!   `a_tangent_between_a_line_and_a_chamfer_lays_nothing`
//! - A red cut keeps the line tangent. A cut taken off, or a fillet become a
//!   chamfer, lets it go where it stands —
//!   `a_red_fillet_keeps_its_line_tangent`,
//!   `taking_a_fillet_off_lets_its_tangent_line_go_where_it_stands`,
//!   `a_fillet_become_a_chamfer_lets_its_tangent_line_go_where_it_stands`
//! - A cut nothing hangs on costs the solver nothing more than in #617 — no
//!   test: what the solver is asked is read only inside the sketch crate; held
//!   there, in `corner_cut/hanging/tests.rs`, by
//!   `a_cut_nothing_hangs_on_asks_the_solver_for_nothing` (#620) and by
//!   `a_fillet_nothing_hangs_on_asks_nothing_beside_one_a_line_is_tangent_to`

use cao_part::history::{CutAsked, CutChange, Operation, PointRef};
use cao_part::{History, PartState};
use cao_sketch::{
    ArcDraft, Constraint, Corner, CornerCutId, CutCurve, Element, PointId, Rule, RuleIntent,
    RulePick, SegmentId, Sketch, WorkPlane, rule_intent, sweep_of,
};
use glam::DVec2;

/// How near two places must stand to be the same place after a solve.
const SAME: f64 = 1e-6;

const CUT: CornerCutId = CornerCutId(0);
/// The corner, and the far ends of its two sides.
const CORNER: PointId = PointId(1);
const RIGHT_END: PointId = PointId(2);
const UP_END: PointId = PointId(3);
const RADIUS: f64 = 5.0;
/// The line drawn after the corner was rounded.
const LINE: SegmentId = SegmentId(2);

/// A corner standing on its own at (10, 10): one trait right to (60, 10),
/// one up to (10, 50), rounded to 5 about (15, 15). Its first value fixes the
/// scale: a unit is a millimetre. Beyond the corner a free line runs from
/// (0, 18) to (18, 0), square to the fillet's middle and some 8.5 from its
/// centre: touching nothing, crossing nothing.
fn a_rounded_corner_beside_a_line() -> History {
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
        corners: vec![Corner::Between(SegmentId(0), SegmentId(1))],
        radius: RADIUS.into(),
        construction: false,
    });
    history.push(Operation::AddSegment {
        sketch: 0,
        start: PointRef::New(DVec2::new(0.0, 18.0)),
        end: PointRef::New(DVec2::new(18.0, 0.0)),
        construction: false,
    });
    history
}

/// What the constraint tool hands the rule, the line clicked first or the
/// fillet's arc clicked first.
fn clicked(line_first: bool) -> (&'static str, [RulePick; 2]) {
    let line = RulePick::Element(Element::Segment(LINE));
    let arc = RulePick::CutCurve(CUT);
    match line_first {
        true => ("the line clicked first", [line, arc]),
        false => ("the fillet clicked first", [arc, line]),
    }
}

/// The rule « Tangence » makes of the line and the fillet, clicked in the
/// order `picks` says.
fn the_tangent(sketch: &Sketch, picks: [RulePick; 2]) -> Constraint {
    let Some(RuleIntent::Constrain(rule)) = rule_intent(Rule::Tangent, &picks, sketch) else {
        panic!("the tangent rule between a line and a fillet's arc lays nothing");
    };
    rule
}

/// The drawing once « Tangence » is laid between the line and the fillet,
/// clicked in the order `picks` says, as the constraint tool lays it.
fn tangent_laid(history: &History, picks: [RulePick; 2]) -> Sketch {
    let mut state = PartState::rebuild(history);
    let rule = the_tangent(&state.sketches[0], picks);
    let outcome = state.apply(&Operation::Constrain {
        sketch: 0,
        constraint: rule,
    });
    assert!(outcome.is_none(), "the tangency was refused: {outcome:?}");
    state.sketches[0].clone()
}

/// The line of `a_rounded_corner_beside_a_line` held tangent to the fillet,
/// the fillet clicked first: the line came to it, square to its middle, the
/// corner where it was.
fn a_line_held_tangent() -> History {
    let mut history = a_rounded_corner_beside_a_line();
    let (_, picks) = clicked(false);
    let rule = the_tangent(&the_drawing(&history), picks);
    history.push(Operation::Constrain {
        sketch: 0,
        constraint: rule,
    });
    history
}

fn move_point(history: &mut History, point: PointId, to: DVec2) {
    history.push(Operation::MovePoint {
        sketch: 0,
        point,
        position: to,
        merged_into: None,
        on: Vec::new(),
        let_go: false,
    });
}

fn the_drawing(history: &History) -> Sketch {
    PartState::rebuild(history).sketches[0].clone()
}

/// The fillet's arc as it is drawn: its centre and its radius.
fn the_fillet(sketch: &Sketch) -> (DVec2, f64) {
    match sketch.drawn_cut(CUT).expect("the fillet is drawn").curve {
        CutCurve::Arc(arc) => (arc.centre, arc.centre.distance(arc.start)),
        CutCurve::Straight { .. } => panic!("the fillet is drawn straight"),
    }
}

/// How far the line, taken whole, runs from the fillet's centre.
fn reach_from_the_centre(sketch: &Sketch) -> f64 {
    let (centre, _) = the_fillet(sketch);
    let (start, end) = sketch.endpoints(LINE);
    (end - start).normalize().perp_dot(centre - start).abs()
}

fn assert_tangent(sketch: &Sketch, case: &str) {
    let reach = reach_from_the_centre(sketch);
    let (_, radius) = the_fillet(sketch);
    assert!(
        (reach - radius).abs() < SAME,
        "{case}: the line runs {reach} from the fillet's centre, its radius is {radius}"
    );
}

/// Where the line, taken whole, touches the fillet's circle: the foot of the
/// centre on it.
fn the_touch(sketch: &Sketch) -> DVec2 {
    let (centre, _) = the_fillet(sketch);
    let (start, end) = sketch.endpoints(LINE);
    let along = (end - start).normalize();
    start + along * along.dot(centre - start)
}

/// Whether a place lies within the sweep of the fillet's drawn arc, seen from
/// its centre.
fn within_the_drawn_arc(sketch: &Sketch, place: DVec2) -> bool {
    let CutCurve::Arc(arc) = sketch.drawn_cut(CUT).expect("the fillet is drawn").curve else {
        panic!("the fillet is drawn straight");
    };
    sweep_of(ArcDraft { end: place, ..arc }) <= sweep_of(arc)
}

fn the_line(sketch: &Sketch) -> [DVec2; 2] {
    let (start, end) = sketch.endpoints(LINE);
    [start, end]
}

fn the_corner_and_its_sides(sketch: &Sketch) -> [DVec2; 3] {
    [CORNER, RIGHT_END, UP_END].map(|point| sketch.point(point))
}

/// How far a set of places travelled, taken as the worst of them.
fn moved<const N: usize>(before: [DVec2; N], after: [DVec2; N]) -> f64 {
    before
        .iter()
        .zip(after)
        .map(|(was, is)| was.distance(is))
        .fold(0.0, f64::max)
}

#[test]
fn a_line_and_a_fillet_clicked_in_either_order_are_made_tangent() {
    let history = a_rounded_corner_beside_a_line();
    assert!(
        reach_from_the_centre(&the_drawing(&history)) - RADIUS > 1.0,
        "the line already touches the fillet before the rule is laid"
    );

    for line_first in [true, false] {
        let (order, picks) = clicked(line_first);
        let sketch = tangent_laid(&history, picks);

        assert_tangent(&sketch, order);
    }
}

#[test]
fn a_tangent_with_a_fillet_leaves_the_first_clicked_where_it_was() {
    let history = a_rounded_corner_beside_a_line();
    let before = the_drawing(&history);

    for line_first in [true, false] {
        let (order, picks) = clicked(line_first);
        let sketch = tangent_laid(&history, picks);

        let line_moved = moved(the_line(&before), the_line(&sketch));
        let fillet_moved = moved(
            the_corner_and_its_sides(&before),
            the_corner_and_its_sides(&sketch),
        );
        let (stayed, came) = match line_first {
            true => (line_moved, fillet_moved),
            false => (fillet_moved, line_moved),
        };
        assert!(stayed < SAME, "{order}: it moved by {stayed}");
        assert!(came > SAME, "{order}: the other stayed where it was");
        let (_, radius) = the_fillet(&sketch);
        assert!(
            (radius - RADIUS).abs() < SAME,
            "{order}: the fillet's value became {radius}"
        );
    }
}

#[test]
fn a_line_tangent_to_a_fillet_follows_a_new_radius() {
    let mut history = a_line_held_tangent();

    history.push(Operation::CutChange(CutChange::Reshape {
        sketch: 0,
        cut: CUT,
        value: CutAsked::Fillet(8.0.into()),
    }));

    let sketch = the_drawing(&history);
    let (_, radius) = the_fillet(&sketch);
    assert!(
        (radius - 8.0).abs() < SAME,
        "the fillet was to be rounded to 8, it is {radius}"
    );
    assert_tangent(&sketch, "rounded to 8");
}

#[test]
fn a_line_tangent_to_a_fillet_follows_its_corner_moved() {
    let mut history = a_line_held_tangent();
    let to = DVec2::new(0.0, 4.0);

    move_point(&mut history, CORNER, to);

    let sketch = the_drawing(&history);
    assert!(
        sketch.point(CORNER).distance(to) < SAME,
        "the corner was to be dragged to {to}, it is at {}",
        sketch.point(CORNER),
    );
    assert_tangent(&sketch, "the corner dragged");
}

#[test]
fn a_line_tangent_to_a_fillet_follows_its_side_turned() {
    let mut history = a_line_held_tangent();
    let to = DVec2::new(40.0, 50.0);

    move_point(&mut history, UP_END, to);

    let sketch = the_drawing(&history);
    assert!(
        sketch.point(UP_END).distance(to) < SAME,
        "the side's end was to be dragged to {to}, it is at {}",
        sketch.point(UP_END),
    );
    assert_tangent(&sketch, "a side turned");
}

#[test]
fn a_line_tangent_to_a_fillet_stays_on_its_circle_once_the_touch_is_past_the_arc() {
    let mut history = a_line_held_tangent();
    let held = the_drawing(&history);
    assert!(
        within_the_drawn_arc(&held, the_touch(&held)),
        "the line was to touch the drawn arc before the side turned"
    );
    let opened_wide = DVec2::new(-30.0, 20.0);

    move_point(&mut history, UP_END, opened_wide);

    let sketch = the_drawing(&history);
    assert!(
        sketch.point(UP_END).distance(opened_wide) < SAME,
        "the side's end was to be dragged to {opened_wide}, it is at {}",
        sketch.point(UP_END),
    );
    assert!(
        !within_the_drawn_arc(&sketch, the_touch(&sketch)),
        "the corner opened wide was to leave the touch past the drawn arc, at {}",
        the_touch(&sketch),
    );
    assert_tangent(&sketch, "the touch past the drawn arc");
}
