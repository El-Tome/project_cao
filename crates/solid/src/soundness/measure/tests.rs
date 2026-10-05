//! What a solid encloses, measured along lines that take nothing it says of
//! itself on trust.
//!
//! Closes #448.
//! - the volume is what the operation promised — the same lines of measure
//!   for a boolean, within a tolerance written in the code — caught out by a
//!   solid broken on purpose —
//!   `a_smaller_cube_than_promised_is_a_volume_flaw_along_a_line_that_lost_matter`,
//!   `a_box_left_whole_inside_the_one_it_was_added_to_is_more_matter_than_promised`,
//!   `a_cube_turned_inside_out_is_not_the_cube_promised`

use std::f64::consts::TAU;

use super::*;
use crate::mesh::Mesh;
use crate::mesh::tests::{box_of, fan};
use crate::soundness::tests::{cube, unit_cube};
use crate::sweep::Loop;

fn spans(stretches: Vec<(f64, f64)>) -> Spans {
    Spans {
        stretches,
        surplus: 0.0,
    }
}

fn inside_out(triangles: &[Triangle]) -> Vec<Triangle> {
    triangles.iter().map(|[a, b, c]| [*a, *c, *b]).collect()
}

#[test]
fn a_unit_cube_encloses_one() {
    assert!((enclosed(&unit_cube()) - 1.0).abs() < 1e-12);
}

#[test]
fn a_cube_turned_inside_out_encloses_minus_one() {
    assert!((enclosed(&inside_out(&unit_cube())) + 1.0).abs() < 1e-12);
}

#[test]
fn the_length_of_spans_is_the_sum_of_their_stretches() {
    assert_eq!(spans(vec![(0.0, 1.0), (2.0, 4.0)]).length(), 3.0);
    assert_eq!(Spans::default().length(), 0.0);
}

#[test]
fn two_stretches_that_overlap_join_into_one() {
    let joined = spans(vec![(0.0, 2.0)]).union(&spans(vec![(1.0, 3.0)]));
    assert_eq!(joined.stretches(), [(0.0, 3.0)]);
}

#[test]
fn two_stretches_that_touch_join_into_one() {
    let joined = spans(vec![(0.0, 1.0)]).union(&spans(vec![(1.0, 2.0)]));
    assert_eq!(joined.stretches(), [(0.0, 2.0)]);
}

#[test]
fn stretches_apart_stay_apart_and_in_order() {
    let joined = spans(vec![(3.0, 4.0), (6.0, 7.0)]).union(&spans(vec![(0.0, 1.0), (5.0, 5.5)]));
    assert_eq!(
        joined.stretches(),
        [(0.0, 1.0), (3.0, 4.0), (5.0, 5.5), (6.0, 7.0)]
    );
}

#[test]
fn taking_a_stretch_out_of_the_middle_leaves_a_hole() {
    let left = spans(vec![(0.0, 10.0)]).without(&spans(vec![(4.0, 6.0)]));
    assert_eq!(left.stretches(), [(0.0, 4.0), (6.0, 10.0)]);
}

#[test]
fn taking_away_what_only_touches_a_stretch_leaves_it_whole() {
    let left = spans(vec![(0.0, 10.0)]).without(&spans(vec![(-2.0, 0.0), (10.0, 12.0)]));
    assert_eq!(left.stretches(), [(0.0, 10.0)]);
}

#[test]
fn taking_away_one_stretch_across_several_trims_each() {
    let left = spans(vec![(0.0, 2.0), (3.0, 5.0), (6.0, 8.0)]).without(&spans(vec![(1.0, 7.0)]));
    assert_eq!(left.stretches(), [(0.0, 1.0), (7.0, 8.0)]);
}

#[test]
fn taking_away_everything_leaves_nothing() {
    let left = spans(vec![(0.0, 1.0), (2.0, 3.0)]).without(&spans(vec![(-1.0, 5.0)]));
    assert_eq!(left, Spans::default());
}

#[test]
fn lines_across_a_unit_cube_measure_its_volume_within_a_thousandth() {
    let lines = Lines::across(DVec3::ZERO, DVec3::ONE, 64);
    let measured = lines.volume(&lines.inside(&unit_cube()));
    assert!((measured - 1.0).abs() < 1e-3, "{measured}");
}

