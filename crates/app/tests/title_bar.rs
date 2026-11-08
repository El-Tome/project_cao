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
//! - hiding « Pièce » hides the history and the reader with it (T8, in the
//!   issue's "Decided on the way") — no test: out of scope here. In this issue
//!   the history is still a panel of its own, toggled by H, and there is no
//!   reader yet; the history folds into the Part panel, and T8 with it, in the
//!   Part panel lot, #735
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
//!   dialog where to write, which a headless driver cannot answer. That it
//!   hands back its error is held in `crates/app/src/export/tests.rs`, by
//!   `a_file_the_system_refuses_is_handed_back_as_an_error` and its STEP twin;
//!   the screen takes it as it takes a save's, and the bullet above holds the
//!   road from there to the pill
//! - an error takes the pill, and the message it replaces is gone —
//!   `an_error_takes_the_pill_from_the_message_before_it`
//! - an error gives way to the next message or the next command, like every
//!   message today — `an_error_gives_way_to_the_next_command`, which says
//!   nothing in its turn, and `an_error_gives_way_to_the_next_message`, the
//!   pill back in the tone of what is said. A message said again unchanged at
//!   every frame is not a next message: choosing a plane says its sentence
//!   with every move of the mouse, and an error from the Preferences stays
//!   through it — `a_preferences_error_stays_while_a_plane_is_being_chosen`.
//!   The rule, case by case, is held beside the title bar's presenter, in
//!   `crates/app/src/screens/title_bar/state/tests.rs`, under its own
//!   `Closes #726`
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
//! A save is made to fail with no window by putting a file where the folder of
//! the parts was, the folder set aside; bringing it back lets the next write
//! succeed, so that what is left of an error is only what the screen kept.
//! An error is a text naming the part's file inside a sentence — every file
//! error says the path — and its tone is read off the frame: the fill of the
//! pill around it, the colour of its dot, the colour its text is painted in.
//!
//! A row of the screen is read off the accessibility tree: every name and every
//! text whose middle stands within the height of one widget known to be on it,
//! left to right, a text once rather than once more for the run of glyphs
//! `egui` lays under it. A separator carries no name, and is read off the frame's
//! shapes instead, as are a pill — a filled or outlined rectangle around the
//! text it carries, its corners rounded, and the message's rounded into round
//! ends — and the message's dot, a filled circle in `accent.amber`.

mod driver;

use cao_app::theme::tokens;
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
    let pills: Vec<egui::Rect> = backgrounds_painted(&app)
        .into_iter()
        .filter(|(_, least_radius, _)| *least_radius > 0.0)
        .map(|(pill, ..)| pill)
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

#[test]
fn a_tool_s_hint_shows_centred_in_the_title_bar_in_a_pill_with_an_amber_dot() {
    let mut app = a_part("a_tool_s_hint_shows_centred_in_the_title_bar");
    driver::start_a_sketch(&mut app);
    driver::click_exactly(&mut app, "Arc");

    let mut wrong = Vec::new();
    if !driver::says(&app, THE_ARC_S_HINT) {
        wrong.push(format!(
            "`driver::says` no longer finds the hint, and every test reading a message \
             with it goes blind; the screen says {:?}",
            driver::said(&app),
        ));
    }
    let hint = the_topmost_reading(&app, THE_ARC_S_HINT);
    let bar = the_topmost_reading(&app, "Accueil");
    if !bar.y_range().contains(hint.center().y) {
        wrong.push(format!(
            "the hint at {hint:?} is not on the title bar's row, « Accueil » at {bar:?}"
        ));
    }
    let view = the_topmost_reading(&app, "Esquisse 1");
    let pill = backgrounds_painted(&app)
        .into_iter()
        .filter(|(around, least_radius, _)| {
            *least_radius >= around.height() / 2.0 - ROUND_WITHIN
                && around.expand(HUGGED_WITHIN).contains_rect(hint)
                && !around.intersects(view)
        })
        .map(|(around, ..)| around)
        .min_by(|one, other| one.area().total_cmp(&other.area()));
    if pill.is_none() {
        wrong.push(format!(
            "nothing with round ends is painted around the hint at {hint:?} alone, the \
             view's « Esquisse 1 » at {view:?} left outside it"
        ));
    }
    let message = pill.unwrap_or(hint);
    let middle = app.ctx.content_rect().center().x;
    if (message.center().x - middle).abs() > CENTRED_WITHIN {
        wrong.push(format!(
            "the message stands at {message:?}, its middle at {} where the window's is at \
             {middle}",
            message.center().x,
        ));
    }
    let amber = tokens::ACCENT_AMBER;
    let dots = dots_painted(&app, amber);
    let beside_the_hint = dots.iter().any(|(at, wide)| {
        (wide - DOT).abs() < SIZED_WITHIN
            && message.contains(*at)
            && at.x < hint.left()
            && hint.y_range().contains(at.y)
    });
    if !beside_the_hint {
        wrong.push(format!(
            "no amber dot {DOT} points wide stands in the message before the hint at \
             {hint:?}; amber dots painted: {dots:?}"
        ));
    }

    assert!(
        wrong.is_empty(),
        "the tool's hint is not shown as the redesign draws it:\n  {}",
        wrong.join("\n  "),
    );
}

