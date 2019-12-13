use std::collections::{HashMap, VecDeque};
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
                let address = self.get_address(0, instruction.param_modes[0]);

                // let num = prompt_for_input();
                let num = self.inputs.pop_front().expect("`inputs` is empty");

                self.code[address] = num;

                self.pc += 2;
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

#[derive(Debug, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

use Direction::*;

impl From<i64> for Direction {
    fn from(num: i64) -> Self {
        match num {
            0 => Left,
            1 => Right,
            _ => panic!(),
        }
    }
}

impl Direction {
    fn turn(&self, dir: Direction) -> Self {
        match self {
            Up => match dir {
                Left => Left,
                Right => Right,
                _ => panic!(),
            },
            Right => match dir {
                Left => Up,
                Right => Down,
                _ => panic!(),
            },
            Down => match dir {
                Left => Right,
                Right => Left,
                _ => panic!(),
            },
            Left => match dir {
                Left => Down,
                Right => Up,
                _ => panic!(),
            },
        }
    }
}

#[derive(Clone, Copy, Debug)]
enum Color {
    Black,
    White,
}

use Color::*;

impl From<i64> for Color {
    fn from(num: i64) -> Self {
        match num {
            0 => Black,
            1 => White,
            _ => panic!(),
        }
    }
}

impl Into<i64> for Color {
    fn into(self) -> i64 {
        match self {
            Black => 0,
            White => 1,
        }
    }
}

impl Into<char> for Color {
    fn into(self) -> char {
        match self {
            Black => ' ',
            White => '█',
        }
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
    let part: i32 = env::args()
        .nth(1)
        .expect("Please provide part (1 or 2)")
        .parse()
        .expect("Part was not a valid number");

    let file_name = env::args().nth(2).expect("Please provide input file");
    let code = load_code(file_name);

    let mut processer = Processor::new(code);

    let initial_input = match part {
        1 => 0,
        2 => 1,
        _ => panic!("Not a valid part!"),
    };

    processer.inputs.push_back(initial_input);

    // let mut x: i32 = 0;
    // let mut y: i32 = 0;
    // let mut facing: Direction = Up;
    // let mut grid: HashMap<(i32, i32), Color> = HashMap::new();
    // let mut total_painted = 0;

    // let mut image: Vec<i64> = Vec::new();

    let mut image: [i64; 798] = [0; 798];
    // for y in 0..21 {
    //     for x in 0..38 {
    //         let offset = (y * 38 + x) as usize;
    //         image[offset] = 0;
    //     }
    // }

    processer.state = State::Running;
    while processer.state != State::Halted {
        processer.run_program();
        if processer.state == State::Halted {
            break;
        }

        let x: i64 = processer.output.expect("No color_to_paint output");

        processer.run_program();
        let y: i64 = processer.output.expect("No turn_direction output");

        processer.run_program();
        // let tile_id: Tile = processer.output.expect("No turn_direction output").try_into().unwrap();
        let tile_id: i64 = processer.output.expect("No turn_direction output");

        let offset = (y * 38 + x) as usize;
        // match image[offset] {
        //     1 => {}, // wall = noop
        //     2 => {}
        // }
        image[offset] = tile_id;

        // println!("{} {}: {}", x, y, tile_id);
    }

    for y in 0..21 {
        for x in 0..38 {
            let offset = (y * 38 + x) as usize;
            // image[offset] = 0;
            print!("{}", image[offset]);
        }
        println!();
    }
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
