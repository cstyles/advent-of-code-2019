use std::collections::{HashMap, HashSet, VecDeque};
use std::fs;
use std::env::args;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let orbits: Vec<&str> = input.lines().collect();

    let mut bodies: HashSet<&str> = HashSet::new();
    let mut orbiters = HashMap::<&str, Vec<&str>>::new();

    for string in &orbits {
        let parsed = parse_orbit(string);
        let orbited = parsed[0];
        let orbiter = parsed[1];

        bodies.insert(orbited);
        bodies.insert(orbiter);

        let empty = vec![];
        let mut orbiteds_orbiters = match orbiters.get(orbited) {
            Some(something) => something.clone(),
            None            => empty,
        };

        orbiteds_orbiters.push(orbiter);
        orbiters.insert(orbited, orbiteds_orbiters);
    }


    let part = args().nth(1).expect("Please specify a part to run");

    if part == "1" {
        // println!("{:#?}", orbiters);

        let mut queue = VecDeque::<&str>::new();
        queue.push_back("COM");

        let mut orbits = HashMap::<&str, usize>::new();
        orbits.insert("COM", 0);

        while !queue.is_empty() {
            let body = queue.pop_front().unwrap();

            let empty = vec![];
            let body_orbiters = match orbiters.get(body) {
                Some(something) => something.clone(),
                None            => empty,
            };

            // println!("{:#?}", body_orbiters);

            let count = match orbits.get(body) {
                Some(i) => *i,
                None    => 0,
            };

            for body_orbiter in body_orbiters {
                queue.push_back(body_orbiter);
                orbits.insert(body_orbiter, count + 1);
            }
        }

        // println!("{:#?}", orbits);

        let mut total = 0;
        for body in bodies {
            total += orbits.get(body).unwrap_or(&0);
        }

        println!("{}", total);
    } else if part == "2" {
        let path_to_you = find_path(&mut orbiters, "COM", "YOU").unwrap();
        let path_to_san = find_path(&mut orbiters, "COM", "SAN").unwrap();

        println!("{:#?}", path_to_you);
        println!("{:#?}", path_to_san);
    }
}

fn parse_orbit(string: &str) -> Vec<&str> {
    string.split(')').collect()
}

fn find_path<'a>(
    orbiters: &mut HashMap<&'a str, Vec<&'a str>>,
    start: &'a str,
    end: &'a str
) -> Result<Vec<&'a str>, ()> {
    if start == end {
        let end_vec = vec![end];
        Ok(end_vec)
    } else {
        let empty = vec![];
        let body_orbiters = match orbiters.get(start) {
            Some(something) => something.clone(),
            None            => empty,
        };

        for body_orbiter in body_orbiters {
            if let Ok(mut list) = find_path(orbiters, body_orbiter, end) {
                list.push(start);
                return Ok(list);
            }
        }

        Err(())
    }
}
