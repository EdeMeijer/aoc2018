use regex::Regex;

use utils::data::load_data;
use utils::data::non_empty_lines;
use utils::matrix::Matrix;

#[allow(dead_code)]
pub fn part1() {
    println!("{}", solve_part1(get_puzzle_input()));
}

#[derive(Debug, Eq, PartialEq)]
struct Claim {
    left: usize,
    top: usize,
    width: usize,
    height: usize,
}

impl Claim {
    fn new(left: usize, top: usize, width: usize, height: usize) -> Claim {
        Claim { left, top, width, height }
    }

    fn parse(repr: String) -> Claim {
        let re = Regex::new(r"^#(\d+) @ (\d+),(\d+): (\d+)x(\d+)$").unwrap();
        let cap = re.captures(&repr).unwrap();

        Claim::new(
            cap[2].parse().unwrap(),
            cap[3].parse().unwrap(),
            cap[4].parse().unwrap(),
            cap[5].parse().unwrap(),
        )
    }
}


fn solve_part1(claims: Vec<Claim>) -> u32 {
    let mut state = Matrix::new(1000, 1000, 0u32);

    let mut conflicting_cells = 0;

    for claim in claims {
        for x in claim.left..claim.left + claim.width {
            for y in claim.top..claim.top + claim.height {
                state[(y, x)] += 1;
                if state[(y, x)] == 2 {
                    // The second claim on the same cell is the first conflict, record it
                    conflicting_cells += 1;
                }
            }
        }
    }

    conflicting_cells
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
    fn test_part1() {
        assert_eq!(
            solve_part1(vec![
                Claim::new(1, 3, 4, 4),
                Claim::new(3, 1, 4, 4),
                Claim::new(5, 5, 2, 2),
            ]),
            4
        )
    }

    #[test]
    fn test_parse_claim() {
        assert_eq!(
            Claim::parse(String::from("#758 @ 738,834: 21x13")),
            Claim::new(738, 834, 21, 13)
        )
    }
}
