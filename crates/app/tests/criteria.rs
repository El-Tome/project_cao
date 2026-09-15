//! What an issue asked for, held against the tests that answer it.
//!
//! The criteria of an issue are transcribed into the module documentation of
//! the test that covers them, rather than into a file of their own: one file
//! per issue accumulates without end, goes stale on the first rename, and sits
//! away from the code it describes. Beside the test, whoever renames the test
//! is already looking at the line that names it.
//!
//! ```text
//! //! Closes #317.
//! //! - a box dragged over an arc takes the arc — `a_box_catches_an_arc`
//! //! - the click was never broken — no test: `pick` already returns arcs
//! ```
//!
//! Closes #334.
//! - a bullet naming a test the file does not define is refused, and the
//!   complaint names it — `a_bullet_naming_a_test_the_file_does_not_define_is_refused`
//! - a bullet with neither a test nor a reason is refused —
//!   `a_bullet_with_neither_a_test_nor_a_reason_is_refused`
//! - a `Closes #n` block with no bullet is refused —
//!   `a_closes_block_with_no_bullet_at_all_is_refused`
//! - a file with no block is nobody's business —
//!   `a_file_with_no_closes_block_is_not_this_test_s_business`
//! - the parser is tried against strings, not against the repository — no test:
//!   every test above holds its own fixture, which is the shape of the file
//!   rather than something an assertion can reach
//! - one real issue is transcribed — no test: this block is it
//! - `open-a-task` asks for the transcription at the opening — no test: it is
//!   prose, held by language.rs and by nothing that asserts
//!
//! The issue wrote that marker `not done:`. `no test:` is what landed: a
//! criterion can be settled and still have no assertion to point at, and the
//! first two bullets above are exactly that. The word had to cover both.
//!
//! Only the documentation at the head of a file is read. `//!` is module
//! documentation, which is valid nowhere else — and a test holding an example
//! of a block in a string would otherwise be taken for a file that closes the
//! issue the example names. This file was that test.
//!
//! What this cannot tell: whether the transcription is faithful to the issue,
//! and whether the named test asserts what the bullet claims. Both need the
//! issue itself read, which is not a thing a test does. Saying so is the point
//! — a check that oversells itself is worse than none.

use std::fs;
use std::path::{Path, PathBuf};

#[test]
fn a_bullet_naming_a_test_the_file_does_not_define_is_refused() {
    let source = "\
//! Closes #317.
//! - a box over an arc takes the arc — `a_box_catches_an_arc`

#[test]
fn something_else() {}
";
    let complaints = complaints_about(source, "somewhere.rs");

    assert_eq!(complaints.len(), 1, "{complaints:?}");
    assert!(
        complaints[0].contains("a_box_catches_an_arc"),
        "the complaint has to name the test that is missing: {}",
        complaints[0],
    );
}

#[test]
fn a_bullet_with_neither_a_test_nor_a_reason_is_refused() {
    let source = "\
//! Closes #316.
//! - the typed values are left in the drawing as dimensions
";
    let complaints = complaints_about(source, "somewhere.rs");

    assert_eq!(complaints.len(), 1, "{complaints:?}");
    assert!(
        complaints[0].contains("no test:"),
        "the complaint has to say what would settle it: {}",
        complaints[0],
    );
}

#[test]
fn a_bullet_that_says_why_it_has_no_test_is_let_through() {
    let source = "\
//! Closes #317.
//! - the click was never broken — no test: pick already returns arcs
";

    assert!(complaints_about(source, "somewhere.rs").is_empty());
}

#[test]
fn a_bullet_that_claims_no_test_and_stops_there_is_refused() {
    let source = "\
//! Closes #317.
//! - the click was never broken — no test:
";

    assert_eq!(complaints_about(source, "somewhere.rs").len(), 1);
}

#[test]
fn a_bullet_naming_a_test_the_file_defines_is_let_through() {
    let source = "\
//! Closes #317.
//! - a box over an arc takes the arc — `a_box_catches_an_arc`

#[test]
fn a_box_catches_an_arc() {}
";

    assert!(complaints_about(source, "somewhere.rs").is_empty());
}

#[test]
fn a_closes_block_with_no_bullet_at_all_is_refused() {
    let source = "\
//! Closes #317.

#[test]
fn a_box_catches_an_arc() {}
";
    let complaints = complaints_about(source, "somewhere.rs");

    assert_eq!(complaints.len(), 1, "{complaints:?}");
    assert!(complaints[0].contains("317"), "{}", complaints[0]);
}

#[test]
fn a_file_with_no_closes_block_is_not_this_test_s_business() {
    let source = "\
//! What a box dragged across the drawing takes hold of.

#[test]
fn a_box_catches_an_arc() {}
";

    assert!(complaints_about(source, "somewhere.rs").is_empty());
}

#[test]
fn two_issues_closed_in_one_file_are_read_apart() {
    let source = "\
//! Closes #317.
//! - a box over an arc takes the arc — `a_box_catches_an_arc`
//!
//! Closes #320.
//! - the copy stays symmetric — `a_copy_stays_symmetric`

#[test]
fn a_box_catches_an_arc() {}
";
    let complaints = complaints_about(source, "somewhere.rs");

    assert_eq!(complaints.len(), 1, "{complaints:?}");
    assert!(
        complaints[0].contains("a_copy_stays_symmetric"),
        "{}",
        complaints[0]
    );
}

#[test]
fn a_criterion_wrapped_over_several_lines_is_one_criterion() {
    let source = "\
//! Closes #334.
//! - a bullet naming a test the file does not define is refused, and the
//!   complaint names it — `a_bullet_naming_a_test_is_refused`

#[test]
fn a_bullet_naming_a_test_is_refused() {}
";

    assert!(complaints_about(source, "somewhere.rs").is_empty());
}

