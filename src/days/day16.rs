use regex::Regex;

use days::day16::Op::*;
use utils::data::load_data;
use utils::data::non_empty_lines;

#[allow(dead_code)]
pub fn part1() {
    println!("{}", solve_part1(get_puzzle_samples()));
}


#[derive(Eq, PartialEq, Debug, Clone)]
struct Instruction {
    opcode: usize,
    a: u8,
    b: u8,
    target: usize,
}

type Register = [u8; 4];

#[derive(Eq, PartialEq, Debug)]
struct InstructionSample {
    before: Register,
    instruction: Instruction,
    after: Register,
}

enum Op {
    Addr,
    Addi,
    Mulr,
    Muli,
    Banr,
    Bani,
    Borr,
    Bori,
    Setr,
    Seti,
    Gtir,
    Gtri,
    Gtrr,
    Eqir,
    Eqri,
    Eqrr,
}

const OPS: [Op; 16] = [
    Addr, Addi, Mulr, Muli, Banr, Bani, Borr, Bori, Setr, Seti, Gtir, Gtri, Gtrr, Eqir, Eqri, Eqrr
];

fn execute_instruction(instr: &Instruction, mut reg: Register) -> Register {
    let Instruction { opcode, a, b, target } = *instr;

    let op = &OPS[opcode];
    let ar = a as usize;
    let br = b as usize;

    let v = match op {
        Addr => reg[ar] + reg[br],
        Addi => reg[ar] + b,
        Mulr => reg[ar] * reg[br],
        Muli => reg[ar] * b,
        Banr => reg[ar] & reg[br],
        Bani => reg[ar] & b,
        Borr => reg[ar] | reg[br],
        Bori => reg[ar] | b,
        Setr => reg[ar],
        Seti => a,
        Gtir => (a > reg[br]) as u8,
        Gtri => (reg[ar] > b) as u8,
        Gtrr => (reg[ar] > reg[br]) as u8,
        Eqir => (a == reg[br]) as u8,
        Eqri => (reg[ar] == b) as u8,
        Eqrr => (reg[ar] == reg[br]) as u8,
    };

    reg[target] = v;

    reg
}

fn solve_part1(input: Vec<InstructionSample>) -> usize {
    input.into_iter()
        .map(count_matching_opcodes)
        .filter(|c| *c >= 3)
        .count()
}

fn count_matching_opcodes(sample: InstructionSample) -> usize {
    (0..OPS.len())
        .filter(|opcode| {
            let mut instr = sample.instruction.clone();
            instr.opcode = *opcode;
            let res = execute_instruction(&instr, sample.before.clone());
            res == sample.after
        })
        .count()
}

fn get_puzzle_samples() -> Vec<InstructionSample> {
    parse_samples(load_data("day16_samples"))
}

fn parse_samples(samples: String) -> Vec<InstructionSample> {
    let lines = non_empty_lines(samples);

    let mut iter = lines.into_iter();
    let mut result = vec![];
    while let (Some(b), Some(i), Some(a)) = (iter.next(), iter.next(), iter.next()) {
        result.push(InstructionSample {
            before: parse_register(b),
            instruction: parse_instruction(i),
            after: parse_register(a),
        });
    }
    result
}

fn parse_register(reg: String) -> Register {
    let re = Regex::new(r"^[^:]+:\s+\[([\d, ]+)]$$").unwrap();
    let cap = re.captures(&reg).unwrap();
    let reg: Vec<_> = cap[1].split(", ").map(|v| v.parse::<u8>().unwrap()).collect();
    [reg[0], reg[1], reg[2], reg[3]]
}

fn parse_instruction(instr: String) -> Instruction {
    let parts: Vec<_> = instr.split(" ").map(|p| p.parse::<u8>().unwrap()).collect();
    Instruction {
        opcode: parts[0] as usize,
        a: parts[1],
        b: parts[2],
        target: parts[3] as usize,
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(
            solve_part1(parse_samples(get_test_input())),
            1
        );
    }
    
    #[test]
    fn test_count_matching_opcodes() {
        let first = parse_samples(get_test_input()).into_iter().next().unwrap();
        assert_eq!(
            count_matching_opcodes(first),
            3
        );
    }

    #[test]
    fn test_parse() {
        let expected = InstructionSample {
            before: [3, 2, 1, 1],
            instruction: Instruction {
                opcode: 9,
                a: 2,
                b: 1,
                target: 2,
            },
            after: [3, 2, 2, 1],
        };

        assert_eq!(
            parse_samples(get_test_input()),
            vec![expected],
        );
    }

    fn get_test_input() -> String {
        String::from(r"
Before: [3, 2, 1, 1]
9 2 1 2
After:  [3, 2, 2, 1]
")
    }
}
