#[cfg(test)]
use cao_part::Capped;

use super::*;

#[test]
fn a_count_brought_down_names_the_copy_that_would_land_on_the_original() {
    assert_eq!(
        count_brought_down(&Catalogue::french(), 4),
        "4 au plus : la 5e retomberait sur l'original"
    );
}

#[test]
fn a_capped_step_says_how_many_it_lays_at_its_angle_and_how_many_were_asked() {
    let capped = Capped {
        asked: 6,
        laid: 4,
        degrees: 90.0,
    };

    assert_eq!(
        step_capped(&Catalogue::french(), capped),
        "4 copies au plus à 90° ; 6 demandées"
    );
}
