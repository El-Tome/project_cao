//! What the campaigns of `random_exact_solids.rs` found wrong in the exact
//! kernel of #498 — its boolean, not its triangles — each failure shrunk and
//! kept as the test it was fixed against, or, when it is understood and not
//! fixed, ignored with its diagnosis.
//!
//! The tangency band's findings are gathered in `the_tangency_band.rs`.
//!
//! The turned campaigns of #533 (`random_turned_solids.rs`, 6 October,
//! `docs/exact-kernel-failures.md`) are named here by family, one seed for
//! each, the rest of each family listed in the journal. The night's campaign
//! through the application's body is then sorted by the place in the code at
//! fault, several seeds to each family, its key at the head of the reason:
//! `kernel-drawing` and `kernel-declines`. The two families of a wall a hair
//! thin turned on the flats, `flats-ends` and `flats-join`, hold since
//! laying makes such a wall nothing, and the exact kernel turns the
//! section; `rounded-a-hair` holds since a straight run of no length is
//! left out of a raise.
//!
//! The second round of #536 sorts the slanted campaign through the
//! application's body the same way, every seed of it named: `laying-declines`
//! where laying gives up a section with a run a hair long and the flats it
//! falls back on break, `triangles` where a body that lists itself on its
//! geometry and holds every line is drawn open, crossing or short, and
//! `kernel-declines` where the boolean declines or mislists. Most of
//! `laying-declines` holds since a section laid to no area is nothing, and a
//! slant whose end a run brings back to within the tolerance of it ends where
//! that run does.
//!
//! The third round of #536 names the seeds that held at `ae20fa9` and fail
//! after round 2, each the case as drawn where its shrunk core fails at
//! `ae20fa9` too, sorted by the place in the code at fault:
//! `carried-one-with` where a slide is refused for carrying a wall one with
//! the first operand's onto it, `cone-axis-unmet` where a wall is refused a
//! move for a coaxial cone it never meets, and `triangles-sliver` where two
//! triangles cross across a sliver a hair wide. All three hold after round 3:
//! `carried-one-with` since a carried surface that stays one with the first
//! operand's and ends no further from it is allowed, `cone-axis-unmet` since a
//! cone holds a wall about its axis only where faces of the two meet on the
//! circle they cross along, and `triangles-sliver` since the triangles of two
//! faces folded onto each other across an edge are cut the other way.

// The drawing, the promise and the checks are shared with the campaigns;
// this file uses its own part of them.
#[allow(dead_code, unused_imports)]
mod random_solids;

use random_solids::{Axis, Case, Leaf, Outline, Plane, Section, Step};

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
/// line the bore then cut. A wall carrying corners of its operand, slid
/// along a touch that holds exactly, takes them along: the pocket moves
/// with it, its end side and its other corner too.
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

/// Found by the review of round 5, from seed 82504133 of a campaign of
/// profiles: a rounded block notched at a corner, the notch's floor 1e-5
/// above the block's bottom, then cut by a rounded rectangle whose bottom
/// is the notch's floor and whose side stands 3e-7 off the block's, six
/// tenths of a tolerance. The corner's wall touches its own bottom exactly
/// and the block's side a hair off. Not slid along its bottom onto the
/// side, since it carries its operand's corners, it was left a hair off,
/// and the kernel declined, where round 3 slid it and held. Slid, the wall
/// takes its operand's corners along, as decision 8 takes them, and the
/// side, the other corner and the lines between with it.
#[test]
fn seed_82504133_a_notched_rounded_block_cut_by_a_rounded_rectangle_a_hair_off_its_side() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(
            Plane::yz(-60.0),
            Outline::rounded([270.0, 180.0], [315.0, 390.0], 15.0),
            60.0,
        ),
        vec![
            Step::cut(Leaf::prism(
                Plane::yz(-60.000006),
                Outline::rectangle([270.0, 180.00001], [300.0, 210.00001]),
                120.0,
            )),
            Step::cut(Leaf::prism(
                Plane::yz(-60.0),
                Outline::rounded([270.0000003, 180.00001], [495.0000003, 390.00001], 15.0),
                240.0,
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

/// A hole a hair inside its outline's far side, a wall 1e-8 thick, turned
/// 270 degrees. The exact kernel declined the section, the levels of the
/// wall's two sides being one, and the flats turned it with the wall and
/// left an end facing the wrong way along it. Laying now holds to its own
/// levels: the wall is not there, the hole opens onto the outline as a
/// notch, and the exact kernel turns it.
#[test]
fn seed_533400107_a_wall_a_hair_thin_turned_part_way_on_the_flats() {
    random_solids::holds_through_the_application(&Case::new(
        Leaf::turned(
            Plane::xy(1.0),
            Axis::second(0.0),
            Section::bands(-2.0, &[[7.5, 0.0, 5.0]]).with_holes(&[([3.0, 2.5], [5.0, 4.99999999])]),
            270.0,
        ),
        vec![],
    ));
}

/// Was ignored as: plane-a-hair: a block's side 1.8e-6 from the axis of a
/// cylinder turned before it cuts the cylinder a hair from its widest rulings;
/// two faces drawn crossing, as with the cylinder raised. Holds since the
/// triangles of two faces folded onto each other across an edge are cut the
/// other way, round 3 of #536.
#[test]
fn seed_533300839_a_block_s_side_a_hair_from_a_turned_cylinder_s_axis() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(
            Plane::yz(150.0),
            Outline::circle([180.0, 300.0], 105.0),
            -30.0,
        ),
        vec![
            Step::cut(Leaf::turned(
                Plane::yz(135.0),
                Axis::second(255.0000018),
                Section::bands(135.0, &[[330.0, 0.0, 15.0]]),
                360.0,
            )),
            Step::add(Leaf::prism(
                Plane::xy(30.0),
                Outline::rectangle([75.0, 255.0], [165.0, 345.0]),
                240.0,
            )),
        ],
    ));
}

#[test]
#[ignore = "band: a bore turned 3e-7 under the half width of the block it runs through, a hair from touching both sides; the kernel refuses to answer along a line, and through the application's body the last turn is declined as unverified"]
fn seed_533301022_a_turned_bore_a_hair_from_touching_a_block_s_sides() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(
            Plane::xy(30.0),
            Outline::rectangle([90.0, 30.0], [150.0, 90.0]),
            -75.0,
        ),
        vec![
            Step::cut(Leaf::turned(
                Plane::yz(120.0),
                Axis::second(60.0),
                Section::bands(-105.0, &[[120.0, 0.0, 29.9999997]]),
                360.0,
            )),
            Step::add(Leaf::prism(
                Plane::xy(210.0),
                Outline::rectangle([0.0, 60.0], [180.0, 120.0]),
                180.0,
            )),
            Step::add(Leaf::turned(
                Plane::yz(120.0),
                Axis::second(0.0),
                Section::bands(-105.0, &[[120.0, -60.0, -30.0]]),
                360.0,
            )),
        ],
    ));
}

#[test]
#[ignore = "plane-a-hair: a turned bore's end and a post's end 6e-6 from the faces they meet; the kernel's triangles leave the body open, which the application declines as undrawn"]
fn seed_533341432_a_turned_bore_and_a_post_ending_a_hair_from_a_face() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(
            Plane::yz(210.0),
            Outline::circle([90.0, 210.0], 15.0),
            150.0,
        ),
        vec![
            Step::cut(Leaf::turned(
                Plane::xz(119.999994),
                Axis::second(285.0),
                Section::bands(165.0, &[[90.0, 0.0, 30.0]]),
                360.0,
            )),
            Step::cut(Leaf::prism(
                Plane::xz(164.99999400000002),
                Outline::circle([285.0, 210.0], 15.0),
                90.0,
            )),
        ],
    ));
}

/// Was ignored as: kernel: a stepped shaft turned about an axis leaning 1e-7,
/// cut by a cylinder square to it; two faces drawn crossing where the shaft's
/// shoulder meets the cut. Holds since the triangles of two faces folded onto
/// each other across an edge are cut the other way, round 3 of #536.
#[test]
fn seed_533335255_a_shaft_turned_about_an_axis_leaning_a_hair() {
    random_solids::holds_exactly(&Case::new(
        Leaf::turned(
            Plane::xy(30.0),
            Axis::second(0.0).leaning(1e-7),
            Section::bands(-15.0, &[[5.0, 0.0, 30.0], [8.0, 0.0, 40.0]]),
            360.0,
        ),
        vec![Step::cut(Leaf::turned(
            Plane::yz(30.0),
            Axis::second(5.0),
            Section::bands(25.0, &[[18.0, -15.0, 10.0]]),
            360.0,
        ))],
    ));
}

/// line-of-touch: a cylinder turned about a line lying on the tangent plane
/// of a ring's outer wall, then a post through that line of touch; the
/// kernel's triangles left the body open, alike with the cylinder raised as
/// a prism, until a line took every sample standing on it within rounding
/// (`tessellation/sampling/touches.rs`).
#[test]
fn seed_533330824_a_turned_cylinder_whose_axis_touches_a_ring_s_wall() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(
            Plane::xy(40.0),
            Outline::ring([30.0, 30.0], 30.0, 17.5),
            38.0,
        ),
        vec![
            Step::cut(Leaf::turned(
                Plane::xz(60.0),
                Axis::first(59.0),
                Section::bands(0.0, &[[60.0, 0.0, 12.5]]),
                360.0,
            )),
            Step::cut(Leaf::prism(
                Plane::xy(39.0),
                Outline::circle([30.0, 60.0], 12.5),
                40.0,
            )),
        ],
    ));
}

/// line-of-touch: a cylinder turned square to a post, touching it at a
/// point, then a block across both; declined as undrawn through the
/// application's body, alike with the cylinder raised as a prism, until a
/// line took every sample standing on it within rounding
/// (`tessellation/sampling/touches.rs`).
#[test]
fn seed_533413234_a_turned_cylinder_touching_a_post_at_a_point() {
    random_solids::holds_through_the_application(&Case::new(
        Leaf::prism(Plane::xy(6.0), Outline::circle([7.0, 2.0], 1.5), 2.0),
        vec![
            Step::add(Leaf::turned(
                Plane::xy(7.0),
                Axis::second(11.5),
                Section::bands(0.0, &[[4.0, 0.0, 3.0]]),
                360.0,
            )),
            Step::add(Leaf::prism(
                Plane::xz(2.0),
                Outline::rectangle([2.0, 5.0], [10.0, 10.0]),
                9.0,
            )),
        ],
    ));
}

/// A sector turned 270 degrees cut about the axis of a cylinder turned
/// before it; the ends of the cut, planes holding that axis, meet the
/// cylinder along its rulings, and every read of one region along its chord
/// was a tie. It is settled read either side of the chord, as 5365202020 is.
#[test]
fn seed_533328889_a_partial_turn_cut_about_the_axis_of_a_turned_cylinder() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(Plane::xy(4.0), Outline::circle([6.0, 4.0], 1.0), 3.0),
        vec![
            Step::add(Leaf::turned(
                Plane::xy(5.5),
                Axis::first(5.5),
                Section::bands(4.5, &[[3.0, 0.0, 0.5]]),
                360.0,
            )),
            Step::cut(Leaf::turned(
                Plane::xy(5.5),
                Axis::first(5.5),
                Section::bands(5.5, &[[1.0, -1.0, 0.0]]),
                270.0,
            )),
        ],
    ));
}

/// Was ignored as: kernel: a sector turned a hundredth of a degree about the
/// axis of a cylinder raised along it; the sector's ends, planes holding that
/// axis 1.7e-4 rad apart, meet the cylinder along its rulings and two faces are
/// drawn crossing, alike with the cylinder turned; at a degree it holds. Holds
/// since the triangles of two faces folded onto each other across an edge are
/// cut the other way, round 3 of #536.
#[test]
fn seed_533603007_a_sliver_turned_about_the_axis_of_a_cylinder() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(
            Plane::xy(4.0),
            Outline::slot([6.0, 9.0], [6.0, 5.5], 0.5),
            -3.0,
        ),
        vec![
            Step::add(Leaf::prism(
                Plane::xz(5.0),
                Outline::circle([7.0, 2.5], 0.5),
                -4.5,
            )),
            Step::add(Leaf::turned(
                Plane::xy(2.5),
                Axis::second(7.0),
                Section::bands(7.0, &[[1.0, 0.0, 2.0]]),
                0.01,
            )),
        ],
    ));
}

/// A band on its axis, its hole 2e-7 under its outer radius, turned a
/// quarter: the flats lost the strip a hair thin at their ends. Laid, the
/// hole is a notch in the outline, turned by the exact kernel.
#[test]
fn seed_533617180_a_wall_a_hair_thin_on_the_axis_turned_a_quarter_on_the_flats() {
    random_solids::holds_through_the_application(&Case::new(
        Leaf::turned(
            Plane::xy(0.0),
            Axis::first(0.0),
            Section::bands(0.0, &[[7.0, 0.0, 5.0]]).with_holes(&[([2.5, 3.0], [6.5, 4.9999998])]),
            90.0,
        ),
        vec![],
    ));
}

