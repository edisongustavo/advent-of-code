use crate::containers::get_mut2;
use crate::files::lines;
use itertools::Itertools;
// use ndarray::prelude::*;
use std::collections::HashSet;
use std::fmt::{Debug, Display, Formatter, Write};
use std::ops;

type PuzzleResult = usize;

pub fn day9() -> (PuzzleResult, PuzzleResult) {
    let lines_of_file = lines("inputs/2022/day9.txt");
    inner(lines_of_file)
}

#[derive(Eq, Hash, Ord, PartialOrd, PartialEq, Debug, Copy, Clone)]
pub struct Position(pub i32, pub i32);

impl Position {
    fn manhattan_distance(&self, other: &Position) -> u32 {
        self.0.abs_diff(other.0) + self.1.abs_diff(other.1)
    }
    fn euclidean_distance(&self, other: &Position) -> u32 {
        let inner = (self.0 - other.0).pow(2) + (self.1 - other.1).pow(2);
        let sqrt = (inner as f64).sqrt();
        let rounded_up = sqrt.floor();
        rounded_up as u32
    }
    fn touching(&self, other: &Position) -> bool {
        self.0.abs_diff(other.0) <= 1 && self.1.abs_diff(other.1) <= 1
    }
}

impl Position {}

impl ops::Sub<Position> for Position {
    type Output = Position;

    fn sub(self, rhs: Position) -> Self::Output {
        Position(self.0 - rhs.0, self.1 - rhs.1)
    }
}

impl ops::Add<(i32, i32)> for Position {
    type Output = Position;

    fn add(self, rhs: (i32, i32)) -> Position {
        let x = self.0 + rhs.0;
        let y = self.1 + rhs.1;
        Position(x, y)
    }
}

impl ops::AddAssign<(i32, i32)> for Position {
    fn add_assign(&mut self, rhs: (i32, i32)) {
        self.0 += rhs.0;
        self.1 += rhs.1;
    }
}

#[derive(Clone)]
struct Rope(Vec<Position>);

impl Rope {
    fn new(rope_size: usize) -> Rope {
        Rope(vec![Position(0, 0); rope_size])
    }

    fn from_vec(knots: Vec<(i32, i32)>) -> Rope {
        let positions = knots.iter().map(|(x, y)| Position(*x, *y)).collect_vec();
        Rope(positions)
    }

    fn to_vec(&self) -> Vec<(i32, i32)> {
        self.0.iter().map(|pos| (pos.0, pos.1)).collect_vec()
        // let positions = knots.iter().map(|(x, y)| Position(*x, *y)).collect_vec();
        // Rope(positions)
    }

    fn tail(&self) -> &Position {
        self.0.last().unwrap()
    }

    fn head(&self) -> &Position {
        self.0.first().unwrap()
    }
}



#[derive(PartialOrd, PartialEq, Debug)]
enum Direction {
    Right,
    Left,
    Up,
    Down,
}

#[derive(PartialOrd, PartialEq, Debug)]
struct Movement {
    direction: Direction,
    size: usize,
}

impl Display for Movement {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let c = match self.direction {
            Direction::Right => 'R',
            Direction::Left => 'L',
            Direction::Up => 'U',
            Direction::Down => 'D',
        };
        f.write_char(c)?;
        f.write_char(' ')?;
        f.write_str(self.size.to_string().as_str())
    }
}

impl Movement {
    fn new(direction: Direction, size: usize) -> Movement {
        Movement { direction, size }
    }
}

fn parse(lines: Vec<String>) -> Result<Vec<Movement>, String> {
    let ret = lines
        .iter()
        .map(|line| {
            let chars = line.split_ascii_whitespace().collect_vec();
            let direction_char = chars[0];
            let direction = match direction_char {
                "R" => Direction::Right,
                "U" => Direction::Up,
                "D" => Direction::Down,
                "L" => Direction::Left,
                _ => return Err(format!("Invalid direction: {direction_char}")),
            };
            let size_char = chars[1];
            let size: usize = size_char
                .parse()
                .map_err(|err| format!("Invalid size: {size_char}"))?;
            return Ok(Movement { direction, size });
        })
        .try_collect()?;
    Ok(ret)
}

