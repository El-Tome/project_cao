use std::f64::consts::TAU;

use glam::{DVec2, DVec3};

use super::*;
use crate::brep::curve::Curve;
use crate::brep::relation::{Relation, relation};
use crate::brep::scale::Scale;
use crate::brep::surface::{Plane, Surface};
use crate::brep::topology::{Edge, Vertex};
use crate::profile::{Contour, Frame, Run};
use crate::soundness::listed;

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

fn arena(first: &Body, second: &Body) -> Arena {
    let operands = Operands::of(first, second, first.scale().joined(second.scale()));
    laid(&operands).expect("the arena is laid")
}

/// The surfaces an arc of the arena lies on.
fn support_of<'a>(laid: &'a Arena, edge: &Edge) -> &'a [SurfaceId] {
    let rank = laid
        .body
        .edges
        .iter()
        .position(|known| std::ptr::eq(known, edge))
        .expect("an arc of the arena");
    &laid.supports[rank]
}

/// The ranks of the planes of `body` at `offset` along a world axis.
fn plane_at(body: &Body, axis: DVec3, offset: f64) -> SurfaceId {
    let rank = body
        .surfaces
        .iter()
        .position(|surface| {
            matches!(surface, Surface::Plane(plane)
                if plane.normal.abs_diff_eq(axis, 1e-12) && (plane.offset() - offset).abs() < 1e-12)
        })
        .expect("the plane is there");
    SurfaceId(rank as u32)
}

#[test]
fn two_blocks_sharing_a_wall_share_the_four_corners_of_that_wall() {
    let one = block([-20.0, -20.0, 0.0], [20.0, 20.0, 10.0]);
    let other = block([20.0, -20.0, 0.0], [60.0, 20.0, 10.0]);
    let laid = arena(&one, &other);
    assert_eq!(laid.body.vertices.len(), 12);
    let wall = plane_at(&laid.body, DVec3::X, 20.0);
    let on_the_wall: Vec<&Vertex> = laid
        .body
        .vertices
        .iter()
        .filter(|vertex| vertex.on.contains(&wall))
        .collect();
    assert_eq!(on_the_wall.len(), 4);
    for vertex in on_the_wall {
        assert_eq!(vertex.on.len(), 3, "{vertex:?}");
    }
}

#[test]
fn a_hole_s_circle_on_the_cap_it_is_flush_with_is_one_edge_on_the_cap_and_on_the_hole() {
    let stock = standing([0.0, 0.0], 20.0, 0.0, 10.0);
    let hole = standing([8.0, 0.0], 5.0, 0.0, 10.0);
    let laid = arena(&stock, &hole);
    let top = plane_at(&laid.body, DVec3::Z, 10.0);
    let on_the_top_of_the_hole: Vec<&Edge> = laid
        .body
        .edges
        .iter()
        .filter(|edge| {
            matches!(laid.body.curve(edge.curve), Curve::Circle(circle)
                if circle.radius == 5.0 && circle.center.z == 10.0)
        })
        .collect();
    let [edge] = on_the_top_of_the_hole.as_slice() else {
        panic!("one edge: {on_the_top_of_the_hole:?}");
    };
    assert_eq!(edge.ends, None);
    let support = support_of(&laid, edge);
    assert_eq!(support.len(), 2);
    assert!(support.contains(&top), "{support:?}");
    assert_eq!(laid.body.edges.len(), 4);
}

#[test]
fn a_circle_where_a_top_meets_a_cylinder_is_kept_across_the_top_and_nowhere_else() {
    let one = block([0.0, 0.0, 0.0], [10.0, 10.0, 10.0]);
    let tall = standing([12.0, 5.0], 5.0, -5.0, 15.0);
    let laid = arena(&one, &tall);
    let kept: Vec<&Edge> = laid
        .body
        .edges
        .iter()
        .filter(|edge| {
            matches!(laid.body.curve(edge.curve), Curve::Circle(circle) if circle.center.z == 10.0)
        })
        .collect();
    let [edge] = kept.as_slice() else {
        panic!("one arc of the circle: {kept:?}");
    };
    let middle = laid
        .body
        .curve(edge.curve)
        .point((edge.from + edge.to) / 2.0);
    assert!(middle.x < 10.0, "{middle}");
    assert!(edge.ends.is_some());
}

