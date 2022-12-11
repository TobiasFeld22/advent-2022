use std::{
    collections::{HashMap, VecDeque},
    fs,
};

use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref RE: Regex = Regex::new(r"Monkey (?P<id>\d)\W+Starting items: (?P<items>(\d+,?\s?)*)  Operation: new = old (?P<Operator>[\+\-\*]) (?P<amount>\d+|old)\s*Test: divisible by (?P<div_by>\d+)\s*If true: throw to monkey (?P<true_monkey>\d+)\s+If false: throw to monkey (?P<false_monkey>\d+)").unwrap();
}

fn main() {
    println!("Hello, world!");

    let input = fs::read_to_string("input.txt").expect("file_input");
    let (m, mut monkeys) = parse_monkeys(&input);

    for c in 0..100000 {
        for i in 0..=*monkeys.keys().max().unwrap() {
            {
                let mut monkey = monkeys.get_mut(&i).unwrap().clone();
                // println!("[Monkey] {} | {:?}", i, monkey.items);

                while monkey.items.len() != 0 {
                    {
                        monkeys.get_mut(&i).unwrap().inspection_count += 1
                    }
                    let item = monkey.items.pop_front().unwrap();
                    // println!("[{}]<Item>: {}", i, item);
                    let new_item = match monkey.operation {
                        Operation::Add => match monkey.operate_by {
                            OperationAmount::NR(nr) => item + nr,
                            OperationAmount::OLD => item + item,
                        },
                        Operation::Multiply => match monkey.operate_by {
                            OperationAmount::NR(nr) => item * nr,
                            OperationAmount::OLD => item * item,
                        },
                        Operation::Subtr => match monkey.operate_by {
                            OperationAmount::NR(nr) => item - nr,
                            OperationAmount::OLD => item - item,
                        },
                    };
                    let new_item = new_item % m;
                    {
                        if new_item % monkey.divisible_by == 0 {
                            monkeys
                                .get_mut(&monkey.true_monkey)
                                .unwrap()
                                .items
                                .push_back(new_item);
                        } else {
                            monkeys
                                .get_mut(&monkey.false_monkey)
                                .unwrap()
                                .items
                                .push_back(new_item);
                        }
                    }
                }
                monkeys.get_mut(&i).unwrap().items = VecDeque::new();
            }
        }
    }
    let mut results = monkeys
        .values()
        .map(|m| return (m.id, m.inspection_count))
        .collect::<Vec<(usize, usize)>>();
    results.sort_by(|a, b| b.1.cmp(&a.1));
    println!("{:?}", results[0].1 * results[1].1);
}

fn parse_monkeys(input: &str) -> (usize, HashMap<usize, Monkey>) {
    let split = input.split("\n\n");
    let mut monkeys = HashMap::<usize, Monkey>::new();
    let mut m = 1;
    for (index, chunk) in split.enumerate() {
        let captures = RE.captures(chunk);
        for capture in captures {
            let items = capture
                .name("items")
                .unwrap()
                .as_str()
                .split(",")
                .map(|item| item.trim().parse::<usize>().unwrap())
                .collect::<VecDeque<usize>>();
            let div_condition = capture
                .name("div_by")
                .unwrap()
                .as_str()
                .parse::<usize>()
                .unwrap();

            m *= div_condition;

            let monkey = Monkey {
                inspection_count: 0,
                id: index,
                items,
                operation: match capture.name("Operator").unwrap().as_str() {
                    "*" => Operation::Multiply,
                    "+" => Operation::Add,
                    "-" => Operation::Subtr,
                    _ => panic!("Unknown operator"),
                },
                operate_by: {
                    let capt = capture.name("amount").unwrap().as_str().trim();
                    let number = capt.parse::<usize>();
                    match number {
                        Ok(nr) => OperationAmount::NR(nr),
                        Err(_) => OperationAmount::OLD,
                    }
                },
                divisible_by: div_condition,
                true_monkey: capture
                    .name("true_monkey")
                    .unwrap()
                    .as_str()
                    .parse::<usize>()
                    .unwrap(),
                false_monkey: capture
                    .name("false_monkey")
                    .unwrap()
                    .as_str()
                    .parse::<usize>()
                    .unwrap(),
            };
            monkeys.insert(monkey.id, monkey);
        }
    }
    (m, monkeys)
}

#[derive(Debug, Clone)]
pub struct Monkey {
    id: usize,
    items: VecDeque<usize>,
    operation: Operation,
    operate_by: OperationAmount,
    divisible_by: usize,
    true_monkey: usize,
    false_monkey: usize,
    inspection_count: usize,
}

#[derive(Debug, Clone)]
enum Operation {
    Add,
    Multiply,
    Subtr,
}

#[derive(Debug, Clone)]
enum OperationAmount {
    NR(usize),
    OLD,
}
