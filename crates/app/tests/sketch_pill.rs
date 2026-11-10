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
//! - found trying the branch: the pill's texts stand on the middle line of its
//!   button, the design drawing them all on one line —
//!   `the_pill_s_texts_stand_on_the_middle_line_of_its_button`
//! - decided in review: the button names the keys that finish the sketch, as
//!   the ribbon does — « Terminer ↵ » while that is Enter alone, which the
//!   first test holds; « Terminer » and the chord as the ribbon writes it
//!   while it is any other —
//!   `the_pill_s_button_names_the_keys_that_finish_the_sketch_once_rebound`;
//!   « Terminer » alone while nothing does —
//!   `the_pill_s_button_names_no_key_once_finishing_is_unbound`
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
//! - found trying the branch: a sketch whose drawing was all erased shows no
//!   count either, something drawn being a point besides the origin still
//!   there rather than one ever laid —
//!   `a_sketch_whose_drawing_is_all_erased_shows_no_count`, and the rule
//!   itself in `crates/sketch/src/settled/tests.rs` by
//!   `a_sketch_holds_a_drawing_only_while_a_point_besides_the_origin_is_left`
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
//! - decided on the way: a drag begun in the view carries on over the pill: a
//!   point pressed in the drawing, dragged into the pill's frame and let go
//!   there lands where it was let go, in one step —
//!   `a_drag_begun_in_the_view_carries_on_over_the_pill_and_lands_where_it_is_let_go`
//!
//! Two things were decided on the way, before this file was written. A sketch
//! laid on a section the part no longer keeps names its plane rather than a
//! section that is not there. And a sketch with nothing drawn shows no count:
//! the solver says 0 there, and an empty sketch announced as fully set, in
//! green, would mislead. A third came while the count was laid in: the pill
//! stands over the top of the view, and a pull of the drawing let go on it lost
//! its gesture, so the pill takes no pointer while something is being dragged.
//! « Terminer » then left the default ribbon (#732), where a user can still
//! put it back: the tests that compare the pill's button with the ribbon's do
//! so on settings that have it back, `driver::the_moved_commands_put_back`.
//!
//! The pill is told apart from the breadcrumb, which reads « Esquisse N » too
//! during a sketch, by where it stands: inside the view, which the title bar
//! is not. What it reads is taken off the row of its « Terminer ↵ », left to
//! right, so that a layout in one text or in several reads the same; the pill
//! itself is the rounded patch the frame paints around that button and the
//! sketch's name, and the button is primary when an amber patch is painted
//! under it.
//!
//! A section is named by its id, which deleting another leaves alone, and not
//! by its place among the sections still kept. The sketch read on a kept
//! section is laid on the second one kept, once the first is deleted: a pill
//! counting the sections left would read « Coupe 1 » where the tree reads
//! « Coupe 2 ». A section deleted under a sketch leaves the sketch where it
//! stood, and the step that opened the sketch still names that section: the
//! sketch reopened then reads its plane's name, as decided on the way, and
//! not the name of a section the tree no longer shows.
//!
//! The count is held against the solver rather than against a number written
//! here: the pill is read while the sketch is open, then the part is put away
//! and `Sketch::freedom` asked of the sketch it saved, at the part's scale.
//! Each test first checks that the solver still puts its drawing in the case
//! it is about — two or more, one, none — and a rectangle is among them
//! because the solver leaves it fewer degrees than its points and rules
//! counted one by one would. The count stands between the plane and
//! « Terminer ↵ », read word for word, so that « 1 liberté restante » is not
//! found inside « 1 liberté restantes ». The green of « entièrement réglée »
//! is `state.success_text`, read off the design file; the token that holds it
//! is `STATE_SUCCESS_TEXT`, which `theme.rs` holds to the same value.
//!
//! Finishing as the ribbon does is read off what the screen is left showing:
//! every button by its name, whether it can be pressed and whether it is on,
//! and every field — which holds the ribbon turned to its Extrusion tab and
//! whatever the extrusion offered shows — besides the sketch closed and the
//! pill gone. The pill's button is clicked by a pointer at its middle, as a
//! hand would, so that the drawing beneath would hear the click if the pill let
//! it through. A line begun and a click just below the pill lays a trait, the
//! leak the line test watches the history and the saved sketch for.

mod driver;

use std::fs;
use std::path::Path;

use cao_app::theme::tokens;
use cao_prefs::command::Command;
use cao_prefs::shortcuts::{Chord, Key};
use egui::accesskit::Role;
use egui_kittest::kittest::{NodeT, Queryable};
use glam::{DVec2, DVec3};

