//! The architecture rules, in a form that fails the build.
//!
//! The rules themselves live in `.claude/skills/architecture-rust`. Prose holds
//! until someone moves something; this is the part that keeps holding after.
//!
//! It lives under `cao_app` because that crate sits above every other one and
//! is the only place the whole graph is visible. It reads sources as text and
//! links against nothing, so a crate with no library target hosts it fine.
//!
//! Two of the rules are ratchets: the debt they describe already exists, so
//! they record exactly how much of it there is and refuse any more. The figures
//! are meant to fall to zero and the entries to disappear. Raising one to make
//! a test pass is the one move that empties this file of meaning.

use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Path, PathBuf};

const CRATE_DIRECTORIES: [&str; 5] = ["sketch", "solid", "render", "core", "app"];

const ALLOWED_EDGES: [(&str, &[&str]); 5] = [
    ("sketch", &[]),
    ("solid", &[]),
    ("render", &[]),
    ("core", &["cao_sketch", "cao_solid"]),
    (
        "app",
        &["cao_core", "cao_render", "cao_sketch", "cao_solid"],
    ),
];

const INTERFACE_CRATES: [&str; 4] = ["egui", "eframe", "winit", "egui-wgpu"];

const REACHES_OUTSIDE: [&str; 6] = [
    "std::fs",
    "std::net",
    "directories::",
    "Utc::now",
    "SystemTime::now",
    "std::env::",
];

const FILES_ALLOWED_TO_REACH_OUTSIDE: [&str; 4] = [
    "crates/core/src/document.rs",
    "crates/core/src/recents.rs",
    "crates/core/src/settings.rs",
    "crates/core/src/storage.rs",
];

const READER_TEXT_LEFT_BELOW_THE_INTERFACE: [(&str, usize); 8] = [
    ("crates/core/src/command.rs", 39),
    ("crates/core/src/history.rs", 19),
    ("crates/core/src/settings.rs", 1),
    ("crates/core/src/shortcuts.rs", 5),
    ("crates/core/src/storage.rs", 2),
    ("crates/core/src/toolbar.rs", 4),
    ("crates/sketch/src/constraints.rs", 4),
    ("crates/sketch/src/plane.rs", 1),
];

#[test]
fn a_crate_only_reaches_for_the_crates_the_graph_allows() {
    for (directory, allowed) in ALLOWED_EDGES {
        let declared = declared_dependencies(&manifest(directory));
        let edges: BTreeSet<&str> = declared
            .iter()
            .map(String::as_str)
            .filter(|name| name.starts_with("cao_"))
            .collect();
        let expected: BTreeSet<&str> = allowed.iter().copied().collect();

        assert_eq!(
            edges, expected,
            "crates/{directory} no longer matches the dependency graph",
        );
    }
}

#[test]
fn the_two_geometry_crates_stay_alone_with_their_maths() {
    let expected: BTreeSet<String> = ["glam", "serde"]
        .iter()
        .map(|name| name.to_string())
        .collect();

    for directory in ["sketch", "solid"] {
        assert_eq!(
            declared_dependencies(&manifest(directory)),
            expected,
            "crates/{directory} is a pure domain and takes nothing but glam and serde",
        );
    }
}

#[test]
fn no_interface_crate_is_ever_pulled_in_below_the_shell() {
    for directory in ["sketch", "solid", "render", "core"] {
        let declared = declared_dependencies(&manifest(directory));
        for interface in INTERFACE_CRATES {
            assert!(
                !declared.contains(interface),
                "crates/{directory} depends on {interface}: only cao_app may know the interface",
            );
        }
    }

    for directory in ["sketch", "solid", "core"] {
        assert!(
            !declared_dependencies(&manifest(directory)).contains("wgpu"),
            "crates/{directory} depends on wgpu: the GPU stops at cao_render",
        );
    }
}