#[test]
fn a_save_that_fails_shows_its_error_in_red_in_the_pill_and_not_at_the_bottom() {
    let test = "a_save_that_fails_shows_its_error_in_red_in_the_pill";
    let mut app = a_part(test);
    driver::start_a_sketch(&mut app);

    take_the_folder_away(test);
    driver::draw_a_segment(&mut app);

    let mut wrong = Vec::new();
    let errors = errors_said(&app, test);
    let bar = the_topmost_reading(&app, "Accueil");
    let off_the_bar: Vec<&egui::Rect> = errors
        .iter()
        .filter(|at| !bar.y_range().contains(at.center().y))
        .collect();
    if !off_the_bar.is_empty() {
        wrong.push(format!(
            "the error is said at {off_the_bar:?}, off the title bar's row at {bar:?}, in a \
             window {:?}",
            app.ctx.content_rect(),
        ));
    }
    match errors
        .iter()
        .find(|at| bar.y_range().contains(at.center().y))
    {
        None => wrong.push(format!(
            "no error naming the part's file is said on the title bar; the screen says {:?}",
            driver::said(&app),
        )),
        Some(error) => wrong.extend(not_in_the_error_tone(&app, *error)),
    }
    let ink = colours_the_error_is_written_in(&app, test);
    if ink.iter().any(|colour| *colour != tokens::STATE_ERROR_TEXT) || ink.is_empty() {
        wrong.push(format!(
            "the error is written in {ink:?}, where `state.error_text` is {:?}",
            tokens::STATE_ERROR_TEXT,
        ));
    }

    assert!(
        wrong.is_empty(),
        "a save that failed is not said in the title bar's pill, in red:\n  {}",
        wrong.join("\n  "),
    );
}

#[test]
fn an_error_takes_the_pill_from_the_message_before_it() {
    let test = "an_error_takes_the_pill_from_the_message_before_it";
    let mut app = a_part(test);
    driver::start_a_sketch(&mut app);
    driver::click_exactly(&mut app, "Arc");
    assert!(
        driver::says(&app, THE_ARC_S_HINT),
        "the arc's hint is not said once the tool is taken, which leaves nothing for \
         the error to take the place of: {:?}",
        driver::said(&app),
    );

    take_the_folder_away(test);
    lay_an_arc(&mut app);

    let bar = the_topmost_reading(&app, "Accueil");
    let errors = errors_said(&app, test);
    assert!(
        errors
            .iter()
            .any(|at| bar.y_range().contains(at.center().y))
            && !driver::says(&app, THE_ARC_S_HINT),
        "an arc laid while the part could not be written: the error is said at {errors:?} \
         (the title bar's row is at {bar:?}), and the screen says {:?}",
        driver::said(&app),
    );
}

#[test]
fn an_error_gives_way_to_the_next_command() {
    let test = "an_error_gives_way_to_the_next_command";
    let mut app = a_part_whose_save_failed(test);

    driver::click(&mut app, "Ligne (L)");

    assert!(
        errors_said(&app, test).is_empty(),
        "the error is still said once the line tool is taken: {:?}",
        driver::said(&app),
    );
}

