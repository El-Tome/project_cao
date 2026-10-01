use std::f64::consts::TAU;

use glam::{DVec2, DVec3};

use super::*;
use crate::brep::combine::laid;
use crate::brep::scale::Scale;
use crate::brep::surface::Surface;
use crate::brep::topology::Body;
use crate::profile::{Contour, Frame, Run};

fn ground(height: f64) -> Frame {
    Frame {
        origin: DVec3::Z * height,
        u: DVec3::X,
        v: DVec3::Y,
    }
}

fn block(low: [f64; 3], high: [f64; 3]) -> Body {
    let outline = Contour::rectangle(DVec2::new(low[0], low[1]), DVec2::new(high[0], high[1]));
    Body::raised(&outline, &[], ground(low[2]), DVec3::Z * (high[2] - low[2]))
        .expect("a block raises")
}

fn standing(center: [f64; 2], radius: f64, from: f64, to: f64) -> Body {
    let center = DVec2::from(center);
    let outline = Contour {
        corners: vec![center + DVec2::X * radius],
        runs: vec![Run::Round { center, turn: TAU }],
    };
    Body::raised(&outline, &[], ground(from), DVec3::Z * (to - from)).expect("a cylinder raises")
}

/// The faces an operation keeps on the plane square to `axis` at `offset`,
/// each by whether it is flipped.
fn kept_on(
    first: &Body,
    second: &Body,
    operation: Operation,
    axis: DVec3,
    offset: f64,
) -> Vec<bool> {
    let operands = Operands::of(first, second, first.scale().joined(second.scale()));
    let arena = laid(&operands).expect("the arena is laid");
    let surface = arena
        .body
        .surfaces
        .iter()
        .position(|surface| {
            matches!(surface, Surface::Plane(plane)
                if plane.normal.abs_diff_eq(axis, 1e-12) && (plane.offset() - offset).abs() < 1e-12)
        })
        .expect("the plane is there");
    selected(&operands, &arena, operation)
        .expect("the faces are selected")
        .iter()
        .filter(|face| face.surface.0 as usize == surface)
        .map(|face| face.flipped)
        .collect()
}

#[test]
fn a_region_covered_by_both_operands_facing_one_way_is_kept_once_by_a_join_and_dropped_by_a_cut() {
    let stock = standing([0.0, 0.0], 20.0, 0.0, 10.0);
    let hole = standing([8.0, 0.0], 5.0, 0.0, 10.0);
    assert_eq!(
        kept_on(&stock, &hole, Operation::Or, DVec3::Z, 10.0),
        vec![false, false]
    );
    assert_eq!(
        kept_on(&stock, &hole, Operation::AndNot, DVec3::Z, 10.0),
        vec![false]
    );
}

#[test]
fn a_region_covered_from_each_side_is_dropped_by_a_join_and_kept_by_a_cut() {
    let one = block([-20.0, -20.0, 0.0], [20.0, 20.0, 10.0]);
    let other = block([20.0, -20.0, 0.0], [60.0, 20.0, 10.0]);
    assert_eq!(kept_on(&one, &other, Operation::Or, DVec3::X, 20.0), vec![]);
    assert_eq!(
        kept_on(&one, &other, Operation::AndNot, DVec3::X, 20.0),
        vec![false]
    );
    assert_eq!(
        kept_on(&other, &one, Operation::AndNot, DVec3::X, 20.0),
        vec![true]
    );
}

#[test]
fn a_region_covered_by_one_operand_is_kept_where_the_other_does_not_wrap_it() {
    let one = block([-20.0, -20.0, 0.0], [20.0, 20.0, 10.0]);
    let inside = block([-5.0, -5.0, 2.0], [5.0, 5.0, 8.0]);
    let outside = block([30.0, -5.0, 2.0], [40.0, 5.0, 8.0]);
    for operation in [Operation::Or, Operation::AndNot] {
        assert_eq!(
            kept_on(&one, &inside, operation, DVec3::Z, 10.0),
            vec![false]
        );
        assert_eq!(
            kept_on(&one, &outside, operation, DVec3::Z, 8.0),
            if operation == Operation::Or {
                vec![false]
            } else {
                vec![]
            }
        );
    }
    assert_eq!(kept_on(&one, &inside, Operation::Or, DVec3::Z, 8.0), vec![]);
    assert_eq!(
        kept_on(&one, &inside, Operation::AndNot, DVec3::Z, 8.0),
        vec![true]
    );
}

