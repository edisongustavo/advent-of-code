use crate::files::lines;
use crate::strings;
use itertools::Itertools;

pub fn day3part1() -> i32 {
    let lines_of_file = lines("inputs/2022/day3.txt");
    day3part1_inner(lines_of_file)
}

pub fn day3part2() -> i32 {
    let lines_of_file = lines("inputs/2022/day3.txt");
    day3part2_inner(lines_of_file)
}

struct Rucksack {
    common: char,
}

fn calculate_priority(rucksack: Rucksack) -> i32 {
    let c = rucksack.common;
    assert!(rucksack.common.is_ascii());
    let value = if rucksack.common.is_lowercase() {
        (c as u32) - ('a' as u32) + 1
    } else {
        (c as u32) - ('A' as u32) + 27
    };
    value as i32
}

fn parse_line(line: &str) -> Rucksack {
    let (left, right) = line.split_at(line.len() / 2);
    let common = strings::common_char(&vec![left, right]);

    Rucksack { common }
}

fn day3part1_inner(lines: Vec<String>) -> i32 {
    lines
        .iter()
        .filter(|line| !line.trim().is_empty())
        .map(|line| parse_line(line))
        .map(|rucksack| calculate_priority(rucksack))
        .sum()
}

fn day3part2_inner(lines: Vec<String>) -> i32 {
    lines
        .iter()
        .filter(|line| !line.trim().is_empty())
        .chunks(3)
        .into_iter()
        .map(|chunk| parse_chunk(chunk.collect_vec()))
        .map(|rucksack| calculate_priority(rucksack))
        .sum()
}

fn parse_chunk(chunk: Vec<&String>) -> Rucksack {
    let common = strings::common_char(&vec![chunk[0], chunk[1], chunk[2]]);
    Rucksack { common }
}

#[cfg(test)]
mod tests {
    use super::*;
    use itertools::Itertools;

    fn to_vec(f: Vec<&str>) -> Vec<String> {
        f.into_iter().map(|s| String::from(s)).collect_vec()
    }

    #[test]
    fn test_part1_inner() {
        assert_eq!(
            day3part1_inner(to_vec(vec!["vJrwpWtwJgWrhcsFMMfFFhFp"])),
            16
        );
        assert_eq!(
            day3part1_inner(to_vec(vec!["jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL"])),
            38
        );
        assert_eq!(
            day3part1_inner(to_vec(vec![
                "vJrwpWtwJgWrhcsFMMfFFhFp",
                "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL"
            ])),
            16 + 38
        );
    }

    #[test]
    fn test_part2_inner() {
        assert_eq!(
            day3part2_inner(to_vec(vec![
                "vJrwpWtwJgWrhcsFMMfFFhFp",
                "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL",
                "PmmdzqPrVvPwwTWBwg"
            ])),
            18
        );
    }
}
