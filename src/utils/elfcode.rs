use std::iter;
use std::iter::FromIterator;
use utils::data::non_empty_lines;

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct Instruction {
    opcode: String,
    a: u32,
    b: u32,
    target: usize,
}

pub struct Program {
    num_registers: usize,
    ip_binding: Option<usize>,
    instructions: Vec<Instruction>,
}

impl Program {
    pub fn new(instructions: Vec<Instruction>, num_registers: usize) -> Self {
        Program { instructions, num_registers, ip_binding: None }
    }
    
    pub fn bind_ip(mut self, register: usize) -> Self {
        self.ip_binding = Some(register);
        self
    }
}

type Register = Vec<u32>;

pub struct VM {
    pub program: Program,
    pub register: Register,
    pub ip: usize,
}

pub struct Result {
    pub register: Register,
    pub ip: usize,
}

impl VM {
    pub fn load(program: Program) -> VM {
        let register = Vec::from_iter(iter::repeat(0).take(program.num_registers));
        VM { program, register, ip: 0 }
    }

    pub fn execute(self) -> Result {
        let VM { program, mut register, mut ip } = self;
        let Program { ip_binding, instructions, num_registers: _ } = program;

        while ip < instructions.len() {
            if let Some(b) = ip_binding {
                register[b] = ip as u32;
            }
            register = execute_instruction(&instructions[ip as usize], register);
            if let Some(b) = ip_binding {
                ip = register[b] as usize;
            }
            ip += 1;
        }

        Result { register, ip }
    }
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
        _ => unreachable!(opcode)
    };

    reg
}

pub fn parse_program(input: String) -> Program {
    let mut lines = non_empty_lines(input).into_iter();
    let first = lines.next().unwrap();

    let instructions = lines
        .map(parse_instruction)
        .collect();

    Program::new(instructions, 6).bind_ip(parse_ip_binding(first))
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