#[test]
fn an_error_gives_way_to_the_next_message() {
    let test = "an_error_gives_way_to_the_next_message";
    let mut app = a_part_whose_save_failed(test);

    driver::click_exactly(&mut app, "Arc");

    let mut wrong = Vec::new();
    if !errors_said(&app, test).is_empty() {
        wrong.push("the error is still said".to_owned());
    }
    if !driver::says(&app, THE_ARC_S_HINT) {
        wrong.push("the arc's hint is not said".to_owned());
    }
    let red_dots = dots_painted(&app, tokens::STATE_ERROR);
    if !red_dots.is_empty() {
        wrong.push(format!("an error's dot is still painted, at {red_dots:?}"));
    }
    assert!(
        wrong.is_empty(),
        "once the arc tool is taken after a failed save:\n  {}\nThe screen says {:?}",
        wrong.join("\n  "),
        driver::said(&app),
    );
}

#[test]
fn a_preferences_error_stays_while_a_plane_is_being_chosen() {
    let test = "a_preferences_error_stays_while_a_plane_is_being_chosen";
    let mut app = a_part(test);
    driver::click(&mut app, "Nouvelle esquisse");
    hover_at_world_point(&mut app, DVec3::ZERO);
    assert!(
        driver::says(&app, CHOOSING_A_PLANE),
        "« Nouvelle esquisse » does not ask for a plane: {:?}",
        driver::said(&app),
    );

    let settings = the_settings_folder(test);
    std::fs::rename(&settings, settings.with_extension("away")).expect("the folder is set aside");
    std::fs::write(&settings, b"in the way").expect("a file where the folder was");
    driver::press_with_command(&mut app, egui::Key::Comma);
    driver::click_exactly(&mut app, "Raccourcis");
    let unbind = app
        .query_all_by_label("Retirer")
        .next()
        .expect("a shortcut to take away");
    unbind.scroll_to_me();
    unbind.click_accesskit();
    app.run();
    driver::click_exactly(&mut app, "Close window");
    let raised = settings_errors_said(&app, test);
    for point in [driver::ON_THE_YZ_PLANE, driver::ON_THE_YZ_PLANE / 2.0] {
        hover_at_world_point(&mut app, point);
    }

    assert!(
        !raised.is_empty() && !settings_errors_said(&app, test).is_empty(),
        "the preferences could not be written: said {raised:?} once their window was \
         closed, then the mouse moved over the view while a plane is chosen, and the \
         screen says {:?}",
        driver::said(&app),
    );
}

#[test]
fn a_save_error_stays_in_the_pill_until_the_next_gesture() {
    let test = "a_save_error_stays_in_the_pill_until_the_next_gesture";
    let mut app = a_part_whose_save_failed(test);

    app.run_steps(8);
    driver::hold_the_canvas(&mut app);

    assert!(
        !errors_said(&app, test).is_empty(),
        "two seconds with no gesture, the part written again behind it, and the error \
         is no longer said: {:?}",
        driver::said(&app),
    );
}

#[test]
fn a_part_that_cannot_be_created_says_so_on_the_home_screen() {
    let test = "a_part_that_cannot_be_created_says_so_on_the_home_screen";
    let mut app = driver::open(test);
    std::fs::write(the_folder(test), b"in the way").expect("a file where the folder goes");

    driver::create_a_part(&mut app, "Platine");

    let window = app.ctx.content_rect();
    let at_the_bottom: Vec<egui::Rect> = app
        .query_all_by_role(Role::Label)
        .filter(|node| {
            node.accesskit_node()
                .value()
                .is_some_and(|said| said.contains(&the_folder(test).display().to_string()))
        })
        .map(|node| node.rect())
        .filter(|at| at.bottom() >= window.bottom() - AT_THE_BOTTOM)
        .collect();
    assert!(
        driver::shows(&app, "Créer") && !at_the_bottom.is_empty(),
        "a part that could not be made is not said at the bottom of the home screen, \
         in a window {window:?}; the screen says {:?}",
        driver::said(&app),
    );
}

