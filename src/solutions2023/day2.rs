use crate::files::contents;
use anyhow::{ensure, Context, Result};
use itertools::Itertools;
use ndarray::prelude::*;
use pest::iterators::Pair;
use pest::Parser;
use pest_derive::Parser;
use std::fmt::{Debug, Display, Write};

type PuzzleResult = usize;

#[derive(Eq, Hash, Ord, PartialOrd, PartialEq, Debug, Copy, Clone)]
pub struct Position(pub i32, pub i32);

pub fn solution() -> (PuzzleResult, PuzzleResult) {
    inner(&contents("inputs/2023/day2.txt")).unwrap()
}

fn inner(input: &String) -> Result<(PuzzleResult, PuzzleResult)> {
    let games = parse(input)?;
    let part1 = games
        .iter()
        .filter(|game| {
            game.ball_sets
                .iter()
                .all(|ball_set| ball_set.red <= 12 && ball_set.green <= 13 && ball_set.blue <= 14)
        })
        .map(|game| game.id)
        .sum();

    let part2 = games
        .iter()
        .map(|game| {
            let red = game.ball_sets.iter().map(|bs| bs.red).max().unwrap();
            let green = game.ball_sets.iter().map(|bs| bs.green).max().unwrap();
            let blue = game.ball_sets.iter().map(|bs| bs.blue).max().unwrap();
            (red, green, blue)
        })
        .map(|(red, green, blue)| red * green * blue)
        .sum();
    Ok((part1, part2))
}

fn parse(input: &String) -> Result<Vec<Game>> {
    let mut pairs = GamesParser::parse(Rule::Grammar, &input)?;
    // println!("{}", pairs.to_json());
    let games = pairs.next().unwrap().into_inner();
    let mut ret = vec![];
    for game in games {
        match game.as_rule() {
            Rule::Game => {
                let game_str = format!("Invalid game ⮞ {} ⮜", game.as_str());
                let game = parse_game(game).context(game_str)?;
                ret.push(game);
            }
            Rule::EOI => (),
            _ => {
                println!("{:?}", game.as_rule());
                unreachable!()
            }
        }
    }
    Ok(ret)
}

fn parse_game(pair: Pair<Rule>) -> Result<Game> {
    let mut game = Game::default();

    let mut inner = pair.into_inner();

    // game id
    let game_id_str = inner.next().unwrap().as_str();
    game.id = game_id_str.parse::<usize>().unwrap();

    let game_specs = inner.next().unwrap();
    for ball_sets in game_specs.into_inner() {
        let ball_sets_str = format!("Invalid set of balls ⮞ {} ⮜", ball_sets.as_str());
        let ball_set = parse_ball_set(ball_sets).context(ball_sets_str)?;
        game.ball_sets.push(ball_set);
    }
    Ok(game)
}

fn parse_ball_set(pair: Pair<Rule>) -> Result<BallSet> {
    match pair.as_rule() {
        Rule::ball_set => (),
        _ => panic!(),
    }
    let mut ret = BallSet::default();
    for ball in pair.into_inner() {
        let mut inner = ball.into_inner();

        let number_of_balls = inner.next().unwrap().as_str();
        let number_of_balls = number_of_balls.trim().parse::<usize>().unwrap();
        let color = inner.next().unwrap().as_str();
        match color {
            "red" => {
                ensure!(ret.red == 0, "Duplicated red color");
                ret.red = number_of_balls;
            }
            "green" => {
                ensure!(ret.green == 0, "Duplicated green color");
                ret.green = number_of_balls;
            }
            "blue" => {
                ensure!(ret.blue == 0, "Duplicated blue color");
                ret.blue = number_of_balls;
            }
            _ => unreachable!(),
        }
    }
    Ok(ret)
}

#[derive(Debug, Default)]
struct BallSet {
    red: usize,
    green: usize,
    blue: usize,
}

#[derive(Debug, Default)]
struct Game {
    id: usize,
    ball_sets: Vec<BallSet>,
}

#[derive(Parser)]
#[grammar = "solutions2023/day2.pest"]
struct GamesParser;

#[cfg(test)]
mod tests {
    use std::error::Error;
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
            Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
            Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
            Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
            Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
            Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green",
        )
        .skip_empty_start_lines();
    }
    #[test]
    fn test_part1_inner() -> Result<()> {
        let input_string = program();
        let actual = inner(&input_string)?.0;
        assert_eq!(actual, 8);

        Ok(())
    }

    #[test]
    fn test_part2_inner() -> Result<()> {
        let input_string = program();
        let actual = inner(&input_string)?.1;
        assert_eq!(actual, 2286);
        Ok(())
    }

    #[test]
    fn test_parse_duplicate_colors() -> Result<()> {
        let error = parse(&"Game 1: 3 blue, 4 blue".to_string()).unwrap_err();
        assert_eq!(error.root_cause().to_string(), "Duplicated blue color");

        let error = parse(&"Game 1: 1 red, 2 green; 3 blue, 4 blue".to_string()).unwrap_err();
        assert_eq!(error.root_cause().to_string(), "Duplicated blue color");

        let error = parse(&dedent("
            Game 1: 1 red, 2 green
            Game 2: 1 red, 2 green; 3 blue, 4 blue"
        ).skip_empty_start_lines()).unwrap_err();
        assert_eq!(error.root_cause().to_string(), "Duplicated blue color");
        assert_eq!(format!("{:#}", error), "Invalid game ⮞ Game 2: 1 red, 2 green; 3 blue, 4 blue ⮜: Invalid set of balls ⮞ 3 blue, 4 blue ⮜: Duplicated blue color");
        Ok(())
    }
}
