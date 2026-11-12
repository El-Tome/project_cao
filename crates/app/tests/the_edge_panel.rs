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
//! - With the panel open, hovering an edge of the part in the 3D view lights
//!   it, and clicking it chooses it; clicking another edge replaces the
//!   first —
//!   `with_the_panel_open_the_edge_under_the_hand_is_lit_and_a_click_chooses_it_and_another_replaces_it`;
//!   the light drawn along the edge's own points in
//!   `crates/app/src/screens/viewport/render/tests.rs`; read as a pick in
//!   pixels, within `EDGE_REACH` and none further or inside a face, by
//!   `an_edge_is_taken_within_its_reach_in_pixels_and_none_further_or_well_inside_a_face`,
//!   the nearest of two within reach by
//!   `where_two_edges_are_within_reach_the_nearest_is_taken`; an edge hidden
//!   behind the matter is not taken through the face in front of it —
//!   decided when the criterion was taken up, a pick through a face being a
//!   surprise — by `an_edge_behind_the_part_is_neither_lit_nor_taken_through_the_face_in_front_of_it`;
//!   and with the panel closed nothing of the kind, by
//!   `with_the_panel_closed_no_edge_is_lit_and_a_click_chooses_none`
//! - An edge chosen, a value typed and « Appliquer » pressed, the part is
//!   chamfered or rounded on that edge and one new line appears in the
//!   history — the panel closed, the part's volume the block's less what the
//!   arithmetic takes off that edge, one line more, read as it reads —
//!   `an_edge_chosen_a_value_typed_and_apply_pressed_chamfers_or_rounds_it_and_adds_one_line`
//! - When the kernel reduced the value, that history line reads both values,
//!   as « 5 mm demandé, 2 mm appliqué »; otherwise only the value —
//!   `a_value_the_faces_cannot_give_reads_both_values_on_its_history_line`,
//!   the value alone by the test before it
//! - A decline of the kernel at « Appliquer » records nothing: the panel stays
//!   open and shows the reason in French — on the rim of a disc, which the
//!   step does not handle until #700: no line added, the part as it was, the
//!   panel open and saying why, until another edge is chosen or the panel
//!   closed —
//!   `a_chamfer_the_kernel_declines_records_nothing_and_its_panel_says_why_until_another_edge_or_cancel`

mod driver;

use cao_app::app::EDGE_REACH;
use cao_solid::DrawnEdge;
use egui_kittest::kittest::NodeT;
use glam::{DVec2, DVec3};

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

const _: () = assert!(
    EDGE_REACH > 2.0 && EDGE_REACH <= 10.0,
    "an edge is taken within a few points of it",
);

/// The block of `driver::a_block`, as the part holds it: its rectangle lands
/// where the drawing snapped it, not on the points clicked. The view it is
/// left in looks from +X, -Y and +Z, so its faces towards those three are
/// seen, and the three others are not.
struct Block {
    low: DVec3,
    high: DVec3,
}

impl Block {
    /// Read back from the part's file, which the end of every gesture writes.
    fn of(test: &str) -> Self {
        let (low, high) = the_part(test, "Bloc")
            .body()
            .bounds()
            .expect("the block holds matter");
        Self { low, high }
    }

    fn volume(&self) -> f64 {
        let size = self.high - self.low;
        size.x * size.y * size.z
    }

    fn length_of(edge: [DVec3; 2]) -> f64 {
        edge[0].distance(edge[1])
    }

    /// The top of the face towards +X, seen.
    fn near_top(&self) -> [DVec3; 2] {
        let (low, high) = (self.low, self.high);
        [
            DVec3::new(high.x, low.y, high.z),
            DVec3::new(high.x, high.y, high.z),
        ]
    }

    /// The left of the face towards +X, seen, from the corner it shares with
    /// the near top.
    fn near_left(&self) -> [DVec3; 2] {
        let (low, high) = (self.low, self.high);
        [
            DVec3::new(high.x, low.y, high.z),
            DVec3::new(high.x, low.y, low.z),
        ]
    }

    /// The bottom of the face towards -X, between two faces turned away from
    /// the eye: behind the matter.
    fn far_bottom(&self) -> [DVec3; 2] {
        let (low, high) = (self.low, self.high);
        [
            DVec3::new(low.x, low.y, low.z),
            DVec3::new(low.x, high.y, low.z),
        ]
    }

