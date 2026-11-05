//! Which components the curve a cone meets another wall along has, read off
//! the zeros of `D` round the cone: a span between two simple zeros where
//! `D` is positive and the roots stand on the nappe makes one, a double zero
//! inside it a node, an isolated one a contact, and `D` positive all round
//! two laps winding round the cone, or one where the other wall holds the
//! apex.

use std::f64::consts::TAU;

use glam::DVec2;

use super::{ACROSS_A_NODE, Lap, Ruled, angle_between};
use crate::brep::meet::pair::Configuration;

impl Ruled {
    pub(super) fn configured(&mut self) {
        let inside = self.other.inside(self.cone.apex());
        let positive = |theta: f64| self.discriminant(theta)[0] > 0.0;
        if inside {
            self.laps = vec![Lap {
                span: None,
                sign: 1.0,
                start: 0.0,
                nodes: Vec::new(),
                around: Vec::new(),
            }];
            self.configuration = Configuration::OneLoop;
            return;
        }
        let above = |theta: f64, sign: f64| {
            let l = self.root(theta, sign);
            l >= self.cone.apex_at() && self.other.holds(self.cone.point(DVec2::new(theta, l)))
        };
        if self.zeros.is_empty() {
            if positive(0.0) || self.doubles.iter().any(|double| positive(double + 0.1)) {
                let laps: Vec<Lap> = [1.0, -1.0]
                    .into_iter()
                    .filter(|sign| above(self.probe(), *sign))
                    .map(|sign| Lap {
                        span: None,
                        sign,
                        start: 0.0,
                        nodes: self.doubles.clone(),
                        around: Vec::new(),
                    })
                    .collect();
                self.configuration = match (laps.len(), self.doubles.is_empty()) {
                    (0, _) => Configuration::Apart,
                    (1, _) => Configuration::OneLoop,
                    (_, true) => Configuration::TwoLoops,
                    (_, false) => Configuration::TwoEllipses,
                };
                self.laps = laps;
            } else {
                self.contact = self.doubles.first().copied();
                self.configuration = if self.contact.is_some() {
                    Configuration::Contact
                } else {
                    Configuration::Apart
                };
            }
            return;
        }
        let count = self.zeros.len();
        let mut laps = Vec::new();
        for index in 0..count {
            let from = self.zeros[index];
            let to = self.zeros[(index + 1) % count] + if index + 1 == count { TAU } else { 0.0 };
            let middle = (from + to) / 2.0;
            let clear = [0.5, 0.25, 0.75, 0.125, 0.875]
                .into_iter()
                .map(|share| from + (to - from) * share)
                .find(|theta| {
                    self.doubles
                        .iter()
                        .all(|double| angle_between(*theta, *double) > ACROSS_A_NODE)
                })
                .unwrap_or(middle);
            if !positive(clear) || !above(clear, 1.0) {
                continue;
            }
            let nodes = self
                .doubles
                .iter()
                .map(|double| double + TAU * ((middle - double) / TAU).round())
                .filter(|double| (from..to).contains(double))
                .collect();
            laps.push(Lap {
                span: Some([from, to]),
                sign: 1.0,
                start: 0.0,
                nodes,
                around: Vec::new(),
            });
        }
        let nodes = laps.iter().any(|lap| !lap.nodes.is_empty());
        self.configuration = match (laps.len(), nodes) {
            (0, _) => Configuration::Apart,
            (1, false) => Configuration::OneLoop,
            (1, true) => Configuration::FigureOfEight,
            (_, false) => Configuration::TwoLoops,
            (_, true) => Configuration::TwoEllipses,
        };
        self.laps = laps;
    }
}
