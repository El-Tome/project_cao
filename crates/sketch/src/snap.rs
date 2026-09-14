//! What pulls the cursor, and how hard.
//!
//! Aiming with a mouse is never exact, so the drawing pulls the cursor onto
//! what it is nearly on. Which of those magnets wins is a rule about the
//! drawing — a point already there is worth more than a line, and a line more
//! than the grid behind it — and it belongs with the drawing rather than with
//! the interface showing it. How far each one reaches is the other way round:
//! the interface measures that in pixels and hands it over in world units.

use glam::DVec2;

use crate::sketch::Sketch;

/// How far each magnet reaches, in the drawing's own units.
///
/// `grid_step` says both whether the grid pulls at all and how coarsely: no
/// step, no grid magnet.
#[derive(Clone, Copy, Debug)]
pub struct SnapSettings {
    pub point_reach: f64,
    pub segment_reach: f64,
    pub grid_step: Option<f64>,
    pub grid_reach: f64,
}

/// What the cursor has been pulled onto, when it is worth saying so.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Snap {
    Point,
    /// Where two curves run through the same place without the drawing having
    /// a point there. It carries its position for the same reason a midpoint
    /// does: at a shallow angle nothing on screen tells a crossing from a near
    /// miss.
    Crossing(DVec2),
    /// The middle of a line, which needs a mark of its own: nothing else on
    /// screen says the cursor is exactly halfway along.
    Midpoint(DVec2),
    OnSegment(DVec2),
}

impl Sketch {
    /// Pulls the cursor onto whatever it is near: an existing point first, then
    /// a crossing, then the middle of a trait, then the trait itself, then the
    /// grid.
    ///
    /// The grid magnet is what makes drawing on the origin, or a right angle by
    /// following the lines, a matter of aiming roughly rather than exactly. It
    /// only bites within its own reach, so a deliberate free position is still
    /// possible.
    pub fn magnetise(&self, cursor: DVec2, settings: &SnapSettings) -> (DVec2, Option<Snap>) {
        if let Some(point) = self.nearest_point(cursor, settings.point_reach) {
            return (self.point(point), Some(Snap::Point));
        }

        // A crossing ranks under a drawn point and over a midpoint: the drawing
        // names the one and merely happens to make the other, and between the
        // two of them the crossing is the place a person was aiming at.
        if let Some(at) = self.nearest_crossing(cursor, settings.point_reach) {
            return (at, Some(Snap::Crossing(at)));
        }

        // A line already drawn pulls harder than the grid, and its middle harder
        // still: joining the middle of a side is a thing one aims at, and
        // landing a hair off it leaves geometry that only looks joined.
        if let Some((_, middle)) = self.nearest_midpoint(cursor, settings.segment_reach) {
            return (middle, Some(Snap::Midpoint(middle)));
        }
        if let Some((_, at)) = self.nearest_on_segment(cursor, settings.segment_reach) {
            return (at, Some(Snap::OnSegment(at)));
        }

        let Some(step) = settings.grid_step.filter(|step| *step > 0.0) else {
            return (cursor, None);
        };
        let snapped = DVec2::new(
            (cursor.x / step).round() * step,
            (cursor.y / step).round() * step,
        );
        match snapped.distance(cursor) <= settings.grid_reach {
            true => (snapped, None),
            false => (cursor, None),
        }
    }

