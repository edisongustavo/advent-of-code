use crate::strings::SkipEmptyLines;
use itertools::Itertools;
use std::collections::HashSet;
use std::fmt::{Debug, Display};
use std::fs;

type PuzzleResult = usize;

pub fn day6() -> (PuzzleResult, PuzzleResult) {
    let contents = fs::read_to_string("inputs/2022/day6.txt").unwrap();
    inner(contents).unwrap()
}

fn inner(input: String) -> Result<(PuzzleResult, PuzzleResult), String> {
    let part1 = find_index(&input, 4).ok_or("Can't find index")?;
    let part2 = find_index(&input, 14).ok_or("Can't find index")?;

    Ok((part1, part2))
}

fn find_index(input: &String, n: usize) -> Option<PuzzleResult> {
    for i in 0..input.len() - n - 1 {
        let packet = &input[i..i + n];
        let individual_chars: HashSet<char> = HashSet::from_iter(packet.chars().into_iter());
        if individual_chars.len() == n {
            return Some(i + n);
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use expect_test::expect;
    use itertools::Itertools;
    use pretty_assertions::assert_eq;
    use textwrap::dedent;

    #[test]
    fn test_part1_inner() -> Result<(), String> {
        assert_eq!(inner("abcad".to_string())?.0, 5);
        assert_eq!(inner("mjqjpqmgbljsphdztnvjfqwrcgsmlb".to_string())?.0, 7);
        assert_eq!(inner("bvwbjplbgvbhsrlpgdmjqwftvncz".to_string())?.0, 5);
        assert_eq!(inner("nppdvjthqldpwncqszvftbrmjlhg".to_string())?.0, 6);
        assert_eq!(
            inner("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg".to_string())?.0,
            10
        );
        assert_eq!(inner("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw".to_string())?.0, 11);
        Ok(())
    }

    #[test]
    fn test_part2_inner() -> Result<(), String> {
        Ok(())
    }
}
