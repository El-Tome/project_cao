//! The bodies a fillet leaves (#697), held to the one rule of `soundness`
//! the public body cannot be asked about, since it needs the exact body
//! itself: what it lists itself as made of stands on its geometry. The rest of
//! the fillet's tests are `tests/a_fillet_rounds_one_straight_edge.rs`.

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

/// The leaning trapezoid of the fillet's tests: a front 40 long, a back 37
/// long at Y = 9, and a slant rising 3 for every 1 it comes back, meeting
/// the front at an acute corner and the back at an obtuse one; the front
/// two, the slant three, the back four.
fn leaning_trapezoid() -> Body {
    joined(
        &Body::default(),
        Contour::straight(vec![
            DVec2::ZERO,
            DVec2::new(40.0, 0.0),
            DVec2::new(37.0, 9.0),
            DVec2::new(0.0, 9.0),
        ]),
        5.0,
    )
}

/// The trapezoid of the chamfer's tests: a front 40 long, a back 34 long at
/// Y = 8, and a slant between them; the top one, the front two, the slant
/// three, the back four.
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

/// A regular hexagon of side 10 raised 5, its front the side along the
/// bottom: the top one, the front two.
fn hexagon() -> Body {
    let half = 5.0 * 3.0f64.sqrt();
    joined(
        &Body::default(),
        Contour::straight(vec![
            DVec2::new(-5.0, -half),
            DVec2::new(5.0, -half),
            DVec2::new(10.0, 0.0),
            DVec2::new(5.0, half),
            DVec2::new(-5.0, half),
            DVec2::new(-10.0, 0.0),
        ]),
        5.0,
    )
}

/// A slot 4 wide raised 20, its front 28 long, its round end of half a turn
/// about `(28 + off, 2)`: turning in from the front at a few degrees once
/// moved back.
fn slot(off: f64) -> Body {
    let (end, centre) = (DVec2::new(28.0, 0.0), DVec2::new(28.0 + off, 2.0));
    joined(
        &Body::default(),
        Contour {
            corners: vec![DVec2::ZERO, end, 2.0 * centre - end, DVec2::new(0.0, 4.0)],
            runs: vec![
                Run::Straight,
                Run::Round {
                    center: centre,
                    turn: PI,
                },
                Run::Straight,
                Run::Straight,
            ],
        },
        20.0,
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

/// The L of the fillet's tests: an arm 30 long and 6 deep, and at its left
/// end a stem 12 wide reaching back to 14, raised 10; the top one, the
/// stem's back six.
fn ell() -> Body {
    joined(
        &Body::default(),
        Contour::straight(vec![
            DVec2::ZERO,
            DVec2::new(30.0, 0.0),
            DVec2::new(30.0, 6.0),
            DVec2::new(12.0, 6.0),
            DVec2::new(12.0, 14.0),
            DVec2::new(0.0, 14.0),
        ]),
        10.0,
    )
}

#[test]
fn every_body_a_fillet_leaves_lists_what_it_is_made_of_as_its_geometry_stands() {
    let block = block([30.0, 10.0, 20.0]);
    let leaning = leaning_trapezoid();
    let (trapezoid, hexagon) = (trapezoid(), hexagon());
    let inwards = slot(-0.1);
    let abutting = two_blocks(15.0);
    let first_block_s = between(&two_blocks(20.0), &[1], &[2]);
    let ell = ell();
    let cases = [
        (
            "the block's top front edge by 50, the whole top taken",
            &block,
            between(&block, &[1], &[2]),
            50.0,
        ),
        (
            "the leaning trapezoid's acute edge by 50, the whole slant taken",
            &leaning,
            between(&leaning, &[2], &[3]),
            50.0,
        ),
        (
            "the edge a join bridged from two blocks' by 2",
            &abutting,
            first_block_s,
            2.0,
        ),
        (
            "the L's stem's back top edge by 2",
            &ell,
            between(&ell, &[1], &[6]),
            2.0,
        ),
        (
            "the block's top front edge by 3",
            &block,
            between(&block, &[1], &[2]),
            3.0,
        ),
        (
            "the leaning trapezoid's acute edge by 2",
            &leaning,
            between(&leaning, &[2], &[3]),
            2.0,
        ),
        (
            "the leaning trapezoid's obtuse edge by 2",
            &leaning,
            between(&leaning, &[3], &[4]),
            2.0,
        ),
        (
            "the hexagon's front top edge by 1, running on past both its ends",
            &hexagon,
            between(&hexagon, &[1], &[2]),
            1.0,
        ),
        (
            "the trapezoid's slanted top edge by 1, trimmed by the front and the back",
            &trapezoid,
            between(&trapezoid, &[1], &[3]),
            1.0,
        ),
        (
            "the slot's front, its round turning in at a few degrees, by 1",
            &inwards,
            between(&inwards, &[1], &[2]),
            1.0,
        ),
    ];

    for (what, part, name, radius) in cases {
        let filleted = part
            .fillet(&name, radius)
            .unwrap_or_else(|declined| panic!("{what}: declined, {declined:?}"));

        let body = filleted.body.brep().expect("a fillet leaves an exact body");
        assert_eq!(
            listed(&body.listing(), body.scale().reach()),
            Ok(()),
            "{what}"
        );
    }
}
