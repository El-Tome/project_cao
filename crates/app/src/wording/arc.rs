use cao_sketch::ArcMode;

use crate::lang::Catalogue;

/// The only place a way of drawing an arc is turned into an instruction.
///
/// `cao_sketch` knows what each mode needs and how many clicks that is; this
/// decides how the waiting is said while the tool asks for them.
pub fn asks_for(lang: &Catalogue, mode: ArcMode) -> String {
    lang.t(match mode {
        ArcMode::ByCenter => "arc.asks_for.by_center",
        ArcMode::ByEnds => "arc.asks_for.by_ends",
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn each_way_of_drawing_an_arc_asks_for_something_different() {
        let lang = Catalogue::french();

        assert_ne!(
            asks_for(&lang, ArcMode::ByCenter),
            asks_for(&lang, ArcMode::ByEnds),
            "two modes asking for the same thing leave the user guessing which they are in",
        );
    }
}
