# Rust Balanced Brackets

A simple application used to experiment with the [Rust Programming Language](https://www.rust-lang.org/) using the Balanced Brackets interview question.

The application will load 2 files, one fill is a list of brackets and the other is a check file that states is the brackets are balanced or not with a `YES` or `NO`. The input file has a count on the top, which is skipped.

## Balanced Brackets Instructions
Given a string containing brackets [], braces {}, parentheses (), or any combination thereof, verify that any and all pairs are matched and nested correctly.

For example,
* `[[` would be False is the brackets are not balanced
* `{}` would be True is the brackets are balanced

## Usage

```
rust_balanced_brackets 0.1.0
Simple program to check if an input file of brackets are balanced

USAGE:
    rust_balanced_brackets --input <INPUT> --check <CHECK>

OPTIONS:
    -c, --check <CHECK>    Path to check file
    -h, --help             Print help information
    -i, --input <INPUT>    Path to input file
    -V, --version          Print version information
```