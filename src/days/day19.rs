use utils::data::load_data;
use utils::data::non_empty_lines;

#[allow(dead_code)]
pub fn part1() {
    println!("{}", solve(get_puzzle_input(), [0, 0, 0, 0, 0, 0]));
}

#[allow(dead_code)]
pub fn part2() {
    println!("Nope, figured it out manually");
}

#[derive(Eq, PartialEq, Debug, Clone)]
struct Instruction {
    opcode: String,
    a: u32,
    b: u32,
    target: usize,
}

type Register = [u32; 6];

type Input = (Vec<Instruction>, usize);

fn solve(input: Input, mut register: Register) -> u32 {
    let mut ip = 0;

    let (instructions, ip_binding) = input;

    while ip < instructions.len() {
        register[ip_binding] = ip as u32;
        register = execute_instruction(&instructions[ip as usize], register);
        ip = register[ip_binding] as usize;
        ip += 1;
    }

    register[0]
}

fn execute_instruction(instr: &Instruction, mut reg: Register) -> Register {
    let Instruction { ref opcode, a, b, target } = *instr;

    let ar = a as usize;
    let br = b as usize;

    reg[target] = match opcode.as_ref() {
        "addr" => reg[ar] + reg[br],
        "addi" => reg[ar] + b,
        "mulr" => reg[ar] * reg[br],
        "muli" => reg[ar] * b,
        "banr" => reg[ar] & reg[br],
        "bani" => reg[ar] & b,
        "borr" => reg[ar] | reg[br],
        "bori" => reg[ar] | b,
        "setr" => reg[ar],
        "seti" => a,
        "gtir" => (a > reg[br]) as u32,
        "gtri" => (reg[ar] > b) as u32,
        "gtrr" => (reg[ar] > reg[br]) as u32,
        "eqir" => (a == reg[br]) as u32,
        "eqri" => (reg[ar] == b) as u32,
        "eqrr" => (reg[ar] == reg[br]) as u32,
        _ => panic!("Unsupported opcode {}", opcode)
    };

    reg
}

fn get_puzzle_input() -> Input {
    parse_input(load_data("day19"))
}

fn parse_input(input: String) -> Input {
    let mut lines = non_empty_lines(input).into_iter();
    let first = lines.next().unwrap();

    let instructions = lines
        .map(parse_instruction)
        .collect();

    (instructions, parse_ip_binding(first))
}

fn parse_ip_binding(binding: String) -> usize {
    let parts: Vec<_> = binding.split(" ").collect();
    parts[1].parse().unwrap()
}

fn parse_instruction(instr: String) -> Instruction {
    let parts: Vec<_> = instr.split(" ").collect();

    let opcode = parts[0].to_owned();
    let a = parts[1].parse::<u32>().unwrap();
    let b = parts[2].parse::<u32>().unwrap();
    let target = parts[3].parse::<usize>().unwrap();

    Instruction { opcode, a, b, target }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(
            solve(parse_input(get_test_input()), [0, 0, 0, 0, 0, 0]),
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
