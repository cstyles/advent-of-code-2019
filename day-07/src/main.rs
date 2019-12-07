// use std::collections::{HashMap, HashSet, VecDeque};
// use std::fs;
// use std::env::args;
use std::io::{self, Read, Write};

fn main() {
    // let input = fs::read_to_string("input.txt").unwrap();
    // let orbits: Vec<&str> = input.lines().collect();

    let permutations: Vec<(i32, i32, i32, i32, i32)> = vec![
        (0, 1, 2, 3, 4), (0, 1, 2, 4, 3), (0, 1, 3, 2, 4), (0, 1, 3, 4, 2), (0, 1, 4, 2, 3), (0, 1, 4, 3 , 2), (0, 2, 1, 3, 4), (0, 2, 1, 4, 3), (0, 2, 3, 1, 4), (0, 2, 3, 4, 1), (0, 2, 4, 1, 3), (0, 2, 4, 3, 1), (0, 3, 1, 2, 4), (0, 3, 1, 4, 2), (0, 3, 2, 1, 4), (0, 3, 2, 4, 1), (0, 3, 4, 1, 2), ( 0, 3, 4, 2, 1), (0, 4, 1, 2, 3), (0, 4, 1, 3, 2), (0, 4, 2, 1, 3), (0, 4, 2, 3, 1), (0, 4, 3, 1, 2), (0, 4, 3, 2, 1), (1, 0, 2, 3, 4), (1, 0, 2, 4, 3), (1, 0, 3, 2, 4), (1, 0, 3, 4, 2), (1, 0, 4 , 2, 3), (1, 0, 4, 3, 2), (1, 2, 0, 3, 4), (1, 2, 0, 4, 3), (1, 2, 3, 0, 4), (1, 2, 3, 4, 0), (1, 2, 4, 0, 3), (1, 2, 4, 3, 0), (1, 3, 0, 2, 4), (1, 3, 0, 4, 2), (1, 3, 2, 0, 4), (1, 3, 2, 4, 0) , (1, 3, 4, 0, 2), (1, 3, 4, 2, 0), (1, 4, 0, 2, 3), (1, 4, 0, 3, 2), (1, 4, 2, 0, 3), (1, 4, 2, 3, 0), (1, 4, 3, 0, 2), (1, 4, 3, 2, 0), (2, 0, 1, 3, 4), (2, 0, 1, 4, 3), (2, 0, 3, 1, 4), (2, 0 , 3, 4, 1), (2, 0, 4, 1, 3), (2, 0, 4, 3, 1), (2, 1, 0, 3, 4), (2, 1, 0, 4, 3), (2, 1, 3, 0, 4), (2, 1, 3, 4, 0), (2, 1, 4, 0, 3), (2, 1, 4, 3, 0), (2, 3, 0, 1, 4), (2, 3, 0, 4, 1), (2, 3, 1, 0, 4), (2, 3, 1, 4, 0), (2, 3, 4, 0, 1), (2, 3, 4, 1, 0), (2, 4, 0, 1, 3), (2, 4, 0, 3, 1), (2, 4, 1, 0, 3), (2, 4, 1, 3, 0), (2, 4, 3, 0, 1), (2, 4, 3, 1, 0), (3, 0, 1, 2, 4), (3, 0, 1, 4, 2), (3 , 0, 2, 1, 4), (3, 0, 2, 4, 1), (3, 0, 4, 1, 2), (3, 0, 4, 2, 1), (3, 1, 0, 2, 4), (3, 1, 0, 4, 2), (3, 1, 2, 0, 4), (3, 1, 2, 4, 0), (3, 1, 4, 0, 2), (3, 1, 4, 2, 0), (3, 2, 0, 1, 4), (3, 2, 0, 4, 1), (3, 2, 1, 0, 4), (3, 2, 1, 4, 0), (3, 2, 4, 0, 1), (3, 2, 4, 1, 0), (3, 4, 0, 1, 2), (3, 4, 0, 2, 1), (3, 4, 1, 0, 2), (3, 4, 1, 2, 0), (3, 4, 2, 0, 1), (3, 4, 2, 1, 0), (4, 0, 1, 2, 3), (4, 0, 1, 3, 2), (4, 0, 2, 1, 3), (4, 0, 2, 3, 1), (4, 0, 3, 1, 2), (4, 0, 3, 2, 1), (4, 1, 0, 2 , 3), (4, 1, 0, 3, 2), (4, 1, 2, 0, 3), (4, 1, 2, 3, 0), (4, 1, 3, 0, 2), (4, 1, 3, 2, 0), (4, 2, 0, 1, 3), (4, 2, 0, 3, 1), (4, 2, 1, 0, 3), (4, 2, 1, 3, 0), (4, 2, 3, 0, 1), (4, 2, 3, 1, 0), ( 4, 3, 0, 1, 2), (4, 3, 0, 2, 1), (4, 3, 1, 0, 2), (4, 3, 1, 2, 0), (4, 3, 2, 0, 1), (4, 3, 2, 1, 0)
    ];

    let mut max = 0;
    for (a, b, c, d, e) in permutations {
        // code = vec![
        //     3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0
        // ];
        // code = vec![
        //     3,23,3,24,1002,24,10,24,1002,23,-1,23, 101,5,23,23,1,24,23,23,4,23,99,0,0
        // ];
        // code = vec![
        //     3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33, 1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0
        // ];

        let mut code: Vec<i32> = vec![
            3,8,1001,8,10,8,105,1,0,0,21,42,55,64,77,94,175,256,337,418,99999,3,9,102,4,9,9,1001,9,5,9,102,2,9,9,101,3,9,9,4,9,99,3,9,102,2,9,9,101,5,9,9,4,9,99,3,9,1002,9,4,9,4,9,99,3,9,102,4,9,9,101,5,9,9,4,9,99,3,9,102,5,9,9,1001,9,3,9,1002,9,5,9,4,9,99,3,9,1002,9,2,9,4,9,3,9,101,1,9,9,4,9,3,9,1001,9,1,9,4,9,3,9,101,1,9,9,4,9,3,9,1002,9,2,9,4,9,3,9,101,1,9,9,4,9,3,9,101,2,9,9,4,9,3,9,1001,9,2,9,4,9,3,9,101,1,9,9,4,9,3,9,1002,9,2,9,4,9,99,3,9,1002,9,2,9,4,9,3,9,1001,9,2,9,4,9,3,9,1001,9,1,9,4,9,3,9,1002,9,2,9,4,9,3,9,1002,9,2,9,4,9,3,9,1002,9,2,9,4,9,3,9,1002,9,2,9,4,9,3,9,101,2,9,9,4,9,3,9,1001,9,1,9,4,9,3,9,1001,9,1,9,4,9,99,3,9,1002,9,2,9,4,9,3,9,102,2,9,9,4,9,3,9,101,2,9,9,4,9,3,9,101,1,9,9,4,9,3,9,1002,9,2,9,4,9,3,9,102,2,9,9,4,9,3,9,1001,9,2,9,4,9,3,9,101,1,9,9,4,9,3,9,101,1,9,9,4,9,3,9,1002,9,2,9,4,9,99,3,9,101,2,9,9,4,9,3,9,102,2,9,9,4,9,3,9,1001,9,2,9,4,9,3,9,102,2,9,9,4,9,3,9,1001,9,2,9,4,9,3,9,101,2,9,9,4,9,3,9,102,2,9,9,4,9,3,9,1002,9,2,9,4,9,3,9,101,1,9,9,4,9,3,9,1002,9,2,9,4,9,99,3,9,1001,9,2,9,4,9,3,9,1002,9,2,9,4,9,3,9,102,2,9,9,4,9,3,9,102,2,9,9,4,9,3,9,1002,9,2,9,4,9,3,9,101,1,9,9,4,9,3,9,101,1,9,9,4,9,3,9,1002,9,2,9,4,9,3,9,1002,9,2,9,4,9,3,9,1001,9,1,9,4,9,99
        ];

        println!("{} {} {} {} {}", a, b, c, d, e);
        let (_pc, result) = run_program(&mut code, a, 0);
        let (_pc, result) = run_program(&mut code, b, result);
        let (_pc, result) = run_program(&mut code, c, result);
        let (_pc, result) = run_program(&mut code, d, result);
        let (_pc, result) = run_program(&mut code, e, result);

        println!("{}", result);
        println!("");

        if result > max {
            max = result;
        }
    }

    println!("MAX: {}", max);
}