/// The far corner of a rectangle drawn from the origin on the plane YZ, a node
/// of the grid; a point of its top side and of its right side, the middle of
/// each, and where a dimension of each is left, clear of the rest.
const ITS_FAR_CORNER: DVec2 = DVec2::new(137.5, 37.5);
const ON_ITS_TOP_SIDE: DVec2 = DVec2::new(68.75, 37.5);
const ABOVE_ITS_TOP_SIDE: DVec2 = DVec2::new(68.75, 60.0);
const ON_ITS_RIGHT_SIDE: DVec2 = DVec2::new(137.5, 18.75);
const RIGHT_OF_ITS_RIGHT_SIDE: DVec2 = DVec2::new(160.0, 18.75);

/// A point of the plane XY a new part's eye reaches before the other two
/// planes: where `clicking_the_drawing.rs` opens its sketch on XY.
const ON_THE_XY_PLANE: DVec3 = DVec3::new(40.0, -40.0, 0.0);

/// A point of each plane of the origin a new part's eye reaches before the
/// other two, with the name `wording/plane.rs` gives that plane: those of
/// `clicking_the_drawing.rs`.
const ON_EACH_BASE_PLANE: [(DVec3, &str); 3] = [
    (DVec3::new(0.0, -40.0, 30.0), "Plan YZ"),
    (ON_THE_XY_PLANE, "Plan XY"),
    (DVec3::new(40.0, 0.0, 30.0), "Plan XZ"),
];

/// On the block's front face, 10 mm off the plane its rectangle was drawn on,
/// low on it: where `keeping_a_section.rs` clicks the face a section is laid
/// on.
const ON_THE_BLOCK_S_FRONT_FACE: DVec3 = DVec3::new(10.0, 41.51, -19.82);

/// How far below the top edge of the view the pill may start, in points. The
/// design leaves about sixteen; the rest is room for a margin chosen otherwise.
const NEAR_THE_TOP: f32 = 40.0;

/// How far the pill's middle may stand from the view's, in points.
const CENTRED_WITHIN: f32 = 1.0;

/// How far a patch may sit inside what it carries, in points: the rect the
/// tree gives a widget and the one painted around it are rounded apart.
const HUGGED_WITHIN: f32 = 0.5;

