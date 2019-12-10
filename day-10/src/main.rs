use std::collections::HashMap;
use std::env;
use std::fs;
use std::path::Path;

use num::integer::gcd;

static WIDTH: i32 = 30;
static HEIGHT: i32 = 30;
// static WIDTH: i32 = 20;
// static HEIGHT: i32 = 20;
// static WIDTH: i32 = 10;
// static HEIGHT: i32 = 10;

fn main() {
    part2();
}

fn part2() {
    let visible_asteroids = vec![
        (1, 0), (3, 0), (4, 0), (5, 0), (6, 0), (9, 0), (11, 0), (15, 0), (19, 0), (20, 0), (23,
        0), (25, 0), (28, 0), (3, 1), (5, 1), (9, 1), (15, 1), (21, 1), (6, 2), (14, 2), (15,
        2), (17, 2), (18, 2), (20, 2), (25, 2), (26, 2), (29, 2), (5, 3), (9, 3), (12, 3),
        (13, 3), (19, 3), (21, 3), (24, 3), (25, 3), (26, 3), (28, 3), (2, 4), (5, 4), (9,
        4), (12, 4), (14, 4), (16, 4), (17, 4), (20, 4), (21, 4), (3, 5), (4, 5), (9, 5),
        (11, 5), (16, 5), (19, 5), (20, 5), (23, 5), (25, 5), (29, 5), (1, 6), (2, 6),
        (6, 6), (8, 6), (10, 6), (11, 6), (14, 6), (16, 6), (25, 6), (27, 6), (1, 7), (6,
        7), (9, 7), (15, 7), (17, 7), (18, 7), (20, 7), (24, 7), (26, 7), (27, 7),
        (29, 7), (0, 8), (3, 8), (4, 8), (9, 8), (13, 8), (14, 8), (15, 8), (17, 8),
        (20, 8), (21, 8), (23, 8), (27, 8), (28, 8), (1, 9), (3, 9), (5, 9), (7, 9),
        (11, 9), (21, 9), (25, 9), (27, 9), (29, 9), (1, 10), (4, 10), (6, 10), (8,
        10), (9, 10), (11, 10), (14, 10), (19, 10), (21, 10), (25, 10), (29, 10),
        (2, 11), (6, 11), (9, 11), (11, 11), (12, 11), (13, 11), (14, 11), (16,
        11), (17, 11), (23, 11), (24, 11), (0, 12), (1, 12), (2, 12), (3,
        12), (6, 12), (8, 12), (10, 12), (14, 12), (15, 12), (19, 12), (20,
        12), (25, 12), (28, 12), (29, 12), (1, 13), (7, 13), (9, 13), (11,
        13), (14, 13), (15, 13), (17, 13), (21, 13), (23, 13), (24, 13), (27,
        13), (2, 14), (12, 14), (14, 14), (16, 14), (18, 14), (26, 14), (29,
        14), (3, 15), (6, 15), (16, 15), (26, 15), (29, 15), (0, 16), (1,
        16), (4, 16), (5, 16), (6, 16), (12, 16), (26, 16), (29, 16), (1,
        17), (5, 17), (13, 17), (15, 17), (16, 17), (20, 17), (27, 17), (28,
        17), (2, 18), (5, 18), (6, 18), (9, 18), (10, 18), (11, 18), (15,
        18), (21, 18), (25, 18), (26, 18), (3, 19), (5, 19), (6, 19), (9,
        19), (11, 19), (14, 19), (17, 19), (24, 19), (0, 20), (1, 20), (3,
        20), (5, 20), (6, 20), (8, 20), (9, 20), (11, 20), (13, 20), (18,
        20), (20, 20), (25, 20), (26, 20), (28, 20), (2, 21), (5, 21), (22,
        21), (23, 21), (0, 22), (5, 22), (6, 22), (12, 22), (21, 22), (24,
        22), (25, 22), (27, 22), (1, 23), (2, 23), (3, 23), (5, 23), (6, 23),
        (11, 23), (16, 23), (17, 23), (19, 23), (21, 23), (25, 23), (27, 23),
        (29, 23), (0, 24), (1, 24), (3, 24), (4, 24), (5, 24), (10, 24), (11,
        24), (16, 24), (18, 24), (24, 24), (26, 24), (27, 24), (28, 24),
        (13, 25), (23, 25), (0, 26), (5, 26), (7, 26), (8, 26), (9, 26),
        (11, 26), (12, 26), (14, 26), (18, 26), (20, 26), (22, 26), (24,
        26), (9, 27), (10, 27), (15, 27), (19, 27), (25, 27), (28,
        27), (29, 27), (0, 28), (1, 28), (2, 28), (7, 28), (17, 28),
        (20, 28), (23, 28), (25, 28), (27, 28), (0, 29), (1, 29), (5,
        29), (9, 29), (10, 29), (11, 29), (13, 29), (18, 29),
        (19, 29), (20, 29), (25, 29), (27, 29),
    ];

    let monitor: (f32, f32) = (22.0, 25.0);

    // let mut angles: HashMap<(i32, i32), f32> = HashMap::new();
    let mut angles: Vec<((i32, i32), f32)> = Vec::new();

    for point in &visible_asteroids {
        let f_point = (point.0 as f32, point.1 as f32);
        let delta = (f_point.0 - monitor.0, f_point.1 - monitor.1);
        let angle = delta.1.atan2(delta.0);
        let adjusted_angle = angle + std::f32::consts::PI / 2.0;
        // println!("{:?}", f_point);
        // println!("{:?}", delta);
        // println!("{:?}", angle);
        // println!("{:?}", adjusted_angle);
        // println!();

        // angles.insert(point, adjusted_angle);
        angles.push((*point, adjusted_angle));
    }

    angles.sort_by(|a, b| {
        a.1.partial_cmp(&b.1).unwrap()
    });

    for (i, angle) in angles.iter().enumerate() {
        let point = angle.0;
        let angle = angle.1;
        println!("{}: {:?} :: {:#?}", i, point, angle);
    }

    // let filename = env::args().nth(1).expect("Please provide input file");
    // let map = load_map(filename);
}

