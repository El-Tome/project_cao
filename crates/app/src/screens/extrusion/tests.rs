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
//!   nothing — since #736 it says so as an alert of the part, the pill handed
//!   nothing, and still never that it made nothing —
//!   `an_extrusion_not_built_or_unstable_is_an_alert_and_no_message`
//! - an exact cut that misses the matter still says it removed nothing, though
//!   the kernel handed back a body built anew —
//!   `a_cut_that_misses_the_exact_matter_says_it_removed_nothing`
//!
//! Closes #552.
//! - a step the kernel built at a lean it reads unstably says so once applied,
//!   with the lean, rather than nothing — since #736 as an alert of the part
//!   whose sentence carries the lean —
//!   `an_extrusion_not_built_or_unstable_is_an_alert_and_no_message`
//! - a step it declined says it was declined, as before — no test: a decline
//!   hands back no body, so no note, and
//!   `an_extrusion_not_built_or_unstable_is_an_alert_and_no_message` holds the
//!   sentence, an alert's since #736
//!
//! Since #533 an area across its axis is turned on both sides, so a revolution
//! that changes nothing no longer blames the axis —
//! `a_revolution_across_its_axis_makes_matter_and_has_nothing_to_say`,
//! `a_revolution_that_changes_nothing_says_so_without_blaming_the_axis`.
//!
//! Closes #736 — the whole transcription is in `crates/app/tests/the_alerts.rs`;
//! these bullets are the extrusion panel's half of one decision there.
//! - decided by Tom when the work opened: « Modifier » on an extrusion's alert
//!   reopens the extrusion, its panel filled with its values — an extrusion
//!   reopens on its sketch, its mode, its height as written, its direction and
//!   the places its areas were picked at —
//!   `an_extrusion_reopens_on_the_values_it_was_raised_with`; a revolution on
//!   its mode, its axis, its angle as written and its places —
//!   `a_revolution_reopens_on_its_axis_and_its_angle`
//! - and « Appliquer » on it changes that step rather than raising another:
//!   the design keeps its steps, the step reads its new values, and the part is
//!   raised from them — `applying_a_reopened_extrusion_changes_its_step_rather_than_raising_another`
//! - What describes the part leaves the message pill: an extrusion not built
//!   or unstable is an alert — applied, an extrusion the kernel declines and a
//!   cut it makes at an unstable lean hand the pill nothing, and the part lists
//!   each as an alert saying what the pill used to —
//!   `an_extrusion_not_built_or_unstable_is_an_alert_and_no_message`. It
//!   replaces the two tests that held the pill's sentences under #526 and
//!   #552, whose assertions are what this changes
//! - found in review: a panel reopened on a step the history has undone since
//!   applies nothing, and leaves the redo that brings the step back —
//!   `applying_a_panel_reopened_on_a_step_undone_since_changes_nothing`
//! - found in review: a step changed from its panel is judged by what it does
//!   to the part without it, so that a cut deepened through a plate it already
//!   went through is not said to remove nothing —
//!   `a_through_cut_deepened_from_its_panel_is_not_said_to_remove_nothing`;
//!   and applied as it was, it records no change and says nothing —
//!   `a_reopened_extrusion_applied_unchanged_records_nothing_and_says_nothing`
//!
//! A step is reopened by its place in the history, as the Part panel and the
//! alerts name a volume, and reopens on what the part now replays it as,
//! `PartDocument::raised_as`: once changed, on its change. A height or an angle
//! the other way along the plane reopens as written the right way with the
//! direction reversed, which is how the panel wrote it.

use std::path::Path;

use cao_part::{Operation, PartDocument, PointRef, RevolutionAxis};
use cao_sketch::{PointId, SketchAxis, WorkPlane};
use chrono::Utc;
use glam::DVec3;

use super::*;
use crate::screens::sketch::SketchEditor;
use crate::screens::title_bar::TitleBar;
use crate::screens::viewport::ViewportState;

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

