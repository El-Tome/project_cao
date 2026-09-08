use crate::history::Operation;

/// One group of the history: the operation that opens a feature, and the run of
/// operations recorded under it before the next one opens.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Feature {
    pub start: usize,
    pub end: usize,
    /// The sketch this feature opened, when it opened one. An extrusion or a
    /// revolution starts a feature without starting a sketch, which is what
    /// counting the features got wrong.
    pub sketch: Option<usize>,
}

impl Feature {
    pub fn all(operations: &[Operation]) -> Vec<Self> {
        let mut features: Vec<Self> = Vec::new();
        let mut sketches = 0;

        for (index, operation) in operations.iter().enumerate() {
            if !operation.starts_feature() {
                continue;
            }
            if let Some(previous) = features.last_mut() {
                previous.end = index;
            }
            let sketch = matches!(operation, Operation::CreateSketch { .. }).then(|| {
                sketches += 1;
                sketches - 1
            });
            features.push(Self {
                start: index,
                end: operations.len(),
                sketch,
            });
        }
        features
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::history::{Operation, PointRef};
    use cao_sketch::WorkPlane;
    use glam::DVec2;

    fn segment(sketch: usize) -> Operation {
        Operation::AddSegment {
            sketch,
            start: PointRef::New(DVec2::ZERO),
            end: PointRef::New(DVec2::X),
        }
    }

    fn extrude(sketch: usize) -> Operation {
        Operation::Extrude {
            sketch,
            picks: Vec::new(),
            distance: 10.0,
            mode: crate::history::ExtrusionMode::Add,
        }
    }

    #[test]
    fn an_extrusion_between_two_sketches_does_not_shift_the_second_one() {
        let operations = [
            Operation::CreateSketch {
                plane: WorkPlane::XY,
            },
            segment(0),
            extrude(0),
            Operation::CreateSketch {
                plane: WorkPlane::XY,
            },
            segment(1),
        ];

        let features = Feature::all(&operations);

        assert_eq!(features.len(), 3);
        assert_eq!(features[0].sketch, Some(0));
        assert_eq!(features[1].sketch, None, "an extrusion opens no sketch");
        assert_eq!(features[2].sketch, Some(1));
    }

    #[test]
    fn the_steps_that_follow_a_feature_are_grouped_under_it() {
        let operations = [
            Operation::CreateSketch {
                plane: WorkPlane::XY,
            },
            segment(0),
            segment(0),
            extrude(0),
        ];

        let features = Feature::all(&operations);

        assert_eq!(features.len(), 2);
        assert_eq!((features[0].start, features[0].end), (0, 3));
        assert_eq!((features[1].start, features[1].end), (3, 4));
    }

    #[test]
    fn steps_before_the_first_feature_belong_to_no_feature() {
        assert!(Feature::all(&[segment(0)]).is_empty());
        assert!(Feature::all(&[]).is_empty());
    }
}
