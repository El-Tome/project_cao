use glam::DVec2;

use crate::edges::Crossed;
use crate::regions::signed_area;
use crate::sketch::Sketch;

impl Sketch {
    /// Walks the segment and arc graph and returns each area it encloses, as
    /// a loop of positions turning counter-clockwise.
    /// Walks the segment and arc graph and returns each area it encloses, as
    /// a loop of positions turning counter-clockwise.
    pub(crate) fn closed_outlines(&self) -> Vec<Vec<DVec2>> {
        let Crossed {
            places,
            ends,
            split,
            arcs,
        } = self.crossed();
        if ends.is_empty() {
            return Vec::new();
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
                let (from, to) = (places[ends[half].0], places[ends[half].1]);
                match half.checked_sub(split) {
                    None => outline.push(from),
                    Some(arc) => outline.extend(arcs[arc].points_along(from, to)),
                }
                let Some(following) = next(half) else { break };
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
            if !dead_end && distinct && signed_area(&outline) > 1e-9 {
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
