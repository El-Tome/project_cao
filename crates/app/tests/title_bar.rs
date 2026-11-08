//! The two rows at the top of the part screen, as the redesign draws them: the
//! title bar, a breadcrumb with the message in a pill, and the tab row under
//! it (`docs/redesign/part-screen.md` P1 and P3).
//!
//! Closes #726.
//! - the tab row: the « CAO » logo, shown by default on a fresh install, then
//!   the tabs « Esquisse », « Extrusion », « Export », then exactly two panel
//!   toggles « Pièce » and « Variables », a separator, and « Préférences »; no
//!   « Historique » toggle, no « Fichiers » toggle on the part screen —
//!   `a_fresh_install_reads_the_logo_the_three_tabs_then_part_variables_and_preferences`,
//!   and the separator by
//!   `a_separator_stands_between_the_variables_and_the_preferences`
//! - the existing Preferences switch for the logo stays —
//!   `with_the_logo_switched_off_the_tab_row_opens_on_the_tabs`
//! - H still toggles the history panel — `h_still_opens_and_closes_the_history`
//! - the Files panel is no longer shown on the part screen —
//!   `the_part_screen_shows_no_files_panel_and_b_does_not_bring_it_back`
//! - it stays on the home screen, always shown there as today —
//!   `going_home_shows_the_files_panel_there`
//! - the Files command (`Command::ToggleExplorer`, key B) is removed
//!   altogether: no command, no shortcut, no label —
//!   `the_preferences_list_no_files_command_and_nothing_on_b`. That no variant
//!   is left for it is held at compile time: `screens/ribbon/icons/tests.rs`
//!   lists every command with no wildcard, and no longer lists it
//! - the title bar, first row of the part screen, reads « Accueil » with the
//!   home icon, « / », the part's name, « / », and the view in a pill —
//!   `the_title_bar_reads_home_the_part_s_name_and_the_view_between_slashes`,
//!   the icon by `the_way_home_wears_the_home_icon`, and the pill by
//!   `the_view_stands_in_a_pill_at_the_end_of_the_breadcrumb`
//! - clicking « Accueil » goes home as today —
//!   `home_in_the_title_bar_goes_back_to_the_home_screen`
//! - the part's name shows its file path on hover —
//!   `the_part_s_name_shows_its_file_path_on_hover`
//! - the view pill reads, in this order: « Esquisse N » when a sketch is open,
//!   N its rank as the Part tree words it; else « Vue en coupe » while the
//!   section view is on; else « Coupe N » while a kept section is shown; else
//!   the plane's label while the view stands on a plane with no sketch open;
//!   else « Vue 3D libre ». The order is held case by case beside the wording,
//!   in `crates/app/src/wording/view/tests.rs`, under its own `Closes #726`;
//!   that the screen hands it what it shows, by
//!   `the_view_pill_follows_the_sketch_the_section_view_and_the_kept_section`
//! - the message shows centred in the title bar, in a pill with an amber dot —
//!   `a_tool_s_hint_shows_centred_in_the_title_bar_in_a_pill_with_an_amber_dot`
//! - every hint, refusal and confirmation that shows today still shows, there —
//!   no test: held by every `driver::says` and `driver::said` test of
//!   `crates/app/tests/`, which read the message wherever it is drawn and stay
//!   green on this branch; that it is drawn in the pill is held by the bullet
//!   above, the message having one slot and one place it is drawn
//! - a file, save or export error shows in the pill, in red, on the part
//!   screen, and the red bar at the bottom is no longer drawn there —
//!   `a_save_that_fails_shows_its_error_in_red_in_the_pill_and_not_at_the_bottom`
//! - an export error in particular — no test: an export asks the system's save
//!   dialog where to write, which a headless driver cannot answer; it writes
//!   its error where the save does, `CaoApp.error`, and the bullet above holds
//!   the road from there to the pill
//! - an error takes the pill, and the message it replaces is gone —
//!   `an_error_takes_the_pill_from_the_message_before_it`
//! - an error gives way to the next message or the next command, like every
//!   message today — `an_error_gives_way_to_the_next_command`
//! - the home screen has no pill, so the red bar stays there: an open or create
//!   failure is seen on the home screen —
//!   `a_part_that_cannot_be_created_says_so_on_the_home_screen`, and a failed
//!   save on leaving the part by
//!   `a_save_that_fails_on_leaving_the_part_is_seen_on_the_home_screen`
//!
//! What must not break:
//! - going home, with no unsaved-changes confirmation, as today (autosave) and
//!   with none added — `home_in_the_title_bar_goes_back_to_the_home_screen`
//! - every hint, refusal and confirmation still shows — no test: the
//!   `driver::says` and `driver::said` tests of `crates/app/tests/`, as above
//! - an error on save is seen until the next gesture —
//!   `a_save_error_stays_in_the_pill_until_the_next_gesture`
//!
//! The issue left four things open, decided before this file was written. The
//! view pill reads, in this order: the sketch open, the section view on, a kept
//! section shown, the plane the view stands on, the free view — where the bar
//! used to read the section's name for a sketch laid on one, which
//! `keeping_a_section.rs` held until this issue deleted that test. Which plane
//! or section an open sketch stands on goes to the sketch's own pill, #727. The Files command
//! goes altogether, since the part screen was the only place it acted. An error
//! takes the pill and gives way like a message. The home screen has no pill and
//! keeps the red bar.
//!
//! A row of the screen is read off the accessibility tree: every name and every
//! text whose middle stands within the height of one widget known to be on it,
//! left to right, a text once rather than once more for the run of glyphs
//! `egui` lays under it. A separator carries no name, and is read off the frame's
//! shapes instead.

