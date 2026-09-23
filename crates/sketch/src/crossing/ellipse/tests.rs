//! What sketch · crossing/ellipse.rs is held to.

use super::*;

/// An ellipse about (10, 5), sixty wide and twenty high, turned a sixth of a
/// half-turn so that nothing lines up with the axes by accident.
fn leaning() -> EllipseDraft {
    let turn = DVec2::from_angle(std::f64::consts::FRAC_PI_6);
    EllipseDraft {
        centre: DVec2::new(10.0, 5.0),
        first: turn * 30.0,
        second: 10.0,
    }
}

/// How far the crossing said to stand at those two fractions really is from
/// each of the two curves, the ellipse's place taken as the truth.
fn apart(oval: EllipseDraft, at: f64, place: DVec2) -> f64 {
    oval.at(at * std::f64::consts::TAU).distance(place)
}

#[test]
fn a_straight_run_through_an_ellipse_crosses_it_twice() {
    let oval = leaning();
    let (from, to) = (DVec2::new(-40.0, 5.0), DVec2::new(60.0, 5.0));

    let crossings = where_segment_crosses_ellipse(from, to, oval);

    assert_eq!(crossings.len(), 2, "{crossings:?}");
    for (along, round) in crossings {
        let place = from.lerp(to, along);
        assert!(oval.distance(place) < 1e-9, "{place} is not on the curve");
        assert!(
            apart(oval, round, place) < 1e-9,
            "the turn says another place"
        );
    }
}

#[test]
fn a_straight_run_beside_an_ellipse_crosses_it_nowhere() {
    let oval = leaning();

    assert!(
        where_segment_crosses_ellipse(DVec2::new(-40.0, 60.0), DVec2::new(60.0, 60.0), oval)
            .is_empty()
    );
}

#[test]
fn a_run_that_stops_inside_an_ellipse_crosses_it_once() {
    let oval = leaning();
    let (from, to) = (DVec2::new(-40.0, 5.0), DVec2::new(10.0, 5.0));

    let crossings = where_segment_crosses_ellipse(from, to, oval);

    assert_eq!(crossings.len(), 1, "{crossings:?}");
}

#[test]
fn a_circle_through_an_ellipse_crosses_it_where_both_agree() {
    let oval = leaning();
    let (centre, radius) = (DVec2::new(30.0, 5.0), 12.0);

    let crossings = where_circle_crosses_ellipse(centre, radius, oval);

    assert_eq!(crossings.len(), 2, "{crossings:?}");
    for (round, at) in crossings {
        let place = oval.at(at * std::f64::consts::TAU);
        assert!(oval.distance(place) < 1e-9);
        assert!(
            (place.distance(centre) - radius).abs() < 1e-6,
            "{place} is {} from the circle's centre, not {radius}",
            place.distance(centre),
        );
        let on_the_circle = centre + DVec2::from_angle(round * std::f64::consts::TAU) * radius;
        assert!(on_the_circle.distance(place) < 1e-6, "the turns disagree");
    }
}

#[test]
fn a_circle_inside_an_ellipse_crosses_it_nowhere() {
    let oval = leaning();

    assert!(where_circle_crosses_ellipse(DVec2::new(10.0, 5.0), 4.0, oval).is_empty());
}

#[test]
fn an_arc_crosses_an_ellipse_only_over_the_stretch_it_runs() {
    let oval = leaning();
    let (centre, radius) = (DVec2::new(30.0, 5.0), 12.0);
    let whole = where_circle_crosses_ellipse(centre, radius, oval);
    assert_eq!(whole.len(), 2, "the whole circle crosses twice: {whole:?}");

    // A short stretch of that circle about the first of the two crossings,
    // which leaves the second on the part the arc does not run over.
    let on_the_circle =
        |fraction: f64| centre + DVec2::from_angle(fraction * std::f64::consts::TAU) * radius;
    let (first, _) = whole[0];
    let arc = ArcDraft {
        centre,
        start: on_the_circle(first - 0.05),
        end: on_the_circle(first + 0.05),
    };
    let crossings = where_arc_crosses_ellipse(arc, oval);

    assert_eq!(crossings.len(), 1, "{crossings:?}");
    let (round, at) = crossings[0];
    let place = oval.at(at * std::f64::consts::TAU);
    assert!((place.distance(centre) - radius).abs() < 1e-6, "{place}");
    assert!(
        (round - 0.5).abs() < 0.01,
        "the crossing stands halfway along the sweep, not at {round}",
    );
}

#[test]
fn two_ellipses_that_overlap_cross_where_both_agree() {
    let near = leaning();
    let far = EllipseDraft {
        centre: DVec2::new(40.0, 5.0),
        first: DVec2::new(0.0, 25.0),
        second: 12.0,
    };

    let crossings = where_ellipses_cross(near, far);

    assert_eq!(crossings.len(), 2, "{crossings:?}");
    for (here, there) in crossings {
        let place = near.at(here * std::f64::consts::TAU);
        assert!(far.distance(place) < 1e-6, "{place} is not on the other");
        assert!(apart(far, there, place) < 1e-6, "the turns disagree");
    }
}

#[test]
fn two_ellipses_apart_cross_nowhere() {
    let far = EllipseDraft {
        centre: DVec2::new(200.0, 5.0),
        first: DVec2::new(0.0, 25.0),
        second: 12.0,
    };

    assert!(where_ellipses_cross(leaning(), far).is_empty());
}

#[test]
fn an_ellipse_crossed_four_times_finds_all_four() {
    let near = leaning();
    // A long thin ellipse laid across the other's length: it goes in and out
    // of it twice.
    let far = EllipseDraft {
        centre: near.centre,
        first: near.first.perp().normalize() * 40.0,
        second: 2.0,
    };

    let crossings = where_ellipses_cross(near, far);

    assert_eq!(crossings.len(), 4, "{crossings:?}");
}