/// A band below its axis, its hole 1e-7 inside its far edge, turned a
/// quarter: the flats refused the strip from both ends as a sliver. Laid,
/// the hole is a notch, turned by the exact kernel.
#[test]
fn seed_533612358_a_wall_a_hair_thin_below_the_axis_turned_a_quarter_on_the_flats() {
    random_solids::holds_through_the_application(&Case::new(
        Leaf::turned(
            Plane::xy(2.0),
            Axis::first(0.0),
            Section::bands(2.0, &[[5.5, -7.5, 0.0]])
                .with_holes(&[([3.0, -7.4999999], [5.0, -4.5])]),
            90.0,
        ),
        vec![],
    ));
}

/// A band across its axis, its hole 2e-8 inside the far edge below it,
/// turned a quarter: the side below went to the flats and lost the strip
/// at its ends. Laid, that side's hole is a notch, turned by the exact
/// kernel.
#[test]
fn seed_533622662_a_wall_a_hair_thin_below_an_axis_the_band_crosses_turned_a_quarter_on_the_flats()
{
    random_solids::holds_through_the_application(&Case::new(
        Leaf::turned(
            Plane::xy(2.0),
            Axis::first(0.0),
            Section::bands(2.5, &[[4.0, -2.5, 1.0]])
                .with_holes(&[([4.0, -2.49999998], [6.0, -0.5])]),
            90.0,
        ),
        vec![],
    ));
}

/// A band across its axis, its hole 1e-7 under the edge on one side,
/// turned a quarter: that side's ends were open on the flats before the two
/// sides were joined. Laid, the hole is a notch, turned by the exact
/// kernel.
#[test]
fn seed_533653685_a_wall_a_hair_thin_across_the_axis_turned_a_quarter_on_the_flats() {
    random_solids::holds_through_the_application(&Case::new(
        Leaf::turned(
            Plane::xy(3.0),
            Axis::first(0.0),
            Section::bands(1.0, &[[3.0, -4.0, 2.5]]).with_holes(&[([2.5, 1.0], [3.5, 2.4999999])]),
            90.0,
        ),
        vec![],
    ));
}

/// A band on its axis, its hole 2e-7 under its outer radius, turned a tenth
/// of a degree short of whole: the flats laid ends and lost the strip a
/// hair thin there. Laid, the hole is a notch, turned by the exact kernel.
#[test]
fn seed_533669075_a_wall_a_hair_thin_turned_a_tenth_of_a_degree_short_of_whole_on_the_flats() {
    random_solids::holds_through_the_application(&Case::new(
        Leaf::turned(
            Plane::xy(6.0),
            Axis::first(0.0),
            Section::bands(4.0, &[[7.5, 0.0, 9.5]]).with_holes(&[([5.0, 8.0], [5.5, 9.4999998])]),
            359.9,
        ),
        vec![],
    ));
}

/// A band across its axis, its hole 1e-8 under the edge on one side, turned
/// whole. Each side alone was closed on the flats; their join left the
/// hair-thin wall open, the flats' boolean splitting faces a hair apart
/// into slivers it refuses. The exact kernel declined the side, the
/// levels of the wall's two faces being one, and laying now holds to that:
/// the wall is not there, the hole opens onto the outline as a notch, and
/// both sides are turned and joined exactly.
#[test]
fn seed_533626745_a_wall_a_hair_thin_across_the_axis_turned_whole_joined_on_the_flats() {
    random_solids::holds_through_the_application(&Case::new(
        Leaf::turned(
            Plane::xy(8.0),
            Axis::second(0.0),
            Section::bands(-2.0, &[[4.0, -3.0, 3.0]])
                .with_holes(&[([-0.5, 0.5], [0.5, 2.99999999])]),
            360.0,
        ),
        vec![],
    ));
}

/// A band on its axis, its hole 6e-8 under its outer radius, turned whole,
/// then cut by a block: the flats' cut left the wall a hair thin open.
/// Laid, the hole is a notch, and turn and cut are exact.
#[test]
fn seed_533638603_a_wall_a_hair_thin_turned_whole_then_cut_by_a_block_on_the_flats() {
    random_solids::holds_through_the_application(&Case::new(
        Leaf::turned(
            Plane::xy(8.0),
            Axis::second(0.0),
            Section::bands(2.0, &[[3.5, 0.0, 5.0]]).with_holes(&[([2.5, 3.5], [4.5, 4.99999994])]),
            360.0,
        ),
        vec![Step::cut(Leaf::prism(
            Plane::xy(1.0),
            Outline::rectangle([2.0, 2.0], [7.0, 8.0]),
            7.0,
        ))],
    ));
}

/// A band on its axis, its hole 1e-8 under its outer radius, turned whole,
/// then cut by a post: the flats' cut left it open. Laid, the hole is a
/// notch, and turn and cut are exact.
#[test]
fn seed_533654767_a_wall_a_hair_thin_turned_whole_then_cut_by_a_post_on_the_flats() {
    random_solids::holds_through_the_application(&Case::new(
        Leaf::turned(
            Plane::xy(2.0),
            Axis::first(0.0),
            Section::bands(4.0, &[[7.5, 0.0, 5.0]]).with_holes(&[([5.5, 1.5], [8.0, 4.99999999])]),
            360.0,
        ),
        vec![Step::cut(Leaf::prism(
            Plane::xy(-4.0),
            Outline::circle([8.0, 0.0], 5.0),
            12.0,
        ))],
    ));
}

/// A band below its axis, its hole 6e-8 inside its far edge, turned whole,
/// then cut by a block: the flats' cut left it open. Laid, the hole is a
/// notch, and turn and cut are exact.
#[test]
fn seed_533698627_a_wall_a_hair_thin_below_the_axis_turned_whole_then_cut_on_the_flats() {
    random_solids::holds_through_the_application(&Case::new(
        Leaf::turned(
            Plane::xy(2.0),
            Axis::second(0.0),
            Section::bands(2.0, &[[5.5, -5.5, 0.0]])
                .with_holes(&[([5.0, -5.49999994], [6.5, -3.0])]),
            360.0,
        ),
        vec![Step::cut(Leaf::prism(
            Plane::xy(3.0),
            Outline::rectangle([-7.0, 1.0], [8.0, 16.0]),
            12.0,
        ))],
    ));
}

/// Was ignored as: kernel-drawing: a cylinder turned whole, cut by a post whose
/// axis passes 1e-8 from the cylinder's; two faces drawn crossing, on the exact
/// kernel too: the kernel's hair families, reached by turned cylinders as by
/// raised ones: 70 of the 1 071 failures through the application's body, two
/// faces crossing or the body declined as undrawn. Holds since the triangles of
/// two faces folded onto each other across an edge are cut the other way, round
/// 3 of #536.
#[test]
fn seed_533695958_a_turned_cylinder_cut_by_a_post_whose_axis_passes_a_hair_from_its_own() {
    random_solids::holds_through_the_application(&Case::new(
        Leaf::turned(
            Plane::yz(5.0),
            Axis::first(0.0),
            Section::bands(6.0, &[[10.0, 0.0, 3.0]]),
            360.0,
        ),
        vec![Step::cut(Leaf::prism(
            Plane::xy(1.0),
            Outline::circle([5.00000001, 7.0], 1.0),
            4.0,
        ))],
    ));
}

/// Was ignored as: kernel-drawing: a tube turned about an axis leaning 1e-3
/// degrees, joined to a ring raised across it; two faces drawn crossing, on the
/// exact kernel too. Holds since the triangles of two faces folded onto each
/// other across an edge are cut the other way, round 3 of #536.
#[test]
fn seed_533719875_a_turned_tube_whose_axis_leans_a_thousandth_of_a_degree_joined_to_a_ring() {
    random_solids::holds_through_the_application(&Case::new(
        Leaf::turned(
            Plane::xy(7.0),
            Axis::first(0.0).leaning(0.001),
            Section::bands(2.0, &[[3.5, 0.5, 3.0]]),
            360.0,
        ),
        vec![Step::add(Leaf::prism(
            Plane::xy(7.0),
            Outline::ring([2.0, 2.0], 6.0, 1.5),
            6.0,
        ))],
    ));
}

/// kernel-drawing: a cylinder turned whole touching a post's wall at a
/// point, then a second post of the cylinder's radius through it, its axis
/// meeting the cylinder's. The second post's wall touches the first's along
/// a line, which the node of the two equal walls stands on: the node is a
/// vertex, the line an edge running past it. The line was sampled at its
/// ends alone, the wall's two loops met in the middle of a segment, and the
/// application declined the body as undrawn. A line now takes every sample
/// standing on it within rounding (`tessellation/sampling/touches.rs`).
#[test]
fn seed_533647113_a_turned_cylinder_lying_along_a_post_s_wall_is_drawn_closed() {
    random_solids::holds_through_the_application(&Case::new(
        Leaf::prism(Plane::xy(1.0), Outline::circle([3.0, 10.0], 2.5), 3.0),
        vec![
            Step::add(Leaf::turned(
                Plane::xy(3.0),
                Axis::first(15.0),
                Section::bands(-1.0, &[[8.0, 0.0, 2.5]]),
                360.0,
            )),
            Step::add(Leaf::prism(
                Plane::xy(-1.0),
                Outline::circle([3.0, 15.0], 2.5),
                7.0,
            )),
        ],
    ));
}

#[test]
#[ignore = "kernel-drawing: a cylinder turned whole, cut by a post and joined to a disc; the exact kernel's own triangles hold, the application's finer ones are drawn crossing (body/drawn.rs), as 533666445"]
fn seed_533779006_a_turned_cylinder_cut_and_joined_is_drawn_crossing_through_the_body_alone() {
    random_solids::holds_through_the_application(&Case::new(
        Leaf::turned(
            Plane::xz(5.0),
            Axis::first(4.0),
            Section::bands(0.0, &[[6.0, 0.0, 2.0]]),
            360.0,
        ),
        vec![
            Step::cut(Leaf::prism(
                Plane::xz(8.0),
                Outline::circle([3.0, 4.0], 2.0),
                6.0,
            )),
            Step::add(Leaf::prism(
                Plane::xy(1.0),
                Outline::circle([0.5, 6.0], 5.0),
                9.0,
            )),
        ],
    ));
}

/// Was ignored as: kernel-drawing: a ring raised from a plane 1e-8 off the
/// round numbers, by 4.50000002, cut by a lying post; shrunk to prisms alone,
/// two faces drawn crossing on the exact kernel: the kernel's own hair family,
/// not turning's. Holds since the triangles of two faces folded onto each other
/// across an edge are cut the other way, round 3 of #536.
#[test]
fn seed_533751095_a_ring_raised_a_hair_off_round_numbers_cut_by_a_lying_post() {
    random_solids::holds_through_the_application(&Case::new(
        Leaf::prism(
            Plane::xy(7.00000001),
            Outline::ring([8.0, 10.0], 8.0, 6.0),
            -4.50000002,
        ),
        vec![Step::cut(Leaf::prism(
            Plane::yz(-0.5),
            Outline::circle([6.0, 4.0], 2.5),
            8.5,
        ))],
    ));
}

/// A slot whose floor stands 1e-7 from the point where its lower cap touches
/// a turned cylinder inside, the cap a perpendicular wall. The slot's corner
/// there was taken onto the cylinder, the curve the cap and the cylinder meet
/// along passed 5e-8 from it, was not cut there, and ran on to the point of
/// touch under the floor; the body was declined as unverified. A line a
/// plane cuts the cap along, a hair inside the cylinder by less than a
/// coordinate holds, was rounded onto it and taken to touch it: it is
/// measured from the two radii now, as for 5365239969, and the curve is cut
/// where it crosses.
#[test]
fn seed_533631663_a_slot_a_hair_off_a_turned_cylinder_s_axis_plane_is_cut_where_its_cap_crosses() {
    random_solids::holds_through_the_application(&Case::new(
        Leaf::prism(
            Plane::xz(2.0000001),
            Outline::slot([6.0, 8.0], [6.0, 6.0], 3.0),
            -2.0,
        ),
        vec![Step::cut(Leaf::turned(
            Plane::xz(2.0),
            Axis::second(0.0),
            Section::bands(2.0, &[[4.0, -9.0, 0.0]]),
            360.0,
        ))],
    ));
}

