use std::collections::HashSet;

use utils::data::load_data;
use std::time::Instant;

#[allow(dead_code)]
pub fn part1() {
    println!("{}", solve_part1(&get_puzzle_input()));
}

#[allow(dead_code)]
pub fn part2() {
    println!("{}", solve_part2(&get_puzzle_input()));
}

pub fn time_both_parts() {
    let input_str =  load_data("day5");
    let input = parse_puzzle_input(input_str);

    let start = Instant::now();
    
    solve_part1(&input);
    solve_part2(&input);

    let elapsed = start.elapsed();
    println!("{:?}", elapsed);
}

/// Unit type and unit polarity are the char and bool
#[derive(Clone)]
struct Unit(char, bool);

fn solve_part1(units: &Vec<Unit>) -> u32 {
    reduce(units, None)
}

fn solve_part2(units: &Vec<Unit>) -> u32 {
    // Create a list of all occurring unit types
    let unique_types: HashSet<char> = units.iter()
        .map(|u| u.0)
        .collect();

    // Create polymers by removing one type, reduce them and pick the shortest length
    unique_types.into_iter()
        .map(|t| reduce(units, Some(t)))
        .min().unwrap() as u32
}

fn reduce(units: &Vec<Unit>, ignore: Option<char>) -> u32 {
    let mut stack = Vec::with_capacity(units.len());
    for unit in units {
        if Some(unit.0) == ignore {
            continue;
        }
        if stack.last().map_or(false, |top| does_react(unit, top)) {
            stack.pop();
        } else {
            stack.push(unit.clone());
        }
    }
    stack.len() as u32
}

fn does_react(a: &Unit, b: &Unit) -> bool {
    a.0 == b.0 && a.1 != b.1
}

fn get_puzzle_input() -> Vec<Unit> {
    parse_puzzle_input(load_data("day5"))
}

fn parse_puzzle_input(input: String) -> Vec<Unit> {
    input.trim().chars().into_iter()
        .map(|c| Unit(c.to_ascii_lowercase(), c.is_ascii_uppercase()))
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(
            solve_part1(&parse_puzzle_input(String::from("  dabAcCaCBAcCcaDA  "))),
            10
        )
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            solve_part2(&parse_puzzle_input(String::from("dabAcCaCBAcCcaDA"))),
            4
        )
    }
}
