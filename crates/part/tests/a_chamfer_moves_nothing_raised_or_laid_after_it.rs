//! A chamfer numbers the face it lays where the steps' numbers ended, as the
//! kernel numbers it: what is raised or laid after it, a face a sketch stands
//! on, a later chamfer's edge, must find its own faces again whatever becomes
//! of the chamfer — built, declined on a later replay, or with a step put in
//! before it while the part is read to an earlier one.
//!
//! Closes #920 — the whole transcription is in
//! `crates/part/tests/an_edge_is_chamfered_or_rounded.rs`; these bullets are
//! what review found the step must not break.
//! - Face picking for a sketch behaves as before — found in review: a sketch
//!   on a face of an extrusion raised after a chamfer opens on that face, not
//!   on another —
//!   `a_sketch_on_a_face_raised_after_a_chamfer_opens_on_that_face`; and a
//!   sketch laid on the chamfer's own face stays on it once another extrusion
//!   is raised after it —
//!   `a_sketch_on_a_chamfer_s_own_face_stays_on_it_after_another_extrusion`
//! - The step replays when a size changes upstream; an edge it no longer finds
//!   makes it a broken step — found in review: and only it, a chamfer after
//!   it still standing on its own edge, a sketch laid after both where it was —
//!   `a_chamfer_declined_on_a_later_replay_leaves_the_next_chamfer_on_its_edge`
//! - Every part replays as today — found in review: a step put in before a
//!   chamfer while the part is read to an earlier step leaves the chamfer on
//!   the edge it was given —
//!   `a_step_put_in_before_a_chamfer_while_the_part_is_read_earlier_leaves_it_on_its_edge`

use cao_part::history::{BlendShape, ExtrusionMode, FaceAnchor, Operation, PointRef};
use cao_part::{PartDocument, PartState};
use cao_sketch::WorkPlane;
use cao_solid::{Body, Declined, EdgeName};
use glam::{DVec2, DVec3};

/// Every face here is a plane the exact kernel computes: what is left between
/// it and the arithmetic is rounding.
const EXACT: f64 = 1e-9;

const SIZE: f64 = 2.0;

fn at_nine() -> chrono::DateTime<chrono::Utc> {
    "2026-11-12T09:00:00Z".parse().expect("a date")
}

fn rectangle(part: &mut PartDocument, sketch: usize, corner: DVec2, opposite: DVec2) {
    part.apply(Operation::AddRectangle {
        sketch,
        corner: PointRef::New(corner),
        opposite: PointRef::New(opposite),
        construction: false,
    });
}

fn raise(
    part: &mut PartDocument,
    sketch: usize,
    inside: DVec2,
    distance: f64,
    mode: ExtrusionMode,
) {
    let areas = part.areas_at(sketch, &[inside]);
    part.apply(Operation::Extrude {
        sketch,
        areas,
        distance: distance.into(),
        mode,
    });
}

fn a_sketch_on_xy(part: &mut PartDocument) -> usize {
    part.apply(Operation::CreateSketch {
        plane: WorkPlane::XY,
        on: None,
    });
    part.sketches().len() - 1
}

/// A block 30 along X, 10 along Y and 20 high.
fn a_block() -> PartDocument {
    let mut part = PartDocument::new("Block", at_nine());
    let sketch = a_sketch_on_xy(&mut part);
    rectangle(&mut part, sketch, DVec2::ZERO, DVec2::new(30.0, 10.0));
    raise(
        &mut part,
        sketch,
        DVec2::new(15.0, 5.0),
        20.0,
        ExtrusionMode::Add,
    );
    part
}

/// The one face facing `normal`, standing `level` along it, with every corner
/// at `from_x` or further along X.
fn face(body: &Body, normal: DVec3, level: f64, from_x: f64) -> usize {
    let faces: Vec<usize> = (0..body.faces_end())
        .filter(|face| {
            body.plane_of(*face).is_some_and(|plane| {
                plane.normal.distance(normal) < EXACT
                    && (plane.corners[0].dot(normal) - level).abs() < EXACT
                    && plane
                        .corners
                        .iter()
                        .all(|corner| corner.x >= from_x - EXACT)
            })
        })
        .collect();
    assert_eq!(
        faces.len(),
        1,
        "one face facing {normal} at {level} from x = {from_x}: {faces:?}"
    );
    faces[0]
}

fn edge_between(body: &Body, first: usize, second: usize) -> EdgeName {
    let sides = [
        vec![first.min(second) as u32],
        vec![first.max(second) as u32],
    ];
    body.edge_names()
        .into_iter()
        .find(|name| name.between == sides)
        .unwrap_or_else(|| panic!("an edge between {first} and {second}"))
}

/// Chamfers `edge` by [`SIZE`] and hands back the number of its step, built.
fn chamfered(part: &mut PartDocument, edge: EdgeName) -> u32 {
    let outcome = part.apply(Operation::Blend {
        edge,
        shape: BlendShape::Chamfer,
        size: SIZE.into(),
    });
    assert_eq!(outcome, None, "the chamfer is built");
    part.history.steps().last().expect("a step").operations()[0]
}

