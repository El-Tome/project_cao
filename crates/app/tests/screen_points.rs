//! A driver test names the point it clicks in the drawing's terms, never in
//! pixels of the screen. A point read off a rendered frame lands on something
//! else as soon as the chrome around the view moves, and the test then fails
//! for a reason that has nothing to do with what it checks: #724 broke a score
//! of them that way, in a sweep that only changed spacing and fonts.
//! `clicking_the_drawing.rs` holds the way a point of the drawing is clicked;
//! this file holds the rule that keeps the tests on it.
//!
//! Closes #752.
//! - a driver test that writes a screen point with two numeric literals fails:
//!   either `pos2(…)`, or a numeric tuple handed to `click_at`, `hover_at` or
//!   `drag_and_drop` — `a_driver_test_that_writes_a_screen_point_in_numbers_fails`,
//!   which reads every driver file and the driver itself, with
//!   `pos2_of_two_numbers_is_a_screen_point_written_in_numbers`,
//!   `a_pair_of_numbers_written_bare_or_handed_to_a_helper_is_a_screen_point_too`,
//!   `a_point_of_the_drawing_an_offset_a_size_or_a_place_read_off_the_frame_is_not_one`,
//!   `a_screen_point_in_a_comment_or_a_string_is_not_read` and
//!   `a_screen_point_spread_over_lines_is_found_on_the_line_it_starts` holding
//!   what it reads as one
//! - the few positions that genuinely belong to the window, such as where a
//!   floating panel is left, stay on a short allow-list, each with its reason —
//!   `a_position_that_belongs_to_the_window_is_let_through_with_its_reason`
//!
//! The issue named a numeric tuple handed to `click_at`, `hover_at` or
//! `drag_and_drop`. Read on one line, that misses the points kept in a
//! constant, or handed to a helper that moves them, which is where most of
//! them sat. So the gestures take an `egui::Pos2`, and the rule reads the
//! ways one is written in numbers wherever they stand: `pos2`, `Pos2::new`,
//! `Pos2 { x, y }`, a bare pair of numbers — a tuple or an array, which a
//! constant or a helper carries until it becomes one — and a function of the
//! test's own called with two of them. A window's size is written with
//! `egui::vec2`, as `egui` takes one; an offset in pixels too, and a point of
//! the drawing with `DVec2::new`.

use std::fs;
use std::path::{Path, PathBuf};

/// The places of the window a driver test writes in numbers, each with why
/// no point of the drawing can name it. Matched by the file and by the
/// expression as written, spacing aside — not by its line, which every edit
/// above it moves.
const WINDOW_POSITIONS: &[WindowPosition] = &[
    WindowPosition {
        file: "crates/app/tests/copying_in_a_panel.rs",
        written: "egui::pos2(900.0, 450.0)",
        why: "where the copy tools' panel was last let go, as the settings keep it: a \
              place of the window, top-left in screen points, that no point of the \
              drawing stands for",
    },
    WindowPosition {
        file: "crates/app/tests/variables.rs",
        written: "egui::pos2(10.0, 790.0)",
        why: "the pointer parked in the window's bottom-left corner, off the canvas \
              and off the panel being typed into, as a hand leaves the drawing",
    },
];

struct WindowPosition {
    file: &'static str,
    written: &'static str,
    why: &'static str,
}

#[test]
fn a_driver_test_that_writes_a_screen_point_in_numbers_fails() {
    let mut written = Vec::new();
    let mut by_file = Vec::new();

    for (path, source) in driver_files() {
        let here: Vec<String> = screen_points_written_in_numbers(&source)
            .into_iter()
            .filter(|point| !is_a_window_position(&path, &point.text))
            .map(|point| format!("{path}:{}  {}", point.line, point.text))
            .collect();
        if !here.is_empty() {
            by_file.push(format!("{path}: {}", here.len()));
        }
        written.extend(here);
    }

    assert!(
        written.is_empty(),
        "driver tests that write a point of the screen in numbers:\n  {}\n\n\
         by file:\n  {}\n\n\
         A point read off a rendered frame lands on something else as soon as the \
         chrome around the view moves. Name it in the drawing's terms instead — \
         driver::click_at_sketch_point or click_at_world_point, or \
         sketch_point_on_screen with a reach in pixels added on screen — or read it \
         off the frame: a widget's rect, a mark. A window's size is an egui::vec2. \
         A place that belongs to the window goes on WINDOW_POSITIONS, with why.",
        written.join("\n  "),
        by_file.join("\n  "),
    );
}

