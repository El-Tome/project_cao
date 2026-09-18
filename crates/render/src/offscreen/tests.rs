//! What render · offscreen.rs is held to.

use super::*;

#[test]
fn a_row_already_on_the_boundary_is_left_where_it_is() {
    assert_eq!(padded_row(64), 256);
    assert_eq!(padded_row(128), 512);
    assert_eq!(padded_row(1024), 4096);
}

#[test]
fn a_row_that_falls_short_is_carried_to_the_next_boundary() {
    assert_eq!(padded_row(1), 256, "four bytes still occupy a whole row");
    assert_eq!(padded_row(65), 512);
    assert_eq!(padded_row(100), 512);
}

#[test]
fn a_padded_row_is_never_narrower_than_the_pixels_it_carries() {
    for width in 1..600u32 {
        assert!(
            padded_row(width) >= width * 4,
            "a row of {width} pixels is copied into fewer bytes than it holds",
        );
        assert_eq!(padded_row(width) % wgpu::COPY_BYTES_PER_ROW_ALIGNMENT, 0);
    }
}

#[test]
fn a_picture_says_how_many_bytes_its_pixels_make() {
    let size = Size {
        width: 128,
        height: 128,
        format: wgpu::TextureFormat::Rgba8UnormSrgb,
    };

    assert_eq!(size.bytes(), 65_536);
}
