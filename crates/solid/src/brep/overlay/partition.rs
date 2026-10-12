//! The cycles grouped into regions. The cycle facing up from the top of each
//! connected set of arcs belongs with whatever the first arc above it faces
//! down to; every other cycle is the outer boundary of a region of its own,
//! or on a cylinder the upper boundary of the band under it.

use glam::DVec2;

use super::column::{Column, Columns, Crossing, reach};
use super::piece::Piece;
use super::star::{Cycles, half};
use super::{Arc, Region};
use crate::brep::Declined;

/// Sets of nodes, each named by its least member, so that the naming does
/// not hang on the order the sets were joined in.
struct Sets {
    parent: Vec<usize>,
}

impl Sets {
    fn new(count: usize) -> Sets {
        Sets {
            parent: (0..count).collect(),
        }
    }

    fn find(&mut self, mut node: usize) -> usize {
        while self.parent[node] != node {
            self.parent[node] = self.parent[self.parent[node]];
            node = self.parent[node];
        }
        node
    }

    fn join(&mut self, one: usize, other: usize) {
        let (one, other) = (self.find(one), self.find(other));
        self.parent[one.max(other)] = one.min(other);
    }
}

/// What the grouping reads: the arcs, the cycles they bound, their pieces
/// and the columns across them.
pub(super) struct Layout<'a> {
    pub arcs: &'a [Arc],
    pub cycles: &'a Cycles,
    pub pieces: &'a [Piece],
    pub columns: &'a Columns,
}

