use std::path::Path;

use cao_prefs::Locations;
use directories::{ProjectDirs, UserDirs};

/// Asks the platform, once, where it keeps things.
///
/// `None` when it offers no home directory to build the paths from — there is
/// then nowhere to keep the settings, and the shell says so rather than
/// writing them somewhere nobody would find them again.
pub fn discover() -> Option<Locations> {
    let project = ProjectDirs::from("dev", "cao", "cao")?;
    Some(Locations {
        config: project.config_dir().to_path_buf(),
        data: project.data_dir().to_path_buf(),
        documents: UserDirs::new().and_then(|user| user.document_dir().map(Path::to_path_buf)),
    })
}
