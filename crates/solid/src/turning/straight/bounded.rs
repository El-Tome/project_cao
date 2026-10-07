//! The laid profile read again as the matter it bounds, where laying made it
//! touch itself: two levels closer than the tolerance are one, so a wall or
//! a gap between them is not there. The matter is read column by column
//! between the levels along the axis — inside the outline and in none of the
//! holes — and its boundary traced again, each run of it named by the run it
//! lies on.
//!
//! Every corner stands on a level, shared to the bit, and every run is laid
//! exactly square or parallel to the axis, or slanted between two corners. A
//! slanted run met anywhere but at its own corners is declined first, so a
//! run crosses a column whole, the runs across one never cross inside it,
//! and where a slant passes a level between its corners is worked out once.

use std::cmp::Ordering;
use std::collections::BTreeMap;

use glam::DVec2;

use super::Corner;
use super::contacts::{self, Run};

/// The contours bounding the matter of the laid profile, the outline first
/// and anticlockwise, each hole after it clockwise; `None` when the matter
/// is not one piece, has none, touches itself at a corner, or meets a slant
/// anywhere but at its corners. `contours` are the runs of each laid contour,
/// the outline's first.
pub(super) fn contours(
    laid: &[DVec2],
    contours: &[Vec<Run>],
    tolerance: f64,
) -> Option<Vec<Vec<Corner>>> {
    let runs: Vec<Run> = contours.iter().flatten().copied().collect();
    if contacts::a_slant_is_met(laid, &runs, tolerance) {
        return None;
    }
    let mut along: Vec<f64> = runs.iter().map(|&(from, _)| laid[from].x).collect();
    along.sort_by(f64::total_cmp);
    along.dedup();
    let columns = along
        .windows(2)
        .map(|ends| Column::of(laid, contours, ends[0], ends[1]))
        .collect::<Option<Vec<Column>>>()?;

    let mut leaving: BTreeMap<Node, Edge> = BTreeMap::new();
    let mut edges = Vec::new();
    for (index, column) in columns.iter().enumerate() {
        edges.extend(column.boundary(index)?);
    }
    for (index, &level) in along.iter().enumerate() {
        let west = index.checked_sub(1).and_then(|column| columns.get(column));
        edges.extend(walls(
            laid,
            &runs,
            (index, level),
            west,
            columns.get(index),
        )?);
    }
    for edge in edges {
        if leaving.insert(edge.from, edge).is_some() {
            return None;
        }
    }

    let mut loops: Vec<Vec<Corner>> = Vec::new();
    while let Some((&start, _)) = leaving.first_key_value() {
        let mut walked = Vec::new();
        let mut at = start;
        while let Some(edge) = leaving.remove(&at) {
            at = edge.to;
            walked.push(edge);
        }
        if at != start {
            return None;
        }
        loops.push(corners(&along, &walked));
    }
    let area = |corners: &Vec<Corner>| -> f64 {
        corners
            .iter()
            .zip(corners.iter().cycle().skip(1))
            .map(|(from, to)| from.at.perp_dot(to.at))
            .sum()
    };
    let (outlines, holes): (Vec<_>, Vec<_>) = loops.into_iter().partition(|each| area(each) > 0.0);
    let [outline] = <[Vec<Corner>; 1]>::try_from(outlines).ok()?;
    Some(std::iter::once(outline).chain(holes).collect())
}

/// A point of the boundary: the level along the axis it stands on, by its
/// rank, and its distance from the axis.
type Node = (usize, Away);

/// A distance from the axis, ordered as a number, nought and minus nought
/// one.
#[derive(Clone, Copy, Debug)]
struct Away(f64);

impl PartialEq for Away {
    fn eq(&self, other: &Away) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl Eq for Away {}

impl PartialOrd for Away {
    fn partial_cmp(&self, other: &Away) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Away {
    fn cmp(&self, other: &Away) -> Ordering {
        (self.0 + 0.0).total_cmp(&(other.0 + 0.0))
    }
}

/// A stretch of the boundary along one run, the matter on its left.
#[derive(Clone, Copy)]
struct Edge {
    from: Node,
    to: Node,
    number: u32,
}

/// The runs passing across the space between two neighbouring levels along
/// the axis, from the nearest to the axis, and whether the matter fills the
/// space above each.
struct Column {
    places: Vec<Place>,
    matter: Vec<bool>,
}

/// Where one or more runs pass across a column, at its two ends, and the
/// numbers of the runs passing there.
struct Place {
    ends: [f64; 2],
    numbers: Vec<u32>,
}

impl Column {
    /// `None` when two runs cross inside it.
    fn of(laid: &[DVec2], contours: &[Vec<Run>], start: f64, end: f64) -> Option<Column> {
        let mut lines: Vec<([f64; 2], usize, u32)> = contours
            .iter()
            .enumerate()
            .flat_map(|(contour, runs)| runs.iter().map(move |&run| (contour, run)))
            .filter(|&(_, (from, to))| {
                let (from, to) = (laid[from].x, laid[to].x);
                from != to && from.min(to) <= start && end <= from.max(to)
            })
            .map(|(contour, run)| {
                let ends = [passing(laid, run, start), passing(laid, run, end)];
                (ends, contour, run.0 as u32)
            })
            .collect();
        lines.sort_by(|(one, ..), (other, ..)| {
            one[0]
                .total_cmp(&other[0])
                .then(one[1].total_cmp(&other[1]))
        });
        if lines.windows(2).any(|pair| pair[1].0[1] < pair[0].0[1]) {
            return None;
        }
        let mut inside = vec![false; contours.len()];
        let mut places: Vec<Place> = Vec::new();
        let mut matter = Vec::new();
        for (ends, contour, number) in lines {
            inside[contour] = !inside[contour];
            match places.last_mut() {
                Some(place) if place.ends == ends => {
                    place.numbers.push(number);
                    matter.pop();
                }
                _ => places.push(Place {
                    ends,
                    numbers: vec![number],
                }),
            }
            matter.push(inside[0] && !inside[1..].iter().any(|&hole| hole));
        }
        Some(Column { places, matter })
    }

