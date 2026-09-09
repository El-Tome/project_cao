//! Which circle the clicks gathered so far mean.
//!
//! One step above `construct`: that module answers "where does such a centre
//! land", this one chooses which of those questions the user is asking, and
//! what the tool has been given to answer it with.

use glam::DVec2;

use crate::construct::{
    CircleMode, Line, centre_through, centre_through_at, centre_touching_two,
    circle_touching_three, resize_touching,
};
use crate::sketch::{SegmentId, Sketch};

/// A circle about to be drawn.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Found {
    pub centre: DVec2,
    pub radius: f64,
}

/// The circle the clicks so far and the cursor make, if they make one.
///
/// `diameter` is what the user typed, in millimetres, and `scale` how many of
/// them a unit of the drawing is worth. The same reading serves the preview and
/// the click, so what is shown is what gets drawn.
pub fn circle_from(
    mode: CircleMode,
    places: &[DVec2],
    lines: &[Line],
    cursor: DVec2,
    diameter: Option<f64>,
    scale: f64,
) -> Option<Found> {
    let wanted = diameter.map(|value| value / (2.0 * scale.max(1e-9)));

    let found = match mode {
        CircleMode::Center => {
            let centre = *places.first()?;
            Found {
                centre,
                radius: centre.distance(cursor),
            }
        }
        CircleMode::TwoPoints => {
            let first = *places.first()?;
            // A size typed pushes the far point out along the same direction.
            let far = match wanted {
                Some(radius) => first + (cursor - first).normalize_or(DVec2::X) * radius * 2.0,
                None => cursor,
            };
            Found {
                centre: (first + far) * 0.5,
                radius: first.distance(far) * 0.5,
            }
        }
        CircleMode::ThreePoints => {
            let (first, second) = (*places.first()?, *places.get(1)?);
            let centre = match wanted {
                // A size too small to reach both points is held at the smallest
                // that does. Typing 150 goes through 1 and 15 on the way, and a
                // circle that vanishes at the first keystroke takes the field
                // being typed into with it.
                Some(radius) => centre_through_at(
                    first,
                    second,
                    cursor,
                    radius.max(first.distance(second) * 0.5),
                )?,
                None => centre_through(first, second, cursor)?,
            };
            Found {
                centre,
                radius: centre.distance(first),
            }
        }
        CircleMode::TwoTangents => {
            let mut touching = centre_touching_two(*lines.first()?, *lines.get(1)?, cursor)?;
            if let Some(radius) = wanted {
                touching = resize_touching(touching, radius);
            }
            Found {
                centre: touching.centre,
                radius: touching.radius,
            }
        }
        CircleMode::ThreeTangents => {
            let (centre, radius) =
                circle_touching_three(*lines.first()?, *lines.get(1)?, *lines.get(2)?)?;
            Found { centre, radius }
        }
    };
    (found.radius > 1e-9).then_some(found)
}

/// Which of the places clicked stay as points of the drawing, held on the rim.
///
/// They are what the circle can afterwards be grabbed and measured by. A circle
/// drawn against traits keeps none: what holds it is the tangencies.
pub fn rim_of(mode: CircleMode, places: &[DVec2], cursor: DVec2, centre: DVec2) -> Vec<DVec2> {
    let kept = match mode {
        CircleMode::Center => vec![cursor],
        CircleMode::TwoPoints => match places.first() {
            Some(first) => vec![*first, centre * 2.0 - *first],
            None => Vec::new(),
        },
        CircleMode::ThreePoints => places.to_vec(),
        CircleMode::TwoTangents | CircleMode::ThreeTangents => Vec::new(),
    };
    kept.into_iter()
        .filter(|place| place.distance(centre) > 1e-6)
        .collect()
}

/// What one click of the circle tool does, before enough has been picked to
/// know which circle is meant.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CircleProgress {
    /// A construction touching traits still needs one, and none is under the
    /// cursor.
    NeedsSegment,
    /// The trait under the cursor is one already picked.
    AlreadyPicked,
    /// One more point is enough to remember; the circle still waits.
    AddPoint(DVec2),
    /// One more trait is enough to remember; the circle still waits.
    AddSegment(SegmentId),
    /// Enough has been picked for the click to settle what circle is meant.
    Ready,
}