#[test]
fn an_edge_of_one_block_crossing_the_other_s_plane_off_its_face_makes_no_corner() {
    let one = block([-20.0, -20.0, 0.0], [20.0, 20.0, 10.0]);
    let pocket = block([10.0, -5.0, 5.0], [20.0, 5.0, 10.0]);
    let laid = arena(&one, &pocket);
    let far = DVec3::new(10.0, -20.0, 10.0);
    assert!(
        laid.body
            .vertices
            .iter()
            .all(|vertex| vertex.point.distance(far) > 1.0)
    );
    assert_eq!(laid.body.vertices.len(), 16);
}

#[test]
fn a_ruling_where_a_hole_touches_a_side_is_one_edge_between_the_two_corners_it_touches_at() {
    let one = block([-20.0, -20.0, 0.0], [20.0, 20.0, 10.0]);
    let hole = standing([15.0, 0.0], 5.0, 0.0, 10.0);
    let laid = arena(&one, &hole);
    let rulings: Vec<&Edge> = laid
        .body
        .edges
        .iter()
        .filter(|edge| {
            matches!(laid.body.curve(edge.curve), Curve::Line(line)
                if line.direction.abs_diff_eq(DVec3::Z, 1e-12)
                    && line.origin.abs_diff_eq(DVec3::new(20.0, 0.0, 0.0), 1e-9))
        })
        .collect();
    let [ruling] = rulings.as_slice() else {
        panic!("one ruling: {rulings:?}");
    };
    let ends = ruling.ends.expect("a ruling has two corners");
    let heights = ends.map(|end| laid.body.vertex(end).point.z);
    assert_eq!(heights, [0.0, 10.0]);
    assert_eq!(support_of(&laid, ruling).len(), 2);
}

/// A profile drawn on the plane `y = offset` in `(x, z)` and raised towards
/// `-y`.
fn across(offset: f64, outline: Contour, height: f64) -> Body {
    let frame = Frame {
        origin: DVec3::Y * offset,
        u: DVec3::X,
        v: DVec3::Z,
    };
    Body::raised(&outline, &[], frame, DVec3::NEG_Y * height).expect("a prism raises")
}

fn rectangle(low: [f64; 2], high: [f64; 2]) -> Contour {
    Contour::rectangle(DVec2::from(low), DVec2::from(high))
}

/// Seed 750 of the campaign: a corner an earlier cut left on the top plane
/// and on the bored cylinder stands on the one of the two lines they meet
/// along that is no edge of the body.
#[test]
fn a_corner_on_a_plane_and_a_cylinder_lies_on_the_line_it_stands_on_even_where_that_line_is_no_edge()
 {
    let bored = across(7.0, rectangle([6.0, 8.0], [12.5, 10.5]), 8.0)
        .joined(&across(7.0, rectangle([5.0, 9.0], [9.5, 11.5]), 16.0))
        .and_then(|joined| {
            let center = DVec2::new(9.499_999_98, 10.25);
            let circle = Contour {
                corners: vec![center + DVec2::X * 2.0],
                runs: vec![Run::Round { center, turn: TAU }],
            };
            joined.cut_by(&across(8.0, circle, 32.0))
        })
        .expect("the block is bored");
    let cut = bored.cut_by(&across(7.0, rectangle([6.0, 6.0], [14.0, 9.0]), 32.0));
    assert!(cut.is_ok(), "{cut:?}");
}

fn standing_on(offset: f64, outline: Contour, height: f64) -> Body {
    Body::raised(&outline, &[], ground(offset), DVec3::Z * height).expect("a prism raises")
}

