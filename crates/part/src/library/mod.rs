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

/// What a path is called on screen: a part without its extension, a folder
/// without the road that leads to it.
fn name_of(path: &Path) -> String {
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
mod tests {
    use super::*;

    use crate::adapters::InMemoryFiles;
    use crate::ports::Files;

    fn library_of(paths: &[&str]) -> InMemoryFiles {
        let files = InMemoryFiles::default();
        for path in paths {
            files.write(Path::new(path), b"PK").expect("writes");
        }
        files
    }

    #[test]
    fn a_folder_hands_back_the_parts_it_holds_and_leaves_the_rest_alone() {
        let files = library_of(&[
            "/CAO/support.caopart",
            "/CAO/notes.txt",
            "/CAO/bride.caopart",
        ]);

        let library = read(&files, Path::new("/CAO")).expect("reads");

        let names: Vec<&str> = library
            .parts
            .iter()
            .map(|part| part.name.as_str())
            .collect();
        assert_eq!(names, ["bride", "support"]);
    }

    #[test]
    fn a_part_is_offered_under_the_name_its_file_carries_without_the_extension() {
        let files = library_of(&["/CAO/support 2.caopart"]);

        let library = read(&files, Path::new("/CAO")).expect("reads");

        assert_eq!(library.parts[0].name, "support 2");
        assert_eq!(
            library.parts[0].path,
            PathBuf::from("/CAO/support 2.caopart")
        );
    }

    #[test]
    fn a_subfolder_comes_with_everything_under_it() {
        let files = library_of(&["/CAO/drafts/first/support.caopart"]);

        let library = read(&files, Path::new("/CAO")).expect("reads");

        assert_eq!(library.folders[0].name, "drafts");
        assert_eq!(library.folders[0].folders[0].name, "first");
        assert_eq!(library.folders[0].folders[0].parts[0].name, "support");
    }

    #[test]
    fn folders_and_parts_each_come_in_the_order_a_reader_expects() {
        let files = library_of(&[
            "/CAO/zinc/part.caopart",
            "/CAO/Alpha/part.caopart",
            "/CAO/beta.caopart",
            "/CAO/Alpha.caopart",
        ]);

        let library = read(&files, Path::new("/CAO")).expect("reads");

        let folders: Vec<&str> = library
            .folders
            .iter()
            .map(|folder| folder.name.as_str())
            .collect();
        let parts: Vec<&str> = library
            .parts
            .iter()
            .map(|part| part.name.as_str())
            .collect();
        assert_eq!(
            folders,
            ["Alpha", "zinc"],
            "a capital does not send a folder to the top"
        );
        assert_eq!(parts, ["Alpha", "beta"]);
    }

    #[test]
    fn a_library_nobody_has_started_reads_as_an_empty_one() {
        let library = read(&InMemoryFiles::default(), Path::new("/CAO")).expect("reads");

        assert!(library.folders.is_empty());
        assert!(library.parts.is_empty());
        assert_eq!(library.name, "CAO");
    }
}
