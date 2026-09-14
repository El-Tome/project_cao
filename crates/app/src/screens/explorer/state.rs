use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use cao_part::library::{self, Folder};
use cao_part::{Files, Folders, PartFileError};

/// A name being typed, either over one that is already there or for a folder
/// that does not exist yet.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Naming {
    NewFolder { inside: PathBuf, typed: String },
    Rename { path: PathBuf, typed: String },
}

impl Naming {
    pub fn typed_mut(&mut self) -> &mut String {
        match self {
            Self::NewFolder { typed, .. } | Self::Rename { typed, .. } => typed,
        }
    }
}

/// What the panel knows about the library, and what the user is part-way
/// through doing to it.
///
/// Folders are open unless they were folded shut: a panel that starts closed
/// shows the user nothing they came for.
pub struct Explorer {
    root: PathBuf,
    library: Folder,
    folded: BTreeSet<PathBuf>,
    selected: Option<PathBuf>,
    naming: Option<Naming>,
    confirming: Option<PathBuf>,
    trouble: Option<PartFileError>,
    /// The part this window has open, which cannot be renamed or thrown away
    /// from here: it is being written to behind the panel's back.
    pub in_use: Option<PathBuf>,
    stale: bool,
}

impl Explorer {
    pub fn at(root: PathBuf) -> Self {
        Self {
            library: Folder {
                path: root.clone(),
                name: String::new(),
                folders: Vec::new(),
                parts: Vec::new(),
            },
            root,
            folded: BTreeSet::new(),
            selected: None,
            naming: None,
            confirming: None,
            trouble: None,
            in_use: None,
            stale: true,
        }
    }

    pub fn library(&self) -> &Folder {
        &self.library
    }

    pub fn refresh(&mut self, folders: &impl Folders) {
        self.stale = false;
        match library::read(folders, &self.root) {
            Ok(library) => self.library = library,
            Err(fate) => self.trouble = Some(PartFileError::File(fate)),
        }
    }

    /// Says the library on the disk has moved on without the panel — a part
    /// was made, or the user asked for a fresh read.
    pub fn went_stale(&mut self) {
        self.stale = true;
    }

    /// Walking the whole library is too much to do every frame, so it is done
    /// when something is known to have changed and when the panel opens.
    pub fn refresh_if_stale(&mut self, folders: &impl Folders) {
        if self.stale {
            self.refresh(folders);
        }
    }

    pub fn selected(&self) -> Option<&Path> {
        self.selected.as_deref()
    }

    pub fn select(&mut self, path: &Path) {
        self.selected = Some(path.to_path_buf());
    }

    pub fn unfolded(&self, folder: &Path) -> bool {
        !self.folded.contains(folder)
    }

    pub fn fold(&mut self, folder: &Path, open: bool) {
        if open {
            self.folded.remove(folder);
        } else {
            self.folded.insert(folder.to_path_buf());
        }
    }

    pub fn naming(&self) -> Option<&Naming> {
        self.naming.as_ref()
    }

    /// Takes back the name the field has been editing. The panel hands the
    /// naming out to draw it and hands it back the same frame, which is what
    /// lets the tree be walked while only being read.
    pub fn set_naming(&mut self, naming: Option<Naming>) {
        self.naming = naming;
    }

    pub fn trouble(&self) -> Option<&PartFileError> {
        self.trouble.as_ref()
    }

    pub fn confirming(&self) -> Option<&Path> {
        self.confirming.as_deref()
    }

    /// Where a new folder would go: inside the folder selected, beside the
    /// part selected, and at the root when nothing is.
    fn holding(&self, path: Option<&Path>) -> PathBuf {
        match path {
            Some(path) if self.is_a_folder(path) => path.to_path_buf(),
            Some(path) => path.parent().unwrap_or(&self.root).to_path_buf(),
            None => self.root.clone(),
        }
    }

    fn is_a_folder(&self, path: &Path) -> bool {
        fn under(folder: &Folder, path: &Path) -> bool {
            folder.path == path || folder.folders.iter().any(|under_it| under(under_it, path))
        }
        under(&self.library, path)
    }

    pub fn start_new_folder(&mut self) {
        let inside = self.holding(self.selected.as_deref());
        self.fold(&inside, true);
        self.naming = Some(Naming::NewFolder {
            inside,
            typed: String::new(),
        });
    }

    /// Only ever offered for something selected that this window is not busy
    /// writing to.
    pub fn start_rename(&mut self) {
        let Some(path) = self.selected.clone() else {
            return;
        };
        if self.busy_with(&path) {
            return;
        }
        self.naming = Some(Naming::Rename {
            typed: crate::wording::file::name_of(&path),
            path,
        });
    }

    pub fn cancel_naming(&mut self) {
        self.naming = None;
    }

    /// Writes the name being typed, and selects whatever it produced.
    pub fn confirm_naming(&mut self, files: &impl Files, folders: &impl Folders) {
        let Some(naming) = self.naming.take() else {
            return;
        };
        let done = match &naming {
            Naming::NewFolder { inside, typed } => library::create_folder(folders, inside, typed),
            Naming::Rename { path, typed } if self.is_a_folder(path) => {
                library::rename_folder(folders, path, typed)
            }
            Naming::Rename { path, typed } => library::rename_part(files, folders, path, typed),
        };
        match done {
            Ok(path) => {
                self.trouble = None;
                self.selected = Some(path);
                self.refresh(folders);
            }
            Err(fate) => {
                self.trouble = Some(fate);
                self.naming = Some(naming);
            }
        }
    }

    pub fn ask_to_discard(&mut self) {
        let Some(path) = self.selected.clone() else {
            return;
        };
        if !self.busy_with(&path) {
            self.confirming = Some(path);
        }
    }

    pub fn cancel_discard(&mut self) {
        self.confirming = None;
    }

    pub fn discard(&mut self, folders: &impl Folders) {
        let Some(path) = self.confirming.take() else {
            return;
        };
        match library::discard(folders, &path) {
            Ok(()) => {
                self.trouble = None;
                self.selected = None;
                self.refresh(folders);
            }
            Err(fate) => self.trouble = Some(fate),
        }
    }

    /// Whether the panel must keep its hands off `path`: nothing selected, or
    /// the very part this window is writing to.
    pub fn busy_with(&self, path: &Path) -> bool {
        self.in_use.as_deref() == Some(path)
    }

    pub fn may_tidy(&self) -> bool {
        self.selected
            .as_deref()
            .is_some_and(|path| !self.busy_with(path))
    }
}

#[cfg(test)]
mod tests;