mod driver;

use egui::accesskit::Role;
use egui_kittest::kittest::{NodeT, Queryable};
use glam::{DVec2, DVec3};

#[test]
fn a_fresh_install_reads_the_logo_the_three_tabs_then_part_variables_and_preferences() {
    let app = a_part("a_fresh_install_reads_the_logo_the_three_tabs");

    assert_eq!(
        the_tab_row(&app),
        [
            "CAO",
            "Esquisse",
            "Extrusion",
            "Export",
            "Pièce",
            "Variables",
            "Préférences",
        ],
        "the tab row of a fresh install reads otherwise than the redesign draws it",
    );
}

#[test]
fn a_separator_stands_between_the_variables_and_the_preferences() {
    let app = a_part("a_separator_stands_between_the_variables_and_the_preferences");

    let variables = the_button_named(&app, "Variables");
    let preferences = the_button_named(&app, "Préférences");
    let between = egui::Rangef::new(variables.right(), preferences.left());
    let row = preferences.y_range();

    assert!(
        lines_painted(&app).into_iter().any(|line| {
            line.width() <= THIN
                && between.contains(line.center().x)
                && row.contains(line.center().y)
        }),
        "nothing is painted standing up between « Variables » at {variables:?} and \
         « Préférences » at {preferences:?}",
    );
}

#[test]
fn with_the_logo_switched_off_the_tab_row_opens_on_the_tabs() {
    let mut settings = cao_prefs::Settings::default();
    settings.toolbar.show_logo = false;
    let mut app = driver::open_held_with(
        "with_the_logo_switched_off_the_tab_row_opens_on_the_tabs",
        settings,
    );
    driver::create_a_part(&mut app, "Platine");

    let row = the_tab_row(&app);
    assert_eq!(
        row.first().map(String::as_str),
        Some("Esquisse"),
        "with the logo switched off in the preferences, the tab row reads {row:?}",
    );
}

#[test]
fn h_still_opens_and_closes_the_history() {
    let mut app = a_part("h_still_opens_and_closes_the_history");

    let before = the_history_is_shown(&app);
    driver::press(&mut app, egui::Key::H);
    let opened = the_history_is_shown(&app);
    driver::press(&mut app, egui::Key::H);
    let closed = the_history_is_shown(&app);

    assert_eq!(
        (before, opened, closed),
        (false, true, false),
        "the history shown (before, after H, after H again)",
    );
}

#[test]
fn the_part_screen_shows_no_files_panel_and_b_does_not_bring_it_back() {
    let mut app = a_part("the_part_screen_shows_no_files_panel");

    let mut shown = vec![the_files_are_shown(&app)];
    for _ in 0..2 {
        driver::press(&mut app, egui::Key::B);
        shown.push(the_files_are_shown(&app));
    }

    assert_eq!(
        shown,
        [false, false, false],
        "the files panel shown on the part screen (as it opens, after B, after B again)",
    );
}

#[test]
fn going_home_shows_the_files_panel_there() {
    let mut app = a_part("going_home_shows_the_files_panel_there");

    driver::click(&mut app, "Accueil");

    assert!(
        the_files_are_shown(&app),
        "the home screen shows no files panel: {:?}",
        driver::on_screen(&app),
    );
}

