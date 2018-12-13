//! Solutions for https://adventofcode.com/2018/day/11
#[allow(dead_code)]
pub fn part1() {
    println!("{:?}", solve_part1(300, 8868));
}

#[allow(dead_code)]
pub fn part2() {
    println!("{:?}", solve_part2(300, 8868));
}

fn solve_part1(grid_size: usize, sn: usize) -> (usize, usize, i32) {
    solve(grid_size, sn, 3)
}

fn solve_part2(grid_size: usize, sn: usize) -> (usize, usize, usize, i32) {
    let mut best = (0, 0, 0, -10000);
    for bs in 1..=300 {
        let (x, y, p) = solve(grid_size, sn, bs);
        if p > best.3 {
            best = (x, y, bs, p);
        }
    }
    best
}

fn solve(grid_size: usize, sn: usize, bs: usize) -> (usize, usize, i32) {
    let mut best = (0, 0, -10000);

    // Create a list of sums of vertical slices of the power levels starting at y=0 and height of
    // the block size. One for every x coordinate.
    let mut chunks: Vec<_> = (0..grid_size).map(|x| {
        (0..bs).map(|y| power_level(x, y, sn)).sum::<i32>()
    }).collect();

    for y in 0..grid_size - (bs - 1) {
        // Calculate the total power of the first block at (x=0, y)
        let mut total = chunks.iter().take(bs).sum::<i32>();
        // For every x, update the total power by removing the leftmost chunk and adding one to the
        // right.
        for x in 0..grid_size - (bs - 1) {
            // First update the best score
            if total > best.2 {
                best = (x, y, total);
            }
            if x < grid_size - bs {
                total += chunks[x + bs] - chunks[x];
            }
        }

        // After a horizontal scan, move all the chunks one position down by subtracting the topmost
        // row of power levels and adding one to the bottom.
        if y < grid_size - bs {
            for x in 0..grid_size - (bs - 1) {
                chunks[x] += power_level(x, y + bs, sn) - power_level(x, y, sn);
            }
        }
    }
    best
}

fn power_level(x: usize, y: usize, sn: usize) -> i32 {
    let rack = x + 10;
    let power = rack * y;
    let power = power + sn;
    let power = power * rack;
    let power = (power % 1000) / 100;
    power as i32 - 5
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(solve_part1(300, 18), (33, 45, 29));
    }

    #[test]
    fn test_part2() {
        assert_eq!(solve_part2(300, 18), (90, 269, 16, 113));
        assert_eq!(solve_part2(300, 42), (232, 251, 12, 119));
    }

    #[test]
    fn test_power_level() {
        assert_eq!(power_level(3, 5, 8), 4);
        assert_eq!(power_level(122, 79, 57), -5);
        assert_eq!(power_level(217, 196, 39), 0);
        assert_eq!(power_level(101, 153, 71), 4);
    }
}
