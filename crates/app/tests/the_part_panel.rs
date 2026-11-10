//! The Part panel, driven through the whole application: how its rows stand,
//! what an open sketch shows of itself there, and what an unfolded sketch
//! lists.
//!
//! Closes #733 — the whole transcription is in
//! `crates/app/src/screens/part_tree/state/tests.rs`; these are the bullets
//! only the panel drawn shows.
//! - Tree rows (C10): 28 px high, indented 26 px — a sketch's row and the rows
//!   under it each stand 28 points high, and a row under the sketch starts 26
//!   points right of the sketch's row —
//!   `a_tree_row_stands_28_points_high_and_its_children_26_points_in`
//! - The open sketch is selected and unfolded — reopened by a double-click,
//!   its row reads as selected and what is drawn on it is listed without a
//!   click to unfold it — `the_open_sketch_is_selected_and_unfolded_in_the_tree`
//! - A sketch's children: its traits straight under it, with no « Traits »
//!   group, then « Aires », « Cotes », « Contraintes », each a row whose count
//!   is its value, on the right of its line —
//!   `an_unfolded_sketch_lists_its_traits_then_its_areas_dimensions_and_rules_counted`
//! - found in review: a row a click asks nothing of — a dimension of a closed
//!   sketch, a variable — is announced as no button and is not filled under
//!   the hand — `a_row_that_asks_nothing_is_no_button_and_is_not_filled_under_the_hand`
//! - found in review: a variable whose value is long keeps room for its name,
//!   the value giving way before the panel's edge —
//!   `a_variable_whose_value_is_long_keeps_its_name_and_the_value_gives_way`
//! - found in review: a sketch's row reads its plane and its badge on its own
//!   line — `a_sketch_row_shows_its_plane_and_its_badge_on_its_line`
//! - Hovering a row lights its element in the view, through the whole
//!   application: the hand resting on a trait of a closed sketch lights it,
//!   leaving puts it out, and a click's light stays as it was —
//!   `a_hand_resting_on_a_trait_of_a_closed_sketch_lights_it_and_leaving_puts_it_out`
//! - found when the branch was tried: a sketch's eye, shut or lit, the sketch
//!   closed or open, never runs over its row —
//!   `a_sketch_row_s_eye_never_overlaps_its_row`
//!
//! Closes #734 — the whole transcription is in
//! `crates/app/src/screens/part_tree/state/tests.rs`; these are the bullets
//! only the panel drawn and clicked shows.
//! - a volume row's visibility button cycles opaque → « transparente » →
//!   invisible, and the state is written on the row's line —
//!   `clicking_a_volume_s_visibility_button_writes_its_next_state_on_its_row`
//! - « Tout rendre opaque » sets every volume back to opaque, and is greyed
//!   while they all are — `clicking_make_everything_opaque_brings_every_row_back_to_opaque`;
//!   it replaced the « TOUT LE CORPS » selector of criterion 4 as first
//!   written, struck on 2026-11-10 by Tom, whose test was deleted
//! - the « Corps » row is gone —
//!   `the_part_panel_holds_no_body_row_any_more`
//!
//! A row's left edge and height are read off the widget named as the row is:
//! the one the hand clicks. The driver reads them in points, which is what
//! the design file's pixels are — `egui_kittest` divides the window's
//! physical pixels by its scale before handing a rect back.

mod driver;

use cao_app::app::on_screen::LitRow;
use egui::accesskit::{Role, Toggled};
use egui_kittest::kittest::{NodeT, Queryable};
use glam::DVec2;

const TOLERANCE: f32 = 0.5;

/// On the face of the block at x = 10, the one `keeping_a_section.rs` clicks
/// to lay a section there.
const ON_THE_BLOCK_S_FACE: glam::DVec3 = glam::DVec3::new(10.0, 41.51, -19.82);

