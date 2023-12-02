use crate::containers::get_mut2;
use crate::strings::SkipEmptyLines;
use itertools::Itertools;
use regex::Regex;
use std::fmt::{Debug, Display, Formatter};
use std::fs;

pub fn day5() -> (String, String) {
    let contents = fs::read_to_string("inputs/2022/day5.txt").unwrap();
    inner(contents).unwrap()
}

#[derive(Debug)]
struct Movement {
    n: usize,
    source: usize,
    target: usize,
}

impl Movement {
    pub(crate) fn new(n: usize, source: usize, target: usize) -> Movement {
        Movement {
            n,
            source: source - 1,
            target: target - 1,
        }
    }
}

#[derive(Clone)]
struct Cranes {
    layout: Vec<Vec<char>>,
}

impl Cranes {
    pub(crate) fn get_top_crates(&self) -> String {
        self.layout
            .iter()
            .map(|stack| stack.last().unwrap_or(&' '))
            .join("")
    }
}

impl Display for Cranes {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let height = self
            .layout
            .iter()
            .map(|f| f.len())
            .max()
            .expect("Invalid layout");
        let width = self.layout.len();
        let mut s = String::new();

        for i in (0..height).rev() {
            for j in 0..width {
                let stack = &self.layout[j];
                if i >= stack.len() {
                    s += "    ";
                } else {
                    let string = format!("[{}] ", stack[i]);
                    s += string.as_str();
                }
            }
            s = format!("{}\n", s.trim_end());
        }
        for i in 0..width {
            s += format!(" {}  ", i + 1).as_str();
        }
        s = format!("{}", s.trim_end());
        f.write_str(s.as_str())
    }
}

impl Cranes {
    fn move_one_crate_per_movement(&mut self, movement: &Movement) -> Result<(), String> {
        for _ in 0..movement.n {
            let source = &mut self.layout[movement.source];
            let elem = source
                .pop()
                .ok_or("Can't execute movement {movement:?} because the source stack is empty.")?;

            let target = &mut self.layout[movement.target];
            target.push(elem);
        }
        Ok(())
    }
    fn move_multiple_crates_per_movement(&mut self, movement: &Movement) -> Result<(), String> {
        let (source, target) =
            get_mut2(self.layout.as_mut_slice(), movement.source, movement.target);
        if source.len() < movement.n {
            return Err(format!("Source stack does not have enough crates for this movement. Source stack: {source:?}, movement: {movement:?}"));
        }
        for elem in source.iter().rev().take(movement.n).rev() {
            target.push(*elem);
        }
        source.resize(source.len() - movement.n, ' ');
        Ok(())
    }
}

fn inner(input: String) -> Result<(String, String), String> {
    let input = input.skip_empty_start_lines();
    let lines = input.lines();

    // Now find the division between the crane and the movements
    let (pos, rest_of_text) = lines
        .clone()
        .find_position(|line| line.trim().is_empty())
        .ok_or("Malformed input")?;

    // TODO: remove these clone() calls
    let crane_lines = lines.clone().take(pos).collect_vec();
    let movements_lines = lines.skip(pos + 1).collect_vec();
    let movements = parse_movements(movements_lines)?;

    let mut cranes_part1 = parse_crane_setup(crane_lines).unwrap();
    let mut cranes_part2 = cranes_part1.clone();

    for mov in &movements {
        cranes_part1.move_one_crate_per_movement(&mov)?;
    }
    let part1 = cranes_part1.get_top_crates();

    for mov in &movements {
        cranes_part2.move_multiple_crates_per_movement(&mov)?;
    }
    let part2 = cranes_part2.get_top_crates();
    // let part2 = " ".to_string();

    Ok((part1, part2))
}

fn parse_movements(movements: Vec<&str>) -> Result<Vec<Movement>, String> {
    let movements = movements.skip_empty_start_lines();

    let re = Regex::new(r"^move (?<n>\d+) from (?<source>\d) to (?<target>\d)$").unwrap();
    movements
        .iter()
        .map(|line| match re.captures(line) {
            Some(caps) => {
                let n = caps["n"].parse::<usize>().unwrap();
                let source = caps["source"].parse::<usize>().unwrap();
                let target = caps["target"].parse::<usize>().unwrap();
                let movement = Movement::new(n, source, target);
                return Ok(movement);
            }
            None => Err(format!("Invalid input: {line}")),
        })
        .collect()
}

fn parse_crane_setup(configuration_lines: Vec<&str>) -> Result<Cranes, &str> {
    let lines = configuration_lines.skip_empty_start_lines();

    let last_line = lines.last().ok_or("Can't parse ")?;
    let number_of_stacks = (last_line.len() + 2) / 4;

    let mut layout = Vec::new();
    for i in 0..number_of_stacks {
        layout.push(Vec::new());
    }

    for line in lines.iter().rev().skip(1) {
        for i in 0..number_of_stacks {
            let index = i * 4;
            let crate_name = line.chars().nth(index + 1);
            match crate_name {
                Some(c) => {
                    if c != ' ' {
                        let stack = &mut layout[i];
                        stack.push(c);
                    }
                }
                None => continue,
            }
        }
    }

    return Ok(Cranes { layout: layout });
}

