use super::*;

fn at(text: &str) -> DateTime<Utc> {
    text.parse().expect("a date")
}

#[test]
fn a_part_drawn_to_order_holds_what_the_recipe_asked_for() {
    let recipe = Recipe {
        sketches: 2,
        sides: 5,
        circles: 2,
        arcs: 3,
        dimensions: 2,
        rules: 1,
        storeys: 1,
    };

    let part = recipe.drawn("Test", at("2026-01-02T09:00:00Z"));

    assert_eq!(part.sketches().len(), 3, "two rings and one storey");
    for (index, sketch) in part.sketches().iter().enumerate() {
        assert_eq!(sketch.segments().len(), 5, "sketch {index}: the ring");
        assert_eq!(sketch.circles().len(), 2, "sketch {index}: the bores");
        assert_eq!(sketch.arcs().len(), 3, "sketch {index}: the arcs");
        assert_eq!(sketch.dimensions().len(), 2, "sketch {index}: the values");
        assert_eq!(sketch.constraints().len(), 1, "sketch {index}: the rules");
    }
    assert!(!part.body().is_empty(), "every ring is raised into matter");
    assert!(part.has_scale(), "a value typed is what fixes the scale");
}

#[test]
fn a_storey_is_drawn_on_the_face_the_matter_below_it_left() {
    let now = at("2026-01-02T09:00:00Z");
    let bare = Recipe {
        storeys: 0,
        ..Recipe::default()
    }
    .drawn("Test", now);
    let (_, below) = bare.body().bounds().expect("a volume");

    let part = Recipe {
        storeys: 1,
        ..Recipe::default()
    }
    .drawn("Test", now);

    let storey = part.sketches()[1].plane;
    assert!(
        (storey.origin.z - below.z).abs() < 1e-9,
        "the storey is drawn at {} where the matter below it stops at {}",
        storey.origin.z,
        below.z,
    );
    let (_, top) = part.body().bounds().expect("a volume");
    assert!(top.z > below.z, "and raises the matter higher than it was");
}

#[test]
fn the_same_recipe_draws_the_same_part_twice() {
    let recipe = Recipe::default();
    let now = at("2026-01-02T09:00:00Z");

    let once = recipe.drawn("Test", now);
    let again = recipe.drawn("Test", now);

    assert_eq!(
        serde_json::to_string(&once.history).expect("a history"),
        serde_json::to_string(&again.history).expect("a history"),
        "a figure measured on one part means nothing if the next run draws \
         another",
    );
}
