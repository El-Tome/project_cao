//! What prefs · theme.rs is held to.
//!
//! Closes #286.
//! - the alert colour is a colour of the theme, and a theme saved before it
//!   existed still reads —
//!   `a_theme_saved_before_the_alert_colour_existed_still_reads`
//! - it is edited in the settings like the others — no test: the row is one
//!   call in `settings/appearance.rs`, which draws and decides nothing
//!
//! Closes #724.
//! - the view's background: plain `vue_esquisse` — `background.sketch_view`
//!   since #749 put the tokens in English — in a sketch, a radial gradient in
//!   the free view, as the default of the existing background setting —
//!   `a_sketch_is_drawn_on_the_plain_sketch_view_colour_by_default`,
//!   `the_free_view_is_drawn_on_the_redesign_s_radial_gradient_by_default`;
//!   which of the two a frame draws is held in
//!   `crates/app/src/screens/viewport/render/tests.rs`, under its own block
//! - the « Par défaut » profile takes the redesign's viewport colours as
//!   defaults — `the_default_grid_and_axes_are_the_redesign_s`, and every
//!   colour the redesign does not name keeps the value it had —
//!   `every_colour_the_redesign_does_not_name_keeps_its_default`
//! - a profile Tom saved keeps its own — no test: in this file; held by
//!   `a_profile_saved_with_viewport_colours_of_its_own_reads_back_with_them`,
//!   in `crates/prefs/src/profiles/tests.rs`, under its own block

use super::*;

fn gradient() -> Background {
    Background::Linear {
        angle_degrees: 0.0,
        stops: vec![
            Stop::new(0.0, Rgba::opaque(0.0, 0.0, 0.0)),
            Stop::new(0.5, Rgba::opaque(1.0, 0.0, 0.0)),
            Stop::new(1.0, Rgba::opaque(1.0, 1.0, 1.0)),
        ],
    }
}

#[test]
fn a_gradient_reads_between_its_stops() {
    let background = gradient();
    assert_eq!(background.sample(0.0), Rgba::opaque(0.0, 0.0, 0.0));
    assert_eq!(background.sample(0.5), Rgba::opaque(1.0, 0.0, 0.0));
    assert_eq!(background.sample(1.0), Rgba::opaque(1.0, 1.0, 1.0));

    let quarter = background.sample(0.25);
    assert!((quarter.r - 0.5).abs() < 1e-5, "{quarter:?}");
    assert!(quarter.g.abs() < 1e-5);
}

/// Past either end, a gradient holds its last colour rather than fading to
/// nothing.
#[test]
fn a_gradient_holds_its_ends() {
    let background = gradient();
    assert_eq!(background.sample(-3.0), Rgba::opaque(0.0, 0.0, 0.0));
    assert_eq!(background.sample(9.0), Rgba::opaque(1.0, 1.0, 1.0));
}

#[test]
fn a_gradient_with_no_stops_still_gives_a_colour() {
    let empty = Background::Linear {
        angle_degrees: 0.0,
        stops: Vec::new(),
    };
    assert_eq!(empty.sample(0.5), Rgba::opaque(0.0, 0.0, 0.0));
}

#[test]
fn a_solid_background_is_the_same_everywhere() {
    let background = Background::Solid(Rgba::opaque(0.2, 0.3, 0.4));
    assert_eq!(background.sample(0.0), background.sample(1.0));
}

/// The alert colour landed after people already had themes on disk, so a file
/// that predates it has to read as a theme with the default.
#[test]
fn a_theme_saved_before_the_alert_colour_existed_still_reads() {
    let mut saved = serde_json::to_value(Theme::default()).expect("a theme writes out");
    saved
        .as_object_mut()
        .expect("a theme is an object")
        .remove("going")
        .expect("the alert colour is written out with the rest");

    let read: Theme = serde_json::from_value(saved).expect("a theme from before the colour");

    assert_eq!(read.going, Theme::default().going);
}

/// A colour read back to the bytes it would be written with: the design file
/// writes its colours in hex, so a byte is as near as either side can mean.
fn bytes(color: Rgba) -> [u8; 4] {
    color
        .to_array()
        .map(|channel| (channel * 255.0).round() as u8)
}

/// The design file the chrome's tests read too, taken in when the tests are
/// built rather than opened from the disk: a token delivered anew fails here
/// as it fails there.
const DESIGN: &str = include_str!("../../../../docs/redesign/tokens.json");

/// A colour of `color.`, written `#RRGGBB`, as the bytes of an opaque colour.
fn written(path: &str) -> [u8; 4] {
    let tokens: serde_json::Value = serde_json::from_str(DESIGN).expect("the design file is JSON");
    let hex = path
        .split('.')
        .try_fold(&tokens["color"], |value, key| value.get(key))
        .and_then(serde_json::Value::as_str)
        .and_then(|text| text.strip_prefix('#'))
        .unwrap_or_else(|| panic!("color.{path} is no #RRGGBB in the design file"));
    let channel = |at: usize| {
        u8::from_str_radix(&hex[at..at + 2], 16)
            .unwrap_or_else(|_| panic!("color.{path} is no #RRGGBB in the design file"))
    };
    [channel(0), channel(2), channel(4), 0xFF]
}

/// The same, at the opacity the design file gives the axes.
fn written_at_the_axes_opacity(path: &str) -> [u8; 4] {
    let tokens: serde_json::Value = serde_json::from_str(DESIGN).expect("the design file is JSON");
    let opacity = tokens["color"]["axes"]["opacity"]
        .as_f64()
        .expect("color.axes.opacity is a number");
    let [r, g, b, _] = written(path);
    [r, g, b, (opacity * 255.0).round() as u8]
}

