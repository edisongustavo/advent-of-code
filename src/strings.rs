use crate::geometry::overlaps;
use itertools::Itertools;
use map_macro::hash_map;
use std::collections::{HashMap, HashSet};
use std::iter::zip;

pub fn common_char(strings: &Vec<&str>) -> char {
    if strings.is_empty() {
        panic!("need at least one element");
    }
    let sets: Vec<HashSet<char>> = strings
        .iter()
        .map(|s| HashSet::from_iter(s.chars()))
        .collect();
    let mut sets_iter = sets.iter();

    let common_chars = sets_iter.next().map(|first| {
        sets_iter.fold(first.clone(), |left, right| {
            left.intersection(&right).cloned().collect()
        })
    });
    let chars = common_chars.expect("Should never happen. The `if` from top ensures this!");
    if chars.is_empty() {
        panic!("No common chars found for strings: {strings:?}")
    }
    // HACK: Returning a single char
    *chars.iter().exactly_one().unwrap()
}

pub fn find_indices<const N: usize>(string: &str, patterns: &[&str; N]) -> [Vec<usize>; N] {
    std::array::from_fn(|i| {
        string
            .match_indices(patterns[i])
            .map(|(index, s)| index)
            .collect_vec()
    })
}

pub trait SkipEmptyLines {
    fn skip_empty_start_lines(&self) -> Self;
}

impl SkipEmptyLines for String {
    fn skip_empty_start_lines(&self) -> Self {
        self.lines().skip_while(|s| s.trim().is_empty()).join("\n")
    }
}

impl SkipEmptyLines for Vec<&str> {
    fn skip_empty_start_lines(&self) -> Self {
        let mut r: Vec<&str> = Vec::with_capacity(self.capacity());
        for elem in self {
            r.push(elem);
        }
        return r;
    }
}

// fn replace_from_left(s: &str, replacements: HashMap<&str, &str>) -> String {
//     let indices = find_indices(
//         &line,
//         &[
//             "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
//         ],
//     );
//     let replacements = vec!["0", "1", "2", "3", "4", "5", "6", "7", "8", "9"];
//     let indices_sorted = zip(indices, replacements)
//         .filter(||(index, replacement)|)
//         .filter_map(|(index, replacement)| index.map(|i| (i, replacement)))
//         .sorted_by_key(|(index, replacement)| *index)
//         .collect_vec();
//     if indices_sorted.is_empty() {
//         return String::from(s);
//     }
//
//     let mut indices_to_use = vec![];
//     let mut max_i = -1;
//     for elem in indices_sorted {
//         if elem.0 < max_i {
//             indices_to_use.push(elem);
//             max_i = elem.0 + elem.1.len()
//         }
//     }
//     // Now finally reassemble the string
//     indices_to_use.map(|(index, replacement)| {
//
//     });
//     "".to_string()
// }

#[cfg(test)]
mod tests {
    use super::*;
    use map_macro::hash_map;

    #[test]
    fn test_find_indices() {
        assert_eq!(
            find_indices("onetwothree", &["one", "two"]),
            [vec![0], vec![3]]
        );
        assert_eq!(
            find_indices("eightwo", &["eight", "two"]),
            [vec![0], vec![4]]
        );
        assert_eq!(
            find_indices("eightwo eight", &["eight", "two"]),
            [vec![0, 8], vec![4]]
        );
    }

    // #[test]
    // fn test_replace_from_left() {
    //     assert_eq!(replace_from_left("onetwo", hash_map![
    //         "one" => "1",
    //         "two" => "2",
    //     ]), "12");
    //     assert_eq!(replace_from_left("eightwo three", hash_map![
    //         "eight" => "8",
    //         "two" => "2",
    //         "three" => "3",
    //     ]), "8wo 3");
    // }
}
