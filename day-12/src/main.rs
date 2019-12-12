use std::env;
use std::fs;
use std::path::Path;

use regex::Regex;

#[derive(Clone, Copy, Debug, Hash)]
struct Position {
    x: i32,
    y: i32,
    z: i32,
}

impl Default for Position {
    fn default() -> Self {
        Position { x: 0, y: 0, z: 0 }
    }
}

#[derive(Debug)]
struct Moon {
    position: Position,
    velocity: Position,
}

impl Moon {
    fn apply_gravity(&mut self, other1: &Moon, other2: &Moon, other3: &Moon) {
        for other in &[other1, other2, other3] {
            self.velocity.x += velocity_adjust(self.position.x, other.position.x);
            self.velocity.y += velocity_adjust(self.position.y, other.position.y);
            self.velocity.z += velocity_adjust(self.position.z, other.position.z);
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
    let part: i32 = env::args()
        .nth(1)
        .expect("Please provide part (1 or 2)")
        .parse()
        .expect("Part was not a valid number");

    let file_name = env::args().nth(2).expect("Please provide input file");
    let initial_positions = load_input(file_name);

    let io = Moon {
        position: initial_positions[0],
        velocity: Position::default(),
    };
    let europa = Moon {
        position: initial_positions[1],
        velocity: Position::default(),
    };
    let ganymede = Moon {
        position: initial_positions[2],
        velocity: Position::default(),
    };
    let callisto = Moon {
        position: initial_positions[3],
        velocity: Position::default(),
    };

    match part {
        1 => part1(io, europa, ganymede, callisto),
        2 => part2(io, europa, ganymede, callisto),
        _ => panic!("Not a valid part!"),
    };
}

fn part1(mut io: Moon, mut europa: Moon, mut ganymede: Moon, mut callisto: Moon) {
    let iterations: i32 = env::args()
        .nth(3)
        .expect("Please provide number of iterations")
        .parse()
        .expect("Iterations was not a valid number");

    for _ in 0..iterations {
        io.apply_gravity(&europa, &ganymede, &callisto);
        europa.apply_gravity(&io, &ganymede, &callisto);
        ganymede.apply_gravity(&io, &europa, &callisto);
        callisto.apply_gravity(&io, &europa, &ganymede);

        io.apply_velocity();
        europa.apply_velocity();
        ganymede.apply_velocity();
        callisto.apply_velocity();
    }

    let total_energy = io.total_energy()
        + europa.total_energy()
        + ganymede.total_energy()
        + callisto.total_energy();

    println!("Total Energy: {}", total_energy);
}

fn part2(mut io: Moon, mut europa: Moon, mut ganymede: Moon, mut callisto: Moon) {
    let mut cycle_x = 0;
    let mut cycle_y = 0;
    let mut cycle_z = 0;

    let initial_state_x = (
        (io.position.x, io.velocity.x),
        (europa.position.x, europa.velocity.x),
        (ganymede.position.x, ganymede.velocity.x),
        (callisto.position.x, callisto.velocity.x),
    );

    let initial_state_y = (
        (io.position.y, io.velocity.y),
        (europa.position.y, europa.velocity.y),
        (ganymede.position.y, ganymede.velocity.y),
        (callisto.position.y, callisto.velocity.y),
    );

    let initial_state_z = (
        (io.position.z, io.velocity.z),
        (europa.position.z, europa.velocity.z),
        (ganymede.position.z, ganymede.velocity.z),
        (callisto.position.z, callisto.velocity.z),
    );

    let mut step = 1;
    while [cycle_x, cycle_y, cycle_z].iter().any(|&state| state == 0) {
        io.apply_gravity(&europa, &ganymede, &callisto);
        europa.apply_gravity(&io, &ganymede, &callisto);
        ganymede.apply_gravity(&io, &europa, &callisto);
        callisto.apply_gravity(&io, &europa, &ganymede);

        io.apply_velocity();
        europa.apply_velocity();
        ganymede.apply_velocity();
        callisto.apply_velocity();

        let state_x = (
            (io.position.x, io.velocity.x),
            (europa.position.x, europa.velocity.x),
            (ganymede.position.x, ganymede.velocity.x),
            (callisto.position.x, callisto.velocity.x),
        );

        if state_x == initial_state_x {
            cycle_x = step;
        }

        let state_y = (
            (io.position.y, io.velocity.y),
            (europa.position.y, europa.velocity.y),
            (ganymede.position.y, ganymede.velocity.y),
            (callisto.position.y, callisto.velocity.y),
        );

        if state_y == initial_state_y {
            cycle_y = step;
        }

        let state_z = (
            (io.position.z, io.velocity.z),
            (europa.position.z, europa.velocity.z),
            (ganymede.position.z, ganymede.velocity.z),
            (callisto.position.z, callisto.velocity.z),
        );

        if state_z == initial_state_z {
            cycle_z = step;
        }

        step += 1;
    }

    println!("Steps: {}", lcm(lcm(cycle_x, cycle_y), cycle_z));
}

fn load_input<T>(filename: T) -> Vec<Position>
where
    T: AsRef<Path>,
{
    fs::read_to_string(filename)
        .expect("Error reading input file")
        .lines()
        .map(parse_line)
        .collect()
}

fn parse_line(line: &str) -> Position {
    let regex = Regex::new(r"<x=(?P<x>-?\d+), y=(?P<y>-?\d+), z=(?P<z>-?\d+)>").unwrap();
    let captures = regex.captures(line).unwrap();

    Position {
        x: captures["x"].parse().unwrap(),
        y: captures["y"].parse().unwrap(),
        z: captures["z"].parse().unwrap(),
    }
}

fn velocity_adjust(this: i32, other: i32) -> i32 {
    if this < other {
        1
    } else if this > other {
        -1
    } else {
        0
    }
}

fn lcm(a: i64, b: i64) -> i64 {
    a * b / gcd(a, b)
}

fn gcd(a: i64, b: i64) -> i64 {
    match b {
        0 => a,
        _ => gcd(b, a % b),
    }
}
