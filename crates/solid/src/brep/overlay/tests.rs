use std::f64::consts::{PI, TAU as TURN};

use glam::DVec2;

use super::*;
use crate::brep::Declined;

fn segment(from: [f64; 2], to: [f64; 2], ends: Option<[usize; 2]>) -> Arc {
    Arc {
        trace: Trace::Segment {
            from: DVec2::from(from),
            to: DVec2::from(to),
        },
        ends,
    }
}

fn square(low: [f64; 2], high: [f64; 2], first: usize) -> (Vec<DVec2>, Vec<Arc>) {
    let corners = [
        [low[0], low[1]],
        [high[0], low[1]],
        [high[0], high[1]],
        [low[0], high[1]],
    ];
    let arcs = (0..4)
        .map(|side| {
            let next = (side + 1) % 4;
            segment(
                corners[side],
                corners[next],
                Some([first + side, first + next]),
            )
        })
        .collect();
    (corners.into_iter().map(DVec2::from).collect(), arcs)
}

#[test]
fn the_corners_of_a_square_trace_its_inside_one_way_and_its_outside_the_other() {
    let (_, arcs) = square([0.0, 0.0], [10.0, 10.0], 0);
    let cycles = star::cycles(&arcs).expect("a square is ordered");
    assert_eq!(
        cycles.list,
        vec![
            vec![(0, true), (1, true), (2, true), (3, true)],
            vec![(0, false), (3, false), (2, false), (1, false)],
        ]
    );
}

fn round(center: [f64; 2], radius: f64, start: f64, sweep: f64, ends: Option<[usize; 2]>) -> Arc {
    Arc {
        trace: Trace::Round {
            center: DVec2::from(center),
            radius,
            start,
            sweep,
        },
        ends,
    }
}

fn tangent_inside() -> (Vec<DVec2>, Vec<Arc>) {
    let bottom = -std::f64::consts::FRAC_PI_2;
    let turn = std::f64::consts::TAU;
    (
        vec![DVec2::ZERO],
        vec![
            round([0.0, 10.0], 10.0, bottom, turn, Some([0, 0])),
            round([0.0, 5.0], 5.0, bottom, turn, Some([0, 0])),
        ],
    )
}

#[test]
fn a_disc_tangent_inside_a_disc_at_a_corner_is_told_apart_by_how_it_bends() {
    let (_, arcs) = tangent_inside();
    let cycles = star::cycles(&arcs).expect("the two circles bend apart");
    assert_eq!(
        cycles.list,
        vec![
            vec![(0, true), (1, false)],
            vec![(0, false)],
            vec![(1, true)]
        ]
    );
}

#[test]
fn two_arcs_leaving_a_corner_along_one_another_are_declined() {
    let arcs = [
        segment([0.0, 0.0], [4.0, 0.0], Some([0, 1])),
        segment([0.0, 0.0], [9.0, 0.0], Some([0, 2])),
    ];
    assert!(matches!(star::cycles(&arcs), Err(Declined::Tie)));
}

const SAMPLES: usize = 400;

/// How far every region's point must stand from every arc.
const CLEARANCE: f64 = 0.05;

fn sampled(arc: &Arc, forward: bool) -> Vec<DVec2> {
    (0..=SAMPLES)
        .map(|step| {
            let along = step as f64 / SAMPLES as f64;
            arc.trace.at(if forward { along } else { 1.0 - along })[0]
        })
        .collect()
}

/// A cycle as one closed line, each arc moved by whole turns to start where
/// the one before it ended, and the line brought back to its start, a turn
/// along for a cycle going round.
fn unwrapped(arcs: &[Arc], cycle: &[(usize, bool)], period: Option<f64>) -> Vec<DVec2> {
    let whole = |from: DVec2, to: DVec2| match period {
        Some(period) => ((to.x - from.x) / period).round() * period,
        None => 0.0,
    };
    let mut points: Vec<DVec2> = Vec::new();
    for &(arc, forward) in cycle {
        let mut run = sampled(&arcs[arc], forward);
        if let Some(&last) = points.last() {
            let shift = whole(run[0], last);
            for point in &mut run {
                point.x += shift;
            }
        }
        points.extend(run);
    }
    let first = points[0];
    let shift = whole(first, points[points.len() - 1]);
    points.push(first + DVec2::new(shift, 0.0));
    points
}

/// The crossings of a vertical line through `point` below it, each counted
/// by the way the line crossed runs: the winding number of a closed cycle
/// round the point, and on a cylinder whether it stands above a cycle going
/// round.
fn crossed_below(points: &[DVec2], point: DVec2, period: Option<f64>) -> i32 {
    let turns = match period {
        Some(period) => (-3..=3).map(|turn| f64::from(turn) * period).collect(),
        None => vec![0.0],
    };
    turns
        .into_iter()
        .map(|shift| {
            let x = point.x + shift;
            points
                .windows(2)
                .map(|pair| {
                    let (start, end) = (pair[0], pair[1]);
                    let (low, high) = (start.x.min(end.x), start.x.max(end.x));
                    if x < low || x >= high {
                        return 0;
                    }
                    let y = start.y + (end.y - start.y) * (x - start.x) / (end.x - start.x);
                    match (y < point.y, end.x > start.x) {
                        (false, _) => 0,
                        (true, true) => 1,
                        (true, false) => -1,
                    }
                })
                .sum::<i32>()
        })
        .sum()
}

/// Whether `point` lies in the region these cycles bound, read off the
/// sampled arcs alone: on the left of each cycle, which is inside a cycle
/// turning counterclockwise, outside one turning clockwise, above a cycle
/// going round forward and below one going round backward.
fn located_in(
    arcs: &[Arc],
    cycles: &[Vec<(usize, bool)>],
    period: Option<f64>,
    point: DVec2,
) -> bool {
    cycles.iter().all(|cycle| {
        let line = unwrapped(arcs, cycle, period);
        let shift = line[line.len() - 1].x - line[0].x;
        let area: f64 = line
            .windows(2)
            .map(|pair| pair[0].perp_dot(pair[1]))
            .sum::<f64>()
            / 2.0;
        let turn = period.unwrap_or(f64::INFINITY);
        let expected = if shift.abs() > turn / 2.0 {
            i32::from(shift > 0.0)
        } else {
            i32::from(area > 1e-9)
        };
        crossed_below(&line, point, period) == expected
    })
}

