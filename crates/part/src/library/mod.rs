use std::path::{Path, PathBuf};

mod tidying;

pub use tidying::{create_folder, discard, rename_folder, rename_part};

use crate::file_name::PART_EXTENSION;
use crate::ports::{FileError, Folders};

/// A folder of the library, and everything under it.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Folder {
    pub path: PathBuf,
    pub name: String,
    pub folders: Vec<Folder>,
    pub parts: Vec<Part>,
}

/// A part file, named by what its file is called rather than by what the
/// archive says inside: browsing a hundred parts must not mean opening a
/// hundred archives, and `create_in` writes the two the same.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Part {
    pub path: PathBuf,
    pub name: String,
}

/// Everything the library holds under `root`, folders before parts and each
/// run in the order a reader expects.
///
/// A folder that cannot be read contributes nothing rather than stopping the
/// walk: one folder the system refuses is no reason to show none of the rest.
pub fn read(folders: &impl Folders, root: &Path) -> Result<Folder, FileError> {
    let mut folder = Folder {
        path: root.to_path_buf(),
        name: name_of(root),
        folders: Vec::new(),
        parts: Vec::new(),
    };

    for entry in folders.entries(root)? {
        if entry.folder {
            if let Ok(under) = read(folders, &entry.path) {
                folder.folders.push(under);
            }
        } else if is_a_part(&entry.path) {
            folder.parts.push(Part {
                name: name_of(&entry.path),
                path: entry.path,
            });
        }
    }

    folder
        .folders
        .sort_by(|a, b| in_reading_order(&a.name, &b.name));
    folder
        .parts
        .sort_by(|a, b| in_reading_order(&a.name, &b.name));
    Ok(folder)
}

fn is_a_part(path: &Path) -> bool {
    path.extension()
        .is_some_and(|extension| extension.eq_ignore_ascii_case(PART_EXTENSION))
}

/// What a path is called: a part without its extension, a folder without the
/// road that leads to it. The one place that decides, so the panel, the field
/// a rename starts from and a message about a name taken all agree.
pub fn name_of(path: &Path) -> String {
    let stem = if is_a_part(path) {
        path.file_stem()
    } else {
        path.file_name()
    };
    stem.map(|name| name.to_string_lossy().into_owned())
        .unwrap_or_else(|| path.display().to_string())
}

fn in_reading_order(one: &str, other: &str) -> std::cmp::Ordering {
    one.to_lowercase()
        .cmp(&other.to_lowercase())
        .then_with(|| one.cmp(other))
}

#[cfg(test)]
mod tests;
