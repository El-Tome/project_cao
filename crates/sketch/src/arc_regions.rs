//! The half-edge walk that finds a drawing's closed outlines, and an arc as
//! the two half-edges it contributes to it: kept apart from `regions.rs`,
//! which turns outlines already found into nested, triangulated `Region`s and
//! has no need to know a walk found them or a tangent bent one.

use glam::DVec2;

use crate::arcing::{ArcDraft, places_along};
use crate::regions::signed_area;
use crate::sketch::Sketch;

/// One end of one arc, as a graph half-edge: where it leaves from, the
/// tangent it leaves along — not the straight line to its far end, which is
/// what tells the region walk apart from a plain segment's — and which way
/// round the curve it walks.
struct ArcHalfEdge {
    from: usize,
    to: usize,
    center: DVec2,
    /// Whether this half leaves the arc's own `start`, curving the way it was
    /// drawn, or leaves `end` and so walks the same curve backwards.
    forward: bool,
}

/// Every arc still drawn, as the two half-edges it contributes to the walk.
fn arc_half_edges(sketch: &Sketch) -> Vec<ArcHalfEdge> {
    sketch
        .live_arcs()
        .filter(|(_, arc)| !arc.construction)
        .flat_map(|(_, arc)| {
            let center = sketch.point(arc.center);
            [
                ArcHalfEdge {
                    from: arc.start.0,
                    to: arc.end.0,
                    center,
                    forward: true,
                },
                ArcHalfEdge {
                    from: arc.end.0,
                    to: arc.start.0,
                    center,
                    forward: false,
                },
            ]
        })
        .collect()
}

impl ArcHalfEdge {
    /// The direction it leaves `from` in: perpendicular to the reach from the
    /// centre, turned the way the curve actually bends at that end.
    fn departure(&self, from: DVec2) -> DVec2 {
        let reach = from - self.center;
        match self.forward {
            true => DVec2::new(-reach.y, reach.x),
            false => DVec2::new(reach.y, -reach.x),
        }
    }

    /// The curve this half contributes to an outline: sampled from its own
    /// `from` up to, but not including, `to` — the same convention a
    /// segment's single point already follows, so the next half-edge, or the
    /// walk closing, supplies the rest.
    fn points_along(&self, from: DVec2, to: DVec2) -> Vec<DVec2> {
        let (start, end) = match self.forward {
            true => (from, to),
            false => (to, from),
        };
        let mut sampled = places_along(ArcDraft {
            centre: self.center,
            start,
            end,
        });
        if !self.forward {
            sampled.reverse();
        }
        sampled.pop();
        sampled
    }
}

impl Sketch {
    /// Walks the segment and arc graph and returns each area it encloses, as
    /// a loop of positions turning counter-clockwise.
    pub(crate) fn closed_outlines(&self) -> Vec<Vec<DVec2>> {
        // Only what is still drawn: a deleted side must not close an area that is no longer there.
        let segment_ends: Vec<(usize, usize)> = self
            .live_segments()
            .filter(|(_, segment)| !segment.construction)
            .flat_map(|(_, segment)| {
                [
                    (segment.start.0, segment.end.0),
                    (segment.end.0, segment.start.0),
                ]
            })
            .collect();
        let arcs = arc_half_edges(self);
        let split = segment_ends.len();
        let ends: Vec<(usize, usize)> = segment_ends
            .into_iter()
            .chain(arcs.iter().map(|edge| (edge.from, edge.to)))
            .collect();
        if ends.is_empty() {
            return Vec::new();
        }

        // An arc leaves a point along its tangent, not the straight line to
        // its far end — that is what tells the walk apart from a segment's.
        let departure = |half: usize| -> DVec2 {
            let from = self.points()[ends[half].0];
            match half.checked_sub(split) {
                None => self.points()[ends[half].1] - from,
                Some(arc) => arcs[arc].departure(from),
            }
        };

        let mut leaving: Vec<Vec<usize>> = vec![Vec::new(); self.points().len()];
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
            // The neighbour just clockwise of the way we came: turning as
            // tightly as possible is what keeps the walk hugging one area.
            Some(around[(position + around.len() - 1) % around.len()])
        };

