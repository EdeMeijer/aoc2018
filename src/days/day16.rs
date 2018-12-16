use regex::Regex;

use days::day16::Op::*;
use utils::data::load_data;
use utils::data::non_empty_lines;
use std::collections::HashSet;

#[allow(dead_code)]
pub fn part1() {
    println!("{}", solve_part1(get_puzzle_samples()));
}

#[allow(dead_code)]
pub fn part2() {
    println!("{}", solve_part2(get_puzzle_samples(), get_puzzle_program()));
}

#[derive(Eq, PartialEq, Debug, Clone)]
struct Instruction {
    opcode: usize,
    a: u16,
    b: u16,
    target: usize,
}

type Register = [u16; 4];

#[derive(Eq, PartialEq, Debug, Clone)]
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
        Gtir => (a > reg[br]) as u16,
        Gtri => (reg[ar] > b) as u16,
        Gtrr => (reg[ar] > reg[br]) as u16,
        Eqir => (a == reg[br]) as u16,
        Eqri => (reg[ar] == b) as u16,
        Eqrr => (reg[ar] == reg[br]) as u16,
    };

    reg[target] = v;

    reg
}

fn solve_part1(input: Vec<InstructionSample>) -> usize {
    input.into_iter()
        .map(|samp| get_matching_opcodes(samp).len())
        .filter(|c| *c >= 3)
        .count()
}

fn solve_part2(samples: Vec<InstructionSample>, program: Vec<Instruction>) -> u16 {
    let program = remap_opcodes_from_samples(samples, program);

    // Run the program
    let mut reg = [0, 0, 0, 0];
    for instr in program {
        reg = execute_instruction(&instr, reg);
    }
    reg[0]
}

fn remap_opcodes_from_samples(
    samples: Vec<InstructionSample>, 
    program: Vec<Instruction>
) -> Vec<Instruction> {
    let opcode_mapping = determine_opcode_mapping(samples);
    
    program.into_iter()
        .map(|mut instr| {
            instr.opcode = opcode_mapping[instr.opcode];
            instr
        })
        .collect()
}

fn determine_opcode_mapping(samples: Vec<InstructionSample>) -> Vec<usize> {
    // Initially, any input opcode could map to any internal opcode
    let mut mapping: Vec<_> = (0..OPS.len())
        .map(|_| (0..OPS.len()).collect::<HashSet<_>>())
        .collect();
    
    // Determine the intersection of possible opcodes for all samples
    for sample in samples {
        let input_opcode = sample.instruction.opcode;
        let matches: HashSet<_> = get_matching_opcodes(sample).into_iter().collect();
        mapping[input_opcode] = mapping[input_opcode]
            .intersection(&matches)
            .map(|c| *c)
            .collect();
    }
    
    // Now we need to refine the possibilities. Keep looking for the opcode that maps to just one
    // other opcode; this one must be correct. This opcode can then be removed from the other sets.
    loop {
        // First let's check the opcodes that still have multiple candidates to begin with
        let ambiguous_opcodes: Vec<_> = mapping.iter().enumerate()
            .filter(|tup| tup.1.len() > 1)
            .map(|tup| tup.0)
            .collect();
        
        if ambiguous_opcodes.is_empty() {
            // We're done
            break;
        }
        
        // Now find all opcodes that have a one-to-one mapping
        let resolved: HashSet<_> = mapping.iter()
            .filter(|c| c.len() == 1)
            .map(|c| *c.iter().next().unwrap())
            .collect();
        
        // And filter them from the ambiguous codes
        for opcode in ambiguous_opcodes {
            mapping[opcode] = &mapping[opcode] - &resolved;
        }
    }
    
    mapping.into_iter()
        .map(|c| {
            assert_eq!(c.len(), 1);
            c.into_iter().next().unwrap()
        })
        .collect()
}

fn get_matching_opcodes(sample: InstructionSample) -> Vec<usize> {
    (0..OPS.len())
        .filter(|opcode| {
            let mut instr = sample.instruction.clone();
            instr.opcode = *opcode;
            let res = execute_instruction(&instr, sample.before.clone());
            res == sample.after
        })
        .collect()
}

fn get_puzzle_samples() -> Vec<InstructionSample> {
    parse_samples(load_data("day16_samples"))
}

fn get_puzzle_program() -> Vec<Instruction> {
    parse_instructions(load_data("day16_program"))
}

fn parse_samples(input: String) -> Vec<InstructionSample> {
    let lines = non_empty_lines(input);

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

fn parse_instructions(input: String) -> Vec<Instruction> {
    non_empty_lines(input).into_iter()
        .map(parse_instruction)
        .collect()
}

fn parse_register(reg: String) -> Register {
    let re = Regex::new(r"^[^:]+:\s+\[([\d, ]+)]$$").unwrap();
    let cap = re.captures(&reg).unwrap();
    let reg: Vec<_> = cap[1].split(", ").map(|v| v.parse::<u16>().unwrap()).collect();
    [reg[0], reg[1], reg[2], reg[3]]
}

fn parse_instruction(instr: String) -> Instruction {
    let parts: Vec<_> = instr.split(" ").map(|p| p.parse::<u16>().unwrap()).collect();
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
            get_matching_opcodes(first).len(),
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
