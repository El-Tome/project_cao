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
//!   `a_pair_of_numbers_that_becomes_a_point_is_a_screen_point_too`,
//!   `an_offset_from_the_canvas_or_from_the_screen_s_corner_is_a_screen_point_too`,
//!   `the_driver_reads_the_canvas_where_a_test_may_not`,
//!   `a_point_of_the_drawing_an_offset_a_size_or_a_pair_of_settings_is_not_one`,
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
//! them sat. So the gestures take an `egui::Pos2` — the compiler then refuses
//! a tuple — and the rule reads every way a `Pos2` is made of numbers, wherever
//! it stands: `pos2`, `Pos2::new` and `Pos2 { x, y }` of two numbers; an offset
//! of two numbers, `vec2`, turned into a point or added to the screen's corner;
//! a pair of numbers that becomes a point, handed to what builds one or kept in
//! a name the file builds one from; a helper building a point, of the file or
//! of the driver, called with two numbers. A read of the canvas outside the
//! driver is refused too: adding to where the canvas stands is aiming in pixels
//! again, by another road.
//!
//! What it leaves alone is what never becomes a point of the screen: a point
//! of the drawing (`DVec2::new`, `dvec2`, `DVec2::from`), an offset or a size
//! in pixels left as one, a helper of the test's own in millimetres, a pair of
//! numbers that is the two limits of a setting or two angles read back. A pair
//! like that needs no exemption of its own: it is refused only once the code
//! makes a point of it.

use std::fs;
use std::path::{Path, PathBuf};

