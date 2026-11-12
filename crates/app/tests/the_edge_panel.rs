//! « Chanfrein » and « Congé » on the part, outside a sketch: the same two
//! buttons as in a sketch, which there act on a corner and here open the panel
//! an edge of the part is chamfered or rounded from.
//!
//! Closes #920 — the whole transcription is in
//! `crates/part/tests/an_edge_is_chamfered_or_rounded.rs`; these bullets are
//! the application's half of the ribbon.
//! - Outside a sketch, the part ribbon offers « Chanfrein » and « Congé »;
//!   clicking one opens its panel —
//!   `outside_a_sketch_the_part_tab_offers_chamfer_and_fillet_and_each_opens_its_edge_panel`
//! - Inside a sketch they still act on corners, unchanged — the fillet held
//!   by `the_fillet_picked_from_the_toolbar_rounds_the_corner_clicked_next`
//!   in `picking_a_tool.rs`, the chamfer here by
//!   `inside_a_sketch_the_chamfer_still_cuts_the_corner_clicked_next_and_opens_no_edge_panel`

mod driver;

use egui_kittest::kittest::NodeT;
use glam::DVec2;

/// What the chamfer's panel and the fillet's name the value they ask for.
const THE_CHAMFER_S_VALUE: &str = "Distance :";
const THE_FILLET_S_VALUE: &str = "Rayon :";

fn offered_and_enabled(app: &driver::App, tool: &str) {
    let button = driver::the_ribbon_s(app, tool).unwrap_or_else(|| {
        panic!(
            "the part's tab offers no « {tool} », showing {:?}",
            driver::on_screen(app)
        )
    });
    assert!(
        !button.accesskit_node().is_disabled(),
        "« {tool} » is greyed outside a sketch",
    );
}

fn opens_its_panel(app: &driver::App, tool: &str, value: &str, not: &str) {
    let what = format!("« {tool} » clicked outside a sketch");
    assert!(!driver::a_sketch_is_open(app), "{what}: a sketch opened");
    let field = driver::the_field_beside(app, value);
    assert!(
        !field.accesskit_node().is_disabled(),
        "{what}: the field beside « {value} » cannot be typed in",
    );
    assert!(
        !driver::reads(app, not),
        "{what}: the other panel's « {not} » is still on screen",
    );
    assert!(
        !driver::is_enabled(app, "Appliquer"),
        "{what}: « Appliquer » offered with no edge chosen",
    );
}

#[test]
fn outside_a_sketch_the_part_tab_offers_chamfer_and_fillet_and_each_opens_its_edge_panel() {
    let mut app = driver::a_block("outside_a_sketch_chamfer_and_fillet_open_their_panel");
    driver::click_exactly(&mut app, "Extrusion");

    offered_and_enabled(&app, "Chanfrein");
    offered_and_enabled(&app, "Congé");

    driver::click_exactly(&mut app, "Chanfrein");
    opens_its_panel(&app, "Chanfrein", THE_CHAMFER_S_VALUE, THE_FILLET_S_VALUE);

    driver::click_exactly(&mut app, "Congé");
    opens_its_panel(&app, "Congé", THE_FILLET_S_VALUE, THE_CHAMFER_S_VALUE);

    driver::click_the_button(&mut app, "Annuler");
    assert!(
        !driver::reads(&app, THE_FILLET_S_VALUE) && !driver::reads(&app, THE_CHAMFER_S_VALUE),
        "« Annuler » left the edge panel open, showing {:?}",
        driver::on_screen(&app),
    );
}

/// Where a second trait is drawn to from the end of the first, down and back
/// to the left, so that the two meet in a corner there: `picking_a_tool.rs`'s.
const BELOW_THE_FIRST_TRAIT: DVec2 = DVec2::new(78.22, -83.26);

#[test]
fn inside_a_sketch_the_chamfer_still_cuts_the_corner_clicked_next_and_opens_no_edge_panel() {
    let mut app = driver::open_on(
        "inside_a_sketch_the_chamfer_cuts",
        egui::vec2(1920.0, 800.0),
    );
    driver::create_a_part(&mut app, "Platine");
    driver::start_a_sketch(&mut app);
    driver::click(&mut app, "Ligne (L)");
    driver::click_at_sketch_point(&mut app, driver::A_POINT_ABOVE_THE_ORIGIN);
    driver::click_at_sketch_point(&mut app, driver::A_POINT_BELOW_AND_RIGHT);
    driver::click_at_sketch_point(&mut app, BELOW_THE_FIRST_TRAIT);
    driver::press(&mut app, egui::Key::Escape);

    driver::click_exactly(&mut app, "Chanfrein");
    assert!(
        !driver::reads(&app, THE_CHAMFER_S_VALUE),
        "the chamfer in a sketch opened the edge panel, showing {:?}",
        driver::on_screen(&app),
    );
    driver::click_at_sketch_point(&mut app, driver::A_POINT_BELOW_AND_RIGHT);
    driver::type_into_the_only_field(&mut app, "2");
    driver::press(&mut app, egui::Key::Enter);

    driver::open_the_history(&mut app);
    assert!(
        driver::shows(&app, "Coin chanfreiné"),
        "the corner clicked with the chamfer in hand is a step of the history, \
         showing {:?}",
        driver::on_screen(&app),
    );
}
