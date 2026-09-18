//! What part · file_name.rs is held to.

use super::*;

use crate::adapters::InMemoryFiles;

#[test]
fn a_name_a_filesystem_would_refuse_becomes_one_it_accepts() {
    assert_eq!(sanitize("Arm/left"), "Arm_left");
    assert_eq!(sanitize(" Support 12 "), "Support 12");
}

#[test]
fn a_name_that_is_only_blanks_is_refused_rather_than_replaced_here() {
    let files = InMemoryFiles::default();
    assert!(matches!(
        free_in(&files, Path::new("/parts"), "   "),
        Err(PartFileError::BlankName)
    ));
}