/// Seed 533 of the campaign: a hole whose axis lies in a side of the block
/// and whose wall touches a face an earlier join left, a hair above the top.
#[test]
fn a_corner_on_two_of_the_three_surfaces_a_line_lies_on_cuts_that_line() {
    let joined = standing_on(3.0, rectangle([0.0, 3.0], [2.5, 6.0]), 9.5)
        .joined(&standing_on(
            2.0,
            rectangle([1.000_000_1, 4.0], [4.5, 5.0]),
            6.0,
        ))
        .expect("the blocks join");
    let center = DVec2::new(2.5, 4.5);
    let circle = Contour {
        corners: vec![center + DVec2::X * 0.5],
        runs: vec![Run::Round { center, turn: TAU }],
    };
    let cut = joined.cut_by(&standing_on(3.000_000_02, circle, 9.500_000_1));
    assert!(cut.is_ok(), "{cut:?}");
}

/// Seed 197 of the campaign: a bar lying along X whose side touches, from
/// inside, the wall of a cylinder standing on Z. The curve they meet along is
/// a figure of eight through the point of touch, and passes it twice.
#[test]
fn a_bar_touching_a_cylinder_s_wall_from_inside_is_cut_where_their_curve_crosses_itself() {
    let frame = Frame {
        origin: DVec3::X * 7.0,
        u: DVec3::Y,
        v: DVec3::Z,
    };
    let center = DVec2::new(2.0, 4.0);
    let circle = Contour {
        corners: vec![center + DVec2::X * 2.5],
        runs: vec![Run::Round { center, turn: TAU }],
    };
    let bar = Body::raised(&circle, &[], frame, DVec3::X * 7.0).expect("a bar raises");
    let post = standing([10.0, 4.0], 4.5, -2.0, 5.0);
    let joined = bar.joined(&post).expect("the bar and the post join");
    assert_eq!(listed(&joined.listing(), joined.scale().reach()), Ok(()));
    let apart = block([20.0, 20.0, 0.0], [22.0, 22.0, 2.0]);
    let again = joined.joined(&apart).expect("a block apart joins");
    assert_eq!(listed(&again.listing(), again.scale().reach()), Ok(()));
    assert_eq!(again.edges.len(), joined.edges.len() + 12);
}

/// Seed 793 of the campaign, made small: a boss whose cap stands a hair above
/// the top is joined onto it, then another whose cap stands a hair below.
/// Each cap is the top within the tolerance, and each circle crosses the
/// other where the top and the two walls meet: one corner, whichever circle
/// it is found on.
#[test]
fn two_circles_laid_on_a_top_from_a_hair_either_side_of_it_cross_at_one_corner() {
    let hair = 0.8 * Scale::of(20.0).eps();
    let one = block([0.0, 0.0, 0.0], [20.0, 20.0, 10.0]);
    let above = standing([20.0, 8.0], 4.0, 5.0, 10.0 + hair);
    let below = standing([20.0, 12.0], 4.0, 5.0, 10.0 - hair);
    let body = one
        .joined(&above)
        .and_then(|joined| joined.joined(&below))
        .expect("the bosses join");
    assert_eq!(listed(&body.listing(), body.scale().reach()), Ok(()));
}

/// Seeds 6000830 and 5000221 of the campaign: a bar lying along X across a
/// post standing on Z, the post's wall reaching the bar's end cap just where
/// the cap stands tangent to it, or its side where the post's side does. The
/// curve the two meet along touches the cap's circle there to the fourth
/// order: the two leave that corner with one direction and one bend, and
/// rounding ordered them.
#[test]
fn a_curve_touching_a_circle_to_the_fourth_order_is_ordered_by_where_it_goes() {
    let bar = |center: [f64; 2], radius: f64, from: f64, length: f64| {
        let frame = Frame {
            origin: DVec3::X * from,
            u: DVec3::Y,
            v: DVec3::Z,
        };
        let center = DVec2::from(center);
        let circle = Contour {
            corners: vec![center + DVec2::X * radius],
            runs: vec![Run::Round { center, turn: TAU }],
        };
        Body::raised(&circle, &[], frame, DVec3::X * length).expect("a bar raises")
    };
    let joined = standing([10.0, 7.0], 6.0, 0.0, 9.0)
        .joined(&bar([10.0, 8.0], 3.0, -2.0, 18.0))
        .expect("the bar crosses the post");
    assert_eq!(listed(&joined.listing(), joined.scale().reach()), Ok(()));
    let bored = standing([30.0, 5.0], 25.0, -5.0, 35.0)
        .cut_by(&bar([35.0, 15.0], 30.0, 5.0, 18.0))
        .expect("the post is bored");
    assert_eq!(listed(&bored.listing(), bored.scale().reach()), Ok(()));
}

