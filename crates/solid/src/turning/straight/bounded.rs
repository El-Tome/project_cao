//! The laid profile read again as the matter it bounds, where laying made it
//! touch itself: two levels closer than the tolerance are one, so a wall or
//! a gap between them is not there. The matter is read cell by cell between
//! the levels — inside the outline and in none of the holes — and its
//! boundary traced again, each run of it named by the run it lies on.
//!
//! Every corner stands on a level, shared to the bit, and every run is laid
//! exactly square or parallel to the axis: the cells are read exactly.

use std::collections::BTreeMap;

use glam::DVec2;

use super::Corner;
use super::contacts::Run;

/// The contours bounding the matter of the laid profile, the outline first
/// and anticlockwise, each hole after it clockwise; `None` when the matter
/// is not one piece, has none, or touches itself at a corner. `contours` are
/// the runs of each laid contour, the outline's first.
pub(super) fn contours(laid: &[DVec2], contours: &[Vec<Run>]) -> Option<Vec<Vec<Corner>>> {
    let grid = Grid::of(laid, contours);
    let mut leaving: BTreeMap<(usize, usize), Edge> = BTreeMap::new();
    for edge in grid.boundary() {
        let number = grid.number(laid, contours, &edge)?;
        let edge = Edge { number, ..edge };
        if leaving.insert(edge.from, edge).is_some() {
            return None;
        }
    }
    let mut loops: Vec<Vec<Corner>> = Vec::new();
    while let Some((&start, _)) = leaving.first_key_value() {
        let mut edges = Vec::new();
        let mut at = start;
        while let Some(edge) = leaving.remove(&at) {
            at = edge.to;
            edges.push(edge);
        }
        if at != start {
            return None;
        }
        loops.push(grid.corners(&edges));
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

/// A step of the boundary between two cells, from one point of the grid to
/// the next, the matter on its left, and the run it lies on.
#[derive(Clone, Copy)]
struct Edge {
    from: (usize, usize),
    to: (usize, usize),
    number: u32,
}

/// The levels along the axis and away from it, each once, in order, and
/// whether each cell between them holds matter.
struct Grid {
    along: Vec<f64>,
    away: Vec<f64>,
    matter: Vec<Vec<bool>>,
}

impl Grid {
    fn of(laid: &[DVec2], contours: &[Vec<Run>]) -> Grid {
        let levels = |value: fn(DVec2) -> f64| {
            let mut levels: Vec<f64> = contours
                .iter()
                .flatten()
                .map(|&(from, _)| value(laid[from]))
                .collect();
            levels.sort_by(f64::total_cmp);
            levels.dedup();
            levels
        };
        let (along, away) = (levels(|at| at.x), levels(|at| at.y));
        let matter = (0..along.len().saturating_sub(1))
            .map(|column| {
                (0..away.len().saturating_sub(1))
                    .map(|row| {
                        let middle = DVec2::new(
                            (along[column] + along[column + 1]) / 2.0,
                            (away[row] + away[row + 1]) / 2.0,
                        );
                        let mut inside = contours.iter().map(|runs| encloses(laid, runs, middle));
                        inside.next().unwrap_or(false) && !inside.any(|hole| hole)
                    })
                    .collect()
            })
            .collect();
        Grid {
            along,
            away,
            matter,
        }
    }

    fn holds(&self, column: Option<usize>, row: Option<usize>) -> bool {
        match (column, row) {
            (Some(column), Some(row)) => self
                .matter
                .get(column)
                .and_then(|cells| cells.get(row))
                .copied()
                .unwrap_or(false),
            _ => false,
        }
    }

    /// Every step between a cell with matter and one without, unnamed yet.
    fn boundary(&self) -> Vec<Edge> {
        let mut edges = Vec::new();
        let before = |index: usize| index.checked_sub(1);
        for column in 0..self.along.len() {
            for row in 0..self.away.len().saturating_sub(1) {
                let (left, right) = (
                    self.holds(before(column), Some(row)),
                    self.holds(Some(column), Some(row)),
                );
                if left != right {
                    let (low, high) = ((column, row), (column, row + 1));
                    let (from, to) = if left { (low, high) } else { (high, low) };
                    edges.push(Edge {
                        from,
                        to,
                        number: 0,
                    });
                }
            }
        }
        for row in 0..self.away.len() {
            for column in 0..self.along.len().saturating_sub(1) {
                let (below, above) = (
                    self.holds(Some(column), before(row)),
                    self.holds(Some(column), Some(row)),
                );
                if below != above {
                    let (low, high) = ((column, row), (column + 1, row));
                    let (from, to) = if above { (low, high) } else { (high, low) };
                    edges.push(Edge {
                        from,
                        to,
                        number: 0,
                    });
                }
            }
        }
        edges
    }

    fn at(&self, (column, row): (usize, usize)) -> DVec2 {
        DVec2::new(self.along[column], self.away[row])
    }

    /// The number of the one run the step lies on; `None` when it lies on
    /// none, or on several.
    fn number(&self, laid: &[DVec2], contours: &[Vec<Run>], edge: &Edge) -> Option<u32> {
        let (from, to) = (self.at(edge.from), self.at(edge.to));
        let covers = |&&(start, end): &&Run| {
            let (start, end) = (laid[start], laid[end]);
            let within = |value: f64, one: f64, other: f64| {
                one.min(other) <= value && value <= one.max(other)
            };
            let on =
                |point: DVec2| within(point.x, start.x, end.x) && within(point.y, start.y, end.y);
            (start.x == end.x || start.y == end.y) && start != end && on(from) && on(to)
        };
        let mut lying = contours.iter().flatten().filter(covers);
        let &(number, _) = lying.next()?;
        lying.next().is_none().then_some(number as u32)
    }

    /// The corners of a closed walk of steps: where it turns, or where the
    /// run it lies on changes.
    fn corners(&self, edges: &[Edge]) -> Vec<Corner> {
        let heading = |edge: &Edge| {
            (
                edge.to.0 as i64 - edge.from.0 as i64,
                edge.to.1 as i64 - edge.from.1 as i64,
            )
        };
        let mut corners = Vec::new();
        for (index, edge) in edges.iter().enumerate() {
            let before = &edges[(index + edges.len() - 1) % edges.len()];
            if heading(before) != heading(edge) || before.number != edge.number {
                corners.push(Corner {
                    at: self.at(edge.from),
                    run: edge.number,
                });
            }
        }
        corners
    }
}

/// Whether a laid contour encloses a point standing on none of its levels:
/// the runs square to the axis it crosses going along it, counted.
fn encloses(laid: &[DVec2], runs: &[Run], point: DVec2) -> bool {
    runs.iter()
        .filter(|&&(from, to)| {
            let (from, to) = (laid[from], laid[to]);
            from.x == to.x
                && from.x > point.x
                && from.y.min(to.y) < point.y
                && point.y < from.y.max(to.y)
        })
        .count()
        % 2
        == 1
}
