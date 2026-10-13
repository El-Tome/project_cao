//! An elliptic wall as a listing gives it, read by formulas of its own: the
//! eccentric angle of a place of its section, and how far the place stands
//! off the section's ellipse, from the foot Newton's steps find along that
//! angle — not the kernel's halving of the foot's multiplier, so that a slip
//! in one is not checked by the same slip. A place standing far off may be
//! read further than it stands, never nearer: every point of the ellipse is
//! at least as far as its foot.

/// Newton's steps from the place's own eccentric angle: within the
/// tolerance of the wall, a place's foot is there to rounding after a few.
const STEPS: usize = 16;

/// The eccentric angle of a place `[x, y]` of the section, read from the
/// major half-axis towards the minor.
pub(super) fn angle([major, minor]: [f64; 2], [x, y]: [f64; 2]) -> f64 {
    (y / minor).atan2(x / major)
}

/// How far a place `[x, y]` of the section stands off its ellipse.
pub(super) fn off([major, minor]: [f64; 2], [x, y]: [f64; 2]) -> f64 {
    let spread = (minor - major) * (minor + major);
    let mut theta = angle([major, minor], [x, y]);
    for _ in 0..STEPS {
        let (sin, cos) = theta.sin_cos();
        let slope = spread * sin * cos + major * x * sin - minor * y * cos;
        let bend = spread * (cos - sin) * (cos + sin) + major * x * cos + minor * y * sin;
        if bend == 0.0 {
            break;
        }
        let step = slope / bend;
        theta -= step;
        if step.abs() <= f64::EPSILON {
            break;
        }
    }
    let (sin, cos) = theta.sin_cos();
    (major * cos - x).hypot(minor * sin - y)
}
