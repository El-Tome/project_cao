//! A part written by an older `zip` crate opens, whole, under the one the
//! reader is on now. The fixture `tests/fixtures/a_part_saved_before_880.caopart`
//! was written by `PartDocument::put_away` while `cao_part` was on `zip` 8.6.0,
//! and is kept as those bytes: it is never to be written again.
//!
//! Closes #880.
//! - `zip` is on 9.x (at least 9.0.5) in `crates/part/Cargo.toml` and
//!   `Cargo.lock`; the Dependabot alerts close — no test: the version is read
//!   in the diff of both files, and the alerts close on GitHub, which nothing
//!   in this workspace reads
//! - every existing part test passes unchanged, including the ones that build
//!   archives by hand (`a_section_keeps_its_tilt`, `changing_an_extrusion`) —
//!   no test: it is the suite of `cao_part` as it stands, and the diff changes
//!   no assertion of it. Five helpers that list or copy the entries of an
//!   archive, in `document/tests.rs`, `document/design/tests.rs`,
//!   `document/geometry_cache/tests.rs` and `changing_an_extrusion.rs`, read an
//!   entry's name through zip 9's API, where a name is a `Result`; what they
//!   hand back is unchanged. The archives built by hand are in
//!   `a_section_keeps_its_tilt.rs`, `changing_an_extrusion.rs`,
//!   `document/tests.rs` and `document/geometry_cache/tests.rs`
//! - a part saved before the bump opens after it —
//!   `a_part_written_by_zip_8_opens_and_replays_to_the_same_bracket`,
//!   `a_part_written_by_zip_8_hands_back_its_picture_whichever_way_it_is_read`
//! - the reader still reads entries by name and enables nothing beyond
//!   `deflate` — no test: the calls to `by_name` in `document.rs` and the
//!   features of `zip` in `crates/part/Cargo.toml` are read in the diff
//!
//! What a lost `geometry.json` would cost cannot be seen from here: a part
//! whose cache does not read is replayed to the same body, by design.

use std::path::{Path, PathBuf};

use cao_part::{FileError, Files, PartDocument, Picture};
use glam::DVec3;

struct Fixtures;

impl Files for Fixtures {
    fn read(&self, path: &Path) -> Result<Vec<u8>, FileError> {
        std::fs::read(path).map_err(|_| FileError::Absent(path.to_path_buf()))
    }

    fn write(&self, path: &Path, _bytes: &[u8]) -> Result<(), FileError> {
        panic!("a fixture is never written again, and {path:?} was about to be")
    }

    fn exists(&self, path: &Path) -> bool {
        path.exists()
    }
}

fn saved_before_880() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("fixtures")
        .join("a_part_saved_before_880.caopart")
}

/// Exact surfaces throughout: a block and a round hole, no faceting.
const TOLERANCE: f64 = 1e-9;

/// `thickness` at 10; a rectangle 40 by 20 on XY raised by `thickness`; a
/// circle of radius 5 at its middle on XY, cut 10 through it.
#[test]
fn a_part_written_by_zip_8_opens_and_replays_to_the_same_bracket() {
    let part = PartDocument::load(&Fixtures, &saved_before_880())
        .unwrap_or_else(|err| panic!("a part written by zip 8.6.0 no longer opens: {err:?}"));

    assert_eq!(part.name(), "Bracket");
    assert_eq!(
        part.metadata.id.to_string(),
        "f2472572-0ee5-41c2-81a1-0be099bf2432",
        "part.json came back as another part",
    );
    assert_eq!(
        part.metadata.created_at,
        "2026-11-11T09:00:00Z"
            .parse::<chrono::DateTime<chrono::Utc>>()
            .expect("a date"),
    );
    assert_eq!(
        part.history.operations().len(),
        7,
        "the history came back as {:?}",
        part.history.operations(),
    );
    assert_eq!(part.sketches().len(), 2, "both sketches are in the design");

    let (_, thickness) = part
        .variables()
        .size_of("thickness")
        .expect("design/variables.json holds `thickness`");
    assert!(
        (thickness - 10.0).abs() < TOLERANCE,
        "thickness expected 10, got {thickness}",
    );

    let volume = part.body().volume();
    let block_less_hole = 40.0 * 20.0 * 10.0 - std::f64::consts::PI * 5.0 * 5.0 * 10.0;
    assert!(
        (volume - block_less_hole).abs() < TOLERANCE,
        "the bracket expected {block_less_hole} mm³ once holed, got {volume}",
    );
    let (low, high) = part.body().bounds().expect("the bracket has matter");
    assert!(
        low.abs_diff_eq(DVec3::ZERO, TOLERANCE)
            && high.abs_diff_eq(DVec3::new(40.0, 20.0, 10.0), TOLERANCE),
        "the bracket expected from the origin to (40, 20, 10), got {low:?} to {high:?}",
    );
}

#[test]
fn a_part_written_by_zip_8_hands_back_its_picture_whichever_way_it_is_read() {
    let expected = Picture::new(2, 2, (0u8..16).map(|byte| byte * 16 + 1).collect())
        .expect("sixteen bytes fill two by two");

    let opened = PartDocument::load(&Fixtures, &saved_before_880())
        .unwrap_or_else(|err| panic!("a part written by zip 8.6.0 no longer opens: {err:?}"));
    let pulled_out = PartDocument::picture_in(&Fixtures, &saved_before_880());

    assert_eq!(opened.picture(), Some(&expected), "opening the part");
    assert_eq!(
        pulled_out.as_ref(),
        Some(&expected),
        "pulling the picture out on its own"
    );
}
