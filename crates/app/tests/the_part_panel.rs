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
//!
//! A row's left edge and height are read off the widget named as the row is:
//! the one the hand clicks. The driver reads them in points, which is what
//! the design file's pixels are — `egui_kittest` divides the window's
//! physical pixels by its scale before handing a rect back.

mod driver;

use egui::accesskit::{Role, Toggled};
use egui_kittest::kittest::{NodeT, Queryable};
use glam::DVec2;

const TOLERANCE: f32 = 0.5;

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
