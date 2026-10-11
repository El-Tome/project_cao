//! How the sketch reads what is typed into it.

use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct SketchingConfig {
    /// Whether an angle typed at exactly 0°, 90° or 180° lays the parallel or
    /// the perpendicular rule it means rather than a dimension reading it —
    /// and whether the line tool's own right angle is that rule too.
    pub exact_angles_lay_rules: bool,
}

impl Default for SketchingConfig {
    fn default() -> Self {
        Self {
            exact_angles_lay_rules: true,
        }
    }
}

#[cfg(test)]
mod tests;
