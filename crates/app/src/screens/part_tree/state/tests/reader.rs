//! The step reader, the card above the history in the Part panel: one box per
//! key step of the part, and the point the part is read to
//! (`docs/redesign/part-screen.md` P7, screens `01-part-sketch-line.png` and
//! `07-part-reading-step-3.png`), worked out from the part with no window.
//!
//! Closes #356 — the whole transcription is here; the bullets the canvas or
//! the whole application answer point to where they are held, each file
//! carrying its own block.
//! - One box per key step. The reader lists one box per sketch and per volume
//!   (extrusion, revolution) of the part, in the part's order. A section is
//!   not a box. With one step it lists one box; with none, none —
//!   `one_box_per_sketch_and_per_volume_in_the_part_s_order_and_none_for_a_section`
//! - decided when the work opened: the boxes are the steps applied, as the
//!   tree's rows are; a step undone is not a box — `an_undone_step_is_no_box`
//! - A click sets the reading point. Clicking a box makes the part computed up
//!   to that step included; the boxes after it are marked as not applied, and
//!   so are the arrows between them. Clicking the last box brings the part
//!   back whole — the boxes and the arrows, an arrow going out with the box
//!   it leads to, the one out of the box read to included, as screen 07 draws
//!   it — `the_boxes_and_arrows_after_the_reading_point_are_not_applied`; the part
//!   computed up to that step included, its design replayed through it with
//!   each step whole as it now stands, and the volumes' states applied to what
//!   is read, held in `crates/part/tests/reading_a_part.rs` by
//!   `a_part_read_through_a_key_step_is_its_design_replayed_through_that_step`;
//!   that part drawn, and whole again from the last box, held in
//!   `crates/app/src/screens/viewport/matter/tests.rs` by
//!   `a_part_read_to_a_step_is_drawn_up_to_it_and_whole_again_from_the_last`;
//!   the click on a box reaching the reader through the panel, in
//!   `crates/app/tests/the_step_reader.rs` by
//!   `a_click_on_a_box_reads_the_part_to_it`
//! - found in review: the part read to a step shows its sketches as well as
//!   its volumes as that step left them — a sketch read drawn while nothing
//!   read was raised from it, a sketch past the reading point not drawn,
//!   whatever its eye — no test: here; held in
//!   `crates/app/src/screens/viewport/render/tests.rs` by
//!   `a_part_read_to_a_step_draws_the_sketches_read_as_then_and_none_after_it`
//! - found in review: which box is the one read to is the presenter's to say,
//!   not the view's — exactly one box is current, the box read to, the last
//!   while the part is read to its end, none on a part with no box —
//!   `exactly_the_box_read_to_is_current`
//! - ◀ ▶ move by one box. ◀ goes to the previous box, ▶ to the next; neither
//!   goes past the first or the last box —
//!   `the_arrows_move_the_reading_point_one_box_and_stop_at_either_end`
//! - Alert colour. A box takes the colour of the worst alert of its own step
//!   (warning or error, from #878), none when it has none. A step after the
//!   reading point keeps its alert marker — the level each box wears,
//!   `a_box_wears_the_worst_alert_of_its_own_step_even_past_the_reading_point`;
//!   the colour painted for a level, and the 7 px dot of a dashed box, are the
//!   view's, picked from the level as a badge of the tree is, which no
//!   assertion here reaches
//! - The line under the row. It names the current step, with its alert level
//!   when it has one; adds « · la suite est masquée » only when the reading
//!   point is not at the end; shows the counter « n / total »; and the legend
//!   is present —
//!   `the_line_under_the_row_names_the_step_its_alert_the_rest_hidden_and_the_count`
//! - The reader undoes nothing. Playing to step 3 then to step 6 leaves the
//!   history, `rewind_to`, undo and redo exactly as they were — no test: here,
//!   the reader is handed the part to read and never to change; held in
//!   `crates/app/tests/the_step_reader.rs` by
//!   `playing_to_step_three_then_six_leaves_the_history_undo_and_redo_as_they_were`
//! - Export takes the whole part, STL and STEP alike, whatever the reading
//!   point — no test: settled by construction. The export is handed the
//!   part's document alone, through `SketchContext`, and the reading point is
//!   the state of the panel and the canvas, set with the part only read: it
//!   never enters the document, as a section switched on does not
//!   (`crates/app/tests/keeping_a_section.rs`). An export also asks the
//!   system's save dialog where to write, which the headless driver cannot
//!   answer. That the document stays whole while read is the bullet above
//! - The reading point is not saved. A part saved while read to step 3 opens
//!   read to the end — no test: here; held in
//!   `crates/app/tests/the_step_reader.rs` by
//!   `a_part_saved_while_read_to_step_three_opens_read_to_the_end`
//! - Adding a step brings the reading point back to the end first (until
//!   #737) — no test: here; held in `crates/app/tests/the_step_reader.rs` by
//!   `adding_a_step_while_read_to_step_three_brings_the_reading_point_back_to_the_end`
//! - Forty steps scroll sideways in the row rather than overflowing the panel
//!   — no test: here; held in `crates/app/tests/the_step_reader.rs` by
//!   `forty_steps_scroll_sideways_in_the_row_rather_than_overflowing_the_panel`;
//!   and, found in review, the row scrolls to the box read to, by
//!   `the_row_scrolls_to_the_box_read_to` in the same file
//! - Visible: the To try it above works in the app (Tom's check) — no test:
//!   it is Tom's check in the app, with the « To try it » of the pull request

