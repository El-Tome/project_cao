//! Every sentence the user reads, gathered where the interface is.
//!
//! A layer below `cao_app` returns a named case — `Key::Escape`,
//! `ExtrusionMode::Cut` — and this is where it is decided how that case is
//! said. One file per source, so that no single one accumulates the whole
//! application's wording, and so that a translation system later has one
//! directory to pass under rather than a hunt.

pub mod shortcuts;
pub mod toolbar;
