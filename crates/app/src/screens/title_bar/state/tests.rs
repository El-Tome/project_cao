//! What app · screens/title_bar/state.rs is held to: which of an error and a
//! message the pill keeps once a frame is over, and what the part's problems
//! come to as alerts.
//!
//! Closes #726.
//! - a fresh error, raised in the frame or since the last one was settled (the
//!   Preferences, the copy panel's place), takes the pill, and the message
//!   under it is left where it was —
//!   `a_fresh_error_takes_the_pill_and_leaves_the_message_under_it`
//! - a standing error gives way to a command —
//!   `a_standing_error_gives_way_to_a_command`
//! - or to a message other than the one the frame began with —
//!   `a_standing_error_gives_way_to_a_message_said_since`
//! - a message said again unchanged every frame, as choosing a plane says
//!   « Choisissez un plan d'esquisse », does not make it give way —
//!   `a_message_said_again_unchanged_leaves_the_error_standing`
//! - when it gives way and the message is still the one it covered, that
//!   message goes too: the message before the error never comes back —
//!   `the_message_an_error_covered_does_not_come_back_once_it_gives_way`
//! - a fresh error wins over a message said in the same frame —
//!   `a_fresh_error_wins_over_a_message_said_in_the_same_frame`
//! - a save the disk keeps refusing is a fresh error at every frame, and keeps
//!   the pill whatever is said or asked — decided: a save that keeps failing
//!   stays seen — `a_save_the_disk_keeps_refusing_keeps_the_pill`
//! - a standing error alone stays, a later save that succeeds taking nothing
//!   back — `a_standing_error_alone_stays`
//!
//! The rule replaces one that cleared the message as a frame began, so that
//! any message found at its end read as said in it: choosing a plane says its
//! sentence again at every frame, and an error from the Preferences went at
//! the first move of the mouse. A message is now said in a frame when the one
//! at its end is not the one at its start; one cleared by a gesture, at its end
//! `None`, is not a message said, and the rule leaves that case to the
//! implementation.
//!
//! Closes #736 — the whole transcription is in `crates/app/tests/the_alerts.rs`;
//! these bullets are the ones the title bar's presenter holds.
//! - Two levels. Warning, yellow: an unstable case (`unstable_at`); a sketch
//!   whose face is lost and that stays where it was (`is_adrift`). Error, red:
//!   a volume not built (`is_declined`); a lost area —
//!   `each_problem_of_the_part_is_an_alert_at_its_level`
//! - a kept section whose face is lost, which wears « face perdue » in the tree
//!   today, is a warning like a sketch off its face —
//!   `each_problem_of_the_part_is_an_alert_at_its_level`
//! - « ⚠ N alertes » on the right of the title bar, only when N > 0 — what the
//!   count reads, « 1 alerte » for one, and that a sound part has none —
//!   `the_title_bar_counts_the_alerts_only_while_there_is_one`; where it stands
//!   is the view's, held in `crates/app/tests/the_alerts.rs`
//! - in the colour of the worst level — the level the count is drawn at,
//!   whichever of the two comes first in the part —
//!   `the_count_wears_the_level_of_the_worst_alert`
//! - Every problem detected today is still shown: as an alert now —
//!   `each_problem_of_the_part_is_an_alert_at_its_level`, one part for each
//!   problem the tree wears a badge for today
//!
//! An element counts once, however many problems it has: P2 counts the
//! elements that wear a badge, lost, refused, unstable or drifting, and a
//! volume wears its gravest. The parts are those the tree's own tests are made
//! of, in `crates/app/src/screens/part_tree/state/tests.rs`, written here so
//! that two can be laid one after the other in a part.

use cao_part::history::{
    ExtrusionMode, FaceAnchor, Operation, PointRef, RevolutionAxis, SectionAsked, SideDrawing,
};
use cao_sketch::{DimensionTarget, Element, PointId, SegmentId, SketchAxis, WorkPlane};
use chrono::{DateTime, Utc};
use glam::{DVec2, DVec3};