use cao_part::history::StepKind;

use super::*;
use crate::screens::part_tree::state::StepReader;

/// The boxes of the reader as the user tells them apart: the icon its kind
/// draws, and the name the line under the row gives it.
fn kinds_and_names(reader: &StepReader) -> Vec<(StepKind, &str)> {
    reader
        .boxes
        .iter()
        .map(|step| (step.kind, step.name.as_str()))
        .collect()
}

/// `a_part` — a sketch on XY raised into a block, a second on XZ — then a
/// section kept along YZ through the block, a sketch laid on its cut, and a
/// ball turned whole from a half disc of a sketch of its own, away from the
/// block.
fn a_part_with_every_kind_of_key_step_and_a_section() -> PartDocument {
    let mut document = a_part();
    document.apply(Operation::SaveSection(SectionAsked {
        plane: WorkPlane::YZ,
        on: None,
        offset: 5.0,
        behind: SideDrawing::SeeThrough,
        in_front: SideDrawing::Opaque,
        tilt_about_u_degrees: 0.0,
        tilt_about_v_degrees: 0.0,
        pivot: DVec3::ZERO,
    }));
    document.apply(Operation::CreateSketchOnSection {
        section: 0,
        plane: WorkPlane {
            origin: DVec3::new(5.0, 0.0, 0.0),
            u: DVec3::Y,
            v: DVec3::Z,
        },
    });
    a_ball(&mut document, DVec3::new(200.0, 200.0, 0.0), 4.0);
    document
}

#[test]
fn one_box_per_sketch_and_per_volume_in_the_part_s_order_and_none_for_a_section() {
    let lang = french();
    let every_kind = StepReader::of(&a_part_with_every_kind_of_key_step_and_a_section(), &lang);
    let one_sketch = {
        let mut document = PartDocument::new("Test", at("2026-01-02T09:00:00Z"));
        document.apply(Operation::CreateSketch {
            plane: WorkPlane::XY,
            on: None,
        });
        StepReader::of(&document, &lang)
    };
    let nothing = StepReader::of(
        &PartDocument::new("Test", at("2026-01-02T09:00:00Z")),
        &lang,
    );

    assert_eq!(
        kinds_and_names(&every_kind),
        [
            (StepKind::Sketch, "Esquisse 1"),
            (StepKind::Extrusion, "Extrusion 1"),
            (StepKind::Sketch, "Esquisse 2"),
            (StepKind::Sketch, "Esquisse 3"),
            (StepKind::Sketch, "Esquisse 4"),
            (StepKind::Revolution, "Révolution 2"),
        ],
        "a sketch, its rectangle and dimension, a block raised, a second sketch, a section \
         kept, a sketch on its cut, then a ball's sketch and its turn: one box for each \
         sketch and each volume, in that order, named as their rows in the tree, and none \
         for the drawing done inside a sketch or for the section",
    );
    assert_eq!(
        kinds_and_names(&one_sketch),
        [(StepKind::Sketch, "Esquisse 1")],
        "a part of a single sketch is read as one box",
    );
    assert!(
        nothing.boxes.is_empty(),
        "a part nothing was done to has a box: {:?}",
        kinds_and_names(&nothing),
    );
}

