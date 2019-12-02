fn main() {
    for i in 0..100 {
        for j in 0..100 {
            // println!("{}, {}", i, j);
            let mut code: Vec<i32> = vec![
                1,0,0,3,
                1,1,2,3,
                1,3,4,3,
                1,5,0,3,
                2,10,1,19,
                2,9,19,23,
                2,13,23,27,
                1,6,27,31,
                2,6,31,35,
                2,13,35,39,
                1,39,10,43,
                2,43,13,47,
                1,9,47,51,
                1,51,13,55,
                1,55,13,59,
                2,59,13,63,
                1,63,6,67,
                2,6,67,71,
                1,5,71,75,
                2,6,75,79,
                1,5,79,83,
                2,83,6,87,
                1,5,87,91,
                1,6,91,95,
                2,95,6,99,
                1,5,99,103,
                1,6,103,107,
                1,107,2,111,
                1,111,5,0,
                99,
                2,14,0,0
            ];

            code[1] = i;
            code[2] = j;

            let (_pc, result) = run_program(&mut code);

            if result == 19690720 {
                println!("{}, {}", i, j);
            }

            // println!("{}, {}:", i, j);
            // println!("pc:      {:?}", pc);
            // println!("code[0]: {:?}", result);
            // println!("");
        }
    }
}

fn run_program(code: &mut Vec<i32>) -> (usize, i32) {
    let mut pc = 0;

    while code[pc] != 99 {
        let addr0: usize = code[pc+1] as usize;
        let addr1: usize = code[pc+2] as usize;
        let addr2: usize = code[pc+3] as usize;
        let r0 = code[addr0];
        let r1 = code[addr1];

        let result = match &code[pc] {
            1 => {
                // println!("1: {} + {} = {}", r0, r1, r0 + r1);
                r0 + r1
            },
            2 => {
                // println!("2: {} * {} = {}", r0, r1, r0 * r1);
                r0 * r1
            },
            _ => {
                unreachable!();
            },
        };

        code[addr2] = result;

        pc += 4;
    }

    (pc, code[0])
}
