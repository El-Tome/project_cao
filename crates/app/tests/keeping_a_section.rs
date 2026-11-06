//! A section kept in the part, driven through the whole application: its
//! panel, its row in the part tree, and what the sketches on it do.
//!
//! Closes #692.
//! - The section tool closed without « Enregistrer la coupe » leaves nothing in
//!   the tree nor in the `.caopart`, as today; a sketch started on that cut
//!   stays where it was made —
//!   `the_section_tool_closed_without_saving_leaves_nothing_behind_and_a_sketch_on_its_cut_stays_put`
//! - A section made, closed, then reopened from the tree comes back on its
//!   plane, its offset and its ways of drawing —
//!   `a_saved_section_reopened_from_the_tree_comes_back_on_its_plane_its_offset_and_its_sides`
//! - Two sections in one part: switching the second on switches the first off
//!   — `switching_the_second_section_on_switches_the_first_off`
//! - Saving and reopening keeps every section, and the part opens with none
//!   active — `a_part_reopened_keeps_its_sections_and_opens_with_none_active`,
//!   and what each section keeps through the file by
//!   `every_section_comes_back_from_the_caopart_with_its_plane_its_offset_and_its_sides`,
//!   in `crates/part/tests/a_section_is_kept_in_the_part.rs`
//! - A size changed under a section on a face moves the section with the face
//!   — no test: held by `a_section_on_a_face_moves_with_the_face_when_a_size_changes`,
//!   in `crates/part/tests/a_section_is_kept_in_the_part.rs`
//! - Deleting the face's extrusion leaves the section marked broken and cutting
//!   nothing; picking another plane mends it — no test: held by
//!   `a_section_whose_face_is_gone_is_broken_cuts_nothing_and_another_plane_mends_it`,
//!   in `crates/part/tests/a_section_is_kept_in_the_part.rs`. Nothing deletes a
//!   step of matter today, so the face is taken away by erasing the rectangle
//!   its extrusion was raised from
//! - The volume and the exported STL of a part are the same with a section
//!   active as without — no test: held by
//!   `a_saved_section_leaves_the_volume_and_the_stl_as_they_were`, in
//!   `crates/part/tests/a_section_is_kept_in_the_part.rs`; which section is
//!   active lives in the viewport, and the export is handed the document alone
//! - A sketch made on a section moves with it when the section's offset is
//!   edited, and what was extruded from it is rebuilt — no test: held by
//!   `a_sketch_on_a_section_moves_with_its_offset_and_what_was_cut_from_it_is_rebuilt`,
//!   in `crates/part/tests/a_section_is_kept_in_the_part.rs`
//! - Opening that sketch switches its section on; leaving it puts the sections
//!   back as they were —
//!   `opening_a_sketch_made_on_a_section_switches_it_on_and_leaving_puts_the_sections_back`
//! - Deleting a section a sketch stands on asks first —
//!   `deleting_a_section_a_sketch_stands_on_asks_first`
//! - Undo takes back making, editing and deleting a section —
//!   `undo_takes_back_a_section_saved_from_its_panel`, and the editing and the
//!   deleting by `undo_takes_back_making_editing_and_deleting_a_section`, in
//!   `crates/part/tests/a_section_is_kept_in_the_part.rs`

mod driver;

use cao_part::history::StepKind;
use cao_part::{Operation, PartDocument};
use cao_sketch::WorkPlane;
use egui_kittest::kittest::{NodeT, Queryable};

/// Where the block's face stands in front of the eye, and so where a section
/// across it shows its cut.
const ON_THE_BLOCK: (f32, f32) = (850.0, 510.0);

/// A block raised off a rectangle drawn on a plane of the origin, its sketch
/// then finished.
fn a_block(test: &str) -> driver::App {
    let mut app = driver::open(test);
    driver::create_a_part(&mut app, "Bloc");
    driver::start_a_sketch(&mut app);
    driver::click(&mut app, "Rectangle (R)");
    driver::click_at(&mut app, driver::A_POINT_ABOVE_THE_ORIGIN);
    driver::click_at(&mut app, driver::A_POINT_BELOW_AND_RIGHT);
    driver::click_exactly(&mut app, "Extrusion");
    driver::click_exactly(&mut app, "1");
    driver::click(&mut app, "Ajout de matière");
    driver::click_at(&mut app, ON_THE_BLOCK);
    driver::click(&mut app, "Appliquer");
    driver::press(&mut app, egui::Key::Escape);
    click_in_the_ribbon(&mut app, "Terminer");
    driver::click_the_button(&mut app, "Esquisse");
    assert!(
        !driver::is_enabled(&app, "Terminer"),
        "the sketch the block was raised from is still being edited, showing {:?}",
        driver::on_screen(&app),
    );
    app
}

