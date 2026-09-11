//! A mark the fonts cannot draw comes out as an empty box, and an empty box
//! says less than nothing: five buttons in a row became the same square.
//! `wording/constraints.rs` had already written that reasoning down, for ⊥ and
//! ∥; it was recorded in one module and never applied in the next. This file
//! replaces the note with a gate.
//!
//! Both places the interface keeps its words are read: the language file, and
//! the string literals of `cao_app`. Asking the fonts themselves rather than
//! holding a list of allowed characters is what makes it one rule — the day
//! `egui` ships a font covering an arrow, the arrow is allowed here with no
//! edit.
//!
//! `Fonts::has_glyph` is the obvious way in and the wrong one: it answers by
//! comparing which face owns the character against the face owning the
//! replacement glyph, so every character `NotoEmoji-Regular` alone carries —
//! the pencil and the tick this interface already shows — is reported missing.
//! `Font::characters` lists what the family really reaches, and agrees with
//! what the running application draws.
//!
//! Only the proportional family is read, because nothing here asks for another
//! one. Only characters outside ASCII are read: `Ubuntu-Light` carries all of
//! ASCII, and a literal holds its escapes unresolved, so `\n` would arrive as
//! a backslash and an `n`.
//!
//! What it does not catch: a mark assembled at run time from codepoints, and a
//! font handed to `egui` at start-up, since nothing here does either.

use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Path, PathBuf};

use egui::FontDefinitions;
use egui::epaint::{FontFamily, Fonts, TextOptions};

const FRENCH_FILE: &str = "crates/app/src/lang/fr.json";

/// Where the interface keeps the words it does not read from the language file.
const INTERFACE_SOURCES: &str = "crates/app/src";

#[test]
fn every_mark_the_language_file_writes_is_one_the_fonts_can_draw() {
    let drawable = marks_the_fonts_draw();
    let mut undrawable = Vec::new();

    for (key, said) in french_entries() {
        for mark in marks_missing_from(&drawable, &said) {
            undrawable.push(format!("{mark}  ({key})"));
        }
    }

    assert!(
        undrawable.is_empty(),
        "Marks the fonts shipped with egui cannot draw:\n  {}\n\
         Each one shows as an empty box. Say it in words, or pick a mark the\n\
         fonts carry — ↺ ⚙ ✏ ✔ ↖ ↗ ↙ ↘ ⬅ ➡ ⬆ ⬇ ✖ « » — · all draw.",
        undrawable.join("\n  "),
    );
}

#[test]
fn every_mark_written_into_the_interface_sources_is_one_the_fonts_can_draw() {
    let drawable = marks_the_fonts_draw();
    let mut undrawable = Vec::new();

    for (path, literal, line) in literals_of_the_interface() {
        for mark in marks_missing_from(&drawable, &literal) {
            undrawable.push(format!("{mark}  ({path}:{line})"));
        }
    }

    assert!(
        undrawable.is_empty(),
        "Marks the fonts shipped with egui cannot draw:\n  {}\n\
         Each one shows as an empty box. A mark a reader sees belongs in\n\
         {FRENCH_FILE} anyway, where a translator can reach it.",
        undrawable.join("\n  "),
    );
}

#[test]
fn a_mark_the_fonts_carry_is_left_alone() {
    let drawable = marks_the_fonts_draw();

    for said in ["↺ Barre par défaut", "⚙ Préférences", "Esquisse — {plane}"] {
        assert!(
            marks_missing_from(&drawable, said).is_empty(),
            "{said} draws, and the rule took it for an empty box",
        );
    }
}

#[test]
fn a_mark_no_font_carries_is_named_with_its_codepoint() {
    let drawable = marks_the_fonts_draw();

    assert_eq!(
        marks_missing_from(&drawable, "⌂ Accueil"),
        vec!["'⌂' U+2302".to_string()],
    );
}

/// Every character the proportional family reaches, across the whole fallback
/// chain. These are the fonts `egui` ships with, which is what the shell runs
/// on: nothing here hands it any other.
fn marks_the_fonts_draw() -> BTreeSet<char> {
    let mut fonts = Fonts::new(TextOptions::default(), FontDefinitions::default());
    fonts
        .fonts
        .font(&FontFamily::Proportional)
        .characters()
        .keys()
        .copied()
        .collect()
}

/// Every character of `said` the fonts have no glyph for, named once each and
/// with its codepoint — an empty box is not something one can paste into a
/// search.
fn marks_missing_from(drawable: &BTreeSet<char>, said: &str) -> Vec<String> {
    said.chars()
        .filter(|mark| !mark.is_ascii())
        .collect::<BTreeSet<char>>()
        .into_iter()
        .filter(|mark| !drawable.contains(mark))
        .map(|mark| format!("'{mark}' U+{:04X}", mark as u32))
        .collect()
}

fn french_entries() -> BTreeMap<String, String> {
    let path = workspace_root().join(FRENCH_FILE);
    let text = fs::read_to_string(&path).unwrap_or_else(|_| panic!("{FRENCH_FILE} is readable"));
    serde_json::from_str(&text).unwrap_or_else(|_| panic!("{FRENCH_FILE} is an object of strings"))
}

/// Every string literal of the interface sources, with the file and line
/// holding it. Comment lines are left out: a note may quote a mark precisely
/// because the fonts cannot draw it.
fn literals_of_the_interface() -> Vec<(String, String, usize)> {
    let mut found = Vec::new();

    for path in rust_files(&workspace_root().join(INTERFACE_SOURCES)) {
        let where_from = relative(&path);
        let text = fs::read_to_string(&path).expect("a readable source file");

        for (index, line) in text.lines().enumerate() {
            if line.trim_start().starts_with("//") {
                continue;
            }
            for literal in line.split('"').skip(1).step_by(2) {
                found.push((where_from.clone(), literal.to_string(), index + 1));
            }
        }
    }

    found
}

fn rust_files(directory: &Path) -> Vec<PathBuf> {
    let mut found = Vec::new();
    let mut pending = vec![directory.to_path_buf()];

    while let Some(current) = pending.pop() {
        let Ok(entries) = fs::read_dir(&current) else {
            continue;
        };
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                pending.push(path);
            } else if path.extension().is_some_and(|extension| extension == "rs") {
                found.push(path);
            }
        }
    }

    found.sort();
    found
}

fn relative(path: &Path) -> String {
    path.strip_prefix(workspace_root())
        .unwrap_or(path)
        .display()
        .to_string()
        .replace('\\', "/")
}

fn workspace_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(Path::parent)
        .expect("the workspace root, two levels above crates/app")
        .to_path_buf()
}
