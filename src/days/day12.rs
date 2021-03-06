//! Solutions for https://adventofcode.com/2018/day/12
use utils::data::load_data;
use utils::data::non_empty_lines;

#[allow(dead_code)]
pub fn part1() {
    println!("{}", solve(get_puzzle_input(), 20));
}

#[allow(dead_code)]
pub fn part2() {
    println!("{}", solve(get_puzzle_input(), 50_000_000_000));
}

#[derive(Debug, Eq, PartialEq)]
struct Scenario {
    initial: Vec<u8>,
    kernels: Vec<Vec<u8>>,
}

fn solve(scenario: Scenario, generations: usize) -> i64 {
    let max_generations = 10000;

    let offset = 2 * max_generations;
    let lower = -(offset as i64);
    let upper = scenario.initial.len() as i64 + offset as i64;

    let mut source = vec![0; (upper - lower) as usize];
    let mut target = vec![0; (upper - lower) as usize];

    for (i, p) in scenario.initial.iter().enumerate() {
        source[i + offset] = *p;
    }

    let mut gv = gen_value(&source, lower, upper);
    let mut last_delta_gv = 0;
    let mut same_delta_count = 0;

    for g in 0..generations {
        if g == max_generations {
            panic!("Did not find a solution within {} generations", max_generations);
        }
        let r = apply_generation(source, target, &scenario.kernels);
        source = r.0;
        target = r.1;

        let mut gv_new = gen_value(&source, lower, upper);
        let delta_gv = gv_new - gv;
        gv = gv_new;

        if delta_gv == last_delta_gv {
            same_delta_count += 1;
            if same_delta_count == 10 {
                // Assume the same delta will keep repeating.
                let remaining_gens = generations - g - 1;
                return gv + remaining_gens as i64 * delta_gv;
            }
        } else {
            same_delta_count = 0;
        }

        last_delta_gv = delta_gv;
    }

    gen_value(&source, lower, upper)
}

fn gen_value(gen: &Vec<u8>, lower: i64, upper: i64) -> i64 {
    gen.iter().zip(lower..upper)
        .map(|(has_plant, potno)| potno * *has_plant as i64)
        .sum::<i64>()
}

fn apply_generation(source: Vec<u8>, mut target: Vec<u8>, kernels: &Vec<Vec<u8>>) -> (Vec<u8>, Vec<u8>) {
    for i in 0..source.len() {
        let mut matches = 0;
        for kernel in kernels {
            if kernel_matches(&source, i, kernel) {
                matches = 1;
                break;
            }
        }
        target[i] = matches;
    }

    (target, source)
}

fn kernel_matches(state: &Vec<u8>, offset: usize, kernel: &Vec<u8>) -> bool {
    kernel.iter().enumerate()
        .all(|(i, z)| {
            let si = i as i64 - 2 + offset as i64;
            if si < 0 || si >= state.len() as i64 {
                false
            } else {
                state[si as usize] == *z
            }
        })
}

fn get_puzzle_input() -> Scenario {
    parse_input(load_data("day12"))
}

fn parse_input(input: String) -> Scenario {
    let mut lines = non_empty_lines(input).into_iter();

    let initial = parse_state(lines.next().unwrap().split_at(15).1);

    let kernels: Vec<_> = lines
        .map(|line| line.split(" => ").map(|s| s.to_owned()).collect::<Vec<String>>())
        .filter(|s| &s[1] == "#")
        .map(|s| parse_state(&s[0]))
        .collect();

    Scenario { initial, kernels }
}

fn parse_state(state: &str) -> Vec<u8> {
    state.chars().map(|c| (c == '#') as u8).collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(
            solve(parse_input(get_test_input()), 20),
            325
        );
    }

    #[test]
    fn test_parse() {
        let expected = Scenario {
            initial: vec![1, 0, 0, 1, 0, 1, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0, 1, 1, 1],
            kernels: vec![
                vec![0, 0, 0, 1, 1],
                vec![0, 0, 1, 0, 0],
                vec![0, 1, 0, 0, 0],
                vec![0, 1, 0, 1, 0],
                vec![0, 1, 0, 1, 1],
                vec![0, 1, 1, 0, 0],
                vec![0, 1, 1, 1, 1],
                vec![1, 0, 1, 0, 1],
                vec![1, 0, 1, 1, 1],
                vec![1, 1, 0, 1, 0],
                vec![1, 1, 0, 1, 1],
                vec![1, 1, 1, 0, 0],
                vec![1, 1, 1, 0, 1],
                vec![1, 1, 1, 1, 0],
            ],
        };

        assert_eq!(
            parse_input(get_test_input()),
            expected
        );
    }

    fn get_test_input() -> String {
        String::from("initial state: #..#.#..##......###...###

..... => .
...## => #
..#.. => #
.#... => #
.#.#. => #
.#.## => #
.##.. => #
.#### => #
#.#.# => #
#.### => #
##.#. => #
##.## => #
###.. => #
###.# => #
####. => #")
    }
}
