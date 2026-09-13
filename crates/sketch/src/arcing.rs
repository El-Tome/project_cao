//! Which arc the clicks gathered so far mean.
//!
//! The sibling of [`crate::circling`] for pieces of a circle: the drawing says
//! what a run of clicks adds up to, and the front-end only has to hand over the
//! places they landed on.

use glam::DVec2;

use crate::construct::circumcentre;

/// How an arc is being drawn.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum ArcMode {
    /// The centre, then where the curve starts, then how far round it runs.
    #[default]
    ByCenter,
    /// The two ends, then a point the curve is bent through.
    ByEnds,
}

impl ArcMode {
    /// How many places it needs before the arc is settled.
    pub fn wants(self) -> usize {
        match self {
            Self::ByCenter | Self::ByEnds => 3,
        }
    }
}

/// An arc as the three places it stands on: the one a tool is part-way through
/// drawing, and the one already in the sketch once its points are looked up.
/// The curve is made of nothing else.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ArcDraft {
    pub centre: DVec2,
    pub start: DVec2,
    pub end: DVec2,
}

/// How far round a curve runs, in radians, always between zero and a full
/// turn: the way round is carried by the order of the two ends, so the answer
/// never needs a sign to say it.
pub fn sweep_of(drawn: ArcDraft) -> f64 {
    let from = (drawn.start - drawn.centre).to_angle();
    let to = (drawn.end - drawn.centre).to_angle();
    (to - from).rem_euclid(std::f64::consts::TAU)
}

/// How finely a whole turn would be cut up. An arc takes its share of it.
const FULL_CIRCLE_STEPS: usize = 48;

/// Into how many straight steps the curve is cut.
///
/// Read from the sweep rather than fixed, so that a small fillet does not
/// become a visible polygon and a long arc does not cost what a whole circle
/// costs.
pub fn steps_along(drawn: ArcDraft) -> usize {
    let turns = sweep_of(drawn) / std::f64::consts::TAU;
    ((turns * FULL_CIRCLE_STEPS as f64).ceil() as usize).max(2)
}

/// The curve as a run of places, ends included.
pub fn places_along(drawn: ArcDraft) -> Vec<DVec2> {
    let radius = drawn.centre.distance(drawn.start);
    let sweep = sweep_of(drawn);
    let from = (drawn.start - drawn.centre).to_angle();
    let steps = steps_along(drawn);
    (0..=steps)
        .map(|step| {
            let angle = from + sweep * step as f64 / steps as f64;
            drawn.centre + DVec2::from_angle(angle) * radius
        })
        .collect()
}

/// The arc the clicks so far and the cursor make, if they make one.
///
/// The cursor gives a direction, never a distance: the far end is brought back
/// onto the circle the near one already fixed, so what is drawn is a piece of a
/// circle at every moment rather than only once it is recorded.
pub fn arc_from(mode: ArcMode, places: &[DVec2], cursor: DVec2) -> Option<ArcDraft> {
    match mode {
        ArcMode::ByCenter => {
            let (centre, start) = (*places.first()?, *places.get(1)?);
            let radius = centre.distance(start);
            let reach = cursor - centre;
            if radius < 1e-9 || reach.length() < 1e-9 {
                return None;
            }
            let end = centre + DVec2::from_angle(reach.to_angle()) * radius;
            (end.distance(start) > 1e-6).then_some(ArcDraft { centre, start, end })
        }
        ArcMode::ByEnds => {
            let (a, b) = (*places.first()?, *places.get(1)?);
            let centre = circumcentre(a, b, cursor)?;
            // The walk always turns counter-clockwise, so the two ends are
            // handed over in whichever order makes that turn pass through
            // the point the curve was bent to, rather than the long way round.
            let angle_of = |point: DVec2| (point - centre).to_angle();
            let (from, through, to) = (angle_of(a), angle_of(cursor), angle_of(b));
            let (start, end) = match (through - from).rem_euclid(std::f64::consts::TAU)
                < (to - from).rem_euclid(std::f64::consts::TAU)
            {
                true => (a, b),
                false => (b, a),
            };
            Some(ArcDraft { centre, start, end })
        }
    }
}