/// The post crosses the shaft at a node but for 5e-9 of radius: the curve
/// the two meet along was laid on the post given the shaft's radius, while
/// the post itself was left where it stood, since growing it would lift it
/// off the shaft's end it rests on; the corners found on the post stood
/// three tolerances off that curve. The post grown is slid along the
/// shaft's axis by what it grew, and rests on the end still.
#[test]
fn seed_533609727_a_shaft_whose_two_bands_differ_by_a_hair_joined_to_a_post() {
    random_solids::holds_through_the_application(&Case::new(
        Leaf::turned(
            Plane::yz(2.0),
            Axis::first(0.0),
            Section::bands(1.0, &[[2.5, 0.0, 2.5], [0.5, 0.0, 2.50000001]]),
            360.0,
        ),
        vec![Step::add(Leaf::prism(
            Plane::xy(-5.0),
            Outline::circle([2.0, 3.5], 2.5),
            7.0,
        ))],
    ));
}

/// The block's top crosses the slit the turn leaves, 6e-8 from its axis,
/// where the slit's two ends stand a ten-thousandth of the tolerance apart:
/// one piece of surface, two planes crossing along the axis, which the
/// turn holds as a crack and which was a tie. Two planes are read one above
/// the other at the place, where they stand further apart than rounding.
#[test]
fn seed_533708173_a_turn_short_of_whole_joined_to_a_block_a_hair_off_its_axis() {
    random_solids::holds_through_the_application(&Case::new(
        Leaf::turned(
            Plane::xz(12.5),
            Axis::first(1.5),
            Section::bands(3.0, &[[8.0, 0.0, 1.0]]),
            359.9,
        ),
        vec![Step::add(Leaf::prism(
            Plane::xy(1.50000006),
            Outline::rectangle([0.0, 8.0], [4.0, 15.0]),
            -12.0,
        ))],
    ));
}

#[test]
#[ignore = "kernel-declines: a cylinder turned about an axis 2e-8 off a rounded block's side, its ends on the block's ends, which touch the corners' walls: the curve each corner's wall meets the cylinder along passes 4e-16 from the cylinder's end circle, inside the band of the end and the corner, with no corner, and the cylinder's overlay finds a region with no point inside it (brep/overlay/partition.rs); declined as a tie. So is 533620725"]
fn seed_533648596_a_turned_cylinder_a_hair_from_a_rounded_block_s_side_is_declined_as_a_tie() {
    random_solids::holds_through_the_application(&Case::new(
        Leaf::prism(
            Plane::xy(-2.0),
            Outline::rounded([8.0, 6.0], [12.5, 7.5], 0.5),
            7.0,
        ),
        vec![Step::cut(Leaf::turned(
            Plane::xy(1.0),
            Axis::first(7.50000002),
            Section::bands(8.0, &[[4.5, 0.0, 0.5]]),
            360.0,
        ))],
    ));
}

/// The slot's cap stands 4.99999997e-8 from the disc's wall, just under
/// the tolerance, and was taken for it; its side, which the slot made touch
/// the cap, stands 5.00000006e-8 from the wall, just over it, and crossed
/// the wall it should have touched. A wall taken for the first's a hair off
/// now moves its operand onto it, as decision 8 does further off.
#[test]
fn seed_533613392_a_disc_cut_by_a_slot_a_hair_off_its_centre_with_no_turn() {
    random_solids::holds_through_the_application(&Case::new(
        Leaf::prism(Plane::yz(5.0), Outline::circle([5.0, 25.0], 25.0), -45.0),
        vec![Step::cut(Leaf::prism(
            Plane::yz(5.0),
            Outline::slot([4.99999995, 25.0], [4.99999995, 22.5], 25.0),
            -45.0,
        ))],
    ));
}

#[test]
fn seed_533656030_a_rounded_square_a_hair_wider_than_its_corners_is_raised() {
    random_solids::holds_through_the_application(&Case::new(
        Leaf::prism(
            Plane::xy(3.0),
            Outline::rounded([12.49999999, 0.5], [13.5, 1.5], 0.5),
            1.0,
        ),
        vec![],
    ));
}

#[test]
fn seed_533729250_a_rounded_rectangle_a_hair_wider_than_a_slot_is_raised() {
    random_solids::holds_through_the_application(&Case::new(
        Leaf::prism(
            Plane::xy(7.0),
            Outline::rounded([3.99998999, 11.0], [4.99999, 15.0], 0.5),
            5.0,
        ),
        vec![],
    ));
}

#[test]
fn seed_533684645_a_large_rounded_rectangle_a_hair_wider_than_a_slot_is_raised() {
    random_solids::holds_through_the_application(&Case::new(
        Leaf::prism(
            Plane::xy(-90.0),
            Outline::rounded([-14.9999982, 75.0], [15.0000021, 210.0], 15.0),
            -210.0,
        ),
        vec![],
    ));
}

#[test]
fn seed_536211029_a_point_whose_tip_a_partial_turn_cuts_away_but_for_a_sector() {
    random_solids::holds_through_the_application(&Case::new(
        Leaf::turned(
            Plane::yz(7.0),
            Axis::second(0.0),
            Section::bands(3.0, &[[2.0, 0.0, 4.0]]).sloping_to(&[[0.0, 0.0]]),
            360.0,
        ),
        vec![Step::cut(Leaf::turned(
            Plane::yz(7.0),
            Axis::second(0.0).backwards(),
            Section::bands(-10.0, &[[8.0, -3.0, -0.0]]),
            270.0,
        ))],
    ));
}

/// A partial turn with a slanted run about an axis 1e-8 off a disc's, cut
/// into it: its cone was read about the disc's axis within the tolerance,
/// its end planes holding its own a hair off. It is moved onto the disc's
/// axis (decision 8).
#[test]
fn seed_536202582_a_partial_turn_a_hair_off_the_axis_of_a_disc_it_cuts() {
    random_solids::holds_through_the_application(&Case::new(
        Leaf::prism(Plane::xz(5.0), Outline::circle([8.0, 5.0], 4.5), 5.0),
        vec![Step::cut(Leaf::turned(
            Plane::xy(5.0),
            Axis::second(8.00000001).backwards(),
            Section::bands(-0.0, &[[0.5, 0.0, 4.5]]).sloping_to(&[[0.0, 4.0]]),
            270.0,
        ))],
    ));
}

/// A quarter turn with slanted runs joined to a cylinder of its axis: two
/// of its cones are each other's mirror, of one apex, as 5361118847.
#[test]
fn seed_536210335_a_slanted_quarter_turn_joined_to_a_cylinder_of_its_axis() {
    random_solids::holds_through_the_application(&Case::new(
        Leaf::turned(
            Plane::xy(3.0),
            Axis::second(0.0),
            Section::bands(4.0, &[[1.0, 0.0, 2.0], [2.0, 0.0, 1.0], [1.0, 0.0, 1.0]])
                .sloping_to(&[[0.0, 1.0], [0.0, 1.0], [0.0, 2.0]]),
            90.0,
        ),
        vec![Step::add(Leaf::turned(
            Plane::xy(3.0),
            Axis::second(0.0),
            Section::bands(10.0, &[[1.0, 0.0, 1.0]]),
            360.0,
        ))],
    ));
}

#[test]
fn seed_536101211_a_shaft_whose_top_slants_in_over_a_hair_of_height() {
    random_solids::holds_exactly(&Case::new(
        Leaf::turned(
            Plane::xy(0.0),
            Axis::second(0.0),
            Section::bands(1.0, &[[1.0, 0.0, 4.5], [2e-7, 0.0, 4.5]])
                .sloping_to(&[[0.0, 4.5], [0.0, 1.5]]),
            360.0,
        ),
        vec![],
    ));
}

#[test]
fn seed_536104721_a_shaft_whose_top_slants_in_over_a_hair_its_roots_rounded_into_one() {
    random_solids::holds_exactly(&Case::new(
        Leaf::turned(
            Plane::yz(4.0),
            Axis::second(0.0),
            Section::bands(-2.0, &[[2.4999998, 0.0, 4.5], [2e-7, 0.0, 4.5]])
                .sloping_to(&[[0.0, 4.5], [0.0, 2.0]]),
            360.0,
        ),
        vec![],
    ));
}

/// Was ignored as: kernel: a shaft whose top slants out, turned whole, cut by a
/// half turn about the same axis whose wall has the radius the slant starts
/// from; the ends of the half turn, planes holding that axis, meet the cone
/// along its rulings, and the body comes out open along the rim the wall rests
/// on. Not traced: the family of 533328889, which #533 left for the cones, or
/// the rim's touch. Holds since a face of a cone meeting a sample twice is laid
/// out on one angle for it, round 2 of #536.
#[test]
fn seed_536112193_a_half_turn_cut_about_the_axis_of_a_frustum() {
    random_solids::holds_exactly(&Case::new(
        Leaf::turned(
            Plane::yz(-2.0),
            Axis::second(0.0).backwards(),
            Section::bands(4.0, &[[2.0, 0.0, 6.0], [2.0, 0.0, 6.5]])
                .sloping_to(&[[0.0, 6.5], [0.0, 7.0]]),
            360.0,
        ),
        vec![Step::cut(Leaf::turned(
            Plane::xz(0.0),
            Axis::second(-2.0),
            Section::bands(-7.0, &[[1.0, 0.0, 6.5]]),
            180.0,
        ))],
    ));
}

/// A fin 1e-8 thick on the narrow end of a cone: laid onto the cone's end,
/// the profile turned back on itself and was declined, slanted, where a
/// straight one is bounded; the matter as laid is the cone alone.
#[test]
fn seed_5361100533_a_disc_a_hair_thick_on_the_narrow_end_of_a_cone() {
    random_solids::holds_exactly(&Case::new(
        Leaf::turned(
            Plane::xy(3.0),
            Axis::first(0.0),
            Section::bands(5.0, &[[3.49999999, 0.0, 5.0], [1e-8, 0.0, 5.0]])
                .sloping_to(&[[0.0, 3.0], [0.0, 5.0]]),
            360.0,
        ),
        vec![],
    ));
}

/// 5361100533's fin, its far side slanting from its rim back to the cone's
/// end over 2e-8: laid square, the corner it ends on is the cone's end.
#[test]
fn seed_5361134707_a_disc_a_hair_thick_on_the_narrow_end_of_a_cone_its_far_side_slanting() {
    random_solids::holds_exactly(&Case::new(
        Leaf::turned(
            Plane::xy(2.0),
            Axis::first(0.0),
            Section::bands(4.0, &[[3.99999998, 0.0, 3.0], [2e-8, 0.0, 3.0]])
                .sloping_to(&[[0.0, 2.0], [0.0, 2.0]]),
            360.0,
        ),
        vec![],
    ));
}

/// A shaft slanting in by 1e-7, a hair past the tolerance, before a cone:
/// the shoulder between them stands on a level of its own, and a corner a
/// run longer than the tolerance from a slant's end is not near it.
#[test]
fn seed_5361126605_a_shaft_slanting_in_by_a_hair_before_a_cone() {
    random_solids::holds_exactly(&Case::new(
        Leaf::turned(
            Plane::xy(3.0),
            Axis::second(0.0),
            Section::bands(0.5, &[[1.0, 0.0, 2.0], [1.0, 0.0, 2.0]])
                .sloping_to(&[[0.0, 1.9999999], [0.0, 1.0]]),
            360.0,
        ),
        vec![],
    ));
}

/// The same as 5361126605 before a point, the shaft slanting in by 2e-7.
#[test]
fn seed_5361120170_a_point_whose_shaft_slants_in_by_a_hair_before_it() {
    random_solids::holds_exactly(&Case::new(
        Leaf::turned(
            Plane::xy(7.0),
            Axis::second(0.0),
            Section::bands(-1.0, &[[0.5, 0.0, 1.5], [1.5, 0.0, 1.5]])
                .sloping_to(&[[0.0, 1.4999998], [0.0, 0.0]]),
            360.0,
        ),
        vec![],
    ));
}

/// A section 2e-7 long altogether, a wall a hair thin with no hole, which
/// laying makes nothing: declined, and counted apart as such a wall.
#[test]
fn seed_5361101048_a_cone_a_hair_long() {
    random_solids::holds_exactly(&Case::new(
        Leaf::turned(
            Plane::xy(-1.0),
            Axis::second(0.0),
            Section::bands(-2.0, &[[2e-7, -1.0, -0.0]]).sloping_to(&[[-5.0, -0.0]]),
            360.0,
        ),
        vec![],
    ));
}

/// 5361100533's fin, 2e-7 thick, through the application's body: the exact
/// kernel turns the cone, where the flats left it open along its rim.
#[test]
fn seed_5361217405_a_disc_a_hair_thick_on_the_narrow_end_of_a_cone_turned_on_the_flats() {
    random_solids::holds_through_the_application(&Case::new(
        Leaf::turned(
            Plane::xy(4.0),
            Axis::first(0.0),
            Section::bands(3.0, &[[3.9999998, 0.0, 8.5], [2e-7, 0.0, 8.5]])
                .sloping_to(&[[0.0, 7.5], [0.0, 8.5]]),
            360.0,
        ),
        vec![],
    ));
}

