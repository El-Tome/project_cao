use crate::history::{History, StepKind};

/// One major step of the design, as the history panel needs it: where it
/// begins and ends in the list of operations, and which sketch it opened.
///
/// The grouping itself is not worked out here — the history records it, and
/// the file is laid out by it. This turns it into the flat positions the
/// panel speaks, which are the positions "go back to this step" moves to.
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
    pub fn all(history: &History) -> Vec<Self> {
        let mut start = 0;
        let mut sketches = 0;

        history
            .steps()
            .iter()
            .map(|step| {
                let sketch = (step.kind() == StepKind::Sketch).then(|| {
                    sketches += 1;
                    sketches - 1
                });
                let feature = Self {
                    start,
                    end: start + step.len(),
                    sketch,
                };
                start = feature.end;
                feature
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::history::{Operation, PointRef};
    use cao_sketch::WorkPlane;
    use glam::DVec2;

    fn drawn(operations: impl IntoIterator<Item = Operation>) -> History {
        let mut history = History::default();
        for operation in operations {
            history.push(operation);
        }
        history
    }

    fn sketch() -> Operation {
        Operation::CreateSketch {
            plane: WorkPlane::XY,
            on: None,
        }
    }

    fn segment(sketch: usize) -> Operation {
        Operation::AddSegment {
            sketch,
            start: PointRef::New(DVec2::ZERO),
            end: PointRef::New(DVec2::X),
            construction: false,
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
        let history = drawn([sketch(), segment(0), extrude(0), sketch(), segment(1)]);

        let features = Feature::all(&history);

        assert_eq!(features.len(), 3);
        assert_eq!(features[0].sketch, Some(0));
        assert_eq!(features[1].sketch, None, "an extrusion opens no sketch");
        assert_eq!(features[2].sketch, Some(1));
    }

    #[test]
    fn the_steps_that_follow_a_feature_are_grouped_under_it() {
        let history = drawn([sketch(), segment(0), segment(0), extrude(0)]);

        let features = Feature::all(&history);

        assert_eq!(features.len(), 2);
        assert_eq!((features[0].start, features[0].end), (0, 3));
        assert_eq!((features[1].start, features[1].end), (3, 4));
    }

    #[test]
    fn a_history_with_nothing_in_it_has_no_feature() {
        assert!(Feature::all(&History::default()).is_empty());
    }

    #[test]
    fn a_stroke_that_would_belong_to_no_step_is_not_recorded() {
        let history = drawn([segment(0), segment(0)]);

        assert!(
            Feature::all(&history).is_empty(),
            "every operation is written in a step's folder, so one that names \
             a sketch the part does not have has nowhere to go — and the \
             geometry makes nothing of it either",
        );
        assert!(history.operations().is_empty());
    }
}
