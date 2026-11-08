//! Where the window stood when the application closed, and where it opens next.
//!
//! Everything here is in the desktop's own units — points on macOS, pixels
//! elsewhere — the ones the system lays its monitors out in. The shell maps
//! what the windowing library reports onto them.

use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::locations::Locations;
use crate::ports::Files;
use crate::storage::StorageError;

/// The size a window takes when nothing else decides it, in points.
const DEFAULT_SIZE_IN_POINTS: (u32, u32) = (1280, 800);

/// The window's outer top-left corner and inner size, and whether it is
/// maximised. Maximised, the rect is still the one it goes back to once
/// restored, never the screen's.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct WindowPlace {
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
    pub maximised: bool,
}

/// A monitor as the system lays it out. `scale` is how many desktop units a
/// point takes on it.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Monitor {
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
    pub scale: f64,
}

/// Where the window opens: the remembered place while the setting is on and
/// its centre lies on one of the `monitors`; otherwise maximised, over the
/// default size centred on the main monitor, the first of the slice. With no
/// monitor reported at all, the remembered place stands, or the default size
/// at the origin, and the system places the window.
pub fn where_the_window_opens(
    remembered: Option<WindowPlace>,
    reopen_at_last_size: bool,
    monitors: &[Monitor],
) -> WindowPlace {
    let fits = |place: WindowPlace| {
        monitors.is_empty()
            || monitors
                .iter()
                .any(|monitor| monitor.holds_the_centre_of(place))
    };
    match remembered {
        Some(place) if reopen_at_last_size && fits(place) => place,
        _ => maximised_over_the_default_size(monitors.first()),
    }
}

fn maximised_over_the_default_size(main: Option<&Monitor>) -> WindowPlace {
    let scale = main.map_or(1.0, |main| main.scale);
    let in_units = |points: u32| (f64::from(points) * scale).round() as u32;
    let (width, height) = (
        in_units(DEFAULT_SIZE_IN_POINTS.0),
        in_units(DEFAULT_SIZE_IN_POINTS.1),
    );
    let (x, y) = main.map_or((0, 0), |main| {
        (
            centred(main.x, main.width, width),
            centred(main.y, main.height, height),
        )
    });
    WindowPlace {
        x,
        y,
        width,
        height,
        maximised: true,
    }
}

fn centred(start: i32, span: u32, length: u32) -> i32 {
    (i64::from(start) + (i64::from(span) - i64::from(length)) / 2) as i32
}

impl Monitor {
    fn holds_the_centre_of(&self, place: WindowPlace) -> bool {
        let within = |start: i32, span: u32, corner: i32, length: u32| {
            let centre = i64::from(corner) + i64::from(length) / 2;
            (i64::from(start)..i64::from(start) + i64::from(span)).contains(&centre)
        };
        within(self.x, self.width, place.x, place.width)
            && within(self.y, self.height, place.y, place.height)
    }
}

impl WindowPlace {
    fn state_path(at: &Locations) -> PathBuf {
        at.config.join("window.json")
    }

    /// The place written on the last close; `None` when there is none, or
    /// when it cannot be read back, which costs one launch at the default.
    pub fn load(files: &impl Files, at: &Locations) -> Option<Self> {
        serde_json::from_slice(&files.read(&Self::state_path(at)).ok()?).ok()
    }

    pub fn save(&self, files: &impl Files, at: &Locations) -> Result<(), StorageError> {
        let json = serde_json::to_string_pretty(self)?;
        Ok(files.write(&Self::state_path(at), json.as_bytes())?)
    }
}

#[cfg(test)]
mod tests;
