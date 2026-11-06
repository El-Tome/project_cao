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

#[test]
fn every_body_a_fillet_leaves_lists_what_it_is_made_of_as_its_geometry_stands() {
    let block = block([30.0, 10.0, 20.0]);
    let leaning = leaning_trapezoid();
    let cases = [
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