#[test]
fn a_position_that_belongs_to_the_window_is_let_through_with_its_reason() {
    let files = driver_files();
    let mut wrong = Vec::new();

    for position in WINDOW_POSITIONS {
        if position.why.split_whitespace().count() < 8 {
            wrong.push(format!(
                "{} in {} does not say why it belongs to the window rather than to \
                 the drawing",
                position.written, position.file
            ));
        }
        let still_written = files.iter().any(|(path, source)| {
            path == position.file
                && screen_points_written_in_numbers(source)
                    .iter()
                    .any(|point| point.text == position.written)
        });
        if !still_written {
            wrong.push(format!(
                "{} is no longer written in {}: drop it from WINDOW_POSITIONS",
                position.written, position.file
            ));
        }
    }

    assert!(wrong.is_empty(), "{}", wrong.join("\n"));
}

#[test]
fn pos2_of_two_numbers_is_a_screen_point_written_in_numbers() {
    let source = r#"
fn a_test() {
    driver::click_at(&mut app, egui::pos2(700.0, 400.0));
    app.hover_at(pos2(-10.0, 5.5));
    let at = egui::Pos2::new(1.0, 2.0);
    let there = egui::Pos2 { x: 3.0, y: 4.0 };
    let moved = egui::pos2(900.0 + 320.0, 700.0);
}
"#;

    assert_eq!(
        texts(source),
        [
            "egui::pos2(700.0, 400.0)",
            "pos2(-10.0, 5.5)",
            "egui::Pos2::new(1.0, 2.0)",
            "egui::Pos2 { x: 3.0, y: 4.0 }",
            "egui::pos2(900.0 + 320.0, 700.0)",
        ],
    );
}

#[test]
fn a_pair_of_numbers_written_bare_or_handed_to_a_helper_is_a_screen_point_too() {
    let source = r#"
const ON_THE_TRAIT: (f32, f32) = (850.0, 510.0);

fn a_test() {
    driver::click_at(&mut app, right((900.0, 700.0)));
    let kept = Some([900.0, 450.0]);
    driver::click_at(&mut app, at(700.0, 400.0));
    return (1.5, 2.5);
}
"#;

    assert_eq!(
        texts(source),
        [
            "(850.0, 510.0)",
            "(900.0, 700.0)",
            "[900.0, 450.0]",
            "at(700.0, 400.0)",
            "(1.5, 2.5)",
        ],
    );
}

#[test]
fn a_point_of_the_drawing_an_offset_a_size_or_a_place_read_off_the_frame_is_not_one() {
    let source = r#"
const ON_THE_YZ_PLANE: DVec3 = DVec3::new(0.0, 70.11, 3.02);
const AN_END: glam::DVec2 = glam::DVec2::new(22.7, 51.3);

fn a_test(right: impl Fn((f32, f32)) -> egui::Pos2) -> (f32, f32) {
    driver::click_at_sketch_point(&mut app, DVec2::new(22.7, 51.3));
    let pressed = on_the_top_side - egui::vec2(0.0, 22.0);
    let mut app = driver::open_on(test, egui::vec2(1920.0, 800.0));
    driver::click_at(&mut app, egui::pos2(field.left() + 18.0, field.center().y));
    driver::click_at(&mut app, right((at.x, at.y)));
    let share = ratio.clamp(0.0, 1.0);
    let pair = (0, 1);
    let typed: [f32; 2] = [left_at.x, left_at.y];
    let listed = vec![1.0, 2.0];
    assert_eq!((tilt.0, tilt.1), (30.0, -15.0), "a pair read back");
    assert!(worth < 1.5, "{}", format!("{}{}", 1.0, 2.0));
    (at.x, at.y)
}
"#;

    assert_eq!(texts(source), Vec::<String>::new());
}

