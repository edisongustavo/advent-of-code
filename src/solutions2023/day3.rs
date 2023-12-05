use crate::files::contents;
use anyhow::{Context, Result};
use itertools::Itertools;
use map_macro::hash_set;
use ndarray::prelude::*;
use pest::Parser;
use std::cmp::min;
use std::collections::{HashMap, HashSet};
use std::fmt::{Debug, Display, Write};
use std::iter::zip;

type PuzzleResult = u32;

pub fn solution() -> (PuzzleResult, PuzzleResult) {
    inner(&contents("inputs/2023/day3.txt")).unwrap()
}

fn inner(input: &String) -> Result<(PuzzleResult, PuzzleResult)> {
    let grid = parse(input)?;
    let adjacent_symbols = grid
        .symbols_with_adjacent_part_numbers();
    let part1 = adjacent_symbols
        .values()
        .flatten()
        .sum();
    let part2 = adjacent_symbols
        .iter()
        .filter_map(|(key, value)| {
            let (char, _) = key;
            if *char == '*' && value.len() == 2 {
                Some(value)
            } else {
                None
            }
        })
        .map(|values| values.iter().product::<PuzzleResult>())
        .sum();

    Ok((part1, part2))
}

struct Grid {
    elems: Array2<i32>,
}

type Pos = (usize, usize);

impl Grid {
    fn new(size: (usize, usize)) -> Grid {
        Grid {
            elems: Array2::zeros(size),
        }
    }
    fn set(&mut self, element: Element) {
        match element.value {
            ElementValue::Symbol(symbol) => {
                self.elems[element.pos] = -(symbol as i32);
            }
            ElementValue::Number(num) => {
                self.elems[element.pos] = num as i32;
            }
        };
    }
    fn add(&mut self, element: Element) {
        match element.value {
            ElementValue::Symbol(symbol) => {
                self.elems[element.pos] = -(symbol as i32);
            }
            ElementValue::Number(num) => {
                let num_digits = num.checked_ilog10().unwrap_or(0) + 1;
                for i in 0..num_digits {
                    let pos = (element.pos.0, element.pos.1 + i as usize);
                    self.elems[pos] = num as i32;
                }
            }
        };
    }
    fn symbols_with_adjacent_part_numbers(&self) -> HashMap<(char, (usize, usize)), HashSet<u32>> {
        let mut ret = HashMap::default();
        let shape = self.elems.shape();
        for i in 0..shape[0] {
            for j in 0..shape[1] {
                let symbol = self.elems[[i, j]];
                if symbol >= 0 {
                    // We only want process this if we're in a symbol (< 0), otherwise it doesn't matter
                    continue;
                }
                let symbol: char = (-symbol as u8) as char;
                let positions = s![
                    i.saturating_sub(1)..min(i + 2, shape[0]),
                    j.saturating_sub(1)..min(j + 2, shape[1]),
                ];
                let vals = self.elems.slice(positions);
                let part_numbers: HashSet<&i32> = vals.iter().filter(|v| **v > 0).collect();
                for part_number in part_numbers {
                    let key = (symbol, (i, j));
                    let entry = ret.entry(key);
                    let parts_of_symbol: &mut HashSet<u32> = entry.or_default();
                    parts_of_symbol.insert(*part_number as u32);
                }
            }
        }
        ret
    }
}

struct Element {
    pos: Pos,
    value: ElementValue,
}
#[derive(Copy, Clone)]
enum ElementValue {
    // Empty,
    Symbol(char),
    Number(u32),
}

