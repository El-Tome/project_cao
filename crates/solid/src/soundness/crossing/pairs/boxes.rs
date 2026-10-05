//! Boxes nested by halves, to find the ones near a given box without trying
//! them all: a bore lying through a stock has walls as long as the stock along
//! every axis it could be swept along.

use std::ops::Range;

use glam::DVec3;

/// Boxes at most this many to a leaf.
const LEAF: usize = 8;

/// A box around a run of `order`, around a leaf's boxes or around its two
/// halves; and the least and greatest of the indices under it.
struct Node {
    low: DVec3,
    high: DVec3,
    run: Range<usize>,
    indices: [usize; 2],
    halves: Option<[usize; 2]>,
}

pub(super) struct Boxes<'a> {
    boxes: &'a [[DVec3; 2]],
    order: Vec<usize>,
    nodes: Vec<Node>,
}

impl<'a> Boxes<'a> {
    pub(super) fn of(boxes: &'a [[DVec3; 2]]) -> Self {
        let mut tree = Self {
            boxes,
            order: (0..boxes.len()).collect(),
            nodes: Vec::new(),
        };
        if !boxes.is_empty() {
            tree.split(0..boxes.len());
        }
        tree
    }

    /// Nests the run of `order`, halved along the way its middles spread most,
    /// and hands back the node holding it.
    fn split(&mut self, run: Range<usize>) -> usize {
        let middle = |at: &usize| (self.boxes[*at][0] + self.boxes[*at][1]) / 2.0;
        let (mut low, mut high) = (DVec3::INFINITY, DVec3::NEG_INFINITY);
        let (mut centre_low, mut centre_high) = (DVec3::INFINITY, DVec3::NEG_INFINITY);
        for at in &self.order[run.clone()] {
            let [from, to] = self.boxes[*at];
            (low, high) = (low.min(from), high.max(to));
            let centre = middle(at);
            (centre_low, centre_high) = (centre_low.min(centre), centre_high.max(centre));
        }
        let indices = self.order[run.clone()]
            .iter()
            .fold([usize::MAX, 0], |[least, most], at| {
                [least.min(*at), most.max(*at)]
            });
        let node = self.nodes.len();
        self.nodes.push(Node {
            low,
            high,
            run: run.clone(),
            indices,
            halves: None,
        });
        if run.len() > LEAF {
            let axis = (centre_high - centre_low).max_position();
            let half = run.len() / 2;
            self.order[run.clone()].select_nth_unstable_by(half, |one, other| {
                middle(one)[axis].total_cmp(&middle(other)[axis])
            });
            let first = self.split(run.start..run.start + half);
            let second = self.split(run.start + half..run.end);
            self.nodes[node].halves = Some([first, second]);
        }
        node
    }

    /// Adds to `found` every index within `among` whose box may come within
    /// `room` of the box from `low` to `high`, and some that do not: the
    /// caller weighs each one itself.
    pub(super) fn near(
        &self,
        low: DVec3,
        high: DVec3,
        room: f64,
        among: Range<usize>,
        found: &mut Vec<usize>,
    ) {
        let mut pending = Vec::from_iter((!self.nodes.is_empty()).then_some(0));
        while let Some(node) = pending.pop() {
            let node = &self.nodes[node];
            let [least, most] = node.indices;
            let reached = node.low.cmple(high + room) & low.cmple(node.high + room);
            if !reached.all() || most < among.start || least >= among.end {
                continue;
            }
            match node.halves {
                Some(halves) => pending.extend(halves),
                None => found.extend(
                    self.order[node.run.clone()]
                        .iter()
                        .filter(|at| among.contains(at)),
                ),
            }
        }
    }
}