    fn middle_of_the_near_face(&self) -> DVec3 {
        let middle = (self.low + self.high) / 2.0;
        DVec3::new(self.high.x, middle.y, middle.z)
    }
}

fn middle([start, end]: [DVec3; 2]) -> DVec3 {
    (start + end) / 2.0
}

fn is_the_edge(drawn: Option<Vec<DVec3>>, [start, end]: [DVec3; 2]) -> bool {
    let Some(points) = drawn else {
        return false;
    };
    let near = |a: DVec3, b: DVec3| a.distance(b) < 1e-6;
    match (points.first(), points.last()) {
        (Some(first), Some(last)) => {
            (near(*first, start) && near(*last, end)) || (near(*first, end) && near(*last, start))
        }
        _ => false,
    }
}

fn hover_at_world_point(app: &mut driver::App, point: DVec3) {
    driver::hold_the_canvas(app);
    let at = driver::world_point_on_screen(app, point);
    app.hover_at(at);
    app.run();
}

/// The block, the Extrusion tab open and « Chanfrein » clicked in it.
fn chamfering(test: &str) -> (driver::App, Block) {
    opened_on(test, "Chanfrein")
}

/// The block, the Extrusion tab open and `tool` clicked in it.
fn opened_on(test: &str, tool: &str) -> (driver::App, Block) {
    let mut app = driver::a_block(test);
    let block = Block::of(test);
    driver::click_exactly(&mut app, "Extrusion");
    driver::click_exactly(&mut app, tool);
    assert!(
        !driver::is_enabled(&app, "Appliquer"),
        "« Appliquer » offered before any edge is chosen",
    );
    (app, block)
}

/// Where on screen a point stands `by` points off the middle of an edge,
/// square to it, on the side of the near face.
fn off_the_middle_of(app: &driver::App, block: &Block, edge: [DVec3; 2], by: f32) -> egui::Pos2 {
    let [start, end] = edge.map(|end| driver::world_point_on_screen(app, end));
    let centre = start + (end - start) / 2.0;
    let across = (end - start).normalized().rot90();
    let inwards = driver::world_point_on_screen(app, block.middle_of_the_near_face()) - centre;
    let side = if across.dot(inwards) >= 0.0 {
        1.0
    } else {
        -1.0
    };
    centre + across * side * by
}

fn chosen(app: &driver::App) -> Option<Vec<DVec3>> {
    app.state().the_edge_chosen()
}

fn lit(app: &driver::App) -> Option<Vec<DVec3>> {
    app.state().the_edge_lit_by_the_hand()
}

#[test]
fn with_the_panel_open_the_edge_under_the_hand_is_lit_and_a_click_chooses_it_and_another_replaces_it()
 {
    let (mut app, block) = chamfering("the_edge_under_the_hand_is_lit_and_chosen");

    hover_at_world_point(&mut app, middle(block.near_top()));
    assert!(
        is_the_edge(lit(&app), block.near_top()),
        "the hand on the near top lights {:?}",
        lit(&app),
    );
    assert_eq!(chosen(&app), None, "hovering chose an edge");

    driver::click_at_world_point(&mut app, middle(block.near_top()));
    assert!(
        is_the_edge(chosen(&app), block.near_top()),
        "a click on the near top chose {:?}",
        chosen(&app),
    );
    assert!(
        driver::is_enabled(&app, "Appliquer"),
        "« Appliquer » still greyed with an edge chosen and a size typed",
    );

    driver::click_at_world_point(&mut app, middle(block.near_left()));
    assert!(
        is_the_edge(chosen(&app), block.near_left()),
        "a click on the near left, after the near top, chose {:?}",
        chosen(&app),
    );
    assert!(
        is_the_edge(lit(&app), block.near_left()),
        "the hand on the near left lights {:?}",
        lit(&app),
    );
    assert!(driver::is_enabled(&app, "Appliquer"));
}

