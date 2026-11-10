//! What the part tree says that is decided by a case rather than read off a
//! name.

use cao_render::MatterInFront;

use crate::lang::Catalogue;
use crate::wording::dimension::short;

/// One heading of the panel, each over its own kind of row.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rubric {
    Sketches,
    Volumes,
    Sections,
    Variables,
}

pub fn rubric(lang: &Catalogue, rubric: Rubric) -> String {
    lang.t(match rubric {
        Rubric::Sketches => "part_tree.rubric.sketches",
        Rubric::Volumes => "part_tree.rubric.volumes",
        Rubric::Sections => "part_tree.rubric.sections",
        Rubric::Variables => "part_tree.rubric.variables",
    })
}

/// What is wrong with a row of the panel, said in a badge beside its name.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Problem {
    Unstable,
    FaceLost,
    NotBuilt,
    AreaLost,
}

pub fn badge(lang: &Catalogue, problem: Problem) -> String {
    lang.t(match problem {
        Problem::Unstable => "part_tree.badge.unstable",
        Problem::FaceLost => "part_tree.badge.face_lost",
        Problem::NotBuilt => "part_tree.badge.not_built",
        Problem::AreaLost => "part_tree.badge.area_lost",
    })
}

/// A trait's length, in millimetres, beside its row.
pub fn length(lang: &Catalogue, millimeters: f64) -> String {
    lang.t_with("part_tree.length", &[("value", &short(millimeters))])
}

/// A circle's diameter, in millimetres, beside its row.
pub fn diameter(lang: &Catalogue, millimeters: f64) -> String {
    lang.t_with("part_tree.diameter", &[("value", &short(millimeters))])
}

/// What a click on a sketch's eye would do, given whether it is lit.
pub fn eye(lang: &Catalogue, lit: bool) -> String {
    lang.t(match lit {
        true => "part_tree.eye.hide",
        false => "part_tree.eye.show",
    })
}

/// How a volume is drawn, written on its line.
pub fn volume_state(lang: &Catalogue, mode: MatterInFront) -> String {
    lang.t(match mode {
        MatterInFront::Hidden => "part_tree.volume.hidden",
        MatterInFront::SeeThrough => "part_tree.volume.see_through",
        MatterInFront::Opaque => "part_tree.volume.opaque",
    })
}

/// What a click on a volume's visibility button does, given how it is drawn:
/// opaque, see-through, invisible, and round again.
pub fn visibility_button(lang: &Catalogue, mode: MatterInFront) -> String {
    lang.t(match mode {
        MatterInFront::Opaque => "part_tree.volume.make_see_through",
        MatterInFront::SeeThrough => "part_tree.volume.make_hidden",
        MatterInFront::Hidden => "part_tree.volume.make_opaque",
    })
}