    fn nearest_crossing(&self, cursor: DVec2, reach: f64) -> Option<DVec2> {
        self.crossings()
            .into_iter()
            .filter(|at| at.distance(cursor) <= reach)
            .min_by(|left, right| {
                left.distance_squared(cursor)
                    .total_cmp(&right.distance_squared(cursor))
            })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::WorkPlane;

    fn settings() -> SnapSettings {
        SnapSettings {
            point_reach: 1.0,
            segment_reach: 1.0,
            grid_step: Some(10.0),
            grid_reach: 2.0,
        }
    }

    fn with_two_crossing_traits() -> Sketch {
        let mut sketch = with_a_trait(DVec2::new(0.0, 0.0), DVec2::new(10.0, 0.0));
        let from = sketch.add_point(DVec2::new(4.0, -2.0));
        let to = sketch.add_point(DVec2::new(4.0, 6.0));
        sketch.add_segment(from, to);
        sketch
    }

    /// The two traits above cross here, and no point of the drawing stands on
    /// it — which is what makes it a crossing rather than a point.
    const WHERE_THEY_CROSS: DVec2 = DVec2::new(4.0, 0.0);

    const TOLERANCE: f64 = 1e-9;

    fn caught_the_crossing(at: DVec2, caught: Option<Snap>) {
        assert!(
            matches!(caught, Some(Snap::Crossing(_))),
            "the crossing at {WHERE_THEY_CROSS} was there to be caught, and the cursor met {caught:?}",
        );
        assert!(
            at.distance(WHERE_THEY_CROSS) < TOLERANCE,
            "and it is pulled onto {WHERE_THEY_CROSS}, not to {at}",
        );
    }

    #[test]
    fn the_cursor_is_pulled_onto_the_place_two_traits_cross() {
        let sketch = with_two_crossing_traits();

        let (at, caught) = sketch.magnetise(DVec2::new(4.2, 0.15), &settings());

        caught_the_crossing(at, caught);
    }

    #[test]
    fn a_crossing_holds_the_cursor_against_a_midpoint_that_is_nearer() {
        let sketch = with_two_crossing_traits();

        let (at, caught) = sketch.magnetise(DVec2::new(4.8, 0.1), &settings());

        caught_the_crossing(at, caught);
    }

    #[test]
    fn a_crossing_the_drawing_already_has_a_point_on_is_that_point() {
        let mut sketch = with_two_crossing_traits();
        sketch.add_point(WHERE_THEY_CROSS);

        assert!(
            sketch.crossings().is_empty(),
            "the place is named, so there is nothing left to invent: {:?}",
            sketch.crossings(),
        );

        let (at, caught) = sketch.magnetise(DVec2::new(4.2, 0.15), &settings());

        assert_eq!(caught, Some(Snap::Point));
        assert!(at.distance(WHERE_THEY_CROSS) < TOLERANCE, "got {at}");
    }

    fn with_a_trait(start: DVec2, end: DVec2) -> Sketch {
        let mut sketch = Sketch::new(WorkPlane::XY);
        let from = sketch.add_point(start);
        let to = sketch.add_point(end);
        sketch.add_segment(from, to);
        sketch
    }

    #[test]
    fn an_existing_point_wins_over_the_grid() {
        let mut sketch = Sketch::new(WorkPlane::XY);
        sketch.add_point(DVec2::new(10.4, 0.0));

        let (at, caught) = sketch.magnetise(DVec2::new(10.0, 0.0), &settings());

        assert_eq!(caught, Some(Snap::Point));
        assert_eq!(
            at,
            DVec2::new(10.4, 0.0),
            "the grid line is right there, and the point still wins",
        );
    }

    #[test]
    fn the_middle_of_a_trait_pulls_harder_than_its_body() {
        let sketch = with_a_trait(DVec2::new(0.0, 40.0), DVec2::new(40.0, 40.0));

        let (at, caught) = sketch.magnetise(DVec2::new(20.3, 40.2), &settings());

        assert_eq!(caught, Some(Snap::Midpoint(DVec2::new(20.0, 40.0))));
        assert_eq!(at, DVec2::new(20.0, 40.0));
    }

    #[test]
    fn a_cursor_along_a_trait_but_away_from_its_middle_lands_on_the_body() {
        let sketch = with_a_trait(DVec2::new(0.0, 40.0), DVec2::new(40.0, 40.0));

        let (at, caught) = sketch.magnetise(DVec2::new(33.0, 40.2), &settings());

        assert_eq!(caught, Some(Snap::OnSegment(DVec2::new(33.0, 40.0))));
        assert_eq!(at, DVec2::new(33.0, 40.0));
    }

    #[test]
    fn a_cursor_near_a_grid_line_is_pulled_onto_it_without_a_mark() {
        let sketch = Sketch::new(WorkPlane::XY);

        let (at, caught) = sketch.magnetise(DVec2::new(21.0, 39.0), &settings());

        assert_eq!(at, DVec2::new(20.0, 40.0));
        assert_eq!(caught, None, "the grid needs no mark: it is already drawn");
    }

    #[test]
    fn a_cursor_too_far_from_the_grid_is_left_where_it_is() {
        let sketch = Sketch::new(WorkPlane::XY);
        let cursor = DVec2::new(25.0, 35.0);

        assert_eq!(sketch.magnetise(cursor, &settings()), (cursor, None));
    }

    #[test]
    fn snapping_is_off_when_no_grid_step_is_given() {
        let sketch = Sketch::new(WorkPlane::XY);
        let cursor = DVec2::new(21.0, 39.0);
        let loose = SnapSettings {
            grid_step: None,
            ..settings()
        };

        assert_eq!(sketch.magnetise(cursor, &loose), (cursor, None));
    }
}
