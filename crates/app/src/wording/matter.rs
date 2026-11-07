use cao_render::MatterInFront;

use crate::lang::Catalogue;
use crate::ui::icon::{self, Icon};

/// What the button stepping the matter in front of the sketch reads: the
/// state it is in now.
pub fn in_front(lang: &Catalogue, state: MatterInFront) -> String {
    lang.t(match state {
        MatterInFront::Hidden => "command.label.matter_in_front.hidden",
        MatterInFront::SeeThrough => "command.label.matter_in_front.see_through",
        MatterInFront::Opaque => "command.label.matter_in_front.opaque",
    })
}

/// The same, drawn: the one of its three icons the redesign gives the state.
pub fn in_front_icon(state: MatterInFront) -> Icon {
    match state {
        MatterInFront::Hidden => icon::MATTER_IN_FRONT_HIDDEN,
        MatterInFront::SeeThrough => icon::MATTER_IN_FRONT_SEE_THROUGH,
        MatterInFront::Opaque => icon::MATTER_IN_FRONT_OPAQUE,
    }
}
