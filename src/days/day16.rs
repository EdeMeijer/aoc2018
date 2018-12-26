//! Solutions for https://adventofcode.com/2018/day/16
use std::collections::HashMap;
use std::collections::HashSet;

use regex::Regex;

use utils::data::load_data;
use utils::data::non_empty_lines;
use utils::elfcode::Instruction;
use utils::elfcode::parse_instruction;
use utils::elfcode::parse_program;
use utils::elfcode::Program;
use utils::elfcode::Register;
use utils::elfcode::VM;
use utils::elfcode::get_opcodes;
use utils::elfcode::execute_instruction;

#[allow(dead_code)]
pub fn part1() {
    println!("{}", solve_part1(get_puzzle_samples()));
}

#[allow(dead_code)]
pub fn part2() {
    println!("{}", solve_part2(get_puzzle_samples(), get_puzzle_program()));
}

#[derive(Eq, PartialEq, Debug, Clone)]
struct InstructionSample {
    before: Register,
    instruction: Instruction,
    after: Register,
}

fn solve_part1(input: Vec<InstructionSample>) -> usize {
    input.into_iter()
        .map(|samp| get_matching_opcodes(samp).len())
        .filter(|c| *c >= 3)
        .count()
}

fn solve_part2(samples: Vec<InstructionSample>, program: Program) -> u64 {
    let program = remap_opcodes_from_samples(samples, program);
    VM::load(program).execute().register[0]
}

fn remap_opcodes_from_samples(samples: Vec<InstructionSample>, mut program: Program) -> Program {
    let opcode_mapping = determine_opcode_mapping(samples);
    for instr in program.instructions.iter_mut() {
        instr.opcode = opcode_mapping[&instr.opcode].clone();
    }
    program
}

fn determine_opcode_mapping(samples: Vec<InstructionSample>) -> HashMap<String, String> {
    // Initially, any input opcode could map to any internal opcode
    let mut mapping: HashMap<_, _> = (0..get_opcodes().len())
        .map(|a| (a.to_string(), get_opcodes().into_iter().collect::<HashSet<_>>()))
        .collect();
    
    // Determine the intersection of possible opcodes for all samples
    for sample in samples {
        let input_opcode = sample.instruction.opcode.clone();
        let matches: HashSet<_> = get_matching_opcodes(sample).into_iter().collect();

        let intersected = mapping[&input_opcode]
            .intersection(&matches)
            .map(|c| c.clone())
            .collect();

        mapping.insert(input_opcode, intersected);
    }

    // Now we need to refine the possibilities. Keep looking for the opcode that maps to just one
    // other opcode; this one must be correct. This opcode can then be removed from the other sets.
    loop {
        // First let's check the opcodes that still have multiple candidates to begin with
        let ambiguous_opcodes: Vec<_> = mapping.iter()
            .filter(|tup| tup.1.len() > 1)
            .map(|tup| tup.0.clone())
            .collect();

        if ambiguous_opcodes.is_empty() {
            // We're done
            break;
        }

        // Now find all opcodes that have a one-to-one mapping
        let resolved: HashSet<_> = mapping.values()
            .filter(|c| c.len() == 1)
            .map(|c| c.iter().next().unwrap().clone())
            .collect();

        // And filter them from the ambiguous codes
        for opcode in ambiguous_opcodes {
            let filtered = &mapping[&opcode] - &resolved;
            mapping.insert(opcode, filtered);
        }
    }

    mapping.into_iter()
        .map(|tup| {
            assert_eq!(tup.1.len(), 1);
            (tup.0, tup.1.into_iter().next().unwrap())
        })
        .collect()
}

fn get_matching_opcodes(sample: InstructionSample) -> Vec<String> {
    get_opcodes().into_iter()
        .filter(|opcode| {
            let mut instr = sample.instruction.clone();
            instr.opcode = opcode.clone();
            let res = execute_instruction(&instr, sample.before.clone());
            res == sample.after
        })
        .collect()
}

fn get_puzzle_samples() -> Vec<InstructionSample> {
    parse_samples(load_data("day16_samples"))
}

fn get_puzzle_program() -> Program {
    parse_program(load_data("day16_program"), 4)
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

fn parse_register(reg: String) -> Register {
    let re = Regex::new(r"^[^:]+:\s+\[([\d, ]+)]$$").unwrap();
    let cap = re.captures(&reg).unwrap();
    let reg: Vec<_> = cap[1].split(", ").map(|v| v.parse::<u64>().unwrap()).collect();
    vec![reg[0], reg[1], reg[2], reg[3]]
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
            before: vec![3, 2, 1, 1],
            instruction: Instruction {
                opcode: String::from("9"),
                a: 2,
                b: 1,
                target: 2,
            },
            after: vec![3, 2, 2, 1],
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
