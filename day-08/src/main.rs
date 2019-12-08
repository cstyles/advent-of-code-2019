use std::fs;
use std::path::Path;

fn main() {
    let width: usize = 25;
    let height: usize = 6;
    let numbers = load_file("input.txt");
    let layers: usize = numbers.len() as usize / height / width;

    println!("Part 1:");
    part1(width, height, &numbers, layers);
    println!();
    println!("Part 2:");
    part2(width, height, &numbers, layers);
}

fn part1(width: usize, height: usize, numbers: &[u32], layers: usize) {
    let mut min_zeroes = 999;
    let mut layer_with_min_zeroes = 0;

    for layer in 0..layers {
        let mut num_zeroes = 0;

        let start = layer * width * height;
        let end = start + (width * height);

        for pixel in &numbers[start..end] {
            if *pixel == 0 {
                num_zeroes += 1;
            }
        }

        if num_zeroes < min_zeroes {
            min_zeroes = num_zeroes;
            layer_with_min_zeroes = layer;
        }
    }

    println!("min_zeroes: {}", min_zeroes);
    println!("layer_with_min_zeroes: {}", layer_with_min_zeroes);

    let start = layer_with_min_zeroes * width * height;
    let end = start + width * height;

    let mut num_ones = 0;
    let mut num_twos = 0;
    for pixel in &numbers[start..end] {
        if *pixel == 1 {
            num_ones += 1;
        } else if *pixel == 2 {
            num_twos += 1;
        }
    }

    println!("num_ones: {}", num_ones);
    println!("num_twos: {}", num_twos);
    println!("num_ones * num_twos = {}", num_ones * num_twos);
}

fn part2(width: usize, height: usize, numbers: &[u32], layers: usize) {
    let mut image: Vec<u32> = Vec::with_capacity(width * height);
    for _ in 0..(width * height) {
        image.push(2); // Transparent
    }

    for layer in 0..layers {
        let start = layer * width * height;
        let end = start + width * height;

        for (image_index, pixel) in numbers[start..end].iter().enumerate() {
            if image[image_index] == 2 { // Transparent
                image[image_index] = *pixel;
            }
        }
    }

    for y in 0..height {
        for x in 0..width {
            let offset = y * width + x;
            print!("{}", image[offset]);
        }
        println!();
    }
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
