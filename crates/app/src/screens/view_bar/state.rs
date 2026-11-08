use cao_prefs::config::LengthUnit;
use cao_render::MatterInFront;
use glam::{DVec2, DVec3};

use crate::lang::Catalogue;
use crate::screens::sketch::SketchEditor;
use crate::wording::matter;

/// The bar's « Matière devant »: what it reads, whether it opens, and the
/// three states it offers in its menu.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MatterInFrontMenu {
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