/// A ball of `radius` about `center`: a half disc drawn on the plane holding
/// the upright through it, turned whole about that line.
fn a_ball(document: &mut PartDocument, center: DVec3, radius: f64) {
    let sketch = document.sketches().len();
    document.apply(Operation::CreateSketch {
        plane: WorkPlane {
            origin: center,
            u: DVec3::X,
            v: DVec3::Z,
        },
        on: None,
    });
    let first = document.sketches()[sketch].points().len();
    document.apply(Operation::AddArc {
        sketch,
        center: PointRef::New(DVec2::ZERO),
        start: PointRef::New(DVec2::new(0.0, -radius)),
        end: PointRef::New(DVec2::new(0.0, radius)),
        construction: false,
    });
    document.apply(Operation::AddSegment {
        sketch,
        start: PointRef::Existing(PointId(first + 2)),
        end: PointRef::Existing(PointId(first + 1)),
        construction: false,
    });
    document.apply(Operation::Revolve {
        sketch,
        areas: document.areas_at(sketch, &[DVec2::new(radius / 4.0, 0.0)]),
        axis: RevolutionAxis::Sketch(SketchAxis::V),
        angle: 360.0.into(),
        mode: ExtrusionMode::Add,
    });
}

/// A ball of radius 8 about the origin, and over it a circle on a plane
/// tilted 30° about X: raised down, its wall crosses the ball's sphere off
/// its centre, which the exact kernel does not build.
fn a_ball_under_a_slanted_circle() -> PartDocument {
    let mut document = PartDocument::new("part", Utc::now());
    a_ball(&mut document, DVec3::ZERO, 8.0);
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

/// A shaft Ø20 raised 30 on XY, and on XZ a rectangle from (−20, 0) to
/// (20, 40) whose foot lies on the sketch's horizontal axis.
fn a_shaft_and_a_rectangle_standing_on_its_axis() -> PartDocument {
    let mut document = PartDocument::new("part", Utc::now());
    document.apply(Operation::CreateSketch {
        plane: WorkPlane::XY,
        on: None,
    });
    document.apply(Operation::AddCircle {
        sketch: 0,
        center: PointRef::New(DVec2::ZERO),
        radius: 10.0,
        rim: Vec::new(),
        construction: false,
    });
    document.apply(Operation::Extrude {
        sketch: 0,
        areas: document.areas_at(0, &[DVec2::ZERO]),
        distance: 30.0.into(),
        mode: ExtrusionMode::Add,
    });
    document.apply(Operation::CreateSketch {
        plane: WorkPlane::XZ,
        on: None,
    });
    document.apply(Operation::AddRectangle {
        sketch: 1,
        corner: PointRef::New(DVec2::new(-20.0, 0.0)),
        opposite: PointRef::New(DVec2::new(20.0, 40.0)),
        construction: false,
    });
    document
}

#[test]
fn an_extrusion_not_built_or_unstable_is_an_alert_and_no_message() {
    let lang = Catalogue::french();
    let mut declined = a_ball_under_a_slanted_circle();
    let mut extrusion = ExtrusionState::default();
    extrusion.offer(1);
    extrusion.picks = vec![DVec2::ZERO];
    extrusion.reversed = true;
    extrusion.arm(ExtrusionMode::Add);
    let mut said_of_the_decline = None;
    assert!(apply_extrusion(
        &mut declined,
        &mut extrusion,
        &mut said_of_the_decline,
        &lang
    ));

    let mut unstable = a_shaft_and_a_rectangle_standing_on_its_axis();
    let before = unstable.body().volume();
    let mut extrusion = ExtrusionState::default();
    extrusion.offer(1);
    extrusion.shape = Shape::Revolution;
    extrusion.axis = RevolutionAxis::Sketch(SketchAxis::U);
    extrusion.angle_input = "0.01".to_string();
    extrusion.picks = vec![DVec2::new(0.0, 20.0)];
    extrusion.arm(ExtrusionMode::Cut);
    let mut said_of_the_lean = None;
    assert!(apply_extrusion(
        &mut unstable,
        &mut extrusion,
        &mut said_of_the_lean,
        &lang
    ));

    assert_eq!(
        [
            (said_of_the_decline, alerts_of(&declined, &lang)),
            (said_of_the_lean, alerts_of(&unstable, &lang)),
        ],
        [
            (
                None,
                vec![(
                    "Extrusion 2 · non construite".to_owned(),
                    lang.t("extrusion.declined"),
                )],
            ),
            (
                None,
                vec![(
                    "Révolution 2 · instable".to_owned(),
                    lang.t_with("extrusion.unstable_along", &[("angle", "0.01")]),
                )],
            ),
        ],
        "an extrusion the kernel declines and a slice it cuts at an unstable lean are not said \
         as alerts alone, as (what the pill is handed, the part's alerts as title and sentence)",
    );
    assert!(
        unstable.body().volume() < before,
        "the slice is taken out all the same"
    );
}

/// The part's alerts, each as its title and its sentence, as the title bar
/// lists them with nothing open on the part.
fn alerts_of(document: &PartDocument, lang: &Catalogue) -> Vec<(String, String)> {
    let editor = SketchEditor::default();
    let viewport = ViewportState::default();
    let bar = TitleBar::of(
        document,
        Path::new("part.caopart"),
        &editor,
        &viewport,
        None,
        lang,
    );
    bar.alerts
        .iter()
        .map(|alert| (alert.title.clone(), alert.sentence.clone()))
        .collect()
}

/// Where `a_square_raised_then_cut_the_other_way` picks its area.
const IN_THE_SQUARE: DVec2 = DVec2::new(1.0, 2.0);

/// The square raised 5, then cut 3 the other way along the plane, both from
/// the square's one area.
fn a_square_raised_then_cut_the_other_way() -> PartDocument {
    let mut document = a_part_with_a_square();
    for (distance, mode) in [(5.0, ExtrusionMode::Add), (-3.0, ExtrusionMode::Cut)] {
        document.apply(Operation::Extrude {
            sketch: 0,
            areas: document.areas_at(0, &[IN_THE_SQUARE]),
            distance: distance.into(),
            mode,
        });
    }
    document
}

/// Where the square is raised, then cut, in its history.
const THE_RAISE: usize = 2;
const THE_CUT: usize = 3;

/// What the panel reopened on a step holds: its sketch, its mode, its shape,
/// its height as written, whether it goes the other way, and its picks.
type Held = (
    Option<usize>,
    Option<ExtrusionMode>,
    Shape,
    String,
    bool,
    Vec<DVec2>,
);

fn held(extrusion: &ExtrusionState) -> Held {
    (
        extrusion.sketch,
        extrusion.mode,
        extrusion.shape,
        extrusion.distance_input.clone(),
        extrusion.reversed,
        extrusion.picks.clone(),
    )
}

#[test]
fn an_extrusion_reopens_on_the_values_it_was_raised_with() {
    let document = a_square_raised_then_cut_the_other_way();

    let reopened = [THE_RAISE, THE_CUT]
        .map(|step| ExtrusionState::reopened(&document, step).as_ref().map(held));

    assert_eq!(
        reopened,
        [
            Some((
                Some(0),
                Some(ExtrusionMode::Add),
                Shape::Straight,
                "5".to_owned(),
                false,
                vec![IN_THE_SQUARE],
            )),
            Some((
                Some(0),
                Some(ExtrusionMode::Cut),
                Shape::Straight,
                "3".to_owned(),
                true,
                vec![IN_THE_SQUARE],
            )),
        ],
        "the square raised 5 and cut 3 the other way do not reopen on the values they were \
         raised with",
    );
}

#[test]
fn a_revolution_reopens_on_its_axis_and_its_angle() {
    let mut document = PartDocument::new("part", Utc::now());
    document.apply(Operation::CreateSketch {
        plane: WorkPlane::XZ,
        on: None,
    });
    document.apply(Operation::AddRectangle {
        sketch: 0,
        corner: PointRef::New(DVec2::new(0.0, 5.0)),
        opposite: PointRef::New(DVec2::new(10.0, 10.0)),
        construction: false,
    });
    let beside_the_axis = DVec2::new(5.0, 7.5);
    document.apply(Operation::Revolve {
        sketch: 0,
        areas: document.areas_at(0, &[beside_the_axis]),
        axis: RevolutionAxis::Sketch(SketchAxis::U),
        angle: 90.0.into(),
        mode: ExtrusionMode::Add,
    });

    let reopened = ExtrusionState::reopened(&document, 2).map(|extrusion| {
        (
            extrusion.sketch,
            extrusion.mode,
            extrusion.shape,
            extrusion.axis,
            extrusion.angle_input.clone(),
            extrusion.reversed,
            extrusion.picks.clone(),
        )
    });

    assert_eq!(
        reopened,
        Some((
            Some(0),
            Some(ExtrusionMode::Add),
            Shape::Revolution,
            RevolutionAxis::Sketch(SketchAxis::U),
            "90".to_owned(),
            false,
            vec![beside_the_axis],
        )),
        "a rectangle turned 90° about the sketch's first axis does not reopen on its revolution",
    );
}

#[test]
fn applying_a_reopened_extrusion_changes_its_step_rather_than_raising_another() {
    let mut document = a_square_raised_then_cut_the_other_way();
    let steps = document.history.steps().len();
    let raised = document
        .history
        .number_at(THE_RAISE)
        .expect("the square is raised");
    let mut extrusion =
        ExtrusionState::reopened(&document, THE_RAISE).expect("the square's raise reopens");
    extrusion.distance_input = "12".to_owned();

    let applied = apply_extrusion(
        &mut document,
        &mut extrusion,
        &mut None,
        &Catalogue::french(),
    );

    let raised_so = Operation::Extrude {
        sketch: 0,
        areas: document.areas_at(0, &[IN_THE_SQUARE]),
        distance: 12.0.into(),
        mode: ExtrusionMode::Add,
    };
    let mut wrong = Vec::new();
    if !applied {
        wrong.push("nothing was applied".to_owned());
    }
    if document.history.steps().len() != steps {
        wrong.push(format!(
            "the design has {} steps where it had {steps}",
            document.history.steps().len(),
        ));
    }
    if document.raised_as(raised) != Some(&raised_so) {
        wrong.push(format!(
            "the square's raise reads {:?}, not raised 12",
            document.raised_as(raised),
        ));
    }
    assert!(
        wrong.is_empty(),
        "the square's raise, reopened and applied at 12, did not change its own step:\n  {}",
        wrong.join("\n  "),
    );
}

#[test]
fn applying_a_panel_reopened_on_a_step_undone_since_changes_nothing() {
    let mut document = a_square_raised_then_cut_the_other_way();
    let mut extrusion =
        ExtrusionState::reopened(&document, THE_CUT).expect("the square's cut reopens");
    extrusion.distance_input = "4".to_owned();
    document.undo();
    let held = document.history.clone();

    let applied = apply_extrusion(
        &mut document,
        &mut extrusion,
        &mut None,
        &Catalogue::french(),
    );

    let untouched = document.history == held;
    let redone = document.redo();
    assert!(
        !applied && untouched && redone,
        "the square's cut, reopened then undone, was applied all the same: applied {applied}, \
         the history moved {}, the cut redone {redone}",
        !untouched,
    );
}

/// The square raised 10 with a round hole of radius 3 cut through it, 20
/// deep from the square's plane: a plate with a through hole.
fn a_plate_with_a_through_hole() -> PartDocument {
    let mut document = a_part_with_a_square();
    document.apply(Operation::AddCircle {
        sketch: 0,
        center: PointRef::New(DVec2::ZERO),
        radius: 3.0,
        rim: Vec::new(),
        construction: false,
    });
    document.apply(Operation::Extrude {
        sketch: 0,
        areas: document.areas_at(0, &[DVec2::new(7.0, 7.0), DVec2::ZERO]),
        distance: 10.0.into(),
        mode: ExtrusionMode::Add,
    });
    document.apply(Operation::Extrude {
        sketch: 0,
        areas: document.areas_at(0, &[DVec2::ZERO]),
        distance: 20.0.into(),
        mode: ExtrusionMode::Cut,
    });
    let hole = std::f64::consts::PI * 3.0 * 3.0 * 10.0;
    assert!(
        (20.0 * 20.0 * 10.0 - hole - document.body().volume()).abs() < hole / 10.0,
        "the fixture wants a plate 20 by 20 and 10 thick with a hole of radius 3 through it, and \
         holds {}",
        document.body().volume(),
    );
    document
}

/// Where the hole is cut in the history of `a_plate_with_a_through_hole`.
const THE_HOLE: usize = 4;

#[test]
fn a_through_cut_deepened_from_its_panel_is_not_said_to_remove_nothing() {
    let lang = Catalogue::french();
    let mut document = a_plate_with_a_through_hole();
    let mut extrusion = ExtrusionState::reopened(&document, THE_HOLE).expect("the hole reopens");
    extrusion.distance_input = "30".to_owned();
    let mut said = None;

    assert!(apply_extrusion(
        &mut document,
        &mut extrusion,
        &mut said,
        &lang
    ));

    assert_eq!(
        said, None,
        "the hole through the plate, deepened from 20 to 30, is said to remove nothing, though \
         the plate without it holds more",
    );
}

#[test]
fn a_reopened_extrusion_applied_unchanged_records_nothing_and_says_nothing() {
    let lang = Catalogue::french();
    let mut document = a_square_raised_then_cut_the_other_way();
    let mut extrusion =
        ExtrusionState::reopened(&document, THE_RAISE).expect("the square's raise reopens");
    let held = document.history.clone();
    let mut said = None;

    apply_extrusion(&mut document, &mut extrusion, &mut said, &lang);

    assert!(
        document.history == held && said.is_none(),
        "the square's raise, reopened and applied as it was, moved the history ({}) or said {said:?}",
        document.history != held,
    );
}