fn a_button_reads(app: &driver::App, label: &str) -> bool {
    app.query_all_by_role(egui::accesskit::Role::Button)
        .filter_map(|node| node.accesskit_node().label())
        .any(|read| read.starts_with(label))
}

fn the_button_reading(app: &driver::App, label: &str) -> String {
    app.query_all_by_role(egui::accesskit::Role::Button)
        .filter_map(|node| node.accesskit_node().label())
        .find(|read| read.starts_with(label))
        .unwrap_or_else(|| panic!("no button reads « {label}… »"))
}

/// The ribbon's button that reads this, whichever tab it sits under.
fn click_in_the_ribbon(app: &mut driver::App, label: &str) {
    for tab in ["Esquisse", "Extrusion", "Export"] {
        if a_button_reads(app, label) {
            break;
        }
        driver::click_the_button(app, tab);
    }
    let node = app
        .query_all_by_role(egui::accesskit::Role::Button)
        .find(|node| {
            node.accesskit_node()
                .label()
                .is_some_and(|read| read.starts_with(label))
        })
        .unwrap_or_else(|| {
            panic!(
                "no tab of the ribbon offers « {label} », showing {:?}",
                driver::on_screen(app)
            )
        });
    node.click();
    app.run();
}

fn the_panel_is_open(app: &driver::App) -> bool {
    a_button_reads(app, "Inverser")
}

fn close_the_panel(app: &mut driver::App) {
    if the_panel_is_open(app) {
        driver::click_the_button(app, "Fermer");
    }
}

/// Two presses and releases close enough in time for egui to read one
/// double-click: each frame of the harness otherwise moves the clock a quarter
/// of a second, past what a double-click allows.
fn double_click(app: &mut driver::App, label: &str) {
    let at = app.get_by_label(label).rect().center();
    app.hover_at(at);
    app.run();
    let start = app.ctx.input(|input| input.time);
    for (rank, pressed) in [true, false, true, false].into_iter().enumerate() {
        app.input_mut().time = Some(start + 0.02 * (rank as f64 + 1.0));
        app.event(egui::Event::PointerButton {
            pos: at,
            button: egui::PointerButton::Primary,
            pressed,
            modifiers: egui::Modifiers::NONE,
        });
        app.step();
    }
    app.input_mut().time = None;
    app.run();
}

fn right_click(app: &mut driver::App, label: &str) {
    let at = app.get_by_label(label).rect().center();
    app.hover_at(at);
    app.run();
    for pressed in [true, false] {
        app.event(egui::Event::PointerButton {
            pos: at,
            button: egui::PointerButton::Secondary,
            pressed,
            modifiers: egui::Modifiers::NONE,
        });
        app.run();
    }
}

/// Opens the section tool, lays it along that plane, keeps it with « Enregistrer
/// la coupe » and closes the tool.
fn save_a_section(app: &mut driver::App, plane: &str) {
    click_in_the_ribbon(app, "Vue en coupe");
    driver::click(app, plane);
    driver::click_the_button(app, "Enregistrer la coupe");
    close_the_panel(app);
}

fn in_the_tree(app: &driver::App, name: &str) -> bool {
    app.query_all_by_label(name).next().is_some()
}

fn is_active(app: &driver::App, name: &str) -> bool {
    assert!(
        in_the_tree(app, name),
        "the tree holds no « {name} », showing {:?}",
        driver::on_screen(app),
    );
    driver::is_on(app, name)
}

fn switch_on(app: &mut driver::App, name: &str) {
    if !is_active(app, name) {
        driver::click_exactly(app, name);
    }
}

/// Starts a sketch where the block is, which is on the cut while a section
/// across it shows one, and finishes it.
fn a_sketch_on_the_cut(app: &mut driver::App) {
    click_in_the_ribbon(app, "Nouvelle esquisse");
    driver::click_at(app, ON_THE_BLOCK);
    assert!(
        driver::is_enabled(app, "Terminer"),
        "the click on the cut started no sketch, showing {:?}",
        driver::on_screen(app),
    );
    click_in_the_ribbon(app, "Terminer");
}

/// The part as the application left it on disk, once it was put away.
fn the_part_file(app: &mut driver::App, test: &str) -> PartDocument {
    driver::click(app, "Accueil");
    let path = std::env::temp_dir()
        .join("cao-driver")
        .join(test)
        .join("documents")
        .join("CAO")
        .join("Bloc.caopart");
    PartDocument::load(&cao_app::adapters::files::DiskFiles, &path)
        .unwrap_or_else(|err| panic!("the part at {path:?} does not open: {err:?}"))
}

