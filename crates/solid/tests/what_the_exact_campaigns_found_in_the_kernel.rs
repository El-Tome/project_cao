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
/// longer bit for bit its own: the kernel declined.
///
/// It then answered with the corner where the top, through the pin's axis,
/// meets the pin and the post found on the pin as it stood, and the curve
/// running on the pin as it was moved: 1.04 times the tolerance apart where
/// they cross at a slant (failure 1-10). The move is now the surface's,
/// made once before any curve or corner is found, and both stand on the
/// moved pin.
#[test]
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
/// It holds since the boolean merges the faces of one surface and one side
/// across an arc nothing else uses, and joins two edges of one curve at a
/// vertex only they reach (step 8).
#[test]
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
/// thinner than rounding, and no point can be found inside them. They lie
/// on a cap, across the band of the side and the two walls, not along it:
/// decision 7, which reads a region lying along the surface it stands
/// within the tolerance of, does not reach them, and the listing comes out
/// with edges used one way only.
/// It holds since two parallel cylinders of one radius within twenty
/// tolerances are one surface (decision 8): there are no two circles left.
#[test]
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
/// It holds since the boolean merges the faces of one surface and one side
/// across an arc nothing else uses, and joins two edges of one curve at a
/// vertex only they reach (step 8).
#[test]
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

/// Failure 1-6 of campaign 1: a block on a bar whose side stands two
/// hundredths of a micron in from the block's, the bar bored through above
/// the block, then a third block apart from both. The bore left the block's
/// top two faces, a strip beside the bar's old side and the floor of the
/// bore, parted by a stretch of that side's line lying on the top alone. The
/// third block read it again at the same tolerance, and took that stretch
/// for the block's own edge on its top, two hundredths away: the strip lost
/// its side. Two curves of one operand are never taken for one.
#[test]
fn seed_1004747_a_block_bored_beside_a_step_a_hair_high_then_given_a_block_apart() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(
            Plane::xz(8.0),
            Outline::rectangle([3.0, 10.0], [5.0, 12.0]),
            9.0,
        ),
        vec![
            Step::add(Leaf::prism(
                Plane::xz(8.0),
                Outline::rectangle([3.00000002, 10.0], [7.0, 12.0]),
                -6.0,
            )),
            Step::cut(Leaf::prism(
                Plane::xz(7.99999998),
                Outline::circle([5.0, 10.0], 2.5),
                -12.0,
            )),
            Step::add(Leaf::prism(
                Plane::xy(1.0),
                Outline::rectangle([8.0, 5.0], [12.0, 9.0]),
                8.0,
            )),
        ],
    ));
}

/// Seed 1014146 shrunk once the wall and the notch hold: the block bored
/// through across its end but for a wall three tenths of a micron thick,
/// the bore's bottom at the height of the post's cap, which it touches along
/// the line through the corner the post's wall passes.
///
/// Understood and left: failure 1-12's configuration. The curve the post
/// and the bore meet along touches the post's rim at that corner to the
/// second order, and runs within a femtometre of it back to where the rim
/// crosses the block's end — a corner four tenths of a micron off the line
/// the cap and the bore touch along, so not on the bore by its support. The
/// two arcs share one corner and overlap, set off from it at one angle, and
/// are both too short for their bends to part them: the star ties.
#[test]
#[ignore = "tangent"]
fn seed_1014146_a_wall_a_hair_thick_bored_where_the_post_s_cap_touches_the_bore() {
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
                -30.0,
            )),
            Step::add(Leaf::prism(
                Plane::xy(120.0),
                Outline::circle([3.0000000000000004e-7, 300.0], 75.0),
                255.0,
            )),
        ],
    ));
}

/// Failure 1-9 of campaign 1: a block given two posts of one radius a tenth
/// of a micron apart, both touching its top, both standing a tenth of a
/// micron proud of its end.
///
/// Understood and left: three lines a chain. The line the two posts cross
/// along stands between the two lines each touches the top along, within
/// the tolerance of each, and those two a hair further than it apart. The
/// crossing is taken for the first post's line of touch, so that line lies
/// on both posts; the corner where the posts cross on their cap lies on both
/// lines of touch, and their stretches above the block's end run into it
/// side by side. Taking the crossing for neither line, or keeping a corner
/// within the tolerance of every place merged into it, leaves the strip of
/// the top between the two lines of touch to be wound against the second
/// post, a femtometre from its wall: the strip is no twin of the walls
/// beside it, since the crossing parts each wall in two. Three surfaces
/// touching along one band call for the band decided once, not three pairs.
/// It holds since a corner lies on a curve its support names only within
/// the tolerance of it: the corner on the cap lies on the line of touch it
/// stands on, not on the other a hair further.
#[test]
fn seed_1040082_a_block_given_two_posts_a_hair_apart_touching_its_top() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(
            Plane::xz(0.0),
            Outline::rectangle([-17.5, -2.5], [27.5, 42.5]),
            23.0,
        ),
        vec![
            Step::add(Leaf::prism(
                Plane::xz(1e-7),
                Outline::circle([5.0, 40.0], 2.5),
                45.0,
            )),
            Step::add(Leaf::prism(
                Plane::xz(1e-7),
                Outline::circle([4.9999999, 40.0], 2.5),
                90.0,
            )),
        ],
    ));
}

/// Failure 1-2 of campaign 1: a block given a post of its own width, the
/// post bored by a hole of three quarters its radius, touching the post
/// inside and the block's side two hundredths of a micron from where the
/// post does.
///
/// Understood and left: three lines of touch on one band. The side touches
/// the post and the hole along two lines two hundredths of a micron apart,
/// and the hole touches the post along a third, eight hundredths away, all
/// further apart than the tolerance; across the band the three surfaces
/// stand within a femtometre of each other. The strip of the side between
/// its two lines of touch is the post's, and is wound against the hole from
/// a point a femtometre off its wall: no ray tells. Its twin on the hole's
/// wall is bounded by the third line, not by the strip's own two, so the
/// two are not wound together.
/// It holds since the boolean merges the faces of one surface and one side
/// across an arc nothing else uses, and joins two edges of one curve at a
/// vertex only they reach (step 8).
#[test]
fn seed_1031984_a_post_of_a_block_s_width_bored_by_a_hole_touching_it_and_the_side() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(
            Plane::xy(0.0),
            Outline::rectangle([6.0, 8.0], [10.0, 12.0]),
            19.0,
        ),
        vec![
            Step::add(Leaf::prism(
                Plane::xy(0.0),
                Outline::circle([8.0, 10.0], 2.0),
                19.0,
            )),
            Step::cut(Leaf::prism(
                Plane::xy(0.0),
                Outline::circle([7.99999998, 9.5], 1.5),
                19.0,
            )),
        ],
    ));
}

/// Failure 1-2 of campaign 1: a block whose end a post of its width is
/// centred on, then a second post of that radius nine tenths of a micron
/// aside, both touching the block's two sides.
///
/// Understood and left: the two posts cross along lines a rounding of their
/// radii puts two and a half microns along the band they stand within a
/// tenth of a picometre of each other and of the sides on, outside the two
/// lines each touches a side along. The corners the crossing makes with the
/// caps stand on the band, within the tolerance of every surface there, and
/// the loops of the sides cannot be closed through them: the listing comes
/// out with edges used one way only. Decision 7 does not reach it — what
/// fails is the corners and the loops through them, before any region is
/// read.
/// It holds since decision 8: the second post is the first's wall, and
/// there is no crossing left.
#[test]
fn seed_1044418_a_block_ended_by_a_post_given_a_second_post_a_hair_aside() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(
            Plane::xy(150.0),
            Outline::rectangle([0.0, 60.0], [195.0, 240.0]),
            60.0,
        ),
        vec![
            Step::add(Leaf::prism(
                Plane::xy(150.0),
                Outline::circle([195.0, 150.0], 90.0),
                120.0,
            )),
            Step::add(Leaf::prism(
                Plane::xy(120.0),
                Outline::circle([195.0000009, 150.0], 90.0),
                240.0,
            )),
        ],
    ));
}

/// Failure 1-2 of campaign 1: a block bored ten microns inside its side, a
/// post of the bore's radius centred on the side, then a block apart from
/// both.
///
/// Understood and left: the bore and the post cross at a grazing angle, and
/// a place within the tolerance of both may stand microns from the line
/// they cross along. The corner where the bore met the side at the bore's
/// top lies on both, and read again by the last operation it is taken to
/// lie on that line, five microns away: the line's stretch up the post ends
/// at it and at its own corner both. A corner of one operand lies on a curve
/// of two of that operand's surfaces only as the operand's own edges say,
/// which the support alone cannot tell.
/// It holds since a corner lies on a curve its support names only within
/// the tolerance of it, which the corner five microns from the line does
/// not.
#[test]
fn seed_1013792_a_bored_block_given_a_post_on_its_side_then_a_block_apart() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(
            Plane::xy(2.0),
            Outline::rectangle([9.0, 10.0], [10.5, 16.0]),
            10.0,
        ),
        vec![
            Step::cut(Leaf::prism(
                Plane::xy(2.0),
                Outline::circle([10.49999, 13.0], 1.5),
                5.0,
            )),
            Step::add(Leaf::prism(
                Plane::xy(2.0),
                Outline::circle([10.5, 13.0], 1.5),
                20.0,
            )),
            Step::cut(Leaf::prism(
                Plane::xy(2.0),
                Outline::rectangle([2.0, 5.0], [6.0, 9.0]),
                1.0,
            )),
        ],
    ));
}

/// Failure 1-2 of campaign 1: a bar bored by a hole touching its side at
/// its end, then by a hole of that radius ten microns aside,
/// touching the side too.
///
/// Understood and left: the side's strip between the two lines the holes
/// touch it along is ten microns wide, and the holes' walls over
/// it stand less than a picometre above it — a skin decision 6 is meant to
/// leave out. But the walls over the strip are two, parted by the line the
/// holes cross along, so the strip has no twin bounded by its own arcs, and
/// the skin is kept: the triangles of the strip and of the walls cross.
/// Decision 7 winds each strip beside the faces lying over it as the exact
/// geometry has it, and keeps the skin too. The line the holes cross along
/// stands within a picometre of the side; laid on the side as well, it
/// would part the side's strip where the walls' end and make them twins —
/// the band's lines shared by every surface of the band, which decisions 3
/// and 5 do not make.
#[test]
#[ignore = "band"]
fn seed_1032742_a_bar_bored_twice_a_hair_apart_touching_its_side() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(
            Plane::xy(-10.0),
            Outline::rectangle([40.0, 40.0], [45.0, 70.0]),
            13.0,
        ),
        vec![
            Step::cut(Leaf::prism(
                Plane::xy(-10.0),
                Outline::circle([45.0, 55.0], 15.0),
                25.0,
            )),
            Step::cut(Leaf::prism(
                Plane::xy(-15.0),
                Outline::circle([44.99999, 55.0], 15.0),
                25.0,
            )),
        ],
    ));
}