fn clearance(arcs: &[Arc], period: Option<f64>, point: DVec2) -> f64 {
    arcs.iter()
        .flat_map(|arc| sampled(arc, true))
        .map(|sample| {
            let mut apart = point - sample;
            if let Some(period) = period {
                apart.x -= (apart.x / period).round() * period;
            }
            apart.length()
        })
        .fold(f64::INFINITY, f64::min)
}

#[derive(Default)]
struct Drawing {
    vertices: Vec<DVec2>,
    arcs: Vec<Arc>,
}

impl Drawing {
    fn vertex(&mut self, at: [f64; 2]) -> usize {
        self.vertices.push(DVec2::from(at));
        self.vertices.len() - 1
    }

    fn arc(&mut self, trace: Trace, ends: Option<[usize; 2]>) -> usize {
        self.arcs.push(Arc { trace, ends });
        self.arcs.len() - 1
    }

    /// A straight arc from a vertex to another reached at `at`, which on a
    /// cylinder may lie a turn away from where the vertex is given.
    fn line_to(&mut self, from: usize, to: usize, at: [f64; 2]) -> usize {
        let trace = Trace::Segment {
            from: self.vertices[from],
            to: DVec2::from(at),
        };
        self.arc(trace, Some([from, to]))
    }

    /// A closed straight arc with no vertex, all the way round a cylinder at
    /// height `height`, from the angle `start`.
    fn round_the_cylinder(&mut self, height: f64, start: f64) -> usize {
        let trace = Trace::Segment {
            from: DVec2::new(start, height),
            to: DVec2::new(start + TURN, height),
        };
        self.arc(trace, None)
    }

    fn line(&mut self, from: usize, to: usize) -> usize {
        let trace = Trace::Segment {
            from: self.vertices[from],
            to: self.vertices[to],
        };
        self.arc(trace, Some([from, to]))
    }

    /// A closed path through new corners, each joined to the next by a
    /// straight arc.
    fn polygon(&mut self, corners: &[[f64; 2]]) -> Vec<usize> {
        let ranks: Vec<usize> = corners.iter().map(|&corner| self.vertex(corner)).collect();
        for side in 0..ranks.len() {
            self.line(ranks[side], ranks[(side + 1) % ranks.len()]);
        }
        ranks
    }

    fn square(&mut self, low: [f64; 2], high: [f64; 2]) -> Vec<usize> {
        self.polygon(&[low, [high[0], low[1]], high, [low[0], high[1]]])
    }

    /// A whole circle with no vertex, run counterclockwise from `start`.
    fn circle(&mut self, center: [f64; 2], radius: f64, start: f64) -> usize {
        let trace = Trace::Round {
            center: DVec2::from(center),
            radius,
            start,
            sweep: std::f64::consts::TAU,
        };
        self.arc(trace, None)
    }

    /// A round about `center` from one vertex to another, or all the way
    /// round from a vertex back to it.
    fn round(&mut self, center: [f64; 2], from: usize, to: usize, counterclockwise: bool) -> usize {
        let center = DVec2::from(center);
        let [start, end] = [from, to].map(|vertex| self.vertices[vertex] - center);
        let angle = start.y.atan2(start.x);
        let turn = std::f64::consts::TAU;
        let apart = end.y.atan2(end.x) - angle;
        let way = if counterclockwise { 1.0 } else { -1.0 };
        let sweep = match (way * apart).rem_euclid(turn) {
            0.0 => way * turn,
            ahead => way * ahead,
        };
        let trace = Trace::Round {
            center,
            radius: start.length(),
            start: angle,
            sweep,
        };
        self.arc(trace, Some([from, to]))
    }

    /// The overlay, held to the sampled arcs: every arc used once each way,
    /// every region's point inside that region alone and clear of every arc.
    fn regions(&self, period: Option<f64>) -> Vec<Region> {
        let overlay = Overlay::of(&self.vertices, &self.arcs, period).expect("the overlay is made");
        let mut uses = vec![0; 2 * self.arcs.len()];
        for region in &overlay.regions {
            for &(arc, forward) in region.cycles.iter().flatten() {
                uses[2 * arc + usize::from(!forward)] += 1;
            }
        }
        assert!(
            uses.iter().all(|&count| count == 1),
            "each arc once each way: {uses:?}"
        );
        for (rank, region) in overlay.regions.iter().enumerate() {
            let point = region.inside;
            for (other, cycles) in overlay.regions.iter().enumerate() {
                assert_eq!(
                    located_in(&self.arcs, &cycles.cycles, period, point),
                    other == rank,
                    "the point {point} of region {rank} against region {other}: {:?}",
                    overlay.regions
                );
            }
            let apart = clearance(&self.arcs, period, point);
            assert!(
                apart > CLEARANCE,
                "the point {point} of region {rank} stands {apart} from an arc"
            );
        }
        overlay.regions
    }
}

fn bounded(regions: &[Region]) -> usize {
    regions.iter().filter(|region| !region.unbounded).count()
}

#[test]
fn a_square_with_a_square_hole_is_a_ring_and_the_outside() {
    let mut drawing = Drawing::default();
    drawing.square([0.0, 0.0], [10.0, 10.0]);
    drawing.square([3.0, 4.0], [6.0, 7.0]);
    let regions = drawing.regions(None);
    assert_eq!(regions.len(), 3);
    assert_eq!(bounded(&regions), 2);
    let ring = regions
        .iter()
        .find(|region| region.cycles.len() == 2)
        .expect("the ring has two cycles");
    assert!(!ring.unbounded);
}

#[test]
fn a_square_cut_by_a_segment_has_two_halves() {
    let mut drawing = Drawing::default();
    let corners = drawing.polygon(&[
        [0.0, 0.0],
        [5.0, 0.0],
        [10.0, 0.0],
        [10.0, 10.0],
        [5.0, 10.0],
        [0.0, 10.0],
    ]);
    drawing.line(corners[1], corners[4]);
    let regions = drawing.regions(None);
    assert_eq!(regions.len(), 3);
    assert_eq!(bounded(&regions), 2);
}

#[test]
fn an_isolated_closed_circle_parts_the_plane_in_two() {
    let mut drawing = Drawing::default();
    drawing.circle([1.0, -2.0], 3.0, 0.3);
    let regions = drawing.regions(None);
    assert_eq!(regions.len(), 2);
    assert_eq!(bounded(&regions), 1);
}

#[test]
fn a_disc_tangent_inside_a_disc_leaves_a_crescent_between_them() {
    let mut drawing = Drawing::default();
    let touch = drawing.vertex([0.0, 0.0]);
    drawing.round([0.0, 10.0], touch, touch, true);
    drawing.round([0.0, 5.0], touch, touch, true);
    let regions = drawing.regions(None);
    assert_eq!(regions.len(), 3);
    assert_eq!(bounded(&regions), 2);
}

