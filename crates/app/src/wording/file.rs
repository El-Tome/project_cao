use std::path::Path;

/// The three ways the disk can refuse, said once.
///
/// `cao_part` and `cao_prefs` each carry their own `FileError` — a part
/// archive and a settings file have nothing to say to each other — but a
/// refused write reads the same to whoever is looking at it.
pub fn absent(path: &Path) -> String {
    format!("Aucun fichier à {}.", path.display())
}

pub fn refused(path: &Path) -> String {
    format!("Le système a refusé l'accès à {}.", path.display())
}

pub fn interrupted(path: &Path) -> String {
    format!("L'échange avec {} s'est arrêté en chemin.", path.display())
}
