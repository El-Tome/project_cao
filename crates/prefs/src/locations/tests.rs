//! What prefs · locations.rs is held to.

use super::*;

fn platform(documents: Option<&str>) -> Locations {
    Locations {
        config: "/config/cao".into(),
        data: "/data/cao".into(),
        documents: documents.map(Into::into),
    }
}

#[test]
fn a_platform_with_a_documents_folder_keeps_the_parts_in_it() {
    let at = platform(Some("/home/tom/Documents"));

    assert_eq!(
        default_projects_dir(&at),
        PathBuf::from("/home/tom/Documents/CAO"),
    );
}

#[test]
fn a_platform_without_one_keeps_the_parts_beside_its_own_data() {
    let at = platform(None);

    assert_eq!(
        default_projects_dir(&at),
        PathBuf::from("/data/cao/projects")
    );
}