/// The ribbon's « Terminer », which the pill's « Terminer ↵ » is compared to.
const FINISH_IN_THE_RIBBON: &str = "Terminer (Entrée)";

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
    let row = the_pill_s_row(&app, finish);
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
        driver::a_sketch_is_open(&app),
        "the click on XY opened no sketch, showing {:?}",
        driver::on_screen(&app),
    );
    seen.push(("a sketch open on XY", true, what_the_pill_shows(&app)));

    driver::finish_the_sketch(&mut app);
    seen.push(("the sketch finished", false, what_the_pill_shows(&app)));
    assert!(
        !driver::a_sketch_is_open(&app),
        "the pill's « Terminer ↵ » left the sketch open, showing {:?}",
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

#[test]
fn a_sketch_on_each_origin_plane_reads_that_plane_s_name() {
    let mut wrong = Vec::new();
    for (index, (point, plane)) in ON_EACH_BASE_PLANE.into_iter().enumerate() {
        let mut app = a_part(&format!("a_sketch_on_each_origin_plane_{index}"));
        driver::click(&mut app, "Nouvelle esquisse");
        driver::click_at_world_point(&mut app, point);
        assert_a_sketch_is_open(&app, &format!("the click at {point:?}"));

        let wanted = format!("Esquisse 1 · {plane}");
        let read = what_the_pill_reads(&app);
        if !read.contains(&wanted) {
            wrong.push(format!(
                "a sketch opened at {point:?}: wanted « {wanted} », the pill reads « {read} »"
            ));
        }
    }

    assert!(
        wrong.is_empty(),
        "a sketch on a plane of the origin does not read that plane's name:\n  {}",
        wrong.join("\n  "),
    );
}

#[test]
fn a_sketch_off_the_origin_reads_as_a_face_of_the_part() {
    let mut app = driver::a_block("a_sketch_off_the_origin_reads_as_a_face");
    driver::click_in_the_ribbon(&mut app, "Nouvelle esquisse");
    driver::click_at_world_point(&mut app, ON_THE_BLOCK_S_FRONT_FACE);
    assert_a_sketch_is_open(&app, "the click on the block's front face");

    let read = what_the_pill_reads(&app);

    assert!(
        read.contains("Esquisse 2 · Face de la pièce"),
        "a second sketch laid on the block's face reads « {read} », not « Esquisse 2 · \
         Face de la pièce »",
    );
}

#[test]
fn a_sketch_laid_on_a_kept_section_reads_that_section_s_name() {
    let mut app = driver::a_block("a_sketch_laid_on_a_kept_section_reads_its_name");
    save_a_section(&mut app, "Plan XZ");
    save_a_section(&mut app, "Plan YZ");
    delete_from_the_tree(&mut app, "Coupe 1");
    assert!(
        !driver::in_the_tree(&app, "Coupe 1") && driver::in_the_tree(&app, "Coupe 2"),
        "the first section was not deleted alone, the tree showing {:?}",
        driver::on_screen(&app),
    );
    switch_on(&mut app, "Coupe 2");
    driver::click_in_the_ribbon(&mut app, "Nouvelle esquisse");
    driver::click_at_world_point(&mut app, driver::ON_THE_CUT);
    assert_a_sketch_is_open(&app, "the click on the cut of « Coupe 2 »");

    let read = what_the_pill_reads(&app);

    assert!(
        read.contains("Esquisse 2 · Coupe 2"),
        "a second sketch laid on the section the tree names « Coupe 2 » reads « {read} », \
         not « Esquisse 2 · Coupe 2 »",
    );
}

#[test]
fn a_sketch_whose_section_is_no_longer_kept_reads_its_plane_s_name() {
    let mut app = driver::a_block("a_sketch_whose_section_is_no_longer_kept");
    save_a_section(&mut app, "Plan YZ");
    switch_on(&mut app, "Coupe 1");
    driver::click_in_the_ribbon(&mut app, "Nouvelle esquisse");
    driver::click_at_world_point(&mut app, driver::ON_THE_CUT);
    assert_a_sketch_is_open(&app, "the click on the cut of « Coupe 1 »");
    driver::finish_the_sketch(&mut app);
    delete_from_the_tree(&mut app, "Coupe 1");
    assert!(
        !driver::in_the_tree(&app, "Coupe 1"),
        "the section the sketch stands on was not deleted, the tree showing {:?}",
        driver::on_screen(&app),
    );
    driver::double_click(&mut app, "Esquisse 2");
    assert_a_sketch_is_open(&app, "a double-click on « Esquisse 2 »");

    let read = what_the_pill_reads(&app);

    assert!(
        read.contains("Esquisse 2 · Face de la pièce"),
        "the sketch laid on « Coupe 1 », reopened once that section was deleted, reads \
         « {read} », not « Esquisse 2 · Face de la pièce », its plane's name",
    );
}

#[test]
fn from_two_degrees_of_freedom_left_the_pill_counts_them_in_the_plural() {
    let mut wrong = Vec::new();
    let drawings: [(&str, &str, Gesture); 2] = [
        (
            "from_two_degrees_of_freedom_a_free_rectangle",
            "a rectangle drawn away from the origin",
            a_free_rectangle,
        ),
        (
            "from_two_degrees_of_freedom_a_rectangle_on_the_origin",
            "a rectangle with a corner on the origin",
            a_rectangle_on_the_origin,
        ),
    ];
    for (test, drawing, draw) in drawings {
        let mut app = a_part(test);
        driver::start_a_sketch(&mut app);
        draw(&mut app);
        assert_a_sketch_is_open(&app, drawing);
        let read = what_the_pill_reads(&app);

        let left = the_solver_s_count(&mut app, test);
        assert!(
            left >= 2,
            "the solver leaves {drawing} {left} degrees of freedom, not two or more: the \
             drawing no longer holds the case",
        );
        let wanted = format!("{left} libertés restantes");
        if !reads_in_order(&read, &["Esquisse 1 · Plan YZ", &wanted, "Terminer ↵"]) {
            wrong.push(format!(
                "{drawing}, which the solver leaves {left} degrees of freedom: wanted \
                 « Esquisse 1 · Plan YZ », « {wanted} », « Terminer ↵ », the pill reads \
                 « {read} »"
            ));
        }
    }

    assert!(
        wrong.is_empty(),
        "the pill does not count the solver's degrees of freedom in the plural:\n  {}",
        wrong.join("\n  "),
    );
}

#[test]
fn one_degree_of_freedom_left_reads_in_the_singular() {
    let test = "one_degree_of_freedom_left_reads_in_the_singular";
    let mut app = a_part(test);
    driver::start_a_sketch(&mut app);
    a_rectangle_on_the_origin(&mut app);
    a_dimension(&mut app, ON_ITS_TOP_SIDE, ABOVE_ITS_TOP_SIDE);
    assert_a_sketch_is_open(&app, "the dimension of the rectangle's width");
    let read = what_the_pill_reads(&app);

    let left = the_solver_s_count(&mut app, test);
    assert_eq!(
        left, 1,
        "the solver leaves the rectangle on the origin, its width dimensioned, {left} \
         degrees of freedom, not one: the drawing no longer holds the case",
    );

    assert!(
        reads_in_order(
            &read,
            &["Esquisse 1 · Plan YZ", "1 liberté restante", "Terminer ↵"]
        ),
        "one degree of freedom left: wanted « Esquisse 1 · Plan YZ », « 1 liberté \
         restante », « Terminer ↵ », the pill reads « {read} »",
    );
}

#[test]
fn a_sketch_with_no_freedom_left_reads_fully_set_in_the_success_colour() {
    let test = "a_sketch_with_no_freedom_left_reads_fully_set";
    let mut app = a_part(test);
    driver::start_a_sketch(&mut app);
    a_rectangle_on_the_origin(&mut app);
    a_dimension(&mut app, ON_ITS_TOP_SIDE, ABOVE_ITS_TOP_SIDE);
    a_dimension(&mut app, ON_ITS_RIGHT_SIDE, RIGHT_OF_ITS_RIGHT_SIDE);
    assert_a_sketch_is_open(&app, "the dimension of the rectangle's height");
    let read = what_the_pill_reads(&app);
    let painted_in = colours_painted_for(&app, "entièrement réglée");

    let left = the_solver_s_count(&mut app, test);
    assert_eq!(
        left, 0,
        "the solver leaves the rectangle on the origin, its width and its height \
         dimensioned, {left} degrees of freedom, not none: the drawing no longer holds \
         the case",
    );

    let mut wrong = Vec::new();
    if !reads_in_order(
        &read,
        &["Esquisse 1 · Plan YZ", "entièrement réglée", "Terminer ↵"],
    ) || read.contains("libert")
    {
        wrong.push(format!(
            "wanted « Esquisse 1 · Plan YZ », « entièrement réglée », « Terminer ↵ », and \
             no count, the pill reads « {read} »"
        ));
    }
    let success = the_design_s_success_text();
    if painted_in.is_empty() || painted_in.iter().any(|colour| *colour != success) {
        wrong.push(format!(
            "« entièrement réglée » is painted in {painted_in:?}, not in \
             `state.success_text`, {success:?}"
        ));
    }
    assert!(
        wrong.is_empty(),
        "a sketch the solver leaves no freedom does not say so in green:\n  {}",
        wrong.join("\n  "),
    );
}

#[test]
fn a_sketch_with_nothing_drawn_yet_shows_no_count() {
    let test = "a_sketch_with_nothing_drawn_yet_shows_no_count";
    let mut app = a_part(test);
    driver::start_a_sketch(&mut app);
    let read = what_the_pill_reads(&app);

    let left = the_solver_s_count(&mut app, test);
    assert_eq!(
        left, 0,
        "the solver leaves an empty sketch {left} degrees of freedom: the case the \
         decision answers, an empty sketch counted as fully set, is gone",
    );

    assert!(
        reads_in_order(&read, &["Esquisse 1 · Plan YZ Terminer ↵"]),
        "a sketch with nothing drawn shows no count, neither a number nor « entièrement \
         réglée », between its plane and « Terminer ↵ »: the pill reads « {read} »",
    );
}

#[test]
fn the_pill_s_finish_button_finishes_the_sketch_as_the_ribbon_s_does() {
    let by_the_ribbon = finished_by(
        "finished_by_the_ribbon_s_button_beside_the_pill_s",
        finish_with_the_ribbon,
    );
    let by_the_pill = finished_by("finished_by_the_pill_s_button", finish_with_the_pill);

    assert_eq!(
        what_a_finish_left_wrong(&by_the_pill),
        Vec::<String>::new(),
        "the pill's « Terminer ↵ » did not finish the sketch",
    );
    assert_eq!(
        by_the_pill, by_the_ribbon,
        "the pill's « Terminer ↵ » left the screen otherwise than the ribbon's « Terminer »",
    );
}

#[test]
fn enter_and_the_ribbon_s_finish_button_still_finish_the_sketch() {
    let by_the_ribbon = finished_by("finished_by_the_ribbon_s_button", finish_with_the_ribbon);
    let by_enter = finished_by("finished_by_enter", finish_with_enter);

    assert_eq!(
        (
            what_a_finish_left_wrong(&by_the_ribbon),
            what_a_finish_left_wrong(&by_enter)
        ),
        (Vec::new(), Vec::new()),
        "(the ribbon's « Terminer », ↵) did not finish the sketch",
    );
    assert_eq!(
        by_enter, by_the_ribbon,
        "↵ left the screen otherwise than the ribbon's « Terminer »",
    );
}

#[test]
fn a_click_on_the_pill_s_finish_button_places_no_point_with_the_line_tool_in_hand() {
    let test = "a_click_on_the_pill_s_finish_button_places_no_point";
    let mut app = a_part(test);
    driver::start_a_sketch(&mut app);
    driver::click(&mut app, "Ligne (L)");
    driver::click_at_sketch_point(&mut app, driver::A_POINT_BELOW_AND_RIGHT);

    finish_with_the_pill(&mut app);

    let left = end_state(&app);
    assert_eq!(
        what_a_finish_left_wrong(&left),
        Vec::<String>::new(),
        "the pill's « Terminer ↵ », the line tool in hand, did not finish the sketch",
    );
    driver::open_the_history(&mut app);
    let steps = driver::steps(&app);
    let saved = driver::the_part_put_away(&mut app, test, "Platine");
    let sketch = saved.sketches().first().expect("the sketch drawn");
    assert!(
        steps == ["1. Esquisse — Plan YZ"] && !sketch.holds_a_drawing(),
        "the click on « Terminer ↵ » reached the drawing beneath it: the history reads \
         {steps:?}, and the sketch saved holds the points {:?}",
        sketch.points(),
    );
}

#[test]
fn a_drag_begun_in_the_view_carries_on_over_the_pill_and_lands_where_it_is_let_go() {
    let mut settings = cao_prefs::Settings::default();
    settings.viewport.grid_snap = false;
    let test = "a_drag_begun_in_the_view_carries_on_over_the_pill";
    let mut app = driver::open_held_with(test, settings);
    driver::create_a_part(&mut app, "Platine");
    driver::start_a_sketch(&mut app);
    driver::draw_a_segment(&mut app);
    driver::click(&mut app, "Sélection (S)");
    driver::open_the_history(&mut app);
    let before = driver::steps(&app);
    let finish = the_pill_s_finish(&app).expect("the pill, the sketch being open");
    let on_the_count = the_pill_s_row(&app, finish)
        .into_iter()
        .find(|(_, said)| said.contains("libert"))
        .map(|(at, _)| at.center())
        .unwrap_or_else(|| {
            panic!(
                "the pill counts nothing to drop on: « {} »",
                what_the_pill_reads(&app)
            )
        });
    let pressed = driver::sketch_point_on_screen(&app, driver::A_POINT_ABOVE_THE_ORIGIN);

    driver::drag_and_drop(&mut app, pressed, on_the_count);

    let after = driver::steps(&app);
    let origin = driver::sketch_point_on_screen(&app, DVec2::ZERO);
    let per_millimetre = (
        driver::sketch_point_on_screen(&app, DVec2::X) - origin,
        driver::sketch_point_on_screen(&app, DVec2::Y) - origin,
    );
    let saved = driver::the_part_put_away(&mut app, test, "Platine");
    let sketch = saved.sketches().first().expect("the sketch drawn");
    let landed = sketch
        .points()
        .iter()
        .map(|point| origin + per_millimetre.0 * point.x as f32 + per_millimetre.1 * point.y as f32)
        .min_by(|one, other| {
            one.distance(on_the_count)
                .total_cmp(&other.distance(on_the_count))
        })
        .expect("the segment's ends");

    assert!(
        after.len() == before.len() + 1
            && after
                .last()
                .is_some_and(|last| last.ends_with("Déplacement")),
        "the drag let go on the pill was not one step of its own: {before:?} became {after:?}",
    );
    assert!(
        landed.distance(on_the_count) < LET_GO_WITHIN,
        "the point dragged from {pressed:?} and let go on the pill at {on_the_count:?} \
         landed at {landed:?}",
    );
}

#[test]
fn a_sketch_whose_drawing_is_all_erased_shows_no_count() {
    let mut app = a_part("a_sketch_whose_drawing_is_all_erased_shows_no_count");
    driver::start_a_sketch(&mut app);
    driver::draw_a_segment(&mut app);
    driver::click(&mut app, "Sélection (S)");
    driver::press_with_command(&mut app, egui::Key::A);
    let away_from_the_drawing = driver::sketch_point_on_screen(&app, AWAY_FROM_THE_SEGMENT);
    app.hover_at(away_from_the_drawing);
    app.run();
    driver::press(&mut app, egui::Key::Delete);
    let read = what_the_pill_reads(&app);

    driver::open_the_history(&mut app);
    let steps = driver::steps(&app);
    assert!(
        steps
            .last()
            .is_some_and(|last| last.ends_with("éléments supprimés")),
        "Cmd+A then Delete did not erase the segment and its ends: {steps:?}",
    );
    assert!(
        reads_in_order(&read, &["Esquisse 1 · Plan YZ Terminer ↵"]),
        "a sketch whose drawing was all erased shows no count, neither a number nor \
         « entièrement réglée », between its plane and « Terminer ↵ »: the pill reads \
         « {read} »",
    );
}

#[test]
fn the_pill_s_texts_stand_on_the_middle_line_of_its_button() {
    let mut app = a_part("the_pill_s_texts_stand_on_the_middle_line_of_its_button");
    driver::start_a_sketch(&mut app);
    driver::draw_a_segment(&mut app);
    let finish = the_pill_s_finish(&app).expect("the pill, the sketch being open");
    let row = the_pill_s_row(&app, finish);
    assert!(
        row.iter().any(|(_, said)| said.contains("Esquisse 1"))
            && row.iter().any(|(_, said)| said.contains("libert")),
        "the sketch's name and its count do not both stand on the row of « Terminer ↵ » \
         at {finish:?}: {row:?}",
    );

    let off: Vec<String> = row
        .iter()
        .filter(|(at, _)| (at.center().y - finish.center().y).abs() > ON_ONE_LINE_WITHIN)
        .map(|(at, said)| {
            format!(
                "« {said} » has its middle at {}, {} points off",
                at.center().y,
                at.center().y - finish.center().y,
            )
        })
        .collect();
    assert!(
        off.is_empty(),
        "the pill's texts do not stand on the middle line of « Terminer ↵ », at {}:\n  {}",
        finish.center().y,
        off.join("\n  "),
    );
}

#[test]
fn the_pill_s_button_names_the_keys_that_finish_the_sketch_once_rebound() {
    let mut wrong = Vec::new();
    for (index, chord) in [Chord::new(Key::Enter).cmd(), Chord::new(Key::F9)]
        .into_iter()
        .enumerate()
    {
        let mut settings = driver::the_moved_commands_put_back();
        settings.shortcuts.bind(Command::FinishSketch, chord);
        let mut app =
            driver::open_held_with(&format!("the_pill_s_button_once_rebound_{index}"), settings);
        driver::create_a_part(&mut app, "Platine");
        driver::start_a_sketch(&mut app);

        let (in_the_ribbon, in_the_view) = the_finish_buttons(&app);
        let written = in_the_ribbon
            .as_deref()
            .and_then(|read| read.strip_prefix("Terminer ("))
            .and_then(|rest| rest.strip_suffix(')'))
            .unwrap_or_else(|| {
                panic!(
                    "finishing bound to {chord:?}, the ribbon's button reads {in_the_ribbon:?}, \
                     not « Terminer » and the chord between brackets"
                )
            });
        let wanted = format!("Terminer {written}");
        if in_the_view.as_deref() != Some(wanted.as_str()) {
            wrong.push(format!(
                "finishing bound to {chord:?}: wanted « {wanted} », the pill's button reads \
                 {in_the_view:?}"
            ));
        }
    }

    assert!(
        wrong.is_empty(),
        "the pill's button does not name the keys that finish the sketch:\n  {}",
        wrong.join("\n  "),
    );
}

#[test]
fn the_pill_s_button_names_no_key_once_finishing_is_unbound() {
    let mut app = driver::open_held_with(
        "the_pill_s_button_names_no_key_once_finishing_is_unbound",
        driver::the_moved_commands_put_back(),
    );
    driver::create_a_part(&mut app, "Platine");
    driver::start_a_sketch(&mut app);
    unbind_in_the_preferences(&mut app, "Terminer");

    let (in_the_ribbon, in_the_view) = the_finish_buttons(&app);
    assert_eq!(
        in_the_ribbon.as_deref(),
        Some("Terminer"),
        "« Retirer » on the row of « Terminer » in the preferences left the ribbon's \
         button naming a key",
    );
    assert_eq!(
        in_the_view.as_deref(),
        Some("Terminer"),
        "finishing bound to nothing, the pill's button names a key that does not finish",
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
        driver::a_sketch_is_open(app),
        "the click on XY opened no sketch, showing {:?}",
        driver::on_screen(app),
    );
}

/// How far the middle of a text of the pill may stand from its button's, in
/// points: on one line, as the design draws them.
const ON_ONE_LINE_WITHIN: f32 = 1.0;

/// A point of the sketch on YZ that `driver::draw_a_segment` leaves empty,
/// below and right of the segment: where the pointer rests for Delete, which
/// is read only over the drawing.
const AWAY_FROM_THE_SEGMENT: DVec2 = DVec2::new(212.26, -65.95);

/// What a hand does to the sketch open: draws in it, or finishes it.
type Gesture = fn(&mut driver::App);

/// How far from where it was let go a point dragged may land, in points.
const LET_GO_WITHIN: f32 = 1.0;

/// A rectangle from one node of the grid to another, away from the origin and
/// from the plane's axes.
fn a_free_rectangle(app: &mut driver::App) {
    driver::click(app, "Rectangle (R)");
    driver::click_at_sketch_point(app, driver::A_POINT_ABOVE_THE_ORIGIN);
    driver::click_at_sketch_point(app, driver::A_POINT_BELOW_AND_RIGHT);
    driver::press(app, egui::Key::Escape);
}

/// A rectangle whose first corner is pulled onto the origin, which holds it
/// there.
fn a_rectangle_on_the_origin(app: &mut driver::App) {
    driver::click(app, "Rectangle (R)");
    driver::click_at_sketch_point(app, DVec2::ZERO);
    driver::click_at_sketch_point(app, ITS_FAR_CORNER);
    driver::press(app, egui::Key::Escape);
}

/// A dimension taken off the trait at `on`, left at `left_at`, the length it
/// reads kept.
fn a_dimension(app: &mut driver::App, on: DVec2, left_at: DVec2) {
    driver::click(app, "Cote");
    driver::click_at_sketch_point(app, on);
    driver::click_at_sketch_point(app, left_at);
    driver::press(app, egui::Key::Enter);
    driver::press(app, egui::Key::Escape);
}

/// How many degrees of freedom the solver leaves the part's first sketch,
/// `Sketch::freedom` at the part's scale, read off the part as the application
/// puts it away. The application is left on the home screen.
fn the_solver_s_count(app: &mut driver::App, test: &str) -> usize {
    let part = driver::the_part_put_away(app, test, "Platine");
    let sketch = part.sketches().first().expect("the sketch drawn");
    sketch.freedom(part.scale()).degrees_of_freedom
}

/// What the screen is left showing once a sketch is finished: whether a
/// sketch is still drawn, what of the pill is left, every button by its name,
/// whether it can be pressed and whether it is on, and every field.
#[derive(Debug, PartialEq)]
struct EndState {
    sketch_open: bool,
    pill: Vec<String>,
    buttons: Vec<(String, bool, Option<egui::accesskit::Toggled>)>,
    fields: Vec<String>,
}

fn end_state(app: &driver::App) -> EndState {
    let mut buttons: Vec<(String, bool, Option<egui::accesskit::Toggled>)> = app
        .query_all_by_role(Role::Button)
        .map(|node| {
            let read = node.accesskit_node();
            (
                read.label().unwrap_or_default(),
                !read.is_disabled(),
                read.toggled(),
            )
        })
        .collect();
    buttons.sort_by(|one, other| one.0.cmp(&other.0));
    EndState {
        sketch_open: app
            .state()
            .where_the_view_shows_the_sketch_point(DVec2::ZERO)
            .is_some(),
        pill: what_the_pill_shows(app),
        buttons,
        fields: driver::fields(app),
    }
}

/// What keeps a screen from being one a finished sketch leaves: the sketch
/// closed, the pill gone, the sketch kept in the tree, and the ribbon turned to
/// its Extrusion tab, where `Command::FinishSketch` offers to raise it.
fn what_a_finish_left_wrong(left: &EndState) -> Vec<String> {
    let mut wrong = Vec::new();
    if left.sketch_open {
        wrong.push("the sketch is still open".to_owned());
    }
    if !left.pill.is_empty() {
        wrong.push(format!("the pill still shows {:?}", left.pill));
    }
    let a_button =
        |reading: &dyn Fn(&str) -> bool| left.buttons.iter().any(|(read, ..)| reading(read));
    if !a_button(&|read| read == "Esquisse 1") {
        wrong.push("the tree keeps no « Esquisse 1 »".to_owned());
    }
    if !a_button(&|read| read.starts_with("Ajout de matière")) {
        wrong.push(format!(
            "the ribbon is not turned to its Extrusion tab: {:?}",
            left.buttons
        ));
    }
    wrong
}

/// A sketch with a trait drawn in it, finished that way.
fn finished_by(test: &str, finish: Gesture) -> EndState {
    let mut app = driver::open_held_with(test, driver::the_moved_commands_put_back());
    driver::create_a_part(&mut app, "Platine");
    driver::start_a_sketch(&mut app);
    driver::draw_a_segment(&mut app);
    finish(&mut app);
    end_state(&app)
}

fn finish_with_the_ribbon(app: &mut driver::App) {
    driver::click_in_the_ribbon(app, FINISH_IN_THE_RIBBON);
}

fn finish_with_enter(app: &mut driver::App) {
    driver::press(app, egui::Key::Enter);
}

/// A pointer's click at the middle of the pill's « Terminer ↵ », as a hand
/// would give it.
fn finish_with_the_pill(app: &mut driver::App) {
    let finish = the_pill_s_finish(app).unwrap_or_else(|| {
        panic!(
            "no « Terminer ↵ » on screen to click: {:?}",
            driver::on_screen(app)
        )
    });
    driver::click_at(app, finish.center());
}

/// Whether `read` holds each of `pieces`, word for word, in that order: a
/// number is not found inside a larger one, nor a word inside a longer one.
fn reads_in_order(read: &str, pieces: &[&str]) -> bool {
    let words: Vec<&str> = read.split_whitespace().collect();
    let mut from = 0;
    pieces.iter().all(|piece| {
        let wanted: Vec<&str> = piece.split_whitespace().collect();
        if wanted.len() > words.len() {
            return false;
        }
        match (from..=words.len() - wanted.len())
            .find(|at| words[*at..*at + wanted.len()] == wanted[..])
        {
            Some(at) => {
                from = at + wanted.len();
                true
            }
            None => false,
        }
    })
}

/// The colours every text painted on the last frame that holds `text` is
/// written in, section by section.
fn colours_painted_for(app: &driver::App, text: &str) -> Vec<egui::Color32> {
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
            egui::Shape::Text(painted) if painted.galley.text().contains(text) => {
                for section in &painted.galley.job.sections {
                    found.push(match painted.override_text_color {
                        Some(colour) => colour,
                        None if section.format.color == egui::Color32::PLACEHOLDER => {
                            painted.fallback_color
                        }
                        None => section.format.color,
                    });
                }
            }
            _ => {}
        }
    }
    found
}