#[test]
fn an_edge_is_taken_within_its_reach_in_pixels_and_none_further_or_well_inside_a_face() {
    let (mut app, block) = chamfering("an_edge_is_taken_within_its_reach");

    let beyond = off_the_middle_of(&app, &block, block.near_left(), EDGE_REACH + 3.0);
    driver::click_at(&mut app, beyond);
    assert_eq!(
        chosen(&app),
        None,
        "a click {} points off the near left took an edge",
        EDGE_REACH + 3.0,
    );

    driver::click_at_world_point(&mut app, block.middle_of_the_near_face());
    assert_eq!(
        chosen(&app),
        None,
        "a click in the middle of a face took an edge"
    );
    assert!(!driver::is_enabled(&app, "Appliquer"));

    let within = off_the_middle_of(&app, &block, block.near_left(), EDGE_REACH - 1.0);
    driver::click_at(&mut app, within);
    assert!(
        is_the_edge(chosen(&app), block.near_left()),
        "a click {} points off the near left chose {:?}",
        EDGE_REACH - 1.0,
        chosen(&app),
    );
}

#[test]
fn where_two_edges_are_within_reach_the_nearest_is_taken() {
    let (mut app, block) = chamfering("the_nearest_edge_is_taken");

    for (along, other) in [
        (block.near_top(), block.near_left()),
        (block.near_left(), block.near_top()),
    ] {
        let [corner, far] = along.map(|end| driver::world_point_on_screen(&app, end));
        let on_it = corner + (far - corner).normalized() * EDGE_REACH / 2.0;
        driver::click_at(&mut app, on_it);
        assert!(
            is_the_edge(chosen(&app), along),
            "half a reach from the corner along {along:?}, within reach of {other:?} too, \
             the click chose {:?}",
            chosen(&app),
        );
    }
}

#[test]
fn an_edge_behind_the_part_is_neither_lit_nor_taken_through_the_face_in_front_of_it() {
    let (mut app, block) = chamfering("an_edge_behind_the_part_is_not_taken");
    let hidden = middle(block.far_bottom());
    let seen = middle([
        DVec3::new(block.high.x, block.low.y, block.low.z),
        DVec3::new(block.high.x, block.high.y, block.low.z),
    ]);
    let apart = driver::world_point_on_screen(&app, hidden)
        .distance(driver::world_point_on_screen(&app, seen));
    assert!(
        apart > 2.0 * EDGE_REACH,
        "the fixture's hidden edge stands {apart} points from the near bottom, within reach of it",
    );

    hover_at_world_point(&mut app, hidden);
    assert_eq!(lit(&app), None, "an edge behind the matter is lit");
    driver::click_at_world_point(&mut app, hidden);
    assert_eq!(chosen(&app), None, "an edge behind the matter is taken");
    assert!(!driver::is_enabled(&app, "Appliquer"));
}

#[test]
fn with_the_panel_closed_no_edge_is_lit_and_a_click_chooses_none() {
    let test = "with_the_panel_closed_no_edge_is_lit";
    let mut app = driver::a_block(test);
    let block = Block::of(test);
    driver::click_exactly(&mut app, "Extrusion");

    hover_at_world_point(&mut app, middle(block.near_top()));
    assert_eq!(lit(&app), None, "an edge is lit with no panel open");
    driver::click_at_world_point(&mut app, middle(block.near_top()));

    driver::click_exactly(&mut app, "Chanfrein");
    assert_eq!(
        chosen(&app),
        None,
        "a click before the panel opened chose an edge"
    );
    assert!(!driver::is_enabled(&app, "Appliquer"));
}

/// The part of that name as the application last wrote it, which the end of
/// every gesture does.
fn the_part(test: &str, named: &str) -> cao_part::PartDocument {
    let path = driver::the_part_s_file(test, named);
    cao_part::PartDocument::load(&cao_app::adapters::files::DiskFiles, &path)
        .unwrap_or_else(|err| panic!("the part at {path:?} does not open: {err:?}"))
}

/// The block as the application last wrote it, its volume.
fn the_part_s_volume(test: &str) -> f64 {
    the_part(test, "Bloc").body().volume()
}

/// What a chamfer or a fillet of `size` takes off a square edge `length` long.
fn taken_off(tool: &str, size: f64, length: f64) -> f64 {
    match tool {
        "Chanfrein" => size * size * length / 2.0,
        _ => (1.0 - std::f64::consts::PI / 4.0) * size * size * length,
    }
}

