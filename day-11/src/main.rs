use std::collections::{HashMap, VecDeque};
use std::env;
use std::fs;
use std::io::{self, Write};
use std::path::Path;

#[derive(Debug, PartialEq)]
#[allow(dead_code)]
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
        let command = Command::from(opcode).unwrap_or_else(|_err| {
            eprintln!();
            eprintln!("ERROR:");
            eprintln!("Unrecognized command: {}", opcode);
            eprintln!("pc: {}", self.pc);
            panic!();
        });

        for param_number in 1..=3 {
            let param_mode = instruction / 10i64.pow(param_number + 1) % 10;
            param_modes.push(ParamMode::from(param_mode).unwrap_or_else(|_err| {
                eprintln!();
                eprintln!("ERROR:");
                eprintln!("Unrecognized parameter mode: {}", param_mode);
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

impl Direction {
    // fn from(num: i32) -> Self {
    //     match num {
    //         0 => Direction::Up,
    //         1 => Direction::Right
    //         2 => Direction::Down,
    //         3 => Direction::Left,
    //     }
    // }

    // fn to_i

    fn turn(&self, dir: Direction) -> Self {
        match self {
            Up => {
                if dir == Left {
                    Left
                } else {
                    Right
                }
            }
            Right => {
                if dir == Left {
                    Up
                } else {
                    Down
                }
            }
            Down => {
                if dir == Left {
                    Right
                } else {
                    Left
                }
            }
            Left => {
                if dir == Left {
                    Down
                } else {
                    Up
                }
            }
        }
    }
}

#[derive(Debug, PartialEq)]
enum Color {
    Black,
    White,
}

impl Color {
    fn to_num(&self) -> i64 {
        match self {
            Color::Black => 0,
            Color::White => 1,
        }
    }
}

fn main() {
    let file_name = env::args().nth(1).expect("Please provide input file");
    let code = load_code(file_name);

    let mut processer = Processor::new(code.clone());
    processer.inputs.push_back(0);

    let mut x: i32 = 0;
    let mut y: i32 = 0;
    let mut facing: Direction = Direction::Up;
    let mut grid: HashMap<(i32, i32), Color> = HashMap::new();
    let mut total_painted = 0;
    // grid.insert((x, y), Color::Black);

    processer.state = State::Running;
    while processer.state != State::Halted {
        processer.run_program();
        if processer.state == State::Halted {
            break
        }

        let paint_color = processer.output.unwrap();
        processer.run_program();
        let turn_dir = processer.output.unwrap();
        println!("Outputs: {}, {}", paint_color, turn_dir);

        let color_to_paint = match paint_color {
            0 => Color::Black,
            1 => Color::White,
            _ => unreachable!(),
        };

        // let current_color = grid.get(&(x, y)).unwrap_or(&Color::Black);
        // if current_color != color_to_paint {
        if !grid.contains_key(&(x, y)) {
            // println!("{:?} has not been painted yet", (x, y));
            total_painted += 1;
        } else {
            // println!("{:?} already painted", (x, y));
        }

        println!("Painting {:?} {:?}", (x, y), color_to_paint);
        grid.insert((x, y), color_to_paint);

        let turn_direction = match turn_dir {
            0 => Direction::Left, // left 90 deg
            1 => Direction::Right, // right 90 deg
            _ => {
                println!("{:?}", turn_dir);
                unreachable!();
            },
        };

        println!("Turning {:?}", turn_direction);

        facing = facing.turn(turn_direction);

        println!("Now facing {:?}", facing);

        match facing {
            Up => {
                y -= 1;
            }
            Left => {
                x -= 1;
            }
            Down => {
                y += 1;
            }
            Right => {
                x += 1;
            }
        }

        println!("Now at {:?}", (x, y));

        let current_color = grid.get(&(x, y)).unwrap_or(&Color::Black);
        processer.inputs.push_back(current_color.to_num());

        println!();
    }

    println!("total_painted: {}", total_painted);
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
