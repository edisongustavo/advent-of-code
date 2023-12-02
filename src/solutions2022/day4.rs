use crate::files::lines;
use crate::geometry::{contains, overlaps};

pub fn day4part1() -> i32 {
    let lines_of_file = lines("inputs/2022/day4.txt");
    part1_inner(lines_of_file)
}

pub fn day4part2() -> i32 {
    let lines_of_file = lines("inputs/2022/day4.txt");
    part2_inner(lines_of_file)
}

struct Assignments {
    left: (i32, i32),
    right: (i32, i32),
}

fn calculate_assignments_containing(assignments: Assignments) -> i32 {
    if contains(assignments.left, assignments.right)
        || contains(assignments.right, assignments.left)
    {
        1
    } else {
        0
    }
}

fn calculate_assignments_overlapping(assignments: Assignments) -> i32 {
    if overlaps(assignments.left, assignments.right)
        || overlaps(assignments.right, assignments.left)
    {
        1
    } else {
        0
    }
}

fn parse_line(line: &str) -> Result<Assignments, &str> {
    let error_message = "Can't parse line: {line}";
    let (left_str, right_str) = line.split_once(",").ok_or(error_message)?;
    let left_res = parse_assignment(left_str);
    let right_res = parse_assignment(right_str);
    match (left_res, right_res) {
        (Ok(left), Ok(right)) => Ok(Assignments { left, right }),
        _ => Err(error_message),
    }
}

fn parse_assignment(assignment: &str) -> Result<(i32, i32), &str> {
    let error_message = "Can't parse assignment: {assignment}";
    let (left_str, right_str) = assignment.split_once("-").ok_or(error_message)?;
    let left_num_res = left_str.parse::<i32>();
    let right_num_res = right_str.parse::<i32>();
    match (left_num_res, right_num_res) {
        (Ok(left), Ok(right)) => Ok((left, right)),
        _ => Err(error_message),
    }
}

fn part1_inner(lines: Vec<String>) -> i32 {
    lines
        .iter()
        .filter(|line| !line.trim().is_empty())
        .map(|line| parse_line(line))
        .map(|assignments| calculate_assignments_containing(assignments.unwrap()))
        .sum()
}

fn part2_inner(lines: Vec<String>) -> i32 {
    lines
        .iter()
        .filter(|line| !line.trim().is_empty())
        .map(|line| parse_line(line))
        .map(|assignments| calculate_assignments_overlapping(assignments.unwrap()))
        .sum()
}

#[cfg(test)]
mod tests {
    use itertools::Itertools;

    use super::*;

    fn to_vec(f: Vec<&str>) -> Vec<String> {
        f.into_iter().map(|s| String::from(s)).collect_vec()
    }

    #[test]
    fn test_part1_inner() {
        assert_eq!(part1_inner(to_vec(vec!["2-4,6-8"])), 0);
        assert_eq!(part1_inner(to_vec(vec!["2-3,4-5"])), 0);
        assert_eq!(part1_inner(to_vec(vec!["5-7,7-9"])), 0);
        assert_eq!(part1_inner(to_vec(vec!["2-8,3-7"])), 1);
        assert_eq!(part1_inner(to_vec(vec!["6-6,4-6"])), 1);
        assert_eq!(part1_inner(to_vec(vec!["2-6,4-8"])), 0);
    }

    #[test]
    fn test_part2_inner() {
        assert_eq!(part2_inner(to_vec(vec!["2-4,6-8"])), 0);
        assert_eq!(part2_inner(to_vec(vec!["2-3,4-5"])), 0);
        assert_eq!(part2_inner(to_vec(vec!["5-7,7-9"])), 1);
        assert_eq!(part2_inner(to_vec(vec!["2-8,3-7"])), 1);
        assert_eq!(part2_inner(to_vec(vec!["6-6,4-6"])), 1);
        assert_eq!(part2_inner(to_vec(vec!["2-6,4-8"])), 1);
    }
}
