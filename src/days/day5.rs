use std::collections::HashSet;

use utils::data::load_data;

#[allow(dead_code)]
pub fn part1() {
    println!("{}", solve_part1(get_puzzle_input()));
}

#[allow(dead_code)]
pub fn part2() {
    println!("{}", solve_part2(get_puzzle_input()));
}

/// Unit type and unit polarity are the char and bool
#[derive(Clone)]
struct Unit(char, bool);

fn solve_part1(units: Vec<Unit>) -> u32 {
    reduce(units).len() as u32
}

fn solve_part2(units: Vec<Unit>) -> u32 {
    // Create a list of all occurring unit types
    let unique_types: HashSet<char> = units.iter()
        .map(|u| u.0)
        .collect();

    // Create polymers by removing one type, reduce them and pick the shortest length
    unique_types.into_iter()
        .map(|t| units.clone().into_iter().filter(|u| u.0 != t).collect::<Vec<_>>())
        .map(|p| reduce(p).len())
        .min().unwrap() as u32
}

/// Repeatedly apply reduction until the input stops changing
fn reduce(mut units: Vec<Unit>) -> Vec<Unit> {
    loop {
        let prior_len = units.len();
        units = reduce_pass(units);
        if units.len() == prior_len {
            break units;
        }
    }
}

/// A single pass over a polymer reducing reacting pairs
fn reduce_pass(units: Vec<Unit>) -> Vec<Unit> {
    let mut result = vec![];
    let mut iter = units.into_iter().peekable();

    while let Some(unit) = iter.next() {
        let reacts = iter.peek().map_or(false, |n| does_react(n, &unit));
        if reacts {
            // Consume the next element to finish the reaction
            iter.next();
        } else {
            result.push(unit);
        }
    }

    result
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
            solve_part1(parse_puzzle_input(String::from("  dabAcCaCBAcCcaDA  "))),
            10
        )
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            solve_part2(parse_puzzle_input(String::from("dabAcCaCBAcCcaDA"))),
            4
        )
    }
}