#[test]
fn a_sliver_of_a_face_thinner_than_twice_the_tolerance_is_told_covered_all_the_same() {
    let one = block([0.0, 0.0, 0.0], [10.0, 10.0, 10.0]);
    let eps = Scale::of(15.0).eps();
    let almost = block([-5.0, -5.0, 5.0], [10.0 - 1.5 * eps, 15.0, 15.0]);
    assert_eq!(almost.scale(), Scale::of(15.0));
    assert_eq!(
        kept_on(&one, &almost, Operation::AndNot, DVec3::Z, 10.0),
        vec![false]
    );
}

#[test]
fn a_sliver_of_a_face_standing_a_hair_past_the_other_operand_s_face_on_its_plane_is_wound_all_the_same()
 {
    let one = block([0.0, 0.0, 0.0], [10.0, 10.0, 10.0]);
    let eps = Scale::of(20.0).eps();
    let over = block([0.0, 0.0, 10.0], [10.0 + 1.5 * eps, 10.0, 20.0]);
    assert_eq!(over.scale(), Scale::of(20.0));
    assert_eq!(
        kept_on(&one, &over, Operation::Or, DVec3::Z, 10.0),
        vec![true]
    );
}

/// Seed 6000336 of the campaign: a slab under a tower, cut by a column at the
/// tower's corner, flush with two of its sides and with the slab's bottom.
/// The slab's top under the tower is covered by neither operand, and the
/// point chosen inside it lies on the column's side: asking the column how
/// it wraps the point was a tie, and nothing needed asking.
#[test]
fn a_region_neither_operand_covers_is_left_without_asking_either_how_it_wraps_it() {
    let tower = block([6.0, 4.0, -2.0], [8.0, 6.0, 2.0]);
    let slab = block([4.0, 2.0, -2.0], [10.0, 8.0, 0.0]);
    let column = block([6.0, 5.0, -2.0], [7.0, 6.0, 3.0]);
    let joined = tower.joined(&slab).expect("the tower stands on the slab");
    let cut = joined.cut_by(&column);
    assert!(cut.is_ok(), "{cut:?}");
}

/// The same slab, read where it tied: a point of its top under the tower that
/// the column's side runs through, though no arc of the top does — the
/// column has no face on the top, and the joined body none there either.
/// Neither covers the point, so nothing hangs on how either wraps it, and
/// neither is asked: the column, asked, would tie on its own face.
#[test]
fn a_point_neither_operand_covers_is_left_unasked_though_it_lies_on_a_face_of_one() {
    let tower = block([6.0, 4.0, -2.0], [8.0, 6.0, 2.0]);
    let slab = block([4.0, 2.0, -2.0], [10.0, 8.0, 0.0]);
    let column = block([6.0, 5.0, -2.0], [7.0, 6.0, 3.0]);
    let joined = tower.joined(&slab).expect("the tower stands on the slab");
    let operands = Operands::of(&joined, &column, joined.scale().joined(column.scale()));
    let top = operands
        .surfaces
        .list
        .iter()
        .position(|surface| {
            matches!(surface, Surface::Plane(plane)
                if plane.normal.abs_diff_eq(DVec3::Z, 1e-12) && plane.offset().abs() < 1e-12)
        })
        .expect("the slab's top is there");
    let wrapped = wrapped_at(
        &operands,
        &[Place {
            surface: SurfaceId(top as u32),
            geometry: &operands.surfaces.list[top],
            point: DVec3::new(6.5, 5.0, 0.0),
            turned: false,
            beside: &[],
        }],
    );
    assert!(matches!(wrapped, Ok(None)), "{wrapped:?}");
}

