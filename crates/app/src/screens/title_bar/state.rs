use std::path::Path;

use cao_part::PartDocument;

use crate::lang::Catalogue;
use crate::screens::sketch::SketchEditor;
use crate::screens::viewport::ViewportState;
use crate::wording::view::{Standing, standing_on};

/// What the title bar reads: the part's name and the file it is kept in, the
/// name of where the view stands, and what the pill says.
pub struct TitleBar<'a> {
    pub name: &'a str,
    pub path: &'a Path,
    pub view: String,
    pub said: Option<Said<'a>>,
}

/// The one thing the pill says at a time.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Said<'a> {
    /// What the last gesture answered.
    Message(&'a str),
    /// What went wrong reaching the disk since: a save, an export, the settings.
    Error(&'a str),
}

impl<'a> TitleBar<'a> {
    /// A sketch is ranked by its place among the part's drawings and a kept
    /// section by its id, both from one, as the Part tree names them.
    pub fn of(
        document: &'a PartDocument,
        path: &'a Path,
        editor: &'a SketchEditor,
        viewport: &ViewportState,
        error: Option<&'a str>,
        lang: &Catalogue,
    ) -> Self {
        let standing = Standing {
            open_sketch_rank: editor.active_sketch().map(|sketch| sketch + 1),
            section_view_on: viewport.section().is_some(),
            kept_section_rank: viewport.kept_sections.cutting().map(|id| id + 1),
            plane: viewport.mode().plane(),
        };
        Self {
            name: document.name(),
            path,
            view: standing_on(lang, standing),
            said: error
                .map(Said::Error)
                .or(editor.message.as_deref().map(Said::Message)),
        }
    }
}

/// An error standing as a frame begins has taken the pill: the message before
/// it is gone, so that any message found at the end of the frame was said in it.
pub fn takes_the_pill(error: Option<&str>, message: &mut Option<String>) {
    if error.is_some() {
        *message = None;
    }
}

/// The error the pill holds once a frame is over. One that went wrong in it
/// wins over a message said in it, and clears that message; one standing from
/// before gives way to a message said or a command asked in the frame, and to
/// nothing else — a write that succeeds since takes nothing back.
pub fn settle(
    standing: Option<&str>,
    fresh: Option<String>,
    message: &mut Option<String>,
    commanded: bool,
) -> Option<String> {
    if fresh.is_some() {
        *message = None;
        return fresh;
    }
    if commanded || message.is_some() {
        return None;
    }
    standing.map(str::to_owned)
}
