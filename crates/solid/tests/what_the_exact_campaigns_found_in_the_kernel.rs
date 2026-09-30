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
/// Over that stretch the side and the wall stand closer than the tolerance,
/// and the kernel kept a skin of matter thinner than it can tell between two
/// faces, back to back, their triangles on each other. The arc of the circle
/// and the side's edge between the same two corners are now one edge
/// (decision 6), and the strip of the side and the strip of the wall they
/// bound are one piece of surface, decided once: the cut takes it.
#[test]
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
/// its face.
///
/// What is left is a skin decision 6 does not reach. The block is the join
/// of two, and its side keeps the seam where the two sides met, an edge
/// between two faces of one plane that nothing else uses. The strip of the
/// side between the line the bore touches it along and that seam is closed
/// at its top by another seam, the first block's top edge on the side; the
/// bore's wall beside it has no arc there, the circle where that top meets
/// the wall being on no face, and the two strips are not bounded by the
/// same arcs. Both are kept, back to back five hundredths of a nanometre
/// apart, and their triangles cross. Faces merged across the seams an
/// operation leaves — step 8 of the boolean, not written yet — would leave
/// no strip to keep.
#[test]
#[ignore = "seam"]
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

/// A slab under a tower, cut by a column at the tower's corner flush with
/// two of its sides and the slab's bottom. The slab's top under the tower is
/// covered by neither operand, and the point chosen inside it stood on the
/// column's side: asking the column how it wraps it was a tie, where nothing
/// hung on the answer.
#[test]
fn seed_6000336_a_column_cut_at_a_tower_s_corner_through_the_slab_under_it() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(
            Plane::xy(-2.0),
            Outline::rectangle([6.0, 4.0], [8.0, 6.0]),
            4.0,
        ),
        vec![
            Step::add(Leaf::prism(
                Plane::xy(-2.0),
                Outline::rectangle([4.0, 2.0], [10.0, 8.0]),
                2.0,
            )),
            Step::cut(Leaf::prism(
                Plane::xy(-2.0),
                Outline::rectangle([6.0, 5.0], [7.0, 6.0]),
                5.0,
            )),
        ],
    ));
}

/// The same with the slab's bottom a hair under the tower's.
#[test]
fn seed_6000171_a_block_cut_through_a_tower_on_a_slab_a_hair_lower() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(
            Plane::xy(1.0),
            Outline::rectangle([7.0, 7.5], [8.0, 8.5]),
            10.0,
        ),
        vec![
            Step::add(Leaf::prism(
                Plane::xy(1.00000002),
                Outline::rectangle([5.0, 6.0], [10.0, 11.0]),
                10.0,
            )),
            Step::cut(Leaf::prism(
                Plane::xy(0.0),
                Outline::rectangle([6.0, 8.0], [13.0, 13.0]),
                5.0,
            )),
        ],
    ));
}

/// A block given a boss on its side and bored by a hole whose wall touches
/// the boss's, both centred on the side, then cut: the line where the side
/// cuts the hole a second time was never taken to lie on the side, the pair
/// of the two coming to share a curve only as another pair was completed,
/// and a corner was put on the line where the hole touches the boss.
#[test]
fn seed_6000233_a_bossed_block_bored_by_a_hole_touching_the_boss() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(
            Plane::xy(0.0),
            Outline::rectangle([5.0, 9.0], [10.5, 15.0]),
            8.0,
        ),
        vec![
            Step::add(Leaf::prism(
                Plane::xy(-1.0),
                Outline::circle([10.5, 12.0], 1.0),
                15.0,
            )),
            Step::cut(Leaf::prism(
                Plane::xy(-1.0),
                Outline::circle([10.5, 14.0], 1.0),
                30.0,
            )),
            Step::cut(Leaf::prism(
                Plane::xy(5.0),
                Outline::rectangle([1.0, 0.0], [11.0, 10.0]),
                3.0,
            )),
        ],
    ));
}

/// A bar across a post, the post's wall reaching the bar's end cap where the
/// cap stands tangent to it: the curve the two meet along touches the cap's
/// circle there to the fourth order, leaving that corner with one direction
/// and one bend, and rounding ordered the two.
#[test]
fn seed_6000830_a_bar_across_a_post_whose_wall_touches_the_bar_s_cap() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(Plane::xy(0.0), Outline::circle([10.0, 7.0], 6.0), 9.0),
        vec![Step::add(Leaf::prism(
            Plane::yz(-2.0),
            Outline::circle([10.0, 8.0], 3.0),
            18.0,
        ))],
    ));
}

