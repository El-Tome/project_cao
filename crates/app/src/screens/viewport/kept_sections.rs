//! Which of the sections the part keeps is switched on: the viewport's to
//! decide, never saved nor undone, so a part always opens whole.

use cao_part::PartDocument;

use super::section::Section;

#[derive(Default)]
pub(crate) struct KeptSections {
    /// The kept section switched on, by id. One at most.
    on: Option<usize>,
    /// It, as the part stands now; nothing while it is broken and cuts nothing.
    shown: Option<Section>,
    /// Which was on before a drawing laid on a section was opened, to be put
    /// back once it is left.
    before_the_sketch: Option<Option<usize>>,
}

impl KeptSections {
    pub(crate) fn is_on(&self, section: usize) -> bool {
        self.on == Some(section)
    }

    /// Switches a section on, the one on before off; or off when it was on.
    pub(crate) fn toggle(&mut self, section: usize) {
        self.on = (self.on != Some(section)).then_some(section);
    }

    /// Switches a section on, the one on before off.
    pub(crate) fn switch_on(&mut self, section: usize) {
        self.on = Some(section);
    }

    /// The section switched on, as the frame cuts with it.
    pub(crate) fn shown(&self) -> Option<&Section> {
        self.shown.as_ref()
    }

    /// The id of the section switched on, when it cuts anything.
    pub(crate) fn cutting(&self) -> Option<usize> {
        self.on.filter(|_| self.shown.is_some())
    }

    /// Follows the part and the drawing open: a drawing laid on a section
    /// switches it on while it is open, and leaving it puts back what was on.
    /// A section the part no longer has is switched off. Says whether which
    /// one is on changed.
    pub(crate) fn keep_up_with(&mut self, document: &PartDocument, sketch: Option<usize>) -> bool {
        let was = self.on;
        match sketch.and_then(|open| document.section_under(open)) {
            Some(under) => {
                self.before_the_sketch.get_or_insert(self.on);
                self.on = Some(under);
            }
            None => {
                if let Some(before) = self.before_the_sketch.take() {
                    self.on = before;
                }
            }
        }
        let sections = document.sections();
        let found = self
            .on
            .and_then(|on| sections.iter().find(|kept| kept.id == on));
        if found.is_none() {
            self.on = None;
        }
        self.shown = found.filter(|kept| !kept.broken).map(Section::reopened);
        was != self.on
    }
}
