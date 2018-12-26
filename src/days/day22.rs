//! Solutions for https://adventofcode.com/2018/day/22
use utils::matrix::Matrix;

#[allow(dead_code)]
pub fn part1() {
    println!("{}", solve_part1(10914, (739, 9)));
}


fn solve_part1(cave_depth: usize, target: (usize, usize)) -> usize {
    let mut erosion_levels = Matrix::new(target.0 + 1, target.1 + 1, 0);

    let mut risk_level = 0;

    for y in 0..=target.0 {
        for x in 0..=target.1 {
            let geo_index = match (y, x) {
                (0, 0) => 0,
                (0, x_) => x_ * 16807,
                (y_, 0) => y_ * 48271,
                (y_, x_) => if (y_, x_) == target {
                    0
                } else {
                    erosion_levels[(y, x - 1)] * erosion_levels[(y - 1, x)]
                }
            };
            let ero_level = (geo_index + cave_depth) % 20183;
            erosion_levels[(y, x)] = ero_level;

            risk_level += ero_level % 3;
        }
    }

    risk_level
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(
            solve_part1(510, (10, 10)),
            114
        );
    }
}
