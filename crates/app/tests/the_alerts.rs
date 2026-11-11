//! The part's problems as alerts, driven through the whole application: a
//! count on the right of the title bar, and the list it opens under it
//! (`docs/redesign/part-screen.md` P2, screen
//! `docs/redesign/screens/05-part-alerts-open.png`).
//!
//! Closes #736 — the issue has no Done when of its own; these are the points of
//! its « What it does » and « What must not break », approved as its criteria
//! when the work opened, a long point split into its parts, and the two
//! decisions Tom took then, the last two bullets of « What it does ». The
//! bullets the title bar's presenter holds are transcribed again, checked, in
//! `crates/app/src/screens/title_bar/state/tests.rs`.
//!
//! What it does:
//! - Two levels. Warning, yellow: an unstable case (`unstable_at`); a sketch
//!   whose face is lost and that stays where it was (`is_adrift`). Error, red:
//!   a volume not built (`is_declined`); a lost area — no test: held by
//!   `each_problem_of_the_part_is_an_alert_at_its_level`, in
//!   `crates/app/src/screens/title_bar/state/tests.rs`, which reads the level
//!   of each problem's alert off the presenter. The red a level is drawn in is
//!   held on the count by the bullet after next, and the yellow with the
//!   badges' colours, below
//! - a kept section whose face is lost, which wears « face perdue » in the tree
//!   today, is a warning like a sketch off its face — no test: held by
//!   `each_problem_of_the_part_is_an_alert_at_its_level`, as above. Not one of
//!   the issue's words: read from the first point of « What must not break »,
//!   and from P2, whose count is of every element wearing a badge
//! - anything that stops the computation is an error — no test: nothing stops
//!   the computation today. A step the kernel declines leaves the part as it
//!   stood before it and the rest is computed, which is the volume not built
//!   above; a new kind of problem is out of scope
//! - « ⚠ N alertes » on the right of the title bar, only when N > 0, in the
//!   colour of the worst level —
//!   `the_count_stands_on_the_right_of_the_title_bar_in_the_colour_of_the_worst_alert`;
//!   the count, its singular « 1 alerte » and the level it takes are held case
//!   by case beside the title bar's presenter, by
//!   `the_title_bar_counts_the_alerts_only_while_there_is_one` and
//!   `the_count_wears_the_level_of_the_worst_alert`
//! - A click opens the list (340 px) under the button —
//!   `a_click_on_the_count_opens_the_list_340_points_wide_under_it`
//! - Each alert: the warning or error icon, « Élément · problème »
//!   (« Extrusion 3 · instable »), one sentence of explanation —
//!   `each_alert_reads_its_icon_its_element_and_problem_and_a_sentence`, which
//!   reads them on screen with « Montrer » and « Modifier » under them; what
//!   each kind of problem is titled and says is held case by case beside the
//!   title bar's presenter, by `each_alert_is_titled_by_its_element_and_its_problem`
//!   and `each_alert_says_why_in_a_sentence`
//! - then « Montrer » (selects the element …) — for a sketch, its row in the
//!   tree stands selected, the sketch left closed; for a kept section, it is
//!   switched on as a click on its row does, and a second « Montrer » leaves it
//!   on — `show_selects_the_sketch_or_the_section_and_leaves_the_list_open`;
//!   for a volume, under the bullet Tom decided, below
//! - « … and lights it in the view », for a sketch and a kept section — no
//!   test: put off, nothing to hold it with yet. Nothing lights a whole sketch
//!   today, the tree's light being of one row under it, and a section whose
//!   face is gone cuts nothing, so switching it on changes nothing in the view.
//!   Lighting a whole element is what « Montrer » on a volume adds; whether a
//!   sketch takes the same light waits on that
//! - and « Modifier » (reopens the step) — for a sketch, what a double-click
//!   on its row does; for a kept section, which is no step, the section
//!   reopened in its tool, as a double-click on its row does too, decided when
//!   the list was taken up —
//!   `modify_reopens_the_step_the_alert_is_about_and_closes_the_list`, and what
//!   the button asks is held beside the title bar's presenter by
//!   `modify_asks_what_a_double_click_on_the_sketch_or_the_section_s_row_asks`;
//!   for an extrusion, under the bullet Tom decided, below
//! - decided when « Montrer » and « Modifier » were taken up, the redesign
//!   saying nothing of it: the list stays open after « Montrer », the next
//!   alert still to be looked at, and closes after « Modifier », the user now in
//!   the sketch or the section —
//!   `show_selects_the_sketch_or_the_section_and_leaves_the_list_open` and
//!   `modify_reopens_the_step_the_alert_is_about_and_closes_the_list`
//! - The list follows the part live — `the_open_list_follows_the_part_live`
//! - ✕, Escape or a click elsewhere closes it —
//!   `the_cross_escape_or_a_click_elsewhere_closes_the_list`
//! - What describes the part leaves the message pill: an extrusion not built
//!   or unstable is an alert —
//!   `an_extrusion_not_built_or_unstable_is_an_alert_and_no_message`
//! - and stays while it is true —
//!   `an_alert_stays_while_its_problem_holds_and_goes_with_it`
//! - The pill keeps what answers a gesture —
//!   `an_extrusion_that_changes_nothing_still_says_so_in_the_pill`
//! - The same two colours for the tree's badges —
//!   `a_badge_and_an_alert_of_one_level_wear_the_same_colours`
//! - and, later, the reader's boxes — no test: out of scope here, as the issue
//!   says; the step reader (P7) does not exist yet, and takes the two colours
//!   when it is built
//! - Preferences › Apparence can set and reset them —
//!   `the_alert_colours_are_set_and_reset_in_preferences_appearance`
//! - decided by Tom when the work opened: « Modifier » on an extrusion's alert
//!   reopens the extrusion, its panel filled with its values, where no
//!   extrusion could be reopened before —
//!   `modify_on_an_extrusion_s_alert_reopens_it_with_its_values`
//! - decided by Tom when the work opened: « Montrer » on a volume's alert
//!   selects the volume and lights it in the view —
//!   `show_on_a_volume_s_alert_selects_the_volume_and_lights_it`
//!
//! What must not break:
//! - Every problem detected today is still shown: as an alert now — no test:
//!   held by `each_problem_of_the_part_is_an_alert_at_its_level`, in
//!   `crates/app/src/screens/title_bar/state/tests.rs`, one part for each
//!   problem the tree wears a badge for today; on screen, the count of
//!   `the_count_stands_on_the_right_of_the_title_bar_in_the_colour_of_the_worst_alert`
//!   takes an area lost and a face lost together
//! - The refusal at the moment of the gesture (« Cette extrusion ne change
//!   rien ») still shows in the pill —
//!   `an_extrusion_that_changes_nothing_still_says_so_in_the_pill`
//!
//! The « ⚠ » of the count is the `warning` icon, the triangle the mockup's
//! button wears (`docs/redesign/source/part.html`), and the count is a button
//! named by its words alone, « 2 alertes », as the mockup names it. The colour
//! of a level is the pair the tree's badges are drawn in: `state.error_text` on
//! `state.error_background`, `state.warning_text` on
//! `state.warning_background`.
//!
//! A part with an error and a warning is made as `the_part_panel.rs` makes a
//! sketch lose its face: a block, a sketch laid on its face, then the block's
//! own sketch emptied. The block's extrusion is left with no area, and the
//! sketch on its face with no face. For the list to follow, a section takes
//! the sketch's place on the face, since a section can be deleted from the
//! tree and an undo brings it back while the list stays open: the keyboard
//! changes the part without the click elsewhere that would close it.
//!
//! The list is read as the mockup draws it, in `docs/redesign/source/part.html`:
//! headed « Alertes de la pièce », closed by a button named « Fermer les
//! alertes » on the heading's row, its right edge on the count's and its top
//! just under it. Its 340 are read as the width of the frame painted, whose
//! fill is found as the least one around the heading and that button; the
//! mockup's CSS adds its padding to them, which an `egui` frame would not. An
//! alert is read inside that frame: its title, the `warning` or `error` icon
//! on its left, at least one text under the title, then one « Montrer » and one
//! « Modifier » before the next alert.

