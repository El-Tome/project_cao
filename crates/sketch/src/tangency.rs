use crate::constraints::Constraint;
use crate::sketch::Sketch;

impl Sketch {
    /// Whether a tangency's contact point has slid past one end of the
    /// segment it is supposed to touch.
    ///
    /// The equations that hold a contact point only put it on the segment's
    /// infinite line, square under the circle's centre — nothing stops that
    /// foot from landing beyond either end once the centre is dragged far
    /// enough. Past that point the tangency is no longer physically real, so
    /// a solve that reaches it is treated as having failed rather than as
    /// having quietly moved the touch somewhere the segment does not go.
    pub(crate) fn has_a_flipped_tangent(&self) -> bool {
        self.constraints().iter().any(|constraint| {
            let Constraint::Tangent {
                segment,
                at: Some(at),
                ..
            } = *constraint
            else {
                return false;
            };
            let Some(at) = self.live_point(Some(at)) else {
                return false;
            };
            let Some(line) = self.segments().get(segment.0).copied() else {
                return false;
            };
            let (a, b) = (self.point(line.start), self.point(line.end));
            let span = b - a;
            let length_squared = span.length_squared();
            if length_squared < 1e-12 {
                return false;
            }
            let t = (self.point(at) - a).dot(span) / length_squared;
            const SPAN_MARGIN: f64 = 1e-6;
            !(-SPAN_MARGIN..=1.0 + SPAN_MARGIN).contains(&t)
        })
    }
}