/// A profile drawn on the plane `x = offset` in `(y, z)` and raised towards
/// `+x`.
fn along_x(offset: f64, outline: Contour, height: f64) -> Body {
    let frame = Frame {
        origin: DVec3::X * offset,
        u: DVec3::Y,
        v: DVec3::Z,
    };
    Body::raised(&outline, &[], frame, DVec3::X * height).expect("a prism raises")
}

fn circle(center: [f64; 2], radius: f64) -> Contour {
    let center = DVec2::from(center);
    Contour {
        corners: vec![center + DVec2::X * radius],
        runs: vec![Run::Round { center, turn: TAU }],
    }
}

/// Seed 1019570 of the campaign: a block given a boss whose circle touches
/// its bottom and its top, its centre two hundredths of a micron short of
/// the block's side. On the cap both stand on, the circle runs from where it
/// touches the bottom to the corner within a hair of the bottom's edge: the
/// two are one edge, a line, lying on the boss's wall too.
#[test]
fn a_circle_running_within_the_tolerance_of_a_side_between_two_corners_is_that_side() {
    let block = along_x(4.0, rectangle([2.0, 8.0], [4.5, 13.0]), 5.0);
    let boss = along_x(4.0, circle([4.499_999_98, 10.5], 2.5), 9.0);
    let laid = arena(&block, &boss);
    let body = &laid.body;
    let [touch, corner] = [4.499_999_98, 4.5].map(|y| DVec3::new(4.0, y, 8.0));
    let vertex = |point: DVec3| {
        body.vertex_ids()
            .find(|id| body.vertex(*id).point.distance(point) < 1e-12)
            .expect("a corner there")
    };
    let [touch, corner] = [touch, corner].map(vertex);
    let between: Vec<&Edge> = body
        .edges
        .iter()
        .filter(|edge| matches!(edge.ends, Some(ends) if ends.contains(&touch) && ends.contains(&corner)))
        .collect();
    let [edge] = between.as_slice() else {
        panic!("one edge between the two corners: {between:?}");
    };
    assert!(matches!(body.curves[edge.curve.0 as usize], Curve::Line(_)));
    let wall = SurfaceId(
        body.surfaces
            .iter()
            .position(|surface| matches!(surface, Surface::Cylinder(_)))
            .expect("the boss's wall") as u32,
    );
    assert!(support_of(&laid, edge).contains(&wall));
}

/// Seed 1026137 of the campaign: a block cut by a block flush with its side
/// and its bottom, both a twentieth of a micron off, within the tolerance at
/// a reach of 65, and apart from its end. The tool's corner on its end, the
/// side and the bottom kept the tool's own place, which the lines the block's
/// side and bottom share with that end pass seven hundredths of a micron
/// from: three planes whose normals span space fix one place, and a corner
/// on them stands there.
#[test]
fn a_corner_on_three_planes_whose_normals_span_space_stands_where_they_meet() {
    let block = standing_on(15.0, rectangle([7.5, 7.5], [42.5, 42.5]), 50.0);
    let flush = along_x(
        10.000_000_1,
        rectangle(
            [42.499_999_95, 14.999_999_95],
            [62.500_000_05, 35.000_000_05],
        ),
        50.0,
    );
    let cut = block
        .cut_by(&flush)
        .expect("a block flush with a side is cut");
    assert_eq!(listed(&cut.listing(), cut.scale().reach()), Ok(()));
}