mod driver;

use cao_app::theme::tokens;
use egui::accesskit::Role;
use egui_kittest::kittest::{NodeT, Queryable};
use glam::{DVec2, DVec3};

#[test]
fn the_count_stands_on_the_right_of_the_title_bar_in_the_colour_of_the_worst_alert() {
    let mut app = driver::a_block("the_count_stands_on_the_right_of_the_title_bar");
    lay_a_sketch_on_the_block_s_face(&mut app);
    let before = the_title_bar(&app);

    empty_the_block_s_sketch(&mut app);
    assert!(
        driver::says(&app, "aire perdue") && driver::says(&app, "face perdue"),
        "the block's sketch emptied, the tree does not wear « aire perdue » and « face \
         perdue », which leaves the count nothing to count: {:?}",
        driver::said(&app),
    );

    let mut wrong = Vec::new();
    let counted_before: Vec<&String> = before
        .iter()
        .filter(|read| read.contains("alerte"))
        .collect();
    if !counted_before.is_empty() {
        wrong.push(format!(
            "with nothing wrong with the part, the title bar already reads {counted_before:?}"
        ));
    }
    match the_button_reading(&app, "2 alertes") {
        None => wrong.push(format!(
            "no button reads « 2 alertes »; the title bar reads {:?}",
            the_title_bar(&app),
        )),
        Some(count) => wrong.extend(not_on_the_right_in_red(&app, count)),
    }

    assert!(
        wrong.is_empty(),
        "an area lost and a face lost are not counted as the redesign draws it:\n  {}",
        wrong.join("\n  "),
    );
}

