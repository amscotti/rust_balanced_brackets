use indicatif::ProgressBar;
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

fn balanced_brackets(input: &str) -> &str {
    let mut stack: Vec<char> = Vec::new();
    let mut lookup: HashMap<char, char> = HashMap::with_capacity(3);
    lookup.insert('{', '}');
    lookup.insert('[', ']');
    lookup.insert('(', ')');

    let len_input = input.len();
    let mut count = 0;

    for index in 0..len_input {
        let c = input.chars().nth(index).unwrap();
        count = index;
        if lookup.contains_key(&c) {
            stack.push(c);
            continue;
        } else if !(stack.is_empty()) && lookup[stack.last().unwrap()] == c {
            stack.pop();
            continue;
        } else {
            break;
        }
    }

    if (count == len_input - 1) && stack.is_empty() {
        "YES"
    } else {
        "NO"
    }
}

fn balanced_brackets_load(input: &str, output: &str) {
    let input = File::open(input).unwrap();
    let output = File::open(output).unwrap();

    let mut output_enumerate = BufReader::new(&output).lines().flat_map(|l| l.ok());
    let mut input_enumerate = BufReader::new(&input).lines().flat_map(|l| l.ok());

    let count_of_input = input_enumerate.next().unwrap();
    let pb = ProgressBar::new(count_of_input.parse::<u64>().unwrap());
    for line in input_enumerate {
        pb.inc(1);
        let output = output_enumerate.next().unwrap();
        if output != balanced_brackets(&line) {
            println!("{} {}", line, output);
            break;
        }
    }
    pb.finish_with_message("done");
}

fn main() {
    let args: Vec<String> = env::args().collect();
    balanced_brackets_load(&args[1], &args[2])
}
