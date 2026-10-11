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
//!
//! The tilted campaign of #552 at `0d74713` is sorted the same way, its keys
//! at the head of the reason: `grazing-planes` where two planes crossing at
//! an angle a few times the one read as parallel part two corners that
//! stand within the tolerance of each other and of both planes, and
//! `hair-from-tangent` where a plane a hair from tangent to a wall, or from
//! an edge, by an offset, is held by no band — the issue's second default.
//! Most of `grazing-planes` holds since two planes too near parallel to fix
//! the line they cross along are decided, once, to graze where every corner
//! of either where their faces meet stands within the tolerance of both,
//! and two corners within it of each other, one on each, are then one.
//!
//! Round 2 of #552 sorts the tilted draws measured at `daf2079`, and the six
//! seeds that held at `34a1b64` and fail there, under two keys:
//! `near-parallel-planes` where two planes cross nearer parallel than the
//! angle that fixes the line they share (`ACROSS`, canonical/planes.rs) —
//! decided to graze, or a hair too far apart for that — and the corners
//! they make with a third surface are kept apart or placed off each other
//! along that line (canonical/corners.rs); `hair-from-tangent` where a plane
//! at a slant a hair from tangent to a wall cuts it along an ellipse turning
//! within a few dozen tolerances, or a conic is cut a hair from a cone's rim
//! or apex: the issue's second default, named, with no band and no decline
//! added.
//!
//! Round 3 sorts the tilted draws measured at `cf9093b`, and the three seeds
//! that held at `7411e98` and fail there, under the same two keys, read wider:
//! `hair-from-tangent` now holds every plane a hair off along a wall's axis or
//! off its tangent, the conic it cuts a hair from a ruling, a rim or an apex,
//! the hairpin that decides its corners (relation/crossing/grazed.rs) and the
//! conic kept for an arc it shares with a line (combine/identified.rs) — all
//! three regressions are of it, and hold since two crossings the line finds
//! apart stay two where the hairpin would make them one, and the conic goes
//! before a rim the arc is one with too. Left, most of it: planes within the
//! tolerance of tangent across the whole box at a lean just past the one read
//! as square, which only a band would hold, the issue's second default;
//! `near-parallel-planes` keeps the planes alone, two too near parallel to fix
//! their line, their faces drawn through each other or their corners parted
//! (canonical/planes.rs, canonical/corners.rs). Part of it holds since such a
//! line is fixed across only to rounding over the sine of the planes' angle,
//! and a place within rounding of both is on it. Left: two corners found on the
//! same three planes, two of them so, more than the tolerance apart along that
//! line; corners found on such a line, each within rounding of both planes,
//! standing off the line as computed by more than the tolerance; two whole
//! curves within the tolerance of each other all round, a rim and the ellipse a
//! plane a hair off its own cuts the wall along; and the two planes of one
//! lean, one pair decided to graze and the next not.
//!
//! Round 4 sorts the tilted draws measured at `0d35b52`, where no seed that
//! held at `e1e325f` fails, under the same two keys, split by the way the
//! hair leans: `near-parallel-planes` now holds every face leaning a hair on
//! another flat face — a block's top or floor, a slot's flat side, a disc's
//! or a ring's end — the two too near parallel to fix their line and not
//! decided to graze, and, where that face ends a wall, the ellipse the
//! leaning face cuts the wall along a hair from the rim: the faces drawn
//! through each other, or edges on the planes, the rim or that ellipse left
//! with one face each (canonical/planes.rs, canonical/corners.rs,
//! relation/sliced.rs just past `parallel`). `hair-from-tangent` keeps the
//! faces leaning a hair along a wall's axis, most at the tangent distance,
//! the hairpin ellipse decided just past `square`, and the faces along a
//! cone's rulings standing a hair off: the issue's second default, named,
//! with no band and no decline added.
//!
//! Round 4 also names the twelve shrunk cases of round 3 with no hair in
//! their inputs, which come first (#552, the decision of 10 October). Five
//! hold: a face touching a cone along a ruling at exactly its slope, the
//! apex beyond the box (5543214149, 5543216778), since such a plane holds
//! the ruling as through an apex; the line a floor shares with a cut
//! touching a cone exactly, found inside the cone's section by rounding
//! (5543231406), since such a line only touches the cone; a slant through a
//! cone's apex on the end planes of two quarter turns (5543226572), since a
//! corner found along a ruling past the apex is not on the cone; and a
//! wedge of a hundredth of a degree cut by a post touching its end plane
//! (5543224272), since two arcs leaving a corner straight together are
//! told apart where they run. Left, ignored with their reason: the edge of
//! the tangency band with a hyperbola at the touch (5543307115), and two
//! regions with no point inside them inherited from #533 and #536
//! (5543305067, 5543616767), which hold since round 5; the crossings are in
//! `what_the_exact_campaigns_found_in_the_triangles.rs`.
//!
//! After round 4, #552 decided to round what no band would hold: a plane of
//! one operand a hair off square to, or along, the axis of a wall of the
//! other is laid exactly so, the operand turned with it where the turn
//! moves it by no more than decision 8's hair (decision 11). Sixteen of the
//! seeds named above under both keys hold since, each a block, a face or a
//! half cylinder leaning 4e-8 to 2e-7 degrees on a wall's end or along its
//! side. Measured on round 3's seeds, the turn broke a few that held as
//! drawn on `85a2d2e`, each a lean laid exactly beside an offset hair it had
//! kept apart or beside a turn's walls: they are named last, under the same
//! two keys.
//!
//! Round 5 names what round 4's measurement at `beaf75d` left first, under
//! two keys. `decision-11-regressions`: the five seeds that held on
//! `85a2d2e` and fail by decision 11's own turn, each holding with the turn
//! switched off — a post, a wedge or a shaft about a world axis laid square
//! to a turn about an axis leaning 1e-7°, and a face or a ring laid square
//! to a block 4e-8° off a quarter turn beside a cut at 45°.
//! `plain-pieces`: the shrunk cases with no hair, each failing on `85a2d2e`
//! too — regions with no point inside them beside two walls of one radius,
//! a stepped shaft turned 359.9°, a rod along a rounded block's side, a
//! ruling of a cone left open; the crossings are in
//! `what_the_exact_campaigns_found_in_the_triangles.rs`. Round 5 holds the
//! regions with no point inside them, the rod along the rounded block, and
//! a seed its first reading of a rim inside a cut broke (5545108465); the
//! stepped shaft and the cone's ruling stay ignored, with what was found.
//! Four of decision 11's regressions hold since it turns about where the
//! operands meet and sets the second down on what it held against the first
//! within the tolerance, rather than about the middle of its box, and with
//! them three seeds named under the earlier keys (5543120314, 5543235780,
//! 5543615040).
//! The fifth is a lean the turn moves onto a pair the kernel fails as
//! drawn, named under its family: a ring's end laid 7e-10 off a line a
//! later cut shares with a floor. Two seeds the first way of setting down
//! broke on the campaign are named after it. Measured merged, the two
//! lanes broke four more seeds, all by decision 11's setting down; they
//! are named after those and hold: a straight edge touching a wall is laid
//! on it, a flat touching a wall is not, and a flat near several lines that
//! part lies on none of them. Measured again, turned about where the
//! operands meet, two cuts reaching far past the body would have moved
//! past the hair and were left as drawn; the turn now falls back on the
//! middle of the cut's box, and both hold. Two more, a face meeting a
//! ring's end along the line two planes 1.7e-7 apart cross on and a face
//! drawn on a 0.01° wedge's face 1e-7° off it, held on `f57a5ca` by where
//! its turn happened to put the line; they are named last under
//! `near-parallel-planes`.
//!
//! Round 6 names what round 5's measurement at `784c69e` left first:
//! `plain-pieces`, the shrunk cases with no hair, each failing as drawn on
//! `f57a5ca` too — two ties in a plane's overlay, a block's face holding a
//! turn's axis, a turn of 359.9°, two walls of one radius whose axes meet
//! square, and a body left open along a line. The crossings, and the one
//! seed the round broke (`decision-11-regressions`), are in
//! `what_the_exact_campaigns_found_in_the_triangles.rs`.

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
/// past 1e-12 of the reach, and counted the decline as no answer; it then
/// counted a plane that near the rim as asking for a conic. Since #552 the
/// kernel cuts that hyperbola, and the case holds answered.
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

/// #552: half a turn of a shaft ending in a frustum, its tip beyond the
/// frustum's narrow rim, cut by a plane through that tip at the frustum's
/// slope: it touches the frustum along the ruling the half turn's flat face
/// holds. The line the plane cuts the flat face along runs on past the tip,
/// over the other nappe, to the shaft's end; a corner there was read off it
/// as lying on the frustum, which the shaft's end never meets, and the flat
/// face was not parted along it. A curve lies on a cone only where it
/// stands on the cone's own nappe.
#[test]
fn seed_9304992_a_plane_touching_a_frustum_along_a_ruling_a_flat_face_holds_is_cut_past_its_tip() {
    random_solids::holds_exactly(&Case::new(
        Leaf::turned(
            Plane::xy(-2.0),
            Axis::first(0.0).backwards(),
            Section::bands(3.0, &[[5.0, 0.0, 4.0], [3.0, 0.0, 4.0]])
                .sloping_to(&[[0.0, 4.0], [0.0, 7.0]]),
            180.0,
        ),
        vec![Step::cut(Leaf::prism(
            Plane::tilted([-4.0, 0.0, -2.0], [90.0, 45.0, 0.0]),
            Outline::rectangle([-10.0, -1.0], [6.0, 8.0]),
            10.0,
        ))],
    ));
}

/// #552: the same, three quarters of a turn, the plane through the tip of a
/// frustum between two shafts. Once the corner past the tip lay on the
/// line, the arc between the two was still laid on the frustum's face, its
/// whole length read back onto the tip: a stretch of no length there, which
/// the face's arcs could not be ordered round. An arc past a cone's apex
/// lies on the curve's other surfaces only (9301298 too).
#[test]
fn seed_9302688_a_plane_touching_a_frustum_between_two_shafts_along_a_ruling_is_cut_past_its_tip() {
    random_solids::holds_exactly(&Case::new(
        Leaf::turned(
            Plane::xy(7.0),
            Axis::first(3.5).backwards(),
            Section::bands(-1.0, &[[1.5, 0.0, 4.5], [0.5, 0.0, 2.0], [4.0, 0.0, 4.5]])
                .sloping_to(&[[0.0, 4.5], [0.0, 1.5], [0.0, 4.5]]),
            270.0,
        ),
        vec![Step::cut(Leaf::prism(
            Plane::tilted([-2.5, 3.5, 7.0], [90.0, 45.0, 0.0]),
            Outline::rectangle([-6.0, -6.0], [7.0, 6.0]),
            7.0,
        ))],
    ));
}

/// #552: a cut along a ruling past a frustum's tip as in 9304992, then a
/// block joined well clear of it.
/// The corner past the frustum's tip, read again as a vertex of the first
/// operand found along the tip's line, took the frustum among its surfaces
/// from that line, and so stood apart from the shaft's end it lies on: the
/// flat face lost its edge along that end. A corner past a cone's apex takes
/// none of a curve's surfaces but those it stands on.
#[test]
fn seed_9303477_a_cut_past_a_frustum_s_tip_is_joined_to_a_block_clear_of_it() {
    random_solids::holds_exactly(&Case::new(
        Leaf::turned(
            Plane::xy(8.0),
            Axis::second(0.0),
            Section::bands(3.0, &[[3.5, 0.0, 2.0], [0.5, 0.0, 2.0]])
                .sloping_to(&[[0.0, 2.0], [0.0, 2.5]]),
            180.0,
        ),
        vec![
            Step::cut(Leaf::prism(
                Plane::tilted([0.0, 4.5, 8.0], [90.0, -45.0, 0.0]),
                Outline::rectangle([-7.0, -1.0], [4.0, 4.0]),
                4.0,
            )),
            Step::add(Leaf::prism(
                Plane::xy(2.0),
                Outline::rectangle([2.0, 10.0], [7.0, 17.0]),
                9.0,
            )),
        ],
    ));
}

/// #552: a shaft along X, a plane leaning 45° about Z whose ellipse on the
/// shaft reaches its far end exactly at the end's rim, and touches it there.
/// The line the plane cuts the end along passes that rim a femtometre
/// inside, as rounding left it, and crossed the shaft twice a tenth of a
/// micron apart: the end held a sliver between that chord and the rim no
/// column could find a point in, declined as a tie. It holds since that
/// line, square to the shaft's axis and shared with a plane crossing it
/// steeply, touches the shaft where it stands inside its section by
/// rounding alone (5544100660).
#[test]
fn seed_9304992_an_ellipse_touching_a_shaft_s_far_rim_is_cut_with_one_corner_there() {
    random_solids::holds_exactly(&Case::new(
        Leaf::turned(
            Plane::xy(-2.0),
            Axis::first(0.0).backwards(),
            Section::bands(3.0, &[[7.5, 0.0, 6.5]]),
            180.0,
        ),
        vec![
            Step::add(Leaf::prism(
                Plane::yz(-10.5),
                Outline::circle([0.0, -2.0], 6.5),
                5.0,
            )),
            Step::cut(Leaf::prism(
                Plane::tilted([-4.0, 0.0, -2.0], [90.0, 45.0, 0.0]),
                Outline::rectangle([-10.0, -1.0], [6.0, 8.0]),
                10.0,
            )),
        ],
    ));
}

/// #552: a partial turn of a frustum and a shaft with a groove, cut along
/// the ruling of the frustum's slope through its tip. A corner past the
/// tip on that ruling took the curve's other surfaces into its support,
/// and was read as still missing the frustum on every pass after, so the
/// support grew for ever: the step never came back. It failed, declining,
/// before corners past an apex were read off its cone.
#[test]
fn seed_9501559_a_partial_turn_cut_along_a_frustum_s_ruling_past_its_tip_comes_back() {
    random_solids::holds_exactly(&Case::new(
        Leaf::turned(
            Plane::xz(1.0),
            Axis::first(0.0),
            Section::bands(5.0, &[[5.5, 1.0, 7.0], [2.5, 1.0, 7.0]])
                .sloping_to(&[[1.0, 7.0], [3.5, 7.0]])
                .with_holes(&[([6.0, 4.0], [9.5, 5.0])]),
            -75.0,
        ),
        vec![Step::cut(Leaf::prism(
            Plane::tilted([9.5, 1.0, 0.0], [0.0, 45.0, 0.0]),
            Outline::rectangle([-9.0, -0.5], [3.0, 7.5]),
            -3.5,
        ))],
    ));
}

/// #552, the family most of the tilted campaign's failures fall in: a plane
/// leaning 1e-7° off square to a shaft's axis at its very end, within four
/// times the angle the kernel reads as square. It cuts the shaft along an
/// ellipse within the tolerance of the end's rim all round, and the plane
/// within the tolerance of the end across it. The two planes graze: the
/// corners of the rim and of the ellipse are one, and so are the arcs
/// between them.
#[test]
fn seed_9300728_a_shaft_cut_at_its_end_by_a_plane_a_hair_off_square_to_its_axis() {
    random_solids::holds_exactly(&Case::new(
        Leaf::turned(
            Plane::xy(0.0),
            Axis::second(0.0).backwards(),
            Section::bands(2.5, &[[2.5, 0.0, 3.0]]),
            360.0,
        ),
        vec![Step::cut(Leaf::prism(
            Plane::tilted([2.0, -2.5, 2.0], [-89.9999999, 0.0, 0.0]),
            Outline::rectangle([-4.0, -2.0], [4.0, 2.0]),
            2.0,
        ))],
    ));
}

/// #552: a shaft cut by a plane at 45° through the point where its axis
/// crosses that of a second shaft of the same radius, square to it, then
/// joined to that shaft. The two meet along two ellipses, and one of them
/// lies in the plane: the conic the plane cuts the first shaft along is
/// that very curve, which crossing the second shaft finds lying along it.
#[test]
#[ignore = "kernel-declines, #552: a conic one with a component of the curve two perpendicular cylinders of one radius meet along is declined as unsupported; a conic is never read as one with a meet"]
fn seed_9501873_a_plane_holding_the_ellipse_two_equal_shafts_meet_along() {
    random_solids::holds_exactly(&Case::new(
        Leaf::turned(
            Plane::yz(210.0),
            Axis::second(0.0),
            Section::bands(120.0, &[[60.0, 0.0, 120.0]]),
            360.0,
        ),
        vec![
            Step::cut(Leaf::prism(
                Plane::tilted([210.0, 0.0, 150.0], [-45.0, 0.0, 0.0]),
                Outline::rectangle([-135.0, -135.0], [135.0, 135.0]),
                135.0,
            )),
            Step::add(Leaf::prism(
                Plane::xz(165.0),
                Outline::circle([210.0, 150.0], 120.0),
                330.0,
            )),
        ],
    ));
}