fn run_program(code: &mut Vec<i32>, phase_setting: i32, input_signal: i32) -> (usize, i32) {
    let mut pc = 0;
    let mut inputs_processed = 0;
    let mut output = 0;

    while code[pc] != 99 {
        let mut instruction = code[pc];
        let opcode = instruction % 100;
        instruction /= 100;
        let param_mode0 = instruction % 10;
        instruction /= 10;
        let param_mode1 = instruction % 10;
        instruction /= 10;
        let param_mode2 = instruction % 10;

        // println!("pc: {}, opcode: {}, param_mode0: {}, param_mode1: {}, param_mode2: {}",
        //     pc, opcode, param_mode0, param_mode1, param_mode2);

        match opcode {
            1 => {
                let addr0: usize = code[pc+1] as usize;
                let addr1: usize = code[pc+2] as usize;
                let addr2: usize = code[pc+3] as usize;

                let r0 = match param_mode0 {
                    1     => code[pc+1],
                    0 | _ => code[addr0],
                };

                let r1 = match param_mode1 {
                    1     => code[pc+2],
                    0 | _ => code[addr1],
                };

                // println!("1 => addr0: {}, addr1: {}, addr2: {}", addr0, addr1, addr2);

                let result = r0 + r1;
                code[addr2] = result;
                pc += 4;
            },
            2 => {
                let addr0: usize = code[pc+1] as usize;
                let addr1: usize = code[pc+2] as usize;
                let addr2: usize = code[pc+3] as usize;

                let r0 = match param_mode0 {
                    1     => code[pc+1],
                    0 | _ => code[addr0],
                };

                let r1 = match param_mode1 {
                    1     => code[pc+2],
                    0 | _ => code[addr1],
                };

                // println!("2 => addr0: {}, addr1: {}, addr2: {}", addr0, addr1, addr2);

                let result = r0 * r1;
                code[addr2] = result;
                pc += 4;
            },
            3 => {
                let addr0: usize = code[pc+1] as usize;

                let num = if inputs_processed == 0 {
                    phase_setting
                } else {
                    input_signal
                };

                inputs_processed += 1;

                println!("3 => {}", num);
                // io::stdout().flush().unwrap();
                // let mut buffer = String::new();
                // io::stdin().read_line(&mut buffer).unwrap();
                // let trimmed = buffer.trim();
                // let num: i32 = trimmed.parse().unwrap();

                println!("num: {}, addr0: {}", num, addr0);
                code[addr0] = num;

                pc += 2;
            },
            4 => {
                let addr0: usize = code[pc+1] as usize;
                let r0 = code[addr0];

                println!("4 => {}", r0);
                output = r0;

                pc += 2;

                // Exit early with output
                // return (pc, code[0])
                // return (pc, code[addr0])
            },
            5 => {
                let addr0: usize = code[pc+1] as usize;
                let addr1: usize = code[pc+2] as usize;

                let r0 = match param_mode0 {
                    1     => code[pc+1],
                    0 | _ => code[addr0],
                };

                let r1 = match param_mode1 {
                    1     => code[pc+2],
                    0 | _ => code[addr1],
                };

                if r0 != 0 {
                    pc = r1 as usize;
                } else {
                    pc += 3;
                }
            },
            6 => {
                let addr0: usize = code[pc+1] as usize;
                let addr1: usize = code[pc+2] as usize;

                let r0 = match param_mode0 {
                    1     => code[pc+1],
                    0 | _ => code[addr0],
                };

                let r1 = match param_mode1 {
                    1     => code[pc+2],
                    0 | _ => code[addr1],
                };

                if r0 == 0 {
                    pc = r1 as usize;
                } else {
                    pc += 3;
                }
            },
            7 => {
                let addr0: usize = code[pc+1] as usize;
                let addr1: usize = code[pc+2] as usize;
                let addr2: usize = code[pc+3] as usize;

                let r0 = match param_mode0 {
                    1     => code[pc+1],
                    0 | _ => code[addr0],
                };

                let r1 = match param_mode1 {
                    1     => code[pc+2],
                    0 | _ => code[addr1],
                };

                if r0 < r1 {
                    code[addr2] = 1;
                } else {
                    code[addr2] = 0;
                }

                pc += 4;
            },
            8 => {
                let addr0: usize = code[pc+1] as usize;
                let addr1: usize = code[pc+2] as usize;
                let addr2: usize = code[pc+3] as usize;

                let r0 = match param_mode0 {
                    1     => code[pc+1],
                    0 | _ => code[addr0],
                };

                let r1 = match param_mode1 {
                    1     => code[pc+2],
                    0 | _ => code[addr1],
                };

                if r0 == r1 {
                    code[addr2] = 1;
                } else {
                    code[addr2] = 0;
                }

                pc += 4;
            },
            // 99 => {
            //     return (pc, code[0])
            // },
            _ => {
                unreachable!();
            },
        };
    }

    // (pc, code[0])
    (pc, output)
}
