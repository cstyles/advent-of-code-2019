use std::collections::{HashMap, HashSet, VecDeque};
use std::fs;

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

    println!("{:#?}", find_path(&mut orbiters, "COM", "YOU"));
    println!("{:#?}", find_path(&mut orbiters, "COM", "SAN"));

    return;


    // ==== PART 01 ====


    // println!("{:#?}", orbiters);

    // let mut queue = VecDeque::<(&str, usize)>::new();
    let mut queue = VecDeque::<&str>::new();
    queue.push_back("COM");
    // queue.push_back("857");

    let mut orbits = HashMap::<&str, usize>::new();
    orbits.insert("COM", 0);

    // let mut orbits: usize = 0;

    while queue.len() != 0 {
        let body = queue.pop_front().unwrap();

        let empty = vec![];
        let body_orbiters = match orbiters.get(body) {
            Some(something) => something.clone(),
            None            => empty,
        };

        // println!("{:#?}", body_orbiters);
        // orbits += body_orbiters.len() * count;

        let count = match orbits.get(body) {
            Some(i) => *i,
            None    => 0,
        };

        for body_orbiter in body_orbiters {
            queue.push_back(body_orbiter);
            orbits.insert(body_orbiter, count + 1);
        }
    }

    println!("{:#?}", orbits);

    let mut total = 0;
    for body in bodies {
        total += orbits.get(body).unwrap_or(&0);
    }

    println!("{}", total);
}

fn parse_orbit(string: &str) -> Vec<&str> {
    string.split(')').collect()
}

fn find_path<'a>(orbiters: &mut HashMap<&'a str, Vec<&'a str>>, start: &'a str, end: &'a str) -> Result<Vec<&'a str>, ()> {
    if start == end {
        let mut end_vec = vec![end];
        return Ok(end_vec);
    } else {
        let empty = vec![];
        let body_orbiters = match orbiters.get(start) {
            Some(something) => something.clone(),
            None            => empty,
        };

        if body_orbiters.len() == 0 {
        }

        for body_orbiter in body_orbiters {
            match find_path(orbiters, body_orbiter, end) {
                Ok(mut list) => {
                    list.push(start);
                    return Ok(list);
                },
                Err(_) => {},
            }
        }

        return Err(())
    }
}