#[test]
fn a_click_on_the_count_opens_the_list_340_points_wide_under_it() {
    let mut app = a_part_with_two_alerts("a_click_on_the_count_opens_the_list");
    let open_before = the_list(&app);

    driver::click_the_button(&mut app, "2 alertes");

    let mut wrong = Vec::new();
    if let Some(list) = open_before {
        wrong.push(format!(
            "the list stood open at {list:?} before the count was clicked"
        ));
    }
    match the_list(&app) {
        None => wrong.push(format!(
            "no list headed « {HEADING} » with a « {CLOSE} » button is open; the screen \
             shows {:?}",
            driver::on_screen(&app),
        )),
        Some(list) => wrong.extend(not_340_wide_under_the_count(&app, list)),
    }

    assert!(
        wrong.is_empty(),
        "a click on « 2 alertes » does not open the list as the redesign draws it:\n  {}",
        wrong.join("\n  "),
    );
}

#[test]
fn each_alert_reads_its_icon_its_element_and_problem_and_a_sentence() {
    let mut app = a_part_with_two_alerts("each_alert_reads_its_icon_its_element_and_problem");
    driver::click_the_button(&mut app, "2 alertes");
    let list = the_list(&app).unwrap_or_else(|| {
        panic!(
            "a click on « 2 alertes » opened no list, which leaves no alert to read: {:?}",
            driver::on_screen(&app),
        )
    });

    let read = texts_within(&app, list);
    let mut wrong = Vec::new();
    let mut titles = Vec::new();
    for (title, icon) in [
        ("Esquisse 2 · face perdue", "warning"),
        ("Extrusion 1 · aire perdue", "error"),
    ] {
        match read.iter().find(|(text, _)| text == title) {
            Some((_, at)) => titles.push((title, icon, *at)),
            None => wrong.push(format!("the list reads no « {title} »")),
        }
    }
    for (title, icon, at) in &titles {
        let next = titles
            .iter()
            .map(|(.., other)| other.top())
            .filter(|top| *top > at.top())
            .fold(list.bottom(), f32::min);
        wrong.extend(not_read_as_an_alert(&app, list, (title, *at, next), icon));
    }

    assert!(
        wrong.is_empty(),
        "the list does not read each alert as the redesign draws it:\n  {}\nIt reads {read:?}",
        wrong.join("\n  "),
    );
}

#[test]
fn the_open_list_follows_the_part_live() {
    let mut app = driver::a_block("the_open_list_follows_the_part_live");
    lay_a_section_on_the_block_s_face(&mut app);
    empty_the_block_s_sketch(&mut app);
    driver::right_click(&mut app, "Coupe 1");
    driver::click_the_button(&mut app, "Supprimer");
    assert!(
        !driver::in_the_tree(&app, "Coupe 1") && the_button_reading(&app, "1 alerte").is_some(),
        "a section laid on the block's face, the block's sketch emptied, then the section \
         deleted: the part is not left with the one alert of the block's extrusion; the \
         title bar reads {:?}, the screen shows {:?}",
        the_title_bar(&app),
        driver::on_screen(&app),
    );
    driver::click_the_button(&mut app, "1 alerte");
    let before = alerts_listed(&app);

    driver::press_with_command(&mut app, egui::Key::Z);

    let after = alerts_listed(&app);
    let mut wrong = Vec::new();
    if !driver::in_the_tree(&app, "Coupe 1") || the_button_reading(&app, "2 alertes").is_none() {
        wrong.push(format!(
            "the undo did not bring the section back with its alert, which leaves the list \
             nothing to follow: the title bar reads {:?}",
            the_title_bar(&app),
        ));
    }
    match &before {
        None => wrong.push("a click on « 1 alerte » opened no list".to_owned()),
        Some(read)
            if !read.contains(&AREA_LOST.to_owned()) || read.contains(&FACE_LOST.to_owned()) =>
        {
            wrong.push(format!(
                "with the section deleted, the list reads {read:?}, where it should read « \
                 {AREA_LOST} » alone"
            ))
        }
        Some(_) => {}
    }
    match &after {
        None => wrong.push("the undo closed the list, or took it away".to_owned()),
        Some(read)
            if !read.contains(&AREA_LOST.to_owned()) || !read.contains(&FACE_LOST.to_owned()) =>
        {
            wrong.push(format!(
                "the section brought back by the undo, the open list reads {read:?}, without \
                 « {FACE_LOST} » beside « {AREA_LOST} »"
            ))
        }
        Some(_) => {}
    }

    assert!(
        wrong.is_empty(),
        "the open list does not follow the part:\n  {}",
        wrong.join("\n  "),
    );
}

