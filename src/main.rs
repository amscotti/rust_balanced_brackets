use std::env;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

mod balanced_brackets;

const YES: &str = "YES";

fn balanced_brackets_load(input: File, output: File) {
    let output_list = BufReader::new(&output)
        .lines()
        .flat_map(|l| l.ok())
        .collect::<Vec<String>>();

    let input_list = BufReader::new(&input)
        .lines()
        .skip(1)
        .flat_map(|l| l.ok())
        .collect::<Vec<String>>();

    let mismatched = input_list
        .iter()
        .zip(output_list.iter())
        .map(|(i, o)| (i, balanced_brackets::is_balanced(i), o == YES))
        .filter(|(_, i, o)| i != o)
        .collect::<Vec<(&String, bool, bool)>>();

    println!(
        "{} items checked, {} mismatched",
        input_list.len(),
        mismatched.len()
    );

    for (input, is_balanced, check) in mismatched.iter() {
        println!(
            "\nExpecting {} but got {} for,\n {}",
            check.to_string().to_uppercase(),
            is_balanced.to_string().to_uppercase(),
            input
        );
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let input = File::open(&args[1]).expect("Unable to find input file");
    let output = File::open(&args[2]).expect("Unable to find output file");

    balanced_brackets_load(input, output);
}
