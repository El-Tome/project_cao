use super::shrink;

fn halves(list: &[u32]) -> Vec<Vec<u32>> {
    (0..list.len())
        .map(|index| {
            let mut shorter = list.to_vec();
            shorter.remove(index);
            shorter
        })
        .collect()
}

#[test]
fn a_failing_list_shrinks_to_the_one_element_that_fails() {
    let fails = |list: &Vec<u32>| list.contains(&7);
    let shrunk = shrink(
        vec![3, 9, 7, 1, 4],
        |list: &Vec<u32>| halves(list),
        fails,
        || true,
    );
    assert_eq!(shrunk, vec![7]);
}

#[test]
fn a_case_nothing_smaller_breaks_is_handed_back_as_it_was() {
    let fails = |list: &Vec<u32>| list.len() == 3;
    let shrunk = shrink(
        vec![1, 2, 3],
        |list: &Vec<u32>| halves(list),
        fails,
        || true,
    );
    assert_eq!(shrunk, vec![1, 2, 3]);
}

#[test]
fn a_search_told_to_stop_hands_back_what_it_has() {
    let fails = |list: &Vec<u32>| list.contains(&7);
    let shrunk = shrink(
        vec![3, 9, 7, 1, 4],
        |list: &Vec<u32>| halves(list),
        fails,
        || false,
    );
    assert_eq!(shrunk, vec![3, 9, 7, 1, 4]);
}
