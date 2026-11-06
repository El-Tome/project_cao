//! A section the user keeps belongs to the part: saved with it, following the
//! face it was laid on, carrying the sketches made on it — and never touching
//! the matter. The criteria of #692 are transcribed in
//! `crates/app/tests/keeping_a_section.rs`, which names the tests below.

use std::cell::RefCell;
use std::collections::HashMap;
use std::path::{Path, PathBuf};

use cao_part::feature::Feature;
use cao_part::history::{
    ExtrusionMode, FaceAnchor, Operation, PointRef, SectionAsked, SideDrawing,
};
use cao_part::{FileError, Files, Formula, PartDocument, VariableChange, VariableId};
use cao_sketch::{Element, SegmentId, WorkPlane};
use glam::{DVec2, DVec3};

const TOLERANCE: f64 = 1e-6;

/// The face the top of the block is, as the replay numbers it.
const TOP: usize = 1;

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

/// A block 40 by 20 drawn on XY, raised as thick as the variable `thickness`.
fn block(thickness: f64) -> PartDocument {
    let mut document = PartDocument::new("Bloc", at_nine());
    document.apply(Operation::Variable(VariableChange::Added {
        name: "thickness".to_string(),
        formula: Formula::Number(thickness),
    }));
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
    let distance = document
        .variables()
        .read("thickness")
        .expect("a formula that reads");
    document.apply(Operation::Extrude {
        sketch: 0,
        areas,
        distance,
        mode: ExtrusionMode::Add,
    });
    document
}

fn set_thickness(document: &mut PartDocument, value: f64) {
    document.apply(Operation::Variable(VariableChange::Edited {
        variable: VariableId(0),
        name: "thickness".to_string(),
        formula: Formula::Number(value),
    }));
}

/// A section along XY, moved `offset` up its normal.
fn across(offset: f64) -> SectionAsked {
    SectionAsked {
        plane: WorkPlane::XY,
        on: None,
        offset,
        behind: SideDrawing::Opaque,
        in_front: SideDrawing::Hidden,
    }
}

/// A section laid on the top of a block `thickness` tall, moved `offset` along
/// the face's outward normal.
fn on_the_top(thickness: f64, offset: f64) -> SectionAsked {
    SectionAsked {
        plane: WorkPlane {
            origin: DVec3::new(0.0, 0.0, thickness),
            u: DVec3::X,
            v: DVec3::Y,
        },
        on: Some(FaceAnchor {
            face: TOP,
            up: DVec3::Y,
        }),
        offset,
        behind: SideDrawing::SeeThrough,
        in_front: SideDrawing::Hidden,
    }
}

fn the_rectangle_of_the_block() -> Operation {
    Operation::EraseMany {
        sketch: 0,
        elements: (0..4)
            .map(|side| Element::Segment(SegmentId(side)))
            .collect(),
        dimensions: Vec::new(),
        constraints: Vec::new(),
    }
}

fn cut_height(document: &PartDocument) -> f64 {
    let sections = document.sections();
    sections[0]
        .cutting_plane()
        .expect("a section whose face stands cuts somewhere")
        .origin
        .z
}

#[test]
fn every_section_comes_back_from_the_caopart_with_its_plane_its_offset_and_its_sides() {
    let mut document = block(10.0);
    document.apply(Operation::SaveSection(SectionAsked {
        plane: WorkPlane::YZ,
        on: None,
        offset: 15.0,
        behind: SideDrawing::SeeThrough,
        in_front: SideDrawing::Opaque,
    }));
    document.apply(Operation::SaveSection(on_the_top(10.0, -4.0)));
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

        assert_eq!(sections.len(), 2, "{path:?} came back with {sections:?}");
        assert_eq!(sections[0].plane, WorkPlane::YZ, "{path:?}");
        assert!(
            (sections[0].offset - 15.0).abs() < TOLERANCE,
            "{path:?}: the first section came back {} from its plane",
            sections[0].offset,
        );
        assert_eq!(
            (sections[0].behind, sections[0].in_front),
            (SideDrawing::SeeThrough, SideDrawing::Opaque),
            "{path:?}",
        );
        assert!(
            (sections[1].plane.origin.z - 10.0).abs() < TOLERANCE,
            "{path:?}: the section on the top came back on {:?}",
            sections[1].plane,
        );
        assert!(
            (sections[1].offset + 4.0).abs() < TOLERANCE,
            "{path:?}: the section on the top came back {} from it",
            sections[1].offset,
        );
        assert_eq!(
            (sections[1].behind, sections[1].in_front),
            (SideDrawing::SeeThrough, SideDrawing::Hidden),
            "{path:?}",
        );
        assert!(!sections[1].broken, "{path:?}: the top is still there");
    }
}

