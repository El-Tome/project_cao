//! The application's window, put where it was last closed and followed until
//! it closes again, for the next launch to read.
//!
//! What winit reports is in physical pixels, each value counted at the scale
//! of the screen or the window it was read from. The desktop lays its screens
//! out in its own units — points on macOS, pixels elsewhere — and that is what
//! `cao_prefs` decides in, so every value is converted on the way in and set
//! back in those units on the way out.

use std::sync::Arc;

use cao_prefs::{
    Locations, Monitor, WindowPlace, WindowSeen, place_to_hold, where_the_window_opens,
};
use winit::dpi::{LogicalPosition, LogicalSize, PhysicalPosition, PhysicalSize};
use winit::window::Window;

use crate::adapters::files::DiskFiles;

/// The units a desktop lays its screens out in.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Desktop {
    /// macOS: one space in points, whatever the scale of each screen.
    Points,
    /// Windows and X11: one space in pixels, as reported.
    Pixels,
}

const DESKTOP: Desktop = if cfg!(target_os = "macos") {
    Desktop::Points
} else {
    Desktop::Pixels
};

/// A rect as winit reports it, with the scale of the screen or the window it
/// was read from.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PhysicalRect {
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
    pub scale_factor: f64,
}

/// A rect in the desktop's own units.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DesktopRect {
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
}

impl Desktop {
    /// A screen in the desktop's units, with how many of them a point takes.
    pub fn monitor(self, screen: PhysicalRect) -> Monitor {
        let DesktopRect {
            x,
            y,
            width,
            height,
        } = self.in_units(screen);
        let scale = match self {
            Desktop::Points => 1.0,
            Desktop::Pixels => screen.scale_factor,
        };
        Monitor {
            x,
            y,
            width,
            height,
            scale,
        }
    }

    /// The window's outer corner and inner size in the desktop's units.
    pub fn window(self, window: PhysicalRect) -> DesktopRect {
        self.in_units(window)
    }

    fn in_units(self, physical: PhysicalRect) -> DesktopRect {
        let per_unit = match self {
            Desktop::Points => physical.scale_factor,
            Desktop::Pixels => 1.0,
        };
        let signed = |value: i32| (f64::from(value) / per_unit).round() as i32;
        let unsigned = |value: u32| (f64::from(value) / per_unit).round() as u32;
        DesktopRect {
            x: signed(physical.x),
            y: signed(physical.y),
            width: unsigned(physical.width),
            height: unsigned(physical.height),
        }
    }
}

/// The window followed from the place it opened at to the one it closes at.
/// Holds nothing under the test harness, which has no window, nor where the
/// system does not say where a window stands: nothing is then set, read or
/// written.
pub struct WindowKeeper {
    followed: Option<Followed>,
}

struct Followed {
    window: Arc<Window>,
    at: Option<Locations>,
    held: Option<WindowPlace>,
}

impl WindowKeeper {
    /// Puts the window, still hidden, where it opens: the remembered place, or
    /// maximised over the default size. The rect is set before the window is
    /// maximised, for un-maximising to go back to it.
    pub fn open(
        window: Option<&Arc<Window>>,
        at: Option<&Locations>,
        reopen_at_last_size: bool,
    ) -> Self {
        let followed = window
            .filter(|window| window.outer_position().is_ok())
            .map(|window| {
                let remembered = at.and_then(|at| WindowPlace::load(&DiskFiles, at));
                let place =
                    where_the_window_opens(remembered, reopen_at_last_size, &monitors(window));
                put(window, place);
                Followed {
                    window: Arc::clone(window),
                    at: at.cloned(),
                    held: Some(place),
                }
            });
        Self { followed }
    }

    /// Reads where the window stands now, once a frame.
    pub fn follow(&mut self) {
        if let Some(followed) = &mut self.followed
            && let Some(seen) = seen(&followed.window)
        {
            followed.held = place_to_hold(followed.held, seen);
        }
    }

    /// Reads the window a last time, as a move may not have been followed by a
    /// frame, and writes its place down. A place that cannot be written is
    /// lost rather than an error: the application closes all the same.
    pub fn put_down(&mut self) {
        self.follow();
        if let Some(Followed {
            at: Some(at),
            held: Some(place),
            ..
        }) = &self.followed
        {
            let _ = place.save(&DiskFiles, at);
        }
    }
}

/// The screens, the main one first.
fn monitors(window: &Window) -> Vec<Monitor> {
    let primary = window.primary_monitor();
    let mut screens: Vec<_> = window.available_monitors().collect();
    screens.sort_by_key(|screen| Some(screen) != primary.as_ref());
    screens
        .into_iter()
        .map(|screen| {
            let (position, size) = (screen.position(), screen.size());
            DESKTOP.monitor(PhysicalRect {
                x: position.x,
                y: position.y,
                width: size.width,
                height: size.height,
                scale_factor: screen.scale_factor(),
            })
        })
        .collect()
}

fn seen(window: &Window) -> Option<WindowSeen> {
    let (position, size) = (window.outer_position().ok()?, window.inner_size());
    let rect = DESKTOP.window(PhysicalRect {
        x: position.x,
        y: position.y,
        width: size.width,
        height: size.height,
        scale_factor: window.scale_factor(),
    });
    Some(WindowSeen {
        x: rect.x,
        y: rect.y,
        width: rect.width,
        height: rect.height,
        maximised: window.is_maximized(),
        minimised: window.is_minimized().unwrap_or(false),
        fullscreen: window.fullscreen().is_some(),
    })
}

fn put(window: &Window, place: WindowPlace) {
    match DESKTOP {
        Desktop::Points => {
            window.set_outer_position(LogicalPosition::new(place.x, place.y));
            let _ = window.request_inner_size(LogicalSize::new(place.width, place.height));
        }
        Desktop::Pixels => {
            window.set_outer_position(PhysicalPosition::new(place.x, place.y));
            let _ = window.request_inner_size(PhysicalSize::new(place.width, place.height));
        }
    }
    window.set_maximized(place.maximised);
}

#[cfg(test)]
mod tests;
