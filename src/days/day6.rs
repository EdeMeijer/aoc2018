use std::collections::HashMap;
use std::collections::HashSet;

use utils::data::load_data;
use utils::data::non_empty_lines;
use utils::matrix::Matrix;

type Coord = (usize, usize);
type Coords = Vec<Coord>;
type Frontier = HashSet<Coord>;
type Grid = Matrix<i32>;

#[allow(dead_code)]
pub fn part1() {
    println!("{}", solve_part1(get_puzzle_input()));
}

fn solve_part1(coords: Coords) -> u32 {
    // For every coordinate, we set up an expanding frontier that will propagate outwards
    // iteratively, adding 1 unit of manhattan distance evey step (so only up/down/left/right).
    // Overlapping cells will be removed from the frontiers before applying them since they have
    // the same distance for multiple coordinates.

    let xs: Vec<_> = coords.iter().map(|c| c.0).collect();
    let ys: Vec<_> = coords.iter().map(|c| c.1).collect();

    let min_x = *xs.iter().min().unwrap();
    let max_x = *xs.iter().max().unwrap();
    let min_y = *ys.iter().min().unwrap();
    let max_y = *ys.iter().max().unwrap();

    let init_width = max_x - min_x + 1;
    let init_height = max_y - min_y + 1;

    let coords: Coords = coords.into_iter()
        .map(|c| (c.0 + init_width, c.1 + init_height))
        .collect();

    let mut areas = vec![1; coords.len()];

    let mut grid = Grid::new(init_height * 3, init_width * 3, 0);
    for (i, coord) in coords.iter().enumerate() {
        grid[*coord] = i as i32 + 1;
    }

    let mut frontiers: Vec<_> = coords.iter().map(|c| hashset!(*c)).collect();
    let mut infinite = vec![false; coords.len()];

    loop {
        let new_frontiers: Vec<_> = frontiers.iter()
            .map(|f| expand_frontier(f, &grid))
            .collect();

        let (new_frontiers, conflicts) = dedupe_frontiers(new_frontiers);
        for cc in conflicts.into_iter() {
            grid[cc] = -1;
        }

        let mut done = true;
        for (i, frontier) in new_frontiers.into_iter().enumerate() {
            if !frontier.is_empty() {
                done = false;
            }
            areas[i] += frontier.len();
            for cc in frontier.iter() {
                grid[*cc] = i as i32 + 1;
                if cc.0 == 0 || cc.1 == 0 || cc.0 == grid.height - 1 || cc.1 == grid.width - 1 {
                    // Touching the edge, so it's infinite
                    infinite[i] = true;
                }
            }
            frontiers[i] = frontier;
        }

        let mut dbg_grid = grid.clone();
        for cc in frontiers[5].iter() {
            dbg_grid[*cc] = 9;
        }
//        print_debug_state(&dbg_grid);
//        println!("=====================================");

        if done {
            break;
        }
    }

    areas.into_iter().zip(infinite.into_iter())
        .filter(|(_, inf)| !*inf)
        .map(|(a, _)| a)
        .max().unwrap_or(0) as u32
}

fn print_debug_state(grid: &Grid) {
    for y in 0..grid.height {
        for x in 0..grid.width {
            let v = grid[(y, x)];
            if v == -1 {
                print!(" -");
            } else if v == 0 {
                print!(" .");
            } else {
                print!(" {}", v);
            }
        }
        println!();
    }
}

fn expand_frontier(frontier: &Frontier, grid: &Grid) -> Frontier {
    let mut new_frontier = Frontier::new();

    for (y, x) in frontier.iter() {
        for (dy, dx) in [(-1, 0), (1, 0), (0, -1), (0, 1)].iter() {
            let cy = *y as i32 + dy;
            let cx = *x as i32 + dx;

            if cx >= 0 && cx < grid.width as i32 && cy >= 0 && cy < grid.height as i32 {
                let candidate = (cy as usize, cx as usize);
                if grid[candidate] == 0 {
                    new_frontier.insert(candidate);
                }
            }
        }
    }

    new_frontier
}

/// Remove coordinates from frontiers that are not unique and also return the conflicting part
fn dedupe_frontiers(frontiers: Vec<Frontier>) -> (Vec<Frontier>, Frontier) {
    // Count the occurrences of every coordinate.
    let mut counts = HashMap::new();
    for frontier in frontiers.iter() {
        for coord in frontier.iter() {
            *counts.entry(*coord).or_insert(0) += 1;
        }
    }

    // Get conflicting coordinates
    let conflicts: Frontier = counts.into_iter().filter(|(_, o)| o > &1).map(|(c, _)| c).collect();

    // Filter out conflicting coordinates
    let filtered = frontiers.into_iter()
        .map(|frontier| &frontier - &conflicts)
        .collect();

    (filtered, conflicts)
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
        .map(|c| (c[1], c[0]))
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
