//! What the campaigns of `random_exact_solids.rs` found wrong in the exact
//! kernel of #498 — its boolean, not its triangles — each failure shrunk and
//! kept as the test it was fixed against, or, when it is understood and not
//! fixed, ignored with its diagnosis.

// The drawing, the promise and the checks are shared with the campaigns;
// this file uses its own part of them.
#[allow(dead_code, unused_imports)]
mod random_solids;

use random_solids::{Case, Leaf, Outline, Plane, Step};

/// A bar along X whose side touches, from inside, the wall of a post along
/// Z: the curve they meet along is a figure of eight through the point of
/// touch, which it passes twice. It was cut there on one pass only, and the
/// arc through the other crossed its neighbours at no vertex.
#[test]
fn seed_197_a_bar_touching_a_post_s_wall_from_inside_is_joined_to_it() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(Plane::yz(7.0), Outline::circle([2.0, 4.0], 2.5), 7.0),
        vec![Step::add(Leaf::prism(
            Plane::xy(5.0),
            Outline::circle([10.0, 4.0], 4.5),
            -7.0,
        ))],
    ));
}

/// A block bored across by a pin, then given a post whose wall the pin's
/// side touches but for a hair under the tolerance. The pair of cylinders
/// decided the touch and moved the pin onto it in the curve they meet along,
/// which the pin's own face could not see, the curve's cylinder being no
/// longer bit for bit its own: the kernel declined. It now answers.
///
/// What is left: the corner where the top, through the pin's axis, meets
/// the pin and the post is found on the pin as it stands, and the curve runs
/// on the pin as it was moved. Where they cross at a slant the curve passes
/// the corner a little more than the move away — 1.04 times the tolerance
/// here — and the listing sees an edge's end off its vertex. Putting the
/// corner on the moved curve would hold this case and move the fault to the
/// pin's other edges; the snap itself is what would have to change.
#[test]
#[ignore = "snapped"]
fn seed_290_a_pin_touching_a_post_by_a_hair_is_seen_on_the_pin() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(
            Plane::xy(2.0),
            Outline::rectangle([2.0, 1.0], [5.0, 4.0]),
            -4.0,
        ),
        vec![
            Step::cut(Leaf::prism(
                Plane::xz(2.00001),
                Outline::circle([3.49999999, 2.0], 1.0),
                -4.0,
            )),
            Step::add(Leaf::prism(
                Plane::xy(1.0),
                Outline::circle([7.0, 7.0], 4.5),
                1.0,
            )),
        ],
    ));
}

/// A first cut leaves a wall a tenth of a micron thick, its two faces apart
/// at the tolerance of a reach of 95; a second cut reaching 198 has twice
/// that tolerance. It took the edges of both faces for one line and their
/// corners for one corner, and a face got the corners of both planes:
/// decided once, the two planes stay apart whatever a later operation's
/// tolerance.
#[test]
fn seed_243_a_wall_thinner_than_a_later_tolerance_keeps_its_two_faces() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(
            Plane::xy(13.0),
            Outline::rectangle([22.5, 42.5], [37.5, 47.5]),
            -48.0,
        ),
        vec![
            Step::cut(Leaf::prism(
                Plane::xy(8.0),
                Outline::rectangle([22.5000001, 42.5], [47.5, 47.5]),
                -95.0,
            )),
            Step::cut(Leaf::prism(
                Plane::xy(8.0),
                Outline::rectangle([23.0, 43.0], [58.0, 48.0]),
                190.0,
            )),
        ],
    ));
}

/// A block bored twice by circles of one radius whose centres stand a tenth
/// of a micron apart, more than the tolerance: the line where the top meets
/// the first bore ran within the tolerance of the second, was taken to lie on
/// it though it is not the line the top meets the second along, and a
/// corner of each was put on both.
#[test]
fn seed_460_two_bores_a_hair_more_than_the_tolerance_apart_are_two_walls() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(
            Plane::xz(15.0),
            Outline::rectangle([40.0, 25.0], [75.0, 40.0]),
            28.0,
        ),
        vec![
            Step::cut(Leaf::prism(
                Plane::xz(20.0),
                Outline::circle([74.9999999, 32.5], 15.0),
                27.0,
            )),
            Step::cut(Leaf::prism(
                Plane::xz(14.99999995),
                Outline::circle([75.0, 32.5], 15.0),
                55.0,
            )),
        ],
    ));
}