#[test]
fn an_undone_step_is_no_box() {
    let lang = french();
    let mut document = a_part();
    assert_eq!(
        kinds_and_names(&StepReader::of(&document, &lang)),
        [
            (StepKind::Sketch, "Esquisse 1"),
            (StepKind::Extrusion, "Extrusion 1"),
            (StepKind::Sketch, "Esquisse 2"),
        ],
        "the fixture wants a sketch, its block raised and a second sketch read as three boxes",
    );

    document.undo();
    document.undo();

    assert_eq!(
        kinds_and_names(&StepReader::of(&document, &lang)),
        [(StepKind::Sketch, "Esquisse 1")],
        "the second sketch and the block undone, still there to be redone, are read as boxes",
    );
}

/// Which boxes stand applied, left to right, and which of the arrows between
/// them.
fn applied(reader: &StepReader) -> (Vec<bool>, Vec<bool>) {
    let boxes = reader.boxes.iter().map(|step| step.applied).collect();
    (boxes, reader.arrows.clone())
}

#[test]
fn the_boxes_and_arrows_after_the_reading_point_are_not_applied() {
    let lang = french();
    let document = a_part_with_every_kind_of_key_step_and_a_section();
    let mut viewport = ViewportState::default();
    let read = |viewport: &ViewportState| applied(&StepReader::seen(&document, viewport, &lang));

    assert_eq!(
        read(&viewport),
        (vec![true; 6], vec![true; 5]),
        "a part nobody has read yet is not read to its end, as (boxes, arrows) applied",
    );

    viewport.read_to(2, 6);
    assert_eq!(
        read(&viewport),
        (
            vec![true, true, true, false, false, false],
            vec![true, true, false, false, false],
        ),
        "read to the third of six boxes, the three boxes after it and the three arrows \
         leading to them are not the only ones read not applied, as (boxes, arrows) applied",
    );

    viewport.read_to(0, 6);
    assert_eq!(
        read(&viewport),
        (
            vec![true, false, false, false, false, false],
            vec![false; 5],
        ),
        "read to the first box, every box after it and every arrow do not read not applied, \
         as (boxes, arrows) applied",
    );

    viewport.read_to(5, 6);
    assert_eq!(
        read(&viewport),
        (vec![true; 6], vec![true; 5]),
        "the last box clicked does not bring every box and every arrow back applied",
    );
}

/// How far the part is read, as the counter « n / total » says it: the boxes
/// played.
fn played(document: &PartDocument, viewport: &ViewportState) -> usize {
    StepReader::seen(document, viewport, &french())
        .boxes
        .iter()
        .filter(|step| step.applied)
        .count()
}

#[test]
fn the_arrows_move_the_reading_point_one_box_and_stop_at_either_end() {
    let document = a_part_with_every_kind_of_key_step_and_a_section();
    let boxes = StepReader::of(&document, &french()).boxes.len();
    assert_eq!(boxes, 6, "the fixture wants six boxes");
    let mut viewport = ViewportState::default();

    let mut counted = vec![played(&document, &viewport)];
    viewport.read_next(boxes);
    counted.push(played(&document, &viewport));
    for _ in 0..boxes {
        viewport.read_previous(boxes);
        counted.push(played(&document, &viewport));
    }
    for _ in 0..boxes {
        viewport.read_next(boxes);
        counted.push(played(&document, &viewport));
    }
    assert_eq!(
        counted,
        [6, 6, 5, 4, 3, 2, 1, 1, 2, 3, 4, 5, 6, 6],
        "unread, then ▶ once, ◀ six times and ▶ six times: the counter does not go back one \
         box at a time to the first and on to the last, or goes past either",
    );

    viewport.read_to(2, boxes);
    viewport.read_next(boxes);
    let after_the_third = played(&document, &viewport);
    viewport.read_to(5, boxes);
    viewport.read_previous(boxes);
    let before_the_last = played(&document, &viewport);
    assert_eq!(
        (after_the_third, before_the_last),
        (4, 5),
        "▶ after a click on the third box, and ◀ after a click on the last, do not move one \
         box from the box clicked",
    );

    let mut on_a_part_with_no_box = ViewportState::default();
    on_a_part_with_no_box.read_previous(0);
    on_a_part_with_no_box.read_next(0);
    assert_eq!(
        played(&document, &on_a_part_with_no_box),
        6,
        "◀ and ▶ on a part with no box set a reading point, which there is no box to set",
    );
}