#[test]
fn the_cross_escape_or_a_click_elsewhere_closes_the_list() {
    let mut app = a_part_with_two_alerts("the_cross_escape_or_a_click_elsewhere_closes_the_list");
    let ways: [(&str, Gesture); 3] = [
        ("« Fermer les alertes »", |app| {
            driver::click_the_button(app, CLOSE);
        }),
        ("Escape", |app| driver::press(app, egui::Key::Escape)),
        ("a click in the view", |app| {
            driver::click_at_world_point(app, IN_THE_VIEW);
        }),
    ];

    let mut wrong = Vec::new();
    for (way, close) in ways {
        driver::click_the_button(&mut app, "2 alertes");
        let Some(list) = the_list(&app) else {
            wrong.push(format!(
                "a click on « 2 alertes » opened no list for {way} to close"
            ));
            continue;
        };
        if list.contains(driver::world_point_on_screen(&app, IN_THE_VIEW)) {
            wrong.push(format!(
                "the click in the view would land in the list, at {list:?}"
            ));
        }
        close(&mut app);
        if driver::reads(&app, HEADING) {
            wrong.push(format!("{way} left the list open"));
        }
        if the_button_reading(&app, "2 alertes").is_none() {
            wrong.push(format!(
                "{way} took the count away, the part's problems still there"
            ));
        }
    }

    assert!(
        wrong.is_empty(),
        "the list does not close as the issue asks:\n  {}",
        wrong.join("\n  "),
    );
}

#[test]
fn modify_reopens_the_step_the_alert_is_about_and_closes_the_list() {
    let mut wrong = Vec::new();

    let mut app = a_part_with_two_alerts("modify_reopens_the_sketch");
    let open_before = driver::a_sketch_is_open(&app);
    driver::click_the_button(&mut app, "2 alertes");
    click_the_alert_s(&mut app, SKETCH_OFF_ITS_FACE, "Modifier");
    let bar = the_title_bar(&app);
    if open_before
        || !driver::a_sketch_is_open(&app)
        || !bar.iter().any(|read| read == "Esquisse 2")
    {
        wrong.push(format!(
            "« Modifier » on « {SKETCH_OFF_ITS_FACE} » did not reopen « Esquisse 2 », as a \
             double-click on its row does: a sketch open before {open_before}, after {}, and \
             the title bar reads {bar:?}",
            driver::a_sketch_is_open(&app),
        ));
    }
    if driver::reads(&app, HEADING) {
        wrong.push("the list stands open over the sketch « Modifier » reopened".to_owned());
    }

    let mut app = a_part_with_a_section_off_its_face("modify_reopens_the_section");
    let tool_before = the_section_tool_is_open(&app);
    driver::click_the_button(&mut app, "2 alertes");
    click_the_alert_s(&mut app, FACE_LOST, "Modifier");
    let bar = the_title_bar(&app);
    if tool_before
        || !the_section_tool_is_open(&app)
        || !bar.iter().any(|read| read == "Vue en coupe")
    {
        wrong.push(format!(
            "« Modifier » on « {FACE_LOST} » did not reopen the section in its tool, as a \
             double-click on its row does: the tool open before {tool_before}, after {}, and \
             the title bar reads {bar:?}",
            the_section_tool_is_open(&app),
        ));
    }
    if driver::reads(&app, HEADING) {
        wrong.push("the list stands open over the section « Modifier » reopened".to_owned());
    }

    assert!(
        wrong.is_empty(),
        "« Modifier » does not reopen what its alert is about:\n  {}",
        wrong.join("\n  "),
    );
}