/// The cursor, once a value typed into the live field has had its say: a
/// radius or a distance while the second place is being picked, an angle in
/// degrees while the third is — the latter only for `ByCenter`, since a
/// `ByEnds` arc is bent to where the cursor points, not to an angle. Left
/// alone, the cursor decides everything, the way it always has. `scale` is
/// how many millimetres a unit of the drawing is worth, since a typed radius
/// or distance arrives in millimetres.
pub fn aimed(
    mode: ArcMode,
    places: &[DVec2],
    cursor: DVec2,
    locked: Option<f64>,
    scale: f64,
) -> DVec2 {
    let Some(locked) = locked else {
        return cursor;
    };
    match (mode, places.len()) {
        (ArcMode::ByCenter, 2) => {
            let (centre, start) = (places[0], places[1]);
            let radius = centre.distance(start);
            if radius < 1e-9 {
                return cursor;
            }
            let base = (start - centre).to_angle();
            centre + DVec2::from_angle(base + locked.to_radians()) * radius
        }
        (ArcMode::ByCenter, 1) | (ArcMode::ByEnds, 1) => {
            let radius = locked / scale.max(1e-9);
            match radius > 1e-9 {
                true => places[0] + (cursor - places[0]).normalize_or(DVec2::X) * radius,
                false => cursor,
            }
        }
        _ => cursor,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TOLERANCE: f64 = 1e-9;

    #[test]
    fn the_cursor_says_how_far_round_the_curve_runs_and_never_how_wide_it_is() {
        let centre = DVec2::ZERO;
        let start = DVec2::new(10.0, 0.0);
        let far_off_to_the_north = DVec2::new(0.0, 400.0);

        let drawn = arc_from(ArcMode::ByCenter, &[centre, start], far_off_to_the_north)
            .expect("a centre, a first end and a direction make an arc");

        assert_eq!(drawn.start, start);
        assert!(
            (drawn.end.distance(DVec2::new(0.0, 10.0))) < TOLERANCE,
            "the far end landed at {:?}, off the circle the near one fixed",
            drawn.end,
        );
    }

    #[test]
    fn a_curve_running_nowhere_is_no_arc() {
        let centre = DVec2::ZERO;
        let start = DVec2::new(10.0, 0.0);

        assert_eq!(
            arc_from(ArcMode::ByCenter, &[centre, start], start),
            None,
            "the cursor is back where the curve starts, so there is no curve",
        );
        assert_eq!(
            arc_from(ArcMode::ByCenter, &[centre, centre], DVec2::new(0.0, 5.0)),
            None,
            "a curve no distance from its centre is a point",
        );
        assert_eq!(
            arc_from(ArcMode::ByCenter, &[centre], DVec2::new(0.0, 5.0)),
            None,
            "the centre alone says nothing about how wide the curve is",
        );
    }

    #[test]
    fn an_arc_by_its_ends_passes_through_the_point_it_was_bent_to() {
        let (a, b) = (DVec2::new(-10.0, 0.0), DVec2::new(10.0, 0.0));
        let bent_up = DVec2::new(0.0, 10.0);

        let drawn = arc_from(ArcMode::ByEnds, &[a, b], bent_up)
            .expect("two ends and a point between them make an arc");

        // The walk always turns counter-clockwise, so passing through the top
        // on the way from one end to the other starts at the right-hand one.
        assert_eq!(drawn.start, b);
        assert_eq!(drawn.end, a);
        let radius = drawn.centre.distance(a);
        assert!(
            (drawn.centre.distance(bent_up) - radius).abs() < TOLERANCE,
            "the curve does not run through the point it was bent to: centre = {:?}",
            drawn.centre,
        );
    }

    #[test]
    fn an_arc_by_its_ends_bent_the_other_way_swaps_which_end_it_starts_from() {
        let (a, b) = (DVec2::new(-10.0, 0.0), DVec2::new(10.0, 0.0));
        let bent_down = DVec2::new(0.0, -10.0);

        let drawn = arc_from(ArcMode::ByEnds, &[a, b], bent_down)
            .expect("two ends and a point between them make an arc");

        assert_eq!(
            drawn.start, a,
            "the walk still turns counter-clockwise, so bending the other way starts from the other end",
        );
        assert_eq!(drawn.end, b);
    }

    #[test]
    fn three_points_on_a_line_make_no_arc_by_its_ends() {
        let (a, b) = (DVec2::new(0.0, 0.0), DVec2::new(20.0, 0.0));

        assert_eq!(
            arc_from(ArcMode::ByEnds, &[a, b], DVec2::new(10.0, 0.0)),
            None,
            "a point between the ends and in line with them bends no curve",
        );
    }

    #[test]
    fn a_radius_typed_while_placing_the_second_place_holds_however_the_cursor_turns() {
        let centre = DVec2::ZERO;
        let cursor = DVec2::new(3.0, 4.0);

        let placed = aimed(ArcMode::ByCenter, &[centre], cursor, Some(100.0), 2.0);

        assert!(
            (centre.distance(placed) - 50.0).abs() < TOLERANCE,
            "100 mm at a scale of 2 mm per unit is 50 units: got {}",
            centre.distance(placed),
        );
        assert!(
            (placed - centre)
                .normalize()
                .distance((cursor - centre).normalize())
                < TOLERANCE,
            "the direction still follows the cursor: got {placed:?}",
        );
    }

    #[test]
    fn a_distance_typed_while_placing_the_second_end_of_a_by_ends_arc_holds_too() {
        let first_end = DVec2::new(5.0, 0.0);
        let cursor = DVec2::new(5.0, 100.0);

        let placed = aimed(ArcMode::ByEnds, &[first_end], cursor, Some(30.0), 1.0);

        assert!((first_end.distance(placed) - 30.0).abs() < TOLERANCE);
    }

    #[test]
    fn an_angle_typed_while_placing_the_third_place_turns_the_start_direction_by_that_many_degrees()
    {
        let centre = DVec2::ZERO;
        let start = DVec2::new(10.0, 0.0);

        let placed = aimed(
            ArcMode::ByCenter,
            &[centre, start],
            DVec2::new(-1.0, -1.0),
            Some(90.0),
            1.0,
        );

        assert!(
            placed.distance(DVec2::new(0.0, 10.0)) < TOLERANCE,
            "90 degrees from the start turns straight up regardless of where the cursor is: got {placed:?}",
        );
    }

    #[test]
    fn a_by_ends_arc_has_nothing_to_type_while_placing_the_point_it_is_bent_through() {
        let (a, b) = (DVec2::new(-10.0, 0.0), DVec2::new(10.0, 0.0));
        let cursor = DVec2::new(0.0, 5.0);

        assert_eq!(
            aimed(ArcMode::ByEnds, &[a, b], cursor, Some(42.0), 1.0),
            cursor,
            "a by-ends arc is bent to where the cursor points, not to a typed value",
        );
    }

    #[test]
    fn leaving_the_field_empty_leaves_the_cursor_deciding_everything() {
        let cursor = DVec2::new(7.0, 8.0);

        assert_eq!(
            aimed(ArcMode::ByCenter, &[DVec2::ZERO], cursor, None, 1.0),
            cursor
        );
    }
}
