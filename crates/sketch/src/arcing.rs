//! Which arc the clicks gathered so far mean.
//!
//! The sibling of [`crate::circling`] for pieces of a circle: the drawing says
//! what a run of clicks adds up to, and the front-end only has to hand over the
//! places they landed on.

use glam::DVec2;

/// How an arc is being drawn.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum ArcMode {
    /// The centre, then where the curve starts, then how far round it runs.
    #[default]
    ByCenter,
}

impl ArcMode {
    /// How many places it needs before the arc is settled.
    pub fn wants(self) -> usize {
        match self {
            Self::ByCenter => 3,
        }
    }
}

/// An arc about to be drawn, as the three places it stands on.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ArcDraft {
    pub centre: DVec2,
    pub start: DVec2,
    pub end: DVec2,
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
}
