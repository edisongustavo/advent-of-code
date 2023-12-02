use crate::files::lines;
use PossibilityOrDesiredResult::*;

#[derive(Copy, Clone)]
enum Play {
    Rock,
    Paper,
    Scissors,
}

enum PossibilityOrDesiredResult {
    Possibility(Play),
    DesiredResult(MatchResult),
}

enum MatchResult {
    Win,
    Draw,
    Loss,
}

struct PartialGame {
    player1: Play,
    player2: PossibilityOrDesiredResult,
}

struct Game {
    player1: Play,
    player2: Play,
}

fn parse_line(line: &str, parse_desired_result: bool) -> PartialGame {
    let (player1, player2) = line.split_at(1);
    let player1_play = match parse_possibility(player1.trim(), false) {
        PossibilityOrDesiredResult::Possibility(play) => play,
        _ => panic!("First play must always be a play!"),
    };

    return PartialGame {
        player1: player1_play,
        player2: parse_possibility(player2.trim(), parse_desired_result),
    };
}

fn parse_possibility(raw_play: &str, parse_desired_result: bool) -> PossibilityOrDesiredResult {
    if parse_desired_result {
        match raw_play {
            "A" => Possibility(Play::Rock),
            "B" => Possibility(Play::Paper),
            "C" => Possibility(Play::Scissors),
            "X" => PossibilityOrDesiredResult::DesiredResult(MatchResult::Loss),
            "Y" => PossibilityOrDesiredResult::DesiredResult(MatchResult::Draw),
            "Z" => PossibilityOrDesiredResult::DesiredResult(MatchResult::Win),
            _ => panic!("Unknown play: {raw_play}"),
        }
    } else {
        match raw_play {
            "A" | "X" => Possibility(Play::Rock),
            "B" | "Y" => Possibility(Play::Paper),
            "C" | "Z" => Possibility(Play::Scissors),
            _ => panic!("Unknown play: {raw_play}"),
        }
    }
}

pub fn day2part1() -> i32 {
    let lines_of_file = lines("inputs/2022/day2.txt");
    day2part1_inner(lines_of_file)
}

fn day2part1_inner(l: Vec<String>) -> i32 {
    l.iter()
        .filter(|line| line.len() == 3)
        .map(|line| parse_line(line, false))
        .map(|game| convert_to_game_as_day1(game))
        .map(|game| calculate_game_score(game))
        .sum()
}

fn convert_to_game_as_day1(game: PartialGame) -> Game {
    return Game {
        player1: game.player1,
        player2: match game.player2 {
            Possibility(play) => play,
            _ => panic!("Wrong parsed game!"),
        },
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    use itertools::Itertools;

    fn to_vec(f: Vec<&str>) -> Vec<String> {
        f.into_iter().map(|s| String::from(s)).collect_vec()
    }

    #[test]
    fn test_day2part1_inner() {
        assert_eq!(day2part1_inner(to_vec(vec!["A Y"])), 8);
    }

    #[test]
    fn test_day2part2_inner() {
        assert_eq!(day2part2_inner(to_vec(vec!["A Y"])), 4);
    }
}

fn calculate_game_score(game: Game) -> i32 {
    let match_result = match (&game.player1, &game.player2) {
        (Play::Paper, Play::Scissors) => MatchResult::Win,
        (Play::Paper, Play::Rock) => MatchResult::Loss,
        (Play::Rock, Play::Paper) => MatchResult::Win,
        (Play::Rock, Play::Scissors) => MatchResult::Loss,
        (Play::Scissors, Play::Rock) => MatchResult::Win,
        (Play::Scissors, Play::Paper) => MatchResult::Loss,
        _ => MatchResult::Draw,
    };
    let match_score = match match_result {
        MatchResult::Win => 6,
        MatchResult::Draw => 3,
        MatchResult::Loss => 0,
    };
    let play_score = match game.player2 {
        Play::Rock => 1,
        Play::Paper => 2,
        Play::Scissors => 3,
    };
    return match_score + play_score;
}

pub fn day2part2() -> i32 {
    let lines_of_file = lines("inputs/2022/day2.txt");
    day2part2_inner(lines_of_file)
}

fn play_game(game: PartialGame) -> Game {
    let player2 = match game.player2 {
        Possibility(play) => play,
        PossibilityOrDesiredResult::DesiredResult(desired_result) => match desired_result {
            MatchResult::Win => match game.player1 {
                Play::Rock => Play::Paper,
                Play::Paper => Play::Scissors,
                Play::Scissors => Play::Rock,
            },
            MatchResult::Loss => match game.player1 {
                Play::Rock => Play::Scissors,
                Play::Paper => Play::Rock,
                Play::Scissors => Play::Paper,
            },
            MatchResult::Draw => game.player1,
        },
    };
    Game {
        player1: game.player1,
        player2,
    }
}

fn day2part2_inner(l: Vec<String>) -> i32 {
    l.iter()
        .filter(|line| line.len() == 3)
        .map(|line| parse_line(line, true))
        .map(|game| play_game(game))
        .map(|game| calculate_game_score(game))
        .sum()
}