/// The rectangle of the row of the tree that reads exactly this.
fn row(app: &driver::App, name: &str) -> egui::Rect {
    driver::the_row(app, name)
        .unwrap_or_else(|| {
            panic!(
                "no row of the tree reads « {name} », showing {:?}",
                driver::on_screen(app)
            )
        })
        .rect()
}

/// The text written on the right of a row's line: its value.
fn value_of(app: &driver::App, name: &str) -> Option<String> {
    let line = row(app, name);
    app.query_all_by_role(Role::Label)
        .filter(|node| node.accesskit_node().bounding_box().is_some())
        .filter(|node| {
            let at = node.rect();
            at.center().y > line.top() && at.center().y < line.bottom() && at.left() > line.right()
        })
        .find_map(|node| node.accesskit_node().value().map(|said| said.to_string()))
}

/// Every row of the tree under the sketch row, top to bottom, down to the next
/// heading, with its left edge.
fn rows_under_the_sketch(app: &driver::App, sketch: &str) -> Vec<(String, f32)> {
    let top = row(app, sketch).bottom();
    let next_heading = app
        .query_all_by_role(Role::Label)
        .filter(|node| node.accesskit_node().value().as_deref() == Some("VOLUMES"))
        .map(|node| node.rect().top())
        .next()
        .unwrap_or(f32::INFINITY);
    let mut under: Vec<(String, egui::Rect)> = app
        .query_all_by_role(Role::Button)
        .filter(|node| node.accesskit_node().bounding_box().is_some())
        .filter_map(|node| Some((node.accesskit_node().label()?, node.rect())))
        .filter(|(_, at)| at.top() >= top && at.top() < next_heading && at.left() < 300.0)
        .collect();
    under.sort_by(|(_, a), (_, b)| a.top().total_cmp(&b.top()));
    under
        .into_iter()
        .map(|(name, at)| (name, at.left()))
        .collect()
}

fn is_selected(app: &driver::App, name: &str) -> bool {
    driver::the_row(app, name)
        .is_some_and(|node| node.accesskit_node().toggled() == Some(Toggled::True))
}

/// `driver::a_block`, its sketch reopened to set a dimension on the top side
/// of the rectangle at what it measures, then finished: a sketch holding four
/// traits, one area, one dimension and the rules the rectangle laid.
fn a_block_with_a_dimension(test: &str) -> driver::App {
    let mut app = driver::a_block(test);
    driver::double_click(&mut app, "Esquisse 1");
    let [corner, opposite] = driver::THE_BLOCK_S_CORNERS;
    let on_the_top_side = DVec2::new((corner.x + opposite.x) / 2.0, corner.y);
    driver::pick_the_dimension_tool(&mut app);
    driver::click_at_sketch_point(&mut app, on_the_top_side);
    driver::click_at_sketch_point(&mut app, on_the_top_side + DVec2::new(0.0, 15.0));
    driver::press(&mut app, egui::Key::Enter);
    driver::press(&mut app, egui::Key::Escape);
    driver::finish_the_sketch(&mut app);
    app
}

#[test]
fn a_tree_row_stands_28_points_high_and_its_children_26_points_in() {
    let mut app = driver::a_block("a_tree_row_stands_28_points_high");
    driver::click(&mut app, "Esquisse 1");

    let sketch = row(&app, "Esquisse 1");
    for name in ["Esquisse 1", "Trait 1", "Aires"] {
        let height = row(&app, name).height();
        assert!(
            (height - 28.0).abs() <= TOLERANCE,
            "the row « {name} » stands {height} points high, not 28",
        );
    }
    for name in ["Trait 1", "Aires"] {
        let indent = row(&app, name).left() - sketch.left();
        assert!(
            (indent - 26.0).abs() <= TOLERANCE,
            "the row « {name} » starts {indent} points right of its sketch's row, not 26",
        );
    }
}