/// #552: a shaft cut by a plane leaning 15° about Z that touches the
/// shaft's outermost ruling exactly where the next cut's floor passes:
/// the ellipse is tangent to that floor's line there, and its crossing with
/// the floor stands four microns from the floor's own corner. It was
/// declined there until a corner a cut crosses an edge at was laid on the
/// faces the edge lies on over its stretch alone.
#[test]
fn seed_9519300_a_slanted_cut_touching_a_shaft_s_ruling_at_the_next_cut_s_floor() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(Plane::xz(0.0), Outline::circle([60.0, 180.0], 45.0), 150.0),
        vec![
            Step::cut(Leaf::prism(
                Plane::tilted([105.0, -150.0, 225.0], [90.0, -15.0, 0.0]),
                Outline::rectangle([-255.0, -105.0], [15.0, 30.0]),
                75.0,
            )),
            Step::cut(Leaf::prism(
                Plane::tilted([-30.0, -150.0, 180.0], [0.0, 0.0, 0.0]),
                Outline::rectangle([-150.0, -120.0], [165.0, 105.0]),
                120.0,
            )),
        ],
    ));
}

/// #552: a quarter turn of a shaft and a frustum cut by a plane leaning
/// atan ¼ about X, then by a block square to the axis: the block's side
/// left an arc of no length between two corners a rounding apart, which
/// the face's arcs could not be ordered round. It holds since a corner found
/// along a ruling through a cone's apex is on the cone only on its own
/// nappe (5543226572).
#[test]
fn seed_9507403_a_quarter_turn_cut_at_a_slant_then_square_leaves_an_arc_of_no_length() {
    random_solids::holds_exactly(&Case::new(
        Leaf::turned(
            Plane::xy(35.0),
            Axis::second(-15.0).backwards(),
            Section::bands(-15.0, &[[15.0, -2.5, 15.0], [20.0, -2.5, 15.0]])
                .sloping_to(&[[-2.5, 15.0], [-7.5, 15.0]]),
            90.0,
        ),
        vec![
            Step::cut(Leaf::prism(
                Plane::tilted([-15.0, 10.0, 35.0], [14.036243467926479, 0.0, 0.0]),
                Outline::rectangle([-10.0, -35.0], [18.0, 13.0]),
                -13.0,
            )),
            Step::cut(Leaf::prism(
                Plane::yz(-15.0),
                Outline::rectangle([-23.0, 25.0], [18.0, 53.0]),
                10.0,
            )),
        ],
    ));
}

/// #552, the tilted campaign at `0d74713`: a post cut by a block whose
/// floor lies on the post's top, leaning 4e-8° about X. The two planes cross
/// along a line through the post, and stand within the tolerance of each
/// other across it; the corner where the rim, the block's side and the top
/// meet, and the one where the ellipse, that side and the floor meet, stand
/// 8e-10 apart and are one, though the line the planes cross along passes
/// far from them: the rim and the ellipse between them are one arc, used by
/// the top and by the wall.
#[test]
fn seed_5520121880_a_post_cut_by_a_block_whose_floor_leans_4e_8_degrees_off_its_top() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(Plane::xy(4.0), Outline::circle([4.0, 2.0], 3.0), 5.5),
        vec![Step::cut(Leaf::prism(
            Plane::tilted([5.5, 1.5, 9.5], [4e-8, 0.0, 0.0]),
            Outline::rectangle([-5.0, -7.0], [1.0, 4.0]),
            7.0,
        ))],
    ));
}

/// #552, the tilted campaign through the application's body: a post cut
/// by a block whose far face leans 1e-7° off the post's bottom, the family
/// of 5520121880 on the other end.
#[test]
fn seed_5520206994_a_post_cut_by_a_block_whose_far_face_leans_1e_7_degrees_off_its_bottom() {
    random_solids::holds_through_the_application(&Case::new(
        Leaf::prism(Plane::xy(6.0), Outline::circle([6.0, 4.0], 6.0), 9.0),
        vec![Step::cut(Leaf::prism(
            Plane::tilted([3.0, 8.0, 9.0], [1e-7, 0.0, 0.0]),
            Outline::rectangle([-4.0, -4.0], [4.0, 4.0]),
            -3.0,
        ))],
    ));
}

/// #552, the tilted campaign: a block cut by a block whose far face leans
/// 1e-7° off its side, with no curve but lines. The family of 5520121880
/// with no conic in it: it failed on `9e54a71` the same way, and the tilted
/// draw is what reaches it.
#[test]
fn seed_5520122256_a_block_cut_by_a_block_whose_far_face_leans_1e_7_degrees_off_its_side() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(
            Plane::tilted([2.0, 6.0, 4.0], [45.0, 0.0, 0.0]),
            Outline::rectangle([-3.0, -3.0], [-1.5, 1.5]),
            2.5,
        ),
        vec![Step::cut(Leaf::prism(
            Plane::tilted([4.5, 3.0, 6.0], [0.0, 89.9999999, 0.0]),
            Outline::rectangle([-6.0, -6.0], [6.0, 6.0]),
            -4.0,
        ))],
    ));
}

/// #552, the tilted campaign, shrunk: a block whose far face leans 4e-8°
/// off square to Y, pocketed by a turned post whose end lies on that face,
/// then cut by a block square to X across the pocket's rim. The rim the
/// face is left bounded by is the post's circle, on the face over its
/// stretch alone; where the second cut crosses it, the corner is on the
/// face too, and the line the cut leaves on the face ends there. Drawn as
/// it was, the seed held on `34a1b64` and failed once the face's corners
/// and the end's were one.
#[test]
fn seed_5520140378_a_face_leaning_4e_8_degrees_off_a_pocket_s_end_cut_across_the_pocket_s_rim() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(
            Plane::tilted([3.0, 5.0, 4.0], [-89.99999996, 0.0, 0.0]),
            Outline::rectangle([-4.0, -2.0], [0.0, 2.0]),
            4.0,
        ),
        vec![
            Step::cut(Leaf::turned(
                Plane::xy(5.0),
                Axis::second(0.0),
                Section::bands(5.0, &[[5.0, 0.0, 1.0]]),
                360.0,
            )),
            Step::cut(Leaf::prism(
                Plane::yz(2.0),
                Outline::rectangle([4.0, 4.0], [10.0, 6.0]),
                -1.5,
            )),
        ],
    ));
}

/// #552, the tilted campaign through the application's body, shrunk: half
/// a shaft leaning 15° whose flat side lies on the far face of a block
/// leaning 1e-7° off it, cut by that block. The two planes stand within the
/// tolerance of each other where one of the shaft's rims meets the block
/// and part by more where the other does: they cross, and both ends of the
/// rims stay two. It held on `34a1b64` and failed once a corner within the
/// tolerance of both planes was one, corner by corner.
#[test]
fn seed_5520267036_half_a_leaning_shaft_cut_by_a_block_whose_far_face_leans_1e_7_degrees_off_its_flat_side()
 {
    random_solids::holds_through_the_application(&Case::new(
        Leaf::turned(
            Plane::xz(90.0),
            Axis::first(0.0).leaning(15.0),
            Section::bands(30.0, &[[45.0, 0.0, 135.0]]),
            180.0,
        ),
        vec![Step::cut(Leaf::prism(
            Plane::tilted([60.0, -30.0, 0.0], [-1e-7, 0.0, 0.0]),
            Outline::rectangle([-105.0, -120.0], [90.0, 120.0]),
            -195.0,
        ))],
    ));
}

/// #552, the tilted campaign, its first step: a stepped post turned five
/// sixths round, ending in a cone, cut by a block whose far face leans
/// 1e-7° off the post's end. Every corner of either plane where their faces
/// meet stands within the tolerance of both, but the cone's ruling crosses
/// the two 1.2e-8 apart, past the tolerance: the corner it ends at on the
/// end and the one it crosses the face at stay two, and so must the others
/// of the two planes. It held on `34a1b64`.
#[test]
fn seed_5520167649_a_stepped_post_ending_in_a_cone_cut_by_a_block_whose_far_face_leans_1e_7_degrees_off_its_end()
 {
    random_solids::holds_exactly(&Case::new(
        Leaf::turned(
            Plane::yz(5.0),
            Axis::second(0.0).backwards(),
            Section::bands(
                -1.0,
                &[
                    [2.5, -4.5, 6.499999999999999e-7],
                    [1.5, -2.5, 6.499999999999999e-7],
                    [0.03589838486224517, -5.0, 6.499999999999999e-7],
                    [3.464101615137755, -5.0, 6.499999999999999e-7],
                ],
            )
            .sloping_to(&[
                [-4.4999998, 6.499999999999999e-7],
                [-2.5, 6.499999999999999e-7],
                [-5.0, 6.499999999999999e-7],
                [-3.0, 6.499999999999999e-7],
            ]),
            -300.0,
        ),
        vec![Step::cut(Leaf::prism(
            Plane::tilted([5.0, 3.0, -6.5], [-1e-7, 0.0, 0.0]),
            Outline::rectangle([-5.5, -9.0], [5.5, 2.5]),
            -1.0,
        ))],
    ));
}

/// #552, the tilted campaign through the application's body, shrunk: a
/// block bored by a post whose axis leans 1e-3° off X, then by a post
/// along X. A plane of the leaning post crosses the block's side at a sine
/// of 1.7e-5, a real angle, near the block's corner, and the two part far
/// past the tolerance across the block: a corner within the tolerance of
/// both near the line they cross along is not on both. It held on
/// `34a1b64`.
#[test]
fn seed_5520271162_a_block_bored_by_a_post_leaning_1e_3_degrees_off_its_side_then_by_a_square_one()
{
    random_solids::holds_through_the_application(&Case::new(
        Leaf::prism(
            Plane::xy(35.0),
            Outline::rectangle([0.0, 0.0], [40.0, 28.0]),
            13.0,
        ),
        vec![
            Step::cut(Leaf::turned(
                Plane::xy(15.0),
                Axis::first(0.0).leaning(0.001),
                Section::bands(20.0, &[[20.0, 0.0, 30.0], [4.0, 0.0, 20.0]]),
                360.0,
            )),
            Step::cut(Leaf::turned(
                Plane::xy(40.0),
                Axis::first(0.0),
                Section::bands(-10.0, &[[13.0, 0.0, 15.0]]),
                360.0,
            )),
        ],
    ));
}

/// #552, the tilted campaign, shrunk: a disc joined to a block at its
/// edge, cut by a slab whose floor leans 1e-7° about X off the disc's
/// bottom, along a line touching the disc's rim. Every corner where their
/// faces meet stands within the tolerance of both, but the rim's far side,
/// three hundred units from that line, stands 5.2e-7 off the floor, a hair
/// past the tolerance of 5.1e-7: the two planes cross where their faces
/// meet, and do not graze. It held on `34a1b64`.
#[test]
fn seed_5527004090_a_disc_and_a_block_cut_by_a_slab_whose_floor_leans_1e_7_degrees_off_the_disc_s_bottom()
 {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(
            Plane::xy(-30.0),
            Outline::circle([300.0, 30.0], 150.0),
            60.0,
        ),
        vec![
            Step::add(Leaf::prism(
                Plane::xy(-60.0),
                Outline::rectangle([450.0, 0.0], [510.0, 60.0]),
                120.0,
            )),
            Step::cut(Leaf::prism(
                Plane::tilted([300.0, 180.0, -30.0], [-1e-7, 0.0, 0.0]),
                Outline::rectangle([-165.0, -330.0], [165.0, 15.0]),
                30.0,
            )),
        ],
    ));
}

/// #552, the tilted campaign, shrunk: a block joined to a post standing
/// apart below it, cut by a slab whose far face leans 1e-7° about Y off
/// the post's bottom, along a line beyond the post, the family of
/// 5527004090: the rim's far side, twelve units from that line, stands
/// 2.2e-8 off the slab's face, past the tolerance of 1.6e-8. It held on
/// `34a1b64`.
#[test]
fn seed_5527006753_a_block_and_a_post_cut_by_a_slab_whose_far_face_leans_1e_7_degrees_off_the_post_s_bottom()
 {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(
            Plane::xy(6.0),
            Outline::rectangle([8.0, 7.0], [16.0, 10.0]),
            6.0,
        ),
        vec![
            Step::add(Leaf::prism(
                Plane::xy(4.99999998),
                Outline::circle([2.0, 9.0], 4.0),
                -6.0,
            )),
            Step::cut(Leaf::prism(
                Plane::tilted([10.5, 8.0, -1.999999987845058e-8], [0.0, 1e-7, 0.0]),
                Outline::rectangle([-10.0, -5.0], [1.0, 5.0]),
                -1.0,
            )),
        ],
    ));
}

/// #552, the tilted campaign: a post along X cut by a block whose floor
/// stands 2e-7 inside its top ruling, leaning 2e-8° — under the angle the
/// kernel reads as parallel to the axis, so the floor cuts the wall along
/// two rulings 1.3e-3 apart, drawn through the foot of the wall's origin.
/// At the post's ends the floor stands a fraction of the tolerance higher,
/// and the line it shares with an end crossed the rim microns aside of the
/// rulings: the corners were found twice, and the wall kept the rims of the
/// sliver the floor takes off while the ends kept the chords across it. It
/// failed on `9e54a71` the same way. The chord and the rim now cross the
/// wall and the floor where the rulings cross the end.
#[test]
fn seed_5520135751_a_post_cut_by_a_floor_2e_7_inside_its_top_ruling() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(Plane::yz(4.5), Outline::circle([4.0, 10.0], 1.0), 1.0),
        vec![Step::cut(Leaf::prism(
            Plane::tilted([5.0, 4.0, 10.9999998], [0.0, -2e-8, 0.0]),
            Outline::rectangle([-2.0, -2.0], [1.0, 2.0]),
            3.0,
        ))],
    ));
}

/// #552, the tilted campaign through the application's body: a post cut by
/// a block whose floor stands 1e-7 inside its ruling, leaning 2e-8°, the
/// family of 5520135751.
#[test]
fn seed_5520203207_a_post_cut_by_a_floor_1e_7_inside_its_ruling() {
    random_solids::holds_through_the_application(&Case::new(
        Leaf::prism(Plane::yz(-2.0), Outline::circle([0.0, 3.0], 3.0), 10.0),
        vec![Step::cut(Leaf::prism(
            Plane::tilted([-2.0, 0.0, 1e-7], [0.0, -2e-8, 0.0]),
            Outline::rectangle([-1.0, -4.0], [11.0, 4.0]),
            1.0,
        ))],
    ));
}

/// #552, the tilted campaign: a rounded block cut by a plane leaning 15°
/// that passes 1e-8 from the edge where its end meets its top, along the
/// rounded corners' walls as two ellipses. The cut takes a sliver 4e-8 long
/// off the top's edge; the walls keep lines of that length the top does not.
/// `9e54a71` declined it as asking for a conic.
#[test]
#[ignore = "hair-from-tangent, #552's tilted triage: a plane at a slant a hair from an edge, conics a hair from where the walls end (the issue's second default: no band is added); declined as unverified"]
fn seed_5520109275_a_rounded_block_cut_at_15_degrees_1e_8_from_its_end_s_edge() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(
            Plane::yz(2.0),
            Outline::rounded([2.0, 7.0], [7.0, 15.0], 2.0),
            8.5,
        ),
        vec![Step::cut(Leaf::prism(
            Plane::tilted(
                [10.499999996497152, 7.0, 15.000000009366433],
                [0.0, -15.0, 0.0],
            ),
            Outline::rectangle([-12.0, -6.0], [1.0, 1.0]),
            4.0,
        ))],
    ));
}

/// #552, the tilted campaign: a block cut by a plane leaning 30° that
/// passes 1e-8 above the edge where its end meets its floor, then by a
/// block standing on its top. The first cut leaves the edge's corners where
/// the block had them, on the slanted plane within the tolerance, though
/// that plane, the floor and a side meet 2e-8 beyond, and the end face they
/// were decided on is gone. The second cut moved them there, off the edge
/// they end, and the floor lost its last edge. It failed on `9e54a71` the
/// same way.
#[test]
fn seed_5520107844_a_block_cut_1e_8_above_its_floor_s_edge_at_30_degrees_then_from_its_top() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(
            Plane::yz(1.5),
            Outline::rectangle([6.0, 9.0], [13.0, 11.0]),
            5.0,
        ),
        vec![
            Step::cut(Leaf::prism(
                Plane::tilted(
                    [6.5000000058340115, 6.0, 9.000000008121841],
                    [0.0, 30.0, 0.0],
                ),
                Outline::rectangle([-6.0, -1.0], [1.0, 7.0]),
                2.0,
            )),
            Step::cut(Leaf::prism(
                Plane::xy(11.0),
                Outline::rectangle([-4.0, -3.0], [4.0, 4.0]),
                5.0,
            )),
        ],
    ));
}

/// #552, the tilted campaign through the application's body: a slab cut by
/// a plane leaning 75° a hair from the edge where its end meets its floor,
/// then by a block resting on its top, the family of 5520107844.
#[test]
fn seed_5520210646_a_slab_cut_a_hair_from_its_floor_s_edge_at_75_degrees_then_from_its_top() {
    random_solids::holds_through_the_application(&Case::new(
        Leaf::prism(
            Plane::yz(6.0),
            Outline::rectangle([6.0, 10.0], [10.0, 17.0]),
            1.0,
        ),
        vec![
            Step::cut(Leaf::prism(
                Plane::tilted([6.999999990667421, 6.0, 9.99999999640793], [0.0, 75.0, 0.0]),
                Outline::rectangle([-8.0, -1.0], [1.0, 4.0]),
                -2.0,
            )),
            Step::cut(Leaf::prism(
                Plane::xy(17.0),
                Outline::rectangle([-1.0, -3.0], [0.0, 3.0]),
                2.0,
            )),
        ],
    ));
}

