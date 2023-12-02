use crate::files::lines;
use itertools::Itertools;

pub fn parse_elves_calories() -> Vec<i32> {
    let lines = lines("inputs/2022/day1.txt");

    let mut elves_calories = Vec::new();
    for (_key, group) in &lines.into_iter().group_by(|line| !line.is_empty()) {
        let c: i32 = group
            .take_while(|line| !line.is_empty())
            .map(|line| line.parse::<i32>().unwrap())
            .sum();
        elves_calories.push(c);
    }
    return elves_calories;
}

pub fn part1() -> i32 {
    parse_elves_calories()
        .into_iter()
        .max()
        .expect("day1.txt file is empty")
}

pub fn part2() -> i32 {
    let mut calories = parse_elves_calories();
    calories.sort();
    return calories.iter().rev().take(3).sum();
}
