//! An edge of the part chamfered or rounded as a step of its history: the
//! kernel's `Body::chamfer` and `Body::fillet` (#696, #697), called by a step
//! that names the edge as the body before it named it, and keeps the value
//! asked beside the value the faces gave.
//!
//! Closes #920 — the whole transcription is here until the application's half
//! has a file of its own; the bullets marked "not written yet" are taken up
//! one by one, each in the turn that writes its test.
//! - Outside a sketch, the part ribbon offers « Chanfrein » and « Congé »;
//!   clicking one opens its panel. Inside a sketch they still act on corners,
//!   unchanged — no test: not written yet, the ribbon and the panel are the
//!   application's, driven through `crates/app/tests/driver/`
//! - With the panel open, hovering an edge of the part in the 3D view lights
//!   it, and clicking it chooses it; clicking another edge replaces the
//!   first — no test: the kernel's half, every edge handed out drawn under
//!   its name, is held in `crates/solid/tests/an_edge_is_drawn_where_its_name_finds_it.rs`
//!   by `every_edge_of_a_block_is_drawn_once_under_its_name_from_corner_to_corner`
//!   and the three tests beside it; the viewport's hover and click are not
//!   written yet
//! - An edge chosen, a value typed and « Appliquer » pressed, the part is
//!   chamfered or rounded on that edge and one new line appears in the
//!   history — the part's half, one new step of its own kind taking off what
//!   the arithmetic gives on the edge named, by
//!   `a_chamfer_or_a_fillet_on_an_edge_of_the_part_takes_off_what_the_arithmetic_gives_as_one_new_step`;
//!   the panel and the history's line are still to be written
//! - When the kernel reduced the value, that history line reads both values,
//!   as « 5 mm demandé, 2 mm appliqué »; otherwise only the value — the part's
//!   half, both values kept when the faces give less and nothing reduced when
//!   they give enough, by
//!   `a_value_the_faces_cannot_give_is_reduced_and_the_part_keeps_both` and
//!   `a_chamfer_or_a_fillet_on_an_edge_of_the_part_takes_off_what_the_arithmetic_gives_as_one_new_step`;
//!   the line's wording is still to be written
//! - The step is written in the part file and replays on reopening, and when
//!   a size changes upstream. An edge it no longer finds makes it a broken
//!   step carrying its reason — written and opened again, by the design alone
//!   and on its cache, by
//!   `a_part_holding_a_chamfer_or_a_fillet_opens_again_as_it_was_saved_or_put_away`;
//!   replayed as a variable makes the block lower, then higher, the value
//!   taken following the faces, by
//!   `a_chamfer_or_a_fillet_is_replayed_on_the_block_its_height_variable_changes`;
//!   its edge parted in two by a notch moved across it upstream, by
//!   `an_edge_a_change_upstream_parts_in_two_makes_its_chamfer_or_fillet_a_broken_step_saying_it_is_lost`
//! - found when the step was first made: compacting the history keeps it, on
//!   its edge and with both its values —
//!   `a_history_compacted_keeps_its_chamfer_or_fillet`
//! - A decline of the kernel at « Appliquer » records nothing: the panel stays
//!   open and shows the reason in French — the part's half, the history and
//!   the body left as they were and the kernel's reason handed back with the
//!   outcome, on a disc's rim, which the kernel declines until #700, by
//!   `a_chamfer_or_a_fillet_the_kernel_declines_records_nothing_and_says_why`;
//!   an edge lost on a later replay stays a broken step, as above. The panel
//!   left open and the French sentence are still to be written
//! - Undo removes the step and redo puts it back —
//!   `undo_takes_a_chamfer_or_a_fillet_off_and_redo_lays_it_again`
//! - Face picking for a sketch, the hovered face and the section view behave
//!   as before; every existing test still passes — no test: held by the
//!   existing suite, which this branch leaves as it found it

use std::cell::RefCell;
use std::collections::HashMap;
use std::f64::consts::PI;
use std::path::{Path, PathBuf};

use cao_part::history::{BlendShape, Operation, PointRef, StepKind};
use cao_part::{
    ExtrusionMode, FileError, Files, Formula, Outcome, PartDocument, Reduced, VariableChange,
    VariableId,
};
use cao_sketch::WorkPlane;
use cao_solid::{Body, Declined, EdgeName};
use glam::{DVec2, DVec3};

