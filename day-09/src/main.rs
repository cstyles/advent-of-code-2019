use std::collections::VecDeque;
use std::env;
use std::fs;
use std::io::{self, Write};
use std::path::Path;

#[derive(PartialEq)]
enum State {
    Running,
    Halted,
    Yielded,
}

#[derive(Copy, Clone, Debug)]
enum ParamMode {
    Position,
    Immediate,
    Relative,
}

impl ParamMode {
    fn from(number: i64) -> Result<ParamMode, ()> {
        match number {
            0 => Ok(ParamMode::Position),
            1 => Ok(ParamMode::Immediate),
            2 => Ok(ParamMode::Relative),
            _ => Err(()),
        }
    }
}

#[derive(Debug)]
enum Command {
    Add,
    Multiply,
    Input,
    Output,
    JumpIfTrue,
    JumpIfFalse,
    LessThan,
    Equals,
    RelativeBaseOffset,
    Halt,
}

impl Command {
    fn from(number: i64) -> Result<Command, ()> {
        match number {
            1 => Ok(Command::Add),
            2 => Ok(Command::Multiply),
            3 => Ok(Command::Input),
            4 => Ok(Command::Output),
            5 => Ok(Command::JumpIfTrue),
            6 => Ok(Command::JumpIfFalse),
            7 => Ok(Command::LessThan),
            8 => Ok(Command::Equals),
            9 => Ok(Command::RelativeBaseOffset),
            99 => Ok(Command::Halt),
            _ => Err(()),
        }
    }
}

struct Instruction {
    command: Command,
    param_modes: Vec<ParamMode>,
}

impl Instruction {
    fn new(command: Command, param_modes: Vec<ParamMode>) -> Self {
        Self {
            command,
            param_modes,
        }
    }
}

struct Processor {
    pc: usize,
    code: Vec<i64>,
    state: State,
    inputs: VecDeque<i64>,
    output: Option<i64>,
    relative_base: i64,
}

impl Processor {
    fn new(code: Vec<i64>) -> Self {
        Self {
            pc: 0,
            code,
            state: State::Halted,
            inputs: VecDeque::new(),
            output: None,
            relative_base: 0,
        }
    }

    fn run_program(&mut self) {
        self.state = State::Running;

        while self.state == State::Running {
            self.execute_instruction(self.fetch_instruction());
        }
    }

    fn fetch_instruction(&self) -> Instruction {
        let instruction = self.code[self.pc];
        let mut param_modes = vec![];

        let opcode = instruction % 100;
        let command = Command::from(opcode).unwrap_or_else(|_err| {
            eprintln!();
            eprintln!("ERROR:");
            eprintln!("Unrecognized command: {}", opcode);
            eprintln!("pc: {}", self.pc);
            panic!();
        });

        let param_mode = instruction / 100 % 10;
        param_modes.push(ParamMode::from(param_mode).unwrap_or_else(|_err| {
            eprintln!();
            eprintln!("ERROR:");
            eprintln!("Unrecognized parameter mode: {}", param_mode);
            eprintln!("pc: {}", self.pc);
            panic!();
        }));

        let param_mode = instruction / 1000 % 10;
        param_modes.push(ParamMode::from(param_mode).unwrap_or_else(|_err| {
            eprintln!();
            eprintln!("ERROR:");
            eprintln!("Unrecognized parameter mode: {}", param_mode);
            eprintln!("pc: {}", self.pc);
            panic!();
        }));

        let param_mode = instruction / 10000 % 10;
        param_modes.push(ParamMode::from(param_mode).unwrap_or_else(|_err| {
            eprintln!();
            eprintln!("ERROR:");
            eprintln!("Unrecognized parameter mode: {}", param_mode);
            eprintln!("pc: {}", self.pc);
            panic!();
        }));

        Instruction::new(command, param_modes)
    }

    fn get_param(&self, offset: usize, param_mode: ParamMode, write: bool) -> i64 {
        let immediate = self.code[self.pc + offset + 1];
        // dbg!(immediate);

        match param_mode {
            ParamMode::Position => {
                let address = immediate;
                if write {
                    address
                } else {
                    self.code[address as usize]
                }
                // dbg!(self.code[address])
                // self.code[address]
            }
            // ParamMode::Immediate => dbg!(immediate),
            ParamMode::Immediate => immediate,
            ParamMode::Relative => {
                let address = immediate + self.relative_base;
                if write {
                    address
                } else {
                    self.code[address as usize]
                }
                // dbg!(address);
                // dbg!(self.code[address]);
            }

        }
    }

