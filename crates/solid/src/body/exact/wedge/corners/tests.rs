//! Which corners a step on several edges takes (#801), read on [`admit`]
//! itself: two edges of one sense beside a third of the other, the inner
//! corner of an L or the corner two feet of a standing block meet at; two
//! concave edges beside a third concave one, at a pocket's bottom corner; the
//! three edges at a corner of mixed sense — a standing block's foot corner,
//! two feet and the upright above them, and the top of an L's inner corner,
//! two top edges and the upright below them (#829); and no other corner but
//! a convex one. It is the first guard a step's corners meet: a list it lets
//! through by mistake — a cut and a fill meeting at a corner — is declined
//! further on all the same, so a step on the whole body cannot tell which
//! reading declined it. Here the corner's own answer is held.

use glam::{DVec2, DVec3};

use super::admit;
use crate::body::{Body, Matter};
use crate::brep::{self, Declined, EdgeId};
use crate::profile::{Contour, Frame, Profile};
use crate::sweep::Loop;

/// The number a raised part's top answers to.
const TOP: u32 = 1;

/// An outline raised from `from` by `height`, as `cao_part` raises a step:
/// its faces numbered past the part's, its floor first, then its top, then
/// its walls, one for each run.
fn raised(part: &Body, outline: Contour, from: f64, height: f64) -> Body {
    let profile = Profile {
        exact: Some((outline, Vec::new())),
        sampled: Loop::straight(&[]),
        sampled_holes: Vec::new(),
        triangles: &[],
    };
    let frame = Frame {
        origin: DVec3::Z * from,
        u: DVec3::X,
        v: DVec3::Y,
    };
    part.tool_raised(&profile, frame, DVec3::Z * height)
        .expect("the kernel raises it")
}

/// The part joined to an outline raised from `from` by `height`.
fn joined(part: &Body, outline: Contour, from: f64, height: f64) -> Body {
    part.union(&raised(part, outline, from, height))
        .expect("the kernel joins it")
}

/// The part less an outline raised from `from` by `height`.
fn cut(part: &Body, outline: Contour, from: f64, height: f64) -> Body {
    part.difference(&raised(part, outline, from, height))
        .expect("the kernel cuts it")
}

/// A block 30 by 20 by 10: its top one, its front two, its right three.
fn block() -> Body {
    joined(
        &Body::default(),
        Contour::rectangle(DVec2::ZERO, DVec2::new(30.0, 20.0)),
        0.0,
        10.0,
    )
}

/// The block with another, 10 by 10 by 10, standing on the middle of its
/// top: the upper block's walls, front, right, back and left, eight to
/// eleven. Its feet meet inwards, its uprights outwards.
fn stacked() -> Body {
    joined(
        &block(),
        Contour::rectangle(DVec2::new(10.0, 5.0), DVec2::new(20.0, 15.0)),
        10.0,
        10.0,
    )
}
const STANDING_FRONT: u32 = 8;
const STANDING_RIGHT: u32 = 9;

/// The block with a pocket 10 by 10 sunk 5 into the middle of its top: the
/// pocket's floor six, its walls, front, right, back and left, eight to
/// eleven. Its floor's edges and its uprights all meet inwards.
fn pocketed() -> Body {
    cut(
        &block(),
        Contour::rectangle(DVec2::new(10.0, 5.0), DVec2::new(20.0, 15.0)),
        5.0,
        6.0,
    )
}
const POCKET_FLOOR: u32 = 6;
const POCKET_FRONT: u32 = 8;
const POCKET_RIGHT: u32 = 9;

/// An L: an arm 30 by 6 along the front, a stem 12 wide reaching back to 14,
/// raised 10. Its arm's back, four, and its stem's right side, five, meet
/// inwards at its inner corner, (12, 6).
fn ell() -> Body {
    joined(
        &Body::default(),
        Contour::straight(vec![
            DVec2::new(0.0, 0.0),
            DVec2::new(30.0, 0.0),
            DVec2::new(30.0, 6.0),
            DVec2::new(12.0, 6.0),
            DVec2::new(12.0, 14.0),
            DVec2::new(0.0, 14.0),
        ]),
        0.0,
        10.0,
    )
}
const ARM_BACK: u32 = 4;
const STEM_RIGHT: u32 = 5;

/// The exact body a part is computed as.
fn brep_of(part: &Body) -> &brep::Body {
    match &part.matter {
        Matter::Exact(exact) => &exact.brep,
        Matter::Flats(_) => panic!("the exact kernel computed the part"),
    }
}

/// The edge between the faces numbered `first` and `second`, as the body
/// finds it by its name.
fn edge(body: &brep::Body, [first, second]: [u32; 2]) -> EdgeId {
    let sides = [vec![first], vec![second]];
    let named: Vec<_> = body
        .edge_names()
        .into_iter()
        .filter(|name| name.between == sides || name.between == [vec![second], vec![first]])
        .collect();
    assert_eq!(named.len(), 1, "one edge between {first} and {second}");
    body.find(&named[0]).expect("the body finds its own edge")
}

/// What [`admit`] answers on the part's edges between the pairs of faces
/// `pairs`, in that order: each corner taken beside a concave edge, where it
/// stands, with the two edges' ranks.
fn admitted(part: &Body, pairs: &[[u32; 2]]) -> Result<Vec<(DVec3, [usize; 2])>, Declined> {
    let body = brep_of(part);
    let edges: Vec<EdgeId> = pairs.iter().map(|&pair| edge(body, pair)).collect();
    admit(body, &edges).map(|beside| {
        beside
            .into_iter()
            .map(|(corner, ranks)| (body.vertex(corner).point, ranks))
            .collect()
    })
}

