//! Solutions for https://adventofcode.com/2018/day/10

use utils::data::non_empty_lines;
use utils::data::load_data;
use regex::Regex;
use utils::matrix::Matrix;

#[allow(dead_code)]
pub fn part1() {
    println!("{}", solve_both_parts(get_puzzle_input()).0);
}

#[allow(dead_code)]
pub fn part2() {
    println!("{}", solve_both_parts(get_puzzle_input()).1);
}

struct Point {
    p: (i32, i32),
    v: (i32, i32)
}

fn solve_both_parts(points: Vec<Point>) -> (String, usize) {
    let mut area = area_of(&points);
    let mut t = 0;

    loop {
        t += 1;
        let new_points = project(&points, t);
        let new_area = area_of(&new_points);

        if new_area < area {
            area = new_area;
        } else {
            t -= 1;
            break (format_points(&project(&points, t)), t as usize)
        }
    }
}

fn area_of(points: &Vec<Point>) -> i64 {
    let ((x_min, x_max), (y_min, y_max)) = get_aabb(&points);
    (y_max - y_min) as i64 * (x_max - x_min) as i64
}

fn project(points: &Vec<Point>, time: i32) -> Vec<Point> {
    points.iter()
        .map(|point| {
            Point {
                p: (point.p.0 + point.v.0 * time, point.p.1 + point.v.1 * time),
                v: point.v
            }
        })
        .collect()
}

fn format_points(points: &Vec<Point>) -> String {
    let ((x_min, x_max), (y_min, y_max)) = get_aabb(&points);

    let mut grid = Matrix::new(
        (y_max - y_min + 1) as usize,
        (x_max - x_min + 1) as usize,
        '.'
    );
    for point in points {
        grid[((point.p.1 - y_min) as usize, (point.p.0 - x_min) as usize)] = '#';
    }
    
    let result = grid.rows()
        .map(|row| {
            let row = row.iter()
                .map(|c| c.to_string())
                .collect::<String>();
            format!("{}\n", row)
        })
        .collect::<String>();
    
    result.trim().to_string()
}

fn get_aabb(points: &Vec<Point>) -> ((i32, i32), (i32, i32)) {
    let mut xs: Vec<i32> = points.iter().map(|p| p.p.0).collect();
    let mut ys: Vec<i32> = points.iter().map(|p| p.p.1).collect();
    xs.sort();
    ys.sort();

    ((*xs.first().unwrap(), *xs.last().unwrap()), (*ys.first().unwrap(), *ys.last().unwrap()))
}

fn get_puzzle_input() -> Vec<Point> {
    parse_input(load_data("day10"))
}

fn parse_input(input: String) -> Vec<Point> {
    non_empty_lines(input)
        .into_iter()
        .map(parse_input_line)
        .collect()
}

fn parse_input_line(line: String) -> Point {
    let re = Regex::new(
        r"^position=<\s*(-?\d+),\s*(-?\d+)> velocity=<\s*(-?\d+),\s*(-?\d+)>$"
    ).unwrap();

    let cap = re.captures(&line).unwrap();
    Point {
        p: (cap[1].parse().unwrap(), cap[2].parse().unwrap()),
        v: (cap[3].parse().unwrap(), cap[4].parse().unwrap()),
    }
}


#[cfg(test)]
mod test {
    use super::*;
    
    #[test]
    fn test_solve() {
        let expected = String::from("
#...#..###
#...#...#.
#...#...#.
#####...#.
#...#...#.
#...#...#.
#...#...#.
#...#..###".trim());
        
        assert_eq!(
            solve_both_parts(parse_input(get_test_data())),
            (expected, 3)
        );
    }
    
    fn get_test_data() -> String {
        String::from("
position=< 9,  1> velocity=< 0,  2>
position=< 7,  0> velocity=<-1,  0>
position=< 3, -2> velocity=<-1,  1>
position=< 6, 10> velocity=<-2, -1>
position=< 2, -4> velocity=< 2,  2>
position=<-6, 10> velocity=< 2, -2>
position=< 1,  8> velocity=< 1, -1>
position=< 1,  7> velocity=< 1,  0>
position=<-3, 11> velocity=< 1, -2>
position=< 7,  6> velocity=<-1, -1>
position=<-2,  3> velocity=< 1,  0>
position=<-4,  3> velocity=< 2,  0>
position=<10, -3> velocity=<-1,  1>
position=< 5, 11> velocity=< 1, -2>
position=< 4,  7> velocity=< 0, -1>
position=< 8, -2> velocity=< 0,  1>
position=<15,  0> velocity=<-2,  0>
position=< 1,  6> velocity=< 1,  0>
position=< 8,  9> velocity=< 0, -1>
position=< 3,  3> velocity=<-1,  1>
position=< 0,  5> velocity=< 0, -1>
position=<-2,  2> velocity=< 2,  0>
position=< 5, -2> velocity=< 1,  2>
position=< 1,  4> velocity=< 2,  1>
position=<-2,  7> velocity=< 2, -2>
position=< 3,  6> velocity=<-1, -1>
position=< 5,  0> velocity=< 1,  0>
position=<-6,  0> velocity=< 2,  0>
position=< 5,  9> velocity=< 1, -2>
position=<14,  7> velocity=<-2,  0>
position=<-3,  6> velocity=< 2, -1>".trim())
    }
}