/// Failure 1-10 of campaign 3: a post along Y and a bar along X of radius
/// 150 whose axis stands 3e-7 off touching the post from inside. Decision 2
/// moved the bar onto the touch in the curve the two meet along, while the
/// corners on the bar were found on the bar as drawn: an end of a curve up
/// to 4.6 tolerances off its vertex where the two cross at a slant. The move
/// is now the surface's, made before anything is found on either.
#[test]
fn seed_3154892_a_bar_a_hair_from_touching_a_post_inside_is_joined_on_one_bar() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(
            Plane::xy(150.0),
            Outline::circle([270.0, 120.0], 90.0),
            255.0,
        ),
        vec![Step::add(Leaf::prism(
            Plane::yz(60.0),
            Outline::circle([180.0000003, 210.0], 150.0),
            330.0,
        ))],
    ));
}

/// Failure 1-10 of campaign 3: a hole of radius 0.99999999 along Z through a
/// post along X of radius 4, a hair short of touching its wall from inside.
#[test]
fn seed_3038433_a_hole_a_hair_short_of_touching_a_post_s_wall_inside_cuts_it() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(Plane::yz(3.0), Outline::circle([7.0, 2.0], 4.0), 9.0),
        vec![Step::cut(Leaf::prism(
            Plane::xy(3.0),
            Outline::circle([5.5, 4.0], 0.99999999),
            3.0,
        ))],
    ));
}

/// Failure 1-10 of campaign 3: a block bored by a hole of radius
/// 45.0000003, then cut by a cylinder across that touches the hole from
/// inside but for the hair of its radius.
#[test]
fn seed_3117341_a_bored_block_cut_by_a_cylinder_a_hair_from_touching_the_bore() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(
            Plane::xy(210.0),
            Outline::rectangle([15.0, -105.0], [345.0, 225.0]),
            120.0,
        ),
        vec![
            Step::cut(Leaf::prism(
                Plane::xy(210.0),
                Outline::circle([300.0, 60.0], 45.0000003),
                240.0,
            )),
            Step::cut(Leaf::prism(
                Plane::xz(210.0),
                Outline::circle([210.0, 300.0], 135.0),
                180.0,
            )),
        ],
    ));
}

/// Failure 1-10 of campaign 3: a block bored along Y, then cut by a
/// cylinder along Z whose wall a hair inside touches the bore. Without the
/// move the corner lands 2.8e-4 off the curve; with it moved in the curve
/// only, an end of the curve stands off its vertex.
#[test]
fn seed_3082496_a_bored_block_cut_by_a_post_a_hair_inside_touching_the_bore() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(
            Plane::xz(3.5),
            Outline::rectangle([-2.0, -1.0], [8.0, 9.0]),
            -8.0,
        ),
        vec![
            Step::cut(Leaf::prism(
                Plane::xz(4.5),
                Outline::circle([3.0, 4.0], 2.0),
                5.0,
            )),
            Step::cut(Leaf::prism(
                Plane::xy(-2.0),
                Outline::circle([4.99999999, 4.5], 4.0),
                8.0,
            )),
        ],
    ));
}

/// Failure 2-7 of campaign 3: a post along Z touching a bar along Y from
/// inside, the bar's cap a tenth of a micron from the node of the curve the
/// two meet along. The cap cuts the post along a ruling a femtometre inside
/// the bar, which it crosses a tenth of a micron either side of the node;
/// both crossings were taken for one double root between them, eight
/// tolerances off the curve through either. A line a hair inside a wall
/// crosses it twice unless a surface it lies on touches the wall there.
#[test]
fn seed_3059889_a_bar_s_cap_a_hair_from_the_node_of_a_post_it_touches_inside() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(Plane::xy(3.0), Outline::circle([1.0, 4.0], 2.5), 9.0),
        vec![Step::add(Leaf::prism(
            Plane::xz(4.0000001),
            Outline::circle([0.0, 8.0], 3.5),
            8.0,
        ))],
    ));
}

/// Failure 2-7 of campaign 3, with no node: the line where a hole along Z
/// meets the block's top passes 2e-8 inside a bar along X cut through the
/// block, and was taken to touch it once.
#[test]
fn seed_3039560_a_bored_block_cut_by_a_bar_a_hair_over_the_bore_s_rim() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(
            Plane::xy(1.0),
            Outline::rectangle([4.0, 1.0], [10.0, 8.0]),
            9.5,
        ),
        vec![
            Step::cut(Leaf::prism(
                Plane::xy(1.0),
                Outline::circle([5.0, 5.0], 1.0),
                19.0,
            )),
            Step::cut(Leaf::prism(
                Plane::yz(5.0),
                Outline::circle([2.00000002, 9.0], 2.5),
                7.0,
            )),
        ],
    ));
}

/// Failure 2-7 of campaign 3: a post cut by a bar touching it inside, the
/// bar's cap two tenths of a micron from the node.
#[test]
fn seed_3136522_a_post_cut_by_a_bar_whose_cap_stands_a_hair_from_the_node() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(Plane::xy(6.0), Outline::circle([8.0, 3.0], 4.0), 9.0),
        vec![Step::cut(Leaf::prism(
            Plane::xz(3.0000002),
            Outline::circle([7.0, 9.0], 5.0),
            9.0,
        ))],
    ));
}

/// Drawn by the campaign checking failure 2-7's fix: two posts along Y a
/// hair off one axis joined, then cut by a post along Z decided to touch the
/// larger inside. The top of the cut passes through the node, along a ruling
/// of the larger post that rounding leaves a femtometre inside the cut: it
/// only touches it, at the node, and is not taken to cross it twice a tenth
/// of a micron either side.
#[test]
fn seed_50006145_a_ruling_through_the_node_of_two_posts_touching_inside_touches_once() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(Plane::xz(-2.0), Outline::circle([7.5, 5.0], 4.5), 10.0),
        vec![
            Step::add(Leaf::prism(
                Plane::xz(-1.0),
                Outline::circle([7.50000001, 5.0], 6.5),
                -10.0,
            )),
            Step::cut(Leaf::prism(
                Plane::xy(5.0),
                Outline::circle_from([6.5, 2.0], 5.5, 305.63834048705354),
                -8.0,
            )),
        ],
    ));
}

/// Failure 1-6 of campaign 3: two blocks with their sides in one plane,
/// one a hair shorter, joined, each leaving its edges inside the faces they
/// share; a block apart then grows the tolerance past the hair. The seams'
/// corners were merged across it with the longer block's, and two parallel
/// lines left one vertex the star could not order. Faces of one surface and
/// one side are now merged across an arc nothing else uses, and edges of one
/// curve at a vertex only they reach joined (step 8): no seam is left.
#[test]
fn seed_3052206_two_blocks_a_hair_unequal_joined_then_a_block_apart_grows_the_tolerance() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(
            Plane::yz(0.0),
            Outline::rectangle([4.0, 3.0], [5.0, 8.5]),
            10.0,
        ),
        vec![
            Step::add(Leaf::prism(
                Plane::yz(0.0),
                Outline::rectangle([4.0, 3.0], [7.0, 8.5]),
                9.99999999,
            )),
            Step::cut(Leaf::prism(
                Plane::xy(1.0),
                Outline::rectangle([5.0, 10.0], [9.5, 13.0]),
                7.0,
            )),
        ],
    ));
}

/// Failure 1-6 of campaign 3, the seams left between two blocks joined a
/// hair apart in height: three faces on one side after the join, and the
/// tolerance a tall block apart grows merged their corners across the hair.
#[test]
fn seed_3082034_two_blocks_a_hair_apart_in_height_joined_then_a_tall_block_apart() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(
            Plane::xy(4.0),
            Outline::rectangle([2.0, 1.0], [4.0, 3.5]),
            5.5,
        ),
        vec![
            Step::add(Leaf::prism(
                Plane::xy(4.00000001),
                Outline::rectangle([2.0, 1.0], [6.0, 3.5]),
                5.5,
            )),
            Step::add(Leaf::prism(
                Plane::xy(3.0),
                Outline::rectangle([4.0, 7.0], [5.0, 8.0]),
                11.0,
            )),
        ],
    ));
}

/// Failure 1-8 of campaign 3: four blocks with their bottoms at heights a
/// hair apart, the first swallowed by the second, its bottom edges left as
/// seams on the sides; the last block's bottom stood within the tolerance of
/// the seam and of the cut's bottom, which are not of each other. Without the
/// seams nothing chains.
#[test]
fn seed_3177811_planes_a_hair_apart_chained_through_the_seams_of_a_swallowed_block() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(
            Plane::xy(4.0),
            Outline::rectangle([6.0, 2.5], [8.5, 10.5]),
            4.0,
        ),
        vec![
            Step::add(Leaf::prism(
                Plane::xy(3.99999998),
                Outline::rectangle([6.0, 2.5], [10.5, 10.5]),
                8.0,
            )),
            Step::cut(Leaf::prism(
                Plane::xy(4.00000002),
                Outline::rectangle([6.0, 1.0], [14.0, 4.0]),
                7.0,
            )),
            Step::add(Leaf::prism(
                Plane::xy(4.00000001),
                Outline::rectangle([7.0, 2.0], [16.0, 3.0]),
                14.0,
            )),
        ],
    ));
}

/// Failure 2-1 of campaign 3: a block swallowing a post its side touches
/// kept the line of touch as a seam on the side, and a bore touching the side
/// and the swallowed post within the band of the touch left a strip between
/// the seam and its own line that no ray could wind.
#[test]
fn seed_3024043_a_post_swallowed_by_a_block_leaves_no_touch_line_for_a_bore_beside_it() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(Plane::xy(7.0), Outline::circle([5.0, 0.0], 4.0), 6.0),
        vec![
            Step::add(Leaf::prism(
                Plane::xy(7.0),
                Outline::rectangle([1.0, -4.0], [9.0, 4.0]),
                6.0,
            )),
            Step::cut(Leaf::prism(
                Plane::xy(7.0),
                Outline::circle([4.99999994, 4.5], 0.5),
                7.0,
            )),
        ],
    ));
}

