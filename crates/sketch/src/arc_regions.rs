use glam::DVec2;

use crate::edges::Crossed;
use crate::edges::half_edge::Bend;
use crate::naming::CurveId;
use crate::regions::{Outline, signed_area};
use crate::sketch::Sketch;

/// Below this much of a turn apart, two half-edges leave a vertex the same
/// way, and which of them lies further round is a matter of how hard each of
/// them bends rather than of the last digit of an arctangent. A billionth of a
/// turn is a ten-thousandth of a degree.
const THE_SAME_WAY: f64 = 1e-9;

impl Sketch {
    /// Walks the segment and arc graph and returns each area it encloses, as
    /// a loop of positions turning counter-clockwise.
    ///
    /// A circle or an ellipse nothing cuts never reaches the graph, and comes
    /// back from `crossed` as the closed loop it already is.
    pub(crate) fn closed_outlines(&self) -> Vec<Outline> {
        let Crossed {
            places,
            ends,
            split,
            curves,
            from: cut_from,
            whole,
        } = self.crossed();
        let mut outlines: Vec<Outline> = whole
            .into_iter()
            .filter_map(|(curve, points)| {
                Some(all_of_one_curve(curve, self.bend_of(curve)?, points))
            })
            .collect();
        if ends.is_empty() {
            return outlines;
        }

        let departure = |half: usize| -> DVec2 {
            let from = places[ends[half].0];
            match half.checked_sub(split) {
                None => places[ends[half].1] - from,
                Some(curved) => curves[curved].departure(from),
            }
        };
        // Two curves that touch leave that place in one direction, and which
        // of them lies further round is then a matter of how hard each bends.
        let bending = |half: usize| -> f64 {
            let from = places[ends[half].0];
            match half.checked_sub(split) {
                None => 0.0,
                Some(curved) => curves[curved].bending(from),
            }
        };

        let mut leaving: Vec<Vec<usize>> = vec![Vec::new(); places.len()];
        for (half, (from, to)) in ends.iter().enumerate() {
            if from != to {
                leaving[*from].push(half);
            }
        }
        for half_edges in leaving.iter_mut() {
            // Two curves that touch leave in one and the same direction, which
            // the arithmetic only ever agrees on to the last digit or two. The
            // direction is therefore read to a grain far finer than any drawing
            // and far coarser than that, so the two come out equal and it is
            // how hard each bends that puts them in order.
            let leaving = |half: usize| {
                // Read to the grain first and brought round the turn after:
                // half a turn one way and half a turn the other are the same
                // direction, and so are a hair either side of nought.
                let angle = departure(half).to_angle();
                let round = (std::f64::consts::TAU / THE_SAME_WAY).round();
                (
                    (angle / THE_SAME_WAY).round().rem_euclid(round),
                    bending(half),
                )
            };
            half_edges.sort_by(|a, b| {
                let (near, far) = (leaving(*a), leaving(*b));
                near.0.total_cmp(&far.0).then(near.1.total_cmp(&far.1))
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
                    Some(curved) => {
                        let sampled = curves[curved].points_along(from, to);
                        // The run's number is where its bend is about to land,
                        // so the two cannot drift apart. Counted any other way
                        // they would agree only as long as nobody changed
                        // either — and a run whose bend went missing is read
                        // as the steps it was sampled into, which is the one
                        // answer this whole file exists to improve on.
                        let run = outline.bends.len();
                        outline.bends.push(curves[curved].bend);
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

impl Sketch {
    /// What a curve of the drawing bends along, for a loop that reached the
    /// outlines without passing through the graph — a circle or an ellipse
    /// nothing cut, which keeps its own geometry rather than a half-edge's.
    fn bend_of(&self, curve: CurveId) -> Option<Bend> {
        match curve {
            CurveId::Ellipse(id) => Some(Bend::Oval(self.ellipse_draft(id))),
            CurveId::Circle(id) => Some(Bend::Round(self.point(self.circle(id).center))),
            // Only a circle and an ellipse close a loop on their own, so only
            // those two ever arrive here. Naming a bend for the others would
            // be inventing one: a straight trait about the world origin reads
            // as a plausible wrong area rather than as nothing at all.
            CurveId::Arc(_) | CurveId::Segment(_) => None,
        }
    }
}

/// A circle or an ellipse nothing cut: every one of its segments came from the
/// one curve.
fn all_of_one_curve(curve: CurveId, bend: Bend, points: Vec<DVec2>) -> Outline {
    Outline {
        curves: vec![Some(0); points.len()],
        bounds: vec![curve],
        bends: vec![bend],
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