    fn execute_instruction(&mut self, instruction: Instruction) {
        match instruction.command {
            Command::Add => {
                let r0 = self.get_param(0, instruction.param_modes[0], false);
                let r1 = self.get_param(1, instruction.param_modes[1], false);
                let address = self.get_param(2, instruction.param_modes[2], true) as usize;

                self.code[address] = r0 + r1;
                // println!("Add: code[{}] = {}", address, r0 + r1);

                self.pc += 4;
            }
            Command::Multiply => {
                let r0 = self.get_param(0, instruction.param_modes[0], false);
                let r1 = self.get_param(1, instruction.param_modes[1], false);
                let address = self.get_param(2, instruction.param_modes[2], true) as usize;

                self.code[address] = r0 * r1;
                // println!("Multiply: code[{}] = {}", address, r0 * r1);

                self.pc += 4;
            }
            Command::Input => {
                let address = self.get_param(0, instruction.param_modes[0], true) as usize;

                // let num = prompt_for_input();
                let num = self.inputs.pop_front().expect("`inputs` is empty");

                self.code[address as usize] = num;
                // println!("Input: code[{}] = {}", address, num);

                self.pc += 2;
            }
            Command::Output => {
                let r0 = self.get_param(0, instruction.param_modes[0], false);

                println!("Output: {}", r0);
                // self.output = Some(r0);

                self.pc += 2;
                // self.state = State::Yielded;
            }
            Command::JumpIfTrue => {
                let r0 = self.get_param(0, instruction.param_modes[0], false);

                if r0 != 0 {
                    let r1 = self.get_param(1, instruction.param_modes[1], false);
                    self.pc = r1 as usize;
                    // println!("JumpIfTrue: pc = {}", self.pc);
                } else {
                    // println!("JumpIfTrue: no jump");
                    self.pc += 3;
                }
            }
            Command::JumpIfFalse => {
                let r0 = self.get_param(0, instruction.param_modes[0], false);

                if r0 == 0 {
                    let r1 = self.get_param(1, instruction.param_modes[1], false);
                    self.pc = r1 as usize;
                    // println!("JumpIfFalse: pc = {}", self.pc);
                } else {
                    // println!("JumpIfFalse: no jump");
                    self.pc += 3;
                }
            }
            Command::LessThan => {
                let r0 = self.get_param(0, instruction.param_modes[0], false);
                let r1 = self.get_param(1, instruction.param_modes[1], false);
                let address = self.get_param(2, instruction.param_modes[2], true) as usize;

                if r0 < r1 {
                    self.code[address] = 1;
                    // println!("LessThan: code[{}] = {}", address, 1);
                } else {
                    self.code[address] = 0;
                    // println!("LessThan: code[{}] = {}", address, 0);
                }

                self.pc += 4;
            }
            Command::Equals => {
                let r0 = self.get_param(0, instruction.param_modes[0], false);
                let r1 = self.get_param(1, instruction.param_modes[1], false);
                let address = self.get_param(2, instruction.param_modes[2], true) as usize;

                if r0 == r1 {
                    self.code[address] = 1;
                    // println!("Equals: code[{}] = {}", address, 1);
                } else {
                    self.code[address] = 0;
                    // println!("Equals: code[{}] = {}", address, 0);
                }

                self.pc += 4;
            }
            Command::RelativeBaseOffset => {
                self.relative_base += self.get_param(0, instruction.param_modes[0], false);
                // println!("relative_base = {}", self.relative_base);
                self.pc += 2;
            }
            Command::Halt => {
                self.state = State::Halted;

                self.pc += 1;
            }
        };
    }
}

fn main() {
    let input = env::args().nth(1).expect("Please provide initial input").parse().unwrap();
    let file_name = env::args().nth(2).expect("Please provide input file");
    let code = load_code(file_name);

    let mut processer = Processor::new(code.clone());
    processer.inputs.push_back(input);
    processer.run_program();
}

#[allow(dead_code)]
fn permutate(numbers: Vec<i64>) -> Vec<Vec<i64>> {
    if numbers.len() < 2 {
        return vec![numbers];
    }

    let mut results = vec![];
    for number in &numbers {
        // Vector containing all numbers except the number we've "fixed"
        let rest = numbers
            .clone()
            .into_iter()
            .filter(|elem| elem != number)
            .collect();

        // Recursively generate permutations for the non-fixed numbers
        let permutations = permutate(rest);

        // Create our final permutations by prepending the fixed number
        for mut permutation in permutations {
            permutation.insert(0, *number);
            results.push(permutation);
        }
    }

    results
}

fn load_code<T>(filename: T) -> Vec<i64>
where
    T: AsRef<Path>,
{
    let mut code: Vec<i64> = fs::read_to_string(filename)
        .expect("Error reading input file")
        .trim()
        .split(',')
        .map(|num| num.parse().unwrap())
        .collect();

    for _ in 0..10_000 {
        code.push(0);
    }

    code
}

#[allow(dead_code)]
fn prompt_for_input() -> i64 {
    print!(">>> ");
    io::stdout().flush().unwrap();

    read_from_stdin()
}

#[allow(dead_code)]
fn read_from_stdin() -> i64 {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
    let trimmed = buffer.trim();

    trimmed.parse().expect("Couldn't parse input as i64")
}