#[test]
fn two_discs_tangent_outside_share_the_outside() {
    let mut drawing = Drawing::default();
    let touch = drawing.vertex([0.0, 0.0]);
    drawing.round([0.0, 10.0], touch, touch, true);
    drawing.round([0.0, -5.0], touch, touch, false);
    let regions = drawing.regions(None);
    assert_eq!(regions.len(), 3);
    assert_eq!(bounded(&regions), 2);
    let outside = regions
        .iter()
        .find(|region| region.unbounded)
        .expect("one region is outside");
    assert_eq!(outside.cycles.len(), 1);
    assert_eq!(outside.cycles[0].len(), 2);
}

#[test]
fn two_crossing_circles_make_a_lens_and_two_crescents() {
    let mut drawing = Drawing::default();
    let upper = drawing.vertex([3.0, 4.0]);
    let lower = drawing.vertex([3.0, -4.0]);
    drawing.round([0.0, 0.0], lower, upper, true);
    drawing.round([0.0, 0.0], upper, lower, true);
    drawing.round([6.0, 0.0], upper, lower, true);
    drawing.round([6.0, 0.0], lower, upper, true);
    let regions = drawing.regions(None);
    assert_eq!(regions.len(), 4);
    assert_eq!(bounded(&regions), 3);
}

#[test]
fn slits_inside_a_square_leave_it_one_region() {
    let mut drawing = Drawing::default();
    let corners = drawing.polygon(&[
        [0.0, 0.0],
        [3.0, 0.0],
        [10.0, 0.0],
        [10.0, 10.0],
        [0.0, 10.0],
    ]);
    let tip = drawing.vertex([3.0, 3.0]);
    drawing.line(corners[1], tip);
    let [left, right] = [[2.0, 6.0], [6.0, 6.0]].map(|at| drawing.vertex(at));
    drawing.line(left, right);
    let [low, high] = [[8.0, 2.0], [8.0, 7.0]].map(|at| drawing.vertex(at));
    drawing.line(low, high);
    let regions = drawing.regions(None);
    assert_eq!(regions.len(), 2);
    let inside = regions
        .iter()
        .find(|region| !region.unbounded)
        .expect("the square's inside is bounded");
    assert_eq!(inside.cycles.len(), 3);
}

#[test]
fn nested_holes_alternate_with_the_rings_between_them() {
    let mut drawing = Drawing::default();
    for inset in 0..4 {
        let inset = f64::from(inset);
        drawing.square([inset, inset], [10.0 - inset, 10.0 - inset]);
    }
    let regions = drawing.regions(None);
    assert_eq!(regions.len(), 5);
    assert_eq!(bounded(&regions), 4);
    let rings = regions
        .iter()
        .filter(|region| region.cycles.len() == 2)
        .count();
    assert_eq!(rings, 3);
}

const ROUND: Option<f64> = Some(TURN);

#[test]
fn two_circles_round_a_cylinder_bound_a_band_between_two_unbounded_ends() {
    let mut drawing = Drawing::default();
    drawing.round_the_cylinder(0.0, -PI);
    drawing.round_the_cylinder(5.0, -PI);
    let regions = drawing.regions(ROUND);
    assert_eq!(regions.len(), 3);
    assert_eq!(bounded(&regions), 1);
    let band = regions
        .iter()
        .find(|region| !region.unbounded)
        .expect("the band is bounded");
    assert_eq!(band.cycles, vec![vec![(0, true)], vec![(1, false)]]);
}

#[test]
fn a_ruling_across_the_band_opens_it_into_one_disc() {
    let mut drawing = Drawing::default();
    let low = drawing.vertex([0.0, 0.0]);
    let high = drawing.vertex([0.0, 5.0]);
    drawing.line_to(low, low, [TURN, 0.0]);
    drawing.line_to(high, high, [TURN, 5.0]);
    drawing.line(low, high);
    let regions = drawing.regions(ROUND);
    assert_eq!(regions.len(), 3);
    let band = regions
        .iter()
        .find(|region| !region.unbounded)
        .expect("the band is bounded");
    assert_eq!(band.cycles.len(), 1);
    assert_eq!(band.cycles[0].len(), 4);
}

#[test]
fn a_hole_in_the_band_is_one_more_cycle_of_it() {
    let mut drawing = Drawing::default();
    drawing.round_the_cylinder(0.0, 1.0);
    drawing.round_the_cylinder(5.0, 1.0);
    drawing.square([1.0, 2.0], [2.0, 3.0]);
    let regions = drawing.regions(ROUND);
    assert_eq!(regions.len(), 4);
    assert_eq!(bounded(&regions), 2);
    let band = regions
        .iter()
        .find(|region| region.cycles.len() == 3)
        .expect("the band is bounded by both circles and the hole");
    assert!(!band.unbounded);
}

#[test]
fn a_ruling_alone_in_a_band_leaves_the_band_whole() {
    let mut drawing = Drawing::default();
    drawing.round_the_cylinder(0.0, 0.5);
    drawing.round_the_cylinder(5.0, 0.5);
    let [low, high] = [[-2.0, 1.0], [-2.0, 4.0]].map(|at| drawing.vertex(at));
    drawing.line(low, high);
    let regions = drawing.regions(ROUND);
    assert_eq!(regions.len(), 3);
    let band = regions
        .iter()
        .find(|region| !region.unbounded)
        .expect("the band is bounded");
    assert_eq!(band.cycles.len(), 3);
}

#[test]
fn arcs_crossing_the_seam_of_the_angle_are_read_modulo_the_turn() {
    let mut drawing = Drawing::default();
    drawing.round_the_cylinder(0.0, 2.5);
    drawing.round_the_cylinder(5.0, 2.5);
    let corners =
        [[2.8, 1.0], [3.6 - TURN, 1.0], [3.6 - TURN, 2.0], [2.8, 2.0]].map(|at| drawing.vertex(at));
    drawing.line_to(corners[0], corners[1], [3.6, 1.0]);
    drawing.line(corners[1], corners[2]);
    drawing.line_to(corners[2], corners[3], [2.8 - TURN, 2.0]);
    drawing.line(corners[3], corners[0]);
    drawing.circle([PI, 3.5], 0.4, 0.0);
    let regions = drawing.regions(ROUND);
    assert_eq!(regions.len(), 5);
    assert_eq!(bounded(&regions), 3);
    let band = regions
        .iter()
        .find(|region| region.cycles.len() == 4)
        .expect("the band is bounded by both circles and both holes");
    assert!(!band.unbounded);
}

