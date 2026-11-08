//! The pill a sketch shows over the 3D view while it is open: the sketch's
//! name and its plane, the degrees of freedom the solver leaves it, and the
//! button that finishes it (`docs/redesign/part-screen.md` P9, drawn in
//! `docs/redesign/screens/01-part-sketch-line.png`).
//!
//! Closes #727.
//! - while a sketch is open, a pill at the top centre of the 3D view reads
//!   « Esquisse N · <plane> », N the sketch's rank from one as the Part tree
//!   and the breadcrumb name it (key `part_tree.sketch`), then the count, then
//!   a primary button « Terminer ↵ » —
//!   `a_sketch_on_xy_shows_its_name_its_plane_and_a_finish_button_in_a_pill_at_the_top_centre`;
//!   a rank past one by
//!   `a_sketch_laid_on_a_kept_section_reads_that_section_s_name`, whose sketch
//!   is the part's second, and the count between the two by the tests of the
//!   third bullet
//! - the plane reads as the breadcrumb does: a sketch on an origin plane reads
//!   « Plan XY », and the other origin planes as `wording/plane.rs` `label`
//!   words them — `a_sketch_on_each_origin_plane_reads_that_plane_s_name`; one
//!   on a plane off the origin reads « Face de la pièce » —
//!   `a_sketch_off_the_origin_reads_as_a_face_of_the_part`; one laid on a kept
//!   section reads « Coupe N », N the section's id plus one as the Part tree
//!   names it — `a_sketch_laid_on_a_kept_section_reads_that_section_s_name`;
//!   and, decided on the way, if that section is no longer among the part's
//!   sections the pill falls back to the plane's name —
//!   `a_sketch_whose_section_is_no_longer_kept_reads_its_plane_s_name`
//! - the count is the solver's, `Sketch::freedom(document.scale())`, never a
//!   count of its own: n ≥ 2 reads « n libertés restantes » —
//!   `from_two_degrees_of_freedom_left_the_pill_counts_them_in_the_plural`; 1
//!   reads « 1 liberté restante » —
//!   `one_degree_of_freedom_left_reads_in_the_singular`; 0 reads « entièrement
//!   réglée » in green, a new success token of `ui/theme/tokens.rs` whose
//!   values are `docs/redesign/tokens.json` `state.success` and `success_text`
//!   — `a_sketch_with_no_freedom_left_reads_fully_set_in_the_success_colour`;
//!   and, decided on the way, a sketch with no point drawn yet shows no count
//!   at all — `a_sketch_with_nothing_drawn_yet_shows_no_count`
//! - the pill is gone outside a sketch: before one is opened, and once it is
//!   finished — `the_pill_is_gone_before_a_sketch_is_opened_and_once_it_is_finished`
//!
//! What must not break:
//! - clicking the pill's « Terminer ↵ » finishes the sketch exactly as ↵ and
//!   the ribbon's « Terminer » do (`Command::FinishSketch`) —
//!   `the_pill_s_finish_button_finishes_the_sketch_as_the_ribbon_s_does`; and
//!   ↵ and the ribbon's button still do —
//!   `enter_and_the_ribbon_s_finish_button_still_finish_the_sketch`
//! - a click on the pill's button does not reach the drawing beneath it: with
//!   the Line tool in hand, clicking « Terminer ↵ » places no point —
//!   `a_click_on_the_pill_s_finish_button_places_no_point_with_the_line_tool_in_hand`
//!
//! Two things were decided on the way, before this file was written. A sketch
//! laid on a section the part no longer keeps names its plane rather than a
//! section that is not there. And a sketch with nothing drawn shows no count:
//! the solver says 0 there, and an empty sketch announced as fully set, in
//! green, would mislead. Removing « Terminer » from the default ribbon is left
//! to the issue of the default ribbon; until then the ribbon's button is aimed
//! at by its whole name, `driver::FINISH_IN_THE_RIBBON`.
//!
//! The pill is told apart from the breadcrumb, which reads « Esquisse N » too
//! during a sketch, by where it stands: inside the view, which the title bar
//! is not. What it reads is taken off the row of its « Terminer ↵ », left to
//! right, so that a layout in one text or in several reads the same; the pill
//! itself is the rounded patch the frame paints around that button and the
//! sketch's name, and the button is primary when an amber patch is painted
//! under it.

mod driver;

use cao_app::theme::tokens;
use egui::accesskit::Role;
use egui_kittest::kittest::{NodeT, Queryable};
use glam::DVec3;

/// A point of the plane XY a new part's eye reaches before the other two
/// planes: where `clicking_the_drawing.rs` opens its sketch on XY.
const ON_THE_XY_PLANE: DVec3 = DVec3::new(-40.0, 40.0, 0.0);

/// How far below the top edge of the view the pill may start, in points. The
/// design leaves about sixteen; the rest is room for a margin chosen otherwise.
const NEAR_THE_TOP: f32 = 40.0;