#[test]
fn a_criterion_that_quotes_before_it_names_its_test_is_let_through() {
    let source = "\
//! Closes #334.
//! - a `Closes #n` block with no bullet is refused — `a_block_with_no_bullet_is_refused`

#[test]
fn a_block_with_no_bullet_is_refused() {}
";

    assert!(complaints_about(source, "somewhere.rs").is_empty());
}

#[test]
fn a_closes_line_below_the_code_is_not_module_documentation() {
    let source = "\
//! What a box dragged across the drawing takes hold of.

use std::fs;

//! Closes #999.
//! - something nobody ever wrote a test for
";

    assert!(complaints_about(source, "somewhere.rs").is_empty());
}

#[test]
fn every_criterion_transcribed_in_this_repository_is_answered_for() {
    let mut complaints = Vec::new();

    for file in rust_files(&workspace_root().join("crates")) {
        let source = fs::read_to_string(&file).unwrap_or_default();
        let path = file
            .strip_prefix(workspace_root())
            .unwrap_or(&file)
            .display()
            .to_string();
        complaints.extend(complaints_about(&source, &path));
    }

    assert!(
        complaints.is_empty(),
        "an issue was closed by a branch that did not answer for what it asked:\n{}",
        complaints.join("\n"),
    );
}

/// Every complaint one file's `Closes #n` blocks earn.
///
/// A file with no such block earns none: transcribing is asked of the branch
/// that closes an issue, not of every file in the repository.
fn complaints_about(source: &str, path: &str) -> Vec<String> {
    let mut complaints = Vec::new();

    for (issue, criteria) in blocks_in(source) {
        if criteria.is_empty() {
            complaints.push(format!(
                "{path} says it closes #{issue} and lists not one of the things that \
                 issue asked for. Transcribe its Done when, one bullet each, before \
                 the code rather than after it."
            ));
            continue;
        }
        for criterion in criteria {
            if let Some(complaint) = complaint_about(&criterion, source, path, &issue) {
                complaints.push(complaint);
            }
        }
    }

    complaints
}

/// The `Closes #n` blocks of a file's module documentation, each with the
/// criteria under it.
///
/// A criterion wrapped over several lines is one criterion: the names of the
/// tests this repository writes do not leave much of a line to say anything in.
fn blocks_in(source: &str) -> Vec<(String, Vec<String>)> {
    let mut blocks: Vec<(String, Vec<String>)> = Vec::new();
    let mut criterion: Option<String> = None;

    for line in source.lines() {
        let Some(doc) = line.trim_start().strip_prefix("//!") else {
            lay(&mut criterion, &mut blocks);
            if line.trim().is_empty() {
                continue;
            }
            break;
        };
        let doc = doc.trim();

        if let Some(number) = doc.strip_prefix("Closes #") {
            lay(&mut criterion, &mut blocks);
            let number = number
                .trim_end_matches('.')
                .split_whitespace()
                .next()
                .unwrap_or(number);
            blocks.push((number.to_string(), Vec::new()));
            continue;
        }

        if doc.is_empty() {
            lay(&mut criterion, &mut blocks);
            continue;
        }

        if let Some(bullet) = doc.strip_prefix("- ") {
            lay(&mut criterion, &mut blocks);
            if !blocks.is_empty() {
                criterion = Some(bullet.to_string());
            }
            continue;
        }

        if let Some(carried) = criterion.as_mut() {
            carried.push(' ');
            carried.push_str(doc);
        }
    }

    lay(&mut criterion, &mut blocks);
    blocks
}

fn lay(criterion: &mut Option<String>, blocks: &mut [(String, Vec<String>)]) {
    let Some(criterion) = criterion.take() else {
        return;
    };
    if let Some((_, criteria)) = blocks.last_mut() {
        criteria.push(criterion);
    }
}

/// A criterion answers for itself by naming the test that holds it, or by
/// saying `no test:` and why — which covers a criterion put off as well as one
/// settled by something an assertion cannot reach. `no test:` is read first: a
/// reason may quote a name in backticks without that name being a test.
///
/// Every backticked name in the criterion is tried, not the first: a sentence
/// quotes what it is about before it names what answers for it.
fn complaint_about(criterion: &str, source: &str, path: &str, issue: &str) -> Option<String> {
    if let Some(reason) = criterion.split("no test:").nth(1) {
        return reason.trim().is_empty().then(|| {
            format!(
                "{path}, on #{issue}: \"{criterion}\" claims no test and stops there. \
                 A criterion with no test says why, so that the next person knows \
                 whether it was a decision or an oversight."
            )
        });
    }

    let quoted: Vec<&str> = criterion
        .split('`')
        .skip(1)
        .step_by(2)
        .map(str::trim)
        .filter(|name| !name.is_empty())
        .collect();

    if quoted.is_empty() {
        return Some(format!(
            "{path}, on #{issue}: \"{criterion}\" answers for nothing. A criterion \
             names the test that holds it, in backticks, or says `no test:` and why."
        ));
    }

    let held = quoted
        .iter()
        .any(|name| source.contains(&format!("fn {name}(")));

    (!held).then(|| {
        format!(
            "{path}, on #{issue}: \"{criterion}\" names {quoted:?}, and this file \
             defines none of them as a test. Either the test was renamed and this \
             line follows it, or the criterion lost what answered for it."
        )
    })
}

fn workspace_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(Path::parent)
        .expect("the workspace root, two levels above crates/app")
        .to_path_buf()
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