fn simulate(rope: &Rope, movement: &Movement) -> Vec<Rope> {
    let mut ropes = Vec::with_capacity(movement.size);

    // let mut file = open_for_append("day9-results.txt");
    // append(&mut file,format!("{}", movement));
    let mut intermediate_rope = rope.clone();
    for _movement_i in 0..movement.size {
        let mut factor_x = 0;
        let mut factor_y = 0;
        match movement.direction {
            Direction::Right => factor_x = 1,
            Direction::Left => factor_x = -1,
            Direction::Up => factor_y = -1,
            Direction::Down => factor_y = 1,
        }
        intermediate_rope.0[0] += (factor_x, factor_y);
        for i in 0..intermediate_rope.0.len() - 1 {
            let (head, tail) = get_mut2(&mut intermediate_rope.0, i, i + 1);

            // Only move the tail if they're too distant
            if !head.touching(tail) {
                let df_x = (head.0 - tail.0).signum();
                let df_y = (head.1 - tail.1).signum();
                *tail += (df_x, df_y);
            }
        }

        ropes.push(intermediate_rope.clone());

        // append(&mut file, format!("{}\n", Board{ head, tail }));
    }
    ropes
}

fn inner(lines: Vec<String>) -> (PuzzleResult, PuzzleResult) {
    let movements = parse(lines).unwrap();
    let part1 = calculate_unique_tail_positions(&movements, 2).len();
    let part2 = calculate_unique_tail_positions(&movements, 10).len();
    (part1, part2)
}

fn calculate_unique_tail_positions(
    movements: &Vec<Movement>,
    rope_size: usize,
) -> HashSet<Position> {
    let mut unique_tail_positions = HashSet::new();
    let mut rope = Rope::new(rope_size);
    unique_tail_positions.insert(*rope.tail());
    for movement in movements {
        let intermediate_ropes = simulate(&rope, &movement);
        rope = intermediate_ropes.last().unwrap().clone();
        for intermediate_rope in intermediate_ropes.iter() {
            unique_tail_positions.insert(*intermediate_rope.tail());
        }
    }
    unique_tail_positions
}

pub struct Board {
    pub head: Position,
    pub tail: Position,
}

impl Display for Board {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let head = (self.head.0, self.head.1);
        let tail = (self.tail.0, self.tail.1);

