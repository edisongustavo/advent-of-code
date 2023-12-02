use crate::files::{contents, lines};
use derive_builder::Builder;
use itertools::Itertools;
use ndarray::prelude::*;
use std::fmt::{Debug, Display, Write};
use pest::Parser;
use anyhow;
use pest_derive::Parser;

type PuzzleResult = i32;

#[derive(Eq, Hash, Ord, PartialOrd, PartialEq, Debug, Copy, Clone)]
pub struct Position(pub i32, pub i32);

pub fn day11() -> (PuzzleResult, PuzzleResult) {
    inner(&contents("inputs/2022/day11.txt"))
}

#[derive(Builder, Default)]
struct Monkey {
    items: Vec<usize>,
}

#[derive(Parser)]
#[grammar = "solutions2022/day11.pest"]
struct MonkeyParser;

fn parse(input: &String) -> anyhow::Result<Vec<Monkey>> {
    let mut ret: Vec<Monkey> = Vec::new();
    let monkeys = MonkeyParser::parse(Rule::Grammar, &input)?;
    println!("{}", monkeys.to_json());
    for monkey in monkeys {
        println!("Parsing monkey!\n----------");
        match monkey.as_rule() {
            Rule::Monkey => {
                let monkey = build_monkey(monkey);
                // let mut inner_rules = monkey.into_inner(); // { name }
                // let foo = inner_rules.next().unwrap();
                // println!("{:?}", foo);
                // monkey_builder.monkey_id()
            },
            Rule::EOI => (),
            _ => {
                let mut inner_rules = monkey.into_inner(); // { name }
                let foo = inner_rules.next().unwrap();
                println!("{:?}", foo);
                unreachable!()
            },
        }
        println!("=================> monkey parsed");
    }
    Ok(ret)
}

fn build_monkey(monkey_pair: pest::iterators::Pair<Rule>) -> Monkey {
    let mut monkey_builder = MonkeyBuilder::default();
    println!("monkey_pair: {:?}", monkey_pair);
    let monkey_pairs = monkey_pair.into_inner();
    println!("monkey_pairs: {:?}", &monkey_pairs);
    for pair in monkey_pairs {
        match pair.as_rule() {
            Rule::monkey_id => {
                let mut monkey_id_pair = pair.as_str();
                println!("Rule::monkey_id -> {:?}", monkey_id_pair);
                // let id = monkey_id_pair[0].as_str();
                // println!("Rule::monkey_id -> {:?}", id);
            },
            Rule::items => {
                let items = pair.as_str()
                    .split(",")
                    .map(|s| s.trim().parse::<usize>())
                    .try_collect()
                    .unwrap();
                monkey_builder.items(items);
            },
            Rule::operation => {
                println!("Rule::operation -> {:?}", pair);
                println!("Rule::operation -> {:?}", pair.into_inner());
            },
            _ => {
                println!("unreachable -> {:?}", pair);
                let foo = pair.into_inner().collect_vec();
                println!("unreachable -> {:?}", foo);
                // unreachable!()
            },
        };
    }
    Monkey::default()
    // monkey_builder.build().unwrap()
}

fn inner(input: &String) -> (PuzzleResult, PuzzleResult) {
    let instructions = parse(&input).unwrap();

    let part1 = PuzzleResult::default();
    let part2 = PuzzleResult::default();

    (part1, part2)
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

    fn program() -> String {
        return dedent(
            "
            Monkey 0:
              Starting items: 79, 98
              Operation: new = old * 19
              Test: divisible by 23
                If true: throw to monkey 2
                If false: throw to monkey 3
            ")
            .skip_empty_start_lines();

        return dedent(
            "
            Monkey 0:
              Starting items: 79, 98
              Operation: new = old * 19
              Test: divisible by 23
                If true: throw to monkey 2
                If false: throw to monkey 3

            Monkey 1:
              Starting items: 54, 65, 75, 74
              Operation: new = old + 6
              Test: divisible by 19
                If true: throw to monkey 2
                If false: throw to monkey 0

            Monkey 2:
              Starting items: 79, 60, 97
              Operation: new = old * old
              Test: divisible by 13
                If true: throw to monkey 1
                If false: throw to monkey 3

            Monkey 3:
              Starting items: 74
              Operation: new = old + 3
              Test: divisible by 17
                If true: throw to monkey 0
                If false: throw to monkey 1",
        )
        .skip_empty_start_lines();
    }
    #[test]
    fn test_part1_inner() -> Result<(), String> {
        let input_string = program();
        let actual = inner(&input_string).0;
        assert_eq!(actual, 10605);

        Ok(())
    }

    #[test]
    fn test_part2_inner() -> Result<(), String> {
        let input_string = program();
        let actual = inner(&input_string).1;
        assert_eq!(actual, 0);
        Ok(())
    }

    #[test]
    fn test_parse() -> anyhow::Result<()> {
        let input_string = program();
        let monkeys = parse(&input_string)?;
        // assert_eq!(monkeys, vec![MonkeyBuilder::new()]);
        Ok(())
    }
}