/// Seed 1044340 of the campaign, shrunk: a bar lying along X, its axis on
/// the top of a post lying along Y, is cut from the post. The post's wall
/// inside the loop the two meet along and the bar's are both bounded by that
/// one loop alone, and far apart: two caps, not one piece of surface, and
/// the groove is cut.
#[test]
fn two_walls_bounded_by_the_one_loop_they_meet_along_are_two_faces() {
    let post = Body::raised(
        &circle([10.0, 30.0], 20.0),
        &[],
        Frame {
            origin: DVec3::Y * 40.0,
            u: DVec3::X,
            v: DVec3::Z,
        },
        DVec3::NEG_Y * 45.0,
    )
    .expect("a post raises");
    let bar = along_x(-10.0, circle([25.0, 50.0], 8.0), 45.0);
    let grooved = post.cut_by(&bar).expect("the bar is cut from the post");
    assert_eq!(listed(&grooved.listing(), grooved.scale().reach()), Ok(()));
    assert!(
        grooved.volume() < post.volume() - 1000.0,
        "{}",
        grooved.volume()
    );
}

/// Seed 6000233 of the campaign: a block given a boss on its side and bored
/// by a hole whose wall touches the boss's, both centred on the side. The
/// line where the side cuts the hole a second time was never registered as
/// lying on the side: the pair of the two came to share a curve only as
/// another pair was completed, and a corner on the side and the hole was put
/// on the other line, where the hole touches the boss.
#[test]
fn a_pair_a_curve_comes_to_lie_on_as_another_pair_is_completed_is_completed_too() {
    let bossed = block([5.0, 9.0, 0.0], [10.5, 15.0, 8.0])
        .joined(&standing([10.5, 12.0], 1.0, -1.0, 14.0))
        .and_then(|bossed| bossed.cut_by(&standing([10.5, 14.0], 1.0, -1.0, 29.0)))
        .expect("the block is bossed and bored");
    let cut = bossed
        .cut_by(&block([1.0, 0.0, 5.0], [11.0, 10.0, 8.0]))
        .expect("a corner is cut off");
    assert_eq!(listed(&cut.listing(), cut.scale().reach()), Ok(()));
}

/// A bar a hair into the plane of a block's top, standing far beside the
/// block: no face of the one stands near a face of the other, so the two
/// touch nowhere, and the bar is left where it was drawn.
#[test]
fn a_bar_a_hair_into_a_plane_whose_faces_stand_far_from_its_own_is_not_moved_onto_it() {
    let block = block([0.0, 0.0, 0.0], [10.0, 10.0, 10.0]);
    let eps = Scale::of(53.0).eps();
    let bar = along_x(0.0, circle([50.0, 7.0], 3.0 + eps / 2.0), 10.0);
    let operands = Operands::of(&block, &bar, block.scale().joined(bar.scale()));
    let top = Surface::Plane(Plane::through(DVec3::Z * 10.0, DVec3::Z).0);
    let walls: Vec<&Surface> = operands
        .surfaces
        .list
        .iter()
        .filter(|surface| matches!(surface, Surface::Cylinder(_)))
        .collect();
    let [wall] = walls[..] else {
        panic!("one wall: {walls:?}");
    };
    let own = bar
        .surfaces
        .iter()
        .find(|surface| matches!(surface, Surface::Cylinder(_)))
        .expect("the bar's wall");
    assert!(matches!(
        relation(&top, own, operands.scale),
        Relation::Tangent(_)
    ));
    assert_eq!(wall, own);
}

/// A slot from `from` to `to` in `(x, y)`, of radius `radius`, standing on
/// `z = offset` and raised by `height`.
fn slot(offset: f64, [from, to]: [[f64; 2]; 2], radius: f64, height: f64) -> Body {
    let [from, to] = [DVec2::from(from), DVec2::from(to)];
    let across = (to - from).normalize().perp() * radius;
    let outline = Contour {
        corners: vec![from - across, to - across, to + across, from + across],
        runs: vec![
            Run::Straight,
            Run::Round {
                center: to,
                turn: TAU / 2.0,
            },
            Run::Straight,
            Run::Round {
                center: from,
                turn: TAU / 2.0,
            },
        ],
    };
    standing_on(offset, outline, height)
}

