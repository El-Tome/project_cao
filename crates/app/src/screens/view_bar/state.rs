use cao_prefs::config::LengthUnit;
use cao_prefs::{Command, Shortcuts};
use cao_render::MatterInFront;
use glam::{DVec2, DVec3};

use crate::lang::Catalogue;
use crate::screens::SketchContext;
use crate::screens::ribbon::{icons, is_enabled};
use crate::screens::sketch::SketchEditor;
use crate::screens::viewport::{Shown, ViewportState};
use crate::ui::icon::Icon;
use crate::wording::{command as wording, matter, shortcuts};

/// What the bar shows, left to right: « Recadrer », « Vue en coupe », the
/// menu of the matter in front, and the cursor's coordinates.
#[derive(Clone, Debug, PartialEq)]
pub struct ViewBar {
    pub reframe: Offered,
    pub section_view: Offered,
    pub matter_in_front: MatterInFrontMenu,
    pub coordinates: String,
    /// The longest the coordinates read here, which the bar keeps room for.
    pub widest_coordinates: String,
}

/// A command the bar offers as the ribbon offers it: its name, the key bound
/// to it, its help, whether it can be pressed, and — for one that switches
/// something — whether that is on.
#[derive(Clone, Debug, PartialEq)]
pub struct Offered {
    pub command: Command,
    pub name: String,
    pub key: Option<String>,
    pub hint: String,
    pub enabled: bool,
    pub on: Option<bool>,
}

impl Offered {
    pub fn of(
        command: Command,
        on: Option<bool>,
        part: &SketchContext<'_>,
        bound: &Shortcuts,
    ) -> Self {
        let lang = part.lang;
        Self {
            command,
            name: wording::label(lang, command),
            key: bound
                .chord_for(command)
                .map(|chord| shortcuts::chord(lang, chord)),
            hint: wording::hint(lang, command),
            enabled: is_enabled(command, part.document, part.editor, part.extrusion),
            on,
        }
    }
}

impl ViewBar {
    /// The coordinates are said in the unit the scale bar shows its step in,
    /// so that the two agree.
    pub fn of(
        part: &SketchContext<'_>,
        viewport: &ViewportState,
        shown: &Shown,
        bound: &Shortcuts,
    ) -> Self {
        let (document, editor, lang) = (&*part.document, &*part.editor, part.lang);
        let offered = |command, on| Offered::of(command, on, part, bound);
        let unit = shown.unit.unwrap_or(LengthUnit::Millimeter);
        let pointed = Pointed::of(editor, shown.on_the_part);
        Self {
            reframe: offered(Command::Reframe, None),
            section_view: offered(Command::SectionView, Some(viewport.section().is_some())),
            matter_in_front: MatterInFrontMenu::of(editor, lang),
            coordinates: coordinates(pointed, unit, document.scale(), lang),
            widest_coordinates: widest_coordinates(editor.active_sketch().is_some(), unit, lang),
        }
    }
}

/// What the coordinates read with a sign and four figures before the comma on
/// every axis: room enough for the bar's buttons to stay put as the mouse
/// moves, in a sketch or in the free view.
fn widest_coordinates(in_a_sketch: bool, unit: LengthUnit, lang: &Catalogue) -> String {
    let widest = -9999.99 * unit.millimeters();
    let pointed = if in_a_sketch {
        Pointed::OnThePlane(DVec2::splat(widest))
    } else {
        Pointed::OnThePart(DVec3::splat(widest))
    };
    coordinates(pointed, unit, 1.0, lang)
}

/// The bar's « Matière devant »: what it reads, whether it opens, and the
/// three states it offers in its menu.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MatterInFrontMenu {
    /// The icon of the state the open sketch is in, as the ribbon's wears it.
    pub icon: Icon,
    pub label: String,
    pub enabled: bool,
    pub choices: Vec<MatterInFrontChoice>,
}

/// One state the menu offers, marked when the open sketch is in it.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MatterInFrontChoice {
    pub state: MatterInFront,
    pub label: String,
    pub current: bool,
}

impl MatterInFrontMenu {
    /// Greyed with no sketch open, as the ribbon's button is: there is no
    /// plane to show the matter in front of.
    pub fn of(editor: &SketchEditor, lang: &Catalogue) -> Self {
        let now = editor.matter_in_front;
        let choices = [
            MatterInFront::Hidden,
            MatterInFront::SeeThrough,
            MatterInFront::Opaque,
        ]
        .into_iter()
        .map(|state| MatterInFrontChoice {
            state,
            label: matter::state(lang, state),
            current: state == now,
        })
        .collect();
        Self {
            icon: icons::matter_in_front(now),
            label: matter::in_front(lang, now),
            enabled: editor.active_sketch().is_some(),
            choices,
        }
    }
}

/// What the cursor points at, for the bar's coordinates.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Pointed {
    Nothing,
    /// A point of the open sketch, in its plane's own frame.
    OnThePlane(DVec2),
    /// A point of the part, in the world.
    OnThePart(DVec3),
}

impl Pointed {
    /// In a sketch, where a click would place the point: the end the tool in
    /// hand aims at, else the cursor, and nothing once the cursor leaves the
    /// view. Outside a sketch, the point of the part under the cursor.
    pub fn of(editor: &SketchEditor, on_the_part: Option<DVec3>) -> Self {
        if editor.active_sketch().is_none() {
            return on_the_part.map_or(Self::Nothing, Self::OnThePart);
        }
        editor
            .cursor
            .map(|cursor| editor.aimed.map_or(cursor, |aim| aim.position))
            .map_or(Self::Nothing, Self::OnThePlane)
    }
}

/// The bar's coordinates, to two decimals of `unit`, a unit of the part being
/// worth `millimeters_per_unit` millimetres.
pub fn coordinates(
    pointed: Pointed,
    unit: LengthUnit,
    millimeters_per_unit: f64,
    lang: &Catalogue,
) -> String {
    let separator = lang.t("view_bar.decimal_separator");
    let figure = |value: f64| {
        let rounded = (value * millimeters_per_unit / unit.millimeters() * 100.0).round() / 100.0;
        let rounded = if rounded == 0.0 { 0.0 } else { rounded };
        format!("{rounded:.2}").replace('.', &separator)
    };
    match pointed {
        Pointed::Nothing => lang.t("view_bar.nothing_pointed"),
        Pointed::OnThePlane(at) => lang.t_with(
            "view_bar.on_the_plane",
            &[
                ("x", &figure(at.x)),
                ("y", &figure(at.y)),
                ("unit", unit.suffix()),
            ],
        ),
        Pointed::OnThePart(at) => lang.t_with(
            "view_bar.on_the_part",
            &[
                ("x", &figure(at.x)),
                ("y", &figure(at.y)),
                ("z", &figure(at.z)),
                ("unit", unit.suffix()),
            ],
        ),
    }
}

#[cfg(test)]
mod tests;
