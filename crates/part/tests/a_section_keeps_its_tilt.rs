//! A section kept in the part keeps the two angles it was tilted by, and a part
//! saved before sections could be tilted reads both as zero. The criteria of
//! #693 are transcribed in `crates/app/src/screens/viewport/render/tests.rs`,
//! which names the tests below.

use std::cell::RefCell;
use std::collections::HashMap;
use std::io::{Cursor, Write};
use std::path::{Path, PathBuf};

use cao_part::history::{
    ExtrusionMode, FaceAnchor, Operation, PointRef, SectionAsked, SideDrawing,
};
use cao_part::{FileError, Files, PartDocument};
use cao_sketch::WorkPlane;
use glam::{DVec2, DVec3};

#[derive(Default)]
struct Memory(RefCell<HashMap<PathBuf, Vec<u8>>>);

impl Files for Memory {
    fn read(&self, path: &Path) -> Result<Vec<u8>, FileError> {
        self.0
            .borrow()
            .get(path)
            .cloned()
            .ok_or_else(|| FileError::Absent(path.to_path_buf()))
    }

    fn write(&self, path: &Path, bytes: &[u8]) -> Result<(), FileError> {
        self.0
            .borrow_mut()
            .insert(path.to_path_buf(), bytes.to_vec());
        Ok(())
    }

    fn exists(&self, path: &Path) -> bool {
        self.0.borrow().contains_key(path)
    }
}

fn at_nine() -> chrono::DateTime<chrono::Utc> {
    "2026-11-06T09:00:00Z".parse().expect("a date")
}

#[test]
fn a_tilted_section_comes_back_from_the_caopart_with_its_two_angles() {
    let mut document = PartDocument::new("Bloc", at_nine());
    document.apply(Operation::SaveSection(SectionAsked {
        plane: WorkPlane::YZ,
        on: None,
        offset: 15.0,
        behind: SideDrawing::Opaque,
        in_front: SideDrawing::Hidden,
        tilt_about_u_degrees: 30.0,
        tilt_about_v_degrees: -12.5,
    }));
    let files = Memory::default();
    let saved = Path::new("/parts/saved.caopart");
    let put_away = Path::new("/parts/put-away.caopart");
    document
        .save(&files, saved, at_nine())
        .expect("the part is saved");
    document
        .put_away(&files, put_away, at_nine())
        .expect("the part is put away");

    for path in [saved, put_away] {
        let reopened = PartDocument::load(&files, path).expect("the part opens again");
        let sections = reopened.sections();

        assert_eq!(sections.len(), 1, "{path:?} came back with {sections:?}");
        assert_eq!(
            (
                sections[0].tilt_about_u_degrees,
                sections[0].tilt_about_v_degrees
            ),
            (30.0, -12.5),
            "{path:?}: the section tilted by 30° and -12.5° came back tilted otherwise",
        );
    }
}

/// The archive a part with one section on YZ, 15 along its normal, was written
/// as before sections could be tilted: the files of
/// `tests/fixtures/a_section_saved_before_693/`, the cache of its geometry
/// with them or not.
fn a_part_saved_before_the_angles(with_its_geometry: bool) -> Vec<u8> {
    let folder = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("fixtures")
        .join("a_section_saved_before_693");
    let mut entries = vec![
        "part.json",
        "design/history.json",
        "design/section-0/steps.json",
    ];
    if with_its_geometry {
        entries.push("geometry.json");
    }
    let mut archive = zip::ZipWriter::new(Cursor::new(Vec::new()));
    let options = zip::write::SimpleFileOptions::default()
        .compression_method(zip::CompressionMethod::Deflated);
    for entry in entries {
        let bytes = std::fs::read(folder.join(entry))
            .unwrap_or_else(|err| panic!("the fixture {entry} does not read: {err}"));
        archive.start_file(entry, options).expect("an entry opens");
        archive.write_all(&bytes).expect("an entry is written");
    }
    archive.finish().expect("the archive closes").into_inner()
}

