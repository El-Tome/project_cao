use glam::DVec2;

use super::*;

const CLOSE: f64 = 1e-12;

fn numeric(trace: &Trace, u: f64) -> [DVec2; 2] {
    let step = 1e-5;
    let [before, after, here] = [u - step, u + step, u].map(|at| trace.at(at)[0]);
    [
        (after - before) / (2.0 * step),
        (after - 2.0 * here + before) / (step * step),
    ]
}

#[test]
fn a_round_trace_s_derivatives_are_those_of_its_points() {
    let trace = Trace::Round {
        center: DVec2::new(8.0, 0.0),
        radius: 5.0,
        start: 0.3,
        sweep: -2.0,
    };
    let [_, first, second] = trace.at(0.4);
    let [first_seen, second_seen] = numeric(&trace, 0.4);
    assert!((first - first_seen).length() < 1e-6, "{first} {first_seen}");
    assert!(
        (second - second_seen).length() < 1e-3,
        "{second} {second_seen}"
    );
}

#[test]
fn a_trace_run_the_other_way_starts_where_it_ended() {
    let traces = [
        Trace::Segment {
            from: DVec2::new(1.0, 2.0),
            to: DVec2::new(-3.0, 4.0),
        },
        Trace::Round {
            center: DVec2::ZERO,
            radius: 2.0,
            start: 1.0,
            sweep: 4.0,
        },
    ];
    for trace in traces {
        let back = trace.reversed();
        assert!((back.start() - trace.end()).length() < CLOSE);
        assert!((back.end() - trace.start()).length() < CLOSE);
        assert!((back.at(0.25)[0] - trace.at(0.75)[0]).length() < CLOSE);
    }
}
