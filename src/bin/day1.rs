use advent_of_code_2023::day1::{find_digits, find_digits_including_words};
use std::env::args;
use std::fs::read_to_string;

fn sum_inputs(input: &str, f: fn(&str) -> u32) -> u32 {
    input.lines().map(f).reduce(|acc, code| acc + code).unwrap()
}

fn main() {
    if let Some(input_path) = args().nth(1) {
        let input_text = read_to_string(input_path).unwrap();
        println!("Part 1: {}", sum_inputs(input_text.as_str(), find_digits));
        println!(
            "Part 2: {}",
            sum_inputs(input_text.as_str(), find_digits_including_words)
        )
    }
}