/// What a driver test still writes in numbers, or reads of the window's
/// layout, each with why no point of the drawing can stand for it. Matched by
/// the file and by the expression as written, spacing aside — not by its line,
/// which every edit above it moves.
const WINDOW_POSITIONS: &[WindowPosition] = &[
    WindowPosition {
        file: "crates/app/tests/copying_in_a_panel.rs",
        written: "egui::pos2(900.0, 450.0)",
        why: "where the copy tools' panel was last let go, as the settings keep it: a \
              place of the window, top-left in screen points, that no point of the \
              drawing stands for",
    },
    WindowPosition {
        file: "crates/app/tests/clicking_the_drawing.rs",
        written: "driver::canvas(&app)",
        why: "the canvas read only to check that the window gives one taller than \
              wide, the case the offsets of #724 could not follow: nothing is aimed \
              from it",
    },
    WindowPosition {
        file: "crates/app/tests/holding_the_canvas.rs",
        written: "driver::canvas(&app)",
        why: "the canvas read only for its height, to check the driver holds it at \
              the height its fixtures were tuned on: nothing is aimed from it",
    },
    WindowPosition {
        file: "crates/app/tests/sketch_pill.rs",
        written: "driver::canvas(&app)",
        why: "the view read only to check that the sketch's pill stands at its top \
              centre, where the redesign puts it: nothing is aimed from it",
    },
    WindowPosition {
        file: "crates/app/tests/sketch_pill.rs",
        written: "driver::canvas(app)",
        why: "the view read only to tell the sketch's pill, which stands inside it, \
              from the breadcrumb above it, which reads the same sketch's name: \
              nothing is aimed from it",
    },
    WindowPosition {
        file: "crates/app/tests/view_bar.rs",
        written: "driver::canvas(app)",
        why: "the view read only to check that what « Recadrer » frames stands inside \
              it, fills it and sits in its middle, that the view bar stands \
              against its bottom-right corner, and where the scale bar and the \
              cube stand in theirs: nothing is aimed from it",
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

    for (path, refusals) in what_the_sweep_refuses() {
        let here: Vec<String> = refusals
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
        "driver tests that write a point of the screen in numbers, or read the canvas:\n  \
         {}\n\n\
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
    let refused = what_the_sweep_refuses();
    let mut wrong = Vec::new();

    for position in WINDOW_POSITIONS {
        if position.why.split_whitespace().count() < 8 {
            wrong.push(format!(
                "{} in {} does not say why it belongs to the window rather than to \
                 the drawing",
                position.written, position.file
            ));
        }
        let still_written = refused.iter().any(|(path, refusals)| {
            path == position.file && refusals.iter().any(|point| point.text == position.written)
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
fn a_pair_of_numbers_that_becomes_a_point_is_a_screen_point_too() {
    let source = r#"
const ON_THE_TRAIT: (f32, f32) = (850.0, 510.0);
const ON_THE_TOP_SIDE: (f32, f32) = (850.0, 400.0);

fn right((x, y): (f32, f32)) -> egui::Pos2 {
    egui::pos2(x + 320.0, y)
}

fn a_test() {
    let shifted = |(x, y): (f32, f32)| egui::pos2(x + 320.0, y);
    let at = |x: f32, y: f32| -> egui::Pos2 { egui::pos2(x, y) };
    driver::click_at(&mut app, right(ON_THE_TRAIT));
    driver::click_at(&mut app, shifted((900.0, 700.0)));
    driver::click_at(&mut app, at(700.0, 400.0));
    driver::click_at(&mut app, egui::pos2(ON_THE_TOP_SIDE.0, ON_THE_TOP_SIDE.1));
    let turned: egui::Pos2 = (1.5, 2.5).into();
    let kept = egui::Pos2::from([900.0, 450.0]);
}
"#;

    assert_eq!(
        texts(source),
        [
            "(850.0, 510.0)",
            "(850.0, 400.0)",
            "(900.0, 700.0)",
            "at(700.0, 400.0)",
            "(1.5, 2.5)",
            "[900.0, 450.0]",
        ],
    );
    assert_eq!(
        as_texts(refused(
            "driver::click_at(&mut app, driver::at(700.0, 400.0));",
            false,
            &["at".to_owned()],
        )),
        ["driver::at(700.0, 400.0)"],
        "a helper the driver declares builds a point in every file",
    );
}

#[test]
fn an_offset_from_the_canvas_or_from_the_screen_s_corner_is_a_screen_point_too() {
    let source = r#"
fn a_test() {
    let from_the_centre = driver::canvas(&app).center() + egui::vec2(-197.3, 59.5);
    let from_the_corner = driver::canvas(&app).min + egui::vec2(560.0, 300.0);
    let turned = egui::vec2(700.0, 400.0).to_pos2();
    let added = egui::Pos2::ZERO + egui::vec2(700.0, 400.0);
}
"#;

    assert_eq!(
        texts(source),
        [
            "driver::canvas(&app)",
            "driver::canvas(&app)",
            "egui::vec2(700.0, 400.0).to_pos2()",
            "egui::Pos2::ZERO + egui::vec2(700.0, 400.0)",
        ],
    );
}

#[test]
fn the_driver_reads_the_canvas_where_a_test_may_not() {
    let source = r#"
pub fn canvas(app: &App) -> egui::Rect {
    app.ctx.content_rect()
}

fn inside_the_canvas(app: &App, at: egui::Pos2) -> egui::Pos2 {
    let canvas = canvas(app);
    at
}
"#;

    assert_eq!(as_texts(refused(source, true, &[])), Vec::<String>::new());
    assert_eq!(texts(source), ["canvas(app)"]);
}

#[test]
fn a_point_of_the_drawing_an_offset_a_size_or_a_pair_of_settings_is_not_one() {
    let source = r#"
const ON_THE_YZ_PLANE: DVec3 = DVec3::new(0.0, 70.11, 3.02);
const AN_END: glam::DVec2 = glam::DVec2::new(22.7, 51.3);
const ANOTHER: glam::DVec2 = glam::dvec2(22.7, 51.3);
const A_THIRD: glam::DVec2 = glam::DVec2::from([22.7, 51.3]);

fn a_plate(width: f64, height: f64) -> driver::App {
    todo!()
}

fn a_test(right: impl Fn((f32, f32)) -> egui::Pos2) -> (f32, f32) {
    driver::click_at_sketch_point(&mut app, DVec2::new(22.7, 51.3));
    let app = a_plate(40.0, 20.0);
    settings.viewport.zoom_limits = (0.1, 50.0);
    let limits = (0.1, 50.0);
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
        refused(source, false, &[]),
        [Written {
            line: 4,
            text: "egui::pos2(700.0, 400.0)".to_owned(),
        }],
    );
}

/// What a test file outside the driver has refused, as written.
fn texts(source: &str) -> Vec<String> {
    as_texts(refused(source, false, &[]))
}

fn as_texts(refusals: Vec<Written>) -> Vec<String> {
    refusals.into_iter().map(|point| point.text).collect()
}

fn is_a_window_position(path: &str, text: &str) -> bool {
    WINDOW_POSITIONS
        .iter()
        .any(|position| position.file == path && position.written == text)
}

/// Every driver file, with what the rule refuses in it.
fn what_the_sweep_refuses() -> Vec<(String, Vec<Written>)> {
    let files = driver_files();
    let makers_of_the_driver: Vec<String> = files
        .iter()
        .filter(|(path, _)| is_the_driver(path))
        .flat_map(|(_, source)| pos2_makers_in(&code_alone(source)))
        .collect();
    files
        .into_iter()
        .map(|(path, source)| {
            let refusals = refused(&source, is_the_driver(&path), &makers_of_the_driver);
            (path, refusals)
        })
        .collect()
}

fn is_the_driver(path: &str) -> bool {
    path.starts_with("crates/app/tests/driver/")
}

/// A point of the screen written in numbers: the line it starts on, and the
/// expression as written, its spacing made even.
#[derive(Debug, PartialEq)]
struct Written {
    line: usize,
    text: String,
}

/// What the sweep refuses in one driver file: the points of the screen it
/// writes in numbers and, outside the driver, every read of the canvas. The
/// helpers that build a point the driver declares count in every file, since
/// every file compiles the driver.
fn refused(source: &str, in_the_driver: bool, makers_elsewhere: &[String]) -> Vec<Written> {
    let code = code_alone(source);
    let mut makers = pos2_makers_in(&code);
    makers.extend(makers_elsewhere.iter().cloned());
    let mut found = screen_points_in(&code, &makers);
    if !in_the_driver {
        found.extend(canvas_reads_in(&code));
    }
    found.sort_by_key(|point| point.line);
    found
}

/// Every point of the screen the code writes in numbers.
fn screen_points_in(code: &str, makers: &[String]) -> Vec<Written> {
    let bytes = code.as_bytes();
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
        let span = match bracket {
            b'{' => (last_segment(callee) == "Pos2" && fields_x_and_y_are_numbers(inside))
                .then_some(at..close + 1),
            _ if names_a_screen_point(callee) => {
                two_numbers(inside, false).then_some(at..close + 1)
            }
            _ if is_an_offset(callee) && two_numbers(inside, false) => {
                offset_made_a_point(code, at, close)
            }
            _ if is_bare(callee) => (two_numbers(inside, true)
                && pair_made_a_point(code, open, close, makers))
            .then_some(open..close + 1),
            _ if makes_a_point(callee, makers) && bracket == b'(' => {
                two_numbers(inside, true).then_some(at..close + 1)
            }
            _ => None,
        };
        if let Some(span) = span {
            found.push(Written {
                line: code[..span.start].matches('\n').count() + 1,
                text: as_written(&code[span]),
            });
        }
    }

    found
}

/// Every call of the canvas: where it stands is the driver's to read, since a
/// test that adds to it is aiming in pixels again — and what a variable
/// holding it is later added to is past what this reads.
fn canvas_reads_in(code: &str) -> Vec<Written> {
    let mut reads = Vec::new();
    for (open, _) in code.match_indices('(') {
        let Some((callee, at)) = callee_of(&code[..open]) else {
            continue;
        };
        if last_segment(callee) != "canvas" || code[..at].trim_end().ends_with("fn") {
            continue;
        }
        let Some(close) = matching(code.as_bytes(), open) else {
            continue;
        };
        reads.push(Written {
            line: code[..at].matches('\n').count() + 1,
            text: as_written(&code[at..=close]),
        });
    }
    reads
}

/// An offset in numbers made a point of the screen: turned into one, or added
/// to the screen's corner.
fn offset_made_a_point(code: &str, at: usize, close: usize) -> Option<std::ops::Range<usize>> {
    let after = &code[close + 1..];
    let turned = after.trim_start();
    if turned.starts_with(".to_pos2()") {
        let end = close + 1 + (after.len() - turned.len()) + ".to_pos2()".len();
        return Some(at..end);
    }
    let added_to = code[..at].trim_end().strip_suffix('+')?.trim_end();
    let corner = ["Pos2::ZERO", "Pos2::default()"]
        .into_iter()
        .find(|corner| added_to.ends_with(corner))?;
    let path = &added_to[..added_to.len() - corner.len()];
    let start = path
        .rfind(|c: char| !(c.is_ascii_alphanumeric() || c == '_' || c == ':'))
        .map_or(0, |at| at + 1);
    Some(start..close + 1)
}

/// Whether a bare pair of numbers becomes a point of the screen: handed to
/// what builds one, turned into one, or kept in a name the code then builds
/// one from. Any other pair — two limits of a setting, two angles read back —
/// is two numbers, not a place.
fn pair_made_a_point(code: &str, open: usize, close: usize, makers: &[String]) -> bool {
    if code[close + 1..].trim_start().starts_with(".into()") {
        return true;
    }
    if enclosing_call(code, open).is_some_and(|callee| makes_a_point(callee, makers)) {
        return true;
    }
    bound_name(code[..open].trim_end()).is_some_and(|name| made_a_point(code, name, makers))
}

/// Whether the code builds a point of the screen from what a name holds:
/// `pos2(NAME.0, …)`, a helper building one handed the name, or the name
/// turned into one.
fn made_a_point(code: &str, name: &str, makers: &[String]) -> bool {
    let bytes = code.as_bytes();
    let part_of_a_word =
        |at: Option<&u8>| at.is_some_and(|b| b.is_ascii_alphanumeric() || *b == b'_');
    code.match_indices(name).any(|(at, _)| {
        let end = at + name.len();
        if part_of_a_word(at.checked_sub(1).and_then(|before| bytes.get(before)))
            || part_of_a_word(bytes.get(end))
        {
            return false;
        }
        let after = &code[end..];
        if after.starts_with(".into()") {
            return true;
        }
        match enclosing_call(code, at) {
            Some(callee) if names_a_screen_point(callee) => {
                after.starts_with('.') || after.starts_with('[')
            }
            Some(callee) => makes_a_point(callee, makers),
            None => false,
        }
    })
}

/// The name a `const`, `static` or `let` binds what follows its `=` to.
fn bound_name(before: &str) -> Option<&str> {
    let binding = before.strip_suffix('=')?;
    if binding.ends_with(['=', '<', '>', '!', '+', '-', '*', '/']) {
        return None;
    }
    let statement = &binding[binding.rfind([';', '{', '}']).map_or(0, |at| at + 1)..];
    let mut words = statement
        .split(|c: char| !(c.is_ascii_alphanumeric() || c == '_'))
        .filter(|word| !word.is_empty());
    words.find(|word| matches!(*word, "const" | "static" | "let"))?;
    words.find(|word| *word != "mut")
}

/// The call whose brackets hold the place `at`, if it stands inside one.
fn enclosing_call(code: &str, at: usize) -> Option<&str> {
    let bytes = code.as_bytes();
    let mut depth = 0usize;
    for before in (0..at).rev() {
        match bytes[before] {
            b')' | b']' | b'}' => depth += 1,
            b'(' | b'[' | b'{' if depth > 0 => depth -= 1,
            b'(' => return callee_of(&code[..before]).map(|(callee, _)| callee),
            b'[' | b'{' | b';' => return None,
            _ => {}
        }
    }
    None
}

/// The helpers of a source that build a point of the screen: a function
/// declared to return a `Pos2`, a closure declared so, or one whose body is a
/// `pos2(…)`.
fn pos2_makers_in(code: &str) -> Vec<String> {
    let bytes = code.as_bytes();
    let starts_a_word =
        |at: usize| at == 0 || !(bytes[at - 1].is_ascii_alphanumeric() || bytes[at - 1] == b'_');
    let identifier = |text: &str| -> String {
        text.chars()
            .take_while(|c| c.is_ascii_alphanumeric() || *c == '_')
            .collect()
    };
    let returns_a_point = |after: &str| {
        after.strip_prefix("->").is_some_and(|returned| {
            let returned = &returned[..returned.find(['{', ';']).unwrap_or(returned.len())];
            last_segment(returned.split(" where").next().unwrap_or(returned).trim()) == "Pos2"
        })
    };
    let mut makers = Vec::new();

    for (at, _) in code
        .match_indices("fn ")
        .filter(|(at, _)| starts_a_word(*at))
    {
        let name = identifier(code[at + 3..].trim_start());
        let Some(open) = code[at..].find('(').map(|offset| at + offset) else {
            continue;
        };
        let Some(close) = matching(bytes, open) else {
            continue;
        };
        if !name.is_empty() && returns_a_point(code[close + 1..].trim_start()) {
            makers.push(name);
        }
    }

    for (at, _) in code
        .match_indices("let ")
        .filter(|(at, _)| starts_a_word(*at))
    {
        let rest = code[at + 4..].trim_start();
        let rest = rest.strip_prefix("mut ").unwrap_or(rest).trim_start();
        let name = identifier(rest);
        let Some(closure) = rest[name.len()..]
            .trim_start()
            .strip_prefix('=')
            .map(str::trim_start)
        else {
            continue;
        };
        let closure = closure.strip_prefix("move ").unwrap_or(closure);
        let Some(parameters) = closure.strip_prefix('|') else {
            continue;
        };
        let Some(end) = parameters.find('|') else {
            continue;
        };
        let body = parameters[end + 1..].trim_start();
        let path: String = body
            .chars()
            .take_while(|c| c.is_ascii_alphanumeric() || *c == '_' || *c == ':')
            .collect();
        let builds_one =
            body[path.len()..].trim_start().starts_with('(') && names_a_screen_point(&path);
        if !name.is_empty() && (returns_a_point(body) || builds_one) {
            makers.push(name);
        }
    }

    makers
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

/// What builds a point of the screen from what it is handed: `Pos2::from`, or
/// a helper of the file or of the driver.
fn makes_a_point(callee: &str, makers: &[String]) -> bool {
    callee.ends_with("Pos2::from") || makers.iter().any(|maker| maker == last_segment(callee))
}

/// An offset or a size in pixels — not a point of the screen until it is made
/// one. A point of the drawing, `DVec2::new`, `dvec2` or `DVec2::from`, is
/// neither, and nothing here reads it as one.
fn is_an_offset(callee: &str) -> bool {
    last_segment(callee) == "vec2" || callee == "Vec2::new" || callee.ends_with("::Vec2::new")
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