#[test]
fn the_open_sketch_is_selected_and_unfolded_in_the_tree() {
    let mut app = driver::a_block("the_open_sketch_is_selected_and_unfolded");
    assert!(
        !is_selected(&app, "Esquisse 1"),
        "a sketch nobody is drawing on reads as selected in the tree",
    );
    assert!(
        !driver::in_the_tree(&app, "Trait 1"),
        "a sketch nobody is drawing on is unfolded before anyone asked",
    );

    driver::double_click(&mut app, "Esquisse 1");

    assert!(
        driver::a_sketch_is_open(&app),
        "a double-click on « Esquisse 1 » did not reopen it"
    );
    assert!(
        is_selected(&app, "Esquisse 1"),
        "the sketch being drawn on does not read as selected in the tree",
    );
    assert!(
        driver::in_the_tree(&app, "Trait 1"),
        "the sketch being drawn on is not unfolded in the tree, showing {:?}",
        driver::on_screen(&app),
    );
}

#[test]
fn an_unfolded_sketch_lists_its_traits_then_its_areas_dimensions_and_rules_counted() {
    let test = "an_unfolded_sketch_lists_its_children";
    let mut app = a_block_with_a_dimension(test);
    driver::click(&mut app, "Esquisse 1");

    let under = rows_under_the_sketch(&app, "Esquisse 1");
    let names: Vec<&str> = under.iter().map(|(name, _)| name.as_str()).collect();
    let lefts: Vec<f32> = under.iter().map(|(_, left)| *left).collect();
    let counts: Vec<Option<String>> = ["Aires", "Cotes", "Contraintes"]
        .into_iter()
        .map(|name| value_of(&app, name))
        .collect();
    let document = driver::the_part_put_away(&mut app, test, "Bloc");
    let drawing = &document.sketches()[0];
    assert!(
        !drawing.dimensions().is_empty() && !drawing.constraints().is_empty(),
        "the fixture holds no dimension or no rule, and counts nothing: {} dimensions, {} rules",
        drawing.dimensions().len(),
        drawing.constraints().len(),
    );

    assert_eq!(
        names,
        [
            "Trait 1",
            "Trait 2",
            "Trait 3",
            "Trait 4",
            "Aires",
            "Cotes",
            "Contraintes"
        ],
        "the unfolded sketch does not list its traits straight under it, then its areas, \
         dimensions and rules",
    );
    assert!(
        lefts
            .iter()
            .all(|left| (left - lefts[0]).abs() <= TOLERANCE),
        "the rows under the sketch do not stand at one depth: {under:?}",
    );
    assert_eq!(
        counts,
        [
            Some(drawing.regions().len().to_string()),
            Some(drawing.dimensions().len().to_string()),
            Some(drawing.constraints().len().to_string()),
        ],
        "« Aires », « Cotes », « Contraintes » do not read their count on the right",
    );
}

/// Where a node of the panel reading exactly this stands, whatever it is
/// announced as: a row that asks nothing may be a plain label.
fn reading(app: &driver::App, text: &str) -> Option<egui::Rect> {
    app.get_all_by(|_| true)
        .filter(|node| node.accesskit_node().role() != Role::TextRun)
        .filter(|node| node.accesskit_node().bounding_box().is_some())
        .find(|node| {
            let read = node.accesskit_node();
            read.label().as_deref() == Some(text) || read.value().as_deref() == Some(text)
        })
        .map(|node| node.rect())
}

/// Every rectangle the last frame filled in this colour.
fn filled_in(app: &driver::App, colour: egui::Color32) -> Vec<egui::Rect> {
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
            egui::Shape::Rect(painted) if painted.fill == colour => found.push(painted.rect),
            _ => {}
        }
    }
    found
}

/// Makes a variable in its panel, as a hand types it there.
fn add_a_variable(app: &mut driver::App, name: &str, formula: &str) {
    driver::type_into(app, "nom", name);
    driver::type_into(app, "formule", formula);
    driver::click_exactly(app, "Ajouter");
}

