use crate::files::{contents, lines};
use crate::geometry::overlaps;
use crate::strings::find_indices;
use anyhow::Result;
use itertools::Itertools;
use map_macro::hash_map;
use ndarray::prelude::*;
use std::fmt::{Debug, Display, Write};
use std::iter::zip;
use std::ptr::replace;

type PuzzleResult = i32;

#[derive(Eq, Hash, Ord, PartialOrd, PartialEq, Debug, Copy, Clone)]
pub struct Position(pub i32, pub i32);

pub fn solution() -> (PuzzleResult, PuzzleResult) {
    inner(&contents("inputs/2023/day1.txt")).unwrap()
}

fn inner(input: &String) -> Result<(PuzzleResult, PuzzleResult)> {
    let part1 = input
        .split("\n")
        .map(|line| line.chars().filter(|c| c.is_digit(10)).collect_vec())
        .filter(|line| !line.is_empty())
        .map(|digits| {
            let first_digit = digits.first().expect("First digit doesn't exist");
            let second_digit = digits.last().expect("Last digit doesn't exist");
            format!("{}{}", first_digit, second_digit)
        })
        .map(|digit_chars| digit_chars.parse::<i32>().expect("Not a number!"))
        .sum();

    let part2 = input.split("\n").map(parse_digits).sum();
    Ok((part1, part2))
}

fn parse_digits(line: &str) -> i32 {
    let replacements = hash_map! {
        "zero" => 0,
        "one" => 1,
        "two" => 2,
        "three" => 3,
        "four" => 4,
        "five" => 5,
        "six" => 6,
        "seven" => 7,
        "eight" => 8,
        "nine" => 9,
    };

    let patterns = &[
        "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    let indices = find_indices(&line, patterns);
    // Collect the first and last indexes where each "encoded number" appears.
    let mut all_numbers = indices
        .into_iter()
        .enumerate()
        .filter_map(|(i, indices)| {
            if indices.is_empty() {
                None
            } else {
                Some((
                    replacements[patterns[i]],
                    *indices.first().unwrap(),
                    *indices.last().unwrap(),
                ))
            }
        })
        .collect_vec();
    // Add the real numbers
    for number in replacements.values() {
        let s = number.to_string();
        let first_index = line.find(s.as_str());
        let last_index = line.rfind(s.as_str());
        if first_index.is_some() && last_index.is_some() {
            all_numbers.push((*number, first_index.unwrap(), last_index.unwrap()))
        }
    }
    // Get the first and last of each number
    let (first_of_each_number, last_of_each_number): (Vec<(i32, usize)>, Vec<(i32, usize)>) =
        all_numbers
            .into_iter()
            .map(|(number, first_index, last_index)| {
                let left = (number, first_index);
                let right = (number, last_index);
                (left, right)
            })
            .unzip();
    // Now get the boundary digits
    let first_digit = first_of_each_number
        .iter()
        .min_by_key(|(number, index)| index)
        .map(|(number, index)| number)
        .unwrap();
    let second_digit = last_of_each_number
        .iter()
        .max_by_key(|(number, index)| index)
        .map(|(number, index)| number)
        .unwrap();
    // Some "clever" math so we don't need to parse strings :P
    let ret = first_digit * 10 + second_digit;
    return dbg!(ret);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::strings::SkipEmptyLines;
    use itertools::Itertools;
    use nofmt;
    use pretty_assertions::assert_eq as pretty_assert_eq;
    use textwrap::dedent;

    fn to_vec(f: Vec<&str>) -> Vec<String> {
        f.into_iter().map(|s| String::from(s)).collect_vec()
    }

    fn program() -> String {
        return dedent(
            "
            1abc2
            pqr3stu8vwx
            a1b2c3d4e5f
            treb7uchet",
        )
        .skip_empty_start_lines();
    }
    #[test]
    fn test_part1_inner() -> Result<()> {
        let input_string = program();
        let actual = inner(&input_string)?.0;
        assert_eq!(actual, 142);

        Ok(())
    }

    #[test]
    fn test_part2_inner() -> Result<()> {
        let input_string = dedent(
            "
            two1nine
            eightwothree
            abcone2threexyz
            xtwone3four
            4nineeightseven2
            zoneight234
            7pqrstsixteen
        ",
        )
        .skip_empty_start_lines();
        let actual = inner(&input_string)?.1;
        assert_eq!(actual, 281);
        Ok(())
    }

    #[test]
    fn test_parse_digits() {
        assert_eq!(parse_digits("two1nine"), 29);
        assert_eq!(parse_digits("eightwothree"), 83);
        assert_eq!(parse_digits("abcone2threexyz"), 13);
        assert_eq!(parse_digits("xtwone3four"), 24);
        assert_eq!(parse_digits("4nineeightseven2"), 42);
        assert_eq!(parse_digits("zoneight234"), 14);
        assert_eq!(parse_digits("7pqrstsixteen"), 76);
    }
}
