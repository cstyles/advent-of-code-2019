use std::collections::VecDeque;
use std::convert::{TryFrom, TryInto};
use std::env;
use std::fs;
use std::io::{self, Write};
use std::path::Path;

#[derive(Debug, PartialEq)]
enum State {
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
struct Processor {
    pc: usize,
    code: Vec<i64>,
    state: State,
    inputs: VecDeque<i64>,
    output: Option<i64>,
    relative_base: i64,
    ball_x: i64,
    ball_y: i64,
    paddle_x: i64,
    paddle_y: i64,
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
            ball_x: 0,
            ball_y: 0,
            paddle_x: 0,
            paddle_y: 0,
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
                if self.inputs.len() == 0 {
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
}

#[derive(Clone, Copy, PartialEq, Debug)]
enum Tile {
    Empty,
    Wall,
    Block,
    Paddle,
    Ball,
}

impl TryFrom<i64> for Tile {
    type Error = &'static str;

    fn try_from(number: i64) -> Result<Self, Self::Error> {
        match number {
            0 => Ok(Tile::Empty),
            1 => Ok(Tile::Wall),
            2 => Ok(Tile::Block),
            3 => Ok(Tile::Paddle),
            4 => Ok(Tile::Ball),
            _ => Err("Unrecognized tile"),
        }
    }
}

fn main() {
    let file_name = env::args().nth(1).expect("Please provide input file");
    let code = load_code(file_name);

    let (max_x, max_y) = part1(code.clone());
    part2(code.clone(), max_x, max_y);
}

fn part1(code: Vec<i64>) -> (i64, i64) {
    let mut processor = Processor::new(code);

    let mut max_x = 0;
    let mut max_y = 0;
    let mut blocks = 0;

    processor.inputs.push_back(0);

    processor.state = State::Running;
    while processor.state != State::Halted {
        processor.run_program();

        if processor.state == State::Halted {
            break;
        }

        let x: i64 = processor.output.expect("No x output");

        processor.run_program();
        let y: i64 = processor.output.expect("No y output");

        processor.run_program();
        let tile_id = processor.output.expect("No tile_id output");

        let tile: Tile = tile_id.try_into().unwrap_or_else(|err| {
            eprintln!("{}: {}", err, tile_id);
            panic!();
        });

        if x > max_x {
            max_x = x;
        }

        if y > max_y {
            max_y = y;
        }

        if tile == Tile::Block {
            blocks += 1;
        }
    }

    println!("Part 1 (# of blocks): {}", blocks);

    (max_x + 1, max_y + 1)
}

fn part2(code: Vec<i64>, max_x: i64, max_y: i64) {
    let mut processor = Processor::new(code);
    processor.code[0] = 2; // Insert two coins to play for free!

    // Create two dimensional array from dimensions that we inferred from part 1
    let mut map: Vec<Vec<Tile>> = Vec::new();
    let mut row: Vec<Tile> = Vec::new();
    row.resize(max_x as usize, Tile::Empty);
    map.resize(max_y as usize, row);

    let mut score = 0;

    processor.state = State::Running;
    while processor.state != State::Halted {
        processor.run_program();
        if processor.state == State::Halted {
            break;
        }

        let x: i64 = processor.output.expect("No color_to_paint output");

        processor.run_program();
        let y: i64 = processor.output.expect("No turn_direction output");

        processor.run_program();
        let tile_id: i64 = processor.output.expect("No turn_direction output");

        if x == -1 && y == 0 {
            score = tile_id;
        } else {
            let tile: Tile = tile_id.try_into().unwrap_or_else(|err| {
                eprintln!("{}: {}", err, tile_id);
                panic!();
            });

            if tile == Tile::Paddle {
                processor.paddle_x = x;
                processor.paddle_y = y;
            }

            if tile == Tile::Ball {
                processor.ball_x = x;
                processor.ball_y = y;
            }

            map[y as usize][x as usize] = tile;
        }
    }

    println!("Part 2 (score): {}", score);
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
