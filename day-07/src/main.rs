// use std::collections::{HashMap, HashSet, VecDeque};
use std::fs;
// use std::env::args;
use std::io::{self, Read, Write};

fn main() {
    let permutations = permutate(vec![5, 6, 7, 8, 9]);

    let mut max = 0;
    for permutation in permutations {
        let phase_setting_a = permutation[0];
        let phase_setting_b = permutation[1];
        let phase_setting_c = permutation[2];
        let phase_setting_d = permutation[3];
        let phase_setting_e = permutation[4];

        let code = read_code("input.txt");

        let mut code_a = code.clone();
        let mut code_b = code.clone();
        let mut code_c = code.clone();
        let mut code_d = code.clone();
        let mut code_e = code.clone();

        println!(
            "{} {} {} {} {}",
            phase_setting_a,
            phase_setting_b,
            phase_setting_c,
            phase_setting_d,
            phase_setting_e
        );

        let mut pc_a = 0;
        let mut pc_b = 0;
        let mut pc_c = 0;
        let mut pc_d = 0;
        let mut pc_e = 0;
        let mut ipp_a = 0;
        let mut ipp_b = 0;
        let mut ipp_c = 0;
        let mut ipp_d = 0;
        let mut ipp_e = 0;

        let mut final_result = 0;
        loop {
            println!("running amplifier A: (pc: {}, is: {}, ipp: {})", pc_a, final_result, ipp_a);
            let (ugh_pc_a, ugh_result, ugh_ipp_a) = run_program(&mut code_a, phase_setting_a, final_result, pc_a, ipp_a);
            pc_a = ugh_pc_a;
            ipp_a = ugh_ipp_a;

            println!("running amplifier B: (pc: {}, is: {}, ipp: {})", pc_b, ugh_result, ipp_b);
            let (ugh_pc_b, result, ugh_ipp_b) = run_program(&mut code_b, phase_setting_b, ugh_result, pc_b, ipp_b);
            pc_b = ugh_pc_b;
            ipp_b = ugh_ipp_b;

            println!("running amplifier C: (pc: {}, is: {}, ipp: {})", pc_c, ugh_result, ipp_c);
            let (ugh_pc_c, result, ugh_ipp_c) = run_program(&mut code_c, phase_setting_c, result, pc_c, ipp_c);
            pc_c = ugh_pc_c;
            ipp_c = ugh_ipp_c;

            println!("running amplifier D: (pc: {}, is: {}, ipp: {})", pc_d, ugh_result, ipp_d);
            let (ugh_pc_d, result, ugh_ipp_d) = run_program(&mut code_d, phase_setting_d, result, pc_d, ipp_d);
            pc_d = ugh_pc_d;
            ipp_d = ugh_ipp_d;

            println!("running amplifier E: (pc: {}, is: {}, ipp: {})", pc_e, ugh_result, ipp_e);
            let (ugh_pc_e, result, ugh_ipp_e) = run_program(&mut code_e, phase_setting_e, result, pc_e, ipp_e);
            pc_e = ugh_pc_e;
            ipp_e = ugh_ipp_e;

            final_result = result;

            if code[pc_e] == 99 {
                break
            }

            println!("restarting loop");
        }

        println!("{}", final_result);
        println!();

        if final_result > max {
            max = final_result;
        }
    }

    println!("MAX: {}", max);
}

// fn run_program(code: &mut Vec<i32>, phase_setting: i32, input_signal: i32) -> (usize, i32) {
// fn run_program(code: &mut Vec<i32>, phase_setting: i32, input_signal: i32, pc: usize) -> (usize, i32) {
fn run_program(code: &mut Vec<i32>, phase_setting: i32, input_signal: i32, pc: usize, ipp: i32) -> (usize, i32, i32) {
    let mut pc = pc;
    let mut inputs_processed = ipp;
    // let mut output = 0;
    let mut output = input_signal;

    while code[pc] != 99 {
        let mut instruction = code[pc];
        let opcode = instruction % 100;
        instruction /= 100;
        let param_mode0 = instruction % 10;
        instruction /= 10;
        let param_mode1 = instruction % 10;

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

                println!("1 => addr0: {}, addr1: {}, addr2: {}", addr0, addr1, addr2);

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

                println!("2 => addr0: {}, addr1: {}, addr2: {}", addr0, addr1, addr2);

                let result = r0 * r1;
                code[addr2] = result;
                pc += 4;
            },
            3 => {
                let addr0: usize = code[pc+1] as usize;

                let num = if inputs_processed == 0 {
                    phase_setting
                } else {
                    // input_signal
                    output
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
                println!("pc: {}, output: {}, ipp: {}", pc, output, inputs_processed);
                println!();
                return (pc, code[addr0], inputs_processed)
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

    println!("99!");
    println!("pc: {}, output: {}, ipp: {}", pc, output, inputs_processed);
    println!();

    // (pc, code[0])
    // (pc, output)
    (pc, output, inputs_processed)
}

fn permutate(numbers: Vec<i32>) -> Vec<Vec<i32>> {
    if numbers.len() < 2 {
        return vec![numbers]
    }

    let mut results = vec![];
    for number in &numbers {
        // Vector containing all numbers except the number we've "fixed"
        let rest = numbers.clone().into_iter().filter(|elem| elem != number).collect();

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

fn read_code(filename: &str) -> Vec<i32> {
    fs::read_to_string(filename).expect("Error reading input file")
        .trim()
        .split(',')
        .map(|num| num.parse().unwrap())
        .collect()
}
