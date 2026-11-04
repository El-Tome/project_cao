use std::path::PathBuf;

/// Where the user wants the part `name` written as an STL, asked through the
/// platform's save dialog; nothing when they cancel it.
pub fn stl_destination(name: &str) -> Option<PathBuf> {
    rfd::FileDialog::new()
        .add_filter("STL", &["stl"])
        .set_file_name(format!("{name}.stl"))
        .save_file()
        .map(with_stl_extension)
}

/// The place chosen, ending in `.stl` whatever the dialog let through.
pub fn with_stl_extension(path: PathBuf) -> PathBuf {
    let ends_in_stl = path
        .extension()
        .is_some_and(|extension| extension.eq_ignore_ascii_case("stl"));
    if ends_in_stl {
        return path;
    }
    let mut named = path.into_os_string();
    named.push(".stl");
    named.into()
}

#[cfg(test)]
mod tests;
