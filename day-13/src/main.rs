use std::convert::{TryFrom, TryInto};
use std::env;
use std::fmt;
use std::fs;
use std::io::{self, Write};
use std::path::Path;
use std::thread;
use std::time;

use intcode::{Processor, State};

#[derive(Clone, Copy, PartialEq, Debug)]
enum Tile {
    Empty,
    Wall,
    Block,
    Paddle,
    Ball,
}

impl std::fmt::Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Tile::Empty => " ",
                Tile::Wall => "█",
                Tile::Block => "▒",
                Tile::Paddle => "▔",
                Tile::Ball => "O",
            }
        )
    }
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
    let debug = env::args().any(|arg| arg == "--debug");

    let (max_x, max_y) = part1(code.clone());
    part2(code, max_x, max_y, debug);
}

fn part1(code: Vec<i64>) -> (i64, i64) {
    let mut processor = Processor::new(code);

    let mut max_x = 0;
    let mut max_y = 0;
    let mut blocks = 0;

    while processor.state != State::Halted {
        processor.run_program();

        if processor.state == State::Halted {
            break;
        }

        let x: i64 = processor.output().expect("No x output");

        processor.run_program();
        let y: i64 = processor.output().expect("No y output");

        processor.run_program();
        let tile_id = processor.output().expect("No tile_id output");

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

fn part2(mut code: Vec<i64>, max_x: i64, max_y: i64, debug: bool) {
    code[0] = 2; // Insert two coins to play for free!
    let mut processor = Processor::new(code);

    // Create two dimensional array from dimensions that we inferred from part 1
    let mut map: Vec<Vec<Tile>> = Vec::new();
    let mut row: Vec<Tile> = Vec::new();
    row.resize(max_x as usize, Tile::Empty);
    map.resize(max_y as usize, row);

    let mut score = 0;
    let mut paddle_x = 0;
    let mut ball_x = 0;

    processor.state = State::Running;
    while processor.state != State::Halted {
        processor.run_program();
        if processor.state == State::Halted {
            break;
        } else if processor.state == State::PromptForInput {
            let num = if ball_x < paddle_x {
                -1
            } else if ball_x > paddle_x {
                1
            } else {
                0
            };

            processor.inputs.push_back(num);
            processor.run_program();

            if debug {
                thread::sleep(time::Duration::from_millis(100));
                print_map(&map);
                println!();
            }
        }

        let x: i64 = processor.output().expect("No x output");

        processor.run_program();
        let y: i64 = processor.output().expect("No y output");

        processor.run_program();
        let tile_id: i64 = processor.output().expect("No tile_id output");

        if x == -1 && y == 0 {
            score = tile_id;
        } else {
            let tile: Tile = tile_id.try_into().unwrap_or_else(|err| {
                eprintln!("{}: {}", err, tile_id);
                panic!();
            });

            if tile == Tile::Paddle {
                paddle_x = x;
            }

            if tile == Tile::Ball {
                ball_x = x;
            }

            map[y as usize][x as usize] = tile;
        }
    }

    println!("Part 2 (score): {}", score);
}

fn print_map(map: &[Vec<Tile>]) {
    for row in map {
        for tile in row {
            print!("{}", tile);
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
