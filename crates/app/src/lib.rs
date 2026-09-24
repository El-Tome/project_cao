pub mod adapters;
pub mod app;
mod autosave;
mod commands;
pub mod crash;
mod lang;
mod panels;
mod picture;
mod remembered;
mod screens;
mod shortcuts;
mod ui;
mod wording;

/// Multisampling for the whole surface: the viewport is drawn with thin lines,
/// which alias badly without it. The renderer's pipelines must be built with
/// this same count.
pub const MSAA_SAMPLES: u16 = 4;