#[test]
fn only_the_named_files_reach_for_the_disk_the_clock_or_the_environment() {
    let allowed: BTreeSet<&str> = FILES_ALLOWED_TO_REACH_OUTSIDE.iter().copied().collect();
    let mut still_reaching: BTreeSet<&str> = BTreeSet::new();

    for (path, source) in sources_below_the_interface() {
        let reaches = REACHES_OUTSIDE.iter().any(|call| source.contains(call));
        match allowed.get(path.as_str()) {
            Some(known) => {
                if reaches {
                    still_reaching.insert(known);
                }
            }
            None => assert!(
                !reaches,
                "{path} touches the disk, the clock or the environment. \
                 Take a trait instead, and let the adapter do it.",
            ),
        }
    }

    assert_eq!(
        still_reaching, allowed,
        "a file no longer reaches outside: drop it from FILES_ALLOWED_TO_REACH_OUTSIDE",
    );
}

#[test]
fn text_meant_for_a_reader_never_sinks_below_the_interface() {
    let mut budget: BTreeMap<&str, usize> = READER_TEXT_LEFT_BELOW_THE_INTERFACE
        .iter()
        .copied()
        .collect();

    for (path, source) in sources_below_the_interface() {
        let found = reader_text_lines(&source);
        let left = budget.remove(path.as_str()).unwrap_or(0);

        assert_eq!(
            found, left,
            "{path} holds {found} lines of wording meant for a reader, {left} were left to it. \
             A layer below cao_app returns a named case, never a sentence; \
             once one is moved up, lower the figure here.",
        );
    }

    assert!(
        budget.is_empty(),
        "these files are gone but still hold a budget: {:?}",
        budget.keys().collect::<Vec<_>>(),
    );
}

fn workspace_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(Path::parent)
        .expect("the workspace root, two levels above crates/app")
        .to_path_buf()
}

fn manifest(directory: &str) -> String {
    let path = workspace_root()
        .join("crates")
        .join(directory)
        .join("Cargo.toml");
    fs::read_to_string(&path).unwrap_or_else(|_| panic!("the manifest at {}", path.display()))
}

fn declared_dependencies(manifest: &str) -> BTreeSet<String> {
    let mut names = BTreeSet::new();
    let mut inside = false;

    for line in manifest.lines() {
        let line = line.trim();
        if line.starts_with('[') {
            inside = line == "[dependencies]";
            continue;
        }
        if !inside || line.is_empty() || line.starts_with('#') {
            continue;
        }
        let name = line
            .split('=')
            .next()
            .unwrap_or_default()
            .split('.')
            .next()
            .unwrap_or_default()
            .trim();
        if !name.is_empty() {
            names.insert(name.to_string());
        }
    }
    names
}

fn sources_below_the_interface() -> Vec<(String, String)> {
    let root = workspace_root();
    let mut sources = Vec::new();

    for directory in CRATE_DIRECTORIES.iter().filter(|name| **name != "app") {
        for file in rust_files(&root.join("crates").join(directory).join("src")) {
            let path = file
                .strip_prefix(&root)
                .unwrap_or(&file)
                .display()
                .to_string()
                .replace('\\', "/");
            let source = fs::read_to_string(&file).expect("a readable source file");
            sources.push((path, source));
        }
    }
    sources
}

fn rust_files(directory: &Path) -> Vec<PathBuf> {
    let mut files = Vec::new();
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
                files.push(path);
            }
        }
    }
    files.sort();
    files
}

fn reader_text_lines(source: &str) -> usize {
    let production = source
        .lines()
        .position(|line| line.contains("#[cfg(test)]"))
        .unwrap_or(usize::MAX);

    source
        .lines()
        .take(production)
        .filter(|line| holds_reader_text(line))
        .count()
}

fn holds_reader_text(line: &str) -> bool {
    if line.trim_start().starts_with("//") {
        return false;
    }
    line.split('"')
        .skip(1)
        .step_by(2)
        .any(|literal| literal.chars().any(is_a_french_letter))
}

fn is_a_french_letter(character: char) -> bool {
    matches!(
        character,
        'é' | 'è'
            | 'ê'
            | 'ë'
            | 'à'
            | 'â'
            | 'ç'
            | 'ù'
            | 'û'
            | 'ô'
            | 'î'
            | 'ï'
            | 'œ'
            | 'É'
            | 'È'
            | 'À'
            | 'Ç'
            | '«'
            | '»'
    )
}