/// `a_shaft_cut_by_a_slice_a_hundredth_of_a_degree_thin` — a shaft raised from
/// a circle, a slice turned out of it so thin it is built unstably — then a
/// sketch laid on a face the part does not have, and a rectangle drawn away
/// from the shaft, raised 4, one of its sides then erased. Seven boxes: the
/// shaft's sketch, the shaft, the slice's sketch, the slice, unstable, the
/// sketch off its face, the rectangle's sketch, and the block whose area is
/// gone.
fn a_part_with_alerts_of_both_levels() -> PartDocument {
    let mut document = a_shaft_cut_by_a_slice_a_hundredth_of_a_degree_thin();
    document.apply(Operation::CreateSketch {
        plane: WorkPlane::XY,
        on: Some(FaceAnchor {
            face: 404,
            up: DVec3::Y,
        }),
    });
    document.apply(Operation::CreateSketch {
        plane: WorkPlane::XY,
        on: None,
    });
    document.apply(Operation::AddRectangle {
        sketch: 3,
        corner: PointRef::New(DVec2::new(100.0, 100.0)),
        opposite: PointRef::New(DVec2::new(110.0, 120.0)),
        construction: false,
    });
    document.apply(Operation::Extrude {
        sketch: 3,
        areas: document.areas_at(3, &[DVec2::new(105.0, 110.0)]),
        distance: 4.0.into(),
        mode: ExtrusionMode::Add,
    });
    document.apply(Operation::EraseMany {
        sketch: 3,
        elements: vec![Element::Segment(SegmentId(0))],
        dimensions: Vec::new(),
        constraints: Vec::new(),
    });
    document
}

/// Each box as the user tells its alert: its name, and the level of the worst
/// alert of its own step, the colour it is drawn in.
fn alerts(reader: &StepReader) -> Vec<(&str, Option<Level>)> {
    reader
        .boxes
        .iter()
        .map(|step| (step.name.as_str(), step.alert))
        .collect()
}

#[test]
fn a_box_wears_the_worst_alert_of_its_own_step_even_past_the_reading_point() {
    let lang = french();
    let document = a_part_with_alerts_of_both_levels();
    let mut viewport = ViewportState::default();
    let expected = [
        ("Esquisse 1", None),
        ("Extrusion 1", None),
        ("Esquisse 2", None),
        ("Révolution 2", Some(Level::Warning)),
        ("Esquisse 3", Some(Level::Warning)),
        ("Esquisse 4", None),
        ("Extrusion 3", Some(Level::Error)),
    ];

    assert_eq!(
        alerts(&StepReader::seen(&document, &viewport, &lang)),
        expected,
        "read to its end, a box does not wear the level of its own step's alert — the slice \
         built unstably and the sketch off its face warn, the block whose area is gone is in \
         error — or wears one its step has not, or one of the step before it",
    );

    viewport.read_to(0, 7);
    let read_to_the_first = StepReader::seen(&document, &viewport, &lang);
    assert!(
        read_to_the_first
            .boxes
            .iter()
            .skip(1)
            .all(|step| !step.applied),
        "the fixture wants every box after the first read as not applied",
    );
    assert_eq!(
        alerts(&read_to_the_first),
        expected,
        "read to the first box, a box past the reading point loses or changes its alert",
    );
}

/// The line under the row as the user reads it: what it says, the level whose
/// colour it is written in, and the counter on its right.
type Read = (String, Option<Level>, String);

