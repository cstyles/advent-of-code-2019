use std::fs;
use std::path::Path;

#[allow(unreachable_code)]
fn main() {
    let width: i32 = 25;
    let height: i32 = 6;

    let numbers = load_file("input.txt");
    let layers: i32 = numbers.len() as i32 / height / width;

    // println!("{:#?}", numbers);
    // println!("{:#?}", layers);

    let mut image: Vec<u32> = Vec::with_capacity((width * height) as usize);
    for _ in 0..(width * height) {
        image.push(2); // Transparent
    }

    for layer in 0..layers {
        // layer = (layers - 1) - layer;
        let start = (layer * width * height) as usize;
        let end = start + (width * height) as usize;

        println!("start: {}", start);
        println!("end: {}", end);

        // for j in start..end {
        //     let image_j = j - start;
        //     if numbers[j] != 2 { // Transparent
        //         image[image_j] = numbers[j];
        //     }
        // }

        for j in start..end {
            let image_j = j - start;

            if image_j == 37 {
                dbg!(image[image_j]);
                dbg!(numbers[j]);
            }

            if image[image_j] == 2 { // Transparent
                if numbers[j] != 2 {
                    println!("Setting pixel {:03} to {} (layer {})", image_j, numbers[j], layer);
                    image[image_j] = numbers[j];
                }
            }
        }
    }

    // println!("final image");
    for y in 0..height {
        for x in 0..width {
            let offset: usize = (y * width + x) as usize;
            // let offset = offset + 14_850;
            // let offset = offset + 150;
            // let offset = offset;
            print!("{}", image[offset]);
            // print!("{}", numbers[offset]);
        }
        println!();
    }
    // io::stdout().flush().unwrap();
    // println!("{:#?}", image);

    return;

    // == part 1 ==

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