/// The near top chosen, `size` typed beside `value` and « Appliquer »
/// pressed, the history open: the lines it held before.
fn applied_on_the_near_top(
    app: &mut driver::App,
    block: &Block,
    value: &str,
    size: &str,
) -> Vec<String> {
    driver::open_the_history(app);
    let before = driver::steps(app);
    driver::click_at_world_point(app, middle(block.near_top()));
    driver::type_beside(app, value, size);
    driver::click(app, "Appliquer");
    before
}

#[test]
fn an_edge_chosen_a_value_typed_and_apply_pressed_chamfers_or_rounds_it_and_adds_one_line() {
    for (tool, value, line) in [
        ("Chanfrein", THE_CHAMFER_S_VALUE, "Arête chanfreinée"),
        ("Congé", THE_FILLET_S_VALUE, "Arête arrondie"),
    ] {
        let test = format!("an_edge_is_applied_{}", tool.to_lowercase());
        let (mut app, block) = opened_on(&test, tool);

        let before = applied_on_the_near_top(&mut app, &block, value, "2");

        assert!(
            !driver::reads(&app, value),
            "« {tool} » applied left its panel open, showing {:?}",
            driver::on_screen(&app),
        );
        let after = driver::steps(&app);
        let added = format!("{}. {line}", before.len() + 1);
        assert_eq!(
            after.len(),
            before.len() + 1,
            "« {tool} » applied added {} lines: {after:?}",
            after.len() - before.len(),
        );
        assert_eq!(after.last(), Some(&added), "« {tool} » applied");
        assert_eq!(
            driver::detail_of(&app, &added).as_deref(),
            Some("2 mm"),
            "« {tool} » of 2, which the faces give",
        );
        let expected = block.volume() - taken_off(tool, 2.0, Block::length_of(block.near_top()));
        let made = the_part_s_volume(&test);
        assert!(
            (made - expected).abs() <= 1e-6 * expected,
            "« {tool} » of 2 on the near top leaves {made}, where the arithmetic gives {expected}",
        );
    }
}

#[test]
fn a_value_the_faces_cannot_give_reads_both_values_on_its_history_line() {
    let test = "a_value_the_faces_cannot_give_reads_both";
    let (mut app, block) = chamfering(test);
    let depth = block.high.x - block.low.x;

    let before = applied_on_the_near_top(&mut app, &block, THE_CHAMFER_S_VALUE, "50");

    let added = format!("{}. Arête chanfreinée", before.len() + 1);
    assert_eq!(
        driver::detail_of(&app, &added).as_deref(),
        Some("50 mm demandé, 10 mm appliqué"),
        "a chamfer of 50 on an edge whose top is {depth} deep",
    );
    let expected =
        block.volume() - taken_off("Chanfrein", depth, Block::length_of(block.near_top()));
    let made = the_part_s_volume(test);
    assert!(
        (made - expected).abs() <= 1e-6 * expected,
        "a chamfer of 50 reduced to {depth} leaves {made}, where the arithmetic gives {expected}",
    );
}

const NOT_YET_A_CHAMFER: &str =
    "Ce chanfrein n'est pas encore possible sur cette arête : la pièce reste comme elle était.";

/// A part named « Disque »: a circle drawn on the plane YZ, 40 across its
/// radius, raised 60 along X, its sketch then finished. Its two rims are
/// circles, which a chamfer does not take until #700.
fn a_disc(test: &str) -> driver::App {
    let mut app = driver::open_held_with(test, cao_prefs::Settings::default());
    driver::create_a_part(&mut app, "Disque");
    driver::start_a_sketch(&mut app);
    driver::click_exactly(&mut app, "Cercle (C)");
    driver::click_at_sketch_point(&mut app, driver::ON_THE_BLOCK);
    driver::click_at_sketch_point(&mut app, driver::ON_THE_BLOCK + DVec2::new(40.0, 0.0));
    driver::click_exactly(&mut app, "Extrusion");
    driver::click_the_button(&mut app, "1");
    driver::click(&mut app, "Ajout de matière");
    driver::click_at_sketch_point(&mut app, driver::ON_THE_BLOCK);
    driver::type_beside(&mut app, "Hauteur :", "60");
    driver::click(&mut app, "Appliquer");
    driver::press(&mut app, egui::Key::Escape);
    driver::finish_the_sketch(&mut app);
    driver::click_the_button(&mut app, "Esquisse");
    assert!(
        !driver::a_sketch_is_open(&app),
        "the sketch the disc was raised from is still being edited, showing {:?}",
        driver::on_screen(&app),
    );
    app
}