        let mut visited = vec![false; ends.len()];
        let mut outlines = Vec::new();
        for start in 0..ends.len() {
            if visited[start] || ends[start].0 == ends[start].1 {
                continue;
            }
            let mut loop_edges = Vec::new();
            let mut outline = Vec::new();
            let mut half = start;
            let mut dead_end = false;
            loop {
                if visited[half] {
                    break;
                }
                visited[half] = true;
                loop_edges.push(ends[half].0);
                let (from, to) = (self.points()[ends[half].0], self.points()[ends[half].1]);
                match half.checked_sub(split) {
                    None => outline.push(from),
                    Some(arc) => outline.extend(arcs[arc].points_along(from, to)),
                }
                let Some(following) = next(half) else { break };
                // Nothing else left this vertex: the walk can only bounce
                // straight back the way it came, which is a spur, not an
                // area. Caught here, structurally, rather than by the shoelace
                // sum landing on exactly zero — which a sampled curve walked
                // out and back is not guaranteed to do, only a straight one is.
                if following == (half ^ 1) {
                    dead_end = true;
                    break;
                }
                half = following;
                if half == start {
                    break;
                }
            }

            let distinct = loop_edges.len() >= 2 && {
                let mut sorted = loop_edges.clone();
                sorted.sort_unstable();
                sorted.dedup();
                sorted.len() == loop_edges.len()
            };
            if !dead_end
                && distinct
                && signed_area(&outline) > 1e-9
                && crate::crossing::is_simple(&outline)
            {
                outlines.push(outline);
            }
        }
        outlines
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::plane::WorkPlane;
    use crate::sketch::Sketch;

    fn area(triangles: &[[DVec2; 3]]) -> f64 {
        triangles
            .iter()
            .map(|[a, b, c]| (*b - *a).perp_dot(*c - *a).abs() * 0.5)
            .sum()
    }

    /// The smallest honest test the issue names: one segment for the flat
    /// side, one half-arc for the round one.
    #[test]
    fn a_d_shape_of_one_segment_and_one_half_arc_closes_and_extrudes() {
        let mut sketch = Sketch::new(WorkPlane::XY);
        let radius = 5.0;
        let bottom = sketch.add_point(DVec2::new(0.0, -radius));
        let top = sketch.add_point(DVec2::new(0.0, radius));
        let center = sketch.add_point(DVec2::ZERO);
        sketch.add_segment(top, bottom);
        // Counter-clockwise from the bottom, through the right, to the top:
        // the bulge sits on the positive side of the flat edge.
        sketch.add_arc(center, bottom, top);

        let regions = sketch.regions();
        assert_eq!(regions.len(), 1, "one segment and one arc close one area");
        assert!(regions[0].holes.is_empty());

        let found = area(&regions[0].triangles);
        let expected = std::f64::consts::PI * radius * radius * 0.5;
        assert!(
            (found - expected).abs() / expected < 0.01,
            "area {found}, expected ~{expected}"
        );
        assert!(
            regions[0].contains(DVec2::new(2.0, 0.0)),
            "inside the bulge"
        );
        assert!(
            !regions[0].contains(DVec2::new(-2.0, 0.0)),
            "the flat side is not rounded outwards too"
        );
    }

    /// The outline emits the sampled curve, not the chord between the arc's
    /// two ends: something has to sit out at the bulge's own peak, which a
    /// shape built from the two ends alone never would.
    #[test]
    fn the_round_side_of_a_d_shape_bulges_out_to_the_curve_not_the_chord() {
        let mut sketch = Sketch::new(WorkPlane::XY);
        let radius = 10.0;
        let bottom = sketch.add_point(DVec2::new(0.0, -radius));
        let top = sketch.add_point(DVec2::new(0.0, radius));
        let center = sketch.add_point(DVec2::ZERO);
        sketch.add_segment(top, bottom);
        sketch.add_arc(center, bottom, top);

        let regions = sketch.regions();
        let peak = DVec2::new(radius, 0.0);
        let nearest = regions[0]
            .outline
            .iter()
            .map(|point| point.distance(peak))
            .fold(f64::MAX, f64::min);
        assert!(
            nearest < 0.1,
            "no sampled point came near the arc's own peak at {peak}: nearest was {nearest} away",
        );
    }

    /// An arc joined to nothing is a dangling spur, the same as a lone
    /// segment — not an area. Far from the origin and swept the long way
    /// round, so the walk samples enough points that a shoelace sum landing
    /// on exactly zero cannot be trusted to catch it: the rejection has to be
    /// structural, not a floating-point coincidence.
    #[test]
    fn a_dangling_arc_with_nothing_attached_encloses_nothing() {
        let mut sketch = Sketch::new(WorkPlane::XY);
        let middle = DVec2::new(6870.0099, -4722.8225);
        let radius = 3000.964;
        let (from_angle, sweep) = (3.2719, 6.0475);
        let centre = sketch.add_point(middle);
        let start = sketch.add_point(middle + DVec2::from_angle(from_angle) * radius);
        let end = sketch.add_point(middle + DVec2::from_angle(from_angle + sweep) * radius);
        sketch.add_arc(centre, start, end);

        assert!(sketch.regions().is_empty());
    }
}
