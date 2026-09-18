//! What adding and taking away matter leaves behind.

use glam::DVec3;

use crate::mesh::tests::{box_of, volume};
use crate::sweep::Loop;

#[test]
fn adding_two_solids_apart_keeps_both() {
    let a = box_of(10.0, 10.0, DVec3::ZERO);
    let b = box_of(10.0, 10.0, DVec3::new(30.0, 0.0, 0.0));
    assert!((volume(&a.union(&b)) - 2000.0).abs() < 1.0);
}

/// Overlapping matter is counted once, not twice.
#[test]
fn adding_two_solids_that_overlap_counts_the_middle_once() {
    let a = box_of(10.0, 10.0, DVec3::ZERO);
    let b = box_of(10.0, 10.0, DVec3::new(5.0, 0.0, 0.0));
    let joined = volume(&a.union(&b));
    assert!((joined - 1500.0).abs() < 1.0, "{joined}");
}

/// The case this is all for: a pocket cut into a block.
#[test]
fn cutting_a_pocket_takes_matter_away() {
    let block = box_of(10.0, 10.0, DVec3::ZERO);
    let tool = box_of(4.0, 20.0, DVec3::new(3.0, 3.0, -5.0));
    let cut = volume(&block.difference(&tool));
    assert!((cut - (1000.0 - 160.0)).abs() < 1.0, "{cut}");
}

/// Faces landing exactly on each other are where a boolean goes wrong, and
/// two cuts in a row is when it shows. This has to come out clean, and above
/// all it has to come back at all.
#[test]
fn cutting_twice_with_faces_that_land_on_each_other() {
    let block = box_of(20.0, 10.0, DVec3::ZERO);
    // Both tools start exactly on the block's bottom face and one shares a
    // side with it.
    let small = box_of(4.0, 10.0, DVec3::new(2.0, 2.0, 0.0));
    let wide = box_of(8.0, 10.0, DVec3::new(0.0, 8.0, 0.0));

    let once = block.difference(&small);
    let twice = once.difference(&wide);
    let left = volume(&twice);
    assert!((left - (4000.0 - 160.0 - 640.0)).abs() < 2.0, "{left}");
}

/// Two areas extruded together are one tool, whatever they overlap.
#[test]
fn two_tools_joined_then_cut_leave_one_shape() {
    let block = box_of(20.0, 10.0, DVec3::ZERO);
    let small = box_of(4.0, 30.0, DVec3::new(2.0, 2.0, -10.0));
    let wide = box_of(9.0, 30.0, DVec3::new(1.0, 1.0, -10.0));

    // The small one is entirely inside the wide one.
    let tool = small.union(&wide);
    let left = volume(&block.difference(&tool));
    assert!((left - (4000.0 - 810.0)).abs() < 2.0, "{left}");
}

/// The case that brought the application down: a part made of hundreds of
/// small facets, dug into. The tree of planes degenerates there into a long
/// chain, and a recursive version exhausts the stack.
#[test]
fn cutting_into_a_many_faceted_solid_comes_back() {
    let steps = 96;
    let outline: Vec<glam::DVec2> = vec![
        glam::DVec2::new(3.0, 0.0),
        glam::DVec2::new(9.0, 0.0),
        glam::DVec2::new(9.0, 6.0),
        glam::DVec2::new(3.0, 6.0),
    ];
    let triangles = vec![
        [outline[0], outline[1], outline[2]],
        [outline[0], outline[2], outline[3]],
    ];
    let cylinder = crate::sweep::revolution(
        Loop::straight(&outline),
        &[],
        &triangles,
        |point| DVec3::new(point.x, point.y, 0.0),
        glam::DVec2::ZERO,
        glam::DVec2::Y,
        std::f64::consts::TAU,
    )
    .expect("a cylinder");
    assert!(cylinder.polygons.len() > steps, "enough facets");

    let tool = box_of(3.0, 30.0, DVec3::new(4.0, 1.0, -15.0));
    let cut = cylinder.difference(&tool);
    assert!(volume(&cut) < volume(&cylinder));
    assert!(volume(&cut) > 0.0);
}

#[test]
fn cutting_with_something_that_misses_changes_nothing() {
    let block = box_of(10.0, 10.0, DVec3::ZERO);
    let tool = box_of(4.0, 4.0, DVec3::new(40.0, 40.0, 0.0));
    assert!((volume(&block.difference(&tool)) - 1000.0).abs() < 1.0);
}

/// Cutting a solid clean through leaves nothing behind.
#[test]
fn cutting_everything_away_leaves_nothing() {
    let block = box_of(10.0, 10.0, DVec3::ZERO);
    let tool = box_of(30.0, 30.0, DVec3::new(-10.0, -10.0, -10.0));
    let left = volume(&block.difference(&tool));
    assert!(left.abs() < 1.0, "{left}");
}
