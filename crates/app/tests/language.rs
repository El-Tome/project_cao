//! Everything a developer reads is in English. The interface is the one
//! exception, and it lives in `cao_app`.
//!
//! The rule used to say French documents were "translated when touched
//! anyway", which is how half the repository stayed French: a document nobody
//! had a reason to open never got touched. This test replaces the intention
//! with a check.
//!
//! What it catches: French prose in a document or in a Rust comment, and a
//! French word in a file or folder name. What it does not: a French name built
//! only of words missing from `FRENCH_IN_A_NAME` — the list grows the day one
//! slips through. Detection is by function words, so a lone French noun in an
//! English sentence passes; that is deliberate, since documents quote the
//! interface.

use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};

/// French function words with no English reading. `est`, `plus`, `son`, `sur`,
/// `on`, `a` and `en` are all French too and all excluded: they would fire on
/// ordinary English.
const FRENCH_FUNCTION_WORDS: [&str; 22] = [
    "le", "la", "les", "des", "du", "au", "aux", "une", "qui", "que", "dans", "pour", "avec",
    "sont", "cette", "cet", "donc", "elle", "ils", "nous", "vous", "mais",
];

/// French words seen in a name, or likely to be. A name is one or two words, so
/// a list is enough where prose needs a heuristic.
const FRENCH_IN_A_NAME: [&str; 20] = [
    "carte",
    "du",
    "de",
    "la",
    "le",
    "les",
    "un",
    "une",
    "et",
    "aux",
    "ouvrir",
    "tache",
    "revue",
    "verifier",
    "esquisse",
    "rendu",
    "historique",
    "outillage",
    "reglages",
    "sortie",
];

/// French a short sentence leans on even with no article and no accent. Only
/// words with no English reading: a test message counts things and denies them.
const FRENCH_WITH_NO_ARTICLE: [&str; 11] = [
    "un", "deux", "trois", "quatre", "cinq", "de", "et", "il", "ne", "pas", "rien",
];

/// Three of them in one file is prose, not a quoted interface string.
const FRENCH_WORDS_A_FILE_MAY_HOLD: usize = 2;

#[test]
fn no_document_and_no_comment_is_written_in_french() {
    let mut guilty = Vec::new();

    for path in files_worth_reading() {
        let relative = relative(&path);
        let text = fs::read_to_string(&path).unwrap_or_default();

        let prose = match kind(&path) {
            Some(Kind::Document) => prose_of_a_document(&text),
            Some(Kind::Rust) => prose_of_the_comments(&text, &["///", "//!", "//"]),
            Some(Kind::Shell) => prose_of_the_comments(&text, &["#"]),
            None => continue,
        };

        let found = french_words_in(&prose);
        if found.len() > FRENCH_WORDS_A_FILE_MAY_HOLD {
            guilty.push(format!("{relative} ({})", join(&found)));
        }
    }

    assert!(
        guilty.is_empty(),
        "French prose: {guilty:#?}\n\
         Everything a developer reads is English — see CLAUDE.md, Language. The\n\
         interface is the exception, and it is a string literal, never a comment.",
    );
}

/// Below `cao_app` no sentence is ever aimed at a user, so a French one is a
/// developer being spoken to in French — an assertion message, most of the
/// time. `architecture.rs` guards what ships; this guards what fails.
#[test]
fn no_string_below_the_interface_is_written_in_french() {
    let mut guilty = Vec::new();

    for path in files_worth_reading() {
        if !sits_below_the_interface(&path) {
            continue;
        }
        let text = fs::read_to_string(&path).unwrap_or_default();
        let relative = relative(&path);

        for (number, literal) in french_literals_in(&text) {
            guilty.push(format!("{relative}:{number}  \"{literal}\""));
        }
    }

    assert!(
        guilty.is_empty(),
        "French below cao_app:\n  {}\n\
         An assertion message is read by a developer, so it is English — see\n\
         CLAUDE.md, Language. French lives in cao_app and nowhere else.",
        guilty.join("\n  "),
    );
}

#[test]
fn no_file_and_no_folder_carries_a_french_name() {
    let french: BTreeSet<&str> = FRENCH_IN_A_NAME.iter().copied().collect();
    let mut guilty = Vec::new();

    for path in every_path_worth_naming() {
        let relative = relative(&path);
        let Some(name) = path.file_name().and_then(|name| name.to_str()) else {
            continue;
        };

        let words: Vec<&str> = name
            .split(|c: char| !c.is_ascii_alphabetic())
            .filter(|word| !word.is_empty())
            .filter(|word| french.contains(&word.to_ascii_lowercase().as_str()))
            .collect();

        if !words.is_empty() {
            guilty.push(format!("{relative} ({})", words.join(", ")));
        }
    }

    assert!(
        guilty.is_empty(),
        "French words in a name: {guilty:#?}\n\
         File and folder names are English too — see CLAUDE.md, Language.",
    );
}

/// A document without its code: fenced blocks, inline spans and quoted strings
/// are where the interface is cited word for word.
fn prose_of_a_document(text: &str) -> String {
    let mut prose = String::new();
    let mut fenced = false;

    for line in text.lines() {
        if line.trim_start().starts_with("```") {
            fenced = !fenced;
            continue;
        }
        if fenced || line.starts_with("    ") {
            continue;
        }
        prose.push_str(&without_citations(line));
        prose.push('\n');
    }

    prose
}