/// 5541124475 as drawn, #552's tilted campaign on the exact kernel. Held on
/// `34a1b64` and failed since the grazing planes (`cecca2f`); its shrunk core
/// below failed on `34a1b64` too. Both hold once a plane carries an edge's
/// line only where the line stays within the tolerance of it.
#[test]
fn seed_5541124475_as_drawn_a_block_leaning_1e_7_degrees_joined_to_two_square_blocks() {
    random_solids::holds_exactly(&Case::drawn_tilted_off_the_lattice(5541124475));
}

/// A block leaning 1e-7° about Y joined to a square block, then to a wider
/// one. The leaning block's floor is decided to graze the first block's at
/// the first join; at the second, one pair of the leaning block's faces
/// stands 1.25 tolerances apart across the box their faces may meet in: not
/// grazing, and too near parallel to fix the line they share. The first
/// join's edge along the floor, beside the square block's floor over its
/// stretch, was laid on that floor all along, though the leaning floor
/// parts from it by 1.25 tolerances at the far end, and the overlay of a
/// face found a tie. Failed on `34a1b64` too. The square floor now carries
/// the edge over its stretch alone.
#[test]
fn seed_5541124475_a_block_leaning_1e_7_degrees_joined_to_two_square_blocks() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(
            Plane::tilted([20.0, 5.0, 0.0], [0.0, 1e-7, 0.0]),
            Outline::rectangle([25.0, 13.0], [50.0, 23.0]),
            8.0,
        ),
        vec![
            Step::add(Leaf::prism(
                Plane::xy(0.0),
                Outline::rectangle([28.0, 3.0], [53.0, 28.0]),
                15.0,
            )),
            Step::add(Leaf::prism(
                Plane::xy(0.0),
                Outline::rectangle([20.0, -5.0], [70.0, 45.0]),
                13.0,
            )),
        ],
    ));
}

/// #552's tilted campaign through the application's body: a rounded block
/// joined to a block leaning 1e-7° about Y, then cut by a block and a cone.
/// A face of each is decided to graze the other's at the join, and the cone
/// then crosses a curve the kernel finds no crossing with: declined as
/// unsupported
/// (combine/crossed.rs). Holds on `34a1b64`, fails since the grazing planes
/// (`cecca2f`).
/// Holds since a plane a hair off square to, or along, a wall's axis is
/// laid exactly so, its operand turned with it (decision 11).
#[test]
fn seed_5541221936_a_rounded_block_joined_to_a_block_leaning_1e_7_degrees_then_cut_by_a_block_and_a_cone()
 {
    random_solids::holds_through_the_application(&Case::new(
        Leaf::prism(
            Plane::xy(240.0),
            Outline::rounded([150.0, 255.0], [270.0, 345.0], 15.0),
            195.0,
        ),
        vec![
            Step::add(Leaf::prism(
                Plane::tilted([225.0, 255.0, 405.0], [0.0, 1e-7, 0.0]),
                Outline::rectangle([-60.0, -60.0], [60.0, 75.0]),
                45.0,
            )),
            Step::cut(Leaf::prism(
                Plane::tilted([225.0, 300.0, 360.0], [90.0, 0.0, 0.0]),
                Outline::rectangle([-30.0, 0.0], [120.0, 45.0]),
                75.0,
            )),
            Step::cut(Leaf::turned(
                Plane::xy(195.0),
                Axis::first(0.0),
                Section::bands(90.0, &[[45.0, 0.0, 69.0]]).sloping_to(&[[0.0, 75.0]]),
                360.0,
            )),
        ],
    ));
}

/// 5541236206 as drawn, #552's tilted campaign through the application's
/// body. Holds on `34a1b64` and fails since the grazing planes (`cecca2f`);
/// its shrunk core below fails on `34a1b64` too.
/// Holds since a plane a hair off square to, or along, a wall's axis is
/// laid exactly so, its operand turned with it (decision 11).
#[test]
fn seed_5541236206_as_drawn_a_post_cut_by_a_block_whose_face_leans_2e_7_degrees_off_its_end() {
    random_solids::holds_through_the_application(&Case::drawn_tilted_off_the_lattice(5541236206));
}

/// A post along X cut by a block whose face leans 2e-7° off the post's end
/// and stands 5e-9 past it. The face and the end stand 1.2 tolerances apart
/// across their box: they neither graze nor fix the line they share, and
/// the corners the post's wall makes with each are kept apart. A scratch
/// waiver in `parted` (canonical/corners.rs) for a place within the
/// tolerance of both makes it hold. Fails on `34a1b64` too.
/// Holds since a plane a hair off square to, or along, a wall's axis is
/// laid exactly so, its operand turned with it (decision 11).
#[test]
fn seed_5541236206_a_post_cut_by_a_block_whose_face_leans_2e_7_degrees_off_its_end() {
    random_solids::holds_through_the_application(&Case::new(
        Leaf::turned(
            Plane::xy(5.0),
            Axis::first(0.0),
            Section::bands(4.0, &[[6.0, -2.0, -0.0]]),
            360.0,
        ),
        vec![Step::cut(Leaf::prism(
            Plane::tilted(
                [10.000000005342956, 3.333333831179175e-17, 5.0],
                [0.0, -89.99999980901404, 0.0],
            ),
            Outline::rectangle([-3.0, -1.0], [3.0, 3.0]),
            1.0,
        ))],
    ));
}

/// #552's tilted campaign through the application's body, round 0's seed
/// run again: a stepped double cone cut by a block 1.8e-6 off the lattice,
/// then by a plane leaning 4e-8° off square to its axis. The plane is
/// decided to graze another plane of the part, and two conics on the cone
/// and two lines are left with one face each. Holds on `34a1b64` and with
/// the hair lane alone (`25e824f`), fails with the grazing planes
/// (`cecca2f`).
/// Holds since a plane a hair off square to, or along, a wall's axis is
/// laid exactly so, its operand turned with it (decision 11).
#[test]
fn seed_5520207605_a_stepped_double_cone_cut_by_a_block_off_the_lattice_and_a_plane_leaning_4e_8_degrees()
 {
    random_solids::holds_through_the_application(&Case::new(
        Leaf::turned(
            Plane::xy(210.0),
            Axis::second(0.0),
            Section::bands(90.0, &[[15.0, 0.0, 30.0], [105.0, 0.0, 30.0]])
                .sloping_to(&[[0.0, 45.0], [0.0, 135.0]]),
            360.0,
        ),
        vec![
            Step::cut(Leaf::prism(
                Plane::yz(29.9999982),
                Outline::rectangle([75.0, 60.0], [285.0, 360.0]),
                180.0,
            )),
            Step::cut(Leaf::prism(
                Plane::tilted([0.0, 105.0, 165.0], [-89.99999996, 0.0, 0.0]),
                Outline::rectangle([-150.0, -210.0], [150.0, 120.0]),
                195.0,
            )),
        ],
    ));
}

/// #552's turned campaign through the application's body, round 0's seed
/// run again: a post whose axis leans 1e-7 joined to two blocks. The post's
/// end is decided to graze a block's face at each join, and a dozen of the
/// edges round them were left with one face each. Held on `34a1b64` and
/// with the hair lane alone (`25e824f`), failed with the grazing planes
/// (`cecca2f`); holds once a plane carries an edge's line only where the
/// line stays within the tolerance of it.
#[test]
fn seed_5520600321_a_post_whose_axis_leans_1e_7_joined_to_two_blocks() {
    random_solids::holds_through_the_application(&Case::new(
        Leaf::turned(
            Plane::yz(5.0),
            Axis::second(5.0).backwards().leaning(1e-7),
            Section::bands(-2.0, &[[6.0, 0.0, 6.0]]),
            360.0,
        ),
        vec![
            Step::add(Leaf::prism(
                Plane::xy(2.0),
                Outline::rectangle([4.0, 10.0], [9.0, 15.0]),
                4.0,
            )),
            Step::add(Leaf::prism(
                Plane::xy(1.0),
                Outline::rectangle([6.0, 6.0], [8.0, 13.0]),
                8.0,
            )),
        ],
    ));
}

/// #552's tilted campaign through the application's body: a block leaning
/// 1e-3° about Y cut by two blocks and a post. The square blocks' faces
/// cross the leaning block's at 1e-3°, under the angle that fixes the line
/// two planes share; an operand's own corner on them stands where its
/// operand built it (`placed`, canonical/corners.rs), and edges 1.5e-4 long
/// were left with one face each. Held on `34a1b64` and with the grazing
/// planes alone (`cecca2f`), failed with the hair lane (`25e824f`). The
/// cause was the support that corner was given: the first cut's corner line
/// was laid on the leaning side all along, from an edge beside that side
/// over its last 1.5e-4 alone, and a corner at the cut's floor, twenty
/// tolerances off the side, stood on the line it shares with a wall.
/// Carried over that stretch alone, the side holds no such corner.
#[test]
fn seed_5541239472_a_block_leaning_1e_3_degrees_cut_by_two_blocks_and_a_post() {
    random_solids::holds_through_the_application(&Case::new(
        Leaf::prism(
            Plane::tilted([25.0, 15.0, 25.0], [0.0, -0.001, 0.0]),
            Outline::rectangle([-5.0, 5.0], [17.5, 30.0]),
            -43.0,
        ),
        vec![
            Step::cut(Leaf::prism(
                Plane::xy(20.0),
                Outline::rectangle([15.0, 30.0], [42.5, 42.5]),
                40.0,
            )),
            Step::cut(Leaf::prism(
                Plane::xy(40.0),
                Outline::rectangle([28.0, 18.0], [53.0, 43.0]),
                23.0,
            )),
            Step::cut(Leaf::turned(
                Plane::xy(43.0),
                Axis::first(0.0),
                Section::bands(5.0, &[[25.0, 0.0, 13.0]]),
                360.0,
            )),
        ],
    ));
}

/// #552's tilted campaign: a block cut by a block whose face leans 1e-5°
/// about X off the first's side and crosses it halfway up. The two faces
/// cross along a line inside both, the wedge between them up to sixteen
/// tolerances thick: too far apart to graze, too near parallel to fix that
/// line, and the wedge's edges are left with one face each: the side's
/// corner line crossing the face and the face's line crossing the side,
/// each at the shallow angle of the two faces, stand 1.5 tolerances apart
/// along it. Fails on `34a1b64` too. Crossing both through the line the
/// faces share makes it hold, but moved other corners of fresh seeds off
/// where their other surfaces fix them.
#[test]
#[ignore = "near-parallel-planes, #552's round 2 triage: two faces crossing at 1e-5 degrees inside both leave a wedge whose edges each keep one face; declined as unverified"]
fn seed_5541126238_a_block_cut_by_a_block_whose_face_leans_1e_5_degrees_off_its_side() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(
            Plane::xy(2.0),
            Outline::rectangle([7.0, 6.5], [13.5, 9.5]),
            7.0,
        ),
        vec![Step::cut(Leaf::prism(
            Plane::tilted([8.5, 6.5, 7.75], [89.99999, 0.0, 0.0]),
            Outline::rectangle([-3.0, -2.0], [3.0, 3.0]),
            5.0,
        ))],
    ));
}

/// #552's tilted campaign: a turn of 315° about X cut by a block whose floor
/// leans 1e-7° about Y off the plane the turn starts in. The floor is
/// decided to graze the turn's start face, and four lines on them are left
/// with one face each. Fails on `34a1b64` too.
/// Holds since a plane a hair off square to, or along, a wall's axis is
/// laid exactly so, its operand turned with it (decision 11).
#[test]
fn seed_5541139119_a_turn_of_315_degrees_cut_by_a_block_whose_floor_leans_1e_7_degrees_off_its_start()
 {
    random_solids::holds_exactly(&Case::new(
        Leaf::turned(
            Plane::xy(1.0),
            Axis::first(0.0),
            Section::bands(-2.0, &[[4.0, 0.0, 5.0]]),
            315.0,
        ),
        vec![Step::cut(Leaf::prism(
            Plane::tilted([-4.0, 0.0, 1.0], [0.0, 1e-7, 0.0]),
            Outline::rectangle([-7.0, -6.0], [7.0, 6.0]),
            6.0,
        ))],
    ));
}

/// #552's tilted campaign: a post along Z cut by a block whose sides stand
/// at the post's radius, leaning 1e-5° about X. Each side cuts the wall
/// along an ellipse whose vertex, in the box, turns on a radius of fifty
/// tolerances. `34a1b64` fails it too.
#[test]
#[ignore = "hair-from-tangent, #552's round 2 triage: a plane at a slant a hair from tangent to a wall cuts it along an ellipse turning within fifty tolerances (the issue's second default: no band is added); its corners stand on the hairpin since the triple is decided there, and the sliver it leaves, thinner than the rules' room towards its tip, is drawn crossing itself"]
fn seed_5541111069_a_post_cut_by_a_block_whose_sides_touch_its_wall_leaning_1e_5_degrees() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(Plane::xy(-1.0), Outline::circle([3.0, 4.0], 1.5), 5.0),
        vec![Step::cut(Leaf::prism(
            Plane::tilted([3.0, 4.0, 1.0], [-1e-5, 0.0, 0.0]),
            Outline::rectangle([-1.0, -1.5], [1.0, 1.5]),
            -3.0,
        ))],
    ));
}

/// #552's tilted campaign: a frustum about Y cut by a block whose side holds
/// the axis's direction 1e-7 inside the frustum's wide rim, along a
/// hyperbola whose vertex stands that hair from the rim. `34a1b64` fails it
/// too.
#[test]
fn seed_5541109790_a_frustum_cut_along_its_axis_1e_7_inside_its_wide_rim() {
    random_solids::holds_exactly(&Case::new(
        Leaf::turned(
            Plane::xy(0.0),
            Axis::second(0.0),
            Section::bands(5.0, &[[2.5, 0.0, 12.5]]).sloping_to(&[[0.0, 5.0]]),
            360.0,
        ),
        vec![Step::cut(Leaf::prism(
            Plane::yz(-12.4999999),
            Outline::rectangle([3.0, -18.0], [48.0, 18.0]),
            8.0,
        ))],
    ));
}

/// #552's round 3, through the application's body: a post along X cut by a
/// block whose face leans 4e-8° off parallel to the post's axis. Held on
/// `7411e98` and `0d019ac`; from `cbf3c3c` the hairpin moved the two places
/// the block's edge crosses the post, a dozen tolerances apart, within the
/// tolerance of each other, one corner, and a circle and a conic were left
/// with one face each. The edge's two crossings stay two
/// (relation/crossing/grazed.rs).
#[test]
fn seed_5542230229_a_post_cut_by_a_block_whose_face_leans_4e_8_degrees_off_its_axis() {
    random_solids::holds_through_the_application(&Case::new(
        Leaf::prism(Plane::yz(5.0), Outline::circle([6.0, 2.0], 5.5), 10.0),
        vec![Step::cut(Leaf::prism(
            Plane::tilted([8.0, 3.0, 2.0], [90.0, 89.99999996, 0.0]),
            Outline::rectangle([-9.0, -7.0], [8.5, 7.0]),
            9.0,
        ))],
    ));
}

/// #552's round 3 on the exact kernel: a double cone about Y whose first
/// band starts 2e-8 off its axis, cut by a block leaning 45°, the cones'
/// own slope, a tolerance under a rim. Held on `7411e98` and `0d019ac`;
/// from `cbf3c3c`, where the cone did not see the line the block shares
/// with the rim's flat straight, the rim's arc was kept for the arc the
/// three are, and the block's face saw it only as its chord, declined as
/// unsupported (domain/traced.rs). The conic is kept (combine/identified.rs).
#[test]
fn seed_5542125645_a_double_cone_2e_8_off_its_axis_cut_by_a_block_leaning_45_degrees() {
    random_solids::holds_exactly(&Case::new(
        Leaf::turned(
            Plane::xy(-1.0),
            Axis::second(6.0),
            Section::bands(
                5.0,
                &[[1.9999999989472883e-8, 0.0, 1.5], [0.99999998, 0.0, 1.5]],
            )
            .sloping_to(&[[0.0, 1.5], [0.0, 2.49999998]]),
            360.0,
        ),
        vec![Step::cut(Leaf::prism(
            Plane::tilted([6.0, 6.5, -1.0], [-45.0, 0.0, 0.0]),
            Outline::rectangle([-3.0, -3.5], [3.0, 4.5]),
            -4.0,
        ))],
    ));
}

/// #552's round 3, through the application's body: a cone about Y cut by a
/// block leaning 45° whose corner stands 2e-7 off the lattice. Held on
/// `7411e98` and `0d019ac`; from `cbf3c3c` a rim's arc was kept for the arc
/// it is with a line and a conic, as in 5542125645, and declined as
/// unsupported where the block's face saw it (domain/traced.rs). The conic
/// is kept (combine/identified.rs).
#[test]
fn seed_5542217385_a_cone_cut_by_a_block_leaning_45_degrees_whose_corner_stands_2e_7_off() {
    random_solids::holds_through_the_application(&Case::new(
        Leaf::turned(
            Plane::xz(210.0),
            Axis::second(180.0),
            Section::bands(-75.0, &[[60.0, -105.0, 15.0]]).sloping_to(&[[-15.0, 15.0]]),
            360.0,
        ),
        vec![Step::cut(Leaf::prism(
            Plane::tilted(
                [75.00000020839751, 210.0, -74.99999978419805],
                [0.0, 45.0, 0.0],
            ),
            Outline::rectangle([-165.0, -135.0], [180.0, 135.0]),
            30.0,
        ))],
    ));
}

