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
    let filename = env::args().nth(1).expect("Please provide input file");
    let map = load_map(filename);

    let monitor = (22, 25);

    // let mut angle = (-monitor.0, 0);
    let mut angle = (0, -monitor.1);

    // println!("{:?}", angle);
    // println!("{:?}", reduce(angle));

    let mut vaporized: Vec<(i32, i32)> = Vec::new();

    'outer: while monitor.0 + angle.0 < WIDTH {
        let reduced_angle = reduce(angle);
        let mut final_angle = reduced_angle;
        println!("angle:         {:?}", angle);
        println!("reduced_angle: {:?}", reduced_angle);
        println!();

        while final_angle != angle {
            if try_to_vaporize(&map, monitor, final_angle) {
                let ugh = (monitor.0 + final_angle.0, monitor.1 + final_angle.1);
                vaporized.push(ugh);
                angle.0 += 1;
                println!("vaporized {:?}", ugh);
                continue 'outer;
            }

            final_angle.0 += reduced_angle.0;
            final_angle.1 += reduced_angle.1;
        }

        angle.0 += 1;
    }

    // angle.0 -= 1;
    // angle.1 += 1;
    // while monitor.1 + angle.1 < HEIGHT {
    //     let reduced_angle = reduce(angle);
    //     println!("angle:         {:?}", angle);
    //     println!("reduced_angle: {:?}", reduced_angle);
    //     println!();
    //     angle.1 += 1;
    // }

    // angle.1 -= 1;
    // angle.0 -= 1;
    // while monitor.0 + angle.0 > 0 {
    //     let reduced_angle = reduce(angle);
    //     println!("angle:         {:?}", angle);
    //     println!("reduced_angle: {:?}", reduced_angle);
    //     println!();
    //     angle.0 -= 1;
    // }

    // angle.0 += 1;
    // angle.1 -= 1;
    // while monitor.1 + angle.1 > 0 {
    //     let reduced_angle = reduce(angle);
    //     println!("angle:         {:?}", angle);
    //     println!("reduced_angle: {:?}", reduced_angle);
    //     println!();
    //     angle.1 -= 1;
    // }

    // angle.1 += 1;
    // angle.0 += 1;


    // println!("{:#?}", vaporized);


    // loop {
    // }
}

fn try_to_vaporize(map: &Vec<char>, monitor: (i32, i32), angle: (i32, i32)) -> bool {
    println!("{:?}", angle);
    let coords = (monitor.0 + angle.0, monitor.1 + angle.1);
    println!("{:?}", coords);
    get_tile(&map, coords.0, coords.1) == '#'
}

fn reduce(angle: (i32,  i32)) -> (i32, i32) {
    let mut return_angle = (angle.0, angle.1);

    let gcd = gcd(angle.0, angle.1);

    if gcd > 1 {
        return_angle.0 /= gcd;
        return_angle.1 /= gcd;
    }

    return_angle
}

fn part1() {
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
