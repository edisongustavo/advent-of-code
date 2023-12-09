use crate::files::contents;
use anyhow::{bail, Context, Result};
use itertools::Itertools;
use ndarray::prelude::*;
use rayon::prelude::*;
use std::cmp::min;
use std::fmt::{Debug, Display, Write};
use std::iter::zip;
use std::ops::Add;
use strum::EnumCount;
use strum::IntoEnumIterator;
use strum_macros::{Display, EnumCount, EnumIter};

type PuzzleResult = usize;

pub fn solution() -> (PuzzleResult, PuzzleResult) {
    inner(&contents("inputs/2023/day6.txt")).unwrap()
}

fn inner(input: &String) -> Result<(PuzzleResult, PuzzleResult)> {
    fn solve(races: &Vec<Race>) -> PuzzleResult {
        races.iter()
            .map(|race| {
                let t = race.time as f64;
                let d = race.distance as f64;
                let delta = (t * t - 4. * d).sqrt();

                let max_time = (-t - delta) / -2.;
                let min_time = (-t + delta) / -2.;
                let offset = if max_time.fract() == 0.0 && min_time.fract() == 0.0 {
                    -1.
                } else {
                    1.
                };

                dbg!(dbg!(max_time.floor()) - dbg!(min_time.ceil()) + dbg!(offset))
            })
            .product::<f64>() as PuzzleResult
    }

    let races = &parse(input, true)?;
    let part1 = solve(races);
    let races2 = &parse(input, false)?;
    let part2 = solve(races2);

    Ok((part1, part2))
}

struct Race {
    time: usize,
    distance: usize,
}

fn parse(input: &String, multiple_races: bool) -> Result<Vec<Race>> {
    let lines = input.lines().collect_vec();
    let parse_nums = |line: &str| -> Result<Vec<usize>> {
        let ret = if multiple_races {
            line
                .trim()
                .split_ascii_whitespace()
                .map(|elem| elem.trim().parse::<usize>())
                .try_collect()?
        } else {
            vec![line
                .trim()
                .chars()
                .filter(|c| *c != ' ')
                .join("")
                .trim().parse::<usize>()?]
        };
        Ok(ret)
    };
    let times = parse_nums(&lines[0]["Time:".len()..])?;
    let distances = parse_nums(&lines[1]["Distance:".len()..])?;
    let ret = zip(times, distances)
        .map(|(time, distance)| Race { time, distance })
        .collect_vec();
    Ok(ret)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::strings::SkipEmptyLines;
    use nofmt;
    use textwrap::dedent;

    fn program() -> String {
        return dedent(
            "
            Time:      7  15   30
            Distance:  9  40  200",
        )
        .skip_empty_start_lines();
    }
    #[test]
    fn test_part1_inner() -> Result<()> {
        let input_string = program();
        let actual = inner(&input_string)?.0;
        assert_eq!(actual, 288);

        Ok(())
    }

    #[test]
    fn test_part2_inner() -> Result<()> {
        let input_string = program();
        let actual = inner(&input_string)?.1;
        assert_eq!(actual, 71503);
        Ok(())
    }
}
