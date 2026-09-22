//! The architecture rules, in a form that fails the build.
//!
//! Closes #362.
//! - no inline test module is left under a crate's src, and one added back is
//!   refused — `the_tests_of_a_module_live_in_a_file_of_their_own`
//! - a file's length says what its code weighs, and camera.rs has left the
//!   budget — `a_file_that_outgrew_its_budget_has_to_be_split`
//! - no test file is over the budget without an entry of its own, the budget
//!   reading every source, tests and code alike —
//!   `a_file_that_outgrew_its_budget_has_to_be_split`
//! - the same tests run as before — no test: it is a count, and it was taken
//!   by hand on either side of the sweep. Eight hundred and ninety-four,
//!   unchanged, which is the only thing that says no assertion moved
//!
//! Closes #385.
//! - render.rs is under budget or gone — `a_file_that_outgrew_its_budget_has_to_be_split`,
//!   which refused the commit until its entry was dropped
//! - the grid painter has a test that reads its vertices back onto the sketch
//!   plane — no test: the assertion is
//!   `every_vertex_of_the_grid_lies_in_the_sketch_plane`, colocated in
//!   `render/grid/tests.rs`, and a bullet may only name a test of its own file
//! - every file that earned a test of its own has left the list of places with
//!   no net — `the_places_with_no_net_are_the_ones_already_named`
//! - the transcription was written at the close rather than at the open, which
//!   is what `open-a-task` asks against — no test: it is a slip, recorded here
//!   rather than quietly fixed
//!
//! Closes #387.
//! - the answer stands beside the list, in `docs/code-map.md` — no test: it is
//!   prose, and an assertion on a sentence holds its wording rather than its
//!   reasoning
//! - it stands again where the rule lives, on `PLACES_ALLOWED_TO_HAVE_NO_NET`
//!   — no test: same, and the two copies say the same thing on purpose
//! - the rule itself does not move: a place still leaves the list by earning a
//!   test of its own — `a_place_said_to_carry_no_test_carries_none`
//!
//! Closes #392.
//! - a test file of any length passes the budget —
//!   `the_budget_weighs_code_and_leaves_a_test_file_alone`
//! - a production file past the budget still fails —
//!   `the_budget_weighs_code_and_leaves_a_test_file_alone`
//! - an integration test under `crates/<crate>/tests/` is not weighed either —
//!   `the_budget_weighs_code_and_leaves_a_test_file_alone`, which holds that
//!   the sweep reads `src` and nothing else
//! - `FILES_OVER_THE_LINE_BUDGET` keeps its four entries —
//!   `a_file_that_outgrew_its_budget_has_to_be_split`
//! - and refuses a fifth that is a test file —
//!   `the_list_of_files_over_the_budget_names_no_test_file`
//! - the three other sweeps over `every_source()` keep reading test files —
//!   `a_tests_file_nobody_declared_under_cfg_test_is_not_taken_for_tests`, which
//!   holds what a test file is, and the sweeps that skip one name it themselves
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

/// The heading in `docs/code-map.md` under which the uncovered places are
/// listed, one per bullet, each named by a path from the workspace root.
const NO_NET_HEADING: &str = "## What has no net";

