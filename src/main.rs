use clap::Parser;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

mod balanced_brackets;

/// Simple program to check if an input file of brackets are balanced
#[derive(Parser, Debug)]
#[clap(version, about, long_about = None)]
struct Args {
    /// Path to input file
    #[clap(short, long, value_parser)]
    input: String,

    /// Path to check file
    #[clap(short, long, value_parser)]
    check: String,
}

const YES: &str = "YES";

fn balanced_brackets_load(input: File, check: File) {
    let input_list = BufReader::new(&input)
        .lines()
        .skip(1)
        .flat_map(|l| l.ok())
        .collect::<Vec<String>>();

    let check_list = BufReader::new(&check)
        .lines()
        .flat_map(|l| l.ok())
        .collect::<Vec<String>>();

    let mismatched = input_list
        .iter()
        .zip(check_list.iter())
        .map(|(i, c)| (i, balanced_brackets::is_balanced(i), c == YES))
        .filter(|(_, i, c)| i != c)
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
    let args = Args::parse();

    let input = File::open(&args.input).expect("Unable to find input file");
    let check = File::open(&args.check).expect("Unable to find output file");

    balanced_brackets_load(input, check);
}