#[test]
fn a_screen_point_in_a_comment_or_a_string_is_not_read() {
    let source = r##"
/// Clicked at pos2(700.0, 400.0) once, read off a frame.
fn a_test() {
    // driver::click_at(&mut app, egui::pos2(700.0, 400.0));
    /* (850.0, 510.0) */
    let said = "pos2(1.0, 2.0) \" (5.0, 6.0)";
    let raw = r#"(3.0, 4.0)"#;
    let quote = '(';
    let lifetime: &'static str = "x";
    let escaped = '\'';
}
"##;

    assert_eq!(texts(source), Vec::<String>::new());
}

#[test]
fn a_screen_point_spread_over_lines_is_found_on_the_line_it_starts() {
    let source = r#"fn a_test() {
    driver::click_at(
        &mut app,
        egui::pos2(
            700.0,
            400.0,
        ),
    );
}
"#;

    assert_eq!(
        screen_points_written_in_numbers(source),
        [Written {
            line: 4,
            text: "egui::pos2(700.0, 400.0)".to_owned(),
        }],
    );
}

fn texts(source: &str) -> Vec<String> {
    screen_points_written_in_numbers(source)
        .into_iter()
        .map(|point| point.text)
        .collect()
}

fn is_a_window_position(path: &str, text: &str) -> bool {
    WINDOW_POSITIONS
        .iter()
        .any(|position| position.file == path && position.written == text)
}

/// A point of the screen written in numbers: the line it starts on, and the
/// expression as written, its spacing made even.
#[derive(Debug, PartialEq)]
struct Written {
    line: usize,
    text: String,
}

/// Every point of the screen a source writes in numbers, read in its code
/// alone: what a comment or a string says is not a click.
fn screen_points_written_in_numbers(source: &str) -> Vec<Written> {
    let code = code_alone(source);
    let bytes = code.as_bytes();
    let compared = assertions_in(&code);
    let mut found = Vec::new();

    for (open, &bracket) in bytes.iter().enumerate() {
        if !matches!(bracket, b'(' | b'[' | b'{') {
            continue;
        }
        let Some(close) = matching(bytes, open) else {
            continue;
        };
        let Some((callee, at)) = callee_of(&code[..open]) else {
            continue;
        };
        let inside = &code[open + 1..close];
        let is_a_point = match bracket {
            b'{' => last_segment(callee) == "Pos2" && fields_x_and_y_are_numbers(inside),
            _ if is_bare(callee) && compared.iter().any(|span| span.contains(&open)) => false,
            b'[' => is_bare(callee) && two_numbers(inside, true),
            _ if names_a_screen_point(callee) => two_numbers(inside, false),
            _ if is_bare(callee) || !is_a_point_of_the_drawing_or_a_vector(callee) => {
                two_numbers(inside, true)
            }
            _ => false,
        };
        if is_a_point {
            let start = if is_bare(callee) { open } else { at };
            found.push(Written {
                line: code[..start].matches('\n').count() + 1,
                text: as_written(&code[start..=close]),
            });
        }
    }

    found
}

/// Where the assertions of a source stand: a pair of numbers one compares is a
/// value read back, not a place clicked.
fn assertions_in(code: &str) -> Vec<std::ops::Range<usize>> {
    let bytes = code.as_bytes();
    let mut spans = Vec::new();
    for (open, _) in code.match_indices('(') {
        let before = code[..open].trim_end();
        let Some(name) = before.strip_suffix('!') else {
            continue;
        };
        let name = name.rsplit(|c: char| !(c.is_ascii_alphanumeric() || c == '_'));
        if name
            .clone()
            .next()
            .is_some_and(|name| name.contains("assert"))
            && let Some(close) = matching(bytes, open)
        {
            spans.push(open..close);
        }
    }
    spans
}