/// Every face here is a plane or a cylinder the exact kernel computes: what is
/// left between it and the arithmetic is rounding.
const EXACT: f64 = 1e-9;

const LENGTH: f64 = 30.0;
const BREADTH: f64 = 10.0;
const HEIGHT: f64 = 20.0;
const BLOCK: f64 = LENGTH * BREADTH * HEIGHT;

fn at_nine() -> chrono::DateTime<chrono::Utc> {
    "2026-11-12T09:00:00Z".parse().expect("a date")
}

/// The block 30 along X, 10 along Y, raised as high as the variable `height`,
/// at 20.
fn a_block() -> PartDocument {
    let mut part = PartDocument::new("Block", at_nine());
    part.apply(Operation::Variable(VariableChange::Added {
        name: "height".to_string(),
        formula: Formula::Number(HEIGHT),
    }));
    part.apply(Operation::CreateSketch {
        plane: WorkPlane::XY,
        on: None,
    });
    rectangle(&mut part, 0, DVec2::ZERO, DVec2::new(LENGTH, BREADTH));
    let areas = part.areas_at(0, &[DVec2::new(LENGTH / 2.0, BREADTH / 2.0)]);
    let height = part.variables().read("height").expect("height reads");
    part.apply(Operation::Extrude {
        sketch: 0,
        areas,
        distance: height,
        mode: ExtrusionMode::Add,
    });
    assert_near(part.body().volume(), BLOCK, "the block before any edge");
    part
}

fn rectangle(part: &mut PartDocument, sketch: usize, corner: DVec2, opposite: DVec2) {
    part.apply(Operation::AddRectangle {
        sketch,
        corner: PointRef::New(corner),
        opposite: PointRef::New(opposite),
        construction: false,
    });
}

fn the_height_set_to(part: &mut PartDocument, height: f64) {
    part.apply(Operation::Variable(VariableChange::Edited {
        variable: VariableId(0),
        name: "height".to_string(),
        formula: Formula::Number(height),
    }));
}

/// The one face facing `normal` whose plane stands `level` along it.
fn face(body: &Body, normal: DVec3, level: f64) -> u32 {
    let faces: Vec<usize> = (0..body.faces_end())
        .filter(|face| {
            body.plane_of(*face).is_some_and(|plane| {
                plane.normal.distance(normal) < EXACT
                    && (plane.corners[0].dot(normal) - level).abs() < EXACT
            })
        })
        .collect();
    assert_eq!(
        faces.len(),
        1,
        "one face facing {normal} at {level}: {faces:?}"
    );
    faces[0] as u32
}

fn the_edge_between(body: &Body, first: u32, second: u32) -> EdgeName {
    let sides = [vec![first.min(second)], vec![first.max(second)]];
    let named: Vec<EdgeName> = body
        .edge_names()
        .into_iter()
        .filter(|name| name.between == sides)
        .collect();
    assert_eq!(named.len(), 1, "one edge between {first} and {second}");
    named[0].clone()
}

/// The edge 30 long between the top and the front, which faces -Y: the top
/// 10 deep behind it, the front as high as the block under it.
fn the_top_front_edge(part: &PartDocument) -> EdgeName {
    let body = part.body();
    let (_, top) = body.bounds().expect("some matter");
    the_edge_between(
        body,
        face(body, DVec3::Z, top.z),
        face(body, -DVec3::Y, 0.0),
    )
}

fn applied_on_the_top_front_edge(part: &mut PartDocument, shape: BlendShape, size: f64) -> u32 {
    let edge = the_top_front_edge(part);
    applied_on(part, edge, shape, size)
}

fn applied_on(part: &mut PartDocument, edge: EdgeName, shape: BlendShape, size: f64) -> u32 {
    let steps = part.history.steps().len();
    part.apply(Operation::Blend {
        edge,
        shape,
        size: size.into(),
    });

    assert_eq!(
        part.history.steps().len(),
        steps + 1,
        "{shape:?} {size}: one new step"
    );
    let step = part.history.steps().last().expect("a step");
    assert_eq!(step.kind(), StepKind::Blend, "{shape:?} {size}: its kind");
    assert_eq!(step.len(), 1, "{shape:?} {size}: one operation under it");
    let number = step.operations()[0];
    assert_eq!(
        part.declined_because(number),
        None,
        "{shape:?} {size}: built"
    );
    assert!(part.body().is_exact(), "{shape:?} {size}: still exact");
    number
}

