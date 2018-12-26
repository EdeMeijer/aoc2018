use std::iter;
use std::iter::FromIterator;

use utils::data::non_empty_lines;

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct Instruction {
    pub opcode: String,
    pub a: u64,
    pub b: u64,
    pub target: usize,
}

pub struct Program {
    pub num_registers: usize,
    pub ip_binding: Option<usize>,
    pub instructions: Vec<Instruction>,
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

pub type Register = Vec<u64>;

pub struct VM<'b> {
    pub program: Program,
    pub register: Register,
    pub ip: usize,
    breakpoints: Vec<Breakpoint<'b>>,
}

pub struct Breakpoint<'b> {
    line: usize,
    callback: Box<FnMut(&State) -> bool + 'b>,
}

pub struct State {
    pub register: Register,
    pub ip: usize,
    pub num_executed_instructions: usize,
}

impl <'b> VM<'b> {
    pub fn load(program: Program) -> VM<'b> {
        let register = Vec::from_iter(iter::repeat(0).take(program.num_registers));
        VM { program, register, ip: 0, breakpoints: vec![] }
    }

    pub fn add_breakpoint(&mut self, line: usize, callback: impl FnMut(&State) -> bool + 'b) {
        self.breakpoints.push(Breakpoint { line, callback: Box::new(callback) });
    }

    pub fn execute(&mut self) -> State {
        let prog = &self.program;

        let mut register = self.register.clone();
        let mut ip = self.ip;

        let mut num_executed_instructions = 0;
        let mut halt = false;

        while ip < prog.instructions.len() {
            if let Some(b) = prog.ip_binding {
                register[b] = ip as u64;
            }

            for bp in self.breakpoints.iter_mut() {
                if bp.line == ip {
                    let c = &mut bp.callback;
                    let state = State { register, ip, num_executed_instructions };
                    halt = c(&state);
                    register = state.register;
                    if halt {
                        break;
                    }
                }
            }
            if halt {
                break;
            }

            register = execute_instruction(&prog.instructions[ip as usize], register);
            if let Some(b) = prog.ip_binding {
                ip = register[b] as usize;
            }
            ip += 1;
            num_executed_instructions += 1;
        }

        State { register, ip, num_executed_instructions }
    }
}

const OPCODES: [&str; 16] = [
    "addr", "addi", "mulr", "muli", "banr", "bani", "borr", "bori",
    "setr", "seti", "gtir", "gtri", "gtrr", "eqir", "eqri", "eqrr",
];

pub fn get_opcodes() -> Vec<String> {
    OPCODES.iter()
        .map(|o| String::from(*o))
        .collect()
}

pub fn execute_instruction(instr: &Instruction, mut reg: Register) -> Register {
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
        "gtir" => (a > reg[br]) as u64,
        "gtri" => (reg[ar] > b) as u64,
        "gtrr" => (reg[ar] > reg[br]) as u64,
        "eqir" => (a == reg[br]) as u64,
        "eqri" => (reg[ar] == b) as u64,
        "eqrr" => (reg[ar] == reg[br]) as u64,
        _ => unreachable!(opcode)
    };

    reg
}

pub fn parse_program(input: String, num_register: usize) -> Program {
    let mut lines = non_empty_lines(input).into_iter().peekable();

    let has_binding = lines.peek().unwrap().chars().next().unwrap() == '#';

    let binding = if has_binding {
        Some(parse_ip_binding(lines.next().unwrap()))
    } else {
        None
    };

    let instructions = lines
        .map(parse_instruction)
        .collect();

    let mut prog = Program::new(instructions, num_register);
    if let Some(b) = binding {
        prog = prog.bind_ip(b);
    }
    prog
}

fn parse_ip_binding(binding: String) -> usize {
    let parts: Vec<_> = binding.split(" ").collect();
    parts[1].parse().unwrap()
}

pub fn parse_instruction(instr: String) -> Instruction {
    let parts: Vec<_> = instr.split(" ").collect();

    let opcode = parts[0].to_owned();
    let a = parts[1].parse::<u64>().unwrap();
    let b = parts[2].parse::<u64>().unwrap();
    let target = parts[3].parse::<usize>().unwrap();

    Instruction { opcode, a, b, target }
}

#[allow(dead_code)]
pub fn print_human_readable(program: Program) {
    // Help out a bit by making the instructions a bit more human readable
    let vars = ["a", "b", "c", "d", "e", "f"];

    let get_ref = |r: usize, ip: usize| {
        if Some(r) == program.ip_binding {
            ip.to_string()
        } else {
            if r < vars.len() { vars[r] } else { "" }.to_owned()
        }
    };

    for (i, instr) in program.instructions.iter().enumerate() {
        let is_goto = Some(instr.target) == program.ip_binding;
        let a = if is_goto {
            "GOTO".to_owned()
        } else {
            format!("{} =", vars[instr.target])
        };

        let au = instr.a as usize;
        let bu = instr.b as usize;
        let a_ref = get_ref(au, i);
        let b_ref = get_ref(bu, i);

        let b = match instr.opcode.as_ref() {
            "seti" => format!("{}", instr.a),
            "setr" => format!("{}", a_ref),
            "bani" => format!("{} & {}", a_ref, instr.b),
            "bori" => format!("{} | {}", a_ref, instr.b),
            "muli" => format!("{} * {}", a_ref, instr.b),
            "addi" => format!("{} + {}", a_ref, instr.b),
            "eqri" => format!("{} == {}", a_ref, instr.b),
            "gtir" => format!("{} > {}", instr.a, b_ref),
            "addr" => format!("{} + {}", a_ref, b_ref),
            "gtrr" => format!("{} > {}", a_ref, b_ref),
            "eqrr" => format!("{} == {}", a_ref, b_ref),
            _ => unimplemented!("{}", instr.opcode)
        };
        let c = if is_goto { " + 1" } else { "" };

        println!("{}:\t{} {}{}", i, a, b, c);
    }
}