#[cfg(test)]
mod tests {
    use expect_test::expect;
    use itertools::Itertools;
    use pretty_assertions::{assert_eq, assert_ne};
    use textwrap::dedent;

    use super::*;
    use crate::strings::SkipEmptyLines;

    #[test]
    fn test_part1_inner() -> Result<(), String> {
        assert_eq!(
            inner(dedent(
                "
                [D]
            [N] [C]
            [Z] [M] [P]
             1   2   3

            move 1 from 2 to 1
            move 3 from 1 to 3
            move 2 from 2 to 1
            move 1 from 1 to 2
            "
            ))?
            .0,
            "CMZ"
        );
        Ok(())
    }

    #[test]
    fn test_part2_inner() -> Result<(), String> {
        assert_eq!(
            inner(dedent(
                "
                [D]
            [N] [C]
            [Z] [M] [P]
             1   2   3

            move 1 from 2 to 1
            move 3 from 1 to 3
            move 2 from 2 to 1
            move 1 from 1 to 2
            "
            ))?
            .1,
            "MCD"
        );
        Ok(())
    }

    #[test]
    fn test_parse_crane_setup() -> Result<(), String> {
        let crane_layout = dedent(
            "
                [D]
            [N] [C]
            [Z] [M] [P]
             1   2   3",
        )
        .skip_empty_start_lines();
        let cranes = parse_crane_setup(crane_layout.lines().collect_vec())?;
        assert_eq!(cranes.layout.len(), 3);
        assert_eq!(
            cranes.layout.iter().map(|v| v.len()).collect_vec(),
            vec![2, 3, 1]
        );
        let actual_str = cranes.to_string();
        assert_eq!(actual_str, crane_layout);
        Ok(())
    }

    #[test]
    fn test_parse_crane_setup_from_day5() -> Result<(), String> {
        let crane_layout = dedent(
            "
            [F]         [L]     [M]
            [T]     [H] [V] [G] [V]
            [N]     [T] [D] [R] [N]     [D]
            [Z]     [B] [C] [P] [B] [R] [Z]
            [M]     [J] [N] [M] [F] [M] [V] [H]
            [G] [J] [L] [J] [S] [C] [G] [M] [F]
            [H] [W] [V] [P] [W] [H] [H] [N] [N]
            [J] [V] [G] [B] [F] [G] [D] [H] [G]
             1   2   3   4   5   6   7   8   9",
        );
        let actual = parse_crane_setup(crane_layout.lines().collect_vec())?;
        assert_eq!(actual.layout.len(), 9);
        assert_eq!(actual.to_string(), crane_layout.skip_empty_start_lines());
        Ok(())
    }

    #[test]
    fn test_move_one_crate_per_movement() {
        let mut cranes = Cranes {
            layout: vec![vec!['Z', 'N'], vec!['M', 'C', 'D'], vec!['P']],
        };
        cranes.move_one_crate_per_movement(&Movement::new(1, 2, 1));
        expect![["
            [D]
            [N] [C]
            [Z] [M] [P]
             1   2   3"]]
        .assert_eq(&cranes.to_string());
    }

    #[test]
    fn test_move_multiple_crates_per_movement() {
        let mut cranes = Cranes {
            layout: vec![vec!['Z', 'N'], vec!['M', 'C', 'D'], vec!['P']],
        };
        expect![["
                [D]
            [N] [C]
            [Z] [M] [P]
             1   2   3"]]
        .assert_eq(&cranes.to_string());

        cranes.move_multiple_crates_per_movement(&Movement::new(1, 2, 1));
        expect![["
            [D]
            [N] [C]
            [Z] [M] [P]
             1   2   3"]]
        .assert_eq(&cranes.to_string());

        cranes.move_multiple_crates_per_movement(&Movement::new(2, 1, 3));
        expect![["
                    [D]
                [C] [N]
            [Z] [M] [P]
             1   2   3"]]
        .assert_eq(&cranes.to_string());
    }

    #[test]
    fn test_display_crane() {
        let cranes = Cranes {
            layout: vec![vec!['Z', 'N'], vec!['M', 'C', 'D'], vec!['P']],
        };
        expect![["
                [D]
            [N] [C]
            [Z] [M] [P]
             1   2   3"]]
        .assert_eq(&cranes.to_string());
    }

    #[test]
    fn test_cranes_clone() {
        let cranes1 = Cranes {
            layout: vec![vec!['Z', 'N'], vec!['M', 'C', 'D'], vec!['P']],
        };
        let mut cranes2 = cranes1.clone();
        let cranes1_string = cranes1.to_string();
        assert_eq!(cranes1_string, cranes2.to_string());

        cranes2.layout[0].pop();
        assert_ne!(cranes1_string, cranes2.to_string());
    }
}