/// A post bored by a bar whose side meets the post's where the bar's cap
/// stands tangent to the post: the same touch to the fourth order.
#[test]
fn seed_5000221_a_post_bored_where_the_bore_s_cap_touches_its_wall() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(Plane::xy(-5.0), Outline::circle([30.0, 5.0], 25.0), 40.0),
        vec![Step::cut(Leaf::prism(
            Plane::yz(5.0),
            Outline::circle([35.0, 15.0], 30.0),
            18.0,
        ))],
    ));
}

/// The same bar raised the other way: its cap touches the post at a single
/// point of its wall, the very point the post's band of wall was read at,
/// and asking the bar how it wraps it was a tie. The band is read again
/// further along the same chord.
#[test]
fn seed_5000221_a_bar_whose_cap_touches_a_post_at_one_point_cuts_nothing() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(Plane::xy(-5.0), Outline::circle([30.0, 5.0], 25.0), 40.0),
        vec![Step::cut(Leaf::prism(
            Plane::yz(5.0),
            Outline::circle([35.0, 15.0], 30.0),
            -18.0,
        ))],
    ));
}

/// A post given a block and cut by a second post of its radius three tenths
/// of a micron aside, more than the tolerance. The two circles and a side of
/// the block run within a millionth of a micron of each other between two
/// corners three tenths of a micron apart: the slivers between them are
/// thinner than rounding, and no point can be found inside them.
#[test]
#[ignore = "sliver"]
fn seed_6000839_two_posts_a_hair_apart_both_touching_a_block_s_side() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(Plane::xz(120.0), Outline::circle([30.0, 30.0], 60.0), 270.0),
        vec![
            Step::add(Leaf::prism(
                Plane::xz(119.9999997),
                Outline::rectangle([15.0, 60.0], [45.0, 90.0]),
                270.0,
            )),
            Step::cut(Leaf::prism(
                Plane::xz(120.0),
                Outline::circle([30.0000003, 30.0], 60.0),
                90.0,
            )),
        ],
    ));
}

/// A block given a post, then a second post of its radius six hundredths of
/// a micron aside cut from above, touching nothing but the top. The two
/// circles cross each other at a hair's angle three hundredths of a micron
/// short of where the block's side meets the first at its lowest and its
/// highest point; between each crossing and that corner they part by less
/// than a billionth of a micron, under what rounding reads at a reach of
/// eleven and a half.
///
/// The top came out with the lune between the two circles' halves inside
/// the block listed twice, as two faces turned opposite ways: one runs round
/// it counterclockwise, the other clockwise and through both crossings
/// twice, taking in the slivers at each end. What that face sweeps the wrong
/// way, 3e-7, is more than the room along it; the listing's check left it
/// unjudged while it counted the room at every corner times its distance
/// from where the face is read, and the campaign saw only the triangles
/// left open. Two arcs between the same two corners closer than rounding
/// are now one edge (decision 6), and the slivers are gone.
#[test]
fn seed_7000099_two_posts_a_hair_apart_leave_the_top_a_face_running_backwards() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(
            Plane::xy(2.0),
            Outline::rectangle([6.0, 1.0], [9.0, 9.0]),
            -5.0,
        ),
        vec![
            Step::add(Leaf::prism(
                Plane::xy(2.0),
                Outline::circle([9.0, 4.75], 2.5),
                -5.0,
            )),
            Step::cut(Leaf::prism(
                Plane::xy(2.0),
                Outline::circle([8.99999994, 4.75], 2.5),
                4.0,
            )),
        ],
    ));
}