/// `a_block_with_a_dimension`, holding `largeur` at 60, `marge` at 10 and
/// `total` written « largeur * 2 + marge », made in their panel, then closed.
fn a_block_with_variables(test: &str) -> driver::App {
    let mut app = a_block_with_a_dimension(test);
    driver::click_the_button(&mut app, "Variables");
    add_a_variable(&mut app, "largeur", "60");
    add_a_variable(&mut app, "marge", "10");
    add_a_variable(&mut app, "total", "largeur * 2 + marge");
    driver::click_the_button(&mut app, "Variables");
    app
}

#[test]
fn a_row_that_asks_nothing_is_no_button_and_is_not_filled_under_the_hand() {
    let mut app = a_block_with_variables("a_row_that_asks_nothing");
    driver::click(&mut app, "Esquisse 1");
    driver::click_the_button(&mut app, "Cotes");

    for name in ["Cote 1", "largeur"] {
        let at = reading(&app, name).unwrap_or_else(|| {
            panic!(
                "« {name} » is not on screen, showing {:?}",
                driver::on_screen(&app)
            )
        });
        assert!(
            driver::the_row(&app, name).is_none(),
            "« {name} », a row a click asks nothing of, is announced as a button",
        );
        app.hover_at(at.center());
        app.run();
        let under_the_hand: Vec<egui::Rect> =
            filled_in(&app, cao_app::theme::tokens::BACKGROUND_HOVER)
                .into_iter()
                .filter(|filled| filled.contains(at.center()))
                .collect();
        assert!(
            under_the_hand.is_empty(),
            "« {name} », a row a click asks nothing of, is filled under the hand: {under_the_hand:?}",
        );
    }
}

#[test]
fn a_variable_whose_value_is_long_keeps_its_name_and_the_value_gives_way() {
    let app = a_block_with_variables("a_variable_whose_value_is_long");
    let name = reading(&app, "total").unwrap_or_else(|| {
        panic!(
            "the variable « total » is not on screen, showing {:?}",
            driver::on_screen(&app)
        )
    });
    let value = reading(&app, "largeur * 2 + marge = 130").unwrap_or_else(|| {
        panic!(
            "the value of « total » is not on screen, showing {:?}",
            driver::on_screen(&app)
        )
    });
    let written = app
        .ctx
        .debug_painter()
        .layout_no_wrap(
            "total".to_string(),
            egui::FontId::proportional(cao_app::theme::tokens::TYPOGRAPHY_SIZES_BUTTON),
            egui::Color32::WHITE,
        )
        .size()
        .x;

    assert!(
        name.width() >= written,
        "the name « total » is left {} points, where it needs {written}",
        name.width(),
    );
    let short_value = reading(&app, "60").unwrap_or_else(|| {
        panic!(
            "the value of « largeur » is not on screen, showing {:?}",
            driver::on_screen(&app)
        )
    });
    assert!(
        value.left() >= name.left() + written && value.right() <= short_value.right() + TOLERANCE,
        "the long value does not give way between the name and the edge every value of \
         the panel ends on, where « largeur »'s ends: name at {name:?}, value at {value:?}, \
         « 60 » at {short_value:?}",
    );
}