use super::*;
use crate::ui::badge::Level;

const SAVE_FAILED: &str = "L'échange avec /CAO/Platine.caopart s'est arrêté en chemin.";
const CHOOSE_A_PLANE: &str = "Choisissez un plan d'esquisse";
const ARC_HINT: &str = "Cliquez le centre, puis le départ de la courbe, puis son arrivée";

/// What a frame hands the rule: the error the pill held as it began, the one
/// raised since, the message at its start, and whether a command ran in it.
struct Frame<'a> {
    standing: Option<Covering>,
    fresh: Option<&'a str>,
    at_start: Option<&'a str>,
    commanded: bool,
}

/// The error the pill keeps once the frame is over, and the message left in
/// the slot, from the message the frame ended with.
fn settled(frame: Frame<'_>, at_end: Option<&str>) -> (Option<Covering>, Option<String>) {
    let mut message = at_end.map(str::to_owned);
    let kept = settle(
        frame.standing,
        frame.fresh.map(str::to_owned),
        frame.at_start,
        &mut message,
        frame.commanded,
    );
    (kept, message)
}

fn covering(error: &str, covers: Option<&str>) -> Covering {
    Covering {
        error: error.to_owned(),
        covers: covers.map(str::to_owned),
    }
}

#[test]
fn a_fresh_error_takes_the_pill_and_leaves_the_message_under_it() {
    let frame = Frame {
        standing: None,
        fresh: Some(SAVE_FAILED),
        at_start: Some(ARC_HINT),
        commanded: false,
    };

    let (kept, message) = settled(frame, Some(ARC_HINT));

    assert_eq!(kept, Some(covering(SAVE_FAILED, Some(ARC_HINT))));
    assert_eq!(
        message.as_deref(),
        Some(ARC_HINT),
        "the error lies over the message, and does not wipe it",
    );
}

#[test]
fn a_standing_error_gives_way_to_a_command() {
    let frame = Frame {
        standing: Some(covering(SAVE_FAILED, None)),
        fresh: None,
        at_start: None,
        commanded: true,
    };

    let (kept, _) = settled(frame, None);

    assert_eq!(
        kept, None,
        "a command was asked, and the error is still kept"
    );
}

#[test]
fn a_standing_error_gives_way_to_a_message_said_since() {
    let frame = Frame {
        standing: Some(covering(SAVE_FAILED, Some(CHOOSE_A_PLANE))),
        fresh: None,
        at_start: Some(CHOOSE_A_PLANE),
        commanded: false,
    };

    let (kept, message) = settled(frame, Some(ARC_HINT));

    assert_eq!(
        kept, None,
        "a new message was said, and the error is still kept"
    );
    assert_eq!(
        message.as_deref(),
        Some(ARC_HINT),
        "the message said in the frame is the one the pill is left with",
    );
}

#[test]
fn a_message_said_again_unchanged_leaves_the_error_standing() {
    let standing = covering(SAVE_FAILED, Some(CHOOSE_A_PLANE));
    let frame = Frame {
        standing: Some(standing.clone()),
        fresh: None,
        at_start: Some(CHOOSE_A_PLANE),
        commanded: false,
    };

    let (kept, message) = settled(frame, Some(CHOOSE_A_PLANE));

    assert_eq!(
        kept,
        Some(standing),
        "the sentence choosing a plane says again at every frame took the error away",
    );
    assert_eq!(message.as_deref(), Some(CHOOSE_A_PLANE));
}

#[test]
fn the_message_an_error_covered_does_not_come_back_once_it_gives_way() {
    let frame = Frame {
        standing: Some(covering(SAVE_FAILED, Some(ARC_HINT))),
        fresh: None,
        at_start: Some(ARC_HINT),
        commanded: true,
    };

    let (kept, message) = settled(frame, Some(ARC_HINT));

    assert_eq!(kept, None);
    assert_eq!(
        message, None,
        "a command that leaves the message alone, as opening the Preferences does, \
         brought back the message the error had covered",
    );
}

