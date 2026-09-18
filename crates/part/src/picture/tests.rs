//! What part · picture.rs is held to.

use super::*;

#[test]
fn a_picture_holds_four_bytes_for_every_pixel_of_its_size() {
    let picture = Picture::new(2, 3, vec![0; 24]).expect("the pixels fill the size");

    assert_eq!(picture.width(), 2);
    assert_eq!(picture.height(), 3);
    assert_eq!(picture.pixels().len(), 24);
}

#[test]
fn a_picture_read_back_short_is_refused_rather_than_drawn_from_whatever_follows_it() {
    assert_eq!(Picture::new(2, 3, vec![0; 23]), None);
    assert_eq!(Picture::new(2, 3, vec![0; 25]), None);
}

#[test]
fn a_picture_of_no_size_at_all_is_refused() {
    assert_eq!(Picture::new(0, 128, Vec::new()), None);
    assert_eq!(Picture::new(128, 0, Vec::new()), None);
}