#[test]
fn the_preferences_list_no_files_command_and_nothing_on_b() {
    let mut app = a_part("the_preferences_list_no_files_command_and_nothing_on_b");

    driver::press_with_command(&mut app, egui::Key::Comma);
    driver::click_exactly(&mut app, "Raccourcis");

    assert!(
        driver::says(&app, "Panneau Historique"),
        "the list of shortcuts is not on screen, which would make every check below \
         pass for nothing: {:?}",
        driver::said(&app),
    );
    assert!(
        !driver::says(&app, "Panneau Fichiers"),
        "the shortcuts still list a command for the files panel",
    );
    let on_b: Vec<egui::Rect> = app
        .query_all_by_label("B")
        .filter(|node| node.accesskit_node().role() == Role::Button)
        .map(|node| node.rect())
        .collect();
    assert!(
        on_b.is_empty(),
        "a command still answers to B on a fresh install, its chord at {on_b:?}",
    );
}

#[test]
fn the_title_bar_reads_home_the_part_s_name_and_the_view_between_slashes() {
    let app = a_part("the_title_bar_reads_home_the_part_s_name");

    assert_eq!(
        the_title_bar(&app),
        ["Accueil", "/", "Platine", "/", "Vue 3D libre"],
        "the title bar of a part just made, in the free view, with nothing to say yet",
    );
}

#[test]
fn the_way_home_wears_the_home_icon() {
    let app = a_part("the_way_home_wears_the_home_icon");

    let home = the_button_named(&app, "Accueil");
    let worn = icons_painted_within(&app, home);

    assert!(
        worn.iter().any(|icon| icon == "home"),
        "« Accueil » at {home:?} wears {worn:?}, not the `home` icon",
    );
}

#[test]
fn the_view_stands_in_a_pill_at_the_end_of_the_breadcrumb() {
    let app = a_part("the_view_stands_in_a_pill");

    let view = the_topmost_reading(&app, "Vue 3D libre");
    let name = the_topmost_reading(&app, "Platine");
    let pills: Vec<egui::Rect> = rounded_backgrounds_painted(&app)
        .into_iter()
        .filter(|pill| pill.expand(HUGGED_WITHIN).contains_rect(view) && !pill.intersects(name))
        .collect();

    assert!(
        !pills.is_empty(),
        "nothing rounded and filled is painted around « Vue 3D libre » at {view:?} \
         alone, the part's name at {name:?} left outside it",
    );
}

#[test]
fn home_in_the_title_bar_goes_back_to_the_home_screen() {
    let mut app = a_part("home_in_the_title_bar_goes_back_to_the_home_screen");
    driver::start_a_sketch(&mut app);
    driver::draw_a_segment(&mut app);

    driver::click(&mut app, "Accueil");

    assert!(
        driver::shows(&app, "Créer") && !driver::shows(&app, "Nouvelle esquisse"),
        "one click on « Accueil » with a line just drawn did not land on the home \
         screen, showing {:?}",
        driver::on_screen(&app),
    );
}

#[test]
fn the_part_s_name_shows_its_file_path_on_hover() {
    let test = "the_part_s_name_shows_its_file_path_on_hover";
    let mut app = a_part(test);
    let path = std::env::temp_dir()
        .join("cao-driver")
        .join(test)
        .join("documents")
        .join("CAO")
        .join("Platine.caopart")
        .display()
        .to_string();

    let name = the_topmost_reading(&app, "Platine");
    app.hover_at(name.center());
    app.run_steps(4);

    assert!(
        driver::said(&app).iter().any(|line| line.contains(&path)),
        "hovering the part's name at {name:?} does not say {path}; the screen says {:?}",
        driver::said(&app),
    );
}