fn reopen_the_part(app: &mut driver::App) {
    driver::click(app, "Accueil");
    let recent = app
        .query_all_by_label("Bloc")
        .last()
        .expect("the start menu lists the part");
    recent.click();
    app.run();
}

#[test]
fn the_section_tool_closed_without_saving_leaves_nothing_behind_and_a_sketch_on_its_cut_stays_put()
{
    let test = "keeping_a_section_closed_without_saving";
    let mut app = a_block(test);
    click_in_the_ribbon(&mut app, "Vue en coupe");
    driver::click(&mut app, "Plan YZ");
    let distance: f64 = driver::fields(&app)[0]
        .parse()
        .expect("the panel reads its distance");
    a_sketch_on_the_cut(&mut app);
    click_in_the_ribbon(&mut app, "Vue en coupe");
    close_the_panel(&mut app);

    let tree_holds_a_section = in_the_tree(&app, "Coupe 1");
    let part = the_part_file(&mut app, test);

    assert!(
        !tree_holds_a_section,
        "a section never saved is in the tree"
    );
    let kinds: Vec<StepKind> = part
        .history
        .steps()
        .iter()
        .map(|step| step.kind())
        .collect();
    assert_eq!(
        kinds,
        [StepKind::Sketch, StepKind::Extrusion, StepKind::Sketch],
        "the .caopart holds more than the block and the sketch on the cut",
    );
    let laid = part
        .history
        .operations()
        .iter()
        .rev()
        .find_map(|operation| match operation {
            Operation::CreateSketch { plane, on } => Some((*plane, *on)),
            _ => None,
        });
    let (plane, on) = laid.expect("the sketch on the cut is in the file");
    assert_eq!(on, None, "the sketch on a cut never saved hangs on a face");
    assert!(
        (plane.origin - WorkPlane::YZ.normal() * distance).length() < 1e-6,
        "the sketch on the cut was laid on {plane:?}, not {distance} along YZ",
    );
}

#[test]
fn a_saved_section_reopened_from_the_tree_comes_back_on_its_plane_its_offset_and_its_sides() {
    let mut app = a_block("keeping_a_section_reopened_from_the_tree");
    click_in_the_ribbon(&mut app, "Vue en coupe");
    driver::click(&mut app, "Plan YZ");
    let read = driver::fields(&app)[0].clone();
    driver::type_over(&mut app, &read, "3");
    driver::click(&mut app, "Derrière");
    driver::click(&mut app, "Devant");
    driver::click(&mut app, "Devant");
    let sides = (
        the_button_reading(&app, "Derrière"),
        the_button_reading(&app, "Devant"),
    );
    driver::click_the_button(&mut app, "Enregistrer la coupe");
    close_the_panel(&mut app);
    click_in_the_ribbon(&mut app, "Vue en coupe");
    close_the_panel(&mut app);

    assert!(
        in_the_tree(&app, "Coupe 1"),
        "« Enregistrer la coupe » left no « Coupe 1 » in the tree, showing {:?}",
        driver::on_screen(&app),
    );
    double_click(&mut app, "Coupe 1");

    assert!(
        the_panel_is_open(&app),
        "a double-click on « Coupe 1 » opens no section tool, showing {:?}",
        driver::on_screen(&app),
    );
    assert!(
        driver::is_on(&app, "Plan YZ"),
        "the section came back off YZ"
    );
    assert_eq!(
        driver::fields(&app),
        ["3"],
        "the section came back at another distance",
    );
    assert_eq!(
        (
            the_button_reading(&app, "Derrière"),
            the_button_reading(&app, "Devant"),
        ),
        sides,
        "the section came back drawing its sides another way",
    );
}

#[test]
fn switching_the_second_section_on_switches_the_first_off() {
    let mut app = a_block("keeping_a_section_two_sections");
    save_a_section(&mut app, "Plan YZ");
    save_a_section(&mut app, "Plan XZ");

    switch_on(&mut app, "Coupe 1");
    let first_alone = (is_active(&app, "Coupe 1"), is_active(&app, "Coupe 2"));
    switch_on(&mut app, "Coupe 2");
    let second_alone = (is_active(&app, "Coupe 1"), is_active(&app, "Coupe 2"));

    assert_eq!(
        first_alone,
        (true, false),
        "« Coupe 1 » switched on, the sections read (first, second) = {first_alone:?}",
    );
    assert_eq!(
        second_alone,
        (false, true),
        "« Coupe 2 » switched on, the sections read (first, second) = {second_alone:?}",
    );
}

