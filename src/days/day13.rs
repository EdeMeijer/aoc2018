//! Solutions for https://adventofcode.com/2018/day/13
use std::collections::HashMap;
use std::collections::HashSet;
use std::fmt::Display;
use std::fmt::Error;
use std::fmt::Formatter;
use std::fmt::Write;

use utils::data::load_data;
use utils::data::non_empty_lines;
use utils::matrix::Matrix;

#[allow(dead_code)]
pub fn part1() {
    let (y, x) = solve_part1(get_puzzle_input());
    println!("{},{}", x, y);
}

#[allow(dead_code)]
pub fn part2() {
    let (y, x) = solve_part2(get_puzzle_input());
    println!("{},{}", x, y);
}

struct Scenario {
    track: Matrix<TrackCell>,
    carts: Vec<Cart>,
}

#[derive(Debug, Eq, PartialEq)]
struct Cart {
    // y, x
    pos: (usize, usize),
    // y, x
    dir: (i32, i32),
    intersection_count: usize,
}

enum GridCell {
    // Direction y, x
    Cart(i32, i32),
    Track(char),
}

#[derive(Copy, Clone)]
enum TrackCell {
    Empty,
    Straight,
    Curve { sign: i32 },
    Intersection,
}

impl Display for TrackCell {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        let c = match *self {
            TrackCell::Empty => ' ',
            TrackCell::Straight => '.',
            TrackCell::Curve { sign: 1 } => '\\',
            TrackCell::Curve { sign: -1 } => '/',
            TrackCell::Intersection => '+',
            _ => '?'
        };
        f.write_char(c)
    }
}

fn solve_part1(mut scenario: Scenario) -> (usize, usize) {
    loop {
        if let Some(pos) = do_tick(&mut scenario) {
            break pos;
        }
    }
}

fn solve_part2(mut scenario: Scenario) -> (usize, usize) {
    loop {
        do_tick(&mut scenario);
        if scenario.carts.is_empty() {
            panic!("No carts remaining");
        }
        if scenario.carts.len() == 1 {
            break scenario.carts.remove(0).pos;
        }
    }
}

fn do_tick(scenario: &mut Scenario) -> Option<(usize, usize)> {
    // Sort the carts by evaluation order, which is y, x order
    scenario.carts.sort_by_key(|cart| cart.pos);

    // Record the starting positions of the carts along with their sort order. After performing the
    // step, we need this to detect crashes.
    let start_positions: HashMap<_, _> = scenario.carts.iter().enumerate().map(|(i, cart)| (cart.pos, i)).collect();

    // Perform cart actions
    for cart in &mut scenario.carts {
        // Move one step in the current direction
        cart.pos = (
            (cart.pos.0 as i32 + cart.dir.0) as usize,
            (cart.pos.1 as i32 + cart.dir.1) as usize
        );

        // Check current track type and act if necessary
        match scenario.track[cart.pos] {
            TrackCell::Straight => {}
            TrackCell::Curve { sign } => cart.dir = (sign * cart.dir.1, sign * cart.dir.0),
            TrackCell::Intersection => {
                match cart.intersection_count % 3 {
                    0 => Some(-1),
                    1 => None,
                    2 => Some(1),
                    _ => panic!()
                }.map(|sign| {
                    let sign = sign * if cart.dir.0 == 0 { 1 } else { -1 };
                    cart.dir = (sign * cart.dir.1, sign * cart.dir.0);
                });

                cart.intersection_count += 1;
            }
            _ => panic!("Invalid state")
        };
    }

    // Crash detection
    let mut end_positions: HashMap<(usize, usize), _> = HashMap::with_capacity(scenario.carts.len());

    let mut crashes = vec![];
    let mut crashed_carts = HashSet::new();

    for (i, cart) in scenario.carts.iter().enumerate() {
        if crashed_carts.contains(&i) {
            // Something already crashed into this cart so we consider it to not have moved
            continue;
        }

        let mut crashed_with = None;

        if end_positions.contains_key(&cart.pos) {
            // This cart ends at the same position as another one, so it crashed
            crashed_with = Some(end_positions[&cart.pos]);
        }

        end_positions.insert(cart.pos, i);

        if start_positions.contains_key(&cart.pos) {
            // Another cart started at our end position. If that cart would have moved after the 
            // current cart did, that was a crash.
            if start_positions[&cart.pos] > i {
                crashed_with = Some(start_positions[&cart.pos]);
            }
        }

        if let Some(other_i) = crashed_with {
            crashed_carts.insert(i);
            crashed_carts.insert(other_i);

            crashes.push(cart.pos.clone());
        }
    }

    // Filter out the crashed carts
    let mut filter_indices: Vec<usize> = crashed_carts.into_iter().collect();
    filter_indices.sort();
    for (removed, i) in filter_indices.into_iter().enumerate() {
        scenario.carts.remove(i - removed);
    }

    if crashes.is_empty() { None } else { Some(crashes.remove(0)) }
}