fn the_line(document: &PartDocument, viewport: &ViewportState) -> Option<Read> {
    StepReader::seen(document, viewport, &french())
        .line
        .map(|line| (line.text, line.alert, line.counter))
}

fn read(text: &str, alert: Option<Level>, counter: &str) -> Option<Read> {
    Some((text.to_string(), alert, counter.to_string()))
}

#[test]
fn the_line_under_the_row_names_the_step_its_alert_the_rest_hidden_and_the_count() {
    let lang = french();
    let document = a_part_with_alerts_of_both_levels();
    let mut viewport = ViewportState::default();
    let mut lines = vec![the_line(&document, &viewport)];
    for key_step in [4, 3, 0, 5, 6] {
        viewport.read_to(key_step, 7);
        lines.push(the_line(&document, &viewport));
    }

    assert_eq!(
        lines,
        [
            read("Extrusion 3 — aire perdue", Some(Level::Error), "7 / 7"),
            read(
                "Esquisse 3 — face perdue · la suite est masquée",
                Some(Level::Warning),
                "5 / 7",
            ),
            read(
                "Révolution 2 — instable · la suite est masquée",
                Some(Level::Warning),
                "4 / 7",
            ),
            read("Esquisse 1 · la suite est masquée", None, "1 / 7"),
            read("Esquisse 4 · la suite est masquée", None, "6 / 7"),
            read("Extrusion 3 — aire perdue", Some(Level::Error), "7 / 7"),
        ],
        "unread, then read to the sketch off its face, the slice, the first sketch, the last \
         sketch and the last box: the line does not name the step read to with its problem, \
         in its alert's colour, say the rest is hidden only short of the end, or count the \
         boxes played out of all of them, as (text, colour, counter)",
    );
    assert_eq!(
        the_line(&a_part(), &ViewportState::default()),
        read("Esquisse 2", None, "3 / 3"),
        "a part read to its end, whose last step has no alert, is not named bare",
    );
    assert_eq!(
        the_line(
            &PartDocument::new("Test", at("2026-01-02T09:00:00Z")),
            &ViewportState::default(),
        ),
        None,
        "a part with no box has a line under a row that is not there",
    );
    assert_eq!(
        StepReader::seen(&document, &viewport, &lang).legend,
        [
            (Level::Warning, "avertissement".to_string()),
            (Level::Error, "erreur".to_string()),
        ],
        "the legend does not read each level's word beside its colour",
    );
}

#[test]
fn exactly_the_box_read_to_is_current() {
    let lang = french();
    let document = a_part_with_every_kind_of_key_step_and_a_section();
    let current = |reader: StepReader| -> Vec<usize> {
        reader
            .boxes
            .iter()
            .enumerate()
            .filter(|(_, step)| step.current)
            .map(|(rank, _)| rank)
            .collect()
    };
    let mut viewport = ViewportState::default();

    let mut seen = vec![current(StepReader::seen(&document, &viewport, &lang))];
    viewport.read_to(2, 6);
    seen.push(current(StepReader::seen(&document, &viewport, &lang)));
    viewport.read_previous(6);
    seen.push(current(StepReader::seen(&document, &viewport, &lang)));
    viewport.read_next(6);
    seen.push(current(StepReader::seen(&document, &viewport, &lang)));
    viewport.read_to(0, 6);
    seen.push(current(StepReader::seen(&document, &viewport, &lang)));
    viewport.read_to(5, 6);
    seen.push(current(StepReader::seen(&document, &viewport, &lang)));
    seen.push(current(StepReader::of(&document, &lang)));

    assert_eq!(
        seen,
        [
            vec![5],
            vec![2],
            vec![1],
            vec![2],
            vec![0],
            vec![5],
            vec![5]
        ],
        "unread, read to the third box, ◀, ▶, read to the first, to the last, then read to \
         its end: the box read to is not the one current, or not the only one",
    );
    assert!(
        current(StepReader::of(
            &PartDocument::new("Test", at("2026-01-02T09:00:00Z")),
            &lang,
        ))
        .is_empty(),
        "a part with no box has one current",
    );
}