/// A hole 6e-8 under the rim of a shaft past a cone: laid onto the rim, it
/// opens onto it beside the slant as beside a square corner, where the flats
/// left the cut open.
#[test]
fn seed_5361201830_a_hole_a_hair_under_the_rim_of_a_shaft_past_a_cone_cut_by_a_block() {
    random_solids::holds_through_the_application(&Case::new(
        Leaf::turned(
            Plane::xy(-1.0),
            Axis::second(0.0),
            Section::bands(-0.5, &[[1.5, -5.0, -0.0], [3.0, -3.5, -0.0]])
                .sloping_to(&[[-3.5, -0.0], [-3.5, -0.0]])
                .with_holes(&[([1.5, -3.49999994], [2.0, -2.0])]),
            360.0,
        ),
        vec![Step::cut(Leaf::prism(
            Plane::xy(1.0),
            Outline::rectangle([2.5, -1.5], [7.5, 3.5]),
            3.0,
        ))],
    ));
}

#[test]
#[ignore = "not a cone: the reading lays the slope of 3e-7 level, onto the radius of the wide band, and the cutter, a wall 2.25e-7 wider about an axis 3e-7 off, is slid onto an inside touch along the line where the start plane of the turn passes; the straight form, a 240 degree turn of radius 25.000000075 cut so, is refused as unverified the same way. A hair of #533's cylinders"]
fn seed_5361130486_a_partial_turn_of_cones_cut_by_a_cylinder_a_hair_off_its_axis() {
    random_solids::holds_exactly(&Case::new(
        Leaf::turned(
            Plane::xy(30.0),
            Axis::first(0.0),
            Section::bands(
                25.0,
                &[
                    [12.5, 0.0, 25.0],
                    [17.5, 0.0, 5.0],
                    [20.0, 0.0, 5.0],
                    [2.5, 0.0, 25.0],
                ],
            )
            .sloping_to(&[[0.0, 25.0], [0.0, 5.0], [0.0, 5.0], [0.0, 25.0000003]]),
            240.0,
        ),
        vec![Step::cut(Leaf::turned(
            Plane::xy(30.0),
            Axis::first(3e-7),
            Section::bands(62.5, &[[15.0, -25.0000003, -0.0]]),
            360.0,
        ))],
    ));
}

/// A ring cut by a quarter turn of a cone about an axis 3e-7 off the
/// ring's: decided about the cone's axis within the tolerance, the corner where
/// the cone leaves the ring's wall was found twice, a hair apart along the
/// slope. The cone is now moved onto the ring's axis (decision 8).
#[test]
fn seed_5361108529_a_ring_cut_by_a_quarter_turn_of_a_cone_a_hair_off_its_axis() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(
            Plane::yz(-45.0),
            Outline::ring([210.0, 150.0], 120.0, 105.0),
            120.0,
        ),
        vec![Step::cut(Leaf::turned(
            Plane::xz(210.0),
            Axis::first(150.0000003),
            Section::bands(45.0, &[[30.0, 120.0, 180.0]]).sloping_to(&[[90.0, 180.0]]),
            90.0,
        ))],
    ));
}

/// A point standing on a disc 6e-8 off the axis of the ring it was raised
/// on: decision 8 took the disc onto the ring's axis, and the point, turned
/// about the disc's, met the ring a hair off its axis. It is moved onto it.
#[test]
fn seed_5361110937_a_point_on_a_disc_a_hair_off_the_axis_of_a_ring() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(Plane::xy(2.0), Outline::ring([8.0, 9.0], 6.0, 2.0), 7.0),
        vec![
            Step::add(Leaf::prism(
                Plane::xy(2.0),
                Outline::circle([7.99999994, 9.0], 2.0),
                9.0,
            )),
            Step::add(Leaf::turned(
                Plane::yz(7.99999994),
                Axis::second(9.0),
                Section::bands(10.0, &[[1.0, 0.0, 2.0]]).sloping_to(&[[0.0, 0.0]]),
                360.0,
            )),
        ],
    ));
}

/// A half turn of stepped cones joined to a block: two of its cones are
/// each other's mirror, of one apex, and a ruling the slit's plane cut from
/// one was laid on the other's, which it shares as a whole line. The corners
/// on the first then stood on the second, apart from the plane past its
/// apex, and fell off their rims.
#[test]
fn seed_5361118847_a_half_turn_of_stepped_cones_joined_to_a_block() {
    random_solids::holds_exactly(&Case::new(
        Leaf::turned(
            Plane::xy(-1.0),
            Axis::second(0.0),
            Section::bands(
                -3.5,
                &[
                    [0.5, -2.0, -0.0],
                    [2.0, -1.5, -0.0],
                    [1.5, -1.5, -0.0],
                    [1.0, -3.0, -0.0],
                    [0.5, -3.0, -0.0],
                ],
            )
            .sloping_to(&[
                [-1.5, -0.0],
                [-1.5, -0.0],
                [-1.5, -0.0],
                [-3.0, -0.0],
                [-3.5, -0.0],
            ]),
            180.0,
        ),
        vec![Step::add(Leaf::prism(
            Plane::xy(6.0),
            Outline::rectangle([3.0, 9.0], [11.0, 17.0]),
            5.0,
        ))],
    ));
}

/// A block cut by a quarter turn of stepped cones, two of them each
/// other's mirror, as 5361118847.
#[test]
fn seed_5361135092_a_block_cut_by_a_quarter_turn_of_stepped_cones() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(
            Plane::xy(-1.0),
            Outline::rectangle([4.0, 5.0], [6.0, 12.0]),
            8.0,
        ),
        vec![Step::cut(Leaf::turned(
            Plane::xy(1.0),
            Axis::second(0.0),
            Section::bands(
                2.0,
                &[
                    [2.5, 0.0, 7.0],
                    [2.0, 0.0, 4.5],
                    [1.5, 0.0, 5.0],
                    [2.5, 0.0, 1.5],
                    [1.5, 0.0, 3.5],
                    [0.5, 0.0, 3.0],
                ],
            )
            .sloping_to(&[
                [0.0, 4.5],
                [0.0, 4.5],
                [0.0, 5.0],
                [0.0, 1.5],
                [0.0, 3.5],
                [0.0, 3.5],
            ]),
            90.0,
        ))],
    ));
}

/// A quarter turn of cones about a leaning axis, cut by a cylinder, two
/// of its cones each other's mirror, as 5361118847.
#[test]
fn seed_5361102829_a_quarter_turn_of_cones_about_a_leaning_axis_cut_by_a_cylinder() {
    random_solids::holds_exactly(&Case::new(
        Leaf::turned(
            Plane::xy(5.0),
            Axis::first(0.0).leaning(60.0),
            Section::bands(
                1.0,
                &[[1.0, -2.0, -0.0], [3.0, -1.0, -0.0], [3.0, -2.0, -0.0]],
            )
            .sloping_to(&[[-1.0, -0.0], [-1.0, -0.0], [-5.0, -0.0]]),
            90.0,
        ),
        vec![Step::cut(Leaf::turned(
            Plane::xy(7.0),
            Axis::second(0.0).backwards(),
            Section::bands(2.0, &[[3.0, 0.0, 5.0]]),
            360.0,
        ))],
    ));
}

/// A half turn of stepped cones, two of them each other's mirror, joined
/// to a point of its axis it does not reach, as 5361118847.
#[test]
fn seed_5361201784_a_half_turn_of_stepped_cones_joined_to_a_point_of_its_axis() {
    random_solids::holds_through_the_application(&Case::new(
        Leaf::turned(
            Plane::xy(10.0),
            Axis::second(0.0),
            Section::bands(-15.0, &[[8.0, 0.0, 0.0]]).sloping_to(&[[0.0, 10.0]]),
            360.0,
        ),
        vec![Step::add(Leaf::turned(
            Plane::xy(20.0),
            Axis::second(0.0),
            Section::bands(
                10.0,
                &[
                    [2.5, 0.0, 17.5],
                    [12.5, 0.0, 5.0],
                    [7.5, 0.0, 5.0],
                    [2.5, 0.0, 5.0],
                ],
            )
            .sloping_to(&[[0.0, 15.0], [0.0, 5.0], [0.0, 5.0], [0.0, 7.5]]),
            180.0,
        ))],
    ));
}

/// A cylinder pocketed a hair off its axis, then cut by a ring of its axis
/// whose bore widens along a cone onto its wall: decision 8 moved the bore,
/// of the pocket's radius, onto the pocket's axis, and took the cone a hair
/// off the wall it meets. A move that takes a cone off a surface it was
/// decided with about one axis is not made.
#[test]
fn seed_5361127255_a_ring_whose_bore_widens_onto_a_wall_pocketed_a_hair_off_its_axis() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(Plane::xz(25.0), Outline::circle([40.0, 35.0], 27.5), 48.0),
        vec![
            Step::cut(Leaf::prism(
                Plane::xz(29.99999995),
                Outline::circle([39.9999999, 35.0], 17.5),
                8.0,
            )),
            Step::cut(Leaf::turned(
                Plane::yz(40.0),
                Axis::first(35.0).backwards(),
                Section::bands(12.5, &[[10.0, 27.5, 30.0], [2.5, 17.5, 30.0]])
                    .sloping_to(&[[17.5, 30.0], [17.5, 30.0]]),
                360.0,
            )),
        ],
    ));
}

#[test]
#[ignore = "cone-boolean, the grazing coaxial family: a cone of slope 5e-8 cut by a wall of its axis whose radius it crosses, the two within the tolerance of each other over a band a quarter of the length wide; declined as unverified. Left for Tom's word (the plan's 6.1)"]
fn seed_5361134935_a_cone_a_hair_from_a_wall_of_its_axis_cut_by_it() {
    random_solids::holds_exactly(&Case::new(
        Leaf::turned(
            Plane::xy(-0.5),
            Axis::first(4.5),
            Section::bands(-3.0, &[[3.5, 0.0, 2.5], [4.0, 0.0, 2.49999998]])
                .sloping_to(&[[0.0, 2.5], [0.0, 2.49999978]]),
            90.0,
        ),
        vec![Step::cut(Leaf::prism(
            Plane::yz(0.5),
            Outline::circle([4.5, -0.5], 2.49999998),
            8.0,
        ))],
    ));
}

#[test]
#[ignore = "cone-boolean, a rim a hair off: the cutter's cone starts 5e-8 outside the shaft's wall at its shoulder; its rim is laid on the shaft's circle there (decision 3), and the cone crosses the wall 1e-7 further along (decision 2), two circles on one pair; declined as a tie. Left for Tom's word with the grazing coaxial family"]
fn seed_5361120549_a_shaft_cut_by_a_quarter_turn_of_a_cone_whose_rim_stands_a_hair_off_its_shoulder()
 {
    random_solids::holds_exactly(&Case::new(
        Leaf::turned(
            Plane::xy(12.5),
            Axis::first(0.0),
            Section::bands(
                -10.0,
                &[
                    [5.0, 0.0, 7.5],
                    [2.5, 0.0, 12.5],
                    [5.0, 0.0, 15.0],
                    [20.0, 0.0, 12.5],
                    [12.5, 0.0, 7.5],
                ],
            )
            .sloping_to(&[
                [0.0, 7.5],
                [0.0, 15.0],
                [0.0, 15.0],
                [0.0, 12.5],
                [0.0, 7.5],
            ]),
            360.0,
        ),
        vec![Step::cut(Leaf::turned(
            Plane::xz(0.0),
            Axis::first(12.5),
            Section::bands(-2.5, &[[5.0, 0.0, 15.00000005]]).sloping_to(&[[0.0, 12.50000005]]),
            90.0,
        ))],
    ));
}

/// A pointed cone cut away by a coaxial cone a hair wider whose tip stands
/// 1e-8 off the axis, at the first one's apex. The two meet at the apex
/// alone, their slopes a hair apart, and stand within the tolerance of each
/// other all along their faces; from eba1fa2 on, no curve was laid on both,
/// and the rim of one at the cylinder's end stood beside the other's, 1e-8
/// apart, a region between them thinner than the tolerance: a tie. Only a
/// line, a ruling of a mirror, is kept off two such cones now.
#[test]
fn seed_5365211513_a_pointed_cone_cut_away_by_a_cone_of_its_apex_a_hair_wider() {
    random_solids::holds_exactly(&Case::new(
        Leaf::turned(
            Plane::xz(5.0),
            Axis::second(-3.0),
            Section::bands(2.0, &[[3.0, 0.0, 0.0], [3.0, 0.0, 4.5], [3.5, 0.0, 4.5]])
                .sloping_to(&[[0.0, 2.5], [0.0, 4.5], [0.0, 0.0]]),
            360.0,
        ),
        vec![Step::cut(Leaf::turned(
            Plane::xz(5.0),
            Axis::second(-3.0),
            Section::bands(8.0, &[[3.5, 0.0, 4.50000001]]).sloping_to(&[[0.0, 1e-8]]),
            360.0,
        ))],
    ));
}

