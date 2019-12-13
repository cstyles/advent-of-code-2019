use std::collections::VecDeque;
use std::convert::{TryFrom, TryInto};

#[derive(Debug, PartialEq)]
pub enum State {
    Running,
    Halted,
    Yielded,
    PromptForInput,
}

#[derive(Copy, Clone, Debug)]
enum ParamMode {
    Position,
    Immediate,
    Relative,
}

impl TryFrom<i64> for ParamMode {
    type Error = &'static str;

    fn try_from(number: i64) -> Result<Self, Self::Error> {
        match number {
            0 => Ok(ParamMode::Position),
            1 => Ok(ParamMode::Immediate),
            2 => Ok(ParamMode::Relative),
            _ => Err("Unrecognized parameter mode"),
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

impl TryFrom<i64> for Command {
    type Error = &'static str;

    fn try_from(number: i64) -> Result<Self, Self::Error> {
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
            _ => Err("Unrecognized command"),
        }
    }
}

#[derive(Debug)]
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

#[derive(Debug)]
pub struct Processor {
    pc: usize,
    code: Vec<i64>,
    pub state: State,
    pub inputs: VecDeque<i64>,
    output: Option<i64>,
    relative_base: i64,
}

impl Processor {
    pub fn new(code: Vec<i64>) -> Self {
        Self {
            pc: 0,
            code,
            state: State::Running,
            inputs: VecDeque::new(),
            output: None,
            relative_base: 0,
        }
    }

    pub fn run_program(&mut self) {
        self.state = State::Running;

        while self.state == State::Running {
            self.execute_instruction(self.fetch_instruction());
        }
    }

    fn fetch_instruction(&self) -> Instruction {
        let instruction = self.code[self.pc];
        let mut param_modes = vec![];

        let opcode = instruction % 100;
        let command = opcode.try_into().unwrap_or_else(|err| {
            eprintln!("ERROR:");
            eprintln!("{}: {}", err, opcode);
            eprintln!("pc: {}", self.pc);
            panic!();
        });

        for param_number in 1..=3 {
            let param_mode = instruction / 10i64.pow(param_number + 1) % 10;
            param_modes.push(param_mode.try_into().unwrap_or_else(|err| {
                eprintln!("ERROR:");
                eprintln!("{}: {}", err, param_mode);
                eprintln!("pc: {}", self.pc);
                panic!();
            }));
        }

        Instruction::new(command, param_modes)
    }

    fn get_param(&self, offset: usize, param_mode: ParamMode) -> i64 {
        let immediate: i64 = self.code[self.pc + offset + 1];

        match param_mode {
            ParamMode::Position => self.code[immediate as usize],
            ParamMode::Immediate => immediate,
            ParamMode::Relative => {
                let address = (immediate + self.relative_base) as usize;
                self.code[address]
            }
        }
    }

    fn get_address(&self, offset: usize, param_mode: ParamMode) -> usize {
        let immediate = self.code[self.pc + offset + 1];

        match param_mode {
            ParamMode::Position => immediate as usize,
            ParamMode::Immediate => panic!("Writable params shoud never be in immediate mode"),
            ParamMode::Relative => (immediate + self.relative_base) as usize,
        }
    }

    fn execute_instruction(&mut self, instruction: Instruction) {
        match instruction.command {
            Command::Add => {
                let r0 = self.get_param(0, instruction.param_modes[0]);
                let r1 = self.get_param(1, instruction.param_modes[1]);
                let address = self.get_address(2, instruction.param_modes[2]);

                self.code[address] = r0 + r1;

                self.pc += 4;
            }
            Command::Multiply => {
                let r0 = self.get_param(0, instruction.param_modes[0]);
                let r1 = self.get_param(1, instruction.param_modes[1]);
                let address = self.get_address(2, instruction.param_modes[2]);

                self.code[address] = r0 * r1;

                self.pc += 4;
            }
            Command::Input => {
                if self.inputs.is_empty() {
                    self.state = State::PromptForInput;
                } else {
                    self.state = State::Running;

                    let address = self.get_address(0, instruction.param_modes[0]);

                    // let num = prompt_for_input();
                    let num = self.inputs.pop_front().expect("`inputs` is empty");

                    self.code[address] = num;

                    self.pc += 2;
                }
            }
            Command::Output => {
                let r0 = self.get_param(0, instruction.param_modes[0]);

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
                let address = self.get_address(2, instruction.param_modes[2]);

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
                let address = self.get_address(2, instruction.param_modes[2]);

                if r0 == r1 {
                    self.code[address] = 1;
                } else {
                    self.code[address] = 0;
                }

                self.pc += 4;
            }
            Command::RelativeBaseOffset => {
                self.relative_base += self.get_param(0, instruction.param_modes[0]);
                self.pc += 2;
            }
            Command::Halt => {
                self.state = State::Halted;

                self.pc += 1;
            }
        };
    }

    pub fn output(&self) -> Option<i64> {
        self.output
    }
}