/// What a chamfer or a fillet of `size` takes off a square edge `length` long.
fn taken_off(shape: BlendShape, size: f64, length: f64) -> f64 {
    match shape {
        BlendShape::Chamfer => size * size * length / 2.0,
        BlendShape::Fillet => (1.0 - PI / 4.0) * size * size * length,
    }
}

fn assert_reduced(reduced: Option<Reduced>, asked: f64, taken: f64, what: &str) {
    let reduced = reduced.unwrap_or_else(|| panic!("{what}: {asked} does not fit, and is reduced"));
    assert_near(reduced.asked, asked, &format!("{what}: asked"));
    assert_near(reduced.taken, taken, &format!("{what}: taken"));
}

fn assert_near(made: f64, expected: f64, what: &str) {
    assert!(
        (made - expected).abs() <= EXACT * expected.abs().max(1.0),
        "{what}: {made} where the arithmetic gives {expected}",
    );
}

/// How far down from 100 above the block a ray falling at `across` from the
/// front first meets the part.
fn met_from_above(body: &Body, across: f64) -> f64 {
    body.ray_hit(DVec3::new(13.7, across, 100.0), -DVec3::Z)
        .expect("the ray meets the part")
        .distance
}

#[test]
fn a_chamfer_or_a_fillet_on_an_edge_of_the_part_takes_off_what_the_arithmetic_gives_as_one_new_step()
 {
    let size: f64 = 2.0;
    for shape in [BlendShape::Chamfer, BlendShape::Fillet] {
        let mut part = a_block();
        let step = applied_on_the_top_front_edge(&mut part, shape, size);

        assert_near(
            part.body().volume(),
            BLOCK - taken_off(shape, size, LENGTH),
            &format!("{shape:?} of {size} on the top front edge"),
        );
        assert!(
            met_from_above(part.body(), 0.1) > 100.0 - HEIGHT + 1.0,
            "{shape:?}: the top no longer reaches the front",
        );
        assert_near(
            met_from_above(part.body(), BREADTH - 0.1),
            100.0 - HEIGHT,
            &format!("{shape:?}: the top still reaches the back"),
        );
        assert!(
            part.reduced_at(step).is_none(),
            "{shape:?}: {size} fits, nothing reduced, {:?}",
            part.reduced_at(step),
        );
    }
}

#[test]
fn a_value_the_faces_cannot_give_is_reduced_and_the_part_keeps_both() {
    let asked = 50.0;
    for shape in [BlendShape::Chamfer, BlendShape::Fillet] {
        let mut part = a_block();
        let step = applied_on_the_top_front_edge(&mut part, shape, asked);

        assert_near(
            part.body().volume(),
            BLOCK - taken_off(shape, BREADTH, LENGTH),
            &format!("{shape:?} of {asked} asked, the top 10 deep"),
        );
        assert_reduced(part.reduced_at(step), asked, BREADTH, &format!("{shape:?}"));
    }
}

#[derive(Default)]
struct Memory(RefCell<HashMap<PathBuf, Vec<u8>>>);

impl Files for Memory {
    fn read(&self, path: &Path) -> Result<Vec<u8>, FileError> {
        self.0
            .borrow()
            .get(path)
            .cloned()
            .ok_or_else(|| FileError::Absent(path.to_path_buf()))
    }

    fn write(&self, path: &Path, bytes: &[u8]) -> Result<(), FileError> {
        self.0
            .borrow_mut()
            .insert(path.to_path_buf(), bytes.to_vec());
        Ok(())
    }

    fn exists(&self, path: &Path) -> bool {
        self.0.borrow().contains_key(path)
    }
}