/// Failure 1-9 of campaign 3: a cut touching a bar along its foot left the
/// line of touch inside the wall's own face; a bore of the bar's radius a
/// hair off its axis then crossed the wall at a grazing angle beside it.
#[test]
fn seed_3258890_a_bar_touched_along_its_foot_then_bored_by_its_own_radius_a_hair_aside() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(Plane::yz(-1.0), Outline::circle([8.0, 10.0], 1.0), 10.0),
        vec![
            Step::cut(Leaf::prism(
                Plane::yz(-1.0),
                Outline::rectangle([6.0, 8.0], [10.0, 9.0]),
                10.0,
            )),
            Step::cut(Leaf::prism(
                Plane::yz(-1.0),
                Outline::circle([8.00000006, 10.0], 1.0),
                8.0,
            )),
        ],
    ));
}

/// Failure 1-2 of campaign 3: a post touches the side of an L of two
/// blocks 2.5 tolerances from the line where the upper block ends; the strip
/// of the side between that line and the line of touch had no twin on the
/// post's wall.
#[test]
fn seed_3172512_a_post_touching_the_side_of_two_blocks_a_hair_from_the_step_between_them() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(
            Plane::xy(1.0),
            Outline::rectangle([0.0, 7.0], [3.5, 8.0]),
            4.0,
        ),
        vec![
            Step::add(Leaf::prism(
                Plane::xy(-0.0),
                Outline::rectangle([0.0, 7.0], [6.0, 8.0]),
                2.0,
            )),
            Step::add(Leaf::prism(
                Plane::xy(0.0),
                Outline::circle([3.49999998, 7.5], 0.5),
                3.0,
            )),
        ],
    ));
}

/// Failure 1-2 of campaign 3, drawn: a cut touching the bar's side left a
/// seam on it, and a bore touching both sides a hair inside the bar's end
/// left a strip of each side and of the wall that the seam kept from being
/// twins: a strip of the wall lay on the side.
#[test]
fn seed_3114074_a_bore_touching_both_sides_of_a_bar_a_hair_inside_its_end() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(
            Plane::xy(60.0),
            Outline::rectangle([255.0, 0.0], [360.0, 30.0]),
            135.0,
        ),
        vec![
            Step::cut(Leaf::prism(
                Plane::yz(60.0),
                Outline::rectangle([30.0, 180.0], [225.0, 330.0]),
                540.0,
            )),
            Step::cut(Leaf::prism(
                Plane::xy(60.0),
                Outline::circle([359.99999, 15.0], 15.0),
                270.0,
            )),
        ],
    ));
}

/// Failure 1-6 of campaign 3, drawn: a bore of a post's radius a hair off
/// its axis leaves a sliver, and a block reaching further grows the tolerance
/// past it; the block's side was left with two slits along one line.
#[test]
fn seed_3159337_a_sliver_a_bore_leaves_of_a_post_then_a_block_beside_it() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(
            Plane::xy(210.0),
            Outline::circle([60.0, 300.0], 15.0),
            255.0,
        ),
        vec![
            Step::cut(Leaf::prism(
                Plane::xy(210.0),
                Outline::circle([60.0000006, 300.0], 15.0),
                255.0,
            )),
            Step::add(Leaf::prism(
                Plane::xy(180.0),
                Outline::rectangle([15.0, 285.0], [45.0, 315.0]),
                510.0,
            )),
        ],
    ));
}

/// Failure 1-11 of campaign 3: a third block flush with the second on two
/// walls, each a hair off; the line the second's two walls bound was kept
/// where that block computed it, a hair and a half off the corner derived
/// on the walls kept.
#[test]
fn seed_3127395_three_blocks_flush_on_two_walls_a_hair_off() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(
            Plane::xy(8.0),
            Outline::rectangle([0.0, 35.0], [17.5, 45.0]),
            18.0,
        ),
        vec![
            Step::add(Leaf::prism(
                Plane::xy(0.0),
                Outline::rectangle([15.0, 0.0], [53.0, 8.0]),
                35.0,
            )),
            Step::add(Leaf::prism(
                Plane::xy(-5.0000000000000004e-8),
                Outline::rectangle([17.49999995, 2.5], [62.5, 5.0]),
                35.0,
            )),
        ],
    ));
}

/// Failure 1-6 of campaign 3: a hole's floor 1e-7 above the block's
/// bottom, a skin the block held apart, and a taller boss a hair under it
/// growing the tolerance: the skin's corner was merged onto the boss's circle
/// and stood off it.
#[test]
fn seed_3023305_a_bored_block_given_a_boss_a_hair_under_its_floor() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(
            Plane::xy(15.0),
            Outline::rectangle([7.5, 15.0], [37.5, 45.0]),
            45.0,
        ),
        vec![
            Step::cut(Leaf::prism(
                Plane::xy(15.0000001),
                Outline::circle([40.0, 30.0], 2.5),
                33.0,
            )),
            Step::add(Leaf::prism(
                Plane::xy(14.9999997),
                Outline::circle([25.0, 30.0], 12.49999995),
                90.0,
            )),
        ],
    ));
}

/// Failure 1-8 of campaign 3: a hole whose wall pokes 1e-8 through the
/// block's top, within the tolerance, is decided to touch it. The line they
/// touch along was laid on the top and left a hair off the wall, and a
/// corner on it stood just past the tolerance from the circle where the wall
/// meets the block's side. A touch decided a hair apart is now made exact on
/// the surfaces: the second operand's is moved onto it (decision 2).
#[test]
fn seed_3071596_a_hole_a_hair_past_touching_a_block_s_top_is_moved_onto_the_touch() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(
            Plane::yz(6.0),
            Outline::rectangle([-1.0, 2.0], [7.0, 10.0]),
            2.0,
        ),
        vec![Step::cut(Leaf::prism(
            Plane::yz(5.0),
            Outline::circle([3.0, 7.0], 3.00000001),
            2.0,
        ))],
    ));
}

/// Failure 1-8 of campaign 3: a bar along X 6e-7 past touching the wall a
/// cut left in a post, within the tolerance; the corner on the line they
/// touch along stood on the wall and off the bar, and further still off the
/// curve where the bar meets the post.
#[test]
fn seed_3053678_a_bar_a_hair_past_touching_the_wall_a_cut_left_in_a_post() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(
            Plane::xy(150.0),
            Outline::circle([300.0, 30.0], 75.0),
            255.0,
        ),
        vec![
            Step::cut(Leaf::prism(
                Plane::yz(150.0),
                Outline::rectangle([-30.0, 195.0], [90.0, 315.0]),
                195.0,
            )),
            Step::add(Leaf::prism(
                Plane::xy(120.0),
                Outline::rectangle([240.0, 90.0], [360.0, 210.0]),
                510.0,
            )),
            Step::cut(Leaf::prism(
                Plane::yz(150.0),
                Outline::circle([15.0, 255.0], 45.0000006),
                390.0,
            )),
        ],
    ));
}

/// Failure 2-5 of campaign 3: a bore touching the stock inside but for
/// three tenths of a micron, decided to touch; the corner at the foot of the
/// line of touch stood where it was computed, most of the tolerance off the
/// stock, and a triangle fanned from it passed through the stock's.
#[test]
fn seed_3097513_a_bore_a_hair_inside_touching_the_stock_then_cut_in_half() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(
            Plane::xy(-60.0),
            Outline::circle([300.0, 90.0], 29.9999997),
            60.0,
        ),
        vec![
            Step::cut(Leaf::prism(
                Plane::xy(-59.9999994),
                Outline::circle([285.0, 90.0], 15.0),
                60.0,
            )),
            Step::cut(Leaf::prism(
                Plane::xy(60.0),
                Outline::rectangle([195.0, 30.0], [375.0, 90.0]),
                -120.0,
            )),
        ],
    ));
}

/// Failure 2-5 of campaign 3: a boss lying along X 1e-8 into the top of
/// the block it is joined to, decided to touch it; its corners on the line of
/// touch stood a hair off the top, and the boss's wall passed through it.
#[test]
fn seed_3175976_a_boss_a_hair_into_the_top_of_the_block_it_lies_on() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(
            Plane::yz(-1.5),
            Outline::rectangle([4.0, -0.5], [15.0, 10.5]),
            10.0,
        ),
        vec![Step::add(Leaf::prism(
            Plane::yz(-1.49999994),
            Outline::circle([9.5, 11.0], 0.50000001),
            -4.0,
        ))],
    ));
}

/// Failure 2-5 of campaign 3: a boss standing in a hole, touching its wall
/// inside but for 1e-8 through it; the corner at the top of the line of
/// touch stood off the hole's wall.
#[test]
fn seed_3244679_a_boss_a_hair_through_the_wall_of_the_hole_it_stands_in() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(
            Plane::xy(2.0),
            Outline::rectangle([3.0, -2.0], [14.0, 10.0]),
            3.0,
        ),
        vec![
            Step::cut(Leaf::prism(
                Plane::xy(2.00001),
                Outline::circle([6.0, 2.5], 3.0),
                4.0,
            )),
            Step::add(Leaf::prism(
                Plane::xy(2.0000102),
                Outline::circle([6.0, 4.5], 1.00000001),
                -8.0,
            )),
        ],
    ));
}

/// Failure 2-5 of campaign 3: a bore of radius 2.00000001 pokes 1e-8
/// through the stock it touches inside, and a third circle crosses both
/// within the band where the two stand within the tolerance of each other.
/// Its two crossings, one with each, were computed pair by pair and stood a
/// sliver apart, which no region of the bottom could enclose. Moved onto the
/// touch, the bore crosses the third circle where the stock does.
#[test]
fn seed_3150523_a_bore_a_hair_through_the_stock_crossed_by_a_third_circle_at_the_touch() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(Plane::xy(1.0), Outline::circle([6.0, 0.0], 2.5), 10.0),
        vec![
            Step::cut(Leaf::prism(
                Plane::xy(1.0),
                Outline::circle([6.0, 0.5], 2.00000001),
                9.0,
            )),
            Step::cut(Leaf::prism(
                Plane::xy(1.0),
                Outline::circle([6.0, 3.0], 0.5000002),
                10.0,
            )),
        ],
    ));
}