#[test]
fn a_sketch_row_shows_its_plane_and_its_badge_on_its_line() {
    let mut app = driver::a_block("a_sketch_row_shows_its_plane_and_its_badge");
    driver::click(&mut app, "Nouvelle esquisse");
    driver::click_at_world_point(&mut app, ON_THE_BLOCK_S_FACE);
    driver::finish_the_sketch(&mut app);
    driver::double_click(&mut app, "Esquisse 1");
    driver::click(&mut app, "Sélection (S)");
    driver::press_with_command(&mut app, egui::Key::A);
    app.hover_at(driver::sketch_point_on_screen(
        &app,
        DVec2::new(173.9, -56.77),
    ));
    app.run();
    driver::press(&mut app, egui::Key::Delete);
    driver::finish_the_sketch(&mut app);

    let on_its_line = |name: &str| -> Vec<String> {
        let line = row(&app, name);
        app.query_all_by_role(Role::Label)
            .filter(|node| node.accesskit_node().bounding_box().is_some())
            .filter(|node| {
                let at = node.rect();
                at.center().y > line.top() && at.center().y < line.bottom()
            })
            .filter_map(|node| node.accesskit_node().value().map(|said| said.to_string()))
            .collect()
    };
    let first = on_its_line("Esquisse 1");
    let second = on_its_line("Esquisse 2");

    assert!(
        first.contains(&"YZ".to_string()),
        "« Esquisse 1 », laid on YZ, does not read it on its line: {first:?}",
    );
    assert!(
        second.contains(&"face".to_string()) && second.contains(&"face perdue".to_string()),
        "« Esquisse 2 », laid on a face the block lost, does not read « face » and \
         « face perdue » on its line: {second:?}",
    );
}

#[test]
fn a_hand_resting_on_a_trait_of_a_closed_sketch_lights_it_and_leaving_puts_it_out() {
    let mut app = driver::a_block("a_hand_resting_on_a_trait_lights_it");
    driver::click(&mut app, "Esquisse 1");
    driver::click_the_button(&mut app, "Trait 2");
    let the_trait = |name: &str| LitRow {
        sketch: "Esquisse 1".to_string(),
        row: name.to_string(),
    };
    assert_eq!(
        app.state().the_row_lit_by_a_click(),
        Some(the_trait("Trait 2")),
        "a click on « Trait 2 » of the closed sketch did not light it",
    );

    app.hover_at(row(&app, "Trait 1").center());
    app.run();
    let while_resting = (
        app.state().the_row_lit_by_the_hand(),
        app.state().the_row_lit_by_a_click(),
    );
    let away = reading(&app, "ESQUISSES").unwrap_or_else(|| {
        panic!(
            "no « ESQUISSES » heading on screen, showing {:?}",
            driver::on_screen(&app)
        )
    });
    app.hover_at(away.center());
    app.run();
    let once_left = (
        app.state().the_row_lit_by_the_hand(),
        app.state().the_row_lit_by_a_click(),
    );

    assert_eq!(
        while_resting,
        (Some(the_trait("Trait 1")), Some(the_trait("Trait 2"))),
        "the hand resting on « Trait 1 » does not light it, or puts out what the click lit",
    );
    assert_eq!(
        once_left,
        (None, Some(the_trait("Trait 2"))),
        "the hand gone from « Trait 1 » leaves it lit, or takes the click's light with it",
    );
}

/// Where the eye of the only sketch stands, lit or shut.
fn the_eye(app: &driver::App) -> egui::Rect {
    app.query_all_by_role(Role::Button)
        .find(|node| {
            node.accesskit_node().label().is_some_and(|read| {
                read == "Afficher l'esquisse" || read == "Ne plus afficher l'esquisse"
            })
        })
        .unwrap_or_else(|| {
            panic!(
                "no sketch's eye on screen, showing {:?}",
                driver::on_screen(app)
            )
        })
        .rect()
}

#[test]
fn a_sketch_row_s_eye_never_overlaps_its_row() {
    let mut app = driver::a_block("a_sketch_row_s_eye_never_overlaps_its_row");
    let mut seen = Vec::new();
    seen.push((
        "closed, its eye shut",
        the_eye(&app),
        row(&app, "Esquisse 1"),
    ));
    driver::click_the_button(&mut app, "Afficher l'esquisse");
    seen.push((
        "closed, its eye lit",
        the_eye(&app),
        row(&app, "Esquisse 1"),
    ));
    driver::double_click(&mut app, "Esquisse 1");
    assert!(
        driver::a_sketch_is_open(&app),
        "a double-click on « Esquisse 1 » did not reopen it"
    );
    seen.push(("open, its eye lit", the_eye(&app), row(&app, "Esquisse 1")));

    let overlapping: Vec<String> = seen
        .into_iter()
        .filter(|(_, eye, row)| eye.intersect(*row).is_positive())
        .map(|(state, eye, row)| format!("{state}: eye at {eye:?}, row at {row:?}"))
        .collect();
    assert!(
        overlapping.is_empty(),
        "a sketch's eye runs over its row:\n  {}",
        overlapping.join("\n  "),
    );
}

