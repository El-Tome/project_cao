//! Which corners a step on several edges takes (#801), read on [`admit`]
//! itself. A list it lets through by mistake is declined further on all the
//! same — a fill and a cut meeting at a corner by the fill's reading, two
//! fills by the overlap's — so a step on the whole body cannot tell which
//! reading declined it; here the corner's own answer is held.

use glam::{DVec2, DVec3};

use super::admit;
use crate::body::{Body, Matter};
use crate::brep::{self, Declined, EdgeId};
use crate::profile::{Contour, Frame, Profile};
use crate::sweep::Loop;

/// The numbers a raised part's floor and top answer to.
const TOP: u32 = 1;

/// An outline raised from `from` by `height` and joined to the matter so far,
/// as `cao_part` builds a step: the outline's faces numbered past the part's,
/// its floor first, then its top, then its walls, one for each run.
fn joined(part: &Body, outline: Contour, from: f64, height: f64) -> Body {
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
    let tool = part
        .tool_raised(&profile, frame, DVec3::Z * height)
        .expect("the kernel raises it");
    part.union(&tool).expect("the kernel joins it")
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
fn two_feet_of_a_standing_block_meeting_at_its_corner_are_declined() {
    let feet = [[TOP, STANDING_FRONT], [TOP, STANDING_RIGHT]];

    assert_eq!(admitted(&stacked(), &feet), Err(Declined::Unsupported));
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