/// Of an edge's points, the one standing furthest on screen from `away`.
fn furthest_on_screen_from(app: &driver::App, edge: &DrawnEdge, away: egui::Pos2) -> DVec3 {
    edge.points
        .iter()
        .copied()
        .max_by(|a, b| {
            let from = |point: DVec3| driver::world_point_on_screen(app, point).distance(away);
            from(*a).total_cmp(&from(*b))
        })
        .expect("an edge drawn through points")
}

fn the_middle_on_screen(app: &driver::App, edge: &DrawnEdge) -> egui::Pos2 {
    let sum = edge
        .points
        .iter()
        .map(|point| driver::world_point_on_screen(app, *point).to_vec2())
        .fold(egui::Vec2::ZERO, |sum, at| sum + at);
    (sum / edge.points.len() as f32).to_pos2()
}

#[test]
fn a_chamfer_the_kernel_declines_records_nothing_and_its_panel_says_why_until_another_edge_or_cancel()
 {
    let test = "a_declined_chamfer_says_why";
    let mut app = a_disc(test);
    let disc = the_part(test, "Disque");
    let volume = disc.body().volume();
    let mut rims = disc.body().drawn_edges();
    assert_eq!(rims.len(), 2, "a disc has two rims: {rims:?}");
    rims.sort_by(|a, b| a.points[0].x.total_cmp(&b.points[0].x));
    let [far, near] = [&rims[0], &rims[1]];
    let on_the_near_rim = furthest_on_screen_from(&app, near, the_middle_on_screen(&app, far));
    let on_the_far_rim = furthest_on_screen_from(&app, far, the_middle_on_screen(&app, near));
    let on = |points: Option<Vec<DVec3>>, rim: &DrawnEdge| {
        points.is_some_and(|points| {
            points
                .iter()
                .all(|point| (point.x - rim.points[0].x).abs() < 1e-6)
        })
    };

    driver::click_exactly(&mut app, "Extrusion");
    driver::click_exactly(&mut app, "Chanfrein");
    driver::open_the_history(&mut app);
    let before = driver::steps(&app);
    driver::click_at_world_point(&mut app, on_the_near_rim);
    assert!(
        on(chosen(&app), near),
        "a click on the near rim chose {:?}",
        chosen(&app),
    );
    driver::click(&mut app, "Appliquer");

    assert!(
        driver::reads(&app, THE_CHAMFER_S_VALUE),
        "the panel closed on a chamfer the part could not take, showing {:?}",
        driver::on_screen(&app),
    );
    assert!(
        driver::reads(&app, NOT_YET_A_CHAMFER),
        "the panel does not say why, showing {:?}",
        driver::on_screen(&app),
    );
    assert_eq!(
        driver::steps(&app),
        before,
        "a line was added for nothing done"
    );
    let after = the_part(test, "Disque").body().volume();
    assert!(
        (after - volume).abs() <= 1e-9 * volume,
        "the disc holds {after}, where it held {volume}",
    );

    driver::click_at_world_point(&mut app, on_the_far_rim);
    assert!(
        on(chosen(&app), far),
        "a click on the far rim chose {:?}",
        chosen(&app),
    );
    assert!(
        !driver::reads(&app, NOT_YET_A_CHAMFER),
        "another edge chosen, the panel still says why the last one was declined",
    );

    driver::click(&mut app, "Appliquer");
    assert!(
        driver::reads(&app, NOT_YET_A_CHAMFER),
        "the far rim is declined too"
    );
    driver::click_the_button(&mut app, "Annuler");
    driver::click_exactly(&mut app, "Chanfrein");
    assert!(
        !driver::reads(&app, NOT_YET_A_CHAMFER),
        "the panel closed and opened again still says why an edge was declined",
    );
}