/// What is written on the line of the row named `name` that says how a volume
/// is drawn: « opaque », « transparente » or « invisible ».
fn state_written_on(app: &driver::App, name: &str) -> Vec<String> {
    let line = row(app, name);
    app.get_all_by(|_| true)
        .filter(|node| node.accesskit_node().bounding_box().is_some())
        .filter(|node| {
            let at = node.rect().center().y;
            at > line.top() && at < line.bottom()
        })
        .flat_map(|node| {
            let read = node.accesskit_node();
            [read.label(), read.value()]
        })
        .flatten()
        .filter(|said| ["opaque", "transparente", "invisible"].contains(&said.as_str()))
        .collect()
}

#[test]
fn clicking_a_volume_s_visibility_button_writes_its_next_state_on_its_row() {
    let mut app =
        driver::a_block("clicking_a_volume_s_visibility_button_writes_its_next_state_on_its_row");
    assert_eq!(
        state_written_on(&app, "Extrusion 1"),
        ["opaque"],
        "the block raised, showing {:?}",
        driver::on_screen(&app),
    );

    let mut written = Vec::new();
    for button in ["Rendre transparente", "Rendre invisible", "Rendre opaque"] {
        driver::click_exactly(&mut app, button);
        written.push((button, state_written_on(&app, "Extrusion 1")));
    }

    assert_eq!(
        written,
        [
            ("Rendre transparente", vec!["transparente".to_string()]),
            ("Rendre invisible", vec!["invisible".to_string()]),
            ("Rendre opaque", vec!["opaque".to_string()]),
        ],
        "each click on the block's visibility button, and what its row writes after it",
    );
}

#[test]
fn the_part_panel_holds_no_body_row_any_more() {
    let app = driver::a_block("the_part_panel_holds_no_body_row_any_more");
    assert!(
        driver::the_row(&app, "Extrusion 1").is_some(),
        "the fixture wants the block in the panel, showing {:?}",
        driver::on_screen(&app),
    );

    let body_rows: Vec<String> = app
        .get_all_by(|_| true)
        .flat_map(|node| {
            let read = node.accesskit_node();
            [read.label(), read.value()]
        })
        .flatten()
        .filter(|said| said.starts_with("Corps"))
        .collect();

    assert!(body_rows.is_empty(), "the panel still reads {body_rows:?}",);
}

#[test]
fn clicking_make_everything_opaque_brings_every_row_back_to_opaque() {
    let mut app =
        driver::a_block("clicking_make_everything_opaque_brings_every_row_back_to_opaque");
    assert!(
        !driver::is_enabled(&app, "Tout rendre opaque"),
        "every volume opaque, « Tout rendre opaque » is offered as though it would do something",
    );
    driver::click_exactly(&mut app, "Rendre transparente");
    assert_eq!(
        state_written_on(&app, "Extrusion 1"),
        ["transparente"],
        "the fixture wants the block made see-through first",
    );
    assert!(
        driver::is_enabled(&app, "Tout rendre opaque"),
        "the block see-through, « Tout rendre opaque » is greyed",
    );

    driver::click_the_button(&mut app, "Tout rendre opaque");

    assert_eq!(
        state_written_on(&app, "Extrusion 1"),
        ["opaque"],
        "« Tout rendre opaque » clicked, showing {:?}",
        driver::on_screen(&app),
    );
}