/// The same seed shrunk on the kernel of decision 7: a stock bored by a bore
/// touching it inside at its top, then a pin whose bottom dips 2e-7 into
/// both walls cut from above. The strips of the pin's wall between the
/// lines it crosses the two walls along stand within the tolerance of both
/// all across, and are wound beside them as the arena decided the pairs.
/// The pin's wall crosses the stock's and the bore's facing them, and a
/// point of it inside the stock had been read as standing under the
/// stock's wall, as though the two faced one way: the strip was put outside
/// the bored stock, and the listing did not close.
#[test]
fn seed_3150523_a_pin_dipping_into_a_stock_and_the_bore_touching_it_inside() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(Plane::xy(1.0), Outline::circle([6.0, 0.0], 2.5), -10.0),
        vec![
            Step::cut(Leaf::prism(
                Plane::xy(1.0),
                Outline::circle([6.0, 0.5], 2.0),
                -10.0,
            )),
            Step::cut(Leaf::prism(
                Plane::xy(1.0),
                Outline::circle([6.0, 3.0], 0.5000002),
                -10.0,
            )),
        ],
    ));
}

/// Failure 1-6 of campaign 3: a cut a hair off a post's side leaves a skin
/// of it 3e-7 thick, which a large block then joined grows the tolerance
/// past. The skin's two faces are twins, the post covers both, and the
/// kernel declined that as a tie. Two faces of one operand turning their
/// matter towards each other are now a skin taken for none, two turning it
/// away a crack taken for matter on both sides, which way read off the
/// pair's touch (decision 6).
#[test]
fn seed_3015242_a_post_sliced_a_hair_off_its_side_then_joined_to_a_block() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(Plane::xy(30.0), Outline::circle([270.0, 60.0], 30.0), -75.0),
        vec![
            Step::cut(Leaf::prism(
                Plane::xy(30.0),
                Outline::rectangle([240.0000003, 30.0], [300.0000003, 90.0]),
                -150.0,
            )),
            Step::add(Leaf::prism(
                Plane::xy(30.0),
                Outline::rectangle([15.0, -30.0], [255.0, 210.0]),
                300.0,
            )),
        ],
    ));
}

/// Found by the review of round 3: the same skin, given a block whose side
/// lies on the plane that sliced the post. The skin's wall stands beyond
/// that plane from the post's axis, between the two lines the plane cuts it
/// along, yet the pair was read as touching at the tolerance the block
/// grows, the wall on its axis's side: the skin was taken for a crack,
/// matter on both sides, and the block's side was dropped over it. The
/// kernel failed its own listing. Which way the skin turns is now read off
/// the pair as the post decided it, crossing, at the twins' place.
#[test]
fn seed_3015242_the_same_skin_given_a_block_on_the_plane_that_sliced_it() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(Plane::xy(30.0), Outline::circle([270.0, 60.0], 30.0), -75.0),
        vec![
            Step::cut(Leaf::prism(
                Plane::xy(30.0),
                Outline::rectangle([240.0000003, 30.0], [300.0000003, 90.0]),
                -150.0,
            )),
            Step::add(Leaf::prism(
                Plane::xy(30.0),
                Outline::rectangle([240.0000003, 0.0], [260.0, 120.0]),
                -350.0,
            )),
        ],
    ));
}

/// The same skin cut by that block.
#[test]
fn seed_3015242_the_same_skin_cut_by_a_block_on_the_plane_that_sliced_it() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(Plane::xy(30.0), Outline::circle([270.0, 60.0], 30.0), -75.0),
        vec![
            Step::cut(Leaf::prism(
                Plane::xy(30.0),
                Outline::rectangle([240.0000003, 30.0], [300.0000003, 90.0]),
                -150.0,
            )),
            Step::cut(Leaf::prism(
                Plane::xy(30.0),
                Outline::rectangle([240.0000003, 0.0], [260.0, 120.0]),
                -350.0,
            )),
        ],
    ));
}

/// Failure 1-2 of campaign 3: a bore touching a block's side inside leaves
/// a cusp of matter between the side and its wall, which a block cut from it
/// crosses 1e-5 from the touch. The kernel declined, the block covering both
/// strips of the cusp; it now takes the cusp for a skin and answers right.
/// Its triangles still cross below the cut, where the cut's corner on the
/// side, 1e-5 from the line of touch and within the band where side and wall
/// stand within the tolerance, was given the wall by distance (failure 1-2):
/// a strip of the wall lies on the side. Under the cut's floor the cusp runs
/// on past the cut's side, so the side's region and the wall's there are
/// the cusp a tangency leaves, kept as the exact geometry has it, not
/// strips decision 7 reads; what lies on the side is the wall's triangle
/// from the line of touch to that corner, a vertex of both faces 1e-5 into
/// the band.
#[test]
#[ignore = "band"]
fn seed_3130833_a_cusp_a_bore_touching_a_side_leaves_is_cut_a_hair_from_its_tip() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(
            Plane::xy(0.0),
            Outline::rectangle([1.5, 0.5], [8.5, 7.5]),
            5.0,
        ),
        vec![
            Step::cut(Leaf::prism(
                Plane::xy(0.0),
                Outline::circle([4.99999, 4.0], 3.5),
                10.0,
            )),
            Step::cut(Leaf::prism(
                Plane::xy(1.0),
                Outline::rectangle([3.0, 2.0], [5.0, 9.0]),
                10.0,
            )),
        ],
    ));
}

/// Found by the 120-second campaign from seed 50 000 000 once a touch decided
/// a hair apart was made exact: a bore touching a block's side 1e-8 inside,
/// its floor 1e-8 under the block's. The bore was moved onto the side, but
/// its rims were left where its own operand drew them, a hair off the moved
/// wall and a hair off the block's floor it was taken for: √2 hairs from the
/// circle the floor and the moved wall share, past the tolerance. The rim
/// and that circle stayed two, an arc of each a hair apart at one vertex,
/// and the kernel declined. An edge whose surfaces were taken for others or
/// moved is laid on the curve they share, each of the two having moved by
/// up to the tolerance.
#[test]
fn seed_50000046_a_bore_moved_onto_the_side_it_touches_takes_its_rims_along() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(
            Plane::xy(7.0),
            Outline::rectangle([10.0, 2.0], [12.0, 3.0]),
            5.0,
        ),
        vec![Step::cut(Leaf::prism(
            Plane::xy(6.99999999),
            Outline::circle([11.50000001, 2.5], 1.5),
            3.0,
        ))],
    ));
}

/// The same with a plane moved: a block whose side y = 0.5 a bore's wall
/// pokes through by 1e-8 is moved onto the touch, its floor 1e-8 under the
/// floor it is taken for. Its edge along both stayed where the block drew
/// it, √2 hairs from the corner the three planes fix, and the listing found
/// the edge's end off its vertex.
#[test]
fn seed_50008536_a_block_moved_onto_the_bore_it_touches_takes_its_edges_along() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(Plane::xy(6.0), Outline::circle([8.0, 5.0], 6.0), 5.0),
        vec![
            Step::cut(Leaf::prism(
                Plane::xy(6.0000002),
                Outline::circle([9.0, 5.0], 4.50000001),
                5.0,
            )),
            Step::add(Leaf::prism(
                Plane::xy(6.00000019),
                Outline::rectangle([3.0, 0.5], [8.0, 3.5]),
                5.0,
            )),
        ],
    ));
}

/// A pin under a block, its cap on the block's floor, its wall a hair from
/// touching the block's side from outside: decided to touch, the pin is
/// moved onto the side. Its rim was then read as no longer carried by its
/// own wall, the moved wall standing a rounding past the tolerance from it,
/// and stayed where the pin drew it, a hair off the corner the side's edge
/// touches the wall at: two corners and an edge's uses unbalanced. Which of
/// an edge's surfaces carry its curve is read off its operand as built.
#[test]
fn seed_50000012_a_pin_moved_onto_the_side_it_touches_from_outside_keeps_its_rim_on_its_wall() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(
            Plane::xy(1.0),
            Outline::rectangle([4.0, 2.0], [10.0, 8.0]),
            4.0,
        ),
        vec![Step::cut(Leaf::prism(
            Plane::xy(0.0),
            Outline::circle([7.0, 9.0], 0.99999999),
            1.0,
        ))],
    ));
}

/// Three blocks flush on two walls a hair off, each a tenth of the reach's
/// tolerance under it: the cut's edge along both walls stood where the cut
/// drew it, √2 hairs from the line the two walls it was taken for share.
#[test]
fn seed_50001263_a_slot_cut_flush_with_two_walls_a_hair_off_lies_on_their_line() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(
            Plane::yz(25.0),
            Outline::rectangle([15.0, 50.0], [38.0, 68.0]),
            45.0,
        ),
        vec![
            Step::add(Leaf::prism(
                Plane::yz(15.00001),
                Outline::rectangle([-12.4999999, -2.5], [32.5000001, 42.5]),
                45.0,
            )),
            Step::cut(Leaf::prism(
                Plane::yz(25.0000001),
                Outline::rectangle([25.0, 27.5], [32.5, 55.0]),
                90.0,
            )),
        ],
    ));
}

/// Failure 1-8 of campaign 3: a block given a pin of radius 0.5 whose axis
/// stands 6e-8 off the block's end, the pin cut by one of its radius on
/// the end, then a block cut through both. The two walls cross at a grazing
/// angle, and the cut's side, through their crossing, meets the cap at a
/// corner within the tolerance of both walls: read off its support, it lay
/// on the line they cross along, 3e-8 to 5e-6 away, which was cut there, and
/// two rulings left one vertex. A corner lies on a curve its support names
/// only where it stands within the tolerance of it.
#[test]
fn seed_3031475_a_block_cut_through_the_crossing_of_two_pins_a_hair_apart() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(
            Plane::xy(4.0),
            Outline::rectangle([7.0, 9.0], [13.0, 16.0]),
            5.0,
        ),
        vec![
            Step::add(Leaf::prism(
                Plane::xy(3.0),
                Outline::circle([13.00000006, 12.25], 0.5),
                5.0,
            )),
            Step::cut(Leaf::prism(
                Plane::xy(4.0),
                Outline::circle([13.0, 12.25], 0.5),
                5.0,
            )),
            Step::cut(Leaf::prism(
                Plane::xy(6.0),
                Outline::rectangle([4.0, 10.0], [9.0, 14.0]),
                5.0,
            )),
        ],
    ));
}

/// The same with two bars of one radius 1e-5 apart along a block's end,
/// then a block across them both: the body's own grazing crossing, decided
/// again, laid a corner of the third on the line the bars cross along.
#[test]
fn seed_3094005_two_bars_a_hair_apart_given_a_block_across_their_crossing() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(
            Plane::xz(30.0),
            Outline::rectangle([15.0, 40.0], [30.0, 80.0]),
            25.0,
        ),
        vec![
            Step::add(Leaf::prism(
                Plane::xz(30.00001),
                Outline::circle([30.0, 60.0], 5.0),
                50.0,
            )),
            Step::add(Leaf::prism(
                Plane::xz(34.9999999),
                Outline::circle([30.00001, 60.0], 5.0),
                48.0,
            )),
            Step::add(Leaf::prism(
                Plane::xy(35.0),
                Outline::rectangle([23.0, 53.0], [38.0, 68.0]),
                50.0,
            )),
        ],
    ));
}