/// The path written just before a bracket, empty for none, and where it
/// starts. A method or a macro builds no point here, and gives nothing.
fn callee_of(before: &str) -> Option<(&str, usize)> {
    let trimmed = before.trim_end();
    let start = trimmed
        .rfind(|c: char| !(c.is_ascii_alphanumeric() || c == '_' || c == ':'))
        .map_or(0, |at| at + 1);
    let callee = &trimmed[start..];
    let a_macro = callee.is_empty() && trimmed.ends_with('!');
    let a_method = !callee.is_empty() && trimmed[..start].ends_with('.');
    (!a_macro && !a_method).then_some((callee, start))
}

fn last_segment(callee: &str) -> &str {
    callee.rsplit("::").next().unwrap_or(callee)
}

/// No call at all: a bare tuple or array, or one standing after a keyword.
fn is_bare(callee: &str) -> bool {
    callee.is_empty()
        || matches!(
            callee,
            "return" | "in" | "else" | "match" | "if" | "while" | "break" | "yield"
        )
}

fn names_a_screen_point(callee: &str) -> bool {
    last_segment(callee) == "pos2" || callee.ends_with("Pos2::new")
}

/// What two numbers handed over build, other than a point of the screen: an
/// offset or a size in pixels, or a point of the drawing.
fn is_a_point_of_the_drawing_or_a_vector(callee: &str) -> bool {
    last_segment(callee) == "vec2" || callee.ends_with("Vec2::new")
}

/// Two numbers and nothing else, a trailing comma aside; when a float is
/// asked for, one of them is written as one, which leaves a pair of indices
/// alone.
fn two_numbers(inside: &str, a_float_asked_for: bool) -> bool {
    let arguments = top_level_parts(inside);
    let [one, other] = arguments.as_slice() else {
        return false;
    };
    let float = |part: &str| part.contains('.') || part.contains("f32") || part.contains("f64");
    is_a_number(one) && is_a_number(other) && (!a_float_asked_for || float(one) || float(other))
}

fn fields_x_and_y_are_numbers(inside: &str) -> bool {
    let fields = top_level_parts(inside);
    let value_of = |name: &str| {
        fields.iter().find_map(|field| {
            let (key, value) = field.split_once(':')?;
            (key.trim() == name).then_some(value.trim())
        })
    };
    fields.len() == 2
        && value_of("x").is_some_and(is_a_number)
        && value_of("y").is_some_and(is_a_number)
}

/// A literal, or literals put together with arithmetic.
fn is_a_number(part: &str) -> bool {
    let bare = part.replace("f32", "").replace("f64", "");
    bare.chars().any(|c| c.is_ascii_digit())
        && bare
            .chars()
            .all(|c| c.is_ascii_digit() || "._+-*/() \t\n".contains(c))
}

/// What a bracket holds, cut at its own commas, a trailing one aside.
fn top_level_parts(inside: &str) -> Vec<&str> {
    let mut parts = Vec::new();
    let mut depth = 0usize;
    let mut start = 0;
    for (at, c) in inside.char_indices() {
        match c {
            '(' | '[' | '{' => depth += 1,
            ')' | ']' | '}' => depth = depth.saturating_sub(1),
            ',' if depth == 0 => {
                parts.push(inside[start..at].trim());
                start = at + 1;
            }
            _ => {}
        }
    }
    let last = inside[start..].trim();
    if !last.is_empty() || parts.is_empty() {
        parts.push(last);
    }
    parts
}

/// Where the bracket opened at `open` closes.
fn matching(bytes: &[u8], open: usize) -> Option<usize> {
    let mut depth = 0usize;
    for (at, &byte) in bytes.iter().enumerate().skip(open) {
        match byte {
            b'(' | b'[' | b'{' => depth += 1,
            b')' | b']' | b'}' => {
                depth -= 1;
                if depth == 0 {
                    return Some(at);
                }
            }
            _ => {}
        }
    }
    None
}

