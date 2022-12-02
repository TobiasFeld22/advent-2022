mod rps_option;

use std::fs;

use rps_option::RPSGuess;

fn main() {
    let file = fs::read_to_string("input.txt").unwrap();
    let trimmed_file = file.trim();

    let pairs: Vec<RPSGuess> = trimmed_file
        .split("\n")
        .map(|pair| RPSGuess::new_part_2(&pair))
        .collect();

    let results = pairs
        .iter()
        .map(|pair| return pair.cmp() as u64)
        .reduce(|a, b| a + b)
        .unwrap();

    println!("{}", results);
}

#[allow(dead_code)]
fn part_1() {
    let file = fs::read_to_string("input.txt").unwrap();
    let trimmed_file = file.trim();

    let pairs: Vec<RPSGuess> = trimmed_file
        .split("\n")
        .map(|pair| RPSGuess::new(&pair))
        .collect();

    let results = pairs
        .iter()
        .map(|pair| return pair.cmp() as u64)
        .reduce(|a, b| a + b)
        .unwrap();

    println!("{}", results);
}
