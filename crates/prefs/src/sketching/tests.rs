use crate::Settings;

#[test]
fn a_profile_that_never_wrote_it_lays_rules() {
    let written = serde_json::to_value(Settings::default()).expect("written");
    let mut older = written.as_object().expect("an object").clone();
    older.remove("sketching");

    let read: Settings = serde_json::from_value(older.into()).expect("read");

    assert!(read.sketching.exact_angles_lay_rules);
}
