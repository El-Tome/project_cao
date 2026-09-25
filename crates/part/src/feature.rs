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
                match features.last_mut() {
                    Some(feature) => feature.end = at + 1,
                    // Variables can be made before anything is drawn, and
                    // they are a line of the history like any other.
                    None => features.push(Self {
                        start: at,
                        end: at + 1,
                        sketch: None,
                    }),
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
mod tests;