/// Failure 1-2 of campaign 1: a block given a boss whose circle touches the
/// block's bottom and top, its centre two hundredths of a micron short of
/// the block's side. From each touch to the corner a hair along the side the
/// circle runs within rounding of the side's edge, and the two bounded a
/// sliver of the cap no point could be found inside. They are one edge now,
/// a line lying on the boss's wall too (decision 6); the strip of the bottom
/// and the strip of the wall between the line they touch along and the
/// side's edge are one piece of surface, kept once, on the plane.
#[test]
fn seed_1019570_a_boss_touching_a_block_s_bottom_and_top_a_hair_short_of_its_side_is_joined() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(
            Plane::yz(4.0),
            Outline::rectangle([2.0, 8.0], [4.5, 13.0]),
            5.0,
        ),
        vec![Step::add(Leaf::prism(
            Plane::yz(4.0),
            Outline::circle([4.49999998, 10.5], 2.5),
            9.0,
        ))],
    ));
}

/// The same joined body cut by a block that misses it. Its edge where the
/// cap's circle was taken for the bottom's edge lies on the boss's wall only
/// between its two corners: registered with the wall, the whole line of the
/// bottom's edge came to lie on it, and every corner along it.
#[test]
fn seed_1019570_the_joined_body_keeps_its_edge_on_the_wall_to_its_own_stretch() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(
            Plane::yz(4.0),
            Outline::rectangle([2.0, 8.0], [4.5, 13.0]),
            5.0,
        ),
        vec![
            Step::add(Leaf::prism(
                Plane::yz(4.0),
                Outline::circle([4.49999998, 10.5], 2.5),
                9.0,
            )),
            Step::cut(Leaf::prism(
                Plane::xy(7.0),
                Outline::rectangle([3.0, -1.0], [5.0, 1.0]),
                5.0,
            )),
        ],
    ));
}

/// Failure 1-2 cut rather than joined: the bore takes the strip of the
/// bottom and the strip of its wall, one piece of surface, whose points were
/// ties for a ray, standing a hundred thousandth of a micron from both.
#[test]
fn seed_1042977_a_bore_touching_a_block_s_bottom_and_top_a_hair_short_of_its_side_cuts_it() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(
            Plane::yz(40.0),
            Outline::rectangle([5.0, 45.0], [32.5, 55.0]),
            50.0,
        ),
        vec![Step::cut(Leaf::prism(
            Plane::yz(35.0),
            Outline::circle([32.499999, 50.0], 5.0),
            100.0,
        ))],
    ));
}

/// The same on the XZ plane, the bore flush with the block's end.
#[test]
fn seed_1025756_a_bore_flush_with_a_block_s_end_touching_its_sides_a_hair_short_of_the_third() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(
            Plane::xz(2.0),
            Outline::rectangle([4.0, 10.0], [6.5, 16.0]),
            1.0,
        ),
        vec![Step::cut(Leaf::prism(
            Plane::xz(2.0),
            Outline::circle([6.49999994, 13.0], 3.0),
            4.0,
        ))],
    ));
}

/// The same bored through a thin bar: the triangles of the cap crossed each
/// other at the sliver, drawn from two edges a hair apart.
#[test]
fn seed_1002077_a_hole_through_a_bar_touching_its_sides_a_hair_short_of_its_end_is_uncrossed() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(
            Plane::xy(3.0),
            Outline::rectangle([4.0, 8.5], [11.5, 9.5]),
            7.0,
        ),
        vec![Step::cut(Leaf::prism(
            Plane::xy(2.0),
            Outline::circle([11.4999998, 9.0], 0.5),
            13.0,
        ))],
    ));
}

/// A block cut by a bore touching its bottom and top six tenths of a micron
/// short of its side, then given a block far off, whose reach doubles the
/// tolerance. The block's corner lies on the bottom and on the bore within
/// it, as every place of a band millimetres wide does where two surfaces
/// touch, and was put on the line they touch along: a corner lies on that
/// line only within the tolerance.
#[test]
fn seed_1034172_a_corner_near_where_a_bore_touches_a_block_stays_off_the_line_of_touch() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(
            Plane::yz(90.0),
            Outline::rectangle([120.0, 180.0], [210.0, 270.0]),
            45.0,
        ),
        vec![
            Step::cut(Leaf::prism(
                Plane::yz(60.0),
                Outline::circle([209.9999994, 225.0], 45.0),
                45.0,
            )),
            Step::add(Leaf::prism(
                Plane::xy(0.0),
                Outline::rectangle([90.0, 270.0], [165.0, 315.0]),
                75.0,
            )),
        ],
    ));
}

