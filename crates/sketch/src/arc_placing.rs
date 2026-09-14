//! Which arc the clicks gathered so far mean.
//!
//! The sibling of [`crate::circling`] for pieces of a circle: the drawing says
//! what a run of clicks adds up to, and the front-end only has to hand over the
//! places they landed on.

use glam::DVec2;

use crate::arcing::ArcDraft;
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
/// radius or a distance while the second place is being picked, and while the
/// third is, an angle in degrees for `ByCenter` or the curve's own radius for
/// `ByEnds`. Left alone, the cursor decides everything, the way it always has.
/// `scale` is how many millimetres a unit of the drawing is worth, since a
/// typed radius or distance arrives in millimetres.
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
        (ArcMode::ByEnds, 2) => {
            bent_through(places[0], places[1], cursor, locked / scale.max(1e-9)).unwrap_or(cursor)
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

/// The leg a typed angle opens from, as the two places it runs between: the
/// centre, and the end already picked, which is the direction [`aimed`]
/// measures those degrees against. `None` wherever nothing is being read as an
/// angle, so that a canvas asking what to draw is told rather than guessing.
pub fn angle_reference(mode: ArcMode, places: &[DVec2]) -> Option<(DVec2, DVec2)> {
    match (mode, places.len()) {
        (ArcMode::ByCenter, 2) => Some((places[0], places[1])),
        _ => None,
    }
}

/// The place a curve of that radius, hung on those two ends, has to be bent
/// through. The cursor keeps everything the radius does not say: which side of
/// the chord the curve bulges to, and — by standing further off the chord than
/// half of it is long — whether it takes the short way round or the long one.
///
/// `None` when no such curve exists: a radius shorter than half the chord
/// reaches neither end, and a cursor on the chord itself names no side.
fn bent_through(a: DVec2, b: DVec2, cursor: DVec2, radius: f64) -> Option<DVec2> {
    let chord = b - a;
    let half = chord.length() / 2.0;
    let middle = (a + b) / 2.0;
    let across = chord.perp().normalize_or_zero();
    let side = (cursor - middle).dot(across);
    if half < 1e-9 || radius < half || side.abs() < 1e-9 {
        return None;
    }
    let reach = (radius * radius - half * half).sqrt();
    let rise = match side.abs() > half {
        true => radius + reach,
        false => radius - reach,
    };
    Some(middle + across * side.signum() * rise)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::arcing::sweep_of;

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
    fn a_radius_typed_while_bending_a_by_ends_arc_holds_however_near_the_cursor_stays() {
        let (a, b) = (DVec2::new(-30.0, 0.0), DVec2::new(30.0, 0.0));
        let barely_above_the_chord = DVec2::new(0.0, 1.0);

        let bent = aimed(
            ArcMode::ByEnds,
            &[a, b],
            barely_above_the_chord,
            Some(100.0),
            2.0,
        );
        let drawn =
            arc_from(ArcMode::ByEnds, &[a, b], bent).expect("a radius that reaches both ends");

        assert!(
            (drawn.centre.distance(drawn.start) - 50.0).abs() < TOLERANCE,
            "100 mm at a scale of 2 mm per unit is 50 units: the curve came out at {}",
            drawn.centre.distance(drawn.start),
        );
        assert!(
            bent.y > 0.0,
            "the cursor still says which side it bends to: got {bent:?}",
        );
    }

    #[test]
    fn a_cursor_dragged_well_clear_of_the_chord_bends_that_radius_the_long_way_round() {
        let (a, b) = (DVec2::new(-30.0, 0.0), DVec2::new(30.0, 0.0));
        let well_above_the_chord = DVec2::new(0.0, 100.0);

        let bent = aimed(
            ArcMode::ByEnds,
            &[a, b],
            well_above_the_chord,
            Some(50.0),
            1.0,
        );
        let drawn =
            arc_from(ArcMode::ByEnds, &[a, b], bent).expect("a radius that reaches both ends");

        assert!(
            (drawn.centre.distance(drawn.start) - 50.0).abs() < TOLERANCE,
            "the curve came out at {}, not the 50 that was typed",
            drawn.centre.distance(drawn.start),
        );
        assert!(
            sweep_of(drawn) > std::f64::consts::PI,
            "a cursor that far out asks for the major arc: it runs {} radians",
            sweep_of(drawn),
        );
    }

    #[test]
    fn a_radius_too_short_to_reach_both_ends_is_refused_rather_than_stretched_to_fit() {
        let (a, b) = (DVec2::new(-30.0, 0.0), DVec2::new(30.0, 0.0));
        let cursor = DVec2::new(0.0, 10.0);

        assert_eq!(
            aimed(ArcMode::ByEnds, &[a, b], cursor, Some(10.0), 1.0),
            cursor,
            "no curve 10 wide reaches ends 60 apart, so the cursor goes on deciding",
        );
    }

    #[test]
    fn the_leg_an_angle_opens_from_points_where_an_angle_of_zero_would_land() {
        let centre = DVec2::new(4.0, -2.0);
        let start = DVec2::new(4.0, 8.0);

        let (from, to) = angle_reference(ArcMode::ByCenter, &[centre, start])
            .expect("a by-centre arc reads its third click as an angle");
        let no_angle_at_all = aimed(
            ArcMode::ByCenter,
            &[centre, start],
            DVec2::new(-50.0, -50.0),
            Some(0.0),
            1.0,
        );

        assert_eq!(from, centre, "the angle opens from the centre");
        assert!(
            (to - from)
                .normalize()
                .distance((no_angle_at_all - from).normalize())
                < TOLERANCE,
            "the leg runs towards {to:?} while an angle of zero lands at {no_angle_at_all:?}",
        );
    }

    #[test]
    fn an_arc_placed_by_its_ends_measures_no_angle_and_so_opens_from_nothing() {
        let (a, b) = (DVec2::new(-10.0, 0.0), DVec2::new(10.0, 0.0));

        assert_eq!(angle_reference(ArcMode::ByEnds, &[a, b]), None);
        assert_eq!(
            angle_reference(ArcMode::ByCenter, &[a]),
            None,
            "a centre on its own is not yet measuring anything",
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
