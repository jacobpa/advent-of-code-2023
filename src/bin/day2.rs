use advent_of_code_2023::day2::Game;
use std::env::args;
use std::fs::read_to_string;

const MAX_RED: i32 = 12;
const MAX_GREEN: i32 = 13;
const MAX_BLUE: i32 = 14;

fn main() {
    if let Some(input_path) = args().nth(1) {
        let input_text = read_to_string(input_path).unwrap();
        println!(
            "Part 1: {}",
            input_text
                .lines()
                .map(Game::from_input_row)
                .filter(|g| g.red.iter().all(|c| c <= &MAX_RED)
                    && g.green.iter().all(|c| c <= &MAX_GREEN)
                    && g.blue.iter().all(|c| c <= &MAX_BLUE))
                .fold(0, |acc, g| acc + g.id)
        );
        println!(
            "Part 2: {}",
            input_text
                .lines()
                .map(Game::from_input_row)
                .fold(0, |acc, g| acc
                    + g.red.iter().max().unwrap_or(&1)
                        * g.green.iter().max().unwrap_or(&1)
                        * g.blue.iter().max().unwrap_or(&1))
        )
    }
}
