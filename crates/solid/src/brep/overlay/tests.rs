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
    points
        .windows(2)
        .map(|pair| {
            let (start, end) = (pair[0], pair[1]);
            let (low, high) = (start.x.min(end.x), start.x.max(end.x));
            let x = match period {
                Some(period) => low + (point.x - low).rem_euclid(period),
                None => point.x,
            };
            if low == high || x < low || x >= high {
                return 0;
            }
            let y = start.y + (end.y - start.y) * (x - start.x) / (end.x - start.x);
            match (y < point.y, end.x > start.x) {
                (false, _) => 0,
                (true, true) => 1,
                (true, false) => -1,
            }
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