    /// Whether the matter fills the column just above `low` at one of its
    /// ends: above the last place there not beyond it.
    fn holds(&self, end: usize, low: f64) -> bool {
        self.places
            .iter()
            .rposition(|place| place.ends[end] <= low)
            .is_some_and(|place| self.matter[place])
    }

    /// Each stretch of the boundary across the column, the `index`th: where
    /// a place has matter on one side and none on the other. `None` when it
    /// lies on several runs.
    fn boundary(&self, index: usize) -> Option<Vec<Edge>> {
        let mut edges = Vec::new();
        for (rank, place) in self.places.iter().enumerate() {
            let below = rank.checked_sub(1).is_some_and(|under| self.matter[under]);
            let above = self.matter[rank];
            if below != above {
                let [number] = place.numbers[..] else {
                    return None;
                };
                let (start, end) = (
                    (index, Away(place.ends[0])),
                    (index + 1, Away(place.ends[1])),
                );
                let (from, to) = if above { (start, end) } else { (end, start) };
                edges.push(Edge { from, to, number });
            }
        }
        Some(edges)
    }
}

/// Each stretch of the boundary on a level along the axis, the `index`th,
/// between the column before it and the one after: where the matter fills
/// one side and not the other. `None` when it lies on no run square to the
/// axis, or on several.
fn walls(
    laid: &[DVec2],
    runs: &[Run],
    (index, level): (usize, f64),
    west: Option<&Column>,
    east: Option<&Column>,
) -> Option<Vec<Edge>> {
    let mut heights: Vec<f64> = runs
        .iter()
        .map(|&(from, _)| laid[from])
        .filter(|corner| corner.x == level)
        .map(|corner| corner.y)
        .chain(
            west.into_iter()
                .flat_map(|column| column.places.iter().map(|place| place.ends[1])),
        )
        .chain(
            east.into_iter()
                .flat_map(|column| column.places.iter().map(|place| place.ends[0])),
        )
        .collect();
    heights.sort_by(f64::total_cmp);
    heights.dedup();
    let mut edges = Vec::new();
    for pair in heights.windows(2) {
        let (low, high) = (pair[0], pair[1]);
        let before = west.is_some_and(|column| column.holds(1, low));
        let after = east.is_some_and(|column| column.holds(0, low));
        if before == after {
            continue;
        }
        let mut lying = runs.iter().filter(|&&(from, to)| {
            let (from, to) = (laid[from], laid[to]);
            from.x == level && to.x == level && from.y.min(to.y) <= low && high <= from.y.max(to.y)
        });
        let &(number, _) = lying.next()?;
        if lying.next().is_some() {
            return None;
        }
        let (start, end) = ((index, Away(low)), (index, Away(high)));
        let (from, to) = if before { (start, end) } else { (end, start) };
        edges.push(Edge {
            from,
            to,
            number: number as u32,
        });
    }
    Some(edges)
}

/// How far from the axis a run not square to it passes at `level` along
/// it: its own corner's distance to the bit where it ends there.
fn passing(laid: &[DVec2], (from, to): Run, level: f64) -> f64 {
    let (from, to) = (laid[from], laid[to]);
    if level == from.x {
        from.y
    } else if level == to.x {
        to.y
    } else {
        from.y + (to.y - from.y) * ((level - from.x) / (to.x - from.x))
    }
}

/// The corners of a closed walk of edges: where it turns, or where the run
/// it lies on changes.
fn corners(along: &[f64], edges: &[Edge]) -> Vec<Corner> {
    let at = |(index, away): Node| DVec2::new(along[index], away.0);
    let heading = |edge: &Edge| {
        let step = at(edge.to) - at(edge.from);
        let sign = |value: f64| (value > 0.0) as i8 - (value < 0.0) as i8;
        (sign(step.x), sign(step.y))
    };
    let mut corners = Vec::new();
    for (index, edge) in edges.iter().enumerate() {
        let before = &edges[(index + edges.len() - 1) % edges.len()];
        if heading(before) != heading(edge) || before.number != edge.number {
            corners.push(Corner {
                at: at(edge.from),
                run: edge.number,
            });
        }
    }
    corners
}