#[test]
fn a_section_on_a_face_moves_with_the_face_when_a_size_changes() {
    let mut document = block(10.0);
    let top = document.body().plane_of(TOP).expect("the block has a top");
    assert!(
        top.normal.abs_diff_eq(DVec3::Z, 1e-9),
        "face {TOP} is not the top of the block: its normal is {:?}",
        top.normal,
    );
    document.apply(Operation::SaveSection(on_the_top(10.0, -4.0)));
    let before = cut_height(&document);

    set_thickness(&mut document, 30.0);

    assert!(
        (before - 6.0).abs() < TOLERANCE,
        "4 below the top of a block 10 tall, the cut stood at {before}",
    );
    assert!(
        (cut_height(&document) - 26.0).abs() < TOLERANCE,
        "the block grew to 30 and the cut stayed at {}",
        cut_height(&document),
    );
}

#[test]
fn a_section_whose_face_is_gone_is_broken_cuts_nothing_and_another_plane_mends_it() {
    let mut document = block(10.0);
    document.apply(Operation::SaveSection(on_the_top(10.0, -4.0)));

    document.apply(the_rectangle_of_the_block());

    assert!(
        document.body().bounds().is_none(),
        "the extrusion still raises matter once its rectangle is erased",
    );
    let sections = document.sections();
    assert_eq!(
        sections.len(),
        1,
        "a section that lost its face is still kept"
    );
    assert!(
        sections[0].broken,
        "a section that lost its face is not marked"
    );
    assert_eq!(
        sections[0].cutting_plane(),
        None,
        "a section that lost its face still cuts",
    );
    let section = sections[0].id;

    document.apply(Operation::EditSection {
        section,
        to: SectionAsked {
            plane: WorkPlane::XZ,
            on: None,
            offset: 5.0,
            behind: SideDrawing::Opaque,
            in_front: SideDrawing::Hidden,
        },
    });

    let sections = document.sections();
    let mended = &sections[0];
    assert!(
        !mended.broken,
        "a section moved onto XZ is still marked broken"
    );
    assert_eq!(
        mended.cutting_plane().map(|cut| cut.origin),
        Some(WorkPlane::XZ.origin + WorkPlane::XZ.normal() * 5.0),
        "the mended section cuts 5 along XZ",
    );
}

#[test]
fn a_saved_section_leaves_the_volume_and_the_stl_as_they_were() {
    let whole = block(10.0);
    let mut kept = block(10.0);

    kept.apply(Operation::SaveSection(across(5.0)));

    assert_eq!(kept.sections().len(), 1, "the section was not kept");
    assert!(
        (kept.body().volume() - whole.body().volume()).abs() < TOLERANCE,
        "a section took the volume from {} to {}",
        whole.body().volume(),
        kept.body().volume(),
    );
    let stl = |document: &PartDocument| {
        cao_solid::stl::of(document.body(), 0.02, document.scale()).expect("an STL")
    };
    assert!(
        stl(&kept) == stl(&whole),
        "the STL of a part holding a section differs from the whole part's",
    );
}

