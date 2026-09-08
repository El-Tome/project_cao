use std::path::{Path, PathBuf};

/// File extension used for a CAO part document.
pub(crate) const PART_EXTENSION: &str = "caopart";

/// The name a new part takes in `dir`, and where it lands.
///
/// A part whose file would land on one already there is stepped past rather
/// than written over: `create_in` has no second chance to give back what it
/// overwrote.
pub(crate) fn free_in(dir: &Path, wanted: &str) -> (String, PathBuf) {
    let taken = |name: &str| path_in(dir, name).exists();
    let name = if taken(wanted) {
        (2..)
            .map(|suffix| format!("{wanted} {suffix}"))
            .find(|candidate| !taken(candidate))
            .unwrap_or_else(|| wanted.to_string())
    } else {
        wanted.to_string()
    };
    let path = path_in(dir, &name);
    (name, path)
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
    let trimmed = cleaned.trim();
    if trimmed.is_empty() {
        "Sans titre".to_string()
    } else {
        trimmed.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_name_a_filesystem_would_refuse_becomes_one_it_accepts() {
        assert_eq!(sanitize("Bras/gauche"), "Bras_gauche");
        assert_eq!(sanitize("  "), "Sans titre");
        assert_eq!(sanitize(" Support 12 "), "Support 12");
    }
}
