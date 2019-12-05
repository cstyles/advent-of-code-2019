use std::io::{self, Read, Write};

fn main() {
    let mut code: Vec<i32> = vec![
        3,225,1,225,6,6,1100,1,238,225,104,0,1101,82,10,225,101,94,44,224,101,-165,224,224,4,224,1002,223,8,223,101,3,224,224,1,224,223,223,1102,35,77,225,1102,28,71,225,1102,16,36,225,102,51,196,224,101,-3468,224,224,4,224,102,8,223,223,1001,224,7,224,1,223,224,223,1001,48,21,224,101,-57,224,224,4,224,1002,223,8,223,101,6,224,224,1,223,224,223,2,188,40,224,1001,224,-5390,224,4,224,1002,223,8,223,101,2,224,224,1,224,223,223,1101,9,32,224,101,-41,224,224,4,224,1002,223,8,223,1001,224,2,224,1,223,224,223,1102,66,70,225,1002,191,28,224,101,-868,224,224,4,224,102,8,223,223,101,5,224,224,1,224,223,223,1,14,140,224,101,-80,224,224,4,224,1002,223,8,223,101,2,224,224,1,224,223,223,1102,79,70,225,1101,31,65,225,1101,11,68,225,1102,20,32,224,101,-640,224,224,4,224,1002,223,8,223,1001,224,5,224,1,224,223,223,4,223,99,0,0,0,677,0,0,0,0,0,0,0,0,0,0,0,1105,0,99999,1105,227,247,1105,1,99999,1005,227,99999,1005,0,256,1105,1,99999,1106,227,99999,1106,0,265,1105,1,99999,1006,0,99999,1006,227,274,1105,1,99999,1105,1,280,1105,1,99999,1,225,225,225,1101,294,0,0,105,1,0,1105,1,99999,1106,0,300,1105,1,99999,1,225,225,225,1101,314,0,0,106,0,0,1105,1,99999,8,226,226,224,1002,223,2,223,1006,224,329,101,1,223,223,1008,677,677,224,102,2,223,223,1006,224,344,101,1,223,223,1107,226,677,224,102,2,223,223,1005,224,359,101,1,223,223,1008,226,226,224,1002,223,2,223,1006,224,374,1001,223,1,223,1108,677,226,224,1002,223,2,223,1006,224,389,1001,223,1,223,7,677,226,224,1002,223,2,223,1006,224,404,101,1,223,223,7,226,226,224,1002,223,2,223,1005,224,419,101,1,223,223,8,226,677,224,1002,223,2,223,1006,224,434,1001,223,1,223,7,226,677,224,1002,223,2,223,1006,224,449,1001,223,1,223,107,226,677,224,1002,223,2,223,1005,224,464,1001,223,1,223,1007,677,677,224,102,2,223,223,1005,224,479,101,1,223,223,1007,226,226,224,102,2,223,223,1005,224,494,1001,223,1,223,1108,226,677,224,102,2,223,223,1005,224,509,101,1,223,223,1008,677,226,224,102,2,223,223,1005,224,524,1001,223,1,223,1007,677,226,224,102,2,223,223,1005,224,539,101,1,223,223,1108,226,226,224,1002,223,2,223,1005,224,554,101,1,223,223,108,226,226,224,102,2,223,223,1005,224,569,101,1,223,223,108,677,677,224,102,2,223,223,1005,224,584,101,1,223,223,1107,226,226,224,1002,223,2,223,1006,224,599,101,1,223,223,8,677,226,224,1002,223,2,223,1006,224,614,1001,223,1,223,108,677,226,224,102,2,223,223,1006,224,629,1001,223,1,223,1107,677,226,224,1002,223,2,223,1006,224,644,1001,223,1,223,107,677,677,224,102,2,223,223,1005,224,659,101,1,223,223,107,226,226,224,102,2,223,223,1006,224,674,1001,223,1,223,4,223,99,226
    ];

    // let mut code: Vec<i32> = vec![
    //     3,21,
    //     1008,21,8,20,
    //     1005,20,22,
    //     107,8,21,20,1006,20,31,
    //     1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,
    //     999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99
    // ];

    let (_pc, result) = run_program(&mut code);

    // println!("{}, {}:", i, j);
    // println!("pc:      {:?}", pc);
    // println!("code[0]: {:?}", result);
    // println!("");
}

fn run_program(code: &mut Vec<i32>) -> (usize, i32) {
    let mut pc = 0;

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

                print!("3 => ");
                io::stdout().flush().unwrap();
                let mut buffer = String::new();
                io::stdin().read_line(&mut buffer).unwrap();
                let trimmed = buffer.trim();
                let num: i32 = trimmed.parse().unwrap();
                println!("num: {}, addr0: {}", num, addr0);

                code[addr0] = num;

                pc += 2;
            },
            4 => {
                let addr0: usize = code[pc+1] as usize;
                let r0 = code[addr0];

                println!("4 => {}", r0);

                pc += 2;
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

    (pc, code[0])
}
