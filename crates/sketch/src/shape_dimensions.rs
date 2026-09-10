//! The dimensions a freshly-drawn shape earns on its own, before the user
//! types anything.

use crate::aim::LockedInput;
use crate::constraints::{Constraint, DimensionTarget, SketchAxis};
use crate::sketch::{SegmentId, Sketch};

/// What a freshly-drawn rectangle earns on its own: three right angles — the
/// fourth follows — held as constraints rather than dimensions, since a right
/// angle is a relationship the rectangle holds by construction, not a
/// measurement someone typed and could retype. Alongside them, the length of
/// whichever of the two neighbouring sides the user actually typed a size
/// for. A side left untyped stays at whatever length the cursor gave it,
/// undimensioned, the same as a line drawn with no value entered. A length
/// already redundant with what the drawing holds is left out.
pub fn rectangle_dimensions(
    sketch: &Sketch,
    sides: [SegmentId; 4],
    locked: LockedInput,
    scale: f64,
) -> (Vec<Constraint>, Vec<(DimensionTarget, f64)>) {
    let corners = (0..3)
        .map(|corner| Constraint::Perpendicular {
            first: sides[corner],
            second: sides[corner + 1],
        })
        .collect();

    let mut wanted: Vec<(DimensionTarget, f64)> = Vec::new();
    for (side, typed) in sides.iter().take(2).zip([locked.first, locked.second]) {
        if typed.is_some() {
            wanted.push((
                DimensionTarget::Length(*side),
                sketch.segment_length(*side) * scale,
            ));
        }
    }
    (corners, settled(sketch, wanted, scale))
}

/// Places on the line just drawn whatever the user typed, and the right angle
/// it was aimed at.
///
/// A value that would say nothing is left out: the drawing already holds it,
/// and a second copy could only be redundant.
pub fn line_dimensions(
    sketch: &Sketch,
    segment: SegmentId,
    locked: LockedInput,
    square_with: Option<SegmentId>,
    scale: f64,
) -> Vec<(DimensionTarget, f64)> {
    let mut wanted: Vec<(DimensionTarget, f64)> = Vec::new();
    if let Some(length) = locked.first {
        wanted.push((DimensionTarget::Length(segment), length));
    }
    if let Some(angle) = locked.second {
        wanted.push((
            DimensionTarget::AxisAngle {
                segment,
                axis: SketchAxis::U,
            },
            angle.abs(),
        ));
    }
    if let Some(first) = square_with {
        wanted.push((
            DimensionTarget::Angle {
                first,
                second: segment,
            },
            90.0,
        ));
    }
    settled(sketch, wanted, scale)
}

/// Normalises and drops whatever is already redundant with the drawing.
fn settled(
    sketch: &Sketch,
    wanted: Vec<(DimensionTarget, f64)>,
    scale: f64,
) -> Vec<(DimensionTarget, f64)> {
    wanted
        .into_iter()
        .map(|(target, value)| (target.normalised(), value))
        .filter(|(target, _)| !sketch.would_be_redundant(*target, scale))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::plane::WorkPlane;
    use glam::DVec2;

    fn right_angle_pair() -> (Sketch, SegmentId, SegmentId) {
        let mut sketch = Sketch::new(WorkPlane::XY);
        let a = sketch.add_point(DVec2::new(0.0, 0.0));
        let b = sketch.add_point(DVec2::new(40.0, 0.0));
        let c = sketch.add_point(DVec2::new(40.0, 40.0));
        let first = sketch.add_segment(a, b);
        let second = sketch.add_segment(b, c);
        (sketch, first, second)
    }

    fn rectangle_sides() -> (Sketch, [SegmentId; 4]) {
        let mut sketch = Sketch::new(WorkPlane::XY);
        let a = sketch.add_point(DVec2::new(0.0, 0.0));
        let b = sketch.add_point(DVec2::new(40.0, 0.0));
        let c = sketch.add_point(DVec2::new(40.0, 20.0));
        let d = sketch.add_point(DVec2::new(0.0, 20.0));
        let sides = [
            sketch.add_segment(a, b),
            sketch.add_segment(b, c),
            sketch.add_segment(c, d),
            sketch.add_segment(d, a),
        ];
        (sketch, sides)
    }

    #[test]
    fn a_rectangle_typed_on_both_sides_gets_three_perpendiculars_and_two_lengths() {
        let (sketch, sides) = rectangle_sides();
        let locked = LockedInput {
            first: Some(40.0),
            second: Some(20.0),
        };

        let (corners, wanted) = rectangle_dimensions(&sketch, sides, locked, 1.0);

        assert_eq!(
            corners.len(),
            3,
            "the fourth right angle follows from the other three"
        );
        for corner in 0..3 {
            assert!(corners.contains(&Constraint::Perpendicular {
                first: sides[corner],
                second: sides[corner + 1],
            }));
        }
        let lengths = wanted
            .iter()
            .filter(|(target, _)| matches!(target, DimensionTarget::Length(_)))
            .count();
        assert_eq!(
            lengths, 2,
            "two neighbouring lengths pin the rectangle down"
        );
        assert!(
            wanted
                .iter()
                .all(|(target, _)| !matches!(target, DimensionTarget::Angle { .. })),
            "a right angle is held by construction, not by a dimension someone could retype"
        );
        assert!(wanted.iter().all(|(_, value)| *value > 0.0));
    }

    #[test]
    fn a_rectangle_dragged_with_nothing_typed_gets_no_length() {
        let (sketch, sides) = rectangle_sides();

        let (corners, wanted) = rectangle_dimensions(&sketch, sides, LockedInput::default(), 1.0);

        assert_eq!(
            corners.len(),
            3,
            "square corners hold by construction whether or not a size was typed"
        );
        assert!(
            wanted
                .iter()
                .all(|(target, _)| !matches!(target, DimensionTarget::Length(_))),
            "a side nobody typed a value for stays undimensioned"
        );
    }

    #[test]
    fn a_rectangle_typed_on_one_side_only_dimensions_that_side() {
        let (sketch, sides) = rectangle_sides();
        let locked = LockedInput {
            first: Some(40.0),
            second: None,
        };

        let (corners, wanted) = rectangle_dimensions(&sketch, sides, locked, 1.0);

        assert_eq!(
            corners.len(),
            3,
            "typing only one side still squares all corners"
        );
        assert!(
            wanted
                .iter()
                .any(|(target, _)| *target == DimensionTarget::Length(sides[0]))
        );
        assert!(
            !wanted
                .iter()
                .any(|(target, _)| *target == DimensionTarget::Length(sides[1])),
            "the untyped side earns no dimension"
        );
    }

    #[test]
    fn a_line_typed_with_a_length_and_squared_up_earns_both_dimensions() {
        let (sketch, previous, segment) = right_angle_pair();
        let locked = LockedInput {
            first: Some(40.0),
            second: None,
        };

        let wanted = line_dimensions(&sketch, segment, locked, Some(previous), 1.0);

        assert!(
            wanted.iter().any(
                |(target, value)| *target == DimensionTarget::Length(segment) && *value == 40.0
            )
        );
        assert!(wanted.iter().any(|(target, value)| {
            *target
                == DimensionTarget::Angle {
                    first: previous,
                    second: segment,
                }
                .normalised()
                && *value == 90.0
        }));
    }
}
