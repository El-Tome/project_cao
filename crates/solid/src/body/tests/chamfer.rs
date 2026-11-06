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

/// A slot 4 wide raised 20, its front 28 long, its round end of half a turn
/// about `(28 + off, 2)`: meeting the front at a few degrees once moved off.
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

/// A knife: the triangle (0, 0), (100, 0), (100, 100·tan α) raised 10, its
/// front two, its right three and its long side four, which meet its front
/// at the upright edge through the origin at `degrees`.
fn knife(degrees: f64) -> Body {
    let rise = 100.0 * degrees.to_radians().tan();
    joined(
        &Body::default(),
        Contour::straight(vec![
            DVec2::ZERO,
            DVec2::new(100.0, 0.0),
            DVec2::new(100.0, rise),
        ]),
        10.0,
    )
}

/// A block 100 by 20 raised 10 whose front bends up by `degrees` halfway
/// along: its front two up to (50, 0) and three past it, which meet at the
/// upright edge there at 180 degrees less `degrees`.
fn nearly_flat(degrees: f64) -> Body {
    let rise = 50.0 * degrees.to_radians().tan();
    joined(
        &Body::default(),
        Contour::straight(vec![
            DVec2::ZERO,
            DVec2::new(50.0, 0.0),
            DVec2::new(100.0, rise),
            DVec2::new(100.0, 20.0),
            DVec2::new(0.0, 20.0),
        ]),
        10.0,
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
    let (inwards, outwards, hexagon) = (slot(-0.1), slot(0.1), hexagon());
    let (knife_of_5, knife_of_1) = (knife(5.0), knife(1.0));
    let (bent_by_1, bent_by_a_tenth) = (nearly_flat(1.0), nearly_flat(0.1));
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
        (
            "the slot's front, its round turning in at a few degrees, by 1",
            &inwards,
            between(&inwards, &[1], &[2]),
            1.0,
        ),
        (
            "the slot's front, its round turning out at a few degrees, by 1",
            &outwards,
            between(&outwards, &[1], &[2]),
            1.0,
        ),
        (
            "the trapezoid's slanted top edge by 1, trimmed by the front and the back",
            &trapezoid,
            between(&trapezoid, &[1], &[3]),
            1.0,
        ),
        (
            "the hexagon's front top edge by 1, running on past both its ends",
            &hexagon,
            between(&hexagon, &[1], &[2]),
            1.0,
        ),
        (
            "a knife of 5 degrees by 50",
            &knife_of_5,
            between(&knife_of_5, &[2], &[4]),
            50.0,
        ),
        (
            "a knife of 1 degree by 50",
            &knife_of_1,
            between(&knife_of_1, &[2], &[4]),
            50.0,
        ),
        (
            "a front bent by 1 degree, by 10",
            &bent_by_1,
            between(&bent_by_1, &[2], &[3]),
            10.0,
        ),
        (
            "a front bent by a tenth of a degree, by 10",
            &bent_by_a_tenth,
            between(&bent_by_a_tenth, &[2], &[3]),
            10.0,
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

/// A block 16 wide and 25 high whose back is an arc of radius 10 about
/// `(8, 12)`, from `(16, 18)` over its crest at `(8, 22)` to `(0, 18)`: the
/// top one, the front two.
fn arc_backed() -> Body {
    joined(
        &Body::default(),
        Contour {
            corners: vec![
                DVec2::ZERO,
                DVec2::new(16.0, 0.0),
                DVec2::new(16.0, 18.0),
                DVec2::new(0.0, 18.0),
            ],
            runs: vec![
                Run::Straight,
                Run::Straight,
                Run::Round {
                    center: DVec2::new(8.0, 12.0),
                    turn: PI - 2.0 * 0.75f64.atan(),
                },
                Run::Straight,
            ],
        },
        25.0,
    )
}

/// How far the kernel reads a body as reaching, which every tolerance of a
/// later step on it is taken against.
fn reach_of(body: &Body) -> f64 {
    body.brep().expect("an exact body").scale().reach()
}

#[test]
fn a_chamfer_leaves_the_part_s_reach_as_it_was_within_a_quarter_of_its_distance() {
    let arc_backed = arc_backed();
    let arc_front = between(&arc_backed, &[1], &[2]);
    let block = block([30.0, 10.0, 20.0]);
    let block_front = between(&block, &[1], &[2]);
    let cases = [
        ("the arc-backed part by 1", &arc_backed, &arc_front, 1.0),
        ("the arc-backed part by 5", &arc_backed, &arc_front, 5.0),
        (
            "the arc-backed part by 18, all its top gives",
            &arc_backed,
            &arc_front,
            18.0,
        ),
        ("the block by 3", &block, &block_front, 3.0),
        (
            "the block asked 50, taking the 10 its top gives",
            &block,
            &block_front,
            50.0,
        ),
    ];

    for (what, part, name, distance) in cases {
        let chamfered = part
            .chamfer(name, distance)
            .unwrap_or_else(|declined| panic!("{what}: declined, {declined:?}"));

        let (before, after) = (reach_of(part), reach_of(&chamfered.body));
        assert!(
            after <= before + chamfered.taken / 4.0,
            "{what}: the part reached {before} and reaches {after} after a chamfer of {}: \
             every later step would read it coarser",
            chamfered.taken
        );
    }
}

#[test]
fn a_chamfer_on_a_knife_or_a_nearly_flat_edge_leaves_the_part_s_reach_within_its_distance() {
    let sine = |degrees: f64| degrees.to_radians().sin();
    let knife_whole = |degrees: f64| 50000.0 * degrees.to_radians().tan();
    let bent_whole = |degrees: f64| (2000.0 - 1250.0 * degrees.to_radians().tan()) * 10.0;
    let cases = [
        (
            "a knife of 5 degrees by 50",
            knife(5.0),
            [2, 4],
            50.0,
            knife_whole(5.0),
            sine(5.0),
        ),
        (
            "a knife of 1 degree by 50",
            knife(1.0),
            [2, 4],
            50.0,
            knife_whole(1.0),
            sine(1.0),
        ),
        (
            "a front bent by 1 degree, by 10",
            nearly_flat(1.0),
            [2, 3],
            10.0,
            bent_whole(1.0),
            sine(1.0),
        ),
        (
            "a front bent by a tenth of a degree, by 10",
            nearly_flat(0.1),
            [2, 3],
            10.0,
            bent_whole(0.1),
            sine(0.1),
        ),
    ];

    for (what, part, [first, second], distance, whole, sine) in cases {
        let name = between(&part, &[first], &[second]);

        let chamfered = part
            .chamfer(&name, distance)
            .unwrap_or_else(|declined| panic!("{what}: declined, {declined:?}"));

        let (before, after) = (reach_of(&part), reach_of(&chamfered.body));
        assert!(
            after <= before + chamfered.taken,
            "{what}: the part reached {before} and reaches {after} after a chamfer of {}: \
             the cut stood further outside the part than the distance it took",
            chamfered.taken
        );
        assert!(
            (chamfered.taken - distance).abs() < 1e-9,
            "{what}: took {}",
            chamfered.taken
        );
        let promised = whole - 5.0 * distance * distance * sine;
        let held = chamfered.body.volume();
        assert!(
            (held - promised).abs() <= 1e-9 * promised,
            "{what}: the body holds {held}, where arithmetic promised {promised}"
        );
        let triangles = chamfered.body.triangles();
        assert_eq!(crate::soundness::closed(&triangles), Ok(()), "{what}");
        assert_eq!(crate::soundness::uncrossed(&triangles), Ok(()), "{what}");
    }
}
