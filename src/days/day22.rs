//! Solutions for https://adventofcode.com/2018/day/22
use std::collections::HashMap;
use std::collections::HashSet;

use utils::matrix::Matrix;

#[allow(dead_code)]
pub fn part1() {
    println!("{}", solve_part1(10914, (739, 9)));
}

#[allow(dead_code)]
pub fn part2() {
    println!("{}", solve_part2(10914, (739, 9)));
}


fn solve_part1(cave_depth: usize, target: (usize, usize)) -> usize {
    let risk_levels = get_risk_levels(cave_depth, target, target);

    let mut risk_level = 0;
    for y in 0..=target.0 {
        for x in 0..=target.1 {
            risk_level += risk_levels[(y, x)];
        }
    }
    risk_level
}

fn solve_part2(cave_depth: usize, target: (usize, usize)) -> usize {
    // Pre-calculate the risk levels. Since we can go beyond the target x and y, let's set an
    // upper bound here. Let's assume we use a shortest manhattan distance and have to switch for
    // every cell, that would be (target_x + target_y) * (1 + 7).
    let max_xy = (target.0 + target.1) * (1 + 7);

    let risk_levels = get_risk_levels(cave_depth, target, (max_xy, max_xy));

    // We'll use dijkstra's algorithm to find the fastest path to the target. Imagine the problem as
    // a 3d grid where we have 3 layers where the layers represent the configurations of tools.
    // Layer 0 is none, layer 1 is torch and layer 2 is climbing gear. Switching between layers
    // takes 7 minutes, while switching between x,y coordinates takes 1 minute (but not every move
    // is valid, depending on the layer). We have to end at the target, in layer 1. This gives us
    // a graph with different length edges, so dijkstra's algorithm is perfect here.

    let mut pending = HashSet::new();
    let mut visited = HashSet::new();
    let start = (0, 0, 1); // torch at the entrance
    let target = (target.0, target.1, 1); // torch at target
    let mut distances = HashMap::new();
    distances.insert(start, 0);
    pending.insert(start);

    let offsets = [(-1, 0, 0), (1, 0, 0), (0, -1, 0), (0, 1, 0), (0, 0, 1), (0, 0, 2)];

    while !pending.is_empty() && !visited.contains(&target) {
        let cur = pending.iter().min_by_key(|t| distances[t]).unwrap().clone();

        let cur_dist = distances[&cur];
        let move_dist = cur_dist + 1;
        let switch_dist = cur_dist + 7;

        // Calculate all distances through the current node to neighboring nodes.
        for (oy, ox, ol) in offsets.iter() {
            let (y, x) = (cur.0 as i32 + oy, cur.1 as i32 + ox);
            if x >= 0 && y >= 0 {
                let dist = if *ol == 0 { move_dist } else { switch_dist };

                let (y, x) = (y as usize, x as usize);
                let layer = (cur.2 + ol) % 3;
                let t = (y, x, layer);
                // Check if this is a valid location for the current equipment
                let risk_level = risk_levels[(y, x)];
                let is_valid = match (risk_level, t.2) {
                    (0, 2) | (0, 1) | (1, 2) | (1, 0) | (2, 1) | (2, 0) => true,
                    _ => false
                };
                if is_valid && !visited.contains(&t) {
                    if !distances.contains_key(&t) || dist < distances[&t] {
                        distances.insert(t, dist);
                        pending.insert(t);
                    }
                }
            }
        }

        visited.insert(cur);
        pending.remove(&cur);
    }

    distances[&target]
}

fn get_risk_levels(cave_depth: usize, target: (usize, usize), max: (usize, usize)) -> Matrix<usize> {
    let mut erosion_levels = Matrix::new(max.0 + 1, max.1 + 1, 0);
    let mut risk_levels = Matrix::new(max.0 + 1, max.1 + 1, 0);

    for y in 0..=max.0 {
        for x in 0..=max.1 {
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
            risk_levels[(y, x)] = ero_level % 3;
        }
    }

    risk_levels
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

    #[test]
    fn test_part2() {
        assert_eq!(
            solve_part2(510, (10, 10)),
            45
        );
    }
}
