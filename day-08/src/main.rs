use std::fs;
use std::path::Path;

fn main() {
    let width: i32 = 25;
    let height: i32 = 6;

    let numbers = load_file("input.txt");
    let layers: i32 = numbers.len() as i32 / height / width;

    // println!("{:#?}", numbers);
    println!("{:#?}", layers);

    let mut min_zeroes = 1000000;
    let mut layer_with_min_zeroes = -1;

    for layer in 0..layers {
        let mut num_zeroes = 0;

        let start = (layer * width * height) as usize;
        let end = start + (width * height) as usize;
        for j in start..end {
            if numbers[j] == 0 {
                num_zeroes += 1;
            }
        }

        if num_zeroes < min_zeroes {
            // println!("max_zeroes: {}", max_zeroes);
            // println!("layer_with_max_zeroes: {}", layer_with_max_zeroes);
            min_zeroes = num_zeroes;
            layer_with_min_zeroes = layer;
        }
    }

    println!("min_zeroes: {}", min_zeroes);
    println!("layer_with_min_zeroes: {}", layer_with_min_zeroes);

    let start = (layer_with_min_zeroes * width * height) as usize;
    let end = start + (width * height) as usize;

    let mut num_ones = 0;
    let mut num_twos = 0;
    for i in start..end {
        if numbers[i] == 1 {
            num_ones += 1;
        } else if numbers[i] == 2 {
            num_twos += 1;
        }
    }

    println!("num_ones: {}", num_ones);
    println!("num_twos: {}", num_twos);

    println!("{}", num_ones * num_twos);
}

fn load_file<T>(filename: T) -> Vec<u32>
where
    T: AsRef<Path>,
{
    fs::read_to_string(filename)
        .expect("Error reading input file")
        .trim()
        .chars()
        .map(|num| num.to_digit(10).unwrap())
        .collect()
}