#[test]
fn lines_across_a_wider_region_measure_the_same_cube_the_same() {
    let lines = Lines::across(DVec3::splat(-2.0), DVec3::splat(3.0), 128);
    let measured = lines.volume(&lines.inside(&unit_cube()));
    assert!((measured - 1.0).abs() < 1e-3, "{measured}");
}

#[test]
fn every_line_meets_a_cube_in_one_stretch_at_most() {
    let lines = Lines::across(DVec3::ZERO, DVec3::ONE, 64);
    let spans = lines.inside(&unit_cube());
    assert_eq!(spans.len(), 64 * 64);
    assert!(spans.iter().all(|line| line.stretches().len() <= 1));
    assert!(spans.iter().any(|line| line.stretches().len() == 1));
}

#[test]
fn a_cube_turned_inside_out_encloses_nothing_along_any_line() {
    let lines = Lines::across(DVec3::ZERO, DVec3::ONE, 32);
    let spans = lines.inside(&inside_out(&unit_cube()));
    assert!(spans.iter().all(|line| line.stretches().is_empty()));
}

#[test]
fn lines_count_a_cube_turned_inside_out_as_minus_one_as_enclosed_does() {
    let lines = Lines::across(DVec3::ZERO, DVec3::ONE, 64);
    let measured = lines.volume(&lines.inside(&inside_out(&unit_cube())));
    assert!((measured + 1.0).abs() < 1e-3, "{measured}");
}

#[test]
fn a_cube_turned_inside_out_is_not_the_cube_promised() {
    let lines = Lines::across(DVec3::ZERO, DVec3::ONE, 32);
    let promised = lines.inside(&unit_cube());
    let Err(Flaw::Volume {
        worst: Some(worst), ..
    }) = lines.compare(&promised, &lines.inside(&inside_out(&unit_cube())))
    else {
        panic!("a cube turned inside out holds no matter");
    };
    assert!(worst.enclosed < 0.0, "{worst:?}");
}

#[test]
fn a_line_through_an_edge_two_faces_share_is_counted_by_one_of_them() {
    let (a, b, c, d) = (DVec2::ZERO, DVec2::X, DVec2::ONE, DVec2::Y);
    let on_the_edge = DVec2::splat(0.5);
    let counted = [[a, b, d], [b, c, d]]
        .into_iter()
        .filter(|triangle| covered(*triangle, on_the_edge).is_some())
        .count();
    assert_eq!(counted, 1);
}

#[test]
fn a_line_through_a_corner_several_faces_share_is_counted_by_one_of_them() {
    let ring = [
        DVec2::new(1.0, -1.0),
        DVec2::ONE,
        DVec2::new(-1.0, 1.0),
        DVec2::NEG_ONE,
    ];
    let counted = (0..4)
        .filter(|index| {
            covered(
                [DVec2::ZERO, ring[*index], ring[(index + 1) % 4]],
                DVec2::ZERO,
            )
            .is_some()
        })
        .count();
    assert_eq!(counted, 1);
}

#[test]
fn the_same_cube_measured_twice_is_what_was_promised() {
    let lines = Lines::across(DVec3::ZERO, DVec3::ONE, 32);
    assert_eq!(
        lines.compare(&lines.inside(&unit_cube()), &lines.inside(&unit_cube())),
        Ok(())
    );
}

#[test]
fn a_smaller_cube_than_promised_is_a_volume_flaw_along_a_line_that_lost_matter() {
    let lines = Lines::across(DVec3::ZERO, DVec3::ONE, 32);
    let promised = lines.inside(&unit_cube());
    let smaller = lines.inside(&cube(DVec3::ZERO, DVec3::splat(0.5)));
    let Err(Flaw::Volume {
        promised: whole,
        enclosed,
        worst: Some(worst),
    }) = lines.compare(&promised, &smaller)
    else {
        panic!("a cube an eighth the size is not what was promised");
    };
    assert!((whole - 1.0).abs() < 0.02, "{whole}");
    assert!((enclosed - 0.125).abs() < 0.01, "{enclosed}");
    assert!(worst.promised > worst.enclosed, "{worst:?}");
    assert!((worst.direction.length() - 1.0).abs() < 1e-12);
}

