use crate::containers::split_into_slices;
use crate::files::lines;
use itertools::Itertools;
use ndarray::prelude::*;
use std::fmt::{Debug, Display, Write};
use std::iter::zip;
use std::num::ParseIntError;
use Instruction::*;

type PuzzleResult = i32;
type PuzzleResultPart2 = String;

#[derive(Eq, Hash, Ord, PartialOrd, PartialEq, Debug, Copy, Clone)]
pub struct Position(pub i32, pub i32);

pub fn day10() -> (PuzzleResult, PuzzleResultPart2) {
    let lines_of_file = lines("inputs/2022/day10.txt");
    inner(lines_of_file)
}

#[derive(Eq, PartialEq, Debug, Copy, Clone)]
enum Instruction {
    Noop,
    AddX(i32),
}

fn parse(lines: Vec<String>) -> Result<Vec<Instruction>, String> {
    let ret = lines
        .iter()
        .map(|line| {
            let chars = line.split_ascii_whitespace().collect_vec();
            let slice = chars.as_slice();
            match slice {
                ["noop"] => Ok(Noop),
                ["addx", num] => {
                    let num = num.parse().map_err(|err: ParseIntError| err.to_string())?;
                    Ok(AddX(num))
                }
                _ => Err(format!("Can't parse line: {}", line)),
            }
        })
        .try_collect()?;
    Ok(ret)
}

fn inner(lines: Vec<String>) -> (PuzzleResult, PuzzleResultPart2) {
    let instructions = parse(lines).unwrap();

    let part1 = part1(&instructions);

    let part2_cycles = &[40, 80, 120, 160, 200, 240];
    let x_values = simulate(&instructions, part2_cycles);
    let part2 = render(x_values.iter());

    (part1, part2)
}

fn part1(instructions: &Vec<Instruction>) -> PuzzleResult {
    let part1_cycles = &[20, 60, 100, 140, 180, 220];
    let x_values = simulate(&instructions, part1_cycles);
    let part1 = zip(part1_cycles, x_values)
        .map(|(cycle, x_vals)| {
            let x = x_vals.last().unwrap().x;
            (*cycle as i32) * x
        })
        .sum();
    part1
}

fn ticks(instructions: &Vec<Instruction>) -> Vec<Instruction> {
    let mut ret = vec![];
    let mut cycle = 0;
    for instruction in instructions.iter() {
        match instruction {
            Noop => ret.push(instruction.clone()),
            AddX(_) => {
                ret.push(Noop);
                cycle += 1;
                ret.push(instruction.clone());
            }
        };
        cycle += 1;
    }
    ret
}

#[derive(Copy, Clone, Debug, PartialEq)]
struct XValue {
    x: i32
}

impl XValue {
    fn new(x: i32) -> XValue {
        XValue { x }
    }
    fn val(x: i32) -> XValue {
        XValue { x }
    }
}

fn render<'a>(x_values: impl Iterator<Item = &'a Vec<XValue>>) -> String {
    let mut lines: Vec<String> = vec![];
    for x_values_in_line in x_values {
        let mut line = vec![];
        for (pixel_usize, x_val) in x_values_in_line.iter().enumerate() {
            let pixel = pixel_usize as i32;
            let x = x_val.x;
            if x - 1 <= pixel && pixel <= x + 1 {
                line.push("#");
            } else {
                line.push(".");
            }
        }
        lines.push(line.join(""));
    }
    lines.join("\n")
}

