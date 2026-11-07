//! A point of the drawing clicked where the application drew it. The test
//! names the point in the drawing's own terms — a point of the world, or of the
//! open sketch's plane — and the driver turns it into a place on screen through
//! the view's own camera and canvas, as the last frame drew them. That is what
//! lets a driver test aim at the drawing wherever the chrome puts the view.
//!
//! Closes #752.
//! - a test whose subject is a gesture on the canvas names its point in the
//!   drawing's coordinates, a point of the sketch plane or a world point — no
//!   test: it is the conversion of the driver tests themselves, which aim
//!   through `driver::sketch_point_on_screen` and
//!   `driver::world_point_on_screen`; held by every one of them still passing
//!   and asserting what it did, and kept so by the rule of `screen_points.rs`
//! - the driver turns that point into a screen position through the view's
//!   own camera and canvas, as the last frame drew them —
//!   `a_line_clicked_at_two_points_of_the_sketch_runs_between_those_points`,
//!   `the_same_two_clicks_land_on_the_same_points_whatever_the_size_of_the_window`,
//!   `each_base_plane_clicked_where_the_view_shows_it_opens_a_sketch_on_that_plane`
//! - a reach in pixels, such as "22 px from a side", is added on screen after
//!   the projection, because `PICK_PIXELS` is in pixels — no test: it is how
//!   the converted tests of `selecting_under_the_hand.rs` aim, an offset added
//!   to what `driver::sketch_point_on_screen` returns, and they hold it by
//!   still telling a press 10 px from a point from one 22 px away
//! - a test that clicks only to get somewhere reaches that state by a named
//!   control instead, or sets the state up directly, and no longer clicks a
//!   coordinate — no test: it is the conversion of those tests and of
//!   `driver::start_a_sketch`, read off the diff, and only part of it was
//!   built. None of them writes a coordinate of the screen any more; where no
//!   named control reaches the state, they still click, at a point of the
//!   drawing where the view shows it — the paragraph below names each. No rule
//!   holds this: `screen_points.rs` sees a point written in numbers, not a
//!   click made only to get somewhere
//! - `FromTheCentre` and `on_canvas` go, replaced by the projection — no test:
//!   once they are deleted from the driver, a use left behind does not compile
//! - a driver test that writes a screen point with two numeric literals fails,
//!   the few positions that belong to the window kept on a short allow-list —
//!   no test: not in this file, `screen_points.rs` holds it beside its
//!   allow-list
//! - `cao_app` gains a way, for tests only, to ask where a world point lands on
//!   screen in the last frame drawn —
//!   `each_base_plane_clicked_where_the_view_shows_it_opens_a_sketch_on_that_plane`,
//!   `a_line_clicked_at_two_points_of_the_sketch_runs_between_those_points`,
//!   which reach it through the driver, the `test-support` feature compiling it
//! - every driver test keeps asserting what it asserts today; only where it
//!   clicks changes — no test: it is read off the diff, no assertion moving,
//!   and checked by the whole suite passing as before, then again on a probe
//!   where the canvas stands elsewhere — a window of another size. A verdict
//!   that changes on the way is a finding to report, not one to adjust
//! - the tests about a reach in pixels keep their pixel distances — no test:
//!   it is read off the diff, the 10 and the 22 of
//!   `selecting_under_the_hand.rs` staying as they are
//! - no driver test gets slower: the projection reads the last frame and
//!   renders nothing more — no test: a duration asserted in the gate would
//!   fail on a slow machine rather than on a slow driver. The way there reads
//!   a camera and a rectangle the frame already kept, and the suite's time is
//!   compared by hand on either side of the conversion
//!
//! What was built, against the issue's words: no coordinate of the screen, and
//! a projected point of the drawing instead, wherever a test clicks only to get
//! somewhere and no named control does that one thing. The ribbon has no
//! button for any of these, and the extrusion panel only counts the areas
//! chosen. A sketch opened on a plane of the origin — `driver::start_a_sketch`,
//! held by the third test above, and `section_view.rs` beside a raised block. A
//! sketch started on a section's cut — `a_sketch_on_the_cut` in
//! `keeping_a_section.rs` and in `tilting_a_section.rs`, and the test of
//! `keeping_a_section.rs` that reads the section's name above the view. A face
//! chosen for the section to cut along — the face-lost test of
//! `keeping_a_section.rs`. The area an extrusion raises —
//! `driver::INSIDE_THE_RECTANGLE`, `ON_THE_BLOCK` in the two section files, and
//! the block of `drawing.rs`. And every shape drawn only to have something to
//! act on — a segment, a rectangle, a dimension placed for its field, through
//! `driver::draw_a_segment`, `driver::ON_THE_SEGMENT` and the like: drawing has
//! no named control either.

mod driver;

use cao_part::PartDocument;
use glam::{DVec2, DVec3};

/// A point of each plane of the origin, with the name the history gives a
/// sketch laid on it. Each is a point a new part's eye reaches before it
/// reaches the other two planes, so that a click there can only mean that one.
const ON_EACH_BASE_PLANE: [(DVec3, &str); 3] = [
    (DVec3::new(0.0, 40.0, 30.0), "Plan YZ"),
    (DVec3::new(-40.0, 40.0, 0.0), "Plan XY"),
    (DVec3::new(-40.0, 0.0, 30.0), "Plan XZ"),
];

/// Where the sketch the line is drawn in is opened, on YZ.
const ON_THE_YZ_PLANE: DVec3 = DVec3::new(0.0, 40.0, 30.0);