/// What the design file gives text that says all is well: `state.success_text`
/// of `docs/redesign/tokens.json`. Read there rather than through the token it
/// becomes, which `theme.rs` holds to the same value.
fn the_design_s_success_text() -> egui::Color32 {
    let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../docs/redesign/tokens.json");
    let source = fs::read_to_string(&path).unwrap_or_else(|_| panic!("{path:?} is readable"));
    let design: serde_json::Value =
        serde_json::from_str(&source).unwrap_or_else(|_| panic!("{path:?} is JSON"));
    let written = design["color"]["state"]["success_text"]
        .as_str()
        .unwrap_or_else(|| panic!("{path:?} gives no color.state.success_text"));
    let channel = |at: usize| {
        written
            .get(1 + at..3 + at)
            .and_then(|hex| u8::from_str_radix(hex, 16).ok())
            .unwrap_or_else(|| panic!("color.state.success_text is {written}, not #RRGGBB"))
    };
    egui::Color32::from_rgb(channel(0), channel(2), channel(4))
}

fn assert_a_sketch_is_open(app: &driver::App, after: &str) {
    assert!(
        driver::a_sketch_is_open(app),
        "{after} opened no sketch, showing {:?}",
        driver::on_screen(app),
    );
}

/// Opens the section tool, lays it along that plane, keeps it with
/// « Enregistrer la coupe » and closes the tool.
fn save_a_section(app: &mut driver::App, plane: &str) {
    driver::click_in_the_bar(app, "Vue en coupe");
    driver::click(app, plane);
    driver::click_the_button(app, "Enregistrer la coupe");
    if driver::a_button_reads(app, "Inverser") {
        driver::click_the_button(app, "Fermer");
    }
}

