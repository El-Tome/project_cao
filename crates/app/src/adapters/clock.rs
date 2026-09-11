use chrono::Local;

/// The zone the machine is set to — the clock whoever reads the screen tells
/// the hour on. It is asked for rather than frozen into an offset: `Local`
/// resolves every instant on its own, so a part opened in winter keeps its
/// winter hour in a list drawn in summer.
pub fn reader_zone() -> Local {
    Local
}