        let extra_size = 0;
        let min_x = (head.0).min(tail.0) - extra_size;
        let min_y = (head.1).min(tail.1) - extra_size;
        let max_x = (head.0).max(tail.0) + extra_size;
        let max_y = (head.1).max(tail.1) + extra_size;
        let size_x = (head.0).abs_diff(tail.0) as usize;
        let size_y = (head.1).abs_diff(tail.1) as usize;
        let header = if size_x > 1 {
            format!("|   {min_x:>3}{}{max_x}\n", " ".repeat(size_x - 1))
        } else {
            format!("|   {min_x:>3}\n")
        };
        f.write_str(header.as_str())?;
        for y in (min_y..max_y + 1).rev() {
            f.write_str(format!("| {y:>3} ").as_str())?;

            for x in min_x..max_x + 1 {
                if head == (x, y) {
                    f.write_char('H')?;
                } else if tail == (x, y) {
                    f.write_char('T')?;
                } else {
                    f.write_char('.')?;
                }
            }
            if y > min_y {
                f.write_char('\n')?;
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::strings::SkipEmptyLines;
    use itertools::Itertools;
    use pretty_assertions::assert_eq as pretty_assert_eq;
    use textwrap::dedent;

    fn to_vec(f: Vec<&str>) -> Vec<String> {
        f.into_iter().map(|s| String::from(s)).collect_vec()
    }

    #[test]
    fn test_part1_inner() -> Result<(), String> {
        let movements_string = dedent(
            "
            R 4
            U 4
            L 3
            D 1
            R 4
            D 1
            L 5
            R 2",
        )
        .skip_empty_start_lines();
        let actual = inner(to_vec(movements_string.lines().collect_vec())).0;
        assert_eq!(actual, 13);

        Ok(())
    }

    #[test]
    fn test_part2_inner() -> Result<(), String> {
        let movements_string = dedent(
            "
            R 5
            U 8
            L 8
            D 3
            R 17
            D 10
            L 25
            U 20",
        )
        .skip_empty_start_lines();
        let actual = inner(to_vec(movements_string.lines().collect_vec())).1;
        assert_eq!(actual, 36);

        Ok(())
    }

    #[test]
    fn test_parse_movements() -> Result<(), String> {
        let movements_string = dedent(
            "
            R 4
            U 4
            L 3
            D 1
            R 4",
        )
        .skip_empty_start_lines();
        let lines = to_vec(movements_string.lines().collect_vec());
        let movements = parse(lines)?;
        assert_eq!(
            movements,
            vec![
                Movement::new(Direction::Right, 4),
                Movement::new(Direction::Up, 4),
                Movement::new(Direction::Left, 3),
                Movement::new(Direction::Down, 1),
                Movement::new(Direction::Right, 4),
            ]
        );
        Ok(())
    }

    #[test]
    fn test_simulate_movements() {
        let get_heads =
            |ropes: &Vec<Rope>| ropes.iter().map(|rope| rope.head().clone()).collect_vec();
        let get_tails =
            |ropes: &Vec<Rope>| ropes.iter().map(|rope| rope.tail().clone()).collect_vec();

        let ropes = simulate(&Rope::new(2), &Movement::new(Direction::Right, 4));
        assert_eq!(
            get_heads(&ropes),
            vec![
                Position(1, 0),
                Position(2, 0),
                Position(3, 0),
                Position(4, 0),
            ]
        );
        assert_eq!(
            get_tails(&ropes),
            vec![
                Position(0, 0),
                Position(1, 0),
                Position(2, 0),
                Position(3, 0),
            ]
        );

        let ropes = simulate(ropes.last().unwrap(), &Movement::new(Direction::Up, 4));
        assert_eq!(
            get_heads(&ropes),
            vec![
                Position(4, -1),
                Position(4, -2),
                Position(4, -3),
                Position(4, -4),
            ]
        );
        assert_eq!(
            get_tails(&ropes),
            vec![
                Position(3, 0),
                Position(4, -1),
                Position(4, -2),
                Position(4, -3),
            ]
        );

        let ropes = simulate(ropes.last().unwrap(), &Movement::new(Direction::Left, 3));
        assert_eq!(
            get_heads(&ropes),
            vec![Position(3, -4), Position(2, -4), Position(1, -4),]
        );
        assert_eq!(
            get_tails(&ropes),
            vec![Position(4, -3), Position(3, -4), Position(2, -4),]
        );

        let ropes = simulate(ropes.last().unwrap(), &Movement::new(Direction::Down, 1));
        assert_eq!(get_heads(&ropes), vec![Position(1, -3),]);
        assert_eq!(get_tails(&ropes), vec![Position(2, -4),]);

        let ropes = simulate(ropes.last().unwrap(), &Movement::new(Direction::Right, 4));
        assert_eq!(
            get_heads(&ropes),
            vec![
                Position(2, -3),
                Position(3, -3),
                Position(4, -3),
                Position(5, -3),
            ]
        );
        assert_eq!(
            get_tails(&ropes),
            vec![
                Position(2, -4),
                Position(2, -4),
                Position(3, -3),
                Position(4, -3),
            ]
        );

        let ropes = simulate(ropes.last().unwrap(), &Movement::new(Direction::Down, 1));
        assert_eq!(get_heads(&ropes), vec![Position(5, -2),]);
        assert_eq!(get_tails(&ropes), vec![Position(4, -3),]);
    }

    #[test]
    fn test_simulate_movements_with_longer_rope() {
        let get_heads =
            |ropes: &Vec<Rope>| ropes.iter().map(|rope| rope.head().clone()).collect_vec();
        let get_tails =
            |ropes: &Vec<Rope>| ropes.iter().map(|rope| rope.tail().clone()).collect_vec();

        let ropes = simulate(&Rope::new(4), &Movement::new(Direction::Right, 4));
        assert_eq!(
            ropes.iter().map(|r| r.to_vec()).collect_vec(),
            vec![
                vec![(1, 0), (0, 0), (0, 0), (0, 0)],
                vec![(2, 0), (1, 0), (0, 0), (0, 0)],
                vec![(3, 0), (2, 0), (1, 0), (0, 0)],
                vec![(4, 0), (3, 0), (2, 0), (1, 0)],
            ]
        );

        let ropes = simulate(&ropes.last().unwrap(), &Movement::new(Direction::Down, 4));
        // pretty_assert_eq!(
        assert_eq!(
            ropes.iter().map(|r| r.to_vec()).collect_vec(),
            vec![
                vec![(4, 1), (3, 0), (2, 0), (1, 0)],
                vec![(4, 2), (4, 1), (3, 1), (2, 1)],
                vec![(4, 3), (4, 2), (3, 1), (2, 1)],
                vec![(4, 4), (4, 3), (4, 2), (3, 2)],
            ]
        );
    }

    #[test]
    fn test_distance() {
        assert_eq!(Position(0, 0).euclidean_distance(&Position(0, 0)), 0);
        assert_eq!(Position(1, 0).euclidean_distance(&Position(0, 0)), 1);
        assert_eq!(Position(1, 1).euclidean_distance(&Position(0, 0)), 1);
        assert_eq!(Position(2, 0).euclidean_distance(&Position(0, 0)), 2);
        assert_eq!(Position(2, 1).euclidean_distance(&Position(0, 0)), 2);
        assert_eq!(Position(100, 1).euclidean_distance(&Position(0, 0)), 100);
        assert_eq!(Position(100, 100).euclidean_distance(&Position(0, 0)), 141);
    }

    #[test]
    fn test_calculate_unique_tail_positions() {
        let movements = vec![
            Movement::new(Direction::Right, 4),
            Movement::new(Direction::Up, 4),
        ];
        let actual = calculate_unique_tail_positions(&movements, 2)
            .iter()
            .sorted()
            .copied()
            .collect_vec();
        let mut expected = vec![
            Position(0, 0),
            Position(1, 0),
            Position(2, 0),
            Position(3, 0),
            Position(4, -1),
            Position(4, -2),
            Position(4, -3),
        ];
        expected.sort();
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_touching() {
        assert_eq!(true, Position(0, 0).touching(&Position(0, 0)));
        assert_eq!(true, Position(1, 0).touching(&Position(0, 0)));
        assert_eq!(true, Position(0, 1).touching(&Position(0, 0)));
        assert_eq!(true, Position(1, 1).touching(&Position(0, 0)));
        assert_eq!(false, Position(2, 0).touching(&Position(0, 0)));
        assert_eq!(false, Position(2, 2).touching(&Position(0, 2)));
    }

    #[test]
    fn test_format_board() {
        let board = |(head_x, head_y), (tail_x, tail_y)| {
            let board = Board {
                head: Position(head_x, head_y),
                tail: Position(tail_x, tail_y),
            };
            format!("{}", board)
        };
        pretty_assert_eq!(
            board((0, 0), (0, 0)),
            dedent(
                "
             |   0
             | 0 H"
            )
            .skip_empty_start_lines()
        );
        pretty_assert_eq!(
            board((1, 0), (0, 0)),
            dedent(
                "
             |   0
             | 0 TH"
            )
            .skip_empty_start_lines()
        );
        pretty_assert_eq!(
            board((2, 0), (0, 0)),
            dedent(
                "
             |   0 2
             | 0 T.H"
            )
            .skip_empty_start_lines()
        );
        pretty_assert_eq!(
            board((2, 1), (0, 0)),
            dedent(
                "
             |   0 2
             | 1 ..H
             | 0 T.."
            )
            .skip_empty_start_lines()
        );
        pretty_assert_eq!(
            board((2, 2), (0, 0)),
            dedent(
                "
             |   0 2
             | 2 ..H
             | 1 ...
             | 0 T.."
            )
            .skip_empty_start_lines()
        );
        pretty_assert_eq!(
            board((-1, 2), (0, 0)),
            dedent(
                "
             |  -1
             | 2 H.
             | 1 ..
             | 0 .T"
            )
            .skip_empty_start_lines()
        );
        pretty_assert_eq!(
            board((-1, 2), (2, 0)),
            dedent(
                "
             |  -1  2
             | 2 H...
             | 1 ....
             | 0 ...T"
            )
            .skip_empty_start_lines()
        );
    }
}
