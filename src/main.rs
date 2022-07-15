use indicatif::ProgressBar;
use std::env;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

mod balanced_brackets;

const YES: &str = "YES";
const NO: &str = "NO";

fn bool_to_word(check: bool) -> &'static str {
    if check {
        YES
    } else {
        NO
    }
}

fn balanced_brackets_load(input: &str, output: &str) {
    let input = File::open(input).unwrap();
    let output = File::open(output).unwrap();

    let output_enumerate = BufReader::new(&output).lines().flat_map(|l| l.ok());
    let mut input_enumerate = BufReader::new(&input).lines().flat_map(|l| l.ok());

    let count_of_input = input_enumerate.next().unwrap().parse::<u64>().unwrap();
    let bar = ProgressBar::new(count_of_input);

    for (input,output) in input_enumerate.zip(output_enumerate) {
        bar.inc(1);
        let output_check = balanced_brackets::is_balanced(&input);

        if output != bool_to_word(output_check) {
            println!("{} {}", output, input);
        }
    }
    bar.finish();
}

fn main() {
    let args: Vec<String> = env::args().collect();
    balanced_brackets_load(&args[1], &args[2]);
}