/// How far the pill's middle may stand from the view's, in points.
const CENTRED_WITHIN: f32 = 1.0;

/// How far a patch may sit inside what it carries, in points: the rect the
/// tree gives a widget and the one painted around it are rounded apart.
const HUGGED_WITHIN: f32 = 0.5;

/// What betrays a piece of the pill among what the view holds: the sketch's
/// name, a plane, a section, a face, the count, the finish.
const WORDS_OF_THE_PILL: [&str; 8] = [
    "Esquisse", "Plan", "Coupe", "Face", "libert", "réglée", "Terminer", "↵",
];

#[test]
fn a_sketch_on_xy_shows_its_name_its_plane_and_a_finish_button_in_a_pill_at_the_top_centre() {
    let mut app = a_part("a_sketch_on_xy_shows_its_name_its_plane_and_a_finish_button");
    open_a_sketch_on_xy(&mut app);

    let view = driver::canvas(&app);
    let finish = the_pill_s_finish(&app).unwrap_or_else(|| {
        panic!(
            "no « Terminer ↵ » button is on screen while a sketch on XY is open; the view \
             {view:?} holds {:?}, and the screen names {:?}",
            readings_in_the_view(&app),
            driver::on_screen(&app),
        )
    });
    let row: Vec<(egui::Rect, String)> = readings_in_the_view(&app)
        .into_iter()
        .filter(|(at, _)| finish.y_range().contains(at.center().y))
        .collect();
    let read = one_line(&row);

    let mut wrong = Vec::new();
    let in_order = match (read.find("Esquisse 1 · Plan XY"), read.find("Terminer")) {
        (Some(name), Some(finish)) => name < finish,
        _ => false,
    };
    if !in_order {
        wrong.push(format!(
            "the pill's row reads « {read} », not « Esquisse 1 · Plan XY » then \
             « Terminer ↵ »"
        ));
    }

    let name = row
        .iter()
        .find(|(_, said)| said.contains("Esquisse 1"))
        .map(|(at, _)| *at);
    let pill = name.and_then(|name| {
        backgrounds_painted(&app)
            .into_iter()
            .filter(|(around, least_radius, _)| {
                *least_radius > 0.0
                    && around.expand(HUGGED_WITHIN).contains_rect(finish)
                    && around.expand(HUGGED_WITHIN).contains_rect(name)
            })
            .map(|(around, ..)| around)
            .min_by(|one, other| one.area().total_cmp(&other.area()))
    });
    match pill {
        None => wrong.push(format!(
            "nothing rounded is painted around both the sketch's name at {name:?} and \
             « Terminer ↵ » at {finish:?}"
        )),
        Some(pill) => {
            let off_centre = pill.center().x - view.center().x;
            if off_centre.abs() > CENTRED_WITHIN {
                wrong.push(format!(
                    "the pill {pill:?} stands {off_centre} points off the middle of the \
                     view {view:?}"
                ));
            }
            let below_the_top = pill.top() - view.top();
            if !(0.0..=NEAR_THE_TOP).contains(&below_the_top) {
                wrong.push(format!(
                    "the pill {pill:?} starts {below_the_top} points below the top of the \
                     view {view:?}, not within {NEAR_THE_TOP}"
                ));
            }
        }
    }

    let primary = backgrounds_painted(&app)
        .into_iter()
        .any(|(under, _, fill)| {
            fill == tokens::ACCENT_AMBER
                && under.expand(HUGGED_WITHIN).contains(finish.center())
                && pill.is_none_or(|pill| pill.expand(HUGGED_WITHIN).contains_rect(under))
        });
    if !primary {
        wrong.push(format!(
            "no amber patch is painted under « Terminer ↵ » at {finish:?}: it is not a \
             primary button"
        ));
    }

    assert!(
        wrong.is_empty(),
        "the sketch's pill is not as the redesign draws it:\n  {}",
        wrong.join("\n  "),
    );
}