/// Seed 5000221 of the campaign: a bar lying along X whose cap touches a
/// post standing on Z at a single point of its wall, the very point the
/// post's band of wall is read at. Asking the bar how it wraps that point is
/// a tie; another point along the same chord of the band answers.
#[test]
fn a_region_whose_point_the_other_operand_touches_is_read_at_another_point_of_its_chord() {
    let post = standing([30.0, 5.0], 25.0, -5.0, 35.0);
    let frame = Frame {
        origin: DVec3::X * 5.0,
        u: DVec3::Y,
        v: DVec3::Z,
    };
    let center = DVec2::new(35.0, 15.0);
    let circle = Contour {
        corners: vec![center + DVec2::X * 30.0],
        runs: vec![Run::Round { center, turn: TAU }],
    };
    let bar = Body::raised(&circle, &[], frame, DVec3::NEG_X * 18.0).expect("a bar raises");
    let cut = post.cut_by(&bar);
    assert!(cut.is_ok(), "{cut:?}");
}

/// Seed 3130833 of the campaign: a bore touching a block's side from inside
/// leaves a cusp of matter between the side and its wall, and a block cut
/// from it has its side cross the cusp 1e-5 from the touch. The strip of the
/// side and the strip of the wall there are twins, and the first operand
/// covers both, turning its matter towards the other: a skin thinner than
/// the tolerance, taken for none, so that the operation is decided by the
/// block cut alone.
#[test]
fn twins_one_operand_covers_with_its_matter_between_them_are_a_skin_taken_for_none() {
    let bored = block([1.5, 0.5, 0.0], [8.5, 7.5, 5.0])
        .cut_by(&standing([4.99999, 4.0], 3.5, 0.0, 10.0))
        .expect("the block is bored");
    let cut = bored.cut_by(&block([3.0, 2.0, 1.0], [5.0, 9.0, 11.0]));
    let cut = cut.expect("the cusp is cut through");
    assert_eq!(
        crate::soundness::listed(&cut.listing(), cut.scale().reach()),
        Ok(())
    );
}

/// Seed 3089284 of the campaign, stood on Z: two pins of one radius 6e-8
/// apart leave a crescent whose walls cross at a grazing angle along a line
/// lying on the plane both touch, between their two lines of touch; a block
/// whose side is that plane is cut from it. The strip of the first pin's wall
/// between its line of touch and the crossing lies within the tolerance of
/// the block's side all across: the block wraps it as the side's matter
/// lies, the pin standing outside the side it touches, and no ray is cast
/// from a point a femtometre off the side.
#[test]
fn a_strip_of_a_wall_within_the_tolerance_of_a_side_all_across_is_wound_as_the_side_s_matter_lies()
{
    let crescent = standing([5.0, 4.0], 1.0, -3.5, 1.5)
        .cut_by(&standing([5.00000006, 4.0], 1.0, -8.49999999, 1.50000001))
        .expect("the crescent is cut");
    let cut = crescent.cut_by(&block([4.0, 3.0, -7.5], [10.0, 11.0, 2.5]));
    assert!(cut.is_ok(), "{cut:?}");
}

