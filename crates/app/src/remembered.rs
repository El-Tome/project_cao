use std::path::{Path, PathBuf};

use cao_prefs::{Locations, Profiles, RecentEntry, RecentList, StorageError};
use chrono::{DateTime, Utc};

use crate::adapters::files::DiskFiles;
use crate::lang::Catalogue;

/// What the installation remembers — the settings and the parts lately
/// opened — together with the place the platform gave to keep them.
///
/// Held here rather than in the open part: what is remembered outlives the
/// part being drawn. A platform that offers nowhere leaves `at` empty; what is
/// remembered then lasts as long as the window and nothing is written.
pub struct Remembered {
    at: Option<Locations>,
    pub profiles: Profiles,
    recents: RecentList,
    lang: Catalogue,
}

impl Remembered {
    pub fn read(at: Option<Locations>) -> Self {
        let place = at.as_ref();
        let mut recents = place
            .and_then(|at| RecentList::load(&DiskFiles, at).ok())
            .unwrap_or_default();
        recents.prune_missing(&DiskFiles);
        let profiles = place.map_or_else(Profiles::default, |at| Profiles::load(&DiskFiles, at));
        let lang = place.map_or_else(Catalogue::french, |at| {
            Catalogue::load(
                &DiskFiles,
                &at.config.join("lang"),
                &profiles.active().language,
            )
        });
        Self {
            at,
            profiles,
            recents,
            lang,
        }
    }

    /// What the interface says things with, in the language the settings ask
    /// for. A language dropped in by hand lands in `<config>/lang`.
    pub fn lang(&self) -> &Catalogue {
        &self.lang
    }

    /// `profiles` and `lang` borrowed apart, for a caller that edits one while
    /// reading the other — a single method call would borrow all of `self`.
    pub fn profiles_and_lang(&mut self) -> (&mut Profiles, &Catalogue) {
        (&mut self.profiles, &self.lang)
    }

    pub fn recents(&self) -> &[RecentEntry] {
        self.recents.entries()
    }

    /// Where new parts land, or nothing when the platform offers nowhere.
    pub fn projects_dir(&self) -> Option<PathBuf> {
        self.at.as_ref().map(cao_prefs::default_projects_dir)
    }

    /// Whether the platform offered nowhere at all, in which case nothing is
    /// ever written and the two methods below have nothing to report.
    pub fn has_nowhere_to_keep(&self) -> bool {
        self.at.is_none()
    }

    pub fn save_profiles(&self) -> Option<StorageError> {
        self.at
            .as_ref()
            .and_then(|at| self.profiles.save(&DiskFiles, at).err())
    }

    /// Puts a part at the head of the list, and writes the list down.
    pub fn remember_part(
        &mut self,
        path: &Path,
        name: &str,
        now: DateTime<Utc>,
    ) -> Option<StorageError> {
        self.recents.push(path.to_path_buf(), name, now);
        self.at
            .as_ref()
            .and_then(|at| self.recents.save(&DiskFiles, at).err())
    }
}