/// #552's round 3 on the exact kernel: a block leaning 1e-7° about Y joined
/// to a post along Z whose wall its side stands 3.5e-9 from: the side cuts
/// the wall along a hairpin a hair from tangent, and edges on both are left
/// with one face. It holds where the side is read along the post's axis —
/// a band the issue's second default rules out. Fails on `7411e98` too.
/// Holds since a plane a hair off square to, or along, a wall's axis is
/// laid exactly so, its operand turned with it (decision 11).
#[test]
fn seed_5542100114_a_block_leaning_1e_7_degrees_joined_to_a_post_its_side_nearly_touches() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(
            Plane::tilted([5.0, 6.0, 2.0], [0.0, -1e-7, 0.0]),
            Outline::rectangle([2.0, -3.0], [8.5, 2.5]),
            5.0,
        ),
        vec![Step::add(Leaf::prism(
            Plane::xy(2.0),
            Outline::circle([8.5, 3.0], 5.0),
            4.0,
        ))],
    ));
}

/// #552's round 3 on the exact kernel: a slot along Z cut by a block
/// leaning 60° whose corner stands 9e-9 inside the slot's flat side: the
/// conics the block cuts the round ends along pass a hair from the edges
/// where the ends meet that side, and are left with one face each. Fails
/// on `7411e98` too.
#[test]
#[ignore = "hair-from-tangent, #552's round 3 triage: conics cut 9e-9 from the edge where a slot's round end meets its side; declined as unverified"]
fn seed_5542121248_a_slot_cut_at_60_degrees_by_a_block_whose_corner_stands_9e_9_inside_its_side() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(
            Plane::xy(6.0),
            Outline::slot([10.0, 4.0], [10.0, 2.0], 1.5),
            1.5,
        ),
        vec![Step::cut(Leaf::prism(
            Plane::tilted(
                [11.499999991089934, 5.5, 7.499999995460095],
                [0.0, 60.0, 0.0],
            ),
            Outline::rectangle([-2.0, -6.0], [2.0, 1.0]),
            4.0,
        ))],
    ));
}

/// #552's round 3 on the exact kernel: a post along Y cut by a block
/// whose floor leans 1e-7° on the post's end: the floor cuts the wall along
/// an ellipse a hair from the end's rim, and the sliver of wall between
/// the two is drawn through the end. Fails on `7411e98` too.
/// Holds since a plane a hair off square to, or along, a wall's axis is
/// laid exactly so, its operand turned with it (decision 11).
#[test]
fn seed_5542133393_a_post_cut_by_a_block_whose_floor_leans_1e_7_degrees_on_its_end() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(Plane::xz(-1.0), Outline::circle([5.0, 5.0], 4.0), 10.0),
        vec![Step::cut(Leaf::prism(
            Plane::tilted([1.0, -1.0, 5.0], [90.0, -1e-7, 0.0]),
            Outline::rectangle([-1.0, -5.0], [9.0, 5.0]),
            11.0,
        ))],
    ));
}

/// #552's round 3 on the exact kernel: a rounded block cut by a block
/// leaning 1e-5° about Y from its top: the leaning faces and the block's
/// cross too near parallel to fix their line, without grazing, and the
/// overlay of a face finds a tie. Fails on `7411e98` too. Holds since a
/// place within rounding of two planes too near parallel to fix their line
/// is on it, however far from it the line was computed: the corner the
/// block's side and round make with the top, and the one they make with
/// the leaning face, are one (canonical/corners.rs).
#[test]
fn seed_5542123080_a_rounded_block_cut_by_a_block_leaning_1e_5_degrees() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(
            Plane::xy(2.0),
            Outline::rounded([10.0, 1.0], [12.0, 6.0], 0.5),
            5.0,
        ),
        vec![Step::cut(Leaf::prism(
            Plane::tilted([10.5, 3.0, 7.0], [0.0, -1e-5, 0.0]),
            Outline::rectangle([-2.0, -3.0], [2.0, 4.0]),
            6.0,
        ))],
    ));
}

/// #552's round 3 on the exact kernel: a disc cut by a block leaning 4e-8°
/// about Y whose floor lies on the disc's: the two floors, a hair apart,
/// are drawn through each other. Fails on `7411e98` too.
/// Holds since a plane a hair off square to, or along, a wall's axis is
/// laid exactly so, its operand turned with it (decision 11).
#[test]
fn seed_5542100263_a_disc_cut_by_a_block_leaning_4e_8_degrees_on_its_floor() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(Plane::xy(7.0), Outline::circle([1.0, 9.0], 4.5), 3.0),
        vec![Step::cut(Leaf::prism(
            Plane::tilted([6.0, 9.0, 7.0], [0.0, 4e-8, 0.0]),
            Outline::rectangle([-10.0, -5.0], [1.0, 5.0]),
            3.0,
        ))],
    ));
}

/// #552's round 3, through the application's body: a block cut by a block
/// leaning 1e-5° about Y whose floor lies on the first block's floor: edges
/// between planes alone are left with one face each. Fails on `7411e98`
/// too.
#[test]
#[ignore = "near-parallel-planes, #552's round 3 triage: a floor leaning 1e-5 degrees on a block's floor; declined as unverified"]
fn seed_5542234414_a_block_cut_by_a_block_leaning_1e_5_degrees_on_its_floor() {
    random_solids::holds_through_the_application(&Case::new(
        Leaf::prism(
            Plane::xy(5.0),
            Outline::rectangle([8.0, 5.0], [16.0, 11.0]),
            8.5,
        ),
        vec![Step::cut(Leaf::prism(
            Plane::tilted([12.0, 5.0, 5.0], [0.0, 1e-5, 0.0]),
            Outline::rectangle([-1.0, 0.0], [2.0, 5.0]),
            8.5,
        ))],
    ));
}

/// #552's round 3 on the exact kernel: a slot along Y cut by a block turned
/// 1e-7° short of a quarter turn about Y: one pair of its faces and the
/// slot's is decided to graze, another of the same lean is not, and the
/// overlay of a face finds a tie. Fails on `7411e98` too.
#[test]
#[ignore = "near-parallel-planes, #552's round 3 triage: two pairs of planes of one lean, one decided to graze and one not; declined as a tie in a face's overlay"]
fn seed_5542111371_a_slot_cut_by_a_block_turned_1e_7_degrees_short_of_a_quarter_turn() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(
            Plane::xz(5.0),
            Outline::slot([0.0, 2.5], [0.0, -1.0], 2.5),
            4.0,
        ),
        vec![Step::cut(Leaf::prism(
            Plane::tilted([-2.5, 3.5, -1.0], [0.0, 89.9999999, 0.0]),
            Outline::rectangle([-4.0, -4.0], [4.0, 4.0]),
            6.0,
        ))],
    ));
}

/// #552's round 4 on the exact kernel: a disc cut by a block whose floor
/// leans 1e-7° about Y through the leftmost point of the disc's bottom rim:
/// the two floors, too near parallel to fix their line and not decided to
/// graze, are drawn through each other.
/// Holds since a plane a hair off square to, or along, a wall's axis is
/// laid exactly so, its operand turned with it (decision 11).
#[test]
fn seed_5543135340_a_disc_cut_by_a_block_whose_floor_leans_1e_7_degrees_through_its_rim() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(Plane::xy(0.0), Outline::circle([0.0, 7.0], 5.0), -7.0),
        vec![Step::cut(Leaf::prism(
            Plane::tilted([-5.0, 7.0, -7.0], [0.0, -1e-7, 0.0]),
            Outline::rectangle([-1.0, -6.0], [11.0, 6.0]),
            8.0,
        ))],
    ));
}

/// #552's round 4 on the exact kernel, planes alone: a block cut by a block
/// whose floor leans 1e-5° about Y across the first's top, the two crossing
/// halfway along it: edges between the planes are left with one face each.
#[test]
#[ignore = "near-parallel-planes, #552's round 4 triage: a floor leaning 1e-5 degrees across a block's top; declined as unverified"]
fn seed_5543139042_a_block_cut_by_a_block_whose_floor_leans_1e_5_degrees_across_its_top() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(
            Plane::yz(6.0),
            Outline::rectangle([4.0, 9.0], [11.5, 16.5]),
            4.0,
        ),
        vec![Step::cut(Leaf::prism(
            Plane::tilted([9.0, 9.0, 16.5], [0.0, -1e-5, 0.0]),
            Outline::rectangle([-3.0, -2.0], [1.0, 2.0]),
            1.0,
        ))],
    ));
}

/// #552's round 4, through the application's body: a slot along X cut by a
/// block leaning 4e-8° about X whose side lies on the slot's flat side:
/// edges between the planes are left with one face each.
/// Holds since a plane a hair off square to, or along, a wall's axis is
/// laid exactly so, its operand turned with it (decision 11).
#[test]
fn seed_5543226436_a_slot_cut_by_a_block_whose_side_leans_4e_8_degrees_on_its_flat_side() {
    random_solids::holds_through_the_application(&Case::new(
        Leaf::prism(
            Plane::xy(4.0),
            Outline::slot([10.0, 4.0], [7.0, 4.0], 1.0),
            8.0,
        ),
        vec![Step::cut(Leaf::prism(
            Plane::tilted([7.5, 4.5, 3.0], [-4e-8, 0.0, 0.0]),
            Outline::rectangle([-2.5, -1.5], [4.0, 8.0]),
            3.0,
        ))],
    ));
}

/// #552's round 4 on the exact kernel: a cylinder turned about Y cut by a
/// block turned 1e-7° short of a quarter turn about X: its face a hair off
/// square to the axis cuts the wall along an ellipse a hair from a rim, and
/// the rim, that ellipse and edges between planes are left with one face
/// each.
/// Holds since a plane a hair off square to, or along, a wall's axis is
/// laid exactly so, its operand turned with it (decision 11).
#[test]
fn seed_5543106272_a_cylinder_cut_by_a_block_turned_1e_7_degrees_short_of_a_quarter_turn() {
    random_solids::holds_exactly(&Case::new(
        Leaf::turned(
            Plane::xy(-1.0),
            Axis::second(0.0).backwards(),
            Section::bands(3.0, &[[0.5, 0.0, 3.5]]),
            360.0,
        ),
        vec![Step::cut(Leaf::prism(
            Plane::tilted([0.0, -3.5, -4.0], [89.9999999, 0.0, 0.0]),
            Outline::rectangle([-1.0, -1.0], [4.0, 4.0]),
            1.0,
        ))],
    ));
}

/// #552's round 4, through the application's body: a ring cut by a block
/// leaning 1e-7° about X whose floor lies on the ring's: the two floors, a
/// hair apart, are drawn through each other. It holds where a plane within
/// about 1e-3° of square to a wall's axis is read square to it — a band the
/// issue's second default rules out, used here only to locate the fault.
/// Holds since a plane a hair off square to, or along, a wall's axis is
/// laid exactly so, its operand turned with it (decision 11).
#[test]
fn seed_5543205130_a_ring_cut_by_a_block_leaning_1e_7_degrees_on_its_floor() {
    random_solids::holds_through_the_application(&Case::new(
        Leaf::prism(Plane::xy(-2.0), Outline::ring([2.0, 5.0], 2.0, 0.5), 5.0),
        vec![Step::cut(Leaf::prism(
            Plane::tilted([2.0, 4.5, -2.0], [1e-7, 0.0, 0.0]),
            Outline::rectangle([-3.0, -2.0], [3.0, 4.0]),
            6.0,
        ))],
    ));
}

/// #552's round 4, through the application's body: a slot along X cut by a
/// block leaning 1e-7° about Y whose floor lies on the slot's top and whose
/// side stands at the far point of the slot's round end: the side, a hair
/// from tangent to that end, cuts it along a hairpin ellipse, and edges on
/// the wall and the planes are left with one face each. It holds where the
/// side is read along the end's axis — a band the issue's second default
/// rules out.
/// Holds since a plane a hair off square to, or along, a wall's axis is
/// laid exactly so, its operand turned with it (decision 11).
#[test]
fn seed_5543238102_a_slot_cut_by_a_block_whose_side_leans_1e_7_degrees_at_its_round_end() {
    random_solids::holds_through_the_application(&Case::new(
        Leaf::prism(
            Plane::xy(-1.0),
            Outline::slot([4.0, 7.0], [1.0, 7.0], 3.0),
            5.5,
        ),
        vec![Step::cut(Leaf::prism(
            Plane::tilted([-1.0, 6.0, 4.5], [0.0, -1e-7, 0.0]),
            Outline::rectangle([-1.0, -8.0], [7.0, 1.0]),
            6.0,
        ))],
    ));
}

/// #552's round 4 on the exact kernel: a rounded block along Y cut by a
/// block leaning 1e-7° about X whose floor lies on the first's bottom, a
/// hair from tangent to its rounded edges: the floor cuts their walls along
/// hairpin ellipses, a stretch traced on a wall stands off its chord, and
/// edges on the walls are left with one face each.
/// Holds since a plane a hair off square to, or along, a wall's axis is
/// laid exactly so, its operand turned with it (decision 11).
#[test]
fn seed_5543114074_a_rounded_block_cut_by_a_block_whose_floor_leans_1e_7_degrees_on_its_bottom() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(
            Plane::xz(1.0),
            Outline::rounded([7.0, 7.0], [10.5, 12.5], 1.5),
            4.0,
        ),
        vec![Step::cut(Leaf::prism(
            Plane::tilted([10.0, 0.5, 7.0], [1e-7, 0.0, 0.0]),
            Outline::rectangle([-2.0, -1.0], [3.0, 5.0]),
            3.0,
        ))],
    ));
}

/// #552's round 4, through the application's body: a slot along X cut by a
/// block leaning 4e-8° about X whose side lies on the slot's flat side,
/// tangent to its round ends: a stretch traced on an end's wall stands off
/// its chord (domain/traced.rs), and where faces are selected, the rays
/// cast to decide them meet a tie (ray.rs, selection.rs).
/// Holds since a plane a hair off square to, or along, a wall's axis is
/// laid exactly so, its operand turned with it (decision 11).
#[test]
fn seed_5543216265_a_slot_cut_by_a_block_whose_side_leans_4e_8_degrees_tangent_to_its_ends() {
    random_solids::holds_through_the_application(&Case::new(
        Leaf::prism(
            Plane::xy(-1.0),
            Outline::slot([4.0, 0.0], [3.0, 0.0], 1.0),
            10.0,
        ),
        vec![Step::cut(Leaf::prism(
            Plane::tilted([5.0, -10.0, 1.0], [-4e-8, 0.0, 0.0]),
            Outline::rectangle([-4.0, -1.0], [1.0, 11.0]),
            -3.0,
        ))],
    ));
}

/// #552's round 4 on the exact kernel: a turn about X, a cylinder and a
/// cone at 45°, cut by a block leaning 45°, along the cone's rulings, whose
/// corner stands 1e-8 off the lattice: a stretch traced on a wall stands
/// off its chord, and edges on the walls are left with one face each.
#[test]
#[ignore = "hair-from-tangent, #552's round 4 triage: a face along a cone's rulings standing 1e-8 off; declined as unverified"]
fn seed_5543126616_a_cone_cut_by_a_block_along_its_rulings_whose_corner_stands_1e_8_off() {
    random_solids::holds_exactly(&Case::new(
        Leaf::turned(
            Plane::xy(1.0),
            Axis::first(5.0),
            Section::bands(-3.0, &[[2.0, -1.5, -0.0], [1.0, -1.5, -0.0]])
                .sloping_to(&[[-1.5, -0.0], [-2.5, -0.0]]),
            360.0,
        ),
        vec![Step::cut(Leaf::prism(
            Plane::tilted([-2.49999999, 5.0, 1.0], [0.0, 45.0, 0.0]),
            Outline::rectangle([-3.0, -3.0], [6.0, 3.0]),
            3.0,
        ))],
    ));
}

/// #552's round 4 on the exact kernel: a rounded block cut by a half
/// cylinder turned about an axis leaning 1e-7° off Y, its wall at the
/// tangent distance of the block's side: the side cuts the wall along a
/// hairpin ellipse, and edges on the side and on the wall are left with one
/// face each.
/// Holds since a plane a hair off square to, or along, a wall's axis is
/// laid exactly so, its operand turned with it (decision 11).
#[test]
fn seed_5543103909_a_rounded_block_cut_by_a_half_cylinder_leaning_1e_7_degrees() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(
            Plane::xy(2.0),
            Outline::rounded([0.0, 1.5], [5.5, 7.0], 0.5),
            8.0,
        ),
        vec![Step::cut(Leaf::turned(
            Plane::xy(5.0),
            Axis::second(0.0).leaning(1e-7),
            Section::bands(1.5, &[[3.5, 0.0, 5.5]]),
            180.0,
        ))],
    ));
}

