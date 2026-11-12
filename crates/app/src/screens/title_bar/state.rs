use std::path::Path;

use cao_part::{History, PartDocument};

use crate::lang::Catalogue;
use crate::screens::part_tree::TreeAction;
use crate::screens::part_tree::state::{About, Flagged, PartTree};
use crate::screens::sketch::SketchEditor;
use crate::screens::viewport::ViewportState;
use crate::ui::badge::Level;
use crate::wording::view::{Standing, standing_on};
use crate::wording::{self, alerts};

/// What the title bar reads: the part's name and the file it is kept in, the
/// name of where the view stands, what the pill says, and what is wrong with
/// the part.
pub struct TitleBar<'a> {
    pub name: &'a str,
    pub path: &'a Path,
    pub view: String,
    pub said: Option<Said<'a>>,
    pub alerts: Vec<Alert>,
    /// How many alerts there are, while there is one.
    pub alerts_button: Option<AlertsButton>,
}

/// What the title bar holds from one frame to the next: the error on its
/// pill, whether the list of alerts is open, and the part's alerts with the
/// history they were read from.
#[derive(Debug, Default)]
pub struct Held {
    pub covering: Option<Covering>,
    pub alerts_open: bool,
    alerts: Option<(History, Vec<Alert>)>,
}

impl Held {
    /// The part's alerts, read again only once its history has moved: they
    /// are read off the whole tree of the part, which a frame cannot afford,
    /// and the history is all they follow — the catalogue is the session's.
    pub fn alerts(&mut self, document: &PartDocument, lang: &Catalogue) -> Vec<Alert> {
        if let Some((history, alerts)) = &self.alerts
            && *history == document.history
        {
            return alerts.clone();
        }
        let alerts = alerts_of(document, lang);
        self.alerts = Some((document.history.clone(), alerts.clone()));
        alerts
    }
}

/// One element of the part wearing a badge in the tree, at the level of the
/// gravest of its problems.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Alert {
    pub about: About,
    pub level: Level,
    /// « Extrusion 2 · non construite ».
    pub title: String,
    /// Why, in one sentence.
    pub sentence: String,
}

impl Alert {
    fn of(flagged: Flagged<'_>, lang: &Catalogue) -> Self {
        Self {
            about: flagged.about,
            level: flagged.problem.level(),
            title: alerts::title(lang, flagged.name, flagged.problem),
            sentence: alerts::why(lang, flagged.problem, flagged.unstable),
        }
    }

    /// What one of its buttons asks of the tree. « Modifier » reopens what
    /// the alert is about, a sketch or a section as a double-click on its row
    /// does; « Montrer » marks a sketch or a volume, the sketch left closed,
    /// and switches a section on, never off.
    pub fn asks(&self, button: AlertButton) -> TreeAction {
        match (button, self.about) {
            (AlertButton::Show, About::Sketch(sketch)) => TreeAction::MarkSketch(sketch),
            (AlertButton::Show, About::Section(id)) => TreeAction::SwitchSectionOn(id),
            (AlertButton::Show, About::Volume { step }) => TreeAction::MarkVolume(step),
            (AlertButton::Modify, About::Sketch(sketch)) => TreeAction::EditSketch(sketch),
            (AlertButton::Modify, About::Section(id)) => TreeAction::EditSection(id),
            (AlertButton::Modify, About::Volume { step }) => TreeAction::EditExtrusion(step),
        }
    }
}

/// « Montrer » and « Modifier », under each alert.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AlertButton {
    Show,
    Modify,
}

impl AlertButton {
    /// « Modifier » takes the user into what it reopens; after « Montrer »
    /// the next alert is still to be looked at.
    pub fn closes_the_list(self) -> bool {
        self == Self::Modify
    }
}

/// « 2 alertes », drawn at the level of the worst of them.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AlertsButton {
    pub text: String,
    pub level: Level,
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
    /// The bar with the part's alerts read afresh, which only the tests ask
    /// for: the shell reads them through its `Held`, which keeps them.
    #[cfg(test)]
    pub fn of(
        document: &'a PartDocument,
        path: &'a Path,
        editor: &'a SketchEditor,
        viewport: &ViewportState,
        error: Option<&'a str>,
        lang: &Catalogue,
    ) -> Self {
        let alerts = alerts_of(document, lang);
        Self::listing(alerts, (document, path), editor, viewport, error, lang)
    }

    /// The bar over the part's alerts. A sketch is ranked by its place among
    /// the part's drawings and a kept section by its id, both from one, as the
    /// Part tree names them.
    pub(super) fn listing(
        alerts: Vec<Alert>,
        (document, path): (&'a PartDocument, &'a Path),
        editor: &'a SketchEditor,
        viewport: &ViewportState,
        error: Option<&'a str>,
        lang: &Catalogue,
    ) -> Self {
        let standing = Standing {
            open_sketch_number: editor
                .active_sketch()
                .map(|sketch| wording::sketch::number(document, sketch)),
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
            alerts_button: counted(&alerts, lang),
            alerts,
        }
    }
}

/// One alert for each element of the part wearing a badge in its tree.
fn alerts_of(document: &PartDocument, lang: &Catalogue) -> Vec<Alert> {
    PartTree::of(document, lang)
        .problems()
        .map(|flagged| Alert::of(flagged, lang))
        .collect()
}

fn counted(alerts: &[Alert], lang: &Catalogue) -> Option<AlertsButton> {
    let key = match alerts.len() {
        0 => return None,
        1 => "app.alerts_one",
        _ => "app.alerts_many",
    };
    let level = match alerts.iter().any(|alert| alert.level == Level::Error) {
        true => Level::Error,
        false => Level::Warning,
    };
    Some(AlertsButton {
        text: lang.t_with(key, &[("count", &alerts.len().to_string())]),
        level,
    })
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
