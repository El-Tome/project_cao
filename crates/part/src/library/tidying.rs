use std::path::{Path, PathBuf};

use crate::PartDocument;
use crate::errors::PartFileError;
use crate::file_name;
use crate::ports::{FileError, Files, Folders};

/// Makes a folder called `name` inside `inside`, and says where it landed.
pub fn create_folder(
    folders: &impl Folders,
    inside: &Path,
    name: &str,
) -> Result<PathBuf, PartFileError> {
    let path = file_name::folder_asked_for(inside, name).ok_or(PartFileError::BlankName)?;
    refuse_if_taken(folders, inside, &path)?;
    folders.create(&path)?;
    Ok(path)
}

/// Gives `folder` another name, keeping everything under it.
pub fn rename_folder(
    folders: &impl Folders,
    folder: &Path,
    name: &str,
) -> Result<PathBuf, PartFileError> {
    let inside = above(folder)?;
    let path = file_name::folder_asked_for(inside, name).ok_or(PartFileError::BlankName)?;
    if path == folder {
        return Ok(path);
    }
    refuse_if_taken(folders, inside, &path)?;
    folders.rename(folder, &path)?;
    Ok(path)
}

/// Gives `part` another name, inside the archive as well as on the disk.
///
/// The two are written together on purpose: the panel reads the file's name
/// and the title bar reads the archive's, and a rename that moved only one of
/// them would leave the part answering to two names at once.
///
/// The file is moved rather than copied and thrown away: a copy would leave
/// one in the bin per rename, and would stand as two files on the disk for as
/// long as it took the second write to fail.
///
/// The part is written back under the hour it already carried: giving
/// something a new name is not working on it.
pub fn rename_part(
    files: &impl Files,
    folders: &impl Folders,
    part: &Path,
    name: &str,
) -> Result<PathBuf, PartFileError> {
    let inside = above(part)?;
    let path = file_name::asked_for(inside, name).ok_or(PartFileError::BlankName)?;
    if path == part {
        return Ok(path);
    }
    if !the_same_file(&path, part) {
        refuse_if_taken(folders, inside, &path)?;
    }

    let mut document = PartDocument::load(files, part)?;
    let untouched_since = document.metadata.modified_at;
    folders.rename(part, &path)?;
    document.metadata.name = super::name_of(&path);
    document.save(files, &path, untouched_since)?;
    Ok(path)
}

/// Hands a part or a folder to wherever the platform keeps what was thrown
/// away, so a wrong click can be undone outside the application.
pub fn discard(folders: &impl Folders, path: &Path) -> Result<(), PartFileError> {
    Ok(folders.discard(path)?)
}

fn above(path: &Path) -> Result<&Path, PartFileError> {
    path.parent()
        .ok_or_else(|| PartFileError::File(FileError::Absent(path.to_path_buf())))
}

fn refuse_if_taken(
    folders: &impl Folders,
    inside: &Path,
    wanted: &Path,
) -> Result<(), PartFileError> {
    let taken = folders
        .entries(inside)?
        .into_iter()
        .any(|entry| the_same_file(&entry.path, wanted));
    if taken {
        return Err(PartFileError::NameTaken(wanted.to_path_buf()));
    }
    Ok(())
}

/// Whether two paths would land on one file. Case is ignored, because the
/// filesystem this runs on most often ignores it: a `Brides` already there
/// answers to `brides`, and a folder made under the second name would quietly
/// be the first one. The answer must not change with the disk the parts folder
/// happens to sit on.
fn the_same_file(one: &Path, other: &Path) -> bool {
    one == other
        || one.as_os_str().to_string_lossy().to_lowercase()
            == other.as_os_str().to_string_lossy().to_lowercase()
}

#[cfg(test)]
mod tests;