#[test]
fn no_arc_leaves_the_whole_surface_one_unbounded_region() {
    for period in [None, ROUND] {
        let overlay = Overlay::of(&[], &[], period).expect("nothing to cut");
        assert_eq!(overlay.regions.len(), 1);
        assert!(overlay.regions[0].unbounded);
        assert!(overlay.regions[0].cycles.is_empty());
    }
}

fn cycle_counts(regions: &[Region]) -> Vec<usize> {
    let mut counts: Vec<usize> = regions.iter().map(|region| region.cycles.len()).collect();
    counts.sort_unstable();
    counts
}

#[test]
fn a_circle_resting_on_a_side_at_a_shared_corner_pinches_the_square_without_parting_it() {
    let mut drawing = Drawing::default();
    let corners = drawing.polygon(&[
        [0.0, 0.0],
        [5.0, 0.0],
        [10.0, 0.0],
        [10.0, 10.0],
        [0.0, 10.0],
    ]);
    drawing.round([5.0, 3.0], corners[1], corners[1], true);
    let regions = drawing.regions(None);
    assert_eq!(cycle_counts(&regions), vec![1, 1, 1]);
    let pinched = regions
        .iter()
        .find(|region| !region.unbounded && region.cycles[0].len() == 6)
        .expect("the square less the disc runs round the square and round the disc");
    assert!(pinched.cycles[0].contains(&(5, false)));
}

#[test]
fn a_hair_of_side_leaving_a_tangent_circle_is_told_apart_by_how_it_bends_whatever_the_rounding() {
    for hair in [1e-7, 1e-5] {
        for rounding in [4e-15, -4e-15] {
            let mut drawing = Drawing::default();
            let corners = drawing.polygon(&[
                [0.0, 0.0],
                [5.0, 0.0],
                [5.0 + hair, rounding],
                [10.0, 0.0],
                [10.0, 10.0],
                [0.0, 10.0],
            ]);
            drawing.round([5.0, 3.0], corners[1], corners[1], true);
            let regions = drawing.regions(None);
            assert_eq!(
                cycle_counts(&regions),
                vec![1, 1, 1],
                "a hair of {hair} risen by {rounding}"
            );
        }
    }
}

#[test]
fn two_holes_whose_circles_touch_leave_the_square_one_region() {
    let mut drawing = Drawing::default();
    drawing.square([-10.0, -10.0], [10.0, 10.0]);
    let touch = drawing.vertex([0.0, 1.0]);
    drawing.round([-3.0, 1.0], touch, touch, false);
    drawing.round([2.0, 1.0], touch, touch, true);
    let regions = drawing.regions(None);
    assert_eq!(cycle_counts(&regions), vec![1, 1, 1, 2]);
}

#[test]
fn a_disc_touching_a_circle_round_the_cylinder_from_below_hangs_in_the_band() {
    let mut drawing = Drawing::default();
    let touch = drawing.vertex([1.0, 5.0]);
    drawing.round_the_cylinder(0.0, -PI);
    drawing.line_to(touch, touch, [1.0 + TURN, 5.0]);
    drawing.round([1.0, 3.5], touch, touch, true);
    let regions = drawing.regions(ROUND);
    assert_eq!(regions.len(), 4);
    assert_eq!(bounded(&regions), 2);
}

#[test]
fn an_upright_slit_right_under_or_over_a_corner_lies_in_the_region_beside_that_corner() {
    let mut drawing = Drawing::default();
    drawing.square([0.0, 0.0], [10.0, 10.0]);
    drawing.square([2.0, 2.0], [4.0, 4.0]);
    for [low, high] in [
        [[0.0, -5.0], [0.0, -2.0]],
        [[10.0, 12.0], [10.0, 15.0]],
        [[2.0, 5.0], [2.0, 8.0]],
    ] {
        let [low, high] = [low, high].map(|at| drawing.vertex(at));
        drawing.line(low, high);
    }
    let regions = drawing.regions(None);
    assert_eq!(cycle_counts(&regions), vec![1, 3, 3]);
}

#[test]
fn an_upright_slit_at_the_side_of_a_circle_lies_beside_it() {
    let mut drawing = Drawing::default();
    drawing.square([-10.0, -10.0], [20.0, 20.0]);
    drawing.circle([10.0, 5.0], 5.0, 1.0);
    for [low, high] in [
        [[5.0, 1.0], [5.0, 3.0]],
        [[5.0, 7.0], [5.0, 9.0]],
        [[15.0, 6.0], [15.0, 8.0]],
    ] {
        let [low, high] = [low, high].map(|at| drawing.vertex(at));
        drawing.line(low, high);
    }
    let regions = drawing.regions(None);
    assert_eq!(cycle_counts(&regions), vec![1, 1, 5]);
}

#[test]
fn a_ruling_on_the_seam_where_the_circles_start_lies_in_the_band() {
    let mut drawing = Drawing::default();
    drawing.round_the_cylinder(0.0, -PI);
    drawing.round_the_cylinder(5.0, -PI);
    let [low, high] = [[PI, 1.0], [PI, 4.0]].map(|at| drawing.vertex(at));
    drawing.line(low, high);
    let regions = drawing.regions(ROUND);
    assert_eq!(cycle_counts(&regions), vec![1, 1, 3]);
    let band = regions
        .iter()
        .find(|region| region.cycles.len() == 3)
        .expect("the band holds the ruling");
    assert!(!band.unbounded);
}

#[test]
fn a_ruling_written_turns_away_from_where_the_cylinder_reads_it_lies_in_the_square_round_it() {
    for turns in [-2.0, 2.0, 3.0] {
        let mut drawing = Drawing::default();
        drawing.round_the_cylinder(0.0, -PI);
        drawing.round_the_cylinder(5.0, -PI);
        drawing.square([2.5, 0.5], [3.5, 4.5]);
        drawing.square([-2.0, 1.0], [-1.0, 4.0]);
        let [low, high] = [[3.0, 1.0], [3.0, 4.0]].map(|at| drawing.vertex(at));
        let x = 3.0 + turns * TURN;
        drawing.arc(
            Trace::Segment {
                from: DVec2::new(x, 1.0),
                to: DVec2::new(x, 4.0),
            },
            Some([low, high]),
        );
        let regions = drawing.regions(ROUND);
        assert_eq!(cycle_counts(&regions), vec![1, 1, 1, 2, 4], "{turns} turns");
    }
}