#[test]
fn show_selects_the_sketch_or_the_section_and_leaves_the_list_open() {
    let mut wrong = Vec::new();

    let mut app = a_part_with_two_alerts("show_selects_the_sketch");
    let before = the_row_is_selected(&app, "Esquisse 2");
    driver::click_the_button(&mut app, "2 alertes");
    click_the_alert_s(&mut app, SKETCH_OFF_ITS_FACE, "Montrer");
    let after = the_row_is_selected(&app, "Esquisse 2");
    if before || !after {
        wrong.push(format!(
            "« Montrer » on « {SKETCH_OFF_ITS_FACE} » did not select the row of « Esquisse 2 » \
             in the tree: selected before {before}, after {after}"
        ));
    }
    if driver::a_sketch_is_open(&app) {
        wrong.push(format!(
            "« Montrer » on « {SKETCH_OFF_ITS_FACE} » opened the sketch, which is « Modifier »'s \
             to do"
        ));
    }
    if !driver::reads(&app, HEADING) {
        wrong.push(format!(
            "« Montrer » on « {SKETCH_OFF_ITS_FACE} » closed the list"
        ));
    }

    let mut app = a_part_with_a_section_off_its_face("show_selects_the_section");
    let before = the_row_is_selected(&app, "Coupe 1");
    driver::click_the_button(&mut app, "2 alertes");
    click_the_alert_s(&mut app, FACE_LOST, "Montrer");
    let once = the_row_is_selected(&app, "Coupe 1");
    let still_open = driver::reads(&app, HEADING);
    if still_open {
        click_the_alert_s(&mut app, FACE_LOST, "Montrer");
    }
    let twice = the_row_is_selected(&app, "Coupe 1");
    if before || !once || !twice {
        wrong.push(format!(
            "« Montrer » on « {FACE_LOST} » did not switch « Coupe 1 » on in the tree, or a second \
             « Montrer » switched it off: on before {before}, after one {once}, after two {twice}"
        ));
    }
    if the_section_tool_is_open(&app) {
        wrong.push(format!(
            "« Montrer » on « {FACE_LOST} » opened the section tool, which is « Modifier »'s to do"
        ));
    }
    if !still_open || !driver::reads(&app, HEADING) {
        wrong.push(format!("« Montrer » on « {FACE_LOST} » closed the list"));
    }

    assert!(
        wrong.is_empty(),
        "« Montrer » does not select what its alert is about, the list left open:\n  {}",
        wrong.join("\n  "),
    );
}

/// Whether the row of the tree that reads exactly this stands selected: the
/// sketch open, the section switched on.
fn the_row_is_selected(app: &driver::App, name: &str) -> bool {
    driver::the_row(app, name)
        .is_some_and(|row| row.accesskit_node().toggled() == Some(egui::accesskit::Toggled::True))
}

/// What the sketch laid on the block's face is alerted for, once the block's
/// sketch is emptied.
const SKETCH_OFF_ITS_FACE: &str = "Esquisse 2 · face perdue";

/// A block whose sketch was emptied once a section was laid on its face: the
/// title bar counts « 2 alertes », the block's extrusion having lost its area
/// and « Coupe 1 » its face.
fn a_part_with_a_section_off_its_face(test: &str) -> driver::App {
    let mut app = driver::a_block(test);
    lay_a_section_on_the_block_s_face(&mut app);
    empty_the_block_s_sketch(&mut app);
    assert!(
        driver::in_the_tree(&app, "Coupe 1") && the_button_reading(&app, "2 alertes").is_some(),
        "the block's sketch emptied under a section on its face, the tree holds no « Coupe 1 » \
         or the title bar does not count « 2 alertes »: {:?}",
        the_title_bar(&app),
    );
    app
}

/// Clicks the button named `action` of the alert titled `title`, in the open
/// list: the first of that name under its title.
fn click_the_alert_s(app: &mut driver::App, title: &str, action: &str) {
    let at = the_alert_s_button(app, title, action).unwrap_or_else(|| {
        panic!(
            "the open list offers no « {action} » under « {title} »; the screen shows {:?}",
            driver::on_screen(app),
        )
    });
    driver::click_at(app, at.center());
}

fn the_alert_s_button(app: &driver::App, title: &str, action: &str) -> Option<egui::Rect> {
    let list = the_list(app)?;
    let (_, at) = texts_within(app, list)
        .into_iter()
        .find(|(text, _)| text == title)?;
    app.query_all_by_label(action)
        .filter(|node| node.accesskit_node().role() == Role::Button)
        .map(|node| node.rect())
        .filter(|button| {
            list.contains(button.center()) && button.top() >= at.bottom() - HUGGED_WITHIN
        })
        .min_by(|one, other| one.top().total_cmp(&other.top()))
}

/// Whether the section tool is open: its « Inverser » is its own.
fn the_section_tool_is_open(app: &driver::App) -> bool {
    driver::a_button_reads(app, "Inverser")
}

/// What a hand does to the application.
type Gesture = fn(&mut driver::App);