/// #552's round 4, through the application's body: a half cone about X,
/// sloping at atan(1/8), cut by a block whose face touches it along a ruling
/// at exactly its slope, the apex beyond the box. The face was cut along the
/// conic, which is nothing for a doubled ruling, and the ruling's corners,
/// found once by the half turn's end plane and once by the face, were kept
/// apart: the body was refused. The face holds the ruling now, read at its
/// foot as through an apex beyond the box.
#[test]
fn seed_5543214149_a_half_cone_cut_by_a_block_whose_face_touches_it_along_a_ruling_beyond_its_apex()
{
    random_solids::holds_through_the_application(&Case::new(
        Leaf::turned(
            Plane::xy(4.0),
            Axis::first(1.0),
            Section::bands(4.0, &[[2.5, 3.0, 7.0], [5.5, 2.6875, 7.0]])
                .sloping_to(&[[3.0, 7.0], [2.0, 7.0]]),
            180.0,
        ),
        vec![Step::cut(Leaf::prism(
            Plane::tilted([28.0, 1.0, 4.0], [90.0, 7.125016348901798, 0.0]),
            Outline::rectangle([-26.0, -8.0], [-15.0, 1.0]),
            6.0,
        ))],
    ));
}

/// #552's round 4, through the application's body: a cone about Y sloping
/// at atan(2/7), cut by a block whose face touches it along a ruling at
/// exactly its slope, the apex beyond the box: refused, a tie left where the
/// ruling was cut as a conic. It holds as the seed before it.
#[test]
fn seed_5543216778_a_cone_cut_by_a_block_whose_face_touches_it_along_a_ruling_beyond_its_apex() {
    random_solids::holds_through_the_application(&Case::new(
        Leaf::turned(
            Plane::xy(20.0),
            Axis::second(30.0),
            Section::bands(-10.0, &[[17.5, -20.0, 17.5]]).sloping_to(&[[-15.0, 17.5]]),
            360.0,
        ),
        vec![Step::cut(Leaf::prism(
            Plane::tilted([30.0, 60.0, 20.0], [15.945395900922854, 0.0, 0.0]),
            Outline::rectangle([-23.0, -78.0], [23.0, -40.0]),
            43.0,
        ))],
    ));
}

/// #552's round 4, through the application's body: a half cone about Y cut
/// at 30°, then at 45°, then by a floor square to its axis. The 30° cut and
/// the floor share a line square to the axis, which touches the cone exactly
/// at the vertex of the hyperbola the floor cuts; found a femtometre inside
/// the cone's section there, it crossed it twice four tolerances apart,
/// across a sliver no face agreed on, and the body was refused. A line
/// inside the section by rounding alone touches the cone now.
#[test]
fn seed_5543231406_a_half_cone_cut_at_30_and_45_degrees_whose_floor_s_line_touches_it_at_a_hyperbola_s_vertex()
 {
    random_solids::holds_through_the_application(&Case::new(
        Leaf::turned(
            Plane::xy(240.0),
            Axis::second(0.0),
            Section::bands(0.0, &[[52.0, -75.0, 15.0]]).sloping_to(&[[-75.0, 45.0]]),
            180.0,
        ),
        vec![
            Step::cut(Leaf::prism(
                Plane::tilted([0.0, 25.98076211353316, 240.0], [-30.0, 0.0, 0.0]),
                Outline::rectangle([-90.0, -75.0], [90.0, 135.0]),
                -105.0,
            )),
            Step::cut(Leaf::prism(
                Plane::tilted([-15.0, -0.0, 240.0], [90.0, 45.0, 0.0]),
                Outline::rectangle([-45.0, -90.0], [165.0, 90.0]),
                -135.0,
            )),
            Step::cut(Leaf::prism(
                Plane::xy(255.0),
                Outline::rectangle([-90.0, -15.0], [90.0, 135.0]),
                60.0,
            )),
        ],
    ));
}

/// #552's round 4, through the application's body: cones and cylinders
/// about X joined to a quarter turn, cut at 45° by a plane through the
/// cone's apex on the axis, touching the cone along the ruling the quarter
/// turn's end plane holds, then joined to a second quarter turn. Past the
/// apex the ruling runs over the cone's other nappe, where the second quarter
/// turn's end corner stands; found along the ruling, that corner was counted
/// on the cone, kept apart from the same corner found by the other curves
/// through it on a plane apart from the cone, and the body was refused.
#[test]
fn seed_5543226572_a_cone_cut_at_45_degrees_through_its_apex_on_the_end_planes_of_two_quarter_turns()
 {
    random_solids::holds_through_the_application(&Case::new(
        Leaf::turned(
            Plane::xy(-45.0),
            Axis::first(-30.0).backwards(),
            Section::bands(30.0, &[[90.0, 30.0, 60.0], [15.0, 30.0, 60.0]])
                .sloping_to(&[[30.0, 60.0], [30.0, 75.0]]),
            360.0,
        ),
        vec![
            Step::add(Leaf::turned(
                Plane::xz(-30.0),
                Axis::first(-45.0),
                Section::bands(-195.0, &[[165.0, 0.0, 30.0]]),
                90.0,
            )),
            Step::cut(Leaf::prism(
                Plane::tilted([-60.0, -30.0, -45.0], [90.0, -45.0, 0.0]),
                Outline::rectangle([-180.0, -150.0], [135.0, 150.0]),
                -135.0,
            )),
            Step::add(Leaf::turned(
                Plane::xy(-45.0),
                Axis::first(-30.0),
                Section::bands(-195.0, &[[210.0, -30.0, -0.0]]),
                90.0,
            )),
        ],
    ));
}

/// #552's round 4, through the application's body: a stepped wedge turned a
/// hundredth of a degree about X, cut by a post along Y whose bottom
/// touches the wedge's first end plane, its axis in the wedge's end disc.
/// On the post, the curve it meets the wedge's wall along leaves the corner
/// where the post starts along the post's rim, both straight there, parting
/// as the fourth power of the way run: a tenth of the shorter, their chords
/// stood 8e-10 apart, under what tells two chords apart, and the corner was
/// declined as a tie. Halfway along the shorter, where it still crosses
/// nothing, they stand far enough apart.
#[test]
fn seed_5543224272_a_wedge_of_a_hundredth_of_a_degree_cut_by_a_post_touching_its_end_plane() {
    random_solids::holds_through_the_application(&Case::new(
        Leaf::turned(
            Plane::xy(1.0),
            Axis::first(0.0),
            Section::bands(-1.0, &[[2.0, 0.0, 4.0], [2.0, 0.0, 5.0], [3.0, 0.0, 1.0]]),
            0.01,
        ),
        vec![Step::cut(Leaf::prism(
            Plane::xz(1.0),
            Outline::circle([6.0, 2.0], 1.0),
            8.0,
        ))],
    ));
}

/// #552's round 4, measured merged: a frustum about X sloping at 30°, its
/// apex at x ≈ −345.79 beyond the box, cut by a block at that slope near the
/// apex and by a second at 15° whose face passes 3e-7 off the apex. Held as
/// the two rulings through the apex beyond the box, the face's cut was
/// declined as unverified; the hyperbola cut as on `85a2d2e` holds.
#[test]
fn seed_5543114315_a_frustum_cut_by_a_face_steeper_than_its_slope_a_hair_off_its_apex_beyond_the_box()
 {
    random_solids::holds_exactly(&Case::new(
        Leaf::turned(
            Plane::xz(-60.0),
            Axis::first(0.0),
            Section::bands(
                -60.0,
                &[
                    [25.98076211353316, -165.0, 0.0],
                    [139.01923788646684, -180.0, 0.0],
                ],
            )
            .sloping_to(&[[-180.0, 0.0], [-180.0, 0.0]]),
            360.0,
        ),
        vec![
            Step::cut(Leaf::prism(
                Plane::tilted([-346.0, -60.0, 0.0], [90.0, -30.0, 0.0]),
                Outline::rectangle([135.0, -210.0], [510.0, 210.0]),
                30.0,
            )),
            Step::cut(Leaf::prism(
                Plane::tilted(
                    [-345.7883833265105, -60.0, 2.897777478867205e-7],
                    [0.0, -15.0, 0.0],
                ),
                Outline::rectangle([210.0, -195.0], [510.0, 195.0]),
                -315.0,
            )),
        ],
    ));
}

/// #552's round 4, measured merged, through the application's body: a
/// stack of cones about X sloping at atan ⅛, cut by a block at half that
/// slope whose face passes 3e-7 off the apex of one beyond the box. As the
/// seed before it: declined as unverified with the two rulings, held with
/// the hyperbola.
#[test]
fn seed_5543213205_a_stack_of_cones_cut_by_a_face_steeper_than_one_s_slope_a_hair_off_its_apex_beyond_the_box()
 {
    random_solids::holds_through_the_application(&Case::new(
        Leaf::turned(
            Plane::xy(60.0),
            Axis::first(0.0),
            Section::bands(
                -90.0,
                &[
                    [120.0, 0.0, 120.0],
                    [15.0, 0.0, 90.0],
                    [120.0, 0.0, 90.0],
                    [120.0, 0.0, 90.0],
                ],
            )
            .sloping_to(&[[0.0, 120.0], [0.0, 90.0], [0.0, 90.0], [0.0, 75.0]]),
            360.0,
        ),
        vec![Step::cut(Leaf::prism(
            Plane::tilted(
                [884.9999999813588, 0.0, 59.99999970057972],
                [0.0, 3.562508174450899, 0.0],
            ),
            Outline::rectangle([-1005.0, -30.0], [-570.0, 135.0]),
            90.0,
        ))],
    ));
}

/// #552's round 4, measured merged, through the application's body: a
/// cone about Y joined to a ring, cut by a block whose face passes through
/// the cone's apex beyond the box steeper than its slope by a third of a
/// billionth, within the angle read as touching. Held as the ruling of
/// touch, the face could not be drawn; the hyperbola, two arms a few
/// thousandths apart across the box, is drawn as on `85a2d2e`.
#[test]
fn seed_5543216548_a_cone_cut_by_a_face_through_its_apex_beyond_the_box_a_hair_steeper_than_its_slope()
 {
    random_solids::holds_through_the_application(&Case::new(
        Leaf::turned(
            Plane::xy(7.5),
            Axis::second(-5.0),
            Section::bands(10.0, &[[25.0, 0.0, 27.5], [2.5, 0.0, 34.31818181818182]])
                .sloping_to(&[[0.0, 27.5], [0.0, 35.0]]),
            360.0,
        ),
        vec![
            Step::add(Leaf::turned(
                Plane::xy(7.5),
                Axis::second(-5.0),
                Section::bands(10.0, &[[28.0, 35.0, 45.0]]),
                360.0,
            )),
            Step::cut(Leaf::prism(
                Plane::tilted(
                    [-5.0, -90.8333333333333, 7.5],
                    [15.25511868305778, 0.0, 0.0],
                ),
                Outline::rectangle([-38.0, 85.0], [38.0, 138.0]),
                10.0,
            )),
        ],
    ));
}

/// #552's round 4, a slanted draw on the exact kernel: a rounded block cut
/// by a turn of a hundredth of a degree about Y, a wall and a cone of one
/// axis with the corner's wall. The block's side touches its corner's wall
/// along x = 8, and the cone cuts the side along a hyperbola whose vertex
/// sits on that line of touch; the turn's far end plane crosses the side
/// 1.75e-4 from the touch, where the side and the cone stand 1.52e-8 apart
/// at a tolerance of 1.6e-8. Two corners that far apart, on the same four
/// surfaces, are kept apart: the side and the cone cross along the
/// hyperbola, 3e-8 from both. The edge of the tangency band, with a conic at
/// the touch, where no band is laid (decision 7).
#[test]
#[ignore = "kernel-declines, #552's round 4: a hyperbola's vertex on a line of touch, a hundredth of a degree's end plane crossing the side at the band's edge; two corners within the tolerance parted by the hyperbola; declined as unverified"]
fn seed_5543307115_a_rounded_block_cut_by_a_turn_of_a_hundredth_of_a_degree_whose_cone_s_hyperbola_starts_on_a_line_of_touch()
 {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(
            Plane::xy(4.0),
            Outline::rounded([7.0, 1.0], [10.0, 5.0], 1.0),
            10.0,
        ),
        vec![Step::cut(Leaf::turned(
            Plane::yz(8.0),
            Axis::second(2.0),
            Section::bands(12.0, &[[2.0, 1.0, 3.0], [2.0, 0.0, 3.0]])
                .sloping_to(&[[0.0, 3.0], [0.0, 3.0]]),
            0.01,
        ))],
    ));
}

/// #552's round 4, a slanted draw on the exact kernel, failing at `9e54a71`
/// too, before conics: a post, a turned ring whose axis meets the post's,
/// and a second post. A region of a face was found with no column to put a
/// point inside it, and the step was declined as a tie
/// (`overlay/partition.rs`). It holds since a corner standing where two
/// walls were decided to touch at a point lies on both (5544320693).
#[test]
fn seed_5543305067_two_posts_and_a_ring_turned_about_an_axis_meeting_the_first() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(Plane::xy(-2.0), Outline::circle([6.0, 9.5], 1.5), -4.0),
        vec![
            Step::add(Leaf::turned(
                Plane::xy(-4.0),
                Axis::second(8.5),
                Section::bands(7.0, &[[5.0, 0.0, 1.0]]),
                360.0,
            )),
            Step::add(Leaf::prism(
                Plane::xy(-6.0),
                Outline::circle([8.5, 9.5], 1.0),
                4.0,
            )),
        ],
    ));
}

/// #552's round 4, a turned draw through the application's body, failing
/// at `9e54a71` too: a ring along Y cut by a turn about X, then by a post
/// along Y. A region of a face was found with no column to put a point
/// inside it, and the step was declined as a tie (`overlay/partition.rs`).
/// It holds since a corner standing where two walls were decided to touch
/// at a point lies on both (5544320693).
#[test]
fn seed_5543616767_a_ring_cut_by_a_turn_about_x_then_by_a_post() {
    random_solids::holds_through_the_application(&Case::new(
        Leaf::prism(
            Plane::xz(-15.0),
            Outline::ring([225.0, 270.0], 105.0, 30.0),
            240.0,
        ),
        vec![
            Step::cut(Leaf::turned(
                Plane::xy(315.0),
                Axis::first(-135.0),
                Section::bands(120.0, &[[210.0, 0.0, 15.0]]),
                360.0,
            )),
            Step::cut(Leaf::prism(
                Plane::xz(-75.0),
                Outline::circle([225.0, 315.0], 15.0),
                120.0,
            )),
        ],
    ));
}

/// Decision 11 measured on round 3's tilted seeds through the application's
/// body: a point turned about an axis leaning 1e-7° off X, cut by a block
/// at 42° whose face passes through the leaning axis at the apex. Turned
/// about the middle of its box to lie exactly along the axis, the face stood
/// a hair beside the apex, and was cut along two rulings it does not hold.
/// A turn that lays a plane a hair beside a cone's apex is not made, and the
/// face is cut as drawn, as on `85a2d2e`.
#[test]
fn seed_5543232861_a_point_cut_by_a_face_through_its_apex_leaning_1e_7_degrees_off_its_axis() {
    random_solids::holds_through_the_application(&Case::new(
        Leaf::turned(
            Plane::xy(5.0),
            Axis::first(0.0).backwards().leaning(1e-7),
            Section::bands(0.0, &[[2.0, 0.0, 0.0]]).sloping_to(&[[0.0, 2.0]]),
            360.0,
        ),
        vec![Step::cut(Leaf::prism(
            Plane::tilted(
                [-2.0, -6.6913060635885824e-9, 4.999999992568552],
                [-42.0, 0.0, 0.0],
            ),
            Outline::rectangle([-4.0, -7.0], [3.0, 7.0]),
            -7.0,
        ))],
    ));
}

/// Decision 11 measured on round 3's tilted seeds on the exact kernel: a
/// shaft leaning 1e-7° off X joined to a point whose end plane stands a
/// hair past the shaft's end. Turned square to the shaft, the point's end is
/// laid on the shaft's, which it crossed at its lean, and its walls about
/// the shaft's axis: declined. Holds as drawn on `85a2d2e`, and since
/// round 5, the point turned about where the two meet and set down on the
/// shaft's end it was drawn past.
#[test]
fn seed_5543120314_a_shaft_leaning_1e_7_degrees_joined_to_a_point_a_hair_past_its_end() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(
            Plane::tilted([1.8e-6, 150.0, 60.0], [90.0, 89.9999999, 0.0]),
            Outline::circle([240.0, -97.5], 30.0),
            195.0,
        ),
        vec![Step::add(Leaf::turned(
            Plane::xz(390.000006),
            Axis::first(-37.5),
            Section::bands(195.0000018, &[[15.0, 0.0, 30.0]]).sloping_to(&[[0.0, 0.0]]),
            360.0,
        ))],
    ));
}

