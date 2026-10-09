//! Where a curve crosses a surface that one of the surfaces it lies on was
//! decided to cut along lines parallel to an axis: through those lines, so
//! that the corner is the one those lines give, whichever curve reaches it.

use super::*;

/// A post along X about `(0, 4, 10)`, of radius 1.
fn post() -> Surface {
    cylinder(DVec3::new(0.0, 4.0, 10.0), DVec3::X, 1.0)
}

/// An end of the post, 30 along its axis from where its wall was drawn.
fn end() -> Surface {
    plane(DVec3::new(30.0, 0.0, 0.0), DVec3::X)
}

/// A floor 2e-7 inside the post's top ruling where the wall was drawn,
/// leaning by less than the angle read as square to the axis: read along
/// the axis, it cuts the wall along two rulings 1.3e-3 apart, though it
/// stands a fraction of the tolerance higher at the end, where its line
/// with the end crosses the rim a hundred tolerances aside of them.
fn floor() -> Surface {
    let lean = 0.8 * scale().eps() / (2.0 * scale().reach());
    plane(
        DVec3::new(0.0, 4.0, 11.0 - 2e-7),
        DVec3::new(-lean, 0.0, 1.0),
    )
}

/// The surfaces a curve lies on, each with what it was decided to make with
/// `surface`.
fn lying<'a>(own: &[&'a Surface], surface: &Surface) -> Vec<Lying<'a>> {
    own.iter()
        .map(|own| Lying {
            surface: own,
            with: relation(own, surface, scale()),
        })
        .collect()
}

/// The points, sorted along Y.
fn points(found: Crossings) -> Vec<DVec3> {
    let Crossings::At(found) = found else {
        panic!("points expected, got {found:?}");
    };
    let mut points: Vec<DVec3> = found.iter().map(|crossing| crossing.point).collect();
    points.sort_by(|one, other| one.y.total_cmp(&other.y));
    points
}

/// Where the rulings the floor cuts the post along cross the end.
fn ends_of_the_rulings() -> Vec<DVec3> {
    let Relation::Lines(rulings) = relation(&floor(), &post(), scale()) else {
        panic!("the floor is read along the post's axis");
    };
    let mut found = Vec::new();
    for ruling in rulings {
        found.extend(points(crossings(&Curve::Line(ruling), &end(), scale())));
    }
    found.sort_by(|one, other| one.y.total_cmp(&other.y));
    found
}

#[test]
fn the_line_a_floor_read_along_a_wall_s_axis_shares_with_an_end_crosses_the_wall_at_the_rulings_ends()
 {
    let eps = scale().eps();
    let Relation::Line(chord) = relation(&end(), &floor(), scale()) else {
        panic!("the end and the floor cross");
    };
    let chord = Curve::Line(chord);
    let unaware = points(crossings(&chord, &post(), scale()));
    let rulings = ends_of_the_rulings();
    assert!(unaware[0].distance(rulings[0]) > 100.0 * eps);
    let found = points(crossings_lying(
        &chord,
        &lying(&[&end(), &floor()], &post()),
        &post(),
        scale(),
        &Touches::default(),
    ));
    assert_eq!(found.len(), 2);
    for (found, ruling) in found.iter().zip(&rulings) {
        assert!(found.distance(*ruling) <= 1e-12 * REACH, "{found} {ruling}");
    }
}

#[test]
fn the_rim_of_an_end_crosses_a_floor_read_along_its_wall_s_axis_at_the_rulings_ends() {
    let Surface::Cylinder(wall) = post() else {
        unreachable!()
    };
    let rim = Curve::Circle(Circle::on(&wall, 30.0));
    let found = points(crossings_lying(
        &rim,
        &lying(&[&end(), &post()], &floor()),
        &floor(),
        scale(),
        &Touches::default(),
    ));
    let rulings = ends_of_the_rulings();
    assert_eq!(found.len(), 2);
    for (found, ruling) in found.iter().zip(&rulings) {
        assert!(found.distance(*ruling) <= 1e-12 * REACH, "{found} {ruling}");
    }
}

#[test]
fn a_line_along_the_axis_is_crossed_as_before() {
    let side = plane(DVec3::new(0.0, 4.0, 0.0), DVec3::Y);
    let Relation::Line(line) = relation(&side, &floor(), scale()) else {
        panic!("the side and the floor cross");
    };
    let line = Curve::Line(line);
    assert_eq!(
        crossings_lying(
            &line,
            &lying(&[&side, &floor()], &post()),
            &post(),
            scale(),
            &Touches::default(),
        ),
        crossings(&line, &post(), scale()),
    );
}

#[test]
fn a_line_on_a_plane_holding_the_axis_to_rounding_is_crossed_as_before() {
    let level = plane(DVec3::new(0.0, 4.0, 10.5), DVec3::Z);
    let Relation::Line(chord) = relation(&end(), &level, scale()) else {
        panic!("the end and the level plane cross");
    };
    let chord = Curve::Line(chord);
    assert_eq!(
        crossings_lying(
            &chord,
            &lying(&[&end(), &level], &post()),
            &post(),
            scale(),
            &Touches::default(),
        ),
        crossings(&chord, &post(), scale()),
    );
}

#[test]
fn the_rulings_cross_the_plane_of_the_line_they_cross_most_steeply() {
    let Relation::Line(chord) = relation(&end(), &floor(), scale()) else {
        panic!("the end and the floor cross");
    };
    let chord = Curve::Line(chord);
    let along = plane(DVec3::new(0.0, 4.0, 0.0), DVec3::new(1e-9, 1.0, 0.0));
    let found = points(crossings_lying(
        &chord,
        &lying(&[&floor(), &along, &end()], &post()),
        &post(),
        scale(),
        &Touches::default(),
    ));
    let rulings = ends_of_the_rulings();
    assert_eq!(found.len(), 2);
    for (found, ruling) in found.iter().zip(&rulings) {
        assert!(found.distance(*ruling) <= 1e-12 * REACH, "{found} {ruling}");
    }
}