/// Where the view is clicked away from the list: the origin, nothing standing
/// there once the block's sketch is emptied.
const IN_THE_VIEW: DVec3 = DVec3::ZERO;

/// What the block's extrusion and the section on its face are alerted for,
/// once the block's sketch is emptied.
const AREA_LOST: &str = "Extrusion 1 · aire perdue";
const FACE_LOST: &str = "Coupe 1 · face perdue";

/// Every name and every text the open list reads, nothing while it is shut.
fn alerts_listed(app: &driver::App) -> Option<Vec<String>> {
    let list = the_list(app)?;
    Some(
        texts_within(app, list)
            .into_iter()
            .map(|(text, _)| text)
            .collect(),
    )
}

/// « Coupe 1 », laid on the block's face with the section tool and kept, the
/// tool then closed.
fn lay_a_section_on_the_block_s_face(app: &mut driver::App) {
    driver::click_in_the_bar(app, "Vue en coupe");
    driver::click(app, "Face cliquée");
    driver::click_at_world_point(app, ON_THE_BLOCK_S_FACE);
    driver::click_the_button(app, "Enregistrer la coupe");
    if driver::a_button_reads(app, "Inverser") {
        driver::click_the_button(app, "Fermer");
    }
}

/// What keeps the alert titled `title`, standing at `at` in the list and
/// running down to `next`, from wearing `icon` on its left, then saying a
/// sentence under its title, then offering « Montrer » and « Modifier ».
fn not_read_as_an_alert(
    app: &driver::App,
    list: egui::Rect,
    (title, at, next): (&str, egui::Rect, f32),
    icon: &str,
) -> Vec<String> {
    let mut wrong = Vec::new();
    let beside = egui::Rect::from_x_y_ranges(
        list.left()..=at.left(),
        (at.top() - ICON_SLACK)..=(at.bottom() + ICON_SLACK),
    );
    let worn = icons_painted_within(app, beside);
    if !worn.iter().any(|id| id == icon) {
        wrong.push(format!(
            "« {title} » wears {worn:?} on its left, not the `{icon}` icon"
        ));
    }
    let under = (at.bottom() - HUGGED_WITHIN)..=next;
    let mut actions_top = next;
    for action in ["Montrer", "Modifier"] {
        let offered: Vec<egui::Rect> = app
            .query_all_by_label(action)
            .filter(|node| node.accesskit_node().role() == Role::Button)
            .map(|node| node.rect())
            .filter(|button| {
                list.x_range().contains(button.center().x) && under.contains(&button.center().y)
            })
            .collect();
        match offered.as_slice() {
            [button] => actions_top = actions_top.min(button.top()),
            _ => wrong.push(format!(
                "« {title} » offers {} « {action} » under it, at {offered:?}, before the next \
                 alert at {next}",
                offered.len(),
            )),
        }
    }
    let said: Vec<String> = texts_within(app, list)
        .into_iter()
        .filter(|(text, place)| {
            text != title
                && place.top() >= at.bottom() - HUGGED_WITHIN
                && place.bottom() <= actions_top + HUGGED_WITHIN
        })
        .map(|(text, _)| text)
        .collect();
    if said.is_empty() {
        wrong.push(format!(
            "« {title} » says no sentence between its title and its actions"
        ));
    }
    wrong
}

/// How far above or below its title an alert's icon may reach, in points: the
/// icon stands higher than the text beside it.
const ICON_SLACK: f32 = 6.0;

/// Every name and every text whose middle stands inside `within`, with where it
/// stands.
fn texts_within(app: &driver::App, within: egui::Rect) -> Vec<(String, egui::Rect)> {
    app.get_all_by(|_| true)
        .filter_map(|node| {
            let read = node.accesskit_node();
            read.bounding_box()?;
            if read.role() == Role::TextRun {
                return None;
            }
            let said = read
                .label()
                .or_else(|| read.value())
                .filter(|said| !said.trim().is_empty())?;
            let at = node.rect();
            within.contains(at.center()).then_some((said, at))
        })
        .collect()
}