#[test]
fn the_pill_is_gone_before_a_sketch_is_opened_and_once_it_is_finished() {
    let mut app = a_part("the_pill_is_gone_before_a_sketch_is_opened");
    let mut seen = vec![("a part just made", false, what_the_pill_shows(&app))];

    driver::click(&mut app, "Nouvelle esquisse");
    seen.push((
        "« Nouvelle esquisse » clicked, no plane chosen yet",
        false,
        what_the_pill_shows(&app),
    ));

    driver::click_at_world_point(&mut app, ON_THE_XY_PLANE);
    assert!(
        driver::is_enabled(&app, driver::FINISH_IN_THE_RIBBON),
        "the click on XY opened no sketch, showing {:?}",
        driver::on_screen(&app),
    );
    seen.push(("a sketch open on XY", true, what_the_pill_shows(&app)));

    driver::click(&mut app, driver::FINISH_IN_THE_RIBBON);
    seen.push(("the sketch finished", false, what_the_pill_shows(&app)));
    driver::click_the_button(&mut app, "Esquisse");
    assert!(
        !driver::is_enabled(&app, driver::FINISH_IN_THE_RIBBON),
        "the ribbon's « Terminer » left the sketch open, showing {:?}",
        driver::on_screen(&app),
    );

    let wrong: Vec<String> = seen
        .iter()
        .filter(|(_, wanted, shown)| {
            if *wanted {
                !shown.iter().any(|said| said.contains('↵'))
                    || !shown.iter().any(|said| said.contains("Esquisse 1"))
            } else {
                !shown.is_empty()
            }
        })
        .map(|(when, wanted, shown)| {
            let what = if *wanted { "wanted" } else { "wanted no" };
            format!("{when}: {what} pill, the screen shows {shown:?} of one")
        })
        .collect();
    assert!(
        wrong.is_empty(),
        "the pill does not follow the sketch:\n  {}",
        wrong.join("\n  "),
    );
}

fn a_part(test: &str) -> driver::App {
    let mut app = driver::open(test);
    driver::create_a_part(&mut app, "Platine");
    app
}

fn open_a_sketch_on_xy(app: &mut driver::App) {
    driver::click(app, "Nouvelle esquisse");
    driver::click_at_world_point(app, ON_THE_XY_PLANE);
    assert!(
        driver::is_enabled(app, driver::FINISH_IN_THE_RIBBON),
        "the click on XY opened no sketch, showing {:?}",
        driver::on_screen(app),
    );
}

/// The pill's own « Terminer ↵ », wherever it stands: a button named for
/// finishing with the key that does it recalled, which the ribbon's
/// « Terminer (Entrée) » is not.
fn the_pill_s_finish(app: &driver::App) -> Option<egui::Rect> {
    app.query_all_by_role(Role::Button)
        .find(|node| {
            node.accesskit_node()
                .label()
                .is_some_and(|read| read.starts_with("Terminer") && read.contains('↵'))
        })
        .map(|node| node.rect())
}

/// Every name and every text standing inside the view, left to right, with
/// where it stands: a text once, rather than once more for the run of glyphs
/// `egui` lays under it.
fn readings_in_the_view(app: &driver::App) -> Vec<(egui::Rect, String)> {
    let view = driver::canvas(app);
    let mut read: Vec<(egui::Rect, String)> = app
        .get_all_by(|_| true)
        .filter_map(|node| {
            let read = node.accesskit_node();
            read.bounding_box()?;
            if read.role() == Role::TextRun {
                return None;
            }
            let said = read
                .label()
                .or_else(|| read.value())
                .filter(|said| !said.is_empty())?;
            let at = node.rect();
            view.contains_rect(at).then_some((at, said))
        })
        .collect();
    read.sort_by(|one, other| one.0.left().total_cmp(&other.0.left()));
    read
}

/// What of the pill the screen shows: what the view holds that names a
/// sketch, a plane, a count or the finish, and the pill's « Terminer ↵ »
/// wherever it stands.
fn what_the_pill_shows(app: &driver::App) -> Vec<String> {
    let mut shown: Vec<String> = readings_in_the_view(app)
        .into_iter()
        .map(|(_, said)| said)
        .filter(|said| WORDS_OF_THE_PILL.iter().any(|word| said.contains(word)))
        .collect();
    if the_pill_s_finish(app).is_some() && !shown.iter().any(|said| said.contains('↵')) {
        shown.push("Terminer ↵".to_owned());
    }
    shown
}

/// A row's texts as one line, left to right, every run of blanks one space.
fn one_line(row: &[(egui::Rect, String)]) -> String {
    row.iter()
        .flat_map(|(_, said)| said.split_whitespace())
        .collect::<Vec<_>>()
        .join(" ")
}

/// Every rectangle the last frame painted with something to see — a fill or
/// an outline — with the least of its four corner radii and its fill.
fn backgrounds_painted(app: &driver::App) -> Vec<(egui::Rect, f32, egui::Color32)> {
    let mut found = Vec::new();
    let mut pending: Vec<&egui::Shape> = app
        .output()
        .shapes
        .iter()
        .map(|clipped| &clipped.shape)
        .collect();
    while let Some(shape) = pending.pop() {
        match shape {
            egui::Shape::Vec(group) => pending.extend(group),
            egui::Shape::Rect(rect) if rect.fill.a() > 0 || rect.stroke.width > 0.0 => {
                let corners = rect.corner_radius;
                let least = [corners.nw, corners.ne, corners.sw, corners.se]
                    .into_iter()
                    .min()
                    .unwrap_or_default();
                found.push((rect.rect, f32::from(least), rect.fill));
            }
            _ => {}
        }
    }
    found
}