/// The same with a bore 1e-5 inside a block's end, a post of its radius on
/// the end, then a block cut through their crossing.
#[test]
fn seed_3230678_a_block_cut_through_the_crossing_of_a_bore_and_a_post_a_hair_apart() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(
            Plane::xy(5.0),
            Outline::rectangle([10.0, 2.0], [17.0, 10.0]),
            1.0,
        ),
        vec![
            Step::cut(Leaf::prism(
                Plane::xy(5.0),
                Outline::circle([16.99999, 5.75], 2.5),
                9.0,
            )),
            Step::add(Leaf::prism(
                Plane::xy(5.0),
                Outline::circle([17.0, 5.75], 2.5),
                1.0,
            )),
            Step::cut(Leaf::prism(
                Plane::xy(5.0),
                Outline::rectangle([15.0, 4.0], [20.0, 8.0]),
                3.0,
            )),
        ],
    ));
}

/// The same without grazing: a bar touching a post's top along x = 30, its
/// cap 3e-7 short of where that line meets the post. The corner of the line
/// on the post and its corner on the cap, within the tolerance, were one,
/// lying by its support on the post's ruling through x = 30 and on its
/// ruling through the cap, 5e-7 apart where the cap meets the wall at a
/// slant: both were cut there, and two rulings left one vertex.
#[test]
fn seed_3275966_a_bar_touching_a_post_s_top_its_cap_a_hair_short_of_the_wall() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(
            Plane::xy(210.0),
            Outline::circle([120.0, 180.0], 150.0),
            135.0,
        ),
        vec![
            Step::add(Leaf::prism(
                Plane::xz(60.0),
                Outline::rectangle([30.0, 210.0], [210.0, 330.0]),
                240.0,
            )),
            Step::add(Leaf::prism(
                Plane::xz(59.9999997),
                Outline::circle([30.0, 180.0], 165.0),
                480.0,
            )),
        ],
    ));
}

/// Failure 2-2 of campaign 3: a bore's circle passing 1.3 tolerances from a
/// block's edge, its cap crossing there. The circle's corner on the cap was
/// taken onto the side's support without being merged with the block's
/// corner, and the edge was cut at two vertices at one parameter. Read off
/// the support alone, the edge lay on a corner standing past the tolerance
/// from it.
#[test]
fn seed_3040872_a_bore_a_little_over_the_tolerance_from_a_block_s_edge() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(
            Plane::yz(7.99999),
            Outline::rectangle([0.5, 6.0], [2.5, 9.0]),
            2.0,
        ),
        vec![Step::cut(Leaf::prism(
            Plane::yz(6.99999),
            Outline::circle([2.50000001, 7.5], 2.5),
            2.0,
        ))],
    ));
}

/// Failure 1-9 of campaign 3: two parallel cylinders of one radius 2e-7
/// apart crossing at a grazing angle, a side x = 8.5 standing inside the
/// band where they stand within the tolerance of each other. The side's
/// corner on both was taken as the end of the line they cross along, 9.9e-6
/// away, and the listing found the edge's end off its vertex.
#[test]
fn seed_3108519_a_side_inside_the_band_two_cylinders_a_hair_apart_cross_on() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(
            Plane::xy(1.0),
            Outline::rectangle([6.0, 7.0], [11.0, 12.0]),
            1.0,
        ),
        vec![
            Step::cut(Leaf::prism(
                Plane::xy(-2.0),
                Outline::rectangle([2.5, 7.0], [8.5, 14.5]),
                5.0,
            )),
            Step::add(Leaf::prism(
                Plane::xy(-3.0),
                Outline::circle([8.50001, 10.75], 3.0),
                9.0,
            )),
            Step::cut(Leaf::prism(
                Plane::xy(-4.0),
                Outline::circle([8.500009799999999, 10.75], 3.0),
                17.0,
            )),
            Step::add(Leaf::prism(
                Plane::xy(-3.0),
                Outline::rectangle([9.0, 9.0], [12.0, 12.0]),
                17.0,
            )),
        ],
    ));
}

/// The same with two pins 1e-5 apart and a side x = 2.5 5e-6 off the line
/// they cross along.
#[test]
fn seed_3173439_a_side_inside_the_band_two_pins_a_hair_apart_cross_on() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(
            Plane::xz(5.5),
            Outline::rectangle([1.5, 10.0], [2.5, 16.0]),
            9.0,
        ),
        vec![
            Step::cut(Leaf::prism(
                Plane::xz(5.49999),
                Outline::rectangle([0.0, 11.0], [4.0, 15.0]),
                9.0,
            )),
            Step::add(Leaf::prism(
                Plane::xz(5.4999902),
                Outline::circle([2.49999, 13.0], 0.5),
                7.0,
            )),
            Step::cut(Leaf::prism(
                Plane::xz(6.4999902),
                Outline::circle([2.5, 13.0], 0.5),
                6.0,
            )),
            Step::cut(Leaf::prism(
                Plane::xy(4.0),
                Outline::rectangle([8.0, 10.0], [14.0, 15.0]),
                6.0,
            )),
        ],
    ));
}

/// Found by the 120-second campaign from seed 50 000 000 (failure 1-6): a
/// post cut by a cylinder of its radius 5e-8 aside leaves a crescent, its
/// two walls crossing at a grazing angle along two rulings — decided to
/// cross at the post's tolerance, 4.8e-8. A block cut from its tip grows
/// the reach, and at 5.5e-8 the two walls were one: decided apart, so no
/// corner lay on both, and the crescent's own corners were cut into neither
/// rim. A pair of surfaces one operand alone carries is now decided at that
/// operand's tolerance, as it stood when it was made.
///
/// The cut's radius, read off a corner and a centre, is 22.499999999999996,
/// and that rounding moves the line the walls cross along 1.6e-6 from
/// halfway between their axes, along the band where they stand within the
/// tolerance of each other. The block's side x = 15 passes halfway, and on
/// both walls the strip between its line and the crescent's own is a twin
/// the post covers on both sides, straddling where the walls truly cross:
/// measured at the strip's point, which wall stands above could not be
/// read, and the kernel declined. Two walls crossing stand one above the
/// other all along each stretch between the lines they were decided to
/// cross along, read at its middle, where they stand furthest apart: the
/// twins are a skin, and the strip goes.
#[test]
fn seed_50002029_a_crescent_a_hair_wide_cut_at_its_tip_by_a_block_further_out() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(Plane::xy(28.0), Outline::circle([15.0, 20.0], 22.5), 10.0),
        vec![
            Step::cut(Leaf::prism(
                Plane::xy(28.0),
                Outline::circle([15.00000005, 20.0], 22.5),
                20.0,
            )),
            Step::cut(Leaf::prism(
                Plane::xy(25.0),
                Outline::rectangle([15.0, 35.0], [55.0, 45.0]),
                25.0,
            )),
        ],
    ));
}

/// The same crescent on a pin, given a block that swallows it.
#[test]
fn seed_50002633_a_crescent_a_hair_wide_swallowed_by_a_block_further_out() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(Plane::xz(40.0), Outline::circle([35.0, 40.0], 2.5), 10.0),
        vec![
            Step::cut(Leaf::prism(
                Plane::xz(44.99999),
                Outline::circle([35.00000005, 40.0], 2.5),
                20.0,
            )),
            Step::add(Leaf::prism(
                Plane::xy(35.0),
                Outline::rectangle([-8.0, -23.0], [48.0, 33.0]),
                15.0,
            )),
        ],
    ));
}

/// The same with two posts 3e-7 apart joined, then cut by a block whose
/// reach grows the tolerance past their gap: taken for one at the new
/// tolerance, the posts' walls were decided apart, and no corner of the
/// union lay on both.
#[test]
fn seed_50006672_two_posts_a_hair_apart_joined_then_cut_by_a_block_further_out() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(Plane::xy(90.0), Outline::circle([30.0, 0.0], 15.0), 165.0),
        vec![
            Step::add(Leaf::prism(
                Plane::xy(60.0),
                Outline::circle([29.9999997, 0.0], 15.0),
                165.0,
            )),
            Step::cut(Leaf::prism(
                Plane::xy(90.0),
                Outline::rectangle([0.0, 210.0], [240.0, 435.0]),
                165.0,
            )),
        ],
    ));
}

/// Failure 1-6 of campaign 3: two bars of one radius 1e-7 apart joined, then
/// a block whose reach grows the tolerance past their gap.
#[test]
fn seed_3137406_two_bars_a_hair_apart_joined_then_given_a_block_further_out() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(Plane::xz(30.0), Outline::circle([32.5, 40.0], 15.0), 45.0),
        vec![
            Step::add(Leaf::prism(
                Plane::xz(30.0),
                Outline::circle([32.4999999, 40.0], 15.0),
                90.0,
            )),
            Step::add(Leaf::prism(
                Plane::xy(30.0),
                Outline::rectangle([28.0, -10.0], [68.0, 30.0]),
                180.0,
            )),
        ],
    ));
}

/// Found by the review of round 3: the same body, which the kernel answered
/// and verified, given a block apart that grows the reach again. The pair of
/// bars was decided at the scale the body it came in had — the scale of the
/// block, which had already grown past their gap — and the kernel declined
/// its own answer. A body now keeps, for each surface, the scale of the
/// operation that brought it in, and a pair of its surfaces is read at the
/// later of the two, the operation that decided it.
#[test]
fn seed_3137406_the_same_bars_and_block_then_given_a_block_apart() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(Plane::xz(30.0), Outline::circle([32.5, 40.0], 15.0), 45.0),
        vec![
            Step::add(Leaf::prism(
                Plane::xz(30.0),
                Outline::circle([32.4999999, 40.0], 15.0),
                90.0,
            )),
            Step::add(Leaf::prism(
                Plane::xy(30.0),
                Outline::rectangle([28.0, -10.0], [68.0, 30.0]),
                180.0,
            )),
            Step::add(Leaf::prism(
                Plane::xy(0.0),
                Outline::rectangle([290.0, 290.0], [300.0, 300.0]),
                10.0,
            )),
        ],
    ));
}

