//! What the part tree says that is decided by a case rather than read off a
//! name.

use cao_render::MatterInFront;

use crate::lang::Catalogue;

/// What a click on a sketch's eye would do, given whether it is lit.
pub fn eye(lang: &Catalogue, lit: bool) -> String {
    lang.t(match lit {
        true => "part_tree.eye.hide",
        false => "part_tree.eye.show",
    })
}

/// The « Corps » rubric, with how the body is drawn.
pub fn body(lang: &Catalogue, mode: MatterInFront) -> String {
    let mode = lang.t(match mode {
        MatterInFront::Hidden => "part_tree.body.hidden",
        MatterInFront::SeeThrough => "part_tree.body.see_through",
        MatterInFront::Opaque => "part_tree.body.opaque",
    });
    lang.t_with("part_tree.bodies_drawn", &[("mode", &mode)])
}
