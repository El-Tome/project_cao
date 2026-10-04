//! What following a name through the cuts of a replay is held to.
//!
//! Closes #345.
//! - a trait divided keeps one border, holding both of its pieces —
//!   `a_trait_cut_in_two_keeps_one_border_holding_both_pieces`
//! - a trait cut twice is followed in one step —
//!   `a_piece_cut_again_is_followed_past_the_piece`
//! - an area one of whose bounding traits is trimmed away raises nothing —
//!   `a_border_cut_away_altogether_loses_the_name`
//!
//! Closes #493.
//! - an area whose border runs along another curve keeps its name when either
//!   of the two is cut away, and loses it only with both —
//!   `a_border_two_curves_lie_along_is_lost_only_with_both`

use cao_sketch::SegmentId;
use glam::DVec2;

use super::*;

const INSIDE: DVec2 = DVec2::new(1.0, 1.0);

fn segment(id: usize) -> CurveId {
    CurveId::Segment(SegmentId(id))
}

fn area(bounds: &[usize]) -> Area {
    Area {
        bounds: bounds.iter().map(|id| vec![segment(*id)]).collect(),
        inside: INSIDE,
    }
}

fn shared(borders: &[&[usize]]) -> Area {
    Area {
        bounds: borders
            .iter()
            .map(|border| border.iter().copied().map(segment).collect())
            .collect(),
        inside: INSIDE,
    }
}

fn standing(borders: &[&[usize]]) -> Standing {
    Standing::new(
        borders
            .iter()
            .map(|border| border.iter().copied().map(segment).collect())
            .collect(),
        INSIDE,
    )
}

#[test]
fn a_name_nothing_was_cut_out_of_is_the_name_it_was() {
    let descent = Descent::default();

    assert_eq!(
        descent.follow(&area(&[0, 1, 2])),
        Some(standing(&[&[0], &[1], &[2]])),
    );
}

#[test]
fn a_trait_cut_in_two_keeps_one_border_holding_both_pieces() {
    let mut descent = Descent::default();
    descent.record(segment(1), vec![segment(7), segment(8)]);

    assert_eq!(
        descent.follow(&area(&[0, 1, 2])),
        Some(standing(&[&[0], &[7, 8], &[2]])),
        "one border, not two: an area that kept only one of the pieces kept \
         that border",
    );
}

#[test]
fn a_piece_cut_again_is_followed_past_the_piece() {
    let mut descent = Descent::default();
    descent.record(segment(1), vec![segment(7), segment(8)]);
    descent.record(segment(7), vec![segment(9), segment(10)]);

    assert_eq!(
        descent.follow(&area(&[1])),
        Some(standing(&[&[9, 10, 8]])),
        "the name speaks of trait 1, which is two cuts behind",
    );
}

#[test]
fn a_border_cut_away_altogether_loses_the_name() {
    let mut descent = Descent::default();
    descent.record(segment(1), Vec::new());

    assert_eq!(
        descent.follow(&area(&[0, 1, 2])),
        None,
        "dropping it from the name would hand the area to whatever larger \
         area swallowed it",
    );
}

#[test]
fn the_place_that_was_clicked_travels_with_the_name() {
    let mut descent = Descent::default();
    descent.record(segment(1), vec![segment(7)]);

    assert_eq!(descent.follow(&area(&[1])), Some(standing(&[&[7]])));
}

#[test]
fn a_border_two_curves_lie_along_is_lost_only_with_both() {
    let named = shared(&[&[0], &[1, 5], &[2]]);
    for (first, other) in [(1, 5), (5, 1)] {
        let mut descent = Descent::default();
        descent.record(segment(first), Vec::new());
        assert_eq!(
            descent.follow(&named),
            Some(standing(&[&[0], &[other], &[2]])),
            "trait {other} still runs along that border",
        );

        descent.record(segment(other), Vec::new());
        assert_eq!(descent.follow(&named), None);
    }
}