/// The edge between the top and the front of what was raised from `from_x`
/// on, `height` high, its front standing at `front` along Y.
fn top_front(body: &Body, from_x: f64, height: f64, front: f64) -> EdgeName {
    edge_between(
        body,
        face(body, DVec3::Z, height, from_x),
        face(body, -DVec3::Y, -front, from_x),
    )
}

fn assert_on(plane: WorkPlane, normal: DVec3, level: f64, what: &str) {
    let facing = plane.u.cross(plane.v);
    assert!(
        facing.distance(normal) < 1e-6 && (plane.origin.dot(normal) - level).abs() < 1e-6,
        "{what}: the sketch stands at {} facing {facing}, where the face stands {level} along {normal}",
        plane.origin,
    );
}

fn assert_near(made: f64, expected: f64, what: &str) {
    assert!(
        (made - expected).abs() <= EXACT * expected.abs().max(1.0),
        "{what}: {made} where the arithmetic gives {expected}",
    );
}

/// How high the part stands under a ray falling at (`x`, `y`).
fn height_at(body: &Body, x: f64, y: f64) -> f64 {
    100.0
        - body
            .ray_hit(DVec3::new(x, y, 100.0), -DVec3::Z)
            .expect("the ray meets the part")
            .distance
}

#[test]
fn a_sketch_on_a_face_raised_after_a_chamfer_opens_on_that_face() {
    let mut part = a_block();
    let edge = top_front(part.body(), 0.0, 20.0, 0.0);
    chamfered(&mut part, edge);
    let sketch = a_sketch_on_xy(&mut part);
    rectangle(
        &mut part,
        sketch,
        DVec2::new(40.0, 0.0),
        DVec2::new(50.0, 10.0),
    );
    raise(
        &mut part,
        sketch,
        DVec2::new(45.0, 5.0),
        5.0,
        ExtrusionMode::Add,
    );

    let top = face(part.body(), DVec3::Z, 5.0, 40.0);
    part.apply(Operation::CreateSketch {
        plane: WorkPlane::XY,
        on: Some(FaceAnchor {
            face: top,
            up: DVec3::Y,
        }),
    });
    let on_it = part.sketches().len() - 1;

    let what = "a sketch on the top of a block raised 5 after the chamfer";
    assert!(!part.is_adrift(on_it), "{what}: adrift");
    assert_on(part.sketches()[on_it].plane, DVec3::Z, 5.0, what);
    assert_on(
        PartState::rebuild(&part.history).sketches[on_it].plane,
        DVec3::Z,
        5.0,
        &format!("{what}, replayed"),
    );
}

#[test]
fn a_sketch_on_a_chamfer_s_own_face_stays_on_it_after_another_extrusion() {
    let mut part = a_block();
    let edge = top_front(part.body(), 0.0, 20.0, 0.0);
    chamfered(&mut part, edge);
    let slant = DVec3::new(0.0, -1.0, 1.0).normalize();
    let level = DVec3::new(0.0, 0.0, 20.0 - SIZE).dot(slant);
    let chamfer = face(part.body(), slant, level, 0.0);
    part.apply(Operation::CreateSketch {
        plane: WorkPlane::XY,
        on: Some(FaceAnchor {
            face: chamfer,
            up: DVec3::X,
        }),
    });
    let on_it = part.sketches().len() - 1;
    assert_on(
        part.sketches()[on_it].plane,
        slant,
        level,
        "a sketch on the chamfer",
    );

    let sketch = a_sketch_on_xy(&mut part);
    rectangle(
        &mut part,
        sketch,
        DVec2::new(40.0, 0.0),
        DVec2::new(50.0, 10.0),
    );
    raise(
        &mut part,
        sketch,
        DVec2::new(45.0, 5.0),
        5.0,
        ExtrusionMode::Add,
    );

    let replayed = PartState::rebuild(&part.history);
    assert!(
        !replayed.adrift.contains(&on_it),
        "the sketch on the chamfer is adrift once a block is raised after it",
    );
    assert_on(
        replayed.sketches[on_it].plane,
        slant,
        level,
        "a sketch on the chamfer, a block raised after it, replayed",
    );
}