#[test]
fn a_fresh_error_wins_over_a_message_said_in_the_same_frame() {
    let frame = Frame {
        standing: None,
        fresh: Some(SAVE_FAILED),
        at_start: Some(CHOOSE_A_PLANE),
        commanded: false,
    };

    let (kept, message) = settled(frame, Some(ARC_HINT));

    assert_eq!(kept, Some(covering(SAVE_FAILED, Some(ARC_HINT))));
    assert_eq!(message.as_deref(), Some(ARC_HINT));
}

#[test]
fn a_save_the_disk_keeps_refusing_keeps_the_pill() {
    let frame = Frame {
        standing: Some(covering(SAVE_FAILED, Some(CHOOSE_A_PLANE))),
        fresh: Some(SAVE_FAILED),
        at_start: Some(CHOOSE_A_PLANE),
        commanded: true,
    };

    let (kept, message) = settled(frame, Some(ARC_HINT));

    assert_eq!(
        kept,
        Some(covering(SAVE_FAILED, Some(ARC_HINT))),
        "a save failing again in a frame where a command ran and a hint was said is \
         not kept, or not over that hint",
    );
    assert_eq!(message.as_deref(), Some(ARC_HINT));
}

#[test]
fn a_standing_error_alone_stays() {
    let standing = covering(SAVE_FAILED, None);
    let frame = Frame {
        standing: Some(standing.clone()),
        fresh: None,
        at_start: None,
        commanded: false,
    };

    let (kept, message) = settled(frame, None);

    assert_eq!(
        kept,
        Some(standing),
        "a frame where nothing was said nor asked took the error away",
    );
    assert_eq!(message, None);
}

#[test]
fn each_problem_of_the_part_is_an_alert_at_its_level() {
    let read = [
        ("an area lost", levels_listed(&[a_block_whose_area_is_gone])),
        (
            "a volume not built",
            levels_listed(&[a_ball_under_a_slanted_circle_raised]),
        ),
        (
            "a step built unstably",
            levels_listed(&[a_shaft_cut_by_a_slice_a_hundredth_of_a_degree_thin]),
        ),
        (
            "a sketch off its face",
            levels_listed(&[a_sketch_off_its_face]),
        ),
        (
            "a section off its face",
            levels_listed(&[a_block, two_sections_one_off_its_face]),
        ),
        ("a sound part", levels_listed(&[a_block])),
    ];

    assert_eq!(
        read,
        [
            ("an area lost", vec![Level::Error]),
            ("a volume not built", vec![Level::Error]),
            ("a step built unstably", vec![Level::Warning]),
            ("a sketch off its face", vec![Level::Warning]),
            ("a section off its face", vec![Level::Warning]),
            ("a sound part", Vec::new()),
        ],
        "a problem of the part is not listed as one alert at its level, or a sound part \
         lists one",
    );
}

#[test]
fn the_title_bar_counts_the_alerts_only_while_there_is_one() {
    let unstable = a_shaft_cut_by_a_slice_a_hundredth_of_a_degree_thin;

    let read = [
        ("a sound part", the_count_s_words(&[a_block])),
        ("one problem", the_count_s_words(&[unstable])),
        (
            "two problems",
            the_count_s_words(&[unstable, a_sketch_off_its_face]),
        ),
    ];

    assert_eq!(
        read,
        [
            ("a sound part", None),
            ("one problem", Some("1 alerte".to_owned())),
            ("two problems", Some("2 alertes".to_owned())),
        ],
        "the title bar does not count the part's alerts, or counts them while there is none",
    );
}

