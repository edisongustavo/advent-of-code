use crate::solutions2022::day1::part1;
use crate::solutions2022::day1::part2;
use crate::solutions2022::day2::{day2part1, day2part2};
use crate::solutions2022::day3::{day3part1, day3part2};
use crate::solutions2022::day4::{day4part1, day4part2};
use crate::solutions2022::day5::day5;
use crate::solutions2022::day6::day6;
use crate::solutions2022::day7::day7;
use crate::solutions2022::day8::day8;
use crate::solutions2022::day9::day9;
use crate::solutions2022::day10::day10;
use crate::solutions2022::day11::day11;

mod containers;
mod solutions2022;
mod solutions2023;
mod files;
mod geometry;
mod strings;
mod array;
mod asserts;

fn main() {
    // println!("Day1 part1: {}", part1());
    // println!("Day1 part2: {}", part2());
    //
    // println!("Day2 part1: {}", day2part1());
    // println!("Day2 part2: {}", day2part2());
    //
    // println!("Day3 part1: {}", day3part1());
    // println!("Day3 part2: {}", day3part2());
    //
    // println!("Day4 part1: {}", day4part1());
    // println!("Day4 part2: {}", day4part2());
    //
    // println!("Day5: {:?}", day5());
    // println!("Day6: {:?}", day6());
    // println!("Day7: {:?}", day7());
    // println!("Day8: {:?}", day8());
    // println!("Day9: {:?}", day9());
    // let day10_answer = day10();
    // println!("Day10: {:?}", day10_answer.0);
    // println!("{}", day10_answer.1);

    // println!("Day11: {:?}", day11());

    println!("********");
    println!("2023");
    println!("********");
    println!("Day1: {:?}", solutions2023::day1::solution());
    println!("Day2: {:?}", solutions2023::day2::solution());
    println!("Day3: {:?}", solutions2023::day3::solution());
    println!("Day4: {:?}", solutions2023::day4::solution());
}