/// Deletes a section through the menu of its row in the tree, answering
/// « Supprimer » too when the application asks first, as it does for a section
/// a sketch stands on.
fn delete_from_the_tree(app: &mut driver::App, section: &str) {
    driver::right_click(app, section);
    driver::click_the_button(app, "Supprimer");
    if app.query_all_by_label("Annuler").next().is_some() {
        driver::click_the_button(app, "Supprimer");
    }
}

fn switch_on(app: &mut driver::App, name: &str) {
    let row = driver::the_row(app, name).unwrap_or_else(|| {
        panic!(
            "the tree holds no « {name} », showing {:?}",
            driver::on_screen(app)
        )
    });
    if row.accesskit_node().toggled() != Some(egui::accesskit::Toggled::True) {
        row.click();
        app.run();
        driver::hold_the_canvas(app);
    }
}

/// « Retirer » clicked on the row of that command in Préférences › Raccourcis,
/// then the window closed. A profile saved with a command bound to nothing is
/// handed its default chord again as it is read back, so the binding is taken
/// away where the user takes it away.
fn unbind_in_the_preferences(app: &mut driver::App, command: &str) {
    driver::press_with_command(app, egui::Key::Comma);
    driver::click_exactly(app, "Raccourcis");
    let its_name = |node: &egui_kittest::kittest::AccessKitNode<'_>| {
        !matches!(node.role(), Role::Button | Role::TextRun)
            && (node.label().as_deref() == Some(command)
                || node.value().as_deref() == Some(command))
    };
    app.query_all_by(its_name)
        .next()
        .unwrap_or_else(|| {
            panic!(
                "the shortcuts list no « {command} »: {:?}",
                driver::on_screen(app)
            )
        })
        .scroll_to_me();
    app.run();
    let row = app
        .query_all_by(its_name)
        .next()
        .expect("the row just scrolled to")
        .rect();
    app.query_all_by_role(Role::Button)
        .filter(|node| node.accesskit_node().label().as_deref() == Some("Retirer"))
        .find(|node| node.rect().y_range().contains(row.center().y))
        .unwrap_or_else(|| panic!("no « Retirer » stands on the row of « {command} » at {row:?}"))
        .click_accesskit();
    app.run();
    driver::click_exactly(app, "Close window");
}