#[test]
fn the_count_wears_the_level_of_the_worst_alert() {
    let unstable = a_shaft_cut_by_a_slice_a_hundredth_of_a_degree_thin;
    let declined = a_ball_under_a_slanted_circle_raised;

    let read = [
        ("a warning alone", the_count_s_level(&[unstable])),
        (
            "two warnings",
            the_count_s_level(&[unstable, a_sketch_off_its_face]),
        ),
        ("an error alone", the_count_s_level(&[declined])),
        (
            "an error after a warning",
            the_count_s_level(&[a_sketch_off_its_face, declined]),
        ),
        (
            "a warning after an error",
            the_count_s_level(&[declined, a_sketch_off_its_face]),
        ),
    ];

    assert_eq!(
        read,
        [
            ("a warning alone", Some(Level::Warning)),
            ("two warnings", Some(Level::Warning)),
            ("an error alone", Some(Level::Error)),
            ("an error after a warning", Some(Level::Error)),
            ("a warning after an error", Some(Level::Error)),
        ],
        "the count is not drawn at the level of the worst of the part's alerts",
    );
}

/// What `read` takes off the title bar of a part made of `laid`, with nothing
/// open on it.
fn read_off_the_bar<T>(laid: &[fn(&mut PartDocument)], read: impl FnOnce(&TitleBar<'_>) -> T) -> T {
    let document = a_part_of(laid);
    let editor = SketchEditor::default();
    let viewport = ViewportState::default();
    let lang = Catalogue::french();
    let bar = TitleBar::of(
        &document,
        Path::new("Test.caopart"),
        &editor,
        &viewport,
        None,
        &lang,
    );
    read(&bar)
}

/// The level of each alert the title bar lists, in its order.
fn levels_listed(laid: &[fn(&mut PartDocument)]) -> Vec<Level> {
    read_off_the_bar(laid, |bar| {
        bar.alerts.iter().map(|alert| alert.level).collect()
    })
}

/// What the title bar's count reads, nothing while it is not drawn.
fn the_count_s_words(laid: &[fn(&mut PartDocument)]) -> Option<String> {
    read_off_the_bar(laid, |bar| {
        bar.alerts_button
            .as_ref()
            .map(|count| count.text.to_string())
    })
}

/// The level the title bar's count is drawn at, nothing while it is not drawn.
fn the_count_s_level(laid: &[fn(&mut PartDocument)]) -> Option<Level> {
    read_off_the_bar(laid, |bar| {
        bar.alerts_button.as_ref().map(|count| count.level)
    })
}

/// A part made of these, laid one after the other.
fn a_part_of(laid: &[fn(&mut PartDocument)]) -> PartDocument {
    let mut document = PartDocument::new("Test", at("2026-01-02T09:00:00Z"));
    for lay in laid {
        lay(&mut document);
    }
    document
}

fn at(text: &str) -> DateTime<Utc> {
    text.parse().expect("a date")
}

/// A rectangle on the plane XY, its first trait dimensioned, raised into a
/// block.
fn a_block(document: &mut PartDocument) {
    let sketch = document.sketches().len();
    document.apply(Operation::CreateSketch {
        plane: WorkPlane::XY,
        on: None,
    });
    document.apply(Operation::AddRectangle {
        sketch,
        corner: PointRef::New(DVec2::ZERO),
        opposite: PointRef::New(DVec2::new(10.0, 20.0)),
        construction: false,
    });
    document.apply(Operation::SetDimension {
        sketch,
        target: DimensionTarget::Length(SegmentId(0)),
        value: 100.0.into(),
        placement: None,
    });
    document.apply(Operation::Extrude {
        sketch,
        areas: document.areas_at(sketch, &[DVec2::new(5.0, 10.0)]),
        distance: 4.0.into(),
        mode: ExtrusionMode::Add,
    });
}

/// `a_block`, the first trait of its rectangle then erased: the area the block
/// was raised from is gone.
fn a_block_whose_area_is_gone(document: &mut PartDocument) {
    let sketch = document.sketches().len();
    a_block(document);
    document.apply(Operation::EraseMany {
        sketch,
        elements: vec![Element::Segment(SegmentId(0))],
        dimensions: Vec::new(),
        constraints: Vec::new(),
    });
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

/// A ball of radius 4 about (5, 10, 0), then a circle on a plane tilted 30°
/// about X raised down across its sphere, three off its centre: the exact
/// kernel cannot follow where the two meet, and declines the circle's step.
fn a_ball_under_a_slanted_circle_raised(document: &mut PartDocument) {
    a_ball(document, DVec3::new(5.0, 10.0, 0.0), 4.0);
    let sketch = document.sketches().len();
    let (sin, cos) = (std::f64::consts::PI / 6.0).sin_cos();
    document.apply(Operation::CreateSketch {
        plane: WorkPlane {
            origin: DVec3::new(5.0, 10.0, 6.0),
            u: DVec3::X,
            v: DVec3::new(0.0, cos, sin),
        },
        on: None,
    });
    document.apply(Operation::AddCircle {
        sketch,
        center: PointRef::New(DVec2::ZERO),
        radius: 2.0,
        rim: Vec::new(),
        construction: false,
    });
    document.apply(Operation::Extrude {
        sketch,
        areas: document.areas_at(sketch, &[DVec2::ZERO]),
        distance: (-10.0).into(),
        mode: ExtrusionMode::Add,
    });
}

/// A shaft Ø20 raised 30 on XY, and a slice of it turned a hundredth of a
/// degree about X cut out: the slice's second side leans that much off along
/// the shaft's axis, and the kernel builds it though it reads it unstably.
fn a_shaft_cut_by_a_slice_a_hundredth_of_a_degree_thin(document: &mut PartDocument) {
    let shaft = document.sketches().len();
    document.apply(Operation::CreateSketch {
        plane: WorkPlane::XY,
        on: None,
    });
    document.apply(Operation::AddCircle {
        sketch: shaft,
        center: PointRef::New(DVec2::ZERO),
        radius: 10.0,
        rim: Vec::new(),
        construction: false,
    });
    document.apply(Operation::Extrude {
        sketch: shaft,
        areas: document.areas_at(shaft, &[DVec2::ZERO]),
        distance: 30.0.into(),
        mode: ExtrusionMode::Add,
    });
    let slice = document.sketches().len();
    document.apply(Operation::CreateSketch {
        plane: WorkPlane::XZ,
        on: None,
    });
    document.apply(Operation::AddRectangle {
        sketch: slice,
        corner: PointRef::New(DVec2::new(-20.0, 0.0)),
        opposite: PointRef::New(DVec2::new(20.0, 40.0)),
        construction: false,
    });
    document.apply(Operation::Revolve {
        sketch: slice,
        areas: document.areas_at(slice, &[DVec2::new(0.0, 20.0)]),
        axis: RevolutionAxis::Sketch(SketchAxis::U),
        angle: 0.01.into(),
        mode: ExtrusionMode::Cut,
    });
}

/// A sketch laid on a face the part does not have: it stays on the plane it
/// was given, adrift.
fn a_sketch_off_its_face(document: &mut PartDocument) {
    document.apply(Operation::CreateSketch {
        plane: WorkPlane::XY,
        on: Some(FaceAnchor {
            face: 404,
            up: DVec3::Y,
        }),
    });
}

/// Two sections kept, one on the plane XZ and one laid on a face the part
/// does not have.
fn two_sections_one_off_its_face(document: &mut PartDocument) {
    for on in [
        None,
        Some(FaceAnchor {
            face: 404,
            up: DVec3::Y,
        }),
    ] {
        document.apply(Operation::SaveSection(SectionAsked {
            plane: WorkPlane::XZ,
            on,
            offset: 3.0,
            behind: SideDrawing::SeeThrough,
            in_front: SideDrawing::Opaque,
            tilt_about_u_degrees: 0.0,
            tilt_about_v_degrees: 0.0,
            pivot: DVec3::ZERO,
        }));
    }
}