/// Failure 1-2 over three steps: a bar given a boss touching its sides a
/// hair past its end, cut twice by blocks.
#[test]
fn seed_1002023_a_bar_given_a_boss_touching_its_sides_a_hair_past_its_end_is_cut_twice() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(
            Plane::yz(30.0),
            Outline::rectangle([25.0, 35.0], [65.0, 40.0]),
            33.0,
        ),
        vec![
            Step::add(Leaf::prism(
                Plane::yz(24.9999999),
                Outline::circle([65.0000001, 37.5], 2.5),
                33.0,
            )),
            Step::cut(Leaf::prism(
                Plane::xy(5.0),
                Outline::rectangle([45.0, 30.0], [65.0, 68.0]),
                48.0,
            )),
            Step::cut(Leaf::prism(
                Plane::yz(20.0000001),
                Outline::rectangle([35.0, 40.0], [68.0, 68.0]),
                40.0,
            )),
        ],
    ));
}

/// Failure 1-9 of campaign 1: a block bored by a post, then by a second of
/// its radius three tenths of a micron aside. Where the two circles run
/// within the tolerance of each other near where they cross, their arcs
/// bounded slivers of the cap no point could be found inside; they are one
/// edge now (decision 6).
#[test]
fn seed_1015786_two_bores_of_one_radius_a_hair_apart_grazing_each_other() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(
            Plane::xz(-5.0),
            Outline::rectangle([20.0, 13.0], [55.0, 53.0]),
            15.0,
        ),
        vec![
            Step::cut(Leaf::prism(
                Plane::xz(1e-7),
                Outline::circle([55.0, 32.5], 10.0),
                15.0,
            )),
            Step::cut(Leaf::prism(
                Plane::xz(-5.0),
                Outline::circle([55.0000003, 32.5], 10.0),
                15.0,
            )),
        ],
    ));
}

/// The same with bores sunk the other way, two microns apart at a reach of
/// 450: the listing's check read a loop of the slivers backwards.
#[test]
fn seed_1002151_two_bores_of_one_radius_two_microns_apart_leave_no_face_running_backwards() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(
            Plane::xy(-60.0),
            Outline::rectangle([270.0, 270.0], [375.0, 450.0]),
            255.0,
        ),
        vec![
            Step::cut(Leaf::prism(
                Plane::xy(-60.0),
                Outline::circle([375.0, 360.0], 45.0),
                -270.0,
            )),
            Step::cut(Leaf::prism(
                Plane::xy(-60.0),
                Outline::circle([374.9999982, 360.0], 45.0),
                -300.0,
            )),
        ],
    ));
}

/// Failure 1-8 of campaign 1: a block cut by a bore touching its sides a
/// hundredth of a micron past its end, then cut across by a block.
#[test]
fn seed_1045974_a_block_bored_a_hair_past_its_end_is_cut_across() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(
            Plane::yz(1.0),
            Outline::rectangle([0.0, 2.0], [2.0, 6.0]),
            8.0,
        ),
        vec![
            Step::cut(Leaf::prism(
                Plane::yz(1.0000001),
                Outline::circle([2.00000001, 4.0], 2.5),
                3.0,
            )),
            Step::cut(Leaf::prism(
                Plane::xz(0.0),
                Outline::rectangle([-4.5, 2.5], [6.5, 13.5]),
                3.0,
            )),
        ],
    ));
}

/// Failure 1-9 of campaign 1: a post along Y grooved by a bar along X, then
/// cut by a post of its radius a tenth of a micron aside. The two posts
/// cross along two lines at a grazing angle; the curve the first post meets
/// the bar along was scanned for where it crosses the second, which stands
/// within the tolerance of it over millimetres, and the corner came out
/// eight tenths of a micron off those lines. It is found where the lines
/// pass through the bar: a triple is solved from its most degenerate pair.
#[test]
fn seed_1044340_a_grooved_post_cut_by_a_post_of_its_radius_a_hair_aside() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(Plane::xz(40.0), Outline::circle([10.0, 30.0], 20.0), 45.0),
        vec![
            Step::cut(Leaf::prism(
                Plane::yz(-10.0),
                Outline::circle([25.0, 50.0], 8.0),
                45.0,
            )),
            Step::cut(Leaf::prism(
                Plane::xz(45.0),
                Outline::circle([10.0000001, 30.0], 20.0),
                90.0,
            )),
        ],
    ));
}