#[test]
fn a_save_that_fails_on_leaving_the_part_is_seen_on_the_home_screen() {
    let test = "a_save_that_fails_on_leaving_the_part_is_seen_on_the_home_screen";
    let mut app = a_part(test);
    driver::start_a_sketch(&mut app);
    driver::draw_a_segment(&mut app);

    take_the_folder_away(test);
    driver::click(&mut app, "Accueil");

    let window = app.ctx.content_rect();
    let errors = errors_said(&app, test);
    assert!(
        driver::shows(&app, "Créer")
            && errors
                .iter()
                .any(|at| at.bottom() >= window.bottom() - AT_THE_BOTTOM),
        "the part could not be put away, and the home screen does not say so at its \
         bottom: the error is said at {errors:?} in a window {window:?}; the screen \
         says {:?}",
        driver::said(&app),
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

/// The tool's hint the message pill is read with: the arc's, which the tool
/// says the moment it is taken.
const THE_ARC_S_HINT: &str = "Cliquez le centre, puis le départ de la courbe, puis son arrivée";

/// How wide the message's dot is drawn, in points: `docs/redesign/part-screen.md`
/// P1.
const DOT: f32 = 7.0;

/// How far a size may be from the one drawn, in points.
const SIZED_WITHIN: f32 = 0.01;

/// How far a pill's middle may stand from the window's, in points.
const CENTRED_WITHIN: f32 = 1.0;

/// How far a corner may fall short of half the side it rounds and still read as
/// a round end, in points: a radius is held in whole points.
const ROUND_WITHIN: f32 = 0.5;

/// How far from the bottom of the window the red bar's text may stand, in
/// points: a row of text and the bar's margin.
const AT_THE_BOTTOM: f32 = 40.0;

/// The folder the driver's application keeps its parts in.
fn the_folder(test: &str) -> std::path::PathBuf {
    std::env::temp_dir()
        .join("cao-driver")
        .join(test)
        .join("documents")
        .join("CAO")
}

/// The pointer left over a point of the world, as a hand resting there.
fn hover_at_world_point(app: &mut driver::App, point: DVec3) {
    let at = driver::world_point_on_screen(app, point);
    app.hover_at(at);
    app.run();
}

/// What the view says while a plane to draw on is being chosen.
const CHOOSING_A_PLANE: &str = "Choisissez un plan d'esquisse";

/// The folder the driver's application keeps its settings in.
fn the_settings_folder(test: &str) -> std::path::PathBuf {
    std::env::temp_dir()
        .join("cao-driver")
        .join(test)
        .join("config")
}

/// Where each text that says the settings could not be written stands: a
/// sentence around the path of their folder.
fn settings_errors_said(app: &driver::App, test: &str) -> Vec<egui::Rect> {
    let folder = the_settings_folder(test).display().to_string();
    app.query_all_by_role(Role::Label)
        .filter(|node| {
            node.accesskit_node()
                .value()
                .is_some_and(|said| said.contains(&folder))
        })
        .map(|node| node.rect())
        .collect()
}

/// Where the part `a_part` makes is written.
fn the_part_s_file(test: &str) -> String {
    the_folder(test)
        .join("Platine.caopart")
        .display()
        .to_string()
}

/// Puts a file where the folder of the parts was, so that writing the part
/// fails; the folder itself is set aside, and `bring_the_folder_back` undoes
/// both.
fn take_the_folder_away(test: &str) {
    let folder = the_folder(test);
    std::fs::rename(&folder, folder.with_extension("away")).expect("the folder is set aside");
    std::fs::write(&folder, b"in the way").expect("a file where the folder was");
}

fn bring_the_folder_back(test: &str) {
    let folder = the_folder(test);
    std::fs::remove_file(&folder).expect("the file in the way is taken out");
    std::fs::rename(folder.with_extension("away"), &folder).expect("the folder comes back");
}

/// A part whose save failed when a line was drawn, the error said, the folder
/// then brought back so that the next write succeeds: what is left of the
/// error is then only what the screen kept of it.
fn a_part_whose_save_failed(test: &str) -> driver::App {
    let mut app = a_part(test);
    driver::start_a_sketch(&mut app);
    take_the_folder_away(test);
    driver::draw_a_segment(&mut app);
    assert!(
        !errors_said(&app, test).is_empty(),
        "a line drawn while the part could not be written said no error: {:?}",
        driver::said(&app),
    );
    bring_the_folder_back(test);
    app.run();
    driver::hold_the_canvas(&mut app);
    app
}

/// Where each text that says the part's file went wrong stands: a sentence
/// around its path, never the bare path the part's name shows on hover.
fn errors_said(app: &driver::App, test: &str) -> Vec<egui::Rect> {
    let file = the_part_s_file(test);
    app.query_all_by_role(Role::Label)
        .filter(|node| {
            node.accesskit_node()
                .value()
                .is_some_and(|said| said.contains(&file) && said != file)
        })
        .map(|node| node.rect())
        .collect()
}

/// Every colour the text naming the part's file is painted in.
fn colours_the_error_is_written_in(app: &driver::App, test: &str) -> Vec<egui::Color32> {
    let file = the_part_s_file(test);
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
            egui::Shape::Text(text) if text.galley.text().contains(&file) => {
                for section in &text.galley.job.sections {
                    found.push(match text.override_text_color {
                        Some(colour) => colour,
                        None if section.format.color == egui::Color32::PLACEHOLDER => {
                            text.fallback_color
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

/// What keeps the error at `error` from standing in a pill in the error's
/// tone: round ends filled with `state.error_background`, and a dot of
/// `state.error` before the text.
fn not_in_the_error_tone(app: &driver::App, error: egui::Rect) -> Vec<String> {
    let mut wrong = Vec::new();
    let pill = backgrounds_painted(app)
        .into_iter()
        .filter(|(around, least_radius, _)| {
            *least_radius >= around.height() / 2.0 - ROUND_WITHIN
                && around.expand(HUGGED_WITHIN).contains_rect(error)
        })
        .min_by(|one, other| one.0.area().total_cmp(&other.0.area()));
    let Some((pill, _, fill)) = pill else {
        return vec![format!(
            "nothing with round ends is painted around the error at {error:?}"
        )];
    };
    if fill != tokens::STATE_ERROR_BACKGROUND {
        wrong.push(format!(
            "the error's pill is filled with {fill:?}, where `state.error_background` is {:?}",
            tokens::STATE_ERROR_BACKGROUND,
        ));
    }
    let dots = dots_painted(app, tokens::STATE_ERROR);
    let before_the_error = dots.iter().any(|(at, wide)| {
        (wide - DOT).abs() < SIZED_WITHIN
            && pill.contains(*at)
            && at.x < error.left()
            && error.y_range().contains(at.y)
    });
    if !before_the_error {
        wrong.push(format!(
            "no dot of `state.error`, {DOT} points wide, stands in the pill before the \
             error at {error:?}; such dots painted: {dots:?}"
        ));
    }
    wrong
}

/// An arc laid with the tool in hand: its centre, where it starts, where it
/// ends.
fn lay_an_arc(app: &mut driver::App) {
    driver::click_at_sketch_point(app, driver::A_POINT_ABOVE_THE_ORIGIN);
    driver::click_at_sketch_point(app, driver::A_POINT_BELOW_AND_RIGHT);
    driver::click_at_sketch_point(app, driver::A_POINT_FURTHER_ALONG);
}

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
    click_in_the_ribbon(app, driver::FINISH_IN_THE_RIBBON);
    driver::click_the_button(app, "Esquisse");
    assert!(
        !driver::is_enabled(app, driver::FINISH_IN_THE_RIBBON),
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

/// Every dot the last frame painted in this colour: where its middle stands,
/// and how wide it is. A filled circle, or a square rounded into one.
fn dots_painted(app: &driver::App, colour: egui::Color32) -> Vec<(egui::Pos2, f32)> {
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
            egui::Shape::Circle(circle) if circle.fill == colour => {
                found.push((circle.center, 2.0 * circle.radius));
            }
            egui::Shape::Rect(rect)
                if rect.fill == colour
                    && (rect.rect.width() - rect.rect.height()).abs() < SIZED_WITHIN
                    && f32::from(rect.corner_radius.nw)
                        >= rect.rect.width() / 2.0 - ROUND_WITHIN =>
            {
                found.push((rect.rect.center(), rect.rect.width()));
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
