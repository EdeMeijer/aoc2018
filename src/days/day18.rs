use std::collections::HashMap;

use day18::Acre::*;
use utils::data::load_data;
use utils::data::non_empty_lines;
use utils::matrix::Matrix;

#[allow(dead_code)]
pub fn part1() {
    println!("{}", solve(get_puzzle_input(), 10));
}

#[allow(dead_code)]
pub fn part2() {
    println!("{}", solve(get_puzzle_input(), 1_000_000_000));
}

#[derive(Debug, Eq, PartialEq, Copy, Clone, Hash)]
enum Acre {
    Open,
    Trees,
    Lumberyard,
}

type Area = Matrix<Acre>;

fn solve(mut area: Area, minutes: usize) -> usize {
    let mut count_hist = vec![];

    for m in 0..minutes {
        area = do_step(area);

        let counts = get_total_counts(&area);
        count_hist.push(counts.clone());

        // See if we can find these counts previously and detect a cycle.
        for offset in 1..m {
            if offset * 2 <= m {
                if count_hist[m - offset] == counts && count_hist[m - offset * 2] == counts {
                    // We found a cycle so we can shortcut from here.
                    let remaining_minutes = minutes - m - 1;
                    let o = remaining_minutes % offset;
                    let counts = &count_hist[m - offset + o];
                    return counts[&Trees] * counts[&Lumberyard];
                }
            }
        }
    }

    let counts = count_hist.last().unwrap();
    counts[&Trees] * counts[&Lumberyard]
}

fn get_total_counts(area: &Area) -> HashMap<Acre, usize> {
    let mut counts = hashmap!(Open => 0, Trees => 0, Lumberyard => 0);
    for y in 0..area.height {
        for x in 0..area.width {
            *counts.entry(area[(y, x)]).or_insert(0) += 1;
        }
    }
    counts
}

fn do_step(area: Area) -> Area {
    let mut new = Matrix::new(area.height, area.width, Acre::Open);

    for y in 0..area.height {
        for x in 0..area.width {
            let c = count_surrounding(&area, (y, x));

            new[(y, x)] = match area[(y, x)] {
                Open => if c[&Trees] >= 3 { Trees } else { Open },
                Trees => if c[&Lumberyard] >= 3 { Lumberyard } else { Trees },
                Lumberyard => if c[&Lumberyard] > 0 && c[&Trees] > 0 { Lumberyard } else { Open }
            }
        }
    }
    new
}

fn count_surrounding(area: &Area, pos: (usize, usize)) -> HashMap<Acre, usize> {
    let mut count = hashmap!(Open => 0, Trees => 0, Lumberyard => 0);

    for oy in -1..=1 {
        for ox in -1..=1 {
            if !(oy == 0 && ox == 0) {
                let (y, x) = (pos.0 as i32 - oy, pos.1 as i32 - ox);
                if y >= 0 && y < area.height as i32 && x >= 0 && x < area.width as i32 {
                    *count.entry(area[(y as usize, x as usize)]).or_insert(0) += 1;
                }
            }
        }
    }
    count
}

fn get_puzzle_input() -> Area {
    parse_input(load_data("day18"))
}

fn parse_input(input: String) -> Area {
    let lines = non_empty_lines(input);
    let height = lines.len();
    let width = lines[0].len();

    let mut area = Matrix::new(height, width, Open);

    for (y, line) in lines.into_iter().enumerate() {
        for (x, cell) in line.chars().enumerate() {
            area[(y, x)] = match cell {
                '.' => Open,
                '|' => Trees,
                '#' => Lumberyard,
                _ => panic!("{}", cell)
            };
        }
    }

    area
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(
            solve(parse_input(get_test_input()), 10),
            1147
        );
    }

    fn get_test_input() -> String {
        String::from(r"
.#.#...|#.
.....#|##|
.|..|...#.
..|#.....#
#.#|||#|#|
...#.||...
.|....|...
||...#|.#|
|.||||..|.
...#.|..|.
")
    }
}