#[test]
fn the_turns_of_a_round_are_found_again_from_its_derivatives_alone() {
    for (start, sweep) in [(0.3, 5.0), (-2.0, -4.5), (1.0, TURN), (PI, 1.0), (0.1, 0.2)] {
        let trace = Trace::Round {
            center: DVec2::new(3.0, -1.0),
            radius: 2.0,
            start,
            sweep,
        };
        let exact = piece::turns(&trace);
        let sampled = piece::sampled_turns(&trace);
        assert_eq!(exact.len(), sampled.len(), "{start} {sweep}");
        for (exact, sampled) in exact.iter().zip(&sampled) {
            assert!((exact - sampled).abs() < 1e-12, "{exact} {sampled}");
        }
    }
}

/// Numbers in a fixed sequence, so that a failing drawing comes back from
/// its seed.
struct Draws(u64);

impl Draws {
    fn next(&mut self) -> u64 {
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 7;
        self.0 ^= self.0 << 17;
        self.0
    }

    fn below(&mut self, count: usize) -> usize {
        (self.next() % count as u64) as usize
    }

    fn pick<T: Copy>(&mut self, among: &[T]) -> T {
        among[self.below(among.len())]
    }

    /// A multiple of `step` from `low` to `high`: on a coarse grid, where
    /// the abscissae of different shapes meet exactly and often.
    fn on_grid(&mut self, low: f64, high: f64, step: f64) -> f64 {
        let steps = ((high - low) / step).round() as usize;
        low + step * self.below(steps + 1) as f64
    }
}

#[derive(Clone, Copy, Debug)]
struct Frame {
    low: DVec2,
    high: DVec2,
}

impl Frame {
    fn shifted(self, shift: f64) -> Frame {
        let by = DVec2::new(shift, 0.0);
        Frame {
            low: self.low + by,
            high: self.high + by,
        }
    }

    fn apart(self, other: Frame) -> bool {
        self.high.x < other.low.x
            || other.high.x < self.low.x
            || self.high.y < other.low.y
            || other.high.y < self.low.y
    }

    fn within(self, outer: Frame) -> bool {
        outer.low.x < self.low.x
            && self.high.x < outer.high.x
            && outer.low.y < self.low.y
            && self.high.y < outer.high.y
    }
}

/// Where a shape stands, and the box inside it another shape may stand in
/// without meeting it.
struct Footprint {
    outer: Frame,
    inner: Option<Frame>,
}

impl Footprint {
    fn clear_of(&self, other: &Footprint, period: Option<f64>) -> bool {
        let shifts = match period {
            Some(period) => vec![-period, 0.0, period],
            None => vec![0.0],
        };
        shifts.into_iter().all(|shift| {
            let outer = other.outer.shifted(shift);
            let inner = other.inner.map(|inner| inner.shifted(shift));
            self.outer.apart(outer)
                || inner.is_some_and(|inner| self.outer.within(inner))
                || self.inner.is_some_and(|mine| outer.within(mine))
        })
    }
}

impl Drawing {
    /// A vertex given where a cylinder reads it, within the turn about
    /// nought, whatever turn `at` is written in.
    fn corner(&mut self, at: DVec2, period: Option<f64>) -> usize {
        let x = period.map_or(at.x, |period| at.x - (at.x / period).round() * period);
        self.vertex([x, at.y])
    }

    fn straight(&mut self, from: (usize, DVec2), to: (usize, DVec2)) -> usize {
        self.arc(
            Trace::Segment {
                from: from.1,
                to: to.1,
            },
            Some([from.0, to.0]),
        )
    }

    /// A closed path through corners, each joined to the next.
    fn path(&mut self, corners: &[DVec2], period: Option<f64>) -> Vec<usize> {
        let ranks: Vec<usize> = corners
            .iter()
            .map(|&corner| self.corner(corner, period))
            .collect();
        for side in 0..corners.len() {
            let next = (side + 1) % corners.len();
            self.straight((ranks[side], corners[side]), (ranks[next], corners[next]));
        }
        ranks
    }
}

/// Places a random shape clear of those already placed, and says how many
/// bounded regions it adds.
fn place(
    drawing: &mut Drawing,
    draws: &mut Draws,
    period: Option<f64>,
    placed: &mut Vec<Footprint>,
) -> usize {
    let [left, right] = match period {
        Some(_) => [-PI - 1.0, PI],
        None => [-6.0, 6.0],
    };
    for _ in 0..30 {
        let kind = draws.below(5);
        let at = DVec2::new(
            draws.on_grid(left, right, 0.5),
            draws.on_grid(-6.0, 6.0, 0.5),
        );
        let size = DVec2::new(
            draws.pick(&[0.5, 1.0, 1.5, 2.0]),
            draws.pick(&[0.5, 1.0, 1.5, 2.0]),
        );
        let (footprint, regions) = match kind {
            0 | 1 => {
                let size = if kind == 1 {
                    size.max(DVec2::ONE)
                } else {
                    size
                };
                let frame = Frame {
                    low: at,
                    high: at + size,
                };
                let inner = (kind == 0).then_some(frame);
                (
                    Footprint {
                        outer: frame,
                        inner,
                    },
                    1 + kind,
                )
            }
            2 => {
                let radius = size.x;
                let reach = DVec2::splat(radius);
                let inscribed = reach / std::f64::consts::SQRT_2;
                let footprint = Footprint {
                    outer: Frame {
                        low: at - reach,
                        high: at + reach,
                    },
                    inner: Some(Frame {
                        low: at - inscribed,
                        high: at + inscribed,
                    }),
                };
                (footprint, 1)
            }
            _ => {
                let end = if kind == 3 {
                    at + DVec2::new(0.0, size.y)
                } else {
                    at + DVec2::new(size.x, 0.0)
                };
                let footprint = Footprint {
                    outer: Frame { low: at, high: end },
                    inner: None,
                };
                (footprint, 0)
            }
        };
        if placed
            .iter()
            .any(|other| !footprint.clear_of(other, period))
        {
            continue;
        }
        let Frame { low, high } = footprint.outer;
        match kind {
            0 => {
                drawing.path(
                    &[
                        low,
                        DVec2::new(high.x, low.y),
                        high,
                        DVec2::new(low.x, high.y),
                    ],
                    period,
                );
            }
            1 => {
                let middle = low.x + 0.5 * ((high.x - low.x) / 0.5).floor().max(1.0) * 0.5;
                let ranks = drawing.path(
                    &[
                        low,
                        DVec2::new(middle, low.y),
                        DVec2::new(high.x, low.y),
                        high,
                        DVec2::new(middle, high.y),
                        DVec2::new(low.x, high.y),
                    ],
                    period,
                );
                drawing.straight(
                    (ranks[1], DVec2::new(middle, low.y)),
                    (ranks[4], DVec2::new(middle, high.y)),
                );
            }
            2 => circle_in_pieces(drawing, draws, period, at, size.x),
            _ => {
                let [start, end] = [low, high].map(|point| (drawing.corner(point, period), point));
                drawing.straight(start, end);
            }
        }
        placed.push(footprint);
        return regions;
    }
    0
}

