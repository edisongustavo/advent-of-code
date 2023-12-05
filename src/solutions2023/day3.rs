use crate::files::contents;
use anyhow::{Context, Result};
use itertools::{Either, Itertools};
use map_macro::hash_set;
use ndarray::prelude::*;
use pest::Parser;
use std::cmp::min;
use std::collections::{HashMap, HashSet};
use std::fmt::{Debug, Display, Write};
use regex::Regex;
use regex_split::RegexSplit;

type PuzzleResult = u32;

pub fn solution() -> (PuzzleResult, PuzzleResult) {
    inner(&contents("inputs/2023/day3.txt")).unwrap()
}

fn inner(input: &String) -> Result<(PuzzleResult, PuzzleResult)> {
    let part2 = PuzzleResult::default();
    let grid = parse(input)?;
    // let part1 = PuzzleResult::default();
    let part1 = grid
        .symbols_with_adjacent_part_numbers()
        .values()
        .flatten()
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
        // dbg!(&self.elems);
        for i in 0..shape[0] {
            for j in 0..shape[1] {
                let symbol = self.elems[[i, j]];
                if symbol >= 0 {
                    // We only want process this if we're in a symbol (< 0), otherwise it doesn't matter
                    continue;
                }
                // dbg!((i, j));
                let symbol: char = (-symbol as u8) as char;
                let positions = s![
                    i.saturating_sub(1)..min(i + 2, shape[0]),
                    j.saturating_sub(1)..min(j + 2, shape[1]),
                ];
                // dbg!(&positions);
                let vals = self.elems.slice(positions);
                // assert_eq!(vals.shape(), [3, 3]);
                // dbg!(&vals);
                let part_numbers: HashSet<&i32> = vals.iter().filter(|v| **v > 0).collect();
                // dbg!(&part_numbers);
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
enum ElementValue {
    // Empty,
    Symbol(char),
    Number(u32),
}

fn parse(input: &String) -> Result<Grid> {
    let lines = input.lines().collect_vec();
    let mut grid = Grid::new((lines.len(), lines[0].len()));
    let re = Regex::new(r"\D").unwrap();
    for (x, line) in lines.into_iter().enumerate() {
        // let split = line.split_inclusive(".").collect_vec();
        let split  = re.split_inclusive(line).collect_vec();
        let mut y = 0;
        for elem in split.into_iter() {
            if elem == "." {
                y += 1;
                continue;
            }

            // The `split` method is a bit weird and can return items in very different flavors, such as:
            //
            // - %.
            // - *123.
            // - 123.
            // - 123 (if it's the last part of the string)
            //
            // So we need to handle all cases. For this we discard the chars with `.` and then group them
            let (number, symbols): (Vec<(String, Pos)>, Vec<(String, Pos)>) = elem
                // First we group the chars which are digits or not
                .chars()
                .filter(|c| *c != '.')
                .group_by(|c| c.is_digit(10))
                .into_iter()
                .map(|(key, mut group)| {
                    let s = group.join("");
                    (key, s)
                })
                // Now process the groups
                .enumerate()
                .partition_map(|(i, (is_number, s))| {
                    // We assume that the groups returned are in order that they appear in the string
                    let new_pos = (x, y);
                    y += s.len();
                    if is_number {
                        Either::Left((s, new_pos))
                    } else {
                        Either::Right((s, new_pos))
                    }
                });
            y += 1; // Because we dropped the `.`, so we need to increase 1 more time.
            assert!(number.len() <= 1);
            let number = number.first();

            if number.is_some() {
                let (number, pos) = number.unwrap();
                let val = number.parse::<u32>().context(format!(
                    "Couldn't parse '{number}', the element from the split is '{elem}', and the line is '{line}'"
                ))?;
                grid.add(Element {
                    pos: *pos,
                    value: ElementValue::Number(val),
                });
            };

            assert!(symbols.len() <= 1);
            symbols.first().map(|(symbol, pos)| {
                for (i, c) in symbol.chars().enumerate() {
                    grid.add(Element {
                        pos: (pos.0, pos.1 + i),
                        value: ElementValue::Symbol(c),
                    });
                }
            });
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
        assert_eq!(actual, 0);
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
    fn test_examples() -> Result<()> {
        // https://www.reddit.com/r/adventofcode/comments/189qyze/comment/kbtg5sd/?utm_source=share&utm_medium=web2x&context=3
        // let input_string = dedent("
        //     ........
        //     .24..4..
        //     ......*.").skip_empty_start_lines();
        // let grid = parse(&String::from(input_string))?;
        // pretty_assert_eq!(grid.elems, array![
        //     [0, 0, 0, 0, 0, 0, 0, 0],
        //     [0, 24, 24, 0, 0, 4, 0, 0],
        //     [0, 0, 0, 0, 0, 0, -42, 0]
        // ]);
        // compare_maps(grid.symbols_with_adjacent_part_numbers(), hash_map![
        //     ('*', (2, 6)) => hash_set![4],
        // ]);

        let input_string = dedent("
            ........
            .24$-4..
            ......*.").skip_empty_start_lines();
        let grid = parse(&String::from(input_string))?;
        pretty_assert_eq!(grid.elems, array![
            [0, 0, 0, 0, 0, 0, 0, 0],
            [0, 24, 24, -36, -45, 4, 0, 0],
            [0, 0, 0, 0, 0, 0, -42, 0]
        ]);
        compare_maps(grid.symbols_with_adjacent_part_numbers(), hash_map![
            ('$', (1, 3)) => hash_set![24],
            ('-', (1, 4)) => hash_set![4],
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
        compare_maps(actual, expected);
        Ok(())
    }

    fn compare_maps(actual: HashMap<(char, (usize, usize)), HashSet<u32>>, expected: HashMap<(char, (usize, usize)), HashSet<u32>>) {
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
}
