use std::collections::{HashMap, HashSet};
use itertools::Itertools;

pub fn compare_maps(actual: HashMap<(char, (usize, usize)), HashSet<u32>>, expected: HashMap<(char, (usize, usize)), HashSet<u32>>) {
    let actual_keys = actual.keys().sorted().collect_vec();
    let expected_keys = expected.keys().sorted().collect_vec();
    assert_eq!(actual_keys, expected_keys);
    for k in actual.keys().sorted() {
        let v_actual = actual.get(k).unwrap();
        let v_expected = expected.get(k).unwrap();
        assert_eq!(
            v_actual, v_expected,
            "Different values for key '{k:?}'.\nActual:   {v_actual:?}\nExpected: {v_expected:?}\n"
        );
    }
    assert_eq!(actual, expected);
}
