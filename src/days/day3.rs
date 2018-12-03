use std::collections::HashSet;

use regex::Regex;

use utils::data::load_data;
use utils::data::non_empty_lines;
use utils::matrix::Matrix;

#[allow(dead_code)]
pub fn part1() {
    println!("{}", solve_combined(get_puzzle_input()).0);
}

#[allow(dead_code)]
pub fn part2() {
    println!("{}", solve_combined(get_puzzle_input()).1);
}

#[derive(Debug, Eq, PartialEq)]
struct Claim {
    id: u32,
    left: usize,
    top: usize,
    width: usize,
    height: usize,
}

impl Claim {
    fn new(id: u32, left: usize, top: usize, width: usize, height: usize) -> Claim {
        Claim { id, left, top, width, height }
    }

    fn parse(repr: String) -> Claim {
        let re = Regex::new(r"^#(\d+) @ (\d+),(\d+): (\d+)x(\d+)$").unwrap();
        let cap = re.captures(&repr).unwrap();

        Claim::new(
            cap[1].parse().unwrap(),
            cap[2].parse().unwrap(),
            cap[3].parse().unwrap(),
            cap[4].parse().unwrap(),
            cap[5].parse().unwrap(),
        )
    }
}


fn solve_combined(claims: Vec<Claim>) -> (u32, u32) {
    // Use a single signed integer to represent the state of the claims.
    // 0 represents unclaimed cells
    // Positive numbers represent the ID of a claim, if that claim is the only one
    // -1 represents a cell that's claimed multiple times
    let mut state = Matrix::new(1000, 1000, 0i32);

    let mut conflicting_cells = 0;
    let mut conflicting_ids = HashSet::new();

    for claim in claims.iter() {
        for x in claim.left..claim.left + claim.width {
            for y in claim.top..claim.top + claim.height {
                let idx = (y, x);
                let cur = state[idx];
                match cur {
                    // Empty cell, assign it to the current claim
                    0 => state[idx] = claim.id as i32,
                    // Had a single claim, but now there's more. This means that both the previous
                    // claim and the current one have issues and are not the result
                    _ => {
                        if cur > 0 {
                            conflicting_ids.insert(cur as u32);
                            conflicting_cells += 1;
                        }
                        conflicting_ids.insert(claim.id);
                        state[idx] = -1
                    }
                }
            }
        }
    }

    // Find the only ID that did not conflict with anything
    let valid_id = claims.iter()
        .map(|c| c.id)
        .find(|id| !conflicting_ids.contains(id))
        .unwrap();

    (conflicting_cells, valid_id)
}

fn get_puzzle_input() -> Vec<Claim> {
    non_empty_lines(load_data("day3"))
        .into_iter()
        .map(Claim::parse)
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_solver() {
        assert_eq!(
            solve_combined(vec![
                Claim::new(1, 1, 3, 4, 4),
                Claim::new(2, 3, 1, 4, 4),
                Claim::new(3, 5, 5, 2, 2),
            ]),
            (4, 3)
        )
    }

    #[test]
    fn test_parse_claim() {
        assert_eq!(
            Claim::parse(String::from("#758 @ 738,834: 21x13")),
            Claim::new(758, 738, 834, 21, 13)
        )
    }
}
