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

/// An error on the pill, and the message it was laid over.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Covering {
    pub error: String,
    pub covers: Option<String>,
}

/// What the pill holds once a frame is over. An error raised since the last
/// frame was settled takes it, over whatever message stands, a message said in
/// the same frame included. One standing from before gives way to a command,
/// or to a message other than the one the frame began with — a sentence said
/// again unchanged at every frame is not a new one — and takes the message it
/// covered with it, so that one never comes back. Nothing else moves it: a
/// write that succeeds since takes nothing back.
pub fn settle(
    standing: Option<Covering>,
    fresh: Option<String>,
    at_start: Option<&str>,
    message: &mut Option<String>,
    commanded: bool,
) -> Option<Covering> {
    if let Some(error) = fresh {
        return Some(Covering {
            error,
            covers: message.clone(),
        });
    }
    let standing = standing?;
    let said_since = message.is_some() && message.as_deref() != at_start;
    if commanded || said_since {
        if *message == standing.covers {
            *message = None;
        }
        return None;
    }
    Some(standing)
}

#[cfg(test)]
mod tests;
