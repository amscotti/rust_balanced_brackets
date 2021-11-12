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

    let mut output_enumerate = BufReader::new(&output).lines().flat_map(|l| l.ok());
    let mut input_enumerate = BufReader::new(&input).lines().flat_map(|l| l.ok());

    let count_of_input = input_enumerate.next().unwrap();
    let bar = ProgressBar::new(count_of_input.parse::<u64>().unwrap());
    
    for line in input_enumerate {
        bar.inc(1);
        let output = output_enumerate.next().unwrap();
        let output_check = balanced_brackets::is_balanced(&line);

        if output != bool_to_word(output_check) {
            println!("{} {}", output, line);
        }
    }
    bar.finish();
}

fn main() {
    let args: Vec<String> = env::args().collect();
    balanced_brackets_load(&args[1], &args[2]);
}