#[test]
fn the_worst_line_is_the_one_the_two_disagree_most_along() {
    let lines = Lines::across(DVec3::ZERO, DVec3::ONE, 16);
    let promised = lines.inside(&unit_cube());
    let (slight, most) = (3 * 16 + 7, 9 * 16 + 8);
    let [(from, to)] = *promised[slight].stretches() else {
        panic!("line {slight} crosses the cube once");
    };
    let [(start, end)] = *promised[most].stretches() else {
        panic!("line {most} crosses the cube once");
    };
    let mut enclosed = promised.clone();
    enclosed[slight] = spans(vec![(from + 0.01, to)]);
    enclosed[most] = Spans::default();

    let Err(Flaw::Volume {
        worst: Some(worst), ..
    }) = lines.compare(&promised, &enclosed)
    else {
        panic!("two lines lost matter");
    };
    assert_eq!((worst.promised, worst.enclosed), (end - start, 0.0));
    let middle = worst.origin + worst.direction * (start + end) / 2.0;
    assert!(
        middle.cmpgt(DVec3::ZERO).all() && middle.cmplt(DVec3::ONE).all(),
        "the line reported is not the one that lost matter: {middle}"
    );
}

#[test]
fn two_cubes_overlapping_enclose_along_every_line_what_the_box_they_make_does() {
    let lines = Lines::across(DVec3::splat(-1.0), DVec3::new(4.0, 2.0, 2.0), 48);
    let left = lines.inside(&cube(DVec3::ZERO, DVec3::new(2.0, 1.0, 1.0)));
    let right = lines.inside(&cube(DVec3::new(1.0, 0.0, 0.0), DVec3::new(3.0, 1.0, 1.0)));
    let together: Vec<Spans> = left.iter().zip(&right).map(|(l, r)| l.union(r)).collect();
    let whole = lines.inside(&cube(DVec3::ZERO, DVec3::new(3.0, 1.0, 1.0)));
    assert_eq!(lines.compare(&whole, &together), Ok(()));
}

#[test]
fn a_cube_cut_by_another_encloses_along_every_line_what_is_left_of_it() {
    let lines = Lines::across(DVec3::splat(-1.0), DVec3::new(4.0, 2.0, 2.0), 48);
    let block = lines.inside(&cube(DVec3::ZERO, DVec3::new(3.0, 1.0, 1.0)));
    let tool = lines.inside(&cube(
        DVec3::new(2.0, -1.0, -1.0),
        DVec3::new(4.0, 2.0, 2.0),
    ));
    let cut: Vec<Spans> = block.iter().zip(&tool).map(|(b, t)| b.without(t)).collect();
    let left = lines.inside(&cube(DVec3::ZERO, DVec3::new(2.0, 1.0, 1.0)));
    assert_eq!(lines.compare(&left, &cut), Ok(()));
}

fn lines_around(solids: &[&Mesh], count: usize) -> Lines {
    let (mut low, mut high) = (DVec3::INFINITY, DVec3::NEG_INFINITY);
    for (least, most) in solids.iter().filter_map(|solid| solid.bounds()) {
        (low, high) = (low.min(least), high.max(most));
    }
    Lines::across(low - 1.0, high + 1.0, count)
}

#[test]
fn two_boxes_the_kernel_added_enclose_along_every_line_what_either_did() {
    let a = box_of(10.0, 10.0, DVec3::ZERO);
    let b = box_of(10.0, 10.0, DVec3::new(5.0, 3.0, 2.0));
    let joined = a.union(&b);
    let lines = lines_around(&[&a, &b], 64);
    let (a, b) = (lines.inside(&a.triangles()), lines.inside(&b.triangles()));
    let promised: Vec<Spans> = a.iter().zip(&b).map(|(a, b)| a.union(b)).collect();
    assert_eq!(
        lines.compare(&promised, &lines.inside(&joined.triangles())),
        Ok(())
    );
}

#[test]
fn a_pocket_the_kernel_cut_encloses_along_every_line_what_the_block_kept() {
    let block = box_of(10.0, 10.0, DVec3::ZERO);
    let tool = box_of(4.0, 20.0, DVec3::new(3.0, 3.0, -5.0));
    let cut = block.difference(&tool);
    let lines = lines_around(&[&block, &tool], 64);
    let (block, tool) = (
        lines.inside(&block.triangles()),
        lines.inside(&tool.triangles()),
    );
    let promised: Vec<Spans> = block.iter().zip(&tool).map(|(b, t)| b.without(t)).collect();
    assert_eq!(
        lines.compare(&promised, &lines.inside(&cut.triangles())),
        Ok(())
    );
}