/// A circle whole with no vertex, looped at one vertex, or in two arcs
/// between two, the vertices often where it turns back.
fn circle_in_pieces(
    drawing: &mut Drawing,
    draws: &mut Draws,
    period: Option<f64>,
    center: DVec2,
    radius: f64,
) {
    let angles = [0.0, PI, 0.5 * PI, -0.5 * PI, 0.7];
    let first = draws.pick(&angles);
    let round = |start: f64, sweep: f64| Trace::Round {
        center,
        radius,
        start,
        sweep,
    };
    match draws.below(3) {
        0 => {
            drawing.arc(round(first, TURN), None);
        }
        1 => {
            let vertex = drawing.corner(center + DVec2::from_angle(first) * radius, period);
            drawing.arc(round(first, TURN), Some([vertex, vertex]));
        }
        _ => {
            let second = first + draws.pick(&[PI, 0.5 * PI, 2.0]);
            let [one, other] = [first, second]
                .map(|angle| drawing.corner(center + DVec2::from_angle(angle) * radius, period));
            let sweep = second - first;
            drawing.arc(round(first, sweep), Some([one, other]));
            drawing.arc(round(second, TURN - sweep), Some([other, one]));
        }
    }
}

fn random_drawing(seed: u64, period: Option<f64>) -> (Drawing, usize, usize) {
    let mut draws = Draws(seed.wrapping_mul(0x9E37_79B9_7F4A_7C15) | 1);
    let mut drawing = Drawing::default();
    let mut placed = Vec::new();
    let mut rounds = 0;
    if period.is_some() {
        for _ in 0..draws.below(3) {
            let height = draws.on_grid(-7.0, 7.0, 0.5);
            let footprint = Footprint {
                outer: Frame {
                    low: DVec2::new(-10.0, height),
                    high: DVec2::new(10.0, height),
                },
                inner: None,
            };
            if placed.iter().any(|other| !footprint.clear_of(other, None)) {
                continue;
            }
            drawing.round_the_cylinder(height, draws.pick(&[-PI, 0.5, 2.0]));
            placed.push(footprint);
            rounds += 1;
        }
    }
    let mut regions = 1 + rounds;
    for _ in 0..1 + draws.below(8) {
        regions += place(&mut drawing, &mut draws, period, &mut placed);
    }
    let unbounded = if rounds > 0 { 2 } else { 1 };
    (drawing, regions, unbounded)
}

fn random_drawings_hold(period: Option<f64>) {
    for seed in 0..300 {
        let (drawing, expected, unbounded) = random_drawing(seed, period);
        if drawing.arcs.is_empty() {
            continue;
        }
        println!("seed {seed}");
        let regions = drawing.regions(period);
        assert_eq!(regions.len(), expected, "seed {seed}");
        assert_eq!(regions.len() - bounded(&regions), unbounded, "seed {seed}");
    }
}

#[test]
fn random_shapes_on_a_plane_part_it_into_the_regions_they_bound() {
    random_drawings_hold(None);
}

#[test]
fn random_shapes_on_a_cylinder_part_it_into_the_regions_they_bound() {
    random_drawings_hold(ROUND);
}

type Cell = (i64, i64);

fn turning(one: Cell, other: Cell, at: Cell) -> i64 {
    (other.0 - one.0) * (at.1 - one.1) - (other.1 - one.1) * (at.0 - one.0)
}

fn on_side(at: Cell, [one, other]: [Cell; 2]) -> bool {
    turning(one, other, at) == 0
        && (one.0.min(other.0)..=one.0.max(other.0)).contains(&at.0)
        && (one.1.min(other.1)..=one.1.max(other.1)).contains(&at.1)
}

/// Whether two straight arcs between grid points share anything but an end:
/// a crossing, a point of one inside the other, or a stretch along it.
fn meet_between_ends(one: [Cell; 2], other: [Cell; 2]) -> bool {
    let inside = |at: Cell, side: [Cell; 2]| on_side(at, side) && !side.contains(&at);
    if one.iter().any(|&at| inside(at, other)) || other.iter().any(|&at| inside(at, one)) {
        return true;
    }
    let [a, b] = one;
    let [c, d] = other;
    if turning(a, b, c) == 0 && turning(a, b, d) == 0 {
        let shared = one.iter().find(|&&at| other.contains(&at));
        return match shared {
            Some(&at) => {
                let far = |side: [Cell; 2]| if side[0] == at { side[1] } else { side[0] };
                let [mine, theirs] = [far(one), far(other)];
                (mine.0 - at.0) * (theirs.0 - at.0) + (mine.1 - at.1) * (theirs.1 - at.1) > 0
            }
            None => false,
        };
    }
    let apart = |value: i64, other: i64| value.signum() * other.signum() < 0;
    apart(turning(a, b, c), turning(a, b, d)) && apart(turning(c, d, a), turning(c, d, b))
}

/// A drawing grown at random on a grid: straight arcs between grid points,
/// slanted as often as not, meeting only at their ends; then rounds, whole,
/// bulging between two grid points or hung tangent on a side at one of its
/// ends, kept clear of everything but the ends they share. On a cylinder the
/// grid closes on itself after `cells`.
struct Web {
    drawing: Drawing,
    cells: Option<i64>,
    step: f64,
    corners: std::collections::BTreeMap<Cell, usize>,
    sides: Vec<([Cell; 2], usize)>,
    rounds: Vec<(Vec<DVec2>, Option<[usize; 2]>)>,
    links: Vec<([usize; 2], i64)>,
    closed: usize,
}

impl Web {
    fn new(cells: Option<i64>) -> Web {
        Web {
            drawing: Drawing::default(),
            cells,
            step: cells.map_or(1.0, |cells| TURN / cells as f64),
            corners: std::collections::BTreeMap::new(),
            sides: Vec::new(),
            rounds: Vec::new(),
            links: Vec::new(),
            closed: 0,
        }
    }

    fn period(&self) -> Option<f64> {
        self.cells.map(|_| TURN)
    }

