//! The dot of one numbered step of a floating tool panel: 22 px, done, current or to come.

use egui::{Align2, Color32, FontId, Response, Sense, Ui, vec2};

use crate::ui::theme::tokens::{
    ACCENT_AMBER, BACKGROUND_ACTIVE_NEUTRAL, STATE_SUCCESS, STATE_SUCCESS_TEXT, TEXT_ON_ACCENT,
    TEXT_SECONDARY,
};

/// How wide the dot stands: C13's 22 px.
const SIDE: f32 = 22.0;

/// The size `docs/redesign/source/part.html` writes the mark at, which the
/// tokens do not carry.
const TEXT_SIZE: f32 = 11.0;

/// How much of the success green a step done is filled with: C13's 18%.
const DONE_OPACITY: f32 = 0.18;

/// Where a step stands in the order of its panel.
#[cfg_attr(not(test), expect(dead_code))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Progress {
    Done,
    Current,
    ToCome,
}

/// What a dot shows: its fill, and the mark written on it in its ink.
#[derive(Clone, Debug, PartialEq)]
pub struct Face {
    pub fill: Color32,
    pub ink: Color32,
    pub mark: String,
}

/// The face of step `number`: a check once it is done, its number otherwise.
pub fn face(progress: Progress, number: usize) -> Face {
    match progress {
        Progress::Done => Face {
            fill: STATE_SUCCESS.gamma_multiply(DONE_OPACITY),
            ink: STATE_SUCCESS_TEXT,
            mark: "✓".to_owned(),
        },
        Progress::Current => Face {
            fill: ACCENT_AMBER,
            ink: TEXT_ON_ACCENT,
            mark: number.to_string(),
        },
        Progress::ToCome => Face {
            fill: BACKGROUND_ACTIVE_NEUTRAL,
            ink: TEXT_SECONDARY,
            mark: number.to_string(),
        },
    }
}

/// The round dot of step `number`, in the face [`face`] gives it.
#[cfg_attr(not(test), expect(dead_code))]
pub fn step_dot(ui: &mut Ui, number: usize, progress: Progress) -> Response {
    let face = face(progress, number);
    let (rect, response) = ui.allocate_exact_size(vec2(SIDE, SIDE), Sense::click());
    let painter = ui.painter();
    painter.circle_filled(rect.center(), SIDE / 2.0, face.fill);
    painter.text(
        rect.center(),
        Align2::CENTER_CENTER,
        face.mark,
        FontId::proportional(TEXT_SIZE),
        face.ink,
    );
    response
}

#[cfg(test)]
mod tests;
