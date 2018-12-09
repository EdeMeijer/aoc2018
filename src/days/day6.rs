//! Solutions for https://adventofcode.com/2018/day/6
use utils::data::load_data;
use utils::data::non_empty_lines;

type Coord = (usize, usize);
type Coords = Vec<Coord>;

#[allow(dead_code)]
pub fn part1() {
    println!("{}", solve_part1(get_puzzle_input()));
}

#[allow(dead_code)]
pub fn part2() {
    println!("{}", solve_part2(get_puzzle_input(), 10000));
}

fn solve_part1(coords: Coords) -> u32 {
    let (min_x, max_x, min_y, max_y) = get_initial_bounds(&coords);

    let mut areas = vec![0; coords.len()];
    let mut infinite = vec![false; coords.len()];

    for x in min_x..=max_x {
        for y in min_y..=max_y {
            let dists = get_coord_distances(x, y, &coords);
            let closest_dist = *dists.iter().min().unwrap();
            let closest_indices: Vec<_> = dists.iter().enumerate()
                .filter(|(_, d)| **d == closest_dist)
                .map(|(i, _)| i)
                .collect();

            if closest_indices.len() == 1 {
                // Exactly one is closest
                let closest_i = closest_indices[0];
                areas[closest_i] += 1;
                if x == min_x || x == max_x || y == min_y || y == max_y {
                    // Touching the edge, so this one's infinite
                    infinite[closest_i] = true;
                }
            }
        }
    }

    areas.into_iter().zip(infinite.into_iter())
        .filter(|(_, inf)| !*inf)
        .map(|(a, _)| a)
        .max().unwrap_or(0) as u32
}

fn solve_part2(coords: Coords, max: i32) -> u32 {
    let (min_x, max_x, min_y, max_y) = get_initial_bounds(&coords);

    let mut region = 0;

    for x in min_x..=max_x {
        for y in min_y..=max_y {
            let total_dist: i32 = get_coord_distances(x, y, &coords).iter().sum();

            if total_dist < max {
                region += 1;
            }
        }
    }

    region
}

fn get_coord_distances(x: i32, y: i32, coords: &Coords) -> Vec<i32> {
    coords.iter()
        .map(|(cx, cy)| (*cy as i32 - y).abs() + (*cx as i32 - x).abs())
        .collect()
}

fn get_initial_bounds(coords: &Coords) -> (i32, i32, i32, i32) {
    let xs: Vec<_> = coords.iter().map(|c| c.0 as i32).collect();
    let ys: Vec<_> = coords.iter().map(|c| c.1 as i32).collect();

    let min_x = *xs.iter().min().unwrap();
    let max_x = *xs.iter().max().unwrap();
    let min_y = *ys.iter().min().unwrap();
    let max_y = *ys.iter().max().unwrap();

    (min_x, max_x, min_y, max_y)
}

fn get_puzzle_input() -> Coords {
    parse_puzzle_input(load_data("day6"))
}

fn parse_puzzle_input(input: String) -> Coords {
    non_empty_lines(input)
        .into_iter()
        .map(|l| l
            .split(", ")
            .into_iter()
            .map(|c| c.parse::<usize>().unwrap())
            .collect::<Vec<_>>()
        )
        .map(|c| (c[0], c[1]))
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse() {
        assert_eq!(
            get_test_input(),
            vec![
                (1, 1),
                (1, 6),
                (8, 3),
                (3, 4),
                (5, 5),
                (8, 9),
            ]
        )
    }

    #[test]
    fn test_part1() {
        assert_eq!(
            solve_part1(get_test_input()),
            17
        )
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            solve_part2(get_test_input(), 32),
            16
        )
    }

    fn get_test_input() -> Coords {
        parse_puzzle_input(get_test_input_string())
    }

    fn get_test_input_string() -> String {
        "1, 1
1, 6
8, 3
3, 4
5, 5
8, 9".to_owned()
    }
}
