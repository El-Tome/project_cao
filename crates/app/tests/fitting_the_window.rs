//! The toolbar laid out in windows of every size, and every one of its buttons
//! read back from the accessibility tree to see whether it is on screen.
//!
//! Closes #650.
//! - at 1280 wide, every button of the sketch toolbar is inside the window,
//!   « Congé » and « Chanfrein » included —
//!   `every_tool_of_the_toolbar_along_the_top_is_inside_the_window`
//! - the same holds with the toolbar on a side edge, at a window too short for
//!   one column — `every_tool_of_the_toolbar_down_a_side_can_be_scrolled_into_the_window`:
//!   a column that does not fit scrolls rather than wrapping into a second
//!   one, which egui lays out across the canvas
//! - a test checks it at several window sizes —
//!   `every_tool_of_the_toolbar_along_the_top_is_inside_the_window`, at three
//!   sizes, and the side one at three more

mod driver;

use cao_prefs::{Edge, Settings};
use egui_kittest::kittest::{NodeT, Queryable};

fn toolbar_on(edge: Edge) -> Settings {
    let mut settings = Settings::default();
    settings.toolbar.edge = edge;
    settings
}

fn opened(test: &str, edge: Edge, size: (f32, f32)) -> driver::App {
    let mut app = driver::open_with(test, size, toolbar_on(edge));
    driver::create_a_part(&mut app, "Platine");
    for tool in ["Congé", "Chanfrein", "Mesurer (M)"] {
        assert!(
            driver::shows(&app, tool),
            "{tool} is on the toolbar: {:?}",
            driver::on_screen(&app)
        );
    }
    app
}

fn is_outside(node: &egui_kittest::Node<'_>, (width, height): (f32, f32)) -> bool {
    node.accesskit_node()
        .bounding_box()
        .is_some_and(|rect| rect.x1 > f64::from(width) + 0.5 || rect.y1 > f64::from(height) + 0.5)
}

/// What lies, even in part, outside the window.
fn off_screen(app: &driver::App, size: (f32, f32)) -> Vec<String> {
    app.get_all_by(|_| true)
        .filter(|node| is_outside(node, size))
        .filter_map(|node| node.accesskit_node().label())
        .collect()
}

#[test]
fn every_tool_of_the_toolbar_along_the_top_is_inside_the_window() {
    for size in [(1280.0, 800.0), (1024.0, 768.0), (1512.0, 900.0)] {
        let app = opened("every_tool_along_the_top", Edge::Top, size);
        let outside = off_screen(&app, size);
        assert!(outside.is_empty(), "at {size:?}, off screen: {outside:?}");
    }
}

#[test]
fn every_tool_of_the_toolbar_down_a_side_can_be_scrolled_into_the_window() {
    for (edge, size) in [
        (Edge::Left, (1280.0, 800.0)),
        (Edge::Left, (1280.0, 600.0)),
        (Edge::Right, (1280.0, 600.0)),
    ] {
        let mut app = opened("every_tool_down_a_side", edge, size);
        let below = off_screen(&app, size);
        if size.1 < 700.0 {
            assert!(
                !below.is_empty(),
                "{edge:?} at {size:?} is short enough that the column runs past the bottom"
            );
        }
        for label in below {
            app.get_all_by_label(&label)
                .next()
                .expect("still there")
                .scroll_to_me();
            app.run();
            let node = app.get_all_by_label(&label).next().expect("still there");
            assert!(
                !is_outside(&node, size),
                "{edge:?} at {size:?}, {label} cannot be brought into the window"
            );
        }
    }
}
