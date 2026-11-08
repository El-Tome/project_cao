//! A section tilted in its panel, driven through the whole application: the
//! sketch laid on its cut, and the angles kept with it in the part. The
//! criteria of #693 are transcribed in
//! `crates/app/src/screens/viewport/render/tests.rs`, which names the tests
//! below.

mod driver;

use cao_part::PartDocument;
use cao_sketch::WorkPlane;
use egui_kittest::kittest::{NodeT, Queryable};
use glam::{DVec2, DVec3};

/// Inside the rectangle the block is raised off, a point of its sketch: where
/// the extrusion is told which region to raise.
const ON_THE_BLOCK: DVec2 = DVec2::new(57.5, -4.05);

/// Where the block's face stands in front of the eye, and so where a section
/// across it shows its cut: a point of the cut through the middle of the
/// block, on the line of sight the clicks first written for it took.
const ON_THE_CUT: DVec3 = DVec3::new(5.0, 46.2, -2.47);

/// The corners of the rectangle the block is raised off, points of its sketch:
/// the first is pulled onto the plane's axis.
const THE_BLOCK_S_CORNERS: [DVec2; 2] = [DVec2::new(-8.82, 44.59), DVec2::new(123.83, -52.69)];

/// A block raised off a rectangle drawn on a plane of the origin, its sketch
/// then finished.
fn a_block(test: &str) -> driver::App {
    let mut app = driver::open(test);
    driver::create_a_part(&mut app, "Bloc");
    driver::start_a_sketch(&mut app);
    driver::click(&mut app, "Rectangle (R)");
    driver::click_at_sketch_point(&mut app, THE_BLOCK_S_CORNERS[0]);
    driver::click_at_sketch_point(&mut app, THE_BLOCK_S_CORNERS[1]);
    driver::click_exactly(&mut app, "Extrusion");
    driver::click_exactly(&mut app, "1");
    driver::click(&mut app, "Ajout de matière");
    driver::click_at_sketch_point(&mut app, ON_THE_BLOCK);
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

/// The button of the tree that reads exactly this.
fn the_row<'a>(app: &'a driver::App, name: &str) -> Option<egui_kittest::Node<'a>> {
    app.query_all_by_role(egui::accesskit::Role::Button)
        .find(|node| node.accesskit_node().label().as_deref() == Some(name))
}

/// Switches a kept section on from its row in the tree, unless it is on.
fn switch_on(app: &mut driver::App, name: &str) {
    let row = the_row(app, name).unwrap_or_else(|| {
        panic!(
            "the tree holds no « {name} », showing {:?}",
            driver::on_screen(app)
        )
    });
    if row.accesskit_node().toggled() != Some(egui::accesskit::Toggled::True) {
        row.click();
        app.run();
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

/// Starts a sketch where the block is, which is on the cut while a section
/// across it shows one, and finishes it.
fn a_sketch_on_the_cut(app: &mut driver::App) {
    click_in_the_ribbon(app, "Nouvelle esquisse");
    driver::click_at_world_point(app, ON_THE_CUT);
    assert!(
        driver::is_enabled(app, "Terminer"),
        "the click on the tilted cut started no sketch, showing {:?}",
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

/// The middle of the block, where a section opened along a plane of the
/// origin stands its square.
fn the_middle_of(part: &PartDocument) -> DVec3 {
    let (low, high) = part.body().bounds().expect("the block holds matter");
    (low + high) / 2.0
}

/// Says how a plane a sketch was laid on stands against YZ turned 10° about
/// its first axis, Y, through the middle of the block: the plane the cut
/// stands on, or what is wrong with it.
fn lies_on_yz_turned_ten_degrees_through(plane: WorkPlane, middle: DVec3) -> Result<(), String> {
    let normal = plane.normal();
    let off_yz = normal
        .dot(DVec3::X)
        .abs()
        .clamp(-1.0, 1.0)
        .acos()
        .to_degrees();
    if (off_yz - 10.0).abs() > 1e-6 {
        return Err(format!("it stands {off_yz}° off YZ, not 10°: {plane:?}"));
    }
    if normal.dot(DVec3::Y).abs() > 1e-9 {
        return Err(format!(
            "it is turned about another axis than Y, which lies in YZ: {plane:?}"
        ));
    }
    if normal.dot(middle - plane.origin).abs() > 1e-6 {
        return Err(format!(
            "it misses the square's centre, the middle of the block {middle:?}: {plane:?}"
        ));
    }
    Ok(())
}

#[test]
fn a_sketch_started_on_a_tilted_cut_lands_on_the_tilted_plane() {
    let test = "tilting_a_section_sketch_on_the_cut";
    let mut app = a_block(test);
    click_in_the_ribbon(&mut app, "Vue en coupe");
    driver::click(&mut app, "Plan YZ");
    driver::type_beside(&mut app, "Inclinaison 1", "10");

    a_sketch_on_the_cut(&mut app);
    let part = the_part_file(&mut app, test);

    let laid = part.sketches()[1].plane;
    if let Err(wrong) = lies_on_yz_turned_ten_degrees_through(laid, the_middle_of(&part)) {
        panic!("the sketch started on the cut tilted 10° does not lie on it: {wrong}");
    }
}

#[test]
fn a_sketch_on_a_kept_tilted_section_lands_on_the_tilted_plane() {
    let test = "tilting_a_section_sketch_on_a_kept_one";
    let mut app = a_block(test);
    click_in_the_ribbon(&mut app, "Vue en coupe");
    driver::click(&mut app, "Plan YZ");
    driver::type_beside(&mut app, "Inclinaison 1", "10");
    driver::click_the_button(&mut app, "Enregistrer la coupe");
    close_the_panel(&mut app);
    switch_on(&mut app, "Coupe 1");

    a_sketch_on_the_cut(&mut app);
    let part = the_part_file(&mut app, test);

    let laid = part.sketches()[1].plane;
    if let Err(wrong) = lies_on_yz_turned_ten_degrees_through(laid, the_middle_of(&part)) {
        panic!(
            "the sketch started on « Coupe 1 », kept tilted 10°, does not lie on its cut: {wrong}"
        );
    }
}

/// What the field beside a label reads, as a number, whichever minus sign it
/// is written with.
fn the_number_beside(app: &driver::App, label: &str) -> f64 {
    let read = driver::field_beside(app, label);
    read.replace('\u{2212}', "-")
        .parse()
        .unwrap_or_else(|_| panic!("« {label} » reads {read:?}, which is no number"))
}

#[test]
fn a_tilted_section_comes_back_with_its_two_angles_from_the_caopart_and_from_the_tree() {
    let mut app = a_block("tilting_a_section_kept_tilted");
    click_in_the_ribbon(&mut app, "Vue en coupe");
    driver::click(&mut app, "Plan YZ");
    driver::type_beside(&mut app, "Inclinaison 1", "30");
    driver::type_beside(&mut app, "Inclinaison 2", "-15");
    driver::click_the_button(&mut app, "Enregistrer la coupe");
    close_the_panel(&mut app);

    reopen_the_part(&mut app);
    double_click(&mut app, "Coupe 1");

    assert!(
        the_panel_is_open(&app),
        "a double-click on « Coupe 1 » in the reopened part opens no section tool, showing {:?}",
        driver::on_screen(&app),
    );
    assert!(
        driver::is_on(&app, "Plan YZ"),
        "the section came back off YZ"
    );
    assert_eq!(
        (
            the_number_beside(&app, "Inclinaison 1"),
            the_number_beside(&app, "Inclinaison 2"),
        ),
        (30.0, -15.0),
        "the section kept tilted by 30° and -15° came back tilted otherwise",
    );
}