#[test]
fn a_sketch_is_drawn_on_the_plain_sketch_view_colour_by_default() {
    let background = Theme::default().sketch_background;

    assert!(
        matches!(background, Background::Solid(colour) if bytes(colour) == written("background.sketch_view")),
        "a sketch's default background is {background:?}, not background.sketch_view, \
         a plain {:?}",
        written("background.sketch_view"),
    );
}

#[test]
fn the_free_view_is_drawn_on_the_redesign_s_radial_gradient_by_default() {
    let background = Theme::default().background;
    let Background::Radial {
        center,
        radius,
        stops,
    } = background
    else {
        panic!("the free view's default background is {background:?}, not a radial gradient");
    };

    assert!(
        (center[0] - 0.5).abs() < 1e-6 && (center[1] - 0.55).abs() < 1e-6,
        "the gradient opens from {center:?}: the mockup's cy=45% counts from the top, \
         and a centre here counts from the bottom, so slightly above the middle is 0.55",
    );
    assert!((radius - 0.7).abs() < 1e-6, "the gradient reaches {radius}");
    let stops: Vec<(f32, [u8; 4])> = stops
        .iter()
        .map(|stop| (stop.at, bytes(stop.color)))
        .collect();
    assert_eq!(
        stops,
        [
            (0.0, written("background.view3d_centre")),
            (1.0, written("background.view3d_edge"))
        ],
        "background.view3d_centre to background.view3d_edge",
    );
}

#[test]
fn the_default_grid_and_axes_are_the_redesign_s() {
    let theme = Theme::default();

    let wrong: Vec<String> = [
        ("grid_minor", theme.grid_minor, written("sketch.grid_minor")),
        ("grid_major", theme.grid_major, written("sketch.grid_major")),
        (
            "axis_x",
            theme.axis_x,
            written_at_the_axes_opacity("axes.x"),
        ),
        (
            "axis_y",
            theme.axis_y,
            written_at_the_axes_opacity("axes.y"),
        ),
        (
            "axis_z",
            theme.axis_z,
            written_at_the_axes_opacity("axes.z"),
        ),
    ]
    .into_iter()
    .filter(|(_, worn, design)| bytes(*worn) != *design)
    .map(|(name, worn, design)| {
        format!(
            "{name} is {:?}, the redesign writes {design:?}",
            bytes(worn)
        )
    })
    .collect();

    assert!(
        wrong.is_empty(),
        "the default profile draws its grid and axes in other colours:\n  {}",
        wrong.join("\n  "),
    );
}

/// What the redesign leaves alone is pinned as it stood before it: the sketch
/// strokes are #742's to change and the extrusion's colours #740's, and an
/// edit to any of them should show here rather than slip through with this
/// issue.
#[test]
fn every_colour_the_redesign_does_not_name_keeps_its_default() {
    let theme = Theme::default();

    let moved: Vec<String> = [
        (
            "sketch_free",
            theme.sketch_free,
            Rgba::new(0.98, 0.85, 0.35, 1.0),
        ),
        (
            "sketch_settled",
            theme.sketch_settled,
            Rgba::new(0.45, 0.85, 0.55, 1.0),
        ),
        (
            "sketch_inactive",
            theme.sketch_inactive,
            Rgba::new(0.70, 0.72, 0.76, 0.8),
        ),
        (
            "dimension",
            theme.dimension,
            Rgba::new(0.98, 0.85, 0.35, 0.9),
        ),
        (
            "dimension_driven",
            theme.dimension_driven,
            Rgba::new(0.62, 0.65, 0.70, 0.8),
        ),
        ("fixed", theme.fixed, Rgba::new(0.95, 0.45, 0.25, 1.0)),
        ("rule", theme.rule, Rgba::new(0.55, 0.75, 0.95, 1.0)),
        ("picked", theme.picked, Rgba::new(0.85, 0.40, 0.95, 1.0)),
        (
            "region_fill",
            theme.region_fill,
            Rgba::new(0.45, 0.65, 0.95, 0.10),
        ),
        ("solid", theme.solid, Rgba::new(0.78, 0.80, 0.84, 1.0)),
        (
            "highlight",
            theme.highlight,
            Rgba::new(0.30, 0.60, 0.95, 0.40),
        ),
        ("refused", theme.refused, Rgba::new(0.95, 0.35, 0.35, 0.40)),
        ("going", theme.going, Rgba::new(0.95, 0.25, 0.25, 1.0)),
        ("measure", theme.measure, Rgba::new(0.97, 0.97, 1.0, 1.0)),
        (
            "extrusion_add",
            theme.extrusion_add,
            Rgba::new(0.40, 0.85, 0.60, 0.45),
        ),
        (
            "extrusion_cut",
            theme.extrusion_cut,
            Rgba::new(0.95, 0.45, 0.40, 0.45),
        ),
    ]
    .into_iter()
    .filter(|(_, worn, before)| worn != before)
    .map(|(name, worn, before)| format!("{name} is {worn:?}, it was {before:?}"))
    .collect();

    assert!(
        moved.is_empty(),
        "colours the redesign does not name have moved:\n  {}",
        moved.join("\n  "),
    );
    assert_eq!(
        (
            theme.axis_width,
            theme.grid_minor_width,
            theme.grid_major_width,
            theme.grid_major_every,
            theme.sketch_width,
        ),
        (2.0, 1.0, 1.5, 10, 2.5),
        "the widths and the spacing of the major grid lines have moved too",
    );
}
