use glam::DVec2;

use crate::edges::Crossed;
use crate::naming::CurveId;
use crate::regions::{Outline, signed_area};
use crate::sketch::{CircleId, Sketch};

impl Sketch {
    /// Walks the segment and arc graph and returns each area it encloses, as
    /// a loop of positions turning counter-clockwise.
    ///
    /// A circle nothing cuts never reaches the graph, and comes back from
    /// `crossed` as the closed loop it already is.
    pub(crate) fn closed_outlines(&self) -> Vec<Outline> {
        let Crossed {
            places,
            ends,
            split,
            arcs,
            from: cut_from,
            whole,
        } = self.crossed();
        let mut outlines: Vec<Outline> = whole
            .into_iter()
            .map(|(circle, points)| all_of_one_curve(circle, points))
            .collect();
        if ends.is_empty() {
            return outlines;
        }

        let departure = |half: usize| -> DVec2 {
            let from = places[ends[half].0];
            match half.checked_sub(split) {
                None => places[ends[half].1] - from,
                Some(arc) => arcs[arc].departure(from),
            }
        };

        let mut leaving: Vec<Vec<usize>> = vec![Vec::new(); places.len()];
        for (half, (from, to)) in ends.iter().enumerate() {
            if from != to {
                leaving[*from].push(half);
            }
        }
        for half_edges in leaving.iter_mut() {
            half_edges.sort_by(|a, b| {
                departure(*a)
                    .to_angle()
                    .total_cmp(&departure(*b).to_angle())
            });
        }

        let next = |half: usize| -> Option<usize> {
            let twin = half ^ 1;
            let around = &leaving[ends[half].1];
            let position = around.iter().position(|candidate| *candidate == twin)?;
            Some(around[(position + around.len() - 1) % around.len()])
        };

        let mut visited = vec![false; ends.len()];
        for start in 0..ends.len() {
            if visited[start] || ends[start].0 == ends[start].1 {
                continue;
            }
            let mut walked = Vec::new();
            let mut half = start;
            let mut closed = false;
            loop {
                if visited[half] {
                    break;
                }
                visited[half] = true;
                walked.push(half);
                let Some(following) = next(half) else { break };
                half = following;
                if half == start {
                    closed = true;
                    break;
                }
            }

            // Only a walk that came back on itself bounds anything. One that
            // ran out of edges, or into edges an earlier face had taken,
            // leaves an open chain — which used to be kept, and drew a shape
            // closed by an edge nobody had drawn.
            if !closed {
                continue;
            }

            let mut outline = Outline::default();
            for half in without_spurs(&walked) {
                outline.bounds.push(cut_from[half]);
                let (from, to) = (places[ends[half].0], places[ends[half].1]);
                match half.checked_sub(split) {
                    None => {
                        outline.points.push(from);
                        outline.curves.push(None);
                    }
                    Some(arc) => {
                        let sampled = arcs[arc].points_along(from, to);
                        let run = outline
                            .curves
                            .iter()
                            .flatten()
                            .max()
                            .map_or(0, |last| last + 1);
                        outline
                            .curves
                            .extend(std::iter::repeat_n(Some(run), sampled.len()));
                        outline.points.extend(sampled);
                    }
                }
            }

            // Turning the other way round the same edges walks the outside of
            // the drawing, which is not an area: only the face the walk keeps
            // on its left has a positive signed area. A face pinched at a
            // point — a bowtie's crossing, a point dropped on a trait — walks
            // that point twice, quite correctly, so nothing here may ask for
            // the corners to be distinct.
            if signed_area(&outline.points) > 1e-9 {
                outline.bounds.sort_unstable();
                outline.bounds.dedup();
                outlines.push(outline);
            }
        }
        outlines
    }
}

/// A circle nothing cut: every one of its segments came from the one curve.
fn all_of_one_curve(circle: CircleId, points: Vec<DVec2>) -> Outline {
    Outline {
        curves: vec![Some(0); points.len()],
        bounds: vec![CurveId::Circle(circle)],
        points,
    }
}

/// The half-edges that really bound the face, with every trait the walk had to
/// go out along and give straight back taken out.
///
/// A trait poking into a face is walked twice, once each way, and bounds
/// nothing at all: a slit of no width is not a slit. Leaving it in would make
/// the outline double back on itself, which is the one shape an ear-clipper
/// cannot cut into triangles.
///
/// Cancelling pairs against a stack takes out a whole beard of them and not
/// just the last hair, since taking one out can leave its neighbours face to
/// face. The walk is a ring, so the join is closed up too.
fn without_spurs(walked: &[usize]) -> Vec<usize> {
    let mut bounding: Vec<usize> = Vec::with_capacity(walked.len());
    for half in walked {
        match bounding.last() {
            Some(previous) if *previous == (half ^ 1) => {
                bounding.pop();
            }
            _ => bounding.push(*half),
        }
    }
    while bounding.len() >= 2 && bounding[0] == (bounding[bounding.len() - 1] ^ 1) {
        bounding.pop();
        bounding.remove(0);
    }
    bounding
}

#[cfg(test)]
mod tests;