/// Seed 3049074 of the campaign: a bore of radius a hair over a half pokes
/// 6e-8 past the wall of a bore of radius 1.5 it all but touches inside, the
/// two crossing at a grazing angle along two lines 3e-4 either side of the
/// touch. 2.9e-4 round, inside the stretch between those lines, the small
/// wall stands 5e-9 outside the large one; the gap read to the first order
/// there said the large stood 1e-7 outside the small. Two walls crossing
/// stand one above the other all along each stretch between the lines they
/// cross along, read at its middle, where they stand furthest apart.
#[test]
fn two_walls_crossing_at_a_grazing_angle_are_ordered_at_the_middle_of_the_stretch_between_their_lines()
 {
    let bored = standing([8.0, 9.0], 2.5, -2.0, 0.0)
        .cut_by(&standing([9.0, 9.0], 1.5, -3.0, 1.0))
        .expect("the stock is bored");
    let bore = standing([10.0, 9.0], 0.50000006, -2.0, 0.0);
    let operands = Operands::of(&bored, &bore, bored.scale().joined(bore.scale()));
    let wall = |radius: f64| {
        let rank = operands
            .surfaces
            .list
            .iter()
            .position(|surface| {
                matches!(surface, Surface::Cylinder(cylinder) if (cylinder.radius - radius).abs() < 1e-6)
            })
            .expect("the wall is there");
        SurfaceId(rank as u32)
    };
    let pair = [wall(0.5), wall(1.5)];
    let place = DVec3::new(10.499999977670807, 8.999713069353858, -1.0);
    assert_eq!(
        wrapped::lies_above(&operands, operands.scale_of(pair), pair, place),
        Some(false)
    );
}

/// Seed 3192987 of the campaign, shrunk another way and stood on Z: a bar
/// bored along its length by a bore touching it inside at its top, then a
/// block whose top both walls touch, 1.8e-6 apart, joined to it. The strip
/// of the block's top between the two lines of touch stands within the
/// tolerance of both walls all across, and both lie under it: the nearer,
/// the bar's, tells that the strip stands outside the bar's matter, which
/// the bore's alone would put it in.
#[test]
fn a_strip_both_walls_of_a_cusp_lie_under_is_wound_as_the_nearer_wall_s_matter_lies() {
    let bored = standing([344.9999982, 285.0], 90.0, -89.99999, 480.00001)
        .cut_by(&standing([345.0, 315.0], 60.0, -89.99999, 1050.00001))
        .expect("the bar is bored");
    let block = block([255.0, 195.0, -60.0], [435.0, 375.0, 195.0]);
    let operands = Operands::of(&bored, &block, bored.scale().joined(block.scale()));
    let arena = laid(&operands).expect("the arena is laid");
    let selected = selected(&operands, &arena, Operation::Or);
    assert!(selected.is_ok(), "{:?}", selected.err());
}

/// Seed 3150523 of the campaign: a stock bored by a bore touching it inside
/// at its top, then a pin a hair over half a unit across, whose bottom dips
/// 2e-7 into both walls, cut from above. The pin's wall crosses the stock's
/// 4.08e-4 either side of the touch and the bore's 4e-4: 4.04e-4 round from
/// its bottom, between the two, the stock's wall lies inside the pin's and
/// the bore's outside it. The walls face each other there, so a point of
/// the pin's wall inside the stock has the stock's wall on its inner side.
#[test]
fn two_walls_facing_each_other_across_a_crossing_are_ordered_by_which_way_each_faces() {
    let bored = standing([6.0, 0.0], 2.5, -9.0, 1.0)
        .cut_by(&standing([6.0, 0.5], 2.0, -9.0, 1.0))
        .expect("the stock is bored");
    let pin = standing([6.0, 3.0], 0.5000002, -9.0, 1.0);
    let operands = Operands::of(&bored, &pin, bored.scale().joined(pin.scale()));
    let wall = |radius: f64| {
        let rank = operands
            .surfaces
            .list
            .iter()
            .position(|surface| {
                matches!(surface, Surface::Cylinder(cylinder) if (cylinder.radius - radius).abs() < 1e-9)
            })
            .expect("the wall is there");
        SurfaceId(rank as u32)
    };
    let place = DVec3::new(5.999595875815821, 2.499999963316318, -4.0);
    let above =
        |pair: [SurfaceId; 2]| wrapped::lies_above(&operands, operands.scale_of(pair), pair, place);
    assert_eq!(above([wall(0.5000002), wall(2.5)]), Some(false));
    assert_eq!(above([wall(0.5000002), wall(2.0)]), Some(true));
}