fn get_puzzle_input() -> Scenario {
    parse_input(load_data("day13"))
}

fn parse_input(input: String) -> Scenario {
    let lines: Vec<_> = non_empty_lines(input)
        .into_iter()
        .map(|line| String::from(line.trim_right()))
        .collect();

    // Determine the width of the grid by checking the longest trimmed input line
    let width = lines.iter().map(|line| line.len()).max().unwrap_or(0);

    // Build the grid of tracks, replacing the carts with straight tracks
    let mut grid = Matrix::new(lines.len(), width, TrackCell::Empty);
    // Create a list of carts
    let mut carts = vec![];

    for (y, line) in lines.into_iter().enumerate() {
        for (x, cell) in line.chars().enumerate() {
            let cell = match cell {
                '>' => GridCell::Cart(0, 1),
                '<' => GridCell::Cart(0, -1),
                '^' => GridCell::Cart(-1, 0),
                'v' => GridCell::Cart(1, 0),
                _ => GridCell::Track(cell)
            };

            match cell {
                GridCell::Cart(vy, vx) => {
                    carts.push(Cart { pos: (y, x), dir: (vy, vx), intersection_count: 0 });
                    grid[(y, x)] = TrackCell::Straight
                }
                GridCell::Track(t) => grid[(y, x)] = match t {
                    '|' | '-' => TrackCell::Straight,
                    '/' => TrackCell::Curve { sign: -1 },
                    '\\' => TrackCell::Curve { sign: 1 },
                    '+' => TrackCell::Intersection,
                    ' ' => TrackCell::Empty,
                    _ => panic!("Unsupported track type {}", t),
                }
            }
        }
    }

    Scenario { track: grid, carts }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(
            solve_part1(parse_input(get_test_input())),
            (3, 7)
        )
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            solve_part2(parse_input(get_test_input_part2())),
            (4, 6)
        )
    }


    #[test]
    fn test_parse() {
        let scenario = parse_input(get_test_input());

        let expected_grid = String::from(r"
/...\        
.   .  /....\
. /.+..+.\  .
. . .  . .  .
\.+./  \.+../
  \....../   ");

        let expected_carts = vec![
            Cart { pos: (0, 2), dir: (0, 1), intersection_count: 0 },
            Cart { pos: (3, 9), dir: (1, 0), intersection_count: 0 },
        ];

        assert_eq!(expected_grid.trim(), scenario.track.format_dense().unwrap().trim());
        assert_eq!(expected_carts, scenario.carts);
    }

    fn get_test_input() -> String {
        return String::from(r"
/->-\        
|   |  /----\
| /-+--+-\  |
| | |  | v  |
\-+-/  \-+--/
  \------/   ");
    }

    fn get_test_input_part2() -> String {
        return String::from(r"
/>-<\  
|   |  
| /<+-\
| | | v
\>+</ |
  |   ^
  \<->/");
    }
}
