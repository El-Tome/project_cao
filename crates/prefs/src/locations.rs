use std::path::PathBuf;

/// Where the platform keeps what the installation remembers.
///
/// A value, not a behaviour: the shell asks the machine once, at startup, and
/// everything below is told rather than going to look. That is what lets a
/// test say `/config` and mean it.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Locations {
    pub config: PathBuf,
    pub data: PathBuf,
    /// The platform's own documents folder, when it has one.
    pub documents: Option<PathBuf>,
}

/// Where new parts land by default: `<Documents>/CAO` when the platform offers
/// a documents folder, otherwise beside the application's own data.
pub fn default_projects_dir(at: &Locations) -> PathBuf {
    match &at.documents {
        Some(documents) => documents.join("CAO"),
        None => at.data.join("projects"),
    }
}

#[cfg(test)]
mod tests;