#[test]
fn a_part_holding_a_chamfer_or_a_fillet_opens_again_as_it_was_saved_or_put_away() {
    let asked = 50.0;
    let files = Memory::default();
    for shape in [BlendShape::Chamfer, BlendShape::Fillet] {
        let mut part = a_block();
        let step = applied_on_the_top_front_edge(&mut part, shape, asked);
        let path = Path::new("/parts/Block.caopart");

        for how in ["saved", "put away"] {
            let written = match how {
                "saved" => part.save(&files, path, at_nine()),
                _ => part.put_away(&files, path, at_nine()),
            };
            written.unwrap_or_else(|err| panic!("{shape:?} {how}: not written, {err:?}"));
            let opened = PartDocument::load(&files, path)
                .unwrap_or_else(|err| panic!("{shape:?} {how}: does not open, {err:?}"));
            let what = format!("{shape:?}, {how} and opened again");

            assert_eq!(opened.history.steps(), part.history.steps(), "{what}");
            assert_eq!(opened.declined_because(step), None, "{what}: built");
            assert_near(
                opened.body().volume(),
                BLOCK - taken_off(shape, BREADTH, LENGTH),
                &what,
            );
            assert_reduced(opened.reduced_at(step), asked, BREADTH, &what);
        }
    }
}

#[test]
fn a_chamfer_or_a_fillet_is_replayed_on_the_block_its_height_variable_changes() {
    let asked = 15.0;
    for shape in [BlendShape::Chamfer, BlendShape::Fillet] {
        let mut part = a_block();
        let step = applied_on_the_top_front_edge(&mut part, shape, asked);

        for (height, taken) in [(8.0, 8.0), (25.0, BREADTH)] {
            the_height_set_to(&mut part, height);
            let what = format!("{shape:?} of {asked} asked, the block {height} high");

            assert_eq!(part.declined_because(step), None, "{what}: built");
            assert!(part.body().is_exact(), "{what}: exact");
            assert_near(
                part.body().volume(),
                LENGTH * BREADTH * height - taken_off(shape, taken, LENGTH),
                &what,
            );
            assert_reduced(part.reduced_at(step), asked, taken, &what);
        }
    }
}

/// A notch 10 wide and 5 high cut into the foot of the block, across its back
/// or across its front, from a second drawing on XY holding both outlines.
const THE_BACK_NOTCH: DVec2 = DVec2::new(15.0, 10.0);
const THE_FRONT_NOTCH: DVec2 = DVec2::new(15.0, 0.0);
const NOTCH: f64 = 10.0 * 3.0 * 5.0;

fn notched_across(part: &PartDocument, notch: DVec2) -> Operation {
    Operation::Extrude {
        sketch: 1,
        areas: part.areas_at(1, &[notch]),
        distance: 5.0.into(),
        mode: ExtrusionMode::Cut,
    }
}

#[test]
fn an_edge_a_change_upstream_parts_in_two_makes_its_chamfer_or_fillet_a_broken_step_saying_it_is_lost()
 {
    let size = 2.0;
    for shape in [BlendShape::Chamfer, BlendShape::Fillet] {
        let mut part = a_block();
        part.apply(Operation::CreateSketch {
            plane: WorkPlane::XY,
            on: None,
        });
        rectangle(&mut part, 1, DVec2::new(10.0, 7.0), DVec2::new(20.0, 13.0));
        rectangle(&mut part, 1, DVec2::new(10.0, -3.0), DVec2::new(20.0, 3.0));
        part.apply(notched_across(&part, THE_BACK_NOTCH));
        let notch = part.history.steps().last().expect("the notch").operations()[0];
        let body = part.body();
        let floor_front =
            the_edge_between(body, face(body, -DVec3::Z, 0.0), face(body, -DVec3::Y, 0.0));
        let step = applied_on(&mut part, floor_front, shape, size);
        assert_near(
            part.body().volume(),
            BLOCK - NOTCH - taken_off(shape, size, LENGTH),
            &format!("{shape:?} on the foot of the front, the notch across the back"),
        );

        let across_the_front = notched_across(&part, THE_FRONT_NOTCH);
        part.apply(Operation::EditMatter {
            raised_by: notch,
            to: Box::new(across_the_front),
        });
        let what = format!("{shape:?} on the foot of the front, the notch moved across it");

        assert!(
            matches!(part.declined_because(step), Some(Declined::Lost(_))),
            "{what}: broken, its edge lost, where it reads {:?}",
            part.declined_because(step),
        );
        assert!(
            part.history
                .steps()
                .iter()
                .any(|kept| kept.kind() == StepKind::Blend && kept.operations()[0] == step),
            "{what}: still a step of the history",
        );
        assert!(part.body().is_exact(), "{what}: exact");
        assert_near(
            part.body().volume(),
            BLOCK - NOTCH,
            &format!("{what}: the notched block alone"),
        );
    }
}

