use crate::files::contents;
use anyhow::{Context, Result};
use itertools::Itertools;
use map_macro::{hash_map, hash_set};
use ndarray::prelude::*;
use num::pow;
use pest::Parser;
use std::cmp::min;
use std::collections::{HashMap, HashSet, VecDeque};
use std::fmt::{Debug, Display, Write};
use std::iter::zip;
use std::ops::Add;

type PuzzleResult = usize;

pub fn solution() -> (PuzzleResult, PuzzleResult) {
    inner(&contents("inputs/2023/day4.txt")).unwrap()
}

type Card = (HashSet<u32>, HashSet<u32>);

fn inner(input: &String) -> Result<(PuzzleResult, PuzzleResult)> {
    let part2 = PuzzleResult::default();
    let cards = parse(input)?;
    let part1 = cards
        .iter()
        .filter_map(|(winners, numbers)| {
            let intersecting = numbers.intersection(winners).count();
            if intersecting > 0 {
                let val = pow::<PuzzleResult>(2, intersecting - 1);
                Some(val)
            } else {
                None
            }
        })
        .sum();

    let num_intersecting = cards
        .iter()
        .map(|(winners, numbers)| numbers.intersection(winners).count())
        .collect_vec();
    let mut ret = 0;
    let mut cards_to_process: VecDeque<usize> = (0..cards.len()).collect();
    while !cards_to_process.is_empty() {
        let original = cards_to_process.pop_front();
        if original.is_none() {
            break;
        }
        let original = original.unwrap();
        let intersecting = num_intersecting[original];

        ret += 1;
        for copy in 0..intersecting {
            let index = original + copy + 1;
            cards_to_process.push_back(index);
        }
    }
    let part2 = ret;

    Ok((part1, part2))
}

fn parse(input: &String) -> Result<Vec<Card>> {
    let lines = input.lines().collect_vec();
    let mut ret = vec![];
    for line in lines.into_iter() {
        let elements = line.split_ascii_whitespace().collect_vec();
        let winners = elements
            .iter()
            .skip(2)
            .take_while(|elem| **elem != "|")
            .map(|entry| entry.parse::<u32>().context(format!("Entry: {entry}")))
            .try_collect()?;
        let numbers = elements
            .iter()
            .skip_while(|elem| **elem != "|")
            .skip(1)
            .map(|entry| entry.parse::<u32>().context(format!("Entry: {entry}")))
            .try_collect()?;
        ret.push((winners, numbers));
    }
    Ok(ret)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::strings::SkipEmptyLines;
    use itertools::Itertools;
    use map_macro::hash_map;
    use nofmt;
    use pretty_assertions::assert_eq as pretty_assert_eq;
    use textwrap::dedent;

    fn to_vec(f: Vec<&str>) -> Vec<String> {
        f.into_iter().map(|s| String::from(s)).collect_vec()
    }

    fn program() -> String {
        return dedent(
            "
            Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
            Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
            Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
            Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
            Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
            Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11",
        )
        .skip_empty_start_lines();
    }
    #[test]
    fn test_part1_inner() -> Result<()> {
        let input_string = program();
        let actual = inner(&input_string)?.0;
        assert_eq!(actual, 13);

        Ok(())
    }

    #[test]
    fn test_part2_inner() -> Result<()> {
        let input_string = program();
        let actual = inner(&input_string)?.1;
        assert_eq!(actual, 30);
        Ok(())
    }
}
