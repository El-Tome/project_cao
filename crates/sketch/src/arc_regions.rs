use glam::DVec2;

use crate::edges::Crossed;
use crate::regions::signed_area;
use crate::sketch::Sketch;

impl Sketch {
    /// Walks the segment and arc graph and returns each area it encloses, as
    /// a loop of positions turning counter-clockwise.
    ///
    /// A circle nothing cuts never reaches the graph, and comes back from
    /// `crossed` as the closed loop it already is.
    pub(crate) fn closed_outlines(&self) -> Vec<Vec<DVec2>> {
        let Crossed {
            places,
            ends,
            split,
            arcs,
            whole,
        } = self.crossed();
        let mut outlines = whole;
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

            let mut outline = Vec::new();
            for half in without_spurs(&walked) {
                let (from, to) = (places[ends[half].0], places[ends[half].1]);
                match half.checked_sub(split) {
                    None => outline.push(from),
                    Some(arc) => outline.extend(arcs[arc].points_along(from, to)),
                }
            }

            // Turning the other way round the same edges walks the outside of
            // the drawing, which is not an area: only the face the walk keeps
            // on its left has a positive signed area. A face pinched at a
            // point — a bowtie's crossing, a point dropped on a trait — walks
            // that point twice, quite correctly, so nothing here may ask for
            // the corners to be distinct.
            if signed_area(&outline) > 1e-9 {
                outlines.push(outline);
            }
        }
        outlines
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

    fn a_closed_shape() -> (Sketch, Vec<crate::sketch::PointId>) {
        let mut sketch = Sketch::new(WorkPlane::XY);
        let corners: Vec<crate::sketch::PointId> = [
            (0.0, 0.0),
            (70.0, 25.0),
            (57.7, 92.7),
            (25.0, 140.0),
            (-95.0, 125.0),
            (10.0, 60.0),
            (8.1, 33.1),
        ]
        .iter()
        .map(|(x, y)| sketch.add_point(DVec2::new(*x, *y)))
        .collect();
        for rank in 0..corners.len() {
            sketch.add_segment(corners[rank], corners[(rank + 1) % corners.len()]);
        }
        (sketch, corners)
    }

    fn laid(sketch: &mut Sketch, from: DVec2, to: DVec2) {
        let a = sketch.add_point(from);
        let b = sketch.add_point(to);
        sketch.add_segment(a, b);
    }

    #[test]
    fn a_trait_poking_into_a_shape_leaves_its_area_whole() {
        let (mut sketch, _) = a_closed_shape();
        let alone = sketch.regions();
        assert_eq!(alone.len(), 1);
        let whole = area(&alone[0].triangles);

        laid(&mut sketch, DVec2::new(40.0, 90.0), DVec2::new(60.0, 125.0));

        let regions = sketch.regions();
        assert_eq!(
            regions.len(),
            1,
            "a trait that encloses nothing encloses nothing: it has one end \
             inside the shape and one outside, so it cuts no second area out \
             of it",
        );
        assert_eq!(
            regions[0].outline.len(),
            8,
            "the shape's seven corners, and the place the trait crosses its side",
        );
        assert!(
            (area(&regions[0].triangles) - whole).abs() < 1e-6,
            "the area was {whole} before the trait was laid and {} after",
            area(&regions[0].triangles),
        );
    }

    #[test]
    fn a_corner_dropped_exactly_on_a_trait_leaves_the_area_it_pinches() {
        let mut sketch = Sketch::new(WorkPlane::XY);
        let corners: Vec<crate::sketch::PointId> = [
            (25.0, 85.0),
            (37.5, 115.0),
            (20.0, 145.0),
            (-35.0, 130.0),
            (-50.0, 170.0),
            (-60.0, 90.0),
            (-105.0, 50.0),
            (-35.0, -10.0),
            (-40.0, 65.0),
            (-20.0, 40.0),
        ]
        .iter()
        .map(|(x, y)| sketch.add_point(DVec2::new(*x, *y)))
        .collect();
        for rank in 0..corners.len() {
            sketch.add_segment(corners[rank], corners[(rank + 1) % corners.len()]);
        }

        // A whisker off the trait that runs from (-35, 130) to (-50, 170).
        sketch.move_point(corners[2], DVec2::new(-42.499, 150.0));
        let beside = area(&sketch.regions()[0].triangles);

        // And exactly on it, where the face pinches and walks that corner twice.
        sketch.move_point(corners[2], DVec2::new(-42.5, 150.0));

        let regions = sketch.regions();
        assert_eq!(
            regions.len(),
            1,
            "a corner laid on a trait pinches the area at that corner; it does \
             not take it away",
        );
        assert!(
            (area(&regions[0].triangles) - beside).abs() < 1.0,
            "the area was {beside} a thousandth away from the trait and {} on \
             it, where it should barely have moved",
            area(&regions[0].triangles),
        );
    }

    #[test]
    fn a_trait_laid_right_across_a_shape_still_cuts_it_in_two() {
        let (mut sketch, _) = a_closed_shape();
        let whole = area(&sketch.regions()[0].triangles);

        laid(
            &mut sketch,
            DVec2::new(-60.0, 130.0),
            DVec2::new(60.0, 125.0),
        );

        let regions = sketch.regions();
        assert_eq!(
            regions.len(),
            2,
            "both ends outside, so it goes clean through"
        );
        let cut: f64 = regions.iter().map(|region| area(&region.triangles)).sum();
        assert!(
            (cut - whole).abs() < 1e-6,
            "the two areas together are the one they were cut from: {cut} against {whole}",
        );
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