#[test]
fn a_history_compacted_keeps_its_chamfer_or_fillet() {
    let asked = 50.0;
    for shape in [BlendShape::Chamfer, BlendShape::Fillet] {
        let mut part = a_block();
        applied_on_the_top_front_edge(&mut part, shape, asked);

        part.compact_history();
        let what = format!("{shape:?}, the history compacted");

        let step = part
            .history
            .steps()
            .iter()
            .find(|step| step.kind() == StepKind::Blend)
            .unwrap_or_else(|| panic!("{what}: the step is gone"))
            .operations()[0];
        assert_eq!(part.declined_because(step), None, "{what}: built");
        assert_near(
            part.body().volume(),
            BLOCK - taken_off(shape, BREADTH, LENGTH),
            &what,
        );
        assert_reduced(part.reduced_at(step), asked, BREADTH, &what);
    }
}

#[test]
fn a_chamfer_or_a_fillet_the_kernel_declines_records_nothing_and_says_why() {
    for shape in [BlendShape::Chamfer, BlendShape::Fillet] {
        let mut part = PartDocument::new("Disc", at_nine());
        part.apply(Operation::CreateSketch {
            plane: WorkPlane::XY,
            on: None,
        });
        part.apply(Operation::AddCircle {
            sketch: 0,
            center: PointRef::New(DVec2::ZERO),
            radius: 5.0,
            rim: Vec::new(),
            construction: false,
        });
        let areas = part.areas_at(0, &[DVec2::ZERO]);
        part.apply(Operation::Extrude {
            sketch: 0,
            areas,
            distance: 10.0.into(),
            mode: ExtrusionMode::Add,
        });
        let disc = PI * 25.0 * 10.0;
        let top = face(part.body(), DVec3::Z, 10.0);
        let rim = part
            .body()
            .edge_names()
            .into_iter()
            .find(|name| name.between.contains(&vec![top]))
            .expect("the rim of the top");
        let before = part.history.clone();
        let what = format!("{shape:?} on the rim of a disc");

        let outcome = part.apply(Operation::Blend {
            edge: rim,
            shape,
            size: 1.0.into(),
        });

        assert_eq!(
            outcome,
            Some(Outcome::BlendDeclined(Declined::Unsupported)),
            "{what}: the kernel's reason handed back",
        );
        assert_eq!(part.history, before, "{what}: nothing recorded");
        assert!(part.body().is_exact(), "{what}: exact");
        assert_near(
            part.body().volume(),
            disc,
            &format!("{what}: the disc as it was"),
        );

        part.undo();
        assert!(
            part.body().is_empty(),
            "{what}: one undo takes the disc's extrusion back, there being no step after it",
        );
    }
}

#[test]
fn undo_takes_a_chamfer_or_a_fillet_off_and_redo_lays_it_again() {
    let asked = 50.0;
    for shape in [BlendShape::Chamfer, BlendShape::Fillet] {
        let mut part = a_block();
        let before = part.history.replay_order().len();
        let step = applied_on_the_top_front_edge(&mut part, shape, asked);
        let laid = part.history.steps().to_vec();

        assert!(part.undo(), "{shape:?}: something to undo");
        let what = format!("{shape:?} undone");
        assert_eq!(part.history.replay_order().len(), before, "{what}");
        assert!(
            part.history
                .replay_order()
                .iter()
                .all(|(number, _)| *number != step),
            "{what}: the step is no longer replayed",
        );
        assert_near(
            part.body().volume(),
            BLOCK,
            &format!("{what}: the block whole"),
        );
        assert!(
            part.reduced_at(step).is_none(),
            "{what}: nothing reduced on a step not in effect, {:?}",
            part.reduced_at(step),
        );

        assert!(part.redo(), "{shape:?}: something to redo");
        let what = format!("{shape:?} redone");
        assert_eq!(part.history.steps(), laid.as_slice(), "{what}");
        assert_eq!(part.declined_because(step), None, "{what}: built");
        assert_near(
            part.body().volume(),
            BLOCK - taken_off(shape, BREADTH, LENGTH),
            &what,
        );
        assert_reduced(part.reduced_at(step), asked, BREADTH, &what);
    }
}
