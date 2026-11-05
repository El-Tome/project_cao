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

pub fn step_destination(name: &str) -> Option<PathBuf> {
    rfd::FileDialog::new()
        .add_filter("STEP", &["step", "stp"])
        .set_file_name(format!("{name}.step"))
        .save_file()
        .map(with_step_extension)
}

/// The place chosen, ending in `.stl` whatever the dialog let through.
pub fn with_stl_extension(path: PathBuf) -> PathBuf {
    with_extension_among(path, &["stl"])
}

pub fn with_step_extension(path: PathBuf) -> PathBuf {
    with_extension_among(path, &["step", "stp"])
}

fn with_extension_among(path: PathBuf, kept: &[&str]) -> PathBuf {
    let ends_in_one = path
        .extension()
        .is_some_and(|extension| kept.iter().any(|kept| extension.eq_ignore_ascii_case(kept)));
    if ends_in_one {
        return path;
    }
    let mut named = path.into_os_string();
    named.push(".");
    named.push(kept[0]);
    named.into()
}

#[cfg(test)]
mod tests;