impl Layout<'_> {
    fn above(&self, piece: usize) -> usize {
        let piece = &self.pieces[piece];
        self.cycles.of_half[half(piece.arc, piece.rightward)]
    }

    fn below(&self, piece: usize) -> usize {
        let piece = &self.pieces[piece];
        self.cycles.of_half[half(piece.arc, !piece.rightward)]
    }

    /// The regions, in the order of their first cycle.
    pub fn regions(&self) -> Result<Vec<Region>, Declined> {
        let count = self.cycles.list.len();
        let top = count;
        let bottom = if self.columns.period.is_some() {
            count + 1
        } else {
            top
        };
        let mut sets = Sets::new(count + 2);
        let components = self.components();
        let mut placed = vec![false; self.arcs.len()];
        let mut widest: Vec<usize> = (0..self.columns.list.len()).collect();
        widest.sort_by(|&one, &other| {
            let width = |rank: usize| self.columns.list[rank].width;
            width(other).total_cmp(&width(one))
        });
        for &rank in &widest {
            let crossings = &self.columns.list[rank].crossings;
            for (level, crossing) in crossings.iter().enumerate().rev() {
                let component = components[self.pieces[crossing.piece].arc];
                if placed[component] {
                    continue;
                }
                placed[component] = true;
                let over = crossings
                    .get(level + 1)
                    .map_or(top, |next| self.below(next.piece));
                sets.join(self.above(crossing.piece), over);
            }
        }
        for arc in 0..self.arcs.len() {
            let component = components[arc];
            if !placed[component] {
                placed[component] = true;
                let over = self.over_upright(component, &components, top)?;
                sets.join(self.cycles.of_half[half(component, true)], over);
            }
        }
        let under = widest
            .first()
            .and_then(|&rank| self.columns.list[rank].crossings.first())
            .map_or(top, |lowest| self.below(lowest.piece));
        sets.join(bottom, under);
        self.gather(&mut sets, top, bottom, widest.first().copied())
    }

    /// Each arc's connected set, named by its least arc: arcs round one
    /// cycle, and the two halves of an arc, are in one set.
    fn components(&self) -> Vec<usize> {
        let mut sets = Sets::new(self.arcs.len());
        for cycle in &self.cycles.list {
            for &(arc, _) in cycle {
                sets.join(cycle[0].0, arc);
            }
        }
        (0..self.arcs.len()).map(|arc| sets.find(arc)).collect()
    }

    /// What lies just above a set of arcs no column meets: upright segments
    /// at one abscissa. The column in the gap to its right crosses what a
    /// line just to the right of it would, in the same order; each is read
    /// at the set's own abscissa, and the first standing over its top is the
    /// one the set lies under.
    fn over_upright(
        &self,
        component: usize,
        components: &[usize],
        top: usize,
    ) -> Result<usize, Declined> {
        let members: Vec<usize> = (0..self.arcs.len())
            .filter(|&arc| components[arc] == component)
            .collect();
        let cycle = self.cycles.of_half[half(component, true)];
        if members.iter().any(|&arc| {
            self.cycles.of_half[half(arc, true)] != cycle
                || self.cycles.of_half[half(arc, false)] != cycle
        }) {
            return Err(Declined::Tie);
        }
        let own: Vec<&Piece> = self
            .pieces
            .iter()
            .filter(|piece| components[piece.arc] == component)
            .collect();
        let x = own[0].left.x;
        let height = own
            .iter()
            .flat_map(|piece| [piece.left.y, piece.right.y])
            .fold(f64::NEG_INFINITY, f64::max);
        let cluster = self.columns.cluster_of(x);
        let beside = if self.columns.period.is_some() || cluster < self.columns.list.len() {
            cluster
        } else if cluster > 0 {
            cluster - 1
        } else {
            return Ok(top);
        };
        let Some(column) = self.columns.list.get(beside) else {
            return Ok(top);
        };
        let offset = match self.columns.period {
            Some(period) => (column.x - x).rem_euclid(period),
            None => column.x - x,
        };
        for crossing in &column.crossings {
            let piece = &self.pieces[crossing.piece];
            let at = reach(piece, column.x, self.columns.period).unwrap_or(column.x);
            if piece.height(&self.arcs[piece.arc].trace, at - offset) > height {
                return Ok(self.below(crossing.piece));
            }
        }
        Ok(top)
    }

    fn gather(
        &self,
        sets: &mut Sets,
        top: usize,
        bottom: usize,
        widest: Option<usize>,
    ) -> Result<Vec<Region>, Declined> {
        let mut best: Vec<Option<(f64, &Column, [Crossing; 2])>> =
            vec![None; self.cycles.list.len() + 2];
        for column in &self.columns.list {
            for pair in column.crossings.windows(2) {
                let (low, high) = (pair[0], pair[1]);
                let root = sets.find(self.above(low.piece));
                if root != sets.find(self.below(high.piece)) {
                    continue;
                }
                let score = (high.height - low.height).min(column.width);
                if best[root].is_none_or(|(kept, ..)| score > kept) {
                    best[root] = Some((score, column, [low, high]));
                }
            }
        }
        let [far_below, far_above] = self.far(widest);
        let (top, bottom) = (sets.find(top), sets.find(bottom));
        let mut roots: Vec<usize> = Vec::new();
        let mut regions: Vec<Region> = Vec::new();
        for (rank, cycle) in self.cycles.list.iter().enumerate() {
            let root = sets.find(rank);
            let place = match roots.iter().position(|&known| known == root) {
                Some(place) => place,
                None => {
                    let (inside, chord, aside) = if root == top {
                        (far_above, [far_above.y; 2], [far_above; 2])
                    } else if root == bottom {
                        (far_below, [far_below.y; 2], [far_below; 2])
                    } else {
                        let (_, column, [low, high]) = best[root].ok_or(Declined::Tie)?;
                        let middle = 0.5 * (low.height + high.height);
                        (
                            DVec2::new(self.principal(column.x), middle),
                            [low.height, high.height],
                            [-0.25, 0.25].map(|share| self.between(column, [low, high], share)),
                        )
                    };
                    roots.push(root);
                    regions.push(Region {
                        cycles: Vec::new(),
                        inside,
                        chord,
                        aside,
                        unbounded: root == top || root == bottom,
                    });
                    regions.len() - 1
                }
            };
            regions[place].cycles.push(cycle.clone());
        }
        Ok(regions)
    }

    /// A point well below everything and one well above, beyond the arcs'
    /// box by its height.
    fn far(&self, widest: Option<usize>) -> [DVec2; 2] {
        let mut low = DVec2::splat(f64::INFINITY);
        let mut high = DVec2::splat(f64::NEG_INFINITY);
        for arc in self.arcs {
            for step in 0..=FAR_SAMPLES {
                let point = arc.trace.at(step as f64 / FAR_SAMPLES as f64)[0];
                low = low.min(point);
                high = high.max(point);
            }
        }
        let margin = (high.y - low.y).max(1.0);
        let x = match (self.columns.period, widest) {
            (Some(_), Some(rank)) => self.principal(self.columns.list[rank].x),
            (Some(_), None) => 0.0,
            (None, _) => 0.5 * (low.x + high.x),
        };
        [
            DVec2::new(x, low.y - margin),
            DVec2::new(x, high.y + margin),
        ]
    }

    /// The point `share` of a column's gap from the column, midway between
    /// two pieces it crosses one above the other: between the same two all
    /// across the gap, where no piece ends and none crosses another.
    fn between(&self, column: &Column, pair: [Crossing; 2], share: f64) -> DVec2 {
        let x = column.x + column.width * share;
        let [low, high] = pair.map(|crossing| {
            let piece = &self.pieces[crossing.piece];
            reach(piece, x, self.columns.period).map_or(crossing.height, |at| {
                piece.height(&self.arcs[piece.arc].trace, at)
            })
        });
        DVec2::new(self.principal(x), 0.5 * (low + high))
    }

    /// An abscissa brought into the turn about nought, as a cylinder reads
    /// its angles.
    fn principal(&self, x: f64) -> f64 {
        match self.columns.period {
            Some(period) => x - (x / period).round() * period,
            None => x,
        }
    }
}

/// How many points of each arc the box round them is taken from.
const FAR_SAMPLES: usize = 64;