fn _part1() {
    let filename = env::args().nth(1).expect("Please provide input file");
    let map = load_map(filename);

    // println!("{:#?}", map);

    let mut asteroids: Vec<(i32, i32)> = Vec::new();

    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            let offset = y * WIDTH + x;
            if map[offset as usize] == '#' {
                asteroids.push((x, y));
            }
        }
    }

    // println!("{:#?}", asteroids);
    // println!("{:#?}", asteroids.len());

    let mut hm: HashMap<(i32, i32), i32> = HashMap::new();

    for current_asteroid in &asteroids {
        let mut visible = 0;

        'other: for other_asteroid in &asteroids {
            if current_asteroid == other_asteroid {
                continue;
            }

            let ca_x = current_asteroid.0;
            let ca_y = current_asteroid.1;

            let oa_x = other_asteroid.0;
            let oa_y = other_asteroid.1;

            // let dx: i32 = ca_x - oa_x;
            let dx: i32 = oa_x - ca_x;
            // let dy: i32 = ca_y - oa_y;
            let dy: i32 = oa_y - ca_y;

            let mut dx_gcd = dx;
            let mut dy_gcd = dy;

            // println!("{:?}", (dx, dy));

            // if dx == 0 {
            //     if dy < 0 {
            //         dy = -1;
            //     } else {
            //         dy = 1;
            //     }
            // }

            // if dy == 0 {
            //     if dx < 0 {
            //         dx = -1;
            //     } else {
            //         dx = 1;
            //     }
            // }

            // println!("{:?}, {:?}", other_asteroid, current_asteroid);
            // println!("{:?}", (dx, dy));

            let gcd = gcd(dx, dy);
            // println!("{}", gcd);

            if gcd > 1 {
                dx_gcd /= gcd;
                dy_gcd /= gcd;
            }

            let save_dx_gcd = dx_gcd;
            let save_dy_gcd = dy_gcd;

            // println!("ca: {:?}", (ca_x, ca_y));
            // println!("oa: {:?}", (oa_x, oa_y));
            // println!("d:  {:?}", (dx, dy));
            // println!("dg: {:?}", (dx_gcd, dy_gcd));

            while dx_gcd != dx || dy_gcd != dy {
                let tile = (ca_x + dx_gcd, ca_y + dy_gcd);
                if get_tile(&map, tile.0, tile.1) == '#' {
                    // println!("blocked!");
                    continue 'other;
                }
                // println!("ug: {:?}", tile);
                // println!("hm: {:?}", (dx_gcd, dy_gcd));
                dx_gcd += save_dx_gcd;
                dy_gcd += save_dy_gcd;
            }

            // println!();

            visible += 1;
            // println!("{:?}", other_asteroid);
        }
        hm.insert(*current_asteroid, visible);
    }

    // println!("{:#?}", hm);

    let mut max = 0;
    for (_asteroid, &count) in &hm {
        if count > max {
            max = count;
        }
    }

    println!("max: {}", max);
}

fn load_map<T>(filename: T) -> Vec<char>
where
    T: AsRef<Path>,
{
    fs::read_to_string(filename)
        .expect("Error reading input file")
        .trim()
        .chars()
        .filter(|&c| c != '\n')
        .collect()
}

fn get_tile(map: &Vec<char>, x: i32, y: i32) -> char {
    let offset = y * WIDTH + x;
    map[offset as usize]
}
