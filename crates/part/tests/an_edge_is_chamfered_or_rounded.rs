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
//!   first — no test: not written yet, it needs each edge's name with its
//!   shape out of `cao_solid`, then the viewport's hover and click
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
//!   step carrying its reason — no test: not written yet
//! - A decline of the kernel at « Appliquer » records nothing: the panel stays
//!   open and shows the reason in French — no test: not written yet, the
//!   part's half (nothing recorded, the reason in the outcome) and the
//!   panel's
//! - Undo removes the step and redo puts it back — no test: not written yet
//! - Face picking for a sketch, the hovered face and the section view behave
//!   as before; every existing test still passes — no test: held by the
//!   existing suite, which this branch leaves as it found it

use std::f64::consts::PI;

use cao_part::history::{BlendShape, Operation, PointRef, StepKind};
use cao_part::{ExtrusionMode, PartDocument};
use cao_sketch::WorkPlane;
use cao_solid::{Body, EdgeName};
use glam::{DVec2, DVec3};

/// Every face here is a plane or a cylinder the exact kernel computes: what is
/// left between it and the arithmetic is rounding.
const EXACT: f64 = 1e-9;

const LENGTH: f64 = 30.0;
const BREADTH: f64 = 10.0;
const HEIGHT: f64 = 20.0;
const BLOCK: f64 = LENGTH * BREADTH * HEIGHT;

fn a_block() -> PartDocument {
    let now = "2026-11-12T09:00:00Z".parse().expect("a date");
    let mut part = PartDocument::new("Block", now);
    part.apply(Operation::CreateSketch {
        plane: WorkPlane::XY,
        on: None,
    });
    part.apply(Operation::AddRectangle {
        sketch: 0,
        corner: PointRef::New(DVec2::ZERO),
        opposite: PointRef::New(DVec2::new(LENGTH, BREADTH)),
        construction: false,
    });
    let areas = part.areas_at(0, &[DVec2::new(LENGTH / 2.0, BREADTH / 2.0)]);
    part.apply(Operation::Extrude {
        sketch: 0,
        areas,
        distance: HEIGHT.into(),
        mode: ExtrusionMode::Add,
    });
    assert_near(part.body().volume(), BLOCK, "the block before any edge");
    part
}

fn facing(body: &Body, normal: DVec3) -> u32 {
    let faces: Vec<usize> = (0..body.faces_end())
        .filter(|face| {
            body.plane_of(*face)
                .is_some_and(|plane| plane.normal.distance(normal) < EXACT)
        })
        .collect();
    assert_eq!(faces.len(), 1, "one face facing {normal}: {faces:?}");
    faces[0] as u32
}

/// The edge 30 long between the top and the front, which faces -Y: the top
/// 10 deep behind it, the front 20 high under it.
fn the_top_front_edge(body: &Body) -> EdgeName {
    let top = facing(body, DVec3::Z);
    let front = facing(body, -DVec3::Y);
    let sides = [vec![top.min(front)], vec![top.max(front)]];
    let named: Vec<EdgeName> = body
        .edge_names()
        .into_iter()
        .filter(|name| name.between == sides)
        .collect();
    assert_eq!(named.len(), 1, "one edge between the top and the front");
    named[0].clone()
}

fn applied_on_the_top_front_edge(part: &mut PartDocument, shape: BlendShape, size: f64) -> u32 {
    let steps = part.history.steps().len();
    let edge = the_top_front_edge(part.body());
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
    for (shape, taken_off) in [
        (BlendShape::Chamfer, size * size * LENGTH / 2.0),
        (BlendShape::Fillet, (1.0 - PI / 4.0) * size * size * LENGTH),
    ] {
        let mut part = a_block();
        let step = applied_on_the_top_front_edge(&mut part, shape, size);

        assert_near(
            part.body().volume(),
            BLOCK - taken_off,
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
    for (shape, taken_off) in [
        (BlendShape::Chamfer, BREADTH * BREADTH * LENGTH / 2.0),
        (
            BlendShape::Fillet,
            (1.0 - PI / 4.0) * BREADTH * BREADTH * LENGTH,
        ),
    ] {
        let mut part = a_block();
        let step = applied_on_the_top_front_edge(&mut part, shape, asked);

        assert_near(
            part.body().volume(),
            BLOCK - taken_off,
            &format!("{shape:?} of {asked} asked, the top 10 deep"),
        );
        let reduced = part
            .reduced_at(step)
            .unwrap_or_else(|| panic!("{shape:?}: {asked} does not fit, and is reduced"));
        assert_near(reduced.asked, asked, &format!("{shape:?}: asked"));
        assert_near(reduced.taken, BREADTH, &format!("{shape:?}: taken"));
    }
}
