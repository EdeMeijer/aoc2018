//! Solutions for https://adventofcode.com/2018/day/21
use utils::data::load_data;
use utils::elfcode::parse_program;
use utils::elfcode::Program;
use utils::elfcode::VM;

#[allow(dead_code)]
pub fn part1() {
    println!("{}", solve_part1(get_puzzle_input()));
}

fn solve_part1(program: Program) -> u64 {
    // I figured out that the program will halt if register 3 matches register 0 at a comparison
    // at instruction 28. Let's break the program there and return the register value.
    let mut vm = VM::load(program);
    vm.add_breakpoint(28, |_| true);
    let result = vm.execute();
    result.register[3]
}

fn get_puzzle_input() -> Program {
    parse_program(load_data("day21"), 6)
}