fn turned(outline: &[DVec2], turn: f64) -> Mesh {
    crate::sweep::revolution(
        Loop::straight(outline),
        &[],
        &fan(outline),
        |point| DVec3::new(point.x, point.y, 0.0),
        DVec2::ZERO,
        DVec2::Y,
        turn,
    )
    .expect("a profile on one side of the axis")
}

#[test]
fn a_ring_the_kernel_turned_then_cut_encloses_along_every_line_what_the_ring_kept() {
    let outline = [
        DVec2::new(3.0, 0.0),
        DVec2::new(9.0, 0.0),
        DVec2::new(9.0, 6.0),
        DVec2::new(3.0, 6.0),
    ];
    let ring = turned(&outline, TAU);
    let tool = box_of(3.0, 30.0, DVec3::new(4.0, 1.0, -15.0));
    let cut = ring.difference(&tool);
    let lines = lines_around(&[&ring, &tool], 64);
    let (ring, tool) = (
        lines.inside(&ring.triangles()),
        lines.inside(&tool.triangles()),
    );
    let promised: Vec<Spans> = ring.iter().zip(&tool).map(|(r, t)| r.without(t)).collect();
    assert_eq!(
        lines.compare(&promised, &lines.inside(&cut.triangles())),
        Ok(())
    );
}

#[test]
fn two_boxes_the_kernel_added_face_to_face_enclose_along_every_line_what_either_did() {
    let a = box_of(10.0, 10.0, DVec3::ZERO);
    let b = box_of(10.0, 10.0, DVec3::new(10.0, 4.0, 0.0));
    let joined = a.union(&b);
    let lines = lines_around(&[&a, &b], 64);
    let (a, b) = (lines.inside(&a.triangles()), lines.inside(&b.triangles()));
    let promised: Vec<Spans> = a.iter().zip(&b).map(|(a, b)| a.union(b)).collect();
    assert_eq!(
        lines.compare(&promised, &lines.inside(&joined.triangles())),
        Ok(())
    );
}

#[test]
fn two_cuts_the_kernel_made_with_faces_landing_on_each_other_enclose_what_was_left() {
    let block = box_of(20.0, 10.0, DVec3::ZERO);
    let small = box_of(4.0, 10.0, DVec3::new(2.0, 2.0, 0.0));
    let wide = box_of(8.0, 10.0, DVec3::new(0.0, 8.0, 0.0));
    let twice = block.difference(&small).difference(&wide);
    let lines = lines_around(&[&block, &small, &wide], 64);
    let [block, small, wide] =
        [&block, &small, &wide].map(|solid| lines.inside(&solid.triangles()));
    let promised: Vec<Spans> = (0..block.len())
        .map(|index| block[index].without(&small[index]).without(&wide[index]))
        .collect();
    assert_eq!(
        lines.compare(&promised, &lines.inside(&twice.triangles())),
        Ok(())
    );
}

#[test]
fn a_box_left_whole_inside_the_one_it_was_added_to_is_more_matter_than_promised() {
    let block = box_of(10.0, 10.0, DVec3::ZERO);
    let buried = box_of(2.0, 2.0, DVec3::splat(4.0));
    let both: Vec<Triangle> = [block.triangles(), buried.triangles()].concat();
    let lines = lines_around(&[&block, &buried], 64);
    let (block, buried) = (
        lines.inside(&block.triangles()),
        lines.inside(&buried.triangles()),
    );
    let promised: Vec<Spans> = block.iter().zip(&buried).map(|(b, i)| b.union(i)).collect();

    let Err(Flaw::Volume {
        promised: whole,
        enclosed,
        worst: Some(worst),
    }) = lines.compare(&promised, &lines.inside(&both))
    else {
        panic!("a shell inside another is counted twice, and is no solid");
    };
    assert!(enclosed > whole, "{enclosed} / {whole}");
    assert!(worst.enclosed > worst.promised, "{worst:?}");
}

#[test]
fn lines_count_a_shell_left_inside_another_twice_as_enclosed_does() {
    let both: Vec<Triangle> = [
        box_of(10.0, 10.0, DVec3::ZERO).triangles(),
        box_of(2.0, 2.0, DVec3::splat(4.0)).triangles(),
    ]
    .concat();
    let lines = Lines::across(DVec3::splat(-1.0), DVec3::splat(11.0), 64);
    let measured = lines.volume(&lines.inside(&both));
    assert!((enclosed(&both) - 1008.0).abs() < 1e-9);
    assert!((measured - 1008.0).abs() < 10.0, "{measured}");
}

