use std::path::{Path, PathBuf};

use crate::PartFileError;
use crate::ports::Files;

/// File extension used for a CAO part document.
pub(crate) const PART_EXTENSION: &str = "caopart";

/// The name a new part takes in `dir`, and where it lands.
///
/// A part whose file would land on one already there is stepped past rather
/// than written over: `create_in` has no second chance to give back what it
/// overwrote.
///
/// A name nothing survives is refused rather than replaced: which word an
/// untitled part carries is the interface's to choose, not this crate's.
pub(crate) fn free_in(
    files: &impl Files,
    dir: &Path,
    wanted: &str,
) -> Result<(String, PathBuf), PartFileError> {
    if sanitize(wanted).is_empty() {
        return Err(PartFileError::BlankName);
    }
    let taken = |name: &str| files.exists(&path_in(dir, name));
    let name = if taken(wanted) {
        (2..)
            .map(|suffix| format!("{wanted} {suffix}"))
            .find(|candidate| !taken(candidate))
            .unwrap_or_else(|| wanted.to_string())
    } else {
        wanted.to_string()
    };
    let path = path_in(dir, &name);
    Ok((name, path))
}

fn path_in(dir: &Path, name: &str) -> PathBuf {
    dir.join(format!("{}.{PART_EXTENSION}", sanitize(name)))
}

fn sanitize(name: &str) -> String {
    let cleaned: String = name
        .chars()
        .map(|c| {
            if c.is_alphanumeric() || c == '-' || c == '_' || c == ' ' {
                c
            } else {
                '_'
            }
        })
        .collect();
    cleaned.trim().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::adapters::InMemoryFiles;

    #[test]
    fn a_name_a_filesystem_would_refuse_becomes_one_it_accepts() {
        assert_eq!(sanitize("Bras/gauche"), "Bras_gauche");
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
}