/// A slot whose round end stands 1e-5 off a cone's axis, its arc a hair off the
/// wall of a rounded cut the cone's rim stands on: was drawn open along the
/// slot's side. The cone's group gathered a ray its rim took in contact, the
/// cut's wall took it, and the slot's arc, close to that wall, did not: the
/// sliver between the two was sampled apart and left uncut. The gathered rays
/// are now shared once more (round 2 of #536).
#[test]
fn seed_5365114896_a_slot_a_hair_off_a_cone_s_axis_is_drawn_closed() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(
            Plane::xz(240.0),
            Outline::slot([270.00001, 60.0], [270.00001, 150.0], 180.0),
            75.0,
        ),
        vec![
            Step::cut(Leaf::prism(
                Plane::xz(210.0),
                Outline::rounded([90.0, -120.0], [450.0, 240.0], 180.0),
                240.0,
            )),
            Step::add(Leaf::turned(
                Plane::xy(60.0),
                Axis::second(270.0),
                Section::bands(0.0, &[[30.0, 180.0, 195.0]]).sloping_to(&[[150.0, 195.0]]),
                360.0,
            )),
        ],
    ));
}

/// A turned shaft whose last band is 2e-8 long, laid as its end a tolerance
/// short of the wall of a cylinder about the axis of a quarter cone the body
/// was cut by. Decision 2 moved that cylinder onto the touch with the end, a
/// tolerance off the axis it shares with the cone, and the two, still
/// decided about one axis, met along a circle the moved wall stood off: the
/// step was declined as unverified. A wall about a cone's axis is no longer
/// moved off it.
#[test]
fn seed_5365205368_a_shaft_ending_on_a_band_a_hair_long_joined_to_a_quarter_cone() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(Plane::xz(-1.0), Outline::circle([3.0, 4.0], 3.0), -10.0),
        vec![
            Step::cut(Leaf::turned(
                Plane::yz(3.0),
                Axis::first(4.0),
                Section::bands(7.0, &[[2.0, -5.0, -3.0]]).sloping_to(&[[-5.0, -1.0]]),
                90.0,
            )),
            Step::add(Leaf::turned(
                Plane::xy(5.0),
                Axis::first(0.0),
                Section::bands(
                    -2.0,
                    &[[2.0, 0.0, 3.5], [0.99999998, 0.0, 2.5], [2e-8, 0.0, 2.5]],
                ),
                360.0,
            )),
            Step::add(Leaf::turned(
                Plane::xy(4.0),
                Axis::second(3.0),
                Section::bands(-3.0, &[[13.0, -2.0, -0.0]]),
                360.0,
            )),
        ],
    ));
}

/// A disc 1e-6 thick and 25 across turned about its edge: laid, it bounds no
/// area, and it was turned on the flats, which left its rim open. It is now
/// nothing.
#[test]
fn seed_5365201759_a_disc_a_hair_thick_turned_whole_is_nothing() {
    random_solids::holds_through_the_application(&Case::new(
        Leaf::turned(
            Plane::xy(40.0),
            Axis::first(0.0),
            Section::bands(27.499999, &[[1.0000000010279564e-6, 0.0, 25.0]]),
            360.0,
        ),
        vec![],
    ));
}

/// A tube whose outer wall tapers from 30 to 10, then a band 1e-7 long, then a
/// tube 15 to 27.5 across: laying declines the section, and the flats leave it
/// open along a rim.
#[test]
#[ignore = "laying-declines, found by round 2's triage: the band 1e-7 long is the only thing joining the tube before it to the tube after it, which do not overlap across it; laid to nothing, it leaves the matter in two pieces, which laying declines (turning/straight/bounded.rs, `contours`), and the application turns the section on the flats, which leave it open along a rim. Whether a section laying parts in two is turned as two pieces is Tom's to decide"]
fn seed_5365202888_a_band_1e_7_long_between_a_tapered_tube_and_a_wider_one() {
    random_solids::holds_through_the_application(&Case::new(
        Leaf::turned(
            Plane::xy(20.0),
            Axis::second(0.0),
            Section::bands(
                0.0,
                &[
                    [4.9999999, 5.0, 30.0],
                    [1.0000000000000001e-7, 5.0, 30.0],
                    [12.5, 15.0, 27.5],
                ],
            )
            .sloping_to(&[[5.0, 10.0], [5.0, 30.0], [15.0, 27.5]]),
            360.0,
        ),
        vec![],
    ));
}

/// A disc 6e-8 thick, as 5365201759.
#[test]
fn seed_5365205927_a_disc_6e_8_thick_is_nothing() {
    random_solids::holds_through_the_application(&Case::new(
        Leaf::turned(
            Plane::xy(1.0),
            Axis::first(0.0),
            Section::bands(-3.0000000000000004, &[[6.000000007944095e-8, 0.0, 8.5]]),
            360.0,
        ),
        vec![],
    ));
}

/// A shaft tapering by 1e-5, ending on a band 1e-5 long whose wall falls from
/// 40 to 25, a slope a hair from square: laying declined it, and the flats drew
/// it crossing at the rim. The slant now ends where the run back along it does,
/// and the section is turned exactly.
#[test]
fn seed_5365208537_a_shaft_tapering_by_1e_5_ending_on_a_band_1e_5_long_sloping_to_its_end() {
    random_solids::holds_through_the_application(&Case::new(
        Leaf::turned(
            Plane::xy(10.0),
            Axis::first(0.0),
            Section::bands(25.0, &[[4.99999, 0.0, 40.0], [1e-5, 0.0, 40.0]])
                .sloping_to(&[[0.0, 39.99999], [0.0, 25.0]]),
            360.0,
        ),
        vec![],
    ));
}

/// As 5365208537, 4.5 across.
#[test]
fn seed_5365208894_a_short_shaft_tapering_by_1e_5_ending_on_a_band_1e_5_long() {
    random_solids::holds_through_the_application(&Case::new(
        Leaf::turned(
            Plane::xy(1.0),
            Axis::second(0.0),
            Section::bands(4.0, &[[1.99999, 0.0, 4.5], [1e-5, 0.0, 4.5]])
                .sloping_to(&[[0.0, 4.49999], [0.0, 3.5]]),
            360.0,
        ),
        vec![],
    ));
}

/// A shaft whose wall slopes by 6e-6 over 45, then a cone whose tip stands
/// 1.8e-6 off the axis: laying declined the section, and the flats left it
/// open. The slant now ends where the run back along it does, and the section
/// is turned exactly.
#[test]
fn seed_5365210847_a_shaft_sloping_by_6e_6_before_a_point_1_8e_6_off_its_axis() {
    random_solids::holds_through_the_application(&Case::new(
        Leaf::turned(
            Plane::xy(-30.0),
            Axis::second(0.0),
            Section::bands(-90.0, &[[45.0, -105.0, -0.0], [60.0, -105.0, -0.0]])
                .sloping_to(&[[-104.999994, -0.0], [-1.8e-6, -0.0]]),
            360.0,
        ),
        vec![],
    ));
}

/// A disc 2e-8 thick drawn below its axis, as 5365201759.
#[test]
fn seed_5365213065_a_disc_2e_8_thick_below_its_axis_is_nothing() {
    random_solids::holds_through_the_application(&Case::new(
        Leaf::turned(
            Plane::xy(1.0),
            Axis::first(0.0),
            Section::bands(7.49999998, &[[1.999999987845058e-8, -6.5, -0.0]]),
            360.0,
        ),
        vec![],
    ));
}

/// A disc 3e-7 thick and 135 in radius, as 5365201759.
#[test]
fn seed_5365214883_a_disc_3e_7_thick_and_wide_is_nothing() {
    random_solids::holds_through_the_application(&Case::new(
        Leaf::turned(
            Plane::xy(240.0),
            Axis::first(0.0),
            Section::bands(89.9999997, &[[2.9999999640040187e-7, -135.0, -0.0]]),
            360.0,
        ),
        vec![],
    ));
}

/// As 5365208537, 6 across.
#[test]
fn seed_5365216402_a_shaft_six_across_tapering_by_1e_5_ending_on_a_band_1e_5_long() {
    random_solids::holds_through_the_application(&Case::new(
        Leaf::turned(
            Plane::xy(1.0),
            Axis::second(0.0),
            Section::bands(0.0, &[[2.49999, 0.0, 6.0], [1e-5, 0.0, 6.0]])
                .sloping_to(&[[0.0, 5.99999], [0.0, 5.0]]),
            360.0,
        ),
        vec![],
    ));
}

/// A tube whose outer wall falls from 135 to 120, then a band 6e-6 long whose
/// outer wall falls by 30, a slope a hair from square: laying declined it, and
/// the flats left it open. The slant now ends where the run back along it does,
/// and the section is turned exactly.
#[test]
fn seed_5365218392_a_tube_ending_on_a_band_6e_6_long_whose_top_falls_by_thirty() {
    random_solids::holds_through_the_application(&Case::new(
        Leaf::turned(
            Plane::xy(150.0),
            Axis::second(0.0),
            Section::bands(30.0, &[[14.999994, 90.0, 135.0], [6e-6, 90.0, 135.0]])
                .sloping_to(&[[90.0, 120.0], [90.0, 105.0]]),
            360.0,
        ),
        vec![],
    ));
}

/// As 5365208537, 8 across.
#[test]
fn seed_5365219857_a_shaft_eight_across_tapering_by_1e_5_ending_on_a_band_1e_5_long() {
    random_solids::holds_through_the_application(&Case::new(
        Leaf::turned(
            Plane::xy(6.0),
            Axis::first(0.0),
            Section::bands(4.0, &[[1.49999, 0.0, 8.0], [1e-5, 0.0, 8.0]])
                .sloping_to(&[[0.0, 7.99999], [0.0, 6.5]]),
            360.0,
        ),
        vec![],
    ));
}

/// A disc 2e-8 thick turned about the plane's second axis, as 5365201759.
#[test]
fn seed_5365221239_a_disc_2e_8_thick_about_the_second_axis_is_nothing() {
    random_solids::holds_through_the_application(&Case::new(
        Leaf::turned(
            Plane::xy(3.0),
            Axis::second(0.0),
            Section::bands(-3.0, &[[1.999999987845058e-8, 0.0, 3.0]]),
            360.0,
        ),
        vec![],
    ));
}

/// A shaft tapering by 2e-7 into a cone, joined to a coaxial stepped shaft:
/// laying declined the first section, and the flats it fell back on, joined to
/// the exact shaft, were drawn crossing. The slant now ends where the run back
/// along it does, and the section is turned exactly.
#[test]
fn seed_5365224843_a_shaft_tapering_by_2e_7_into_a_cone_joined_to_a_stepped_shaft() {
    random_solids::holds_through_the_application(&Case::new(
        Leaf::turned(
            Plane::xy(-2.0),
            Axis::second(0.0),
            Section::bands(
                -2.0,
                &[[0.922649730810374, 0.0, 2.0], [0.577350269189626, 0.0, 2.0]],
            )
            .sloping_to(&[[0.0, 2.0000002], [0.0, 3.0]]),
            360.0,
        ),
        vec![Step::add(Leaf::turned(
            Plane::xy(3.0),
            Axis::second(0.0),
            Section::bands(-1.0, &[[1.0, -1.5, 2.5], [2.0, -2.5, 3.5]]),
            360.0,
        ))],
    ));
}

/// As 5365208537, drawn below its axis.
#[test]
fn seed_5365227230_a_shaft_below_its_axis_tapering_by_1e_5_ending_on_a_band_1e_5_long() {
    random_solids::holds_through_the_application(&Case::new(
        Leaf::turned(
            Plane::xy(8.0),
            Axis::first(0.0),
            Section::bands(3.0, &[[4.99999, -6.0, 0.0], [1e-5, -6.0, 0.0]])
                .sloping_to(&[[-5.99999, 0.0], [-4.0, 0.0]]),
            360.0,
        ),
        vec![],
    ));
}

/// A disc 6e-7 thick and 75 in radius, as 5365201759.
#[test]
fn seed_5365227767_a_disc_6e_7_thick_is_nothing() {
    random_solids::holds_through_the_application(&Case::new(
        Leaf::turned(
            Plane::xy(210.0),
            Axis::first(0.0),
            Section::bands(-120.0, &[[6.000000070116585e-7, 0.0, 75.0]]),
            360.0,
        ),
        vec![],
    ));
}

/// A disc 2e-8 thick and 7 in radius about the second axis, as 5365201759.
#[test]
fn seed_5365228671_a_disc_2e_8_thick_seven_across_is_nothing() {
    random_solids::holds_through_the_application(&Case::new(
        Leaf::turned(
            Plane::xy(-3.0),
            Axis::second(0.0),
            Section::bands(-3.0, &[[1.999999987845058e-8, 0.0, 7.0]]),
            360.0,
        ),
        vec![],
    ));
}