/// Every place `docs/code-map.md` is allowed to list under [`NO_NET_HEADING`].
/// The list may only shrink: an entry added here is a place that went into the
/// repository with no test, which is the one move that empties the rule of
/// meaning.
///
/// A place leaves it by earning a test of its own, never by being walked
/// through from above. #387 weighed counting the headless driver's runs and
/// refused: no text proves which files a run touched, so the link would be
/// asserted by hand, and a ratchet asserted by hand drifts. The list is not a
/// coverage report — it says a change here is caught by nothing local, which a
/// test driving the whole application from outside does not make false.
/// `docs/code-map.md` carries the argument beside the list.
const PLACES_ALLOWED_TO_HAVE_NO_NET: [&str; 16] = [
    "crates/app/src/screens/annotations.rs",
    "crates/app/src/screens/extrusion_row.rs",
    "crates/app/src/screens/history_tree.rs",
    "crates/app/src/screens/mod.rs",
    "crates/app/src/screens/settings/",
    "crates/app/src/screens/start_menu.rs",
    "crates/app/src/screens/viewport/input/arcs.rs",
    "crates/app/src/screens/viewport/input/circles.rs",
    "crates/app/src/screens/viewport/input/constrain.rs",
    "crates/app/src/screens/viewport/input/mod.rs",
    "crates/app/src/screens/viewport/input/rectangle.rs",
    "crates/app/src/screens/viewport/input/resizing.rs",
    "crates/app/src/screens/viewport/input/symmetric_line.rs",
    "crates/app/src/screens/viewport/navigation.rs",
    "crates/app/src/screens/viewport/view.rs",
    "crates/sketch/src/solver.rs",
];

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
const FILES_ALLOWED_TO_REACH_OUTSIDE: [&str; 0] = [];

/// Past this, a file is holding more than one responsibility. The figure is
/// arbitrary; what is not is that every file above it can be named.
const LINE_BUDGET: usize = 400;

const FILES_OVER_THE_LINE_BUDGET: [(&str, usize); 4] = [
    ("crates/app/src/screens/viewport/input/mod.rs", 565),
    ("crates/render/src/renderer.rs", 426),
    ("crates/sketch/src/sketch.rs", 1000),
    ("crates/sketch/src/solver.rs", 1199),
];

/// Sweeps of the whole drawing, each with the test that walks the exhaustive
/// fixture rather than a list somebody typed. The list may only grow.
///
/// `duplicate` — which the mirror and both patterns go through, so it is one
/// sweep and not three — and `erase` already `match` on `Element` with no
/// wildcard, and the compiler has been holding them all along. They carry the
/// test all the same: a sweep held by a match today is a sweep somebody
/// rewrites as a chain of `if let` tomorrow, which is what `pick` is.
///
/// Owed: the `.caopart` round trip. The fixture is `#[cfg(test)]` inside
/// `cao_sketch`, so `cao_part` cannot see it without a `test-support` feature,
/// and #327 is building that machinery there.
const OPERATIONS_THAT_SWEEP_THE_WHOLE_DRAWING: [(&str, &str); 4] = [
    (
        "crates/sketch/src/banding.rs",
        "a_box_over_the_whole_drawing_catches_one_of_every_kind",
    ),
    (
        "crates/sketch/src/duplicating.rs",
        "a_copy_answers_for_one_of_every_kind",
    ),
    (
        "crates/sketch/src/element.rs",
        "erasing_answers_for_one_of_every_kind",
    ),
    (
        "crates/sketch/src/picking.rs",
        "a_click_on_the_drawing_finds_one_of_every_kind",
    ),
];