/// The two ends a line is aimed at, in the coordinates of the sketch on YZ:
/// away from the origin and from the plane's axes, which pull a point laid
/// near them, so that what is laid is the point under the hand. Their pull is
/// counted in pixels, so the margin is kept wide enough for a canvas a good
/// deal shorter than today's.
const THE_TWO_ENDS: [DVec2; 2] = [DVec2::new(41.3, 63.7), DVec2::new(77.9, 36.4)];

/// How far an end laid may stand from the point aimed at, in millimetres:
/// about a tenth of a pixel at the framing a fresh sketch opens on, so that a
/// click one pixel off is caught, and still ten times what the camera's
/// single-precision arithmetic loses there and back.
const TOLERANCE_MM: f64 = 0.05;

/// The name of the part the lines are drawn in, which names its file too.
const THE_PART: &str = "Entretoise";

#[test]
fn each_base_plane_clicked_where_the_view_shows_it_opens_a_sketch_on_that_plane() {
    for (index, (point, plane)) in ON_EACH_BASE_PLANE.into_iter().enumerate() {
        let mut app = driver::open(&format!("clicking_the_drawing_base_plane_{index}"));
        driver::create_a_part(&mut app, "Platine");
        driver::click(&mut app, "Nouvelle esquisse");

        driver::click_at_world_point(&mut app, point);
        driver::open_the_history(&mut app);

        assert!(
            driver::shows(&app, &format!("Esquisse — {plane}")),
            "{point:?} clicked where the view shows it opened no sketch on « {plane} », \
             showing {:?}",
            driver::on_screen(&app),
        );
    }
}

#[test]
fn a_line_clicked_at_two_points_of_the_sketch_runs_between_those_points() {
    let test = "clicking_the_drawing_a_line_between_two_points";
    let mut app = a_sketch_on_yz(test, driver::SCREEN);

    a_line_through(&mut app, THE_TWO_ENDS);

    let laid = the_lines_laid(&mut app, test);
    assert!(
        matches!(laid.as_slice(), [line] if runs_between(*line, THE_TWO_ENDS)),
        "two clicks where the view shows {THE_TWO_ENDS:?} laid {laid:?}, not one line \
         between those points to within {TOLERANCE_MM} mm",
    );
}

#[test]
fn the_same_two_clicks_land_on_the_same_points_whatever_the_size_of_the_window() {
    let windows = [
        ("taller", egui::vec2(960.0, 1200.0)),
        ("wider", egui::vec2(1920.0, 800.0)),
    ];
    for (shape, size) in windows {
        let test = format!("clicking_the_drawing_on_a_{shape}_window");
        let mut app = a_sketch_on_yz(&test, size);
        let canvas = driver::canvas(&app);
        if shape == "taller" {
            assert!(
                canvas.height() > canvas.width(),
                "a window of {size:?} was meant to give a canvas taller than wide, \
                 the case the offsets from the centre of #724 could not follow, and \
                 gave {canvas:?}",
            );
        }

        a_line_through(&mut app, THE_TWO_ENDS);

        let laid = the_lines_laid(&mut app, &test);
        assert!(
            matches!(laid.as_slice(), [line] if runs_between(*line, THE_TWO_ENDS)),
            "on a window of {size:?}, the canvas at {canvas:?}, two clicks where the view \
             shows {THE_TWO_ENDS:?} laid {laid:?}, not one line between those points to \
             within {TOLERANCE_MM} mm",
        );
    }
}

/// A new part open on a sketch of the plane YZ, on a window of this size,
/// with the grid's pull switched off: what a click lays is then the point
/// under the hand, not the nearest node of the grid.
fn a_sketch_on_yz(test: &str, size: egui::Vec2) -> driver::App {
    let mut settings = cao_prefs::Settings::default();
    settings.viewport.grid_snap = false;
    let mut app = driver::open_with(test, size, settings);
    driver::create_a_part(&mut app, THE_PART);
    driver::click(&mut app, "Nouvelle esquisse");
    driver::click_at_world_point(&mut app, ON_THE_YZ_PLANE);
    assert!(
        driver::is_enabled(&app, "Terminer"),
        "the click on YZ opened no sketch, showing {:?}",
        driver::on_screen(&app),
    );
    app
}

/// The line tool, a click at each point where the view shows it, then Escape
/// to lay down the tool.
fn a_line_through(app: &mut driver::App, ends: [DVec2; 2]) {
    driver::click(app, "Ligne (L)");
    for end in ends {
        driver::click_at_sketch_point(app, end);
    }
    driver::press(app, egui::Key::Escape);
}

/// The ends of every line of the part's sketch, as the application left the
/// part on disk once it was put away.
fn the_lines_laid(app: &mut driver::App, test: &str) -> Vec<(DVec2, DVec2)> {
    driver::click(app, "Accueil");
    let path = std::env::temp_dir()
        .join("cao-driver")
        .join(test)
        .join("documents")
        .join("CAO")
        .join(format!("{THE_PART}.caopart"));
    let part = PartDocument::load(&cao_app::adapters::files::DiskFiles, &path)
        .unwrap_or_else(|err| panic!("the part at {path:?} does not open: {err:?}"));
    let sketch = &part.sketches()[0];
    sketch
        .live_segments()
        .map(|(segment, _)| sketch.endpoints(segment))
        .collect()
}

/// Whether a line runs between these two points, whichever end it starts
/// from.
fn runs_between((start, end): (DVec2, DVec2), [one, other]: [DVec2; 2]) -> bool {
    let close = |laid: DVec2, aimed: DVec2| laid.distance(aimed) < TOLERANCE_MM;
    (close(start, one) && close(end, other)) || (close(start, other) && close(end, one))
}