/// A block cut by a circle tangent to two of its sides, its centre a hundred
/// thousandth inside the third. At each tangent side a sliver of the bottom
/// is left between the side and the circle, from the tangent point to the
/// corner the circle passes within the tolerance of: a face sweeping less
/// area than rounding, turned the right way and read backwards by the
/// listing's check, which now leaves unread a loop too thin to be read.
///
/// What is left: over that stretch the side and the wall stand closer than
/// the tolerance, and the corner lies on both, so the kernel keeps a skin of
/// matter thinner than it can tell between two faces, back to back — their
/// triangles lie on each other. The arc of the circle and the side's edge
/// run between the same two corners within the tolerance of each other; the
/// kernel would have to take them for one edge, which no decision of the
/// design does yet.
#[test]
#[ignore = "skin"]
fn seed_2790_a_block_cut_by_a_circle_centred_a_hair_inside_its_side() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(
            Plane::xy(5.0),
            Outline::rectangle([5.0, 4.0], [12.0, 7.0]),
            7.0,
        ),
        vec![Step::cut(Leaf::prism(
            Plane::xy(4.99999),
            Outline::circle([11.99999, 5.5], 1.5),
            14.0,
        ))],
    ));
}

/// A block given a boss tangent to two of its sides, its centre a hundred
/// thousandth inside the third: the slivers between each tangent side and
/// the circle, turned the right way, were read backwards by rounding.
#[test]
fn seed_4901_a_block_given_a_boss_centred_a_hair_inside_its_side() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(
            Plane::xy(6.0),
            Outline::rectangle([1.0, 5.0], [6.5, 11.0]),
            2.0,
        ),
        vec![Step::add(Leaf::prism(
            Plane::xy(5.9999999),
            Outline::circle([6.49999, 8.0], 3.0),
            4.0,
        ))],
    ));
}

/// The same on the YZ plane, some six times larger.
#[test]
fn seed_10495_a_block_given_a_boss_centred_a_hair_inside_its_side() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(
            Plane::yz(35.0),
            Outline::rectangle([25.0, 0.0], [32.5, 30.0]),
            35.0,
        ),
        vec![Step::add(Leaf::prism(
            Plane::yz(30.0),
            Outline::circle([32.49999, 15.0], 15.0),
            35.0,
        ))],
    ));
}

/// Two blocks whose side and bottom stand three tenths of a micron from each
/// other's, within the tolerance each way: each plane is one, but the corner
/// of each where the side meets the bottom stood more than the tolerance
/// from the other's, and the edges there too. Three planes whose normals
/// span space fix one corner, and two across each other one line, however
/// far apart they were found.
#[test]
fn seed_5001541_two_blocks_a_hair_apart_each_way_share_their_corners() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(
            Plane::xy(180.0),
            Outline::rectangle([105.0, 240.0], [315.0, 420.0]),
            -300.0,
        ),
        vec![Step::add(Leaf::prism(
            Plane::xy(180.0),
            Outline::rectangle([104.9999997, 240.0], [375.0, 420.0]),
            -299.9999997,
        ))],
    ));
}

/// A post whose wall a block's side is tangent to, the block's corner three
/// tenths of a micron short of the touch, within the tolerance. Read at the
/// corner, the circle where the block's bottom cuts the post still rose
/// towards the side and was ordered above it, though it runs below it: the
/// directions leaving a corner are known only to the tolerance it was
/// merged within, over the arc's lever.
#[test]
fn seed_5001310_a_block_whose_corner_stands_a_hair_short_of_touching_a_post() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(
            Plane::xy(-60.0),
            Outline::circle([239.9999997, 165.0], 60.0),
            390.0,
        ),
        vec![Step::add(Leaf::prism(
            Plane::xy(150.0),
            Outline::rectangle([90.0, 180.0], [240.0, 225.0]),
            45.0,
        ))],
    ));
}

/// A block cut by a circle tangent to two of its sides, its centre a hundred
/// thousandth inside the third: the sliver left between a side and the
/// circle was read about the plane's origin, eleven away, where the offset
/// of its corner from the circle, within the tolerance, swept a million
/// times its area and turned it backwards; it is now read about a place of
/// its face. What is left is seed 2790's skin.
#[test]
#[ignore = "skin"]
fn seed_5000136_a_sliver_far_from_the_origin_is_not_read_backwards() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(
            Plane::xy(4.0),
            Outline::rectangle([9.0, 4.0], [11.5, 6.0]),
            6.0,
        ),
        vec![
            Step::add(Leaf::prism(
                Plane::xy(4.0),
                Outline::rectangle([11.0, 5.0], [12.0, 6.0]),
                11.0,
            )),
            Step::cut(Leaf::prism(
                Plane::xy(3.0),
                Outline::circle([11.49999002, 5.0], 1.0),
                11.0,
            )),
        ],
    ));
}
