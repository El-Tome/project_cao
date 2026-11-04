use cao_render::MatterInFront;

use crate::lang::Catalogue;

/// What the button stepping the matter in front of the sketch reads: the
/// state it is in now.
pub fn in_front(lang: &Catalogue, state: MatterInFront) -> String {
    lang.t(match state {
        MatterInFront::Hidden => "command.label.matter_in_front.hidden",
        MatterInFront::SeeThrough => "command.label.matter_in_front.see_through",
        MatterInFront::Opaque => "command.label.matter_in_front.opaque",
    })
}

/// The same, in the two letters a button keeps with labels off.
pub fn in_front_short(lang: &Catalogue, state: MatterInFront) -> String {
    lang.t(match state {
        MatterInFront::Hidden => "command.short.matter_in_front.hidden",
        MatterInFront::SeeThrough => "command.short.matter_in_front.see_through",
        MatterInFront::Opaque => "command.short.matter_in_front.opaque",
    })
}
