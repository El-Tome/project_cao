use crate::history::{History, StepKind};

/// One major step of the design, as the history panel needs it: where it
/// begins and ends in the list of operations, and which sketch it opened.
///
/// These are runs of the list as it was typed, not the steps the design is
/// replayed by. The panel is the journal: it reads in the order things
/// happened, and the positions it speaks are the ones "go back to this step"
/// moves the cursor to. An edit to an earlier sketch belongs to that sketch's
/// step and is replayed with it, and still shows here where it was typed.
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
        let mut features: Vec<Self> = Vec::new();
        let mut sketches = 0;

        for (at, operation) in history.operations().iter().enumerate() {
            let Some(kind) = StepKind::opened_by(operation) else {
                if let Some(feature) = features.last_mut() {
                    feature.end = at + 1;
                }
                continue;
            };
            let sketch = (kind == StepKind::Sketch).then(|| {
                sketches += 1;
                sketches - 1
            });
            features.push(Self {
                start: at,
                end: at + 1,
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

    /// An edit to an earlier sketch is replayed with that sketch, and still
    /// reads in the journal where it was typed — at the end, under the last
    /// feature opened, rather than pulling the features after it into the
    /// first one.
    #[test]
    fn an_edit_to_an_earlier_sketch_reads_where_it_was_typed() {
        let history = drawn([
            sketch(),
            segment(0),
            Operation::Extrude {
                sketch: 0,
                picks: Vec::new(),
                distance: 1.0,
                mode: crate::history::ExtrusionMode::Add,
            },
            sketch(),
            segment(0),
        ]);

        let features = Feature::all(&history);

        assert_eq!(features.len(), 3);
        assert_eq!((features[0].start, features[0].end), (0, 2));
        assert_eq!((features[1].start, features[1].end), (2, 3));
        assert_eq!(
            (features[2].start, features[2].end),
            (3, 5),
            "the edit typed last reads under the feature that was open",
        );
    }
}
