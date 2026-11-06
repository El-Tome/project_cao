//! The net a fillet's result is held to before it is handed back (#697): its
//! round a face of its own. The floor on how far a round stands off its faces
//! declines every case the net was written for before the cut is made, so no
//! fillet reaches it failing; it is reached here directly, on bodies each
//! built to fail one of its conditions while keeping the others.

use std::f64::consts::TAU;

use glam::{DVec2, DVec3};

use crate::body::exact::Exact;
use crate::body::{Body, Matter};
use crate::brep::{EdgeName, Lost};
use crate::profile::{Contour, Frame, Profile, Run};
use crate::sweep::Loop;

const GROUND: Frame = Frame {
    origin: DVec3::ZERO,
    u: DVec3::X,
    v: DVec3::Y,
};

/// The number where a block's one step ends, which the step after it gives
/// the face it makes.
const MADE: u32 = 6;

/// An outline on the ground raised by `height` and joined to the matter so
/// far, as `cao_part` builds a step: the outline's faces numbered past the
/// part's.
fn joined(part: &Body, outline: Contour, height: f64) -> Body {
    let profile = Profile {
        exact: Some((outline, Vec::new())),
        sampled: Loop::straight(&[]),
        sampled_holes: Vec::new(),
        triangles: &[],
    };
    let tool = part
        .tool_raised(&profile, GROUND, DVec3::Z * height)
        .expect("the kernel raises it");
    part.union(&tool).expect("the kernel joins it")
}

/// A block 30 by 10 by 20: its top one, its front two.
fn block() -> Body {
    joined(
        &Body::default(),
        Contour::rectangle(DVec2::ZERO, DVec2::new(30.0, 10.0)),
        20.0,
    )
}

/// The name of the block's top front edge.
fn top_front(body: &Body) -> EdgeName {
    let sides = [vec![1], vec![2]];
    let named: Vec<EdgeName> = body
        .edge_names()
        .into_iter()
        .filter(|name| name.between == sides)
        .collect();
    assert_eq!(named.len(), 1, "one top front edge: {named:?}");
    named[0].clone()
}

fn exact(body: &Body) -> &Exact {
    match &body.matter {
        Matter::Exact(exact) => exact,
        Matter::Flats(_) => panic!("the exact kernel computed the body"),
    }
}

/// Whether some faces answer to `number`, each to it alone, and curved: every
/// condition of the net but the edge's.
fn curved_alone(body: &Exact, number: u32) -> bool {
    let mut faces = body.named(number as usize).peekable();
    faces.peek().is_some()
        && faces.all(|face| body.brep.numbers(face) == [number] && !body.is_flat(number as usize))
}

#[test]
fn a_fillet_s_result_holds_its_round_as_a_face_of_its_own() {
    let block = block();
    let name = top_front(&block);
    let filleted = block.fillet(&name, 3.0).expect("the fillet is laid");

    let alone = exact(&filleted.body).rounded_alone(&name, MADE);

    assert!(
        alone,
        "the round answers to 6 alone, curved, and the edge is gone"
    );
}

#[test]
fn a_result_whose_face_under_the_number_is_flat_is_no_round_of_its_own() {
    let block = block();
    let name = top_front(&block);
    let chamfered = block.chamfer(&name, 3.0).expect("the chamfer is cut");
    let body = exact(&chamfered.body);
    assert_eq!(
        body.lost(&name),
        Some(Lost::Gone),
        "the chamfer took the edge"
    );
    let faces: Vec<_> = body.named(MADE as usize).collect();
    assert!(
        faces.len() == 1 && body.brep.numbers(faces[0]) == [MADE],
        "the chamfer is one face answering to 6 alone"
    );

    let alone = body.rounded_alone(&name, MADE);

    assert!(!alone, "the face answering to 6 is the chamfer's, flat");
}

#[test]
fn a_result_where_the_edge_s_name_still_finds_it_is_no_round_of_its_own() {
    let block = block();
    let name = top_front(&block);
    let centre = DVec2::new(50.0, 5.0);
    let disc = Contour {
        corners: vec![centre + DVec2::X * 4.0],
        runs: vec![Run::Round {
            center: centre,
            turn: TAU,
        }],
    };
    let beside = joined(&block, disc, 20.0);
    let body = exact(&beside);
    let wall = 8;
    assert!(
        curved_alone(body, wall),
        "a disc apart from the block, drawn as one turn: its wall one curved face answering to 8 \
         alone"
    );
    assert_eq!(body.lost(&name), None, "nothing touched the block's edge");

    let alone = body.rounded_alone(&name, wall);

    assert!(!alone, "the edge the fillet was asked on is still there");
}

#[test]
fn a_result_with_no_face_under_the_number_is_no_round_of_its_own() {
    let block = block();
    let name = top_front(&block);
    let filleted = block.fillet(&name, 3.0).expect("the fillet is laid");
    let body = exact(&filleted.body);
    let past = MADE + 1;
    assert!(
        body.named(past as usize).next().is_none() && body.lost(&name).is_some(),
        "no face answers to 7, and the edge is gone"
    );

    let alone = body.rounded_alone(&name, past);

    assert!(!alone, "no face answers to 7: there is no round");
}

#[test]
fn a_result_whose_curved_face_answers_to_another_number_too_is_no_round_of_its_own() {
    let block = block();
    let name = top_front(&block);
    let disc = joined(&Body::default(), Contour::circle(DVec2::ZERO, 10.0), 5.0);
    let body = exact(&disc);
    assert!(
        body.named(2).count() == 1
            && body.named(2).all(|face| body.brep.numbers(face) == [2, 3])
            && !disc.is_flat(2)
            && body.lost(&name).is_some(),
        "the disc's wall is one curved face answering to both its halves, two and three, and \
         the block's edge is nowhere on it"
    );

    let alone = body.rounded_alone(&name, 2);

    assert!(!alone, "the curved face answering to 2 answers to 3 too");
}
