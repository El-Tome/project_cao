//! `Catalogue::t` answers with the key itself when nothing claims it, so a
//! misspelt key names itself on screen instead of leaving a blank label. What
//! that costs is a typo nobody sees until they open the screen carrying it.
//! This file turns the hand check three batches went through into a failing
//! gate.
//!
//! A key is recognised by its shape — dotted lowercase segments — rather than
//! by a list of known prefixes. A prefix list would go quiet the day a module
//! invents one, and a gate that goes quiet is worse than no gate. File names
//! share that shape, so the extensions below are named instead: getting that
//! list wrong costs a loud failure, never a silent pass.
//!
//! What it does not catch: a key assembled at run time, `format!("history.{x}")`
//! rather than written out. None exist today, and the day one does it is
//! invisible here — which is an argument for writing keys out.

use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Path, PathBuf};

/// A literal ending on one of these names a file, not a sentence to say.
const EXTENSIONS_A_FILE_NAME_ENDS_ON: [&str; 8] =
    ["bin", "caopart", "json", "log", "md", "rs", "sh", "toml"];

/// Where a key is named. Below `cao_app` nothing speaks to a user, so nothing
/// there holds one.
const INTERFACE_SOURCES: &str = "crates/app/src";

const FRENCH_FILE: &str = "crates/app/src/lang/fr.json";

#[test]
fn every_key_a_rust_file_names_has_an_entry_in_the_french_file() {
    let said = french_entries();

    let unclaimed: Vec<String> = keys_named_in_rust()
        .into_iter()
        .filter(|(key, _)| !said.contains_key(key))
        .map(|(key, where_from)| format!("{key}  ({where_from})"))
        .collect();

    assert!(
        unclaimed.is_empty(),
        "Keys with no entry in {FRENCH_FILE}:\n  {}\n\
         The interface would say the key itself. Write what each one stands for,\n\
         or fix the spelling.",
        unclaimed.join("\n  "),
    );
}

#[test]
fn every_entry_in_the_french_file_is_named_by_a_rust_file() {
    let named: BTreeSet<String> = keys_named_in_rust()
        .into_iter()
        .map(|(key, _)| key)
        .collect();

    let forgotten: Vec<String> = french_entries()
        .into_keys()
        .filter(|key| !named.contains(key))
        .collect();

    assert!(
        forgotten.is_empty(),
        "Entries in {FRENCH_FILE} nothing names:\n  {}\n\
         A rename left them behind, or the code that said them is gone. A\n\
         translator should never be handed a sentence the interface cannot show.",
        forgotten.join("\n  "),
    );
}

fn french_entries() -> BTreeMap<String, String> {
    let path = workspace_root().join(FRENCH_FILE);
    let text = fs::read_to_string(&path).unwrap_or_else(|_| panic!("{FRENCH_FILE} is readable"));
    serde_json::from_str(&text).unwrap_or_else(|_| panic!("{FRENCH_FILE} is an object of strings"))
}

/// Every key named in the interface sources, each with the file and line that
/// names it — a failure has to say where to go.
fn keys_named_in_rust() -> Vec<(String, String)> {
    let mut named = Vec::new();

    for path in rust_files(&workspace_root().join(INTERFACE_SOURCES)) {
        let where_from = relative(&path);
        let text = fs::read_to_string(&path).expect("a readable source file");

        for (number, line) in text.lines().enumerate() {
            if line.trim_start().starts_with("//") {
                continue;
            }
            for literal in line.split('"').skip(1).step_by(2) {
                if is_shaped_like_a_key(literal) {
                    named.push((literal.to_string(), format!("{where_from}:{}", number + 1)));
                }
            }
        }
    }

    named
}

/// Dotted lowercase segments, and a last one that is not a file extension.
fn is_shaped_like_a_key(literal: &str) -> bool {
    let mut segments = literal.split('.').peekable();
    let mut counted = 0;

    while let Some(segment) = segments.next() {
        let readable = !segment.is_empty()
            && segment.starts_with(|c: char| c.is_ascii_lowercase())
            && segment
                .chars()
                .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '_');
        if !readable {
            return false;
        }
        if segments.peek().is_none() && EXTENSIONS_A_FILE_NAME_ENDS_ON.contains(&segment) {
            return false;
        }
        counted += 1;
    }

    counted > 1
}

fn rust_files(directory: &Path) -> Vec<PathBuf> {
    let mut found = Vec::new();
    walk(directory, &mut found);
    found.sort();
    found
}

fn walk(directory: &Path, found: &mut Vec<PathBuf>) {
    let Ok(entries) = fs::read_dir(directory) else {
        return;
    };

    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            walk(&path, found);
        } else if path.extension().and_then(|extension| extension.to_str()) == Some("rs") {
            found.push(path);
        }
    }
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

#[test]
fn a_file_name_is_not_taken_for_a_key() {
    for literal in ["fr.json", "piece.caopart", "plantages.log", "part.bin"] {
        assert!(
            !is_shaped_like_a_key(literal),
            "{literal} names a file and the rule took it for a key",
        );
    }
}

#[test]
fn a_key_is_told_apart_from_the_prose_around_it() {
    for literal in [
        "history.point",
        "command.label.new_sketch",
        "circle.asks_for.center",
    ] {
        assert!(is_shaped_like_a_key(literal), "{literal} is a key");
    }
    for literal in [
        "Point",
        "Esquisse — {plane}",
        "crates/app",
        "PlaneKind",
        "",
        ".",
    ] {
        assert!(!is_shaped_like_a_key(literal), "{literal} is not a key");
    }
}
