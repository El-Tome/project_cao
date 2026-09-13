use cao_sketch::ArcMode;

use crate::lang::Catalogue;

/// The only place a way of drawing an arc is turned into an instruction.
///
/// `cao_sketch` knows what each mode needs and how many clicks that is; this
/// decides how the waiting is said while the tool asks for them.
pub fn asks_for(lang: &Catalogue, mode: ArcMode) -> String {
    lang.t(match mode {
        ArcMode::ByCenter => "arc.asks_for.by_center",
    })
}