    fn at(&self, cell: Cell) -> DVec2 {
        DVec2::new(cell.0 as f64, cell.1 as f64) * self.step
    }

    fn turns(&self, cell: Cell) -> (Cell, i64) {
        match self.cells {
            Some(cells) => ((cell.0.rem_euclid(cells), cell.1), cell.0.div_euclid(cells)),
            None => (cell, 0),
        }
    }

    fn corner(&mut self, cell: Cell) -> usize {
        let (key, _) = self.turns(cell);
        if let Some(&rank) = self.corners.get(&key) {
            return rank;
        }
        let at = self.at(key);
        let x = self
            .period()
            .map_or(at.x, |period| at.x - (at.x / period).round() * period);
        let rank = self.drawing.vertex([x, at.y]);
        self.corners.insert(key, rank);
        rank
    }

    fn link(&mut self, from: Cell, to: Cell) -> [usize; 2] {
        let ends = [self.corner(from), self.corner(to)];
        let turns = self.turns(to).1 - self.turns(from).1;
        self.links.push((ends, turns));
        ends
    }

    fn side(&mut self, from: Cell, to: Cell) {
        if self.turns(from) == self.turns(to) {
            return;
        }
        let shifts = self.cells.map_or(vec![0], |cells| vec![-cells, 0, cells]);
        let clashes = self.sides.iter().any(|&([one, other], _)| {
            shifts.iter().any(|&shift| {
                let moved = [(one.0 + shift, one.1), (other.0 + shift, other.1)];
                meet_between_ends([from, to], moved)
            })
        });
        if clashes {
            return;
        }
        let ends = self.link(from, to);
        let trace = Trace::Segment {
            from: self.at(from),
            to: self.at(to),
        };
        let arc = self.drawing.arc(trace, Some(ends));
        self.sides.push(([from, to], arc));
    }

    fn apart(&self, one: DVec2, other: DVec2) -> f64 {
        let mut apart = one - other;
        if let Some(period) = self.period() {
            apart.x -= (apart.x / period).round() * period;
        }
        apart.length()
    }

    /// Whether a round through `samples`, ending at `ends`, keeps clear of
    /// every arc and corner drawn, far enough for the regions between them
    /// to hold a point clear of both: near an end it shares, only of the
    /// arcs that do not reach that end, and leaving it well apart from those
    /// that do; along the side it is hung on, only away from where it
    /// touches it.
    fn clear(&self, samples: &[DVec2], ends: Option<[usize; 2]>, hung: Option<Hang>) -> bool {
        const NEAR: f64 = 0.12;
        const SHARED: f64 = 0.08;
        let shared = ends.map_or(Vec::new(), |ends| ends.to_vec());
        let excused = |sample: DVec2, reaches: &[usize]| {
            reaches.iter().any(|&end| {
                shared.contains(&end) && self.apart(sample, self.drawing.vertices[end]) < SHARED
            })
        };
        let hugged = |sample: DVec2, arc: usize| {
            hung.is_some_and(|hang| {
                hang.side == arc
                    && self.apart(sample, self.drawing.vertices[hang.corner]) < hang.reach
            })
        };
        let sides_clear = self.sides.iter().all(|&([one, other], arc)| {
            let reaches = [
                self.corners[&self.turns(one).0],
                self.corners[&self.turns(other).0],
            ];
            let (from, to) = (self.at(one), self.at(other));
            samples.iter().all(|&sample| {
                excused(sample, &reaches)
                    || hugged(sample, arc)
                    || distance_to_side(sample, from, to, self.period()) > NEAR
            })
        });
        let rounds_clear = self.rounds.iter().all(|(theirs, reaches)| {
            let reaches = reaches.map_or(Vec::new(), |ends| ends.to_vec());
            samples.iter().all(|&sample| {
                excused(sample, &reaches)
                    || theirs.iter().all(|&other| self.apart(sample, other) > NEAR)
            })
        });
        let corners_clear = self.corners.values().all(|&corner| {
            shared.contains(&corner)
                || samples
                    .iter()
                    .all(|&sample| self.apart(sample, self.drawing.vertices[corner]) > NEAR)
        });
        let hugging = hung.map(|hang| hang.side);
        sides_clear && rounds_clear && corners_clear && self.leaves_apart(samples, ends, hugging)
    }

    /// Whether a round leaves each end it shares at least half a radian
    /// away from every arc already leaving that end, but the one it is hung
    /// on.
    fn leaves_apart(
        &self,
        samples: &[DVec2],
        ends: Option<[usize; 2]>,
        hugging: Option<usize>,
    ) -> bool {
        let Some([first, last]) = ends else {
            return true;
        };
        let angle = |direction: DVec2| direction.y.atan2(direction.x);
        let mine = [
            (first, angle(samples[1] - samples[0])),
            (
                last,
                angle(samples[samples.len() - 2] - samples[samples.len() - 1]),
            ),
        ];
        mine.iter().all(|&(end, leaving)| {
            self.drawing.arcs.iter().enumerate().all(|(rank, arc)| {
                let Some([start, finish]) = arc.ends else {
                    return true;
                };
                if hugging == Some(rank) {
                    return true;
                }
                let [_, first, _] = arc.trace.at(0.0);
                let [_, last, _] = arc.trace.at(1.0);
                [(start, first), (finish, -last)]
                    .iter()
                    .filter(|&&(corner, _)| corner == end)
                    .all(|&(_, direction)| {
                        let between = (angle(direction) - leaving).rem_euclid(TURN);
                        between.min(TURN - between) > 0.5
                    })
            })
        })
    }

    fn whole(&mut self, center: DVec2, radius: f64, start: f64) {
        let trace = Trace::Round {
            center,
            radius,
            start,
            sweep: TURN,
        };
        let samples = sampled(&Arc { trace, ends: None }, true);
        if self.clear(&samples, None, None) {
            self.drawing.arc(trace, None);
            self.rounds.push((samples, None));
            self.closed += 1;
        }
    }

    fn bulge(&mut self, from: Cell, to: Cell, sweep: f64) {
        if self.turns(from) == self.turns(to) {
            return;
        }
        let (start, end) = (self.at(from), self.at(to));
        let half = 0.5 * (end - start);
        let center = start + half + half.perp() / (0.5 * sweep).tan();
        let reach = start - center;
        let trace = Trace::Round {
            center,
            radius: reach.length(),
            start: reach.y.atan2(reach.x),
            sweep,
        };
        let known = |cell: Cell| self.corners.get(&self.turns(cell).0).copied();
        let ends = [known(from), known(to)];
        let shared = [ends[0].unwrap_or(usize::MAX), ends[1].unwrap_or(usize::MAX)];
        let samples = sampled(&Arc { trace, ends: None }, true);
        if self.clear(&samples, Some(shared), None) {
            let ends = self.link(from, to);
            self.drawing.arc(trace, Some(ends));
            self.rounds.push((samples, Some(ends)));
        }
    }