/// What keeps the list at `list` from standing 340 points wide just under the
/// count, its right edge on the count's, its heading on its first row with the
/// button that closes it on the right.
fn not_340_wide_under_the_count(app: &driver::App, list: egui::Rect) -> Vec<String> {
    let mut wrong = Vec::new();
    if (list.width() - LIST_WIDTH).abs() > PLACED_WITHIN {
        wrong.push(format!(
            "the list at {list:?} is {} points wide, where the redesign draws it \
             {LIST_WIDTH}",
            list.width(),
        ));
    }
    let Some(count) = the_button_reading(app, "2 alertes") else {
        wrong.push("the count is gone once the list is open".to_owned());
        return wrong;
    };
    let gap = list.top() - count.bottom();
    if !(-HUGGED_WITHIN..=UNDER_WITHIN).contains(&gap) {
        wrong.push(format!(
            "the list's top at {} does not stand just under the count, whose bottom is at {}",
            list.top(),
            count.bottom(),
        ));
    }
    if (list.right() - count.right()).abs() > PLACED_WITHIN {
        wrong.push(format!(
            "the list's right edge at {} is not the count's, at {}",
            list.right(),
            count.right(),
        ));
    }
    let heading = the_topmost_reading(app, HEADING);
    if let Some(close) = the_button_reading(app, CLOSE)
        && (!heading.y_range().contains(close.center().y) || close.left() < heading.right())
    {
        wrong.push(format!(
            "« {CLOSE} » at {close:?} does not stand right of the heading at {heading:?}, on \
             its row"
        ));
    }
    wrong
}

/// What the list of alerts is headed, and what the button that closes it is
/// named: `docs/redesign/source/part.html`.
const HEADING: &str = "Alertes de la pièce";
const CLOSE: &str = "Fermer les alertes";

/// How wide the list stands, in points: P2 of `docs/redesign/part-screen.md`.
const LIST_WIDTH: f32 = 340.0;

/// How far a size or an edge may be from where it is drawn, in points.
const PLACED_WITHIN: f32 = 1.0;

/// How far under the count the list may start, in points.
const UNDER_WITHIN: f32 = 12.0;

/// A block whose sketch was emptied once a sketch was laid on its face: the
/// title bar counts « 2 alertes », the block's extrusion having lost its area
/// and the sketch its face.
fn a_part_with_two_alerts(test: &str) -> driver::App {
    let mut app = driver::a_block(test);
    lay_a_sketch_on_the_block_s_face(&mut app);
    empty_the_block_s_sketch(&mut app);
    assert!(
        the_button_reading(&app, "2 alertes").is_some(),
        "the block's sketch emptied under a sketch on its face, the title bar does not count \
         « 2 alertes », which leaves no list to open: {:?}",
        the_title_bar(&app),
    );
    app
}

/// The frame of the list of alerts while it is open: the least rectangle
/// painted with a fill around its heading and the button that closes it.
fn the_list(app: &driver::App) -> Option<egui::Rect> {
    let heading = driver::where_it_reads(app, HEADING)?;
    let close = the_button_reading(app, CLOSE)?;
    let both = heading.union(close);
    shapes_painted(app)
        .into_iter()
        .filter_map(|shape| match shape {
            egui::Shape::Rect(rect)
                if rect.fill.a() > 0 && rect.rect.expand(HUGGED_WITHIN).contains_rect(both) =>
            {
                Some(rect.rect)
            }
            _ => None,
        })
        .min_by(|one, other| one.area().total_cmp(&other.area()))
}

/// What keeps the count at `count` from standing at the right end of the title
/// bar, wearing the warning triangle, written in `state.error_text` on
/// `state.error_background`.
fn not_on_the_right_in_red(app: &driver::App, count: egui::Rect) -> Vec<String> {
    let mut wrong = Vec::new();
    let home = the_topmost_reading(app, "Accueil");
    if !home.y_range().contains(count.center().y) {
        wrong.push(format!(
            "the count at {count:?} is not on the title bar's row, « Accueil » at {home:?}"
        ));
    }
    let window = app.ctx.content_rect();
    if count.left() < window.center().x || window.right() - count.right() > AT_THE_RIGHT_EDGE {
        wrong.push(format!(
            "the count stands at {count:?}, not at the right end of a window {window:?}"
        ));
    }
    let worn = icons_painted_within(app, count);
    if !worn.iter().any(|icon| icon == "warning") {
        wrong.push(format!(
            "the count at {count:?} wears {worn:?}, not the `warning` triangle"
        ));
    }
    let ink = colours_written_in(app, "2 alertes");
    if ink.is_empty() || ink.iter().any(|colour| *colour != tokens::STATE_ERROR_TEXT) {
        wrong.push(format!(
            "« 2 alertes » is written in {ink:?}, where `state.error_text` is {:?}",
            tokens::STATE_ERROR_TEXT,
        ));
    }
    let filled = fills_around(app, count);
    if !filled.contains(&tokens::STATE_ERROR_BACKGROUND) {
        wrong.push(format!(
            "the count at {count:?} stands on {filled:?}, where `state.error_background` \
             is {:?}",
            tokens::STATE_ERROR_BACKGROUND,
        ));
    }
    wrong
}