/// The same with the bar joined and the second post six tenths of a micron
/// aside at a reach of 255.
#[test]
fn seed_1034890_a_barred_post_cut_by_a_post_of_its_radius_a_hair_aside() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(
            Plane::xy(30.0),
            Outline::circle([120.0, 120.0], 60.0),
            210.0,
        ),
        vec![
            Step::add(Leaf::prism(
                Plane::yz(-30.0),
                Outline::circle([60.0, 120.0], 75.0),
                255.0,
            )),
            Step::cut(Leaf::prism(
                Plane::xy(30.0),
                Outline::circle([120.0000006, 120.0], 60.0),
                225.0,
            )),
        ],
    ));
}

/// Failure 1-8 of campaign 1: two blocks joined with their tops six tenths
/// of a micron apart, more than the tolerance at a reach of 465, then a
/// third whose top stands halfway between, within it of both. Taken for the
/// nearer, the third's top stood, as the third was made, across the strip of
/// wall between the other two, and a point of that strip was a tie: a
/// surface standing between two of the body's within the tolerance of each
/// is taken for neither.
#[test]
fn seed_1008675_a_block_whose_top_stands_between_two_tops_a_hair_apart_is_joined() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(
            Plane::xz(150.0),
            Outline::rectangle([120.0, 240.0], [285.0, 465.0]),
            30.0,
        ),
        vec![
            Step::add(Leaf::prism(
                Plane::xz(150.0000006),
                Outline::rectangle([120.0000018, 240.0], [345.0, 465.0]),
                60.0,
            )),
            Step::add(Leaf::prism(
                Plane::xz(150.00000029999998),
                Outline::rectangle([135.0, 255.0], [405.0, 450.0]),
                255.0,
            )),
        ],
    ));
}

/// Failure 1-8 of campaign 1: a block cut by a block flush with its side and
/// its bottom a twentieth of a micron off each. The tool's corner on its end
/// kept the tool's own place, seven hundredths of a micron from the lines
/// the block's side and bottom share with that end: a corner on three planes
/// whose normals span space stands where they meet.
#[test]
fn seed_1026137_a_block_cut_by_a_block_flush_with_two_of_its_sides_by_a_hair() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(
            Plane::xy(15.0),
            Outline::rectangle([7.5, 7.5], [42.5, 42.5]),
            50.0,
        ),
        vec![Step::cut(Leaf::prism(
            Plane::yz(10.0000001),
            Outline::rectangle([42.49999995, 14.99999995], [62.50000005, 35.00000005]),
            50.0,
        ))],
    ));
}

/// The same seed shrunk further once that was fixed: a bore touching the
/// block's side five hundredths of a micron inside, then a boss touching the
/// side a tenth of a micron outside and the bore inside. The corner the bore
/// left on the side stands within the tolerance of the line the side and the
/// boss touch along, and that line within it of the boss, but the corner a
/// tenth and a half from the boss: it is on the line only within the
/// tolerance of both surfaces.
#[test]
fn seed_1026137_a_corner_a_bore_left_on_a_side_stays_off_a_boss_touching_it_by_two_hairs() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(
            Plane::xy(15.0),
            Outline::rectangle([7.5, 7.5], [42.5, 42.5]),
            50.0,
        ),
        vec![
            Step::cut(Leaf::prism(
                Plane::yz(10.0000001),
                Outline::circle([52.5, 25.0], 10.00000005),
                50.0,
            )),
            Step::add(Leaf::prism(
                Plane::yz(9.9999999),
                Outline::circle([47.5, 25.0], 4.9999999),
                100.0,
            )),
        ],
    ));
}

