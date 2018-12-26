//! Solutions for https://adventofcode.com/2018/day/21
use std::collections::HashMap;

use utils::data::load_data;
use utils::elfcode::parse_program;
use utils::elfcode::Program;
use utils::elfcode::VM;

#[allow(dead_code)]
pub fn part1() {
    println!("{}", solve_part1(get_puzzle_input()));
}

#[allow(dead_code)]
pub fn part2() {
    println!("{}", solve_part2(get_puzzle_input()));
}

fn solve_part1(program: Program) -> u64 {
    // I figured out that the program will halt if register 3 matches register 0 at a comparison
    // at instruction 28. Let's break the program there and return the register value.
    let mut vm = VM::load(program);
    vm.add_breakpoint(28, |_| true);
    let result = vm.execute();
    result.register[3]
}

fn solve_part2(program: Program) -> u64 {
    // Let's assume that register d will stop changing at some point when the program goes into an
    // infinite loop, or at least it will enter a repeating pattern. Find the last value before the
    // first repeated value.
    let mut counter = HashMap::new();
    let mut result = 0;

    {
        let mut vm = VM::load(program);
        vm.add_breakpoint(28, |s| {
            let d = s.register[3];
            *counter.entry(d).or_insert(0) += 1;
            if counter[&d] == 1 {
                result = d;
            }
            counter[&d] > 1
        });
        vm.execute();
    }
    
    result
}

fn get_puzzle_input() -> Program {
    parse_program(load_data("day21"), 6)
}
