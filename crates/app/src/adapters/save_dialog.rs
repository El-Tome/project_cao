use std::path::PathBuf;

/// Where the user wants the part `name` written as an STL, asked through the
/// platform's save dialog; nothing when they cancel it.
pub fn stl_destination(name: &str) -> Option<PathBuf> {
    rfd::FileDialog::new()
        .add_filter("STL", &["stl"])
        .set_file_name(format!("{name}.stl"))
        .save_file()
}