/// A disc 6e-7 thick standing 270 along its axis, as 5365201759.
#[test]
fn seed_5365229905_a_disc_6e_7_thick_far_along_its_axis_is_nothing() {
    random_solids::holds_through_the_application(&Case::new(
        Leaf::turned(
            Plane::xy(30.0),
            Axis::first(0.0),
            Section::bands(269.9999994, &[[6.000000212225132e-7, 0.0, 105.0]]),
            360.0,
        ),
        vec![],
    ));
}

/// A pointed shaft of four bands whose steps stand 3e-7 and 1e-5 off their
/// neighbours: laying declined it, and the flats left it open. The slant now
/// ends where the run back along it does, and the section is turned exactly.
#[test]
fn seed_5365230087_a_shaft_whose_steps_stand_3e_7_and_1e_5_off_their_neighbours() {
    random_solids::holds_through_the_application(&Case::new(
        Leaf::turned(
            Plane::xy(-60.0),
            Axis::first(0.0),
            Section::bands(
                60.0,
                &[
                    [15.0, 0.0, 150.0],
                    [104.9999997, 0.0, 149.9999997],
                    [15.000000299999996, 0.0, 45.0],
                    [15.0, 0.0, 45.00001],
                ],
            )
            .sloping_to(&[[0.0, 150.0], [0.0, 149.9999997], [0.0, 45.0], [0.0, 0.0]]),
            360.0,
        ),
        vec![],
    ));
}

/// A disc 1e-7 thick and 30 in radius, as 5365201759.
#[test]
fn seed_5365233246_a_disc_1e_7_thick_is_nothing() {
    random_solids::holds_through_the_application(&Case::new(
        Leaf::turned(
            Plane::xy(-10.0),
            Axis::first(0.0),
            Section::bands(-35.0, &[[1.0000000116860974e-7, 0.0, 30.0]]),
            360.0,
        ),
        vec![],
    ));
}

/// A ring whose inner wall slants out by a half, then a band 2e-7 long whose
/// outer wall falls by one: laying declines it, and the flats leave it open.
#[test]
#[ignore = "laying-declines, found by round 2's triage: the band's falling top passes 1e-7 from the corner where the ring's slanted inner wall ends, two runs away from its own end, and the band below that corner is a fin 1e-7 to 2e-7 thick, from under to over the tolerance of 1.6e-7: no wall wholly thinner than the tolerance to take away, so laying declines the corner near the slant (turning/straight/contacts.rs, `a_slant_is_met`), and the application turns the section on the flats, which leave it open along a rim"]
fn seed_5365236403_a_ring_ending_on_a_band_2e_7_long_whose_top_falls_by_one() {
    random_solids::holds_through_the_application(&Case::new(
        Leaf::turned(
            Plane::xy(4.0),
            Axis::first(0.0),
            Section::bands(-1.0, &[[2.9999998, 3.0, 4.0], [2e-7, 3.0, 4.0]])
                .sloping_to(&[[3.5, 4.0], [3.0, 3.0]]),
            360.0,
        ),
        vec![],
    ));
}

/// A disc 5e-8 thick about the second axis, as 5365201759.
#[test]
fn seed_5365236670_a_disc_5e_8_thick_about_the_second_axis_is_nothing() {
    random_solids::holds_through_the_application(&Case::new(
        Leaf::turned(
            Plane::xy(40.0),
            Axis::second(0.0),
            Section::bands(27.49999995, &[[5.000000058430487e-8, 0.0, 30.0]]),
            360.0,
        ),
        vec![],
    ));
}

/// A disc 6e-8 thick and 3.5 in radius, as 5365201759.
#[test]
fn seed_5365237332_a_disc_6e_8_thick_three_and_a_half_across_is_nothing() {
    random_solids::holds_through_the_application(&Case::new(
        Leaf::turned(
            Plane::xy(1.0),
            Axis::first(0.0),
            Section::bands(-6.0, &[[5.999999963535174e-8, 0.0, 3.5]]),
            360.0,
        ),
        vec![],
    ));
}

/// A stepped cone joined to a lying cylinder 2e-8 narrower than its rim, cut by
/// a block and by a cylinder about a third axis: was drawn open along an arc.
///
/// The body keeps two coaxial walls of radii 1.3e-8 apart, under its
/// tolerance, and a ring that wide between a circle of each. Only one of them
/// took the rays of the curve the bore meets it along, and the ring's two
/// rims were sampled apart: its chords crossed and it was left uncut. Walls
/// at one place now take each other's rays (round 2 of #536).
#[test]
fn seed_5365207818_a_cone_joined_to_a_cylinder_a_hair_inside_its_rim_cut_twice() {
    random_solids::holds_exactly(&Case::new(
        Leaf::turned(
            Plane::xy(5.0),
            Axis::first(0.0),
            Section::bands(
                3.0,
                &[
                    [2.8452994731677537, 0.0, 4.5],
                    [1.1547005268322466, 0.0, 4.5],
                    [0.5, 0.0, 6.5],
                ],
            )
            .sloping_to(&[[0.0, 4.5], [0.0, 6.49999998], [0.0, 6.5]]),
            360.0,
        ),
        vec![
            Step::add(Leaf::prism(
                Plane::yz(7.0),
                Outline::circle([0.0, 5.0], 6.49999998),
                -3.0,
            )),
            Step::cut(Leaf::prism(
                Plane::xy(1.0),
                Outline::rectangle([-3.0, 1.0], [3.0, 7.0]),
                15.0,
            )),
            Step::cut(Leaf::turned(
                Plane::yz(4.0),
                Axis::second(0.0),
                Section::bands(-3.0, &[[2.0, -3.0, 2.0]]),
                360.0,
            )),
        ],
    ));
}

/// A cylinder ending on a cone, joined to a coaxial ring turned 315 degrees the
/// other way: was drawn open along an arc.
///
/// A face of a cone that meets a sample twice — along an edge it runs out and
/// back along, or at a corner it passes twice — was laid out by angles summed
/// step by step, and its two passes stood a rounding apart and crossed: the face
/// was left uncut. Each angle is now the sample's own, a whole number of turns
/// on (round 2 of #536).
#[test]
fn seed_5365207903_a_cylinder_and_cone_joined_to_a_coaxial_turn_of_315_degrees() {
    random_solids::holds_exactly(&Case::new(
        Leaf::turned(
            Plane::xy(-5.0),
            Axis::first(-10.0),
            Section::bands(10.0, &[[2.5, 0.0, 27.5], [2.5, 0.0, 37.5]])
                .sloping_to(&[[0.0, 27.5], [0.0, 27.5]]),
            360.0,
        ),
        vec![Step::add(Leaf::turned(
            Plane::xy(-5.0),
            Axis::first(-10.0).backwards(),
            Section::bands(-17.5, &[[3.75, -40.0, -32.5]]),
            315.0,
        ))],
    ));
}

/// Stepped cones cut by a half turn of a cone about a square axis whose ends
/// stand 2e-7 off their steps: two faces were drawn crossing.
///
/// A face of a cone that meets a sample twice — along an edge it runs out and
/// back along, or at a corner it passes twice — was laid out by angles summed
/// step by step, and its two passes stood a rounding apart and crossed: the face
/// was left uncut. Each angle is now the sample's own, a whole number of turns
/// on (round 2 of #536).
#[test]
fn seed_5365210781_stepped_cones_cut_by_a_half_cone_2e_7_off_their_steps() {
    random_solids::holds_exactly(&Case::new(
        Leaf::turned(
            Plane::xy(3.0),
            Axis::second(0.0),
            Section::bands(
                4.0,
                &[
                    [1.0, 0.0, 3.0],
                    [1.5, 0.0, 3.5],
                    [0.5, 0.0, 3.5],
                    [1.0, 0.0, 4.0],
                ],
            )
            .sloping_to(&[[0.0, 3.0], [0.0, 3.5], [0.0, 4.0], [0.0, 4.0]]),
            360.0,
        ),
        vec![Step::cut(Leaf::turned(
            Plane::yz(0.0),
            Axis::first(3.0),
            Section::bands(6.500000200000001, &[[2.4999997999999994, -3.5, -0.0]])
                .sloping_to(&[[-5.999999799999998, -0.0]]),
            180.0,
        ))],
    ));
}

/// Prisms alone: a block joined to a post, then to a post whose centre stands
/// 6e-6 off the block's side: two faces drawn crossing, the kernel's hair
/// family of #533 and no turn's.
///
/// Was ignored as: triangles, found by round 2's triage: prisms alone, a block
/// joined to a post and to a post 6e-6 off its side; the exact body lists
/// itself on its geometry and holds every line's promise, and two of its faces
/// are drawn crossing: the hair family of #533, no turn's. Holds since the
/// triangles of two faces folded onto each other across an edge are cut the
/// other way, round 3 of #536.
#[test]
fn seed_5365212787_a_block_joined_to_a_post_and_to_a_post_6e_6_off_its_side() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(
            Plane::xy(0.0),
            Outline::rectangle([-30.0, 60.0], [225.0, 150.0]),
            420.0,
        ),
        vec![
            Step::add(Leaf::prism(
                Plane::xy(0.0),
                Outline::circle([60.0, 0.0], 60.0),
                840.0,
            )),
            Step::add(Leaf::prism(
                Plane::xy(-30.0),
                Outline::circle([59.999994, 45.0], 14.9999997),
                3360.0,
            )),
        ],
    ));
}

/// A cone of a section across its axis joined to a half turn about a square
/// axis: was drawn open along an arc.
///
/// A face of a cone that meets a sample twice — along an edge it runs out and
/// back along, or at a corner it passes twice — was laid out by angles summed
/// step by step, and its two passes stood a rounding apart and crossed: the face
/// was left uncut. Each angle is now the sample's own, a whole number of turns
/// on (round 2 of #536).
#[test]
fn seed_5365216755_a_cone_across_its_axis_joined_to_a_half_turn_about_another() {
    random_solids::holds_exactly(&Case::new(
        Leaf::turned(
            Plane::xy(0.0),
            Axis::second(0.0).backwards(),
            Section::bands(-3.0, &[[2.0, -1.0, 1.5]]).sloping_to(&[[-1.0, 3.5]]),
            360.0,
        ),
        vec![Step::add(Leaf::turned(
            Plane::yz(0.0),
            Axis::first(0.0),
            Section::bands(1.5, &[[1.0, 3.0, 5.5]]),
            180.0,
        ))],
    ));
}

/// A funnel joined to a coaxial post whose top rim stands 3e-7 inside the
/// funnel's cone, between its rims: the application's finer triangles were
/// drawn crossing (body/drawn.rs). The cone's triangles reach from rim to rim,
/// and the post's rim, sampled on angles of its own, poked through them. A
/// coaxial circle within a sag of a cone's face now stands in the cone's
/// group, sampled on the rays its rims take (round 2 of #536).
#[test]
fn seed_5365223411_a_cone_joined_to_a_post_3e_7_inside_its_rim() {
    random_solids::holds_through_the_application(&Case::new(
        Leaf::turned(
            Plane::xz(0.0),
            Axis::second(0.0),
            Section::bands(0.0, &[[15.0, -165.0, -90.0], [45.0, -165.0, -82.5]])
                .sloping_to(&[[-165.0, -82.5], [-165.0, -60.0]]),
            360.0,
        ),
        vec![Step::add(Leaf::prism(
            Plane::xy(15.0),
            Outline::circle([0.0, 0.0], 82.4999997),
            -45.0,
        ))],
    ));
}

/// A cone whose tip stands 1e-7 past its axis, cut by a cylinder about a square
/// axis and by a coaxial one: two faces drawn crossing.
#[test]
#[ignore = "triangles, found by round 2's triage: a cone pointed 1e-7 past its axis cut by a cylinder about a square axis and by a coaxial one; the exact body lists itself on its geometry and holds every line's promise, and two of its faces are drawn crossing"]
fn seed_5365223651_a_cone_pointed_1e_7_off_its_axis_cut_by_two_cylinders() {
    random_solids::holds_exactly(&Case::new(
        Leaf::turned(
            Plane::xz(4.0),
            Axis::first(0.0),
            Section::bands(-2.0, &[[5.0, -4.5, 0.0]]).sloping_to(&[[-4.5000001, 0.0]]),
            360.0,
        ),
        vec![
            Step::cut(Leaf::turned(
                Plane::xy(4.0),
                Axis::second(0.0),
                Section::bands(1.0, &[[7.0, 1.0, 4.0]]),
                360.0,
            )),
            Step::cut(Leaf::turned(
                Plane::xz(4.0),
                Axis::first(0.0),
                Section::bands(0.0, &[[2.0, 0.0, 4.5]]),
                360.0,
            )),
        ],
    ));
}