/// Decision 11 measured on round 3's tilted seeds on the exact kernel: a
/// cylinder turned about an axis leaning 1e-7° off Y, sloping to a point
/// 1.8e-6 off its corner, cut by a block, then joined to a turn about X
/// whose start plane leans 1e-7° along the cylinder's axis. Turned along it,
/// the plane cuts the wall along two lines where it cut a hyperbola, and
/// meets the hair the slope stands off exactly along: declined. Holds as drawn on
/// `85a2d2e`, where the plane is left at its lean.
#[test]
#[ignore = "hair-from-tangent, decision 11: a plane laid exactly along a leaning cylinder meets a slope 1.8e-6 off its corner; declined, holds as drawn on 85a2d2e"]
fn seed_5543112269_a_turn_about_x_joined_to_a_cylinder_leaning_1e_7_degrees_whose_tip_stands_a_hair_off()
 {
    random_solids::holds_exactly(&Case::new(
        Leaf::turned(
            Plane::xy(210.0),
            Axis::second(0.0).leaning(1e-7),
            Section::bands(150.0, &[[120.0, 0.0, 90.0]]).sloping_to(&[[0.0, 90.0000018]]),
            360.0,
        ),
        vec![
            Step::cut(Leaf::prism(
                Plane::tilted([90.0, 210.0, 270.0], [90.0, 90.0, 0.0]),
                Outline::rectangle([-90.0, -90.0], [90.0, 60.0]),
                120.0,
            )),
            Step::add(Leaf::turned(
                Plane::yz(90.0),
                Axis::first(0.0),
                Section::bands(-105.0, &[[210.0, 0.0, 15.0]]),
                210.0,
            )),
        ],
    ));
}

/// Decision 11 measured on round 3's tilted seeds through the application's
/// body: a cylinder turned about an axis at 30° in the plane, cut at 34°,
/// then by two blocks leaning 1e-7° whose floors stand 2e-8 apart. Turned
/// along the wall's axis, both floors cut it along lines, two pairs a hair
/// apart: declined. Holds as drawn on `85a2d2e`, where they cut it along
/// ellipses.
#[test]
#[ignore = "hair-from-tangent, decision 11: two floors 2e-8 apart laid exactly along a wall's axis cut it along two pairs of lines a hair apart; declined, holds as drawn on 85a2d2e"]
fn seed_5543218891_a_wall_at_30_degrees_cut_by_two_blocks_leaning_1e_7_degrees_whose_floors_stand_2e_8_apart()
 {
    random_solids::holds_through_the_application(&Case::new(
        Leaf::turned(
            Plane::xy(6.0),
            Axis::second(0.0).leaning(30.0),
            Section::bands(2.0, &[[3.0, 0.0, 5.0]]),
            360.0,
        ),
        vec![
            Step::cut(Leaf::prism(
                Plane::tilted([-4.0, 3.0, 9.0], [0.0, -34.0, 0.0]),
                Outline::rectangle([-6.0, -5.0], [6.0, 5.0]),
                6.0,
            )),
            Step::cut(Leaf::prism(
                Plane::tilted([-4.0, 3.0, 9.0], [0.0, 1e-7, 0.0]),
                Outline::rectangle([-5.0, -5.0], [5.0, 5.0]),
                4.0,
            )),
            Step::cut(Leaf::prism(
                Plane::tilted([-4.0, 3.0, 8.99999998], [0.0, 1e-7, 0.0]),
                Outline::rectangle([6.0, 10.0], [10.0, 18.0]),
                8.0,
            )),
        ],
    ));
}

/// Decision 11 measured on round 3's tilted seeds through the application's
/// body: a ring about X cut by a block whose face stands at y = 1.5000002,
/// then by a slot leaning 2e-8° whose face stands 6e-8 short of it. Turned
/// square, the slot's face is parallel to the block's, a few tolerances
/// apart, and the ring's walls cross both: the triangles of the two faces
/// cross. Holds as drawn on `85a2d2e`, and since round 5, the slot turned
/// about where it meets the body and set down on what it held there.
#[test]
fn seed_5543235780_a_ring_cut_by_a_block_then_by_a_slot_leaning_2e_8_degrees_a_hair_short_of_its_face()
 {
    random_solids::holds_through_the_application(&Case::new(
        Leaf::turned(
            Plane::yz(6.0),
            Axis::first(4.0),
            Section::bands(-2.0, &[[7.0, 1.0, 6.0]]),
            360.0,
        ),
        vec![
            Step::cut(Leaf::prism(
                Plane::tilted([6.0, 1.5000002, 4.0], [90.0, 0.0, 0.0]),
                Outline::rectangle([-7.0, -7.0], [7.0, 7.0]),
                -4.0,
            )),
            Step::cut(Leaf::prism(
                Plane::tilted([6.0, 1.5000001399999998, 4.0], [90.0, 2e-8, 0.0]),
                Outline::slot([6.0, 0.0], [3.0, 0.0], 2.0),
                -8.0,
            )),
        ],
    ));
}

/// #552's decision 11 measured on fresh tilted seeds through the
/// application's body (5549204865): a tube cut by a block whose face stands
/// 5e-7 off tangent to its wall and leans 1.9e-6° off square to its axis —
/// past the hair decision 11 lays, and left as drawn. Fails on `85a2d2e`
/// too.
#[test]
#[ignore = "hair-from-tangent: a face 5e-7 off tangent to a tube's wall leaning 1.9e-6 degrees off square, past decision 11's hair; fails on 85a2d2e too"]
fn seed_5549204865_a_tube_cut_by_a_face_a_hair_off_tangent_and_1_9e_6_degrees_off_square() {
    random_solids::holds_through_the_application(&Case::new(
        Leaf::turned(
            Plane::xy(13.0),
            Axis::first(0.0),
            Section::bands(10.0, &[[27.5, -35.0, 0.0]]),
            360.0,
        ),
        vec![Step::cut(Leaf::prism(
            Plane::tilted([37.4999995, 25.0, 12.5], [0.0, -89.99999809014069, 0.0]),
            Outline::rectangle([-40.0, -40.0], [40.0, 40.0]),
            5.0,
        ))],
    ));
}

/// Decision 11 measured on round 3's turned seeds through the application's
/// body: a section turned whole about an axis leaning 1e-7° off X, cut by a
/// turn of 240° about Y. Laid square to the cut's end planes, its walls met
/// the cut's along the curve of two perpendicular cylinders, where they had
/// stood skew: declined. Holds as drawn on `85a2d2e`. Refusing a turn that
/// lays two skew walls square held it, and broke two seeds whose earlier
/// step was turned, the later one then left at its lean (5543215175).
/// Holds since round 5: the cut turned about where the two meet, and set
/// down on what it held there, rather than about the middle of its box.
#[test]
fn seed_5543615040_a_turn_leaning_1e_7_degrees_cut_by_a_turn_of_240_degrees_about_y() {
    random_solids::holds_through_the_application(&Case::new(
        Leaf::turned(
            Plane::xy(240.0),
            Axis::first(150.0).backwards().leaning(1e-7),
            Section::bands(120.0, &[[90.0, -180.0, 0.0]]),
            360.0,
        ),
        vec![Step::cut(Leaf::turned(
            Plane::xy(120.0),
            Axis::second(0.0),
            Section::bands(-90.0, &[[90.0, -240.0, 0.0]]),
            240.0,
        ))],
    ));
}

/// Decision 11 measured on round 3's slanted seeds on the exact kernel: a
/// section turned whole about Y, cut by one turned about X, then joined to a
/// turn of 150° about an axis leaning 1e-7° off Y. Laid along the body's
/// walls, the turn's end planes cut them along lines where they cut
/// hyperbolas: declined. Holds as drawn on `85a2d2e`.
#[test]
#[ignore = "hair-from-tangent, decision 11: a turn's end planes laid exactly along walls they leaned 1e-7 degrees off; declined, holds as drawn on 85a2d2e"]
fn seed_5543312609_a_turn_about_y_cut_by_one_about_x_then_joined_to_a_turn_of_150_degrees_leaning_1e_7_degrees()
 {
    random_solids::holds_exactly(&Case::new(
        Leaf::turned(
            Plane::yz(150.0),
            Axis::second(0.0),
            Section::bands(-30.0, &[[180.0, 0.0, 210.0]]),
            360.0,
        ),
        vec![
            Step::cut(Leaf::turned(
                Plane::xy(30.0),
                Axis::first(0.0),
                Section::bands(0.0, &[[120.0, 0.0, 240.0]]),
                360.0,
            )),
            Step::add(Leaf::turned(
                Plane::xy(90.0),
                Axis::second(-60.0).leaning(1e-7),
                Section::bands(-90.0, &[[120.0, 60.0, 225.0]]),
                150.0,
            )),
        ],
    ));
}

/// #552's round 5, decision 11's own, measured on fresh turned seeds on the
/// exact kernel: a turn of 240° about an axis along Z leaning 1e-7° towards
/// Y, joined to a post about Y across it. Decision 11 turns the post so that
/// its ends lie exactly along the leaning axis, and its wall stands exactly
/// square to the turn's, where the two were skew by the lean; the lines,
/// the rims and the ellipse where the turn's end plane cuts the post are
/// left with one face each: declined as unverified. Turned about the middle
/// of its box, the post's end stood 9e-9 off a line of the turn it held,
/// where the turn's end plane cuts its wall of radius one; turned about the
/// middle of where the two meet and set down on that line, it holds.
#[test]
fn seed_5544510380_a_turn_of_240_degrees_leaning_1e_7_degrees_joined_to_a_post_across_it() {
    random_solids::holds_exactly(&Case::new(
        Leaf::turned(
            Plane::yz(0.0),
            Axis::second(6.0).leaning(1e-7),
            Section::bands(-3.0, &[[4.0, -2.0, 1.0]]),
            240.0,
        ),
        vec![Step::add(Leaf::turned(
            Plane::xy(5.0),
            Axis::second(0.0),
            Section::bands(2.0, &[[3.5, 0.0, 5.5]]),
            360.0,
        ))],
    ));
}

/// #552's round 5, decision 11's own, through the application's body: a
/// disc turned about an axis leaning 1e-7° off Y, joined to a wedge of 15°
/// turned about Y. Turned square to the leaning axis, the wedge's end
/// planes and its axis line stand a few hundredths of a micron off where
/// they cross the disc, and edges along them are left with one face each:
/// declined as unverified. The wedge's side, drawn on the disc's end, was
/// turned about the middle of its box and stood 1.2e-8 off it, its corners
/// at the wedge's axis four times further; set down on the end, it holds.
#[test]
fn seed_5544600162_a_disc_leaning_1e_7_degrees_joined_to_a_wedge_of_15_degrees_about_y() {
    random_solids::holds_through_the_application(&Case::new(
        Leaf::turned(
            Plane::xy(-1.0),
            Axis::second(6.0).leaning(1e-7),
            Section::bands(-3.0, &[[3.0, 0.0, 8.0]]),
            360.0,
        ),
        vec![Step::add(Leaf::turned(
            Plane::xz(0.0),
            Axis::second(0.0),
            Section::bands(-2.0, &[[7.0, 0.0, 4.0]]),
            15.0,
        ))],
    ));
}

/// #552's round 5, decision 11's own, through the application's body: a
/// disc turned about an axis leaning 1e-7° off X, joined to a shaft about Y
/// that decision 11 lays square to it, cut by a post along Z, then by a
/// stepped turn about Y laid square as well. Edges on the post's rims, its
/// lines, and the curves where its wall meets the disc's and the shaft's
/// are left with one face each: declined as unverified. The stepped turn's
/// axis, drawn in the disc's end, was turned about the middle of its box and
/// stood 2e-8 off it; set down in it, and its end on the shaft's, it holds.
#[test]
fn seed_5544600864_a_disc_leaning_1e_7_degrees_joined_to_a_shaft_then_cut_by_a_post_and_a_turn() {
    random_solids::holds_through_the_application(&Case::new(
        Leaf::turned(
            Plane::xy(0.0),
            Axis::first(0.0).leaning(1e-7),
            Section::bands(-10.0, &[[10.0, 0.0, 25.0]]),
            360.0,
        ),
        vec![
            Step::add(Leaf::turned(
                Plane::yz(2.5),
                Axis::first(0.0).backwards(),
                Section::bands(3.0, &[[20.0, 0.0, 8.0]]),
                360.0,
            )),
            Step::cut(Leaf::prism(
                Plane::xy(-18.0),
                Outline::circle([2.5, -22.5], 7.5),
                35.0,
            )),
            Step::cut(Leaf::turned(
                Plane::xy(-3.0),
                Axis::second(0.0).backwards(),
                Section::bands(
                    0.0,
                    &[[3.0, 0.0, 20.0], [3.0, 0.0, 23.0], [15.0, 0.0, 10.0]],
                ),
                360.0,
            )),
        ],
    ));
}

/// #552's round 5, decision 11's own, through the application's body: a
/// cone about X cut at 45°, then by a block 4e-8° off a quarter turn whose
/// face stands 3e-7 short of x = 225, at a reach of hundreds. Decision 11
/// lays the face square to the cone's axis, which moves it a few tenths of
/// a micron where it crosses the 45° cut; the cone's rim, the conic of the
/// 45° cut and the line the two planes share are left with one face each:
/// declined as unverified. Holds with decision 11 switched off, and as drawn
/// on `85a2d2e`. Laid square, the face stands 3.84e-7 short of the circle at
/// x = 225 where the cone's wall meets the shaft's, at a tolerance of
/// 3.90e-7, wherever the turn is made about; drawn exactly square that far
/// short, with no lean to lay, it fails the same way. Set down on the plane
/// of that circle, which it stood within the tolerance of, it holds.
#[test]
fn seed_5544232904_a_cone_cut_at_45_degrees_then_by_a_face_4e_8_degrees_off_square_to_it() {
    random_solids::holds_through_the_application(&Case::new(
        Leaf::turned(
            Plane::xy(75.0),
            Axis::first(0.0),
            Section::bands(60.0, &[[165.0, 0.0, 120.0], [75.0, 0.0, 120.0]])
                .sloping_to(&[[0.0, 120.0], [0.0, 195.0]]),
            360.0,
        ),
        vec![
            Step::cut(Leaf::prism(
                Plane::tilted([263.0, 0.0, 75.0], [0.0, 45.0, 0.0]),
                Outline::rectangle([-300.0, -225.0], [180.0, 225.0]),
                -300.0,
            )),
            Step::cut(Leaf::prism(
                Plane::tilted([224.9999997, 120.0, 75.0], [90.0, 89.99999996, 0.0]),
                Outline::rectangle([-345.0, -225.0], [105.0, 210.0]),
                -180.0,
            )),
        ],
    ));
}

/// #552's round 5, decision 11's own, through the application's body: a
/// block 4e-8° off a quarter turn, joined to a ring about X whose band
/// starts 2e-8 off its face, the ring laid square to the block by decision
/// 11, then cut at 45° a hair aside. The cut meets a tie in the star of the
/// overlay (overlay/star.rs). Holds with decision 11 switched off, and as
/// drawn on `85a2d2e`. The cut's 45° face crosses the block's floor along a
/// line the ring's end holds exactly as drawn; laid square to the block, the
/// ring's end leans 7e-10 off that line, wherever the turn is made about.
/// Drawn with the ring and the block exact and the cut leaning 4e-8° about
/// Z instead, decision 11 turning nothing, it meets the same tie.
#[test]
#[ignore = "near-parallel-planes, #552's round 5: a ring laid square to a block 4e-8 degrees off a quarter turn, its end then 7e-10 off the line a later 45 degree cut shares with the floor; a tie in the overlay's star, as when the cut is drawn leaning"]
fn seed_5544236192_a_block_4e_8_degrees_off_a_quarter_turn_joined_to_a_ring_then_cut_at_45_degrees()
{
    random_solids::holds_through_the_application(&Case::new(
        Leaf::prism(
            Plane::tilted([3.0, 4.0, 3.0], [90.0, -89.99999996, 0.0]),
            Outline::rectangle([0.0, 0.0], [4.0, 8.0]),
            3.0,
        ),
        vec![
            Step::add(Leaf::turned(
                Plane::xy(4.5),
                Axis::first(3.0).backwards(),
                Section::bands(-1.9999999800000006, &[[0.5, 1.5, 3.0]]),
                360.0,
            )),
            Step::cut(Leaf::prism(
                Plane::tilted(
                    [0.49999998000000057, 2.5000000006981318, 4.5],
                    [0.0, 45.0, 0.0],
                ),
                Outline::rectangle([-3.0, -4.0], [4.0, 4.0]),
                3.0,
            )),
        ],
    ));
}

/// #552's round 5, a piece with no hair, tilted on the exact kernel and
/// failing on `85a2d2e` too: a tube along Z joined to a half tube of its
/// axis whose bore is its own, then cut by a block at 60° about X through
/// the bore's lowest line at the tube's end. The line the cut shares with
/// the end, found 3e-14 inside the bore's section, crossed the bore twice
/// 3.2e-6 apart, where the cut's ellipse touches the end once, at its
/// vertex: a sliver of the end with no point found inside it
/// (overlay/partition.rs), declined as a tie. A line square to a wall's
/// axis, shared by a plane square to it and one crossing it steeply,
/// touches the wall where it stands inside its section by rounding alone.
#[test]
fn seed_5544100660_a_tube_joined_to_a_half_tube_of_its_bore_then_cut_at_60_degrees() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(
            Plane::xy(-30.0),
            Outline::ring([240.0, 180.0], 120.0, 45.0),
            105.0,
        ),
        vec![
            Step::add(Leaf::turned(
                Plane::yz(240.0),
                Axis::second(180.0),
                Section::bands(-90.0, &[[165.0, 45.0, 90.0]]),
                180.0,
            )),
            Step::cut(Leaf::prism(
                Plane::tilted([240.0, 135.0, 75.0], [60.0, 0.0, 0.0]),
                Outline::rectangle([-15.0, -195.0], [105.0, 90.0]),
                225.0,
            )),
        ],
    ));
}