fn simulate<const N: usize>(
    instructions: &Vec<Instruction>,
    cycles_to_capture_value_of_x: &[usize; N],
) -> [Vec<XValue>; N] {
    let mut x = 1;
    let ticks = ticks(&instructions);
    let mut tracked_x_vals = Vec::with_capacity(ticks.len());
    for (_i, instruction) in ticks.into_iter().enumerate() {
        let delta_x = match instruction {
            Noop => 0,
            AddX(val) => val,
        };
        tracked_x_vals.push(XValue::new(x));
        x += delta_x;
    }
    //Report
    let slices = split_into_slices(&tracked_x_vals, cycles_to_capture_value_of_x).unwrap();
    let ret = std::array::from_fn(|i| Vec::from(slices[i]));
    ret
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

    fn big_program() -> String {
        dedent(
            "
        addx 15
        addx -11
        addx 6
        addx -3
        addx 5
        addx -1
        addx -8
        addx 13
        addx 4
        noop
        addx -1
        addx 5
        addx -1
        addx 5
        addx -1
        addx 5
        addx -1
        addx 5
        addx -1
        addx -35
        addx 1
        addx 24
        addx -19
        addx 1
        addx 16
        addx -11
        noop
        noop
        addx 21
        addx -15
        noop
        noop
        addx -3
        addx 9
        addx 1
        addx -3
        addx 8
        addx 1
        addx 5
        noop
        noop
        noop
        noop
        noop
        addx -36
        noop
        addx 1
        addx 7
        noop
        noop
        noop
        addx 2
        addx 6
        noop
        noop
        noop
        noop
        noop
        addx 1
        noop
        noop
        addx 7
        addx 1
        noop
        addx -13
        addx 13
        addx 7
        noop
        addx 1
        addx -33
        noop
        noop
        noop
        addx 2
        noop
        noop
        noop
        addx 8
        noop
        addx -1
        addx 2
        addx 1
        noop
        addx 17
        addx -9
        addx 1
        addx 1
        addx -3
        addx 11
        noop
        noop
        addx 1
        noop
        addx 1
        noop
        noop
        addx -13
        addx -19
        addx 1
        addx 3
        addx 26
        addx -30
        addx 12
        addx -1
        addx 3
        addx 1
        noop
        noop
        noop
        addx -9
        addx 18
        addx 1
        addx 2
        noop
        noop
        addx 9
        noop
        noop
        noop
        addx -1
        addx 2
        addx -37
        addx 1
        addx 3
        noop
        addx 15
        addx -21
        addx 22
        addx -6
        addx 1
        noop
        addx 2
        addx 1
        noop
        addx -10
        noop
        noop
        addx 20
        addx 1
        addx 2
        addx 2
        addx -6
        addx -11
        noop
        noop
        noop",
        )
        .skip_empty_start_lines()
    }
    #[test]
    fn test_part1_inner() -> Result<(), String> {
        let input_string = big_program();
        let actual = inner(to_vec(input_string.lines().collect_vec())).0;
        assert_eq!(actual, 13140);

        Ok(())
    }

    #[test]
    fn test_part2_inner() -> Result<(), String> {
        let input_string = big_program();
        let actual = inner(to_vec(input_string.lines().collect_vec())).1;
        assert_ne!(actual, "");

        Ok(())
    }

    #[test]
    fn test_parse() -> Result<(), String> {
        let input_string = dedent(
            "
            noop
            addx 3
            addx -5",
        )
        .skip_empty_start_lines();
        let lines = to_vec(input_string.lines().collect_vec());
        let instructions = parse(lines)?;
        assert_eq!(
            instructions,
            vec![
                Instruction::Noop,
                Instruction::AddX(3),
                Instruction::AddX(-5),
            ]
        );
        Ok(())
    }

    #[test]
    fn test_simulate() -> Result<(), String> {
        let instructions = vec![
            Instruction::Noop,
            Instruction::AddX(3),
            Instruction::AddX(-5),
            Instruction::Noop,
        ];
        let actual = simulate(&instructions, &[1, 2, 3, 4, 5, 6]);
        nofmt::pls! {
            let expected = [
                // Cycle 1
                vec![XValue::new(1)],
                // Cycle 2
                vec![XValue::new(1)],
                // Cycle 3
                vec![XValue::new(1)],
                // Cycle 4
                vec![XValue::new(4)],
                // Cycle 5
                vec![XValue::new(4)],
                // Cycle 6
                vec![XValue::new(-1)],
            ];
        }
        assert_eq!(actual, expected);
        Ok(())
    }

    #[test]
    fn test_ticks() {
        let actual = ticks(&vec![
            Instruction::Noop,
            Instruction::AddX(1),
            Instruction::Noop,
            Instruction::Noop,
            Instruction::AddX(3),
        ]);
        assert_eq!(
            actual,
            vec![
                Instruction::Noop,
                Instruction::Noop,
                Instruction::AddX(1),
                Instruction::Noop,
                Instruction::Noop,
                Instruction::Noop,
                Instruction::AddX(3),
            ]
        );
    }

    #[test]
    fn test_simulate_long_instructions() -> Result<(), String> {
        let lines = to_vec(big_program().lines().collect_vec());
        let instructions = parse(lines)?;
        let vals = simulate(&instructions, &[20, 60, 100, 140, 180, 220]);
        // let vals = simulate(&instructions, &[180, 220]);

        let lengths = vals.iter().map(|v| v.len()).collect_vec();
        pretty_assert_eq!(lengths, vec![20, 40, 40, 40, 40, 40]);

        let x_vals = vals.iter().map(|v| v.last().unwrap().x).collect_vec();
        pretty_assert_eq!(x_vals, vec![21, 19, 18, 21, 16, 18]);

        nofmt::pls! {
            let expected = vec![
                XValue::new(1),
                // XValue::new(1),
                 // +15
                XValue::new(1),
                XValue::new(16),
                // -11
                XValue::new(16),
                XValue::new(5),
                 // +6
                XValue::new(5),
                XValue::new(11),
                 // -3
                XValue::new(11),
                XValue::new(8),
                 // +5
                XValue::new(8),
                XValue::new(13),
                 // -1
                XValue::new(13),
                XValue::new(12),
                 // -8
                XValue::new(12),
                XValue::new(4),
                 // +13
                XValue::new(4),
                XValue::new(17),
                 // +4
                XValue::new(17),
                XValue::new(21),
                XValue::new(21),
            ];
        };
        assert_eq!(vals[0], expected);

        Ok(())
    }

    #[test]
    fn test_render() {
        let vals = [
            [1, 3, 5, 5, 5],  //
            [1, 1, 1, 1, 1],  //
            [-1, 0, 1, 3, 3], //
        ];
        let vals = vals.map(|line| {
            let v = line.map(|val| XValue::val(val)).into_iter().collect_vec();
            v
        });
        let actual = render(vals.iter());
        pretty_assert_eq!(
            actual,
            dedent(
                "
            #...#
            ###..
            #####"
            )
            .skip_empty_start_lines()
        )
    }
}