/// A cylinder cut by a ring turned 359.9 degrees about a square axis, then by a
/// coaxial pointed cone: was drawn open along an arc.
///
/// A face of a cone that meets a sample twice — along an edge it runs out and
/// back along, or at a corner it passes twice — was laid out by angles summed
/// step by step, and its two passes stood a rounding apart and crossed: the face
/// was left uncut. Each angle is now the sample's own, a whole number of turns
/// on (round 2 of #536).
#[test]
fn seed_5365226551_a_cylinder_cut_by_a_turn_of_359_9_degrees_and_by_a_coaxial_point() {
    random_solids::holds_exactly(&Case::new(
        Leaf::turned(
            Plane::xy(6.5),
            Axis::second(0.0),
            Section::bands(3.0, &[[2.0, -4.0, 3.0]]),
            360.0,
        ),
        vec![
            Step::cut(Leaf::turned(
                Plane::yz(0.0),
                Axis::first(6.5),
                Section::bands(3.5, &[[0.5, -5.5, -2.5]]),
                359.9,
            )),
            Step::cut(Leaf::turned(
                Plane::xy(6.5),
                Axis::second(0.0),
                Section::bands(2.5, &[[3.0, -1.5, -0.0]]).sloping_to(&[[-3.5, -0.0]]),
                360.0,
            )),
        ],
    ));
}

/// The grazing coaxial family: a cone whose rim stands 2e-8 under a cylinder
/// whose axis stands 1e-8 off the cone's: the triangles enclose less than the
/// exact body.
#[test]
#[ignore = "triangles, found by round 2's triage: the grazing coaxial family, a cone whose rim stands 2e-8 under a cylinder 1e-8 off its axis; the exact body lists itself on its geometry and holds every line's promise, and its triangles enclose less than the body along a line (Volume). Left for Tom's word if a fix needs a fall-back or a decline"]
fn seed_5365227506_a_cone_2e_8_under_a_cylinder_whose_axis_stands_1e_8_off() {
    random_solids::holds_exactly(&Case::new(
        Leaf::turned(
            Plane::xz(0.0),
            Axis::second(5.0),
            Section::bands(5.0, &[[1.0, 0.0, 4.5]]).sloping_to(&[[0.0, 4.49999998]]),
            360.0,
        ),
        vec![Step::add(Leaf::turned(
            Plane::xz(0.0),
            Axis::second(5.00000001),
            Section::bands(6.0, &[[3.0, 0.0, 4.5]]),
            360.0,
        ))],
    ));
}

/// A post cut by another and by a cylinder turned about an axis 2e-8 above its
/// top: two faces drawn crossing. No cone.
#[test]
#[ignore = "triangles, found by round 2's triage: a post cut by a post and by a cylinder whose axis stands 2e-8 above its top, no cone; the exact body lists itself on its geometry and holds every line's promise, and two of its faces are drawn crossing"]
fn seed_5365231546_a_post_cut_by_a_cylinder_whose_axis_stands_2e_8_above_its_top() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(Plane::xy(3.0), Outline::circle([6.0, 7.0], 4.5), 8.5),
        vec![
            Step::cut(Leaf::prism(
                Plane::xy(3.0),
                Outline::circle([0.0, 8.0], 2.5),
                17.0,
            )),
            Step::cut(Leaf::turned(
                Plane::yz(4.99999999),
                Axis::first(11.50000002),
                Section::bands(5.5, &[[5.0, 0.0, 2.5]]),
                360.0,
            )),
        ],
    ));
}

/// A stepped shaft turned about an axis leaning 1e-7, cut by a cylinder about a
/// square axis: two faces drawn crossing. No cone.
///
/// Was ignored as: triangles, found by round 2's triage: a stepped shaft about
/// an axis leaning 1e-7 cut by a cylinder about a square axis, no cone; the
/// exact body lists itself on its geometry and holds every line's promise, and
/// two of its faces are drawn crossing. Holds since the triangles of two faces
/// folded onto each other across an edge are cut the other way, round 3 of
/// #536.
#[test]
fn seed_5365232431_a_stepped_shaft_leaning_1e_7_cut_by_a_cylinder_about_a_square_axis() {
    random_solids::holds_exactly(&Case::new(
        Leaf::turned(
            Plane::xy(5.0),
            Axis::first(0.0).leaning(1e-7),
            Section::bands(5.0, &[[3.0, -1.0, 3.0], [1.0, -2.0, 2.0]]),
            360.0,
        ),
        vec![Step::cut(Leaf::turned(
            Plane::yz(1.0),
            Axis::second(0.0),
            Section::bands(1.0, &[[3.0, 0.0, 7.0]]),
            360.0,
        ))],
    ));
}

/// A stepped cone cut by a quarter turn of a band about a parallel axis, whose
/// end touches the cone along an arc and leaves it there as an edge the cone's
/// face runs out and back along: was drawn open along the cone's rim.
///
/// A face of a cone that meets a sample twice — along an edge it runs out and
/// back along, or at a corner it passes twice — was laid out by angles summed
/// step by step, and its two passes stood a rounding apart and crossed: the face
/// was left uncut. Each angle is now the sample's own, a whole number of turns
/// on (round 2 of #536).
#[test]
fn seed_5365237361_a_stepped_cone_cut_by_a_quarter_turn_about_a_parallel_axis() {
    random_solids::holds_exactly(&Case::new(
        Leaf::turned(
            Plane::xy(-1.0),
            Axis::first(0.0),
            Section::bands(-3.0, &[[3.5, 0.0, 5.0], [1.0, 2.0, 5.0], [3.0, 2.0, 5.0]])
                .sloping_to(&[[0.0, 5.0], [2.0, 5.0], [5.0, 5.0]]),
            360.0,
        ),
        vec![Step::cut(Leaf::turned(
            Plane::xz(0.0),
            Axis::first(-1.0).backwards(),
            Section::bands(-3.5, &[[3.5, 4.0, 6.0]]),
            90.0,
        ))],
    ));
}

/// A rounded block cut by a block, by a cylinder turned whole and by a slot:
/// two faces drawn crossing. No cone.
#[test]
#[ignore = "triangles, found by round 2's triage: a rounded block cut by a block, a cylinder and a slot, no cone; the exact body lists itself on its geometry and holds every line's promise, and two of its faces are drawn crossing"]
fn seed_5365239260_a_rounded_block_cut_by_a_block_a_cylinder_and_a_slot() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(
            Plane::xy(-60.0),
            Outline::rounded([180.0, 0.0], [240.0, 165.0], 15.0),
            135.0,
        ),
        vec![
            Step::cut(Leaf::prism(
                Plane::xy(120.0),
                Outline::rectangle([135.0, -75.0], [405.0, 195.0]),
                150.0,
            )),
            Step::cut(Leaf::turned(
                Plane::xz(90.0),
                Axis::first(7.5),
                Section::bands(180.0, &[[60.0, 0.0, 90.0]]),
                360.0,
            )),
            Step::cut(Leaf::prism(
                Plane::xz(30.0),
                Outline::slot([270.0, 90.0], [210.0, 90.0], 60.0),
                180.0,
            )),
        ],
    ));
}

/// A block cut by a cylinder that leaves a strip of its top 2e-8 wide at the
/// rim, then joined to a block whose side stands 1e-8 off the cylinder's
/// wall, in the middle of the strip. That side was taken for the first
/// block's, 1e-8 off, and the strip was a region between the cylinder's line
/// and that side's, its middle on the second block's own edge all along its
/// chord: read there and further along, a tie. The region is read either side
/// of its chord now. No cone.
#[test]
fn seed_5365202020_a_block_cut_by_a_cylinder_joined_to_a_block_1e_8_off_its_wall() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(
            Plane::xy(5.0),
            Outline::rectangle([-1.5, 5.5], [7.5, 14.5]),
            5.0,
        ),
        vec![
            Step::cut(Leaf::turned(
                Plane::xy(10.0),
                Axis::second(3.0).backwards(),
                Section::bands(-11.0, &[[7.5, -4.49999998, -0.0]]),
                360.0,
            )),
            Step::add(Leaf::prism(
                Plane::xy(4.0),
                Outline::rectangle([7.49999999, 9.0], [7.99999998, 11.0]),
                6.0,
            )),
        ],
    ));
}

/// 5365202020 as drawn, shrunk again once the block above held: a round post
/// in place of the block.
#[test]
#[ignore = "kernel-declines, found by round 2's lane: a round post cut by a cylinder whose top line stands 2e-8 inside the post's wall, joined to a block whose side stands between the two. The post's rim and the cylinder's line, both laid on the block's side, cross there 8.5e-4 apart and run as one in its parameters between: two arcs leave a corner along one another (overlay/star.rs), a tie. The band's family, decisions 7 and 9; the seed as drawn fails at ae20fa9 too"]
fn seed_5365202020_as_drawn_a_round_post_whose_rim_grazes_a_cylinder_s_line_on_a_joined_side() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(Plane::xy(5.0), Outline::circle([3.0, 10.0], 4.5), 5.0),
        vec![
            Step::cut(Leaf::turned(
                Plane::xy(10.0),
                Axis::second(3.0).backwards(),
                Section::bands(-11.0, &[[7.5, -4.49999998, -0.0]]),
                360.0,
            )),
            Step::add(Leaf::prism(
                Plane::xy(4.0),
                Outline::rectangle([7.49999999, 9.0], [7.99999998, 11.0]),
                6.0,
            )),
        ],
    ));
}

/// The grazing coaxial family: a turned tube joined to a coaxial cone that
/// stands 1e-7 off the tube's bore. Decision 2 moved a wall of the bore's
/// radius onto a touch, 2.5e-8 off the axis it shares with the cone, and the
/// two, still decided about one axis, met along a circle the moved wall stood
/// off: an edge listed itself 6e-8 off its face. A wall about a cone's axis
/// is no longer moved off it; no fall-back and no decline.
#[test]
fn seed_5365208585_a_tube_whose_bore_stands_1e_7_off_a_coaxial_cone_s_tip() {
    random_solids::holds_exactly(&Case::new(
        Leaf::turned(
            Plane::xy(-10.0),
            Axis::second(0.0),
            Section::bands(20.0, &[[25.0, 0.0, 25.0]])
                .with_holes(&[([25.0, 12.5], [32.5, 24.9999999])]),
            360.0,
        ),
        vec![Step::add(Leaf::turned(
            Plane::xz(0.0),
            Axis::second(0.0),
            Section::bands(
                -5.0,
                &[[12.5, 0.0, 25.0], [9.99999, 0.0, 25.0], [1e-5, 0.0, 25.0]],
            )
            .sloping_to(&[[0.0, 24.9999999], [0.0, 25.0], [0.0, 20.0]]),
            360.0,
        ))],
    ));
}

/// A cone cut by a block whose side, parallel to its axis, stands 5e-8 past
/// its widest rim, a tolerance and a tenth: the kernel tells a face clear of
/// another only four tolerances off it, and declines the step as
/// unsupported, a hyperbola it may cut. The harness took the side for clear
/// past 1e-12 of the reach, and counted the decline as no answer; it counts
/// a plane that near the rim as asking for a conic now.
#[test]
fn seed_5365216226_a_cone_cut_by_a_block_whose_side_grazes_its_rim() {
    random_solids::holds_exactly(&Case::new(
        Leaf::turned(
            Plane::xy(35.0),
            Axis::first(-15.0).backwards(),
            Section::bands(-5.0, &[[2.5, -2.5, 2.5]]).sloping_to(&[[-2.5, 5.0]]),
            360.0,
        ),
        vec![Step::cut(Leaf::prism(
            Plane::yz(2.5),
            Outline::rectangle([-9.99999995, 25.0], [-5.0, 45.0]),
            20.0,
        ))],
    ));
}

/// A rounded block cut by a turn of a hundredth of a degree lying on its
/// side, the turn's axis on the line the block's corner round touches that
/// side along. The turn's other end grazes the round within the tolerance,
/// and was moved onto it with the whole turn, carried a hair off the side:
/// the end lying on it was still read as the block's side, the other went,
/// and the line the two ends meet along stood ninety microns along it. The
/// body was declined as unverified. A move now carries nothing off a surface
/// of the first operand it is one with, nor a plane its operand drew corners
/// on further than the tolerance off the line it crosses another along. No
/// cone.
#[test]
fn seed_5365230254_a_rounded_block_cut_by_a_turn_of_a_hundredth_of_a_degree() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(
            Plane::xy(1.0),
            Outline::rounded([9.0, 0.0], [16.0, 4.0], 1.0),
            3.0,
        ),
        vec![Step::cut(Leaf::turned(
            Plane::xz(4.0),
            Axis::second(10.0).backwards(),
            Section::bands(-5.0, &[[2.0, 0.0, 3.0]]),
            0.01,
        ))],
    ));
}