/// The expression with its spacing made even, the way a line written by hand
/// and one `rustfmt` spread over several read the same.
fn as_written(span: &str) -> String {
    span.split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
        .replace("( ", "(")
        .replace("[ ", "[")
        .replace(" )", ")")
        .replace(" ]", "]")
        .replace(",)", ")")
        .replace(",]", "]")
}

/// The source with what comments say and what strings and characters hold
/// blanked out, lines kept where they were.
fn code_alone(source: &str) -> String {
    let chars: Vec<char> = source.chars().collect();
    let mut code = String::with_capacity(source.len());
    let blank = |code: &mut String, c: char| code.push(if c == '\n' { '\n' } else { ' ' });
    let mut at = 0;

    while at < chars.len() {
        let c = chars[at];
        let next = chars.get(at + 1).copied();
        let after_an_identifier =
            at > 0 && (chars[at - 1].is_alphanumeric() || chars[at - 1] == '_');

        if c == '/' && next == Some('/') {
            while at < chars.len() && chars[at] != '\n' {
                blank(&mut code, chars[at]);
                at += 1;
            }
        } else if c == '/' && next == Some('*') {
            let mut depth = 0usize;
            while at < chars.len() {
                if chars[at] == '/' && chars.get(at + 1) == Some(&'*') {
                    depth += 1;
                    code.push_str("  ");
                    at += 2;
                } else if chars[at] == '*' && chars.get(at + 1) == Some(&'/') {
                    depth -= 1;
                    code.push_str("  ");
                    at += 2;
                    if depth == 0 {
                        break;
                    }
                } else {
                    blank(&mut code, chars[at]);
                    at += 1;
                }
            }
        } else if c == 'r' && !after_an_identifier && matches!(next, Some('"' | '#')) {
            let hashes = chars[at + 1..].iter().take_while(|&&c| c == '#').count();
            if chars.get(at + 1 + hashes) != Some(&'"') {
                code.push(c);
                at += 1;
                continue;
            }
            code.push('"');
            at += 2 + hashes;
            while at < chars.len() {
                let closes = chars[at] == '"'
                    && chars[at + 1..]
                        .iter()
                        .take(hashes)
                        .filter(|&&c| c == '#')
                        .count()
                        == hashes;
                if closes {
                    code.push('"');
                    at += 1 + hashes;
                    break;
                }
                blank(&mut code, chars[at]);
                at += 1;
            }
        } else if c == '"' {
            code.push('"');
            at += 1;
            while at < chars.len() && chars[at] != '"' {
                if chars[at] == '\\' {
                    blank(&mut code, chars[at]);
                    at += 1;
                }
                if at < chars.len() {
                    blank(&mut code, chars[at]);
                    at += 1;
                }
            }
            code.push('"');
            at += 1;
        } else if c == '\'' && (next == Some('\\') || chars.get(at + 2) == Some(&'\'')) {
            code.push('\'');
            at += 1;
            while at < chars.len() && chars[at] != '\'' {
                if chars[at] == '\\' {
                    blank(&mut code, chars[at]);
                    at += 1;
                }
                if at < chars.len() {
                    blank(&mut code, chars[at]);
                    at += 1;
                }
            }
            code.push('\'');
            at += 1;
        } else {
            code.push(c);
            at += 1;
        }
    }

    code
}

/// Every test file that drives the application, and the driver itself.
fn driver_files() -> Vec<(String, String)> {
    let tests = workspace_root().join("crates/app/tests");
    let mut files = Vec::new();

    for path in rust_files(&tests) {
        let Ok(source) = fs::read_to_string(&path) else {
            continue;
        };
        let in_the_driver = path.starts_with(tests.join("driver"));
        let drives = source.lines().any(|line| line.trim() == "mod driver;");
        if in_the_driver || drives {
            let relative = path
                .strip_prefix(workspace_root())
                .unwrap_or(&path)
                .display()
                .to_string();
            files.push((relative, source));
        }
    }

    files
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
