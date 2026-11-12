//! A part closed, written down with what it shows.

use super::CaoApp;
use crate::adapters::files::DiskFiles;
use crate::screens::Screen;

impl CaoApp {
    /// Writes the part down with a picture of itself and the geometry it is
    /// showing, on the way out of it.
    ///
    /// Here rather than in the autosave because the autosave runs at the end of
    /// every gesture, and a render with a readback at every gesture is the cost
    /// the picture exists to avoid. Once a part is closed is enough for both:
    /// nothing looks at the picture until a panel lists the folder, and nothing
    /// reads the geometry back until the part is opened again.
    pub(super) fn put_the_part_away(&mut self) {
        let Screen::PartOpened(part) = &mut self.screen else {
            return;
        };

        let taken = self
            .painter
            .as_mut()
            .and_then(|painter| painter.take(&part.doc));
        // A part only looked at and closed again comes out of the renderer
        // exactly as it went in, and writing it would move its hour for
        // nothing.
        if let Some(picture) = taken
            && part.doc.picture() != Some(&picture)
        {
            part.doc.set_picture(picture);
            part.autosave.touched();
            self.explorer.went_stale();
        }

        let lang = self.remembered.lang();
        if let Some(message) = part
            .autosave
            .put_away_if_due(&DiskFiles, &part.doc, &part.path, lang)
        {
            self.error = Some(message);
        }
    }
}
