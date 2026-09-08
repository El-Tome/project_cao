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
mod tests {
    use super::*;

    fn platform(documents: Option<&str>) -> Locations {
        Locations {
            config: "/config/cao".into(),
            data: "/data/cao".into(),
            documents: documents.map(Into::into),
        }
    }

    #[test]
    fn a_platform_with_a_documents_folder_keeps_the_parts_in_it() {
        let at = platform(Some("/home/tom/Documents"));

        assert_eq!(
            default_projects_dir(&at),
            PathBuf::from("/home/tom/Documents/CAO"),
        );
    }

    #[test]
    fn a_platform_without_one_keeps_the_parts_beside_its_own_data() {
        let at = platform(None);

        assert_eq!(
            default_projects_dir(&at),
            PathBuf::from("/data/cao/projects")
        );
    }
}