/// #552's round 5, a turned draw through the application's body that held
/// on `f57a5ca`: a rod along X joined to a turned rod of one axis a hair
/// thinner, to a block, then to a post along Z of the rod's radius whose
/// axis meets theirs. The post has a node with each rod, a hair apart; read
/// as on both rods, decided apart, the corner left an edge on one of the
/// curves the post meets them along with no face against it. A wall
/// decided apart from one a corner stands on is not taken onto it.
#[test]
fn seed_5545618519_a_post_across_two_rods_of_one_axis_a_hair_apart_in_radius() {
    random_solids::holds_through_the_application(&Case::new(
        Leaf::prism(Plane::yz(4.0), Outline::circle([3.0, 0.0], 0.5), 3.0),
        vec![
            Step::add(Leaf::turned(
                Plane::xy(0.0),
                Axis::first(3.0),
                Section::bands(4.0, &[[4.0, -0.49999999, 0.0]]),
                360.0,
            )),
            Step::add(Leaf::prism(
                Plane::xy(3.0),
                Outline::rectangle([8.0, 0.0], [14.0, 1.0]),
                7.0,
            )),
            Step::add(Leaf::prism(
                Plane::xy(-2.0),
                Outline::circle([6.0, 3.0], 0.5),
                4.0,
            )),
        ],
    ));
}

/// #552's round 5, a tilted draw through the application's body that held
/// on `f57a5ca`: a rounded block cut by a block, then by a 270° turn whose
/// inner wall is a cone with its apex on the turn's axis, then by a plane
/// at 45° through a hair off that apex, at the cone's slope. Read as on
/// every surface through the apex, corners within the tolerance of it cut
/// curves that pass it by. Only two walls touching at a point put a corner
/// there on both.
#[test]
fn seed_5545235515_a_turn_with_a_cone_inside_cut_a_hair_off_its_apex_at_its_slope() {
    random_solids::holds_through_the_application(&Case::new(
        Leaf::prism(
            Plane::yz(-60.0),
            Outline::rounded([120.0, 60.0], [225.0, 90.0], 15.0),
            45.0,
        ),
        vec![
            Step::cut(Leaf::prism(
                Plane::xy(135.0),
                Outline::rectangle([210.0, 30.0], [420.0, 60.0]),
                285.0,
            )),
            Step::cut(Leaf::turned(
                Plane::xy(75.0),
                Axis::first(135.0),
                Section::bands(-30.0, &[[15.0, -30.0, -15.0], [15.0, -30.0, -0.0]])
                    .sloping_to(&[[-30.0, -0.0], [-30.0, -0.0]]),
                270.0,
            )),
            Step::cut(Leaf::prism(
                Plane::tilted(
                    [-14.999999787867965, 135.00000021213202, 75.0],
                    [90.0, -45.0, 0.0],
                ),
                Outline::rectangle([-60.0, -45.0], [60.0, 45.0]),
                -60.0,
            )),
        ],
    ));
}

/// #552's round 5, a tilted draw on the exact kernel that held on
/// `f57a5ca`: a shaft turned about X, its narrow band giving way to a cone
/// whose apex stands at the shaft's end, cut by a plane at 45° through that
/// apex, a hair off the cone's slope. The rim where the shaft's wall gives
/// way to the cone stands inside the cut by rounding alone; read as a touch
/// there, it left the cut's hairpin on the cone crossing the rim where its
/// vertex stands a hair beyond, and a region of the cone had no point
/// inside it. A circle no plane of the triple holds keeps crossing twice.
#[test]
fn seed_5545108465_a_shaft_into_a_cone_cut_through_its_apex_a_hair_off_its_slope() {
    random_solids::holds_exactly(&Case::new(
        Leaf::turned(
            Plane::xy(210.0),
            Axis::first(0.0).backwards(),
            Section::bands(
                120.0,
                &[
                    [60.000006, -60.0, 0.0],
                    [14.999994, -60.0, 0.0],
                    [120.0, -75.0, 0.0],
                ],
            )
            .sloping_to(&[[-60.0, 0.0], [-74.999994, 0.0], [-75.0, 0.0]]),
            360.0,
        ),
        vec![Step::cut(Leaf::prism(
            Plane::tilted([-120.00000600000004, 0.0, 210.0], [90.0, -45.0, 0.0]),
            Outline::rectangle([-210.0, -90.0], [75.0, 90.0]),
            210.0,
        ))],
    ));
}

/// #552's round 5, a piece with no hair, tilted through the application's
/// body and failing on `85a2d2e` too: a disc and a turn of 270° about one
/// axis along Y and of one radius, joined, then cut by a block at 45° about
/// X through the wall's lowest line at the disc's end. A region of the
/// overlay had no point found inside it (overlay/partition.rs), declined
/// as a tie. It holds since a line square to a wall's axis, shared by a
/// plane square to it and one crossing it steeply, touches the wall where
/// it stands inside its section by rounding alone (5544100660).
#[test]
fn seed_5544229207_two_turns_about_one_axis_cut_at_45_degrees() {
    random_solids::holds_through_the_application(&Case::new(
        Leaf::turned(
            Plane::xy(25.0),
            Axis::second(-15.0),
            Section::bands(15.0, &[[17.5, -10.0, 7.5]]),
            360.0,
        ),
        vec![
            Step::add(Leaf::turned(
                Plane::xy(25.0),
                Axis::second(-15.0).backwards(),
                Section::bands(-32.5, &[[25.0, 0.0, 10.0]]),
                270.0,
            )),
            Step::cut(Leaf::prism(
                Plane::tilted([-15.0, 32.5, 15.0], [-45.0, 0.0, 0.0]),
                Outline::rectangle([-13.0, -35.0], [13.0, 3.0]),
                18.0,
            )),
        ],
    ));
}

/// #552's round 5, a piece with no hair, slanted on the exact kernel and
/// failing on `85a2d2e` too: a post along X joined to a rod of radius 0.5
/// along Z touching its wall, then cut by a pin along X of the rod's radius
/// whose axis meets the rod's at the touch: the two walls of one radius
/// cross along two ellipses whose node lies on the post's wall
/// (meet/pair.rs), where the line the pin touches the post along passes.
/// The union left no edge where the rod touches the post, and the node,
/// found on the pin and the rod alone, did not cut the line: a region of
/// the pin's overlay had no point found inside it (overlay/partition.rs),
/// declined as a tie. A corner standing where two walls were decided to
/// touch at a point lies on both.
#[test]
fn seed_5544320693_a_post_joined_to_a_rod_touching_it_then_cut_by_a_pin_of_its_radius() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(Plane::yz(4.0), Outline::circle([5.0, 3.5], 5.5), -7.0),
        vec![
            Step::add(Leaf::turned(
                Plane::xz(11.0),
                Axis::second(0.0),
                Section::bands(-3.0, &[[13.0, 0.0, 0.5]]),
                360.0,
            )),
            Step::cut(Leaf::prism(
                Plane::yz(-0.75),
                Outline::circle([11.0, 3.5], 0.5),
                3.0,
            )),
        ],
    ));
}

/// #552's round 5, a piece with no hair, turned on the exact kernel and
/// failing on `85a2d2e` too (#533's family of a nearly whole turn): a
/// stepped shaft turned 359.9° about X, joined to a turn of 105° about an
/// axis along Z crossing the shaft's wider band. The turn's inner wall
/// touches the shaft's step along a line through the step's inner rim
/// exactly at the slit, where the curve the shaft's wall meets that inner
/// wall along leaves the rim touching it to the fourth order, within the
/// tolerance of it across the whole slit. The step's face is lost: the
/// lines of the 0.1° slit, the step's rims and the turn's own lines are
/// left with one face each, declined as unverified. Reading the meeting
/// curve on the step's plane as the rim (domain/traced.rs) removed a
/// decline there and changed nothing else (#552's round 5).
#[test]
#[ignore = "plain-pieces, #552's round 5: a stepped shaft turned 359.9 degrees joined to a partial turn across it; the slit's lines left with one face each, declined as unverified"]
fn seed_5544504441_a_stepped_shaft_turned_359_9_degrees_joined_to_a_turn_of_105_degrees() {
    random_solids::holds_exactly(&Case::new(
        Leaf::turned(
            Plane::xy(0.0),
            Axis::first(0.0),
            Section::bands(20.0, &[[7.5, 0.0, 20.0], [7.5, 0.0, 27.5]]),
            359.9,
        ),
        vec![Step::add(Leaf::turned(
            Plane::yz(30.0),
            Axis::second(20.0),
            Section::bands(-15.0, &[[20.0, -2.5, 17.5]]),
            105.0,
        ))],
    ));
}

/// #552's round 5, a piece with no hair, turned on the exact kernel and
/// failing on `85a2d2e` too: a block along Z rounded by 0.5, joined to a
/// rod of radius 0.5 along Y whose axis lies on the block's side, touching
/// the rounded corners' walls, of its own radius, then cut by a post along
/// Z, touching the block's front where the corner's wall does. On the rod,
/// the curves the post and the corner's wall meet it along leave their
/// common corner together, flat, touching to the fourth order; read by
/// their chords a tenth along, the shorter one's below the longer though it
/// runs above, the rod's cycles were traced across each other, and edges
/// were left with one face, or three: declined as unverified. Two arcs
/// leaving together are read first where each stands at one run.
#[test]
fn seed_5544515442_a_rounded_block_joined_to_a_rod_along_its_side_then_cut_by_a_post() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(
            Plane::xy(-2.0),
            Outline::rounded([8.0, 3.0], [9.5, 10.5], 0.5),
            7.0,
        ),
        vec![
            Step::add(Leaf::turned(
                Plane::yz(9.5),
                Axis::first(0.0),
                Section::bands(1.0, &[[11.5, 0.0, 0.5]]),
                360.0,
            )),
            Step::cut(Leaf::prism(
                Plane::xy(4.0),
                Outline::circle([9.0, 4.0], 1.0),
                -5.0,
            )),
        ],
    ));
}

/// #552's round 5, a piece with no hair, slanted on the exact kernel and
/// failing on `85a2d2e` too: three turns about one axis along Y, a solid
/// turn of -150°, a half ring round it and a cone across the ring's end.
/// The stretch of a ruling of the cone in the plane the turns end on, from
/// radius 180 to the solid turn's wall at 210, is run one way only: left
/// open. It was drawn with a hair of 3e-7, which shrinking took away. The
/// body is verified; the cone's face, whose wall runs on above and below
/// the circle where the half ring's top and inner wall end, is not cut
/// into triangles: its loop runs along that circle and back, and the sweep
/// in the cone's parameters gives up (tessellation/sweep.rs).
#[test]
#[ignore = "plain-pieces, #552's round 5: a partial turn, a half ring and a cone about one axis; left open along a ruling of the cone"]
fn seed_5544323650_a_partial_turn_a_half_ring_and_a_cone_about_one_axis() {
    random_solids::holds_exactly(&Case::new(
        Leaf::turned(
            Plane::xy(150.0),
            Axis::second(0.0),
            Section::bands(30.0, &[[120.0, 0.0, 210.0]]),
            -150.0,
        ),
        vec![
            Step::add(Leaf::turned(
                Plane::yz(0.0),
                Axis::first(150.0).backwards(),
                Section::bands(-150.0, &[[120.0, 210.0, 240.0]]),
                180.0,
            )),
            Step::add(Leaf::turned(
                Plane::yz(0.0),
                Axis::first(150.0),
                Section::bands(135.0, &[[30.0, -180.0, -0.0]]).sloping_to(&[[-240.0, -0.0]]),
                360.0,
            )),
        ],
    ));
}

/// #552's round 5, measuring decision 11 set down where it meets, tilted
/// through the application's body: a shaft about an axis leaning 1e-5° off
/// X, cut at 75°, then by a face leaning 4° about X, a hair off along the
/// shaft's axis. Set down on every pair standing within `Scale::HAIR`
/// tolerances, the face was moved three tolerances onto what it only
/// passed near: declined as unverified. Only pairs within the tolerance are
/// laid. Holds on `f57a5ca`.
#[test]
fn seed_5545209574_a_shaft_leaning_1e_5_degrees_cut_at_75_degrees_then_by_a_face_4_degrees_off() {
    random_solids::holds_through_the_application(&Case::new(
        Leaf::turned(
            Plane::xy(60.0),
            Axis::first(-1.8e-6).backwards().leaning(-1e-5),
            Section::bands(-30.0, &[[105.0, -30.0, -0.0]]),
            360.0,
        ),
        vec![
            Step::cut(Leaf::prism(
                Plane::tilted([-30.0, 0.0, 60.0], [0.0, 75.0, 0.0]),
                Outline::rectangle([-60.0, -60.0], [60.0, 60.0]),
                90.0,
            )),
            Step::cut(Leaf::prism(
                Plane::tilted([0.0, 0.0, 60.0], [-4.0, 0.0, 0.0]),
                Outline::rectangle([-75.0, -90.0], [90.0, 90.0]),
                -90.0,
            )),
        ],
    ));
}

/// #552's round 5, measuring decision 11 set down where it meets, tilted
/// through the application's body: a cone and a shaft about an axis leaning
/// 1e-7° off X, joined to a block whose end stands on the circle where the
/// cone meets the shaft. Turned square about where the two meet, the end
/// stood a hair off that circle, and the triangles crossed. The plane of a
/// circle is laid on as a plane is. Holds on `f57a5ca`.
#[test]
fn seed_5545217348_a_cone_leaning_1e_7_degrees_joined_to_a_block_ending_where_it_meets_its_shaft() {
    random_solids::holds_through_the_application(&Case::new(
        Leaf::turned(
            Plane::xy(2.0),
            Axis::first(3.0).leaning(1e-7),
            Section::bands(-2.0, &[[1.5, -1.0, 2.0], [0.5, -1.0, 3.5]])
                .sloping_to(&[[-1.0, 3.5], [-1.0, 3.5]]),
            360.0,
        ),
        vec![Step::add(Leaf::prism(
            Plane::yz(5.0),
            Outline::rectangle([6.0, 0.0], [9.0, 8.0]),
            -5.5,
        ))],
    ));
}

/// #552's round 5, measuring the two lanes merged, slanted on the exact
/// kernel: a disc about an axis leaning 1e-7° off X, cut by a slot drawn on
/// a plane along that axis. Decision 11 lays the slot along the axis, and
/// its side, drawn on the disc's wall, was left a fraction of the tolerance
/// off it: refused. Holds on `f57a5ca` and on the pieces with no hair, and
/// since the side's edges are laid on the wall they touch.
#[test]
fn seed_5546322326_a_disc_leaning_1e_7_degrees_cut_by_a_slot_along_its_axis() {
    random_solids::holds_exactly(&Case::new(
        Leaf::turned(
            Plane::xy(30.0),
            Axis::first(-30.0).leaning(1e-7),
            Section::bands(-60.0, &[[210.0, 0.0, 165.0]]),
            360.0,
        ),
        vec![Step::cut(Leaf::prism(
            Plane::xy(120.0),
            Outline::slot([300.0, 120.0], [150.0, 120.0], 15.0),
            165.0,
        ))],
    ));
}

/// #552's round 5, measuring the two lanes merged, turned through the
/// application's body: a ring about an axis leaning 1e-7° off Y, joined to
/// a rounded block drawn on a plane along that axis. Decision 11 lays the
/// block along the axis, and the seam where a rounded corner meets its end,
/// drawn on the ring's bore, was left a fraction of the tolerance inside
/// it: refused. Holds on `f57a5ca` and on the pieces with no hair, and since
/// the seam is laid on the wall it touches.
#[test]
fn seed_5546607658_a_ring_leaning_1e_7_degrees_joined_to_a_rounded_block_along_its_axis() {
    random_solids::holds_through_the_application(&Case::new(
        Leaf::turned(
            Plane::xy(-5.0),
            Axis::second(25.0).leaning(1e-7),
            Section::bands(25.0, &[[20.0, 10.0, 38.0]]),
            360.0,
        ),
        vec![Step::add(Leaf::prism(
            Plane::xy(-10.0),
            Outline::rounded([12.5, 2.5], [20.0, 40.0], 2.5),
            28.0,
        ))],
    ));
}