#[test]
fn the_view_pill_follows_the_sketch_the_section_view_and_the_kept_section() {
    let mut app = a_part("the_view_pill_follows_the_sketch_and_the_sections");
    let mut seen = vec![("a part just made", "Vue 3D libre", the_title_bar(&app))];

    driver::start_a_sketch(&mut app);
    seen.push(("its first sketch open", "Esquisse 1", the_title_bar(&app)));

    raise_a_block_and_finish_the_sketch(&mut app);
    seen.push(("the sketch finished", "Vue 3D libre", the_title_bar(&app)));

    click_in_the_ribbon(&mut app, "Vue en coupe");
    seen.push(("the section view on", "Vue en coupe", the_title_bar(&app)));

    driver::click(&mut app, "Plan YZ");
    driver::click_the_button(&mut app, "Enregistrer la coupe");
    driver::click_the_button(&mut app, "Fermer");
    driver::click_the_button(&mut app, "Coupe 1");
    assert!(
        the_tree_shows_on(&app, "Coupe 1"),
        "a click on its row in the tree did not switch the section kept on, showing {:?}",
        driver::on_screen(&app),
    );
    seen.push(("the section kept and shown", "Coupe 1", the_title_bar(&app)));

    click_in_the_ribbon(&mut app, "Nouvelle esquisse");
    driver::click_at_world_point(&mut app, ON_THE_CUT);
    seen.push((
        "a second sketch open on the cut",
        "Esquisse 2",
        the_title_bar(&app),
    ));

    let wrong: Vec<String> = seen
        .iter()
        .filter(|(_, view, bar)| !bar.iter().any(|read| read == view))
        .map(|(when, view, bar)| format!("{when}: wanted « {view} », the bar reads {bar:?}"))
        .collect();
    assert!(
        wrong.is_empty(),
        "the title bar does not name the view the screen shows:\n  {}",
        wrong.join("\n  "),
    );
}

/// How wide a separator may be painted, in points: a line, or a rectangle
/// standing in for one.
const THIN: f32 = 2.0;

fn a_part(test: &str) -> driver::App {
    let mut app = driver::open(test);
    driver::create_a_part(&mut app, "Platine");
    app
}

/// How far a pill may sit inside the text it carries, in points: the rect the
/// tree gives a label and the frame painted around it are rounded apart.
const HUGGED_WITHIN: f32 = 0.5;

/// The corners of the rectangle the block is raised off, points of its sketch
/// on the plane YZ, and a point inside it: those `keeping_a_section.rs` tuned.
const THE_BLOCK_S_CORNERS: [DVec2; 2] = [DVec2::new(-8.82, 44.59), DVec2::new(123.83, -52.69)];
const ON_THE_BLOCK: DVec2 = DVec2::new(57.5, -4.05);

/// Where a section of the block along YZ, through its middle, shows its cut.
const ON_THE_CUT: DVec3 = DVec3::new(5.0, 46.2, -2.47);

/// What the title bar reads, left to right: the row of « Accueil ».
fn the_title_bar(app: &driver::App) -> Vec<String> {
    the_row_of(app, the_topmost_reading(app, "Accueil"))
}

/// In the sketch just opened, a rectangle raised into a block by 1 mm, and the
/// sketch finished.
fn raise_a_block_and_finish_the_sketch(app: &mut driver::App) {
    driver::click(app, "Rectangle (R)");
    driver::click_at_sketch_point(app, THE_BLOCK_S_CORNERS[0]);
    driver::click_at_sketch_point(app, THE_BLOCK_S_CORNERS[1]);
    driver::click_exactly(app, "Extrusion");
    driver::click_exactly(app, "1");
    driver::click(app, "Ajout de matière");
    driver::click_at_sketch_point(app, ON_THE_BLOCK);
    driver::click(app, "Appliquer");
    driver::press(app, egui::Key::Escape);
    click_in_the_ribbon(app, "Terminer");
    driver::click_the_button(app, "Esquisse");
    assert!(
        !driver::is_enabled(app, "Terminer"),
        "the sketch the block was raised from is still being edited, showing {:?}",
        driver::on_screen(app),
    );
}

/// Whether the row of the Part tree that reads exactly this is switched on.
fn the_tree_shows_on(app: &driver::App, row: &str) -> bool {
    app.query_all_by_role(Role::Button)
        .find(|node| node.accesskit_node().label().as_deref() == Some(row))
        .is_some_and(|node| node.accesskit_node().toggled() == Some(egui::accesskit::Toggled::True))
}

/// The ribbon's button that reads this, whichever tab it sits under.
fn click_in_the_ribbon(app: &mut driver::App, label: &str) {
    let offered = |app: &driver::App| {
        app.query_all_by_role(Role::Button).any(|node| {
            node.accesskit_node()
                .label()
                .is_some_and(|read| read.starts_with(label))
        })
    };
    for tab in ["Esquisse", "Extrusion", "Export"] {
        if offered(app) {
            break;
        }
        driver::click_the_button(app, tab);
    }
    let node = app
        .query_all_by_role(Role::Button)
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
    driver::hold_the_canvas(app);
}

