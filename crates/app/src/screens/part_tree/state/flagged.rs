//! The elements of the tree wearing a badge, and what each badge is pinned on.

use super::PartTree;
use crate::wording::part_tree::Problem;

/// What a badge of the tree is pinned on: a sketch by its place among the
/// part's drawings, a volume by its step in the design, a kept section by its
/// id. A chamfer or a fillet, which has no row, by its step too.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum About {
    Sketch(usize),
    Volume { step: usize },
    Section(usize),
    Blend { step: usize },
}

/// An element wearing a badge: what it is, named as the tree names it, and
/// the gravest of its problems.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Flagged<'a> {
    pub about: About,
    pub name: &'a str,
    pub problem: Problem,
    /// What the user is told of a step built unstably, when it was.
    pub unstable: Option<&'a str>,
}

impl PartTree {
    /// Every element that wears a badge, in the order the panel shows them.
    pub fn problems(&self) -> impl Iterator<Item = Flagged<'_>> {
        let sketches = self.sketches.iter().filter_map(|drawn| {
            Some(Flagged {
                about: About::Sketch(drawn.sketch),
                name: &drawn.name,
                problem: drawn.badge?,
                unstable: None,
            })
        });
        let bodies = self.bodies.iter().filter_map(|body| {
            Some(Flagged {
                about: About::Volume { step: body.step },
                name: &body.name,
                problem: body.badge?,
                unstable: body.unstable.as_deref(),
            })
        });
        let sections = self.sections.iter().filter_map(|kept| {
            Some(Flagged {
                about: About::Section(kept.id),
                name: &kept.name,
                problem: kept.badge?,
                unstable: None,
            })
        });
        sketches.chain(bodies).chain(sections)
    }
}