const SPOKEN_TO_A_DEVELOPER: [&str; 8] = [
    "#[error(",
    ".expect(",
    "panic!(",
    "assert!(",
    "assert_eq!(",
    "assert_ne!(",
    "unreachable!(",
    "todo!(",
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

/// Sentences `cao_app` still writes out instead of naming a key. #203 emptied
/// it, so the rule it was ratcheting towards now stands on its own: a file
/// added here is a sentence out of a translator's reach. A single character is
/// not counted — a glyph is drawn rather than read, which is why
/// `wording/constraints.rs` keeps its marks.
const SENTENCES_STILL_WRITTEN_OUT: [(&str, usize); 0] = [];

/// Modes under `screens/` that still decide and draw in the same place.
/// `explorer` and `ribbon` show the shape: a `state.rs` that holds what the
/// screen knows, a `view.rs` that draws it. The list may only shrink.
const MODES_WITHOUT_A_PRESENTER: [&str; 2] = ["settings", "sketch"];

const RAW_WIDGETS_LEFT_IN_THE_SCREENS: [(&str, usize); 5] = [
    ("crates/app/src/screens/extrusion_row.rs", 2),
    ("crates/app/src/screens/history_tree.rs", 1),
    ("crates/app/src/screens/settings/viewport.rs", 3),
    ("crates/app/src/screens/start_menu.rs", 1),
    ("crates/app/src/screens/viewport/render/live_fields.rs", 1),
];

#[test]
fn a_tests_file_nobody_declared_under_cfg_test_is_not_taken_for_tests() {
    assert!(
        is_nothing_but_tests("crates/app/src/screens/explorer/state/tests.rs"),
        "the file beside state.rs is declared under #[cfg(test)] and is tests",
    );
    assert!(
        is_nothing_but_tests("crates/part/src/document/tests.rs"),
        "the rule holds below the shell too, where the strictest of these tests runs",
    );
    assert!(!is_nothing_but_tests("crates/app/src/app.rs"));
    assert!(
        !is_nothing_but_tests("crates/app/src/screens/nothing/tests.rs"),
        "a tests.rs with no module above it ships",
    );
    assert!(!declares_tests_under_cfg("mod tests;"));
    assert!(declares_tests_under_cfg("#[cfg(test)]\nmod tests;"));
}

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
    let mut said_too_low: Vec<String> = Vec::new();

    for (path, source) in sources_below_the_interface() {
        if is_nothing_but_tests(&path) {
            continue;
        }
        for (number, literal) in reader_text_in(&source) {
            said_too_low.push(format!("{path}:{number}  \"{literal}\""));
        }
    }

    assert!(
        said_too_low.is_empty(),
        "a layer below cao_app is choosing words a reader will see:\n  {}\n\
         Return a named case and let cao_app say it. That is what makes \
         translation a wiring job rather than a rewrite.",
        said_too_low.join("\n  "),
    );
}