/// The same with the block apart coming before the block, and its reach
/// grown past the bars' gap when the block is joined.
#[test]
fn seed_3137406_the_same_bars_given_a_block_apart_then_the_block() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(Plane::xz(30.0), Outline::circle([32.5, 40.0], 15.0), 45.0),
        vec![
            Step::add(Leaf::prism(
                Plane::xz(30.0),
                Outline::circle([32.4999999, 40.0], 15.0),
                90.0,
            )),
            Step::add(Leaf::prism(
                Plane::xy(0.0),
                Outline::rectangle([290.0, 290.0], [300.0, 300.0]),
                10.0,
            )),
            Step::add(Leaf::prism(
                Plane::xy(30.0),
                Outline::rectangle([28.0, -10.0], [68.0, 30.0]),
                180.0,
            )),
        ],
    ));
}

/// Found by the review of round 3, a regression of the scale each surface
/// came in at: a bar joined beside a block, touching its side, then a block
/// apart growing the reach, then a post joined a hair from touching the bar
/// outside. The post and the bar are made to touch by moving the bar, the
/// first operand's, by a little under the new tolerance; read again at the
/// scale the bar came in at, the moved bar no longer touched the side but
/// crossed it along two lines, and the kernel declined. A surface the
/// boolean moves is the boolean's: its pairs are decided at its scale.
#[test]
fn a_bar_beside_a_block_then_a_block_apart_then_a_post_a_hair_from_the_bar() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(
            Plane::xy(0.0),
            Outline::rectangle([0.0, 0.0], [20.0, 20.0]),
            10.0,
        ),
        vec![
            Step::add(Leaf::prism(
                Plane::yz(0.0),
                Outline::circle([22.0, 5.0], 2.0),
                20.0,
            )),
            Step::add(Leaf::prism(
                Plane::xy(0.0),
                Outline::rectangle([290.0, 290.0], [300.0, 300.0]),
                10.0,
            )),
            Step::add(Leaf::prism(
                Plane::xy(0.0),
                Outline::circle([10.0, 27.0000002], 3.0),
                15.0,
            )),
        ],
    ));
}

/// Found by the review of round 3, and failing before it: a bar joined
/// beside a block, touching its side, then a post joined a hair into the
/// bar, which decision 2 makes touch it at one point. That point is where
/// the bar's wall is read, the middle of its one region: the post's wall
/// passes through it, a ray from there is taken inside the post, and the
/// bar's whole wall was dropped. A region's point standing on a corner of
/// its surface, which a surface of the other operand touches there, is
/// read again further along its chord.
#[test]
fn a_bar_beside_a_block_given_a_post_touching_it_where_its_wall_is_read() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(
            Plane::xy(0.0),
            Outline::rectangle([0.0, 0.0], [20.0, 20.0]),
            10.0,
        ),
        vec![
            Step::add(Leaf::prism(
                Plane::yz(0.0),
                Outline::circle([22.0, 5.0], 2.0),
                20.0,
            )),
            Step::add(Leaf::prism(
                Plane::xy(0.0),
                Outline::circle([10.0, 24.99999999], 1.0),
                15.0,
            )),
        ],
    ));
}

/// The same with two pins, and a block cut through them.
#[test]
fn seed_3248399_two_pins_a_hair_apart_joined_then_cut_by_a_block_further_out() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(Plane::yz(20.0), Outline::circle([5.0, 50.0], 2.5), 23.0),
        vec![
            Step::add(Leaf::prism(
                Plane::yz(19.999999),
                Outline::circle([5.0000001, 50.0], 2.5),
                45.0,
            )),
            Step::cut(Leaf::prism(
                Plane::xy(20.0),
                Outline::rectangle([3.0, -13.0], [38.0, 23.0]),
                90.0,
            )),
        ],
    ));
}

/// Failure 1-9 of campaign 3, its reach grown by the last block: two posts
/// of one radius 1e-7 apart joined, then cut by a block reaching further.
#[test]
fn seed_3069912_two_posts_a_hair_apart_joined_then_cut_by_a_block_further_out() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(Plane::xy(10.0), Outline::circle([2.5, 7.5], 12.5), 28.0),
        vec![
            Step::add(Leaf::prism(
                Plane::xy(10.0),
                Outline::circle([2.4999999, 7.5], 12.5),
                20.0,
            )),
            Step::cut(Leaf::prism(
                Plane::xy(0.0),
                Outline::rectangle([-10.0, -5.0], [30.0, 35.0]),
                110.0,
            )),
        ],
    ));
}

/// Failure 1-6 of campaign 3, found by the triangles: two posts 5e-8 apart
/// joined, then cut by a block reaching further; a lune between their rings
/// was left on the bottom.
#[test]
fn seed_3033847_two_posts_a_hair_apart_joined_leave_no_lune_on_their_floor() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(Plane::xy(0.0), Outline::circle([27.5, 10.0], 20.0), 35.0),
        vec![
            Step::add(Leaf::prism(
                Plane::xy(-5.0),
                Outline::circle([27.50000005, 10.0], 20.0),
                35.0,
            )),
            Step::cut(Leaf::prism(
                Plane::xy(35.0),
                Outline::rectangle([40.0, 15.0], [50.0, 53.0]),
                25.0,
            )),
        ],
    ));
}

/// Failure 1-2 of campaign 3: the crescent two bores of one radius 1e-7
/// apart leave, cut by a third cylinder near its tip. Both walls of the
/// crescent cover the twins the cut leaves there, and the kernel declined:
/// the pair was not decided to touch. Two parallel cylinders of one operand
/// stand one above the other wherever they stand further apart than
/// rounding, which the crescent's walls do at the twins' place.
#[test]
fn seed_3113289_the_crescent_of_two_bores_a_hair_apart_cut_near_its_tip() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(Plane::yz(32.5), Outline::circle([25.0, 40.0], 5.0), 30.0),
        vec![
            Step::cut(Leaf::prism(
                Plane::yz(32.5),
                Outline::circle([25.0000001, 40.0], 5.0),
                30.0,
            )),
            Step::cut(Leaf::prism(
                Plane::yz(32.499999),
                Outline::circle([35.0, 25.0], 22.5),
                60.0,
            )),
        ],
    ));
}

/// Failure 2-2 of campaign 3: a block given a post, then a box cut whose
/// corner line stands 2.4e-7 inside the post's wall, its side 3e-7 off the
/// place the wall crosses the other side. The post's bottom rim crosses the
/// box's two sides 5e-7 apart, past the tolerance, and each crossing lay
/// within it of the other side: two corners on the same four surfaces, and
/// a sliver of the wall between them crossed its next triangle. The place
/// the box's corner line meets the post's bottom, fixed by three planes,
/// is now pooled before the crossings found on a wall, and both crossings
/// are merged into it.
#[test]
fn seed_3108592_a_box_cut_whose_corner_line_stands_a_hair_inside_a_post() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(
            Plane::xy(180.0),
            Outline::rectangle([165.0, 195.0], [375.0, 405.0]),
            75.0,
        ),
        vec![
            Step::add(Leaf::prism(
                Plane::xy(210.0),
                Outline::circle([270.0, 135.0], 75.0),
                45.0,
            )),
            Step::cut(Leaf::prism(
                Plane::yz(179.9999997),
                Outline::rectangle([90.0, 30.0], [135.0, 225.0]),
                150.0,
            )),
        ],
    ));
}

/// Failure 1-2 of campaign 3, found by the triangles: a post touching a
/// block's side from outside, and a second block whose side crosses that
/// side 0.83 tolerances from the line of touch. The two lines are one, and
/// its corners were left where each was found: the post's on the touch,
/// the second block's on its planes, a hair across, so that the edge leaned
/// between them and the side's triangles crossed the post's. A corner on
/// three planes and on a cylinder touching one of them stands where the
/// planes meet: they fix the line the touch was taken for.
#[test]
fn seed_3218403_a_post_touching_a_side_a_hair_from_where_a_block_crosses_it() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(
            Plane::yz(3.0),
            Outline::rectangle([-5.0, -4.0], [7.0, 8.0]),
            9.0,
        ),
        vec![
            Step::add(Leaf::prism(
                Plane::xz(5.99999999),
                Outline::rectangle([2.0, 1.0], [7.0, 5.0]),
                4.0,
            )),
            Step::add(Leaf::prism(
                Plane::xy(2.0),
                Outline::circle([1.0, 6.0], 2.0),
                6.0,
            )),
        ],
    ));
}

/// The same with a bore touching the end of a block 3e-7 inside, where a
/// second block's end crosses the first's.
#[test]
fn seed_3151875_a_bore_touching_a_block_s_end_a_hair_from_where_another_crosses_it() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(
            Plane::xz(195.0),
            Outline::rectangle([255.0, 90.0], [480.0, 330.0]),
            195.0,
        ),
        vec![
            Step::add(Leaf::prism(
                Plane::xz(194.99999),
                Outline::rectangle([285.0, 120.0], [540.0, 300.0]),
                195.0,
            )),
            Step::cut(Leaf::prism(
                Plane::xz(195.0000006),
                Outline::circle([479.9999997, 210.0], 90.0),
                390.0,
            )),
        ],
    ));
}

/// Failure 1-2 of campaign 3, found by the triangles, its largest group: a
/// block given a second block beside it, then a bore touching the first's
/// side y = 12 1e-7 from where the second's side x = 11.5 crosses it, inside
/// the band where the side and the bore's wall stand within the tolerance
/// of each other.
///
/// Understood and left: the line x = 11.5 on the side and the bore's ruling
/// on x = 11.5, 3e-15 apart, are one line, on the side, the second block's
/// side and the wall, and its corners stand on the side. The strip of the
/// wall between that line and the line of touch, and the strip of the side
/// between the same two lines, hold a skin 3e-15 thick, and the wall's
/// triangles there lie on the side's. They are twins, but not bounded by
/// the same arcs: the second block ends at z = 11 and the line with it,
/// while the side's region runs on into the band above and the wall's all
/// round. Decision 6 asks for regions bounded by the same arcs; decision 7
/// reads a region standing within the tolerance of the other surface all
/// across, and neither strip is a region: each is the part of a region
/// whose point stands beyond the band, decided there — kept, both, as the
/// exact geometry has them, a cusp of matter 3e-15 thick that the second
/// block ends. Dropping it wants both regions parted where the line ends,
/// along an arc across the band the boolean has no curve for.
#[test]
#[ignore = "band"]
fn seed_3156716_a_bore_touching_a_side_a_hair_from_where_a_second_block_s_side_crosses_it() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(
            Plane::xy(6.0),
            Outline::rectangle([9.0, 6.0], [15.0, 12.0]),
            6.0,
        ),
        vec![
            Step::add(Leaf::prism(
                Plane::xy(6.0),
                Outline::rectangle([7.0, 7.5], [11.5, 12.5]),
                5.0,
            )),
            Step::cut(Leaf::prism(
                Plane::xy(6.0),
                Outline::circle([11.5000001, 10.5], 1.5),
                6.0,
            )),
        ],
    ));
}

