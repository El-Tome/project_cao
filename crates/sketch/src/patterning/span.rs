//! How wide what is held stands, whichever way it is measured.
//!
//! What a rectangular pattern's step opens on, so that the fields appear on a
//! number worth correcting rather than on nothing at all.

use glam::DVec2;

use crate::element::Element;
use crate::sketch::Sketch;

use super::NO_STEP;

impl Sketch {
    /// How wide what is held stands, measured whichever way gives the widest
    /// answer.
    ///
    /// A pattern's step opens on this: the click naming the direction has not
    /// happened yet, so the extent along it cannot be read, and the widest way
    /// round is the one number no direction can make too small.
    ///
    /// Nothing when what is held stands all in one place — a lone point has no
    /// width to go on.
    pub fn widest_span(&self, of: &[Element]) -> Option<f64> {
        let reach: Vec<(DVec2, f64)> = of.iter().flat_map(|held| self.reach_of(*held)).collect();
        let mut widest = 0.0_f64;
        for (here, out) in &reach {
            for (there, back) in &reach {
                widest = widest.max(here.distance(*there) + out + back);
            }
        }
        (widest > NO_STEP).then_some(widest)
    }

    /// Every place one piece of the drawing stands on, and how far it reaches
    /// out from there.
    fn reach_of(&self, held: Element) -> Vec<(DVec2, f64)> {
        match held {
            Element::Circle(id) => match self.circles().get(id.0) {
                Some(round) => vec![(self.point(round.center), round.radius)],
                None => Vec::new(),
            },
            Element::Arc(id) => match self.arcs().get(id.0) {
                Some(curve) => vec![(self.point(curve.center), self.arc_radius(id))],
                None => Vec::new(),
            },
            other => self
                .points_it_leans_on(other)
                .into_iter()
                .map(|point| (self.point(point), 0.0))
                .collect(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::plane::WorkPlane;

    use super::*;

    const TOLERANCE: f64 = 1e-9;

    #[test]
    fn a_lone_circle_stands_as_wide_as_it_is_across() {
        let mut sketch = Sketch::new(WorkPlane::XY);
        let centre = sketch.add_point(DVec2::new(1.0, 1.0));
        let round = Element::Circle(sketch.add_circle(centre, 3.0));

        let span = sketch.widest_span(&[round]).expect("a circle has a width");

        assert!(
            (span - 6.0).abs() <= TOLERANCE,
            "a circle of radius 3 spans {span}, wanted 6"
        );
    }

    #[test]
    fn an_arc_stands_as_wide_as_the_circle_it_is_cut_from() {
        let mut sketch = Sketch::new(WorkPlane::XY);
        let centre = sketch.add_point(DVec2::ZERO);
        let start = sketch.add_point(DVec2::new(5.0, 0.0));
        let end = sketch.add_point(DVec2::new(0.0, 5.0));
        let curve = Element::Arc(sketch.add_arc(centre, start, end));

        let span = sketch.widest_span(&[curve]).expect("an arc has a width");

        assert!(
            (span - 10.0).abs() <= TOLERANCE,
            "a quarter arc of radius 5 spans {span}, wanted 10"
        );
    }

    #[test]
    fn a_square_stands_as_wide_as_its_diagonal() {
        let mut sketch = Sketch::new(WorkPlane::XY);
        let corners: Vec<_> = [(0.0, 0.0), (4.0, 0.0), (4.0, 4.0), (0.0, 4.0)]
            .map(|(x, y)| sketch.add_point(DVec2::new(x, y)))
            .to_vec();
        let sides: Vec<Element> = (0..4)
            .map(|corner| {
                Element::Segment(sketch.add_segment(corners[corner], corners[(corner + 1) % 4]))
            })
            .collect();

        let span = sketch.widest_span(&sides).expect("a square has a width");

        assert!(
            (span - 32.0_f64.sqrt()).abs() <= TOLERANCE,
            "a square of side 4 spans {span}, wanted its diagonal: no direction it is later \
             stepped along can make that too small"
        );
    }

    #[test]
    fn a_lone_point_has_no_width_to_open_a_step_on() {
        let mut sketch = Sketch::new(WorkPlane::XY);
        let alone = Element::Point(sketch.add_point(DVec2::new(2.0, 2.0)));

        assert_eq!(sketch.widest_span(&[alone]), None);
        assert_eq!(sketch.widest_span(&[]), None, "nothing held stands nowhere");
    }
}