/// Whether a click adds to what a circle needs, or is enough to draw it.
///
/// The order mirrors what each construction needs: a mode built from traits
/// asks for one before it looks at points, since that is the only thing it
/// can ever ask for.
pub fn circle_progress(
    mode: CircleMode,
    points: &[DVec2],
    segments: &[SegmentId],
    sketch: &Sketch,
    cursor: DVec2,
    snap: f64,
) -> CircleProgress {
    if mode.touches_traits() && segments.len() < mode.wants() - 1 {
        return match sketch.nearest_segment(cursor, snap) {
            None => CircleProgress::NeedsSegment,
            Some(segment) if segments.contains(&segment) => CircleProgress::AlreadyPicked,
            Some(segment) => CircleProgress::AddSegment(segment),
        };
    }
    if !mode.touches_traits() && points.len() < mode.wants() - 1 {
        return CircleProgress::AddPoint(cursor);
    }
    CircleProgress::Ready
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::plane::WorkPlane;

    #[test]
    fn a_three_point_circle_waits_for_two_places_before_it_draws() {
        let sketch = Sketch::new(WorkPlane::XY);
        let cursor = DVec2::new(10.0, 10.0);

        assert_eq!(
            circle_progress(CircleMode::ThreePoints, &[], &[], &sketch, cursor, 1.0),
            CircleProgress::AddPoint(cursor),
            "the first of three places only waits for the next",
        );
        assert_eq!(
            circle_progress(
                CircleMode::ThreePoints,
                &[DVec2::new(0.0, 0.0)],
                &[],
                &sketch,
                cursor,
                1.0,
            ),
            CircleProgress::AddPoint(cursor),
            "the second of three places still only waits",
        );
        assert_eq!(
            circle_progress(
                CircleMode::ThreePoints,
                &[DVec2::new(0.0, 0.0), DVec2::new(20.0, 0.0)],
                &[],
                &sketch,
                cursor,
                1.0,
            ),
            CircleProgress::Ready,
            "the third click has enough to settle a circle",
        );
    }

    #[test]
    fn a_construction_built_from_traits_asks_for_one_when_none_is_under_the_cursor() {
        let sketch = Sketch::new(WorkPlane::XY);

        assert_eq!(
            circle_progress(
                CircleMode::ThreeTangents,
                &[],
                &[],
                &sketch,
                DVec2::new(10.0, 10.0),
                1.0,
            ),
            CircleProgress::NeedsSegment,
        );
    }

    #[test]
    fn a_trait_already_picked_is_told_apart_from_one_freshly_found() {
        let mut sketch = Sketch::new(WorkPlane::XY);
        let a = sketch.add_point(DVec2::new(0.0, 0.0));
        let b = sketch.add_point(DVec2::new(20.0, 0.0));
        let segment = sketch.add_segment(a, b);
        let midpoint = DVec2::new(10.0, 0.0);

        assert_eq!(
            circle_progress(
                CircleMode::TwoTangents,
                &[],
                &[segment],
                &sketch,
                midpoint,
                1.0
            ),
            CircleProgress::AlreadyPicked,
        );
    }

    #[test]
    fn a_two_point_construction_is_ready_as_soon_as_it_has_its_first_point() {
        let sketch = Sketch::new(WorkPlane::XY);

        assert_eq!(
            circle_progress(
                CircleMode::TwoPoints,
                &[DVec2::new(0.0, 0.0)],
                &[],
                &sketch,
                DVec2::new(10.0, 10.0),
                1.0,
            ),
            CircleProgress::Ready,
        );
    }

    #[test]
    fn a_diameter_too_small_to_reach_both_points_is_held_at_the_smallest_that_does() {
        let (first, second) = (DVec2::new(-50.0, 0.0), DVec2::new(50.0, 0.0));
        let places = [first, second];

        let squeezed = circle_from(
            CircleMode::ThreePoints,
            &places,
            &[],
            DVec2::new(0.0, 10.0),
            Some(20.0),
            1.0,
        )
        .expect("two points and a cursor make a circle");

        assert!(
            (squeezed.radius - 50.0).abs() < 1e-9,
            "typing 150 goes through 1 and 15 on the way, and a circle that \
             vanishes at the first keystroke takes the field with it: got {}",
            squeezed.radius,
        );

        let roomy = circle_from(
            CircleMode::ThreePoints,
            &places,
            &[],
            DVec2::new(0.0, 10.0),
            Some(200.0),
            1.0,
        )
        .expect("two points and a cursor make a circle");

        assert!(
            (roomy.radius - 100.0).abs() < 1e-9,
            "a size that does reach both points is taken as typed: got {}",
            roomy.radius,
        );
    }
}
