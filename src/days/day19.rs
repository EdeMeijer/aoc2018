//! Solutions for https://adventofcode.com/2018/day/19
use utils::data::load_data;
use utils::elfcode::parse_program;
use utils::elfcode::Program;
use utils::elfcode::VM;

#[allow(dead_code)]
pub fn part1() {
    println!("{}", solve(get_puzzle_input()));
}

#[allow(dead_code)]
pub fn part2() {
    println!("Nope, figured it out manually");
}

fn solve(program: Program) -> u32 {
    VM::load(program).execute().register[0]
}

fn get_puzzle_input() -> Program {
    parse_program(load_data("day19"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(
            solve(parse_program(get_test_input())),
            6
        );
    }

    fn get_test_input() -> String {
        String::from(r"
#ip 0
seti 5 0 1
seti 6 0 2
addi 0 1 0
addr 1 2 3
setr 1 0 0
seti 8 0 4
seti 9 0 5
")
    }
}