#[test]
fn a_box_the_kernel_added_inside_another_encloses_along_every_line_what_the_outer_did() {
    let block = box_of(10.0, 10.0, DVec3::ZERO);
    let buried = box_of(2.0, 2.0, DVec3::splat(4.0));
    let joined = block.union(&buried);
    let lines = lines_around(&[&block, &buried], 64);
    let (block, buried) = (
        lines.inside(&block.triangles()),
        lines.inside(&buried.triangles()),
    );
    let promised: Vec<Spans> = block.iter().zip(&buried).map(|(b, i)| b.union(i)).collect();
    assert_eq!(
        lines.compare(&promised, &lines.inside(&joined.triangles())),
        Ok(())
    );
}

#[test]
fn a_tool_left_inside_out_apart_from_the_block_it_cut_is_less_matter_than_promised() {
    let block = box_of(10.0, 10.0, DVec3::ZERO);
    let tool = box_of(2.0, 2.0, DVec3::new(20.0, 4.0, 4.0));
    let both: Vec<Triangle> = [block.triangles(), inside_out(&tool.triangles())].concat();
    let lines = lines_around(&[&block, &tool], 64);
    let (block, tool) = (
        lines.inside(&block.triangles()),
        lines.inside(&tool.triangles()),
    );
    let promised: Vec<Spans> = block.iter().zip(&tool).map(|(b, t)| b.without(t)).collect();

    let Err(Flaw::Volume {
        promised: whole,
        enclosed,
        worst: Some(worst),
    }) = lines.compare(&promised, &lines.inside(&both))
    else {
        panic!("a shell turned inside out on its own is no hole in anything");
    };
    assert!(enclosed < whole, "{enclosed} / {whole}");
    assert!(worst.enclosed < worst.promised, "{worst:?}");
}

#[test]
fn twenty_thousand_facets_are_measured_along_sixty_thousand_lines_within_a_second() {
    let outline: Vec<DVec2> = (0..160)
        .map(|step| DVec2::new(5.0, 0.0) + 2.0 * DVec2::from_angle(TAU * step as f64 / 160.0))
        .collect();
    let torus = turned(&outline, TAU).triangles();
    assert!(torus.len() >= 20_000, "{}", torus.len());
    let lines = Lines::across(DVec3::new(-7.0, -2.0, -7.0), DVec3::new(7.0, 2.0, 7.0), 256);

    let began = std::time::Instant::now();
    let measured = lines.volume(&lines.inside(&torus));
    let took = began.elapsed();

    let expected = enclosed(&torus);
    assert!(
        (measured - expected).abs() / expected < 0.01,
        "{measured} / {expected}"
    );
    assert!(took.as_secs_f64() < 1.0, "{took:?}");
}

/// A round bar lying along `y`, its sides as long as the bar: every one of its
/// walls a sliver slanting across the whole bundle of lines.
fn rod(sides: usize, radius: f64, length: f64) -> Vec<Triangle> {
    let ring: Vec<DVec3> = (0..sides)
        .map(|step| {
            let (sine, cosine) = (TAU * step as f64 / sides as f64).sin_cos();
            DVec3::new(radius * cosine, 0.0, radius * sine)
        })
        .collect();
    let along = DVec3::new(0.0, length, 0.0);
    let mut triangles = Vec::new();
    for index in 0..sides {
        let (a, b) = (ring[index], ring[(index + 1) % sides]);
        triangles.push([a, a + along, b + along]);
        triangles.push([a, b + along, b]);
    }
    for index in 1..sides - 1 {
        let (b, c) = (ring[index], ring[index + 1]);
        triangles.push([ring[0], b, c]);
        triangles.push([ring[0] + along, c + along, b + along]);
    }
    triangles
}

#[test]
fn twenty_thousand_long_facets_slanting_across_the_lines_are_measured_within_a_second() {
    let bar = rod(5000, 1.0, 100.0);
    assert!(bar.len() >= 19_990, "{}", bar.len());
    let lines = Lines::across(
        DVec3::new(-2.0, -1.0, -2.0),
        DVec3::new(2.0, 101.0, 2.0),
        256,
    );

    let began = std::time::Instant::now();
    let measured = lines.volume(&lines.inside(&bar));
    let took = began.elapsed();

    let expected = enclosed(&bar);
    assert!(
        (measured - expected).abs() / expected < 0.01,
        "{measured} / {expected}"
    );
    assert!(took.as_secs_f64() < 1.0, "{took:?}");
}

