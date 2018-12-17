use std::fmt::Display;
use std::fmt::Error;
use std::fmt::Formatter;

use regex::Regex;

use days::day17::Square::*;
use utils::data::load_data;
use utils::data::non_empty_lines;
use utils::matrix::Matrix;

#[allow(dead_code)]
pub fn part1() {
    println!("{}", solve_part1(get_puzzle_input()));
}

#[allow(dead_code)]
pub fn part2() {
    println!("{}", solve_part2(get_puzzle_input()));
}

type Coord = (usize, usize);

const SPRING_X: usize = 500;

#[derive(Clone, Eq, PartialEq)]
struct World {
    grid: Grid,
    offset: Coord,
}

type Grid = Matrix<Square>;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum Square {
    Sand,
    Clay,
    SettledWater,
    StreamingWater,
}

impl Display for Square {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        let c = match self {
            Sand => ".",
            Clay => "#",
            SettledWater => "~",
            StreamingWater => "|",
        };
        f.write_str(c)
    }
}

#[derive(Eq, PartialEq, Debug)]
struct Vein {
    y: Coord,
    x: Coord,
}

fn solve_part1(veins: Vec<Vein>) -> usize {
    count_squares_of_type(fully_simulate(veins), vec![StreamingWater, SettledWater])
}

fn solve_part2(veins: Vec<Vein>) -> usize {
    count_squares_of_type(fully_simulate(veins), vec![SettledWater])
}

fn fully_simulate(veins: Vec<Vein>) -> World {
    let mut world = build_world(veins);

    // Let the source spawn streaming water right beneath it
    let local_spring = (0, SPRING_X - world.offset.1);
    world = project_stream_down(world, local_spring);

    loop {
        let new_world = do_tick(world.clone());
        if new_world == world {
            // Nothing changed, so we're done
            break world;
        }
        world = new_world;
    }
}

fn count_squares_of_type(world: World, types: Vec<Square>) -> usize {
    let mut count = 0;
    for y in 0..world.grid.height {
        for x in 0..world.grid.width {
            if types.contains(&world.grid[(y, x)]) {
                count += 1;
            }
        }
    }
    count
}

fn build_world(veins: Vec<Vein>) -> World {
    let min_x = veins.iter().map(|v| v.x.0).min().unwrap() - 1;
    let max_x = veins.iter().map(|v| v.x.1).max().unwrap() + 1;
    let min_y = veins.iter().map(|v| v.y.0).min().unwrap();
    let max_y = veins.iter().map(|v| v.y.1).max().unwrap();

    let height = max_y - min_y + 1;
    let width = max_x - min_x + 1;

    let mut grid = Grid::new(height, width, Sand);
    for vein in veins {
        for y in vein.y.0..=vein.y.1 {
            for x in vein.x.0..=vein.x.1 {
                grid[(y - min_y, x - min_x)] = Clay;
            }
        }
    }

    World { grid, offset: (min_y, min_x) }
}

fn do_tick(mut world: World) -> World {
    let (height, width) = (world.grid.height, world.grid.width);

    // Step 1: project down the streaming water
    for y in 0..height {
        for x in 0..width {
            let coord = (y, x);
            if world.grid[coord] == StreamingWater {
                world = project_stream_down(world, coord);
            }
        }
    }

    // Step 1: flood fill
    for y in 0..height - 1 {
        for x in 0..width {
            let coord = (y, x);
            if world.grid[coord] == StreamingWater {
                let beneath = world.grid[(y + 1, x)];
                if beneath == Clay || beneath == SettledWater {
                    world = flood_fill(world, coord);
                }
            }
        }
    }

    world
}

fn project_stream_down(mut world: World, local_coord: Coord) -> World {
    let (start_y, x) = local_coord;
    let height = world.grid.height;
    for y in start_y..height {
        let coord = (y, x);
        if world.grid[coord] != Sand && y > start_y {
            break;
        }
        world.grid[coord] = StreamingWater;
    }
    world
}

fn flood_fill(mut world: World, local_coord: Coord) -> World {
    let y = local_coord.0;
    let (span_left, span_right, filler) = calc_flood_fill(&world, local_coord);

    for x in span_left..=span_right {
        world.grid[(y, x)] = filler;
    }
    world
}

fn calc_flood_fill(world: &World, local_coord: Coord) -> (usize, usize, Square) {
    let y = local_coord.0;

    let partial_fill = |dir: i32| {
        let mut found_clay = false;
        let mut x = local_coord.1;
        let mut span = x;
        while x > 0 && x < world.grid.width - 1 {
            x = (x as i32 + dir) as usize;
            if world.grid[(y, x)] == Clay {
                // Found clay on the left
                found_clay = true;
                break;
            }
            span = x;
            let beneath = world.grid[(y + 1, x)];
            if beneath == Sand || beneath == StreamingWater {
                // Found sand or streaming water beneath, so we can stop here
                break;
            }
        }
        (span, found_clay)
    };

    let (span_left, clay_left) = partial_fill(-1);
    let (span_right, clay_right) = partial_fill(1);
    let filler = if clay_left && clay_right { SettledWater } else { StreamingWater };

    (span_left, span_right, filler)
}


fn get_puzzle_input() -> Vec<Vein> {
    parse_input(load_data("day17"))
}


fn parse_input(input: String) -> Vec<Vein> {
    non_empty_lines(input).into_iter()
        .map(parse_vein)
        .collect()
}

fn parse_vein(input: String) -> Vein {
    let re = Regex::new(r"^([xy])=(\d+), [xy]=(\d+)..(\d+)$").unwrap();
    let cap = re.captures(&input).unwrap();

    let vertical = &cap[1] == "x";
    let c1: Coord = (cap[2].parse().unwrap(), cap[2].parse().unwrap());
    let c2: Coord = (cap[3].parse().unwrap(), cap[4].parse().unwrap());
    let (x, y) = if vertical { (c1, c2) } else { (c2, c1) };
    Vein { y, x }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(
            solve_part1(parse_input(get_test_input())),
            57
        );
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            solve_part2(parse_input(get_test_input())),
            29
        );
    }

    #[test]
    fn test_parse() {
        let expected = vec![
            Vein { x: (495, 495), y: (2, 7) },
            Vein { x: (495, 501), y: (7, 7) },
            Vein { x: (501, 501), y: (3, 7) },
            Vein { x: (498, 498), y: (2, 4) },
            Vein { x: (506, 506), y: (1, 2) },
            Vein { x: (498, 498), y: (10, 13) },
            Vein { x: (504, 504), y: (10, 13) },
            Vein { x: (498, 504), y: (13, 13) },
        ];

        assert_eq!(
            parse_input(get_test_input()),
            expected,
        );
    }

    fn get_test_input() -> String {
        String::from(r"
x=495, y=2..7
y=7, x=495..501
x=501, y=3..7
x=498, y=2..4
x=506, y=1..2
x=498, y=10..13
x=504, y=10..13
y=13, x=498..504
")
    }
}