/// Failure 1-9 of campaign 3: two pins of one radius 6e-8 apart leave a
/// crescent whose walls cross at a grazing angle along a line lying on the
/// plane both touch, between their two lines of touch; a block's floor on
/// that plane then cuts it.
///
/// The strip of the first pin's wall between its line of touch and the
/// crossing has no twin bounded by the same arcs — the floor runs past the
/// crescent's caps — and its point stands a tenth of a femtometre off the
/// floor: no ray told whether the block wraps it, and the kernel declined.
/// The strip stands within the tolerance of the floor all across: the block
/// wraps it as the floor's matter lies, the pin standing below the floor it
/// touches (decision 7).
#[test]
fn seed_3089284_the_crescent_of_two_pins_a_hair_apart_cut_by_a_floor_both_touch() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(Plane::xz(1.5), Outline::circle([5.0, 4.0], 1.0), 5.0),
        vec![
            Step::cut(Leaf::prism(
                Plane::xz(1.50000001),
                Outline::circle([5.00000006, 4.0], 1.0),
                10.0,
            )),
            Step::cut(Leaf::prism(
                Plane::xz(2.5),
                Outline::rectangle([4.0, 3.0], [10.0, 11.0]),
                10.0,
            )),
        ],
    ));
}

/// Failure 2-1 of campaign 3: a bar bored along its length by a bore whose
/// wall touches the bar's top from inside, 1.8e-6 off the line where a
/// wider bar joined to it touches the top too, and touches the bore inside
/// along a third line 3.6e-6 off — three lines in one band, each pair
/// decided apart. The strip of the top between the bore's line and the wide
/// bar's stands within a hundredth of a femtometre of the wide bar's wall
/// all across, and no ray told whether the wide bar wraps it: the kernel
/// declined. The wide bar wraps it as its wall's matter lies, the top
/// standing outside the wall it touches (decision 7), and the strip of the
/// wide bar's wall between its two lines of touch lies between the bore and
/// the top, inside the cusp they leave.
#[test]
fn seed_3192987_a_bar_bored_under_its_top_joined_to_a_wider_bar_touching_both() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(
            Plane::yz(-89.99999),
            Outline::rectangle([255.0, 195.0], [435.0, 375.0]),
            570.0,
        ),
        vec![
            Step::cut(Leaf::prism(
                Plane::yz(-89.99999),
                Outline::circle([344.9999982, 315.0], 60.0),
                1140.0,
            )),
            Step::add(Leaf::prism(
                Plane::yz(-60.0),
                Outline::circle([345.0, 285.0], 90.0),
                255.0,
            )),
        ],
    ));
}

/// Failure 1-9 of campaign 3, the same band joined: two pins of one radius
/// 6e-8 apart, then a block whose floor both touch cut from above. The strip
/// of the first pin's wall between its line of touch and the line the pins
/// cross along stands within the tolerance of the floor all across, its
/// point a tenth of a femtometre under it, and the kernel declined. The
/// block wraps it as the floor's matter lies, the pin standing below the
/// floor it touches (decision 7).
#[test]
fn seed_3239627_two_pins_a_hair_apart_joined_then_cut_by_a_floor_both_touch() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(Plane::xz(3.0), Outline::circle([1.0, 1.0], 2.0), 8.0),
        vec![
            Step::add(Leaf::prism(
                Plane::xz(3.9999999),
                Outline::circle([1.00000006, 1.0], 2.0),
                9.0,
            )),
            Step::cut(Leaf::prism(
                Plane::xy(8.0),
                Outline::rectangle([-6.0, -2.0], [6.0, 9.0]),
                -5.0,
            )),
        ],
    ));
}

/// Failure 2-1 of campaign 3, as drawn: two pins of one radius 6e-8 apart
/// joined beside a block, then a pin cut from them whose wall touches both
/// at their tops from outside. The strip of the second pin's wall between
/// its line of touch with the cut and the line the two pins cross along
/// stands within the tolerance of the cut's wall all across, its point a
/// femtometre under it, and no ray told whether the cut wraps it: the
/// kernel declined. The cut wraps it as its wall's matter lies, the pin
/// standing outside the cut it touches (decision 7).
#[test]
fn seed_3024043_two_pins_a_hair_apart_cut_by_a_pin_touching_both_at_their_tops() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(Plane::xy(7.0), Outline::circle([5.0, 0.0], 4.0), 5.5),
        vec![
            Step::add(Leaf::prism(
                Plane::xy(6.0),
                Outline::rectangle([0.0, 6.0], [3.5, 9.5]),
                2.5,
            )),
            Step::add(Leaf::prism(
                Plane::xy(7.0),
                Outline::circle([4.99999994, 0.0], 4.0),
                5.5,
            )),
            Step::cut(Leaf::prism(
                Plane::xy(7.0),
                Outline::circle([4.99999994, 4.5], 0.5),
                6.5,
            )),
        ],
    ));
}

/// Failure 4-3 of campaign 4: a post cut by a post of its radius whose axis
/// stands 5.0000000058e-8 off, a rounding more than the tolerance at a reach
/// of fifty. Not one surface, the two were taken to touch inside and moved
/// onto each other as two, and the floor's rims stood on both a rounding
/// apart, a ring no point could be found inside. Two parallel cylinders of
/// one radius within twenty tolerances are one surface (decision 8).
#[test]
fn seed_4178972_a_post_cut_by_a_post_of_its_radius_a_tolerance_aside() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(Plane::xy(10.0), Outline::circle([17.5, 30.0], 20.0), 5.0),
        vec![Step::cut(Leaf::prism(
            Plane::xy(10.0),
            Outline::circle([17.50000005, 30.0], 20.0),
            10.0,
        ))],
    ));
}

/// From a campaign of profiles, decision 8: a post cut by a slot of its
/// radius whose first cap stands 5e-8 along the slot from the post's axis.
/// The two walls crossed along two rulings at a grazing angle, and the
/// triangles of the floor crossed there. The cap is the post's wall now;
/// the slot's sides, which the move slides along, stay where they were made.
#[test]
fn seed_80503332_a_post_cut_by_a_slot_of_its_radius_whose_cap_stands_a_hair_along() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(Plane::xy(20.0), Outline::circle([10.0, 0.0], 15.0), 25.0),
        vec![Step::cut(Leaf::prism(
            Plane::xy(15.0),
            Outline::slot([10.00000005, 0.0], [35.00000005, 0.0], 15.0),
            25.0,
        ))],
    ));
}

/// The same across: a slot bored at its end by a circle of its radius 6e-8
/// across its axis, then cut by a block. The circle is moved onto the cap.
#[test]
fn seed_80504197_a_slot_bored_at_its_end_a_hair_across_then_cut_by_a_block() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(
            Plane::xy(1.0),
            Outline::slot([7.0, 6.0], [4.0, 6.0], 1.5),
            7.0,
        ),
        vec![
            Step::cut(Leaf::prism(
                Plane::xy(1.0),
                Outline::circle([4.0, 6.00000006], 1.5),
                7.0,
            )),
            Step::cut(Leaf::prism(
                Plane::xy(1.0),
                Outline::rectangle(
                    [2.5000099999999996, 5.999999963535174e-8],
                    [5.50001, 7.50000006],
                ),
                13.0,
            )),
        ],
    ));
}

/// A rounded block bored at a corner by a hole of the corner's radius 1e-5
/// across, thirty-three tolerances at a reach of three hundred, then given a
/// cylinder lying across it: the kernel declined.
///
/// Understood and left: with a hair of fifty tolerances, decision 8 took the
/// hole for the corner's wall and it held. The hair is twenty, which no
/// line of measure crossing the moved wall at a slant can see: the two walls
/// stay two, crossing at a grazing angle within a hair of the sides the
/// corner touches, and the kernel declines — failure 1-9, the band.
#[test]
#[ignore = "band"]
fn seed_80506598_a_rounded_block_bored_a_hair_off_its_corner_then_given_a_cylinder_across() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(
            Plane::xy(60.0),
            Outline::rounded([120.0, 120.0], [195.0, 315.0], 15.0),
            180.0,
        ),
        vec![
            Step::cut(Leaf::prism(
                Plane::xy(30.0),
                Outline::circle([135.0, 134.99999], 15.0),
                225.0,
            )),
            Step::add(Leaf::prism(
                Plane::yz(120.0),
                Outline::circle([150.0, 30.0], 120.0),
                105.0,
            )),
        ],
    ));
}

/// A block whose rounded pocket is filled again by a rounded block 6e-8
/// aside, its corners each a hair off the pocket's: the kernel declined.
/// The whole outline is moved onto the pocket's, its sides with its
/// corners, and it fills the pocket but for its shorter end.
#[test]
fn seed_80505156_a_rounded_pocket_filled_again_by_a_rounded_block_a_hair_aside() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(
            Plane::xy(-2.0),
            Outline::rectangle([10.0, 9.0], [12.0, 13.0]),
            9.0,
        ),
        vec![
            Step::cut(Leaf::prism(
                Plane::xy(-3.0),
                Outline::rounded([10.0, 9.0], [12.0, 13.0], 1.0),
                9.0,
            )),
            Step::add(Leaf::prism(
                Plane::xy(-3.0),
                Outline::rounded([9.99999994, 9.0], [11.99999994, 12.5], 1.0),
                9.0,
            )),
        ],
    ));
}

/// A ring filled by a disc of its bore's radius a tolerance aside: a crescent
/// of the bore was lost, and the body came out short of matter by a volume
/// any line saw. The disc's wall is the bore's.
#[test]
fn seed_80511593_a_ring_filled_by_a_disc_a_tolerance_off_its_bore() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(
            Plane::xy(15.0),
            Outline::ring([0.0, 10.0], 27.5, 17.5),
            18.0,
        ),
        vec![Step::add(Leaf::prism(
            Plane::xy(14.99999995),
            Outline::circle([5.0000000000000004e-8, 10.0], 17.5),
            35.0,
        ))],
    ));
}