/// #552's round 5, measuring the two lanes merged, tilted on the exact
/// kernel: a turn of 330° about an axis leaning 1e-7° off Y, cut at 45°
/// and then at -15° through a point a hair off the first cut's corner.
/// Decision 11 lays the cuts square to the axis and set the second on the
/// axis, the nearest of three lines a fraction of the tolerance apart: the
/// slit's line and the first cut's corner then stood off it, and it was
/// refused. Holds on `f57a5ca` and on the pieces with no hair, and since a
/// flat near lines that part lies on none of them.
#[test]
fn seed_5546116488_a_turn_leaning_1e_7_degrees_cut_at_45_degrees_then_at_15_degrees_a_hair_aside() {
    random_solids::holds_exactly(&Case::new(
        Leaf::turned(
            Plane::xy(7.0),
            Axis::second(-1.99999994).leaning(1e-7),
            Section::bands(-3.0, &[[6.0, 0.0, 5.0]]),
            330.0,
        ),
        vec![
            Step::cut(Leaf::prism(
                Plane::tilted([-2.0, 0.0, 7.0], [0.0, 45.0, 0.0]),
                Outline::rectangle([-8.0, -4.0], [8.0, 5.0]),
                8.0,
            )),
            Step::cut(Leaf::prism(
                Plane::tilted(
                    [-1.999999997580781, 0.0, 6.9999999902970425],
                    [0.0, -15.0, 0.0],
                ),
                Outline::rectangle([-7.0, -4.0], [7.0, 5.0]),
                -7.0,
            )),
        ],
    ));
}

/// #552's round 5, measuring the two lanes merged once decision 11 laid
/// flats and edges on walls they touch, tilted on the exact kernel: a turn
/// of 45° about Z, cut by a face 1e-7° off square to the turn's plane, then
/// by a face square to it through a point a hair off its axis. The first
/// cut's face stands between the turn's wall of radius 20 and one 1.5e-7
/// larger, within the tolerance of the larger alone; laid on it as a flat
/// touching a wall, it was refused. Holds on `f57a5ca`, and since a flat is
/// not laid on a wall it touches.
#[test]
fn seed_5546111832_a_turn_of_45_degrees_cut_by_a_face_1e_7_degrees_off_then_by_one_a_hair_off_its_axis()
 {
    random_solids::holds_exactly(&Case::new(
        Leaf::turned(
            Plane::yz(0.0),
            Axis::second(10.0),
            Section::bands(15.0, &[[15.0, 0.0, 20.0], [12.5, 0.0, 20.0000003]]),
            45.0,
        ),
        vec![
            Step::cut(Leaf::prism(
                Plane::tilted(
                    [1.0000000000000001e-7, 10.0, 36.25],
                    [0.0, -89.9999999, 0.0],
                ),
                Outline::rectangle([-25.0, -25.0], [30.0, 3.0]),
                -20.0,
            )),
            Step::cut(Leaf::prism(
                Plane::tilted([0.0, 9.9999999, 36.25], [-90.0, 0.0, 0.0]),
                Outline::rectangle([-3.0, -30.0], [20.0, 25.0]),
                5.0,
            )),
        ],
    ));
}

/// #552's round 5, measuring the two lanes merged once decision 11 laid
/// edges on walls, tilted through the application's body: a turn of 270°
/// about X, cut by a face drawn through its axis on the slit's face, leaning
/// 1.4e-6° off it, the cut reaching past the turn. Turned about where the
/// two meet, the cut's far end would move past the hair, so it was left as
/// drawn and refused. Holds on `f57a5ca`, which turned it about the middle
/// of its box, and since the turn falls back on that middle.
#[test]
fn seed_5547227649_a_turn_of_270_degrees_cut_along_its_slit_s_face_by_a_face_1_4e_6_degrees_off_reaching_past_it()
 {
    random_solids::holds_through_the_application(&Case::new(
        Leaf::turned(
            Plane::xy(6.0),
            Axis::first(0.0),
            Section::bands(-2.0, &[[8.0, 0.0, 5.0]]),
            270.0,
        ),
        vec![Step::cut(Leaf::prism(
            Plane::tilted([4.5, 0.0, 6.0], [0.0, -1.4323944918439405e-6, 0.0]),
            Outline::rectangle([-7.0, -6.0], [5.0, 6.0]),
            6.0,
        ))],
    ));
}

/// #552's round 5, measuring the two lanes merged once decision 11 laid
/// edges on walls, tilted through the application's body, as drawn: a ring
/// on its side joined to a tube about X and a post, cut by a block, then by
/// a face 9.5e-7° off square to the tube's axis a hair inside its end, then
/// by half a turn about Y. Turned about where the face meets the body, the
/// cut would move past the hair and was left as drawn, refused once built
/// in release. Holds on `f57a5ca`, and since the turn falls back on the
/// middle of the cut's box. Shrunk further, the case fails on `f57a5ca`
/// too.
#[test]
fn seed_5547233090_a_tube_and_a_ring_cut_by_a_block_by_a_face_9_5e_7_degrees_off_square_and_by_half_a_turn()
 {
    random_solids::holds_through_the_application(&Case::new(
        Leaf::prism(Plane::xz(1.0), Outline::ring([2.0, 9.0], 1.5, 0.5), 6.5),
        vec![
            Step::add(Leaf::turned(
                Plane::xy(1.0),
                Axis::first(0.0),
                Section::bands(-2.0, &[[5.9999999, 4.0, 8.0], [1e-7, 4.0, 8.0]])
                    .sloping_to(&[[4.0, 8.0], [4.0, 5.0]]),
                360.0,
            )),
            Step::add(Leaf::prism(
                Plane::xy(-9.0),
                Outline::circle([0.9999999999999998, 0.0], 1.0),
                20.0,
            )),
            Step::cut(Leaf::prism(
                Plane::xz(-5.8192558480765015),
                Outline::rectangle([-2.5, -7.5], [4.5, 9.5]),
                3.0,
            )),
            Step::cut(Leaf::prism(
                Plane::tilted(
                    [3.9999999899583827, 0.0, 1.0],
                    [0.0, 89.99999904507035, 0.0],
                ),
                Outline::rectangle([-9.0, -8.5], [8.5, 8.5]),
                -7.0,
            )),
            Step::cut(Leaf::turned(
                Plane::xy(9.0),
                Axis::second(2.00000001).backwards(),
                Section::bands(4.5, &[[1.0, -3.5, -1.5], [2.0, -3.5, -0.5]])
                    .sloping_to(&[[-3.5, -0.5], [-3.5, -0.5]]),
                -180.0,
            )),
        ],
    ));
}

/// #552's round 5, measuring the two lanes merged, tilted through the
/// application's body: a ring about X leaning 1e-5°, cut by a face drawn
/// through a point of its end, 2e-8° off square to X. The two planes lean
/// 1.7e-7 off each other and meet along a line that crosses the end's inner
/// rim near its lowest point, wherever a hair of the face's place puts it.
/// Decision 11 lays the face's sides along the ring's axis, turning it by
/// 3.5e-10 radian; about where the two meet, the line lands at 0.004 of the
/// rim's lowest point and the triangles cross; about the middle of the
/// cut's box, on `f57a5ca`, it landed at 0.001 and held. No pair of the two
/// lies within the tolerance to lay.
#[test]
#[ignore = "near-parallel-planes, #552's round 5: a face 2e-8 degrees off square to a ring's axis leaning 1e-5 degrees meets the ring's end along a line a hair's turn moves across its inner rim; the triangles cross"]
fn seed_5547225260_a_ring_leaning_1e_5_degrees_cut_by_a_face_through_its_end_2e_8_degrees_off_square()
 {
    random_solids::holds_through_the_application(&Case::new(
        Leaf::turned(
            Plane::xy(5.0),
            Axis::first(0.0).leaning(-1e-5),
            Section::bands(0.0, &[[2.0, 1.0, 5.0]]),
            360.0,
        ),
        vec![Step::cut(Leaf::prism(
            Plane::tilted([2.0, 0.0, 9.0], [0.0, -89.99999998, 0.0]),
            Outline::rectangle([-5.0, -5.0], [6.0, 6.0]),
            4.0,
        ))],
    ));
}

/// #552's round 5, measuring the two lanes merged once the turn fell back
/// on the middle of the cut's box, tilted through the application's body: a
/// wedge of 0.01° about an axis leaning 60° in its sketch plane, cut by a
/// face drawn on that plane 1e-7° off it. Decision 11 lays the face along
/// the wedge's axis; what is left of its lean, 1.5e-9 radian about the
/// axis, keeps it off the wedge's face it was drawn on, and the two cross
/// along a line a hair's turn moves. About where the two meet, the step is
/// refused; about the middle of the cut's box, on `f57a5ca`, it held. No
/// pair of the two lies within the tolerance to lay.
#[test]
#[ignore = "near-parallel-planes, #552's round 5: a face drawn on a 0.01 degree wedge's face 1e-7 degrees off it, laid along the wedge's axis, still leans 1.5e-9 radian off that face; the step is refused"]
fn seed_5548213817_a_wedge_of_0_01_degrees_cut_by_a_face_1e_7_degrees_off_its_own() {
    random_solids::holds_through_the_application(&Case::new(
        Leaf::turned(
            Plane::xz(20.0),
            Axis::second(0.0).backwards().leaning(60.0),
            Section::bands(0.0, &[[8.0, 0.0, 5.0]]),
            0.01,
        ),
        vec![Step::cut(Leaf::prism(
            Plane::tilted([15.0, 20.0, -5.0], [-89.9999999, 0.0, 0.0]),
            Outline::rectangle([-17.5, -10.0], [15.0, 10.0]),
            5.0,
        ))],
    ));
}

/// #552's round 6, tilted on the exact kernel, no hair: a half turn about X
/// cut by a cylinder along Y, by a turn about a line along Z, then by a
/// block leaning 15°. The triangles leave the body open along a line at x =
/// 150, the edge held by one face only; through the application's body the
/// step is refused. As drawn, the seed fails on `f57a5ca` too; not
/// diagnosed further.
#[test]
#[ignore = "plain-pieces, #552's round 6: a half turn cut by a cylinder, a turn and a block leaning 15 degrees; the triangles leave it open along a line"]
fn seed_5550117749_a_half_turn_cut_by_a_cylinder_a_turn_and_a_block_leaning_15_degrees() {
    random_solids::holds_exactly(&Case::new(
        Leaf::turned(
            Plane::xy(-30.0),
            Axis::first(0.0),
            Section::bands(150.0, &[[30.0, -30.0, 75.0]]),
            180.0,
        ),
        vec![
            Step::cut(Leaf::prism(
                Plane::xz(120.0),
                Outline::circle([165.0, -30.0], 30.0),
                240.0,
            )),
            Step::cut(Leaf::turned(
                Plane::xz(210.0),
                Axis::second(150.0),
                Section::bands(-60.0, &[[60.0, -210.0, 0.0]]),
                360.0,
            )),
            Step::cut(Leaf::prism(
                Plane::tilted([165.0, 0.0, -30.0], [90.0, 15.0, 0.0]),
                Outline::rectangle([-90.0, -60.0], [90.0, 60.0]),
                150.0,
            )),
        ],
    ));
}

/// #552's round 6, tilted through the application's body, no hair: a block
/// turned 45° about X, cut by a cone's wedge of 0.01° turned about Y. The
/// wedge's two faces stand 8.7e-5 apart where they cross the block's side
/// at x = 3, and that side's overlay finds a region with no point inside it
/// (overlay/partition.rs, `gather`): declined as a tie, on the exact kernel
/// too. As drawn, the seed fails on `f57a5ca` too.
#[test]
#[ignore = "plain-pieces, #552's round 6: a block at 45 degrees cut by a cone's wedge of 0.01 degrees; declined as a tie in the overlay of the block's side"]
fn seed_5550223366_a_block_at_45_degrees_cut_by_a_cone_s_wedge_of_0_01_degrees() {
    random_solids::holds_through_the_application(&Case::new(
        Leaf::prism(
            Plane::tilted([0.0, 6.0, 3.5], [-45.0, 0.0, 0.0]),
            Outline::rectangle([2.0, -1.0], [3.0, 2.0]),
            3.0,
        ),
        vec![Step::cut(Leaf::turned(
            Plane::xy(3.0),
            Axis::second(2.5),
            Section::bands(5.5, &[[1.5, -0.5, -0.0], [1.5, -0.5, -0.0]])
                .sloping_to(&[[-0.5, -0.0], [-2.0, -0.0]]),
            0.01,
        ))],
    ));
}

/// #552's round 6, tilted through the application's body, no hair: a slot
/// joined to a turn about a line along X at y = 7.5, then cut by a block
/// turned a quarter about X whose face holds that line. Four lines on that
/// face are left run twice one way and once the other: declined as
/// unverified, on the exact kernel too. As drawn, the seed fails on
/// `f57a5ca` too.
///
/// Was ignored as: plain-pieces, #552's round 6: a slot joined to a turn and
/// cut by a block turned a quarter, its face holding the turn's axis; declined
/// as unverified. Holds since two walls of one operand touching at a point that
/// a face of the other passes through put a corner there on the three: the
/// slot's round end touches the turn on the block's face, round 6 of #552.
#[test]
fn seed_5550230611_a_slot_joined_to_a_turn_and_cut_by_a_block_turned_a_quarter() {
    random_solids::holds_through_the_application(&Case::new(
        Leaf::prism(
            Plane::xz(30.0),
            Outline::slot([45.0, 40.0], [45.0, 62.5], 15.0),
            45.0,
        ),
        vec![
            Step::add(Leaf::turned(
                Plane::xy(90.0),
                Axis::first(7.5),
                Section::bands(30.0, &[[30.0, 0.0, 12.5]]),
                360.0,
            )),
            Step::cut(Leaf::prism(
                Plane::tilted([45.0, 7.5, 90.0], [90.0, 0.0, 0.0]),
                Outline::rectangle([-20.0, -18.0], [20.0, 18.0]),
                18.0,
            )),
        ],
    ));
}

/// #552's round 6, tilted through the application's body, no hair: a turn
/// about Z joined to a turn of 359.9° about a line along X crossing it. The
/// trace of a curve is declined as unsupported (domain/traced.rs, `chord`),
/// and the slit's lines and the rims are left with one face each: declined
/// as unverified, on the exact kernel too. A turn of 359.9° as 5544504441.
/// As drawn, the seed fails on `f57a5ca` too.
#[test]
#[ignore = "plain-pieces, #552's round 6: a turn joined to a turn of 359.9 degrees about a crossing axis; its slit's lines and rims are left with one face each, declined as unverified"]
fn seed_5550232595_a_turn_joined_to_a_turn_of_359_9_degrees_about_a_crossing_axis() {
    random_solids::holds_through_the_application(&Case::new(
        Leaf::turned(
            Plane::xz(0.0),
            Axis::second(0.0),
            Section::bands(-5.0, &[[15.0, -8.0, 15.0]]),
            360.0,
        ),
        vec![Step::add(Leaf::turned(
            Plane::xy(5.0),
            Axis::first(10.0).backwards(),
            Section::bands(15.0, &[[10.0, 10.0, 23.0]]),
            359.9,
        ))],
    ));
}

/// #552's round 6, tilted through the application's body, no hair: a wedge
/// of 45° turned about Y, bored along Z, then given a pin along X of the
/// bore's radius whose axis meets the bore's. The two walls of one radius
/// whose axes meet square cross along two ellipses, which the kernel
/// declines as unsupported (combine/crossed.rs), on the exact kernel too.
/// As drawn, the seed fails on `f57a5ca` too.
#[test]
#[ignore = "plain-pieces, #552's round 6: a wedge of 45 degrees bored and given a pin of the bore's radius whose axis meets its own; declined as unsupported"]
fn seed_5550232928_a_wedge_of_45_degrees_bored_and_given_a_pin_across() {
    random_solids::holds_through_the_application(&Case::new(
        Leaf::turned(
            Plane::xy(0.0),
            Axis::second(0.0),
            Section::bands(5.0, &[[4.0, 0.0, 5.0]]),
            45.0,
        ),
        vec![
            Step::cut(Leaf::prism(
                Plane::xy(-9.0),
                Outline::circle([0.0, 8.0], 1.5),
                17.0,
            )),
            Step::add(Leaf::prism(
                Plane::yz(-8.0),
                Outline::circle([8.0, 0.0], 1.5),
                16.0,
            )),
        ],
    ));
}

/// #552's round 6, slanted on the exact kernel, no hair: the prism of an
/// arc of 280° along X, joined to two turns about lines along Z. The
/// overlay of the plane z = 8.5, through the arc's centre, finds a region
/// with no point inside it (overlay/partition.rs, `gather`): declined as a
/// tie, through the application's body too. As drawn, the seed fails on
/// `f57a5ca` too.
///
/// Was ignored as: plain-pieces, #552's round 6: an arc's prism joined to two
/// turns; declined as a tie in the overlay of the plane its top stands on.
/// Holds since two walls of one operand touching at a point that a face of the
/// other passes through put a corner there on the three, round 6 of #552.
#[test]
fn seed_5550316656_an_arc_s_prism_joined_to_two_turns() {
    random_solids::holds_exactly(&Case::new(
        Leaf::prism(
            Plane::yz(-2.0),
            Outline::circle_from([0.0, 8.5], 1.0, 279.77494960142656),
            3.5,
        ),
        vec![
            Step::add(Leaf::turned(
                Plane::yz(-0.25),
                Axis::second(3.0).backwards(),
                Section::bands(-11.0, &[[4.0, 0.0, 2.0]]),
                360.0,
            )),
            Step::add(Leaf::turned(
                Plane::xz(3.0),
                Axis::second(0.0),
                Section::bands(8.5, &[[1.0, 0.0, 3.5]]),
                360.0,
            )),
        ],
    ));
}
