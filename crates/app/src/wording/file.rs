//! The three ways the disk can refuse, said once.
//!
//! `cao_part` and `cao_prefs` each carry their own `FileError` — a part
//! archive and a settings file have nothing to say to each other — but a
//! refused write reads the same to whoever is looking at it.

use std::path::Path;

use crate::lang::Catalogue;

pub fn absent(lang: &Catalogue, path: &Path) -> String {
    lang.t_with("file.absent", &[("path", &path.display().to_string())])
}

pub fn refused(lang: &Catalogue, path: &Path) -> String {
    lang.t_with("file.refused", &[("path", &path.display().to_string())])
}

pub fn interrupted(lang: &Catalogue, path: &Path) -> String {
    lang.t_with("file.interrupted", &[("path", &path.display().to_string())])
}
