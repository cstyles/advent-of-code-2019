use std::collections::{HashMap, VecDeque};
use std::env;
use std::fs;
use std::path::Path;

use regex::Regex;

fn main() {
    let file_name = env::args().nth(1).expect("Please provide input file");
    let lines = load_code(file_name);

    let mut reactions: HashMap<String, (i64, Vec<ChemQuant>)> = HashMap::new();
    let mut amounts_produced: HashMap<String, i64> = HashMap::new();

    for (inputs, output) in lines {
        reactions.insert(
            output.chemical,
            (
                output.quantity,
                inputs,
            )
        );
    }

    let mut queue: VecDeque<ChemQuant> = VecDeque::new();
    let (_, fuel_inputs): &(i64, Vec<ChemQuant>) = reactions.get(
        &env::args().nth(2).expect("Please provide a chemical")
    ).unwrap();

    for input in fuel_inputs {
        queue.push_back(input.clone());
    }

    let mut total_ore_needed = 0;
    let mut i = 0;

    while total_ore_needed < 1_000_000_000_000 {
        while !queue.is_empty() {
            let chem_quant: ChemQuant = queue.pop_front().expect("queue was empty");
            let amount_needed = chem_quant.quantity;
            let amount_already_produced = amounts_produced.get(&chem_quant.chemical).unwrap_or(&0);
            let amount_to_make = amount_needed - amount_already_produced;

            // println!("Make {} of {}, already have {} so only need {}",
                // amount_needed,
                // chem_quant.chemical,
                // amount_already_produced,
                // amount_to_make
            // );

            if amount_to_make <= 0 {
                // println!("just using what we have");
                amounts_produced.insert(
                    chem_quant.chemical.clone(),
                    -amount_to_make,
                );
            } else if chem_quant.chemical == "ORE" {
                total_ore_needed += amount_to_make;
            } else {
                let (output_produced, inputs) = reactions.get(&chem_quant.chemical).expect("no entry in reactions for chemical");
                let reaction_times = min_needed(amount_to_make, *output_produced);

                // println!("Reaction makes {} so will run {} times", *output_produced, reaction_times);

                for input in inputs {
                    let new_item = ChemQuant {
                        chemical: input.chemical.clone(),
                        quantity: input.quantity * reaction_times,
                    };

                    amounts_produced.insert(
                        chem_quant.chemical.clone(),
                        reaction_times * *output_produced - amount_to_make,
                    );
                    queue.push_back(new_item);
                }
            }

            // println!();
        }

        i += 1;
        println!("Made {} FUEL", i);
        println!("total_ore_needed: {}", total_ore_needed);

        let (_, fuel_inputs): &(i64, Vec<ChemQuant>) = reactions.get(
            &env::args().nth(2).expect("Please provide a chemical")
        ).unwrap();

        for input in fuel_inputs {
            queue.push_back(input.clone());
        }
    }

    println!("Total ORE needed: {}", total_ore_needed);
}

fn min_needed(quantity_needed: i64, output_quantity: i64) -> i64 {
    if quantity_needed % output_quantity == 0 {
        quantity_needed / output_quantity
    } else {
        quantity_needed / output_quantity + 1
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
struct ChemQuant {
    chemical: String,
    quantity: i64,
}

fn load_code<T>(filename: T) -> Vec<(Vec<ChemQuant>, ChemQuant)>
where
    T: AsRef<Path>,
{
    // let lines: Vec<String> = fs::read_to_string(filename)
    let lines = fs::read_to_string(filename)
        .expect("Error reading input file");

    let mut return_vec = vec![];
    for line in lines.lines() {
        let re = Regex::new(r"^(?P<inputs>.+) => (?P<output>.+)").expect("invalid regex");
        let caps = re.captures(line).expect("regex didn't match anything");

        let output = &caps["output"];
        let mut split_output = output.split(" ");
        let (quant, chem) = (
            split_output.next().expect("no quantity").parse().expect("couldn't parse"),
            split_output.next().expect("no chemical"),
        );
        let output_cq = ChemQuant { chemical: String::from(chem), quantity: quant };

        let inputs_iter = caps["inputs"].split(", ");
        let mut inputs = vec![];
        for input in inputs_iter {
            let mut split_input = input.split(" ");
            let (quant, chem) = (
                split_input.next().expect("no quantity").parse().expect("couldn't parse"),
                split_input.next().expect("no chemical"),
            );
            let input_cq = ChemQuant { chemical: String::from(chem), quantity: quant };
            inputs.push(input_cq);
        }

        return_vec.push((inputs, output_cq));
    }

    return_vec
}
