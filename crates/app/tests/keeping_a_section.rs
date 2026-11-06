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
//!   `a_saved_section_reopened_from_the_tree_comes_back_on_its_plane_its_offset_and_its_sides`,
//!   and changed there and kept again by
//!   `editing_a_saved_section_through_its_panel_keeps_one_section_at_the_new_distance`
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
//!   nothing; picking another plane mends it —
//!   `a_section_whose_face_is_gone_reads_face_lost_in_the_tree`; that it cuts
//!   nothing and is mended by
//!   `a_section_whose_face_is_gone_is_broken_cuts_nothing_and_another_plane_mends_it`
//!   and `a_section_whose_face_a_later_step_takes_away_is_broken_and_cuts_nothing`,
//!   in `crates/part/tests/a_section_is_kept_in_the_part.rs`. Nothing deletes a
//!   step of matter today, so the face is taken away by erasing the rectangle
//!   its extrusion was raised from
//! - The volume and the exported STL of a part are the same with a section
//!   active as without — no test: held by
//!   `a_saved_section_leaves_the_volume_the_stl_and_the_step_as_they_were`, in
//!   `crates/part/tests/a_section_is_kept_in_the_part.rs`; which section is
//!   active lives in the viewport, and the export is handed the document alone
//! - A sketch made on a section moves with it when the section's offset is
//!   edited, and what was extruded from it is rebuilt — no test: held by
//!   `a_sketch_on_a_section_moves_with_its_offset_and_what_was_cut_from_it_is_rebuilt`
//!   and, through the file and a compaction,
//!   `a_sketch_on_a_section_still_follows_it_after_the_caopart_and_a_compaction`,
//!   in `crates/part/tests/a_section_is_kept_in_the_part.rs`
//! - Opening that sketch switches its section on; leaving it puts the sections
//!   back as they were —
//!   `opening_a_sketch_made_on_a_section_switches_it_on_and_leaving_puts_the_sections_back`
//! - Deleting a section a sketch stands on asks first —
//!   `deleting_a_section_a_sketch_stands_on_asks_first`, and the sketch left
//!   where the question says by
//!   `deleting_a_section_leaves_the_sketch_on_it_where_it_last_stood` and
//!   `deleting_a_section_on_a_face_leaves_the_sketch_on_it_where_the_face_had_taken_it`,
//!   in `crates/part/tests/a_section_is_kept_in_the_part.rs`
//! - Undo takes back making, editing and deleting a section —
//!   `undo_takes_back_a_section_saved_from_its_panel` and
//!   `saving_again_after_the_save_was_undone_keeps_a_section_again`, and the editing and the
//!   deleting by `undo_takes_back_making_editing_and_deleting_a_section`, in
//!   `crates/part/tests/a_section_is_kept_in_the_part.rs`

mod driver;

use cao_part::history::StepKind;
use cao_part::{Operation, PartDocument};
use cao_sketch::WorkPlane;
use egui_kittest::kittest::{NodeT, Queryable};

/// Where the block's face stands in front of the eye, and so where a section
/// across it shows its cut: given from the centre of the drawing area, as
/// `driver::on_canvas` reads it.
const ON_THE_BLOCK: (f32, f32) = (-28.55, 16.0);

/// The same, lower: halfway down the half of the block the section opened
/// through its middle leaves standing, well clear of the line it cuts along.
const ON_THE_BLOCK_UNDER_THE_CUT: (f32, f32) = (-27.6, 60.0);

/// The corners of the rectangle the block is raised off, given from the centre
/// of the drawing area: the clicks aimed at the block land on it only if the
/// block was drawn in the same terms.
const THE_BLOCK_S_CORNERS: [(f32, f32); 2] = [(-178.55, -94.0), (121.45, 126.0)];

/// A block raised off a rectangle drawn on a plane of the origin, its sketch
/// then finished.
fn a_block(test: &str) -> driver::App {
    let mut app = driver::open(test);
    driver::create_a_part(&mut app, "Bloc");
    driver::start_a_sketch(&mut app);
    driver::click(&mut app, "Rectangle (R)");
    driver::click_on_canvas(&mut app, THE_BLOCK_S_CORNERS[0]);
    driver::click_on_canvas(&mut app, THE_BLOCK_S_CORNERS[1]);
    driver::click_exactly(&mut app, "Extrusion");
    driver::click_exactly(&mut app, "1");
    driver::click(&mut app, "Ajout de matière");
    driver::click_on_canvas(&mut app, ON_THE_BLOCK);
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
    let at = the_row(app, label)
        .unwrap_or_else(|| panic!("no button reads « {label} »"))
        .rect()
        .center();
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
    let at = the_row(app, label)
        .unwrap_or_else(|| panic!("no button reads « {label} »"))
        .rect()
        .center();
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

/// The button of the tree, or of a panel, that reads exactly this — never the
/// header above the view, which names the same section while a sketch on it is
/// open.
fn the_row<'a>(app: &'a driver::App, name: &str) -> Option<egui_kittest::Node<'a>> {
    app.query_all_by_role(egui::accesskit::Role::Button)
        .find(|node| node.accesskit_node().label().as_deref() == Some(name))
}

