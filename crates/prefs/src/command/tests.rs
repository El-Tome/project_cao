//! What prefs · command.rs is held to.

use super::*;

#[test]
fn no_family_comes_back_once_the_palette_has_left_it() {
    let mut headed: Vec<CommandFamily> = Vec::new();
    let mut current: Option<CommandFamily> = None;

    for command in Command::ALL {
        let family = command.family();
        if current == Some(family) {
            continue;
        }
        assert!(
            !headed.contains(&family),
            "{family:?} comes back after {current:?}: the palette heads a family \
             where it changes, so a family listed in two runs shows twice and the \
             user reads the same heading over a different half",
        );
        headed.push(family);
        current = Some(family);
    }
}
