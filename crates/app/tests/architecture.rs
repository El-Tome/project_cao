//! The architecture rules, in a form that fails the build.
//!
//! The rules themselves live in `.claude/skills/architecture-rust`. Prose holds
//! until someone moves something; this is the part that keeps holding after.
//!
//! It lives under `cao_app` because that crate sits above every other one and
//! is the only place the whole graph is visible. It reads sources as text and
//! links against nothing, so a crate with no library target hosts it fine.
//!
//! Several of the rules are ratchets: the debt they describe already exists, so
//! they record exactly how much of it there is and refuse any more. The figures
//! are meant to fall to zero and the entries to disappear. Raising one to make
//! a test pass is the one move that empties this file of meaning.
//!
//! Some of the rules below describe folders nobody has created yet — `ui/`,
//! `ports/`, `adapters/`, `services/`. They pass over an empty set today and
//! bite the moment the first such file lands, which is the point: the rule is
//! already there when the work starts, rather than written afterwards against
//! code that has already chosen otherwise. `docs/code-layout.md` says what each
//! folder is for.

use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Path, PathBuf};

const CRATE_DIRECTORIES: [&str; 6] = ["sketch", "solid", "render", "part", "prefs", "app"];

/// The graph as it is, not a list of permissions. The test compares this with
/// the `cao_*` dependencies every manifest actually declares, so an edge named
/// here that no `Cargo.toml` carries fails just as surely as one nobody allowed.
/// An entry is added in the same commit as the dependency it describes.
const ALLOWED_EDGES: [(&str, &[&str]); 6] = [
    ("sketch", &[]),
    ("solid", &[]),
    ("render", &[]),
    ("part", &["cao_sketch", "cao_solid"]),
    ("prefs", &[]),
    (
        "app",
        &[
            "cao_part",
            "cao_prefs",
            "cao_render",
            "cao_sketch",
            "cao_solid",
        ],
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

/// An equality too, despite the name: a file listed here that has stopped
/// reaching outside fails the test. It is a ratchet — the entry disappears in
/// the commit that gives the file its port, and no entry is added ahead of the
/// code that needs it.
const FILES_ALLOWED_TO_REACH_OUTSIDE: [&str; 4] = [
    "crates/part/src/document.rs",
    "crates/prefs/src/recents.rs",
    "crates/prefs/src/settings.rs",
    "crates/prefs/src/storage.rs",
];

const READER_TEXT_LEFT_BELOW_THE_INTERFACE: [(&str, usize); 8] = [
    ("crates/part/src/errors.rs", 2),
    ("crates/part/src/history.rs", 19),
    ("crates/prefs/src/command.rs", 39),
    ("crates/prefs/src/settings.rs", 1),
    ("crates/prefs/src/shortcuts.rs", 5),
    ("crates/prefs/src/toolbar.rs", 4),
    ("crates/sketch/src/constraints.rs", 4),
    ("crates/sketch/src/plane.rs", 1),
];

/// Past this, a file is holding more than one responsibility. The figure is
/// arbitrary; what is not is that every file above it can be named.
const LINE_BUDGET: usize = 400;

const FILES_OVER_THE_LINE_BUDGET: [(&str, usize); 17] = [
    ("crates/app/src/app.rs", 630),
    ("crates/app/src/screens/annotations.rs", 519),
    ("crates/app/src/screens/ribbon.rs", 428),
    ("crates/app/src/screens/settings.rs", 801),
    ("crates/app/src/screens/sketch.rs", 432),
    ("crates/app/src/screens/viewport.rs", 4234),
    ("crates/part/src/history.rs", 539),
    ("crates/part/src/state.rs", 1297),
    ("crates/prefs/src/toolbar.rs", 475),
    ("crates/render/src/camera.rs", 528),
    ("crates/render/src/geometry.rs", 556),
    ("crates/render/src/renderer.rs", 426),
    ("crates/sketch/src/regions.rs", 549),
    ("crates/sketch/src/sketch.rs", 2384),
    ("crates/sketch/src/solver.rs", 1466),
    ("crates/solid/src/boolean.rs", 449),
    ("crates/solid/src/mesh.rs", 627),
];

const BUCKETS_NAMED_AFTER_NOTHING: [&str; 11] = [
    "util", "utils", "helper", "helpers", "common", "misc", "shared", "manager", "handler",
    "stuff", "various",
];

/// Widgets a screen styles by hand. Layout containers — `Frame`, `Area`,
/// `ScrollArea`, the panels — are not here: arranging a screen is a screen's
/// own business. Dressing a slider for the twenty-second time is not.
const WIDGETS_A_SCREEN_SHOULD_NOT_DRESS: [&str; 11] = [
    "egui::Button::",
    "egui::Label::",
    "egui::RichText::",
    "egui::TextEdit::",
    "egui::Slider::",
    "egui::ComboBox::",
    "egui::Checkbox::",
    "egui::DragValue::",
    "egui::SelectableLabel::",
    "egui::ProgressBar::",
    "egui::Hyperlink::",
];

const RAW_WIDGETS_LEFT_IN_THE_SCREENS: [(&str, usize); 5] = [
    ("crates/app/src/screens/history_tree.rs", 1),
    ("crates/app/src/screens/ribbon.rs", 2),
    ("crates/app/src/screens/settings.rs", 26),
    ("crates/app/src/screens/start_menu.rs", 1),
    ("crates/app/src/screens/viewport.rs", 1),
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
fn a_crate_reached_only_by_the_tests_is_still_read_off_the_manifest() {
    let manifest = "\
[package]
name = \"probe\"

[dependencies]
glam = \"0.33\"

[dev-dependencies]
egui = \"0.36\"

[build-dependencies]
cc = \"1\"

[target.'cfg(windows)'.dependencies]
winit = \"0.30\"

[dependencies.serde]
version = \"1\"
features = [\"derive\"]

[[bin]]
name = \"probe\"
";

    let expected: BTreeSet<String> = ["cc", "egui", "glam", "serde", "winit"]
        .iter()
        .map(|name| name.to_string())
        .collect();

    assert_eq!(declared_dependencies(manifest), expected);
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
    for directory in ["sketch", "solid", "render", "part", "prefs"] {
        let declared = declared_dependencies(&manifest(directory));
        for interface in INTERFACE_CRATES {
            assert!(
                !declared.contains(interface),
                "crates/{directory} depends on {interface}: only cao_app may know the interface",
            );
        }
    }

    for directory in ["sketch", "solid", "part", "prefs"] {
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
        if path.contains("/adapters/") {
            continue;
        }
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

#[test]
fn every_name_is_snake_case_and_none_of_them_is_a_bucket() {
    let buckets: BTreeSet<&str> = BUCKETS_NAMED_AFTER_NOTHING.iter().copied().collect();

    for (path, _) in every_source() {
        let inside_src = path.split("/src/").nth(1).expect("a path under src/");

        for name in inside_src.trim_end_matches(".rs").split('/') {
            assert!(
                name.chars().all(|character| character.is_ascii_lowercase()
                    || character.is_ascii_digit()
                    || character == '_'),
                "{path}: `{name}` is not snake_case. Rust module names are identifiers, \
                 so neither kebab-case nor a dotted suffix can ever name a file here. \
                 The role is carried by the folder — see docs/code-layout.md.",
            );
            assert!(
                !buckets.contains(name),
                "{path}: `{name}` names no responsibility, so everything ends up in it. \
                 Name what is inside, or put it with the thing it serves.",
            );
        }
    }
}

#[test]
fn a_file_that_outgrew_its_budget_has_to_be_split() {
    let budgets: BTreeMap<&str, usize> = FILES_OVER_THE_LINE_BUDGET.iter().copied().collect();
    let mut seen: BTreeSet<&str> = BTreeSet::new();

    for (path, source) in every_source() {
        let length = source.lines().count();
        let Some((known, budget)) = budgets.get_key_value(path.as_str()) else {
            assert!(
                length <= LINE_BUDGET,
                "{path} is {length} lines, past the {LINE_BUDGET} a single responsibility fits in. \
                 Split it, or say here what it is that it does.",
            );
            continue;
        };

        seen.insert(known);
        assert!(
            length <= *budget,
            "{path} grew from {budget} lines to {length}. It was already too long: \
             what you are adding belongs somewhere else.",
        );
        assert!(
            length > LINE_BUDGET,
            "{path} is down to {length} lines, back under the budget. \
             Drop it from FILES_OVER_THE_LINE_BUDGET.",
        );
    }

    let listed: BTreeSet<&str> = budgets.keys().copied().collect();
    assert_eq!(seen, listed, "these files are gone but still hold a budget",);
}

#[test]
fn a_screen_reaches_for_a_primitive_rather_than_dressing_a_widget() {
    let mut budget: BTreeMap<&str, usize> =
        RAW_WIDGETS_LEFT_IN_THE_SCREENS.iter().copied().collect();

    for (path, source) in sources_of("app") {
        if path.starts_with("crates/app/src/ui/") {
            continue;
        }
        let found = widgets_dressed_by_hand(&source);
        let left = budget.remove(path.as_str()).unwrap_or(0);

        assert_eq!(
            found, left,
            "{path} dresses {found} widgets by hand, {left} were left to it. \
             A styled widget belongs in crates/app/src/ui/, where every screen \
             gets the same one; once one is moved there, lower the figure here.",
        );
    }

    assert!(
        budget.is_empty(),
        "these files are gone but still hold a budget: {:?}",
        budget.keys().collect::<Vec<_>>(),
    );
}

#[test]
fn a_primitive_knows_the_interface_and_nothing_else() {
    for (path, source) in sources_of("app") {
        if !path.starts_with("crates/app/src/ui/") {
            continue;
        }

        for line in imports(&source) {
            assert!(
                !line.contains("cao_"),
                "{path} imports a workspace crate: {line}. \
                 A primitive takes plain values and hands back what the user did. \
                 Knowing the part is the screen's job.",
            );
            assert!(
                !line.contains("screens"),
                "{path} imports a screen: {line}. The arrow runs the other way.",
            );
        }
    }
}

#[test]
fn a_presenter_never_takes_the_interface() {
    for (path, source) in sources_of("app") {
        if !path.ends_with("/state.rs") && !path.ends_with("/presenter.rs") {
            continue;
        }

        for marker in ["egui::Ui", "&mut Ui", "ui: &"] {
            assert!(
                !production(&source).contains(marker),
                "{path} mentions {marker}. A presenter holds what the screen knows \
                 and answers what the view asks; the moment it draws, it can only be \
                 tested with a window open. Drawing goes to view.rs.",
            );
        }
    }
}

#[test]
fn the_layers_of_a_context_only_reach_downwards() {
    const DOWNWARDS: [(&str, [&str; 2]); 3] = [
        ("/model/", ["ports", "adapters"]),
        ("/ports/", ["adapters", "services"]),
        ("/services/", ["adapters", "screens"]),
    ];

    for (path, source) in every_source() {
        for (folder, forbidden) in DOWNWARDS {
            if !path.contains(folder) {
                continue;
            }
            for line in imports(&source) {
                for layer in forbidden {
                    assert!(
                        !line.split("::").any(|segment| segment.trim() == layer),
                        "{path} sits in {folder} and imports {layer}: {line}. \
                         A layer names what it needs and lets the one above wire it; \
                         see docs/code-layout.md.",
                    );
                }
            }
        }
    }
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

/// Every table that declares an edge, not just `[dependencies]`: a crate reached
/// only by the tests or only on one platform is still a crate this manifest
/// pulls in, and the rules below have nothing to say about it if it is invisible.
fn declared_dependencies(manifest: &str) -> BTreeSet<String> {
    let mut names = BTreeSet::new();
    let mut inside = false;

    for line in manifest.lines() {
        let line = line.trim();
        if let Some(header) = table_header(line) {
            inside = declares_dependencies(header);
            if let Some(name) = dependency_given_its_own_table(header) {
                names.insert(name.to_string());
            }
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

fn table_header(line: &str) -> Option<&str> {
    line.strip_prefix('[')?.strip_suffix(']')
}

fn declares_dependencies(header: &str) -> bool {
    matches!(
        header.rsplit('.').next(),
        Some("dependencies" | "dev-dependencies" | "build-dependencies")
    )
}

fn dependency_given_its_own_table(header: &str) -> Option<&str> {
    let (table, name) = header.rsplit_once('.')?;
    declares_dependencies(table).then_some(name)
}

fn sources_below_the_interface() -> Vec<(String, String)> {
    CRATE_DIRECTORIES
        .iter()
        .filter(|name| **name != "app")
        .flat_map(|directory| sources_of(directory))
        .collect()
}

fn every_source() -> Vec<(String, String)> {
    CRATE_DIRECTORIES
        .iter()
        .flat_map(|directory| sources_of(directory))
        .collect()
}

fn sources_of(directory: &str) -> Vec<(String, String)> {
    let root = workspace_root();
    let mut sources = Vec::new();

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
    sources
}

fn imports(source: &str) -> Vec<&str> {
    production(source)
        .lines()
        .map(str::trim)
        .filter(|line| line.starts_with("use ") || line.starts_with("pub use "))
        .collect()
}

fn widgets_dressed_by_hand(source: &str) -> usize {
    let body = production(source);
    WIDGETS_A_SCREEN_SHOULD_NOT_DRESS
        .iter()
        .map(|widget| body.matches(widget).count())
        .sum()
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

/// What ships, without the tests that check it. A French sentence or a hand-made
/// widget in a `#[cfg(test)]` module reaches no user and dresses no screen.
fn production(source: &str) -> &str {
    match source.find("#[cfg(test)]") {
        Some(offset) => &source[..offset],
        None => source,
    }
}

fn reader_text_lines(source: &str) -> usize {
    production(source)
        .lines()
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
