use std::fs;

use itertools::Itertools;
use substring::Substring;

fn main() {
    let string = fs::read_to_string("input.txt").expect("File");

    let groups = string.split("\n").tuples::<(&str, &str, &str)>();

    let result = groups
        .map(|group| {
            let output = get_same_in_three_rs(group.0, group.1, group.2).unwrap();
            let value = get_value_from_char(output);
            return value;
        })
        .fold(0, |a, b| a + b);

    println!("{}", result);
}

#[allow(dead_code)]
fn part_one(string: String) {
    let score = string
        .split("\n")
        .map(|bags| {
            let input = bags.trim();
            let length = input.len() / 2;
            let first = input.substring(0, length);
            let second = input.substring(length, length * 2);
            let same_char = get_same_in_both_rs(first, second).unwrap();
            return get_value_from_char(same_char);
        })
        .fold(0, |a, b| a + b);

    println!("{}", score);
}

fn get_value_from_char(input: char) -> i32 {
    let start = if input.is_uppercase() { 38 } else { 96 };

    return (input as i32) - start;
}

fn get_same_in_both_rs(a: &str, b: &str) -> Option<char> {
    let mut char = Option::None;
    for c in a.chars() {
        for c2 in b.chars() {
            if c.eq(&c2) {
                char = Some(c);
            }
        }
    }
    return char;
}

fn get_same_in_three_rs(a: &str, b: &str, x: &str) -> Option<char> {
    let mut char = Option::None;
    for c in a.chars() {
        for c2 in b.chars() {
            for c3 in x.chars() {
                for c2 in b.chars() {
                    if c.eq(&c2) && c.eq(&c3) {
                        char = Some(c);
                    }
                }
            }
        }
    }
    return char;
}