#[test]
fn two_feet_of_a_standing_block_are_taken_beside_its_upright_by_their_ranks() {
    let feet = [[TOP, STANDING_FRONT], [TOP, STANDING_RIGHT]];

    let beside = admitted(&stacked(), &feet).expect("the corner is taken");

    assert_eq!(
        beside.len(),
        1,
        "one corner beside a convex edge: {beside:?}"
    );
    let (at, ranks) = beside[0];
    assert!(
        at.distance(DVec3::new(20.0, 5.0, 10.0)) < 1e-9,
        "the corner at the foot of the front right upright, not {at}"
    );
    assert_eq!(
        ranks,
        [0, 1],
        "the two edges' ranks in the list, the lower first"
    );
}

#[test]
fn two_edges_of_a_pocket_s_floor_are_taken_at_its_corner_by_their_ranks() {
    let floor = [[POCKET_FLOOR, POCKET_FRONT], [POCKET_FLOOR, POCKET_RIGHT]];

    let beside = admitted(&pocketed(), &floor).expect("the corner is taken");

    assert_eq!(
        beside.len(),
        1,
        "one corner, all three of its edges concave: {beside:?}"
    );
    let (at, ranks) = beside[0];
    assert!(
        at.distance(DVec3::new(20.0, 5.0, 5.0)) < 1e-9,
        "the pocket's front right bottom corner, not {at}"
    );
    assert_eq!(
        ranks,
        [0, 1],
        "the two edges' ranks in the list, the lower first"
    );
}

#[test]
fn a_foot_and_the_upright_above_it_are_declined_whichever_the_list_names_first() {
    let foot = [TOP, STANDING_FRONT];
    let upright = [STANDING_FRONT, STANDING_RIGHT];

    for list in [[foot, upright], [upright, foot]] {
        assert_eq!(
            admitted(&stacked(), &list),
            Err(Declined::Unsupported),
            "{list:?}"
        );
    }
}

#[test]
fn an_l_s_top_edge_and_its_inner_upright_are_declined() {
    let list = [[TOP, ARM_BACK], [ARM_BACK, STEM_RIGHT]];

    assert_eq!(admitted(&ell(), &list), Err(Declined::Unsupported));
}

#[test]
fn the_l_s_two_top_edges_at_its_inner_corner_are_taken_beside_its_inner_upright_by_their_ranks() {
    let list = [[TOP, ARM_BACK], [TOP, STEM_RIGHT]];

    let beside = admitted(&ell(), &list).expect("the corner is taken");

    assert_eq!(
        beside.len(),
        1,
        "one corner beside a concave edge: {beside:?}"
    );
    let (at, ranks) = beside[0];
    assert!(
        at.distance(DVec3::new(12.0, 6.0, 10.0)) < 1e-9,
        "the corner above the inner upright, not {at}"
    );
    assert_eq!(
        ranks,
        [0, 1],
        "the two edges' ranks in the list, the lower first"
    );
}

#[test]
fn two_edges_of_a_block_s_convex_corner_are_taken_with_no_corner_beside_a_concave_edge() {
    let front = 2;
    let right = 3;
    let list = [[TOP, front], [TOP, right]];

    assert_eq!(admitted(&block(), &list), Ok(Vec::new()));
}

#[test]
fn the_three_edges_at_the_top_of_an_l_s_inner_corner_are_taken_with_no_corner_beside_an_edge_whatever_the_order()
 {
    let corner = [[TOP, ARM_BACK], [TOP, STEM_RIGHT], [ARM_BACK, STEM_RIGHT]];

    for [first, second, third] in [
        [0, 1, 2],
        [0, 2, 1],
        [1, 0, 2],
        [1, 2, 0],
        [2, 0, 1],
        [2, 1, 0],
    ] {
        let list = [corner[first], corner[second], corner[third]];
        assert_eq!(admitted(&ell(), &list), Ok(Vec::new()), "{list:?}");
    }
}

#[test]
fn two_feet_of_a_standing_block_and_the_upright_above_them_are_taken_with_no_corner_beside_an_edge_whatever_the_order()
 {
    let corner = [
        [TOP, STANDING_FRONT],
        [TOP, STANDING_RIGHT],
        [STANDING_FRONT, STANDING_RIGHT],
    ];

    for [first, second, third] in [
        [0, 1, 2],
        [0, 2, 1],
        [1, 0, 2],
        [1, 2, 0],
        [2, 0, 1],
        [2, 1, 0],
    ] {
        let list = [corner[first], corner[second], corner[third]];
        assert_eq!(admitted(&stacked(), &list), Ok(Vec::new()), "{list:?}");
    }
}

#[test]
fn the_three_edges_at_a_pocket_s_bottom_corner_are_taken_with_no_corner_beside_an_edge() {
    let corner = [
        [POCKET_FLOOR, POCKET_FRONT],
        [POCKET_FLOOR, POCKET_RIGHT],
        [POCKET_FRONT, POCKET_RIGHT],
    ];

    assert_eq!(admitted(&pocketed(), &corner), Ok(Vec::new()));
}