fn parse(input: &String) -> Result<Grid> {
    let lines = input.lines().collect_vec();
    let mut grid = Grid::new((lines.len(), lines[0].len()));
    for (x, line) in lines.into_iter().enumerate() {
        // Split all elements together, while keeping numbers into their own group
        let (numbers, elements): (Vec<bool>, Vec<String>) = line.chars()
            .group_by(|c| c.is_digit(10))
            .into_iter()
            .map(|(is_number, mut group)| {
                if is_number {
                    vec![(true, group.join(""))]
                } else {
                    group.map(|c| (false, c.to_string())).collect_vec()
                }
            })
            .flatten()
            // Unzip so that it's easier to debug
            .unzip();
        let mut y = 0;
        for (is_number, elem) in zip(numbers, elements) {
            if elem == "." {
                y += 1;
                continue;
            }

            let value = match is_number {
                true => {
                    let parsed_value = elem.parse::<u32>().context(format!(
                        "Couldn't parse '{elem}' from the line '{line}'"
                    ))?;
                    ElementValue::Number(parsed_value)
                },
                false => {
                    let c = elem.chars().nth(0).unwrap();
                    ElementValue::Symbol(c)
                },
            };
            grid.add(Element {
                pos: (x, y),
                value: value.clone(),
            });
            y += elem.len();
        }
    }
    Ok(grid)
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
    use crate::asserts;

    fn to_vec(f: Vec<&str>) -> Vec<String> {
        f.into_iter().map(|s| String::from(s)).collect_vec()
    }

    fn program() -> String {
        return dedent(
            "
            467..114..
            ...*......
            ..35..633.
            ......#...
            617*......
            .....+.58.
            ..592.....
            ......755.
            ...$.*....
            .664.598..",
        )
        .skip_empty_start_lines();
    }
    #[test]
    fn test_part1_inner() -> Result<()> {
        let input_string = program();
        let actual = inner(&input_string)?.0;
        assert_eq!(actual, 4361);

        Ok(())
    }

    #[test]
    fn test_part2_inner() -> Result<()> {
        let input_string = program();
        let actual = inner(&input_string)?.1;
        assert_eq!(actual, 467835);
        Ok(())
    }

    #[test]
    fn test_parse() -> Result<()> {
        let input_string = program();
        let grid = parse(&input_string)?;
        nofmt::pls! {
        let expected_array = array![
            [467, 467, 467,   0,   0, 114, 114, 114,   0, 0],
            [  0,   0,   0, -42,   0,   0,   0,   0,   0, 0],
            [  0,   0,  35,  35,   0,   0, 633, 633, 633, 0],
            [  0,   0,   0,   0,   0,   0, -35,   0,   0, 0],
            [617, 617, 617, -42,   0,   0,   0,   0,   0, 0],
            [  0,   0,   0,   0,   0, -43,   0, 58,   58, 0],
            [  0,   0, 592, 592, 592,   0,   0,   0,   0, 0],
            [  0,   0,   0,   0,   0,   0, 755, 755, 755, 0],
            [  0,   0,   0, -36,   0, -42,   0,   0,   0, 0],
            [  0, 664, 664, 664,   0, 598, 598, 598,   0, 0],
        ];
        }
        pretty_assert_eq!(grid.elems, expected_array);
        Ok(())
    }

    #[test]
    fn test_parse_2() -> Result<()> {
        // let input_string = "....%..863..#......................36.............956..337%......692..............*744....$..........*......../.....187..-..................";
        let input_string = "*744..456*..123";
        let grid = parse(&String::from(input_string))?;
        let expected_array = array![[
            -42, 744, 744, 744, 0, 0, 456, 456, 456, -42, 0, 0, 123, 123, 123
        ]];
        pretty_assert_eq!(grid.elems, expected_array);
        Ok(())
    }

    #[test]
    #[ignore]
    fn test_examples() -> Result<()> {
        // https://www.reddit.com/r/adventofcode/comments/189qyze/comment/kbtg5sd/?utm_source=share&utm_medium=web2x&context=3
        let input_string = dedent("
            ........
            .24..4..
            ......*.").skip_empty_start_lines();
        let grid = parse(&String::from(input_string))?;
        pretty_assert_eq!(grid.elems, array![
            [0, 0, 0, 0, 0, 0, 0, 0],
            [0, 24, 24, 0, 0, 4, 0, 0],
            [0, 0, 0, 0, 0, 0, -42, 0]
        ]);
        asserts::compare_maps(grid.symbols_with_adjacent_part_numbers(), hash_map![
            ('*', (2, 6)) => hash_set![4],
        ]);

        let input_string = dedent("
            ........
            .24$-4..
            ......*.").skip_empty_start_lines();
        let grid = parse(&String::from(input_string))?;
        pretty_assert_eq!(grid.elems, array![
            [0,  0,  0,   0,   0, 0,   0, 0],
            [0, 24, 24, -36, -45, 4,   0, 0],
            [0,  0,  0,   0,   0, 0, -42, 0]
        ]);
        asserts::compare_maps(grid.symbols_with_adjacent_part_numbers(), hash_map![
            ('$', (1, 3)) => hash_set![24],
            ('-', (1, 4)) => hash_set![4],
            ('*', (2, 6)) => hash_set![4],
        ]);

        let input_string = dedent("
            11....11
            ..$..$..
            11....11").skip_empty_start_lines();
        let grid = parse(&String::from(input_string))?;
        pretty_assert_eq!(grid.elems, array![
            [0, 0, 0, 0, 0, 0, 0, 0],
            [0, 24, 24, 0, 0, 4, 0, 0],
            [0, 0, 0, 0, 0, 0, -42, 0]
        ]);

        let input_string = dedent("
            $......$
            .1....1.
            .1....1.
            $......$").skip_empty_start_lines();
        let grid = parse(&String::from(input_string))?;
        pretty_assert_eq!(grid.elems, array![
            [0, 0, 0, 0, 0, 0, 0, 0],
            [0, 24, 24, 0, 0, 4, 0, 0],
            [0, 0, 0, 0, 0, 0, -42, 0]
        ]);

        let input_string = dedent("
            $......$
            .11..11.
            .11..11.
            $......$").skip_empty_start_lines();
        let grid = parse(&String::from(input_string))?;
        pretty_assert_eq!(grid.elems, array![
            [0, 0, 0, 0, 0, 0, 0, 0],
            [0, 24, 24, 0, 0, 4, 0, 0],
            [0, 0, 0, 0, 0, 0, -42, 0]
        ]);

        let input_string = dedent("
            $11
            ...
            11$
            ...").skip_empty_start_lines();
        let grid = parse(&String::from(input_string))?;
        pretty_assert_eq!(grid.elems, array![
            [0, 0, 0, 0, 0, 0, 0, 0],
            [0, 24, 24, 0, 0, 4, 0, 0],
            [0, 0, 0, 0, 0, 0, -42, 0]
        ]);

        let input_string = dedent("
            $..
            .11
            .11
            $..
            ..$
            11.
            11.
            ..$").skip_empty_start_lines();
        let grid = parse(&String::from(input_string))?;
        pretty_assert_eq!(grid.elems, array![
            [0, 0, 0, 0, 0, 0, 0, 0],
            [0, 24, 24, 0, 0, 4, 0, 0],
            [0, 0, 0, 0, 0, 0, -42, 0]
        ]);


        Ok(())

    }

    #[test]
    fn test_symbols_with_adjacent_part_numbers() -> Result<()> {
        let input_string = program();
        let grid = parse(&input_string)?;
        let actual = grid.symbols_with_adjacent_part_numbers();
        let expected = hash_map![
            ('*', (1, 3)) => hash_set![35, 467],
            ('#', (3, 6)) => hash_set![633],
            ('*', (4, 3)) => hash_set![617],
            ('+', (5, 5)) => hash_set![592],
            ('$', (8, 3)) => hash_set![664],
            ('*', (8, 5)) => hash_set![598, 755],
        ];
        asserts::compare_maps(actual, expected);
        Ok(())
    }
}