#[test]
fn a_part_reopened_keeps_its_sections_and_opens_with_none_active() {
    let mut app = a_block("keeping_a_section_reopened_part");
    save_a_section(&mut app, "Plan YZ");
    save_a_section(&mut app, "Plan XZ");
    switch_on(&mut app, "Coupe 2");

    reopen_the_part(&mut app);

    assert!(
        in_the_tree(&app, "Coupe 1") && in_the_tree(&app, "Coupe 2"),
        "the part came back without its sections, showing {:?}",
        driver::on_screen(&app),
    );
    assert!(
        !is_active(&app, "Coupe 1") && !is_active(&app, "Coupe 2"),
        "the part opened with a section active",
    );
    assert!(
        !the_panel_is_open(&app),
        "the part opened with the section tool open"
    );
}

#[test]
fn opening_a_sketch_made_on_a_section_switches_it_on_and_leaving_puts_the_sections_back() {
    let mut app = a_block("keeping_a_section_sketch_on_it");
    save_a_section(&mut app, "Plan YZ");
    save_a_section(&mut app, "Plan XZ");
    switch_on(&mut app, "Coupe 1");
    a_sketch_on_the_cut(&mut app);
    switch_on(&mut app, "Coupe 2");
    let before = (is_active(&app, "Coupe 1"), is_active(&app, "Coupe 2"));

    double_click(&mut app, "Esquisse 2");
    let opened = driver::is_enabled(&app, "Terminer");
    let while_open = (is_active(&app, "Coupe 1"), is_active(&app, "Coupe 2"));
    click_in_the_ribbon(&mut app, "Terminer");
    let after = (is_active(&app, "Coupe 1"), is_active(&app, "Coupe 2"));

    assert_eq!(
        before,
        (false, true),
        "the second section was not the one on"
    );
    assert!(opened, "a double-click on « Esquisse 2 » did not reopen it");
    assert_eq!(
        while_open,
        (true, false),
        "the sketch on « Coupe 1 » open, the sections read (first, second) = {while_open:?}",
    );
    assert_eq!(
        after, before,
        "the sketch left, the sections were not put back as they were",
    );
}

#[test]
fn deleting_a_section_a_sketch_stands_on_asks_first() {
    let mut app = a_block("keeping_a_section_deleting");
    save_a_section(&mut app, "Plan YZ");
    save_a_section(&mut app, "Plan XZ");
    switch_on(&mut app, "Coupe 1");
    a_sketch_on_the_cut(&mut app);
    driver::click_the_button(&mut app, "Fichiers");
    assert!(
        !a_button_reads(&app, "Supprimer"),
        "the file list is still open, and its « Supprimer » would be clicked instead",
    );
    let asks = |app: &driver::App| app.query_all_by_label("Annuler").next().is_some();

    right_click(&mut app, "Coupe 2");
    driver::click_the_button(&mut app, "Supprimer");
    let free_one_asked = asks(&app);
    let free_one_gone = !in_the_tree(&app, "Coupe 2");
    right_click(&mut app, "Coupe 1");
    driver::click_the_button(&mut app, "Supprimer");
    let held_one_asked = asks(&app);
    let held_one_kept_while_asking = in_the_tree(&app, "Coupe 1");
    if held_one_asked {
        driver::click_exactly(&mut app, "Annuler");
    }
    let held_one_kept_on_cancel = in_the_tree(&app, "Coupe 1");

    assert!(
        free_one_gone && !free_one_asked,
        "a section nothing stands on is not deleted at once: asked {free_one_asked}, \
         gone {free_one_gone}",
    );
    assert!(
        held_one_asked,
        "a section a sketch stands on was deleted without a question, showing {:?}",
        driver::on_screen(&app),
    );
    assert!(
        held_one_kept_while_asking && held_one_kept_on_cancel,
        "« Annuler » did not keep the section the sketch stands on",
    );
}

#[test]
fn undo_takes_back_a_section_saved_from_its_panel() {
    let mut app = a_block("keeping_a_section_undo");
    save_a_section(&mut app, "Plan YZ");
    let saved = in_the_tree(&app, "Coupe 1");

    driver::press_with_command(&mut app, egui::Key::Z);
    let undone = !in_the_tree(&app, "Coupe 1");
    driver::press_with(
        &mut app,
        egui::Modifiers::COMMAND | egui::Modifiers::SHIFT,
        egui::Key::Z,
    );
    let redone = in_the_tree(&app, "Coupe 1");

    assert!(
        saved,
        "« Enregistrer la coupe » left no « Coupe 1 » in the tree"
    );
    assert!(undone, "undo left « Coupe 1 » in the tree");
    assert!(redone, "redo did not bring « Coupe 1 » back");
}