fn in_the_tree(app: &driver::App, name: &str) -> bool {
    the_row(app, name).is_some()
}

fn is_active(app: &driver::App, name: &str) -> bool {
    assert!(
        in_the_tree(app, name),
        "the tree holds no « {name} », showing {:?}",
        driver::on_screen(app),
    );
    the_row(app, name)
        .is_some_and(|row| row.accesskit_node().toggled() == Some(egui::accesskit::Toggled::True))
}

fn switch_on(app: &mut driver::App, name: &str) {
    if !is_active(app, name) {
        the_row(app, name).expect("the row is in the tree").click();
        app.run();
    }
}

/// Starts a sketch where the block is, which is on the cut while a section
/// across it shows one, and finishes it.
fn a_sketch_on_the_cut(app: &mut driver::App) {
    click_in_the_ribbon(app, "Nouvelle esquisse");
    driver::click_on_canvas(app, ON_THE_BLOCK);
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
    let distance: f64 = driver::field_beside(&app, "Distance")
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
    driver::type_beside(&mut app, "Distance", "3");
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
        driver::field_beside(&app, "Distance"),
        "3",
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

/// The rows of the tree that name a section, whatever mark they carry.
fn sections_in_the_tree(app: &driver::App) -> Vec<String> {
    app.query_all_by_role(egui::accesskit::Role::Button)
        .filter_map(|node| node.accesskit_node().label())
        .filter(|read| {
            read.strip_prefix("Coupe ")
                .is_some_and(|rest| rest.starts_with(|first: char| first.is_ascii_digit()))
        })
        .collect()
}

#[test]
fn saving_again_after_the_save_was_undone_keeps_a_section_again() {
    let mut app = a_block("keeping_a_section_saved_undone_saved");
    click_in_the_ribbon(&mut app, "Vue en coupe");
    driver::click(&mut app, "Plan YZ");
    driver::click_the_button(&mut app, "Enregistrer la coupe");
    driver::press_with_command(&mut app, egui::Key::Z);
    assert!(
        sections_in_the_tree(&app).is_empty() && the_panel_is_open(&app),
        "undo should leave the panel open and no section in the tree, showing {:?}",
        driver::on_screen(&app),
    );

    driver::click_the_button(&mut app, "Enregistrer la coupe");

    assert_eq!(
        sections_in_the_tree(&app).len(),
        1,
        "« Enregistrer la coupe » after an undo kept nothing, showing {:?}",
        driver::on_screen(&app),
    );
}

#[test]
fn saving_after_the_section_was_deleted_from_the_tree_makes_a_new_one() {
    let mut app = a_block("keeping_a_section_saved_deleted_saved");
    driver::click_the_button(&mut app, "Fichiers");
    click_in_the_ribbon(&mut app, "Vue en coupe");
    driver::click(&mut app, "Plan YZ");
    driver::click_the_button(&mut app, "Enregistrer la coupe");
    right_click(&mut app, "Coupe 1");
    driver::click_the_button(&mut app, "Supprimer");
    assert!(
        sections_in_the_tree(&app).is_empty() && the_panel_is_open(&app),
        "deleting should leave the panel open and no section in the tree, showing {:?}",
        driver::on_screen(&app),
    );

    driver::click_the_button(&mut app, "Enregistrer la coupe");

    assert_eq!(
        sections_in_the_tree(&app).len(),
        1,
        "« Enregistrer la coupe » after the section was deleted kept nothing, showing {:?}",
        driver::on_screen(&app),
    );
}

#[test]
fn editing_a_saved_section_through_its_panel_keeps_one_section_at_the_new_distance() {
    let mut app = a_block("keeping_a_section_edited_in_its_panel");
    save_a_section(&mut app, "Plan YZ");
    double_click(&mut app, "Coupe 1");
    driver::type_beside(&mut app, "Distance", "2");
    driver::click_the_button(&mut app, "Enregistrer la coupe");
    close_the_panel(&mut app);

    let kept = sections_in_the_tree(&app);
    double_click(&mut app, "Coupe 1");

    assert_eq!(kept, ["Coupe 1"], "editing the section made another one");
    assert_eq!(
        driver::field_beside(&app, "Distance"),
        "2",
        "the section edited to 2 came back at another distance",
    );
}

/// The rows of the history panel, numbered as it numbers them, each with
/// whether it reads as the current one.
fn history_rows(app: &driver::App) -> Vec<(usize, bool)> {
    app.query_all_by_role(egui::accesskit::Role::Button)
        .filter_map(|node| {
            let node = node.accesskit_node();
            let label = node.label()?;
            let number = label.split_once(". ")?.0.parse().ok()?;
            let current = node.toggled() == Some(egui::accesskit::Toggled::True);
            Some((number, current))
        })
        .collect()
}

#[test]
fn the_history_still_reads_its_last_row_as_current_once_a_section_is_saved_and_clicking_it_keeps_the_section()
 {
    let mut app = a_block("keeping_a_section_history");
    save_a_section(&mut app, "Plan YZ");
    driver::open_the_history(&mut app);

    let rows = history_rows(&app);
    let &(last, current) = rows
        .iter()
        .max_by_key(|(number, _)| *number)
        .unwrap_or_else(|| {
            panic!(
                "the history lists no row, showing {:?}",
                driver::on_screen(&app)
            )
        });
    let label = app
        .query_all_by_role(egui::accesskit::Role::Button)
        .filter_map(|node| node.accesskit_node().label())
        .find(|read| read.starts_with(&format!("{last}. ")))
        .expect("the last row is on screen");
    driver::click_exactly(&mut app, &label);

    assert!(
        current,
        "with a section saved last, no row of the history reads as current: {rows:?}",
    );
    assert!(
        in_the_tree(&app, "Coupe 1"),
        "clicking the last row of the history took « Coupe 1 » away",
    );
}

#[test]
fn a_section_whose_face_is_gone_reads_face_lost_in_the_tree() {
    let mut app = a_block("keeping_a_section_face_lost");
    click_in_the_ribbon(&mut app, "Vue en coupe");
    driver::click(&mut app, "Face cliquée");
    driver::click_on_canvas(&mut app, ON_THE_BLOCK_UNDER_THE_CUT);
    driver::click_the_button(&mut app, "Enregistrer la coupe");
    close_the_panel(&mut app);
    double_click(&mut app, "Esquisse 1");
    driver::click(&mut app, "Sélection (S)");
    driver::press_with_command(&mut app, egui::Key::A);
    let away_from_the_drawing = driver::on_canvas(&app, (321.45, 156.0));
    app.hover_at(away_from_the_drawing);
    app.run();
    driver::press(&mut app, egui::Key::Delete);
    click_in_the_ribbon(&mut app, "Terminer");

    assert_eq!(
        sections_in_the_tree(&app),
        ["Coupe 1 — face perdue"],
        "the rectangle under the block erased, the section on its face is not marked",
    );
}

#[test]
fn the_history_numbers_its_rows_one_after_another_around_the_hidden_section_steps() {
    let mut app = a_block("keeping_a_section_history_numbers");
    save_a_section(&mut app, "Plan YZ");
    save_a_section(&mut app, "Plan XZ");
    double_click(&mut app, "Coupe 1");
    driver::type_beside(&mut app, "Distance", "4");
    driver::click_the_button(&mut app, "Enregistrer la coupe");
    close_the_panel(&mut app);
    switch_on(&mut app, "Coupe 1");
    a_sketch_on_the_cut(&mut app);
    driver::open_the_history(&mut app);

    let mut numbers: Vec<usize> = history_rows(&app)
        .into_iter()
        .map(|(number, _)| number)
        .collect();
    numbers.sort_unstable();

    let one_after_another: Vec<usize> = (1..=numbers.len()).collect();
    assert_eq!(
        numbers, one_after_another,
        "the history skips numbers where the section steps are hidden",
    );
}

/// What the bar above the view says, beside the part's name: the plane the
/// view is on. The bar is the screen's first row, so its labels are the ones
/// standing level with the highest label of all.
fn the_header(app: &driver::App) -> Vec<String> {
    let labels: Vec<_> = app
        .query_all_by_role(egui::accesskit::Role::Label)
        .map(|node| (node.rect(), node.accesskit_node().value()))
        .collect();
    let first_row = labels
        .iter()
        .map(|(rect, _)| rect.min.y)
        .fold(f32::INFINITY, f32::min);
    labels
        .into_iter()
        .filter(|(rect, _)| rect.min.y < first_row + 1.0)
        .filter_map(|(_, said)| said)
        .collect()
}

#[test]
fn a_sketch_on_a_section_names_the_section_above_the_view() {
    let mut app = a_block("keeping_a_section_header");
    save_a_section(&mut app, "Plan YZ");
    switch_on(&mut app, "Coupe 1");

    click_in_the_ribbon(&mut app, "Nouvelle esquisse");
    driver::click_on_canvas(&mut app, ON_THE_BLOCK);
    let header = the_header(&app);

    assert!(
        driver::is_enabled(&app, "Terminer"),
        "the click on the cut started no sketch, showing {:?}",
        driver::on_screen(&app),
    );
    assert!(
        header.iter().any(|said| said.contains("Coupe 1"))
            && !header.iter().any(|said| said.contains("Face de la pièce")),
        "a sketch on « Coupe 1 » is announced above the view as {header:?}",
    );
}