#[test]
fn a_section_saved_before_its_angles_existed_reads_both_as_zero_and_cuts_where_it_did() {
    for with_its_geometry in [false, true] {
        let files = Memory::default();
        let path = Path::new("/parts/before-693.caopart");
        files
            .write(path, &a_part_saved_before_the_angles(with_its_geometry))
            .expect("the fixture is laid down");

        let part = PartDocument::load(&files, path).unwrap_or_else(|err| {
            panic!(
                "a part saved before #693, its geometry kept {with_its_geometry}, \
                 no longer opens: {err:?}"
            )
        });
        let sections = part.sections();

        assert_eq!(
            sections.len(),
            1,
            "the part came back with {sections:?}, its geometry kept {with_its_geometry}",
        );
        assert_eq!(
            (
                sections[0].tilt_about_u_degrees,
                sections[0].tilt_about_v_degrees
            ),
            (0.0, 0.0),
            "a section saved before #693 came back tilted, its geometry kept {with_its_geometry}",
        );
        assert_eq!(
            sections[0].cutting_plane(),
            Some(WorkPlane {
                origin: WorkPlane::YZ.origin + WorkPlane::YZ.normal() * 15.0,
                ..WorkPlane::YZ
            }),
            "a section saved before #693 no longer cuts where it did, its geometry kept \
             {with_its_geometry}",
        );
    }
}

/// The face the top of the block is, as the replay numbers it.
const TOP: usize = 1;

/// A block 40 by 20 drawn on XY, raised 10.
fn block() -> PartDocument {
    let mut document = PartDocument::new("Bloc", at_nine());
    document.apply(Operation::CreateSketch {
        plane: WorkPlane::XY,
        on: None,
    });
    document.apply(Operation::AddRectangle {
        sketch: 0,
        corner: PointRef::New(DVec2::ZERO),
        opposite: PointRef::New(DVec2::new(40.0, 20.0)),
        construction: false,
    });
    let areas = document.areas_at(0, &[DVec2::new(20.0, 10.0)]);
    document.apply(Operation::Extrude {
        sketch: 0,
        areas,
        distance: 10.0.into(),
        mode: ExtrusionMode::Add,
    });
    document
}

#[test]
fn a_kept_section_on_a_face_turns_about_the_point_of_the_face_under_the_middle_of_the_part() {
    let mut document = block();
    let top = document.body().plane_of(TOP).expect("the block has a top");
    assert!(
        top.normal.abs_diff_eq(DVec3::Z, 1e-9),
        "face {TOP} is not the top of the block: its normal is {:?}",
        top.normal,
    );
    let under_the_middle = DVec3::new(20.0, 10.0, 10.0);
    document.apply(Operation::SaveSection(SectionAsked {
        plane: WorkPlane::from_face(&top.corners, top.normal, DVec3::Y),
        on: Some(FaceAnchor {
            face: TOP,
            up: DVec3::Y,
        }),
        offset: -1.0,
        behind: SideDrawing::Opaque,
        in_front: SideDrawing::Hidden,
        tilt_about_u_degrees: 10.0,
        tilt_about_v_degrees: 0.0,
    }));
    let files = Memory::default();
    let path = Path::new("/parts/tilted-on-its-top.caopart");
    document
        .save(&files, path, at_nine())
        .expect("the part is saved");

    let reopened = PartDocument::load(&files, path).expect("the part opens again");
    let cut = reopened.sections()[0]
        .cutting_plane()
        .expect("a section whose face stands cuts somewhere");

    let normal = cut.normal();
    assert!(
        (normal.dot(DVec3::Z) - 10_f64.to_radians().cos()).abs() < 1e-9,
        "the section on the top tilted 10° faces {normal:?}",
    );
    assert!(
        (normal.dot(cut.origin - under_the_middle) + 1.0).abs() < 1e-9,
        "the section on the top, tilted 10° and moved -1, cuts {} from the point of the top \
         under the middle of the block, {under_the_middle:?}, along its normal, not -1",
        normal.dot(cut.origin - under_the_middle),
    );
}