#[test]
fn a_sketch_on_a_section_moves_with_its_offset_and_what_was_cut_from_it_is_rebuilt() {
    let mut document = block(10.0);
    document.apply(Operation::SaveSection(across(4.0)));
    let section = document.sections()[0].id;
    document.apply(Operation::CreateSketchOnSection {
        section,
        plane: WorkPlane {
            origin: DVec3::new(0.0, 0.0, 4.0),
            u: DVec3::X,
            v: DVec3::Y,
        },
    });
    document.apply(Operation::AddRectangle {
        sketch: 1,
        corner: PointRef::New(DVec2::new(10.0, 5.0)),
        opposite: PointRef::New(DVec2::new(30.0, 15.0)),
        construction: false,
    });
    let areas = document.areas_at(1, &[DVec2::new(20.0, 10.0)]);
    document.apply(Operation::Extrude {
        sketch: 1,
        areas,
        distance: 10.0.into(),
        mode: ExtrusionMode::Cut,
    });
    let pocketed_from_4 = document.body().volume();

    document.apply(Operation::EditSection {
        section,
        to: across(6.0),
    });

    assert!(
        (pocketed_from_4 - (8000.0 - 1200.0)).abs() < TOLERANCE,
        "a pocket from 4 up through a block 10 tall left {pocketed_from_4}",
    );
    assert!(
        (document.sketches()[1].plane.origin.z - 6.0).abs() < TOLERANCE,
        "the section moved to 6 and the sketch on it stayed on {:?}",
        document.sketches()[1].plane,
    );
    assert!(
        (document.body().volume() - (8000.0 - 800.0)).abs() < TOLERANCE,
        "the pocket's floor followed the section to 6: the block holds {}",
        document.body().volume(),
    );
}

#[test]
fn a_sketch_on_a_section_that_lost_its_face_is_adrift_where_it_was() {
    let mut document = block(10.0);
    document.apply(Operation::SaveSection(on_the_top(10.0, -4.0)));
    let section = document.sections()[0].id;
    let where_it_was = WorkPlane {
        origin: DVec3::new(0.0, 0.0, 6.0),
        u: DVec3::X,
        v: DVec3::Y,
    };
    document.apply(Operation::CreateSketchOnSection {
        section,
        plane: where_it_was,
    });

    document.apply(the_rectangle_of_the_block());

    assert!(
        document.is_adrift(1),
        "a sketch whose section lost its face is not marked",
    );
    assert!(
        (document.sketches()[1].plane.origin.z - 6.0).abs() < TOLERANCE,
        "a sketch whose section lost its face moved to {:?}",
        document.sketches()[1].plane,
    );
}

#[test]
fn undo_takes_back_making_editing_and_deleting_a_section() {
    let mut document = block(10.0);

    document.apply(Operation::SaveSection(across(4.0)));
    document.undo();
    let after_undoing_the_making = document.sections().len();
    document.redo();
    let section = document.sections()[0].id;
    document.apply(Operation::EditSection {
        section,
        to: across(7.0),
    });
    document.undo();
    let after_undoing_the_edit = document.sections()[0].offset;
    document.redo();
    document.apply(Operation::DeleteSection { section });
    let after_deleting = document.sections().len();
    document.undo();

    assert_eq!(after_undoing_the_making, 0, "undo left the section made");
    assert!(
        (after_undoing_the_edit - 4.0).abs() < TOLERANCE,
        "undo left the section edited, {after_undoing_the_edit} from its plane",
    );
    assert_eq!(after_deleting, 0, "deleting left the section in the part");
    let sections = document.sections();
    assert_eq!(sections.len(), 1, "undo left the section deleted");
    assert!(
        (sections[0].offset - 7.0).abs() < TOLERANCE,
        "the section came back {} from its plane, not as it was edited",
        sections[0].offset,
    );
}

#[test]
fn a_section_never_enters_the_list_of_what_was_done() {
    let mut document = block(10.0);
    let done = Feature::all(&document.history);

    document.apply(Operation::SaveSection(across(4.0)));
    let section = document.sections()[0].id;
    document.apply(Operation::EditSection {
        section,
        to: across(6.0),
    });
    document.apply(Operation::DeleteSection { section });

    assert_eq!(Feature::all(&document.history), done);
}
