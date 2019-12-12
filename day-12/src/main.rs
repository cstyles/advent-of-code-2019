// use std::collections::{HashMap, VecDeque};
// use std::convert::{TryFrom, TryInto};
use std::env;
use std::fs;
// use std::io::{self, Write};
use std::path::Path;

#[derive(Debug)]
struct Position {
    x: i32,
    y: i32,
    z: i32,
}

impl Default for Position {
    fn default() -> Self {
        Position {
            x: 0,
            y: 0,
            z: 0,
        }
    }
}

#[derive(Debug)]
struct Moon {
    position: Position,
    velocity: Position,
}

impl Moon {
    fn apply_gravity(&mut self, other1: &Moon, other2: &Moon, other3: &Moon) {
        for other in vec![other1, other2, other3] {
            self.velocity.x += if self.position.x < other.position.x {
                1
            } else if self.position.x > other.position.x {
                -1
            } else {
                0
            };

            self.velocity.y += if self.position.y < other.position.y {
                1
            } else if self.position.y > other.position.y {
                -1
            } else {
                0
            };

            self.velocity.z += if self.position.z < other.position.z {
                1
            } else if self.position.z > other.position.z {
                -1
            } else {
                0
            };
        }
    }

    fn apply_velocity(&mut self) {
        self.position.x += self.velocity.x;
        self.position.y += self.velocity.y;
        self.position.z += self.velocity.z;
    }

    fn potential_energy(&self) -> i32 {
        self.position.x.abs() + self.position.y.abs() + self.position.z.abs()
    }

    fn kinetic_energy(&self) -> i32 {
        self.velocity.x.abs() + self.velocity.y.abs() + self.velocity.z.abs()
    }

    fn total_energy(&self) -> i32 {
        self.potential_energy() * self.kinetic_energy()
    }
}

fn main() {
    // let file_name = env::args().nth(1).expect("Please provide input file");
    // let input = load_input(file_name);

    // Debug1
    let mut io = Moon {
        position: Position { x: -1, y: 0, z: 2 },
        velocity: Position::default(),
    };

    let mut europa  = Moon {
        position: Position { x: 2, y: -10, z: -7 },
        velocity: Position::default(),
    };

    let mut ganymede = Moon {
        position: Position { x: 4, y: -8, z: 8 },
        velocity: Position::default(),
    };

    let mut callisto = Moon {
        position: Position { x: 3, y: 5, z: -1 },
        velocity: Position::default(),
    };

    // Debug 2
    let mut io = Moon {
        position: Position { x: -8, y: -10, z: 0 },
        velocity: Position::default(),
    };

    let mut europa  = Moon {
        position: Position { x: 5, y: 5, z: 10 },
        velocity: Position::default(),
    };

    let mut ganymede = Moon {
        position: Position { x: 2, y: -7, z: 3 },
        velocity: Position::default(),
    };

    let mut callisto = Moon {
        position: Position { x: 9, y: -8, z: -3 },
        velocity: Position::default(),
    };

    // My input
    let mut io = Moon {
        position: Position { x: 3, y: 15, z: 8 },
        velocity: Position::default(),
    };

    let mut europa  = Moon {
        position: Position { x: 5, y: -1, z: -2 },
        velocity: Position::default(),
    };

    let mut ganymede = Moon {
        position: Position { x: -10, y: 8, z: 2 },
        velocity: Position::default(),
    };

    let mut callisto = Moon {
        position: Position { x: 8, y: 4, z: -5 },
        velocity: Position::default(),
    };

    println!("{:?}", io);
    println!("{:?}", europa);
    println!("{:?}", ganymede);
    println!("{:?}", callisto);
    println!();

    for step in 0..1000 {
        io.apply_gravity(&europa, &ganymede, &callisto);
        europa.apply_gravity(&io, &ganymede, &callisto);
        ganymede.apply_gravity(&io, &europa, &callisto);
        callisto.apply_gravity(&io, &europa, &ganymede);

        io.apply_velocity();
        europa.apply_velocity();
        ganymede.apply_velocity();
        callisto.apply_velocity();

        if step % 10 != 99 {
            println!("{}", step);
            println!("{:?}", io);
            println!("{:?}", europa);
            println!("{:?}", ganymede);
            println!("{:?}", callisto);
            println!();
        }
    }

    println!("{:?}", io);
    println!("{:?}", europa);
    println!("{:?}", ganymede);
    println!("{:?}", callisto);
    println!();

    println!("pot: {}, kin: {}, total: {}", io.potential_energy(), io.kinetic_energy(), io.total_energy());
    println!("pot: {}, kin: {}, total: {}", europa.potential_energy(), europa.kinetic_energy(), europa.total_energy());
    println!("pot: {}, kin: {}, total: {}", ganymede.potential_energy(), ganymede.kinetic_energy(), ganymede.total_energy());
    println!("pot: {}, kin: {}, total: {}", callisto.potential_energy(), callisto.kinetic_energy(), callisto.total_energy());

    println!();

    println!("total total: {}", io.total_energy() + europa.total_energy() + ganymede.total_energy() + callisto.total_energy());
}

fn load_input<T>(filename: T) -> Vec<i64>
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