#[test]
fn a_chamfer_declined_on_a_later_replay_leaves_the_next_chamfer_on_its_edge() {
    let mut part = a_block();
    let notches = a_sketch_on_xy(&mut part);
    rectangle(
        &mut part,
        notches,
        DVec2::new(10.0, 7.0),
        DVec2::new(20.0, 13.0),
    );
    rectangle(
        &mut part,
        notches,
        DVec2::new(10.0, -3.0),
        DVec2::new(20.0, 3.0),
    );
    raise(
        &mut part,
        notches,
        DVec2::new(15.0, 10.0),
        5.0,
        ExtrusionMode::Cut,
    );
    let notch = part.history.steps().last().expect("the notch").operations()[0];
    let body = part.body();
    let foot_of_the_front = edge_between(
        body,
        face(body, -DVec3::Z, 0.0, 0.0),
        face(body, -DVec3::Y, 0.0, 0.0),
    );
    let first = chamfered(&mut part, foot_of_the_front);
    let boss = a_sketch_on_xy(&mut part);
    rectangle(
        &mut part,
        boss,
        DVec2::new(22.0, 2.0),
        DVec2::new(27.0, 8.0),
    );
    raise(
        &mut part,
        boss,
        DVec2::new(24.5, 5.0),
        30.0,
        ExtrusionMode::Add,
    );
    let edge = top_front(part.body(), 22.0, 30.0, 2.0);
    let second = chamfered(&mut part, edge);
    part.apply(Operation::CreateSketch {
        plane: WorkPlane::XY,
        on: Some(FaceAnchor {
            face: face(part.body(), DVec3::Z, 30.0, 22.0),
            up: DVec3::Y,
        }),
    });
    let sketch = part.sketches().len() - 1;

    let across_the_front = Operation::Extrude {
        sketch: notches,
        areas: part.areas_at(notches, &[DVec2::new(15.0, 0.0)]),
        distance: 5.0.into(),
        mode: ExtrusionMode::Cut,
    };
    part.apply(Operation::EditMatter {
        raised_by: notch,
        to: Box::new(across_the_front),
    });

    let what = "the notch moved across the foot of the front";
    assert!(
        matches!(part.declined_because(first), Some(Declined::Lost(_))),
        "{what}: the first chamfer reads {:?}",
        part.declined_because(first),
    );
    assert_eq!(
        part.declined_because(second),
        None,
        "{what}: the chamfer on the boss",
    );
    assert_near(
        height_at(part.body(), 24.5, 2.1),
        30.0 - (SIZE - 0.1),
        &format!("{what}: the boss's front, under its chamfer"),
    );
    let block = 30.0 * 10.0 * 20.0 - 10.0 * 3.0 * 5.0;
    let boss_over_it = 5.0 * 6.0 * 10.0 - SIZE * SIZE * 5.0 / 2.0;
    assert_near(part.body().volume(), block + boss_over_it, what);
    assert!(
        !part.is_adrift(sketch),
        "{what}: the sketch on the boss is adrift"
    );
    assert_on(
        part.sketches()[sketch].plane,
        DVec3::Z,
        30.0,
        &format!("{what}: the sketch on the boss"),
    );
}

/// The boss put in stands lower than the chamfered one: with both tops in one
/// plane the kernel declines that chamfer however the part was built, a limit
/// of its own and not of the step's naming.
const THE_BOSS_PUT_IN: f64 = 25.0;

#[test]
fn a_step_put_in_before_a_chamfer_while_the_part_is_read_earlier_leaves_it_on_its_edge() {
    let mut part = a_block();
    let boss = a_sketch_on_xy(&mut part);
    rectangle(&mut part, boss, DVec2::new(5.0, 2.0), DVec2::new(10.0, 8.0));
    raise(
        &mut part,
        boss,
        DVec2::new(7.5, 5.0),
        30.0,
        ExtrusionMode::Add,
    );
    let edge = top_front(part.body(), 5.0, 30.0, 2.0);
    let chamfer = chamfered(&mut part, edge);

    part.insert_after(
        1,
        Operation::CreateSketch {
            plane: WorkPlane::XY,
            on: None,
        },
    );
    rectangle(&mut part, 2, DVec2::new(20.0, 2.0), DVec2::new(25.0, 8.0));
    let areas = part.areas_at(2, &[DVec2::new(22.5, 5.0)]);
    part.insert_after(
        2,
        Operation::Extrude {
            sketch: 2,
            areas,
            distance: THE_BOSS_PUT_IN.into(),
            mode: ExtrusionMode::Add,
        },
    );

    let what = "a lower boss put in before the chamfered one";
    assert_eq!(part.declined_because(chamfer), None, "{what}: the chamfer");
    assert_near(
        height_at(part.body(), 7.5, 2.1),
        30.0 - (SIZE - 0.1),
        &format!("{what}: the chamfered boss's front"),
    );
    assert_near(
        height_at(part.body(), 22.5, 2.1),
        THE_BOSS_PUT_IN,
        &format!("{what}: the boss put in, which nothing chamfers"),
    );
    let chamfered_boss_over_the_block = 5.0 * 6.0 * 10.0 - SIZE * SIZE * 5.0 / 2.0;
    let boss_put_in_over_the_block = 5.0 * 6.0 * (THE_BOSS_PUT_IN - 20.0);
    assert_near(
        part.body().volume(),
        30.0 * 10.0 * 20.0 + chamfered_boss_over_the_block + boss_put_in_over_the_block,
        what,
    );
}
