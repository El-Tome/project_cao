use cao_sketch::CircleMode;

/// The only place a way of drawing a circle is turned into an instruction.
///
/// `cao_sketch` knows what each mode needs and how many clicks that is; this
/// decides how the waiting is said while the tool asks for them.
pub fn asks_for(mode: CircleMode) -> &'static str {
    match mode {
        CircleMode::Center => "Cliquez le centre, puis un point du bord",
        CircleMode::TwoPoints => "Cliquez deux points opposés du bord",
        CircleMode::ThreePoints => "Cliquez deux points du bord, puis le centre",
        CircleMode::TwoTangents => "Cliquez deux droites, puis le centre",
        CircleMode::ThreeTangents => "Cliquez trois droites",
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeSet;

    #[test]
    fn each_way_of_drawing_a_circle_asks_for_something_different() {
        let modes = [
            CircleMode::Center,
            CircleMode::TwoPoints,
            CircleMode::ThreePoints,
            CircleMode::TwoTangents,
            CircleMode::ThreeTangents,
        ];
        let said: BTreeSet<&str> = modes.iter().map(|mode| asks_for(*mode)).collect();

        assert_eq!(
            said.len(),
            modes.len(),
            "two modes asking for the same thing leave the user guessing which they are in"
        );
    }
}