#[test]
fn a_face_is_tried_against_every_line_it_covers_however_it_lies_on_the_grid() {
    let lines = Lines::across(DVec3::ZERO, DVec3::ONE, 16);
    let mut random = crate::soundness::Random::seeded(448);
    let lifted = |flat: DVec2, depth: f64| {
        lines.across * flat.x + lines.up * flat.y + lines.direction * depth
    };
    let mut tried = 0;
    while tried < 20_000 {
        let mut corner = || {
            let a = lines.place(random.below(256));
            let b = lines.place(random.below(256));
            let flat = match random.below(4) {
                0 => a,
                1 => a.lerp(b, random.unit()),
                2 => DVec2::new(a.x, b.y),
                _ => lines.low + DVec2::new(random.unit(), random.unit()) * lines.cell * 16.0,
            };
            lifted(flat, random.between(-1.0, 1.0))
        };
        let triangle = [corner(), corner(), corner()];
        let mut flat = triangle.map(|corner| lines.flat(corner));
        let area = (flat[1] - flat[0]).perp_dot(flat[2] - flat[0]);
        if area.abs() < 1e-9 {
            continue;
        }
        if area < 0.0 {
            flat.swap(1, 2);
        }
        tried += 1;

        let mut crossings = vec![Vec::new(); 256];
        lines.meet(&triangle, &mut crossings);
        for (index, found) in crossings.iter().enumerate() {
            let covers = covered(flat, lines.place(index)).is_some();
            assert_eq!(!found.is_empty(), covers, "{triangle:?} at line {index}");
        }
    }
}

/// Where the line through `origin` along `direction` enters and leaves the
/// box between two corners, by the slabs of its three axes.
fn through_box(origin: DVec3, direction: DVec3, low: DVec3, high: DVec3) -> Option<(f64, f64)> {
    let (mut enter, mut leave) = (f64::NEG_INFINITY, f64::INFINITY);
    for axis in 0..3 {
        let [one, other] =
            [low[axis], high[axis]].map(|side| (side - origin[axis]) / direction[axis]);
        enter = enter.max(one.min(other));
        leave = leave.min(one.max(other));
    }
    (enter < leave).then_some((enter, leave))
}

#[test]
fn a_line_of_measure_found_by_its_rank_crosses_a_box_where_its_spans_say() {
    let (low, high) = (DVec3::new(-1.0, 0.5, 2.0), DVec3::new(3.0, 2.0, 4.5));
    let lines = Lines::across(low - 0.5, high + 0.5, 24);
    let spans = lines.inside(&cube(low, high));
    assert_eq!(lines.count(), spans.len());
    let mut crossing = 0;
    for (index, measured) in spans.iter().enumerate() {
        let (origin, direction) = lines.line(index);
        assert!((direction.length() - 1.0).abs() < 1e-12);
        match through_box(origin, direction, low, high) {
            Some((enter, leave)) => {
                crossing += 1;
                let [(from, to)] = measured.stretches() else {
                    panic!("line {index} crosses the box once: {measured:?}");
                };
                assert!(
                    (from - enter).abs() < 1e-12,
                    "line {index}: {from} against {enter}"
                );
                assert!(
                    (to - leave).abs() < 1e-12,
                    "line {index}: {to} against {leave}"
                );
            }
            None => assert!(measured.stretches().is_empty(), "line {index}"),
        }
    }
    assert!(crossing > 100, "{crossing} lines cross the box");
}

#[test]
fn spans_gathered_from_stretches_in_any_order_come_sorted_and_joined() {
    let gathered = Spans::gathered(vec![(3.0, 4.0), (0.0, 1.0), (0.5, 2.0), (5.0, 5.0)]);
    assert_eq!(gathered.stretches(), [(0.0, 2.0), (3.0, 4.0)]);
    assert_eq!(gathered.surplus(), 0.0);
}

#[test]
fn a_shell_left_inside_another_shows_as_surplus_along_the_lines_through_it() {
    let both: Vec<Triangle> = [
        box_of(10.0, 10.0, DVec3::ZERO).triangles(),
        box_of(2.0, 2.0, DVec3::splat(4.0)).triangles(),
    ]
    .concat();
    let lines = Lines::across(DVec3::splat(-1.0), DVec3::splat(11.0), 64);
    let surplus: f64 = lines.inside(&both).iter().map(Spans::surplus).sum();
    assert!(surplus > 0.0, "{surplus}");
}