/// The same with a post for the block. The bore touching the post left a
/// seam along the line they touch, laid where the two circles would touch;
/// the boss touching the post a tenth of a micron outside decides its own
/// line of touch, within the tolerance of the seam, and the seam is kept for
/// both, a tenth and a third of a micron off the boss.
///
/// Understood and left: a line taken for another within the tolerance keeps
/// the first's geometry (decision 3), whether or not that stands within the
/// tolerance of the surfaces the other lies on, and arc identity keeps the
/// first registered too. Keeping instead the curve that stands within the
/// tolerance of every surface of the joined support — or refusing the join
/// where none does — is decision 3's refusal still to write; three surfaces
/// touching along one line by two hairs are where it shows.
#[test]
#[ignore = "chained"]
fn seed_1026137_a_seam_a_bore_left_on_a_post_is_kept_off_a_boss_touching_it() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(Plane::yz(15.0), Outline::circle([25.0, 25.0], 17.5), 50.0),
        vec![
            Step::cut(Leaf::prism(
                Plane::yz(10.0000001),
                Outline::circle([52.5, 25.0], 10.00000005),
                50.0,
            )),
            Step::add(Leaf::prism(
                Plane::yz(9.9999999),
                Outline::circle([47.5, 25.0], 4.9999999),
                100.0,
            )),
        ],
    ));
}

/// Failure 1-6 of campaign 1: a block grooved three tenths of a micron deep
/// on its end, the tolerance of the groove's reach, then a post whose wall
/// passes through the block's corner. At the post's reach the groove's floor
/// stands within the tolerance of the end, and a stretch of the groove's rim
/// four tenths of a micron long was taken for the post's circle between the
/// same two corners: seen on the post, a circle square to its axis was
/// declined rather than read as the chord it stands within the tolerance of.
#[test]
fn seed_1014146_a_grooved_block_given_a_post_through_its_corner() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(
            Plane::yz(60.0),
            Outline::rectangle([225.0, 60.0], [255.0, 270.0]),
            -30.0,
        ),
        vec![
            Step::cut(Leaf::prism(
                Plane::yz(59.9999997),
                Outline::circle([255.0, 165.0], 45.0),
                30.0,
            )),
            Step::add(Leaf::prism(
                Plane::xy(120.0),
                Outline::circle([3.0000000000000004e-7, 300.0], 75.0),
                255.0,
            )),
        ],
    ));
}

/// The same seed shrunk further once that was fixed: the groove a notch
/// across the block's end. At the post's reach the notch's edge on the
/// block's side stands within the tolerance of the line the side and the
/// post's wall cross along — the block's own edge, three tenths of a micron
/// away and apart from the notch's — and was taken to lie on the post: a
/// stretch of it past the notch, up to the block's top, stood on the side
/// beside the block's edge, and the two left the top's corner as one. A
/// curve lies along a surface through a pair only when the pair's curve is
/// not already another curve apart from it.
#[test]
fn seed_1014146_a_block_notched_a_hair_deep_given_a_post_through_its_corner() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(
            Plane::yz(60.0),
            Outline::rectangle([225.0, 60.0], [255.0, 270.0]),
            -30.0,
        ),
        vec![
            Step::cut(Leaf::prism(
                Plane::yz(59.9999997),
                Outline::rectangle([210.0, 120.0], [300.0, 210.0]),
                30.0,
            )),
            Step::add(Leaf::prism(
                Plane::xy(120.0),
                Outline::circle([0.0, 300.0], 75.0),
                255.0,
            )),
        ],
    ));
}

/// The same seed shrunk once more: the block cut through but for a wall
/// three tenths of a micron thick on its end, then the post through its
/// corner. The corner the wall's inner face, its top and the block's side
/// share was merged with where the post's rim crosses the side's edge, three
/// tenths away on the end: it took the post's wall, and with it the block's
/// edge on the end and the line the post crosses the inner face along, four
/// tenths away. Both ran up to it beside the wall's inner edge. No corner
/// lies on a curve on a surface apart from one of its own, and two corners
/// are not merged where a surface of each crosses the other's along lines
/// that all stand further than the tolerance from them.
#[test]
fn seed_1014146_a_wall_a_hair_thick_given_a_post_through_its_corner() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(
            Plane::yz(60.0),
            Outline::rectangle([225.0, 60.0], [255.0, 270.0]),
            -30.0,
        ),
        vec![
            Step::cut(Leaf::prism(
                Plane::yz(59.9999997),
                Outline::rectangle([210.0, 120.0], [300.0, 210.0]),
                -30.0,
            )),
            Step::add(Leaf::prism(
                Plane::xy(120.0),
                Outline::circle([0.0, 300.0], 75.0),
                255.0,
            )),
        ],
    ));
}