/// Every rectangle the last frame painted with rounded corners and something
/// to see — a fill or an outline.
fn rounded_backgrounds_painted(app: &driver::App) -> Vec<egui::Rect> {
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
            egui::Shape::Rect(rect) => {
                let corners = rect.corner_radius;
                let rounded = [corners.nw, corners.ne, corners.sw, corners.se]
                    .into_iter()
                    .all(|radius| radius > 0);
                let seen = rect.fill.a() > 0 || rect.stroke.width > 0.0;
                if rounded && seen {
                    found.push(rect.rect);
                }
            }
            _ => {}
        }
    }
    found
}

/// The ids of the icons the last frame painted inside a rectangle of the
/// screen, by the file each was loaded from — `bytes://icon/home.svg` is
/// `home`.
fn icons_painted_within(app: &driver::App, within: egui::Rect) -> Vec<String> {
    let within = within.expand(1.0);
    let textures = app.ctx.tex_manager();
    let textures = textures.read();
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
            egui::Shape::Rect(rect)
                if rect.fill_texture_id() != egui::TextureId::default()
                    && within.contains_rect(rect.rect) =>
            {
                let loaded_as = textures
                    .meta(rect.fill_texture_id())
                    .map(|meta| meta.name.clone())
                    .unwrap_or_default();
                let file = loaded_as.rsplit('/').next().unwrap_or(&loaded_as);
                found.push(file.strip_suffix(".svg").unwrap_or(file).to_owned());
            }
            _ => {}
        }
    }
    found
}

/// Whether the files panel is drawn: its « Nouveau dossier » is its own.
fn the_files_are_shown(app: &driver::App) -> bool {
    driver::shows(app, "Nouveau dossier")
}

/// Whether the history panel is drawn: its heading, which the tree carries as
/// a text where it carries the toggle of the same word as a name.
fn the_history_is_shown(app: &driver::App) -> bool {
    driver::says(app, "Historique")
}

/// What the tab row reads, left to right: every name and every text standing
/// on the row of the « Export » tab.
fn the_tab_row(app: &driver::App) -> Vec<String> {
    the_row_of(app, the_topmost_reading(app, "Export"))
}

/// Every name and every text whose middle stands within the height of `on`,
/// left to right.
fn the_row_of(app: &driver::App, on: egui::Rect) -> Vec<String> {
    let mut read: Vec<(f32, String)> = app
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
            on.y_range()
                .contains(at.center().y)
                .then_some((at.left(), said))
        })
        .collect();
    read.sort_by(|one, other| one.0.total_cmp(&other.0));
    read.into_iter().map(|(_, said)| said).collect()
}

/// The highest widget on screen named or reading exactly this.
fn the_topmost_reading(app: &driver::App, text: &str) -> egui::Rect {
    app.query_all_by(|node| {
        node.label().as_deref() == Some(text) || node.value().as_deref() == Some(text)
    })
    .map(|node| node.rect())
    .min_by(|one, other| one.top().total_cmp(&other.top()))
    .unwrap_or_else(|| {
        panic!(
            "nothing on screen reads « {text} »: {:?}",
            driver::on_screen(app)
        )
    })
}

/// The button of that name, for a word a heading of the screen carries too.
fn the_button_named(app: &driver::App, name: &str) -> egui::Rect {
    app.query_all_by_label(name)
        .find(|node| node.accesskit_node().role() == Role::Button)
        .map(|node| node.rect())
        .unwrap_or_else(|| {
            panic!(
                "no button is named « {name} »; the screen names {:?}",
                driver::on_screen(app)
            )
        })
}

/// The bounds of every line the last frame painted, and of every rectangle
/// painted as thin as one.
fn lines_painted(app: &driver::App) -> Vec<egui::Rect> {
    let mut lines = Vec::new();
    let mut pending: Vec<&egui::Shape> = app
        .output()
        .shapes
        .iter()
        .map(|clipped| &clipped.shape)
        .collect();
    while let Some(shape) = pending.pop() {
        match shape {
            egui::Shape::Vec(group) => pending.extend(group),
            egui::Shape::LineSegment { points, .. } => {
                lines.push(egui::Rect::from_two_pos(points[0], points[1]));
            }
            egui::Shape::Rect(rect) if rect.rect.width() <= THIN => lines.push(rect.rect),
            _ => {}
        }
    }
    lines
}