/// Every corner of `body` stands on every surface it lies on, and every edge
/// ends on its corners, within rounding.
fn on_its_surfaces(body: &Body) {
    let rounding = 1e-12 * body.reach();
    for vertex in &body.vertices {
        for on in &vertex.on {
            let off = body.surface(*on).distance(vertex.point).abs();
            assert!(off < rounding, "{vertex:?} stands {off:e} off {on:?}");
        }
    }
    for edge in body.edge_ids() {
        let stretch = body.edge(edge);
        let ends = stretch.ends.into_iter().flatten();
        for (end, at) in ends.zip([stretch.from, stretch.to]) {
            let off = body.point_on(edge, at).distance(body.vertex(end).point);
            assert!(off < rounding, "{edge:?} ends {off:e} off {end:?}");
        }
    }
}

/// Seed 8511092 of the profile draw: a slot whose top stands a hair under
/// the block's, taken for it. The corners where its runs meet its caps'
/// arcs stand on the block's top, where the lines its runs touch its caps
/// along cross it, and not a hair under it where the slot was drawn.
#[test]
fn a_slot_s_corners_on_a_top_taken_for_the_block_s_stand_on_that_top() {
    let block = block([-45.0, 165.0, -225.0], [105.0, 315.0, 0.0]);
    let hair = 3e-7;
    let slot = slot(-450.0 - hair, [[30.0, 240.0], [-30.0, 240.0]], 37.5, 450.0);
    let cut = block.cut_by(&slot).expect("the slot is cut");
    on_its_surfaces(&cut);
}

/// Seed 8581852 of the profile draw: a block given a second whose side
/// stands a tenth of a micron off its own, told apart at their reach, then a
/// disc touching the first side, raised far enough to bring both within the
/// tolerance of its wall. The wall touches the side it touches and crosses
/// the other, as the two blocks told them apart.
#[test]
fn a_disc_touching_one_of_two_sides_a_block_told_apart_a_hair_off_is_joined() {
    let blocks = along_x(12.5, rectangle([20.0, 30.0], [43.0, 43.0]), 45.0)
        .joined(&along_x(
            12.5,
            rectangle([20.000_000_1, 30.0], [42.500_000_1, 42.5]),
            -90.0,
        ))
        .expect("the blocks join");
    let disc = along_x(12.500_000_1, circle([25.0, 35.0], 5.0), 180.0);
    let joined = blocks.joined(&disc).expect("the disc joins");
    on_its_surfaces(&joined);
}

/// Seed 8 582 887 of the campaign through the application's body: a disc
/// whose circle starts at a slant, joined to a block whose side its wall
/// touches. Moved onto the touch, the wall's rim starts a rounding past
/// nought on the curve it is laid on, and the one corner the touch puts on
/// it cuts it into an arc running from that corner round to it again,
/// whose middle stands a rounding past the stretch the rim was measured
/// over. The rim is whole all the same: the disc's floor is kept with it.
#[test]
fn a_rim_held_whole_is_kept_whole_wherever_rounding_starts_it() {
    let block = block([-6.0, 2.0, -2.0], [1.0, 10.0, 6.0]);
    let center = DVec2::new(6.0, 5.0);
    let rim = Contour {
        corners: vec![center + DVec2::from_angle(328.151_913_568_525_23_f64.to_radians()) * 5.0],
        runs: vec![Run::Round { center, turn: TAU }],
    };
    let disc = standing_on(0.0, rim, 7.0);
    let joined = block.joined(&disc).expect("the disc is joined");
    assert_eq!(joined.faces.len(), 9, "{:?}", joined.faces);
    assert_eq!(listed(&joined.listing(), joined.scale().reach()), Ok(()));
}