/// The names of the two buttons that finish the sketch, the ribbon's and the
/// pill's, told apart by where they stand: the pill's inside the view.
fn the_finish_buttons(app: &driver::App) -> (Option<String>, Option<String>) {
    let view = driver::canvas(app);
    let (mut in_the_ribbon, mut in_the_view) = (None, None);
    for node in app.query_all_by_role(Role::Button) {
        let Some(read) = node
            .accesskit_node()
            .label()
            .filter(|read| read.starts_with("Terminer"))
        else {
            continue;
        };
        if view.contains_rect(node.rect()) {
            in_the_view = Some(read);
        } else {
            in_the_ribbon = Some(read);
        }
    }
    (in_the_ribbon, in_the_view)
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

/// What the view holds on the row of the pill's « Terminer ↵ », left to right.
fn the_pill_s_row(app: &driver::App, finish: egui::Rect) -> Vec<(egui::Rect, String)> {
    readings_in_the_view(app)
        .into_iter()
        .filter(|(at, _)| finish.y_range().contains(at.center().y))
        .collect()
}

/// What the pill reads, as one line; nothing when no pill is on screen.
fn what_the_pill_reads(app: &driver::App) -> String {
    the_pill_s_finish(app)
        .map(|finish| one_line(&the_pill_s_row(app, finish)))
        .unwrap_or_default()
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