    /// A whole round looped at an end of a side and tangent to it there, on
    /// its left or on its right, run either way.
    fn hang(&mut self, side: usize, at_start: bool, radius: f64, left: bool, sweep: f64) {
        let ([from, to], arc) = self.sides[side];
        let (corner, along) = if at_start {
            (from, self.at(to) - self.at(from))
        } else {
            (to, self.at(from) - self.at(to))
        };
        let corner = self.corners[&self.turns(corner).0];
        let touch = self.drawing.vertices[corner];
        let across = along.normalize().perp() * if left { radius } else { -radius };
        let trace = Trace::Round {
            center: touch + across,
            radius,
            start: (-across).y.atan2(-across.x),
            sweep,
        };
        let samples = sampled(&Arc { trace, ends: None }, true);
        let hang = Hang {
            side: arc,
            corner,
            reach: 2.0 * (2.0 * radius * 0.12).sqrt(),
        };
        if self.clear(&samples, Some([corner, corner]), Some(hang)) {
            let ends = [corner, corner];
            self.drawing.arc(trace, Some(ends));
            self.links.push((ends, 0));
            self.rounds.push((samples, Some(ends)));
        }
    }

    fn expected_regions(&self) -> usize {
        let (components, _) = self.components();
        let vertices = self.corners.len();
        self.links.len() + components + self.closed + 1 - vertices
    }

    /// How many connected sets the linked corners make, and whether one of
    /// them goes round the cylinder: a loop of links whose turns do not add
    /// up to nought.
    fn components(&self) -> (usize, bool) {
        let count = self.drawing.vertices.len();
        let mut parent: Vec<usize> = (0..count).collect();
        let mut lift = vec![0_i64; count];
        let find = |parent: &mut Vec<usize>, lift: &mut Vec<i64>, mut node: usize| {
            let mut total = 0;
            while parent[node] != node {
                total += lift[node];
                node = parent[node];
            }
            (node, total)
        };
        let mut round = false;
        for &([from, to], turns) in &self.links {
            let (from_root, from_lift) = find(&mut parent, &mut lift, from);
            let (to_root, to_lift) = find(&mut parent, &mut lift, to);
            if from_root == to_root {
                round |= to_lift - from_lift != turns;
            } else {
                parent[to_root] = from_root;
                lift[to_root] = from_lift + turns - to_lift;
            }
        }
        let components = (0..count)
            .filter(|&node| {
                parent[node] == node && self.links.iter().any(|link| link.0.contains(&node))
            })
            .count();
        (components, round)
    }
}

/// A round hung on the side drawn as arc `side`, touching it at `corner`:
/// within `reach` of there the two run too close for the clearance asked of
/// anything else.
#[derive(Clone, Copy)]
struct Hang {
    side: usize,
    corner: usize,
    reach: f64,
}

fn distance_to_side(point: DVec2, from: DVec2, to: DVec2, period: Option<f64>) -> f64 {
    let shifts = period.map_or(vec![0.0], |period| vec![-period, 0.0, period]);
    shifts
        .into_iter()
        .map(|shift| {
            let point = point + DVec2::new(shift, 0.0);
            let along =
                ((point - from).dot(to - from) / (to - from).length_squared()).clamp(0.0, 1.0);
            (point - (from + (to - from) * along)).length()
        })
        .fold(f64::INFINITY, f64::min)
}

fn random_web(seed: u64, cells: Option<i64>) -> Web {
    let mut draws = Draws(seed.wrapping_mul(0x9E37_79B9_7F4A_7C15) | 1);
    let mut web = Web::new(cells);
    let span = 5;
    let cell = |draws: &mut Draws| {
        let mut coordinate = || draws.below(2 * span + 1) as i64 - span as i64;
        (coordinate(), coordinate())
    };
    for _ in 0..3 + draws.below(25) {
        let from = cell(&mut draws);
        let to = match (draws.below(6), cells) {
            (0, Some(cells)) => (from.0 + cells, from.1),
            (1 | 2, _) => (
                from.0 + draws.below(3) as i64 - 1,
                from.1 + draws.below(3) as i64 - 1,
            ),
            _ => cell(&mut draws),
        };
        web.side(from, to);
    }
    for _ in 0..draws.below(8) {
        let radius = draws.pick(&[0.3, 0.6, 1.1, 2.3]);
        let kind = draws.below(3);
        if kind == 0 {
            let center = web.at(cell(&mut draws)) + DVec2::splat(0.25 * draws.below(4) as f64);
            web.whole(center, radius, draws.pick(&[0.0, PI, 0.5 * PI, 1.3]));
        } else if kind == 1 && !web.sides.is_empty() {
            let side = draws.below(web.sides.len());
            let [at_start, left] = [draws.below(2) == 0, draws.below(2) == 0];
            web.hang(side, at_start, radius, left, draws.pick(&[TURN, -TURN]));
        } else {
            let from = cell(&mut draws);
            let to = (
                from.0 + draws.below(5) as i64 - 2,
                from.1 + draws.below(5) as i64 - 2,
            );
            let sweep = draws.pick(&[0.5 * PI, -0.5 * PI, PI, -PI, 1.5 * PI, -1.5 * PI]);
            web.bulge(from, to, sweep);
        }
    }
    web
}

fn random_webs_hold(cells: Option<i64>) {
    for seed in 0..300 {
        let web = random_web(seed, cells);
        if web.drawing.arcs.is_empty() {
            continue;
        }
        let regions = web.drawing.regions(web.period());
        assert_eq!(regions.len(), web.expected_regions(), "seed {seed}");
        let unbounded = if web.components().1 { 2 } else { 1 };
        assert_eq!(regions.len() - bounded(&regions), unbounded, "seed {seed}");
    }
}

#[test]
fn random_webs_of_slanted_sides_and_rounds_bulging_between_or_tangent_to_them_part_a_plane_into_the_regions_they_bound()
 {
    random_webs_hold(None);
}

#[test]
fn random_webs_of_slanted_sides_and_rounds_bulging_between_or_tangent_to_them_part_a_cylinder_into_the_regions_they_bound()
 {
    random_webs_hold(Some(12));
}
