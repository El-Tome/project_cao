//! Every sentence the user reads, gathered where the interface is.
//!
//! A layer below `cao_app` returns a named case — `Key::Escape`,
//! `ExtrusionMode::Cut` — and this is where it is decided how that case is
//! said. One module per source, so that no single one accumulates the whole
//! application's wording, and so that a translation system later has one
//! directory to pass under rather than a hunt.
//!
//! What a key says lives in [`crate::lang`], not here: this decides which key
//! a case of the domain earns.

pub mod circle;
pub mod command;
pub mod constraints;
pub mod dimension;
pub mod file;
pub mod history;
pub mod part_file;
pub mod plane;
pub mod settings;
pub mod shortcuts;
pub mod storage;
pub mod toolbar;
