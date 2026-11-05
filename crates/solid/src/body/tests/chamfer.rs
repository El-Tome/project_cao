//! The bodies a chamfer leaves (#696), held to the one rule of `soundness`
//! the public body cannot be asked about, since it needs the exact body
//! itself: what it lists itself as made of stands on its geometry. The rest of
//! the chamfer's tests are `tests/a_chamfer_cuts_one_straight_edge.rs`.

use super::*;
use crate::EdgeName;
use crate::soundness::listed;

/// An outline on the ground raised by `height` and joined to the matter so
/// far, as `cao_part` builds a step: the outline's faces numbered past the
/// part's.
fn joined(part: &Body, outline: Contour, height: f64) -> Body {
    let tool = raised(part, &exact(outline, vec![]), height);
    part.union(&tool).expect("the kernel joins it")
}

/// The name of the one edge between the faces numbered `first` and `second`.
fn between(body: &Body, first: &[u32], second: &[u32]) -> EdgeName {
    let sides = [first.to_vec(), second.to_vec()];
    let named: Vec<EdgeName> = body
        .edge_names()
        .into_iter()
        .filter(|name| name.between == sides)
        .collect();
    assert_eq!(named.len(), 1, "one edge between {sides:?}: {named:?}");
    named[0].clone()
}

fn block([length, breadth, height]: [f64; 3]) -> Body {
    joined(
        &Body::default(),
        Contour::rectangle(DVec2::ZERO, DVec2::new(length, breadth)),
        height,
    )
}

/// The trapezoid of the chamfer's tests: a front 40 long, a back 34 long at
/// Y = 8, and a slant between them meeting the front at an acute corner and
/// the back at an obtuse one; the front two, the slant three, the back four.
fn trapezoid() -> Body {
    joined(
        &Body::default(),
        Contour::straight(vec![
            DVec2::ZERO,
            DVec2::new(40.0, 0.0),
            DVec2::new(34.0, 8.0),
            DVec2::new(0.0, 8.0),
        ]),
        5.0,
    )
}

/// Two blocks 15 by 10 by 10 side by side, the second `from` along: apart,
/// or abutting so that their tops and their fronts merge.
fn two_blocks(from: f64) -> Body {
    joined(
        &block([15.0, 10.0, 10.0]),
        Contour::rectangle(DVec2::new(from, 0.0), DVec2::new(from + 15.0, 10.0)),
        10.0,
    )
}

/// A D standing on its flat: a half disc of radius 5 raised by 20, its
/// straight front from −5 to 5 and its arc behind; the top one, the front
/// two.
fn d_shape() -> Body {
    joined(
        &Body::default(),
        Contour {
            corners: vec![DVec2::new(-5.0, 0.0), DVec2::new(5.0, 0.0)],
            runs: vec![
                Run::Straight,
                Run::Round {
                    center: DVec2::ZERO,
                    turn: PI,
                },
            ],
        },
        20.0,
    )
}

#[test]
fn every_body_a_chamfer_leaves_lists_what_it_is_made_of_as_its_geometry_stands() {
    let block = block([30.0, 10.0, 20.0]);
    let top_front = between(&block, &[1], &[2]);
    let trapezoid = trapezoid();
    let abutting = two_blocks(15.0);
    let first_block_s = between(&two_blocks(20.0), &[1], &[2]);
    let d = d_shape();
    let flat = between(&d, &[1], &[2]);
    let cases = [
        (
            "the block's top front edge by 3",
            &block,
            top_front.clone(),
            3.0,
        ),
        (
            "the block's top front edge by 50, the whole top taken",
            &block,
            top_front,
            50.0,
        ),
        (
            "the trapezoid's acute edge by 2",
            &trapezoid,
            between(&trapezoid, &[2], &[3]),
            2.0,
        ),
        (
            "the trapezoid's obtuse edge by 2",
            &trapezoid,
            between(&trapezoid, &[3], &[4]),
            2.0,
        ),
        (
            "the trapezoid's acute edge by 50, the whole slant taken",
            &trapezoid,
            between(&trapezoid, &[2], &[3]),
            50.0,
        ),
        (
            "the edge a join bridged from two blocks' by 2",
            &abutting,
            first_block_s,
            2.0,
        ),
        ("the D's flat top edge by 3", &d, flat.clone(), 3.0),
        (
            "the D's flat top edge by 50, the whole top taken",
            &d,
            flat,
            50.0,
        ),
    ];

    for (what, part, name, distance) in cases {
        let chamfered = part
            .chamfer(&name, distance)
            .unwrap_or_else(|declined| panic!("{what}: declined, {declined:?}"));

        let body = chamfered
            .body
            .brep()
            .expect("a chamfer leaves an exact body");
        assert_eq!(
            listed(&body.listing(), body.scale().reach()),
            Ok(()),
            "{what}"
        );
    }
}