#[test]
fn a_sentence_the_interface_shows_is_named_rather_than_written_out() {
    let mut left: BTreeMap<&str, usize> = SENTENCES_STILL_WRITTEN_OUT.iter().copied().collect();

    for (path, source) in sources_of("app") {
        if is_nothing_but_tests(&path) {
            continue;
        }
        let written_out = sentences_written_out(&source);
        let allowed = left.remove(path.as_str()).unwrap_or(0);

        assert_eq!(
            written_out.len(),
            allowed,
            "{path} writes out {} sentences, {allowed} were left to it:\n  {}\n\
             A sentence the user reads belongs in crates/app/src/lang/fr.json under \
             a key the code names. Once one is moved, lower the figure here.",
            written_out.len(),
            written_out
                .iter()
                .map(|(number, literal)| format!("{path}:{number}  \"{literal}\""))
                .collect::<Vec<_>>()
                .join("\n  "),
        );
    }

    assert!(
        left.is_empty(),
        "these files are gone but are still owed sentences: {:?}",
        left.keys().collect::<Vec<_>>(),
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

/// Since #362 a module keeps its tests in a file of its own, so that a file's
/// length says what its code weighs rather than what checks it.
#[test]
fn the_tests_of_a_module_live_in_a_file_of_their_own() {
    let mut inline: Vec<String> = Vec::new();

    for (path, source) in every_source() {
        if source.contains("#[cfg(test)]\nmod tests {")
            || source.contains("#[cfg(test)]\npub(crate) mod tests {")
        {
            inline.push(path);
        }
    }

    assert!(
        inline.is_empty(),
        "these keep their tests inline: {inline:?}. A module's tests go in a \
         tests.rs of its own beside it — `#[cfg(test)] mod tests;` — so that \
         the length of the file says what its code weighs.",
    );
}

#[test]
fn a_file_that_outgrew_its_budget_has_to_be_split() {
    let budgets: BTreeMap<&str, usize> = FILES_OVER_THE_LINE_BUDGET.iter().copied().collect();
    let mut seen: BTreeSet<&str> = BTreeSet::new();

    for (path, source) in every_source() {
        if let Some((known, _)) = budgets.get_key_value(path.as_str()) {
            seen.insert(known);
        }
        if let Some(complaint) = what_the_budget_says(&path, source.lines().count(), &budgets) {
            panic!("{complaint}");
        }
    }

    let listed: BTreeSet<&str> = budgets.keys().copied().collect();
    assert_eq!(seen, listed, "these files are gone but still hold a budget",);
}

#[test]
fn the_budget_weighs_code_and_leaves_a_test_file_alone() {
    let nothing_listed = BTreeMap::new();
    let far_past = LINE_BUDGET + 100;

    assert!(
        what_the_budget_says("crates/app/src/app.rs", far_past, &nothing_listed).is_some(),
        "a production file past the budget still fails",
    );
    assert!(
        what_the_budget_says(
            "crates/part/src/document/tests.rs",
            far_past,
            &nothing_listed
        )
        .is_none(),
        "a tests.rs holds one responsibility, which is checking the module beside it",
    );
    assert!(
        what_the_budget_says(
            "crates/sketch/src/sketch/tests/rules.rs",
            far_past,
            &nothing_listed,
        )
        .is_none(),
        "so does a file the tests of a module are carved into by subject",
    );
    assert!(
        every_source()
            .iter()
            .all(|(path, _)| path.contains("/src/")),
        "the sweep reads src only, so an integration test under crates/<crate>/tests/ \
         is not weighed either",
    );
}

#[test]
fn the_list_of_files_over_the_budget_names_no_test_file() {
    let tests: Vec<&str> = FILES_OVER_THE_LINE_BUDGET
        .iter()
        .map(|(path, _)| *path)
        .filter(|path| is_nothing_but_tests(path))
        .collect();

    assert!(
        tests.is_empty(),
        "these are test files, and the budget does not weigh one: {tests:?}. \
         An entry here undoes that exemption from the other end.",
    );
}

#[test]
fn a_screen_reaches_for_a_primitive_rather_than_dressing_a_widget() {
    let mut budget: BTreeMap<&str, usize> =
        RAW_WIDGETS_LEFT_IN_THE_SCREENS.iter().copied().collect();

    for (path, source) in sources_of("app") {
        if path.starts_with("crates/app/src/ui/") || is_nothing_but_tests(&path) {
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
fn a_word_the_user_reads_is_said_without_drawing_it() {
    for (path, source) in sources_of("app") {
        if !path.starts_with("crates/app/src/wording/") {
            continue;
        }

        for line in imports(&source) {
            assert!(
                !line.contains("egui"),
                "{path} imports the interface: {line}. Wording turns a named case \
                 into a sentence and hands it back as text; how it is styled and \
                 where it is drawn is the screen's business.",
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
fn a_mode_keeps_what_it_knows_apart_from_what_it_draws() {
    let owed: BTreeSet<&str> = MODES_WITHOUT_A_PRESENTER.iter().copied().collect();
    let screens = workspace_root()
        .join("crates")
        .join("app")
        .join("src")
        .join("screens");

    let mut seen: BTreeSet<String> = BTreeSet::new();
    let mut folders: Vec<PathBuf> = fs::read_dir(&screens)
        .expect("a readable crates/app/src/screens")
        .flatten()
        .map(|entry| entry.path())
        .filter(|path| path.is_dir())
        .collect();
    folders.sort();

    for folder in folders {
        let name = folder
            .file_name()
            .unwrap_or_default()
            .to_string_lossy()
            .to_string();
        // A folder holding nothing but tests.rs is a file's tests, not a mode:
        // since #362 that is where every module keeps them.
        if only_tests(&folder) {
            continue;
        }
        let split = folder.join("state.rs").exists() && folder.join("view.rs").exists();

        if owed.contains(name.as_str()) {
            seen.insert(name.clone());
            assert!(
                !split,
                "screens/{name} holds its state.rs and its view.rs now. \
                 Drop it from MODES_WITHOUT_A_PRESENTER.",
            );
            continue;
        }

        assert!(
            split,
            "screens/{name} decides and draws in the same place, so nothing it \
             decides can be checked without opening a window. A mode carries a \
             state.rs that holds what the screen knows and a view.rs that draws \
             it — screens/ribbon is the shape to copy.",
        );
    }

    let gone: Vec<&&str> = owed.iter().filter(|name| !seen.contains(**name)).collect();
    assert!(
        gone.is_empty(),
        "{gone:?} are named as owing a presenter and no such mode is left. \
         Take them out.",
    );
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

#[test]
fn the_places_with_no_net_are_the_ones_already_named() {
    let allowed: BTreeSet<&str> = PLACES_ALLOWED_TO_HAVE_NO_NET.iter().copied().collect();
    let listed: BTreeSet<String> = places_with_no_net().into_iter().collect();

    let joined: Vec<&String> = listed
        .iter()
        .filter(|place| !allowed.contains(place.as_str()))
        .collect();
    assert!(
        joined.is_empty(),
        "docs/code-map.md has gained {joined:?} under \"{NO_NET_HEADING}\". A place \
         with no test is not added to this repository: write the test, or say in \
         PLACES_ALLOWED_TO_HAVE_NO_NET what is owed and why.",
    );

    let paid: Vec<&&str> = allowed
        .iter()
        .filter(|place| !listed.contains(**place))
        .collect();
    assert!(
        paid.is_empty(),
        "{paid:?} carry a test now, and docs/code-map.md says so. Drop them from \
         PLACES_ALLOWED_TO_HAVE_NO_NET.",
    );
}

#[test]
fn a_sweep_of_the_drawing_answers_for_one_of_every_kind() {
    for (path, test) in OPERATIONS_THAT_SWEEP_THE_WHOLE_DRAWING {
        // The sweep is in `path`; since #362 the test that holds it honest is
        // beside it, in the module's own tests file.
        let source = [path.to_string(), beside(path)]
            .iter()
            .filter_map(|at| fs::read_to_string(workspace_root().join(at)).ok())
            .collect::<String>();
        assert!(!source.is_empty(), "a readable {path}");

        assert!(
            source.contains(&format!("fn {test}(")),
            "{path} is said to sweep the whole drawing, and defines no {test}. \
             Either the test was renamed and this line follows it, or the sweep \
             lost the one thing that kept it honest.",
        );
        assert!(
            source.contains("one_of_every_kind"),
            "{path}'s {test} does not walk one_of_every_kind, so it enumerates the \
             kinds somebody remembered. That is #317, and it is why the fixture \
             makes the compiler keep the list instead.",
        );
    }
}

#[test]
fn a_place_said_to_carry_no_test_carries_none() {
    let places = places_with_no_net();
    assert!(
        !places.is_empty(),
        "docs/code-map.md lists nothing under \"{NO_NET_HEADING}\". Either the \
         section moved and this test follows it, or the last uncovered place \
         got a test and the section goes.",
    );

    for place in places {
        let path = workspace_root().join(&place);
        assert!(
            path.exists(),
            "docs/code-map.md says {place} carries no test, and no such path is \
             left. Take the line out.",
        );

        // Since #362 a module's tests live in a file beside it rather than in
        // it, so reading the file alone would clear every entry for ever.
        let files = if path.is_dir() {
            rust_files(&path)
        } else {
            let beside = workspace_root().join(beside(&place));
            [path, beside]
                .into_iter()
                .filter(|at| at.exists())
                .collect()
        };
        let covered: Vec<String> = files
            .iter()
            .filter(|file| {
                fs::read_to_string(file)
                    .unwrap_or_default()
                    .contains("#[test]")
            })
            .map(|file| file.display().to_string())
            .collect();

        assert!(
            covered.is_empty(),
            "docs/code-map.md says {place} carries no test, and these carry one: \
             {}. The net is there now — narrow the line or drop it, and the \
             skills pointing at it stop warning about a place that is covered.",
            covered.join(", "),
        );
    }
}

/// The list `docs/code-map.md` keeps under [`NO_NET_HEADING`]. One document
/// names the uncovered places and the skills point at it, because the previous
/// arrangement — the same names written out in the map and in each skill that
/// warned about them — drifted the day a test landed, in every copy at once.
fn places_with_no_net() -> Vec<String> {
    let map = fs::read_to_string(workspace_root().join("docs").join("code-map.md"))
        .expect("a readable docs/code-map.md");
    let below = map
        .split_once(NO_NET_HEADING)
        .unwrap_or_else(|| panic!("{NO_NET_HEADING} in docs/code-map.md"))
        .1;
    let section = below.split_once("\n## ").map_or(below, |(above, _)| above);

    section
        .lines()
        .filter_map(|line| line.trim_start().strip_prefix("- `"))
        .filter_map(|rest| rest.split_once('`'))
        .map(|(path, _)| path.to_string())
        .filter(|path| path.starts_with("crates/"))
        .collect()
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

/// The tests of a module, which live in a file of its own name since #362.
fn beside(path: &str) -> String {
    format!("{}/tests.rs", path.trim_end_matches(".rs"))
}

/// Whether a folder holds a module's tests and nothing else.
fn only_tests(folder: &Path) -> bool {
    fs::read_dir(folder)
        .map(|entries| {
            entries
                .flatten()
                .all(|entry| entry.file_name() == "tests.rs")
        })
        .unwrap_or(false)
}

/// What the budget has against a file of this length, or nothing.
///
/// A test file is not weighed. The figure says a file holds one responsibility,
/// and a tests.rs holds exactly one — checking the module beside it; applied
/// there it stops measuring a responsibility and starts measuring coverage.
fn what_the_budget_says(
    path: &str,
    length: usize,
    budgets: &BTreeMap<&str, usize>,
) -> Option<String> {
    if is_nothing_but_tests(path) {
        return None;
    }
    let Some(budget) = budgets.get(path) else {
        return (length > LINE_BUDGET).then(|| {
            format!(
                "{path} is {length} lines, past the {LINE_BUDGET} a single responsibility fits \
                 in. Split it, or say here what it is that it does."
            )
        });
    };
    if length > *budget {
        return Some(format!(
            "{path} grew from {budget} lines to {length}. It was already too long: \
             what you are adding belongs somewhere else."
        ));
    }
    (length <= LINE_BUDGET).then(|| {
        format!(
            "{path} is down to {length} lines, back under the budget. \
             Drop it from FILES_OVER_THE_LINE_BUDGET."
        )
    })
}

/// A file that holds nothing but tests. Since #362 every module keeps them in
/// a `tests.rs` beside it, and a long one is carved by subject into a `tests/`
/// folder under that.
///
/// The `mod tests;` line above is what is read, rather than the name alone: a
/// `tests.rs` nobody declared under `#[cfg(test)]` ships, and would be a hole
/// in every rule below that skips it.
fn is_nothing_but_tests(path: &str) -> bool {
    // A module's tests may be carved by subject, one file each, under the
    // tests file that declares them — which is how the drawing's own are kept.
    if let Some((folder, _)) = path.rsplit_once('/')
        && let Some(above) = folder.strip_suffix("/tests")
    {
        return is_nothing_but_tests(&format!("{above}/tests.rs"));
    }
    if !path.ends_with("/tests.rs") {
        return false;
    }
    let folder = match path.strip_suffix("/tests.rs") {
        Some(folder) => folder,
        None => return false,
    };
    [format!("{folder}.rs"), format!("{folder}/mod.rs")]
        .iter()
        .filter_map(|above| fs::read_to_string(workspace_root().join(above)).ok())
        .any(|source| declares_tests_under_cfg(&source))
}

/// Whether `mod tests;` in this source is behind `#[cfg(test)]`, whatever sits
/// between the two lines.
fn declares_tests_under_cfg(source: &str) -> bool {
    let Some(declaration) = source.find("mod tests;") else {
        return false;
    };
    source[..declaration]
        .rsplit_once("#[cfg(test)]")
        .is_some_and(|(_, between)| matches!(between.trim(), "" | "pub" | "pub(crate)"))
}

/// What ships, without the tests that check it. A French sentence or a hand-made
/// widget in a `#[cfg(test)]` module reaches no user and dresses no screen.
fn production(source: &str) -> &str {
    match source.find("#[cfg(test)]") {
        Some(offset) => &source[..offset],
        None => source,
    }
}

fn reader_text_in(source: &str) -> Vec<(usize, String)> {
    let mut found = Vec::new();
    for (index, line) in production(source).lines().enumerate() {
        for literal in literals_of(line) {
            if is_reader_text(literal, line) {
                found.push((index + 1, literal.to_string()));
            }
        }
    }
    found
}

/// What [`reader_text_in`] finds, minus the lone glyphs: `mark()` draws `T` for
/// a tangency and `X` for a fixed point, and a language file has nothing to say
/// about either.
fn sentences_written_out(source: &str) -> Vec<(usize, String)> {
    reader_text_in(source)
        .into_iter()
        .filter(|(_, literal)| literal.chars().count() > 1)
        .collect()
}

fn literals_of(line: &str) -> Vec<&str> {
    if line.trim_start().starts_with("//") {
        return Vec::new();
    }
    line.split('"').skip(1).step_by(2).collect()
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

fn is_reader_text(literal: &str, line: &str) -> bool {
    if speaks_to_a_developer(line) {
        return false;
    }
    literal.chars().any(is_a_french_letter)
        || is_a_capitalised_word(literal)
        || holds_two_words(literal)
}

/// Where a sentence is aimed at whoever is reading the crash, not at whoever is
/// using the software. `language.rs` is what keeps those in English.
fn speaks_to_a_developer(line: &str) -> bool {
    SPOKEN_TO_A_DEVELOPER
        .iter()
        .any(|position| line.contains(position))
}

fn holds_two_words(text: &str) -> bool {
    text.split(' ').filter(|word| is_a_word(word)).count() > 1
}

fn is_a_word(text: &str) -> bool {
    text.chars().count() > 1 && text.chars().all(char::is_alphabetic)
}

fn is_a_capitalised_word(text: &str) -> bool {
    let mut characters = text.chars();
    matches!(characters.next(), Some(first) if first.is_uppercase())
        && characters.all(char::is_lowercase)
}

#[test]
fn a_french_label_with_no_accent_is_still_text_a_reader_sees() {
    assert!(is_reader_text(
        "Enregistrer",
        "    let _ = \"Enregistrer\";"
    ));
}

#[test]
fn two_words_side_by_side_are_text_a_reader_sees() {
    assert!(is_reader_text(
        "Sans titre",
        "        \"Sans titre\".to_string()"
    ));
}

#[test]
fn a_sentence_only_a_developer_reads_stays_where_it_is() {
    assert!(!is_reader_text(
        "part file has no entry named {0}",
        "    #[error(\"part file has no entry named {0}\")]",
    ));
    assert!(!is_reader_text(
        "a cube tangent always points at another face",
        "        .expect(\"a cube tangent always points at another face\");",
    ));
}

#[test]
fn one_accented_word_on_its_own_is_text_a_reader_sees() {
    assert!(is_reader_text("créé", "    let _ = \"créé\";"));
}

#[test]
fn what_only_a_machine_reads_is_left_where_it_is() {
    for token in [
        "part.json",
        "caopart",
        "cao_scene_pipeline_layout",
        "shaders/scene.wgsl",
        "Theme::default_fixed",
        "test-support",
        "default_segment_snap_pixels",
        "{wanted} {suffix}",
        "{value:.1}",
        "CAO",
        "mm",
    ] {
        assert!(
            !is_reader_text(token, &format!("        \"{token}\"")),
            "{token} is a machine token and the rule took it for a sentence",
        );
    }
}
