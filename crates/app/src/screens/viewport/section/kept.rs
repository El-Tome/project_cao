//! A section the part keeps, as the section view takes it up and hands it
//! back: the part says how each side is drawn in words of its own, which the
//! renderer's are mapped to here.

use cao_part::history::{Operation, SectionAsked, SideDrawing};
use cao_part::{PartDocument, SavedSection};
use cao_render::MatterInFront;

use super::Section;

impl Section {
    /// The section view on a kept section, as the part holds it now.
    pub(crate) fn reopened(saved: &SavedSection) -> Self {
        let mut section = Self::new(saved.plane);
        section.offset = saved.offset;
        section.behind = drawn_as(saved.behind);
        section.in_front = drawn_as(saved.in_front);
        section.on = saved.on;
        section.editing = Some(saved.id);
        section
    }

    pub(crate) fn asked(&self) -> SectionAsked {
        SectionAsked {
            plane: self.plane,
            on: self.on,
            offset: self.offset,
            behind: kept_as(self.behind),
            in_front: kept_as(self.in_front),
        }
    }

    /// Keeps the section in the part: a new one, or the one it was reopened
    /// from, changed. Kept again later, it changes the same one.
    pub(crate) fn keep_in(&mut self, document: &mut PartDocument) {
        let to = self.asked();
        match self.editing {
            Some(section) => {
                document.apply(Operation::EditSection { section, to });
            }
            None => {
                document.apply(Operation::SaveSection(to));
                self.editing = document.sections().last().map(|kept| kept.id);
            }
        }
    }
}

fn drawn_as(side: SideDrawing) -> MatterInFront {
    match side {
        SideDrawing::Hidden => MatterInFront::Hidden,
        SideDrawing::SeeThrough => MatterInFront::SeeThrough,
        SideDrawing::Opaque => MatterInFront::Opaque,
    }
}

fn kept_as(drawn: MatterInFront) -> SideDrawing {
    match drawn {
        MatterInFront::Hidden => SideDrawing::Hidden,
        MatterInFront::SeeThrough => SideDrawing::SeeThrough,
        MatterInFront::Opaque => SideDrawing::Opaque,
    }
}
