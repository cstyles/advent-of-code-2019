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

#[derive(Copy, Clone)]
enum ParamMode {
    Position,
    Immediate,
}

impl ParamMode {
    fn from(number: i32) -> Result<ParamMode, ()> {
        match number {
            0 => Ok(ParamMode::Position),
            1 => Ok(ParamMode::Immediate),
            _ => Err(()),
        }
    }
}

enum Command {
    Add,
    Multiply,
    Input,
    Output,
    JumpIfTrue,
    JumpIfFalse,
    LessThan,
    Equals,
    Halt,
}

impl Command {
    fn from(number: i32) -> Result<Command, ()> {
        match number {
            1 => Ok(Command::Add),
            2 => Ok(Command::Multiply),
            3 => Ok(Command::Input),
            4 => Ok(Command::Output),
            5 => Ok(Command::JumpIfTrue),
            6 => Ok(Command::JumpIfFalse),
            7 => Ok(Command::LessThan),
            8 => Ok(Command::Equals),
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
    code: Vec<i32>,
    state: State,
    inputs: VecDeque<i32>,
    output: Option<i32>,
}

impl Processor {
    fn new(code: Vec<i32>) -> Self {
        Self {
            pc: 0,
            code,
            state: State::Halted,
            inputs: VecDeque::new(),
            output: None,
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

        Instruction::new(command, param_modes)
    }

    fn get_param(&self, offset: usize, param_mode: ParamMode) -> i32 {
        match param_mode {
            ParamMode::Position => {
                let address = self.code[self.pc + offset + 1] as usize;
                self.code[address]
            }
            ParamMode::Immediate => self.code[self.pc + offset + 1],
        }
    }

    fn execute_instruction(&mut self, instruction: Instruction) {
        match instruction.command {
            Command::Add => {
                let r0 = self.get_param(0, instruction.param_modes[0]);
                let r1 = self.get_param(1, instruction.param_modes[1]);

                let address = self.code[self.pc + 3] as usize;
                self.code[address] = r0 + r1;

                self.pc += 4;
            }
            Command::Multiply => {
                let r0 = self.get_param(0, instruction.param_modes[0]);
                let r1 = self.get_param(1, instruction.param_modes[1]);

                let address = self.code[self.pc + 3] as usize;
                self.code[address] = r0 * r1;

                self.pc += 4;
            }
            Command::Input => {
                let address = self.code[self.pc + 1] as usize;

                // let num = prompt_for_input();
                let num = self.inputs.pop_front().expect("`inputs` is empty");

                self.code[address] = num;

                self.pc += 2;
            }
            Command::Output => {
                let address = self.code[self.pc + 1] as usize;
                let r0 = self.code[address];

                // println!("{}", r0);
                self.output = Some(r0);

                self.pc += 2;
                self.state = State::Yielded;
            }
            Command::JumpIfTrue => {
                let r0 = self.get_param(0, instruction.param_modes[0]);

                if r0 != 0 {
                    let r1 = self.get_param(1, instruction.param_modes[1]);
                    self.pc = r1 as usize;
                } else {
                    self.pc += 3;
                }
            }
            Command::JumpIfFalse => {
                let r0 = self.get_param(0, instruction.param_modes[0]);

                if r0 == 0 {
                    let r1 = self.get_param(1, instruction.param_modes[1]);
                    self.pc = r1 as usize;
                } else {
                    self.pc += 3;
                }
            }
            Command::LessThan => {
                let r0 = self.get_param(0, instruction.param_modes[0]);
                let r1 = self.get_param(1, instruction.param_modes[1]);
                let address = self.code[self.pc + 3] as usize;

                if r0 < r1 {
                    self.code[address] = 1;
                } else {
                    self.code[address] = 0;
                }

                self.pc += 4;
            }
            Command::Equals => {
                let r0 = self.get_param(0, instruction.param_modes[0]);
                let r1 = self.get_param(1, instruction.param_modes[1]);
                let address = self.code[self.pc + 3] as usize;

                if r0 == r1 {
                    self.code[address] = 1;
                } else {
                    self.code[address] = 0;
                }

                self.pc += 4;
            }
            Command::Halt => {
                self.state = State::Halted;

                self.pc += 1;
            }
        };
    }
}

fn main() {
    let file_name = env::args().nth(1).expect("Please provide input file");
    let code = load_code(file_name);

    let mut max = 0;
    for phase_settings in permutate(vec![5, 6, 7, 8, 9]) {
        let mut amplifier_a = Processor::new(code.clone());
        let mut amplifier_b = Processor::new(code.clone());
        let mut amplifier_c = Processor::new(code.clone());
        let mut amplifier_d = Processor::new(code.clone());
        let mut amplifier_e = Processor::new(code.clone());

        amplifier_a.inputs.push_back(phase_settings[0]);
        amplifier_b.inputs.push_back(phase_settings[1]);
        amplifier_c.inputs.push_back(phase_settings[2]);
        amplifier_d.inputs.push_back(phase_settings[3]);
        amplifier_e.inputs.push_back(phase_settings[4]);

        amplifier_a.inputs.push_back(0);

        loop {
            amplifier_a.run_program();

            amplifier_b.inputs.push_back(amplifier_a.output.unwrap());
            amplifier_b.run_program();

            amplifier_c.inputs.push_back(amplifier_b.output.unwrap());
            amplifier_c.run_program();

            amplifier_d.inputs.push_back(amplifier_c.output.unwrap());
            amplifier_d.run_program();

            amplifier_e.inputs.push_back(amplifier_d.output.unwrap());
            amplifier_e.run_program();

            amplifier_a.inputs.push_back(amplifier_e.output.unwrap());

            if amplifier_e.state == State::Halted {
                break;
            }
        }

        let final_result = amplifier_e.output.unwrap();
        if final_result > max {
            max = final_result;
        }
    }

    println!("max: {}", max);
}

fn permutate(numbers: Vec<i32>) -> Vec<Vec<i32>> {
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

fn load_code<T>(filename: T) -> Vec<i32>
where
    T: AsRef<Path>,
{
    fs::read_to_string(filename)
        .expect("Error reading input file")
        .trim()
        .split(',')
        .map(|num| num.parse().unwrap())
        .collect()
}

#[allow(dead_code)]
fn prompt_for_input() -> i32 {
    print!(">>> ");
    io::stdout().flush().unwrap();

    read_from_stdin()
}

#[allow(dead_code)]
fn read_from_stdin() -> i32 {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
    let trimmed = buffer.trim();

    trimmed.parse().expect("Couldn't parse input as i32")
}
