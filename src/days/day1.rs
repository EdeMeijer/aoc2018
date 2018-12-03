use std::collections::HashSet;

use utils::load_data;
use utils::non_empty_lines;

#[allow(dead_code)]
pub fn part1() {
    println!("{}", solve_part1(get_puzzle_input()));
}

#[allow(dead_code)]
pub fn part2() {
    println!("{}", solve_part2(get_puzzle_input()));
}

fn solve_part1(changes: Vec<i32>) -> i32 {
    changes.into_iter().sum()
}

fn solve_part2(changes: Vec<i32>) -> i32 {
    // Track all observed frequencies with a hash set
    let mut seen = HashSet::new();

    // Start at 0 and mark it as seen
    let mut frequency = 0;
    seen.insert(frequency);

    // Cycle through the list of changes infinitely
    let mut change_cycle = changes.into_iter().cycle();

    loop {
        frequency += change_cycle.next().unwrap();
        let added = seen.insert(frequency);

        if !added {
            // This was seen before, so the current frequency is the answer
            break frequency;
        }
    }
}

fn get_puzzle_input() -> Vec<i32> {
    non_empty_lines(load_data("day1"))
        .into_iter()
        .map(|c| c.parse::<i32>().unwrap())
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(
            solve_part1(vec![1, 1, 1]),
            3
        );
        assert_eq!(
            solve_part1(vec![1, 1, -2]),
            0
        );
        assert_eq!(
            solve_part1(vec![-1, -2, -3]),
            -6
        );
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            solve_part2(vec![1, -1]),
            0
        );
        assert_eq!(
            solve_part2(vec![3, 3, 4, -2, -4]),
            10
        );
        assert_eq!(
            solve_part2(vec![-6, 3, 8, 5, -6]),
            5
        );
        assert_eq!(
            solve_part2(vec![7, 7, -2, -7, -4]),
            14
        );
    }
}