/// A rounded block joined to a cylinder whose last band is 6e-8 long, laid
/// as its end 3e-8 short of where the block's corner round touches the
/// cylinder's wall from inside. The end's rim crosses the round twice, 8.5e-8
/// either side of the block's bottom, but the line the end's plane cuts the
/// round along stands 4.5e-16 inside the wall, less than a coordinate holds:
/// rounded onto it, the rim was taken to touch the round at the bottom, the
/// curve the round and the wall meet along was not cut where the rim crosses
/// it, and the body was declined as unverified. The line is now measured
/// from the two radii. No cone.
#[test]
fn seed_5365239969_a_rounded_block_joined_to_a_cylinder_ending_on_a_band_6e_8_long() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(
            Plane::xy(5.0),
            Outline::rounded([3.0, 2.0], [8.0, 5.0], 1.0),
            -5.0,
        ),
        vec![Step::add(Leaf::turned(
            Plane::xy(0.0),
            Axis::second(0.0),
            Section::bands(-1.0, &[[3.99999994, 0.0, 8.0], [6e-8, 0.0, 8.0]]),
            360.0,
        ))],
    ));
}

/// A block cut by a circle, then by a rounded block whose sides stand 5e-8
/// past the block's. Holds at ae20fa9 and fails since 8a008ea.
#[test]
fn seed_5366611312_a_block_cut_by_a_circle_and_by_a_rounded_block_5e_8_off_its_sides() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(
            Plane::xy(5.0),
            Outline::rectangle([35.0, 40.0], [50.0, 52.5]),
            15.0,
        ),
        vec![
            Step::cut(Leaf::prism(
                Plane::xy(5.0),
                Outline::circle([40.0, 30.0], 22.5),
                30.0,
            )),
            Step::cut(Leaf::prism(
                Plane::xy(0.0),
                Outline::rounded([35.00000005, 40.0], [50.00000005, 52.5], 5.0),
                30.0,
            )),
        ],
    ));
}

/// A block joined to a rounded block whose sides stand 1e-8 and whose base
/// stands 2e-7 off its own, then cut by a circle. Holds at ae20fa9 and fails
/// since 8a008ea.
#[test]
fn seed_5366614710_a_block_joined_to_a_rounded_block_2e_7_off_then_cut_by_a_circle() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(
            Plane::yz(5.5),
            Outline::rectangle([9.0, 8.0], [13.0, 14.0]),
            1.0,
        ),
        vec![
            Step::add(Leaf::prism(
                Plane::yz(5.5000002),
                Outline::rounded([9.00000001, 8.0], [13.00000001, 13.5], 1.0),
                2.0,
            )),
            Step::cut(Leaf::prism(
                Plane::xy(3.0),
                Outline::circle([9.0, 8.0], 5.5),
                7.0,
            )),
        ],
    ));
}

/// A block joined to a cylinder across it, then to a rounded block whose
/// sides stand 3e-7 past the block's. Holds at ae20fa9 and fails since
/// 8a008ea, on the exact kernel too.
#[test]
fn seed_5366205754_a_block_joined_to_a_cylinder_and_to_a_rounded_block_3e_7_off_its_sides() {
    random_solids::holds_through_the_application(&Case::new(
        Leaf::prism(
            Plane::xy(150.0),
            Outline::rectangle([225.0, 120.0], [435.0, 360.0]),
            300.0,
        ),
        vec![
            Step::add(Leaf::turned(
                Plane::yz(435.0),
                Axis::first(60.0),
                Section::bands(120.0, &[[240.0, 0.0, 90.0]]),
                360.0,
            )),
            Step::add(Leaf::prism(
                Plane::xy(120.0),
                Outline::rounded([225.0000003, 120.0], [435.0000003, 360.0], 90.0),
                135.0,
            )),
        ],
    ));
}

/// 5365110795 as drawn. Holds at ae20fa9 and fails since 8a008ea; its
/// shrunk core, below, fails at ae20fa9 too.
#[test]
fn seed_5365110795_as_drawn_a_slide_carrying_a_wall_onto_the_first_s() {
    random_solids::holds_exactly(&Case::drawn_slanted_off_the_lattice(5365110795));
}

/// 5366215526 as drawn. Holds at ae20fa9 and fails since 8a008ea, its
/// triangles crossing; its shrunk core, below, fails at ae20fa9 too.
///
/// Was ignored as: carried-one-with, found by round 3's triage: drawn crossing
/// through the application's body since 8a008ea (canonical/carried.rs, `kept`).
/// Round 3 of #536 holds it twice over, each fix alone enough: a carried
/// surface that stays one with the first's and ends no further from it is
/// allowed, and the triangles of two faces folded onto each other across an
/// edge are cut the other way.
#[test]
fn seed_5366215526_as_drawn_a_slide_carrying_a_wall_onto_the_first_s() {
    random_solids::holds_through_the_application(&Case::drawn_slanted_off_the_lattice(5366215526));
}

/// A stepped section about Z cutting a cylinder about X, radius 240, whose
/// axis stands 90 below its own: the step of radius 150 touches the cylinder
/// from inside, and the next, 6e-7 narrower, is laid with it as one wall of
/// 149.9999997. The section ends on a cone pointed on its axis, which that
/// wall never meets. Held at ae20fa9 and failed since beb95aa: decision 2
/// moved the wall 3e-7 across its axis onto the touch, and the pointed cone,
/// crossing it along a circle 60 below the section, held it on the axis; the
/// step was declined as unverified. A cone holds a wall about its axis only
/// where faces of the two meet on the circle they cross along.
#[test]
fn seed_5365100952_a_wall_touching_a_cylinder_inside_coaxial_with_a_cone_it_never_meets() {
    random_solids::holds_exactly(&Case::new(
        Leaf::turned(
            Plane::xz(0.0),
            Axis::first(90.0),
            Section::bands(-90.0, &[[30.0, 0.0, 240.0]]),
            360.0,
        ),
        vec![Step::cut(Leaf::turned(
            Plane::xy(180.0),
            Axis::second(0.0),
            Section::bands(
                0.0,
                &[
                    [75.0, 0.0, 45.0],
                    [30.0, 0.0, 150.0],
                    [15.0, 0.0, 149.9999994],
                    [45.0, 0.0, 30.0],
                ],
            )
            .sloping_to(&[[0.0, 45.0], [0.0, 150.0], [0.0, 149.9999994], [0.0, 0.0]]),
            360.0,
        ))],
    ));
}

/// 5366415073 as drawn. Holds at ae20fa9 and fails since af8e71a.
///
/// Was ignored as: triangles-sliver, found by round 3's triage: drawn crossing
/// through the application's body since af8e71a (relation/crossing/cut.rs), two
/// triangles of one plane's face crossing across a sliver 1e-8 wide; its shrunk
/// core below. Holds since the triangles of two faces folded onto each other
/// across an edge are cut the other way, and a skin of no thickness two faces
/// are drawn on is left out, round 3 of #536.
#[test]
fn seed_5366415073_as_drawn_a_post_cut_by_a_post_1e_8_past_its_axis() {
    random_solids::holds_through_the_application(&Case::drawn_turned_off_the_lattice(5366415073));
}

/// A post along X cut by a post along Z whose top stands 1e-8 above the
/// first's axis, and whose wall passes through the first's end on the line
/// the first's wall touches it along. On the exact kernel it holds with
/// af8e71a undone at the tip; through the application's body it fails at
/// ae20fa9 too.
///
/// Was ignored as: triangles-sliver, found by round 3's triage: two triangles
/// cross across a sliver 1e-8 wide where the second post's top, the first's end
/// and the line the first's wall touches the second's along meet; the sweep
/// (tessellation/sweep.rs) cuts a region exactly only while its boundaries do
/// not cross, not traced further. Holds since the triangles of two faces folded
/// onto each other across an edge are cut the other way, and a skin of no
/// thickness two faces are drawn on is left out, round 3 of #536.
#[test]
fn seed_5366415073_a_post_cut_by_a_post_whose_top_stands_1e_8_past_its_axis() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(Plane::yz(2.0), Outline::circle([4.0, 9.5], 0.5), 1.0),
        vec![Step::cut(Leaf::prism(
            Plane::xy(5.49999999),
            Outline::circle([2.0, 4.0], 0.5),
            4.00000002,
        ))],
    ));
}

/// 5366116515 as drawn. Holds at ae20fa9 and fails since 6fd4c2a.
///
/// Was ignored as: triangles-sliver, found by round 3's triage: drawn crossing
/// on the exact kernel since 6fd4c2a (tessellation/outline/cone.rs), the end
/// face of a turn of 315 degrees crossing itself across a sliver 5e-8 wide; its
/// shrunk core below. Holds since the triangles of two faces folded onto each
/// other across an edge are cut the other way, and a skin of no thickness two
/// faces are drawn on is left out, round 3 of #536.
#[test]
fn seed_5366116515_as_drawn_a_stepped_cone_joined_to_a_turn_of_315_degrees() {
    random_solids::holds_exactly(&Case::drawn_slanted_off_the_lattice(5366116515));
}

/// A stepped section about Y whose steps of 5 and 4.9999999 end on a cone
/// pointed on its axis, joined to a turn of 315 degrees about the same axis,
/// of radius 4.9999999, whose end planes meet the cone's tip. Fails at
/// ae20fa9 too; with 6fd4c2a undone at the tip, on two triangles of the
/// turn's start face instead.
///
/// Was ignored as: triangles-sliver, found by round 3's triage: two triangles
/// of the turn's end face, one reaching the cone's tip, cross across a sliver
/// 5e-8 wide at the cone's rim, where the steps of 5 and 4.9999999 end
/// (tessellation/sweep.rs), not traced further. Holds since the triangles of
/// two faces folded onto each other across an edge are cut the other way, and a
/// skin of no thickness two faces are drawn on is left out, round 3 of #536.
#[test]
fn seed_5366116515_a_stepped_cone_joined_to_a_coaxial_turn_of_315_degrees() {
    random_solids::holds_exactly(&Case::new(
        Leaf::turned(
            Plane::xy(1.0),
            Axis::second(0.0),
            Section::bands(
                3.0,
                &[
                    [3.5, 0.0, 3.5],
                    [1.0, 0.0, 3.0],
                    [1.5, 0.0, 5.0],
                    [0.5, 0.0, 4.9999999],
                    [1.0, 0.0, 4.9999999],
                ],
            )
            .sloping_to(&[
                [0.0, 3.5],
                [0.0, 3.0],
                [0.0, 5.0],
                [0.0, 4.9999999],
                [0.0, 0.0],
            ]),
            360.0,
        ),
        vec![Step::add(Leaf::turned(
            Plane::yz(0.0),
            Axis::first(1.0),
            Section::bands(9.5, &[[3.0, -4.9999999, -0.0]]),
            315.0,
        ))],
    ));
}

/// 5366215526 shrunk: a turn of a hundredth of a degree cut by a quarter
/// turn about a parallel axis 6e-8 off, both sections in one plane. Fails at
/// ae20fa9 too.
///
/// Was ignored as: triangles-sliver, found by round 3's triage: the two turns'
/// start faces lie in one plane, and two triangles of it cross across the
/// sliver 6e-8 wide between the two axes (tessellation/sweep.rs). Holds since
/// the triangles of two faces folded onto each other across an edge are cut the
/// other way, and a skin of no thickness two faces are drawn on is left out,
/// round 3 of #536.
#[test]
fn seed_5366215526_a_hundredth_of_a_degree_s_turn_cut_by_a_quarter_turn_6e_8_off_its_axis() {
    random_solids::holds_exactly(&Case::new(
        Leaf::turned(
            Plane::xy(8.0),
            Axis::second(-2.0),
            Section::bands(2.0, &[[3.0, -9.0, -0.0]]),
            0.01,
        ),
        vec![Step::cut(Leaf::turned(
            Plane::xy(8.0),
            Axis::second(-1.99999994).backwards(),
            Section::bands(-2.0, &[[1.0, 0.0, 7.0]]),
            90.0,
        ))],
    ));
}

/// 5365110795 shrunk: a quarter turn of a solid cylinder about X joined to
/// a cone turned 270 degrees about Y, its axis passing 3e-7 from the
/// cylinder's. Fails at ae20fa9 too.
#[test]
#[ignore = "kernel-declines, found by round 3's triage: declined as unverified at ae20fa9 too; a cone's axis crossing a square cylinder's 3e-7 off it, not traced"]
fn seed_5365110795_a_quarter_turn_joined_to_a_cone_whose_axis_passes_3e_7_off_its_own() {
    random_solids::holds_exactly(&Case::new(
        Leaf::turned(
            Plane::xy(60.0),
            Axis::first(0.0),
            Section::bands(60.0, &[[255.0, -105.0, -0.0]]),
            90.0,
        ),
        vec![Step::add(Leaf::turned(
            Plane::yz(210.0),
            Axis::first(59.9999997),
            Section::bands(135.0, &[[45.0, -30.0, -0.0]]).sloping_to(&[[-0.0, -0.0]]),
            270.0,
        ))],
    ));
}