/// A block pocketed by a rounded rectangle 1e-8 aside, then bored at the
/// pocket's corner. The corner's wall touches the block's bottom side
/// exactly and its end side a hair off: moved along the bottom side onto
/// the end, it took its line of touch with the bottom side along, and left
/// the pocket's own corner on that line where it was drawn, a hair off the
/// line the bore then cut. A wall carrying corners of its operand is never
/// slid along a touch that holds exactly: it is left a hair off the other.
#[test]
fn seed_80501596_a_rounded_pocket_a_hair_aside_bored_at_its_corner() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(
            Plane::xy(8.0),
            Outline::rectangle([4.0, 4.0], [10.0, 12.0]),
            -4.0,
        ),
        vec![
            Step::cut(Leaf::prism(
                Plane::xy(7.0),
                Outline::rounded([4.00000001, 4.0], [10.00000001, 12.0], 2.5),
                5.0,
            )),
            Step::cut(Leaf::prism(
                Plane::xy(7.0),
                Outline::circle([6.5, 6.5], 2.5),
                -7.0,
            )),
        ],
    ));
}

/// A block bored by a hole touching its side and 1e-5 off its floor, then
/// cut by a bore of its radius touching both, 1e-5 aside: seven hundred
/// tolerances at a reach of fourteen.
///
/// Understood and left: beyond decision 8's hair, which a line of measure
/// crossing the moved wall at a slant would see, the two walls stay two.
/// They cross along two rulings at an angle of four millionths, within a
/// hair of the side both touch, and the kernel declines: failure 1-9, the
/// band of two walls of one radius a few hundred tolerances apart.
#[test]
#[ignore = "band"]
fn seed_80505830_a_hole_bored_again_seven_hundred_tolerances_aside_where_both_touch_a_side() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(
            Plane::yz(5.0),
            Outline::rectangle([7.0, 2.0], [13.0, 9.0]),
            5.0,
        ),
        vec![
            Step::cut(Leaf::prism(
                Plane::yz(5.0),
                Outline::circle([9.75, 4.75001], 2.75),
                5.0,
            )),
            Step::cut(Leaf::prism(
                Plane::yz(4.99999994),
                Outline::circle([9.75, 4.75], 2.75),
                9.0,
            )),
        ],
    ));
}

/// The same with a profile: a slot given a disc of its radius 1e-5 across
/// its cap, five hundred tolerances at a reach of fifteen, then a block
/// over the cap. Understood and left, as for seed 80505830: the disc and
/// the cap stay two walls crossing at a grazing angle.
#[test]
#[ignore = "band"]
fn seed_80509593_a_slot_given_a_disc_five_hundred_tolerances_across_its_cap() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(
            Plane::xz(1.0),
            Outline::slot([7.0, 3.0], [13.0, 3.0], 2.0),
            10.0,
        ),
        vec![
            Step::add(Leaf::prism(
                Plane::xz(1.0),
                Outline::circle([13.0, 2.99999], 2.0),
                10.0,
            )),
            Step::add(Leaf::prism(
                Plane::xy(1.0),
                Outline::rectangle([11.0, 1.0], [15.0, 7.0]),
                10.0,
            )),
        ],
    ));
}

/// A slot given a disc of its radius a hundred tolerances off its cap, 1e-5
/// at a reach of a hundred, standing far above it. Taken for the cap with a
/// hair of a hundred tolerances, the disc was moved by 1e-5, and a line of
/// measure crossing its wall at a slant of a sixth saw it moved by 1.3e-4,
/// more than the room along a line. Twenty tolerances is the hair: the two
/// walls stay two and cross, as they did before decision 8.
#[test]
fn seed_80502201_a_disc_a_hundred_tolerances_off_a_slot_s_cap_stays_where_it_was_drawn() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(
            Plane::xy(25.0),
            Outline::slot([30.0, 10.0], [30.0, 35.0], 15.0),
            40.0,
        ),
        vec![Step::add(Leaf::prism(
            Plane::xy(20.00000005),
            Outline::circle([30.0, 34.99999], 15.0),
            80.0,
        ))],
    ));
}

/// Found by the review of round 5: a block bored, then bored again by a
/// hole of its radius 2.4e-6 aside, forty-eight tolerances at a reach of
/// fifty. Taken for the first bore with a hair of fifty tolerances, the
/// second was moved onto it by its whole offset, and a line of measure
/// crossing the wall by a tangent, at a slant of a nineteenth, saw the move
/// nineteen times over at each of its two crossings: half again the room
/// along a line. The hair is twenty tolerances, which no line the rules
/// hold can see; two bores further apart stay two, and cross.
#[test]
fn a_block_bored_twice_forty_eight_tolerances_apart_is_bored_where_both_were_drawn() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(
            Plane::xy(0.0),
            Outline::rectangle([0.0, 0.0], [50.0, 50.0]),
            40.0,
        ),
        vec![
            Step::cut(Leaf::prism(
                Plane::xy(0.0),
                Outline::circle([22.125, 25.0], 10.0),
                40.0,
            )),
            Step::cut(Leaf::prism(
                Plane::xy(0.0),
                Outline::circle([22.125, 24.9999976], 10.0),
                40.0,
            )),
        ],
    ));
}

/// A block bored by a hole touching two of its sides but for 1e-5 off one,
/// then the hole filled by a post touching both: thirty-seven tolerances at
/// a reach of two hundred and seventy.
///
/// Understood and left: with a hair of fifty tolerances, decision 8 took the
/// post for the hole, and it filled it. The hair is twenty, which no line of
/// measure can see the merge through: the post and the hole stay two walls
/// of one radius crossing at a grazing angle where both touch a side, and
/// the kernel declines — failure 1-9, the band.
#[test]
#[ignore = "band"]
fn seed_80509744_a_hole_a_hair_off_a_side_filled_by_a_post_touching_it() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(
            Plane::xz(60.0),
            Outline::rectangle([90.0, 90.0], [270.0, 255.0]),
            60.0,
        ),
        vec![
            Step::cut(Leaf::prism(
                Plane::xz(60.0),
                Outline::circle([105.0, 105.00001], 15.0),
                120.0,
            )),
            Step::add(Leaf::prism(
                Plane::xz(60.0),
                Outline::circle([105.0, 105.0], 15.0),
                120.0,
            )),
        ],
    ));
}

/// Failure 4-1 of campaign 4: a post of radius 0.50000001 between two
/// parallel sides of the body a unit apart, a hair inside touching each.
/// Moved onto one touch, it went twice as deep into the other, past the
/// tolerance, and the kernel declined. A cylinder between two parallel
/// planes it touches on opposite sides is moved midway between them, its
/// radius half their gap: both touches are exact.
#[test]
fn seed_4105585_a_post_a_hair_wider_than_the_gap_between_two_sides() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(
            Plane::xy(6.0),
            Outline::rectangle([7.0, 8.0], [14.5, 15.5]),
            3.0,
        ),
        vec![
            Step::add(Leaf::prism(
                Plane::xy(6.0),
                Outline::rectangle([13.5, 11.0], [15.5, 13.0]),
                10.0,
            )),
            Step::add(Leaf::prism(
                Plane::xy(5.0),
                Outline::circle([15.0, 11.75], 0.50000001),
                10.0,
            )),
        ],
    ));
}

/// The same with a cut, the second side a plane of the body whose face
/// stands far from the post: a touch of faces that never meet moved the
/// post and broke the touch beside it. Only the pairs whose faces' boxes
/// meet are moved onto a touch.
#[test]
fn seed_4071426_a_hole_a_hair_wider_than_a_bar_beside_a_plane_whose_face_is_far() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(
            Plane::xz(6.0),
            Outline::rectangle([10.0, 8.0], [17.0, 16.0]),
            4.5,
        ),
        vec![
            Step::add(Leaf::prism(
                Plane::xy(5.0),
                Outline::rectangle([3.5, -0.5], [4.5, 0.5]),
                10.0,
            )),
            Step::cut(Leaf::prism(
                Plane::xy(5.0),
                Outline::circle([4.0, 1.0], 0.50000001),
                10.0,
            )),
        ],
    ));
}

/// A post notched by a block, then given a bar of radius 15 touching the
/// notch's floor and ceiling exactly and the post's wall from inside 3e-7
/// off. Settled by its exact touches, the bar was left off the post. It
/// moves along the planes it touches, which keeps both touches, onto the
/// post.
#[test]
fn seed_4146145_a_bar_touching_two_planes_exactly_is_moved_along_them_onto_a_post() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(
            Plane::xy(120.0),
            Outline::circle([120.0, 210.0], 165.0),
            285.0,
        ),
        vec![
            Step::cut(Leaf::prism(
                Plane::yz(150.0),
                Outline::rectangle([300.0, 180.0], [360.0, 210.0]),
                300.0,
            )),
            Step::add(Leaf::prism(
                Plane::yz(150.0),
                Outline::circle([359.9999997, 195.0], 15.0),
                -45.0,
            )),
        ],
    ));
}

/// Failure 4-2 of campaign 4: a bar of radius 105 cut by a post, then by a
/// second post of radius 105.0000003 whose axis crosses the bar's. The
/// node they meet at moved the bar, the first operand's, to the post's
/// radius, and the bar's own corners and edges stayed on its old wall. A
/// cylinder carrying the first operand's corners is never moved: the post
/// is, to the bar's radius.
#[test]
fn seed_4048009_a_bar_cut_by_a_post_of_its_radius_but_for_a_hair_keeps_its_wall() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(
            Plane::yz(180.0),
            Outline::circle([210.0, 30.0], 105.0),
            45.0,
        ),
        vec![
            Step::cut(Leaf::prism(
                Plane::xy(30.0),
                Outline::circle([270.0, 210.0], 135.0),
                300.0,
            )),
            Step::cut(Leaf::prism(
                Plane::xy(30.0),
                Outline::circle([240.0, 210.0], 105.0000003),
                300.0,
            )),
        ],
    ));
}

/// A bore 5e-8 from touching a boss inside along X and 5e-8 from touching a
/// block's side along Y. Moved once, onto the side, it left the touch with
/// the boss open, and the line they touch along was laid a hair off both.
/// A surface moves onto each of its touches in turn, so long as no move
/// parts it from a touch it had.
#[test]
fn seed_4176492_a_bore_a_hair_from_two_touches_square_to_each_other() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(
            Plane::yz(-5.0),
            Outline::rectangle([15.0, 45.0], [27.5, 62.5]),
            43.0,
        ),
        vec![
            Step::add(Leaf::prism(
                Plane::xy(35.0),
                Outline::circle([10.0, 25.0], 5.0),
                43.0,
            )),
            Step::cut(Leaf::prism(
                Plane::xy(35.0),
                Outline::circle([12.5, 25.0], 2.49999995),
                42.4999997,
            )),
        ],
    ));
}
