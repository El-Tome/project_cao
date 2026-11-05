//! What the part tree says that is decided by a case rather than read off a
//! name.

use crate::lang::Catalogue;

/// What a click on a sketch's eye would do, given whether it is lit.
pub fn eye(lang: &Catalogue, lit: bool) -> String {
    lang.t(match lit {
        true => "part_tree.eye.hide",
        false => "part_tree.eye.show",
    })
}
