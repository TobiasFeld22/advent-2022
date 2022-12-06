use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;
use std::{
    collections::{
        hash_map::Values,
        vec_deque::{self, VecDeque},
        HashMap,
    },
    fs,
};

lazy_static! {
    static ref INSTRUCTIONS: Regex =
        Regex::new(r"(?P<amount>\d+).*?(?P<from>\d+).*?(?P<to>\d+)").unwrap();
}

fn main() {
    let file_input = fs::read_to_string("input.txt").expect("File input");
    let mut split = file_input.split("\n\n");
    let stacks = split.next().unwrap();
    let instructions = split.next().unwrap().trim();

    let mut data = fill_stacks(stacks);

    for instruction in instructions.split("\n") {
        // parse_instructions(&mut data, InstructionSet::new(instruction));
        parse_instructions_but_more(&mut data, InstructionSet::new(instruction));
    }

    // Get solution string
    let mut solution = String::new();
    let mut temp = Vec::<(i32, Vec<char>)>::new();
    for d in data {
        temp.push(d);
    }
    temp.sort_by(|a, b| a.0.cmp(&b.0));
    for t in temp {
        solution = format!("{}{}", solution, t.1.last().expect("Is empty?"));
    }
    println!("{}", solution);
}

fn parse_instructions(data: &mut HashMap<i32, Vec<char>>, instruction: InstructionSet) {
    for i in 0..instruction.amount {
        let from = data
            .get_mut(&instruction.from)
            .expect("Stack from index doesn't exist")
            .pop();

        if let Some(from) = from {
            data.get_mut(&instruction.to)
                .expect("Stack to index doesn't exist")
                .push(from);
        }
    }
}

fn parse_instructions_but_more(data: &mut HashMap<i32, Vec<char>>, instruction: InstructionSet) {
    let mut temp = VecDeque::<char>::new();
    let from = data
        .get_mut(&instruction.from)
        .expect("Stack from index doesn't exist");

    {
        for _ in 0..instruction.amount {
            let item = from.pop();

            if let Some(item) = item {
                temp.push_front(item);
            }
        }
    }
    for _ in 0..temp.len() {
        data.get_mut(&instruction.to)
            .expect("Stack to index doesn't exist")
            .push(temp.pop_front().expect("Nothing in temp anymore"));
    }
}

fn fill_stacks(input: &str) -> HashMap<i32, Vec<char>> {
    let rows = input.split("\n").collect::<Vec<&str>>();
    let reversed_rows = rows.into_iter().rev().collect::<Vec<&str>>();
    let mut stacks = HashMap::<i32, Vec<char>>::new();
    reversed_rows.iter().for_each(|row| {
        if row.contains("1") {
            return;
        }

        for (index, mut chunk) in row.chars().chunks(4).into_iter().enumerate() {
            if chunk.next() == Some('[') {
                let index = index as i32;
                let value = chunk.next().expect("Chunk is a char");
                if stacks.contains_key(&index) {
                    let old = stacks.get(&index);
                    let mut new = old.unwrap().clone();
                    new.push(value);
                    stacks.insert(index, new);
                } else {
                    let v = vec![value];
                    stacks.insert(index, v);
                }
            }
        }
    });
    return stacks;
}

#[derive(Debug)]
struct InstructionSet {
    amount: i32,
    from: i32,
    to: i32,
}

impl InstructionSet {
    fn new(input: &str) -> Self {
        let captures = INSTRUCTIONS.captures(input).expect("Parsed instruction");

        Self {
            amount: captures
                .name("amount")
                .unwrap()
                .as_str()
                .parse::<i32>()
                .unwrap(),
            from: captures
                .name("from")
                .unwrap()
                .as_str()
                .parse::<i32>()
                .unwrap()
                - 1,
            to: captures
                .name("to")
                .unwrap()
                .as_str()
                .parse::<i32>()
                .unwrap()
                - 1,
        }
    }
}