/// The comments alone. A string literal below here is the interface, and answers
/// to another rule.
fn prose_of_the_comments(text: &str, markers: &[&str]) -> String {
    text.lines()
        .filter_map(|line| {
            let line = line.trim_start();
            markers.iter().find_map(|marker| line.strip_prefix(marker))
        })
        .map(without_citations)
        .collect::<Vec<_>>()
        .join("\n")
}

/// What kind of file this is, as far as reading it goes.
enum Kind {
    Document,
    Rust,
    Shell,
}

fn kind(path: &Path) -> Option<Kind> {
    match extension(path) {
        Some("md") => Some(Kind::Document),
        Some("rs") => Some(Kind::Rust),
        Some("sh") => Some(Kind::Shell),
        // A git hook carries no extension, and is read all the same.
        None if path.file_name()? == "pre-commit" => Some(Kind::Shell),
        _ => None,
    }
}

/// Drops what sits between backticks and between double quotes: a document that
/// says the button reads "Nouvelle esquisse" is not written in French.
fn without_citations(line: &str) -> String {
    let mut kept = String::new();
    let mut quoted = false;
    let mut backticked = false;

    for character in line.chars() {
        match character {
            '`' => backticked = !backticked,
            '"' | '«' | '»' => quoted = !quoted,
            _ if !quoted && !backticked => kept.push(character),
            _ => {}
        }
    }

    kept
}

fn french_words_in(prose: &str) -> BTreeSet<String> {
    let french: BTreeSet<&str> = FRENCH_FUNCTION_WORDS.iter().copied().collect();

    prose
        .split(|c: char| !c.is_alphabetic())
        .filter(|word| !word.is_empty())
        .map(|word| word.to_lowercase())
        .filter(|word| french.contains(word.as_str()))
        .collect()
}

/// Every crate but `cao_app`, which is where the interface says things.
fn sits_below_the_interface(path: &Path) -> bool {
    let relative = relative(path).replace('\\', "/");
    extension(path) == Some("rs")
        && relative.starts_with("crates/")
        && !relative.starts_with("crates/app/")
}

fn french_literals_in(text: &str) -> Vec<(usize, String)> {
    let mut found = Vec::new();

    for (index, line) in text.lines().enumerate() {
        if line.trim_start().starts_with("//") {
            continue;
        }
        for literal in line.split('"').skip(1).step_by(2) {
            if is_written_in_french(literal) {
                found.push((index + 1, literal.to_string()));
            }
        }
    }

    found
}

/// A French sentence is caught by its grammar — an accent, an article, a
/// numeral, a negation. A lone French noun such as `rayon` still gets through,
/// and review is what catches that one.
fn is_written_in_french(literal: &str) -> bool {
    literal.chars().any(is_a_french_letter)
        || !french_words_in(literal).is_empty()
        || words_of(literal).any(|word| FRENCH_WITH_NO_ARTICLE.contains(&word.as_str()))
}

fn words_of(text: &str) -> impl Iterator<Item = String> + '_ {
    text.split(|c: char| !c.is_alphabetic())
        .filter(|word| !word.is_empty())
        .map(|word| word.to_lowercase())
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

#[test]
fn a_message_with_no_accent_is_french_all_the_same() {
    assert!(is_written_in_french("les deux traits tiennent"));
}

#[test]
fn an_english_message_is_left_where_it_is() {
    for message in [
        "both segments still hold the shared corner",
        "seed {seed}: point {rank} drifted to {point}",
        "a cube tangent always points at another face",
        "part.json",
    ] {
        assert!(
            !is_written_in_french(message),
            "{message} is English and the rule took it for French",
        );
    }
}

fn files_worth_reading() -> Vec<PathBuf> {
    every_path_worth_naming()
        .into_iter()
        .filter(|path| path.is_file())
        .collect()
}

fn every_path_worth_naming() -> Vec<PathBuf> {
    let mut found = Vec::new();
    walk(&workspace_root(), &mut found);
    found.sort();
    found
}

fn walk(directory: &Path, found: &mut Vec<PathBuf>) {
    let Ok(entries) = fs::read_dir(directory) else {
        return;
    };

    for entry in entries.flatten() {
        let path = entry.path();
        let name = entry.file_name();
        let name = name.to_string_lossy();

        if matches!(name.as_ref(), "target" | ".git" | ".auth") {
            continue;
        }

        found.push(path.clone());
        if path.is_dir() {
            walk(&path, found);
        }
    }
}

fn extension(path: &Path) -> Option<&str> {
    path.extension().and_then(|extension| extension.to_str())
}

fn relative(path: &Path) -> String {
    path.strip_prefix(workspace_root())
        .unwrap_or(path)
        .display()
        .to_string()
}

fn join(words: &BTreeSet<String>) -> String {
    words.iter().cloned().collect::<Vec<_>>().join(", ")
}

fn workspace_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(Path::parent)
        .expect("the workspace root, two levels above crates/app")
        .to_path_buf()
}
