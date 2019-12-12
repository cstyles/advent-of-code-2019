use std::collections::{HashMap, HashSet};
use std::env;
use std::fs;
use std::path::Path;

#[derive(PartialEq)]
enum Tile {
    Empty,
    Asteroid,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct Position {
    x: i32,
    y: i32,
}

impl Default for Position {
    fn default() -> Self {
        Position { x: 0, y: 0 }
    }
}

fn main() {
    let filename = env::args().nth(1).expect("Please provide input file");
    let map = load_map(filename);

    let mut asteroids: Vec<Position> = Vec::new();

    for (y, row) in map.iter().enumerate() {
        for (x, tile) in row.iter().enumerate() {
            if *tile == Tile::Asteroid {
                let position = Position {
                    x: x as i32,
                    y: y as i32,
                };
                asteroids.push(position);
            }
        }
    }

    let mut max_angles = 0;
    let mut monitoring_station = Position::default();

    for current_asteroid in &asteroids {
        let mut angles: HashSet<i64> = HashSet::with_capacity(asteroids.len());

        for other_asteroid in &asteroids {
            if current_asteroid == other_asteroid {
                continue;
            }

            let dx = f64::from(other_asteroid.x - current_asteroid.x);
            let dy = f64::from(other_asteroid.y - current_asteroid.y);
            let angle = dx.atan2(dy);
            angles.insert(float_to_int(angle));
        }

        if angles.len() > max_angles {
            max_angles = angles.len();
            monitoring_station = *current_asteroid;
        }
    }

    println!("Part 1 (Max Visible): {}", max_angles);

    let visible_asteroids = get_visible_asteroids(&asteroids, monitoring_station);
    let mut angles: Vec<&i64> = visible_asteroids.keys().collect();

    angles.sort();

    let up_angle = float_to_int(-std::f64::consts::PI / 2.0);
    let last = angles
        .iter()
        .cycle()
        .skip_while(|&angle| **angle != up_angle)
        .take(200)
        .last()
        .unwrap();

    let final_asteroid = visible_asteroids.get(last).unwrap();

    println!(
        "Part 2 (200th asteroid): {}",
        final_asteroid.x * 100 + final_asteroid.y
    );
}

fn load_map<T>(filename: T) -> Vec<Vec<Tile>>
where
    T: AsRef<Path>,
{
    let input = fs::read_to_string(filename).expect("Error reading input file");

    let mut map = Vec::new();
    map.push(Vec::new());

    for character in input.trim().chars() {
        match character {
            '\n' => map.push(Vec::new()),
            '.' => map.last_mut().unwrap().push(Tile::Empty),
            '#' => map.last_mut().unwrap().push(Tile::Asteroid),
            _ => panic!("Invalid character"),
        }
    }

    map
}

// f64 doesn't implement Eq so we need to convert to i64
// This shoud be fine for this simple use case
fn float_to_int(float: f64) -> i64 {
    (float * 1_000_000_000.0).round() as i64
}

fn get_visible_asteroids(
    asteroids: &[Position],
    monitoring_station: Position,
) -> HashMap<i64, Position> {
    let mut angles: HashMap<i64, Position> = HashMap::with_capacity(asteroids.len());

    for other_asteroid in asteroids {
        if monitoring_station == *other_asteroid {
            continue;
        }

        let dx = f64::from(other_asteroid.x - monitoring_station.x);
        let dy = f64::from(other_asteroid.y - monitoring_station.y);
        let angle = dy.atan2(dx);
        let angle_as_int = float_to_int(angle);

        match angles.get(&angle_as_int) {
            None => {
                angles.insert(angle_as_int, *other_asteroid);
            }
            Some(existing_asteroid) => {
                let dx_other = other_asteroid.x - monitoring_station.x;
                let dx_existing = existing_asteroid.x - monitoring_station.x;
                if dx_other < dx_existing {
                    angles.insert(angle_as_int, *other_asteroid);
                }
            }
        }
    }

    angles
}