/// Seed 8 520 500 of the campaign: a slot whose cap's centre stands a hair
/// inside a block's side, drawn on a plane a hair off the block's back. Its
/// corner where a side meets the cap stands on three planes, each taken for
/// one of the block's: it stands where those meet, √2 hairs from where the
/// slot put it, and so does the corner where the cap's rim crosses the
/// block's side. Measured where the slot put it, the two were two corners
/// at one place.
#[test]
fn a_corner_found_where_an_operand_s_corner_stands_once_its_planes_are_taken_is_that_corner() {
    let block = across(180.0, rectangle([300.0, 90.0], [375.0, 195.0]), 150.0);
    let (from, to, radius) = (
        DVec2::new(359.999_999_7, 142.5),
        DVec2::new(299.999_999_7, 142.5),
        15.0,
    );
    let side = (to - from).normalize().perp() * radius;
    let slot = Contour {
        corners: vec![from - side, to - side, to + side, from + side],
        runs: vec![
            Run::Straight,
            Run::Round {
                center: to,
                turn: std::f64::consts::PI,
            },
            Run::Straight,
            Run::Round {
                center: from,
                turn: std::f64::consts::PI,
            },
        ],
    };
    let joined = block
        .joined(&across(179.999_999_7, slot, 300.0))
        .expect("the slot is joined");
    assert_eq!(listed(&joined.listing(), joined.scale().reach()), Ok(()));
}

/// Seed 8 554 524 of the campaign through the application's body: a disc
/// joined to a rounded bar whose end grazes the line the bar's far side
/// touches the disc's wall along, a hair off it. The curve the bar's end
/// and the disc's wall meet along is a loop below that side, cut by no
/// corner, turning at the very place the side touches the wall: read there
/// alone, it lay on the bar's end, and the disc's wall was parted by a loop
/// bounding nothing of the bar.
#[test]
fn a_loop_no_corner_cuts_grazing_a_face_at_its_middle_is_not_kept() {
    let disc = standing_on(6.0, circle([5.999_999_98, 0.0], 5.0), 8.0);
    let quarter = std::f64::consts::FRAC_PI_2;
    let [right, left] = [DVec2::new(10.75, 6.25), DVec2::new(8.25, 6.25)];
    let bar = Contour {
        corners: vec![
            DVec2::new(8.25, 4.0),
            DVec2::new(10.75, 4.0),
            DVec2::new(13.0, 6.25),
            DVec2::new(10.75, 8.5),
            DVec2::new(8.25, 8.5),
            DVec2::new(6.0, 6.25),
        ],
        runs: vec![
            Run::Straight,
            Run::Round {
                center: right,
                turn: quarter,
            },
            Run::Round {
                center: right,
                turn: quarter,
            },
            Run::Straight,
            Run::Round {
                center: left,
                turn: quarter,
            },
            Run::Round {
                center: left,
                turn: quarter,
            },
        ],
    };
    let joined = disc
        .joined(&across(7.0, bar, 2.0))
        .expect("the bar is joined");
    assert_eq!(listed(&joined.listing(), joined.scale().reach()), Ok(()));
}

/// Seed 8 010 303 of the campaign: a bore whose wall touches a block's
/// side, its axis a hair past the plane of the block's end. The corners
/// along the line that side and that end share, some found on the planes
/// and some on the bore where it touches the side, each stood where it was
/// found, a hair apart across the line: its edges leant across both
/// planes, and the bore's strip beside them was drawn through the side.
/// Every corner on the two planes stands on the line they share.
#[test]
fn a_corner_on_two_planes_across_each_other_stands_on_the_line_they_share() {
    let disc = across(5.0, circle([8.0, 8.0], 5.0), 3.0);
    let block = standing_on(7.0, rectangle([2.0, 1.0], [7.0, 5.0]), 5.0);
    let bore = standing_on(7.0, circle([7.000_000_01, 3.0], 2.0), 9.0);
    let cut = disc
        .cut_by(&block)
        .expect("the block is cut out of the disc");
    let laid = arena(&cut, &bore);
    let end = plane_at(&laid.body, DVec3::X, 7.0);
    let side = plane_at(&laid.body, DVec3::Y, 5.0);
    let on_both: Vec<&Vertex> = laid
        .body
        .vertices
        .iter()
        .filter(|vertex| vertex.on.contains(&end) && vertex.on.contains(&side))
        .collect();
    assert!(on_both.len() > 2, "{on_both:?}");
    for vertex in on_both {
        assert_eq!([vertex.point.x, vertex.point.y], [7.0, 5.0], "{vertex:?}");
    }
}