/// How far from the window's right edge the count may end, in points: the
/// title bar's margin, and some.
const AT_THE_RIGHT_EDGE: f32 = 32.0;

/// How far a frame may sit inside the widget it is drawn around, in points:
/// the rect the tree gives a widget and the frame painted around it are
/// rounded apart.
const HUGGED_WITHIN: f32 = 0.5;

/// On the face of the block at x = 10, the one `keeping_a_section.rs` clicks
/// to lay a section there.
const ON_THE_BLOCK_S_FACE: DVec3 = DVec3::new(10.0, 41.51, -19.82);

/// A point of the block's sketch away from what is drawn on it, where the hand
/// rests while the selection is deleted.
const AWAY_FROM_THE_BLOCK_S_TRAITS: DVec2 = DVec2::new(173.9, -56.77);

/// « Esquisse 2 », laid on the block's face, and finished.
fn lay_a_sketch_on_the_block_s_face(app: &mut driver::App) {
    driver::click(app, "Nouvelle esquisse");
    driver::click_at_world_point(app, ON_THE_BLOCK_S_FACE);
    driver::finish_the_sketch(app);
}

/// Everything drawn on « Esquisse 1 », the block's sketch, deleted, and the
/// sketch finished: the block's extrusion loses its area, and the sketch on
/// its face loses the face.
fn empty_the_block_s_sketch(app: &mut driver::App) {
    driver::double_click(app, "Esquisse 1");
    driver::click(app, "Sélection (S)");
    driver::press_with_command(app, egui::Key::A);
    app.hover_at(driver::sketch_point_on_screen(
        app,
        AWAY_FROM_THE_BLOCK_S_TRAITS,
    ));
    app.run();
    driver::press(app, egui::Key::Delete);
    driver::finish_the_sketch(app);
}

/// Where the button named exactly this stands.
fn the_button_reading(app: &driver::App, name: &str) -> Option<egui::Rect> {
    app.query_all_by_label(name)
        .find(|node| node.accesskit_node().role() == Role::Button)
        .map(|node| node.rect())
}

/// What the title bar reads, left to right: the row of « Accueil ».
fn the_title_bar(app: &driver::App) -> Vec<String> {
    the_row_of(app, the_topmost_reading(app, "Accueil"))
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

/// Every shape the last frame painted, groups opened.
fn shapes_painted(app: &driver::App) -> Vec<egui::Shape> {
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
            shape => found.push(shape.clone()),
        }
    }
    found
}

/// Every colour a text reading exactly this is painted in.
fn colours_written_in(app: &driver::App, text: &str) -> Vec<egui::Color32> {
    let mut found = Vec::new();
    for shape in shapes_painted(app) {
        let egui::Shape::Text(written) = shape else {
            continue;
        };
        if written.galley.text() != text {
            continue;
        }
        for section in &written.galley.job.sections {
            found.push(match written.override_text_color {
                Some(colour) => colour,
                None if section.format.color == egui::Color32::PLACEHOLDER => {
                    written.fallback_color
                }
                None => section.format.color,
            });
        }
    }
    found
}

/// The fill of every rectangle the last frame painted around this one.
fn fills_around(app: &driver::App, within: egui::Rect) -> Vec<egui::Color32> {
    shapes_painted(app)
        .into_iter()
        .filter_map(|shape| match shape {
            egui::Shape::Rect(rect)
                if rect.fill.a() > 0 && rect.rect.expand(HUGGED_WITHIN).contains_rect(within) =>
            {
                Some(rect.fill)
            }
            _ => None,
        })
        .collect()
}

/// The ids of the icons the last frame painted inside a rectangle of the
/// screen, by the file each was loaded from — `bytes://icon/warning.svg` is
/// `warning`.
fn icons_painted_within(app: &driver::App, within: egui::Rect) -> Vec<String> {
    let within = within.expand(1.0);
    let textures = app.ctx.tex_manager();
    let textures = textures.read();
    shapes_painted(app)
        .into_iter()
        .filter_map(|shape| match shape {
            egui::Shape::Rect(rect)
                if rect.fill_texture_id() != egui::TextureId::default()
                    && within.contains_rect(rect.rect) =>
            {
                let loaded_as = textures
                    .meta(rect.fill_texture_id())
                    .map(|meta| meta.name.clone())
                    .unwrap_or_default();
                let file = loaded_as.rsplit('/').next().unwrap_or(&loaded_as);
                Some(file.strip_suffix(".svg").unwrap_or(file).to_owned())
            }
            _ => None,
        })
        .collect()
}
